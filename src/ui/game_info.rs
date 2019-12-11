use crate::consts;
use crate::ui::animations;
use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Blended, Background::Col, Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    sound::Sound,
    Result,
};

const BG_AREA: Rectangle = Rectangle {
    pos: Vector {
        x: consts::SCREEN_WIDTH / 2.0 - 100.0,
        y: 0.0,
    },
    size: Vector { x: 200.0, y: 70.0 },
};

pub struct GameInfo {
    pub game_duration: f64,
    pub moves_made: i32,
    font: Asset<Font>,
    enabled: bool,
}

impl GameInfo {
    pub fn new() -> Self {
        GameInfo {
            font: Asset::new(Font::load("coolvetica.ttf")),
            moves_made: 0,
            game_duration: 0.0,
            enabled: true,
        }
    }

    pub fn update_info(&mut self, game_duration: f64, moves: i32) {
        self.moves_made = moves;
        self.game_duration = game_duration;
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        window.draw_ex(
            &BG_AREA,
            Col(Color::BLACK.with_alpha(0.3)),
            Transform::IDENTITY,
            10,
        );

        let text_1 = format!("Moves: {}", self.moves_made);
        let time = GameInfo::split_time_mm_ss(self.game_duration);
        let text_2 = format!("{:02}:{:02}", time.0, time.1);

        self.font.execute(|f| {
            let style = FontStyle::new(25.0, Color::WHITE);
            let text_1 = f.render(&text_1, &style)?;
            let text_2 = f.render(&text_2, &style)?;
            window.draw_ex(
                &text_1
                    .area()
                    .with_center((consts::SCREEN_WIDTH / 2.0, 20.0)),
                Img(&text_1),
                Transform::IDENTITY,
                15,
            );
            window.draw_ex(
                &text_2
                    .area()
                    .with_center((consts::SCREEN_WIDTH / 2.0, 50.0)),
                Img(&text_2),
                Transform::IDENTITY,
                15,
            );
            Ok(())
        })
    }

    fn split_time_mm_ss(time: f64) -> (i32, i32) {
        (time as i32 / 60, time as i32 % 60)
    }
}
