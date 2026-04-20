//! Geometric primitives shared by layout and rendering.
//!
//! All dimensions are stored as [`SubPixel`] (an `f32` newtype) so that
//! fractional positions round-trip correctly through the layout engine
//! without the "1-pixel drift" bug that plagues integer-only pipelines.
//!
//! Types in this module are `#[repr(C)]` so they can be memcpy'd into a
//! GPU command buffer without translation.

/// An identifier into the state store's node slotmap. The `u32` here is
/// a *logical* id, not a raw pointer; generational versioning lives in
/// `slate-state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NodeId(pub u32);

/// Sub-pixel scalar. `f32` is deliberate: modern GPUs prefer 32-bit
/// floats and SIMD lanes on x86-64/AArch64 hold 4–16 of them natively.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct SubPixel(pub f32);

impl SubPixel {
    pub const ZERO: SubPixel = SubPixel(0.0);
    #[inline(always)]
    pub const fn new(v: f32) -> Self { SubPixel(v) }
    #[inline(always)]
    pub fn raw(self) -> f32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Point {
    pub x: SubPixel,
    pub y: SubPixel,
}

impl Point {
    pub const ORIGIN: Point = Point { x: SubPixel::ZERO, y: SubPixel::ZERO };
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Point { x: SubPixel(x), y: SubPixel(y) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Size {
    pub w: SubPixel,
    pub h: SubPixel,
}

impl Size {
    pub const ZERO: Size = Size { w: SubPixel::ZERO, h: SubPixel::ZERO };
    #[inline(always)]
    pub const fn new(w: f32, h: f32) -> Self {
        Size { w: SubPixel(w), h: SubPixel(h) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Rect {
    pub origin: Point,
    pub size:   Size,
}

impl Rect {
    #[inline(always)]
    pub const fn from_ltwh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { origin: Point::new(x, y), size: Size::new(w, h) }
    }
}
