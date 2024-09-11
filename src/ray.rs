use crate::vec3::{Vec3, Point3};

#[derive(Clone, Copy, Debug)]
pub struct Ray{
    origin: Point3,
    direction: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin: origin, direction: direction, tm: 0.0 }
    }

    pub fn new_with_time(origin: Point3, direction: Vec3, tm: f64) -> Self {
        Ray { origin: origin, direction: direction , tm: tm}
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f64 {
        self.tm
    }
}