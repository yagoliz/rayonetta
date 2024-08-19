use log::info;
use rayon::prelude::*;

use crate::{
    color::{write_color, Color},
    hittable::HitRecord,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    utils::{degrees_to_radians, random_uniform, INFINITY},
    vec3::{cross, random_unit_disk, unit_vector, Point3, Vec3},
};
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    
    pub vfov: f64, // Field of View Angle
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    initialized: bool,
    image_height: i32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel_00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Camera basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // Focus blur
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 0.0,
            image_width: 0,
            samples_per_pixel: 5,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::empty(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_sample_scale: 1.0/5.0,
            center: Point3::empty(),
            pixel_00_loc: Point3::empty(),
            pixel_delta_u: Point3::empty(),
            pixel_delta_v: Point3::empty(),
            u: Vec3::empty(),
            v: Vec3::empty(),
            w: Vec3::empty(),
            defocus_disk_u: Vec3::empty(),
            defocus_disk_v: Vec3::empty(),
            initialized: false,
        }
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate u, v, w camera basis vectors
        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // Viewport managing
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Focus disk basis vectors
        let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

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
    
        let origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_uniform() - 0.5, random_uniform() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    pub fn render(mut self, world: &HittableList) {
        if !self.initialized {
            self.initialize();
        }

        // Render
        println!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);

            // Create a Vec of Vecs (dynamic 2D array)
        let mut pixels: Vec<Vec<Point3>> = Vec::with_capacity(self.image_height as usize);

        // Initialize each row
        for _ in 0..self.image_height {
            let row = vec![Default::default(); self.image_width as usize];
            pixels.push(row);
        }

        // Parallelize the outer loop (over scanlines/rows)
        pixels.par_iter_mut().enumerate().for_each(|(j, row)| { 
            info!("Scanlines remaining: {}", self.image_height - j as i32);
            for i in 0..self.image_width {
                let mut pixel_color = Color::empty();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j as i32);
                    pixel_color = pixel_color + self.ray_color(&r, world, self.max_depth);
                }
                row[i as usize] = pixel_color; // Assuming you have a conversion method
            }
        });
        
        // Writing to stdout
        for row in &pixels {
            for pixel in row {
                write_color(&(*pixel*self.pixel_sample_scale));
            }
        }

        info!("Done!");
    }
}
