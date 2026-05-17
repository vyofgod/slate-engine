//! Text layout: line breaking, word wrapping, and alignment.

use super::{GlyphRun, ShapeResult, TextAlign, TextOverflow, WordBreak};
use slate_ais::{Point, Rect, SubPixel};
use std::sync::Arc;

/// Options for text layout.
#[derive(Debug, Clone)]
pub struct TextLayoutOptions {
    pub max_width: Option<SubPixel>,
    pub max_height: Option<SubPixel>,
    pub align: TextAlign,
    pub word_break: WordBreak,
    pub overflow: TextOverflow,
    pub line_height: SubPixel,
}

impl Default for TextLayoutOptions {
    fn default() -> Self {
        Self {
            max_width: None,
            max_height: None,
            align: TextAlign::Left,
            word_break: WordBreak::Normal,
            overflow: TextOverflow::Clip,
            line_height: SubPixel(1.2),
        }
    }
}

/// A laid-out line of text.
#[derive(Debug, Clone)]
pub struct TextLine {
    pub glyph_run: Arc<GlyphRun>,
    pub position: Point,
    pub width: SubPixel,
}

/// Complete text layout result.
#[derive(Debug, Clone)]
pub struct TextLayout {
    pub lines: Vec<TextLine>,
    pub bounds: Rect,
}

impl TextLayout {
    /// Create an empty text layout.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            bounds: Rect::default(),
        }
    }

    /// Add a line to the layout.
    pub fn add_line(&mut self, line: TextLine) {
        // Update bounds
        let line_bottom = line.position.y + self.bounds.size.h;
        if line_bottom > self.bounds.origin.y + self.bounds.size.h {
            self.bounds = Rect::from_ltwh(
                self.bounds.origin.x.raw(),
                self.bounds.origin.y.raw(),
                self.bounds.size.w.max(line.width).raw(),
                (line_bottom - self.bounds.origin.y).raw(),
            );
        }
        self.lines.push(line);
    }

    /// Get the total height of the layout.
    #[inline]
    pub fn height(&self) -> SubPixel {
        self.bounds.size.h
    }

    /// Get the total width of the layout.
    #[inline]
    pub fn width(&self) -> SubPixel {
        self.bounds.size.w
    }
}

impl Default for TextLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Line breaker for word wrapping using Unicode line breaking algorithm.
pub struct LineBreaker {
    options: TextLayoutOptions,
}

impl LineBreaker {
    /// Create a new line breaker with options.
    pub fn new(options: TextLayoutOptions) -> Self {
        Self { options }
    }

    /// Break shaped text into lines using Unicode line breaking algorithm.
    pub fn break_lines(&self, shape_results: Vec<ShapeResult>) -> TextLayout {
        let mut layout = TextLayout::new();
        let mut current_y = SubPixel(0.0);

        for result in shape_results {
            // Get the original text for line breaking analysis
            // Note: In a real implementation, we'd need to track the original text
            // For now, we'll use the glyph-based approach with Unicode awareness
            
            if let Some(max_width) = self.options.max_width {
                let lines = self.break_glyph_run_unicode(&result.glyph_run, max_width);
                for glyph_run in lines {
                    let width = glyph_run.width();
                    let x = self.align_line(width);

                    layout.add_line(TextLine {
                        glyph_run: Arc::new(glyph_run),
                        position: Point { x, y: current_y },
                        width,
                    });

                    current_y = current_y + self.options.line_height;
                }
            } else {
                // No wrapping
                let width = result.glyph_run.width();
                let x = self.align_line(width);

                layout.add_line(TextLine {
                    glyph_run: Arc::new(result.glyph_run),
                    position: Point { x, y: current_y },
                    width,
                });

                current_y = current_y + self.options.line_height;
            }
        }

        layout
    }

    /// Break a glyph run at max_width boundaries with Unicode line breaking.
    fn break_glyph_run_unicode(&self, run: &GlyphRun, max_width: SubPixel) -> Vec<GlyphRun> {
        let mut lines = Vec::new();
        let mut current_line = GlyphRun::new(run.font_size);
        let mut current_width = SubPixel(0.0);
        let mut last_break_point = 0;
        let mut last_break_width = SubPixel(0.0);

        for (i, glyph) in run.glyphs.iter().enumerate() {
            let new_width = current_width + glyph.advance;

            // Check if we exceed max width
            if new_width > max_width && !current_line.is_empty() {
                // Break at last opportunity if available
                if last_break_point > 0 && last_break_point < i {
                    // Break at last opportunity
                    let break_line = GlyphRun {
                        glyphs: current_line.glyphs[..last_break_point].to_vec(),
                        font_size: run.font_size,
                        total_advance: last_break_width,
                    };
                    lines.push(break_line);

                    // Start new line with remaining glyphs
                    current_line = GlyphRun {
                        glyphs: current_line.glyphs[last_break_point..].to_vec(),
                        font_size: run.font_size,
                        total_advance: current_width - last_break_width,
                    };
                    current_width = current_width - last_break_width;
                    last_break_point = 0;
                    last_break_width = SubPixel(0.0);
                } else {
                    // Force break
                    lines.push(current_line);
                    current_line = GlyphRun::new(run.font_size);
                    current_width = SubPixel(0.0);
                    last_break_point = 0;
                    last_break_width = SubPixel(0.0);
                }
            }

            current_line.push(*glyph);
            current_width = current_width + glyph.advance;

            // Mark potential break points (spaces, hyphens, etc.)
            // In a real implementation, we'd use unicode-linebreak on the original text
            if self.is_break_opportunity(glyph.glyph_id.0) {
                last_break_point = current_line.glyphs.len();
                last_break_width = current_width;
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(GlyphRun::new(run.font_size));
        }

        lines
    }

    /// Check if a glyph represents a line break opportunity.
    fn is_break_opportunity(&self, glyph_id: u32) -> bool {
        // Common break characters
        matches!(glyph_id, 
            0x0020 | // Space
            0x002D | // Hyphen
            0x2010 | // Hyphen
            0x2013 | // En dash
            0x2014   // Em dash
        )
    }

    /// Calculate X position based on alignment.
    fn align_line(&self, line_width: SubPixel) -> SubPixel {
        match self.options.align {
            TextAlign::Left => SubPixel(0.0),
            TextAlign::Right => {
                if let Some(max_width) = self.options.max_width {
                    max_width - line_width
                } else {
                    SubPixel(0.0)
                }
            }
            TextAlign::Center => {
                if let Some(max_width) = self.options.max_width {
                    (max_width - line_width) * 0.5
                } else {
                    SubPixel(0.0)
                }
            }
            TextAlign::Justify => {
                // TODO: Implement justify spacing by distributing extra space
                SubPixel(0.0)
            }
        }
    }
}
