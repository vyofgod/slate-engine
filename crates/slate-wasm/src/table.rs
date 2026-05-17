//! WebAssembly tables.

/// WebAssembly table.
pub struct Table {
    /// Table ID
    id: u32,
    
    /// Initial size
    initial: u32,
    
    /// Maximum size
    maximum: Option<u32>,
}

impl Table {
    /// Create a new table.
    pub fn new(id: u32, initial: u32, maximum: Option<u32>) -> Self {
        Self {
            id,
            initial,
            maximum,
        }
    }
    
    /// Get table ID.
    pub fn id(&self) -> u32 {
        self.id
    }
}
