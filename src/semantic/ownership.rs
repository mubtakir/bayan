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
    /// Borrow check state (Expert recommendation)
    borrow_check_state: BorrowCheckState,
}

/// Borrow check state for tracking ownership and moves (Expert recommendation)
#[derive(Debug, Clone)]
pub struct BorrowCheckState {
    /// Variables that need to be destroyed at end of scope
    variables_to_destroy: HashMap<String, DestroyInfo>,
    /// Variables that have been moved
    moved_variables: HashSet<String>,
    /// Active borrows (Expert recommendation: &/&mut tracking)
    active_borrows: HashMap<String, Vec<BorrowInfo>>,
    /// Current function being analyzed
    current_function: Option<String>,
}

/// Information about an active borrow (Expert recommendation)
#[derive(Debug, Clone)]
pub struct BorrowInfo {
    /// Type of borrow
    pub borrow_kind: BorrowKind,
    /// Scope depth where borrow was created
    pub scope_depth: usize,
    /// Borrow path for field-level borrowing (Expert recommendation: Priority 1)
    pub path: BorrowPath,
}

/// Borrow path for field-level borrowing (Expert recommendation: Priority 1)
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowPath {
    /// Variable borrow: &var
    Variable(String),
    /// Field borrow: &var.field
    Field(String, String), // (variable_name, field_name)
    /// Nested field borrow: &var.field1.field2 (for future expansion)
    NestedField(String, Vec<String>), // (variable_name, field_path)
}

/// Kind of borrow (Expert recommendation)
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowKind {
    Immutable, // &
    Mutable,   // &mut
}

/// Information about variables that need destruction (Expert recommendation)
#[derive(Debug, Clone)]
pub struct DestroyInfo {
    /// Variable name
    name: String,
    /// Variable type
    var_type: ResolvedType,
    /// Scope depth where variable should be destroyed
    scope_depth: usize,
}

/// Information about a variable's ownership
#[derive(Debug, Clone)]
pub struct OwnershipInfo {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: ResolvedType,
    /// Whether the variable is moved
    pub is_moved: bool,
    /// Whether the variable is mutable
    pub is_mutable: bool,
    /// Scope where the variable was declared
    pub scope_depth: usize,
}



impl BorrowCheckState {
    /// Create a new borrow check state (Expert recommendation)
    pub fn new() -> Self {
        Self {
            variables_to_destroy: HashMap::new(),
            moved_variables: HashSet::new(),
            active_borrows: HashMap::new(),
            current_function: None,
        }
    }

    /// Mark a variable as moved (Expert recommendation)
    pub fn mark_as_moved(&mut self, name: &str) {
        self.moved_variables.insert(name.to_string());
        // Remove from variables to destroy since it's moved
        self.variables_to_destroy.remove(name);
    }

    /// Check if a variable has been moved (Expert recommendation)
    pub fn is_moved(&self, name: &str) -> bool {
        self.moved_variables.contains(name)
    }

    /// Add a borrow (Expert recommendation: &/&mut tracking with path analysis)
    pub fn add_borrow(&mut self, name: &str, borrow_kind: BorrowKind, scope_depth: usize) -> Result<(), SemanticError> {
        self.add_borrow_with_path(name, borrow_kind, scope_depth, BorrowPath::Variable(name.to_string()))
    }

    /// Add a borrow with path (Expert recommendation: Priority 1 - Field-level borrowing)
    pub fn add_borrow_with_path(&mut self, name: &str, borrow_kind: BorrowKind, scope_depth: usize, path: BorrowPath) -> Result<(), SemanticError> {
        // Check for conflicting borrows with path analysis
        if let Some(existing_borrows) = self.active_borrows.get(name) {
            for existing_borrow in existing_borrows {
                // Check if paths conflict
                if self.paths_conflict(&existing_borrow.path, &path) {
                    match (&existing_borrow.borrow_kind, &borrow_kind) {
                        // &mut conflicts with any other borrow on the same path
                        (BorrowKind::Mutable, _) | (_, BorrowKind::Mutable) => {
                            return Err(SemanticError::ConflictingBorrow(format!(
                                "Cannot borrow '{}' as {:?} because it conflicts with existing {:?} borrow",
                                name, borrow_kind, existing_borrow.borrow_kind
                            )));
                        }
                        // & with & is allowed on the same path
                        (BorrowKind::Immutable, BorrowKind::Immutable) => {}
                    }
                }
            }
        }

        // Add the new borrow
        let borrow_info = BorrowInfo {
            borrow_kind,
            scope_depth,
            path,
        };

        self.active_borrows
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(borrow_info);

        Ok(())
    }

