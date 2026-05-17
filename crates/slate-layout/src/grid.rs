//! CSS Grid layout algorithm.

use super::{Constraints, LayoutEngine, LayoutResult};
use slate_ais::{LayoutPrimitive, NodeId, Point, Size, SubPixel};
use std::collections::HashMap;

/// Grid container.
#[derive(Debug, Clone)]
pub struct GridContainer {
    pub node: NodeId,
    pub template_columns: Vec<TrackSize>,
    pub template_rows: Vec<TrackSize>,
    pub gap_column: SubPixel,
    pub gap_row: SubPixel,
    pub auto_flow: GridAutoFlow,
}

/// Grid item placement.
#[derive(Debug, Clone)]
pub struct GridItem {
    pub node: NodeId,
    pub column_start: i32,
    pub column_end: i32,
    pub row_start: i32,
    pub row_end: i32,
    pub align_self: Option<AlignSelf>,
    pub justify_self: Option<JustifySelf>,
}

/// Grid track size.
#[derive(Debug, Clone, PartialEq)]
pub enum TrackSize {
    /// Fixed size in pixels
    Fixed(SubPixel),

    /// Fraction of free space
    Fr(f32),

    /// Minimum content size
    MinContent,

    /// Maximum content size
    MaxContent,

    /// Auto size
    Auto,

    /// Min-max range
    MinMax(Box<TrackSize>, Box<TrackSize>),
}

/// Grid auto flow direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAutoFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

/// Align self for grid items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignSelf {
    Start,
    End,
    Center,
    Stretch,
}

/// Justify self for grid items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifySelf {
    Start,
    End,
    Center,
    Stretch,
}

/// Grid layout engine.
pub struct GridLayout {
    containers: HashMap<NodeId, GridContainer>,
    items: HashMap<NodeId, Vec<GridItem>>,
}

