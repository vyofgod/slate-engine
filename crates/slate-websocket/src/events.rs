//! WebSocket event handling.

use crate::Message;

/// WebSocket event.
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    /// Connection opened
    Open,
    
    /// Message received
    Message(Message),
    
    /// Connection closed
    Close { code: u16, reason: String },
    
    /// Error occurred
    Error(String),
}

/// Event handler trait.
pub trait EventHandler: Send + Sync {
    /// Handle an event.
    fn handle(&self, event: WebSocketEvent);
}
