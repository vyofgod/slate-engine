//! CSS Inline layout (text flow).

use super::{Constraints, LayoutEngine, LayoutResult};
use slate_ais::{LayoutPrimitive, NodeId, Point, Size, SubPixel};
use std::collections::HashMap;

/// Inline formatting context.
#[derive(Debug, Clone)]
pub struct InlineContext {
    pub node: NodeId,
    pub children: Vec<InlineBox>,
    pub line_height: SubPixel,
}

/// An inline box (text or inline element).
#[derive(Debug, Clone)]
pub struct InlineBox {
    pub node: NodeId,
    pub width: SubPixel,
    pub height: SubPixel,
    pub baseline: SubPixel,
}

impl InlineBox {
    /// Create a new inline box from text content.
    /// In real implementation, this would use text shaping (Harfbuzz).
    pub fn from_text(node: NodeId, text: &str, font_size: SubPixel) -> Self {
        // Approximate width: 0.6 * font_size per character
        let char_count = text.chars().count();
        let width = font_size * (char_count as f32 * 0.6);
        
        // Height is typically 1.2 * font_size (with line-height)
        let height = font_size * 1.2;
        
        // Baseline is typically 0.8 * font_size from top
        let baseline = font_size * 0.8;
        
        Self {
            node,
            width,
            height,
            baseline,
        }
    }
    
    /// Create a new inline box from an inline element.
    pub fn from_element(node: NodeId, width: SubPixel, height: SubPixel) -> Self {
        Self {
            node,
            width,
            height,
            baseline: height * 0.8, // Default baseline
        }
    }
}

/// Inline layout engine for text flow.
pub struct InlineLayout {
    contexts: HashMap<NodeId, InlineContext>,
}

impl InlineLayout {
    /// Create a new inline layout engine.
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    /// Register an inline formatting context.
    pub fn add_context(&mut self, context: InlineContext) {
        self.contexts.insert(context.node, context);
    }

    /// Layout an inline container.
    fn layout_inline(&self, context: &InlineContext, constraints: Constraints) -> LayoutResult {
        let mut result = LayoutResult::new(Size::ZERO);
        let mut current_x = SubPixel::ZERO;
        let mut current_y = SubPixel::ZERO;
        let mut line_height = context.line_height;
        let mut max_width = SubPixel::ZERO;

        // Layout children horizontally with line wrapping
        for child in &context.children {
            // Check if we need to wrap
            if current_x + child.width > constraints.max_width && current_x > SubPixel::ZERO {
                // Wrap to next line
                current_x = SubPixel::ZERO;
                current_y = current_y + line_height;
                line_height = context.line_height;
            }

            // Position child
            result.push(LayoutPrimitive::SetPosition {
                node: child.node,
                point: Point {
                    x: current_x,
                    y: current_y,
                },
            });

            result.push(LayoutPrimitive::SetSize {
                node: child.node,
                size: Size {
                    w: child.width,
                    h: child.height,
                },
            });

            current_x = current_x + child.width;
            line_height = line_height.max(child.height);
            max_width = max_width.max(current_x);
        }

        // Set container size
        result.size = Size {
            w: max_width,
            h: current_y + line_height,
        };

        result
    }
}

impl Default for InlineLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine for InlineLayout {
    fn layout(&mut self, node: NodeId, constraints: Constraints) -> LayoutResult {
        if let Some(context) = self.contexts.get(&node).cloned() {
            self.layout_inline(&context, constraints)
        } else {
            LayoutResult::new(Size::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_layout_single_line() {
        let mut inline = InlineLayout::new();

        let context = InlineContext {
            node: NodeId(1),
            children: vec![
                InlineBox::from_text(NodeId(2), "Hello", SubPixel(16.0)),
                InlineBox::from_text(NodeId(3), "World", SubPixel(16.0)),
            ],
            line_height: SubPixel(20.0),
        };

        inline.add_context(context);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(200.0),
        };

        let result = inline.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
    }

    #[test]
    fn inline_layout_wrapping() {
        let mut inline = InlineLayout::new();

        let context = InlineContext {
            node: NodeId(1),
            children: vec![
                InlineBox::from_text(NodeId(2), "Very", SubPixel(16.0)),
                InlineBox::from_text(NodeId(3), "Long", SubPixel(16.0)),
                InlineBox::from_text(NodeId(4), "Text", SubPixel(16.0)),
            ],
            line_height: SubPixel(20.0),
        };

        inline.add_context(context);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(100.0), // Force wrapping
            min_height: SubPixel::ZERO,
            max_height: SubPixel(200.0),
        };

        let result = inline.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
        // Should have multiple lines
        assert!(result.size.h > SubPixel(20.0));
    }

    #[test]
    fn inline_box_from_text() {
        let inline_box = InlineBox::from_text(NodeId(1), "Test", SubPixel(16.0));
        
        assert_eq!(inline_box.node, NodeId(1));
        assert!(inline_box.width > SubPixel::ZERO);
        assert!(inline_box.height > SubPixel::ZERO);
        assert!(inline_box.baseline > SubPixel::ZERO);
    }

    #[test]
    fn inline_box_from_element() {
        let inline_box = InlineBox::from_element(NodeId(1), SubPixel(100.0), SubPixel(50.0));
        
        assert_eq!(inline_box.node, NodeId(1));
        assert_eq!(inline_box.width, SubPixel(100.0));
        assert_eq!(inline_box.height, SubPixel(50.0));
        assert_eq!(inline_box.baseline, SubPixel(40.0)); // 80% of height
    }
}
