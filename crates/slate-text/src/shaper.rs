//! Text shaping using rustybuzz (harfbuzz implementation).

use super::{Font, GlyphId, GlyphRun, PositionedGlyph, TextDirection};
use slate_ais::{Point, SubPixel};
use rustybuzz::{Face, UnicodeBuffer, Direction as HbDirection};
use unicode_bidi::BidiInfo;

/// Text shaping result.
#[derive(Debug, Clone)]
pub struct ShapeResult {
    pub glyph_run: GlyphRun,
    pub direction: TextDirection,
}

/// Text shaper for converting Unicode text to positioned glyphs.
pub struct TextShaper {
    // Stateless - rustybuzz handles everything
}

impl TextShaper {
    /// Create a new text shaper.
    pub fn new() -> Self {
        Self {}
    }

    /// Shape a text string into glyphs using harfbuzz.
    pub fn shape(
        &self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
        direction: TextDirection,
    ) -> ShapeResult {
        // Parse font face with rustybuzz
        let face = match Face::from_slice(&font.data, 0) {
            Some(face) => face,
            None => {
                // Fallback to simple shaping if font parsing fails
                return self.shape_fallback(text, font_size, direction);
            }
        };

        // Create harfbuzz buffer
        let mut buffer = UnicodeBuffer::new();
        buffer.push_str(text);
        
        // Set direction
        let hb_direction = match direction {
            TextDirection::LeftToRight => HbDirection::LeftToRight,
            TextDirection::RightToLeft => HbDirection::RightToLeft,
        };
        buffer.set_direction(hb_direction);

        // Shape the text
        let output = rustybuzz::shape(&face, &[], buffer);

        // Convert to our glyph format
        let mut glyph_run = GlyphRun::new(font_size);
        let positions = output.glyph_positions();
        let infos = output.glyph_infos();

        // Scale factor from font units to pixels
        let scale = font_size.0 / font.metrics.units_per_em as f32;
        
        let mut x = SubPixel::from(0);
        let mut y = SubPixel::from(0);

        for (info, pos) in infos.iter().zip(positions.iter()) {
            let glyph_id = GlyphId(info.glyph_id);
            
            // Apply position offsets
            let glyph_x = x + SubPixel(pos.x_offset as f32 * scale);
            let glyph_y = y + SubPixel(pos.y_offset as f32 * scale);
            
            let advance = SubPixel(pos.x_advance as f32 * scale);

            glyph_run.push(PositionedGlyph {
                glyph_id,
                position: Point { x: glyph_x, y: glyph_y },
                advance,
            });

            x = x + advance;
            y = y + SubPixel(pos.y_advance as f32 * scale);
        }

        ShapeResult {
            glyph_run,
            direction,
        }
    }

    /// Fallback shaping for when harfbuzz fails.
    fn shape_fallback(
        &self,
        text: &str,
        font_size: SubPixel,
        direction: TextDirection,
    ) -> ShapeResult {
        let mut glyph_run = GlyphRun::new(font_size);
        let mut x = SubPixel::from(0);

        for ch in text.chars() {
            let glyph_id = GlyphId(ch as u32);
            let advance = font_size * 0.6; // Rough approximation

            glyph_run.push(PositionedGlyph {
                glyph_id,
                position: Point { x, y: 0.into() },
                advance,
            });

            x = x + advance;
        }

        ShapeResult {
            glyph_run,
            direction,
        }
    }

    /// Shape text with BiDi support using Unicode BiDi algorithm.
    pub fn shape_bidi(
        &self,
        text: &str,
        font: &Font,
        font_size: SubPixel,
    ) -> Vec<ShapeResult> {
        // Run Unicode BiDi algorithm
        let bidi_info = BidiInfo::new(text, None);
        
        // Get the paragraph
        if bidi_info.paragraphs.is_empty() {
            return vec![self.shape(text, font, font_size, TextDirection::LeftToRight)];
        }
        
        let para = &bidi_info.paragraphs[0];
        let line = para.range.clone();
        
        // Process each level run
        let mut results = Vec::new();
        let (levels, runs) = bidi_info.visual_runs(para, line.clone());

        for (level, run) in levels.iter().zip(runs.iter()) {
            let text_slice = &text[run.clone()];
            let direction = if level.is_ltr() {
                TextDirection::LeftToRight
            } else {
                TextDirection::RightToLeft
            };

            results.push(self.shape(text_slice, font, font_size, direction));
        }

        // If no runs, fallback to LTR
        if results.is_empty() {
            results.push(self.shape(text, font, font_size, TextDirection::LeftToRight));
        }

        results
    }
}

impl Default for TextShaper {
    fn default() -> Self {
        Self::new()
    }
}
