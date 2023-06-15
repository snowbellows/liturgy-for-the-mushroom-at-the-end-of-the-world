// use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{
    collections::HashMap,
};

#[allow(dead_code)]
mod unused;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const NUM_ITERS: u64 = 5;
pub const NUM_SPOKES: u64 = 20;
pub const BRANCH_LENGTH: f32 = 300.0;
pub const TRANSPARENT_BLANCHED_ALMOND: (f32, f32, f32, f32) = (255.0, 235.0, 205.0, 0.000001);
pub const STEP_LENGTH: f32 = 10.0;
pub const FPS: u64 = 10;

struct Model {
    // rand_seed: u64,
    // rng: ChaCha8Rng,
    // joins: Vec<(Point2, Point2)>,
    // starting_point: Point2,
    // spoke_angles: Vec<f32>,
    starting_points: Vec<Point2>,
    lines: HashMap<String, Line>,
    // frame_capture: FrameCapture,
    window_id: WindowId,
}

#[derive(Clone)]
struct Line {
    start: Point2,
    end: Point2,
    points: Vec<Point2>,
    finished: bool,
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Line {
            start,
            end,
            points: vec![start],
            finished: false,
        }
    }
}


impl Model {
    fn new_from_app(app: &App, window_id: WindowId) -> Self {
        let rand_seed: u64 = random();

        println!("{rand_seed}");

        let mut rng = ChaCha8Rng::seed_from_u64(rand_seed);

        let window_rect = app.window_rect();

        let starting_points = (0..NUM_SPOKES)
            .map(|_| {
                vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                )
            })
            .collect();

        Model {
            // rand_seed,
            // rng,
            // starting_point,
            lines: HashMap::new(),
            starting_points,
            // spoke_angles,
            // joins,
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
    if app.elapsed_frames() % FPS != 0 {
        return;
    }
    // model.step_circles(app.duration.since_start);
    model.lines = step_lines(&model);

    // model.lines = move_lines(&model);

    (model.starting_points, model.lines) = change_points(&model, &app);
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


    for (_, line) in &model.lines {
        draw_polyline(&draw, &line.points);
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

fn vec2_hash(v1: Vec2, v2: Vec2) -> String {
    format!("{v1}:{v2}")
}

fn step_lines(model: &Model) -> HashMap<String, Line> {
    let mut lines = model.lines.clone();
    let mut rng = thread_rng();

    for p_start in &model.starting_points {
        for p_end in &model.starting_points {

            // for every start and end we grab the corresponding line between the two points or create a new one
            let line = lines
                .entry(vec2_hash(*p_start, *p_end))
                .or_insert(Line::new(*p_start, *p_end));

            // take the last point from the line
            if let Some(p_last) = line.points.last() {
                // if the last point in the line is the same as the "end point" we're done
                if p_last != p_end {
                    // println!("p_last: {p_last}");

                    // if not check if we're within x pixels of the "end point" and return that
                    let length = 3.0;
                    let d_left = p_last.distance(*p_end);
                    let p_next = if d_left <= length {
                        *p_end
                    } else {
                        // println!("percent_traversed: {percent_traversed}");

                        // randomise where the end point is for fun, curly lines
                        let rand_amount = 100;
                        let p_random = vec2(
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                        )
                        .normalize();

                        // get the vector towards the "end point"
                        let v_to_end = (*p_last - *p_end).normalize();

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

    lines
}

fn move_lines(model: &Model) -> HashMap<String, Line> {
    let mut rng = thread_rng();

    model
        .lines
        .clone()
        .into_iter()
        .map(|(hash, line)| {
            let mut line = line.clone();
            line.points = line
                .points
                .iter()
                .map(|p| {
                    let rand_amount = 100;
                    let x = rng.gen_range(-rand_amount..=rand_amount) as f32;
                    let y = rng.gen_range(-rand_amount..=rand_amount) as f32;

                    *p + (vec2(x, y).normalize() / 5.0)
                })
                .collect();
            (hash, line)
        })
        .collect()
}

fn change_points(model: &Model, app: &App) -> (Vec<Point2>, HashMap<String, Line>) {
    let mut starting_points = model.starting_points.clone();
    let mut lines = model.lines.clone();
    for (index, p_start) in (starting_points.clone()).iter().enumerate() {
        let is_finished = starting_points.iter().fold(true, |acc, p_end| {
            let line = lines.get(&vec2_hash(*p_start, *p_end));
            if let Some(line) = line {
                // take the last point from the line
                if let Some(p_last) = line.points.last() {
                    // if the last point in the line isn't the same as the "end point" we're not finished yet!
                    if p_last != p_end {
                        return false;
                    }
                }
            }
            acc
        });

        if is_finished {
            // remove the point
            if index < starting_points.len() {
                starting_points.remove(index);
            }
            // remove all it's lines
            lines = lines
                .into_iter()
                .filter(|(_, line)| {
                    if line.start == *p_start || line.end == *p_start {
                        return false;
                    }
                    true
                })
                .collect();

            // add a new point
            let window_rect = app.window_rect();
            let mut rng = thread_rng();

            let p_new = vec2(
                rng.gen_range(window_rect.x.start..window_rect.x.end),
                rng.gen_range(window_rect.y.start..window_rect.y.end),
            );

            starting_points.push(p_new);
        }
    }
    (starting_points, lines)
}

fn draw_polyline(draw: &Draw, line: &Vec<Point2>) {
    // for p in line {
    //     draw_cirlce(draw, p)
    // }
    let mut rng = thread_rng();
    let line: Vec<Point2> = line
        .iter()
        .map(|p| {
            let rand_amount = 100;
            let x = rng.gen_range(-rand_amount..=rand_amount) as f32;
            let y = rng.gen_range(-rand_amount..=rand_amount) as f32;

            *p + (vec2(x, y).normalize() * 2.0)
        })
        .collect();

    draw.polyline()
        .weight(3.0)
        .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND))
        .points(line);
}

