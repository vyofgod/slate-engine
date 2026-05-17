//! Service Worker implementation.

/// Service Worker.
///
/// A service worker acts as a proxy between web applications and the network.
pub struct ServiceWorker {
    /// Worker ID
    id: u32,
    
    /// Scope
    scope: String,
}

impl ServiceWorker {
    /// Create a new service worker.
    pub fn new(id: u32, scope: String, _script_url: String) -> Self {
        Self { id, scope }
    }
    
    /// Get worker ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get scope.
    pub fn scope(&self) -> &str {
        &self.scope
    }
}
