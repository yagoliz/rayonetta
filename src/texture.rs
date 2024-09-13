use std::sync::Arc;

use crate::color::Color;
use crate::image::RayonettaImage;
use crate::interval::Interval;
use crate::vec3::Point3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn from_color(color: Color) -> Self {
        SolidColor { albedo: color }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        SolidColor {
            albedo: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        return self.albedo;
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: even.clone(),
            odd: odd.clone(),
        }
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        CheckerTexture {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::from_color(c1)),
            odd: Arc::new(SolidColor::from_color(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = f64::floor(self.inv_scale * p.x()) as i32;
        let y_integer = f64::floor(self.inv_scale * p.y()) as i32;
        let z_integer = f64::floor(self.inv_scale * p.z()) as i32;

        match (x_integer + y_integer + z_integer) % 2 {
            0 => self.even.value(u, v, p),
            1 => self.odd.value(u, v, p),
            -1 => self.odd.value(u, v, p),
            _ => panic!("Modulo by 2 returned more than 1"),
        }
        
    }
}

pub struct ImageTexture {
    image: RayonettaImage,
}

impl ImageTexture {
    pub fn from_image(filename: &str) -> Self {
        let image = match RayonettaImage::from_file(filename) {
            Ok(im) => im,
            _ => panic!("Error opening file"),
        };

        ImageTexture { image: image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let um = Interval::new(0.0, 1.0).clamp(u);
        let vm = 1.0 - Interval::new(0.0, 1.0).clamp(v);

        let i = (um * self.image.width() as f64) as u32;
        let j = (vm * self.image.height() as f64) as u32;
        
        self.image.pixel_data(i, j)
    }
}