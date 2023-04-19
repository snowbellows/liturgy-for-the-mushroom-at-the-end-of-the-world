use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::prelude::*;
use std::time::Duration;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const CIRCLE_MIN: f32 = WINDOW_SIZE / 10.0;
pub const CIRCLE_MAX: f32 = WINDOW_SIZE / 3.0;

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

struct Model {
    circles: Vec<Circle>,
    frame_capture: FrameCapture,
}

impl Model {
    fn step_circles(&mut self, since_start: Duration) {
        for c in &mut self.circles {
            c.calculate_circle_radius(since_start);
        }
    }
}

pub fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .simple_window(view)
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .run();
}

fn model(app: &App) -> Model {
    let mut model = Model {
        circles: (0..3)
            .map(|i| Circle::new(CIRCLE_MIN, CIRCLE_MAX, 0.05 * i as f32))
            .collect(),
        frame_capture: FrameCapture::new_from_app(app),
    };

    model.step_circles(Duration::from_micros(0));

    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.step_circles(app.duration.since_start);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(BLACK);
    for c in &model.circles {
        draw.translate(window_rect.xy().extend(0.0));
        draw.ellipse()
            .stroke_weight(3.0)
            .stroke(BLANCHEDALMOND)
            .no_fill()
            .radius(c.radius())
            .x_y(0.0, 0.0);
    }
    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    model.frame_capture.capture_main_window_frame(app);

    if app.duration.since_start >= Duration::from_secs(10) {
        app.quit()
    }
}
