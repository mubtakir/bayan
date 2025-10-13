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

pub use symbol_table::{SymbolTable, StructFieldInfo, FunctionInfo};
pub use type_checker::TypeChecker;
pub use ownership::{OwnershipAnalyzer, DestroyInfo};

/// Main semantic analyzer
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_checker: TypeChecker,
    ownership_analyzer: OwnershipAnalyzer,
    options: CompilerOptions,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new(options: &CompilerOptions) -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            type_checker: TypeChecker::new(),
            ownership_analyzer: OwnershipAnalyzer::new(),
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
                Item::Trait(trait_decl) => {
                    self.symbol_table.declare_trait(&trait_decl.name, trait_decl)?;  // NEWLY ADDED: Expert recommendation
                }
                Item::Impl(impl_decl) => {
                    self.symbol_table.declare_impl(impl_decl)?;  // NEWLY ADDED: Expert recommendation
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
            Item::Enum(enum_decl) => {
                let annotated_enum = self.analyze_enum(enum_decl)?;
                Ok(AnnotatedItem::Enum(annotated_enum))
            }
            Item::Trait(trait_decl) => {
                let annotated_trait = self.analyze_trait(trait_decl)?;  // NEWLY ADDED: Expert recommendation
                Ok(AnnotatedItem::Trait(annotated_trait))
            }
            Item::Impl(impl_decl) => {
                let annotated_impl = self.analyze_impl(impl_decl)?;  // NEWLY ADDED: Expert recommendation
                Ok(AnnotatedItem::Impl(annotated_impl))
            }
            _ => todo!("Analysis for other item types not yet implemented"),
        }
    }

    /// Analyze a function
    fn analyze_function(&mut self, func: &FunctionDecl) -> Result<AnnotatedFunction, SemanticError> {
        // Enter function scope
        self.symbol_table.enter_scope();
        self.ownership_analyzer.enter_scope();

        // Set current function for borrow checking (Expert recommendation)
        self.ownership_analyzer.set_current_function(Some(func.name.clone()));

        // Add generic parameters to scope as type variables (Expert recommendation: Priority 1)
        let annotated_generics = if let Some(ref generics) = func.generic_params {
            for generic in generics {
                // Add generic type parameter to scope
                // This allows T, U, etc. to be recognized as valid types
                self.symbol_table.declare_generic_param(&generic.name)?;
            }
            Some(self.analyze_generic_params(generics)?)
        } else {
            None
        };

        // Add parameters to scope (Expert recommendation: Priority 2 - &self and &mut self support)
        let mut annotated_params = Vec::new();
        for param in &func.parameters {
            match param {
                Parameter::Regular { name, param_type } => {
                    let resolved_type = self.symbol_table.resolve_type_name(param_type)?;
                    self.symbol_table.declare_variable(name, &resolved_type)?;
                    // Declare in ownership analyzer too (Expert recommendation)
                    self.ownership_analyzer.declare_variable(name, resolved_type.clone(), false)?;
                    annotated_params.push(AnnotatedParameter {
                        name: name.clone(),
                        param_type: resolved_type,
                    });
                }
                Parameter::SelfValue => {
                    // self parameter - type will be determined by the impl context
                    // For now, we'll use a placeholder type
                    let self_type = ResolvedType::Unit; // TODO: Get actual self type from context
                    self.symbol_table.declare_variable("self", &self_type)?;
                    self.ownership_analyzer.declare_variable("self", self_type.clone(), false)?;
                    annotated_params.push(AnnotatedParameter {
                        name: "self".to_string(),
                        param_type: self_type,
                    });
                }
                Parameter::SelfRef => {
                    // &self parameter
                    let self_type = ResolvedType::Unit; // TODO: Get actual self type from context
                    let self_ref_type = ResolvedType::Reference(Box::new(self_type), false);
                    self.symbol_table.declare_variable("self", &self_ref_type)?;
                    self.ownership_analyzer.declare_variable("self", self_ref_type.clone(), false)?;
                    annotated_params.push(AnnotatedParameter {
                        name: "self".to_string(),
                        param_type: self_ref_type,
                    });
                }
                Parameter::SelfMutRef => {
                    // &mut self parameter
                    let self_type = ResolvedType::Unit; // TODO: Get actual self type from context
                    let self_mut_ref_type = ResolvedType::Reference(Box::new(self_type), true);
                    self.symbol_table.declare_variable("self", &self_mut_ref_type)?;
                    self.ownership_analyzer.declare_variable("self", self_mut_ref_type.clone(), false)?;
                    annotated_params.push(AnnotatedParameter {
                        name: "self".to_string(),
                        param_type: self_mut_ref_type,
                    });
                }
            }
        }

        // Analyze function body
        let annotated_body = self.analyze_block(&func.body)?;

        // Check return type consistency
        let return_type = if let Some(ret_type) = &func.return_type {
            Some(self.symbol_table.resolve_type_name(ret_type)?)
        } else {
            None
        };

        // Check return path analysis (as recommended by expert)
        if let Some(ref ret_type) = return_type {
            if !self.analyze_block_for_return(&func.body, ret_type)? {
                return Err(SemanticError::MissingReturn(func.name.clone()));
            }
        }

        // Exit function scope and get variables to destroy (Expert recommendation)
        let _variables_to_destroy = self.ownership_analyzer.exit_scope();
        self.symbol_table.exit_scope();

        // Clear function context (Expert recommendation)
        self.ownership_analyzer.set_current_function(None);

        Ok(AnnotatedFunction {
            name: func.name.clone(),
            generic_params: annotated_generics,
            parameters: annotated_params,
            return_type,
            body: annotated_body,
        })
    }

    /// Analyze a struct
    fn analyze_struct(&mut self, struct_decl: &StructDecl) -> Result<AnnotatedStruct, SemanticError> {
        // Enter struct scope for generic parameters (Expert recommendation: Priority 1)
        self.symbol_table.enter_scope();

        // Add generic parameters to scope (Expert recommendation: Priority 1)
        let annotated_generics = if let Some(ref generics) = struct_decl.generic_params {
            for generic in generics {
                self.symbol_table.declare_generic_param(&generic.name)?;
            }
            Some(self.analyze_generic_params(generics)?)
        } else {
            None
        };

        let mut annotated_fields = Vec::new();
        let mut struct_field_infos = Vec::new();

        for field in &struct_decl.fields {
            let resolved_type = self.symbol_table.resolve_type_name(&field.field_type)?;
            annotated_fields.push(AnnotatedStructField {
                name: field.name.clone(),
                field_type: resolved_type.clone(),
            });
            struct_field_infos.push(StructFieldInfo {
                name: field.name.clone(),
                field_type: resolved_type,
            });
        }

        // Update struct info in symbol table (Expert recommendation: Priority 1)
        self.symbol_table.update_struct_info(&struct_decl.name, struct_field_infos)?;

        // Exit struct scope
        self.symbol_table.exit_scope();

        Ok(AnnotatedStruct {
            name: struct_decl.name.clone(),
            generic_params: annotated_generics,
            fields: annotated_fields,
        })
    }

    /// Analyze an enum declaration (Expert recommendation: Enum support)
    fn analyze_enum(&mut self, enum_decl: &EnumDecl) -> Result<AnnotatedEnum, SemanticError> {
        let mut annotated_variants = Vec::new();

        for variant in &enum_decl.variants {
            let variant_fields = if let Some(field_types) = &variant.fields {
                let mut resolved_fields = Vec::new();
                for field_type in field_types {
                    let resolved_type = self.type_checker.resolve_type(field_type)?;
                    resolved_fields.push(resolved_type);
                }
                Some(resolved_fields)
            } else {
                None
            };

            annotated_variants.push(AnnotatedEnumVariant {
                name: variant.name.clone(),
                fields: variant_fields,
            });
        }

        Ok(AnnotatedEnum {
            name: enum_decl.name.clone(),
            variants: annotated_variants,
        })
    }

    /// Analyze a trait declaration (Expert recommendation: Priority 1)
    fn analyze_trait(&mut self, trait_decl: &TraitDecl) -> Result<AnnotatedTrait, SemanticError> {
        // Enter trait scope for generic parameters (Expert recommendation: Priority 1)
        self.symbol_table.enter_scope();

        // Add generic parameters to scope (Expert recommendation: Priority 1)
        let generic_params = if let Some(generics) = &trait_decl.generic_params {
            for generic in generics {
                self.symbol_table.declare_generic_param(&generic.name)?;
            }
            Some(self.analyze_generic_params(generics)?)
        } else {
            None
        };

        let mut annotated_methods = Vec::new();

        for method in &trait_decl.methods {
            let mut annotated_params = Vec::new();
            for param in &method.parameters {
                match param {
                    Parameter::Regular { name, param_type } => {
                        let resolved_type = self.symbol_table.resolve_type_name(param_type)?;
                        annotated_params.push(AnnotatedParameter {
                            name: name.clone(),
                            param_type: resolved_type,
                        });
                    }
                    Parameter::SelfValue => {
                        // self parameter in trait method
                        let self_type = ResolvedType::Unit; // Will be resolved when trait is implemented
                        annotated_params.push(AnnotatedParameter {
                            name: "self".to_string(),
                            param_type: self_type,
                        });
                    }
                    Parameter::SelfRef => {
                        // &self parameter in trait method
                        let self_type = ResolvedType::Unit; // Will be resolved when trait is implemented
                        let self_ref_type = ResolvedType::Reference(Box::new(self_type), false);
                        annotated_params.push(AnnotatedParameter {
                            name: "self".to_string(),
                            param_type: self_ref_type,
                        });
                    }
                    Parameter::SelfMutRef => {
                        // &mut self parameter in trait method
                        let self_type = ResolvedType::Unit; // Will be resolved when trait is implemented
                        let self_mut_ref_type = ResolvedType::Reference(Box::new(self_type), true);
                        annotated_params.push(AnnotatedParameter {
                            name: "self".to_string(),
                            param_type: self_mut_ref_type,
                        });
                    }
                }
            }

            let return_type = if let Some(ret_type) = &method.return_type {
                Some(self.symbol_table.resolve_type_name(ret_type)?)
            } else {
                None
            };

            let body = if let Some(method_body) = &method.body {
                Some(self.analyze_block(method_body)?)
            } else {
                None
            };

            annotated_methods.push(AnnotatedTraitMethod {
                name: method.name.clone(),
                parameters: annotated_params,
                return_type,
                body,
            });
        }

        // Exit trait scope
        self.symbol_table.exit_scope();

        Ok(AnnotatedTrait {
            name: trait_decl.name.clone(),
            generic_params,
            methods: annotated_methods,
        })
    }

    /// Analyze an impl declaration (Expert recommendation: Priority 1)
    fn analyze_impl(&mut self, impl_decl: &ImplDecl) -> Result<AnnotatedImpl, SemanticError> {
        // Enter impl scope for generic parameters (Expert recommendation: Priority 1)
        self.symbol_table.enter_scope();

        // Add generic parameters to scope (Expert recommendation: Priority 1)
        let generic_params = if let Some(generics) = &impl_decl.generic_params {
            for generic in generics {
                self.symbol_table.declare_generic_param(&generic.name)?;
            }
            Some(self.analyze_generic_params(generics)?)
        } else {
            None
        };

        let mut annotated_methods = Vec::new();

        for method in &impl_decl.methods {
            let annotated_method = self.analyze_function(method)?;
            annotated_methods.push(annotated_method);
        }

        // Exit impl scope
        self.symbol_table.exit_scope();

        Ok(AnnotatedImpl {
            trait_name: impl_decl.trait_name.clone(),
            type_name: impl_decl.type_name.clone(),
            generic_params,
            methods: annotated_methods,
        })
    }

    /// Analyze generic parameters (Expert recommendation: Priority 1)
    fn analyze_generic_params(&mut self, generics: &[GenericParam]) -> Result<Vec<AnnotatedGenericParam>, SemanticError> {
        let mut annotated_generics = Vec::new();

        for generic in generics {
            let mut annotated_bounds = Vec::new();
            for bound in &generic.bounds {
                annotated_bounds.push(AnnotatedTraitBound {
                    trait_name: bound.trait_name.clone(),
                });
            }

            annotated_generics.push(AnnotatedGenericParam {
                name: generic.name.clone(),
                bounds: annotated_bounds,
            });
        }

        Ok(annotated_generics)
    }

    /// Analyze a relation declaration (Expert recommendation: Priority 2 - Logic Core)
    fn analyze_relation(&mut self, relation: &RelationDecl) -> Result<AnnotatedRelation, SemanticError> {
        let mut resolved_arg_types = Vec::new();

        for arg_type in &relation.arg_types {
            let resolved_type = self.type_checker.resolve_type(arg_type)?;
            resolved_arg_types.push(resolved_type);
        }

        // Register relation in symbol table for logic programming (Expert recommendation)
        self.symbol_table.declare_relation(&relation.name, relation)?;

        Ok(AnnotatedRelation {
            name: relation.name.clone(),
            arg_types: resolved_arg_types,
        })
    }

    /// Analyze a rule declaration (Expert recommendation: Priority 2 - Logic Core)
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

        // Validate that all relations in the rule exist (Expert recommendation)
        self.validate_rule_relations(&annotated_head, &annotated_body)?;

        Ok(AnnotatedRule {
            head: annotated_head,
            body: annotated_body,
        })
    }

    /// Validate that all relations in a rule exist (Expert recommendation: Priority 2)
    fn validate_rule_relations(&self, head: &AnnotatedLogicTerm, body: &[AnnotatedLogicTerm]) -> Result<(), SemanticError> {
        // Check head relation
        if self.symbol_table.lookup_relation(&head.name).is_none() {
            return Err(SemanticError::UndefinedRelation(head.name.clone()));
        }

        // Check body relations
        for term in body {
            if self.symbol_table.lookup_relation(&term.name).is_none() {
                return Err(SemanticError::UndefinedRelation(term.name.clone()));
            }
        }

        Ok(())
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
        self.ownership_analyzer.enter_scope();

        let mut annotated_statements = Vec::new();
        for stmt in &block.statements {
            let annotated_stmt = self.analyze_statement(stmt)?;
            annotated_statements.push(annotated_stmt);
        }

        // Exit scope and get variables that need destruction (Expert recommendation: Priority 1)
        let variables_to_destroy = self.ownership_analyzer.exit_scope();
        self.symbol_table.exit_scope();

        Ok(AnnotatedBlock {
            statements: annotated_statements,
            // Store variables that need destruction for IRGenerator (Expert recommendation)
            variables_to_destroy: Some(variables_to_destroy),
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
            Statement::Match(match_stmt) => {
                let annotated_match = self.analyze_match_statement(match_stmt)?;
                Ok(AnnotatedStatement::Match(annotated_match))
            }
            Statement::If(if_stmt) => {
                // Enhanced if statement analysis with control flow (Expert recommendation: Priority 2)
                let condition = self.analyze_expression(&if_stmt.condition)?;

                // Ensure condition is boolean
                if !matches!(condition.result_type, ResolvedType::Bool) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Bool,
                        found: condition.result_type,
                    });
                }

                // Analyze if statement with control flow merging in ownership analyzer
                self.ownership_analyzer.analyze_if_statement_ownership(if_stmt)?;

                // Analyze blocks for semantic correctness
                let then_block = self.analyze_block(&if_stmt.then_block)?;
                let else_block = if let Some(ref else_block) = if_stmt.else_block {
                    Some(self.analyze_block(else_block)?)
                } else {
                    None
                };

                // Create annotated if statement
                let annotated_if = AnnotatedIfStatement {
                    condition,
                    then_block,
                    else_block,
                };
                Ok(AnnotatedStatement::If(annotated_if))
            }
            Statement::While(while_stmt) => {
                // For now, treat while statements as expressions
                let _condition = self.analyze_expression(&while_stmt.condition)?;
                let _body = self.analyze_block(&while_stmt.body)?;

                // Create a dummy expression for the while statement
                let while_expr = AnnotatedExpression {
                    result_type: ResolvedType::Int, // Placeholder
                    expr: AnnotatedExpressionKind::Literal(Literal::Integer(0)),
                };
                Ok(AnnotatedStatement::Expression(while_expr))
            }
            Statement::For(_for_stmt) => {
                // For now, treat for statements as expressions
                // TODO: Implement proper for loop analysis
                let for_expr = AnnotatedExpression {
                    result_type: ResolvedType::Int, // Placeholder
                    expr: AnnotatedExpressionKind::Literal(Literal::Integer(0)),
                };
                Ok(AnnotatedStatement::Expression(for_expr))
            }
            Statement::Block(block) => {
                // Analyze the block and create a dummy expression
                let _analyzed_block = self.analyze_block(block)?;
                let block_expr = AnnotatedExpression {
                    result_type: ResolvedType::Int, // Placeholder
                    expr: AnnotatedExpressionKind::Literal(Literal::Integer(0)),
                };
                Ok(AnnotatedStatement::Expression(block_expr))
            }
            _ => {
                // For any other statement types, create a dummy expression
                let dummy_expr = AnnotatedExpression {
                    result_type: ResolvedType::Int, // Placeholder
                    expr: AnnotatedExpressionKind::Literal(Literal::Integer(0)),
                };
                Ok(AnnotatedStatement::Expression(dummy_expr))
            }
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

        // Declare in ownership analyzer too (Expert recommendation)
        // For now, assume variables are immutable by default
        self.ownership_analyzer.declare_variable(&let_stmt.name, var_type.clone(), false)?;

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

    /// Analyze a return statement (Expert recommendation: Priority 1 - Dangling Reference Prevention)
    fn analyze_return_statement(&mut self, ret_stmt: &ReturnStatement) -> Result<AnnotatedReturnStatement, SemanticError> {
        let value = if let Some(expr) = &ret_stmt.value {
            let annotated_expr = self.analyze_expression(expr)?;

            // Expert recommendation: Check for dangling references
            self.check_return_value_for_dangling_references(&annotated_expr)?;

            Some(annotated_expr)
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

                // Check read access (Expert recommendation)
                self.ownership_analyzer.check_read_access(name)?;

                Ok(AnnotatedExpression {
                    expr: AnnotatedExpressionKind::Identifier(name.clone()),
                    result_type: var_info.var_type.clone(),
                })
            }
            Expression::Binary(bin_expr) => {
                self.analyze_binary_expression(bin_expr)
            }
            Expression::Struct(struct_expr) => {
                self.analyze_struct_literal(struct_expr)
            }
            Expression::Enum(enum_expr) => {
                self.analyze_enum_expression(enum_expr)
            }
            Expression::FieldAccess(field_access) => {
                self.analyze_field_access(field_access)
            }
            Expression::Array(array_expr) => {
                self.analyze_array_literal(array_expr)
            }
            Expression::Index(index_expr) => {
                self.analyze_index_access(index_expr)
            }
            Expression::Match(match_expr) => {
                self.analyze_match_expression(match_expr)
            }
            Expression::Call(call_expr) => {
                self.analyze_call_expression(call_expr)
            }
            Expression::Unary(unary_expr) => {
                self.analyze_unary_expression(unary_expr)
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

    /// Analyze a struct literal expression (as recommended by expert)
    fn analyze_struct_literal(&mut self, struct_expr: &StructExpression) -> Result<AnnotatedExpression, SemanticError> {
        // Look up struct definition in TypeSystem and clone the fields
        let struct_fields = {
            let struct_info = self.symbol_table.lookup_type(&struct_expr.name)
                .ok_or_else(|| SemanticError::UndefinedType(struct_expr.name.clone()))?;

            match &struct_info.kind {
                symbol_table::TypeKind::Struct(fields) => fields.clone(),
                _ => return Err(SemanticError::TypeMismatch {
                    expected: ResolvedType::Struct(struct_expr.name.clone()),
                    found: ResolvedType::String, // placeholder
                }),
            }
        };

        // Check that all required fields are provided
        let mut provided_fields = std::collections::HashMap::new();
        let mut annotated_fields = Vec::new();

        for (field_name, field_expr) in &struct_expr.fields {
            // Check if field exists in struct definition
            let field_info = struct_fields.iter()
                .find(|f| f.name == *field_name)
                .ok_or_else(|| SemanticError::UndefinedField {
                    struct_name: struct_expr.name.clone(),
                    field_name: field_name.clone(),
                })?;

            // Analyze field expression
            let annotated_field_expr = self.analyze_expression(field_expr)?;

            // Check type compatibility
            if !self.type_checker.types_compatible(&field_info.field_type, &annotated_field_expr.result_type) {
                return Err(SemanticError::TypeMismatch {
                    expected: field_info.field_type.clone(),
                    found: annotated_field_expr.result_type,
                });
            }

            provided_fields.insert(field_name.clone(), annotated_field_expr.clone());
            annotated_fields.push((field_name.clone(), annotated_field_expr));
        }

        // Check that all required fields are provided
        for field_info in struct_fields {
            if !provided_fields.contains_key(&field_info.name) {
                return Err(SemanticError::MissingField {
                    struct_name: struct_expr.name.clone(),
                    field_name: field_info.name.clone(),
                });
            }
        }

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::StructLiteral {
                name: struct_expr.name.clone(),
                fields: annotated_fields,
            },
            result_type: ResolvedType::Struct(struct_expr.name.clone()),
        })
    }

    /// Analyze an enum expression (Expert recommendation: Enum support)
    fn analyze_enum_expression(&mut self, enum_expr: &EnumExpression) -> Result<AnnotatedExpression, SemanticError> {
        // Look up enum definition in TypeSystem and clone the variants
        let enum_variants = {
            let enum_info = self.symbol_table.lookup_type(&enum_expr.enum_name)
                .ok_or_else(|| SemanticError::UndefinedType(enum_expr.enum_name.clone()))?;

            match &enum_info.kind {
                symbol_table::TypeKind::Enum(variants) => variants.clone(),
                _ => return Err(SemanticError::TypeMismatch {
                    expected: ResolvedType::Enum(enum_expr.enum_name.clone()),
                    found: ResolvedType::String, // placeholder
                }),
            }
        };

        // Find the variant
        let variant_info = enum_variants.iter()
            .find(|v| v.name == enum_expr.variant_name)
            .ok_or_else(|| SemanticError::UndefinedVariant {
                enum_name: enum_expr.enum_name.clone(),
                variant_name: enum_expr.variant_name.clone(),
            })?;

        // Check variant fields
        let annotated_fields = match (&variant_info.fields, &enum_expr.fields) {
            (None, None) => None, // Unit variant
            (Some(expected_types), Some(provided_exprs)) => {
                if expected_types.len() != provided_exprs.len() {
                    return Err(SemanticError::ArityMismatch {
                        expected: expected_types.len(),
                        found: provided_exprs.len(),
                    });
                }

                let mut annotated_fields = Vec::new();
                for (expected_type, provided_expr) in expected_types.iter().zip(provided_exprs.iter()) {
                    let annotated_expr = self.analyze_expression(provided_expr)?;

                    if !self.type_checker.types_compatible(expected_type, &annotated_expr.result_type) {
                        return Err(SemanticError::TypeMismatch {
                            expected: expected_type.clone(),
                            found: annotated_expr.result_type,
                        });
                    }

                    annotated_fields.push(annotated_expr);
                }
                Some(annotated_fields)
            }
            (None, Some(_)) => {
                return Err(SemanticError::ArityMismatch {
                    expected: 0,
                    found: enum_expr.fields.as_ref().unwrap().len(),
                });
            }
            (Some(expected), None) => {
                return Err(SemanticError::ArityMismatch {
                    expected: expected.len(),
                    found: 0,
                });
            }
        };

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::EnumLiteral {
                enum_name: enum_expr.enum_name.clone(),
                variant_name: enum_expr.variant_name.clone(),
                fields: annotated_fields,
            },
            result_type: ResolvedType::Enum(enum_expr.enum_name.clone()),
        })
    }

    /// Analyze a field access expression (as recommended by expert)
    fn analyze_field_access(&mut self, field_access: &FieldAccessExpression) -> Result<AnnotatedExpression, SemanticError> {
        // Analyze the object expression
        let annotated_object = self.analyze_expression(&field_access.object)?;

        // Get the struct type
        let struct_name = match &annotated_object.result_type {
            ResolvedType::Struct(name) => name,
            _ => return Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Struct("any".to_string()),
                found: annotated_object.result_type,
            }),
        };

        // Look up struct definition
        let struct_info = self.symbol_table.lookup_type(struct_name)
            .ok_or_else(|| SemanticError::UndefinedType(struct_name.clone()))?;

        // Get struct fields
        let struct_fields = match &struct_info.kind {
            symbol_table::TypeKind::Struct(fields) => fields,
            _ => return Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Struct(struct_name.clone()),
                found: ResolvedType::String, // placeholder
            }),
        };

        // Find the field
        let field_info = struct_fields.iter()
            .find(|f| f.name == field_access.field)
            .ok_or_else(|| SemanticError::UndefinedField {
                struct_name: struct_name.clone(),
                field_name: field_access.field.clone(),
            })?;

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::FieldAccess {
                object: Box::new(annotated_object),
                field: field_access.field.clone(),
            },
            result_type: field_info.field_type.clone(),
        })
    }

    /// Analyze an array literal expression (Expert recommendation: List<T> support)
    fn analyze_array_literal(&mut self, array_expr: &ArrayExpression) -> Result<AnnotatedExpression, SemanticError> {
        let mut annotated_elements = Vec::new();
        let mut element_type: Option<ResolvedType> = None;

        // Analyze each element and infer the common type
        for element in &array_expr.elements {
            let annotated_element = self.analyze_expression(element)?;

            // Determine the element type (for now, all elements must be the same type)
            match &element_type {
                None => {
                    element_type = Some(annotated_element.result_type.clone());
                }
                Some(expected_type) => {
                    if !self.type_checker.types_compatible(expected_type, &annotated_element.result_type) {
                        return Err(SemanticError::TypeMismatch {
                            expected: expected_type.clone(),
                            found: annotated_element.result_type.clone(),
                        });
                    }
                }
            }

            annotated_elements.push(annotated_element);
        }

        // Default to int list if empty
        let final_element_type = element_type.unwrap_or(ResolvedType::Int);

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Array {
                elements: annotated_elements,
            },
            result_type: ResolvedType::List(Box::new(final_element_type)),
        })
    }

    /// Analyze an index access expression (Expert recommendation: List<T> support)
    fn analyze_index_access(&mut self, index_expr: &IndexExpression) -> Result<AnnotatedExpression, SemanticError> {
        // Analyze the object being indexed
        let annotated_object = self.analyze_expression(&index_expr.object)?;

        // Analyze the index expression
        let annotated_index = self.analyze_expression(&index_expr.index)?;

        // Ensure index is an integer
        if !matches!(annotated_index.result_type, ResolvedType::Int) {
            return Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Int,
                found: annotated_index.result_type,
            });
        }

        // Determine the result type based on the object type
        let result_type = match &annotated_object.result_type {
            ResolvedType::List(element_type) => (**element_type).clone(),
            ResolvedType::Tuple(element_types) => {
                // For tuple indexing, we'd need to check if the index is a compile-time constant
                // For now, return the first element type or error
                if element_types.is_empty() {
                    return Err(SemanticError::Other("Cannot index empty tuple".to_string()));
                }
                element_types[0].clone() // Simplified - should check constant index
            }
            _ => {
                return Err(SemanticError::Other(format!(
                    "Cannot index type: {:?}", annotated_object.result_type
                )));
            }
        };

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Index {
                object: Box::new(annotated_object),
                index: Box::new(annotated_index),
            },
            result_type,
        })
    }

    /// Analyze a match expression (Expert recommendation: Priority 1 - Complete match support)
    fn analyze_match_expression(&mut self, match_expr: &Box<MatchStatement>) -> Result<AnnotatedExpression, SemanticError> {

        // Reuse match statement analysis
        let annotated_match = self.analyze_match_statement(match_expr)?;

        // Match expression result type
        let result_type = annotated_match.result_type.clone();

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Match {
                expression: Box::new(annotated_match.expression),
                arms: annotated_match.arms,
            },
            result_type,
        })
    }

    /// Analyze a call expression (Expert recommendation: Priority 1 - Method Resolution)
    fn analyze_call_expression(&mut self, call_expr: &CallExpression) -> Result<AnnotatedExpression, SemanticError> {
        match call_expr.callee.as_ref() {
            // Simple function call: function_name(args)
            Expression::Identifier(function_name) => {
                self.analyze_function_call(function_name, &call_expr.arguments)
            }
            // Method call: obj.method(args)
            Expression::FieldAccess(field_access) => {
                self.analyze_method_call(field_access, &call_expr.arguments)
            }
            _ => Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Unit,
                found: ResolvedType::Unit,
            }),
        }
    }

    /// Analyze a simple function call (Expert recommendation: Priority 1)
    fn analyze_function_call(&mut self, function_name: &str, arguments: &[Expression]) -> Result<AnnotatedExpression, SemanticError> {
        // Look up the function in the symbol table
        let func_info = self.symbol_table.lookup_function(function_name)
            .ok_or_else(|| SemanticError::UndefinedVariable(function_name.to_string()))?
            .clone(); // Clone to avoid borrowing issues

        // Check argument count
        if arguments.len() != func_info.parameters.len() {
            return Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Unit, // Placeholder
                found: ResolvedType::Unit,    // Placeholder
            });
        }

        // Analyze arguments and check types
        let mut annotated_args = Vec::new();
        for (i, arg) in arguments.iter().enumerate() {
            let annotated_arg = self.analyze_expression(arg)?;
            let expected_type = &func_info.parameters[i];

            // Simple type compatibility check
            if annotated_arg.result_type != *expected_type {
                return Err(SemanticError::TypeMismatch {
                    expected: expected_type.clone(),
                    found: annotated_arg.result_type.clone(),
                });
            }

            annotated_args.push(annotated_arg);
        }

        let return_type = func_info.return_type.clone().unwrap_or(ResolvedType::Unit);

        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Call {
                function: function_name.to_string(),
                arguments: annotated_args,
            },
            result_type: return_type,
        })
    }

    /// Analyze a method call (Expert recommendation: Priority 2 - &self and &mut self support)
    fn analyze_method_call(&mut self, field_access: &FieldAccessExpression, arguments: &[Expression]) -> Result<AnnotatedExpression, SemanticError> {
        // Analyze the object expression
        let annotated_object = self.analyze_expression(&field_access.object)?;
        let method_name = &field_access.field;

        // Get the object type (clone to avoid borrowing issues)
        let object_type = annotated_object.result_type.clone();

        // Check if we need to create a borrow for method call (Expert recommendation: Priority 2)
        if let Expression::Identifier(var_name) = field_access.object.as_ref() {
            // We'll determine the borrow type based on the method signature
            // For now, we'll check the method and create appropriate borrow
            self.check_method_call_borrowing(var_name, method_name, &object_type)?;
        }

        // Check if this is a trait object method call (Expert recommendation: Priority 1 - Dynamic Dispatch)
        if let ResolvedType::TraitObject(trait_names) = &object_type {
            return self.analyze_trait_object_method_call(trait_names, method_name, arguments, annotated_object);
        }

        // Check if this is a reference to trait object method call (Expert recommendation: Priority 1 - &dyn Trait)
        if let ResolvedType::Reference(inner_type, _is_mutable) = &object_type {
            if let ResolvedType::TraitObject(trait_names) = inner_type.as_ref() {
                return self.analyze_trait_object_method_call(trait_names, method_name, arguments, annotated_object);
            }
        }

        // Try to find the method in impl blocks (Expert recommendation: Priority 1)
        if let Some(method_info) = self.find_method_in_impls(&object_type, method_name) {
            // Check argument count (including self parameter)
            let expected_param_count = method_info.parameters.len();
            let actual_arg_count = arguments.len() + 1; // +1 for self

            if actual_arg_count != expected_param_count {
                return Err(SemanticError::TypeMismatch {
                    expected: ResolvedType::Unit, // Placeholder
                    found: ResolvedType::Unit,    // Placeholder
                });
            }

            // Analyze arguments and check types (skip first parameter which is self)
            let mut annotated_args = Vec::new();
            annotated_args.push(annotated_object); // Add self as first argument

            for (i, arg) in arguments.iter().enumerate() {
                let annotated_arg = self.analyze_expression(arg)?;
                let expected_type = &method_info.parameters[i + 1]; // +1 to skip self parameter

                // Simple type compatibility check
                if annotated_arg.result_type != *expected_type {
                    return Err(SemanticError::TypeMismatch {
                        expected: expected_type.clone(),
                        found: annotated_arg.result_type.clone(),
                    });
                }

                annotated_args.push(annotated_arg);
            }

            let return_type = method_info.return_type.clone().unwrap_or(ResolvedType::Unit);

            Ok(AnnotatedExpression {
                expr: AnnotatedExpressionKind::Call {
                    function: format!("{:?}::{}", object_type, method_name), // Mangled method name
                    arguments: annotated_args,
                },
                result_type: return_type,
            })
        } else {
            Err(SemanticError::UndefinedVariable(format!("Method {} not found for type {:?}", method_name, object_type)))
        }
    }

    /// Find method in impl blocks (Expert recommendation: Priority 1)
    fn find_method_in_impls(&self, object_type: &ResolvedType, method_name: &str) -> Option<FunctionInfo> {
        let type_name = match object_type {
            ResolvedType::Struct(name) => name,
            _ => return None,
        };

        // Look for inherent impl (impl TypeName)
        for impl_info in self.symbol_table.get_impls() {
            if impl_info.type_name == *type_name && impl_info.trait_name.is_none() {
                for method in &impl_info.methods {
                    if method.name == method_name {
                        return Some(method.clone());
                    }
                }
            }
        }

        // Look for trait impl (impl TraitName for TypeName) - Expert recommendation: Priority 1
        for impl_info in self.symbol_table.get_impls() {
            if impl_info.type_name == *type_name && impl_info.trait_name.is_some() {
                for method in &impl_info.methods {
                    if method.name == method_name {
                        return Some(method.clone());
                    }
                }
            }
        }

        None
    }

    /// Analyze trait object method call (Expert recommendation: Priority 1 - Dynamic Dispatch)
    fn analyze_trait_object_method_call(
        &mut self,
        trait_names: &[String],
        method_name: &str,
        arguments: &[Expression],
        annotated_object: AnnotatedExpression
    ) -> Result<AnnotatedExpression, SemanticError> {
        // Find the trait that contains this method
        let mut found_trait = None;
        let mut method_info = None;

        for trait_name in trait_names {
            if let Some(trait_info) = self.symbol_table.lookup_trait(trait_name) {
                for trait_method in &trait_info.methods {
                    if trait_method.name == method_name {
                        found_trait = Some(trait_name.clone());
                        method_info = Some(trait_method.clone());
                        break;
                    }
                }
                if found_trait.is_some() {
                    break;
                }
            }
        }

        let (trait_name, method_info) = match (found_trait, method_info) {
            (Some(trait_name), Some(method_info)) => (trait_name, method_info),
            _ => return Err(SemanticError::UndefinedVariable(
                format!("Method {} not found in trait object", method_name)
            )),
        };

        // Check argument count (trait object method calls don't need explicit self)
        let expected_param_count = method_info.parameters.len();
        let actual_arg_count = arguments.len();

        if actual_arg_count != expected_param_count {
            return Err(SemanticError::TypeMismatch {
                expected: ResolvedType::Unit, // Placeholder
                found: ResolvedType::Unit,    // Placeholder
            });
        }

        // Analyze arguments and check types
        let mut annotated_args = Vec::new();
        annotated_args.push(annotated_object); // Add trait object as first argument

        for (i, arg) in arguments.iter().enumerate() {
            let annotated_arg = self.analyze_expression(arg)?;

            // Only check types if we have parameter info
            if i < method_info.parameters.len() {
                let expected_type = &method_info.parameters[i];

                // Simple type compatibility check
                if annotated_arg.result_type != *expected_type {
                    return Err(SemanticError::TypeMismatch {
                        expected: expected_type.clone(),
                        found: annotated_arg.result_type.clone(),
                    });
                }
            }

            annotated_args.push(annotated_arg);
        }

        let return_type = method_info.return_type.clone().unwrap_or(ResolvedType::Unit);

        // Mark this as a dynamic dispatch call (Expert recommendation: Priority 1)
        Ok(AnnotatedExpression {
            expr: AnnotatedExpressionKind::Call {
                function: format!("dyn_{}::{}", trait_name, method_name), // Dynamic dispatch marker
                arguments: annotated_args,
            },
            result_type: return_type,
        })
    }

    /// Analyze a unary expression (Expert recommendation: &/&mut support)
    fn analyze_unary_expression(&mut self, unary_expr: &UnaryExpression) -> Result<AnnotatedExpression, SemanticError> {
        match &unary_expr.operator {
            UnaryOperator::Reference => {
                // For &var, we need to check that var exists and create an immutable borrow
                if let Expression::Identifier(var_name) = &*unary_expr.operand {
                    // Check that variable exists
                    let var_info = self.symbol_table.lookup_variable(var_name)
                        .ok_or_else(|| SemanticError::UndefinedVariable(var_name.clone()))?;

                    // Add immutable borrow (Expert recommendation)
                    let scope_depth = self.ownership_analyzer.get_scope_depth();
                    self.ownership_analyzer.get_borrow_check_state_mut().add_borrow(
                        var_name,
                        crate::semantic::ownership::BorrowKind::Immutable,
                        scope_depth
                    )?;

                    // Return reference type
                    let ref_type = ResolvedType::Reference(Box::new(var_info.var_type.clone()), false);
                    Ok(AnnotatedExpression {
                        expr: AnnotatedExpressionKind::Unary(AnnotatedUnaryExpression {
                            operator: unary_expr.operator.clone(),
                            operand: Box::new(AnnotatedExpression {
                                expr: AnnotatedExpressionKind::Identifier(var_name.clone()),
                                result_type: var_info.var_type.clone(),
                            }),
                        }),
                        result_type: ref_type,
                    })
                } else {
                    return Err(SemanticError::Other("Reference operator can only be applied to variables".to_string()));
                }
            }
            UnaryOperator::MutableReference => {
                // For &mut var, we need to check that var is mutable and create a mutable borrow
                if let Expression::Identifier(var_name) = &*unary_expr.operand {
                    // Check that variable exists
                    let var_info = self.symbol_table.lookup_variable(var_name)
                        .ok_or_else(|| SemanticError::UndefinedVariable(var_name.clone()))?;

                    // Check that variable is mutable (Expert recommendation)
                    let ownership_info = self.ownership_analyzer.check_variable_use(var_name)?;
                    if !ownership_info.is_mutable {
                        return Err(SemanticError::BorrowMutableFromImmutable(var_name.clone()));
                    }

                    // Add mutable borrow (Expert recommendation)
                    let scope_depth = self.ownership_analyzer.get_scope_depth();
                    self.ownership_analyzer.get_borrow_check_state_mut().add_borrow(
                        var_name,
                        crate::semantic::ownership::BorrowKind::Mutable,
                        scope_depth
                    )?;

                    // Return mutable reference type
                    let ref_type = ResolvedType::Reference(Box::new(var_info.var_type.clone()), true);
                    Ok(AnnotatedExpression {
                        expr: AnnotatedExpressionKind::Unary(AnnotatedUnaryExpression {
                            operator: unary_expr.operator.clone(),
                            operand: Box::new(AnnotatedExpression {
                                expr: AnnotatedExpressionKind::Identifier(var_name.clone()),
                                result_type: var_info.var_type.clone(),
                            }),
                        }),
                        result_type: ref_type,
                    })
                } else {
                    return Err(SemanticError::Other("Mutable reference operator can only be applied to variables".to_string()));
                }
            }
            _ => {
                // Handle other unary operators (Not, Negate, etc.)
                let annotated_operand = self.analyze_expression(&unary_expr.operand)?;
                let result_type = self.type_checker.check_unary_operation(&unary_expr.operator, &annotated_operand.result_type)?;

                Ok(AnnotatedExpression {
                    expr: AnnotatedExpressionKind::Unary(AnnotatedUnaryExpression {
                        operator: unary_expr.operator.clone(),
                        operand: Box::new(annotated_operand),
                    }),
                    result_type,
                })
            }
        }
    }

    /// Analyze a match statement (Expert recommendation: Priority 1 - Complete match support)
    fn analyze_match_statement(&mut self, match_stmt: &MatchStatement) -> Result<AnnotatedMatchStatement, SemanticError> {
        // Analyze the expression being matched
        let annotated_expr = self.analyze_expression(&match_stmt.expression)?;
        let match_type = annotated_expr.result_type.clone();

        let mut annotated_arms = Vec::new();
        let mut arm_types = Vec::new();

        // Analyze each match arm
        for arm in &match_stmt.arms {
            // Enter new scope for pattern variables
            self.symbol_table.enter_scope();

            // Analyze the pattern and bind variables
            let annotated_pattern = self.check_pattern(&arm.pattern, &match_type)?;

            // Analyze guard if present
            let annotated_guard = if let Some(guard) = &arm.guard {
                let guard_expr = self.analyze_expression(guard)?;
                // Guard must be boolean
                if !matches!(guard_expr.result_type, ResolvedType::Bool) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Bool,
                        found: guard_expr.result_type,
                    });
                }
                Some(guard_expr)
            } else {
                None
            };

            // Analyze the arm body
            let annotated_body = self.analyze_block(&arm.body)?;

            // Determine body type (last expression or unit)
            let body_type = if let Some(last_stmt) = annotated_body.statements.last() {
                match last_stmt {
                    AnnotatedStatement::Expression(expr) => expr.result_type.clone(),
                    AnnotatedStatement::Return(ret) => {
                        if let Some(ret_expr) = &ret.value {
                            ret_expr.result_type.clone()
                        } else {
                            ResolvedType::Unit
                        }
                    }
                    _ => ResolvedType::Unit,
                }
            } else {
                ResolvedType::Unit
            };

            arm_types.push(body_type.clone());

            annotated_arms.push(AnnotatedMatchArm {
                pattern: annotated_pattern,
                guard: annotated_guard,
                body: annotated_body,
                body_type,
            });

            // Exit scope
            self.symbol_table.exit_scope();
        }

        // Check arm type compatibility for match expressions (Expert recommendation: Enhanced type checking)
        let result_type = if arm_types.is_empty() {
            // No arms - this should be caught by exhaustiveness checking
            ResolvedType::Unit
        } else if arm_types.len() == 1 {
            // Single arm - use its type
            arm_types[0].clone()
        } else {
            // Multiple arms - find common super type
            let mut common_type = arm_types[0].clone();
            for arm_type in &arm_types[1..] {
                if let Some(super_type) = self.type_checker.common_super_type(&common_type, arm_type) {
                    common_type = super_type;
                } else {
                    // No common type found - report detailed error
                    return Err(SemanticError::TypeMismatch {
                        expected: common_type,
                        found: arm_type.clone(),
                    });
                }
            }
            common_type
        };

        // Check exhaustiveness (Expert recommendation: Enhanced exhaustiveness checking)
        self.check_match_exhaustiveness(&match_type, &annotated_arms)?;

        Ok(AnnotatedMatchStatement {
            expression: annotated_expr,
            arms: annotated_arms,
            result_type,
        })
    }

    /// Analyze return path in a block (improved as recommended by expert)
    /// Returns true if the block guarantees a return on ALL possible execution paths
    fn analyze_block_for_return(&mut self, block: &Block, func_ret_type: &ResolvedType) -> Result<bool, SemanticError> {
        let mut guarantees_return = false;
        let mut found_unreachable = false;

        for (i, stmt) in block.statements.iter().enumerate() {
            if guarantees_return && !found_unreachable {
                // Code after guaranteed return is unreachable
                self.errors.push(SemanticError::UnreachableCode(format!(
                    "Statement at position {} after guaranteed return is unreachable", i
                )));
                found_unreachable = true;
                // Continue analysis to find more errors, but don't change return status
            }

            // Analyze this statement for return guarantee
            let stmt_guarantees = self.analyze_statement_for_return(stmt, func_ret_type)?;

            if stmt_guarantees && !guarantees_return {
                guarantees_return = true;
            }
        }

        Ok(guarantees_return)
    }

    /// Analyze if a statement guarantees a return (improved as recommended by expert)
    /// Returns true only if ALL possible execution paths through this statement end with return
    fn analyze_statement_for_return(&mut self, stmt: &Statement, func_ret_type: &ResolvedType) -> Result<bool, SemanticError> {
        match stmt {
            Statement::Return(_) => {
                // Direct return statement always guarantees return
                Ok(true)
            }
            Statement::If(if_stmt) => {
                // If statement guarantees return ONLY if:
                // 1. Both then and else branches exist
                // 2. Both branches guarantee return
                let then_guarantees = self.analyze_block_for_return(&if_stmt.then_block, func_ret_type)?;

                if let Some(else_block) = &if_stmt.else_block {
                    let else_guarantees = self.analyze_block_for_return(else_block, func_ret_type)?;

                    // Both branches must guarantee return
                    Ok(then_guarantees && else_guarantees)
                } else {
                    // No else branch means some execution paths don't return
                    Ok(false)
                }
            }
            Statement::Block(block) => {
                // Block guarantees return if its contents guarantee return
                self.analyze_block_for_return(block, func_ret_type)
            }
            Statement::While(_) => {
                // While loops don't guarantee return because:
                // 1. The condition might be false initially
                // 2. Even if infinite, we can't prove it statically
                Ok(false)
            }
            Statement::For(_) => {
                // For loops don't guarantee return for similar reasons
                Ok(false)
            }
            Statement::Match(match_stmt) => {
                let annotated_match = self.analyze_match_statement(match_stmt)?;

                // Match guarantees return only if:
                // 1. All patterns are covered (exhaustive) - already checked in analyze_match_statement
                // 2. All arms guarantee return
                let all_arms_return = annotated_match.arms.iter().all(|arm| {
                    self.analyze_annotated_block_for_return(&arm.body, func_ret_type).unwrap_or(false)
                });

                Ok(all_arms_return)
            }
            Statement::Expression(_) |
            Statement::Let(_) => {
                // These statements never guarantee return
                Ok(false)
            }
            _ => {
                // Conservative approach: unknown statements don't guarantee return
                Ok(false)
            }
        }
    }

    /// Analyze an annotated block for return guarantee (helper for match statements)
    fn analyze_annotated_block_for_return(&mut self, block: &AnnotatedBlock, func_ret_type: &ResolvedType) -> Result<bool, SemanticError> {
        let mut guarantees_return = false;

        for stmt in &block.statements {
            match stmt {
                AnnotatedStatement::Return(_) => {
                    guarantees_return = true;
                    break; // Found return, no need to check further
                }
                AnnotatedStatement::Match(match_stmt) => {
                    // Check if this match guarantees return
                    let all_arms_return = match_stmt.arms.iter().all(|arm| {
                        self.analyze_annotated_block_for_return(&arm.body, func_ret_type).unwrap_or(false)
                    });
                    if all_arms_return {
                        guarantees_return = true;
                        break;
                    }
                }
                _ => {
                    // Other statements don't guarantee return
                }
            }
        }

        Ok(guarantees_return)
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
    Enum(AnnotatedEnum),
    Trait(AnnotatedTrait),      // NEWLY ADDED: Expert recommendation
    Impl(AnnotatedImpl),        // NEWLY ADDED: Expert recommendation
    Relation(AnnotatedRelation),
    Rule(AnnotatedRule),
}

