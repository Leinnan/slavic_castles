use crate::board::Board;
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

pub struct MyGame {
    board: Board,
}

impl State for MyGame {
    fn new() -> Result<Self> {
        let board = if Board::has_save() { Board::load_board() } else {
            Board::new_board()
        };

        Ok(MyGame {
            board: board,
        })
    }

    fn event(&mut self, _event: &Event, window: &mut Window) -> Result<()> {
        self.board.event(_event,window)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let delta = window.current_fps() / 1000.0;
        self.board.update(delta);

        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.board.ui.as_mut().unwrap().draw(window)
    }
}
