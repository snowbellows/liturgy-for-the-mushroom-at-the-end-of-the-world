// use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use self::growth::{Growth, Line};

mod growth;
#[allow(dead_code)]
mod unused;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const NUM_ITERS: u64 = 5;
pub const NUM_GROWTHS: u64 = 20;
pub const BRANCH_LENGTH: f32 = 300.0;
pub const TRANSPARENT_BLANCHED_ALMOND: (f32, f32, f32, f32) = (255.0, 235.0, 205.0, 0.000001);
pub const STEP_LENGTH: f32 = 10.0;
pub const FPS: u64 = 10;

struct Model {
    growths: Vec<Growth>,
    // frame_capture: FrameCapture,
    window_id: WindowId,
}

impl Model {
    fn new_from_app(app: &App, window_id: WindowId) -> Self {
        let rand_seed: u64 = random();

        println!("{rand_seed}");

        let mut rng = ChaCha8Rng::seed_from_u64(rand_seed);

        let window_rect = app.window_rect();

        let growths = (0..NUM_GROWTHS)
            .map(|_| {
                let centre = vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                );
                Growth::new(centre)
            })
            .collect();

        Model {
            growths,
            // frame_capture: FrameCapture::new_from_app_with_seed(app, &rand_seed.to_string()),
            window_id,
        }
    }
}

pub fn main() {
    nannou::app(model).update(update).run();
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
    // if app.elapsed_frames() % FPS != 0 {
    //     return;
    // }
    // model.step_circles(app.duration.since_start);
    model.growths = step_growths(&model);
    // model.lines = move_lines(&model);

    model.growths = change_points(&model, &app);
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::Return {
        *model = Model::new_from_app(app, model.window_id);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // if app.elapsed_frames() % FPS != 0 {
    //     return;
    // }
    // let draw = app.draw().xy(model.starting_point);
    let draw = app.draw();
    draw.background().color(BLACK);

    for growth in &model.growths {
        growth.draw(&draw)
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
    let mut rng = thread_rng();

    for g_start in &mut growths {
        for g_end in &model.growths {
            let p_start = g_start.centre;
            let p_end = g_end.centre;
            // for every start and end we grab the corresponding line between the two points or create a new one
            let line = g_start
                .lines
                .entry(g_end.centre.to_string())
                .or_insert(Line::new(p_start, p_end));

            // take the last point from the line
            if let Some(p_last) = line.points.last() {
                // if the last point in the line is the same as the "end point" we're done
                if *p_last == p_end {
                    line.finished = true;
                } else {

                    // if not check if we're within x pixels of the "end point" and return that
                    let length = 2.0;
                    let d_left = p_last.distance(p_end);
                    let p_next = if d_left <= length {
                        p_end
                    } else {
                        // randomise where the end point is for fun, curly lines
                        let rand_amount = 100;
                        let rand_factor = 2.0;
                        let p_random = vec2(
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                        )
                        .normalize() * rand_factor;

                        // get the vector towards the "end point"
                        let v_to_end = (*p_last - p_end).normalize();

                        // move towards the end point and add random for fun
                        *p_last - (v_to_end * length + p_random)
                    };

                    line.points.push(p_next);
                }
            } else {
                println!("no p_last");
            }
        }
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

        let mut new_growths = (0..num_removed_growths)
            .map(|_| {
                let centre = vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                );
                Growth::new(centre)
            })
            .collect();

        growths.append(&mut new_growths);
    }

    growths
}
