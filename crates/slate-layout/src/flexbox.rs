//! CSS Flexbox layout algorithm.

use super::{Constraints, LayoutEngine, LayoutResult};
use slate_ais::{LayoutPrimitive, NodeId, Point, Size, SubPixel};
use std::collections::HashMap;

/// Flexbox container.
#[derive(Debug, Clone)]
pub struct FlexContainer {
    pub node: NodeId,
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap: SubPixel,
}

/// Flexbox item.
#[derive(Debug, Clone)]
pub struct FlexItem {
    pub node: NodeId,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: SubPixel,
    pub align_self: Option<AlignItems>,
    pub order: i32,
}

/// Flex direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

/// Flex wrap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

/// Justify content (main axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Align items (cross axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

/// Align content (multi-line cross axis alignment).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    Stretch,
}

/// Flexbox layout engine.
pub struct FlexLayout {
    containers: HashMap<NodeId, FlexContainer>,
    items: HashMap<NodeId, Vec<FlexItem>>,
}

impl FlexLayout {
    /// Create a new flexbox layout engine.
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            items: HashMap::new(),
        }
    }

    /// Register a flex container.
    pub fn add_container(&mut self, container: FlexContainer) {
        self.containers.insert(container.node, container);
    }

    /// Register flex items for a container.
    pub fn add_items(&mut self, container: NodeId, items: Vec<FlexItem>) {
        self.items.insert(container, items);
    }

    /// Layout a flex container.
    fn layout_flex(&self, container: &FlexContainer, constraints: Constraints) -> LayoutResult {
        let mut result = LayoutResult::new(Size::ZERO);

        let items = self.items.get(&container.node).cloned().unwrap_or_default();
        if items.is_empty() {
            return result;
        }

        // Determine main and cross axis
        let is_row = matches!(container.direction, FlexDirection::Row | FlexDirection::RowReverse);
        let main_size = if is_row {
            constraints.max_width
        } else {
            constraints.max_height
        };

        // Calculate flex basis for each item
        let mut total_basis = SubPixel::ZERO;
        let mut total_grow = 0.0;
        let mut total_shrink = 0.0;

        for item in &items {
            total_basis = total_basis + item.flex_basis;
            total_grow += item.flex_grow;
            total_shrink += item.flex_shrink;
        }

        // Calculate free space
        let free_space = main_size - total_basis - (container.gap * (items.len() as f32 - 1.0));

        // Distribute free space
        let mut item_sizes = Vec::new();
        for item in &items {
            let size = if free_space > SubPixel::ZERO && total_grow > 0.0 {
                // Grow
                item.flex_basis + (free_space * (item.flex_grow / total_grow))
            } else if free_space < SubPixel::ZERO && total_shrink > 0.0 {
                // Shrink
                item.flex_basis + (free_space * (item.flex_shrink / total_shrink))
            } else {
                item.flex_basis
            };
            item_sizes.push(size);
        }

        // Calculate baseline alignment if needed
        let baselines = if container.align_items == AlignItems::Baseline {
            self.calculate_baselines(&items, &item_sizes)
        } else {
            vec![SubPixel::ZERO; items.len()]
        };

        // Position items
        let mut main_pos = self.calculate_justify_start(
            &container.justify_content,
            main_size,
            total_basis,
            items.len(),
        );

        for (idx, (item, &size)) in items.iter().zip(&item_sizes).enumerate() {
            let cross_offset = self.calculate_cross_offset(
                &container.align_items,
                item.align_self.as_ref(),
                constraints.max_height,
                size,
                baselines[idx],
            );

            let position = if is_row {
                Point {
                    x: main_pos,
                    y: cross_offset,
                }
            } else {
                Point {
                    x: cross_offset,
                    y: main_pos,
                }
            };

            result.push(LayoutPrimitive::SetPosition {
                node: item.node,
                point: position,
            });

            let item_size = if is_row {
                Size {
                    w: size,
                    h: constraints.max_height,
                }
            } else {
                Size {
                    w: constraints.max_width,
                    h: size,
                }
            };

            result.push(LayoutPrimitive::SetSize {
                node: item.node,
                size: item_size,
            });

            main_pos = main_pos + size + container.gap;
        }

        // Update container size
        result.size = if is_row {
            Size {
                w: main_pos - container.gap,
                h: constraints.max_height,
            }
        } else {
            Size {
                w: constraints.max_width,
                h: main_pos - container.gap,
            }
        };

        result
    }

    /// Calculate baselines for all items.
    fn calculate_baselines(&self, items: &[FlexItem], sizes: &[SubPixel]) -> Vec<SubPixel> {
        let mut baselines = Vec::new();
        
        for (item, &size) in items.iter().zip(sizes) {
            // Calculate baseline for this item
            // In real implementation, this would query the actual content
            // For now, use a reasonable approximation (80% of height)
            let baseline = self.calculate_item_baseline(item, size);
            baselines.push(baseline);
        }
        
        baselines
    }

    /// Calculate baseline for a single item.
    fn calculate_item_baseline(&self, _item: &FlexItem, size: SubPixel) -> SubPixel {
        // In real implementation, this would:
        // 1. Check if item has text content
        // 2. Get the first line's baseline
        // 3. For replaced elements (images), use bottom edge
        // 4. For other elements, use the baseline of the first line box
        
        // For now, approximate as 80% of the height (typical for text)
        size * 0.8
    }

    /// Calculate cross-axis offset for an item.
    fn calculate_cross_offset(
        &self,
        container_align: &AlignItems,
        item_align: Option<&AlignItems>,
        container_cross_size: SubPixel,
        item_cross_size: SubPixel,
        baseline: SubPixel,
    ) -> SubPixel {
        let align = item_align.unwrap_or(container_align);
        
        match align {
            AlignItems::FlexStart => SubPixel::ZERO,
            AlignItems::FlexEnd => container_cross_size - item_cross_size,
            AlignItems::Center => (container_cross_size - item_cross_size) * 0.5,
            AlignItems::Baseline => {
                // Align items by their baselines
                // In real implementation, find the max baseline and align to it
                baseline
            }
            AlignItems::Stretch => SubPixel::ZERO,
        }
    }

    /// Calculate starting position for justify-content.
    fn calculate_justify_start(
        &self,
        justify: &JustifyContent,
        container_size: SubPixel,
        content_size: SubPixel,
        item_count: usize,
    ) -> SubPixel {
        match justify {
            JustifyContent::FlexStart => SubPixel::ZERO,
            JustifyContent::FlexEnd => container_size - content_size,
            JustifyContent::Center => (container_size - content_size) * 0.5,
            JustifyContent::SpaceBetween => SubPixel::ZERO,
            JustifyContent::SpaceAround => {
                let space = (container_size - content_size) / (item_count as f32 * 2.0);
                space
            }
            JustifyContent::SpaceEvenly => {
                let space = (container_size - content_size) / (item_count as f32 + 1.0);
                space
            }
        }
    }
}

