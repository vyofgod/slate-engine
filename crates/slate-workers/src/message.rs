//! Worker message passing.

use serde::{Serialize, Deserialize};

/// Message passed between main thread and worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message data (JSON-serializable)
    pub data: MessageData,
    
    /// Transferable objects
    pub transfer: Vec<Transferable>,
}

/// Message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageData {
    /// String data
    String(String),
    
    /// Number data
    Number(f64),
    
    /// Boolean data
    Boolean(bool),
    
    /// Null
    Null,
    
    /// Object (JSON)
    Object(serde_json::Value),
    
    /// Array
    Array(Vec<MessageData>),
}

/// Transferable object (zero-copy transfer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transferable {
    /// ArrayBuffer
    ArrayBuffer(Vec<u8>),
    
    /// MessagePort
    MessagePort(u32),
    
    /// ImageBitmap
    ImageBitmap(u32),
}

/// Message port for bidirectional communication.
pub struct MessagePort {
    /// Port ID
    id: u32,
}

impl MessagePort {
    /// Create a new message port.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    
    /// Get port ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Post a message.
    pub fn post_message(&self, _message: Message) {
        // TODO: Implement message posting
    }
}

impl Message {
    /// Create a new message.
    pub fn new(data: MessageData) -> Self {
        Self {
            data,
            transfer: Vec::new(),
        }
    }
    
    /// Create a message with transferables.
    pub fn with_transfer(data: MessageData, transfer: Vec<Transferable>) -> Self {
        Self { data, transfer }
    }
}
