//! # Semantic Analysis Module
//!
//! This module implements semantic analysis for the AlBayan programming language.
//! It performs type checking, scope resolution, ownership analysis, and logic validation.

pub mod symbol_table;
pub mod type_checker;
pub mod ownership;
pub mod logic_analyzer;

use crate::parser::ast::*;
use crate::CompilerOptions;
use std::collections::HashMap;

pub use symbol_table::SymbolTable;
pub use type_checker::TypeChecker;

/// Main semantic analyzer
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_checker: TypeChecker,
    options: CompilerOptions,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new(options: &CompilerOptions) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            type_checker: TypeChecker::new(),
            options: options.clone(),
            errors: Vec::new(),
        }
    }

    /// Analyze the AST and return an annotated version
    pub fn analyze(&mut self, program: Program) -> Result<AnnotatedProgram, SemanticError> {
        // Two-pass analysis

        // Pass 1: Collect all top-level symbols
        self.collect_symbols(&program)?;

        // Pass 2: Detailed analysis
        self.analyze_program(&program)
    }

    /// First pass: collect all top-level declarations
    fn collect_symbols(&mut self, program: &Program) -> Result<(), SemanticError> {
        for item in &program.items {
            match item {
                Item::Function(func) => {
                    self.symbol_table.declare_function(&func.name, func)?;
                }
                Item::Struct(struct_decl) => {
                    self.symbol_table.declare_struct(&struct_decl.name, struct_decl)?;
                }
                Item::Enum(enum_decl) => {
                    self.symbol_table.declare_enum(&enum_decl.name, enum_decl)?;
                }
                Item::Class(class_decl) => {
                    self.symbol_table.declare_class(&class_decl.name, class_decl)?;
                }
                Item::Interface(interface_decl) => {
                    self.symbol_table.declare_interface(&interface_decl.name, interface_decl)?;
                }
                Item::Relation(relation_decl) => {
                    self.symbol_table.declare_relation(&relation_decl.name, relation_decl)?;
                }
                _ => {} // Rules, facts, modules, etc. handled in second pass
            }
        }
        Ok(())
    }

    /// Second pass: detailed analysis
    fn analyze_program(&mut self, program: &Program) -> Result<AnnotatedProgram, SemanticError> {
        let mut annotated_items = Vec::new();

        for item in &program.items {
            let annotated_item = self.analyze_item(item)?;
            annotated_items.push(annotated_item);
        }

        if !self.errors.is_empty() {
            return Err(self.errors.remove(0)); // Return first error for now
        }

        Ok(AnnotatedProgram {
            items: annotated_items,
            symbol_table: self.symbol_table.clone(),
        })
    }

    /// Analyze a top-level item
    fn analyze_item(&mut self, item: &Item) -> Result<AnnotatedItem, SemanticError> {
        match item {
            Item::Function(func) => {
                let annotated_func = self.analyze_function(func)?;
                Ok(AnnotatedItem::Function(annotated_func))
            }
            Item::Struct(struct_decl) => {
                let annotated_struct = self.analyze_struct(struct_decl)?;
                Ok(AnnotatedItem::Struct(annotated_struct))
            }
            Item::Relation(relation_decl) => {
                let annotated_relation = self.analyze_relation(relation_decl)?;
                Ok(AnnotatedItem::Relation(annotated_relation))
            }
            Item::Rule(rule_decl) => {
                let annotated_rule = self.analyze_rule(rule_decl)?;
                Ok(AnnotatedItem::Rule(annotated_rule))
            }
            _ => todo!("Analysis for other item types not yet implemented"),
        }
    }

    /// Analyze a function
    fn analyze_function(&mut self, func: &FunctionDecl) -> Result<AnnotatedFunction, SemanticError> {
        // Enter function scope
        self.symbol_table.enter_scope();

        // Add parameters to scope
        let mut annotated_params = Vec::new();
        for param in &func.parameters {
            let resolved_type = self.type_checker.resolve_type(&param.param_type)?;
            self.symbol_table.declare_variable(&param.name, &resolved_type)?;
            annotated_params.push(AnnotatedParameter {
                name: param.name.clone(),
                param_type: resolved_type,
            });
        }

        // Analyze function body
        let annotated_body = self.analyze_block(&func.body)?;

        // Check return type consistency
        let return_type = if let Some(ret_type) = &func.return_type {
            Some(self.type_checker.resolve_type(ret_type)?)
        } else {
            None
        };

        // Check return path analysis (as recommended by expert)
        if let Some(ref ret_type) = return_type {
            if !self.analyze_block_for_return(&func.body, ret_type)? {
                return Err(SemanticError::MissingReturn(func.name.clone()));
            }
        }

        // Exit function scope
        self.symbol_table.exit_scope();

        Ok(AnnotatedFunction {
            name: func.name.clone(),
            parameters: annotated_params,
            return_type,
            body: annotated_body,
        })
    }

    /// Analyze a struct
    fn analyze_struct(&mut self, struct_decl: &StructDecl) -> Result<AnnotatedStruct, SemanticError> {
        let mut annotated_fields = Vec::new();

        for field in &struct_decl.fields {
            let resolved_type = self.type_checker.resolve_type(&field.field_type)?;
            annotated_fields.push(AnnotatedStructField {
                name: field.name.clone(),
                field_type: resolved_type,
            });
        }

        Ok(AnnotatedStruct {
            name: struct_decl.name.clone(),
            fields: annotated_fields,
        })
    }

    /// Analyze a relation declaration
    fn analyze_relation(&mut self, relation: &RelationDecl) -> Result<AnnotatedRelation, SemanticError> {
        let mut resolved_arg_types = Vec::new();

        for arg_type in &relation.arg_types {
            let resolved_type = self.type_checker.resolve_type(arg_type)?;
            resolved_arg_types.push(resolved_type);
        }

        Ok(AnnotatedRelation {
            name: relation.name.clone(),
            arg_types: resolved_arg_types,
        })
    }

    /// Analyze a rule declaration
    fn analyze_rule(&mut self, rule: &RuleDecl) -> Result<AnnotatedRule, SemanticError> {
        // Analyze head and body terms
        let annotated_head = self.analyze_logic_term(&rule.head)?;

        let mut annotated_body = Vec::new();
        for term in &rule.body {
            let annotated_term = self.analyze_logic_term(term)?;
            annotated_body.push(annotated_term);
        }

        // Check that all variables in the head are bound in the body
        self.check_rule_safety(&annotated_head, &annotated_body)?;

        Ok(AnnotatedRule {
            head: annotated_head,
            body: annotated_body,
        })
    }

    /// Analyze a logic term
    fn analyze_logic_term(&mut self, term: &LogicTerm) -> Result<AnnotatedLogicTerm, SemanticError> {
        // Check if relation exists and get its info
        let relation_info = {
            let info = self.symbol_table.lookup_relation(&term.name)
                .ok_or_else(|| SemanticError::UndefinedRelation(term.name.clone()))?;

            // Check argument count
            if term.args.len() != info.arg_types.len() {
                return Err(SemanticError::ArityMismatch {
                    relation: term.name.clone(),
                    expected: info.arg_types.len(),
                    found: term.args.len(),
                });
            }

            info.clone()
        };

        // Analyze arguments
        let mut annotated_args = Vec::new();
        for (arg, expected_type) in term.args.iter().zip(&relation_info.arg_types) {
            let annotated_arg = self.analyze_logic_arg(arg, expected_type)?;
            annotated_args.push(annotated_arg);
        }

        Ok(AnnotatedLogicTerm {
            name: term.name.clone(),
            args: annotated_args,
            relation_type: relation_info,
        })
    }

    /// Analyze a logic argument
    fn analyze_logic_arg(&mut self, arg: &LogicArg, expected_type: &ResolvedType) -> Result<AnnotatedLogicArg, SemanticError> {
        match arg {
            LogicArg::Variable(name) => {
                Ok(AnnotatedLogicArg::Variable {
                    name: name.clone(),
                    var_type: expected_type.clone(),
                })
            }
            LogicArg::Constant(name) => {
                // Check if constant exists and has correct type
                Ok(AnnotatedLogicArg::Constant {
                    name: name.clone(),
                    const_type: expected_type.clone(),
                })
            }
            LogicArg::StringConstant(s) => {
                if !matches!(expected_type, ResolvedType::String) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::String,
                        found: expected_type.clone(),
                    });
                }
                Ok(AnnotatedLogicArg::StringConstant(s.clone()))
            }
            LogicArg::IntConstant(n) => {
                if !matches!(expected_type, ResolvedType::Int) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Int,
                        found: expected_type.clone(),
                    });
                }
                Ok(AnnotatedLogicArg::IntConstant(*n))
            }
            LogicArg::FloatConstant(f) => {
                if !matches!(expected_type, ResolvedType::Float) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Float,
                        found: expected_type.clone(),
                    });
                }
                Ok(AnnotatedLogicArg::FloatConstant(*f))
            }
        }
    }

    /// Check rule safety (all head variables must be bound in body)
    fn check_rule_safety(&self, head: &AnnotatedLogicTerm, body: &[AnnotatedLogicTerm]) -> Result<(), SemanticError> {
        // Extract variables from head
        let mut head_vars = std::collections::HashSet::new();
        for arg in &head.args {
            if let AnnotatedLogicArg::Variable { name, .. } = arg {
                head_vars.insert(name.clone());
            }
        }

        // Extract variables from body
        let mut body_vars = std::collections::HashSet::new();
        for term in body {
            for arg in &term.args {
                if let AnnotatedLogicArg::Variable { name, .. } = arg {
                    body_vars.insert(name.clone());
                }
            }
        }

        // Check that all head variables are in body
        for head_var in &head_vars {
            if !body_vars.contains(head_var) {
                return Err(SemanticError::UnboundVariable(head_var.clone()));
            }
        }

        Ok(())
    }

    /// Analyze a block of statements
    fn analyze_block(&mut self, block: &Block) -> Result<AnnotatedBlock, SemanticError> {
        self.symbol_table.enter_scope();

        let mut annotated_statements = Vec::new();
        for stmt in &block.statements {
            let annotated_stmt = self.analyze_statement(stmt)?;
            annotated_statements.push(annotated_stmt);
        }

        self.symbol_table.exit_scope();

        Ok(AnnotatedBlock {
            statements: annotated_statements,
        })
    }

    /// Analyze a statement
    fn analyze_statement(&mut self, stmt: &Statement) -> Result<AnnotatedStatement, SemanticError> {
        match stmt {
            Statement::Let(let_stmt) => {
                let annotated_let = self.analyze_let_statement(let_stmt)?;
                Ok(AnnotatedStatement::Let(annotated_let))
            }
            Statement::Return(ret_stmt) => {
                let annotated_ret = self.analyze_return_statement(ret_stmt)?;
                Ok(AnnotatedStatement::Return(annotated_ret))
            }
            Statement::Expression(expr) => {
                let annotated_expr = self.analyze_expression(expr)?;
                Ok(AnnotatedStatement::Expression(annotated_expr))
            }
            _ => todo!("Analysis for other statement types not yet implemented"),
        }
    }

    /// Analyze a let statement
    fn analyze_let_statement(&mut self, let_stmt: &LetStatement) -> Result<AnnotatedLetStatement, SemanticError> {
        let var_type = if let Some(type_annotation) = &let_stmt.var_type {
            self.type_checker.resolve_type(type_annotation)?
        } else if let Some(initializer) = &let_stmt.initializer {
            let annotated_init = self.analyze_expression(initializer)?;
            annotated_init.result_type.clone()
        } else {
            return Err(SemanticError::CannotInferType(let_stmt.name.clone()));
        };

        // Declare variable in current scope
        self.symbol_table.declare_variable(&let_stmt.name, &var_type)?;

        let annotated_initializer = if let Some(initializer) = &let_stmt.initializer {
            Some(self.analyze_expression(initializer)?)
        } else {
            None
        };

        Ok(AnnotatedLetStatement {
            name: let_stmt.name.clone(),
            var_type: var_type,
            initializer: annotated_initializer,
        })
    }

    /// Analyze a return statement
    fn analyze_return_statement(&mut self, ret_stmt: &ReturnStatement) -> Result<AnnotatedReturnStatement, SemanticError> {
        let value = if let Some(expr) = &ret_stmt.value {
            Some(self.analyze_expression(expr)?)
        } else {
            None
        };

        Ok(AnnotatedReturnStatement { value })
    }

    /// Analyze an expression
    fn analyze_expression(&mut self, expr: &Expression) -> Result<AnnotatedExpression, SemanticError> {
        match expr {
            Expression::Literal(lit) => {
                let result_type = self.type_checker.infer_literal_type(lit);
                Ok(AnnotatedExpression {
                    expr: AnnotatedExpressionKind::Literal(lit.clone()),
                    result_type,
                })
            }
            Expression::Identifier(name) => {
                let var_info = self.symbol_table.lookup_variable(name)
                    .ok_or_else(|| SemanticError::UndefinedVariable(name.clone()))?;

                Ok(AnnotatedExpression {
                    expr: AnnotatedExpressionKind::Identifier(name.clone()),
                    result_type: var_info.var_type.clone(),
                })
            }
            Expression::Binary(bin_expr) => {
                self.analyze_binary_expression(bin_expr)
            }
            _ => todo!("Analysis for other expression types not yet implemented"),
        }
    }

    /// Analyze a binary expression
    fn analyze_binary_expression(&mut self, bin_expr: &BinaryExpression) -> Result<AnnotatedExpression, SemanticError> {
        let left = self.analyze_expression(&bin_expr.left)?;
        let right = self.analyze_expression(&bin_expr.right)?;

        let result_type = self.type_checker.check_binary_operation(
            &bin_expr.operator,
            &left.result_type,
            &right.result_type,
        )?;

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Binary {
                left: Box::new(left),
                operator: bin_expr.operator.clone(),
                right: Box::new(right),
            },
            result_type,
        })
    }

    /// Analyze return path in a block (as recommended by expert)
    /// Returns true if the block guarantees a return
    fn analyze_block_for_return(&mut self, block: &Block, func_ret_type: &Type) -> Result<bool, SemanticError> {
        let mut guarantees_return = false;

        for stmt in &block.statements {
            if guarantees_return {
                // Code after guaranteed return is unreachable
                self.errors.push(SemanticError::UnreachableCode(format!(
                    "Statement after return is unreachable"
                )));
                continue; // Continue analysis to find more errors, but don't change return status
            }

            if self.analyze_statement_for_return(stmt, func_ret_type)? {
                guarantees_return = true;
            }
        }

        Ok(guarantees_return)
    }

    /// Analyze if a statement guarantees a return
    fn analyze_statement_for_return(&mut self, stmt: &Statement, func_ret_type: &Type) -> Result<bool, SemanticError> {
        match stmt {
            Statement::Return(_) => Ok(true),
            Statement::If(if_stmt) => {
                // If statement guarantees return only if both branches guarantee return
                let then_guarantees = self.analyze_block_for_return(&if_stmt.then_block, func_ret_type)?;

                if let Some(else_block) = &if_stmt.else_block {
                    let else_guarantees = match else_block {
                        Statement::Block(block) => self.analyze_block_for_return(block, func_ret_type)?,
                        Statement::If(nested_if) => self.analyze_statement_for_return(else_block, func_ret_type)?,
                        _ => false,
                    };
                    Ok(then_guarantees && else_guarantees)
                } else {
                    // No else branch means no guaranteed return
                    Ok(false)
                }
            }
            Statement::Block(block) => {
                self.analyze_block_for_return(block, func_ret_type)
            }
            Statement::While(_) | Statement::Loop(_) => {
                // Loops don't guarantee return (could be infinite, but we can't prove it)
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

/// Annotated AST types (with type information)

#[derive(Debug, Clone)]
pub struct AnnotatedProgram {
    pub items: Vec<AnnotatedItem>,
    pub symbol_table: SymbolTable,
}

#[derive(Debug, Clone)]
pub enum AnnotatedItem {
    Function(AnnotatedFunction),
    Struct(AnnotatedStruct),
    Relation(AnnotatedRelation),
    Rule(AnnotatedRule),
}

#[derive(Debug, Clone)]
pub struct AnnotatedFunction {
    pub name: String,
    pub parameters: Vec<AnnotatedParameter>,
    pub return_type: Option<ResolvedType>,
    pub body: AnnotatedBlock,
}

#[derive(Debug, Clone)]
pub struct AnnotatedParameter {
    pub name: String,
    pub param_type: ResolvedType,
}

#[derive(Debug, Clone)]
pub struct AnnotatedStruct {
    pub name: String,
    pub fields: Vec<AnnotatedStructField>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedStructField {
    pub name: String,
    pub field_type: ResolvedType,
}

#[derive(Debug, Clone)]
pub struct AnnotatedRelation {
    pub name: String,
    pub arg_types: Vec<ResolvedType>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedRule {
    pub head: AnnotatedLogicTerm,
    pub body: Vec<AnnotatedLogicTerm>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedLogicTerm {
    pub name: String,
    pub args: Vec<AnnotatedLogicArg>,
    pub relation_type: RelationInfo,
}

#[derive(Debug, Clone)]
pub enum AnnotatedLogicArg {
    Variable { name: String, var_type: ResolvedType },
    Constant { name: String, const_type: ResolvedType },
    StringConstant(String),
    IntConstant(i64),
    FloatConstant(f64),
}

#[derive(Debug, Clone)]
pub struct AnnotatedBlock {
    pub statements: Vec<AnnotatedStatement>,
}

#[derive(Debug, Clone)]
pub enum AnnotatedStatement {
    Let(AnnotatedLetStatement),
    Return(AnnotatedReturnStatement),
    Expression(AnnotatedExpression),
}

#[derive(Debug, Clone)]
pub struct AnnotatedLetStatement {
    pub name: String,
    pub var_type: ResolvedType,
    pub initializer: Option<AnnotatedExpression>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedReturnStatement {
    pub value: Option<AnnotatedExpression>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedExpression {
    pub expr: AnnotatedExpressionKind,
    pub result_type: ResolvedType,
}

#[derive(Debug, Clone)]
pub enum AnnotatedExpressionKind {
    Literal(Literal),
    Identifier(String),
    Binary {
        left: Box<AnnotatedExpression>,
        operator: BinaryOperator,
        right: Box<AnnotatedExpression>,
    },
}

/// Resolved type information
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    // Primitive types
    Int,
    Float,
    Bool,
    String,
    Char,
    Null,

    // User-defined types
    Struct(String),
    Enum(String),

    // Function types
    Function(Vec<ResolvedType>, Box<ResolvedType>),
    Generic(String, Vec<ResolvedType>),

    // Collection types
    Array(Box<ResolvedType>),
    Matrix(Box<ResolvedType>, Vec<usize>),
    Vector(Box<ResolvedType>, usize),
    Set(Box<ResolvedType>),
    Map(Box<ResolvedType>, Box<ResolvedType>),
    Queue(Box<ResolvedType>),
    Stack(Box<ResolvedType>),
    Tree(Box<ResolvedType>),
    Graph(Box<ResolvedType>, Box<ResolvedType>),

    // Union and optional types
    Union(Vec<ResolvedType>),
    Optional(Box<ResolvedType>),
    Result(Box<ResolvedType>, Box<ResolvedType>),

    // Reference types
    Reference(Box<ResolvedType>),
    MutableReference(Box<ResolvedType>),

    // Concurrent types
    Channel(Box<ResolvedType>),
    Mutex(Box<ResolvedType>),
    Atomic(Box<ResolvedType>),

    // AI types
    Tensor(Vec<usize>),
    Dataset(Box<ResolvedType>),
    Model(String),
}

/// Information about a relation
#[derive(Debug, Clone)]
pub struct RelationInfo {
    pub name: String,
    pub arg_types: Vec<ResolvedType>,
}

/// Semantic analysis errors
#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Undefined relation: {0}")]
    UndefinedRelation(String),

    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },

    #[error("Arity mismatch for relation {relation}: expected {expected} arguments, found {found}")]
    ArityMismatch {
        relation: String,
        expected: usize,
        found: usize,
    },

    #[error("Unbound variable in rule head: {0}")]
    UnboundVariable(String),

    #[error("Cannot infer type for variable: {0}")]
    CannotInferType(String),

    #[error("Redefinition of symbol: {0}")]
    Redefinition(String),

    #[error("Invalid binary operation: {0:?} between {1:?} and {2:?}")]
    InvalidBinaryOperation(BinaryOperator, ResolvedType, ResolvedType),

    #[error("Function {0} is missing a return statement")]
    MissingReturn(String),

    #[error("Unreachable code: {0}")]
    UnreachableCode(String),
}
