use std::sync::Arc;

use env_logger::Env;

use rayonetta::camera::Camera;
use rayonetta::color::Color;
use rayonetta::hittable_list::HittableList;
use rayonetta::material::{Dielectric, Lambertian, Metal};
use rayonetta::plane::Plane;
use rayonetta::sphere::Sphere;
use rayonetta::utils::{random_interval, random_uniform};
use rayonetta::vec3::{Point3, Vec3};

fn main() {
    // Logging functions
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    // World
    let mut world = HittableList::new();
    
    // Ground Plane
    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        material_ground
    )));

    // Smaller balls
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_uniform();
            let center = Point3::new(a as f64 + 0.9*random_uniform(), 0.2, b as f64 + 0.9*random_uniform());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_interval(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Bigger balls
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let mut cam = Camera::new();

    cam.aspect_ratio = aspect_ratio;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;

    cam.vfov = vfov;
    cam.lookfrom = lookfrom;
    cam.lookat = lookat;
    cam.vup = vup;

    // Focus blur
    cam.defocus_angle = defocus_angle;
    cam.focus_dist = focus_dist;

    // Render
    cam.render(&world);

}
