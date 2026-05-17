//! WebSocket message types.

use bytes::Bytes;

/// WebSocket message.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message type
    pub message_type: MessageType,
    
    /// Message data
    pub data: MessageData,
}

/// Message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Text message
    Text,
    
    /// Binary message
    Binary,
    
    /// Ping
    Ping,
    
    /// Pong
    Pong,
    
    /// Close
    Close,
}

/// Message data.
#[derive(Debug, Clone)]
pub enum MessageData {
    /// Text data
    Text(String),
    
    /// Binary data
    Binary(Bytes),
    
    /// Empty
    Empty,
}

impl Message {
    /// Create a text message.
    pub fn text(data: String) -> Self {
        Self {
            message_type: MessageType::Text,
            data: MessageData::Text(data),
        }
    }
    
    /// Create a binary message.
    pub fn binary(data: Bytes) -> Self {
        Self {
            message_type: MessageType::Binary,
            data: MessageData::Binary(data),
        }
    }
    
    /// Create a ping message.
    pub fn ping() -> Self {
        Self {
            message_type: MessageType::Ping,
            data: MessageData::Empty,
        }
    }
    
    /// Create a pong message.
    pub fn pong() -> Self {
        Self {
            message_type: MessageType::Pong,
            data: MessageData::Empty,
        }
    }
    
    /// Check if message is text.
    pub fn is_text(&self) -> bool {
        self.message_type == MessageType::Text
    }
    
    /// Check if message is binary.
    pub fn is_binary(&self) -> bool {
        self.message_type == MessageType::Binary
    }
    
    /// Get text data.
    pub fn as_text(&self) -> Option<&str> {
        match &self.data {
            MessageData::Text(s) => Some(s),
            _ => None,
        }
    }
    
    /// Get binary data.
    pub fn as_binary(&self) -> Option<&Bytes> {
        match &self.data {
            MessageData::Binary(b) => Some(b),
            _ => None,
        }
    }
}
