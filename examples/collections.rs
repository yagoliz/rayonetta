use std::sync::Arc;

use clap::Parser;
use env_logger::Env;

use rayonetta::bvh::BVH;
use rayonetta::camera::Camera;
use rayonetta::color::Color;
use rayonetta::hittable_list::HittableList;
use rayonetta::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use rayonetta::planar::{create_box, Quadrilateral};
use rayonetta::plane::Plane;
use rayonetta::sphere::Sphere;
use rayonetta::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use rayonetta::transformations::{RotateY, Translate};
use rayonetta::utils::{random_interval, random_uniform};
use rayonetta::vec3::{Point3, Vec3};

/// This program selects the raytracing demo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Demo number to show
    #[arg(short, long)]
    demo_number: usize,
}

fn bouncing_spheres() {
    // World
    let mut world = HittableList::new();

    // Smaller balls
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_uniform();
            let center = Point3::new(
                a as f64 + 0.9 * random_uniform(),
                0.2,
                b as f64 + 0.9 * random_uniform(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random_interval(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_dynamic(
                        center,
                        center2,
                        0.2,
                        sphere_material.clone(),
                    )));
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
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

    // We now change the world to a BVH
    world = HittableList::from_object(Arc::new(BVH::from_hittable(world)));

    // Ground Plane
    let checker_texture = Arc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Arc::new(Lambertian::from_texture(checker_texture));
    world.add(Arc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::empty(),
        material_ground,
    )));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let background = Color::new(0.70, 0.80, 1.0);

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
    cam.background = background;

    // Render
    cam.render(&world);
}

fn checkered_spheres() {
    // World
    let mut world = HittableList::new();

    // Bigger balls
    let checker_texture = Arc::new(CheckerTexture::from_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let material = Arc::new(Lambertian::from_texture(checker_texture));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material.clone(),
    )));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let background = Color::new(0.70, 0.80, 1.0);

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
    cam.background = background;

    // Render
    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::from_image("assets/mars.webp"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::empty(), 2.0, earth_surface.clone()));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(12.0, 3.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let background = Color::new(0.70, 0.80, 1.0);

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
    cam.background = background;

    // Render
    let mut world = HittableList::new();
    world.add(globe);
    cam.render(&world);
}

fn perlin_spheres() {
    // Creating the precious world
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));
    let sphere = Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    ));
    let floor = Arc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::empty(),
        perlin_material.clone(),
    ));

    world.add(sphere);
    world.add(floor);

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(12.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 1.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let background = Color::new(0.70, 0.80, 1.0);

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
    cam.background = background;

    // Render
    cam.render(&world);
}

fn quadrilaterals() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.8, 0.2, 0.1)));
    let blue = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.8)));
    let green = Arc::new(Lambertian::new(Color::new(0.1, 0.8, 0.2)));
    let orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let cyan = Arc::new(Lambertian::new(Color::new(0.0, 1.0, 1.0)));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        red.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        blue.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        green.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(-2.0, -3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        cyan.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        orange.clone(),
    )));

    // Camera settings
    let aspect_ratio = 1.0;
    let image_width = 800;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 80.0;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let background = Color::new(0.70, 0.80, 1.0);

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
    cam.background = background;

    // Render
    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));
    let sphere = Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    ));
    let floor = Arc::new(Plane::new(
        Vec3::new(0.0, 1.0, 0.0),
        Point3::empty(),
        perlin_material.clone(),
    ));

    world.add(sphere);
    world.add(floor);

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    let quadlight = Arc::new(Quadrilateral::new(
        Point3::new(3.0, 1.0, -3.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    ));

    world.add(quadlight);
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;

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

    // Render
    cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    world.add(Arc::new(Quadrilateral::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1 = create_box(Point3::empty(), Point3::new(165.0, 330.0, 165.0), white.clone());
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2 = create_box(Point3::empty(), Point3::new(165.0, 165.0, 165.0), white.clone());
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));    

    world.add(box2);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::empty();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    // Logging functions
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let args = Args::parse();
    match args.demo_number {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quadrilaterals(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => panic!("Non-existing demo number!"),
    }
}
