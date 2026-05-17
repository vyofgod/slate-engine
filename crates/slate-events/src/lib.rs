//! # Slate Event System
//!
//! DOM event handling with bubbling, capturing, and preventDefault.

use slate_ais::{NodeId, Point};

pub mod dispatcher;
pub mod listener;
pub mod types;

pub use dispatcher::EventDispatcher;
pub use listener::{EventListener, EventPhase};

/// An event that can be dispatched to nodes.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub target: NodeId,
    pub current_target: Option<NodeId>,
    pub phase: EventPhase,
    pub bubbles: bool,
    pub cancelable: bool,
    pub timestamp: u64,
    pub data: EventData,
    pub default_prevented: bool,
    pub propagation_stopped: bool,
}

impl Event {
    /// Create a new event.
    pub fn new(event_type: EventType, target: NodeId, data: EventData) -> Self {
        let bubbles = event_type.bubbles();
        let cancelable = event_type.cancelable();

        Self {
            event_type,
            target,
            current_target: None,
            phase: EventPhase::AtTarget,
            bubbles,
            cancelable,
            timestamp: 0, // TODO: Use monotonic clock
            data,
            default_prevented: false,
            propagation_stopped: false,
        }
    }

    /// Prevent the default action.
    pub fn prevent_default(&mut self) {
        if self.cancelable {
            self.default_prevented = true;
        }
    }

    /// Stop event propagation.
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }

    /// Check if default was prevented.
    #[inline]
    pub fn is_default_prevented(&self) -> bool {
        self.default_prevented
    }

    /// Check if propagation was stopped.
    #[inline]
    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_stopped
    }
}

/// Event type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    // Mouse events
    Click,
    DblClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseEnter,
    MouseLeave,
    MouseOver,
    MouseOut,
    ContextMenu,
    Wheel,

    // Keyboard events
    KeyDown,
    KeyUp,
    KeyPress,

    // Focus events
    Focus,
    Blur,
    FocusIn,
    FocusOut,

    // Form events
    Submit,
    Change,
    Input,
    Invalid,
    Reset,
    Select,

    // Touch events
    TouchStart,
    TouchEnd,
    TouchMove,
    TouchCancel,

    // Pointer events
    PointerDown,
    PointerUp,
    PointerMove,
    PointerCancel,
    PointerEnter,
    PointerLeave,
    PointerOver,
    PointerOut,

    // Drag events
    Drag,
    DragStart,
    DragEnd,
    DragEnter,
    DragLeave,
    DragOver,
    Drop,

    // Scroll events
    Scroll,

    // Load events
    Load,
    Unload,
    BeforeUnload,
    Error,
    Abort,

    // Animation events
    AnimationStart,
    AnimationEnd,
    AnimationIteration,

    // Transition events
    TransitionStart,
    TransitionEnd,
    TransitionRun,
    TransitionCancel,

    // Custom events
    Custom(u32), // Hash of custom event name
}

impl EventType {
    /// Check if this event type bubbles.
    pub fn bubbles(&self) -> bool {
        !matches!(
            self,
            EventType::Focus
                | EventType::Blur
                | EventType::Load
                | EventType::Unload
                | EventType::MouseEnter
                | EventType::MouseLeave
                | EventType::PointerEnter
                | EventType::PointerLeave
        )
    }

    /// Check if this event type is cancelable.
    pub fn cancelable(&self) -> bool {
        !matches!(
            self,
            EventType::Load
                | EventType::Unload
                | EventType::Scroll
                | EventType::MouseEnter
                | EventType::MouseLeave
        )
    }

    /// Parse an event type from a string.
    pub fn from_str(s: &str) -> Self {
        match s {
            "click" => EventType::Click,
            "dblclick" => EventType::DblClick,
            "mousedown" => EventType::MouseDown,
            "mouseup" => EventType::MouseUp,
            "mousemove" => EventType::MouseMove,
            "mouseenter" => EventType::MouseEnter,
            "mouseleave" => EventType::MouseLeave,
            "keydown" => EventType::KeyDown,
            "keyup" => EventType::KeyUp,
            "focus" => EventType::Focus,
            "blur" => EventType::Blur,
            "submit" => EventType::Submit,
            "change" => EventType::Change,
            "input" => EventType::Input,
            "scroll" => EventType::Scroll,
            "load" => EventType::Load,
            _ => {
                let hash = s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
                EventType::Custom(hash)
            }
        }
    }
}

/// Event-specific data.
#[derive(Debug, Clone)]
pub enum EventData {
    Mouse(MouseEventData),
    Keyboard(KeyboardEventData),
    Touch(TouchEventData),
    Pointer(PointerEventData),
    Wheel(WheelEventData),
    Focus(FocusEventData),
    None,
}

/// Mouse event data.
#[derive(Debug, Clone, Copy)]
pub struct MouseEventData {
    pub position: Point,
    pub screen_position: Point,
    pub button: MouseButton,
    pub buttons: u8,
    pub modifiers: Modifiers,
}

/// Mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,
}

/// Keyboard event data.
#[derive(Debug, Clone)]
pub struct KeyboardEventData {
    pub key: String,
    pub code: String,
    pub modifiers: Modifiers,
    pub repeat: bool,
}

/// Touch event data.
#[derive(Debug, Clone)]
pub struct TouchEventData {
    pub touches: Vec<Touch>,
    pub changed_touches: Vec<Touch>,
    pub target_touches: Vec<Touch>,
}

/// A single touch point.
#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub identifier: i32,
    pub position: Point,
    pub screen_position: Point,
    pub radius: Point,
    pub rotation_angle: f32,
    pub force: f32,
}

/// Pointer event data.
#[derive(Debug, Clone, Copy)]
pub struct PointerEventData {
    pub pointer_id: i32,
    pub position: Point,
    pub screen_position: Point,
    pub width: f32,
    pub height: f32,
    pub pressure: f32,
    pub tilt_x: f32,
    pub tilt_y: f32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
    pub button: MouseButton,
    pub buttons: u8,
    pub modifiers: Modifiers,
}

/// Pointer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

/// Wheel event data.
#[derive(Debug, Clone, Copy)]
pub struct WheelEventData {
    pub delta_x: f32,
    pub delta_y: f32,
    pub delta_z: f32,
    pub delta_mode: WheelDeltaMode,
    pub modifiers: Modifiers,
}

/// Wheel delta mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page,
}

/// Focus event data.
#[derive(Debug, Clone, Copy)]
pub struct FocusEventData {
    pub related_target: Option<NodeId>,
}

/// Keyboard modifiers.
#[derive(Debug, Clone, Copy, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_bubbling() {
        assert!(EventType::Click.bubbles());
        assert!(!EventType::Focus.bubbles());
    }

    #[test]
    fn event_cancelable() {
        assert!(EventType::Click.cancelable());
        assert!(!EventType::Load.cancelable());
    }

    #[test]
    fn prevent_default() {
        let mut event = Event::new(
            EventType::Click,
            NodeId(1),
            EventData::None,
        );

        assert!(!event.is_default_prevented());
        event.prevent_default();
        assert!(event.is_default_prevented());
    }
}
