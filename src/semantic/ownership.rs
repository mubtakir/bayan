//! # Ownership and Borrowing Analysis
//! 
//! This module implements ownership and borrowing checking for memory safety.
//! It ensures that the AlBayan language's ownership rules are enforced at compile time.

use crate::parser::ast::*;
use super::{ResolvedType, SemanticError};
use std::collections::{HashMap, HashSet};

/// Ownership analyzer for memory safety
#[derive(Debug)]
pub struct OwnershipAnalyzer {
    /// Variables and their ownership status
    variables: HashMap<String, OwnershipInfo>,
    /// Active borrows
    active_borrows: Vec<BorrowInfo>,
    /// Current scope depth
    scope_depth: usize,
}

/// Information about a variable's ownership
#[derive(Debug, Clone)]
struct OwnershipInfo {
    /// Variable name
    name: String,
    /// Variable type
    var_type: ResolvedType,
    /// Whether the variable is moved
    is_moved: bool,
    /// Whether the variable is mutable
    is_mutable: bool,
    /// Scope where the variable was declared
    scope_depth: usize,
}

/// Information about an active borrow
#[derive(Debug, Clone)]
struct BorrowInfo {
    /// Borrowed variable name
    variable: String,
    /// Type of borrow
    borrow_type: BorrowType,
    /// Scope where the borrow was created
    scope_depth: usize,
}

/// Type of borrow
#[derive(Debug, Clone, PartialEq)]
enum BorrowType {
    /// Immutable borrow (&T)
    Immutable,
    /// Mutable borrow (&mut T)
    Mutable,
}

impl OwnershipAnalyzer {
    /// Create a new ownership analyzer
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            active_borrows: Vec::new(),
            scope_depth: 0,
        }
    }
    
    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }
    
    /// Exit the current scope
    pub fn exit_scope(&mut self) {
        // Remove variables declared in this scope
        self.variables.retain(|_, info| info.scope_depth < self.scope_depth);
        
        // Remove borrows created in this scope
        self.active_borrows.retain(|borrow| borrow.scope_depth < self.scope_depth);
        
        if self.scope_depth > 0 {
            self.scope_depth -= 1;
        }
    }
    
    /// Declare a new variable
    pub fn declare_variable(
        &mut self,
        name: &str,
        var_type: ResolvedType,
        is_mutable: bool,
    ) -> Result<(), SemanticError> {
        if self.variables.contains_key(name) {
            return Err(SemanticError::Redefinition(name.to_string()));
        }
        
        self.variables.insert(name.to_string(), OwnershipInfo {
            name: name.to_string(),
            var_type,
            is_moved: false,
            is_mutable,
            scope_depth: self.scope_depth,
        });
        
        Ok(())
    }
    
    /// Check if a variable can be used (not moved)
    pub fn check_variable_use(&self, name: &str) -> Result<&OwnershipInfo, SemanticError> {
        let var_info = self.variables.get(name)
            .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;
        
        if var_info.is_moved {
            return Err(SemanticError::UseAfterMove(name.to_string()));
        }
        
        Ok(var_info)
    }
    
    /// Move a variable (transfer ownership)
    pub fn move_variable(&mut self, name: &str) -> Result<(), SemanticError> {
        // First check if variable exists and get its type
        let var_type = {
            let var_info = self.variables.get(name)
                .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;

            if var_info.is_moved {
                return Err(SemanticError::UseAfterMove(name.to_string()));
            }

            var_info.var_type.clone()
        };

        // Check if the type is movable (Copy types don't move)
        if !self.is_copy_type(&var_type) {
            let var_info = self.variables.get_mut(name).unwrap();
            var_info.is_moved = true;
        }

        Ok(())
    }
    
    /// Create an immutable borrow
    pub fn borrow_immutable(&mut self, name: &str) -> Result<(), SemanticError> {
        let var_info = self.check_variable_use(name)?;
        
        // Check for conflicting mutable borrows
        for borrow in &self.active_borrows {
            if borrow.variable == name && borrow.borrow_type == BorrowType::Mutable {
                return Err(SemanticError::ConflictingBorrow(name.to_string()));
            }
        }
        
        self.active_borrows.push(BorrowInfo {
            variable: name.to_string(),
            borrow_type: BorrowType::Immutable,
            scope_depth: self.scope_depth,
        });
        
        Ok(())
    }
    
    /// Create a mutable borrow
    pub fn borrow_mutable(&mut self, name: &str) -> Result<(), SemanticError> {
        let var_info = self.check_variable_use(name)?;
        
        if !var_info.is_mutable {
            return Err(SemanticError::BorrowMutableFromImmutable(name.to_string()));
        }
        
        // Check for any existing borrows
        for borrow in &self.active_borrows {
            if borrow.variable == name {
                return Err(SemanticError::ConflictingBorrow(name.to_string()));
            }
        }
        
        self.active_borrows.push(BorrowInfo {
            variable: name.to_string(),
            borrow_type: BorrowType::Mutable,
            scope_depth: self.scope_depth,
        });
        
        Ok(())
    }
    
    /// Check if a type implements Copy (doesn't move on assignment)
    fn is_copy_type(&self, type_: &ResolvedType) -> bool {
        match type_ {
            // Primitive types are Copy
            ResolvedType::Int | ResolvedType::Float | ResolvedType::Bool | ResolvedType::Char => true,
            
            // String is not Copy (it's owned)
            ResolvedType::String => false,
            
            // User-defined types are not Copy by default
            ResolvedType::Struct(_) | ResolvedType::Enum(_) => false,
            
            // Functions are Copy
            ResolvedType::Function(_, _) => true,
            
            // Generic types depend on their parameters
            ResolvedType::Generic(_, _) => false, // Conservative approach
            
            ResolvedType::Null => true,
            _ => false, // For all new types, default to non-copyable
        }
    }
    
    /// Analyze an expression for ownership
    pub fn analyze_expression(&mut self, expr: &Expression) -> Result<OwnershipResult, SemanticError> {
        match expr {
            Expression::Identifier(name) => {
                self.check_variable_use(name)?;
                
                // For now, we assume all identifier access moves the value
                // unless it's a Copy type
                let var_info = &self.variables[name];
                if self.is_copy_type(&var_info.var_type) {
                    Ok(OwnershipResult::Copy)
                } else {
                    self.move_variable(name)?;
                    Ok(OwnershipResult::Move)
                }
            }
            
            Expression::Literal(_) => Ok(OwnershipResult::Copy),
            
            Expression::Binary(bin_expr) => {
                // Analyze both operands
                let left_result = self.analyze_expression(&bin_expr.left)?;
                let right_result = self.analyze_expression(&bin_expr.right)?;
                
                // Binary operations typically produce new values
                Ok(OwnershipResult::Copy)
            }
            
            Expression::Unary(unary_expr) => {
                match unary_expr.operator {
                    UnaryOperator::Reference => {
                        // Taking a reference creates a borrow
                        if let Expression::Identifier(name) = &*unary_expr.operand {
                            self.borrow_immutable(name)?;
                            Ok(OwnershipResult::Borrow)
                        } else {
                            Err(SemanticError::InvalidBorrow("Cannot borrow non-variable".to_string()))
                        }
                    }
                    
                    UnaryOperator::Dereference => {
                        // Dereferencing uses the borrowed value
                        self.analyze_expression(&unary_expr.operand)?;
                        Ok(OwnershipResult::Copy)
                    }
                    
                    _ => {
                        self.analyze_expression(&unary_expr.operand)?;
                        Ok(OwnershipResult::Copy)
                    }
                }
            }
            
            Expression::Call(call_expr) => {
                // Analyze the callee
                self.analyze_expression(&call_expr.callee)?;
                
                // Analyze all arguments (they may be moved)
                for arg in &call_expr.arguments {
                    self.analyze_expression(arg)?;
                }
                
                Ok(OwnershipResult::Copy)
            }
            
            _ => {
                // For other expressions, conservatively assume they're safe
                Ok(OwnershipResult::Copy)
            }
        }
    }
    
    /// Analyze a statement for ownership
    pub fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), SemanticError> {
        match stmt {
            Statement::Let(let_stmt) => {
                // Analyze the initializer first
                if let Some(initializer) = &let_stmt.initializer {
                    self.analyze_expression(initializer)?;
                }
                
                // Declare the variable
                // TODO: Extract mutability from AST
                let is_mutable = false; // Placeholder
                let var_type = ResolvedType::Int; // Placeholder - should come from type checker
                self.declare_variable(&let_stmt.name, var_type, is_mutable)?;
            }
            
            Statement::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            
            Statement::Return(ret_stmt) => {
                if let Some(value) = &ret_stmt.value {
                    self.analyze_expression(value)?;
                }
            }
            
            _ => {
                // Other statements not yet implemented
            }
        }
        
        Ok(())
    }
}

