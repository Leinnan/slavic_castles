use crate::board::Board;
use crate::card::Card;
use crate::consts;
use crate::player::*;
use crate::ui::board_ui::BoardUI;
use quicksilver::{
    graphics::Color,
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path;
use std::str;

pub struct MyGame {
    board: Board,
    ui: BoardUI,
}

impl State for MyGame {
    fn new() -> Result<Self> {
        let mut players = HashMap::new();
        players.insert(PlayerNumer::First, Player::new(true, true));
        players.insert(PlayerNumer::Second, Player::new(false, false));
        let ui = BoardUI::new()?;

        let mut game = MyGame {
            players,
            active_player: PlayerNumer::First,
            time_before_next_move: 0.0,
            game_ended: false,
            ui: ui,
        };
        game.reset_game(false);
        Ok(game)
    }

    fn event(&mut self, _event: &Event, window: &mut Window) -> Result<()> {
        match _event {
            Event::MouseMoved(_) => {
                let mouse_pos = window.mouse().pos();
                self.ui.update_hovered_card(mouse_pos.x, mouse_pos.y);
            }
            Event::MouseButton(_, _) => self.handle_mouse_input(window),
            Event::Key(_, _) => self.handle_keyboard(window),
            _ => {}
        };

        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let delta = window.current_fps() / 1000.0;
        self.ui.update(self.is_game_ended(), delta);
        if self.is_game_ended() {
            return Ok(());
        }
        if !self.can_active_player_move() {
            self.time_before_next_move -= delta;
            return Ok(());
        }

        if !self.players[&self.active_player].is_active() {
            self.handle_move_end();
        } else if !self.is_human_playing() {
            let (i, discard) = self.players[&self.active_player].get_possible_move();
            let card = self.players[&self.active_player].deck.cards[i as usize];
            self.try_use_card(&card, i, discard)
        }

        Ok(())
    }
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.ui.draw(window)
    }
}
