use ggez::event;
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use std::env;
use std::path;

type Point2 = na::Point2<f32>;

pub struct Player {
    human: bool,
    alive: bool,
    id: i32,
    tower_hp: i32,
    walls_hp: i32,
}

impl Player {
    pub fn new(new_id: i32) -> Player {
        Player {
            human: true,
            alive: true,
            id: new_id,
            tower_hp: 20,
            walls_hp: 15,
        }
    }

    pub fn make_tower_higher(&mut self, amount: i32) {
        self.tower_hp += amount;
    }

    pub fn give_damage(&mut self, amount: i32, ignore_wall: bool) {
        if ignore_wall {
            self.tower_hp -= amount;
        }

        if self.walls_hp < amount {
            self.tower_hp -= amount - self.walls_hp;
            self.walls_hp = 0;
        } else {
            self.tower_hp -= amount;
            if self.tower_hp < 0 {
                self.tower_hp = 0;
            }
        }

        if self.tower_hp == 0 {
            self.alive = false;
        }
    }
}

struct MyGame {
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
        MyGame::draw_player_text(ctx, &self.player_one, Point2::new(10.0, 200.0), self.font);
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let window_mode = ggez::conf::WindowMode {
        width: 1280.0,
        height: 720.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };
    // Make a Context.
    let cb = ContextBuilder::new("my_game", "Cool Game Author")
    .window_mode(window_mode)
    .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;
    let my_game = &mut MyGame::new(ctx)?;
    event::run(ctx, event_loop, my_game)
}
