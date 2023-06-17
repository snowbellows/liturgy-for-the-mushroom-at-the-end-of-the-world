use nannou::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;

use super::TRANSPARENT_BLANCHED_ALMOND;

#[derive(Clone, Debug)]
pub struct Growth {
    pub centre: Point2,
    pub lines: HashMap<String, Line>,
}

impl Growth {
    pub fn new(centre: Point2) -> Self {
        Growth {
            centre,
            lines: HashMap::new(),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.lines
            .iter()
            .fold(true, |acc, (_, line)| acc && line.finished)
    }

    pub fn draw(&self, draw: &Draw) {
        for (_, line) in &self.lines {
            line.draw(draw)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point2,
    pub end: Point2,
    pub points: Vec<Point2>,
    pub finished: bool,
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

    pub fn step_line(&mut self) {

    }

    pub fn draw(&self, draw: &Draw) {
        // let mut rng = thread_rng();
        // let line: Vec<Point2> = self
        //     .points
        //     .iter()
        //     .map(|p| {
        //         let rand_amount = 100;
        //         let x = rng.gen_range(-rand_amount..=rand_amount) as f32;
        //         let y = rng.gen_range(-rand_amount..=rand_amount) as f32;

        //         *p + (vec2(x, y).normalize() * 2.0)
        //     })
        //     .collect();

        draw.polyline()
            .weight(3.0)
            .color(Rgba::from_components(TRANSPARENT_BLANCHED_ALMOND))
            .points(self.points.clone());
    }
}
