use std::{
    path::PathBuf,
    time::{self, Duration},
};

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

pub struct FrameCapture {
    /// Create a folder path that we want to save the frames to
    dir_path: PathBuf,
}

impl FrameCapture {
    /// Standard directory called `/<path_to_project>/output/<start_time>`.
    pub fn new_from_app(app: &App) -> Self {
        let start_time = chrono::Local::now().format("%Y-%m-%d:%H:%M:%S");

        return FrameCapture {
            dir_path: app
                .project_path()
                .expect("failed to locate `project_path`")
                .join("output")
                .join(start_time.to_string()),
        };
    }

    pub fn capture_main_window_frame(&self, app: &App) {
        let file_path = self
            .dir_path
            .join(format!("{:05}", app.elapsed_frames() + 1))
            .with_extension("png");

        app.main_window().capture_frame(file_path);
    }
}
