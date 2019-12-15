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

pub struct ButtonTexts {
    pub base_text: Image,
    pub hover_text: Image,
    pub disabled_text: Image,
    pub disabled_color: Color,
    pub base_color: Color,
    pub hover_color: Color,
}
impl ButtonTexts {
    pub fn new(text: String, font: Font) -> Result<Self> {
        let base = FontStyle::new(25.0, consts::FONT_WHITE_COLOR);
        let disabled = FontStyle::new(25.0, consts::BTN_DISABLED_COLOR);
        let hovered = FontStyle::new(25.0, consts::BTN_HOVERED_COLOR);
        let base_text = font.render(&text, &base)?;
        let hover_text = font.render(&text, &hovered)?;
        let disabled_text = font.render(&text, &disabled)?;
        Ok(Self {
            base_text,
            hover_text,
            disabled_text,
            base_color: consts::FONT_WHITE_COLOR,
            hover_color: consts::BTN_HOVERED_COLOR,
            disabled_color: consts::BTN_DISABLED_COLOR,
        })
    }
    pub fn get_color(&self, state: ButtonState) -> Color {
        match state {
            ButtonState::Disabled => self.disabled_color,
            ButtonState::Hovered => self.hover_color,
            _ => self.base_color,
        }
    }
    pub fn get_text(&self, state: ButtonState) -> &Image {
        match state {
            ButtonState::Disabled => &self.disabled_text,
            ButtonState::Hovered => &self.hover_text,
            _ => &self.base_text,
        }
    }
}

pub struct Button {
    pos_center: (f32, f32),
    state: ButtonState,
    text: Asset<(Image, ButtonTexts)>,
    // pressed_color: Color,
    // pressed_anim: animations::AnimationFloat,
}

impl Button {
    pub fn new(text: String, enabled: bool, pos: (f32, f32)) -> Self {
        Button {
            pos_center: pos,
            state: if enabled {
                ButtonState::Normal
            } else {
                ButtonState::Disabled
            },
            text: Asset::new(Font::load("coolvetica.ttf").and_then(move |v| {
                ButtonTexts::new(text, v)
                    .map(|v| (Image::from_bytes(&consts::BASE_BTN_IMG).unwrap(), v))
            })),
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
        let pos_center = self.pos_center.clone();
        let state = self.state;
        self.text.execute(|(bg, texts)| {
            let color = texts.get_color(state);
            window.draw_ex(
                &bg.area().with_center(pos_center),
                Blended(bg, color),
                Transform::IDENTITY,
                1,
            );
            let text = texts.get_text(state);
            window.draw_ex(
                &text.area().with_center(pos_center),
                Img(&text),
                Transform::IDENTITY,
                2,
            );
            Ok(())
        });
    }
}
