use std::{time::Duration, path::PathBuf};

use nannou::prelude::*;

pub fn cycle_value_over_time(
    current_time: Duration,
    cycle_duration: Duration,
    min_value: f32,
    max_value: f32,
) -> f32 {
    let fraction = (current_time.div_f32(cycle_duration.as_secs_f32()))
        .as_secs_f32()
        .fract();
    let cycled_fraction = (fraction - 0.5).abs();
    return map_range(cycled_fraction, 0.0, 0.5, min_value, max_value);
}

pub fn cycle_value_factory(
    cycle_duration: Duration,
    min_value: f32,
    max_value: f32,
) -> impl Fn(Duration) -> f32 {
    move |current_time: Duration| {
        cycle_value_over_time(current_time, cycle_duration, min_value, max_value)
    }
}