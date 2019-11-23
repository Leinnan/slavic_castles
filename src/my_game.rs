use crate::board::Board;
use crate::ui::start_game_screen;
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

pub struct MyGame {
    board: Board,
    start_game_screen: start_game_screen::StartGameScreen,
}

impl State for MyGame {
    fn new() -> Result<Self> {
        let board = if Board::has_save() {
            Board::load_board()
        } else {
            Board::new_board()
        };

        Ok(MyGame {
            board: board,
            start_game_screen: start_game_screen::StartGameScreen::new(),
        })
    }

    fn event(&mut self, _event: &Event, window: &mut Window) -> Result<()> {
        if self.start_game_screen.visible {
            match _event {
                Event::MouseButton(_, _) => self.start_game_screen.visible = false,
                _ => {}
            };
            Ok(())
        } else {
            self.board.event(_event, window)
        }
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let delta = window.current_fps() / 1000.0;
        if self.start_game_screen.visible {
            self.start_game_screen.update(delta);
        } else {
            self.board.update(delta);
        }

        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        if self.start_game_screen.visible {
            self.start_game_screen.draw(window);
            Ok(())
        } else {
            self.board.ui.as_mut().unwrap().draw(window)
        }
    }
}
