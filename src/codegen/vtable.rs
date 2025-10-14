// src/codegen/vtable.rs
//! V-Table generation for dynamic dispatch (Expert recommendation: Priority 1)
//!
//! This module implements the core V-Table system for `dyn Trait` support.
//! Each trait implementation gets a V-Table containing function pointers.

use std::collections::HashMap;
use inkwell::values::{FunctionValue, PointerValue, StructValue};
use inkwell::types::{StructType, PointerType};
use inkwell::AddressSpace;
use crate::semantic::{ResolvedType, SemanticError};
use crate::codegen::llvm_codegen::LLVMCodeGenerator;

/// V-Table entry representing a method in the trait
#[derive(Debug, Clone)]
pub struct VTableEntry {
    /// Method name
    pub method_name: String,
    /// Function pointer to the actual implementation
    pub function_ptr: Option<FunctionValue<'static>>,
    /// Method signature for type checking
    pub signature: MethodSignature,
}

/// Method signature for V-Table entries
#[derive(Debug, Clone)]
pub struct MethodSignature {
    /// Parameter types (including self)
    pub parameters: Vec<ResolvedType>,
    /// Return type
    pub return_type: Option<ResolvedType>,
}

/// V-Table for a specific trait implementation
#[derive(Debug, Clone)]
pub struct VTable {
    /// Trait name this V-Table implements
    pub trait_name: String,
    /// Type that implements this trait
    pub implementing_type: ResolvedType,
    /// Ordered list of method entries
    pub entries: Vec<VTableEntry>,
    /// LLVM struct representing this V-Table
    pub llvm_vtable: Option<StructValue<'static>>,
}

/// V-Table manager for the entire compilation unit
#[derive(Debug)]
pub struct VTableManager {
    /// All V-Tables indexed by (trait_name, implementing_type)
    pub vtables: HashMap<(String, String), VTable>,
    /// Trait method order for consistent V-Table layout
    pub trait_method_order: HashMap<String, Vec<String>>,
}

impl VTableManager {
    /// Create a new V-Table manager
    pub fn new() -> Self {
        Self {
            vtables: HashMap::new(),
            trait_method_order: HashMap::new(),
        }
    }

    /// Register trait method order for consistent V-Table layout
    pub fn register_trait_methods(&mut self, trait_name: &str, method_names: Vec<String>) {
        self.trait_method_order.insert(trait_name.to_string(), method_names);
    }

    /// Create V-Table for a trait implementation
    pub fn create_vtable(
        &mut self,
        trait_name: &str,
        implementing_type: &ResolvedType,
        method_implementations: HashMap<String, FunctionValue<'static>>,
    ) -> Result<&VTable, SemanticError> {
        let type_key = self.type_to_string(implementing_type);
        let vtable_key = (trait_name.to_string(), type_key.clone());

        // Get method order for this trait
        let method_order = self.trait_method_order
            .get(trait_name)
            .ok_or_else(|| SemanticError::UndefinedType(format!("Trait {} not registered", trait_name)))?;

        // Create V-Table entries in the correct order
        let mut entries = Vec::new();
        for method_name in method_order {
            let function_ptr = method_implementations.get(method_name).copied();

            // For now, use simplified signature - this should be enhanced
            let signature = MethodSignature {
                parameters: vec![implementing_type.clone()], // self parameter
                return_type: Some(ResolvedType::Int), // simplified
            };

            entries.push(VTableEntry {
                method_name: method_name.clone(),
                function_ptr,
                signature,
            });
        }

        let vtable = VTable {
            trait_name: trait_name.to_string(),
            implementing_type: implementing_type.clone(),
            entries,
            llvm_vtable: None, // Will be set during LLVM generation
        };

        self.vtables.insert(vtable_key.clone(), vtable);
        Ok(self.vtables.get(&vtable_key).unwrap())
    }

    /// Get V-Table for a specific trait implementation
    pub fn get_vtable(&self, trait_name: &str, implementing_type: &ResolvedType) -> Option<&VTable> {
        let type_key = self.type_to_string(implementing_type);
        let vtable_key = (trait_name.to_string(), type_key);
        self.vtables.get(&vtable_key)
    }

    /// Get method index in V-Table for consistent ordering
    pub fn get_method_index(&self, trait_name: &str, method_name: &str) -> Option<usize> {
        self.trait_method_order
            .get(trait_name)?
            .iter()
            .position(|name| name == method_name)
    }