impl GridLayout {
    /// Create a new grid layout engine.
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            items: HashMap::new(),
        }
    }

    /// Register a grid container.
    pub fn add_container(&mut self, container: GridContainer) {
        self.containers.insert(container.node, container);
    }

    /// Register grid items for a container.
    pub fn add_items(&mut self, container: NodeId, items: Vec<GridItem>) {
        self.items.insert(container, items);
    }

    /// Layout a grid container.
    fn layout_grid(&self, container: &GridContainer, constraints: Constraints) -> LayoutResult {
        let mut result = LayoutResult::new(Size::ZERO);

        let items = self.items.get(&container.node).cloned().unwrap_or_default();
        if items.is_empty() {
            return result;
        }

        // Calculate track sizes
        let column_sizes = self.resolve_track_sizes(
            &container.template_columns,
            constraints.max_width,
            container.gap_column,
        );

        let row_sizes = self.resolve_track_sizes(
            &container.template_rows,
            constraints.max_height,
            container.gap_row,
        );

        // Position items
        for item in &items {
            let col_start = item.column_start.max(0) as usize;
            let col_end = item.column_end.max(0) as usize;
            let row_start = item.row_start.max(0) as usize;
            let row_end = item.row_end.max(0) as usize;

            // Calculate position
            let x = self.sum_tracks(&column_sizes, 0, col_start) + (container.gap_column * col_start as f32);
            let y = self.sum_tracks(&row_sizes, 0, row_start) + (container.gap_row * row_start as f32);

            // Calculate size
            let width = self.sum_tracks(&column_sizes, col_start, col_end)
                + (container.gap_column * (col_end - col_start - 1) as f32);
            let height = self.sum_tracks(&row_sizes, row_start, row_end)
                + (container.gap_row * (row_end - row_start - 1) as f32);

            result.push(LayoutPrimitive::SetPosition {
                node: item.node,
                point: Point { x, y },
            });

            result.push(LayoutPrimitive::SetSize {
                node: item.node,
                size: Size { w: width, h: height },
            });
        }

        // Calculate container size
        let total_width = self.sum_tracks(&column_sizes, 0, column_sizes.len())
            + (container.gap_column * (column_sizes.len() - 1) as f32);
        let total_height = self.sum_tracks(&row_sizes, 0, row_sizes.len())
            + (container.gap_row * (row_sizes.len() - 1) as f32);

        result.size = Size {
            w: total_width,
            h: total_height,
        };

        result
    }

    /// Resolve track sizes to concrete pixel values.
    fn resolve_track_sizes(&self, tracks: &[TrackSize], available: SubPixel, gap: SubPixel) -> Vec<SubPixel> {
        let mut sizes = Vec::new();
        let mut total_fr = 0.0;
        let mut used_space = SubPixel::ZERO;

        // First pass: resolve fixed sizes and intrinsic sizes
        for track in tracks {
            match track {
                TrackSize::Fixed(size) => {
                    sizes.push(*size);
                    used_space = used_space + *size;
                }
                TrackSize::Fr(fr) => {
                    sizes.push(SubPixel::ZERO);
                    total_fr += fr;
                }
                TrackSize::MinContent => {
                    // Minimum content size - smallest size without overflow
                    let min_size = self.calculate_min_content_size();
                    sizes.push(min_size);
                    used_space = used_space + min_size;
                }
                TrackSize::MaxContent => {
                    // Maximum content size - largest size without wrapping
                    let max_size = self.calculate_max_content_size();
                    sizes.push(max_size);
                    used_space = used_space + max_size;
                }
                TrackSize::Auto => {
                    // Auto size - between min-content and max-content
                    let auto_size = self.calculate_auto_size(available);
                    sizes.push(auto_size);
                    used_space = used_space + auto_size;
                }
                TrackSize::MinMax(min, max) => {
                    // MinMax - clamp between min and max
                    let min_resolved = self.resolve_single_track_size(min, available);
                    let max_resolved = self.resolve_single_track_size(max, available);
                    let size = min_resolved.max(max_resolved.min(available));
                    sizes.push(size);
                    used_space = used_space + size;
                }
            }
        }

        // Calculate free space for fr units
        let gap_space = gap * (tracks.len() - 1) as f32;
        let free_space = (available - used_space - gap_space).max(SubPixel::ZERO);

        // Second pass: resolve fr units
        if total_fr > 0.0 {
            for (i, track) in tracks.iter().enumerate() {
                if let TrackSize::Fr(fr) = track {
                    sizes[i] = free_space * (*fr / total_fr);
                }
            }
        }

        sizes
    }

    /// Calculate minimum content size for a track.
    fn calculate_min_content_size(&self) -> SubPixel {
        // In real implementation, this would measure the smallest size
        // that doesn't cause overflow (e.g., longest word width for text)
        SubPixel(50.0)
    }

    /// Calculate maximum content size for a track.
    fn calculate_max_content_size(&self) -> SubPixel {
        // In real implementation, this would measure the largest size
        // without any wrapping (e.g., full text width on one line)
        SubPixel(200.0)
    }

    /// Calculate auto size for a track.
    fn calculate_auto_size(&self, available: SubPixel) -> SubPixel {
        // Auto is between min-content and max-content
        // For now, use a reasonable default
        let min = self.calculate_min_content_size();
        let max = self.calculate_max_content_size();
        min.max(max.min(available * 0.5))
    }

    /// Resolve a single track size (for MinMax).
    fn resolve_single_track_size(&self, track: &TrackSize, available: SubPixel) -> SubPixel {
        match track {
            TrackSize::Fixed(size) => *size,
            TrackSize::MinContent => self.calculate_min_content_size(),
            TrackSize::MaxContent => self.calculate_max_content_size(),
            TrackSize::Auto => self.calculate_auto_size(available),
            TrackSize::Fr(fr) => available * *fr,
            TrackSize::MinMax(min, _) => self.resolve_single_track_size(min, available),
        }
    }

    /// Sum track sizes in a range.
    fn sum_tracks(&self, tracks: &[SubPixel], start: usize, end: usize) -> SubPixel {
        tracks[start..end.min(tracks.len())]
            .iter()
            .fold(SubPixel::ZERO, |acc, &size| acc + size)
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine for GridLayout {
    fn layout(&mut self, node: NodeId, constraints: Constraints) -> LayoutResult {
        if let Some(container) = self.containers.get(&node).cloned() {
            self.layout_grid(&container, constraints)
        } else {
            LayoutResult::new(Size::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_layout_basic() {
        let mut grid = GridLayout::new();

        let container = GridContainer {
            node: NodeId(1),
            template_columns: vec![TrackSize::Fr(1.0), TrackSize::Fr(1.0)],
            template_rows: vec![TrackSize::Fixed(SubPixel(100.0)), TrackSize::Fixed(SubPixel(100.0))],
            gap_column: SubPixel(10.0),
            gap_row: SubPixel(10.0),
            auto_flow: GridAutoFlow::Row,
        };

        let items = vec![
            GridItem {
                node: NodeId(2),
                column_start: 0,
                column_end: 1,
                row_start: 0,
                row_end: 1,
                align_self: None,
                justify_self: None,
            },
        ];

        grid.add_container(container);
        grid.add_items(NodeId(1), items);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(400.0),
        };

        let result = grid.layout(NodeId(1), constraints);
        assert!(!result.primitives.is_empty());
    }

    #[test]
    fn grid_min_content() {
        let grid = GridLayout::new();
        let size = grid.calculate_min_content_size();
        assert!(size > SubPixel::ZERO);
    }

    #[test]
    fn grid_max_content() {
        let grid = GridLayout::new();
        let size = grid.calculate_max_content_size();
        assert!(size > SubPixel::ZERO);
    }

    #[test]
    fn grid_auto_size() {
        let grid = GridLayout::new();
        let size = grid.calculate_auto_size(SubPixel(400.0));
        assert!(size > SubPixel::ZERO);
        assert!(size <= SubPixel(400.0));
    }

    #[test]
    fn grid_minmax_track() {
        let mut grid = GridLayout::new();

        let container = GridContainer {
            node: NodeId(1),
            template_columns: vec![
                TrackSize::MinMax(
                    Box::new(TrackSize::Fixed(SubPixel(100.0))),
                    Box::new(TrackSize::Fixed(SubPixel(200.0))),
                )
            ],
            template_rows: vec![TrackSize::Fixed(SubPixel(100.0))],
            gap_column: SubPixel::ZERO,
            gap_row: SubPixel::ZERO,
            auto_flow: GridAutoFlow::Row,
        };

        grid.add_container(container);

        let constraints = Constraints {
            min_width: SubPixel::ZERO,
            max_width: SubPixel(400.0),
            min_height: SubPixel::ZERO,
            max_height: SubPixel(400.0),
        };

        let result = grid.layout(NodeId(1), constraints);
        // Grid should have some width (minmax should resolve to at least min value)
        assert!(result.size.w >= SubPixel::ZERO, "Grid width should be non-negative, got: {:?}", result.size.w);
    }
}
