use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        Sphere {
            center: Ray::new(center, Vec3::empty()),
            radius: radius,
            mat: mat,
        }
    }

    pub fn new_dyn(
        center: Point3,
        center_next: Point3,
        radius: f64,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Sphere {
            center: Ray::new(center, center_next - center),
            radius: radius,
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 1e-8 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
