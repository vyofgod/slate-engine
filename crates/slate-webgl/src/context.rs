//! WebGL rendering context.

use crate::{Result, WebGLError};
use crate::state::WebGLState;
use crate::buffer::Buffer;
use crate::shader::Shader;
use crate::program::Program;
use crate::texture::Texture;
use crate::framebuffer::Framebuffer;
use parking_lot::RwLock;
use std::sync::Arc;
use dashmap::DashMap;

/// WebGL version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebGLVersion {
    /// WebGL 1.0
    WebGL1,
    /// WebGL 2.0
    WebGL2,
}

/// WebGL rendering context.
///
/// This is the main entry point for WebGL operations.
/// It manages GPU resources, state, and command submission.
pub struct WebGLContext {
    /// WebGL version
    version: WebGLVersion,
    
    /// wgpu device
    device: Arc<wgpu::Device>,
    
    /// wgpu queue
    queue: Arc<wgpu::Queue>,
    
    /// Current WebGL state
    state: Arc<RwLock<WebGLState>>,
    
    /// Buffer registry
    buffers: Arc<DashMap<u32, Arc<Buffer>>>,
    
    /// Shader registry
    shaders: Arc<DashMap<u32, Arc<Shader>>>,
    
    /// Program registry
    programs: Arc<DashMap<u32, Arc<Program>>>,
    
    /// Texture registry
    textures: Arc<DashMap<u32, Arc<Texture>>>,
    
    /// Framebuffer registry
    framebuffers: Arc<DashMap<u32, Arc<Framebuffer>>>,
    
    /// Next object ID
    next_id: Arc<RwLock<u32>>,
    
    /// Context lost flag
    context_lost: Arc<RwLock<bool>>,
}

impl WebGLContext {
    /// Create a new WebGL context.
    pub async fn new(version: WebGLVersion) -> Result<Self> {
        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Request adapter
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(WebGLError::ContextLost)?;
        
        // Request device
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WebGL Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|_| WebGLError::ContextLost)?;
        