    /// Check if two borrow paths conflict (Expert recommendation: Priority 1)
    fn paths_conflict(&self, path1: &BorrowPath, path2: &BorrowPath) -> bool {
        match (path1, path2) {
            // Variable borrows conflict with any other borrow of the same variable
            (BorrowPath::Variable(var1), BorrowPath::Variable(var2)) => var1 == var2,

            // Variable borrow conflicts with field borrow of the same variable
            (BorrowPath::Variable(var1), BorrowPath::Field(var2, _)) |
            (BorrowPath::Field(var1, _), BorrowPath::Variable(var2)) => var1 == var2,

            // Field borrows conflict if they're the same field of the same variable
            (BorrowPath::Field(var1, field1), BorrowPath::Field(var2, field2)) => {
                var1 == var2 && field1 == field2
            }

            // For nested fields, check if any part of the path overlaps
            (BorrowPath::NestedField(var1, path1), BorrowPath::NestedField(var2, path2)) => {
                if var1 != var2 { return false; }
                // Check if one path is a prefix of the other
                let min_len = path1.len().min(path2.len());
                path1[..min_len] == path2[..min_len]
            }

            // Mixed nested field cases (simplified for now)
            _ => false,
        }
    }

    /// Check if a variable has any active borrows (Expert recommendation)
    pub fn has_active_borrows(&self, name: &str) -> bool {
        self.active_borrows.get(name).map_or(false, |borrows| !borrows.is_empty())
    }

    /// Check if a variable has mutable borrows (Expert recommendation)
    pub fn has_mutable_borrow(&self, name: &str) -> bool {
        self.active_borrows.get(name).map_or(false, |borrows| {
            borrows.iter().any(|b| b.borrow_kind == BorrowKind::Mutable)
        })
    }

    /// Add a field borrow (Expert recommendation: Priority 1 - Field-level borrowing)
    pub fn add_field_borrow(&mut self, var_name: &str, field_name: &str, borrow_kind: BorrowKind, scope_depth: usize) -> Result<(), SemanticError> {
        let path = BorrowPath::Field(var_name.to_string(), field_name.to_string());
        self.add_borrow_with_path(var_name, borrow_kind, scope_depth, path)
    }

    /// Check if a field has active borrows (Expert recommendation: Priority 1)
    pub fn has_field_borrow(&self, var_name: &str, field_name: &str) -> bool {
        self.active_borrows.get(var_name).map_or(false, |borrows| {
            borrows.iter().any(|b| {
                matches!(&b.path, BorrowPath::Field(var, field) if var == var_name && field == field_name)
            })
        })
    }

    /// Register a variable for destruction at end of scope (Expert recommendation)
    pub fn register_for_destruction(&mut self, name: &str, var_type: ResolvedType, scope_depth: usize) {
        // Register types that need destruction (Expert recommendation: Priority 1)
        let needs_destruction = match &var_type {
            ResolvedType::List(_) => true,
            ResolvedType::String => true,
            ResolvedType::Struct(_) => true,
            // AI types need destruction (Expert recommendation: Priority 1)
            ResolvedType::Model(_) => true,
            ResolvedType::Tensor(_) => true,
            // Copy types don't need destruction
            ResolvedType::Int | ResolvedType::Float | ResolvedType::Bool | ResolvedType::Char => false,
            _ => false, // Conservative: assume other types need destruction
        };

        if needs_destruction {
            self.variables_to_destroy.insert(name.to_string(), DestroyInfo {
                name: name.to_string(),
                var_type,
                scope_depth,
            });
        }
    }

