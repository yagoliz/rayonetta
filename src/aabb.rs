use std::ops;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut bbox = AABB { x: x, y: y, z: z };
        bbox.pad_to_minimum();
        bbox
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() < b.x() { Interval::new(a.x(), b.x()) } else { Interval::new(b.x(), a.x()) };
        let y = if a.y() < b.y() { Interval::new(a.y(), b.y()) } else { Interval::new(b.y(), a.y()) };
        let z = if a.z() < b.z() { Interval::new(a.z(), b.z()) } else { Interval::new(b.z(), a.z()) };

        AABB::new(x, y, z)
    }

    pub fn from_bboxes(box0: &AABB, box1: &AABB) -> Self {
        AABB {
            x: Interval::from_interval(&box0.x, &box1.x),
            y: Interval::from_interval(&box0.y, &box1.y),
            z: Interval::from_interval(&box0.z, &box1.z)
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis for interval selected")
        }
    }

    pub fn hit(&self, r: &Ray, ray: Interval) -> bool {
        let mut ray_t = ray;
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        // Returns the index of the longest axis of the bbox
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    fn pad_to_minimum(&mut self) {

        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }

        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }

        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub const EMPTY: AABB = AABB {x: Interval::EMPTY, y: Interval::EMPTY, z: Interval::EMPTY};
    pub const UNIVERSE: AABB = AABB {x: Interval::UNIVERSE, y: Interval::UNIVERSE, z: Interval::UNIVERSE};
}

// Sumation
impl ops::Add<Vec3> for AABB {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> AABB {
        AABB::new(self.x + _rhs.x(), self.y + _rhs.y(), self.z + _rhs.z())
    }
}

impl ops::Add<AABB> for Vec3 {
    type Output = AABB;
    fn add(self, rhs: AABB) -> Self::Output {
        rhs + self
    }
}