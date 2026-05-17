//! WebGL buffer management.

use crate::{Result, WebGLError};
use parking_lot::RwLock;
use std::sync::Arc;

/// Buffer target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferTarget {
    /// Array buffer (vertex data)
    ArrayBuffer,
    
    /// Element array buffer (indices)
    ElementArrayBuffer,
}

/// Buffer usage hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    /// Stream draw (data changes frequently)
    StreamDraw,
    
    /// Static draw (data changes rarely)
    StaticDraw,
    
    /// Dynamic draw (data changes occasionally)
    DynamicDraw,
}

/// WebGL buffer.
pub struct Buffer {
    /// Buffer ID
    id: u32,
    
    /// wgpu buffer
    buffer: Arc<RwLock<Option<wgpu::Buffer>>>,
    
    /// Buffer size in bytes
    size: Arc<RwLock<u64>>,
    
    /// Buffer usage
    usage: Arc<RwLock<BufferUsage>>,
}

impl Buffer {
    /// Create a new buffer.
    pub fn new(id: u32, _device: Arc<wgpu::Device>) -> Self {
        Self {
            id,
            buffer: Arc::new(RwLock::new(None)),
            size: Arc::new(RwLock::new(0)),
            usage: Arc::new(RwLock::new(BufferUsage::StaticDraw)),
        }
    }
    
    /// Get buffer ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Set buffer data.
    pub fn set_data(
        &self,
        data: &[u8],
        usage_const: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        let usage = match usage_const {
            crate::constants::STREAM_DRAW => BufferUsage::StreamDraw,
            crate::constants::STATIC_DRAW => BufferUsage::StaticDraw,
            crate::constants::DYNAMIC_DRAW => BufferUsage::DynamicDraw,
            _ => return Err(WebGLError::InvalidEnum("Invalid buffer usage".to_string())),
        };
        
        *self.usage.write() = usage;
        *self.size.write() = data.len() as u64;
        
        // Create wgpu buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("Buffer {}", self.id)),
            size: data.len() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Write data
        queue.write_buffer(&buffer, 0, data);
        
        *self.buffer.write() = Some(buffer);
        
        Ok(())
    }
    
    /// Get buffer size.
    pub fn size(&self) -> u64 {
        *self.size.read()
    }
    
    /// Get buffer usage.
    pub fn usage(&self) -> BufferUsage {
        *self.usage.read()
    }
    
    /// Get wgpu buffer.
    pub fn wgpu_buffer(&self) -> Option<wgpu::Buffer> {
        self.buffer.read().clone()
    }
}
