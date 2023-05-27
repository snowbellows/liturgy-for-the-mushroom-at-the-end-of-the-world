use crate::helpers::{cycle_value_over_time, FrameCapture};
use nannou::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    iter::{repeat, zip},
    time::Duration,
};

pub const WINDOW_SIZE: f32 = 900.0;
pub const CYCLE_SECONDS: f32 = 10.0;
pub const NUM_ITERS: u64 = 5;
pub const NUM_SPOKES: u64 = 20;
pub const BRANCH_LENGTH: f32 = 300.0;
pub const TRANSPARENT_BLANCHED_ALMOND: (f32, f32, f32, f32) = (255.0, 235.0, 205.0, 0.000001);
pub const STEP_LENGTH: f32 = 10.0;

struct Model {
    rand_seed: u64,
    rng: ChaCha8Rng,
    // joins: Vec<(Point2, Point2)>,
    // starting_point: Point2,
    spoke_angles: Vec<f32>,
    starting_points: Vec<Point2>,
    lines: HashMap<String, Vec<Point2>>,
    frame_capture: FrameCapture,
    window_id: WindowId,
}

// fn draw_branch(
//     current_point: Point2,
//     length: f32,
//     acc: Vec<(Point2, Point2)>,
// ) -> Vec<(Point2, Point2)> {
//     let new_length = length / 2.0;

//     if new_length <= 2.0 {
//         return acc;
//     }
//     let next_point_positive = current_point + vec2(length, length);
//     let next_point_negative = current_point + vec2(-length, -length);

//     draw_branch(
//         next_point_negative,
//         new_length,
//         [
//             acc,
//             vec![
//                 (current_point, next_point_positive),
//                 (current_point, next_point_negative),
//             ],
//         ]
//         .concat(),
//     )
// }

impl Model {
    fn new_from_app(app: &App, window_id: WindowId) -> Self {
        let rand_seed: u64 = random();

        println!("{rand_seed}");

        let mut rng = ChaCha8Rng::seed_from_u64(rand_seed);
        // let mut rng = ChaCha8Rng::seed_from_u64(3743695093727203978);

        let point_range = -WINDOW_SIZE / 6.0..WINDOW_SIZE / 6.0;
        let window_rect = app.window_rect();

        let starting_point = vec2(
            rng.gen_range(point_range.clone()),
            rng.gen_range(point_range.clone()),
        );
        let starting_points = (0..NUM_SPOKES)
            .map(|_| {
                vec2(
                    rng.gen_range(window_rect.x.start..window_rect.x.end),
                    rng.gen_range(window_rect.y.start..window_rect.y.end),
                )
            })
            .collect();
        let start_angle = rng.gen_range(0.0..360.0f32);
        let spoke_angles = (0..NUM_SPOKES)
            .fold(vec![start_angle], |acc, _| {
                let val = acc.last().unwrap()
                    + rng.gen_range((360.0 / NUM_SPOKES as f32 * 0.4)..(360.0 / NUM_SPOKES as f32));
                [acc, vec![val]].concat()
            })
            .iter()
            .map(|theta| deg_to_rad(*theta))
            .collect();
        // let centre_point = vec2(0.1, 0.1);
        // let direction = starting_point.angle_between(centre_point);

        // let next_point = starting_point.rotate(direction) + vec2(100.0, 0.0);
        // let joins = draw_branch(next_point, 50.0, vec![(starting_point, next_point)]);
        // println!("{joins:?}");
        Model {
            rand_seed,
            rng,
            // starting_point,
            lines: HashMap::new(),
            starting_points,
            spoke_angles,
            // joins,
            frame_capture: FrameCapture::new_from_app_with_seed(app, &rand_seed.to_string()),
            window_id,
        }
    }
}

