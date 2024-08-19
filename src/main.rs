use std::rc::Rc;

use env_logger::Env;

use rayonetta::camera::Camera;
use rayonetta::color::Color;
use rayonetta::hittable_list::HittableList;
use rayonetta::material::{Dielectric, Lambertian, Metal};
use rayonetta::plane::Plane;
use rayonetta::sphere::Sphere;
use rayonetta::vec3::{Point3, Vec3};

fn main() {
    // Logging functions
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut cam = Camera::new();

    cam.aspect_ratio = aspect_ratio;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;

    // World
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.4)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00/1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.00));

    let mut world = HittableList::new();
    
    // Ground Plane
    world.add(Rc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, -0.5, 0.0),
        material_ground
    )));

    // Spheres around
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.4, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(-1.4, 0.0, -2.4), 0.4, material_bubble.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(1.3, 0.15, -3.0), 0.65, material_right.clone())));

    // Render
    cam.render(&world);

}
