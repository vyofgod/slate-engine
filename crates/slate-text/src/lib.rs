//! # Slate Text Rendering Engine
//!
//! High-performance text shaping, layout, and rendering using rustybuzz
//! and ab_glyph. Supports complex scripts, BiDi, and subpixel positioning.

use slate_ais::{NodeId, Point, SubPixel};
use std::sync::Arc;

pub mod font;
pub mod glyph;
pub mod layout;
pub mod shaper;
pub mod rasterizer;

pub use font::{Font, FontCache, FontFamily, FontStyle, FontWeight};
pub use glyph::{GlyphId, GlyphRun, PositionedGlyph};
pub use layout::{LineBreaker, TextLayout, TextLayoutOptions};
pub use shaper::{ShapeResult, TextShaper};
pub use rasterizer::{GlyphRasterizer, RasterizedGlyph};

/// A text primitive for the AIS.
#[derive(Debug, Clone, PartialEq)]
pub struct TextPrimitive {
    pub node: NodeId,
    pub glyphs: Arc<GlyphRun>,
    pub position: Point,
    pub color: [u8; 4],
}

/// Font metrics for layout calculations.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    pub ascent: SubPixel,
    pub descent: SubPixel,
    pub line_gap: SubPixel,
    pub units_per_em: u16,
}

/// Text direction for BiDi support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
}

/// Word breaking strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordBreak {
    Normal,
    BreakAll,
    KeepAll,
}

/// Text overflow handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOverflow {
    Clip,
    Ellipsis,
    Fade,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_primitive_size() {
        // Keep text primitives cache-friendly
        assert!(std::mem::size_of::<TextPrimitive>() <= 64);
    }
}
