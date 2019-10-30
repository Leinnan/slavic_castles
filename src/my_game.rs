use crate::card::Card;
use crate::player::*;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

type Point2 = na::Point2<f32>;
const FONT_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);
const ACTIVE_FONT_COLOR: (f32, f32, f32, f32) = (1.0, 0.4, 0.35, 1.0);

pub struct MyGame {
    players: HashMap<PlayerNumer, Player>,
    font: graphics::Font,
    active_player: PlayerNumer,
    help_enabled: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new());
        players.insert(PlayerNumer::Second, Player::new());

        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let game = MyGame {
            players,
            font,
            active_player: PlayerNumer::First,
            help_enabled: true,
        };
        Ok(game)
    }

    pub fn other_player(&self) -> PlayerNumer {
        if self.active_player == PlayerNumer::First {
            PlayerNumer::Second
        } else {
            PlayerNumer::First
        }
    }

    pub fn try_use_card(&mut self, card: &Card) {
        if !card.can_aford(&self.players[&self.active_player].resources) {
            return;
        }
        let mut player = 
        self.players
            .get_mut(&self.other_player())
            .unwrap();

        player.change_resource_amount(&card.cost_resource, -card.cost_amount);
        player.make_tower_higher(card.tower_growth);
        player.make_walls_higher(card.walls_growth);
        player.change_resource_production(&card.production_resource, card.production_change);
        self.players
            .get_mut(&self.other_player())
            .unwrap()
            .give_damage(card.damage, false);


        println!("Card used: {}", &card.id);
        self.switch_player();
    }

    fn switch_player(&mut self) {
        self.active_player = self.other_player();
        self.players
            .get_mut(&self.active_player)
            .unwrap()
            .start_new_turn();
    }

    // DRAWING START

    fn draw_player_text(
        ctx: &mut Context,
        player: &Player,
        pos: Point2,
        font: graphics::Font,
        active: bool,
    ) {
        let color: graphics::Color = if active {
            ACTIVE_FONT_COLOR.into()
        } else {
            FONT_COLOR.into()
        };

        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(color)
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));

        let text = if active {
            graphics::Text::new((format!("{}\n{}", player, player.deck), font, 26.0))
        } else {
            graphics::Text::new((format!("{}", player), font, 26.0))
        };
        graphics::draw(ctx, &text, drawparams);
    }

    pub fn draw_help(ctx: &mut Context, pos: Point2, font: graphics::Font) {
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(FONT_COLOR.into())
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));
        let text = graphics::Text::new((
            "______________\nHelp\n Space- change player\n H- show this info\n______________",
            font,
            26.0,
        ));

        graphics::draw(ctx, &text, drawparams);
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        if keycode == KeyCode::Space {
            self.switch_player();
        }
        if keycode == KeyCode::H {
            self.help_enabled = !self.help_enabled;
        }
        if keycode == KeyCode::Key1 {
            let card = self.players[&self.active_player].deck.cards[0];
            self.try_use_card(&card);
        }
        if keycode == KeyCode::Key2 {
            let card = self.players[&self.active_player].deck.cards[1];
            self.try_use_card(&card);
        }
        if keycode == KeyCode::Key3 {
            let card = self.players[&self.active_player].deck.cards[2];
            self.try_use_card(&card);
        }
        if keycode == KeyCode::Key4 {
            let card = self.players[&self.active_player].deck.cards[3];
            self.try_use_card(&card);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.62, 0.88, 1.0, 1.0].into());
        if self.help_enabled {
            MyGame::draw_help(ctx, Point2::new(10.0, 560.0), self.font);
        }
        MyGame::draw_player_text(
            ctx,
            &self.players[&self.active_player],
            Point2::new(10.0, 140.0),
            self.font,
            true,
        );
        MyGame::draw_player_text(
            ctx,
            &self.players[&self.other_player()],
            Point2::new(10.0, 10.0),
            self.font,
            false,
        );
        graphics::present(ctx)
    }
}
