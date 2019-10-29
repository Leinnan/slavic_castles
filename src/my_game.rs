use crate::player::*;
use ggez::nalgebra as na;
use ggez::{event, graphics, Context, GameResult};

type Point2 = na::Point2<f32>;

pub struct MyGame {
    player_one: Player,
    player_two: Player,
    font: graphics::Font,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let player_one = Player::new(1);
        let player_two = Player::new(2);
        let font = graphics::Font::new(ctx, "/carolingia.ttf")?;
        let game = MyGame {
            player_one,
            player_two,
            font,
        };
        Ok(game)
    }

    pub fn draw_player_text(ctx: &mut Context, player: &Player, pos: Point2, font: graphics::Font) {
        let drawparams = graphics::DrawParam::new()
            .dest(pos)
            .rotation(0.0 as f32)
            .offset(Point2::new(0.0, 0.0));
        let text = graphics::Text::new((
            format!(
                "Player{0}: {1} tower, {2} walls",
                player.id, player.tower_hp, player.walls_hp
            ),
            font,
            20.0,
        ));

        graphics::draw(ctx, &text, drawparams);
    }
}

impl event::EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // print!("{}",&self.player_two.tower_hp);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let color = [0.0, 0.3, 1.0, 1.0].into();
        let rect = ggez::graphics::Rect::new(0.0, 0.0, 100.0, 300.0);
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
        graphics::draw(ctx, &rectangle, (na::Point2::new(20.0, 380.0),));
        MyGame::draw_player_text(ctx, &self.player_one, Point2::new(10.0, 10.0), self.font);
        MyGame::draw_player_text(ctx, &self.player_two, Point2::new(10.0, 200.0), self.font);
        graphics::present(ctx)
    }
}
