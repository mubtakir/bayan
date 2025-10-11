//! # Improved Semantic Analyzer
//! 
//! This module implements an improved semantic analyzer with better scope management
//! using RAII pattern and improved context handling.

use std::collections::HashMap;
use crate::parser::ast::*;

/// Unique identifier for scopes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

/// Symbol information stored in the symbol table
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: Type,
    pub scope_id: ScopeId,
    pub is_mutable: bool,
    pub is_initialized: bool,
}

/// Scope information
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String, Symbol>,
    pub scope_type: ScopeType,
}

/// Type of scope for different semantic rules
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Loop,
    Conditional,
}

/// Improved symbol table with hierarchical scope management
#[derive(Debug)]
pub struct SymbolTable {
    scopes: HashMap<ScopeId, Scope>,
    current_scope: ScopeId,
    next_scope_id: usize,
}

impl SymbolTable {
    /// Create a new symbol table with global scope
    pub fn new() -> Self {
        let global_scope_id = ScopeId(0);
        let mut scopes = HashMap::new();
        
        scopes.insert(global_scope_id, Scope {
            id: global_scope_id,
            parent: None,
            symbols: HashMap::new(),
            scope_type: ScopeType::Global,
        });
        
        Self {
            scopes,
            current_scope: global_scope_id,
            next_scope_id: 1,
        }
    }
    
    /// Enter a new scope and return a RAII guard
    pub fn enter_scope(&mut self, scope_type: ScopeType) -> ScopeGuard {
        let new_scope_id = ScopeId(self.next_scope_id);
        self.next_scope_id += 1;
        
        let new_scope = Scope {
            id: new_scope_id,
            parent: Some(self.current_scope),
            symbols: HashMap::new(),
            scope_type,
        };
        
        self.scopes.insert(new_scope_id, new_scope);
        let previous_scope = self.current_scope;
        self.current_scope = new_scope_id;
        
        ScopeGuard {
            symbol_table: self,
            previous_scope,
        }
    }
    
    /// Leave the current scope (called by ScopeGuard)
    fn leave_scope(&mut self, previous_scope: ScopeId) {
        self.current_scope = previous_scope;
    }
    
    /// Add a symbol to the current scope
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<(), SemanticError> {
        let current_scope = self.scopes.get_mut(&self.current_scope)
            .ok_or(SemanticError::InternalError("Current scope not found".to_string()))?;
        
        if current_scope.symbols.contains_key(&symbol.name) {
            return Err(SemanticError::SymbolAlreadyDefined(symbol.name));
        }
        
        current_scope.symbols.insert(symbol.name.clone(), symbol);
        Ok(())
    }
    
    /// Look up a symbol in the current scope and parent scopes
    pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        let mut current_scope_id = self.current_scope;
        
        loop {
            let scope = self.scopes.get(&current_scope_id)?;
            
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
            
            match scope.parent {
                Some(parent_id) => current_scope_id = parent_id,
                None => return None,
            }
        }
    }
    
    /// Get the current scope ID
    pub fn current_scope_id(&self) -> ScopeId {
        self.current_scope
    }
    
    /// Get scope information
    pub fn get_scope(&self, scope_id: ScopeId) -> Option<&Scope> {
        self.scopes.get(&scope_id)
    }
}

/// RAII guard for automatic scope management
pub struct ScopeGuard<'a> {
    symbol_table: &'a mut SymbolTable,
    previous_scope: ScopeId,
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.symbol_table.leave_scope(self.previous_scope);
    }
}

/// Analysis context for semantic analysis
#[derive(Debug)]
pub struct AnalysisContext<'a> {
    pub symbol_table: &'a mut SymbolTable,
    pub type_system: &'a mut TypeSystem,
    pub current_function: Option<&'a FunctionDecl>,
    pub in_loop: bool,
}

impl<'a> AnalysisContext<'a> {
    /// Create a new analysis context
    pub fn new(symbol_table: &'a mut SymbolTable, type_system: &'a mut TypeSystem) -> Self {
        Self {
            symbol_table,
            type_system,
            current_function: None,
            in_loop: false,
        }
    }
    
    /// Enter a function context
    pub fn with_function(mut self, function: &'a FunctionDecl) -> Self {
        self.current_function = Some(function);
        self
    }
    
    /// Enter a loop context
    pub fn with_loop(mut self) -> Self {
        self.in_loop = true;
        self
    }
}