    /// Get variables that need to be destroyed at given scope depth (Expert recommendation)
    pub fn get_variables_to_destroy_at_scope(&self, scope_depth: usize) -> Vec<&DestroyInfo> {
        self.variables_to_destroy
            .values()
            .filter(|info| info.scope_depth == scope_depth)
            .collect()
    }

    /// Clear variables at scope exit (Expert recommendation)
    pub fn clear_scope(&mut self, scope_depth: usize) {
        self.variables_to_destroy.retain(|_, info| info.scope_depth < scope_depth);
        // Clear borrows created in this scope (Expert recommendation)
        for borrows in self.active_borrows.values_mut() {
            borrows.retain(|borrow| borrow.scope_depth < scope_depth);
        }
        // Remove empty borrow entries
        self.active_borrows.retain(|_, borrows| !borrows.is_empty());
        // Note: moved_variables persist across scopes until function exit
    }

    /// Set current function being analyzed (Expert recommendation)
    pub fn set_current_function(&mut self, function_name: Option<String>) {
        let is_none = function_name.is_none();
        self.current_function = function_name;
        if is_none {
            // Clear all state when exiting function
            self.moved_variables.clear();
            self.variables_to_destroy.clear();
            self.active_borrows.clear();
        }
    }

    /// Check if write access is allowed (Expert recommendation)
    pub fn check_write_access(&self, name: &str) -> Result<(), SemanticError> {
        if self.is_moved(name) {
            return Err(SemanticError::UseAfterMove(name.to_string()));
        }

        // Check if variable is borrowed immutably (Expert recommendation)
        if let Some(borrows) = self.active_borrows.get(name) {
            for borrow in borrows {
                if borrow.borrow_kind == BorrowKind::Immutable {
                    return Err(SemanticError::WriteWhileBorrowed(name.to_string()));
                }
            }
        }

        Ok(())
    }

    /// Clone the current state for control flow analysis (Expert recommendation: Priority 2)
    pub fn clone_state(&self) -> Self {
        Self {
            variables_to_destroy: self.variables_to_destroy.clone(),
            moved_variables: self.moved_variables.clone(),
            active_borrows: self.active_borrows.clone(),
            current_function: self.current_function.clone(),
        }
    }

    /// Merge two borrow check states at control flow join points (Expert recommendation: Priority 2)
    pub fn merge_states(&mut self, other: &BorrowCheckState) {
        // For moved variables: if a variable is moved in ANY path, consider it moved
        // This is the conservative approach - "worst case" analysis
        for moved_var in &other.moved_variables {
            self.moved_variables.insert(moved_var.clone());
        }

        // For active borrows: keep borrows that exist in BOTH paths
        // This ensures we don't have false conflicts after control flow merge
        let mut merged_borrows = HashMap::new();
        for (var_name, self_borrows) in &self.active_borrows {
            if let Some(other_borrows) = other.active_borrows.get(var_name) {
                // Keep borrows that exist in both paths
                let mut common_borrows = Vec::new();
                for self_borrow in self_borrows {
                    for other_borrow in other_borrows {
                        // If the same borrow exists in both paths, keep it
                        if self_borrow.borrow_kind == other_borrow.borrow_kind &&
                           self_borrow.path == other_borrow.path &&
                           self_borrow.scope_depth == other_borrow.scope_depth {
                            common_borrows.push(self_borrow.clone());
                            break;
                        }
                    }
                }
                if !common_borrows.is_empty() {
                    merged_borrows.insert(var_name.clone(), common_borrows);
                }
            }
        }
        self.active_borrows = merged_borrows;

        // For variables to destroy: merge both sets
        for (var_name, destroy_info) in &other.variables_to_destroy {
            self.variables_to_destroy.insert(var_name.clone(), destroy_info.clone());
        }
    }

