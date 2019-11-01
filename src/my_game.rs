use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::console::Console;
use ggez::event;
use ggez::timer;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

type Point2 = na::Point2<f32>;

pub struct MyGame {
    players: HashMap<PlayerNumer, Player>,
    font: graphics::Font,
    active_player: PlayerNumer,
    help_enabled: bool,
    console: Console,
    time_before_next_move: f64,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true,true));
        players.insert(PlayerNumer::Second, Player::new(false,false));

        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let game = MyGame {
            players,
            font,
            active_player: PlayerNumer::First,
            help_enabled: true,
            console: Console::new(),
            time_before_next_move: 0.0,
        };
        Ok(game)
    }

    pub fn reset_game(&mut self) {
        self.players.get_mut(&PlayerNumer::First).unwrap().reset(true);
        self.players.get_mut(&PlayerNumer::Second).unwrap().reset(false);
        self.time_before_next_move = consts::DELAY_BETWEEN_MOVES;
        self.console.clear();
        self.console.message("Game restarted");
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
            self.console.message(format!("[{0}]Card discarded: {1}", self.active_player, card).as_str());
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

        self.console.message(format!("[{0}]Card used: {1}", self.active_player, card).as_str());
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

    // DRAWING START

    fn draw_player_text(
        ctx: &mut Context,
        player: &Player,
        pos: Point2,
        font: graphics::Font,
        active: bool,
        align: graphics::Align,
    ) {
        let color: graphics::Color = if active {
            consts::ACTIVE_FONT_COLOR.into()
        } else {
            consts::FONT_COLOR.into()
        };

        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(color)
            .rotation(0.0 as f32)
            .offset(Point2::new(10.0, 10.0));

        let mut text = if player.is_human() {
            graphics::Text::new((format!("{}\n{}", player, player.deck), font, 26.0))
        } else {
            graphics::Text::new((format!("{}", player), font, 26.0))
        };

        text.set_bounds(
            Point2::new(consts::SCREEN_WIDTH / 2.0, consts::SCREEN_HEIGHT / 2.0),
            align,
        );

        graphics::draw(ctx, &text, drawparams);
    }

    pub fn draw_help(ctx: &mut Context, pos: Point2, font: graphics::Font) {
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(consts::FONT_COLOR.into())
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));
        let text = graphics::Text::new((consts::HELP, font, 26.0));

        graphics::draw(ctx, &text, drawparams);
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.can_active_player_move() {
            println!("time_before_next_move {}",self.time_before_next_move);
            self.time_before_next_move -= timer::duration_to_f64(timer::delta(ctx));
            return Ok(());
        }

        if !self.players[&self.active_player].is_active() {
            self.switch_player();
        }
        else if !self.is_human_playing() {
            let (i,discard) = self.players[&self.active_player].get_possible_move();
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

        if !self.is_human_playing() {
            return;
        }

        if keycode == KeyCode::R {
            self.reset_game();
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
        graphics::clear(ctx, [0.62, 0.88, 1.0, 1.0].into());
        if self.help_enabled {
            MyGame::draw_help(ctx, Point2::new(10.0, 360.0), self.font);
        }
        MyGame::draw_player_text(
            ctx,
            &self.players[&PlayerNumer::First],
            Point2::new(10.0, 10.0),
            self.font,
            PlayerNumer::First == self.active_player,
            graphics::Align::Left,
        );
        MyGame::draw_player_text(
            ctx,
            &self.players[&PlayerNumer::Second],
            Point2::new(consts::SCREEN_WIDTH / 2.0 - 10.0, 10.0),
            self.font,
            PlayerNumer::Second == self.active_player,
            graphics::Align::Right,
        );
        self.console.draw(ctx, self.font);
        graphics::present(ctx)
    }
}
