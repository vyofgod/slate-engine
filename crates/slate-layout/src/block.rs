//! CSS Block layout (normal flow).

use super::{Constraints, LayoutEngine, LayoutResult};
use slate_ais::{LayoutPrimitive, NodeId, Point, Size, SubPixel};
use std::collections::HashMap;

/// Block formatting context.
#[derive(Debug, Clone)]
pub struct BlockContext {
    pub node: NodeId,
    pub children: Vec<NodeId>,
    pub margin_collapse: bool,
}

/// Block layout engine for normal flow.
pub struct BlockLayout {
    contexts: HashMap<NodeId, BlockContext>,
    node_sizes: HashMap<NodeId, Size>,
}

impl BlockLayout {
    /// Create a new block layout engine.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
            node_sizes: HashMap::new(),
        }
    }

    /// Register a block formatting context.
    pub fn add_context(&mut self, context: BlockContext) {
        self.contexts.insert(context.node, context);
    }

    /// Set intrinsic size for a node.
    pub fn set_node_size(&mut self, node: NodeId, size: Size) {
        self.node_sizes.insert(node, size);
    }

    /// Layout a block container.
    fn layout_block(&self, context: &BlockContext, constraints: Constraints) -> LayoutResult {
        let mut result = LayoutResult::new(Size::ZERO);
        let mut current_y = SubPixel::ZERO;
        let mut max_width = SubPixel::ZERO;

        // Layout children vertically
        for &child in &context.children {
            // Position child
            result.push(LayoutPrimitive::SetPosition {
                node: child,
                point: Point {
                    x: SubPixel::ZERO,
                    y: current_y,
                },
            });

            // Get or calculate child size
            let child_size = self.node_sizes.get(&child).copied().unwrap_or(Size {
                w: constraints.max_width,
                h: SubPixel(100.0), // Default height
            });

            result.push(LayoutPrimitive::SetSize {
                node: child,
                size: child_size,
            });

            current_y = current_y + child_size.h;
            max_width = max_width.max(child_size.w);
        }

        // Set container size
        result.size = Size {
            w: max_width,
            h: current_y,
        };

        result
    }
}

impl Default for BlockLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine for BlockLayout {
    fn layout(&mut self, node: NodeId, constraints: Constraints) -> LayoutResult {
        if let Some(context) = self.contexts.get(&node).cloned() {
            self.layout_block(&context, constraints)
        } else {
            LayoutResult::new(Size::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_layout_vertical() {
        let mut block = BlockLayout::new();

        let context = BlockContext {
            node: NodeId(1),
            children: vec![NodeId(2), NodeId(3)],
            margin_collapse: true,
        };

        block.add_context(context);
        block.set_node_size(NodeId(2), Size { w: SubPixel(200.0), h: SubPixel(100.0) });
        block.set_node_size(NodeId(3), Size { w: SubPixel(200.0), h: SubPixel(150.0) });

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(600.0),
        };

        let result = block.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
        assert_eq!(result.size.h, SubPixel(250.0));
    }

    #[test]
    fn block_layout_empty() {
        let mut block = BlockLayout::new();

        let context = BlockContext {
            node: NodeId(1),
            children: vec![],
            margin_collapse: false,
        };

        block.add_context(context);

        let constraints = Constraints::UNBOUNDED;
        let result = block.layout(NodeId(1), constraints);
        assert_eq!(result.size.h, SubPixel::ZERO);
    }

    #[test]
    fn block_set_node_size() {
        let mut block = BlockLayout::new();
        let size = Size { w: SubPixel(100.0), h: SubPixel(50.0) };
        
        block.set_node_size(NodeId(1), size);
        
        // Verify size was stored
        assert_eq!(block.node_sizes.get(&NodeId(1)), Some(&size));
    }
}
