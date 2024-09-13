use std::sync::Arc;

use env_logger::Env;

use rayonetta::bvh::BVH;
use rayonetta::camera::Camera;
use rayonetta::color::Color;
use rayonetta::hittable_list::HittableList;
use rayonetta::material::{Dielectric, Lambertian, Metal};
use rayonetta::plane::Plane;
use rayonetta::sphere::Sphere;
use rayonetta::texture::CheckerTexture;
use rayonetta::vec3::{Point3, Vec3};

fn main() {
    // Logging functions
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    // World
    let mut world = HittableList::new();
    
    // Bigger balls
    let diffuse = Arc::new(Metal::new(Color::new(0.9, 0.4, 0.1), 0.10));
    world.add(Arc::new(Sphere::new(Point3::new(-0.25, 0.00, -1.0), 0.25, diffuse.clone())));

    let glass = Arc::new(Dielectric::new(1.5));
    let bubble = Arc::new(Dielectric::new(1.0/1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.25, 0.00, -1.2), 0.25, glass.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(0.75, 0.00, -1.2), 0.22, bubble.clone())));

    world = HittableList::from_object(Arc::new(BVH::from_hittable(world)));

    // Ground Plane
    let checker_texture = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let material_ground = Arc::new(Lambertian::from_texture(checker_texture));
    world.add(Arc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, -0.25, 0.0),
        material_ground
    )));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 30.0;
    let lookfrom = Point3::new(0.0, 0.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 1.0;

    let mut cam = Camera::new();

    cam.aspect_ratio = aspect_ratio;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;

    cam.vfov = vfov;
    cam.lookfrom = lookfrom;
    cam.lookat = lookat;
    cam.vup = vup;

    // // Focus blur
    cam.defocus_angle = defocus_angle;
    cam.focus_dist = focus_dist;

    // Render
    cam.render(&world);

}