impl Default for FlexLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine for FlexLayout {
    fn layout(&mut self, node: NodeId, constraints: Constraints) -> LayoutResult {
        if let Some(container) = self.containers.get(&node).cloned() {
            self.layout_flex(&container, constraints)
        } else {
            LayoutResult::new(Size::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_row_layout() {
        let mut flex = FlexLayout::new();

        let container = FlexContainer {
            node: NodeId(1),
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            gap: SubPixel(10.0),
        };

        let items = vec![
            FlexItem {
                node: NodeId(2),
                flex_grow: 1.0,
                flex_shrink: 1.0,
                flex_basis: SubPixel(100.0),
                align_self: None,
                order: 0,
            },
            FlexItem {
                node: NodeId(3),
                flex_grow: 1.0,
                flex_shrink: 1.0,
                flex_basis: SubPixel(100.0),
                align_self: None,
                order: 0,
            },
        ];

        flex.add_container(container);
        flex.add_items(NodeId(1), items);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(200.0),
        };

        let result = flex.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
    }

    #[test]
    fn flex_baseline_alignment() {
        let mut flex = FlexLayout::new();

        let container = FlexContainer {
            node: NodeId(1),
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Baseline,
            align_content: AlignContent::Stretch,
            gap: SubPixel(10.0),
        };

        let items = vec![
            FlexItem {
                node: NodeId(2),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                flex_basis: SubPixel(100.0),
                align_self: None,
                order: 0,
            },
        ];

        flex.add_container(container);
        flex.add_items(NodeId(1), items);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(200.0),
        };

        let result = flex.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
    }

    #[test]
    fn flex_calculate_baseline() {
        let flex = FlexLayout::new();
        let item = FlexItem {
            node: NodeId(1),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            flex_basis: SubPixel(100.0),
            align_self: None,
            order: 0,
        };
        
        let baseline = flex.calculate_item_baseline(&item, SubPixel(100.0));
        assert!(baseline > SubPixel::ZERO);
        assert!(baseline <= SubPixel(100.0));
    }

    #[test]
    fn flex_cross_offset_center() {
        let flex = FlexLayout::new();
        let offset = flex.calculate_cross_offset(
            &AlignItems::Center,
            None,
            SubPixel(200.0),
            SubPixel(100.0),
            SubPixel(80.0),
        );
        assert_eq!(offset, SubPixel(50.0));
    }

    #[test]
    fn flex_cross_offset_baseline() {
        let flex = FlexLayout::new();
        let offset = flex.calculate_cross_offset(
            &AlignItems::Baseline,
            None,
            SubPixel(200.0),
            SubPixel(100.0),
            SubPixel(80.0),
        );
        assert_eq!(offset, SubPixel(80.0));
    }
}
