//! WebAssembly import resolution.

use std::collections::HashMap;

/// Import resolver.
pub struct ImportResolver {
    /// Imports by module name
    imports: HashMap<String, HashMap<String, ImportValue>>,
}

/// Import value.
#[derive(Debug, Clone)]
pub enum ImportValue {
    /// Function
    Function(u32),
    
    /// Memory
    Memory(u32),
    
    /// Table
    Table(u32),
    
    /// Global
    Global(u32),
}

impl ImportResolver {
    /// Create a new import resolver.
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
        }
    }
    
    /// Register an import.
    pub fn register(&mut self, module: String, name: String, value: ImportValue) {
        self.imports
            .entry(module)
            .or_insert_with(HashMap::new)
            .insert(name, value);
    }
    
    /// Resolve an import.
    pub fn resolve(&self, module: &str, name: &str) -> Option<&ImportValue> {
        self.imports.get(module)?.get(name)
    }
}

impl Default for ImportResolver {
    fn default() -> Self {
        Self::new()
    }
}
