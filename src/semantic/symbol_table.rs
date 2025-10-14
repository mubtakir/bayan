//! # Symbol Table Implementation
//!
//! This module implements the symbol table for tracking variables, functions,
//! types, and other symbols during semantic analysis.

use crate::parser::ast::*;
use super::{ResolvedType, RelationInfo, SemanticError};
use std::collections::HashMap;

/// Symbol table for managing scopes and symbol resolution
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Stack of scopes (global scope at bottom)
    scopes: Vec<Scope>,
    /// Global type definitions
    types: HashMap<String, TypeInfo>,
    /// Global function definitions
    functions: HashMap<String, FunctionInfo>,
    /// Global relation definitions
    relations: HashMap<String, RelationInfo>,
    /// Global trait definitions (Expert recommendation: Priority 1)
    traits: HashMap<String, TraitInfo>,
    /// Global impl definitions (Expert recommendation: Priority 1)
    impls: Vec<ImplInfo>,
}

/// A single scope containing local symbols
#[derive(Debug, Clone)]
struct Scope {
    /// Variables in this scope
    variables: HashMap<String, VariableInfo>,
    /// Generic type parameters in this scope (Expert recommendation: Priority 1)
    generic_params: HashMap<String, String>,  // name -> name (for now)
    /// Scope type (function, block, etc.)
    scope_type: ScopeType,
}

/// Type of scope
#[derive(Debug, Clone, PartialEq)]
enum ScopeType {
    Global,
    Function,
    Block,
    Loop,
}

/// Information about a variable
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub var_type: ResolvedType,
    pub is_mutable: bool,
    pub is_initialized: bool,
}

/// Information about a function
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub parameters: Vec<ResolvedType>,
    pub return_type: Option<ResolvedType>,
}

/// Information about a type
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
}

/// Kind of type
#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct(Vec<StructFieldInfo>),
    Enum(Vec<EnumVariantInfo>),
    Class(Vec<StructFieldInfo>, Vec<FunctionInfo>),
    Interface(Vec<FunctionInfo>),
    Primitive,
}

/// Information about a struct field
#[derive(Debug, Clone)]
pub struct StructFieldInfo {
    pub name: String,
    pub field_type: ResolvedType,
}

/// Information about an enum variant
#[derive(Debug, Clone)]
pub struct EnumVariantInfo {
    pub name: String,
    pub fields: Option<Vec<ResolvedType>>,
}

/// Information about a trait (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct TraitInfo {
    pub name: String,
    pub methods: Vec<TraitMethodInfo>,
}

/// Information about a trait method (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct TraitMethodInfo {
    pub name: String,
    pub parameters: Vec<ResolvedType>,
    pub return_type: Option<ResolvedType>,
    pub has_default_impl: bool,
}

/// Information about an impl block (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct ImplInfo {
    pub trait_name: Option<String>,  // None for inherent impl
    pub type_name: String,
    pub methods: Vec<FunctionInfo>,
}

impl SymbolTable {
    /// Create a new symbol table with global scope
    pub fn new() -> Self {
        let mut symbol_table = Self {
            scopes: vec![Scope {
                variables: HashMap::new(),
                generic_params: HashMap::new(),
                scope_type: ScopeType::Global,
            }],
            types: HashMap::new(),
            functions: HashMap::new(),
            relations: HashMap::new(),
            traits: HashMap::new(),  // NEWLY ADDED: Expert recommendation
            impls: Vec::new(),       // NEWLY ADDED: Expert recommendation
        };

        // Add built-in types
        symbol_table.add_builtin_types();

        symbol_table
    }

