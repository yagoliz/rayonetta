use std::ops::{self, Index};

use crate::utils::{random_interval, random_uniform, PI};

#[derive(Clone, Copy, Debug, )]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub use Vec3 as Point3;

impl Vec3 {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    // Very simple functions to get the elements of the vector
    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    // Length utility functions
    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_uniform(), random_uniform(), random_uniform())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_interval(min, max), random_interval(min, max), random_interval(min, max))
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }
}

// Default Vec3
impl Default for Vec3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

// Indexing
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index for Vec3")
        }
    }
}

// Negation
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// Sumation
impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
    }
}

// Subtraction
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
    }
}

// Multiplication
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: f64) -> Self::Output {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        Vec3::new(self * _rhs.x, self * _rhs.y, self * _rhs.z)   
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self::Output {
        Vec3::new(self.x * _rhs.x, self.y * _rhs.y, self.z * _rhs.z)
    }
}

// Division
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Self::Output {
        self * (1.0/_rhs)
    }
}

// Vector functions
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x*v.x + u.y*v.y + u.z*v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x 
    )
}

pub fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

// Sampling randomly from a unit disk with Fibonacci sequences:
// https://observablehq.com/@meetamit/fibonacci-lattices
pub fn random_unit_disk() -> Vec3 {
    let sample_x = random_uniform();
    let sample_y = random_uniform();

    let theta = 2.0 * PI * sample_x;
    let r = f64::sqrt(sample_y);

    Vec3::new(r * f64::cos(theta), r * f64::sin(theta), 0.0)
}

// Sampling randomly from a sphere more efficiently with Fibonacci sampling
// https://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
pub fn random_unit_sphere() -> Vec3 {
    let sample_theta = random_uniform();
    let sample_phi = random_uniform();
    let theta = 2.0 * PI * sample_theta;
    let cos_theta = f64::cos(theta);
    let sin_theta = f64::sin(theta);
    
    let phi = f64::acos(1.0 - 2.0 * sample_phi);
    let cos_phi = f64::cos(phi);
    let sin_phi = f64::sin(phi);

    Vec3::new(cos_theta * sin_phi, sin_theta * sin_phi, cos_phi)
}

pub fn random_unit_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_sphere();

    match dot(on_unit_sphere, normal) > 0.0 {
        true => on_unit_sphere,
        false => -on_unit_sphere,
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-v, n), 1.0);
    let r_out_perp = etai_over_etat * (v + cos_theta * n);
    let r_out_para = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_para
}

// Testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let v = Vec3::random_range(-10.0, 10.0);
        let u = unit_vector(v);
        assert!(f64::abs(u.length()-1.0) < 1e-8);
    }

    #[test]
    fn test_reflect() {
        let v = Vec3::new(f64::cos(PI/4.0), -f64::sin(PI/4.0), 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);

        let r = reflect(v, n);
        let m = Vec3::new(f64::cos(PI/4.0), f64::sin(PI/4.0), 0.0); 
        assert!((m-r).length() < 1e-8);
    }

    #[test]
    fn test_refract() {
        let v = Vec3::new(f64::cos(PI/4.0), -f64::sin(PI/4.0), 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);

        let sin_theta2 = 1.0/1.33 * f64::sin(PI/4.0);

        let r = refract(v, n, 1.0/1.33);
        let m = Vec3::new(sin_theta2, -f64::sqrt(1.0-sin_theta2*sin_theta2), 0.0); 
        assert!((m-r).length() < 1e-6);    
    }
}