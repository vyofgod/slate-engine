//! WebGL program management.

use crate::{Result, WebGLError, Shader};
use parking_lot::RwLock;
use std::sync::Arc;

/// WebGL program.
pub struct Program {
    /// Program ID
    id: u32,
    
    /// Attached shaders
    shaders: Arc<RwLock<Vec<Arc<Shader>>>>,
    
    /// Link status
    link_status: Arc<RwLock<bool>>,
    
    /// Info log
    info_log: Arc<RwLock<String>>,
}

impl Program {
    /// Create a new program.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            shaders: Arc::new(RwLock::new(Vec::new())),
            link_status: Arc::new(RwLock::new(false)),
            info_log: Arc::new(RwLock::new(String::new())),
        }
    }
    
    /// Get program ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Attach a shader.
    pub fn attach_shader(&self, shader: Arc<Shader>) {
        self.shaders.write().push(shader);
    }
    
    /// Link program.
    pub fn link(&self, _device: &wgpu::Device) -> Result<()> {
        let shaders = self.shaders.read();
        
        if shaders.len() < 2 {
            return Err(WebGLError::ProgramLinkingFailed(
                "Program must have both vertex and fragment shaders".to_string()
            ));
        }
        
        // TODO: Create render pipeline
        
        *self.link_status.write() = true;
        *self.info_log.write() = "Link successful".to_string();
        
        Ok(())
    }
    
    /// Get program parameter.
    pub fn get_parameter(&self, pname: u32) -> Result<i32> {
        match pname {
            0x8B82 => { // LINK_STATUS
                Ok(if *self.link_status.read() { 1 } else { 0 })
            }
            0x8B83 => { // VALIDATE_STATUS
                Ok(1) // Always valid for now
            }
            0x8B86 => { // ATTACHED_SHADERS
                Ok(self.shaders.read().len() as i32)
            }
            _ => Err(WebGLError::InvalidEnum("Invalid parameter name".to_string())),
        }
    }
    
    /// Get info log.
    pub fn get_info_log(&self) -> String {
        self.info_log.read().clone()
    }
}
