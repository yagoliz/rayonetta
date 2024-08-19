use rand::{distributions::Uniform, prelude::Distribution};
use lazy_static::lazy_static;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;
pub const GOLDEN_RATIO: f64 = 1.6180339887498948482;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

lazy_static!{
    static ref UNIFORM_DIST: Uniform<f64> = Uniform::new(0.0, 1.0);
}

pub fn random_uniform() -> f64 {
    let mut rng = rand::thread_rng();
    UNIFORM_DIST.sample(&mut rng)
}

pub fn random_interval(min: f64, max: f64) -> f64 {
    min + (max - min) * random_uniform()
}