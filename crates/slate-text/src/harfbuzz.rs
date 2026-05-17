//! Harfbuzz text shaping integration.

use super::{Font, GlyphId, GlyphRun, PositionedGlyph, TextDirection};
use slate_ais::{Point, SubPixel};
use std::collections::HashMap;

/// Harfbuzz shaper wrapper.
pub struct HarfbuzzShaper {
    // In real implementation, this would hold hb_font_t and hb_buffer_t
    cache: HashMap<String, ShapedResult>,
}

/// Shaped text result from Harfbuzz.
#[derive(Clone)]
struct ShapedResult {
    glyphs: Vec<ShapedGlyph>,
}

/// A shaped glyph with position info.
#[derive(Clone, Copy)]
struct ShapedGlyph {
    glyph_id: GlyphId,
    x_offset: f32,
    y_offset: f32,
    x_advance: f32,
    y_advance: f32,
}

impl HarfbuzzShaper {
    /// Create a new Harfbuzz shaper.
    pub fn new() -> Result<Self, HarfbuzzError> {
        Ok(Self {
            cache: HashMap::new(),
        })
    }

    /// Shape text using Harfbuzz.
    pub fn shape(
        &mut self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
        direction: TextDirection,
    ) -> Result<GlyphRun, HarfbuzzError> {
        // Check cache
        let cache_key = format!("{}:{}:{:?}", text, font_size.to_f32(), direction);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(self.convert_to_glyph_run(cached, font_size));
        }

        // In real implementation:
        // 1. hb_buffer_create()
        // 2. hb_buffer_add_utf8(buffer, text, ...)
        // 3. hb_buffer_set_direction(buffer, direction)
        // 4. hb_buffer_set_script(buffer, script)
        // 5. hb_buffer_set_language(buffer, language)
        // 6. hb_shape(font, buffer, features)
        // 7. hb_buffer_get_glyph_infos(buffer)
        // 8. hb_buffer_get_glyph_positions(buffer)

        // Simulate shaping
        let shaped = self.shape_simple(text, font_size)?;
        self.cache.insert(cache_key, shaped.clone());

        Ok(self.convert_to_glyph_run(&shaped, font_size))
    }

    /// Simple shaping (fallback when Harfbuzz not available).
    fn shape_simple(&self, text: &str, font_size: SubPixel) -> Result<ShapedResult, HarfbuzzError> {
        let mut glyphs = Vec::new();

        for ch in text.chars() {
            let glyph = ShapedGlyph {
                glyph_id: GlyphId(ch as u32),
                x_offset: 0.0,
                y_offset: 0.0,
                x_advance: font_size.to_f32() * 0.6,
                y_advance: 0.0,
            };
            glyphs.push(glyph);
        }

        Ok(ShapedResult { glyphs })
    }

    /// Convert shaped result to GlyphRun.
    fn convert_to_glyph_run(&self, shaped: &ShapedResult, font_size: SubPixel) -> GlyphRun {
        let mut glyph_run = GlyphRun::new(font_size);
        let mut x = SubPixel::from(0);

        for shaped_glyph in &shaped.glyphs {
            let positioned = PositionedGlyph {
                glyph_id: shaped_glyph.glyph_id,
                position: Point {
                    x: x + SubPixel::from(shaped_glyph.x_offset),
                    y: SubPixel::from(shaped_glyph.y_offset),
                },
                advance: SubPixel::from(shaped_glyph.x_advance),
            };

            glyph_run.push(positioned);
            x = x + SubPixel::from(shaped_glyph.x_advance);
        }

        glyph_run
    }

    /// Shape text with BiDi support.
    pub fn shape_bidi(
        &mut self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
    ) -> Result<Vec<GlyphRun>, HarfbuzzError> {
        // In real implementation:
        // 1. Use unicode-bidi crate to analyze text
        // 2. Split into runs with same direction
        // 3. Shape each run separately
        // 4. Reorder runs according to BiDi algorithm

        // For now, assume LTR
        let run = self.shape(text, font, font_size, TextDirection::LeftToRight)?;
        Ok(vec![run])
    }

    /// Clear the shaping cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for HarfbuzzShaper {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Harfbuzz errors.
#[derive(Debug, thiserror::Error)]
pub enum HarfbuzzError {
    #[error("shaping failed")]
    ShapingFailed,

    #[error("invalid text")]
    InvalidText,

    #[error("font not loaded")]
    FontNotLoaded,
}

/// Advanced text shaper with full Unicode support.
pub struct AdvancedTextShaper {
    harfbuzz: HarfbuzzShaper,
}

impl AdvancedTextShaper {
    /// Create a new advanced text shaper.
    pub fn new() -> Result<Self, HarfbuzzError> {
        Ok(Self {
            harfbuzz: HarfbuzzShaper::new()?,
        })
    }

    /// Shape text with full Unicode support.
    pub fn shape_unicode(
        &mut self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
    ) -> Result<Vec<GlyphRun>, HarfbuzzError> {
        // Handle complex scripts
        if self.is_complex_script(text) {
            self.shape_complex(text, font, font_size)
        } else {
            // Simple LTR text
            let run = self.harfbuzz.shape(text, font, font_size, TextDirection::LeftToRight)?;
            Ok(vec![run])
        }
    }

    /// Check if text contains complex scripts.
    fn is_complex_script(&self, text: &str) -> bool {
        text.chars().any(|ch| {
            // Arabic, Hebrew, Devanagari, Thai, etc.
            matches!(ch as u32,
                0x0600..=0x06FF | // Arabic
                0x0590..=0x05FF | // Hebrew
                0x0900..=0x097F | // Devanagari
                0x0E00..=0x0E7F   // Thai
            )
        })
    }

    /// Shape complex script text.
    fn shape_complex(
        &mut self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
    ) -> Result<Vec<GlyphRun>, HarfbuzzError> {
        // In real implementation:
        // 1. Detect script runs
        // 2. Apply script-specific shaping
        // 3. Handle ligatures and contextual forms
        // 4. Apply OpenType features

        self.harfbuzz.shape_bidi(text, font, font_size)
    }
}

impl Default for AdvancedTextShaper {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_shaper() {
        let shaper = HarfbuzzShaper::new();
        assert!(shaper.is_ok());
    }

    #[test]
    fn shape_simple_text() {
        let mut shaper = HarfbuzzShaper::new().unwrap();
        let font = Font::from_bytes(vec![]).unwrap();
        let result = shaper.shape("Hello", &font, 16.into(), TextDirection::LeftToRight);
        assert!(result.is_ok());
    }

    #[test]
    fn detect_complex_script() {
        let shaper = AdvancedTextShaper::new().unwrap();
        assert!(!shaper.is_complex_script("Hello"));
        assert!(shaper.is_complex_script("مرحبا")); // Arabic
        assert!(shaper.is_complex_script("שלום")); // Hebrew
    }
}
