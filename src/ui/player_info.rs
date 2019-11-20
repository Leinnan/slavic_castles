use crate::consts;
use crate::player::*;
use crate::resource::*;
use crate::ui::animations;
use crate::ui::resource_info::ResourceInfo;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use rand::prelude::*;

const TEXTS_Y_POS: f32 = 175.0;
const TOWER_TEXT_X_OFFSET: f32 = 164.0;
const WALLS_TEXT_X_OFFSET: f32 = 13.0;
const FRAME_SIZE: f32 = 200.0;

pub struct PlayerInfo {
    active: bool,
    align_right: bool,
    name: String,
    tower_hp: i32,
    walls_hp: i32,
    avatar: Asset<Image>,
    frame: Asset<Image>,
    font: Asset<Font>,
    tools: ResourceInfo,
    magic: ResourceInfo,
    soldiers: ResourceInfo,
    shake_duration: f64,
    offset: (f32, f32),
    scale_anim: animations::AnimationFloat,
}

impl PlayerInfo {
    pub fn new(
        name: String,
        active: bool,
        avatar_path: String,
        align_right: bool,
    ) -> Result<PlayerInfo> {
        let avatar = Asset::new(Image::load(avatar_path));
        let frame = Asset::new(Image::load("frame.png"));
        let tools = ResourceInfo::new("tools.png".to_string(), consts::TOOLS_COLOR.into())?;
        let magic = ResourceInfo::new("potionBlue.png".to_string(), consts::MAGIC_COLOR.into())?;
        let soldiers = ResourceInfo::new("axe.png".to_string(), consts::SOLDIERS_COLOR.into())?;

        let info = PlayerInfo {
            active: active,
            align_right: align_right,
            name: name,
            tower_hp: consts::BASE_TOWER_HP,
            walls_hp: consts::BASE_WALLS_HP,
            avatar: avatar,
            frame: frame,
            font: Asset::new(Font::load("coolvetica.ttf")),
            tools: tools,
            magic: magic,
            soldiers: soldiers,
            shake_duration: 0.0,
            offset: (0f32, 0f32),
            scale_anim: animations::AnimationFloat::new(0.0, 1.0, 0.5, 0.9),
        };
        Ok(info)
    }

    pub fn game_restarted(&mut self) {
        self.scale_anim.reset();
    }

    pub fn update_info(&mut self, player: &Player, active: bool, delta: f64) {
        self.scale_anim.update(delta);
        if self.walls_hp > player.walls_hp || self.tower_hp > player.tower_hp {
            self.shake_duration = consts::AVATAR_SHAKE_DURATION;
        }
        if self.shake_duration >= 0.0 {
            self.shake_duration -= delta;
            let mut rng = thread_rng();
            self.offset = (
                rng.gen_range(
                    -consts::AVATAR_SHAKE_STRENGTH.0,
                    consts::AVATAR_SHAKE_STRENGTH.0,
                ),
                rng.gen_range(
                    -consts::AVATAR_SHAKE_STRENGTH.1,
                    consts::AVATAR_SHAKE_STRENGTH.1,
                ),
            );
        } else {
            self.offset = (0.0, 0.0);
        }
        self.walls_hp = player.walls_hp;
        self.tower_hp = player.tower_hp;
        self.active = active;

        self.tools
            .update_values(&player.resources[&ResourceType::Tools]);
        self.magic
            .update_values(&player.resources[&ResourceType::Magic]);
        self.soldiers
            .update_values(&player.resources[&ResourceType::Soldiers]);

        self.tools.update(delta);
        self.magic.update(delta);
        self.soldiers.update(delta);
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let base_y_pos = 10.0 + self.offset.1;
        let base_x_pos = if self.align_right {
            1280.0 as f32 - 210.0
        } else {
            10.0
        } + self.offset.0;
        let center = Vector::new(
            base_x_pos + (FRAME_SIZE / 2.0),
            base_y_pos + (FRAME_SIZE / 2.0),
        );
        let scale = (
            self.scale_anim.get_current_value(),
            self.scale_anim.get_current_value(),
        );
        let mut is_ok;

        let color = if self.active {
            Color::WHITE
        } else {
            Color::WHITE.multiply(consts::GREY)
        };

        is_ok = self.avatar.execute(|image| {
            window.draw_ex(
                &image.area().with_center(center),
                Blended(&image, color),
                Transform::scale(scale),
                0,
            );
            Ok(())
        });

        if !is_ok.is_ok() {
            return is_ok;
        }

        is_ok = self.frame.execute(|image| {
            window.draw_ex(
                &image.area().with_center(center),
                Img(&image),
                Transform::scale(scale),
                1,
            );
            Ok(())
        });

        if !is_ok.is_ok() {
            return is_ok;
        }

        if !self.scale_anim.is_ended() {
            return is_ok;
        }
        let walls_text = format!("{}", self.walls_hp);
        let style = FontStyle::new(26.0, consts::FONT_COLOR);
        is_ok = self.font.execute(|f| {
            let text = f.render(&walls_text, &style)?;
            window.draw_ex(
                &text
                    .area()
                    .with_center((base_x_pos + 26.0 * scale.0, base_y_pos + 172.0 * scale.1)),
                Img(&text),
                Transform::scale(scale),
                2,
            );
            Ok(())
        });
        if !is_ok.is_ok() {
            return is_ok;
        }
        let tower_text = format!("{}", self.tower_hp);

        is_ok = self.font.execute(|f| {
            let text = f.render(&tower_text, &style)?;
            window.draw_ex(
                &text.area().with_center((
                    base_x_pos + (FRAME_SIZE - 23.0) * scale.0,
                    base_y_pos + 175.0 * scale.1,
                )),
                Img(&text),
                Transform::scale(scale),
                2,
            );
            Ok(())
        });
        if !is_ok.is_ok() {
            return is_ok;
        }

        let resources_offset = if self.align_right {
            base_x_pos - 120.0
        } else {
            base_x_pos + 220.0
        };
        let resource_offset_move = if self.align_right { -95.0 } else { 95.0 };

        is_ok = self.tools.draw(window, resources_offset, 25.0);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self
            .magic
            .draw(window, resources_offset + resource_offset_move, 25.0);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self
            .soldiers
            .draw(window, resources_offset + resource_offset_move * 2.0, 25.0);
        is_ok
    }
}