    /// Check for borrow conflicts (Expert recommendation: Priority 1)
    pub fn check_borrow_conflicts(&self, name: &str, borrow_kind: BorrowKind) -> Result<(), super::SemanticError> {
        if let Some(existing_borrows) = self.active_borrows.get(name) {
            for existing_borrow in existing_borrows {
                // Conflict rules:
                // 1. Multiple immutable borrows are allowed
                // 2. Mutable borrow conflicts with any other borrow
                // 3. Any borrow conflicts with mutable borrow
                match (&existing_borrow.borrow_kind, &borrow_kind) {
                    (BorrowKind::Immutable, BorrowKind::Immutable) => {
                        // Multiple immutable borrows are allowed
                        continue;
                    }
                    (BorrowKind::Mutable, _) | (_, BorrowKind::Mutable) => {
                        // Mutable borrow conflicts with any other borrow
                        return Err(super::SemanticError::BorrowConflict {
                            variable: name.to_string(),
                            existing_borrow: existing_borrow.borrow_kind.clone(),
                            new_borrow: borrow_kind,
                        });
                    }
                }
            }
        }
        Ok(())
    }

}

impl OwnershipAnalyzer {
    /// Create a new ownership analyzer
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            active_borrows: Vec::new(),
            scope_depth: 0,
            borrow_check_state: BorrowCheckState::new(),
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// Exit the current scope
    pub fn exit_scope(&mut self) -> Vec<DestroyInfo> {
        // Get variables that need to be destroyed at this scope (Expert recommendation)
        let variables_to_destroy = self.borrow_check_state
            .get_variables_to_destroy_at_scope(self.scope_depth)
            .into_iter()
            .cloned()
            .collect();

        // Remove variables declared in this scope
        self.variables.retain(|_, info| info.scope_depth < self.scope_depth);

        // Remove borrows created in this scope
        self.active_borrows.retain(|borrow| borrow.scope_depth < self.scope_depth);

        // Clear borrow check state for this scope
        self.borrow_check_state.clear_scope(self.scope_depth);

        if self.scope_depth > 0 {
            self.scope_depth -= 1;
        }

        variables_to_destroy
    }

    /// Get current scope depth (Expert recommendation: Priority 1)
    pub fn get_scope_depth(&self) -> usize {
        self.scope_depth
    }

    /// Register a variable for destruction (Expert recommendation: Priority 1)
    pub fn register_for_destruction(&mut self, name: &str, var_type: ResolvedType, scope_depth: usize) {
        self.borrow_check_state.register_for_destruction(name, var_type, scope_depth);
    }

    /// Check for borrow conflicts (Expert recommendation: Priority 1)
    pub fn check_borrow_conflicts(&mut self, name: &str, borrow_kind: BorrowKind) -> Result<(), super::SemanticError> {
        self.borrow_check_state.check_borrow_conflicts(name, borrow_kind)
    }

    /// Get variable information for dangling reference analysis (Expert recommendation: Priority 1)
    pub fn get_variable_info(&self, name: &str) -> Option<&OwnershipInfo> {
        self.variables.get(name)
    }

