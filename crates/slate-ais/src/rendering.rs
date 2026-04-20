//! Rendering primitives.
//!
//! These map 1:1 to GPU command-buffer operations. They are laid out
//! with `repr(C)` so a slice of them can be uploaded to the GPU without
//! an intermediate marshaling step.
//!
//! No CPU paint phase. No backing bitmap. No compositor trees. The
//! primitives *are* the draw calls.

use crate::geom::{Point, Rect};

/// 8-bit sRGB. The premultiplied-alpha question is resolved by the GPU
/// pipeline, not by this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Rgba8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba8 {
    pub const TRANSPARENT: Rgba8 = Rgba8 { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK:       Rgba8 = Rgba8 { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE:       Rgba8 = Rgba8 { r: 255, g: 255, b: 255, a: 255 };

    #[inline(always)]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Rgba8 { r, g, b, a: 255 }
    }
}

/// Opaque handle into the GPU layer table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LayerId(pub u32);

/// Opaque handle into a pre-shaped glyph run. Shaping is not the
/// Render primitive's job — the Dispatcher owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GlyphRunId(pub u32);

/// Opaque handle into the Bezier path cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PathId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum RenderPrimitive {
    /// Solid-color axis-aligned fill.
    FillRect { rect: Rect, color: Rgba8 },

    /// Stroked axis-aligned rectangle.
    StrokeRect { rect: Rect, color: Rgba8, width: f32 },

    /// Pre-shaped glyph run anchored at `origin`.
    DrawText { run: GlyphRunId, origin: Point, color: Rgba8 },

    /// Fill a cached Bezier path. Curves are shaped once; this primitive
    /// just references the cached result.
    FillPath { path: PathId, color: Rgba8 },

    /// Push a new layer onto the compositor stack.
    PushLayer { id: LayerId, bounds: Rect },

    /// Pop the top layer.
    PopLayer,

    /// Blit one layer onto another with flat opacity.
    LayerBlit { src: LayerId, dst: LayerId, at: Point, opacity: u8 },
}
