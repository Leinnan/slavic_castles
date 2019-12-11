use crate::card::Card;
use crate::card_sounds::*;
use crate::consts;
use crate::player::*;
use crate::ui::card_displayer::CardDisplayer;
use crate::ui::console::Console;
use crate::ui::game_ended_text::GameEndedText;
use crate::ui::game_info::GameInfo;
use crate::ui::help_displayer::HelpDisplayer;
use crate::ui::player_info::PlayerInfo;
use crate::ui::waste_cards::WasteCards;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{
        Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle, Image,
    },
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};
use std::collections::HashMap;

pub struct BoardUI {
    bg: Image,
    console: Console,
    game_ended_text: GameEndedText,
    game_info: GameInfo,
    player_info_left: PlayerInfo,
    player_info_right: PlayerInfo,
    card_displayers: Vec<CardDisplayer>,
    active_player: PlayerNumer,
    help: HelpDisplayer,
    waste_cards: WasteCards,
    card_sounds: CardSounds,
    deck_text_enabled: bool,
    deck_ui_enabled: bool,
    game_ended: bool,
    screen_height: f32,
    screen_width: f32,
}

impl BoardUI {
    pub fn new() -> Result<BoardUI> {
        let player_info_left = PlayerInfo::new(true, false)?;
        let player_info_right = PlayerInfo::new(false, true)?;
        let (w, h) = (1280, 720);
        let mut card_displayers = Vec::new();
        let card_scale = 0.96;
        let base_x_pos = (w as f32
            - (consts::CARDS_IN_DECK as f32 * consts::CARD_SIZE_X * card_scale / 0.9))
            / 2.0;
        for i in 0..consts::CARDS_IN_DECK as usize {
            let card_displayer = CardDisplayer::new(
                card_scale,
                base_x_pos + i as f32 * (consts::CARD_SIZE_X * card_scale),
                h as f32 - (consts::CARD_SIZE_Y * card_scale + 15.0),
                230.0 * card_scale,
            )?;
            card_displayers.push(card_displayer);
        }

        let bg = Image::from_bytes(&consts::BOARD_BG_IMG);
        let result = BoardUI {
            bg: bg.unwrap(),
            console: Console::new()?,
            game_ended_text: GameEndedText::new(),
            game_info: GameInfo::new(),
            player_info_left: player_info_left,
            player_info_right: player_info_right,
            card_displayers: card_displayers,
            active_player: PlayerNumer::First,
            waste_cards: WasteCards::new(0.7, 120.0, 60.0)?,
            card_sounds: CardSounds::new(),
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
        self.game_ended = false;
        self.console.clear();
        self.console.message("Game restarted");
        self.game_ended_text.enable(false);
        self.deck_ui_enabled = false;
        self.player_info_left.game_restarted();
        self.player_info_right.game_restarted();
    }

    pub fn hide_help(&mut self) {
        self.help.hide();
    }

    pub fn end_game_hovered(&self, pos: Vector) -> bool {
        self.game_ended_text.is_hovered(pos)
    }

    pub fn enable_ui_deck(&mut self, show: bool) {
        self.deck_ui_enabled = show;
        for i in 0..consts::CARDS_IN_DECK as usize {
            self.card_displayers[i].show(show);
        }
    }

    pub fn send_message(&mut self, msg: &str) {
        self.console.message(msg);
    }

    pub fn game_ended(&mut self, positive: bool) {
        self.game_ended = true;
        self.game_ended_text.game_ended(positive);
        self.waste_cards.game_ended();
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

    pub fn players_update(
        &mut self,
        players: &HashMap<PlayerNumer, Player>,
        active_player: PlayerNumer,
    ) {
        let player_left_is_active = active_player == PlayerNumer::First;
        self.player_info_left
            .update_info(&players[&PlayerNumer::First], player_left_is_active);
        self.player_info_right
            .update_info(&players[&PlayerNumer::Second], !player_left_is_active);
        self.active_player = active_player;
    }

    pub fn card_used(&mut self, card: &Card) {
        self.card_sounds.play_card_sound(card.cost_resource);
        self.waste_cards.card_used(card);
    }

    pub fn update_deck(&mut self, player: &Player) {
        for i in 0..consts::CARDS_IN_DECK as usize {
            let card = player.deck.cards[i];
            let can_afford = card.can_aford(&player.resources);
            self.card_displayers[i].update_info(&card, can_afford);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        for i in 0..consts::CARDS_IN_DECK as usize {
            self.card_displayers[i].update(delta_time);
        }
        self.player_info_left.update(delta_time);
        self.player_info_right.update(delta_time);
        self.game_ended_text.update(delta_time);
    }

    pub fn update_game_info(&mut self, game_duration: f64, moves: i32) {
        self.game_info.update_info(game_duration, moves);
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(consts::BG_COLOR)?;
        let screen_center = ((self.screen_width / 2.0), (self.screen_height / 2.0));
        let mut is_ok;

        window.draw_ex(
            &self.bg.area().with_center(screen_center),
            Img(&self.bg),
            Transform::IDENTITY,
            0,
        );

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
        is_ok = self.waste_cards.draw(window);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.game_info.draw(window);
        if !is_ok.is_ok() {
            return is_ok;
        }
        is_ok = self.console.draw(window);
        is_ok
    }
}
