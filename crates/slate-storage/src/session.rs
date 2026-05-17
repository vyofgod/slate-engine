//! sessionStorage implementation.

use crate::{Result, StorageError};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

/// sessionStorage implementation.
///
/// Provides session-scoped key-value storage that is cleared when the session ends.
pub struct SessionStorage {
    /// Origin
    origin: String,
    
    /// In-memory storage (not persisted)
    storage: Arc<RwLock<HashMap<String, String>>>,
    
    /// Quota (in bytes)
    quota: u64,
}

impl SessionStorage {
    /// Create a new sessionStorage instance.
    pub fn new(origin: String, quota: u64) -> Self {
        Self {
            origin,
            storage: Arc::new(RwLock::new(HashMap::new())),
            quota,
        }
    }
    
    /// Get the number of items.
    pub fn length(&self) -> usize {
        self.storage.read().len()
    }
    
    /// Get a value by key.
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.storage.read().get(key).cloned()
    }
    
    /// Set a value by key.
    pub fn set_item(&self, key: String, value: String) -> Result<()> {
        let current_size: usize = self.storage.read()
            .iter()
            .map(|(k, v)| k.len() + v.len())
            .sum();
        
        let new_size = key.len() + value.len();
        
        if current_size + new_size > self.quota as usize {
            return Err(StorageError::QuotaExceeded);
        }
        
        self.storage.write().insert(key, value);
        Ok(())
    }
    
    /// Remove a value by key.
    pub fn remove_item(&self, key: &str) {
        self.storage.write().remove(key);
    }
    
    /// Clear all items.
    pub fn clear(&self) {
        self.storage.write().clear();
    }
    
    /// Get key at index.
    pub fn key(&self, index: usize) -> Option<String> {
        self.storage.read().keys().nth(index).cloned()
    }
}
