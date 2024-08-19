use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};

pub struct Plane {
    normal: Vec3,
    offset: f64,
    mat: Arc<dyn Material + Sync + Send>,
}

impl Plane {
    pub fn new(normal: Vec3, center: Point3, mat: Arc<dyn Material + Sync + Send>) -> Self {
        Plane { normal: normal, offset: -dot(normal, center), mat: mat}
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denominator = dot(self.normal, r.direction());
        if f64::abs(denominator) < 1e-6 {
            return false;
        }

        let numerator = dot(self.normal, r.origin()) + self.offset;
        let t = - numerator / denominator;

        if !ray_t.surrounds(t) {
            return false;
        }

        rec.t = t;
        rec.p = r.at(rec.t);
        rec.set_face_normal(r, &self.normal);
        rec.mat = self.mat.clone();
        
        return true;
    }
}
