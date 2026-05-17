//! CSS Animation and Transition support.

use std::collections::HashMap;

/// CSS Animation.
///
/// ```css
/// @keyframes slide {
///   from { left: 0; }
///   to { left: 100px; }
/// }
///
/// .element {
///   animation: slide 2s ease-in-out infinite;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Animation {
    /// Animation name
    pub name: String,
    
    /// Duration in milliseconds
    pub duration: f32,
    
    /// Timing function
    pub timing_function: TimingFunction,
    
    /// Delay in milliseconds
    pub delay: f32,
    
    /// Iteration count (None = infinite)
    pub iteration_count: Option<f32>,
    
    /// Direction
    pub direction: AnimationDirection,
    
    /// Fill mode
    pub fill_mode: AnimationFillMode,
    
    /// Play state
    pub play_state: AnimationPlayState,
    
    /// Keyframes
    pub keyframes: Vec<Keyframe>,
}

/// CSS Transition.
///
/// ```css
/// .element {
///   transition: opacity 0.3s ease-in-out;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Transition {
    /// Property name
    pub property: String,
    
    /// Duration in milliseconds
    pub duration: f32,
    
    /// Timing function
    pub timing_function: TimingFunction,
    
    /// Delay in milliseconds
    pub delay: f32,
}

/// Keyframe in an animation.
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Offset (0.0 = 0%, 1.0 = 100%)
    pub offset: f32,
    
    /// CSS properties at this keyframe
    pub properties: HashMap<String, String>,
    
    /// Timing function for this segment
    pub timing_function: Option<TimingFunction>,
}

/// Timing function (easing).
#[derive(Debug, Clone, PartialEq)]
pub enum TimingFunction {
    /// Linear
    Linear,
    
    /// Ease (default)
    Ease,
    
    /// Ease-in
    EaseIn,
    
    /// Ease-out
    EaseOut,
    
    /// Ease-in-out
    EaseInOut,
    
    /// Cubic bezier
    CubicBezier(f32, f32, f32, f32),
    
    /// Steps
    Steps(u32, StepPosition),
}

/// Step position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepPosition {
    Start,
    End,
}

/// Animation direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    /// Normal (forward)
    Normal,
    
    /// Reverse (backward)
    Reverse,
    
    /// Alternate (forward, then backward)
    Alternate,
    
    /// Alternate reverse (backward, then forward)
    AlternateReverse,
}

/// Animation fill mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationFillMode {
    /// No fill
    None,
    
    /// Forwards (keep final state)
    Forwards,
    
    /// Backwards (apply first keyframe before start)
    Backwards,
    
    /// Both
    Both,
}

/// Animation play state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPlayState {
    /// Running
    Running,
    
    /// Paused
    Paused,
}

impl TimingFunction {
    /// Evaluate timing function at time t (0.0 to 1.0).
    pub fn evaluate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        
        match self {
            TimingFunction::Linear => t,
            
            TimingFunction::Ease => {
                // cubic-bezier(0.25, 0.1, 0.25, 1.0)
                Self::cubic_bezier(t, 0.25, 0.1, 0.25, 1.0)
            }
            
            TimingFunction::EaseIn => {
                // cubic-bezier(0.42, 0, 1.0, 1.0)
                Self::cubic_bezier(t, 0.42, 0.0, 1.0, 1.0)
            }
            
            TimingFunction::EaseOut => {
                // cubic-bezier(0, 0, 0.58, 1.0)
                Self::cubic_bezier(t, 0.0, 0.0, 0.58, 1.0)
            }
            
            TimingFunction::EaseInOut => {
                // cubic-bezier(0.42, 0, 0.58, 1.0)
                Self::cubic_bezier(t, 0.42, 0.0, 0.58, 1.0)
            }
            
            TimingFunction::CubicBezier(x1, y1, x2, y2) => {
                Self::cubic_bezier(t, *x1, *y1, *x2, *y2)
            }
            
            TimingFunction::Steps(steps, position) => {
                let steps = *steps as f32;
                match position {
                    StepPosition::Start => ((t * steps).ceil() / steps).min(1.0),
                    StepPosition::End => ((t * steps).floor() / steps).min(1.0),
                }
            }
        }
    }
    
    /// Cubic bezier interpolation.
    fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        // Simplified cubic bezier (should use Newton-Raphson for accuracy)
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
    }
}

