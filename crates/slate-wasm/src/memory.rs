//! WebAssembly linear memory.

/// WebAssembly memory.
pub struct Memory {
    /// Memory ID
    id: u32,
    
    /// Initial size (in pages, 64KB each)
    initial: u32,
    
    /// Maximum size (in pages)
    maximum: Option<u32>,
}

impl Memory {
    /// Create a new memory.
    pub fn new(id: u32, initial: u32, maximum: Option<u32>) -> Self {
        Self {
            id,
            initial,
            maximum,
        }
    }
    
    /// Get memory ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get initial size.
    pub fn initial(&self) -> u32 {
        self.initial
    }
    
    /// Get maximum size.
    pub fn maximum(&self) -> Option<u32> {
        self.maximum
    }
}
