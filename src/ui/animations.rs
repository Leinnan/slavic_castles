use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AnimationFloat {
    pub current_value: f32,
    pub start_value : f32,
    pub end_value : f32,
    pub delay : f64,
    pub duration : f64,
    pub passed_time : f64,
    pub is_played_back: bool,
}

impl AnimationFloat {
    pub fn new(start_value: f32, end_value: f32, delay: f64, duration: f64) -> Self {
        AnimationFloat {
            current_value: 0.0,
            start_value: start_value,
            end_value: end_value,
            delay: delay,
            duration: duration,
            passed_time: 0.0,
            is_played_back: false,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.passed_time += delta;
        if self.passed_time > self.duration {
            self.passed_time = self.duration;
        }
        let percentage = self.passed_time as f32 / self.duration as f32;
        let start = if self.is_played_back { self.end_value } else { self.start_value };
        let end = if self.is_played_back { self.start_value } else { self.end_value };
        self.current_value = percentage * end + (1.0 - percentage) * start;
    }

    pub fn get_current_value(self) -> f32 {
        self.current_value
    }

    pub fn reset(&mut self) {
        self.passed_time = 0.0;
    }

    pub fn play(&mut self, play_back: bool, reset: bool) {
        if play_back != self.is_played_back {
            self.passed_time = self.duration - self.passed_time;
        }
        self.is_played_back = play_back;
        if reset {
            self.reset();
        }

    }
}