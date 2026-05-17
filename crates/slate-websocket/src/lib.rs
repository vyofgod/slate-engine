//! # Slate WebSocket - Real-Time Communication
//!
//! Full WebSocket implementation (RFC 6455) with automatic reconnection.
//!
//! ## Features
//!
//! - **WebSocket Protocol**: Full RFC 6455 compliance
//! - **Secure WebSocket**: WSS support
//! - **Binary & Text**: Both message types
//! - **Automatic Reconnection**: Configurable backoff
//! - **Ping/Pong**: Heartbeat mechanism
//! - **Backpressure**: Flow control
//! - **Compression**: permessage-deflate
//!
//! ## Architecture
//!
//! ```text
//! JavaScript WebSocket API
//!     ↓
//! WebSocket Manager
//!     ↓
//! tokio-tungstenite
//!     ↓
//! TCP/TLS Connection
//!     ↓
//! WebSocket Server
//! ```

pub mod connection;
pub mod message;
pub mod events;
pub mod reconnect;

pub use connection::WebSocket;
pub use message::{Message, MessageType};
pub use events::{WebSocketEvent, EventHandler};

use thiserror::Error;

/// WebSocket errors.
#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),
    
    #[error("Send failed: {0}")]
    SendFailed(String),
    
    #[error("Receive failed: {0}")]
    ReceiveFailed(String),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, WebSocketError>;

/// WebSocket ready state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadyState {
    /// Connecting (0)
    Connecting = 0,
    
    /// Open (1)
    Open = 1,
    
    /// Closing (2)
    Closing = 2,
    
    /// Closed (3)
    Closed = 3,
}

/// WebSocket configuration.
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// Maximum message size in bytes
    pub max_message_size: usize,
    
    /// Maximum frame size in bytes
    pub max_frame_size: usize,
    
    /// Enable compression
    pub compression: bool,
    
    /// Ping interval in seconds
    pub ping_interval: u64,
    
    /// Pong timeout in seconds
    pub pong_timeout: u64,
    
    /// Automatic reconnection
    pub auto_reconnect: bool,
    
    /// Maximum reconnection attempts (None = infinite)
    pub max_reconnect_attempts: Option<u32>,
    
    /// Initial reconnection delay in milliseconds
    pub reconnect_delay: u64,
    
    /// Maximum reconnection delay in milliseconds
    pub max_reconnect_delay: u64,
    
    /// Reconnection backoff multiplier
    pub reconnect_backoff: f32,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
            compression: true,
            ping_interval: 30,
            pong_timeout: 10,
            auto_reconnect: true,
            max_reconnect_attempts: Some(10),
            reconnect_delay: 1000,
            max_reconnect_delay: 30000,
            reconnect_backoff: 1.5,
        }
    }
}

/// WebSocket close code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseCode {
    /// Normal closure (1000)
    Normal = 1000,
    
    /// Going away (1001)
    GoingAway = 1001,
    
    /// Protocol error (1002)
    ProtocolError = 1002,
    
    /// Unsupported data (1003)
    UnsupportedData = 1003,
    
    /// Invalid frame payload data (1007)
    InvalidFramePayloadData = 1007,
    
    /// Policy violation (1008)
    PolicyViolation = 1008,
    
    /// Message too big (1009)
    MessageTooBig = 1009,
    
    /// Mandatory extension (1010)
    MandatoryExtension = 1010,
    
    /// Internal server error (1011)
    InternalServerError = 1011,
    
    /// TLS handshake (1015)
    TlsHandshake = 1015,
}

impl CloseCode {
    /// Convert to u16.
    pub fn as_u16(self) -> u16 {
        self as u16
    }
    
    /// Convert from u16.
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            1000 => Some(CloseCode::Normal),
            1001 => Some(CloseCode::GoingAway),
            1002 => Some(CloseCode::ProtocolError),
            1003 => Some(CloseCode::UnsupportedData),
            1007 => Some(CloseCode::InvalidFramePayloadData),
            1008 => Some(CloseCode::PolicyViolation),
            1009 => Some(CloseCode::MessageTooBig),
            1010 => Some(CloseCode::MandatoryExtension),
            1011 => Some(CloseCode::InternalServerError),
            1015 => Some(CloseCode::TlsHandshake),
            _ => None,
        }
    }
}
