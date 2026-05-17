//! IndexedDB implementation.

use crate::{Result, StorageError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// IndexedDB database.
pub struct IndexedDB {
    /// Database name
    name: String,
    
    /// Database version
    version: u32,
    
    /// Object stores
    stores: HashMap<String, ObjectStore>,
}

/// Object store.
#[derive(Debug, Clone)]
pub struct ObjectStore {
    /// Store name
    pub name: String,
    
    /// Key path
    pub key_path: Option<String>,
    
    /// Auto increment
    pub auto_increment: bool,
    
    /// Indexes
    pub indexes: HashMap<String, Index>,
}

/// Index.
#[derive(Debug, Clone)]
pub struct Index {
    /// Index name
    pub name: String,
    
    /// Key path
    pub key_path: String,
    
    /// Unique
    pub unique: bool,
    
    /// Multi-entry
    pub multi_entry: bool,
}

/// Transaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionMode {
    /// Read-only
    ReadOnly,
    
    /// Read-write
    ReadWrite,
}

impl IndexedDB {
    /// Create a new IndexedDB database.
    pub fn new(name: String, version: u32) -> Self {
        Self {
            name,
            version,
            stores: HashMap::new(),
        }
    }
    
    /// Create an object store.
    pub fn create_object_store(&mut self, name: String, key_path: Option<String>, auto_increment: bool) {
        let store = ObjectStore {
            name: name.clone(),
            key_path,
            auto_increment,
            indexes: HashMap::new(),
        };
        
        self.stores.insert(name, store);
    }
    
    /// Get an object store.
    pub fn object_store(&self, name: &str) -> Option<&ObjectStore> {
        self.stores.get(name)
    }
}
