//! Storage events for cross-tab synchronization.

/// Storage event.
#[derive(Debug, Clone)]
pub struct StorageEvent {
    /// Key that changed
    pub key: Option<String>,
    
    /// Old value
    pub old_value: Option<String>,
    
    /// New value
    pub new_value: Option<String>,
    
    /// Storage area URL
    pub url: String,
    
    /// Storage type
    pub storage_type: String,
}

impl StorageEvent {
    /// Create a new storage event.
    pub fn new(
        key: Option<String>,
        old_value: Option<String>,
        new_value: Option<String>,
        url: String,
        storage_type: String,
    ) -> Self {
        Self {
            key,
            old_value,
            new_value,
            url,
            storage_type,
        }
    }
}
