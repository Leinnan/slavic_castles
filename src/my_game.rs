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

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        if self.start_game_screen.is_active() {
            self.start_game_screen.event(event, window)
        } else {
            self.board.event(event, window)
        }
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let delta = 1.0 / 60.;
        if self.start_game_screen.is_active() {
            self.start_game_screen.update(delta);
            if self.start_game_screen.is_requesting_change() {
                match self.start_game_screen.get_current_state() {
                    start_game_screen::StartGameState::RequestGameContinue => {
                        self.board = Board::load_board();
                    }
                    _ => {
                        self.board = Board::new_board();
                    }
                }
                self.start_game_screen.disable();
            }
        } else {
            self.board.update(delta);
        }

        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        if self.start_game_screen.is_active() {
            self.start_game_screen.draw(window);
            Ok(())
        } else {
            self.board.ui.as_mut().unwrap().draw(window)
        }
    }
}