#[derive(Debug, Clone)]
pub struct AnnotatedFunction {
    pub name: String,
    pub generic_params: Option<Vec<AnnotatedGenericParam>>,  // Expert recommendation: Priority 1
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
    pub generic_params: Option<Vec<AnnotatedGenericParam>>,  // Expert recommendation: Priority 1
    pub fields: Vec<AnnotatedStructField>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedStructField {
    pub name: String,
    pub field_type: ResolvedType,
}

#[derive(Debug, Clone)]
pub struct AnnotatedEnum {
    pub name: String,
    pub variants: Vec<AnnotatedEnumVariant>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedEnumVariant {
    pub name: String,
    pub fields: Option<Vec<ResolvedType>>,
}

/// Annotated trait declaration (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct AnnotatedTrait {
    pub name: String,
    pub generic_params: Option<Vec<AnnotatedGenericParam>>,
    pub methods: Vec<AnnotatedTraitMethod>,
}

/// Annotated trait method (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct AnnotatedTraitMethod {
    pub name: String,
    pub parameters: Vec<AnnotatedParameter>,
    pub return_type: Option<ResolvedType>,
    pub body: Option<AnnotatedBlock>,  // None for required methods
}

/// Annotated impl declaration (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct AnnotatedImpl {
    pub trait_name: Option<String>,  // None for inherent impl
    pub type_name: String,
    pub generic_params: Option<Vec<AnnotatedGenericParam>>,
    pub methods: Vec<AnnotatedFunction>,
}