/// Result of ownership analysis for an expression
#[derive(Debug, Clone, PartialEq)]
pub enum OwnershipResult {
    /// Value is copied (Copy types)
    Copy,
    /// Value is moved (ownership transferred)
    Move,
    /// Value is borrowed
    Borrow,
}

// Add new error types to SemanticError
impl SemanticError {
    pub fn UseAfterMove(var: String) -> Self {
        SemanticError::UseAfterMove(var)
    }
    
    pub fn ConflictingBorrow(var: String) -> Self {
        SemanticError::ConflictingBorrow(var)
    }
    
    pub fn BorrowMutableFromImmutable(var: String) -> Self {
        SemanticError::BorrowMutableFromImmutable(var)
    }
    
    pub fn InvalidBorrow(msg: String) -> Self {
        SemanticError::InvalidBorrow(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variable_declaration() {
        let mut analyzer = OwnershipAnalyzer::new();
        
        let result = analyzer.declare_variable("x", ResolvedType::Int, false);
        assert!(result.is_ok());
        
        let var_info = analyzer.check_variable_use("x");
        assert!(var_info.is_ok());
    }
    
    #[test]
    fn test_copy_types() {
        let analyzer = OwnershipAnalyzer::new();
        
        assert!(analyzer.is_copy_type(&ResolvedType::Int));
        assert!(analyzer.is_copy_type(&ResolvedType::Bool));
        assert!(!analyzer.is_copy_type(&ResolvedType::String));
    }
    
    #[test]
    fn test_scope_management() {
        let mut analyzer = OwnershipAnalyzer::new();
        
        analyzer.declare_variable("global", ResolvedType::Int, false).unwrap();
        
        analyzer.enter_scope();
        analyzer.declare_variable("local", ResolvedType::String, false).unwrap();
        
        assert!(analyzer.check_variable_use("global").is_ok());
        assert!(analyzer.check_variable_use("local").is_ok());
        
        analyzer.exit_scope();
        
        assert!(analyzer.check_variable_use("global").is_ok());
        assert!(analyzer.check_variable_use("local").is_err());
    }
}
