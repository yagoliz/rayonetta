use std::sync::Arc;

use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Isotropic, Material};
use crate::texture::Texture;
use crate::utils::{random_uniform, INFINITY};
use crate::vec3::Vec3;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_from_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        ConstantMedium {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(texture)),
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        ConstantMedium {
            boundary: boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &mut Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        let mut universe = Interval::UNIVERSE;
        if !self.boundary.hit(r, &mut universe, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, &mut Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_uniform().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
