//! Shared Worker implementation.

/// Shared Worker.
///
/// A shared worker can be accessed by multiple scripts, even from different windows.
pub struct SharedWorker {
    /// Worker ID
    id: u32,
    
    /// Worker name
    name: String,
}

impl SharedWorker {
    /// Create a new shared worker.
    pub fn new(id: u32, name: String, _script_url: String) -> Self {
        Self { id, name }
    }
    
    /// Get worker ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get worker name.
    pub fn name(&self) -> &str {
        &self.name
    }
}
