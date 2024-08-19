use crate::{
    color::{write_color, Color},
    hittable::HitRecord,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    utils::{random_uniform, INFINITY},
    vec3::{unit_vector, Point3, Vec3},
};

use log::info;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    initialized: bool,
    image_height: i32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            samples_per_pixel: 5,
            max_depth: 10,
            image_height: 0,
            pixel_sample_scale: 1.0/5.0,
            center: Point3::empty(),
            pixel_00_loc: Point3::empty(),
            pixel_delta_u: Point3::empty(),
            pixel_delta_v: Point3::empty(),
            initialized: false,
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        self.initialized = true;
    }

    fn ray_color(&self, r: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::empty();
        }

        let mut rec = HitRecord::new();
        if world.hit(r, &mut Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::empty(), Vec3::empty());
            let mut attenuation = Color::empty();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, world, depth-1);
            } else {
                return Color::empty();
            }
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 0.9)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);
    
        let origin = self.center;
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_uniform() - 0.5, random_uniform() - 0.5, 0.0)
    }

    pub fn render(mut self, world: &HittableList) {
        if !self.initialized {
            self.initialize();
        }

        // Render
        println!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, world, self.max_depth);

                }
                write_color(&(self.pixel_sample_scale*pixel_color));
            }
        }
        info!("Done!");
    }
}
