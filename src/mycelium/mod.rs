use std::time::Duration;

// use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::{color::rgb_u32, prelude::*};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::helpers::cycle_value_over_time;

use self::growth::{Growth, Line};

mod growth;
#[allow(dead_code)]
mod unused;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const NUM_ITERS: u64 = 5;
pub const NUM_GROWTHS: u64 = 20;
pub const BRANCH_LENGTH: f32 = 300.0;
pub const STEP_LENGTH: f32 = 10.0;
pub const FPS: u64 = 2;

pub const TRANSPARENT_BLANCHED_ALMOND: (f32, f32, f32, f32) = (255.0, 235.0, 205.0, 0.000001);
pub const FRENCH_GREY: u32 = 0xC6BCC8;
pub const PINK_LAVENDAR: u32 = 0xD7ACCC;
pub const THISTLE: u32 = 0xD1B3BD;
pub const CHAMPAGNE: u32 = 0xFCE5C5;
pub const TEA_GREEN: u32 = 0xC5DCBC;
pub const COLOURS: [u32; 5] = [FRENCH_GREY, PINK_LAVENDAR, THISTLE, TEA_GREEN, CHAMPAGNE];

struct Model {
    growths: Vec<Growth>,
    // frame_capture: FrameCapture,
    window_id: WindowId,
    vary_amount: f32,
}

impl Model {
    fn new_from_app(app: &App, window_id: WindowId) -> Self {
        let rand_seed: u64 = random();

        println!("{rand_seed}");

        let mut rng = ChaCha8Rng::seed_from_u64(rand_seed);

        let window_rect = app.window_rect();
        let centre_points: Vec<Point2> = (0..NUM_GROWTHS)
            .map(|_| {
                vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                )
            })
            .collect();

        let growths: Vec<Growth> = (&centre_points)
            .iter()
            .map(|p_c| {
                Growth::new(
                    *p_c,
                    &centre_points,
                    rgb_u32(COLOURS[rng.gen_range(0..COLOURS.len() - 1)]).into(),
                )
            })
            .collect();

        Model {
            growths,
            // frame_capture: FrameCapture::new_from_app_with_seed(app, &rand_seed.to_string()),
            window_id,
            vary_amount: 1.0
        }
    }
}

pub fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::RefreshSync)
        .run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    app.set_fullscreen_on_shortcut(true);
    let model = Model::new_from_app(app, window_id);
    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.elapsed_frames() % FPS != 0 {
        return;
    }
    // model.step_circles(app.duration.since_start);
    model.growths = step_growths(&model);
    // model.lines = move_lines(&model);

    model.growths = change_points(&model, &app);

    model.vary_amount = cycle_value_over_time(app.duration.since_start, Duration::from_secs(12), 1.0, 20.0)
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::Return {
        *model = Model::new_from_app(app, model.window_id);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() % FPS != 0 {
        return;
    }
    // let draw = app.draw().xy(model.starting_point);
    let draw = app.draw();
    draw.background().color(BLACK);

    for growth in &model.growths {
        growth.draw(&draw, model.vary_amount)
    }

    draw.xy(app.window_rect().bottom_left() + vec2(50.0, 50.0))
        .text(&app.fps().to_string())
        .color(WHEAT);
    draw.finish_remaining_drawings();
    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    // model.frame_capture.capture_main_window_frame(app);

    // if app.duration.since_start >= Duration::from_secs(10) {
    //     app.quit()
    // }
}

fn step_growths(model: &Model) -> Vec<Growth> {
    let mut growths = model.growths.clone();

    for g in &mut growths {
        g.step_growth()
    }

    growths
}

fn change_points(model: &Model, app: &App) -> Vec<Growth> {
    // let mut starting_points = model.starting_points.clone();
    // let mut lines = model.lines.clone();

    // remove any finished growths
    let mut growths: Vec<Growth> = model
        .growths
        .clone()
        .into_iter()
        .filter(|growth| !growth.is_finished())
        .collect();

    // add a new growth for every removed one
    let num_removed_growths = NUM_GROWTHS as usize - growths.len();
    if num_removed_growths != 0 {
        let window_rect = app.window_rect();
        let mut rng = thread_rng();
        let centre_points: Vec<Vec2> = growths.iter().map(|g| g.centre).collect();

        let mut new_growths = (0..num_removed_growths)
            .map(|_| {
                let centre = vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                );

                Growth::new(
                    centre,
                    &centre_points,
                    rgb_u32(COLOURS[rng.gen_range(0..COLOURS.len() - 1)]).into(),
                )
            })
            .collect();

        growths.append(&mut new_growths);
    }

    growths
}
