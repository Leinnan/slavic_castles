use serde::{Deserialize, Serialize};
use quicksilver::saving::{load, save};

#[derive(Serialize, Deserialize)]
pub struct Stats {
    started_games: i32,
    won_games: i32,
    losed_games: i32,
    average_duration: f64,
    fastest_win: f64,
    longest_game: f64,
    average_moves: i32,
}


impl Stats {
    fn new() -> Self {
        Stats{
            started_games: 0,
            won_games: 0,
            losed_games: 0,
            average_duration: 0.0,
            fastest_win: 0.0,
            longest_game: 0.0,
            average_moves: 0,
        }
    }

    pub fn has_save() -> bool {
        let result = load::<Self>("slavic_castles", "stats");

        result.is_ok()
    }

    pub fn get() -> Self {
        if Stats::has_save() {
            Stats::load()
        } else {
            Stats::new()
        }
    }

    fn load() -> Self {
        load::<Self>("slavic_castles", "stats").expect("Could not load Stats")
    }

    fn save(&self) {
        save("slavic_castles", "stats", &self).expect("Could not save Stats");
    }

    pub fn ended_games(&self) -> i32 {
        self.won_games + self.losed_games
    }

    pub fn game_started() {
        let mut stats = Stats::get();

        stats.started_games += 1;

        stats.save();
    }

    pub fn game_ended(won: bool, duration: f64, moves: i32) {
        let mut stats = Stats::get();
        if won {
            stats.won_games += 1;
        } else {
            stats.losed_games += 1;
        }

        stats.average_duration = (stats.average_duration + duration) / stats.ended_games() as f64;
        stats.average_moves = (stats.average_moves + moves) / stats.ended_games();


        if ((stats.fastest_win < 0.1 || stats.fastest_win > duration) && won) {
            stats.fastest_win = duration;
        }

        if stats.longest_game < duration {
            stats.longest_game = duration;
        }

        stats.save();
    }
}