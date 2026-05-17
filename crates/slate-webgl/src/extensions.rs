//! WebGL extensions.

/// WebGL extension.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Extension {
    /// ANGLE_instanced_arrays
    AngleInstancedArrays,
    
    /// EXT_blend_minmax
    ExtBlendMinmax,
    
    /// EXT_color_buffer_half_float
    ExtColorBufferHalfFloat,
    
    /// EXT_disjoint_timer_query
    ExtDisjointTimerQuery,
    
    /// EXT_frag_depth
    ExtFragDepth,
    
    /// EXT_shader_texture_lod
    ExtShaderTextureLod,
    
    /// EXT_texture_filter_anisotropic
    ExtTextureFilterAnisotropic,
    
    /// OES_element_index_uint
    OesElementIndexUint,
    
    /// OES_standard_derivatives
    OesStandardDerivatives,
    
    /// OES_texture_float
    OesTextureFloat,
    
    /// OES_texture_float_linear
    OesTextureFloatLinear,
    
    /// OES_texture_half_float
    OesTextureHalfFloat,
    
    /// OES_texture_half_float_linear
    OesTextureHalfFloatLinear,
    
    /// OES_vertex_array_object
    OesVertexArrayObject,
    
    /// WEBGL_color_buffer_float
    WebglColorBufferFloat,
    
    /// WEBGL_compressed_texture_s3tc
    WebglCompressedTextureS3tc,
    
    /// WEBGL_debug_renderer_info
    WebglDebugRendererInfo,
    
    /// WEBGL_debug_shaders
    WebglDebugShaders,
    
    /// WEBGL_depth_texture
    WebglDepthTexture,
    
    /// WEBGL_draw_buffers
    WebglDrawBuffers,
    
    /// WEBGL_lose_context
    WebglLoseContext,
}

impl Extension {
    /// Get extension name.
    pub fn name(&self) -> &'static str {
        match self {
            Extension::AngleInstancedArrays => "ANGLE_instanced_arrays",
            Extension::ExtBlendMinmax => "EXT_blend_minmax",
            Extension::ExtColorBufferHalfFloat => "EXT_color_buffer_half_float",
            Extension::ExtDisjointTimerQuery => "EXT_disjoint_timer_query",
            Extension::ExtFragDepth => "EXT_frag_depth",
            Extension::ExtShaderTextureLod => "EXT_shader_texture_lod",
            Extension::ExtTextureFilterAnisotropic => "EXT_texture_filter_anisotropic",
            Extension::OesElementIndexUint => "OES_element_index_uint",
            Extension::OesStandardDerivatives => "OES_standard_derivatives",
            Extension::OesTextureFloat => "OES_texture_float",
            Extension::OesTextureFloatLinear => "OES_texture_float_linear",
            Extension::OesTextureHalfFloat => "OES_texture_half_float",
            Extension::OesTextureHalfFloatLinear => "OES_texture_half_float_linear",
            Extension::OesVertexArrayObject => "OES_vertex_array_object",
            Extension::WebglColorBufferFloat => "WEBGL_color_buffer_float",
            Extension::WebglCompressedTextureS3tc => "WEBGL_compressed_texture_s3tc",
            Extension::WebglDebugRendererInfo => "WEBGL_debug_renderer_info",
            Extension::WebglDebugShaders => "WEBGL_debug_shaders",
            Extension::WebglDepthTexture => "WEBGL_depth_texture",
            Extension::WebglDrawBuffers => "WEBGL_draw_buffers",
            Extension::WebglLoseContext => "WEBGL_lose_context",
        }
    }
    
    /// Parse extension from name.
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ANGLE_instanced_arrays" => Some(Extension::AngleInstancedArrays),
            "EXT_blend_minmax" => Some(Extension::ExtBlendMinmax),
            "EXT_color_buffer_half_float" => Some(Extension::ExtColorBufferHalfFloat),
            "EXT_disjoint_timer_query" => Some(Extension::ExtDisjointTimerQuery),
            "EXT_frag_depth" => Some(Extension::ExtFragDepth),
            "EXT_shader_texture_lod" => Some(Extension::ExtShaderTextureLod),
            "EXT_texture_filter_anisotropic" => Some(Extension::ExtTextureFilterAnisotropic),
            "OES_element_index_uint" => Some(Extension::OesElementIndexUint),
            "OES_standard_derivatives" => Some(Extension::OesStandardDerivatives),
            "OES_texture_float" => Some(Extension::OesTextureFloat),
            "OES_texture_float_linear" => Some(Extension::OesTextureFloatLinear),
            "OES_texture_half_float" => Some(Extension::OesTextureHalfFloat),
            "OES_texture_half_float_linear" => Some(Extension::OesTextureHalfFloatLinear),
            "OES_vertex_array_object" => Some(Extension::OesVertexArrayObject),
            "WEBGL_color_buffer_float" => Some(Extension::WebglColorBufferFloat),
            "WEBGL_compressed_texture_s3tc" => Some(Extension::WebglCompressedTextureS3tc),
            "WEBGL_debug_renderer_info" => Some(Extension::WebglDebugRendererInfo),
            "WEBGL_debug_shaders" => Some(Extension::WebglDebugShaders),
            "WEBGL_depth_texture" => Some(Extension::WebglDepthTexture),
            "WEBGL_draw_buffers" => Some(Extension::WebglDrawBuffers),
            "WEBGL_lose_context" => Some(Extension::WebglLoseContext),
            _ => None,
        }
    }
}
