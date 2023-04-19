use std::time::Duration;

use liturgy_for_the_mushroom_at_the_end_of_the_world::{
    captured_frame_path, Circle, CIRCLE_MAX, CIRCLE_MIN, WINDOW_SIZE,
};
use nannou::prelude::*;

struct Model {
    circles: Vec<Circle>,
}

impl Model {
    fn step_circles(&mut self, since_start: Duration) {
        for c in &mut self.circles {
            c.calculate_circle_radius(since_start);
        }
    }
}

fn main() {
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
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);

    if app.duration.since_start >= Duration::from_secs(10) {
        app.quit()
    }
}
