//! Module system for AlBayan language
//! 
//! This module provides functionality for:
//! - Module definition and resolution
//! - Package management
//! - Import/export system
//! - Dependency resolution

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod resolver;
pub mod package;

/// Module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    /// Module name
    pub name: String,
    /// Module path
    pub path: PathBuf,
    /// Exported symbols
    pub exports: Vec<Export>,
    /// Imported modules
    pub imports: Vec<Import>,
    /// Module dependencies
    pub dependencies: Vec<String>,
    /// Module version
    pub version: String,
}

/// Export declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    /// Symbol name
    pub name: String,
    /// Symbol type
    pub symbol_type: ExportType,
    /// Visibility level
    pub visibility: Visibility,
}

/// Import declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    /// Module name to import from
    pub module: String,
    /// Imported symbols
    pub symbols: Vec<ImportSymbol>,
    /// Import alias
    pub alias: Option<String>,
}

/// Imported symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSymbol {
    /// Symbol name
    pub name: String,
    /// Local alias
    pub alias: Option<String>,
}

/// Export types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportType {
    Function,
    Struct,
    Enum,
    Class,
    Interface,
    Relation,
    Constant,
    Type,
    Module,
}

/// Visibility levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,    // Visible to all modules
    Protected, // Visible to submodules
    Private,   // Visible only within module
}

/// Module registry for managing all modules
#[derive(Debug)]
pub struct ModuleRegistry {
    /// Registered modules
    modules: HashMap<String, Module>,
    /// Module search paths
    search_paths: Vec<PathBuf>,
    /// Dependency graph
    dependency_graph: HashMap<String, HashSet<String>>,
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            search_paths: vec![
                PathBuf::from("./"),
                PathBuf::from("./modules/"),
                PathBuf::from("/usr/local/lib/albayan/"),
            ],
            dependency_graph: HashMap::new(),
        }
    }
    
    /// Register a module
    pub fn register_module(&mut self, module: Module) -> Result<()> {
        // Check for circular dependencies
        self.check_circular_dependencies(&module)?;
        
        // Add to dependency graph
        let deps: HashSet<String> = module.dependencies.iter().cloned().collect();
        self.dependency_graph.insert(module.name.clone(), deps);
        
        // Register the module
        self.modules.insert(module.name.clone(), module);
        
        Ok(())
    }
    
    /// Resolve a module by name
    pub fn resolve_module(&self, name: &str) -> Option<&Module> {
        self.modules.get(name)
    }
    
    /// Get module dependencies in topological order
    pub fn get_dependency_order(&self, module_name: &str) -> Result<Vec<String>> {
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        let mut result = Vec::new();
        
        self.topological_sort(module_name, &mut visited, &mut temp_visited, &mut result)?;
        
        Ok(result)
    }
    
    /// Check for circular dependencies
    fn check_circular_dependencies(&self, module: &Module) -> Result<()> {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for dep in &module.dependencies {
            if self.has_cycle(dep, &mut visited, &mut rec_stack)? {
                return Err(anyhow::anyhow!("Circular dependency detected involving module: {}", dep));
            }
        }
        
        Ok(())
    }
    
    /// Check if there's a cycle in dependencies
    fn has_cycle(
        &self,
        module_name: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<bool> {
        if rec_stack.contains(module_name) {
            return Ok(true);
        }
        
        if visited.contains(module_name) {
            return Ok(false);
        }
        
        visited.insert(module_name.to_string());
        rec_stack.insert(module_name.to_string());
        
        if let Some(deps) = self.dependency_graph.get(module_name) {
            for dep in deps {
                if self.has_cycle(dep, visited, rec_stack)? {
                    return Ok(true);
                }
            }
        }
        
        rec_stack.remove(module_name);
        Ok(false)
    }
    
    /// Topological sort for dependency resolution
    fn topological_sort(
        &self,
        module_name: &str,
        visited: &mut HashSet<String>,
        temp_visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.contains(module_name) {
            return Err(anyhow::anyhow!("Circular dependency detected"));
        }
        
        if visited.contains(module_name) {
            return Ok(());
        }
        
        temp_visited.insert(module_name.to_string());
        
        if let Some(deps) = self.dependency_graph.get(module_name) {
            for dep in deps {
                self.topological_sort(dep, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.remove(module_name);
        visited.insert(module_name.to_string());
        result.push(module_name.to_string());
        
        Ok(())
    }
    
    /// Add a search path for modules
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }
    
    /// Find module file in search paths
    pub fn find_module_file(&self, module_name: &str) -> Option<PathBuf> {
        for search_path in &self.search_paths {
            let module_file = search_path.join(format!("{}.ab", module_name));
            if module_file.exists() {
                return Some(module_file);
            }
        }
        None
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Module loader for loading and parsing modules
#[derive(Debug)]
pub struct ModuleLoader {
    registry: ModuleRegistry,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new() -> Self {
        Self {
            registry: ModuleRegistry::new(),
        }
    }
    
    /// Load a module from file
    pub fn load_module(&mut self, module_path: &Path) -> Result<Module> {
        // TODO: Parse module file and extract module information
        let module_name = module_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let module = Module {
            name: module_name,
            path: module_path.to_path_buf(),
            exports: Vec::new(),
            imports: Vec::new(),
            dependencies: Vec::new(),
            version: "1.0.0".to_string(),
        };
        
        self.registry.register_module(module.clone())?;
        Ok(module)
    }
    
    /// Get the module registry
    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }
    
    /// Get mutable access to the module registry
    pub fn registry_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.registry
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}
