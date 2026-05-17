//! Glyph rasterization using ab_glyph (pure Rust font rendering).

use super::{Font, GlyphId};
use ab_glyph::{FontRef, PxScale, Font as AbFont, GlyphId as AbGlyphId};
use slate_ais::SubPixel;

/// Rasterized glyph bitmap.
#[derive(Debug, Clone)]
pub struct RasterizedGlyph {
    pub width: u32,
    pub height: u32,
    pub left: i32,
    pub top: i32,
    pub pixels: Vec<u8>, // Grayscale alpha values
}

/// Glyph rasterizer using ab_glyph.
pub struct GlyphRasterizer {
    // Stateless - ab_glyph handles everything
}

impl GlyphRasterizer {
    /// Create a new glyph rasterizer.
    pub fn new() -> Self {
        Self {}
    }

    /// Rasterize a glyph at the given size.
    pub fn rasterize(
        &self,
        font: &Font,
        glyph_id: GlyphId,
        font_size: SubPixel,
    ) -> Option<RasterizedGlyph> {
        // Parse font with ab_glyph
        let font_ref = FontRef::try_from_slice(&font.data).ok()?;
        
        // Scale to pixel size
        let scale = PxScale::from(font_size.0);

        // Get the glyph using ab_glyph's GlyphId
        let ab_glyph_id = AbGlyphId(glyph_id.0 as u16);
        let glyph = ab_glyph_id.with_scale(scale);
        
        // Outline the glyph
        let outlined = font_ref.outline_glyph(glyph)?;

        // Get bounding box
        let bounds = outlined.px_bounds();
        let width = bounds.width() as u32;
        let height = bounds.height() as u32;

        if width == 0 || height == 0 {
            return None;
        }

        // Rasterize to bitmap
        let mut pixels = vec![0u8; (width * height) as usize];
        outlined.draw(|x, y, coverage| {
            let idx = (y * width + x) as usize;
            if idx < pixels.len() {
                pixels[idx] = (coverage * 255.0) as u8;
            }
        });

        Some(RasterizedGlyph {
            width,
            height,
            left: bounds.min.x as i32,
            top: bounds.min.y as i32,
            pixels,
        })
    }

    /// Rasterize multiple glyphs (for caching).
    pub fn rasterize_batch(
        &self,
        font: &Font,
        glyph_ids: &[GlyphId],
        font_size: SubPixel,
    ) -> Vec<Option<RasterizedGlyph>> {
        glyph_ids
            .iter()
            .map(|&glyph_id| self.rasterize(font, glyph_id, font_size))
            .collect()
    }
}

impl Default for GlyphRasterizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rasterizer_creation() {
        let _rasterizer = GlyphRasterizer::new();
    }
}
