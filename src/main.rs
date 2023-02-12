
use std::time::Duration;

use nannou::prelude::*;

const CIRCLE_MIN: f32 = 10.0;
const CIRCLE_MAX: f32 = 100.0;
const CYCLE_SECONDS: f32 = 10.0;

struct Model {
    circle_radius: f32,
}

impl Model {
    fn calculate_circle_radius(&mut self, since_start: Duration) {
        let fraction = (since_start / CYCLE_SECONDS as u32).as_secs_f32().fract();
        let cycled_fraction = (fraction - 0.5).abs();
        self.circle_radius = map_range(
            cycled_fraction,
                0.0,
                0.5,
                CIRCLE_MIN,
                CIRCLE_MAX,
            );
    }
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {
        circle_radius: CIRCLE_MIN,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let since_start = app.duration.since_start;
    model.calculate_circle_radius(since_start);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window_rect = app.window_rect();
    draw.background().color(BLACK);
    draw.translate(window_rect.xy().extend(0.0));
    draw.ellipse()
        .color(BLANCHEDALMOND)
        .radius(model.circle_radius)
        .x_y(0.0, 0.0);

    draw.to_frame(app, &frame).unwrap();
}
