use std::{collections::HashMap, time::Duration};

// use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::{color::rgb_u32, lyon::geom::euclid::default, prelude::*};
use rand::prelude::*;

use crate::helpers::*;

use self::growth::Growth;

mod growth;
#[allow(dead_code)]
mod unused;

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const NUM_ITERS: u64 = 5;
pub const NUM_GROWTHS: u64 = 20;
pub const BRANCH_LENGTH: f32 = 300.0;
pub const STEP_LENGTH: f32 = 10.0;
pub const FPS: u64 = 5;

pub const TRANSPARENT_BLANCHED_ALMOND: (f32, f32, f32, f32) = (255.0, 235.0, 205.0, 0.000001);
pub const FRENCH_GREY: u32 = 0xC6BCC8;
pub const PINK_LAVENDAR: u32 = 0xD7ACCC;
pub const THISTLE: u32 = 0xD1B3BD;
pub const CHAMPAGNE: u32 = 0xFCE5C5;
pub const TEA_GREEN: u32 = 0xC5DCBC;
pub const COLOURS: [u32; 5] = [FRENCH_GREY, PINK_LAVENDAR, THISTLE, TEA_GREEN, CHAMPAGNE];

pub struct Config {
    // max_vary_amount: f32,
    // step_amount: f32,
    // rand_factor: f32,
    values: HashMap<String, f32>,
    keys: Vec<String>,
    selected_value: usize,
}

impl Config {
    fn new(values: HashMap<String, f32>) -> Self {
        let keys: Vec<String> = values.keys().cloned().collect();
        Self {
            values,
            keys,
            selected_value: 0,
        }
    }

    pub fn get_with_default(&self, key: &str, default: f32) -> f32 {
        *self.values.get(key).unwrap_or(&default)
    }

    pub fn next_key(&mut self) {
        

        if self.selected_value == self.keys.len() -1 {
            self.selected_value = 0;
        } else {
            self.selected_value += 1;
        };
    }

    pub fn prev_key(&mut self) {
        

        if self.selected_value == 0 {
            self.selected_value = self.keys.len() - 1;
        } else {
            self.selected_value -= 1;
        };
    }

    pub fn change_value(&mut self, amount: f32) {
        let key = &self.keys[self.selected_value];
        let mut value = self.values.get_mut(key).unwrap();
        *value += amount;
    }
}
struct Model {
    growths: Vec<Growth>,
    // frame_capture: FrameCapture,
    main_window_id: WindowId,
    config_window_id: WindowId,
    vary_amount: f32,
    config: Config,
}