/// Type system for type checking and inference
#[derive(Debug)]
pub struct TypeSystem {
    pub resolved_types: HashMap<String, ResolvedType>,
    pub struct_definitions: HashMap<String, ResolvedStructDef>,
    pub function_signatures: HashMap<String, FunctionSignature>,
}

impl TypeSystem {
    /// Create a new type system
    pub fn new() -> Self {
        Self {
            resolved_types: HashMap::new(),
            struct_definitions: HashMap::new(),
            function_signatures: HashMap::new(),
        }
    }
    
    /// Register a struct definition
    pub fn register_struct(&mut self, name: String, definition: ResolvedStructDef) {
        self.struct_definitions.insert(name, definition);
    }
    
    /// Register a function signature
    pub fn register_function(&mut self, name: String, signature: FunctionSignature) {
        self.function_signatures.insert(name, signature);
    }
    
    /// Resolve a type from AST to internal representation
    pub fn resolve_type(&self, ast_type: &Type) -> Result<ResolvedType, SemanticError> {
        match ast_type {
            Type::Named(path) => {
                let type_name = path.to_string();
                match type_name.as_str() {
                    "int" => Ok(ResolvedType::Integer),
                    "float" => Ok(ResolvedType::Float),
                    "string" => Ok(ResolvedType::String),
                    "bool" => Ok(ResolvedType::Boolean),
                    _ => {
                        if let Some(struct_def) = self.struct_definitions.get(&type_name) {
                            Ok(ResolvedType::Struct(type_name))
                        } else {
                            Err(SemanticError::UnknownType(type_name))
                        }
                    }
                }
            }
            Type::Generic(path, args) => {
                let base_type = self.resolve_type(&Type::Named(path.clone()))?;
                let resolved_args: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.resolve_type(arg))
                    .collect();
                Ok(ResolvedType::Generic(Box::new(base_type), resolved_args?))
            }
            // Add more type resolution cases as needed
            _ => Err(SemanticError::UnsupportedType(format!("{:?}", ast_type))),
        }
    }
}

/// Resolved type representation for internal use
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    Integer,
    Float,
    String,
    Boolean,
    Struct(String),
    Generic(Box<ResolvedType>, Vec<ResolvedType>),
    Function(Vec<ResolvedType>, Box<ResolvedType>),
    Array(Box<ResolvedType>, Option<usize>),
    Reference(Box<ResolvedType>),
    MutableReference(Box<ResolvedType>),
}

/// Resolved struct definition
#[derive(Debug, Clone)]
pub struct ResolvedStructDef {
    pub name: String,
    pub fields: HashMap<String, ResolvedType>,
}

/// Semantic analysis errors
#[derive(Debug, Clone)]
pub enum SemanticError {
    SymbolAlreadyDefined(String),
    SymbolNotFound(String),
    UnknownType(String),
    UnsupportedType(String),
    TypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },
    InternalError(String),
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::SymbolAlreadyDefined(name) => {
                write!(f, "Symbol '{}' is already defined", name)
            }
            SemanticError::SymbolNotFound(name) => {
                write!(f, "Symbol '{}' not found", name)
            }
            SemanticError::UnknownType(type_name) => {
                write!(f, "Unknown type '{}'", type_name)
            }
            SemanticError::UnsupportedType(type_desc) => {
                write!(f, "Unsupported type: {}", type_desc)
            }
            SemanticError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", expected, found)
            }
            SemanticError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for SemanticError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scope_management() {
        let mut symbol_table = SymbolTable::new();
        
        // Test entering and leaving scopes
        {
            let _guard = symbol_table.enter_scope(ScopeType::Function);
            assert_ne!(symbol_table.current_scope_id(), ScopeId(0));
        }
        // Guard should automatically restore previous scope
        assert_eq!(symbol_table.current_scope_id(), ScopeId(0));
    }
    
    #[test]
    fn test_symbol_lookup() {
        let mut symbol_table = SymbolTable::new();
        
        // Add symbol to global scope
        let global_symbol = Symbol {
            name: "global_var".to_string(),
            symbol_type: Type::Named(Path::single("int".to_string())),
            scope_id: ScopeId(0),
            is_mutable: false,
            is_initialized: true,
        };
        symbol_table.add_symbol(global_symbol).unwrap();
        
        // Enter new scope and verify symbol is still accessible
        {
            let _guard = symbol_table.enter_scope(ScopeType::Function);
            assert!(symbol_table.lookup_symbol("global_var").is_some());
        }
    }
}
