use crate::utils::random_int;
use crate::vec3::{dot, unit_vector, Point3, Vec3};

pub const POINT_COUNT: usize = 256;

pub struct Perlin {
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
    randfloat: [Vec3; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [Vec3::empty(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            randfloat[i] = unit_vector(Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            perm_x: perm_x,
            perm_y: perm_y,
            perm_z: perm_z,
            randfloat: randfloat,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::empty(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut points = [0_i32; POINT_COUNT];
        for i in 0..POINT_COUNT {
            points[i] = i as i32;
        }

        Perlin::permute(&mut points, POINT_COUNT);
        points
    }

    fn permute(points: &mut [i32; 256], n: usize) {
        for i in (0..n-1).rev() {
            let target = random_int(0, i as i32);
            let tmp = points[i];
            points[i] = target;
            points[target as usize] = tmp;
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(c[i][j][k], weight);
                }
            }
        }

        accum
    }
}