/// Annotated generic parameter (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct AnnotatedGenericParam {
    pub name: String,
    pub bounds: Vec<AnnotatedTraitBound>,
}

/// Annotated trait bound (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct AnnotatedTraitBound {
    pub trait_name: String,
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
    /// Variables that need destruction at end of this block (Expert recommendation: Priority 1)
    pub variables_to_destroy: Option<Vec<DestroyInfo>>,
}

#[derive(Debug, Clone)]
pub enum AnnotatedStatement {
    Let(AnnotatedLetStatement),
    Return(AnnotatedReturnStatement),
    Expression(AnnotatedExpression),
    Match(AnnotatedMatchStatement),
    If(AnnotatedIfStatement), // Expert recommendation: Priority 2 - Control flow analysis
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
pub struct AnnotatedMatchStatement {
    pub expression: AnnotatedExpression,
    pub arms: Vec<AnnotatedMatchArm>,
    pub result_type: ResolvedType, // Type of the match expression/statement
}

/// Annotated if statement with control flow analysis (Expert recommendation: Priority 2)
#[derive(Debug, Clone)]
pub struct AnnotatedIfStatement {
    pub condition: AnnotatedExpression,
    pub then_block: AnnotatedBlock,
    pub else_block: Option<AnnotatedBlock>,
}

#[derive(Debug, Clone)]
pub struct AnnotatedMatchArm {
    pub pattern: AnnotatedPattern,
    pub guard: Option<AnnotatedExpression>,
    pub body: AnnotatedBlock,
    pub body_type: ResolvedType,
}

#[derive(Debug, Clone)]
pub enum AnnotatedPattern {
    Wildcard,
    Literal(Literal, ResolvedType),
    Identifier(String, ResolvedType),
    Tuple(Vec<AnnotatedPattern>, ResolvedType),
    Struct(String, Vec<(String, AnnotatedPattern)>, ResolvedType),
    Enum(String, Option<Vec<AnnotatedPattern>>, ResolvedType),
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
    StructLiteral {
        name: String,
        fields: Vec<(String, AnnotatedExpression)>,
    },
    EnumLiteral {
        enum_name: String,
        variant_name: String,
        fields: Option<Vec<AnnotatedExpression>>,
    },
    FieldAccess {
        object: Box<AnnotatedExpression>,
        field: String,
    },
    Array {
        elements: Vec<AnnotatedExpression>,
    },
    Index {
        object: Box<AnnotatedExpression>,
        index: Box<AnnotatedExpression>,
    },
    Match {
        expression: Box<AnnotatedExpression>,
        arms: Vec<AnnotatedMatchArm>,
    },
    Call {
        function: String,
        arguments: Vec<AnnotatedExpression>,
    },
    Unary(AnnotatedUnaryExpression),
}

/// Annotated unary expression (Expert recommendation: &/&mut support)
#[derive(Debug, Clone)]
pub struct AnnotatedUnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<AnnotatedExpression>,
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
    Unit, // Unit type for statements that don't return a value

