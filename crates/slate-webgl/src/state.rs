//! WebGL state management.

use std::collections::HashMap;

/// WebGL rendering state.
///
/// Tracks all WebGL state to minimize redundant GPU commands.
#[derive(Debug, Clone)]
pub struct WebGLState {
    // Buffer bindings
    pub array_buffer: Option<u32>,
    pub element_array_buffer: Option<u32>,
    
    // Program binding
    pub current_program: Option<u32>,
    
    // Viewport and scissor
    pub viewport: (i32, i32, i32, i32),
    pub scissor: (i32, i32, i32, i32),
    
    // Clear values
    pub clear_color: (f32, f32, f32, f32),
    pub clear_depth: f32,
    pub clear_stencil: i32,
    
    // Capabilities
    pub capabilities: HashMap<u32, bool>,
    
    // Blend state
    pub blend_src: u32,
    pub blend_dst: u32,
    
    // Depth state
    pub depth_func: u32,
    pub depth_mask: bool,
    
    // Stencil state
    pub stencil_func: u32,
    pub stencil_ref: i32,
    pub stencil_mask: u32,
    
    // Texture bindings
    pub active_texture: u32,
    pub texture_bindings: HashMap<u32, HashMap<u32, u32>>, // unit -> (target -> texture_id)
    
    // Framebuffer bindings
    pub framebuffer: Option<u32>,
    pub renderbuffer: Option<u32>,
    
    // Vertex attributes
    pub vertex_attrib_enabled: HashMap<u32, bool>,
}

impl WebGLState {
    /// Create a new WebGL state with default values.
    pub fn new() -> Self {
        let mut capabilities = HashMap::new();
        capabilities.insert(crate::constants::BLEND, false);
        capabilities.insert(crate::constants::CULL_FACE, false);
        capabilities.insert(crate::constants::DEPTH_TEST, false);
        capabilities.insert(crate::constants::DITHER, true);
        capabilities.insert(crate::constants::SCISSOR_TEST, false);
        capabilities.insert(crate::constants::STENCIL_TEST, false);
        
        Self {
            array_buffer: None,
            element_array_buffer: None,
            current_program: None,
            viewport: (0, 0, 0, 0),
            scissor: (0, 0, 0, 0),
            clear_color: (0.0, 0.0, 0.0, 0.0),
            clear_depth: 1.0,
            clear_stencil: 0,
            capabilities,
            blend_src: crate::constants::ONE,
            blend_dst: crate::constants::ZERO,
            depth_func: 0x0201, // LESS
            depth_mask: true,
            stencil_func: 0x0207, // ALWAYS
            stencil_ref: 0,
            stencil_mask: 0xFFFFFFFF,
            active_texture: 0x84C0, // TEXTURE0
            texture_bindings: HashMap::new(),
            framebuffer: None,
            renderbuffer: None,
            vertex_attrib_enabled: HashMap::new(),
        }
    }
    
    /// Bind a buffer.
    pub fn bind_buffer(&mut self, target: u32, id: u32) {
        match target {
            crate::constants::ARRAY_BUFFER => {
                self.array_buffer = Some(id);
            }
            crate::constants::ELEMENT_ARRAY_BUFFER => {
                self.element_array_buffer = Some(id);
            }
            _ => {}
        }
    }
    
    /// Get bound buffer.
    pub fn get_bound_buffer(&self, target: u32) -> Option<u32> {
        match target {
            crate::constants::ARRAY_BUFFER => self.array_buffer,
            crate::constants::ELEMENT_ARRAY_BUFFER => self.element_array_buffer,
            _ => None,
        }
    }
    
    /// Use a program.
    pub fn use_program(&mut self, id: u32) {
        self.current_program = Some(id);
    }
    
    /// Set viewport.
    pub fn set_viewport(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.viewport = (x, y, width, height);
    }
    
    /// Set scissor.
    pub fn set_scissor(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.scissor = (x, y, width, height);
    }
    
    /// Set clear color.
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.clear_color = (r, g, b, a);
    }
    
    /// Enable a capability.
    pub fn enable(&mut self, cap: u32) {
        self.capabilities.insert(cap, true);
    }
    
    /// Disable a capability.
    pub fn disable(&mut self, cap: u32) {
        self.capabilities.insert(cap, false);
    }
    
    /// Check if capability is enabled.
    pub fn is_enabled(&self, cap: u32) -> bool {
        self.capabilities.get(&cap).copied().unwrap_or(false)
    }
    
    /// Set blend function.
    pub fn set_blend_func(&mut self, sfactor: u32, dfactor: u32) {
        self.blend_src = sfactor;
        self.blend_dst = dfactor;
    }
    
    /// Set depth function.
    pub fn set_depth_func(&mut self, func: u32) {
        self.depth_func = func;
    }
    
    /// Bind texture.
    pub fn bind_texture(&mut self, target: u32, id: u32) {
        let unit = self.active_texture;
        self.texture_bindings
            .entry(unit)
            .or_insert_with(HashMap::new)
            .insert(target, id);
    }
    
    /// Set active texture unit.
    pub fn active_texture(&mut self, texture: u32) {
        self.active_texture = texture;
    }
    
    /// Enable vertex attribute.
    pub fn enable_vertex_attrib_array(&mut self, index: u32) {
        self.vertex_attrib_enabled.insert(index, true);
    }
    
    /// Disable vertex attribute.
    pub fn disable_vertex_attrib_array(&mut self, index: u32) {
        self.vertex_attrib_enabled.insert(index, false);
    }
}

impl Default for WebGLState {
    fn default() -> Self {
        Self::new()
    }
}
