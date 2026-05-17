//! # Slate Image Loading & Decoding
//!
//! Support for PNG, JPEG, WebP, GIF, and other image formats.

use slate_ais::Rgba8;

pub mod decoder;
pub mod cache;
pub mod loader;

pub use decoder::{ImageDecoder, ImageFormat};
pub use cache::ImageCache;
pub use loader::ImageLoader;

/// A decoded image.
#[derive(Clone)]
pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA8
    pub format: ImageFormat,
}

impl DecodedImage {
    /// Create a new decoded image.
    pub fn new(width: u32, height: u32, pixels: Vec<u8>, format: ImageFormat) -> Self {
        assert_eq!(pixels.len(), (width * height * 4) as usize);
        Self {
            width,
            height,
            pixels,
            format,
        }
    }

    /// Get a pixel at (x, y).
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgba8 {
        if x >= self.width || y >= self.height {
            return Rgba8::TRANSPARENT;
        }

        let idx = ((y * self.width + x) * 4) as usize;
        Rgba8 {
            r: self.pixels[idx],
            g: self.pixels[idx + 1],
            b: self.pixels[idx + 2],
            a: self.pixels[idx + 3],
        }
    }

    /// Set a pixel at (x, y).
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba8) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = ((y * self.width + x) * 4) as usize;
        self.pixels[idx] = color.r;
        self.pixels[idx + 1] = color.g;
        self.pixels[idx + 2] = color.b;
        self.pixels[idx + 3] = color.a;
    }

    /// Resize image to new dimensions.
    pub fn resize(&self, new_width: u32, new_height: u32) -> Self {
        // Simple nearest-neighbor resize
        let mut new_pixels = vec![0u8; (new_width * new_height * 4) as usize];

        let x_ratio = self.width as f32 / new_width as f32;
        let y_ratio = self.height as f32 / new_height as f32;

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = (x as f32 * x_ratio) as u32;
                let src_y = (y as f32 * y_ratio) as u32;

                let pixel = self.get_pixel(src_x, src_y);
                let idx = ((y * new_width + x) * 4) as usize;

                new_pixels[idx] = pixel.r;
                new_pixels[idx + 1] = pixel.g;
                new_pixels[idx + 2] = pixel.b;
                new_pixels[idx + 3] = pixel.a;
            }
        }

        Self {
            width: new_width,
            height: new_height,
            pixels: new_pixels,
            format: self.format,
        }
    }

    /// Convert to grayscale.
    pub fn to_grayscale(&self) -> Self {
        let mut new_pixels = self.pixels.clone();

        for i in (0..new_pixels.len()).step_by(4) {
            let r = new_pixels[i] as f32;
            let g = new_pixels[i + 1] as f32;
            let b = new_pixels[i + 2] as f32;

            // Luminance formula
            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

            new_pixels[i] = gray;
            new_pixels[i + 1] = gray;
            new_pixels[i + 2] = gray;
        }

        Self {
            width: self.width,
            height: self.height,
            pixels: new_pixels,
            format: self.format,
        }
    }

    /// Apply alpha premultiplication.
    pub fn premultiply_alpha(&mut self) {
        for i in (0..self.pixels.len()).step_by(4) {
            let alpha = self.pixels[i + 3] as f32 / 255.0;
            self.pixels[i] = (self.pixels[i] as f32 * alpha) as u8;
            self.pixels[i + 1] = (self.pixels[i + 1] as f32 * alpha) as u8;
            self.pixels[i + 2] = (self.pixels[i + 2] as f32 * alpha) as u8;
        }
    }
}

/// Image loading errors.
#[derive(Debug, thiserror::Error)]
pub enum ImageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("decode error: {0}")]
    DecodeError(String),

    #[error("unsupported format")]
    UnsupportedFormat,

    #[error("invalid dimensions")]
    InvalidDimensions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_image() {
        let pixels = vec![255u8; 100 * 100 * 4];
        let img = DecodedImage::new(100, 100, pixels, ImageFormat::Png);
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
    }

    #[test]
    fn get_set_pixel() {
        let pixels = vec![0u8; 10 * 10 * 4];
        let mut img = DecodedImage::new(10, 10, pixels, ImageFormat::Png);

        img.set_pixel(5, 5, Rgba8::rgb(255, 0, 0));
        let pixel = img.get_pixel(5, 5);

        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 0);
        assert_eq!(pixel.b, 0);
    }

    #[test]
    fn resize_image() {
        let pixels = vec![255u8; 100 * 100 * 4];
        let img = DecodedImage::new(100, 100, pixels, ImageFormat::Png);

        let resized = img.resize(50, 50);
        assert_eq!(resized.width, 50);
        assert_eq!(resized.height, 50);
    }
}
