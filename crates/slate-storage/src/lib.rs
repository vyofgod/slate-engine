//! # Slate Storage - Web Storage APIs
//!
//! Full implementation of localStorage, sessionStorage, and IndexedDB.
//!
//! ## Features
//!
//! - **localStorage**: Persistent key-value storage
//! - **sessionStorage**: Session-scoped key-value storage
//! - **IndexedDB**: Transactional object database
//! - **Quota Management**: Storage limits and eviction
//! - **Storage Events**: Cross-tab synchronization
//!
//! ## Architecture
//!
//! ```text
//! JavaScript Storage API
//!     ↓
//! Storage Manager
//!     ↓
//! Sled (embedded database)
//!     ↓
//! Disk
//! ```

pub mod local;
pub mod session;
pub mod indexed_db;
pub mod quota;
pub mod events;

pub use local::LocalStorage;
pub use session::SessionStorage;
pub use indexed_db::IndexedDB;
pub use quota::QuotaManager;

use thiserror::Error;

/// Storage errors.
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Quota exceeded")]
    QuotaExceeded,
    
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// Storage type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    /// localStorage (persistent)
    Local,
    
    /// sessionStorage (session-scoped)
    Session,
    
    /// IndexedDB (transactional database)
    IndexedDB,
}

/// Storage quota information.
#[derive(Debug, Clone)]
pub struct StorageQuota {
    /// Total quota in bytes
    pub total: u64,
    
    /// Used space in bytes
    pub used: u64,
    
    /// Available space in bytes
    pub available: u64,
}

impl StorageQuota {
    /// Check if quota is exceeded.
    pub fn is_exceeded(&self) -> bool {
        self.used >= self.total
    }
    
    /// Get usage percentage.
    pub fn usage_percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.used as f64 / self.total as f64) * 100.0
        }
    }
}
