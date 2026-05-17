//! Storage quota management.

use crate::StorageQuota;

/// Quota manager.
pub struct QuotaManager {
    /// Default quota per origin (in bytes)
    default_quota: u64,
}

impl QuotaManager {
    /// Create a new quota manager.
    pub fn new(default_quota: u64) -> Self {
        Self { default_quota }
    }
    
    /// Get quota for an origin.
    pub fn get_quota(&self, _origin: &str) -> u64 {
        self.default_quota
    }
    
    /// Request additional quota.
    pub fn request_quota(&self, _origin: &str, _requested: u64) -> bool {
        // TODO: Implement quota request logic
        false
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new(50 * 1024 * 1024) // 50MB default
    }
}
