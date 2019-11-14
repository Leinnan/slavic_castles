use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::board_ui::BoardUI;
use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Settings, State, Window},
    Result,
};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path;
use std::str;

pub struct MyGame {
    players: HashMap<PlayerNumer, Player>,
    active_player: PlayerNumer,
    time_before_next_move: f64,
    game_ended: bool,
    ui: BoardUI,
}

impl MyGame {
    pub fn is_game_ended(&self) -> bool {
        self.game_ended
    }

    pub fn reset_game(&mut self, ai_only: bool) {
        self.players
            .get_mut(&PlayerNumer::First)
            .unwrap()
            .reset(true, !ai_only);
        self.players
            .get_mut(&PlayerNumer::Second)
            .unwrap()
            .reset(false, false);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
        self.game_ended = false;
        self.ui.reset_game();
        self.ui.enable_ui_deck(self.is_human_playing());
    }

    pub fn other_player(&self) -> PlayerNumer {
        if self.active_player == PlayerNumer::First {
            PlayerNumer::Second
        } else {
            PlayerNumer::First
        }
    }

    pub fn try_use_card(&mut self, card: &Card, index: i32, discard: bool) {
        let mut player = self.players.get_mut(&self.active_player).unwrap();
        if discard {
            player.replace_card(index);
            self.ui.send_message(
                format!("[{0}]Card discarded: {1}", self.active_player, card).as_str(),
            );
            self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;

            return;
        }

        if !card.can_aford(&player.resources) {
            return;
        }

        player.change_resource_amount(&card.cost_resource, -card.cost_amount);
        player.make_tower_higher(card.tower_growth);
        player.make_walls_higher(card.walls_growth);
        player.change_resource_production(&card.production_resource, card.production_change);
        player.replace_card(index);

        self.players
            .get_mut(&self.other_player())
            .unwrap()
            .give_damage(card.damage, false);

        self.ui
            .send_message(format!("[{0}]Card used: {1}", self.active_player, card).as_str());
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
    }

    fn can_active_player_move(&self) -> bool {
        self.time_before_next_move <= 0.0
    }

    fn is_human_playing(&self) -> bool {
        self.players[&self.active_player].is_human()
    }

    fn switch_player(&mut self) {
        self.active_player = self.other_player();
        self.players
            .get_mut(&self.active_player)
            .unwrap()
            .start_new_turn();
        self.ui.enable_ui_deck(self.is_human_playing());
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
    }

    fn handle_move_end(&mut self) {
        if !self.players[&PlayerNumer::First].is_alive()
            || !self.players[&PlayerNumer::Second].is_alive()
        {
            let id = if self.players[&PlayerNumer::First].is_alive() {
                PlayerNumer::First
            } else {
                PlayerNumer::Second
            };
            self.ui.set_winner(id.to_string());
            self.game_ended = true;
        } else if self.players[&PlayerNumer::First].has_max_possible_tower()
            || self.players[&PlayerNumer::Second].has_max_possible_tower()
        {
            let id = if self.players[&PlayerNumer::First].has_max_possible_tower() {
                PlayerNumer::First
            } else {
                PlayerNumer::Second
            };
            self.ui.set_winner(id.to_string());
            self.game_ended = true;
        } else {
            self.switch_player();
        }
    }

    fn handle_mouse_input(&mut self, window: &mut Window) {
        let lmb_pressed = window.mouse()[MouseButton::Left] == ButtonState::Pressed;
        let rmb_pressed = window.mouse()[MouseButton::Right] == ButtonState::Pressed;
        let shift_pressed = window.keyboard()[Key::LShift] == ButtonState::Pressed;
        let mouse_pos = window.mouse().pos();
        if !lmb_pressed && !rmb_pressed {
            return;
        }

        if !self.is_human_playing() || self.is_game_ended() || !self.can_active_player_move() {
            return;
        }

        self.ui.hide_help();

        let i = self.ui.card_index_on_pos(mouse_pos.x, mouse_pos.y);
        if i.is_some() {
            let card = self.players[&self.active_player].deck.cards[i.unwrap()];
            self.try_use_card(&card, i.unwrap() as i32, rmb_pressed || shift_pressed);
        }
    }

    fn handle_keyboard(&mut self, window: &mut Window) {
        self.ui.handle_keyboard(window);

        let shift_pressed = window.keyboard()[Key::LShift] == ButtonState::Pressed;

        if window.keyboard()[Key::R] == ButtonState::Pressed {
            self.reset_game(shift_pressed);
        }
    }
}

impl State for MyGame {
    fn new() -> Result<Self> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true, true));
        players.insert(PlayerNumer::Second, Player::new(false, false));
        let ui = BoardUI::new()?;

        let game = MyGame {
            players,
            active_player: PlayerNumer::First,
            time_before_next_move: 0.0,
            game_ended: false,
            ui: ui,
        };
        Ok(game)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.handle_keyboard(window);
        self.handle_mouse_input(window);
        self.ui
            .update(self.is_game_ended(), &self.players, self.active_player);
        if self.is_game_ended() {
            return Ok(());
        }
        if !self.can_active_player_move() {
            self.time_before_next_move -= 1.0 / 60.0;
            return Ok(());
        }

        if !self.players[&self.active_player].is_active() {
            self.handle_move_end();
        } else if !self.is_human_playing() {
            let (i, discard) = self.players[&self.active_player].get_possible_move();
            let card = self.players[&self.active_player].deck.cards[i as usize];
            self.try_use_card(&card, i, discard)
        }

        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.ui.draw(window, &self.players)
    }
}
