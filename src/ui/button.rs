use crate::consts;
use crate::ui::animations;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

pub struct Button {
    pos_center: (f32, f32),
    text: String,
    bg: Image,
    state: ButtonState,
    font: Asset<Font>,
    base_color: Color,
    hovered_color: Color,
    // pressed_color: Color,
    disabled_color: Color,
    // pressed_anim: animations::AnimationFloat,
}

impl Button {
    pub fn new(text: String, enabled: bool, pos: (f32, f32)) -> Self {
        Button {
            pos_center: pos,
            text: text,
            bg: Image::from_bytes(&consts::BASE_BTN_IMG).unwrap(),
            state: if enabled {
                ButtonState::Normal
            } else {
                ButtonState::Disabled
            },
            font: Asset::new(Font::load("coolvetica.ttf")),
            base_color: consts::FONT_WHITE_COLOR,
            disabled_color: consts::BTN_DISABLED_COLOR,
            hovered_color: consts::BTN_HOVERED_COLOR,
        }
    }
    pub fn is_hovered(&self) -> bool {
        self.state == ButtonState::Hovered
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        if self.state == ButtonState::Disabled {
            return;
        }
        self.state = if hovered {
            ButtonState::Hovered
        } else {
            ButtonState::Normal
        };
    }

    pub fn is_pos_over(&self, x: f32, y: f32) -> bool {
        let pos = [self.pos_center.0, self.pos_center.1];
        let scale = [1.0, 1.0];
        let start_x = pos[0] + 10.0 - consts::BTN_SIZE_X / 2.0 * scale[0];
        let end_x = pos[0] - 10.0 + consts::BTN_SIZE_X / 2.0 * scale[0];
        let start_y = pos[1] + 10.0 - consts::BTN_SIZE_Y / 2.0 * scale[1];
        let end_y = pos[1] - 10.0 + consts::BTN_SIZE_Y / 2.0 * scale[1];

        x >= start_x && x <= end_x && y >= start_y && y <= end_y
    }

    pub fn draw(&mut self, window: &mut Window) {
        let color = self.get_color();
        window.draw_ex(
            &self.bg.area().with_center(self.pos_center),
            Blended(&self.bg, color),
            Transform::IDENTITY,
            1,
        );
        let style = FontStyle::new(25.0, color);
        let btn_txt = self.text.clone();
        let pos = self.pos_center;

        self.font.execute(|f| {
            let text = f.render(&btn_txt, &style)?;
            window.draw_ex(
                &text.area().with_center(pos),
                Img(&text),
                Transform::IDENTITY,
                2,
            );
            Ok(())
        });
    }

    fn get_color(&self) -> Color {
        match self.state {
            ButtonState::Disabled => self.disabled_color,
            ButtonState::Hovered => self.hovered_color,
            _ => self.base_color,
        }
    }
}