pub fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
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
    // model.step_circles(app.duration.since_start);
    let stepped_lines = step_lines(&model);
    model.lines = stepped_lines;

    let moved_lines = move_lines(&model);
    model.lines = moved_lines;
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    if key == Key::Return {
        *model = Model::new_from_app(app, model.window_id);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // let draw = app.draw().xy(model.starting_point);
    let draw = app.draw();
    draw.background().color(BLACK);

    // for theta in &model.spoke_angles {
    //     let draw = draw.rotate(*theta);

    //     draw_branch(&draw, BRANCH_LENGTH, deg_to_rad(45.0), NUM_ITERS);
    // }
    // draw_join(&draw);

    // test_vector_math(&draw, 3);

    // for p in &model.starting_points {
    //     draw_cirlce(&draw, p);
    // }

    for (_, line) in &model.lines {
        draw_polyline(&draw, line);
    }

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

fn step_lines(model: &Model) -> HashMap<String, Vec<Point2>> {
    let mut lines = model.lines.clone();
    let mut rng = model.rng.clone();
    for p_start in &model.starting_points {
        for p_end in &model.starting_points {
            // println!("p_start: {p_start}");

            // see if we can find the closest point
            // let maybe_closest_point = model.starting_points.iter().reduce(|acc, p1| {
            //     if p_start != p1 && p_start.distance(*p1) < p_start.distance(*acc) {
            //         p1
            //     } else {
            //         acc
            //     }
            // });

            // select a random point
            // let random_point = model
            //     .starting_points
            //     .get(model.rng.gen_range(0..model.starting_points.len()));

            // if let Some(p_close) = random_point {
            // println!("p_close: {p_close}");

            // once we have the closest point we grab the corresponding line between the two points or create a new one
            let line = lines
                .entry(vec2_hash(*p_start, *p_end))
                .or_insert(vec![*p_start]);

            // println!("line: {line:?}");

            // take the last point from the line
            if let Some(p_last) = line.last() {
                // if the last point in the line is the same as the "end point" we're done
                if p_last != p_end {
                    // println!("p_last: {p_last}");

                    // if not check if we're within x pixels of the "end point" and return that
                    let length = 2.0;
                    let d_left = p_last.distance(*p_end);
                    let p_next = if d_left <= length {
                        *p_end
                    } else {
                        // println!("percent_traversed: {percent_traversed}");

                        // randomise where the end point is for fun, curly lines
                        let rand_amount = 3.0;
                        let p_random = vec2(
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                        );

                        // we lerp towards the "end point"
                        // let percent_traversed =
                        //     p_start.distance(*p_last) / p_start.distance(*p_end);

                        // (*p_last + p_random).lerp(
                        //     *p_end + p_random,
                        //     if percent_traversed > 0.0 {
                        //         percent_traversed / 50.0
                        //     } else {
                        //         0.001
                        //     },
                        // )

                        // we move by distance towards the "end point"
                        let p_last = (*p_last + p_random);
                        let v_to_end = (p_last - *p_end).normalize();

                        (p_last) - v_to_end / 100.0
                    };

                    // println!("p_next: {p_next}");

                    line.push(p_next);
                }
            } else {
                println!("no p_last");
            }
            // } else {
            //     println!("no p_close");
            // }
        }
    }

    lines
}

fn move_lines(model: &Model) -> HashMap<String, Vec<Point2>> {
    let mut rng = model.rng.clone();

    // for (_, mut line) in &lines {
    //     for mut p in line {
    //         let rand_amount = 5;
    //         let x = rng.gen_range(-rand_amount..rand_amount) as f32 / 10.0;
    //         let y = rng.gen_range(-rand_amount..rand_amount) as f32 / 10.0;

    //         p = &(*p + vec2(x, y));
    //     }
    // }

    model
        .lines
        .clone()
        .into_iter()
        .map(|(hash, line)| {
            (
                hash,
                line.iter()
                    .map(|p| {
                        let rand_amount = 5;
                        let x = rng.gen_range(-rand_amount..=rand_amount) as f32 / 10.0;
                        let y = rng.gen_range(-rand_amount..=rand_amount) as f32 / 10.0;

                        *p + vec2(x, y)
                    })
                    .collect(),
            )
        })
        .collect()
}

fn draw_polyline(draw: &Draw, line: &Vec<Point2>) {
    // for p in line {
    //     draw_cirlce(draw, p)
    // }

    draw.polyline()
        .weight(3.0)
        .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND))
        .points(line.clone());
}

fn draw_line(draw: &Draw, start: Point2, end: Point2) {
    draw.line()
        .start(start)
        .end(end)
        .weight(2.0)
        .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND));
}

fn draw_cirlce(draw: &Draw, p: &Point2) {
    draw.ellipse()
        .xy(*p)
        .stroke_color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND))
        .radius(10.0)
        .stroke_weight(4.0)
        .no_fill();
}

fn test_vector_math(draw: &Draw, iterations: u64) {
    let p1 = pt2(0.0, 0.0);
    let p2 = pt2(0.0, 200.0);
    let p3 = pt2(-50.0, 250.0);
    let p4 = pt2(50.0, 250.0);

    let draw_cirlce = |p: Point2| {
        draw.ellipse()
            .xy(p)
            .stroke_color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND))
            .radius(10.0)
            .stroke_weight(4.0)
            .no_fill();
    };

    draw_cirlce(p1);
    draw_cirlce(p2);
    draw_cirlce(p3);
    draw_cirlce(p4);

    draw_line(draw, p1, p2);
    draw_line(draw, p2, p3);
    draw_line(draw, p2, p4);

    let lerp_10_p1_p2 = p1.lerp(p2, 0.5);

    draw_cirlce(lerp_10_p1_p2);

    if iterations > 0 {
        let draw1 = draw.xy(p3).rotate(deg_to_rad(45.0));
        test_vector_math(&draw1, iterations - 1);

        let draw2 = draw.xy(p4).rotate(deg_to_rad(-45.0));
        test_vector_math(&draw2, iterations - 1);
    }
}

