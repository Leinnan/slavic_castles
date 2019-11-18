use crate::consts;
use crate::player::*;
use crate::ui::card_displayer::CardDisplayer;
use crate::ui::console::Console;
use crate::ui::game_ended_text::GameEndedText;
use crate::ui::help_displayer::HelpDisplayer;
use crate::ui::player_info::PlayerInfo;
use nalgebra;
use quicksilver::{
    combinators::result,
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Background::Img, Color, Font, FontStyle, Image},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use std::collections::HashMap;

type Point2 = nalgebra::Point2<f32>;

pub struct BoardUI {
    console: Console,
    game_ended_text: GameEndedText,
    player_info_left: PlayerInfo,
    player_info_right: PlayerInfo,
    card_displayers: Vec<CardDisplayer>,
    active_player: PlayerNumer,
    help: HelpDisplayer,
    font: Asset<Font>,
    deck_text_enabled: bool,
    deck_ui_enabled: bool,
    game_ended: bool,
    screen_height: f32,
    screen_width: f32,
}

impl BoardUI {
    pub fn new() -> Result<BoardUI> {
        let player_info_left =
            PlayerInfo::new("Human".to_string(), true, "avatar.png".to_string(), false)?;
        let player_info_right =
            PlayerInfo::new("Human".to_string(), false, "avatar.png".to_string(), true)?;
        let (w, h) = (1280, 720);
        let mut card_displayers = Vec::new();
        let base_x_pos = (w as f32 - consts::CARDS_IN_DECK as f32 * consts::CARD_SIZE_X) / 2.0;
        for i in 0..consts::CARDS_IN_DECK as usize {
            let mut card_displayer = CardDisplayer::new()?;
            card_displayer.set_pos(
                base_x_pos + i as f32 * consts::CARD_SIZE_X,
                h as f32 - consts::CARD_SIZE_Y,
            );
            card_displayers.push(card_displayer);
        }

        let result = BoardUI {
            console: Console::new()?,
            game_ended_text: GameEndedText::new(),
            player_info_left: player_info_left,
            player_info_right: player_info_right,
            card_displayers: card_displayers,
            active_player: PlayerNumer::First,
            font: Asset::new(Font::load("coolvetica.ttf")),
            help: HelpDisplayer::new()?,
            deck_text_enabled: false,
            deck_ui_enabled: true,
            game_ended: false,
            screen_height: h as f32,
            screen_width: w as f32,
        };
        Ok(result)
    }

    pub fn reset_game(&mut self) {
        self.console.clear();
        self.console.message("Game restarted");
        self.game_ended_text.enable(false);
        self.deck_ui_enabled = false;
    }

    pub fn hide_help(&mut self) {
        self.help.hide();
    }

    pub fn enable_ui_deck(&mut self, show: bool) {
        self.deck_ui_enabled = show;
        let base_x_pos =
            (self.screen_width - consts::CARDS_IN_DECK as f32 * consts::CARD_SIZE_X) / 2.0;
        let y_pos = if show {
            self.screen_height as f32 - consts::CARD_SIZE_Y
        } else {
            self.screen_height as f32 - consts::CARD_SIZE_Y + 100.0
        };
        for i in 0..consts::CARDS_IN_DECK as usize {
            self.card_displayers[i].set_pos(base_x_pos + i as f32 * consts::CARD_SIZE_X, y_pos);
        }
    }

    pub fn send_message(&mut self, msg: &str) {
        self.console.message(msg);
    }

    pub fn set_winner(&mut self, name: String) {
        self.game_ended_text.set_player_name(name);
        self.game_ended_text.enable(true);
    }

    fn draw_deck_text(&self, player: &Player, align_right: bool, active: bool) {
        // let color: graphics::Color = if active {
        //     consts::ACTIVE_FONT_COLOR.into()
        // } else {
        //     consts::FONT_COLOR.into()
        // };

        // let text = graphics::Text::new((format!("{}", player.deck), self.font, consts::TEXT_SIZE));

        // let dest_point = if align_right {
        //     let (w, _) = graphics::drawable_size(ctx);
        //     let text_length =
        //         player.to_string().chars().count() as f32 * consts::FONT_WIDTH * consts::TEXT_SCALE;
        //     Point2::new(w as f32 - text_length - 10.0, 210.0)
        // } else {
        //     Point2::new(10.0, 210.0)
        // };

        // let drawparams = graphics::DrawParam::default()
        //     .dest(dest_point)
        //     .color(color)
        //     .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        // graphics::draw(ctx, &text, drawparams);
    }

    pub fn card_index_on_pos(&mut self, x: f32, y: f32) -> Option<usize> {
        for i in 0..self.card_displayers.len() {
            if self.card_displayers[i].is_pos_over(x, y) {
                return Some(i);
            }
        }
        None
    }

    pub fn update_hovered_card(&mut self, x: f32, y: f32) {
        for i in 0..self.card_displayers.len() {
            let is_over = self.card_displayers[i].is_pos_over(x, y);
            self.card_displayers[i].set_hovered(is_over);
        }
    }

    pub fn handle_keyboard(&mut self, window: &mut Window) {
        if window.keyboard()[Key::H] == ButtonState::Pressed {
            self.help.switch_visibility();
        }
        if window.keyboard()[Key::N] == ButtonState::Pressed {
            self.deck_text_enabled = !self.deck_text_enabled;
        }

        if window.keyboard()[Key::M] == ButtonState::Pressed {
            self.console.switch_visibility();
        }
    }

    pub fn update(
        &mut self,
        game_ended: bool,
        players: &HashMap<PlayerNumer, Player>,
        active_player: PlayerNumer,
        delta_time: f64,
    ) {
        for i in 0..consts::CARDS_IN_DECK as usize {
            let card = players[&PlayerNumer::First].deck.cards[i];
            let can_afford = card.can_aford(&players[&PlayerNumer::First].resources);
            self.card_displayers[i].update_info(&card, can_afford);
        }
        let player_left_is_active = active_player == PlayerNumer::First;
        self.player_info_left.update_info(
            &players[&PlayerNumer::First],
            player_left_is_active,
            delta_time,
        );
        self.player_info_right.update_info(
            &players[&PlayerNumer::Second],
            !player_left_is_active,
            delta_time,
        );
        self.active_player = active_player;
        self.game_ended = game_ended;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut is_ok;
        if !self.game_ended {
            for i in 0..consts::CARDS_IN_DECK as usize {
                is_ok = self.card_displayers[i].draw(window);
                if !is_ok.is_ok() {
                    return is_ok;
                }
            }
        } else {
            is_ok = self.game_ended_text.draw(window);
            if !is_ok.is_ok() {
                return is_ok;
            }
        }
        is_ok = self.player_info_left.draw(window);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.player_info_right.draw(window);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.help.draw(window);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.console.draw(window);
        is_ok
    }
}
