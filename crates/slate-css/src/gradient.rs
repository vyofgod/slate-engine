//! CSS Gradient support.
//!
//! Implements linear, radial, and conic gradients.

use crate::values::Color;

/// Gradient type.
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    /// Linear gradient
    Linear(LinearGradient),
    
    /// Radial gradient
    Radial(RadialGradient),
    
    /// Conic gradient
    Conic(ConicGradient),
    
    /// Repeating linear gradient
    RepeatingLinear(LinearGradient),
    
    /// Repeating radial gradient
    RepeatingRadial(RadialGradient),
    
    /// Repeating conic gradient
    RepeatingConic(ConicGradient),
}

/// Linear gradient.
///
/// ```css
/// linear-gradient(45deg, red, blue)
/// linear-gradient(to right, red 0%, blue 100%)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    /// Gradient angle (in degrees) or direction
    pub direction: GradientDirection,
    
    /// Color stops
    pub stops: Vec<ColorStop>,
}

/// Radial gradient.
///
/// ```css
/// radial-gradient(circle, red, blue)
/// radial-gradient(ellipse at center, red 0%, blue 100%)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    /// Gradient shape
    pub shape: GradientShape,
    
    /// Gradient size
    pub size: GradientSize,
    
    /// Gradient position
    pub position: (f32, f32), // (x%, y%)
    
    /// Color stops
    pub stops: Vec<ColorStop>,
}

/// Conic gradient.
///
/// ```css
/// conic-gradient(red, yellow, lime, aqua, blue, magenta, red)
/// conic-gradient(from 45deg at 50% 50%, red, blue)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ConicGradient {
    /// Starting angle (in degrees)
    pub from_angle: f32,
    
    /// Center position
    pub position: (f32, f32), // (x%, y%)
    
    /// Color stops
    pub stops: Vec<ColorStop>,
}

/// Gradient direction.
#[derive(Debug, Clone, PartialEq)]
pub enum GradientDirection {
    /// Angle in degrees (0deg = to top, 90deg = to right)
    Angle(f32),
    
    /// To top
    ToTop,
    
    /// To bottom
    ToBottom,
    
    /// To left
    ToLeft,
    
    /// To right
    ToRight,
    
    /// To top left
    ToTopLeft,
    
    /// To top right
    ToTopRight,
    
    /// To bottom left
    ToBottomLeft,
    
    /// To bottom right
    ToBottomRight,
}

impl GradientDirection {
    /// Convert to angle in degrees.
    pub fn to_angle(&self) -> f32 {
        match self {
            GradientDirection::Angle(angle) => *angle,
            GradientDirection::ToTop => 0.0,
            GradientDirection::ToRight => 90.0,
            GradientDirection::ToBottom => 180.0,
            GradientDirection::ToLeft => 270.0,
            GradientDirection::ToTopRight => 45.0,
            GradientDirection::ToBottomRight => 135.0,
            GradientDirection::ToBottomLeft => 225.0,
            GradientDirection::ToTopLeft => 315.0,
        }
    }
}

/// Gradient shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientShape {
    /// Circle
    Circle,
    
    /// Ellipse
    Ellipse,
}

/// Gradient size.
#[derive(Debug, Clone, PartialEq)]
pub enum GradientSize {
    /// Closest side
    ClosestSide,
    
    /// Closest corner
    ClosestCorner,
    
    /// Farthest side
    FarthestSide,
    
    /// Farthest corner
    FarthestCorner,
    
    /// Explicit size (width, height)
    Explicit(f32, f32),
}

/// Color stop.
///
/// Defines a color at a specific position in the gradient.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorStop {
    /// Color
    pub color: Color,
    
    /// Position (0.0 = start, 1.0 = end)
    pub position: Option<f32>,
}

impl ColorStop {
    /// Create a new color stop.
    pub fn new(color: Color, position: Option<f32>) -> Self {
        Self { color, position }
    }
}

