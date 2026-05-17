//! Geometric primitives shared by layout and rendering.
//!
//! All dimensions are stored as [`SubPixel`] (an `f32` newtype) so that
//! fractional positions round-trip correctly through the layout engine
//! without the "1-pixel drift" bug that plagues integer-only pipelines.
//!
//! Types in this module are `#[repr(C)]` so they can be memcpy'd into a
//! GPU command buffer without translation.

use core::ops::{Add, Sub, Mul, Div};

/// An identifier into the state store's node slotmap. The `u32` here is
/// a *logical* id, not a raw pointer; generational versioning lives in
/// `slate-state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(transparent)]
pub struct NodeId(pub u32);

// Implement slotmap::Key for NodeId
unsafe impl slotmap::Key for NodeId {
    fn data(&self) -> slotmap::KeyData {
        slotmap::KeyData::from_ffi(self.0 as u64)
    }

    fn null() -> Self {
        NodeId(u32::MAX)
    }

    fn is_null(&self) -> bool {
        self.0 == u32::MAX
    }
}

impl From<slotmap::KeyData> for NodeId {
    fn from(data: slotmap::KeyData) -> Self {
        NodeId(data.as_ffi() as u32)
    }
}

/// Sub-pixel scalar. `f32` is deliberate: modern GPUs prefer 32-bit
/// floats and SIMD lanes on x86-64/AArch64 hold 4–16 of them natively.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
pub struct SubPixel(pub f32);

impl SubPixel {
    pub const ZERO: SubPixel = SubPixel(0.0);
    pub const INFINITY: SubPixel = SubPixel(f32::INFINITY);
    
    #[inline(always)]
    pub const fn new(v: f32) -> Self { SubPixel(v) }
    
    #[inline(always)]
    pub fn raw(self) -> f32 { self.0 }
    
    #[inline(always)]
    pub fn to_f32(self) -> f32 { self.0 }
    
    #[inline(always)]
    pub fn to_i32(self) -> i32 { self.0 as i32 }
    
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        SubPixel(self.0.max(other.0))
    }
    
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        SubPixel(self.0.min(other.0))
    }
    
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        SubPixel(self.0.clamp(min.0, max.0))
    }
}

// Implement From<f32> for SubPixel
impl From<f32> for SubPixel {
    fn from(v: f32) -> Self {
        SubPixel(v)
    }
}

// Implement From<i32> for SubPixel
impl From<i32> for SubPixel {
    fn from(v: i32) -> Self {
        SubPixel(v as f32)
    }
}

// Implement Add for SubPixel
impl Add for SubPixel {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        SubPixel(self.0 + other.0)
    }
}

// Implement Sub for SubPixel
impl Sub for SubPixel {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        SubPixel(self.0 - other.0)
    }
}

// Implement Mul<f32> for SubPixel
impl Mul<f32> for SubPixel {
    type Output = Self;
    
    fn mul(self, other: f32) -> Self {
        SubPixel(self.0 * other)
    }
}

// Implement Mul<SubPixel> for SubPixel
impl Mul for SubPixel {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        SubPixel(self.0 * other.0)
    }
}

// Implement Div<f32> for SubPixel
impl Div<f32> for SubPixel {
    type Output = Self;
    
    fn div(self, other: f32) -> Self {
        SubPixel(self.0 / other)
    }
}

// Implement Div<SubPixel> for SubPixel
impl Div for SubPixel {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        SubPixel(self.0 / other.0)
    }
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
    
    // Convenience accessors for width/height
    #[inline(always)]
    pub fn width(&self) -> SubPixel {
        self.w
    }
    
    #[inline(always)]
    pub fn height(&self) -> SubPixel {
        self.h
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
