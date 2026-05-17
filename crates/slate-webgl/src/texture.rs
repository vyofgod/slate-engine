//! WebGL texture management.

use crate::{Result, WebGLError};

/// Texture target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureTarget {
    /// 2D texture
    Texture2D,
    
    /// Cube map texture
    TextureCubeMap,
}

/// Texture format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// RGBA
    RGBA,
    
    /// RGB
    RGB,
    
    /// Alpha
    Alpha,
    
    /// Luminance
    Luminance,
    
    /// Luminance + Alpha
    LuminanceAlpha,
}

/// WebGL texture.
pub struct Texture {
    /// Texture ID
    id: u32,
}

impl Texture {
    /// Create a new texture.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    
    /// Get texture ID.
    pub fn id(&self) -> u32 {
        self.id
    }
}
