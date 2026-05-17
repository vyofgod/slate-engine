//! # Slate Rasterizer
//!
//! CPU-based rasterizer for Phase 2. Converts display list to pixels.

use slate_ais::{Point, Rect, Rgba8, Size, SubPixel};

pub mod display_list;
pub mod painter;

pub use display_list::{DisplayCommand, DisplayList};
pub use painter::Painter;

/// A rasterized frame buffer.
pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA8
}

impl FrameBuffer {
    /// Create a new frame buffer.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    /// Clear the frame buffer.
    pub fn clear(&mut self, color: Rgba8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set_pixel(x, y, color);
            }
        }
    }

    /// Set a pixel.
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

    /// Get a pixel.
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

    /// Fill a rectangle.
    pub fn fill_rect(&mut self, rect: Rect, color: Rgba8) {
        let x1 = rect.origin.x.to_i32().max(0) as u32;
        let y1 = rect.origin.y.to_i32().max(0) as u32;
        let x2 = (rect.origin.x + rect.size.w).to_i32().min(self.width as i32) as u32;
        let y2 = (rect.origin.y + rect.size.h).to_i32().min(self.height as i32) as u32;

        for y in y1..y2 {
            for x in x1..x2 {
                self.blend_pixel(x, y, color);
            }
        }
    }

    /// Stroke a rectangle.
    pub fn stroke_rect(&mut self, rect: Rect, color: Rgba8, width: f32) {
        // Top
        self.fill_rect(
            Rect {
                origin: Point { x: rect.origin.x, y: rect.origin.y },
                size: Size { w: rect.size.w, h: SubPixel::from(width) },
            },
            color,
        );

        // Bottom
        self.fill_rect(
            Rect {
                origin: Point { 
                    x: rect.origin.x, 
                    y: rect.origin.y + rect.size.h - SubPixel::from(width) 
                },
                size: Size { w: rect.size.w, h: SubPixel::from(width) },
            },
            color,
        );

        // Left
        self.fill_rect(
            Rect {
                origin: Point { x: rect.origin.x, y: rect.origin.y },
                size: Size { w: SubPixel::from(width), h: rect.size.h },
            },
            color,
        );

        // Right
        self.fill_rect(
            Rect {
                origin: Point { 
                    x: rect.origin.x + rect.size.w - SubPixel::from(width), 
                    y: rect.origin.y 
                },
                size: Size { w: SubPixel::from(width), h: rect.size.h },
            },
            color,
        );
    }

    /// Blend a pixel with alpha.
    fn blend_pixel(&mut self, x: u32, y: u32, color: Rgba8) {
        if color.a == 0 {
            return;
        }

        if color.a == 255 {
            self.set_pixel(x, y, color);
            return;
        }

        let bg = self.get_pixel(x, y);
        let alpha = color.a as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;

        let blended = Rgba8 {
            r: ((color.r as f32 * alpha) + (bg.r as f32 * inv_alpha)) as u8,
            g: ((color.g as f32 * alpha) + (bg.g as f32 * inv_alpha)) as u8,
            b: ((color.b as f32 * alpha) + (bg.b as f32 * inv_alpha)) as u8,
            a: 255,
        };

        self.set_pixel(x, y, blended);
    }

    /// Save as PPM format.
    pub fn save_ppm(&self, path: &str) -> std::io::Result<()> {
        use std::io::Write;

        let mut file = std::fs::File::create(path)?;
        writeln!(file, "P6")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) * 4) as usize;
                file.write_all(&[
                    self.pixels[idx],
                    self.pixels[idx + 1],
                    self.pixels[idx + 2],
                ])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_framebuffer() {
        let fb = FrameBuffer::new(800, 600);
        assert_eq!(fb.width, 800);
        assert_eq!(fb.height, 600);
        assert_eq!(fb.pixels.len(), 800 * 600 * 4);
    }

    #[test]
    fn fill_rect() {
        let mut fb = FrameBuffer::new(100, 100);
        fb.fill_rect(
            Rect {
                origin: Point { x: 10.into(), y: 10.into() },
                size: Size { w: 20.into(), h: 20.into() },
            },
            Rgba8::rgb(255, 0, 0),
        );

        let pixel = fb.get_pixel(15, 15);
        assert_eq!(pixel.r, 255);
    }
}
