//! WebAssembly module management.

use crate::{Result, WasmError, WasmConfig};
use wasmtime::{Engine, Module as WasmtimeModule};
use std::sync::Arc;

/// WebAssembly module.
///
/// A compiled WebAssembly module ready for instantiation.
pub struct Module {
    /// Module ID
    id: u32,
    
    /// Wasmtime engine
    engine: Arc<Engine>,
    
    /// Compiled module
    module: WasmtimeModule,
}

impl Module {
    /// Compile a module from bytes.
    pub fn from_bytes(id: u32, bytes: &[u8], config: &WasmConfig) -> Result<Self> {
        // Create engine with config
        let mut engine_config = wasmtime::Config::new();
        
        // Enable features based on config
        engine_config.wasm_simd(config.simd);
        engine_config.wasm_threads(config.threads);
        engine_config.wasm_bulk_memory(config.bulk_memory);
        engine_config.wasm_reference_types(config.reference_types);
        
        // Enable JIT if configured
        if config.jit {
            engine_config.strategy(wasmtime::Strategy::Cranelift);
        }
        
        let engine = Engine::new(&engine_config)
            .map_err(|e| WasmError::CompilationFailed(e.to_string()))?;
        
        // Compile module
        let module = WasmtimeModule::from_binary(&engine, bytes)
            .map_err(|e| WasmError::CompilationFailed(e.to_string()))?;
        
        Ok(Self {
            id,
            engine: Arc::new(engine),
            module,
        })
    }
    
    /// Compile a module from WAT (WebAssembly Text).
    pub fn from_wat(id: u32, wat: &str, config: &WasmConfig) -> Result<Self> {
        let bytes = wat::parse_str(wat)
            .map_err(|e| WasmError::CompilationFailed(e.to_string()))?;
        
        Self::from_bytes(id, &bytes, config)
    }
    
    /// Get module ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get engine.
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
    
    /// Get wasmtime module.
    pub fn wasmtime_module(&self) -> &WasmtimeModule {
        &self.module
    }
    
    /// Get module imports.
    pub fn imports(&self) -> Vec<(String, String, String)> {
        self.module
            .imports()
            .map(|import| {
                (
                    import.module().to_string(),
                    import.name().to_string(),
                    format!("{:?}", import.ty()),
                )
            })
            .collect()
    }
    
    /// Get module exports.
    pub fn exports(&self) -> Vec<(String, String)> {
        self.module
            .exports()
            .map(|export| {
                (
                    export.name().to_string(),
                    format!("{:?}", export.ty()),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn compile_simple_module() {
        let wat = r#"
            (module
                (func (export "add") (param i32 i32) (result i32)
                    local.get 0
                    local.get 1
                    i32.add
                )
            )
        "#;
        
        let config = WasmConfig::default();
        let module = Module::from_wat(1, wat, &config);
        assert!(module.is_ok());
    }
    
    #[test]
    fn module_exports() {
        let wat = r#"
            (module
                (func (export "test") (result i32)
                    i32.const 42
                )
            )
        "#;
        
        let config = WasmConfig::default();
        let module = Module::from_wat(1, wat, &config).unwrap();
        let exports = module.exports();
        
        assert_eq!(exports.len(), 1);
        assert_eq!(exports[0].0, "test");
    }
}
