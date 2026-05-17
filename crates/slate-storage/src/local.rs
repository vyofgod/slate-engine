//! localStorage implementation.

use crate::{Result, StorageError, StorageQuota};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use sled::Db;

/// localStorage implementation.
///
/// Provides persistent key-value storage that persists across browser sessions.
pub struct LocalStorage {
    /// Origin (e.g., "https://example.com")
    origin: String,
    
    /// In-memory cache
    cache: Arc<RwLock<HashMap<String, String>>>,
    
    /// Persistent database
    db: Arc<Db>,
    
    /// Quota (in bytes)
    quota: u64,
}

impl LocalStorage {
    /// Create a new localStorage instance.
    pub fn new(origin: String, db_path: &str, quota: u64) -> Result<Self> {
        let db = sled::open(db_path)
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        let mut cache = HashMap::new();
        
        // Load existing data into cache
        let prefix = format!("local:{}:", origin);
        for item in db.scan_prefix(&prefix) {
            if let Ok((key, value)) = item {
                let key_str = String::from_utf8_lossy(&key[prefix.len()..]).to_string();
                let value_str = String::from_utf8_lossy(&value).to_string();
                cache.insert(key_str, value_str);
            }
        }
        
        Ok(Self {
            origin,
            cache: Arc::new(RwLock::new(cache)),
            db: Arc::new(db),
            quota,
        })
    }
    
    /// Get the number of items.
    pub fn length(&self) -> usize {
        self.cache.read().len()
    }
    
    /// Get a value by key.
    pub fn get_item(&self, key: &str) -> Option<String> {
        self.cache.read().get(key).cloned()
    }
    
    /// Set a value by key.
    pub fn set_item(&self, key: String, value: String) -> Result<()> {
        // Check quota
        let current_size = self.get_used_space();
        let new_size = key.len() + value.len();
        
        if current_size + new_size as u64 > self.quota {
            return Err(StorageError::QuotaExceeded);
        }
        
        // Update cache
        self.cache.write().insert(key.clone(), value.clone());
        
        // Persist to database
        let db_key = format!("local:{}:{}", self.origin, key);
        self.db.insert(db_key.as_bytes(), value.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Remove a value by key.
    pub fn remove_item(&self, key: &str) -> Result<()> {
        // Remove from cache
        self.cache.write().remove(key);
        
        // Remove from database
        let db_key = format!("local:{}:{}", self.origin, key);
        self.db.remove(db_key.as_bytes())
            .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Clear all items.
    pub fn clear(&self) -> Result<()> {
        // Clear cache
        self.cache.write().clear();
        
        // Clear database
        let prefix = format!("local:{}:", self.origin);
        let keys: Vec<_> = self.db.scan_prefix(&prefix)
            .filter_map(|item| item.ok().map(|(k, _)| k))
            .collect();
        
        for key in keys {
            self.db.remove(&key)
                .map_err(|e| StorageError::DatabaseError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    /// Get key at index.
    pub fn key(&self, index: usize) -> Option<String> {
        self.cache.read()
            .keys()
            .nth(index)
            .cloned()
    }
    
    /// Get all keys.
    pub fn keys(&self) -> Vec<String> {
        self.cache.read().keys().cloned().collect()
    }
    
    /// Get storage quota information.
    pub fn quota(&self) -> StorageQuota {
        let used = self.get_used_space();
        
        StorageQuota {
            total: self.quota,
            used,
            available: self.quota.saturating_sub(used),
        }
    }
    
    /// Get used space in bytes.
    fn get_used_space(&self) -> u64 {
        self.cache.read()
            .iter()
            .map(|(k, v)| (k.len() + v.len()) as u64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn create_storage() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(
            "https://example.com".to_string(),
            dir.path().to_str().unwrap(),
            10 * 1024 * 1024, // 10MB
        );
        assert!(storage.is_ok());
    }
    
    #[test]
    fn set_and_get() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(
            "https://example.com".to_string(),
            dir.path().to_str().unwrap(),
            10 * 1024 * 1024,
        ).unwrap();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(storage.get_item("key1"), Some("value1".to_string()));
    }
    
    #[test]
    fn remove_item() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(
            "https://example.com".to_string(),
            dir.path().to_str().unwrap(),
            10 * 1024 * 1024,
        ).unwrap();
        
        storage.set_item("key1".to_string(), "value1".to_string()).unwrap();
        storage.remove_item("key1").unwrap();
        assert_eq!(storage.get_item("key1"), None);
    }
    
    #[test]
    fn quota_exceeded() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(
            "https://example.com".to_string(),
            dir.path().to_str().unwrap(),
            100, // Very small quota
        ).unwrap();
        
        let result = storage.set_item(
            "key".to_string(),
            "a".repeat(200), // Exceeds quota
        );
        
        assert!(matches!(result, Err(StorageError::QuotaExceeded)));
    }
}
