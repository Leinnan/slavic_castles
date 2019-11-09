use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::board_ui::BoardUI;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::timer;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

pub struct MyGame {
    players: HashMap<PlayerNumer, Player>,
    active_player: PlayerNumer,
    time_before_next_move: f64,
    game_ended: bool,
    ui: BoardUI,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true, true));
        players.insert(PlayerNumer::Second, Player::new(false, false));
        let ui = BoardUI::new(ctx)?;

        let game = MyGame {
            players,
            active_player: PlayerNumer::First,
            time_before_next_move: 0.0,
            game_ended: false,
            ui: ui,
        };
        Ok(game)
    }

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
}

impl event::EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ui
            .update(self.is_game_ended(), &self.players, self.active_player);
        if self.is_game_ended() {
            return Ok(());
        }
        if !self.can_active_player_move() {
            self.time_before_next_move -= timer::duration_to_f64(timer::delta(ctx));
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

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.ui.key_up_event(_ctx, keycode, keymod);

        let shift_pressed = keymod.contains(KeyMods::SHIFT);

        if keycode == KeyCode::R {
            self.reset_game(shift_pressed);
        }

        if !self.is_human_playing() || self.is_game_ended() || !self.can_active_player_move() {
            return;
        }

        if keycode == KeyCode::Key1 {
            let card = self.players[&self.active_player].deck.cards[0];
            self.try_use_card(&card, 0, shift_pressed);
        }
        if keycode == KeyCode::Key2 {
            let card = self.players[&self.active_player].deck.cards[1];
            self.try_use_card(&card, 1, shift_pressed);
        }
        if keycode == KeyCode::Key3 {
            let card = self.players[&self.active_player].deck.cards[2];
            self.try_use_card(&card, 2, shift_pressed);
        }
        if keycode == KeyCode::Key4 {
            let card = self.players[&self.active_player].deck.cards[3];
            self.try_use_card(&card, 3, shift_pressed);
        }
        if keycode == KeyCode::Key5 {
            let card = self.players[&self.active_player].deck.cards[4];
            self.try_use_card(&card, 3, shift_pressed);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, consts::BG_COLOR.into());
        self.ui.draw(ctx, &self.players);
        graphics::present(ctx)
    }
}
