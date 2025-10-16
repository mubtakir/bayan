//! # Type Checker Implementation
//!
//! This module implements type checking and type inference for the AlBayan language.

use super::{ResolvedType, SemanticError};
use crate::parser::ast::*;

/// Type checker for the AlBayan language
#[derive(Debug, Clone)]
pub struct TypeChecker {
    /// Current function return type (for checking return statements)
    current_function_return_type: Option<ResolvedType>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            current_function_return_type: None,
        }
    }

    /// Set the current function return type
    pub fn set_function_return_type(&mut self, return_type: Option<ResolvedType>) {
        self.current_function_return_type = return_type;
    }

    /// Resolve a type annotation to a concrete type
    pub fn resolve_type(&self, type_annotation: &Type) -> Result<ResolvedType, SemanticError> {
        match type_annotation {
            Type::Named(name) => {
                match name.to_string().as_str() {
                    "int" => Ok(ResolvedType::Int),
                    "float" => Ok(ResolvedType::Float),
                    "bool" => Ok(ResolvedType::Bool),
                    "string" => Ok(ResolvedType::String),
                    "char" => Ok(ResolvedType::Char),
                    // AI types (Expert recommendation: Priority 1)
                    "Model" => Ok(ResolvedType::Model("Model".to_string())),
                    "Tensor" => Ok(ResolvedType::Tensor(vec![])),
                    // PyTorch types (Expert recommendation: Priority 2)
                    "TorchModel" => Ok(ResolvedType::TorchModel),
                    "TorchOptimizer" => Ok(ResolvedType::TorchOptimizer),
                    "TorchTensor" => Ok(ResolvedType::TorchTensor),
                    "TrainingResult" => Ok(ResolvedType::TrainingResult),
                    _ => {
                        // For user-defined types, we assume they exist
                        // (this should be validated by the symbol table)
                        Ok(ResolvedType::Struct(name.to_string()))
                    }
                }
            }
            Type::Generic(name, args) => {
                let mut resolved_args = Vec::new();
                for arg in args {
                    resolved_args.push(self.resolve_type(arg)?);
                }
                Ok(ResolvedType::Generic(name.to_string(), resolved_args))
            }
            Type::GenericParam(name) => {
                // Generic type parameters are valid during parsing/semantic analysis
                // They will be substituted with concrete types during monomorphization
                Ok(ResolvedType::GenericParam(name.clone()))
            }
            Type::Function(params, ret) => {
                let mut resolved_params = Vec::new();
                for param in params {
                    resolved_params.push(self.resolve_type(param)?);
                }
                let resolved_ret = self.resolve_type(ret)?;
                Ok(ResolvedType::Function(
                    resolved_params,
                    Box::new(resolved_ret),
                ))
            }
            _ => todo!("Other type resolution not yet implemented"),
        }
    }

    /// Infer the type of a literal
    pub fn infer_literal_type(&self, literal: &Literal) -> ResolvedType {
        match literal {
            Literal::Boolean(_) => ResolvedType::Bool,
            Literal::Integer(_) => ResolvedType::Int,
            Literal::Float(_) => ResolvedType::Float,
            Literal::String(_) => ResolvedType::String,
            Literal::Char(_) => ResolvedType::Char,
            Literal::Null => ResolvedType::Null,

            // Tensor Literal (Expert recommendation: Priority 3)
            Literal::Tensor(rows) => {
                // Infer tensor dimensions from the literal structure
                if rows.is_empty() {
                    ResolvedType::Tensor(vec![0]) // Empty tensor
                } else {
                    let num_rows = rows.len();
                    let num_cols = rows[0].len();
                    ResolvedType::Tensor(vec![num_rows, num_cols])
                }
            }
        }
    }

    /// Check if two types are compatible
    pub fn types_compatible(&self, expected: &ResolvedType, actual: &ResolvedType) -> bool {
        match (expected, actual) {
            // Exact matches
            (ResolvedType::Int, ResolvedType::Int) => true,
            (ResolvedType::Float, ResolvedType::Float) => true,
            (ResolvedType::Bool, ResolvedType::Bool) => true,
            (ResolvedType::String, ResolvedType::String) => true,
            (ResolvedType::Char, ResolvedType::Char) => true,
            (ResolvedType::Null, ResolvedType::Null) => true,

            // Numeric coercion: int can be promoted to float
            (ResolvedType::Float, ResolvedType::Int) => true,

            // Struct and enum types
            (ResolvedType::Struct(name1), ResolvedType::Struct(name2)) => name1 == name2,
            (ResolvedType::Enum(name1), ResolvedType::Enum(name2)) => name1 == name2,

            // Collection types
            (ResolvedType::List(inner1), ResolvedType::List(inner2)) => {
                self.types_compatible(inner1, inner2)
            }
            (ResolvedType::Tuple(elems1), ResolvedType::Tuple(elems2)) => {
                elems1.len() == elems2.len()
                    && elems1
                        .iter()
                        .zip(elems2.iter())
                        .all(|(t1, t2)| self.types_compatible(t1, t2))
            }

            // Generic types
            (ResolvedType::Generic(name1, args1), ResolvedType::Generic(name2, args2)) => {
                name1 == name2
                    && args1.len() == args2.len()
                    && args1
                        .iter()
                        .zip(args2.iter())
                        .all(|(a1, a2)| self.types_compatible(a1, a2))
            }

            // Function types
            (ResolvedType::Function(params1, ret1), ResolvedType::Function(params2, ret2)) => {
                params1.len() == params2.len()
                    && params1
                        .iter()
                        .zip(params2.iter())
                        .all(|(p1, p2)| self.types_compatible(p1, p2))
                    && self.types_compatible(ret1, ret2)
            }

            // Generic type parameters - for now, any generic param is compatible with any type
            // This will be refined during monomorphization
            (ResolvedType::GenericParam(_), _) => true,
            (_, ResolvedType::GenericParam(_)) => true,

            // Trait object types (dyn Trait) - Expert recommendation: Priority 1
            (ResolvedType::TraitObject(traits1), ResolvedType::TraitObject(traits2)) => {
                // For now, trait objects are compatible if they have the same traits
                // TODO: Implement proper trait compatibility checking
                traits1 == traits2
            }

            // Reference types (&T, &mut T) - Expert recommendation: Priority 1
            (ResolvedType::Reference(type1, mut1), ResolvedType::Reference(type2, mut2)) => {
                // References are compatible if:
                // 1. The referenced types are compatible
                // 2. Mutability is compatible (&mut T can be used as &T, but not vice versa)
                self.types_compatible(type1, type2) && (*mut1 == *mut2 || (*mut1 && !*mut2))
            }

            _ => false,
        }
    }

    /// Check a binary operation and return the result type
    pub fn check_binary_operation(
        &self,
        operator: &BinaryOperator,
        left_type: &ResolvedType,
        right_type: &ResolvedType,
    ) -> Result<ResolvedType, SemanticError> {
        match operator {
            // Arithmetic operations
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo
            | BinaryOperator::Power => {
                self.check_arithmetic_operation(operator, left_type, right_type)
            }

            // Comparison operations
            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                if self.types_compatible(left_type, right_type)
                    || self.types_compatible(right_type, left_type)
                {
                    Ok(ResolvedType::Bool)
                } else {
                    Err(SemanticError::InvalidBinaryOperation(
                        operator.clone(),
                        left_type.clone(),
                        right_type.clone(),
                    ))
                }
            }

            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual => {
                self.check_comparison_operation(operator, left_type, right_type)
            }

            // Logical operations
            BinaryOperator::And | BinaryOperator::Or => {
                if matches!(left_type, ResolvedType::Bool)
                    && matches!(right_type, ResolvedType::Bool)
                {
                    Ok(ResolvedType::Bool)
                } else {
                    Err(SemanticError::InvalidBinaryOperation(
                        operator.clone(),
                        left_type.clone(),
                        right_type.clone(),
                    ))
                }
            }

            // Assignment operations
            BinaryOperator::Assign => {
                if self.types_compatible(left_type, right_type) {
                    Ok(left_type.clone())
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: left_type.clone(),
                        found: right_type.clone(),
                    })
                }
            }

            BinaryOperator::AddAssign
            | BinaryOperator::SubtractAssign
            | BinaryOperator::MultiplyAssign
            | BinaryOperator::DivideAssign => {
                // For compound assignment, check the operation first
                let base_op = match operator {
                    BinaryOperator::AddAssign => BinaryOperator::Add,
                    BinaryOperator::SubtractAssign => BinaryOperator::Subtract,
                    BinaryOperator::MultiplyAssign => BinaryOperator::Multiply,
                    BinaryOperator::DivideAssign => BinaryOperator::Divide,
                    _ => unreachable!(),
                };

                let result_type = self.check_binary_operation(&base_op, left_type, right_type)?;

                if self.types_compatible(left_type, &result_type) {
                    Ok(left_type.clone())
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: left_type.clone(),
                        found: result_type,
                    })
                }
            }
        }
    }

    /// Check arithmetic operations (+, -, *, /, %, **)
    fn check_arithmetic_operation(
        &self,
        operator: &BinaryOperator,
        left_type: &ResolvedType,
        right_type: &ResolvedType,
    ) -> Result<ResolvedType, SemanticError> {
        match (left_type, right_type) {
            // Integer operations
            (ResolvedType::Int, ResolvedType::Int) => Ok(ResolvedType::Int),

            // Float operations
            (ResolvedType::Float, ResolvedType::Float) => Ok(ResolvedType::Float),

            // Mixed int/float operations (promote to float)
            (ResolvedType::Int, ResolvedType::Float) | (ResolvedType::Float, ResolvedType::Int) => {
                Ok(ResolvedType::Float)
            }

            // String concatenation (only for +)
            (ResolvedType::String, ResolvedType::String)
                if matches!(operator, BinaryOperator::Add) =>
            {
                Ok(ResolvedType::String)
            }

            // Tensor operations (Expert recommendation: Priority 3)
            (ResolvedType::Tensor(dims1), ResolvedType::Tensor(dims2)) => {
                // Check that tensor dimensions are compatible
                if dims1 == dims2 {
                    Ok(ResolvedType::Tensor(dims1.clone()))
                } else {
                    Err(SemanticError::InvalidBinaryOperation(
                        operator.clone(),
                        left_type.clone(),
                        right_type.clone(),
                    ))
                }
            }

            _ => Err(SemanticError::InvalidBinaryOperation(
                operator.clone(),
                left_type.clone(),
                right_type.clone(),
            )),
        }
    }

    /// Check comparison operations (<, <=, >, >=)
    fn check_comparison_operation(
        &self,
        operator: &BinaryOperator,
        left_type: &ResolvedType,
        right_type: &ResolvedType,
    ) -> Result<ResolvedType, SemanticError> {
        match (left_type, right_type) {
            // Numeric comparisons
            (ResolvedType::Int, ResolvedType::Int)
            | (ResolvedType::Float, ResolvedType::Float)
            | (ResolvedType::Int, ResolvedType::Float)
            | (ResolvedType::Float, ResolvedType::Int) => Ok(ResolvedType::Bool),

            // String comparisons
            (ResolvedType::String, ResolvedType::String) => Ok(ResolvedType::Bool),

            // Character comparisons
            (ResolvedType::Char, ResolvedType::Char) => Ok(ResolvedType::Bool),

            _ => Err(SemanticError::InvalidBinaryOperation(
                operator.clone(),
                left_type.clone(),
                right_type.clone(),
            )),
        }
    }

    /// Check a unary operation and return the result type
    pub fn check_unary_operation(
        &self,
        operator: &UnaryOperator,
        operand_type: &ResolvedType,
    ) -> Result<ResolvedType, SemanticError> {
        match operator {
            UnaryOperator::Not => {
                if matches!(operand_type, ResolvedType::Bool) {
                    Ok(ResolvedType::Bool)
                } else {
                    Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Bool,
                        found: operand_type.clone(),
                    })
                }
            }

            UnaryOperator::Negate => {
                match operand_type {
                    ResolvedType::Int => Ok(ResolvedType::Int),
                    ResolvedType::Float => Ok(ResolvedType::Float),
                    _ => Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Int, // or Float
                        found: operand_type.clone(),
                    }),
                }
            }

            UnaryOperator::Reference => {
                // &T -> &T (handled in semantic analysis)
                Ok(ResolvedType::Reference(
                    Box::new(operand_type.clone()),
                    false,
                ))
            }

            UnaryOperator::MutableReference => {
                // &mut T -> &mut T (handled in semantic analysis)
                Ok(ResolvedType::Reference(
                    Box::new(operand_type.clone()),
                    true,
                ))
            }

            UnaryOperator::Dereference => {
                // *&T -> T or *&mut T -> T
                match operand_type {
                    ResolvedType::Reference(inner, _) => Ok((**inner).clone()),
                    _ => Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Reference(Box::new(ResolvedType::Unit), false), // placeholder
                        found: operand_type.clone(),
                    }),
                }
            }
        }
    }

    /// Check function call and return the result type
    pub fn check_function_call(
        &self,
        function_type: &ResolvedType,
        argument_types: &[ResolvedType],
    ) -> Result<ResolvedType, SemanticError> {
        match function_type {
            ResolvedType::Function(param_types, return_type) => {
                if param_types.len() != argument_types.len() {
                    return Err(SemanticError::ArityMismatch {
                        expected: param_types.len(),
                        found: argument_types.len(),
                    });
                }

                for (param_type, arg_type) in param_types.iter().zip(argument_types.iter()) {
                    if !self.types_compatible(param_type, arg_type) {
                        return Err(SemanticError::TypeMismatch {
                            expected: param_type.clone(),
                            found: arg_type.clone(),
                        });
                    }
                }

                Ok((**return_type).clone())
            }
            _ => Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Function(vec![], Box::new(ResolvedType::Null)),
                found: function_type.clone(),
            }),
        }
    }

    /// Check if a type can be used in a boolean context
    pub fn is_truthy_type(&self, type_: &ResolvedType) -> bool {
        match type_ {
            ResolvedType::Bool => true,
            ResolvedType::Null => false, // null is always falsy
            _ => false,                  // Other types need explicit conversion
        }
    }

    /// Get the common type for two types (for type inference)
    pub fn common_type(&self, type1: &ResolvedType, type2: &ResolvedType) -> Option<ResolvedType> {
        if self.types_compatible(type1, type2) {
            Some(type1.clone())
        } else if self.types_compatible(type2, type1) {
            Some(type2.clone())
        } else {
            // Special cases for numeric promotion
            match (type1, type2) {
                (ResolvedType::Int, ResolvedType::Float)
                | (ResolvedType::Float, ResolvedType::Int) => Some(ResolvedType::Float),
                _ => None,
            }
        }
    }

    /// Check if a type is a primitive type
    fn is_primitive_type(&self, type_name: &str) -> bool {
        matches!(type_name, "int" | "float" | "bool" | "string" | "char")
    }

    /// Check if a type is a collection type
    fn is_collection_type(&self, ast_type: &Type) -> bool {
        matches!(
            ast_type,
            Type::Array(_, _)
                | Type::Matrix(_, _)
                | Type::Vector(_, _)
                | Type::Set(_)
                | Type::Map(_, _)
                | Type::Queue(_)
                | Type::Stack(_)
                | Type::Tree(_)
                | Type::Graph(_, _)
        )
    }

    /// Check if a type is a concurrent type
    fn is_concurrent_type(&self, ast_type: &Type) -> bool {
        matches!(
            ast_type,
            Type::Channel(_) | Type::Mutex(_) | Type::Atomic(_)
        )
    }

    /// Check if a type is an AI type
    fn is_ai_type(&self, ast_type: &Type) -> bool {
        matches!(
            ast_type,
            Type::Tensor(_) | Type::Dataset(_) | Type::Model(_)
        )
    }

    /// Get the element type of a collection
    fn get_element_type<'a>(&self, ast_type: &'a Type) -> Option<&'a Type> {
        match ast_type {
            Type::Array(elem_type, _) => Some(elem_type),
            Type::Matrix(elem_type, _) => Some(elem_type),
            Type::Vector(elem_type, _) => Some(elem_type),
            Type::Set(elem_type) => Some(elem_type),
            Type::Queue(elem_type) => Some(elem_type),
            Type::Stack(elem_type) => Some(elem_type),
            Type::Tree(elem_type) => Some(elem_type),
            Type::Dataset(elem_type) => Some(elem_type),
            Type::Optional(elem_type) => Some(elem_type),
            Type::Reference(elem_type, _) => Some(elem_type),
            Type::Channel(elem_type) => Some(elem_type),
            Type::Mutex(elem_type) => Some(elem_type),
            Type::Atomic(elem_type) => Some(elem_type),
            _ => None,
        }
    }

    /// Find common super type between two types (Expert recommendation: Arm type compatibility)
    pub fn common_super_type(
        &self,
        type1: &ResolvedType,
        type2: &ResolvedType,
    ) -> Option<ResolvedType> {
        // If types are identical, return that type
        if type1 == type2 {
            return Some(type1.clone());
        }

        // Handle numeric type promotion
        match (type1, type2) {
            // Int and Float -> Float
            (ResolvedType::Int, ResolvedType::Float) | (ResolvedType::Float, ResolvedType::Int) => {
                Some(ResolvedType::Float)
            }
            // For now, other combinations don't have a common super type
            // In a more advanced type system, we might have:
            // - Union types
            // - Trait objects
            // - Common base classes
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_type_inference() {
        let type_checker = TypeChecker::new();

        assert_eq!(
            type_checker.infer_literal_type(&Literal::Integer(42)),
            ResolvedType::Int
        );
        assert_eq!(
            type_checker.infer_literal_type(&Literal::Float(3.14)),
            ResolvedType::Float
        );
        assert_eq!(
            type_checker.infer_literal_type(&Literal::Boolean(true)),
            ResolvedType::Bool
        );
        assert_eq!(
            type_checker.infer_literal_type(&Literal::String("hello".to_string())),
            ResolvedType::String
        );
    }

    #[test]
    fn test_type_compatibility() {
        let type_checker = TypeChecker::new();

        // Exact matches
        assert!(type_checker.types_compatible(&ResolvedType::Int, &ResolvedType::Int));
        assert!(type_checker.types_compatible(&ResolvedType::String, &ResolvedType::String));

        // Numeric promotion
        assert!(type_checker.types_compatible(&ResolvedType::Float, &ResolvedType::Int));
        assert!(!type_checker.types_compatible(&ResolvedType::Int, &ResolvedType::Float));

        // Incompatible types
        assert!(!type_checker.types_compatible(&ResolvedType::Int, &ResolvedType::String));
    }

    #[test]
    fn test_arithmetic_operations() {
        let type_checker = TypeChecker::new();

        // Integer arithmetic
        let result = type_checker.check_binary_operation(
            &BinaryOperator::Add,
            &ResolvedType::Int,
            &ResolvedType::Int,
        );
        assert_eq!(result.unwrap(), ResolvedType::Int);

        // Mixed arithmetic (should promote to float)
        let result = type_checker.check_binary_operation(
            &BinaryOperator::Add,
            &ResolvedType::Int,
            &ResolvedType::Float,
        );
        assert_eq!(result.unwrap(), ResolvedType::Float);

        // String concatenation
        let result = type_checker.check_binary_operation(
            &BinaryOperator::Add,
            &ResolvedType::String,
            &ResolvedType::String,
        );
        assert_eq!(result.unwrap(), ResolvedType::String);
    }

    #[test]
    fn test_comparison_operations() {
        let type_checker = TypeChecker::new();

        // Numeric comparison
        let result = type_checker.check_binary_operation(
            &BinaryOperator::Less,
            &ResolvedType::Int,
            &ResolvedType::Int,
        );
        assert_eq!(result.unwrap(), ResolvedType::Bool);

        // String comparison
        let result = type_checker.check_binary_operation(
            &BinaryOperator::Equal,
            &ResolvedType::String,
            &ResolvedType::String,
        );
        assert_eq!(result.unwrap(), ResolvedType::Bool);
    }
}
