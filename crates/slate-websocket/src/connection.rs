//! WebSocket connection management.

use crate::{Result, WebSocketError, ReadyState, CloseCode, Message, WebSocketConfig};
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;
use futures_util::{StreamExt, SinkExt};
use url::Url;
use parking_lot::RwLock;
use std::sync::Arc;

/// WebSocket connection.
pub struct WebSocket {
    /// WebSocket URL
    url: String,
    
    /// Ready state
    state: Arc<RwLock<ReadyState>>,
    
    /// Configuration
    config: WebSocketConfig,
}

impl WebSocket {
    /// Create a new WebSocket connection.
    pub fn new(url: String, config: WebSocketConfig) -> Result<Self> {
        // Validate URL
        let _ = Url::parse(&url)
            .map_err(|e| WebSocketError::InvalidUrl(e.to_string()))?;
        
        Ok(Self {
            url,
            state: Arc::new(RwLock::new(ReadyState::Connecting)),
            config,
        })
    }
    
    /// Connect to the WebSocket server.
    pub async fn connect(&self) -> Result<()> {
        *self.state.write() = ReadyState::Connecting;
        
        let url = Url::parse(&self.url)
            .map_err(|e| WebSocketError::InvalidUrl(e.to_string()))?;
        
        let (ws_stream, _) = connect_async(url).await
            .map_err(|e| WebSocketError::ConnectionFailed(e.to_string()))?;
        
        *self.state.write() = ReadyState::Open;
        
        Ok(())
    }
    
    /// Get ready state.
    pub fn ready_state(&self) -> ReadyState {
        *self.state.read()
    }
    
    /// Get URL.
    pub fn url(&self) -> &str {
        &self.url
    }
    
    /// Close the connection.
    pub fn close(&self, code: CloseCode, reason: &str) {
        *self.state.write() = ReadyState::Closing;
        // TODO: Send close frame
        *self.state.write() = ReadyState::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_websocket() {
        let config = WebSocketConfig::default();
        let ws = WebSocket::new("ws://localhost:8080".to_string(), config);
        assert!(ws.is_ok());
    }
    
    #[test]
    fn invalid_url() {
        let config = WebSocketConfig::default();
        let ws = WebSocket::new("invalid-url".to_string(), config);
        assert!(ws.is_err());
    }
}