    // User-defined types
    Struct(String),
    Enum(String),

    // Collection types (Expert recommendation: List<T> support)
    List(Box<ResolvedType>),
    Tuple(Vec<ResolvedType>),

    // Function types
    Function(Vec<ResolvedType>, Box<ResolvedType>),
    Generic(String, Vec<ResolvedType>),

    // Generic type parameters (T, U, K, V, etc.)
    GenericParam(String),

    // Trait object types (dyn Trait) - Expert recommendation: Priority 1
    TraitObject(Vec<String>),

    // Reference types (&T, &mut T) - Expert recommendation: Priority 1
    Reference(Box<ResolvedType>, bool), // bool: true for mutable (&mut), false for immutable (&)

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

    #[error("Undefined type: {0}")]
    UndefinedType(String),

    #[error("Undefined field {field_name} in struct {struct_name}")]
    UndefinedField {
        struct_name: String,
        field_name: String,
    },

    #[error("Missing field {field_name} in struct {struct_name}")]
    MissingField {
        struct_name: String,
        field_name: String,
    },

    #[error("Undefined variant {variant_name} in enum {enum_name}")]
    UndefinedVariant {
        enum_name: String,
        variant_name: String,
    },

    #[error("Type mismatch: expected {expected:?}, found {found:?}")]
    TypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },

    #[error("Arity mismatch: expected {expected} arguments, found {found}")]
    ArityMismatch {
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

    #[error("Other error: {0}")]
    Other(String),

    #[error("Pattern type mismatch: expected {expected:?}, found pattern for {found:?}")]
    PatternTypeMismatch {
        expected: ResolvedType,
        found: ResolvedType,
    },

    #[error("Non-exhaustive match: missing patterns for {missing_patterns:?}")]
    NonExhaustiveMatch {
        missing_patterns: Vec<String>,
    },

    #[error("Variable in fact: {0}")]
    VariableInFact(String),

    #[error("Use after move: {0}")]
    UseAfterMove(String),

    #[error("Conflicting borrow: {0}")]
    ConflictingBorrow(String),

    #[error("Write while borrowed: {0}")]
    WriteWhileBorrowed(String),

    #[error("Borrow mutable from immutable: {0}")]
    BorrowMutableFromImmutable(String),

    #[error("Invalid borrow: {0}")]
    InvalidBorrow(String),

    #[error("Borrow conflict: {0}")]
    BorrowConflict(String),

    #[error("Dangling reference: {message}")]
    DanglingReference {
        variable_name: String,
        message: String,
    },
}

