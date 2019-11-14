use crate::consts;
use crate::player::*;
use crate::resource::*;
use crate::ui::resource_info::ResourceInfo;
use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Shape, Rectangle, Vector},
    graphics::{Background::Img, Background::Col, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};
use nalgebra;

type Point2 = nalgebra::Point2<f32>;

const TEXTS_Y_POS: f32 = 175.0;
const TOWER_TEXT_X_OFFSET: f32 = 164.0;
const WALLS_TEXT_X_OFFSET: f32 = 13.0;
const TEXT_SCALE_MULTIPLIER: f32 = 1.0;

pub struct PlayerInfo {
    active: bool,
    align_right: bool,
    name: String,
    tower_hp: i32,
    walls_hp: i32,
    // avatar: graphics::Image,
    // frame: graphics::Image,
    tools: ResourceInfo,
    magic: ResourceInfo,
    soldiers: ResourceInfo,
}

impl PlayerInfo {
    pub fn new(
        name: String,
        active: bool,
        avatar_path: String,
        align_right: bool,
    ) -> Result<PlayerInfo> {
        // let avatar = graphics::Image::new(ctx, avatar_path)?;
        // let frame = graphics::Image::new(ctx, "/frame.png")?;
        let tools = ResourceInfo::new("tools.png".to_string(), consts::TOOLS_COLOR.into())?;
        let magic = ResourceInfo::new(
            "potionBlue.png".to_string(),
            consts::MAGIC_COLOR.into(),
        )?;
        let soldiers =
            ResourceInfo::new("axe.png".to_string(), consts::SOLDIERS_COLOR.into())?;

        let info = PlayerInfo {
            active: active,
            align_right: align_right,
            name: name,
            tower_hp: consts::BASE_TOWER_HP,
            walls_hp: consts::BASE_WALLS_HP,
            // avatar: avatar,
            // frame: frame,
            tools: tools,
            magic: magic,
            soldiers: soldiers,
        };
        Ok(info)
    }

    pub fn update_info(&mut self, player: &Player) {
        self.walls_hp = player.walls_hp;
        self.tower_hp = player.tower_hp;

        self.tools
            .update_values(&player.resources[&ResourceType::Tools]);
        self.magic
            .update_values(&player.resources[&ResourceType::Magic]);
        self.soldiers
            .update_values(&player.resources[&ResourceType::Soldiers]);
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        // let (w, _) = graphics::drawable_size(ctx);
        let base_x_pos = if self.align_right {
            1280.0 as f32 - 210.0
        } else {
            10.0
        };

        // graphics::draw(
        //     ctx,
        //     &self.avatar,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(base_x_pos + 25.0, 10.0 + 25.0))
        //         .scale([1., 1.]),
        // );
        // graphics::draw(
        //     ctx,
        //     &self.frame,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(base_x_pos, 10.0))
        //         .scale([1., 1.]),
        // );

        // let walls_text =
        //     graphics::Text::new((format!("{}", self.walls_hp), font, consts::TEXT_SIZE));
        // graphics::draw(
        //     ctx,
        //     &walls_text,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(base_x_pos + WALLS_TEXT_X_OFFSET, TEXTS_Y_POS))
        //         .color((consts::FONT_COLOR).into())
        //         .scale([
        //             consts::TEXT_SCALE * TEXT_SCALE_MULTIPLIER,
        //             consts::TEXT_SCALE * TEXT_SCALE_MULTIPLIER,
        //         ]),
        // );

        // let tower_text =
        //     graphics::Text::new((format!("{}", self.tower_hp), font, consts::TEXT_SIZE));
        // graphics::draw(
        //     ctx,
        //     &tower_text,
        //     graphics::DrawParam::default()
        //         .dest(Point2::new(base_x_pos + TOWER_TEXT_X_OFFSET, TEXTS_Y_POS))
        //         .color((0.9, 0.9, 0.9, 1.0).into())
        //         .scale([
        //             consts::TEXT_SCALE * TEXT_SCALE_MULTIPLIER,
        //             consts::TEXT_SCALE * TEXT_SCALE_MULTIPLIER,
        //         ]),
        // );

        let resources_offset = if self.align_right {
            base_x_pos - 120.0
        } else {
            base_x_pos + 220.0
        };
        let resource_offset_move = if self.align_right { -95.0 } else { 95.0 };

        self.tools.draw(window, resources_offset, 25.0);
        self.magic
            .draw(window, resources_offset + resource_offset_move, 25.0);
        self.soldiers.draw(window,
            resources_offset + resource_offset_move * 2.0,
            25.0,
        );
        Ok(())
    }
}
