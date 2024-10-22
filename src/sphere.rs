use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::PI;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f64,
    bbox: AABB,
    mat: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = AABB::from_points(center - rvec, center + rvec);
        Sphere {
            center: Ray::new(center, Vec3::empty()),
            radius: f64::max(radius,0.0),
            bbox: bbox,
            mat: mat,
        }
    }

    pub fn new_dynamic(
        center: Point3,
        center_next: Point3,
        radius: f64,
        mat: Arc<dyn Material + Sync + Send>,
    ) -> Self {

        let center_bounds = Ray::new(center, center_next - center);
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::from_points(center_bounds.at(0.0) - rvec, center_bounds.at(0.0) + rvec);
        let bbox2 = AABB::from_points(center_bounds.at(1.0) - rvec, center_bounds.at(1.0) + rvec);

        Sphere {
            center: center_bounds,
            radius: radius,
            bbox: AABB::from_bboxes(&bbox1, &bbox2),
            mat: mat,
        }
    }

    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
        if discriminant < 0.0 {
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
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