impl SemanticAnalyzer {
    /// Check pattern compatibility with match type (Expert recommendation: Pattern checking)
    fn check_pattern(&mut self, pattern: &Pattern, match_type: &ResolvedType) -> Result<AnnotatedPattern, SemanticError> {
        match pattern {
            Pattern::Wildcard => {
                // Wildcard matches any type
                Ok(AnnotatedPattern::Wildcard)
            }
            Pattern::Literal(literal) => {
                // Check that literal type matches the match type
                let literal_type = self.type_checker.infer_literal_type(literal);
                if !self.type_checker.types_compatible(match_type, &literal_type) {
                    return Err(SemanticError::PatternTypeMismatch {
                        expected: match_type.clone(),
                        found: literal_type,
                    });
                }
                Ok(AnnotatedPattern::Literal(literal.clone(), literal_type))
            }
            Pattern::Identifier(name) => {
                // Bind the identifier to the match type in current scope
                self.symbol_table.declare_variable(name, match_type)?;
                Ok(AnnotatedPattern::Identifier(name.clone(), match_type.clone()))
            }
            Pattern::Tuple(patterns) => {
                // Match type must be a tuple with same arity
                match match_type {
                    ResolvedType::Tuple(element_types) => {
                        if patterns.len() != element_types.len() {
                            return Err(SemanticError::PatternTypeMismatch {
                                expected: match_type.clone(),
                                found: ResolvedType::Tuple(vec![ResolvedType::Unit; patterns.len()]),
                            });
                        }

                        let mut annotated_patterns = Vec::new();
                        for (pattern, element_type) in patterns.iter().zip(element_types.iter()) {
                            let annotated_pattern = self.check_pattern(pattern, element_type)?;
                            annotated_patterns.push(annotated_pattern);
                        }

                        Ok(AnnotatedPattern::Tuple(annotated_patterns, match_type.clone()))
                    }
                    _ => Err(SemanticError::PatternTypeMismatch {
                        expected: match_type.clone(),
                        found: ResolvedType::Tuple(vec![ResolvedType::Unit; patterns.len()]),
                    }),
                }
            }
            Pattern::Struct(struct_name, field_patterns) => {
                // Match type must be the same struct
                match match_type {
                    ResolvedType::Struct(expected_struct_name) => {
                        if struct_name != expected_struct_name {
                            return Err(SemanticError::PatternTypeMismatch {
                                expected: match_type.clone(),
                                found: ResolvedType::Struct(struct_name.clone()),
                            });
                        }

                        // TODO: Check field patterns against struct definition
                        // For now, just return the pattern as-is
                        let annotated_field_patterns = field_patterns.iter()
                            .map(|(field_name, pattern)| {
                                // For simplicity, assume field type is the same as match type
                                // In a real implementation, we'd look up the struct definition
                                let field_pattern = self.check_pattern(pattern, &ResolvedType::Int)?;
                                Ok((field_name.clone(), field_pattern))
                            })
                            .collect::<Result<Vec<_>, SemanticError>>()?;

                        Ok(AnnotatedPattern::Struct(struct_name.clone(), annotated_field_patterns, match_type.clone()))
                    }
                    _ => Err(SemanticError::PatternTypeMismatch {
                        expected: match_type.clone(),
                        found: ResolvedType::Struct(struct_name.clone()),
                    }),
                }
            }
            Pattern::Enum(enum_variant_full, variant_patterns) => {
                // Parse "EnumName::VariantName" format
                let (enum_name, variant_name) = enum_variant_full.split_once("::")
                    .ok_or_else(|| SemanticError::UndefinedType(enum_variant_full.clone()))?;

                // Check if we're matching against an enum type
                match match_type {
                    ResolvedType::Enum(expected_enum_name) => {
                        if enum_name != expected_enum_name {
                            return Err(SemanticError::PatternTypeMismatch {
                                expected: match_type.clone(),
                                found: ResolvedType::Enum(enum_name.to_string()),
                            });
                        }

                        // Look up enum definition
                        let enum_info = self.symbol_table.lookup_type(enum_name)
                            .ok_or_else(|| SemanticError::UndefinedType(enum_name.to_string()))?;

                        let enum_variants = match &enum_info.kind {
                            symbol_table::TypeKind::Enum(variants) => variants,
                            _ => return Err(SemanticError::TypeMismatch {
                                expected: ResolvedType::Enum(enum_name.to_string()),
                                found: ResolvedType::String, // placeholder
                            }),
                        };

                        // Check if variant exists
                        let variant_info = enum_variants.iter()
                            .find(|v| v.name == variant_name)
                            .ok_or_else(|| SemanticError::UndefinedVariant {
                                enum_name: enum_name.to_string(),
                                variant_name: variant_name.to_string(),
                            })?;

                        // Check variant patterns
                        let annotated_variant_patterns = if let Some(patterns) = variant_patterns {
                            let annotated_patterns = patterns.iter()
                                .map(|pattern| self.check_pattern(pattern, &ResolvedType::Int)) // placeholder
                                .collect::<Result<Vec<_>, SemanticError>>()?;
                            Some(annotated_patterns)
                        } else {
                            None
                        };

                        Ok(AnnotatedPattern::Enum(enum_variant_full.clone(), annotated_variant_patterns, match_type.clone()))
                    }
                    _ => Err(SemanticError::PatternTypeMismatch {
                        expected: match_type.clone(),
                        found: ResolvedType::Enum(enum_name.to_string()),
                    }),
                }
            }
        }
    }

