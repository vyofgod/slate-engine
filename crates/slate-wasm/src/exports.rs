//! WebAssembly export handling.

use std::collections::HashMap;

/// Export manager.
pub struct ExportManager {
    /// Exports by name
    exports: HashMap<String, ExportValue>,
}

/// Export value.
#[derive(Debug, Clone)]
pub enum ExportValue {
    /// Function
    Function(u32),
    
    /// Memory
    Memory(u32),
    
    /// Table
    Table(u32),
    
    /// Global
    Global(u32),
}

impl ExportManager {
    /// Create a new export manager.
    pub fn new() -> Self {
        Self {
            exports: HashMap::new(),
        }
    }
    
    /// Register an export.
    pub fn register(&mut self, name: String, value: ExportValue) {
        self.exports.insert(name, value);
    }
    
    /// Get an export.
    pub fn get(&self, name: &str) -> Option<&ExportValue> {
        self.exports.get(name)
    }
    
    /// Get all export names.
    pub fn names(&self) -> Vec<String> {
        self.exports.keys().cloned().collect()
    }
}

impl Default for ExportManager {
    fn default() -> Self {
        Self::new()
    }
}