impl LinearGradient {
    /// Create a new linear gradient.
    pub fn new(direction: GradientDirection, stops: Vec<ColorStop>) -> Self {
        Self { direction, stops }
    }
    
    /// Interpolate color at position.
    pub fn color_at(&self, position: f32) -> Color {
        interpolate_color(&self.stops, position)
    }
}

impl RadialGradient {
    /// Create a new radial gradient.
    pub fn new(
        shape: GradientShape,
        size: GradientSize,
        position: (f32, f32),
        stops: Vec<ColorStop>,
    ) -> Self {
        Self {
            shape,
            size,
            position,
            stops,
        }
    }
    
    /// Interpolate color at position.
    pub fn color_at(&self, position: f32) -> Color {
        interpolate_color(&self.stops, position)
    }
}

impl ConicGradient {
    /// Create a new conic gradient.
    pub fn new(from_angle: f32, position: (f32, f32), stops: Vec<ColorStop>) -> Self {
        Self {
            from_angle,
            position,
            stops,
        }
    }
    
    /// Interpolate color at angle.
    pub fn color_at(&self, angle: f32) -> Color {
        // Normalize angle to 0-1 range
        let normalized = ((angle - self.from_angle) % 360.0) / 360.0;
        interpolate_color(&self.stops, normalized)
    }
}

/// Interpolate color between stops.
fn interpolate_color(stops: &[ColorStop], position: f32) -> Color {
    if stops.is_empty() {
        return Color::transparent();
    }
    
    if stops.len() == 1 {
        return stops[0].color.clone();
    }
    
    // Assign default positions if not specified
    let mut stops_with_pos: Vec<(Color, f32)> = Vec::new();
    let mut last_pos = 0.0;
    
    for (i, stop) in stops.iter().enumerate() {
        let pos = stop.position.unwrap_or_else(|| {
            if i == 0 {
                0.0
            } else if i == stops.len() - 1 {
                1.0
            } else {
                // Interpolate position
                last_pos + (1.0 - last_pos) / (stops.len() - i) as f32
            }
        });
        
        stops_with_pos.push((stop.color.clone(), pos));
        last_pos = pos;
    }
    
    // Find surrounding stops
    if position <= stops_with_pos[0].1 {
        return stops_with_pos[0].0.clone();
    }
    
    if position >= stops_with_pos[stops_with_pos.len() - 1].1 {
        return stops_with_pos[stops_with_pos.len() - 1].0.clone();
    }
    
    for i in 0..stops_with_pos.len() - 1 {
        let (color1, pos1) = &stops_with_pos[i];
        let (color2, pos2) = &stops_with_pos[i + 1];
        
        if position >= *pos1 && position <= *pos2 {
            // Interpolate between color1 and color2
            let t = (position - pos1) / (pos2 - pos1);
            return color1.interpolate(color2, t);
        }
    }
    
    stops_with_pos[0].0.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn linear_gradient() {
        let gradient = LinearGradient::new(
            GradientDirection::ToRight,
            vec![
                ColorStop::new(Color::rgb(255, 0, 0), Some(0.0)),
                ColorStop::new(Color::rgb(0, 0, 255), Some(1.0)),
            ],
        );
        
        let color_start = gradient.color_at(0.0);
        let color_mid = gradient.color_at(0.5);
        let color_end = gradient.color_at(1.0);
        
        assert_eq!(color_start, Color::rgb(255, 0, 0));
        assert_eq!(color_end, Color::rgb(0, 0, 255));
        // Mid should be purple-ish
        assert!(color_mid.r() > 0 && color_mid.b() > 0);
    }
    
    #[test]
    fn gradient_direction_to_angle() {
        assert_eq!(GradientDirection::ToTop.to_angle(), 0.0);
        assert_eq!(GradientDirection::ToRight.to_angle(), 90.0);
        assert_eq!(GradientDirection::ToBottom.to_angle(), 180.0);
        assert_eq!(GradientDirection::ToLeft.to_angle(), 270.0);
    }
}
