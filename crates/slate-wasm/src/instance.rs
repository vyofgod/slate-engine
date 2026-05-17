//! WebAssembly instance management.

use crate::{Result, WasmError, Module, Value};
use wasmtime::{Instance as WasmtimeInstance, Store, Linker};
use std::sync::Arc;
use parking_lot::Mutex;

/// WebAssembly instance.
///
/// An instantiated WebAssembly module with its own memory and state.
pub struct Instance {
    /// Instance ID
    id: u32,
    
    /// Wasmtime store
    store: Arc<Mutex<Store<()>>>,
    
    /// Wasmtime instance
    instance: WasmtimeInstance,
}

impl Instance {
    /// Create a new instance from a module.
    pub fn new(id: u32, module: &Module, imports: Linker<()>) -> Result<Self> {
        let mut store = Store::new(module.engine(), ());
        
        // Instantiate module
        let instance = imports
            .instantiate(&mut store, module.wasmtime_module())
            .map_err(|e| WasmError::InstantiationFailed(e.to_string()))?;
        
        Ok(Self {
            id,
            store: Arc::new(Mutex::new(store)),
            instance,
        })
    }
    
    /// Get instance ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Call an exported function.
    pub fn call(&self, name: &str, args: &[Value]) -> Result<Vec<Value>> {
        let mut store = self.store.lock();
        
        // Get exported function
        let func = self.instance
            .get_func(&mut *store, name)
            .ok_or_else(|| WasmError::ExportNotFound(name.to_string()))?;
        
        // Convert arguments
        let wasm_args: Vec<wasmtime::Val> = args.iter().map(|v| match v {
            Value::I32(x) => wasmtime::Val::I32(*x),
            Value::I64(x) => wasmtime::Val::I64(*x),
            Value::F32(x) => wasmtime::Val::F32(x.to_bits()),
            Value::F64(x) => wasmtime::Val::F64(x.to_bits()),
            _ => wasmtime::Val::I32(0), // TODO: Handle other types
        }).collect();
        
        // Call function
        let mut results = vec![wasmtime::Val::I32(0); func.ty(&*store).results().len()];
        func.call(&mut *store, &wasm_args, &mut results)
            .map_err(|e| WasmError::ExecutionFailed(e.to_string()))?;
        
        // Convert results
        let values: Vec<Value> = results.iter().map(|v| match v {
            wasmtime::Val::I32(x) => Value::I32(*x),
            wasmtime::Val::I64(x) => Value::I64(*x),
            wasmtime::Val::F32(x) => Value::F32(f32::from_bits(*x)),
            wasmtime::Val::F64(x) => Value::F64(f64::from_bits(*x)),
            _ => Value::I32(0), // TODO: Handle other types
        }).collect();
        
        Ok(values)
    }
    
    /// Get exported memory.
    pub fn memory(&self, name: &str) -> Result<Vec<u8>> {
        let mut store = self.store.lock();
        
        let memory = self.instance
            .get_memory(&mut *store, name)
            .ok_or_else(|| WasmError::ExportNotFound(name.to_string()))?;
        
        Ok(memory.data(&*store).to_vec())
    }
    
    /// Write to exported memory.
    pub fn write_memory(&self, name: &str, offset: usize, data: &[u8]) -> Result<()> {
        let mut store = self.store.lock();
        
        let memory = self.instance
            .get_memory(&mut *store, name)
            .ok_or_else(|| WasmError::ExportNotFound(name.to_string()))?;
        
        let mem_data = memory.data_mut(&mut *store);
        
        if offset + data.len() > mem_data.len() {
            return Err(WasmError::OutOfBounds("Memory write out of bounds".to_string()));
        }
        
        mem_data[offset..offset + data.len()].copy_from_slice(data);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WasmConfig;
    
    #[test]
    fn instantiate_and_call() {
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
        let module = Module::from_wat(1, wat, &config).unwrap();
        
        let linker = Linker::new(module.engine());
        let instance = Instance::new(1, &module, linker).unwrap();
        
        let result = instance.call("add", &[Value::I32(5), Value::I32(7)]).unwrap();
        assert_eq!(result.len(), 1);
        
        if let Value::I32(x) = result[0] {
            assert_eq!(x, 12);
        } else {
            panic!("Expected I32 result");
        }
    }
}
