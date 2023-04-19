use std::{
    time::{Duration},
};

use helpers::cycle_value_over_time;

pub mod helpers;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CIRCLE_MIN: f32 = WINDOW_SIZE / 10.0;
pub const CIRCLE_MAX: f32 = WINDOW_SIZE / 3.0;
pub const CYCLE_SECONDS: f32 = 10.0;

pub struct Circle {
    current_radius: f32,
    min_radius: f32,
    max_radius: f32,
    stagger: f32,
}

impl Circle {
    pub fn new(min_radius: f32, max_radius: f32, stagger: f32) -> Self {
        Circle {
            current_radius: min_radius,
            min_radius,
            max_radius,
            stagger,
        }
    }

    pub fn calculate_circle_radius(&mut self, since_start: Duration) {
        self.current_radius = cycle_value_over_time(
            since_start + (Duration::from_millis((CYCLE_SECONDS * 1000.0 * self.stagger) as u64)),
            Duration::from_secs_f32(CYCLE_SECONDS),
            // + (CYCLE_SECONDS * self.stagger)),
            self.min_radius,
            self.max_radius,
        )
    }

    pub fn radius(&self) -> f32 {
        self.current_radius
    }
}
