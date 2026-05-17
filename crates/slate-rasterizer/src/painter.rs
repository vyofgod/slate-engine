//! Painter for executing display list commands.

use super::{DisplayCommand, DisplayList, FrameBuffer};
use slate_ais::{Point, Rect, Rgba8, Size};

/// Painter executes display list commands on a frame buffer.
pub struct Painter {
    clip_stack: Vec<Rect>,
    opacity_stack: Vec<f32>,
}

impl Painter {
    /// Create a new painter.
    pub fn new() -> Self {
        Self {
            clip_stack: Vec::new(),
            opacity_stack: vec![1.0],
        }
    }

    /// Paint a display list to a frame buffer.
    pub fn paint(&mut self, display_list: &DisplayList, frame_buffer: &mut FrameBuffer) {
        for command in &display_list.commands {
            self.execute_command(command, frame_buffer);
        }
    }

    /// Execute a single display command.
    fn execute_command(&mut self, command: &DisplayCommand, fb: &mut FrameBuffer) {
        match command {
            DisplayCommand::FillRect { rect, color } => {
                let color = self.apply_opacity(*color);
                let rect = self.apply_clip(*rect);
                fb.fill_rect(rect, color);
            }

            DisplayCommand::StrokeRect { rect, color, width } => {
                let color = self.apply_opacity(*color);
                let rect = self.apply_clip(*rect);
                fb.stroke_rect(rect, color, *width);
            }

            DisplayCommand::DrawText {
                text,
                position,
                color,
                font_size,
            } => {
                // TODO: Implement text rendering
                // For now, just draw a placeholder rectangle
                let color = self.apply_opacity(*color);
                let width = text.len() as f32 * font_size * 0.6;
                let rect = Rect {
                    origin: *position,
                    size: Size { w: width.into(), h: (*font_size).into() },
                };
                fb.fill_rect(rect, color);
            }

            DisplayCommand::PushClip { rect } => {
                self.clip_stack.push(*rect);
            }

            DisplayCommand::PopClip => {
                self.clip_stack.pop();
            }

            DisplayCommand::PushOpacity { opacity } => {
                let current = *self.opacity_stack.last().unwrap_or(&1.0);
                self.opacity_stack.push(current * opacity);
            }

            DisplayCommand::PopOpacity => {
                if self.opacity_stack.len() > 1 {
                    self.opacity_stack.pop();
                }
            }
        }
    }

    /// Apply current opacity to a color.
    fn apply_opacity(&self, mut color: Rgba8) -> Rgba8 {
        let opacity = *self.opacity_stack.last().unwrap_or(&1.0);
        color.a = ((color.a as f32) * opacity) as u8;
        color
    }

    /// Apply current clip to a rectangle.
    fn apply_clip(&self, rect: Rect) -> Rect {
        if let Some(clip) = self.clip_stack.last() {
            // Intersect with clip rect
            let x1 = rect.origin.x.max(clip.origin.x);
            let y1 = rect.origin.y.max(clip.origin.y);
            let x2 = (rect.origin.x + rect.size.w).min(clip.origin.x + clip.size.w);
            let y2 = (rect.origin.y + rect.size.h).min(clip.origin.y + clip.size.h);

            Rect {
                origin: Point { x: x1, y: y1 },
                size: Size { 
                    w: (x2 - x1).max(0.into()), 
                    h: (y2 - y1).max(0.into()) 
                },
            }
        } else {
            rect
        }
    }
}

impl Default for Painter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paint_display_list() {
        let mut painter = Painter::new();
        let mut fb = FrameBuffer::new(800, 600);
        let mut dl = DisplayList::new();

        dl.push(DisplayCommand::FillRect {
            rect: Rect {
                origin: Point { x: 10.into(), y: 10.into() },
                size: Size { w: 100.into(), h: 100.into() },
            },
            color: Rgba8::rgb(255, 0, 0),
        });

        painter.paint(&dl, &mut fb);

        let pixel = fb.get_pixel(50, 50);
        assert_eq!(pixel.r, 255);
    }

    #[test]
    fn opacity_stack() {
        let mut painter = Painter::new();

        painter.opacity_stack.push(0.5);
        let color = painter.apply_opacity(Rgba8::rgb(255, 0, 0));
        assert_eq!(color.a, 127);
    }
}
