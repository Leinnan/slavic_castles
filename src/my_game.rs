use crate::player::*;
use ggez::input;
use ggez::nalgebra as na;
use ggez::{event, graphics, Context, GameResult};

type Point2 = na::Point2<f32>;
const FONT_COLOR: (f32, f32, f32, f32) = (0.05, 0.05, 0.05, 1.0);

pub struct MyGame {
    player_one: Player,
    player_two: Player,
    font: graphics::Font,
    active_player: i32,
    help_enabled: bool,
}

impl MyGame {
    pub fn get_active_player(&self) -> &Player {
        if self.active_player == 1 {
            return &self.player_one;
        }

        &self.player_two
    }

    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let player_one = Player::new(1);
        let player_two = Player::new(2);
        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let game = MyGame {
            player_one,
            player_two,
            font,
            active_player: 1,
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
        let text = graphics::Text::new((player.get_description(), font, 26.0));

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
        if self.active_player == 1 {
            self.active_player = 2;
        } else {
            self.active_player = 1;
        }
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: ggez::event::KeyCode,
        keymod: ggez::event::KeyMods,
    ) {
        if keycode == ggez::event::KeyCode::Space {
            self.switch_player();
        }
        if keycode == ggez::event::KeyCode::H {
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
        MyGame::draw_player_text(ctx, &self.player_one, Point2::new(10.0, 70.0), self.font);
        MyGame::draw_player_text(ctx, &self.player_two, Point2::new(10.0, 100.0), self.font);
        graphics::present(ctx)
    }
}
