//! Layout primitives.
//!
//! Layout in Slate is a pure function over geometry. A primitive never
//! reads the DOM tree, never triggers a synchronous reflow, and never
//! observes wall time. That makes every batch of layout primitives
//! trivially parallelizable — either across Rayon threads on the CPU
//! or across lanes of a SIMD register.
//!
//! The SIMD path is not implemented in Phase 1 but the data layout here
//! (sub-pixel `f32`, `repr(C)`, fixed-size variants) is chosen so it
//! can be added without breaking the type system.

use crate::geom::{NodeId, Point, Rect, Size, SubPixel};

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutPrimitive {
    /// Place `node` at an absolute document-space position.
    SetPosition { node: NodeId, point: Point },

    /// Set the geometric size of `node`. No min/max/aspect resolution —
    /// those are the Dispatcher's job before this primitive is emitted.
    SetSize { node: NodeId, size: Size },

    /// Rigid clip rect. Children outside are discarded at render time.
    SetClip { node: NodeId, rect: Rect },

    /// Flex-basis along the main axis. Designed to be evaluated in
    /// parallel across a batch of siblings via SIMD.
    FlexBasis { node: NodeId, basis: SubPixel },

    /// Detach `node` from the normal flow. Subsequent SetPosition
    /// instructions for it are honored absolutely.
    DetachFromFlow { node: NodeId },
}

impl LayoutPrimitive {
    /// The node this primitive targets. Used by the kernel to shard
    /// instruction streams across worker threads.
    #[inline(always)]
    pub const fn target(&self) -> NodeId {
        match self {
            LayoutPrimitive::SetPosition    { node, .. } => *node,
            LayoutPrimitive::SetSize        { node, .. } => *node,
            LayoutPrimitive::SetClip        { node, .. } => *node,
            LayoutPrimitive::FlexBasis      { node, .. } => *node,
            LayoutPrimitive::DetachFromFlow { node }     => *node,
        }
    }
}