impl Model {
    fn new_from_app(app: &App, main_window_id: WindowId, config_window_id: WindowId) -> Self {
        let growths: Vec<Growth> = create_new_growths(app, main_window_id, NUM_GROWTHS);

        Model {
            growths,
            // frame_capture: FrameCapture::new_from_app_with_seed(app, &rand_seed.to_string()),
            main_window_id,
            config_window_id,
            vary_amount: 1.0,
            config: Config::new(HashMap::from([
                ("max_vary_amount".to_string(), 8.0),
                ("step_amount".to_string(), 8.0),
                ("rand_factor".to_string(), 1.5),
            ])),
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
    let main_window_id = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .view(main_view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let config_window_id = app
        .new_window()
        .title("Config")
        .size(WINDOW_SIZE as u32, WINDOW_SIZE as u32)
        .view(config_view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    app.set_fullscreen_on_shortcut(true);
    let model = Model::new_from_app(app, main_window_id, config_window_id);
    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // if app.elapsed_frames() % FPS != 0 {
    //     return;
    // }
    // model.step_circles(app.duration.since_start);
    model.growths = step_growths(&model, app);
    // model.lines = move_lines(&model);

    model.growths = change_points(&model, &app);

    model.vary_amount = cycle_value_over_time(
        app.duration.since_start,
        Duration::from_secs(12),
        1.0,
        model.config.get_with_default("max_vary_amount", 1.5),
    )
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::Return {
        *model = Model::new_from_app(app, model.main_window_id, model.config_window_id);
    }

    if key == Key::Left {
        model.config.prev_key()
    }

    if key == Key::Right {
        model.config.next_key()
    }

    if key == Key::Up {
        model.config.change_value(0.1)
    }

    if key == Key::Down {
        model.config.change_value(-0.1)
    }

    if key == Key::PageUp {
        model.config.change_value(1.0)
    }

    if key == Key::PageDown {
        model.config.change_value(-1.0)
    }
}

fn main_view(app: &App, model: &Model, frame: Frame) {
    // if app.elapsed_frames() % FPS != 0 {
    //     return;
    // }
    // let draw = app.draw().xy(model.starting_point);
    let draw = app.draw();
    draw.background().color(BLACK);

    for growth in &model.growths {
        growth.draw(&draw, model.vary_amount)
    }

    // let fps = app.fps();
    // let num_growths = model.growths.len();
    // let finished = model
    //     .growths
    //     .iter()
    //     .enumerate()
    //     .map(|(i, g)| {
    //         let finished = g.is_finished();
    //         format!("{i}: {finished}")
    //     })
    //     .collect::<Vec<_>>()
    //     .join(", ");
    // draw.xy(app.window_rect().bottom_left() + vec2(20.0, 20.0))
    //     .text(&format!("{fps}"))
    //     .color(WHEAT);
    draw.finish_remaining_drawings();
    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    // model.frame_capture.capture_main_window_frame(app);

    // if app.duration.since_start >= Duration::from_secs(10) {
    //     app.quit()
    // }
}

fn step_growths(model: &Model, app: &App) -> Vec<Growth> {
    let mut growths = model.growths.clone();

    for g in &mut growths {
        g.step_growth(app, &model.config)
    }

    growths
}

fn change_points(model: &Model, app: &App) -> Vec<Growth> {
    // let mut starting_points = model.starting_points.clone();
    // let mut lines = model.lines.clone();

    // remove any finished growths
    // let mut growths: Vec<Growth> = model
    //     .growths
    //     .clone()
    //     .into_iter()
    //     .filter(|growth| !growth.is_finished())
    //     .collect();

    // // add a new growth for every removed one
    // let num_removed_growths = NUM_GROWTHS as usize - growths.len();
    // if num_removed_growths != 0 {
    //     let window_rect = app.window_rect();
    //     let mut rng = thread_rng();
    //     let centre_points: Vec<Vec2> = growths.iter().map(|g| g.centre).collect();

    //     let mut new_growths = (0..num_removed_growths)
    //         .map(|_| {
    //             let centre = vec2(
    //                 rng.gen_range(window_rect.x.start - 50.0 ..window_rect.x.end + 50.0),
    //                 rng.gen_range(window_rect.y.start - 50.0..window_rect.y.end - 50.0),
    //             );

    //             Growth::new(
    //                 centre,
    //                 &centre_points,
    //                 rgb_u32(COLOURS[rng.gen_range(0..COLOURS.len() - 1)]).into(),
    //             )
    //         })
    //         .collect();

    //     growths.append(&mut new_growths);
    // }

    // finished if 2/3 are finished
    let is_finished = model
        .growths
        .iter()
        .fold(0, |acc, g| acc + if g.is_finished() { 1 } else { 0 })
        >= model.growths.len() * 2 / 3;

    if is_finished {
        return create_new_growths(app, model.main_window_id, NUM_GROWTHS);
    } else {
        return model.growths.clone();
    }
}

fn create_new_growths(app: &App, window_id: WindowId, num_growths: u64) -> Vec<Growth> {
    let mut rng = thread_rng();
    let window_rect = app.window(window_id).unwrap().rect();
    let centre_points: Vec<Point2> = (0..num_growths)
        .map(|_| {
            vec2(
                rng.gen_range(window_rect.x.start - 50.0..window_rect.x.end + 50.0),
                rng.gen_range(window_rect.y.start - 50.0..window_rect.y.end + 50.0),
            )
        })
        .collect();

    let growths: Vec<Growth> = (&centre_points)
        .iter()
        .map(|p_c| {
            Growth::new(
                *p_c,
                &centre_points,
                rgb_u32(rand_from_slice(&COLOURS)).into(),
            )
        })
        .collect();
    growths
}

fn config_view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHEAT);

    let width = 500.0;
    let step = width / model.config.keys.len() as f32;

    for (i, key) in model.config.keys.iter().enumerate() {
        let value = model.config.values.get(key).unwrap();
        let x = -(width / 2.0) + step * i as f32;
        let colour = if i == model.config.selected_value {
            BLUEVIOLET
        } else {
            BLACK
        };
        draw.x_y(x, -50.0).text(&key).font_size(16).color(colour);
        draw.x_y(x, 0.0)
            .text(&format!("{value}"))
            .font_size(16)
            .color(colour);
    }

    draw.to_frame(app, &frame).unwrap();
}
