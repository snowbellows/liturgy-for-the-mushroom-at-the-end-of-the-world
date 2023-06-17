use nannou::prelude::*;
use std::{
    iter::{repeat, zip},
};

use super::{TRANSPARENT_BLANCHED_ALMOND, BRANCH_LENGTH};

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
    // let points = [
    //     pt2(0.0, 0.0),
    //     pt2(0.0, 200.0),
    //     pt2(-100.0, 300.0),
    //     pt2(100.0, 300.0),
    // ];

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

// fn move_lines(model: &Model) -> Vec<Growth> {
//     let mut rng = thread_rng();

//     model
//         .lines
//         .clone()
//         .into_iter()
//         .map(|(hash, line)| {
//             let mut line = line.clone();
//             line.points = line
//                 .points
//                 .iter()
//                 .map(|p| {
//                     let rand_amount = 100;
//                     let x = rng.gen_range(-rand_amount..=rand_amount) as f32;
//                     let y = rng.gen_range(-rand_amount..=rand_amount) as f32;

//                     *p + (vec2(x, y).normalize() / 5.0)
//                 })
//                 .collect();
//             (hash, line)
//         })
//         .collect()
// }
