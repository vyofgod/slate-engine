//! WebGL framebuffer management.

/// WebGL framebuffer.
pub struct Framebuffer {
    /// Framebuffer ID
    id: u32,
}

impl Framebuffer {
    /// Create a new framebuffer.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    
    /// Get framebuffer ID.
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// WebGL renderbuffer.
pub struct Renderbuffer {
    /// Renderbuffer ID
    id: u32,
}

impl Renderbuffer {
    /// Create a new renderbuffer.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    
    /// Get renderbuffer ID.
    pub fn id(&self) -> u32 {
        self.id
    }
}