    /// Add a borrow (Expert recommendation: Priority 1)
    pub fn add_borrow(&mut self, name: &str, borrow_kind: BorrowKind) -> Result<(), super::SemanticError> {
        self.borrow_check_state.add_borrow(name, borrow_kind, self.scope_depth)
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
            var_type: var_type.clone(),
            is_moved: false,
            is_mutable,
            scope_depth: self.scope_depth,
        });

        // Register for destruction if needed (Expert recommendation)
        self.borrow_check_state.register_for_destruction(name, var_type, self.scope_depth);

        Ok(())
    }

    /// Check if a variable can be used (not moved)
    pub fn check_variable_use(&self, name: &str) -> Result<&OwnershipInfo, SemanticError> {
        let var_info = self.variables.get(name)
            .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;

        if var_info.is_moved || self.borrow_check_state.is_moved(name) {
            return Err(SemanticError::UseAfterMove(name.to_string()));
        }

        Ok(var_info)
    }

    /// Mark a variable as moved (Expert recommendation)
    pub fn mark_as_moved(&mut self, name: &str) -> Result<(), SemanticError> {
        // Check if variable exists
        let var_info = self.variables.get_mut(name)
            .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;

        // Check if already moved
        if var_info.is_moved || self.borrow_check_state.is_moved(name) {
            return Err(SemanticError::UseAfterMove(name.to_string()));
        }

        // Mark as moved in both places
        var_info.is_moved = true;
        self.borrow_check_state.mark_as_moved(name);

        Ok(())
    }

    /// Check read access to a variable (Expert recommendation)
    pub fn check_read_access(&self, name: &str) -> Result<(), SemanticError> {
        // Check if variable exists
        let _var_info = self.variables.get(name)
            .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;

        // Check if moved
        if self.borrow_check_state.is_moved(name) {
            return Err(SemanticError::UseAfterMove(name.to_string()));
        }

        // TODO: Check for conflicting borrows
        // For now, allow all reads
        Ok(())
    }

    /// Set current function for borrow checking (Expert recommendation)
    pub fn set_current_function(&mut self, function_name: Option<String>) {
        self.borrow_check_state.set_current_function(function_name);
    }

    /// Get current scope depth (Expert recommendation: Priority 2)
    pub fn get_current_scope_depth(&self) -> usize {
        self.scope_depth
    }

    /// Add a borrow for method calls (Expert recommendation: Priority 2)
    pub fn add_method_borrow(&mut self, var_name: &str, borrow_kind: BorrowKind, scope_depth: usize) -> Result<(), SemanticError> {
        self.borrow_check_state.add_borrow(var_name, borrow_kind, scope_depth)
    }

    /// Get mutable access to borrow check state (Expert recommendation)
    pub fn get_borrow_check_state_mut(&mut self) -> &mut BorrowCheckState {
        &mut self.borrow_check_state
    }



    /// Get variables that need destruction at current scope (Expert recommendation)
    pub fn get_variables_to_destroy(&self) -> Vec<&DestroyInfo> {
        self.borrow_check_state.get_variables_to_destroy_at_scope(self.scope_depth)
    }

    /// Move a variable (transfer ownership)
    pub fn move_variable(&mut self, name: &str) -> Result<(), SemanticError> {
        // First check if variable exists and get its type
        let var_type = {
            let var_info = self.variables.get(name)
                .ok_or_else(|| SemanticError::UndefinedVariable(name.to_string()))?;

            if var_info.is_moved || self.borrow_check_state.is_moved(name) {
                return Err(SemanticError::UseAfterMove(name.to_string()));
            }

            var_info.var_type.clone()
        };

        // Check if the type is movable (Copy types don't move)
        if !self.is_copy_type(&var_type) {
            // Use the new mark_as_moved function (Expert recommendation)
            self.mark_as_moved(name)?;
        }

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
                // For now, just analyze the operand
                self.analyze_expression(&unary_expr.operand)?;
                Ok(OwnershipResult::Copy)
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

            Statement::If(if_stmt) => {
                // Control flow analysis for if statements (Expert recommendation: Priority 2)
                self.analyze_if_statement_ownership(if_stmt)?;
            }

            _ => {
                // Other statements not yet implemented
            }
        }

        Ok(())
    }

    /// Analyze if statement with control flow merging (Expert recommendation: Priority 2)
    pub fn analyze_if_statement_ownership(&mut self, if_stmt: &IfStatement) -> Result<(), SemanticError> {
        // Analyze condition
        self.analyze_expression(&if_stmt.condition)?;

        // Clone current state before analyzing branches
        let initial_state = self.borrow_check_state.clone_state();

        // Analyze then branch
        self.enter_scope();
        self.analyze_block_ownership(&if_stmt.then_block)?;
        let then_state = self.borrow_check_state.clone_state();
        self.exit_scope();

        // Restore initial state and analyze else branch (if exists)
        self.borrow_check_state = initial_state.clone();
        let else_state = if let Some(else_block) = &if_stmt.else_block {
            self.enter_scope();
            self.analyze_block_ownership(else_block)?;
            let state = self.borrow_check_state.clone_state();
            self.exit_scope();
            state
        } else {
            // If no else branch, the "else" state is the initial state
            initial_state.clone()
        };

        // Merge the states from both branches (Expert recommendation: "worst case" analysis)
        self.borrow_check_state = initial_state;
        self.borrow_check_state.merge_states(&then_state);
        self.borrow_check_state.merge_states(&else_state);

        Ok(())
    }

    /// Analyze a block for ownership (helper method)
    pub fn analyze_block_ownership(&mut self, block: &Block) -> Result<(), SemanticError> {
        for stmt in &block.statements {
            self.analyze_statement(stmt)?;
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
