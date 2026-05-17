//! # Slate Layout Engine
//!
//! Advanced layout algorithms: Flexbox, Grid, Block, Inline.
//! Pure functional layout with no DOM dependencies.

use slate_ais::{LayoutPrimitive, NodeId, Size, SubPixel};

pub mod flexbox;
pub mod grid;
pub mod block;
pub mod inline;

pub use flexbox::{FlexContainer, FlexItem, FlexLayout};
pub use grid::{GridContainer, GridItem, GridLayout};
pub use block::BlockLayout;
pub use inline::InlineLayout;

/// Layout constraints for a node.
#[derive(Debug, Clone, Copy)]
pub struct Constraints {
    pub min_width: SubPixel,
    pub max_width: SubPixel,
    pub min_height: SubPixel,
    pub max_height: SubPixel,
}

impl Constraints {
    /// Unconstrained layout.
    pub const UNBOUNDED: Self = Self {
        min_width: SubPixel::ZERO,
        max_width: SubPixel::INFINITY,
        min_height: SubPixel::ZERO,
        max_height: SubPixel::INFINITY,
    };

    /// Tight constraints (fixed size).
    pub fn tight(size: Size) -> Self {
        Self {
            min_width: size.w,
            max_width: size.w,
            min_height: size.h,
            max_height: size.h,
        }
    }

    /// Constrain a size to these constraints.
    pub fn constrain(&self, size: Size) -> Size {
        Size {
            w: size.w.clamp(self.min_width, self.max_width),
            h: size.h.clamp(self.min_height, self.max_height),
        }
    }
}

/// Layout result for a node.
#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub size: Size,
    pub primitives: Vec<LayoutPrimitive>,
}

impl LayoutResult {
    /// Create a new layout result.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            primitives: Vec::new(),
        }
    }

    /// Add a primitive to the result.
    pub fn push(&mut self, primitive: LayoutPrimitive) {
        self.primitives.push(primitive);
    }
}

/// Layout engine trait.
pub trait LayoutEngine {
    /// Compute layout for a node and its children.
    fn layout(&mut self, node: NodeId, constraints: Constraints) -> LayoutResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraints_clamp() {
        let constraints = Constraints {
            min_width: 100.into(),
            max_width: 200.into(),
            min_height: 50.into(),
            max_height: 150.into(),
        };

        let size = Size {
            w: SubPixel(250.0),
            h: SubPixel(30.0),
        };

        let clamped = constraints.constrain(size);
        assert_eq!(clamped.w, SubPixel(200.0));
        assert_eq!(clamped.h, SubPixel(50.0));
    }
}
