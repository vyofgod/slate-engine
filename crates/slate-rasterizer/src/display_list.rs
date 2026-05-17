//! Display list for rendering.

use slate_ais::{Point, Rect, Rgba8};

/// A display list command.
#[derive(Debug, Clone)]
pub enum DisplayCommand {
    /// Fill a rectangle with a solid color.
    FillRect { rect: Rect, color: Rgba8 },

    /// Stroke a rectangle outline.
    StrokeRect {
        rect: Rect,
        color: Rgba8,
        width: f32,
    },

    /// Draw text at a position.
    DrawText {
        text: String,
        position: Point,
        color: Rgba8,
        font_size: f32,
    },

    /// Push a clip rectangle.
    PushClip { rect: Rect },

    /// Pop the clip rectangle.
    PopClip,

    /// Push opacity.
    PushOpacity { opacity: f32 },

    /// Pop opacity.
    PopOpacity,
}

/// A display list for rendering.
#[derive(Debug, Clone, Default)]
pub struct DisplayList {
    pub commands: Vec<DisplayCommand>,
}

impl DisplayList {
    /// Create a new empty display list.
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Add a command to the display list.
    pub fn push(&mut self, command: DisplayCommand) {
        self.commands.push(command);
    }

    /// Clear the display list.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Get the number of commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if the display list is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
