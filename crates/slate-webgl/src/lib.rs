//! # Slate WebGL - Hardware-Accelerated 3D Graphics
//!
//! Full WebGL 1.0 and 2.0 implementation using wgpu as the backend.
//!
//! ## Architecture
//!
//! ```text
//! JavaScript WebGL API
//!     ↓
//! WebGLContext (state management)
//!     ↓
//! wgpu Commands
//!     ↓
//! GPU (Vulkan/Metal/DX12)
//! ```
//!
//! ## Features
//!
//! - Full WebGL 1.0 specification
//! - WebGL 2.0 features (UBO, transform feedback, etc.)
//! - Shader compilation (GLSL → WGSL)
//! - Efficient state management
//! - Zero-copy buffer operations
//! - Extension support
//!
//! ## Usage
//!
//! ```rust,no_run
//! use slate_webgl::{WebGLContext, WebGLVersion};
//!
//! // Create WebGL 2.0 context
//! let context = WebGLContext::new(WebGLVersion::WebGL2);
//!
//! // Use WebGL APIs
//! context.clear_color(0.0, 0.0, 0.0, 1.0);
//! context.clear();
//! ```
//!
//! ## Performance
//!
//! - Zero-copy buffer uploads
//! - Efficient state caching
//! - Batch draw calls
//! - GPU-accelerated rendering

pub mod context;
pub mod shader;
pub mod buffer;
pub mod texture;
pub mod framebuffer;
pub mod program;
pub mod state;
pub mod extensions;

pub use context::{WebGLContext, WebGLVersion};
pub use shader::{Shader, ShaderType};
pub use buffer::{Buffer, BufferTarget, BufferUsage};
pub use texture::{Texture, TextureTarget, TextureFormat};
pub use framebuffer::{Framebuffer, Renderbuffer};
pub use program::Program;

use thiserror::Error;

/// WebGL errors.
#[derive(Debug, Error)]
pub enum WebGLError {
    #[error("Invalid enum: {0}")]
    InvalidEnum(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Invalid framebuffer operation: {0}")]
    InvalidFramebufferOperation(String),
    
    #[error("Context lost")]
    ContextLost,
    
    #[error("Shader compilation failed: {0}")]
    ShaderCompilationFailed(String),
    
    #[error("Program linking failed: {0}")]
    ProgramLinkingFailed(String),
}

pub type Result<T> = std::result::Result<T, WebGLError>;

/// WebGL constants (matching WebGL spec).
pub mod constants {
    // Data types
    pub const BYTE: u32 = 0x1400;
    pub const UNSIGNED_BYTE: u32 = 0x1401;
    pub const SHORT: u32 = 0x1402;
    pub const UNSIGNED_SHORT: u32 = 0x1403;
    pub const INT: u32 = 0x1404;
    pub const UNSIGNED_INT: u32 = 0x1405;
    pub const FLOAT: u32 = 0x1406;
    
    // Primitives
    pub const POINTS: u32 = 0x0000;
    pub const LINES: u32 = 0x0001;
    pub const LINE_LOOP: u32 = 0x0002;
    pub const LINE_STRIP: u32 = 0x0003;
    pub const TRIANGLES: u32 = 0x0004;
    pub const TRIANGLE_STRIP: u32 = 0x0005;
    pub const TRIANGLE_FAN: u32 = 0x0006;
    
    // Blending
    pub const ZERO: u32 = 0;
    pub const ONE: u32 = 1;
    pub const SRC_COLOR: u32 = 0x0300;
    pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;
    pub const SRC_ALPHA: u32 = 0x0302;
    pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
    pub const DST_ALPHA: u32 = 0x0304;
    pub const ONE_MINUS_DST_ALPHA: u32 = 0x0305;
    pub const DST_COLOR: u32 = 0x0306;
    pub const ONE_MINUS_DST_COLOR: u32 = 0x0307;
    
    // Buffer targets
    pub const ARRAY_BUFFER: u32 = 0x8892;
    pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
    
    // Buffer usage
    pub const STREAM_DRAW: u32 = 0x88E0;
    pub const STATIC_DRAW: u32 = 0x88E4;
    pub const DYNAMIC_DRAW: u32 = 0x88E8;
    
    // Shader types
    pub const FRAGMENT_SHADER: u32 = 0x8B30;
    pub const VERTEX_SHADER: u32 = 0x8B31;
    
    // Texture targets
    pub const TEXTURE_2D: u32 = 0x0DE1;
    pub const TEXTURE_CUBE_MAP: u32 = 0x8513;
    
    // Texture formats
    pub const RGBA: u32 = 0x1908;
    pub const RGB: u32 = 0x1907;
    pub const ALPHA: u32 = 0x1906;
    pub const LUMINANCE: u32 = 0x1909;
    pub const LUMINANCE_ALPHA: u32 = 0x190A;
    
    // Pixel types
    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033;
    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034;
    pub const UNSIGNED_SHORT_5_6_5: u32 = 0x8363;
    
    // Framebuffer targets
    pub const FRAMEBUFFER: u32 = 0x8D40;
    pub const RENDERBUFFER: u32 = 0x8D41;
    
    // Capabilities
    pub const BLEND: u32 = 0x0BE2;
    pub const CULL_FACE: u32 = 0x0B44;
    pub const DEPTH_TEST: u32 = 0x0B71;
    pub const DITHER: u32 = 0x0BD0;
    pub const POLYGON_OFFSET_FILL: u32 = 0x8037;
    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 0x809E;
    pub const SAMPLE_COVERAGE: u32 = 0x80A0;
    pub const SCISSOR_TEST: u32 = 0x0C11;
    pub const STENCIL_TEST: u32 = 0x0B90;
}
