//! Module resolution system for AlBayan
//! 
//! Handles symbol resolution across modules and manages import/export relationships

use super::{Module, Import, Export, ImportSymbol, ExportType, Visibility};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, anyhow};

/// Symbol resolution context
#[derive(Debug)]
pub struct ResolutionContext {
    /// Current module being resolved
    current_module: String,
    /// Available modules
    modules: HashMap<String, Module>,
    /// Resolved symbols cache
    symbol_cache: HashMap<String, ResolvedSymbol>,
    /// Import aliases
    aliases: HashMap<String, String>,
}

/// Resolved symbol information
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    /// Symbol name
    pub name: String,
    /// Module where symbol is defined
    pub module: String,
    /// Symbol type
    pub symbol_type: ExportType,
    /// Full qualified name
    pub qualified_name: String,
}

/// Symbol resolver
#[derive(Debug)]
pub struct SymbolResolver {
    /// Resolution contexts for each module
    contexts: HashMap<String, ResolutionContext>,
}

impl SymbolResolver {
    /// Create a new symbol resolver
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }
    
    /// Add a module to the resolver
    pub fn add_module(&mut self, module: Module) -> Result<()> {
        let context = ResolutionContext {
            current_module: module.name.clone(),
            modules: HashMap::new(),
            symbol_cache: HashMap::new(),
            aliases: HashMap::new(),
        };
        
        self.contexts.insert(module.name.clone(), context);
        
        // Add module to all contexts
        for context in self.contexts.values_mut() {
            context.modules.insert(module.name.clone(), module.clone());
        }
        
        Ok(())
    }
    
    /// Resolve a symbol in the given module context
    pub fn resolve_symbol(&mut self, module_name: &str, symbol_name: &str) -> Result<ResolvedSymbol> {
        // Check cache first
        let cache_key = format!("{}::{}", module_name, symbol_name);
        if let Some(context) = self.contexts.get(module_name) {
            if let Some(cached) = context.symbol_cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Try to resolve the symbol
        let resolved = {
            let context = self.contexts.get(module_name)
                .ok_or_else(|| anyhow!("Module not found: {}", module_name))?;
            self.resolve_symbol_internal(context, symbol_name)?
        };

        // Cache the result
        if let Some(context) = self.contexts.get_mut(module_name) {
            context.symbol_cache.insert(cache_key, resolved.clone());
        }

        Ok(resolved)
    }
    
    /// Internal symbol resolution logic
    fn resolve_symbol_internal(&self, context: &ResolutionContext, symbol_name: &str) -> Result<ResolvedSymbol> {
        // First, check if it's an aliased import
        if let Some(original_name) = context.aliases.get(symbol_name) {
            return self.resolve_symbol_internal(context, original_name);
        }
        
        // Check current module exports
        if let Some(module) = context.modules.get(&context.current_module) {
            for export in &module.exports {
                if export.name == symbol_name {
                    return Ok(ResolvedSymbol {
                        name: symbol_name.to_string(),
                        module: context.current_module.clone(),
                        symbol_type: export.symbol_type.clone(),
                        qualified_name: format!("{}::{}", context.current_module, symbol_name),
                    });
                }
            }
        }
        
        // Check imported symbols
        if let Some(module) = context.modules.get(&context.current_module) {
            for import in &module.imports {
                for import_symbol in &import.symbols {
                    let symbol_to_check = import_symbol.alias.as_ref().unwrap_or(&import_symbol.name);
                    
                    if symbol_to_check == symbol_name {
                        // Find the symbol in the imported module
                        if let Some(imported_module) = context.modules.get(&import.module) {
                            for export in &imported_module.exports {
                                if export.name == import_symbol.name {
                                    // Check visibility
                                    if !self.is_symbol_accessible(&export.visibility, &context.current_module, &import.module) {
                                        return Err(anyhow!("Symbol '{}' is not accessible from module '{}'", 
                                                         symbol_name, context.current_module));
                                    }
                                    
                                    return Ok(ResolvedSymbol {
                                        name: symbol_name.to_string(),
                                        module: import.module.clone(),
                                        symbol_type: export.symbol_type.clone(),
                                        qualified_name: format!("{}::{}", import.module, import_symbol.name),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Err(anyhow!("Symbol '{}' not found in module '{}'", symbol_name, context.current_module))
    }
    
    /// Check if a symbol is accessible based on visibility rules
    fn is_symbol_accessible(&self, visibility: &Visibility, from_module: &str, to_module: &str) -> bool {
        match visibility {
            Visibility::Public => true,
            Visibility::Protected => {
                // Check if from_module is a submodule of to_module
                from_module.starts_with(&format!("{}::", to_module))
            }
            Visibility::Private => from_module == to_module,
        }
    }
    
    /// Resolve all imports for a module
    pub fn resolve_imports(&mut self, module_name: &str) -> Result<Vec<ResolvedSymbol>> {
        let mut resolved_imports = Vec::new();
        
        let context = self.contexts.get(module_name)
            .ok_or_else(|| anyhow!("Module not found: {}", module_name))?;
        
        if let Some(module) = context.modules.get(module_name) {
            for import in &module.imports {
                for import_symbol in &import.symbols {
                    let symbol_name = import_symbol.alias.as_ref().unwrap_or(&import_symbol.name);
                    let resolved = self.resolve_symbol_internal(context, symbol_name)?;
                    resolved_imports.push(resolved);
                }
            }
        }
        
        Ok(resolved_imports)
    }
    
    /// Get all exported symbols from a module
    pub fn get_module_exports(&self, module_name: &str) -> Result<Vec<ResolvedSymbol>> {
        let context = self.contexts.get(module_name)
            .ok_or_else(|| anyhow!("Module not found: {}", module_name))?;
        
        let module = context.modules.get(module_name)
            .ok_or_else(|| anyhow!("Module not found: {}", module_name))?;
        
        let mut exports = Vec::new();
        for export in &module.exports {
            exports.push(ResolvedSymbol {
                name: export.name.clone(),
                module: module_name.to_string(),
                symbol_type: export.symbol_type.clone(),
                qualified_name: format!("{}::{}", module_name, export.name),
            });
        }
        
        Ok(exports)
    }
    
    /// Check for naming conflicts in imports
    pub fn check_import_conflicts(&self, module_name: &str) -> Result<Vec<String>> {
        let mut conflicts = Vec::new();
        let mut symbol_names = HashSet::new();
        
        let context = self.contexts.get(module_name)
            .ok_or_else(|| anyhow!("Module not found: {}", module_name))?;
        
        if let Some(module) = context.modules.get(module_name) {
            for import in &module.imports {
                for import_symbol in &import.symbols {
                    let symbol_name = import_symbol.alias.as_ref().unwrap_or(&import_symbol.name);
                    
                    if !symbol_names.insert(symbol_name.clone()) {
                        conflicts.push(format!("Conflicting import: {}", symbol_name));
                    }
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// Generate import graph for visualization
    pub fn generate_import_graph(&self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        
        for (module_name, context) in &self.contexts {
            let mut dependencies = Vec::new();
            
            if let Some(module) = context.modules.get(module_name) {
                for import in &module.imports {
                    dependencies.push(import.module.clone());
                }
            }
            
            graph.insert(module_name.clone(), dependencies);
        }
        
        graph
    }
}

impl Default for SymbolResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Import analyzer for static analysis of imports
#[derive(Debug)]
pub struct ImportAnalyzer {
    resolver: SymbolResolver,
}

impl ImportAnalyzer {
    /// Create a new import analyzer
    pub fn new(resolver: SymbolResolver) -> Self {
        Self { resolver }
    }
    
    /// Analyze unused imports in a module
    pub fn find_unused_imports(&self, module_name: &str) -> Result<Vec<String>> {
        // TODO: Implement unused import detection
        // This would require analyzing the AST to see which imported symbols are actually used
        Ok(Vec::new())
    }
    
    /// Analyze circular imports
    pub fn find_circular_imports(&self) -> Result<Vec<Vec<String>>> {
        let graph = self.resolver.generate_import_graph();
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for module in graph.keys() {
            if !visited.contains(module) {
                if let Some(cycle) = self.find_cycle_dfs(module, &graph, &mut visited, &mut rec_stack, &mut Vec::new()) {
                    cycles.push(cycle);
                }
            }
        }
        
        Ok(cycles)
    }
    
    /// DFS to find cycles in import graph
    fn find_cycle_dfs(
        &self,
        module: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(module.to_string());
        rec_stack.insert(module.to_string());
        path.push(module.to_string());
        
        if let Some(dependencies) = graph.get(module) {
            for dep in dependencies {
                if !visited.contains(dep) {
                    if let Some(cycle) = self.find_cycle_dfs(dep, graph, visited, rec_stack, path) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    // Found a cycle
                    let cycle_start = path.iter().position(|m| m == dep).unwrap();
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }
        
        rec_stack.remove(module);
        path.pop();
        None
    }
}
