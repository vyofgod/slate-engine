//! WebGL shader management.

use crate::{Result, WebGLError};
use parking_lot::RwLock;
use std::sync::Arc;

/// Shader type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderType {
    /// Vertex shader
    Vertex,
    
    /// Fragment shader
    Fragment,
}

/// WebGL shader.
pub struct Shader {
    /// Shader ID
    id: u32,
    
    /// Shader type
    shader_type: ShaderType,
    
    /// Shader source code (GLSL)
    source: Arc<RwLock<Option<String>>>,
    
    /// Compiled shader module (WGSL)
    module: Arc<RwLock<Option<wgpu::ShaderModule>>>,
    
    /// Compilation status
    compile_status: Arc<RwLock<bool>>,
    
    /// Info log
    info_log: Arc<RwLock<String>>,
}

impl Shader {
    /// Create a new shader.
    pub fn new(id: u32, shader_type_const: u32) -> Result<Self> {
        let shader_type = match shader_type_const {
            crate::constants::VERTEX_SHADER => ShaderType::Vertex,
            crate::constants::FRAGMENT_SHADER => ShaderType::Fragment,
            _ => return Err(WebGLError::InvalidEnum("Invalid shader type".to_string())),
        };
        
        Ok(Self {
            id,
            shader_type,
            source: Arc::new(RwLock::new(None)),
            module: Arc::new(RwLock::new(None)),
            compile_status: Arc::new(RwLock::new(false)),
            info_log: Arc::new(RwLock::new(String::new())),
        })
    }
    
    /// Get shader ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get shader type.
    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }
    
    /// Set shader source.
    pub fn set_source(&self, source: String) {
        *self.source.write() = Some(source);
        *self.compile_status.write() = false;
    }
    
    /// Compile shader.
    pub fn compile(&self, device: &wgpu::Device) -> Result<()> {
        let source = self.source.read();
        let glsl_source = source.as_ref()
            .ok_or_else(|| WebGLError::InvalidOperation("No source set".to_string()))?;
        
        // Convert GLSL to WGSL
        // For now, we'll just create a placeholder module
        // TODO: Implement proper GLSL → WGSL translation
        let wgsl_source = self.glsl_to_wgsl(glsl_source)?;
        
        // Create shader module
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("Shader {}", self.id)),
            source: wgpu::ShaderSource::Wgsl(wgsl_source.into()),
        });
        
        *self.module.write() = Some(module);
        *self.compile_status.write() = true;
        *self.info_log.write() = "Compilation successful".to_string();
        
        Ok(())
    }
    
    /// Convert GLSL to WGSL.
    fn glsl_to_wgsl(&self, glsl: &str) -> Result<String> {
        // TODO: Implement proper GLSL → WGSL translation
        // For now, return a simple placeholder
        
        match self.shader_type {
            ShaderType::Vertex => Ok(r#"
                @vertex
                fn main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
                    return vec4<f32>(position, 1.0);
                }
            "#.to_string()),
            
            ShaderType::Fragment => Ok(r#"
                @fragment
                fn main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
                }
            "#.to_string()),
        }
    }
    
    /// Get shader parameter.
    pub fn get_parameter(&self, pname: u32) -> Result<i32> {
        match pname {
            0x8B81 => { // COMPILE_STATUS
                Ok(if *self.compile_status.read() { 1 } else { 0 })
            }
            0x8B84 => { // SHADER_TYPE
                Ok(match self.shader_type {
                    ShaderType::Vertex => crate::constants::VERTEX_SHADER as i32,
                    ShaderType::Fragment => crate::constants::FRAGMENT_SHADER as i32,
                })
            }
            0x8B88 => { // SHADER_SOURCE_LENGTH
                Ok(self.source.read().as_ref().map(|s| s.len() as i32).unwrap_or(0))
            }
            _ => Err(WebGLError::InvalidEnum("Invalid parameter name".to_string())),
        }
    }
    
    /// Get info log.
    pub fn get_info_log(&self) -> String {
        self.info_log.read().clone()
    }
    
    /// Get shader module.
    pub fn module(&self) -> Option<wgpu::ShaderModule> {
        self.module.read().clone()
    }
}
