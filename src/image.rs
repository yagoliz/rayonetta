use image::{DynamicImage, GenericImageView, ImageReader};

use crate::color::Color;

fn clamp<T: Ord>(x: T, low: T, high: T) -> T {
    x.clamp(low, high)
}

pub struct RayonettaImage {
    width: u32,
    height: u32,
    image: DynamicImage,
}

impl RayonettaImage {
    pub fn from_file(filename: &str) -> Result<Self, String> {
        let bytes = ImageReader::open(filename)
            .map_err(|_| "Image was not found.".to_string())?;

        let image = bytes.decode()
            .map_err(|_| "Malformed Image.".to_string())?;

        Ok(RayonettaImage { width: image.width(), height: image.height(), image: image})
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let xw = clamp(x, 0, self.width);
        let yw = clamp(y, 0, self.height);

        let rgb = self.image.get_pixel(xw, yw).0;
        Color::new(rgb[0] as f64 / 255.0, rgb[1] as f64 / 255.0, rgb[2] as f64 / 255.0)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}