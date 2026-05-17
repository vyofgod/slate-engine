//! Glyph representation and positioning.

use slate_ais::{Point, SubPixel};

/// A glyph identifier within a font.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphId(pub u32);

/// A positioned glyph ready for rendering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionedGlyph {
    pub glyph_id: GlyphId,
    pub position: Point,
    pub advance: SubPixel,
}

/// A run of shaped glyphs with shared properties.
#[derive(Debug, Clone, PartialEq)]
pub struct GlyphRun {
    pub glyphs: Vec<PositionedGlyph>,
    pub font_size: SubPixel,
    pub total_advance: SubPixel,
}

impl GlyphRun {
    /// Create a new empty glyph run.
    pub fn new(font_size: SubPixel) -> Self {
        Self {
            glyphs: Vec::new(),
            font_size,
            total_advance: 0.into(),
        }
    }

    /// Add a glyph to the run.
    pub fn push(&mut self, glyph: PositionedGlyph) {
        self.total_advance = self.total_advance + glyph.advance;
        self.glyphs.push(glyph);
    }

    /// Get the width of the glyph run.
    #[inline]
    pub fn width(&self) -> SubPixel {
        self.total_advance
    }

    /// Get the number of glyphs.
    #[inline]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Check if the run is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }
}
