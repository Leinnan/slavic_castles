use crate::player::*;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};

type Point2 = na::Point2<f32>;
const FONT_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);

pub struct MyGame {
    players: Vec<Player>,
    font: graphics::Font,
    active_player: usize,
    help_enabled: bool,
}

impl MyGame {
    pub fn get_active_player(&self) -> &Player {
        &self.players[self.active_player]
    }

    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let players = vec![Player::new(1), Player::new(2)];

        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let game = MyGame {
            players,
            font,
            active_player: 0,
            help_enabled: true,
        };
        Ok(game)
    }

    fn draw_player_text(ctx: &mut Context, player: &Player, pos: Point2, font: graphics::Font) {
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .color(FONT_COLOR.into())
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));
        let text = graphics::Text::new((format!("{}", player), font, 26.0));

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

    fn switch_player(&mut self) {
        if self.active_player == 0 {
            self.active_player = 1;
        } else {
            self.active_player = 0;
        }

        self.players[self.active_player].start_new_turn();
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
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.62, 0.88, 1.0, 1.0].into());
        if self.help_enabled {
            MyGame::draw_help(ctx, Point2::new(10.0, 160.0), self.font);
        }
        MyGame::draw_player_text(
            ctx,
            &self.get_active_player(),
            Point2::new(10.0, 10.0),
            self.font,
        );
        MyGame::draw_player_text(ctx, &self.players[0], Point2::new(10.0, 70.0), self.font);
        MyGame::draw_player_text(ctx, &self.players[1], Point2::new(10.0, 100.0), self.font);
        graphics::present(ctx)
    }
}