    /// Check return value for dangling references (Expert recommendation: Priority 1)
    fn check_return_value_for_dangling_references(&self, return_expr: &AnnotatedExpression) -> Result<(), SemanticError> {
        // Check if the return type is a reference
        if let ResolvedType::Reference(referenced_type, _is_mutable) = &return_expr.result_type {
            // Check if this is a reference to a local variable
            if let AnnotatedExpressionKind::Unary(unary_expr) = &return_expr.expr {
                if matches!(unary_expr.operator, UnaryOperator::Reference | UnaryOperator::MutableReference) {
                    if let AnnotatedExpressionKind::Identifier(var_name) = &unary_expr.operand.expr {
                        // Check if this variable is local to the current function
                        if self.is_local_variable(var_name) {
                            return Err(SemanticError::DanglingReference {
                                variable_name: var_name.clone(),
                                message: format!("Cannot return reference to local variable '{}'", var_name),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Check if a variable is local to the current function (Expert recommendation: Priority 1)
    fn is_local_variable(&self, var_name: &str) -> bool {
        // A variable is local if it was declared in the current function scope
        // We need to check if the variable was declared in a Function or Block scope
        // (not Global scope or as a function parameter)

        if let Some(_var_info) = self.symbol_table.lookup_variable(var_name) {
            // For now, we'll use a simple heuristic:
            // If we're currently analyzing a function and the variable exists,
            // we'll consider it local unless it's a known global or parameter

            // TODO: Improve this by tracking where variables were declared
            // For the initial implementation, we'll be conservative and assume
            // most variables in function context are local
            true
        } else {
            false
        }
    }

    /// Check method call borrowing requirements (Expert recommendation: Priority 2)
    fn check_method_call_borrowing(&mut self, var_name: &str, method_name: &str, object_type: &ResolvedType) -> Result<(), SemanticError> {
        // Find the method in impl blocks to determine self parameter type
        if let Some(method_info) = self.find_method_in_impls(object_type, method_name) {
            // Check the first parameter to see if it's a self parameter
            if !method_info.parameters.is_empty() {
                let first_param = &method_info.parameters[0];

                match first_param {
                    // &self - create immutable borrow
                    ResolvedType::Reference(_, false) => {
                        let scope_depth = self.ownership_analyzer.get_current_scope_depth();
                        self.ownership_analyzer.add_method_borrow(
                            var_name,
                            crate::semantic::ownership::BorrowKind::Immutable,
                            scope_depth
                        )?;
                    }
                    // &mut self - create mutable borrow
                    ResolvedType::Reference(_, true) => {
                        let scope_depth = self.ownership_analyzer.get_current_scope_depth();
                        self.ownership_analyzer.add_method_borrow(
                            var_name,
                            crate::semantic::ownership::BorrowKind::Mutable,
                            scope_depth
                        )?;
                    }
                    // self (by value) - create move
                    _ => {
                        // For self by value, we need to move the variable
                        self.ownership_analyzer.mark_as_moved(var_name)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Check match exhaustiveness (Expert recommendation: Enhanced exhaustiveness checking)
    fn check_match_exhaustiveness(&self, match_type: &ResolvedType, arms: &[AnnotatedMatchArm]) -> Result<(), SemanticError> {
        // Enhanced exhaustiveness checking for simple types as recommended by expert
        match match_type {
            ResolvedType::Bool => {
                // Check if we have patterns for both true and false, or a catch-all pattern
                let mut has_true = false;
                let mut has_false = false;
                let mut has_catch_all = false;

                for arm in arms {
                    match &arm.pattern {
                        AnnotatedPattern::Wildcard => has_catch_all = true,
                        AnnotatedPattern::Identifier(_, _) => has_catch_all = true, // Identifier patterns catch all
                        AnnotatedPattern::Literal(Literal::Boolean(true), _) => has_true = true,
                        AnnotatedPattern::Literal(Literal::Boolean(false), _) => has_false = true,
                        _ => {}
                    }
                }

                if !has_catch_all && (!has_true || !has_false) {
                    let mut missing = Vec::new();
                    if !has_true { missing.push("true".to_string()); }
                    if !has_false { missing.push("false".to_string()); }
                    return Err(SemanticError::NonExhaustiveMatch { missing_patterns: missing });
                }
            }
            ResolvedType::Int => {
                // For integers, we require a catch-all pattern since we can't enumerate all values
                let has_catch_all = arms.iter().any(|arm| {
                    matches!(arm.pattern,
                        AnnotatedPattern::Wildcard |
                        AnnotatedPattern::Identifier(_, _)
                    )
                });
                if !has_catch_all {
                    return Err(SemanticError::NonExhaustiveMatch {
                        missing_patterns: vec!["_ (wildcard) or identifier pattern".to_string()]
                    });
                }
            }
            ResolvedType::Float => {
                // For floats, we require a catch-all pattern
                let has_catch_all = arms.iter().any(|arm| {
                    matches!(arm.pattern,
                        AnnotatedPattern::Wildcard |
                        AnnotatedPattern::Identifier(_, _)
                    )
                });
                if !has_catch_all {
                    return Err(SemanticError::NonExhaustiveMatch {
                        missing_patterns: vec!["_ (wildcard) or identifier pattern".to_string()]
                    });
                }
            }
            ResolvedType::String => {
                // For strings, we require a catch-all pattern
                let has_catch_all = arms.iter().any(|arm| {
                    matches!(arm.pattern,
                        AnnotatedPattern::Wildcard |
                        AnnotatedPattern::Identifier(_, _)
                    )
                });
                if !has_catch_all {
                    return Err(SemanticError::NonExhaustiveMatch {
                        missing_patterns: vec!["_ (wildcard) or identifier pattern".to_string()]
                    });
                }
            }
            ResolvedType::Enum(enum_name) => {
                // Expert recommendation: Exhaustiveness checking for Enum types
                // Get all variants from the enum definition
                let enum_info = self.symbol_table.lookup_type(enum_name)
                    .ok_or_else(|| SemanticError::UndefinedType(enum_name.clone()))?;

                let enum_variants = match &enum_info.kind {
                    symbol_table::TypeKind::Enum(variants) => variants,
                    _ => return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Enum(enum_name.clone()),
                        found: ResolvedType::String, // placeholder
                    }),
                };

                // Track which variants are covered
                let mut covered_variants = std::collections::HashSet::new();
                let mut has_catch_all = false;

                for arm in arms {
                    match &arm.pattern {
                        AnnotatedPattern::Enum(enum_variant_full, _, _) => {
                            // Parse "EnumName::VariantName" format
                            if let Some((pattern_enum_name, variant_name)) = enum_variant_full.split_once("::") {
                                if pattern_enum_name == enum_name {
                                    covered_variants.insert(variant_name.to_string());
                                }
                            }
                        }
                        AnnotatedPattern::Wildcard | AnnotatedPattern::Identifier(_, _) => {
                            has_catch_all = true;
                        }
                        _ => {}
                    }
                }

                // If we have a catch-all pattern, all remaining variants are covered
                if !has_catch_all {
                    let mut missing_variants = Vec::new();
                    for variant in enum_variants {
                        if !covered_variants.contains(&variant.name) {
                            missing_variants.push(format!("{}::{}", enum_name, variant.name));
                        }
                    }

                    if !missing_variants.is_empty() {
                        return Err(SemanticError::NonExhaustiveMatch {
                            missing_patterns: missing_variants
                        });
                    }
                }
            }
            _ => {
                // For other types (List, Struct, etc.), require a catch-all pattern for now
                let has_catch_all = arms.iter().any(|arm| {
                    matches!(arm.pattern,
                        AnnotatedPattern::Wildcard |
                        AnnotatedPattern::Identifier(_, _)
                    )
                });
                if !has_catch_all {
                    return Err(SemanticError::NonExhaustiveMatch {
                        missing_patterns: vec!["_ (wildcard) or identifier pattern".to_string()]
                    });
                }
            }
        }

        Ok(())
    }

    /// Get variables that need destruction at current scope (Expert recommendation)
    pub fn get_variables_to_destroy(&self) -> Vec<&DestroyInfo> {
        self.ownership_analyzer.get_variables_to_destroy()
    }

    /// Mark a variable as moved (Expert recommendation)
    pub fn mark_variable_as_moved(&mut self, name: &str) -> Result<(), SemanticError> {
        self.ownership_analyzer.mark_as_moved(name)
    }

    /// Check read access to a variable (Expert recommendation)
    pub fn check_variable_read_access(&self, name: &str) -> Result<(), SemanticError> {
        self.ownership_analyzer.check_read_access(name)
    }
}