    /// Add built-in types to the symbol table
    fn add_builtin_types(&mut self) {
        let builtin_types = [
            ("int", TypeKind::Primitive),
            ("float", TypeKind::Primitive),
            ("bool", TypeKind::Primitive),
            ("string", TypeKind::Primitive),
            ("char", TypeKind::Primitive),
        ];

        for (name, kind) in builtin_types {
            self.types.insert(name.to_string(), TypeInfo {
                name: name.to_string(),
                kind,
            });
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            variables: HashMap::new(),
            generic_params: HashMap::new(),
            scope_type: ScopeType::Block,
        });
    }

    /// Enter a function scope
    pub fn enter_function_scope(&mut self) {
        self.scopes.push(Scope {
            variables: HashMap::new(),
            generic_params: HashMap::new(),
            scope_type: ScopeType::Function,
        });
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Declare a variable in the current scope
    pub fn declare_variable(&mut self, name: &str, var_type: &ResolvedType) -> Result<(), SemanticError> {
        let current_scope = self.scopes.last_mut().unwrap();

        if current_scope.variables.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        current_scope.variables.insert(name.to_string(), VariableInfo {
            name: name.to_string(),
            var_type: var_type.clone(),
            is_mutable: false, // TODO: Handle mut keyword
            is_initialized: true, // TODO: Track initialization
        });

        Ok(())
    }

    /// Look up a variable in all scopes (starting from current)
    pub fn lookup_variable(&self, name: &str) -> Option<&VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.variables.get(name) {
                return Some(var_info);
            }
        }
        None
    }

    /// Declare a generic type parameter in the current scope (Expert recommendation: Priority 1)
    pub fn declare_generic_param(&mut self, name: &str) -> Result<(), SemanticError> {
        let current_scope = self.scopes.last_mut().unwrap();

        if current_scope.generic_params.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }


        current_scope.generic_params.insert(name.to_string(), name.to_string());
        Ok(())
    }

    /// Look up a generic type parameter in all scopes (Expert recommendation: Priority 1)
    pub fn lookup_generic_param(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.generic_params.contains_key(name) {
                return true;
            }
        }
        false
    }

    /// Declare a function
    pub fn declare_function(&mut self, name: &str, func: &FunctionDecl) -> Result<(), SemanticError> {
        if self.functions.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        // For generic functions, we need to defer type resolution until analysis phase
        // because generic parameters are not in scope during symbol collection
        if func.generic_params.is_some() {

            // For now, just register the function name without type resolution
            self.functions.insert(name.to_string(), FunctionInfo {
                name: name.to_string(),
                parameters: Vec::new(), // Will be resolved later
                return_type: None,      // Will be resolved later
            });
            return Ok(());
        }

        let mut parameters = Vec::new();
        for param in &func.parameters {
            match param {
                Parameter::Regular { name: _, param_type } => {
                    let resolved_type = self.resolve_type_name(param_type)?;
                    parameters.push(resolved_type);
                }
                Parameter::SelfValue => {
                    // self parameter - type will be determined by context
                    parameters.push(ResolvedType::Unit); // Placeholder
                }
                Parameter::SelfRef => {
                    // &self parameter
                    let self_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), false);
                    parameters.push(self_ref_type);
                }
                Parameter::SelfMutRef => {
                    // &mut self parameter
                    let self_mut_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), true);
                    parameters.push(self_mut_ref_type);
                }
            }
        }

        let return_type = if let Some(ret_type) = &func.return_type {
            Some(self.resolve_type_name(ret_type)?)
        } else {
            None
        };

        self.functions.insert(name.to_string(), FunctionInfo {
            name: name.to_string(),
            parameters,
            return_type,
        });

        Ok(())
    }

    /// Look up a function
    pub fn lookup_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name)
    }

    /// Add a function info directly (Expert recommendation: for built-in functions)
    pub fn add_function_info(&mut self, name: &str, func_info: FunctionInfo) {
        self.functions.insert(name.to_string(), func_info);
    }

    /// Declare a struct
    pub fn declare_struct(&mut self, name: &str, struct_decl: &StructDecl) -> Result<(), SemanticError> {
        if self.types.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        // For generic structs, we need to defer type resolution until analysis phase
        // because generic parameters are not in scope during symbol collection
        if struct_decl.generic_params.is_some() {

            // For now, just register the struct name without field type resolution
            self.types.insert(name.to_string(), TypeInfo {
                name: name.to_string(),
                kind: TypeKind::Struct(Vec::new()), // Will be resolved later
            });
            return Ok(());
        }

        let mut fields = Vec::new();
        for field in &struct_decl.fields {
            let resolved_type = self.resolve_type_name(&field.field_type)?;
            fields.push(StructFieldInfo {
                name: field.name.clone(),
                field_type: resolved_type,
            });
        }

        self.types.insert(name.to_string(), TypeInfo {
            name: name.to_string(),
            kind: TypeKind::Struct(fields),
        });

        Ok(())
    }

    /// Declare an enum
    pub fn declare_enum(&mut self, name: &str, enum_decl: &EnumDecl) -> Result<(), SemanticError> {
        if self.types.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        let mut variants = Vec::new();
        for variant in &enum_decl.variants {
            let fields = if let Some(field_types) = &variant.fields {
                let mut resolved_fields = Vec::new();
                for field_type in field_types {
                    let resolved_type = self.resolve_type_name(field_type)?;
                    resolved_fields.push(resolved_type);
                }
                Some(resolved_fields)
            } else {
                None
            };

            variants.push(EnumVariantInfo {
                name: variant.name.clone(),
                fields,
            });
        }

        self.types.insert(name.to_string(), TypeInfo {
            name: name.to_string(),
            kind: TypeKind::Enum(variants),
        });

        Ok(())
    }

    /// Declare a class
    pub fn declare_class(&mut self, name: &str, class_decl: &ClassDecl) -> Result<(), SemanticError> {
        if self.types.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        let mut fields = Vec::new();
        for field in &class_decl.fields {
            let resolved_type = self.resolve_type_name(&field.field_type)?;
            fields.push(StructFieldInfo {
                name: field.name.clone(),
                field_type: resolved_type,
            });
        }

        let mut methods = Vec::new();
        for method in &class_decl.methods {
            let mut parameters = Vec::new();
            for param in &method.parameters {
                match param {
                    Parameter::Regular { name: _, param_type } => {
                        let resolved_type = self.resolve_type_name(param_type)?;
                        parameters.push(resolved_type);
                    }
                    Parameter::SelfValue => {
                        let self_type = ResolvedType::Struct(class_decl.name.clone());
                        parameters.push(self_type);
                    }
                    Parameter::SelfRef => {
                        let self_type = ResolvedType::Struct(class_decl.name.clone());
                        let self_ref_type = ResolvedType::Reference(Box::new(self_type), false);
                        parameters.push(self_ref_type);
                    }
                    Parameter::SelfMutRef => {
                        let self_type = ResolvedType::Struct(class_decl.name.clone());
                        let self_mut_ref_type = ResolvedType::Reference(Box::new(self_type), true);
                        parameters.push(self_mut_ref_type);
                    }
                }
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.resolve_type_name(ret_type)?)
            } else {
                None
            };

            methods.push(FunctionInfo {
                name: method.name.clone(),
                parameters,
                return_type,
            });
        }

        self.types.insert(name.to_string(), TypeInfo {
            name: name.to_string(),
            kind: TypeKind::Class(fields, methods),
        });

        Ok(())
    }

    /// Declare an interface
    pub fn declare_interface(&mut self, name: &str, interface_decl: &InterfaceDecl) -> Result<(), SemanticError> {
        if self.types.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        let mut methods = Vec::new();
        for method in &interface_decl.methods {
            let mut parameters = Vec::new();
            for param in &method.parameters {
                match param {
                    Parameter::Regular { name: _, param_type } => {
                        let resolved_type = self.resolve_type_name(param_type)?;
                        parameters.push(resolved_type);
                    }
                    Parameter::SelfValue => {
                        parameters.push(ResolvedType::Unit); // Interface self type is abstract
                    }
                    Parameter::SelfRef => {
                        let self_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), false);
                        parameters.push(self_ref_type);
                    }
                    Parameter::SelfMutRef => {
                        let self_mut_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), true);
                        parameters.push(self_mut_ref_type);
                    }
                }
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.resolve_type_name(ret_type)?)
            } else {
                None
            };

            methods.push(FunctionInfo {
                name: method.name.clone(),
                parameters,
                return_type,
            });
        }

        self.types.insert(name.to_string(), TypeInfo {
            name: name.to_string(),
            kind: TypeKind::Interface(methods),
        });

        Ok(())
    }

    /// Declare a relation
    pub fn declare_relation(&mut self, name: &str, relation_decl: &RelationDecl) -> Result<(), SemanticError> {
        if self.relations.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        let mut arg_types = Vec::new();
        for arg_type in &relation_decl.arg_types {
            let resolved_type = self.resolve_type_name(arg_type)?;
            arg_types.push(resolved_type);
        }

        self.relations.insert(name.to_string(), RelationInfo {
            name: name.to_string(),
            arg_types,
        });

        Ok(())
    }

    /// Declare a trait (Expert recommendation: Priority 1)
    pub fn declare_trait(&mut self, name: &str, trait_decl: &TraitDecl) -> Result<(), SemanticError> {
        if self.traits.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }

        // For generic traits, we need to defer type resolution until analysis phase
        // because generic parameters are not in scope during symbol collection
        if trait_decl.generic_params.is_some() {

            // For now, just register the trait name without method type resolution
            self.traits.insert(name.to_string(), TraitInfo {
                name: name.to_string(),
                methods: Vec::new(), // Will be resolved later
            });
            return Ok(());
        }

        let mut methods = Vec::new();
        for method in &trait_decl.methods {
            let mut parameters = Vec::new();
            for param in &method.parameters {
                match param {
                    Parameter::Regular { name: _, param_type } => {
                        let resolved_type = self.resolve_type_name(param_type)?;
                        parameters.push(resolved_type);
                    }
                    Parameter::SelfValue => {
                        parameters.push(ResolvedType::Unit); // Placeholder for self
                    }
                    Parameter::SelfRef => {
                        let self_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), false);
                        parameters.push(self_ref_type);
                    }
                    Parameter::SelfMutRef => {
                        let self_mut_ref_type = ResolvedType::Reference(Box::new(ResolvedType::Unit), true);
                        parameters.push(self_mut_ref_type);
                    }
                }
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.resolve_type_name(ret_type)?)
            } else {
                None
            };

            methods.push(TraitMethodInfo {
                name: method.name.clone(),
                parameters,
                return_type,
                has_default_impl: method.body.is_some(),
            });
        }

        self.traits.insert(name.to_string(), TraitInfo {
            name: name.to_string(),
            methods,
        });

        Ok(())
    }

    /// Declare an impl block (Expert recommendation: Priority 1)
    pub fn declare_impl(&mut self, impl_decl: &ImplDecl) -> Result<(), SemanticError> {
        // For generic impl blocks, we need to defer type resolution until analysis phase
        // because generic parameters are not in scope during symbol collection
        if impl_decl.generic_params.is_some() {

            // For now, just register the impl without method type resolution
            self.impls.push(ImplInfo {
                trait_name: impl_decl.trait_name.clone(),
                type_name: impl_decl.type_name.clone(),
                methods: Vec::new(), // Will be resolved later
            });
            return Ok(());
        }

        let mut methods = Vec::new();
        for method in &impl_decl.methods {
            let mut parameters = Vec::new();
            for param in &method.parameters {
                match param {
                    Parameter::Regular { name: _, param_type } => {
                        let resolved_type = self.resolve_type_name(param_type)?;
                        parameters.push(resolved_type);
                    }
                    Parameter::SelfValue => {
                        // For impl methods, self type should be the implementing type
                        let self_type = ResolvedType::Struct(impl_decl.type_name.clone());
                        parameters.push(self_type);
                    }
                    Parameter::SelfRef => {
                        let self_type = ResolvedType::Struct(impl_decl.type_name.clone());
                        let self_ref_type = ResolvedType::Reference(Box::new(self_type), false);
                        parameters.push(self_ref_type);
                    }
                    Parameter::SelfMutRef => {
                        let self_type = ResolvedType::Struct(impl_decl.type_name.clone());
                        let self_mut_ref_type = ResolvedType::Reference(Box::new(self_type), true);
                        parameters.push(self_mut_ref_type);
                    }
                }
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.resolve_type_name(ret_type)?)
            } else {
                None
            };

            methods.push(FunctionInfo {
                name: method.name.clone(),
                parameters,
                return_type,
            });
        }

        self.impls.push(ImplInfo {
            trait_name: impl_decl.trait_name.clone(),
            type_name: impl_decl.type_name.clone(),
            methods,
        });

        Ok(())
    }

    /// Look up a relation
    pub fn lookup_relation(&self, name: &str) -> Option<&RelationInfo> {
        self.relations.get(name)
    }

    /// Look up a trait (Expert recommendation: Priority 1)
    pub fn lookup_trait(&self, name: &str) -> Option<&TraitInfo> {
        self.traits.get(name)
    }

    /// Find impl for a type and trait (Expert recommendation: Priority 1)
    pub fn find_impl(&self, type_name: &str, trait_name: Option<&str>) -> Option<&ImplInfo> {
        self.impls.iter().find(|impl_info| {
            impl_info.type_name == type_name && impl_info.trait_name.as_deref() == trait_name
        })
    }

    /// Get all impl blocks (Expert recommendation: Priority 1 - Method Resolution)
    pub fn get_impls(&self) -> &Vec<ImplInfo> {
        &self.impls
    }

    /// Look up a type
    pub fn lookup_type(&self, name: &str) -> Option<&TypeInfo> {
        self.types.get(name)
    }

    /// Update struct info after analysis (Expert recommendation: Priority 1)
    pub fn update_struct_info(&mut self, name: &str, fields: Vec<StructFieldInfo>) -> Result<(), SemanticError> {
        if let Some(type_info) = self.types.get_mut(name) {
            type_info.kind = TypeKind::Struct(fields);
            Ok(())
        } else {
            Err(SemanticError::UndefinedType(name.to_string()))
        }
    }

    /// Resolve a type name to a ResolvedType
    pub fn resolve_type_name(&self, type_annotation: &Type) -> Result<ResolvedType, SemanticError> {
        match type_annotation {
            Type::Named(name) => {
                match name.to_string().as_str() {
                    "int" => Ok(ResolvedType::Int),
                    "float" => Ok(ResolvedType::Float),
                    "bool" => Ok(ResolvedType::Bool),
                    "string" => Ok(ResolvedType::String),
                    "char" => Ok(ResolvedType::Char),
                    _ => {
                        let name_str = name.to_string();

                        // Check if it's a generic type parameter first (Expert recommendation: Priority 1)
                        if self.lookup_generic_param(&name_str) {

                            return Ok(ResolvedType::GenericParam(name_str));
                        } else {

                        }

                        if self.types.contains_key(&name_str) {
                            match &self.types[&name_str].kind {
                                TypeKind::Struct(_) => Ok(ResolvedType::Struct(name_str)),
                                TypeKind::Enum(_) => Ok(ResolvedType::Enum(name_str)),
                                TypeKind::Class(_, _) => Ok(ResolvedType::Struct(name_str)), // Treat class as struct for now
                                _ => Ok(ResolvedType::Struct(name_str)),
                            }
                        } else {
                            Err(SemanticError::UndefinedVariable(name.to_string()))
                        }
                    }
                }
            }
            Type::Generic(name, args) => {
                let mut resolved_args = Vec::new();
                for arg in args {
                    resolved_args.push(self.resolve_type_name(arg)?);
                }
                Ok(ResolvedType::Generic(name.to_string(), resolved_args))
            }
            Type::GenericParam(name) => {
                // Generic type parameters are valid during semantic analysis
                Ok(ResolvedType::GenericParam(name.clone()))
            }
            Type::TraitObject(traits) => {
                // Trait object types (dyn Trait) - Expert recommendation: Priority 1
                let mut trait_names = Vec::new();
                for trait_path in traits {
                    let trait_name = trait_path.to_string();

                    // Verify that the trait exists
                    if !self.traits.contains_key(&trait_name) {
                        return Err(SemanticError::UndefinedType(trait_name));
                    }

                    trait_names.push(trait_name);
                }
                Ok(ResolvedType::TraitObject(trait_names))
            }
            Type::Reference(referenced_type, is_mutable) => {
                // Reference types (&T, &mut T) - Expert recommendation: Priority 1
                let resolved_referenced_type = self.resolve_type_name(referenced_type)?;
                Ok(ResolvedType::Reference(Box::new(resolved_referenced_type), *is_mutable))
            }
            Type::Function(params, ret) => {
                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.resolve_type_name(param)?);
                }
                let resolved_ret = self.resolve_type_name(ret)?;
                Ok(ResolvedType::Function(resolved_params, Box::new(resolved_ret)))
            }
            _ => todo!("Other type resolution not yet implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_creation() {
        let symbol_table = SymbolTable::new();
        assert_eq!(symbol_table.scopes.len(), 1);
        assert!(symbol_table.lookup_type("int").is_some());
        assert!(symbol_table.lookup_type("string").is_some());
    }

    #[test]
    fn test_variable_declaration() {
        let mut symbol_table = SymbolTable::new();

        let result = symbol_table.declare_variable("x", &ResolvedType::Int);
        assert!(result.is_ok());

        let var_info = symbol_table.lookup_variable("x");
        assert!(var_info.is_some());
        assert_eq!(var_info.unwrap().var_type, ResolvedType::Int);
    }

    #[test]
    fn test_scope_management() {
        let mut symbol_table = SymbolTable::new();

        // Declare variable in global scope
        symbol_table.declare_variable("global_var", &ResolvedType::Int).unwrap();

        // Enter new scope
        symbol_table.enter_scope();
        symbol_table.declare_variable("local_var", &ResolvedType::String).unwrap();

        // Both variables should be visible
        assert!(symbol_table.lookup_variable("global_var").is_some());
        assert!(symbol_table.lookup_variable("local_var").is_some());

        // Exit scope
        symbol_table.exit_scope();

        // Only global variable should be visible
        assert!(symbol_table.lookup_variable("global_var").is_some());
        assert!(symbol_table.lookup_variable("local_var").is_none());
    }
}
