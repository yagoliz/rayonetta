use lazy_static::lazy_static;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

lazy_static!{
    static ref INTENSITY: Interval = Interval::new(0.0, 0.999);
}

fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        return f64::sqrt(linear);
    }

    0.0
}

pub fn write_color(pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Gamma correction
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translating to byte
    let rbyte = (256.0 * INTENSITY.clamp(r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(b)) as u8;

    println!("{rbyte} {gbyte} {bbyte}");
}