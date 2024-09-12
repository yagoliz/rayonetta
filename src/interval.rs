use crate::utils::INFINITY;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval { min: -INFINITY, max: INFINITY }
    }

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min: min, max: max }
    }

    pub fn from_interval(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
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
