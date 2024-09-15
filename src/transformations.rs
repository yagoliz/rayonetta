use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::{degrees_to_radians, INFINITY};
use crate::vec3::{Point3, Vec3};

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Translate { object: object, offset: offset, bbox: bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &mut crate::interval::Interval, rec: &mut crate::hittable::HitRecord) -> bool {
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx =  cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        RotateY { object: object, sin_theta: sin_theta, cos_theta: cos_theta, bbox: AABB::from_points(min, max) }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &mut crate::interval::Interval, rec: &mut crate::hittable::HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()), 
            r.origin().y(), 
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z())
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()), 
            r.direction().y(), 
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z())
        );

        let rotated_r = Ray::new_with_time(origin, direction, r.time());
        
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()), 
            rec.p.y(), 
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z())
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()), 
            rec.normal.y(), 
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z())
        );
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}