    /// Convert ResolvedType to string key for HashMap
    pub fn type_to_string(&self, resolved_type: &ResolvedType) -> String {
        match resolved_type {
            ResolvedType::Int => "int".to_string(),
            ResolvedType::Float => "float".to_string(),
            ResolvedType::Bool => "bool".to_string(),
            ResolvedType::String => "string".to_string(),
            ResolvedType::Struct(name) => name.clone(),
            ResolvedType::Enum(name) => name.clone(),
            ResolvedType::List(inner) => format!("List<{}>", self.type_to_string(inner)),
            ResolvedType::Tuple(types) => {
                let type_strs: Vec<String> = types.iter().map(|t| self.type_to_string(t)).collect();
                format!("({})", type_strs.join(", "))
            }
            ResolvedType::Function(params, ret) => {
                let param_strs: Vec<String> = params.iter().map(|t| self.type_to_string(t)).collect();
                format!("fn({}) -> {}", param_strs.join(", "), self.type_to_string(ret))
            }
            ResolvedType::Generic(name, args) => {
                let arg_strs: Vec<String> = args.iter().map(|t| self.type_to_string(t)).collect();
                format!("{}<{}>", name, arg_strs.join(", "))
            }
            ResolvedType::GenericParam(name) => name.clone(),
            ResolvedType::TraitObject(traits) => format!("dyn {}", traits.join(" + ")),
            ResolvedType::Reference(inner, is_mutable) => {
                if *is_mutable {
                    format!("&mut {}", self.type_to_string(inner))
                } else {
                    format!("&{}", self.type_to_string(inner))
                }
            }
            _ => "unknown".to_string(),
        }
    }
}

impl Default for VTableManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Fat pointer representation for trait objects (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct FatPointer {
    /// Pointer to the actual data
    pub data_ptr: PointerValue<'static>,
    /// Pointer to the V-Table
    pub vtable_ptr: PointerValue<'static>,
}

impl FatPointer {
    /// Create a new fat pointer
    pub fn new(data_ptr: PointerValue<'static>, vtable_ptr: PointerValue<'static>) -> Self {
        Self { data_ptr, vtable_ptr }
    }
}

/// Trait object creation and manipulation utilities
pub struct TraitObjectUtils;

impl TraitObjectUtils {
    /// Create a trait object from a concrete value and its V-Table
    pub fn create_trait_object(
        codegen: &LLVMCodeGenerator,
        concrete_value: PointerValue,
        vtable: &VTable,
    ) -> Result<StructValue, SemanticError> {
        // Get fat pointer type
        let data_ptr_type = codegen.context.i8_type().ptr_type(AddressSpace::default());
        let vtable_ptr_type = codegen.context.i8_type().ptr_type(AddressSpace::default());

        let fat_pointer_type = codegen.context.struct_type(
            &[data_ptr_type.into(), vtable_ptr_type.into()],
            false
        );

        // Cast concrete value to generic data pointer
        let data_ptr = codegen.builder.build_bitcast(
            concrete_value,
            data_ptr_type,
            "trait_object_data"
        ).map_err(|e| SemanticError::InternalError(format!("Failed to cast data pointer: {}", e)))?;

        // Get V-Table pointer (this would be set during V-Table generation)
        let vtable_ptr = if let Some(llvm_vtable) = &vtable.llvm_vtable {
            // Cast V-Table to generic pointer
            codegen.builder.build_bitcast(
                llvm_vtable.as_pointer_value(),
                vtable_ptr_type,
                "trait_object_vtable"
            ).map_err(|e| SemanticError::InternalError(format!("Failed to cast vtable pointer: {}", e)))?
        } else {
            // Null V-Table for now
            vtable_ptr_type.const_null().into()
        };

        // Create fat pointer struct
        let fat_pointer = fat_pointer_type.const_named_struct(&[
            data_ptr.into(),
            vtable_ptr.into()
        ]);

        Ok(fat_pointer)
    }

    /// Extract data pointer from trait object
    pub fn extract_data_ptr(
        codegen: &LLVMCodeGenerator,
        trait_object: StructValue,
    ) -> Result<PointerValue, SemanticError> {
        let data_ptr = codegen.builder.build_extract_value(
            trait_object,
            0,
            "trait_object_data_ptr"
        ).map_err(|e| SemanticError::InternalError(format!("Failed to extract data pointer: {}", e)))?;

        Ok(data_ptr.into_pointer_value())
    }

    /// Extract V-Table pointer from trait object
    pub fn extract_vtable_ptr(
        codegen: &LLVMCodeGenerator,
        trait_object: StructValue,
    ) -> Result<PointerValue, SemanticError> {
        let vtable_ptr = codegen.builder.build_extract_value(
            trait_object,
            1,
            "trait_object_vtable_ptr"
        ).map_err(|e| SemanticError::InternalError(format!("Failed to extract vtable pointer: {}", e)))?;

        Ok(vtable_ptr.into_pointer_value())
    }
}
