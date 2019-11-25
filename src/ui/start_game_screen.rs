use crate::board::Board;
use crate::consts;
use crate::ui::animations;
use crate::ui::button;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    Future, Result,
};

#[derive(PartialEq, Eq, Hash, Copy, Debug, Clone)]
pub enum StartGameState {
    Active,
    RequestNewGame,
    RequestGameContinue,
    Inactive,
}

pub struct StartGameScreen {
    bg: Image,
    logo: Image,
    logo_scale_anim: animations::AnimationFloat,
    new_game_btn: button::Button,
    continue_game_btn: button::Button,
    state: StartGameState,
}

impl StartGameScreen {
    pub fn new() -> Self {
        let bg = Image::from_bytes(&consts::START_SCREEN_BG_IMG);
        let logo = Image::from_bytes(&consts::LOGO_IMG);
        let has_save = Board::has_save();
        StartGameScreen {
            bg: bg.unwrap(),
            logo: logo.unwrap(),
            logo_scale_anim: animations::AnimationFloat::new(0.0, 1.0, 1.3, 3.3),
            state: StartGameState::Active,
            new_game_btn: button::Button::new(
                "New Game".to_string(),
                true,
                (
                    (consts::SCREEN_WIDTH / 2.0),
                    (consts::SCREEN_HEIGHT / 2.0 + 90.0),
                ),
            ),
            continue_game_btn: button::Button::new(
                "Continue Game".to_string(),
                has_save,
                ((consts::SCREEN_WIDTH / 2.0), (consts::SCREEN_HEIGHT / 2.0)),
            ),
        }
    }

    pub fn disable(&mut self) {
        self.state = StartGameState::Inactive;
    }

    pub fn is_active(&self) -> bool {
        self.state != StartGameState::Inactive
    }

    pub fn is_requesting_change(&self) -> bool {
        self.state == StartGameState::RequestGameContinue
            || self.state == StartGameState::RequestNewGame
    }

    pub fn get_current_state(&self) -> StartGameState {
        self.state
    }

    pub fn update(&mut self, delta_time: f64) {
        self.logo_scale_anim.update(delta_time);
    }

    pub fn event(&mut self, _event: &Event, window: &mut Window) -> Result<()> {
        match _event {
            Event::MouseMoved(_) => {
                let mouse_pos = window.mouse().pos();
                let new_game_btn_hovered = self.new_game_btn.is_pos_over(mouse_pos.x, mouse_pos.y);
                self.new_game_btn.set_hovered(new_game_btn_hovered);
                let continue_game_btn_hovered =
                    self.continue_game_btn.is_pos_over(mouse_pos.x, mouse_pos.y);
                self.continue_game_btn
                    .set_hovered(continue_game_btn_hovered);
            }
            Event::MouseButton(_, _) => {
                let anim_ended = self.logo_scale_anim.is_ended();
                let lmb_pressed = window.mouse()[MouseButton::Left] == ButtonState::Pressed;
                if anim_ended && lmb_pressed && self.new_game_btn.is_hovered() {
                    self.state = StartGameState::RequestNewGame;
                }
                if anim_ended && lmb_pressed && self.continue_game_btn.is_hovered() {
                    self.state = StartGameState::RequestNewGame;
                }
            }
            _ => {}
        };

        Ok(())
    }

    pub fn draw(&mut self, window: &mut Window) {
        if !self.is_active() {
            return;
        }
        let screen_center = ((consts::SCREEN_WIDTH / 2.0), (consts::SCREEN_HEIGHT / 2.0));

        window.draw_ex(
            &self.bg.area().with_center(screen_center),
            Img(&self.bg),
            Transform::IDENTITY,
            0,
        );
        let scale = (
            self.logo_scale_anim.get_current_value(),
            self.logo_scale_anim.get_current_value(),
        );
        window.draw_ex(
            &self.logo.area().with_center((screen_center.0, 120.0)),
            Img(&self.logo),
            Transform::scale(scale),
            1,
        );

        self.new_game_btn.draw(window);
        self.continue_game_btn.draw(window);
    }
}
