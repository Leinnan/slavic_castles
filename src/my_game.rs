use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::console::Console;
use crate::ui::game_ended_text::GameEndedText;
use crate::ui::player_info::PlayerInfo;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

type Point2 = na::Point2<f32>;

pub struct MyGame {
    players: HashMap<PlayerNumer, Player>,
    font: graphics::Font,
    active_player: PlayerNumer,
    help_enabled: bool,
    console: Console,
    game_ended_text: GameEndedText,
    player_info: PlayerInfo,
    time_before_next_move: f64,
    game_ended: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true, true));
        players.insert(PlayerNumer::Second, Player::new(false, false));

        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let player_info =
            PlayerInfo::new("Human".to_string(), true, "/avatar.png".to_string(), ctx)?;
        let game = MyGame {
            players,
            font,
            active_player: PlayerNumer::First,
            help_enabled: true,
            console: Console::new(),
            game_ended_text: GameEndedText::new(),
            player_info: player_info,
            time_before_next_move: 0.0,
            game_ended: false,
        };
        Ok(game)
    }

    pub fn is_game_ended(&self) -> bool {
        self.game_ended
    }

    pub fn reset_game(&mut self) {
        self.players
            .get_mut(&PlayerNumer::First)
            .unwrap()
            .reset(true);
        self.players
            .get_mut(&PlayerNumer::Second)
            .unwrap()
            .reset(false);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
        self.console.clear();
        self.console.message("Game restarted");
        self.game_ended = false;
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
            self.console
                .message(format!("[{0}]Card discarded: {1}", self.active_player, card).as_str());
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

        self.console
            .message(format!("[{0}]Card used: {1}", self.active_player, card).as_str());
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
            self.game_ended_text.set_player_name(id.to_string());
            self.game_ended = true;
        } else if self.players[&PlayerNumer::First].has_max_possible_tower()
            || self.players[&PlayerNumer::Second].has_max_possible_tower()
        {
            let id = if self.players[&PlayerNumer::First].has_max_possible_tower() {
                PlayerNumer::First
            } else {
                PlayerNumer::Second
            };
            self.game_ended_text.set_player_name(id.to_string());
            self.game_ended = true;
        } else {
            self.switch_player();
        }
    }

    // DRAWING START

    fn draw_deck_text(
        ctx: &mut Context,
        player: &Player,
        align_right: bool,
        font: graphics::Font,
        active: bool,
    ) {
        let color: graphics::Color = if active {
            consts::ACTIVE_FONT_COLOR.into()
        } else {
            consts::FONT_COLOR.into()
        };

        let text = 
            graphics::Text::new((
                format!("{}", player.deck),
                font,
                consts::TEXT_SIZE,
            ));

        let dest_point = if align_right {
            let (w, _) = graphics::drawable_size(ctx);
            let text_length =
                player.to_string().chars().count() as f32 * consts::FONT_WIDTH * consts::TEXT_SCALE;
            Point2::new(w as f32 - text_length - 10.0, 210.0)
        } else {
            Point2::new(10.0, 210.0)
        };

        let drawparams = graphics::DrawParam::default()
            .dest(dest_point)
            .color(color)
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        graphics::draw(ctx, &text, drawparams);
    }

    pub fn draw_help(ctx: &mut Context, pos: Point2, font: graphics::Font) {
        let drawparams = graphics::DrawParam::default()
            .dest(pos)
            .color(consts::FONT_COLOR.into())
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);
        let text = graphics::Text::new((consts::HELP, font, consts::TEXT_SIZE));

        graphics::draw(ctx, &text, drawparams);
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
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
        if keycode == KeyCode::H {
            self.help_enabled = !self.help_enabled;
        }

        if keycode == KeyCode::M {
            self.console.switch_visibility();
        }

        if keycode == KeyCode::R {
            self.reset_game();
        }

        if !self.is_human_playing() || self.is_game_ended() || !self.can_active_player_move() {
            return;
        }

        let shift_pressed = keymod.contains(KeyMods::SHIFT);

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
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, consts::BG_COLOR.into());
        if self.help_enabled {
            MyGame::draw_help(ctx, Point2::new(10.0, 360.0), self.font);
        }
        self.player_info
            .update_info(&self.players[&PlayerNumer::First]);
        self.player_info.draw(ctx, self.font, false);
        self.player_info
            .update_info(&self.players[&PlayerNumer::Second]);
        self.player_info.draw(ctx, self.font, true);
        MyGame::draw_deck_text(
            ctx,
            &self.players[&PlayerNumer::First],
            false,
            self.font,
            PlayerNumer::First == self.active_player,
        );
        MyGame::draw_deck_text(
            ctx,
            &self.players[&PlayerNumer::Second],
            true,
            self.font,
            PlayerNumer::Second == self.active_player,
        );
        if self.is_game_ended() {
            self.game_ended_text.draw(ctx, self.font);
        }
        self.console.draw(ctx, self.font);
        graphics::present(ctx)
    }
}
