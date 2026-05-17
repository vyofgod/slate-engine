//! # Slate Workers - Multi-Threading for JavaScript
//!
//! Full Web Workers implementation with true multi-threading.
//!
//! ## Features
//!
//! - **Dedicated Workers**: One-to-one communication
//! - **Shared Workers**: Many-to-one communication
//! - **Service Workers**: Offline-first applications
//! - **Message Passing**: Structured cloning
//! - **Transferable Objects**: Zero-copy transfers
//! - **Worker Pools**: Efficient resource management
//!
//! ## Architecture
//!
//! ```text
//! Main Thread
//!     ↓ postMessage
//! Message Queue
//!     ↓
//! Worker Thread (isolated JS context)
//!     ↓ Heavy computation
//! Message Queue
//!     ↓ onmessage
//! Main Thread
//! ```

pub mod dedicated;
pub mod shared;
pub mod service;
pub mod message;
pub mod pool;

pub use dedicated::DedicatedWorker;
pub use shared::SharedWorker;
pub use service::ServiceWorker;
pub use message::{Message, MessagePort, Transferable};
pub use pool::WorkerPool;

use thiserror::Error;

/// Worker errors.
#[derive(Debug, Error)]
pub enum WorkerError {
    #[error("Worker creation failed: {0}")]
    CreationFailed(String),
    
    #[error("Worker terminated")]
    Terminated,
    
    #[error("Message send failed: {0}")]
    SendFailed(String),
    
    #[error("Message receive failed: {0}")]
    ReceiveFailed(String),
    
    #[error("Script execution failed: {0}")]
    ScriptExecutionFailed(String),
    
    #[error("Invalid transferable: {0}")]
    InvalidTransferable(String),
}

pub type Result<T> = std::result::Result<T, WorkerError>;

/// Worker state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerState {
    /// Worker is initializing
    Initializing,
    
    /// Worker is running
    Running,
    
    /// Worker is paused
    Paused,
    
    /// Worker is terminated
    Terminated,
}

/// Worker type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerType {
    /// Dedicated worker
    Dedicated,
    
    /// Shared worker
    Shared,
    
    /// Service worker
    Service,
}
