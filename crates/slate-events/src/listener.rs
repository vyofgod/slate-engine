//! Event listener trait and phase.

use super::Event;

/// Event listener trait.
pub trait EventListener: Send + Sync {
    /// Handle an event.
    fn handle_event(&self, event: &Event);
}

/// Event propagation phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    /// Capturing phase (from root to target).
    Capturing,

    /// At target phase.
    AtTarget,

    /// Bubbling phase (from target to root).
    Bubbling,
}
