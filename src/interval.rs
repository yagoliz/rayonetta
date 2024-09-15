use std::ops;

use crate::utils::INFINITY;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min: min, max: max }
    }

    pub fn from_interval(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta/2.0;
        Interval { min: self.min - padding, max: self.max + padding }
    }

    pub const EMPTY: Interval = Interval {min: INFINITY, max: -INFINITY};
    pub const UNIVERSE: Interval = Interval {min: -INFINITY, max: INFINITY};
}

impl ops::Add<f64> for Interval {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl ops::Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}