impl Animation {
    /// Create a new animation.
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            name,
            duration,
            timing_function: TimingFunction::Ease,
            delay: 0.0,
            iteration_count: Some(1.0),
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::None,
            play_state: AnimationPlayState::Running,
            keyframes: Vec::new(),
        }
    }
    
    /// Add a keyframe.
    pub fn add_keyframe(&mut self, keyframe: Keyframe) {
        self.keyframes.push(keyframe);
        // Sort by offset
        self.keyframes.sort_by(|a, b| a.offset.partial_cmp(&b.offset).unwrap());
    }
    
    /// Get interpolated properties at time t (0.0 to 1.0).
    pub fn properties_at(&self, t: f32) -> HashMap<String, String> {
        if self.keyframes.is_empty() {
            return HashMap::new();
        }
        
        // Apply timing function
        let eased_t = self.timing_function.evaluate(t);
        
        // Find surrounding keyframes
        let mut prev_keyframe = &self.keyframes[0];
        let mut next_keyframe = &self.keyframes[self.keyframes.len() - 1];
        
        for i in 0..self.keyframes.len() - 1 {
            if eased_t >= self.keyframes[i].offset && eased_t <= self.keyframes[i + 1].offset {
                prev_keyframe = &self.keyframes[i];
                next_keyframe = &self.keyframes[i + 1];
                break;
            }
        }
        
        // Interpolate properties
        let segment_t = if next_keyframe.offset > prev_keyframe.offset {
            (eased_t - prev_keyframe.offset) / (next_keyframe.offset - prev_keyframe.offset)
        } else {
            0.0
        };
        
        // For now, just return the closest keyframe's properties
        // TODO: Implement proper property interpolation
        if segment_t < 0.5 {
            prev_keyframe.properties.clone()
        } else {
            next_keyframe.properties.clone()
        }
    }
}

impl Transition {
    /// Create a new transition.
    pub fn new(property: String, duration: f32) -> Self {
        Self {
            property,
            duration,
            timing_function: TimingFunction::Ease,
            delay: 0.0,
        }
    }
    
    /// Interpolate value at time t (0.0 to 1.0).
    pub fn interpolate(&self, t: f32) -> f32 {
        self.timing_function.evaluate(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn timing_function_linear() {
        let tf = TimingFunction::Linear;
        assert_eq!(tf.evaluate(0.0), 0.0);
        assert_eq!(tf.evaluate(0.5), 0.5);
        assert_eq!(tf.evaluate(1.0), 1.0);
    }
    
    #[test]
    fn timing_function_steps() {
        let tf = TimingFunction::Steps(4, StepPosition::End);
        assert_eq!(tf.evaluate(0.0), 0.0);
        assert_eq!(tf.evaluate(0.24), 0.0);
        assert_eq!(tf.evaluate(0.25), 0.25);
        assert_eq!(tf.evaluate(0.5), 0.5);
        assert_eq!(tf.evaluate(1.0), 1.0);
    }
    
    #[test]
    fn animation_keyframes() {
        let mut anim = Animation::new("test".to_string(), 1000.0);
        
        let mut kf1 = Keyframe {
            offset: 0.0,
            properties: HashMap::new(),
            timing_function: None,
        };
        kf1.properties.insert("opacity".to_string(), "0".to_string());
        
        let mut kf2 = Keyframe {
            offset: 1.0,
            properties: HashMap::new(),
            timing_function: None,
        };
        kf2.properties.insert("opacity".to_string(), "1".to_string());
        
        anim.add_keyframe(kf1);
        anim.add_keyframe(kf2);
        
        assert_eq!(anim.keyframes.len(), 2);
    }
}
