use nannou::prelude::*;
use rand::prelude::*;
use std::ops::{Add, Mul, Sub};

use crate::helpers::rand_normalised_vec;

#[derive(Clone, Debug)]
pub struct Growth {
    pub centre: Point2,
    pub lines: Vec<Line>,
    pub colour: Srgba,
}

impl Growth {
    pub fn new(centre: Point2, other_growths: &[Point2], colour: Srgb<u8>) -> Self {
        let colour: Srgb<f32> = Srgb::from_format(colour);
        let (r, g, b) = colour.into_components();
        Growth {
            centre,
            lines: other_growths
                .iter()
                .map(|p_c| Line::new(centre, *p_c))
                .collect(),
            colour: Srgba::new(r, g, b, 0.1),
        }
    }

    pub fn is_finished(&self) -> bool {
        self.lines
            .iter()
            .fold(true, |acc, line| acc && line.finished)
    }

    pub fn step_growth(&mut self) {
        for l in &mut self.lines {
            l.step_line()
        }
    }

    pub fn draw(&self, draw: &Draw, amount: f32) {
        for line in &self.lines {
            line.draw(draw, &self.colour, amount)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Point(Point2, Vec2);

impl Point {
    pub fn variation(&self) -> Vec2 {
        self.1
    }

    pub fn new(point2: Point2) -> Self {
        Self(point2, rand_normalised_vec())
    }

    pub fn normalize(&self) -> Self {
        Self(self.0.normalize(), self.1)
    }

    pub fn vary_by_amount(&self, amount: f32) -> Point2 {
        self.0 + (self.1 * amount)
    }
}

impl From<Point> for Point2 {
    fn from(value: Point) -> Self {
        value.0
    }
}

impl PartialEq<Point> for Point2 {
    fn eq(&self, other: &Point) -> bool {
        *self == other.0
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, (self.1 + rhs.1).normalize())
    }
}

impl Add<Vec2> for Point {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs, self.1)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, (self.1 - rhs.1).normalize())
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(self.0 - rhs, self.1)
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, (self.1 * rhs.1).normalize())
    }
}

impl Mul<Vec2> for Point {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self(self.0 * rhs, self.1)
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1)
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point2,
    pub end: Point2,
    pub points: Vec<Point>,
    pub finished: bool,
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Line {
            start,
            end,
            points: vec![Point(start, vec2(0.0, 0.0))],
            finished: false,
        }
    }

    pub fn step_line(&mut self) {
        if let Some(p_last) = self.points.last() {
            if !self.finished {
                if self.end == *p_last {
                    self.finished = true;
                } else {
                    // if not check if we're within x pixels of the "end point" and return that
                    let length = 2.0;
                    let d_left = p_last.0.distance(self.end);
                    let p_next = if d_left <= length {
                        Point(self.end, vec2(0.0, 0.0))
                    } else {
                        // randomise where the end point is for fun, curly lines
                        let rand_amount = 100;
                        let rand_factor = 2.0;
                        let mut rng = thread_rng();

                        let p_random = vec2(
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                            rng.gen_range(-rand_amount..=rand_amount) as f32,
                        )
                        .normalize()
                            * rand_factor;

                        // get the vector towards the "end point"
                        let v_to_end = (*p_last - self.end).normalize();

                        // move towards the end point and add random for fun
                        Point::new((*p_last - (v_to_end * length + p_random)).into())
                    };

                    self.points.push(p_next);
                }
            }
        }
    }

    pub fn draw(&self, draw: &Draw, colour: &Srgba, amount: f32) {
        let points: Vec<Point2> = self
            .points
            .iter()
            .map(|p| p.vary_by_amount(amount))
            // .map(|p| (*p + (rand_normalised_vec() * 2.0)).into())
            .collect();

        draw.polyline()
            .weight(3.0)
            .color(colour.clone())
            .points(points);
    }
}