fn draw_join(draw: &Draw) {
    let first_branch_start = pt2(0.0, 0.0);
    let first_branch_end = pt2(0.0, 200.0);
    let second_branch_end = pt2(-100.0, 300.0);
    let third_branch_end = pt2(100.0, 300.0);
    let points = [
        pt2(0.0, 0.0),
        pt2(0.0, 200.0),
        pt2(-100.0, 300.0),
        pt2(100.0, 300.0),
    ];

    let draw_line = |start: Point2, end: Point2| {
        draw.line()
            .start(start)
            .end(end)
            .weight(2.0)
            .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND));
    };

    draw_line(first_branch_start, first_branch_end);
    draw_line(first_branch_end, second_branch_end);
    draw_line(first_branch_end, third_branch_end);

    fn step_line(start: Point2, end: Point2) -> Vec<Point2> {
        let step = 5;
        let xs: Box<dyn ExactSizeIterator<Item = i32>> = if start.x < end.x {
            Box::new((start.x as i32..end.x as i32).step_by(step))
        } else {
            Box::new((end.x as i32..start.x as i32).step_by(step).rev())
        };
        let ys: Box<dyn ExactSizeIterator<Item = i32>> = if start.y < end.y {
            Box::new((start.y as i32..end.y as i32).step_by(step))
        } else {
            Box::new((end.y as i32..start.y as i32).step_by(step).rev())
        };

        let zipped: Box<dyn Iterator<Item = (i32, i32)>> = match (xs.len(), ys.len()) {
            (0, 0) => Box::new(zip(repeat(0), repeat(0))),
            (0, _) => Box::new(zip(repeat(0), ys)),
            (_, 0) => Box::new(zip(xs, repeat(0))),
            (_, _) => Box::new(zip(xs, ys)),
        };

        zipped.map(|(x, y)| pt2(x as f32, y as f32)).collect()
    }
    let steps_first = step_line(first_branch_start, first_branch_end);
    let steps_second = step_line(first_branch_end, second_branch_end);
    let steps_third = step_line(first_branch_end, third_branch_end);
    println!("steps_first (len: {}): {steps_first:?},", steps_first.len());
    println!(
        "steps_second (len: {}): {steps_second:?},",
        steps_second.len()
    );
    println!("steps_third (len: {}): {steps_third:?},", steps_third.len());

    for (start, end) in zip(steps_first.iter().rev(), steps_second.iter().rev()) {
        draw_line(*start, *end);
    }

    for (start, end) in zip(steps_first.iter().rev(), steps_third.iter().rev()) {
        draw_line(*start, *end);
    }

    for (start, end) in zip(&steps_second, steps_third.iter().rev()) {
        draw_line(*start, *end);
    }
}

fn draw_branch(draw: &Draw, length: f32, theta: f32, iterations: u64) {
    let end_point = pt2(0.0, length);

    let sw = map_range(length, 2.0, BRANCH_LENGTH, 1.0, 6.0);

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(end_point)
        .weight(sw)
        .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND));

    let feather_length = map_range(BRANCH_LENGTH - 100.0, 0.0, BRANCH_LENGTH, 0.0, length);

    let feathers = ((feather_length as u32)..(length as u32)).step_by(3);

    for (i, step) in feathers.clone().enumerate() {
        let draw_feather = |draw: Draw| {
            // let feather_length = map_range(length, 2.0, BRANCH_LENGTH, 3.0, 30.0);
            let inner_feather_length = length - feather_length;
            draw.line()
                .start(pt2(0.0, 0.0))
                .end(pt2(
                    0.0,
                    inner_feather_length * (i as f32 / feathers.len() as f32),
                ))
                .weight(sw / 5.0)
                .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND));
        };

        let feather_theta = deg_to_rad(45.0);
        let feather_draw = draw.xy(pt2(0.0, step as f32)).rotate(feather_theta);
        draw_feather(feather_draw);
        let feather_draw = draw.xy(pt2(0.0, step as f32)).rotate(-feather_theta);
        draw_feather(feather_draw);
    }

    let draw = draw.xy(end_point);

    let new_length = length * 0.66;

    if new_length > 2.0 && iterations > 0 {
        let draw2 = draw.rotate(theta);
        draw_branch(&draw2, new_length, theta, iterations - 1);
        let draw3 = draw.rotate(-theta);
        draw_branch(&draw3, new_length, theta, iterations - 1);
    }
}
