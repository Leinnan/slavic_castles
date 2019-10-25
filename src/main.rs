use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;

type Point2 = na::Point2<f32>;

pub struct Player
{
    human: bool,
    alive: bool,
    id: i32,
    tower_hp: i32,
    walls_hp: i32
}

impl Player{
    pub fn new(newId: i32) -> Player {
        Player{
            human: true,
            alive: true,
            id: newId,
            tower_hp: 20,
            walls_hp: 15,
        }
    }

    pub fn make_tower_higher(&mut self, amount: i32)
    {
        self.tower_hp += amount;
    }

    pub fn give_damage(&mut self, amount: i32, ignore_wall: bool)
    {
        if ignore_wall {
            self.tower_hp -= amount;
        }

        if self.walls_hp < amount {
            self.tower_hp -= amount - self.walls_hp;
            self.walls_hp = 0;
        }
        else {
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

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    player_one: Player,
    player_two: Player,
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut player_one = Player::new(1);
        let mut player_two = Player::new(2);
        MyGame {
		    player_one,
            player_two
		}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        print!("{}",&self.player_two.tower_hp);
		Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let color = [0.0, 0.3, 1.0, 1.0].into();
        let rect = ggez::graphics::Rect::new(0.0,0.0,100.0,300.0);
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect , color)?;
        graphics::draw(ctx, &rectangle, (na::Point2::new(20.0, 380.0),));
		graphics::present(ctx)
    }
}