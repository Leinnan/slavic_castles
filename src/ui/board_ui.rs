use crate::consts;
use crate::player::*;
use crate::ui::card_displayer::CardDisplayer;
use crate::ui::console::Console;
use crate::ui::game_ended_text::GameEndedText;
use crate::ui::player_info::PlayerInfo;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use std::collections::HashMap;

type Point2 = na::Point2<f32>;

pub struct BoardUI {
    console: Console,
    game_ended_text: GameEndedText,
    player_info_left: PlayerInfo,
    player_info_right: PlayerInfo,
    card_displayers: Vec<CardDisplayer>,
    active_player: PlayerNumer,
    font: graphics::Font,
    help_enabled: bool,
    deck_text_enabled: bool,
    deck_ui_enabled: bool,
    game_ended: bool,
}

impl BoardUI {
    pub fn new(ctx: &mut Context) -> GameResult<BoardUI> {
        let font = graphics::Font::new(ctx, "/coolvetica.ttf")?;
        let player_info_left = PlayerInfo::new(
            "Human".to_string(),
            true,
            "/avatar.png".to_string(),
            false,
            ctx,
        )?;
        let player_info_right = PlayerInfo::new(
            "Human".to_string(),
            false,
            "/avatar.png".to_string(),
            true,
            ctx,
        )?;
        let (_, h) = graphics::drawable_size(ctx);
        let mut card_displayers = Vec::new();
        for i in 0..consts::CARDS_IN_DECK as usize {
            let mut card_displayer = CardDisplayer::new(ctx)?;
            card_displayer.set_pos(
                    10.0 + i as f32 * consts::CARD_SIZE_X,
                    h as f32 - consts::CARD_SIZE_Y);
            card_displayers.push(card_displayer);
        }

        let result = BoardUI {
            console: Console::new(ctx)?,
            game_ended_text: GameEndedText::new(),
            player_info_left: player_info_left,
            player_info_right: player_info_right,
            card_displayers: card_displayers,
            active_player: PlayerNumer::First,
            font,
            help_enabled: false,
            deck_text_enabled: false,
            deck_ui_enabled: true,
            game_ended: false,
        };
        Ok(result)
    }

    pub fn reset_game(&mut self) {
        self.console.clear();
        self.console.message("Game restarted");
        self.game_ended_text.enable(false);
        self.deck_ui_enabled = false;
    }

    pub fn enable_ui_deck(&mut self, show: bool) {
        self.deck_ui_enabled = show;
    }

    pub fn send_message(&mut self, msg: &str) {
        self.console.message(msg);
    }

    pub fn set_winner(&mut self, name: String) {
        self.game_ended_text.set_player_name(name);
        self.game_ended_text.enable(true);
    }

    fn draw_deck_text(&self, ctx: &mut Context, player: &Player, align_right: bool, active: bool) {
        let color: graphics::Color = if active {
            consts::ACTIVE_FONT_COLOR.into()
        } else {
            consts::FONT_COLOR.into()
        };

        let text = graphics::Text::new((format!("{}", player.deck), self.font, consts::TEXT_SIZE));

        let dest_point = if align_right {
            let (w, _) = graphics::drawable_size(ctx);
            let text_length =
                player.to_string().chars().count() as f32 * consts::FONT_WIDTH * consts::TEXT_SCALE;
            Point2::new(w as f32 - text_length - 10.0, 210.0)
        } else {
            Point2::new(10.0, 210.0)
        };

        let drawparams = graphics::DrawParam::default()
            .dest(dest_point)
            .color(color)
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);

        graphics::draw(ctx, &text, drawparams);
    }

    pub fn draw_help(&self, ctx: &mut Context, pos: Point2) {
        let drawparams = graphics::DrawParam::default()
            .dest(pos)
            .color(consts::FONT_COLOR.into())
            .scale([consts::TEXT_SCALE, consts::TEXT_SCALE]);
        let text = graphics::Text::new((consts::HELP, self.font, consts::TEXT_SIZE));

        graphics::draw(ctx, &text, drawparams);
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        if keycode == KeyCode::H {
            self.help_enabled = !self.help_enabled;
        }
        if keycode == KeyCode::N {
            self.deck_text_enabled = !self.deck_text_enabled;
        }

        if keycode == KeyCode::M {
            self.console.switch_visibility();
        }
    }

    pub fn update(
        &mut self,
        game_ended: bool,
        players: &HashMap<PlayerNumer, Player>,
        active_player: PlayerNumer,
    ) {
        for i in 0..consts::CARDS_IN_DECK as usize {
            let card = players[&PlayerNumer::First].deck.cards[i];
            let can_afford = card.can_aford(&players[&PlayerNumer::First].resources);
            let can_use = self.deck_ui_enabled && can_afford;
            self.card_displayers[i].update_info(&card, can_use);
        }
        self.player_info_left
            .update_info(&players[&PlayerNumer::First]);
        self.player_info_right
            .update_info(&players[&PlayerNumer::Second]);
        self.active_player = active_player;
        self.game_ended = game_ended;
    }

    pub fn draw(&mut self, ctx: &mut Context, players: &HashMap<PlayerNumer, Player>) {
        let (_, h) = graphics::drawable_size(ctx);
        if self.help_enabled {
            self.draw_help(ctx, Point2::new(10.0, h as f32 - 260.0));
        }

        if self.deck_text_enabled {
            self.draw_deck_text(
                ctx,
                &players[&PlayerNumer::First],
                false,
                PlayerNumer::First == self.active_player,
            );
            self.draw_deck_text(
                ctx,
                &players[&PlayerNumer::Second],
                true,
                PlayerNumer::Second == self.active_player,
            );
        }

        if !self.game_ended {
            for i in 0..consts::CARDS_IN_DECK as usize {
                self.card_displayers[i].draw(
                    ctx,
                    self.font,
                );
            }
            self.console.draw(ctx, self.font);
        } else {
            self.game_ended_text.draw(ctx, self.font);
        }
        self.player_info_left.draw(ctx, self.font);
        self.player_info_right.draw(ctx, self.font);
    }
}