        Ok(Self {
            version,
            device: Arc::new(device),
            queue: Arc::new(queue),
            state: Arc::new(RwLock::new(WebGLState::new())),
            buffers: Arc::new(DashMap::new()),
            shaders: Arc::new(DashMap::new()),
            programs: Arc::new(DashMap::new()),
            textures: Arc::new(DashMap::new()),
            framebuffers: Arc::new(DashMap::new()),
            next_id: Arc::new(RwLock::new(1)),
            context_lost: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Get WebGL version.
    pub fn version(&self) -> WebGLVersion {
        self.version
    }
    
    /// Check if context is lost.
    pub fn is_context_lost(&self) -> bool {
        *self.context_lost.read()
    }
    
    /// Generate a new object ID.
    fn gen_id(&self) -> u32 {
        let mut next_id = self.next_id.write();
        let id = *next_id;
        *next_id += 1;
        id
    }
    
    // === Buffer Operations ===
    
    /// Create a buffer.
    pub fn create_buffer(&self) -> u32 {
        let id = self.gen_id();
        let buffer = Arc::new(Buffer::new(id, self.device.clone()));
        self.buffers.insert(id, buffer);
        id
    }
    
    /// Delete a buffer.
    pub fn delete_buffer(&self, id: u32) {
        self.buffers.remove(&id);
    }
    
    /// Bind a buffer.
    pub fn bind_buffer(&self, target: u32, id: u32) -> Result<()> {
        if let Some(buffer) = self.buffers.get(&id) {
            self.state.write().bind_buffer(target, id);
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid buffer".to_string()))
        }
    }
    
    /// Buffer data.
    pub fn buffer_data(&self, target: u32, data: &[u8], usage: u32) -> Result<()> {
        let state = self.state.read();
        let buffer_id = state.get_bound_buffer(target)
            .ok_or_else(|| WebGLError::InvalidOperation("No buffer bound".to_string()))?;
        
        drop(state);
        
        if let Some(buffer) = self.buffers.get(&buffer_id) {
            buffer.set_data(data, usage, &self.device, &self.queue)?;
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid buffer".to_string()))
        }
    }
    
    // === Shader Operations ===
    
    /// Create a shader.
    pub fn create_shader(&self, shader_type: u32) -> Result<u32> {
        let id = self.gen_id();
        let shader = Arc::new(Shader::new(id, shader_type)?);
        self.shaders.insert(id, shader);
        Ok(id)
    }
    
    /// Delete a shader.
    pub fn delete_shader(&self, id: u32) {
        self.shaders.remove(&id);
    }
    
    /// Shader source.
    pub fn shader_source(&self, id: u32, source: String) -> Result<()> {
        if let Some(shader) = self.shaders.get(&id) {
            shader.set_source(source);
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid shader".to_string()))
        }
    }
    
    /// Compile shader.
    pub fn compile_shader(&self, id: u32) -> Result<()> {
        if let Some(shader) = self.shaders.get(&id) {
            shader.compile(&self.device)?;
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid shader".to_string()))
        }
    }
    
    /// Get shader parameter.
    pub fn get_shader_parameter(&self, id: u32, pname: u32) -> Result<i32> {
        if let Some(shader) = self.shaders.get(&id) {
            shader.get_parameter(pname)
        } else {
            Err(WebGLError::InvalidOperation("Invalid shader".to_string()))
        }
    }
    
    /// Get shader info log.
    pub fn get_shader_info_log(&self, id: u32) -> Result<String> {
        if let Some(shader) = self.shaders.get(&id) {
            Ok(shader.get_info_log())
        } else {
            Err(WebGLError::InvalidOperation("Invalid shader".to_string()))
        }
    }
    
    // === Program Operations ===
    
    /// Create a program.
    pub fn create_program(&self) -> u32 {
        let id = self.gen_id();
        let program = Arc::new(Program::new(id));
        self.programs.insert(id, program);
        id
    }
    
    /// Delete a program.
    pub fn delete_program(&self, id: u32) {
        self.programs.remove(&id);
    }
    
    /// Attach shader to program.
    pub fn attach_shader(&self, program_id: u32, shader_id: u32) -> Result<()> {
        let program = self.programs.get(&program_id)
            .ok_or_else(|| WebGLError::InvalidOperation("Invalid program".to_string()))?;
        
        let shader = self.shaders.get(&shader_id)
            .ok_or_else(|| WebGLError::InvalidOperation("Invalid shader".to_string()))?;
        
        program.attach_shader(shader.clone());
        Ok(())
    }
    
    /// Link program.
    pub fn link_program(&self, id: u32) -> Result<()> {
        if let Some(program) = self.programs.get(&id) {
            program.link(&self.device)?;
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid program".to_string()))
        }
    }
    
    /// Use program.
    pub fn use_program(&self, id: u32) -> Result<()> {
        if let Some(_program) = self.programs.get(&id) {
            self.state.write().use_program(id);
            Ok(())
        } else {
            Err(WebGLError::InvalidOperation("Invalid program".to_string()))
        }
    }
    
    /// Get program parameter.
    pub fn get_program_parameter(&self, id: u32, pname: u32) -> Result<i32> {
        if let Some(program) = self.programs.get(&id) {
            program.get_parameter(pname)
        } else {
            Err(WebGLError::InvalidOperation("Invalid program".to_string()))
        }
    }
    
    /// Get program info log.
    pub fn get_program_info_log(&self, id: u32) -> Result<String> {
        if let Some(program) = self.programs.get(&id) {
            Ok(program.get_info_log())
        } else {
            Err(WebGLError::InvalidOperation("Invalid program".to_string()))
        }
    }
    
    // === Drawing Operations ===
    
    /// Clear the framebuffer.
    pub fn clear(&self, mask: u32) {
        // TODO: Implement clear operation
        let _state = self.state.read();
        // Generate clear command
    }
    
    /// Clear color.
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.state.write().set_clear_color(r, g, b, a);
    }
    
    /// Draw arrays.
    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) -> Result<()> {
        if count < 0 {
            return Err(WebGLError::InvalidValue("count < 0".to_string()));
        }
        
        // TODO: Implement draw arrays
        let _state = self.state.read();
        // Generate draw command
        
        Ok(())
    }
    
    /// Draw elements.
    pub fn draw_elements(&self, mode: u32, count: i32, type_: u32, offset: i32) -> Result<()> {
        if count < 0 {
            return Err(WebGLError::InvalidValue("count < 0".to_string()));
        }
        
        // TODO: Implement draw elements
        let _state = self.state.read();
        // Generate draw command
        
        Ok(())
    }
    
    // === State Operations ===
    
    /// Enable capability.
    pub fn enable(&self, cap: u32) {
        self.state.write().enable(cap);
    }
    
    /// Disable capability.
    pub fn disable(&self, cap: u32) {
        self.state.write().disable(cap);
    }
    
    /// Viewport.
    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.state.write().set_viewport(x, y, width, height);
    }
    
    /// Scissor.
    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        self.state.write().set_scissor(x, y, width, height);
    }
    
    /// Blend function.
    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        self.state.write().set_blend_func(sfactor, dfactor);
    }
    
    /// Depth function.
    pub fn depth_func(&self, func: u32) {
        self.state.write().set_depth_func(func);
    }
    
    /// Get wgpu device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    
    /// Get wgpu queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn create_context() {
        let ctx = WebGLContext::new(WebGLVersion::WebGL1).await;
        assert!(ctx.is_ok());
    }
    
    #[tokio::test]
    async fn create_buffer() {
        let ctx = WebGLContext::new(WebGLVersion::WebGL1).await.unwrap();
        let buffer_id = ctx.create_buffer();
        assert!(buffer_id > 0);
    }
    
    #[tokio::test]
    async fn create_shader() {
        let ctx = WebGLContext::new(WebGLVersion::WebGL1).await.unwrap();
        let shader_id = ctx.create_shader(crate::constants::VERTEX_SHADER);
        assert!(shader_id.is_ok());
    }
}
