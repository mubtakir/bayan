//! # Improved IR Generator
//! 
//! This module implements an improved IR generator with proper scope management
//! and complete control flow support as recommended by the expert.

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::{BasicTypeEnum, FunctionType, IntType, FloatType};
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, IntValue, FloatValue};
use inkwell::basic_block::BasicBlock;
use inkwell::IntPredicate, FloatPredicate;
use std::collections::HashMap;
use crate::parser::ast::*;
use crate::semantic::improved_analyzer::{ResolvedType, SymbolResolution};

/// Improved IR Generator with scope management
pub struct ImprovedIRGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub fpm: PassManager<FunctionValue<'ctx>>,
    
    // Scope management for variables
    variable_scopes: Vec<HashMap<String, PointerValue<'ctx>>>,
    
    // Current function being generated
    current_fn: Option<FunctionValue<'ctx>>,
    
    // Analysis results from semantic analyzer
    resolved_types: HashMap<String, ResolvedType>,
    resolved_symbols: HashMap<String, SymbolResolution>,
    
    // Loop context for break/continue
    loop_contexts: Vec<LoopContext<'ctx>>,
}

/// Context for loop generation (break/continue support)
#[derive(Debug, Clone)]
pub struct LoopContext<'ctx> {
    pub continue_bb: BasicBlock<'ctx>,
    pub break_bb: BasicBlock<'ctx>,
}

impl<'ctx> ImprovedIRGenerator<'ctx> {
    /// Create a new improved IR generator
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        
        // Create function pass manager for optimizations
        let fpm = PassManager::create(&module);
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.initialize();
        
        Self {
            context,
            module,
            builder,
            fpm,
            variable_scopes: vec![HashMap::new()], // Global scope
            current_fn: None,
            resolved_types: HashMap::new(),
            resolved_symbols: HashMap::new(),
            loop_contexts: Vec::new(),
        }
    }
    
    /// Set analysis results from semantic analyzer
    pub fn set_analysis_results(
        &mut self,
        resolved_types: HashMap<String, ResolvedType>,
        resolved_symbols: HashMap<String, SymbolResolution>,
    ) {
        self.resolved_types = resolved_types;
        self.resolved_symbols = resolved_symbols;
    }
    
    /// Enter a new variable scope
    pub fn enter_scope(&mut self) {
        self.variable_scopes.push(HashMap::new());
    }
    
    /// Leave the current variable scope
    pub fn leave_scope(&mut self) {
        if self.variable_scopes.len() > 1 {
            self.variable_scopes.pop();
        }
    }
    
    /// Declare a variable in the current scope
    pub fn declare_variable(&mut self, name: &str, alloca: PointerValue<'ctx>) {
        self.variable_scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), alloca);
    }
    
    /// Look up a variable in the scope chain
    pub fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        for scope in self.variable_scopes.iter().rev() {
            if let Some(alloca) = scope.get(name) {
                return Some(*alloca);
            }
        }
        None
    }
    
    /// Check if current block is terminated
    pub fn is_terminated(&self) -> bool {
        self.builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_some()
    }
    
    /// Convert ResolvedType to LLVM BasicTypeEnum
    pub fn resolved_type_to_llvm(&self, resolved_type: &ResolvedType) -> BasicTypeEnum<'ctx> {
        match resolved_type {
            ResolvedType::Integer => self.context.i64_type().into(),
            ResolvedType::Float => self.context.f64_type().into(),
            ResolvedType::Boolean => self.context.bool_type().into(),
            ResolvedType::String => {
                // String as i8* for now (simplified)
                self.context.i8_type().ptr_type(inkwell::AddressSpace::default()).into()
            }
            _ => self.context.i64_type().into(), // Default fallback
        }
    }
    
    /// Generate IR for a function declaration
    pub fn generate_function(&mut self, func: &FunctionDecl) -> Result<FunctionValue<'ctx>, String> {
        // Create function type
        let param_types: Vec<BasicTypeEnum> = func.parameters
            .iter()
            .map(|param| {
                // Get type from resolved types or use default
                let resolved_type = ResolvedType::Integer; // Simplified for now
                self.resolved_type_to_llvm(&resolved_type)
            })
            .collect();
        
        let return_type = if let Some(ret_type) = &func.return_type {
            let resolved_type = ResolvedType::Integer; // Simplified for now
            self.resolved_type_to_llvm(&resolved_type)
        } else {
            self.context.i64_type().into() // Default return type
        };
        
        let fn_type = return_type.fn_type(&param_types, false);
        let function = self.module.add_function(&func.name, fn_type, None);
        
        // Create entry block
        let entry_bb = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_bb);
        
        // Set current function
        self.current_fn = Some(function);
        
        // Enter function scope
        self.enter_scope();
        
        // Allocate parameters
        for (i, param) in func.parameters.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let param_type = self.resolved_type_to_llvm(&ResolvedType::Integer); // Simplified
            let alloca = self.builder.build_alloca(param_type, &param.name);
            self.builder.build_store(alloca, param_value);
            self.declare_variable(&param.name, alloca);
        }
        
        // Generate function body
        self.generate_block(&func.body)?;
        
        // Add default return if not terminated
        if !self.is_terminated() {
            let default_return = self.context.i64_type().const_int(0, false);
            self.builder.build_return(Some(&default_return));
        }
        
        // Leave function scope
        self.leave_scope();
        
        // Run optimizations
        self.fpm.run_on(&function);
        
        Ok(function)
    }
    
    /// Generate IR for a block
    pub fn generate_block(&mut self, block: &Block) -> Result<(), String> {
        self.enter_scope();
        
        for statement in &block.statements {
            self.generate_statement(statement)?;
            
            // Stop if block is terminated
            if self.is_terminated() {
                break;
            }
        }
        
        self.leave_scope();
        Ok(())
    }
    
    /// Generate IR for a statement
    pub fn generate_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Expression(expr) => {
                self.generate_expression(expr)?;
                Ok(())
            }
            Statement::Let { name, type_annotation: _, initializer } => {
                self.generate_let(name, initializer)
            }
            Statement::If(if_stmt) => self.generate_if(if_stmt),
            Statement::While { condition, body } => self.generate_while(condition, body),
            Statement::Return(expr) => self.generate_return(expr),
            Statement::Break => self.generate_break(),
            Statement::Continue => self.generate_continue(),
            _ => Err("Unsupported statement type".to_string()),
        }
    }
    
    /// Generate IR for let statement
    pub fn generate_let(&mut self, name: &str, initializer: &Option<Expression>) -> Result<(), String> {
        let var_type = self.context.i64_type(); // Simplified type inference
        let alloca = self.builder.build_alloca(var_type, name);
        
        if let Some(init_expr) = initializer {
            let init_value = self.generate_expression(init_expr)?;
            self.builder.build_store(alloca, init_value);
        }
        
        self.declare_variable(name, alloca);
        Ok(())
    }
    
    /// Generate IR for if statement (as recommended by expert)
    pub fn generate_if(&mut self, if_stmt: &IfStmt) -> Result<(), String> {
        let function = self.current_fn.expect("Not in a function");
        
        // 1. Generate condition code
        let condition_val = self.generate_expression(&if_stmt.condition)?;
        let bool_condition = condition_val.into_int_value(); // SemanticAnalyzer ensures this is i1 (bool)
        
        // 2. Create basic blocks
        let then_bb = self.context.append_basic_block(function, "then");
        let else_bb = self.context.append_basic_block(function, "else");
        let merge_bb = self.context.append_basic_block(function, "if_cont");
        
        // 3. Build conditional branch
        self.builder.build_conditional_branch(bool_condition, then_bb, else_bb);
        
        // 4. Generate 'then' block code
        self.builder.position_at_end(then_bb);
        self.generate_block(&if_stmt.then_block)?;
        if !self.is_terminated() {
            self.builder.build_unconditional_branch(merge_bb);
        }
        
        // 5. Generate 'else' block code
        self.builder.position_at_end(else_bb);
        if let Some(else_block) = &if_stmt.else_block {
            match else_block {
                Statement::If(nested_if) => self.generate_if(nested_if)?,
                Statement::Block(block) => self.generate_block(block)?,
                _ => return Err("Invalid else clause".to_string()),
            }
        }
        if !self.is_terminated() {
            self.builder.build_unconditional_branch(merge_bb);
        }
        
        // 6. Move to merge block
        self.builder.position_at_end(merge_bb);
        Ok(())
    }
    
    /// Generate IR for while loop
    pub fn generate_while(&mut self, condition: &Expression, body: &Block) -> Result<(), String> {
        let function = self.current_fn.expect("Not in a function");
        
        let loop_bb = self.context.append_basic_block(function, "loop");
        let body_bb = self.context.append_basic_block(function, "loop_body");
        let exit_bb = self.context.append_basic_block(function, "loop_exit");
        
        // Jump to loop condition
        self.builder.build_unconditional_branch(loop_bb);
        
        // Generate loop condition
        self.builder.position_at_end(loop_bb);
        let condition_val = self.generate_expression(condition)?;
        let bool_condition = condition_val.into_int_value();
        self.builder.build_conditional_branch(bool_condition, body_bb, exit_bb);
        
        // Generate loop body with break/continue context
        self.builder.position_at_end(body_bb);
        self.loop_contexts.push(LoopContext {
            continue_bb: loop_bb,
            break_bb: exit_bb,
        });
        
        self.generate_block(body)?;
        
        self.loop_contexts.pop();
        
        if !self.is_terminated() {
            self.builder.build_unconditional_branch(loop_bb);
        }
        
        // Move to exit block
        self.builder.position_at_end(exit_bb);
        Ok(())
    }
    
    /// Generate IR for return statement
    pub fn generate_return(&mut self, expr: &Option<Expression>) -> Result<(), String> {
        if let Some(return_expr) = expr {
            let return_value = self.generate_expression(return_expr)?;
            self.builder.build_return(Some(&return_value));
        } else {
            let default_return = self.context.i64_type().const_int(0, false);
            self.builder.build_return(Some(&default_return));
        }
        Ok(())
    }
    
    /// Generate IR for break statement
    pub fn generate_break(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_contexts.last() {
            self.builder.build_unconditional_branch(loop_ctx.break_bb);
            Ok(())
        } else {
            Err("Break statement outside of loop".to_string())
        }
    }
    
    /// Generate IR for continue statement
    pub fn generate_continue(&mut self) -> Result<(), String> {
        if let Some(loop_ctx) = self.loop_contexts.last() {
            self.builder.build_unconditional_branch(loop_ctx.continue_bb);
            Ok(())
        } else {
            Err("Continue statement outside of loop".to_string())
        }
    }
    
    /// Generate IR for an expression
    pub fn generate_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expression::Literal(literal) => self.generate_literal(literal),
            Expression::Identifier(name) => self.generate_identifier(name),
            Expression::BinaryOp { left, op, right } => {
                self.generate_binary_op(left, op, right)
            }
            Expression::UnaryOp { op, operand } => {
                self.generate_unary_op(op, operand)
            }
            Expression::FunctionCall { name, args } => {
                self.generate_function_call(name, args)
            }
            _ => Err("Unsupported expression type".to_string()),
        }
    }
    
    /// Generate IR for literal
    pub fn generate_literal(&self, literal: &Literal) -> Result<BasicValueEnum<'ctx>, String> {
        match literal {
            Literal::Integer(n) => {
                Ok(self.context.i64_type().const_int(*n as u64, false).into())
            }
            Literal::Float(f) => {
                Ok(self.context.f64_type().const_float(*f).into())
            }
            Literal::Boolean(b) => {
                Ok(self.context.bool_type().const_int(*b as u64, false).into())
            }
            Literal::String(_s) => {
                // Simplified string handling
                Ok(self.context.i64_type().const_int(0, false).into())
            }
        }
    }
    
    /// Generate IR for identifier
    pub fn generate_identifier(&self, name: &str) -> Result<BasicValueEnum<'ctx>, String> {
        if let Some(alloca) = self.lookup_variable(name) {
            Ok(self.builder.build_load(alloca, name))
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
    
    /// Generate IR for binary operation
    pub fn generate_binary_op(
        &mut self,
        left: &Expression,
        op: &str,
        right: &Expression,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let left_val = self.generate_expression(left)?;
        let right_val = self.generate_expression(right)?;
        
        match (left_val, right_val) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = match op {
                    "+" => self.builder.build_int_add(l, r, "add"),
                    "-" => self.builder.build_int_sub(l, r, "sub"),
                    "*" => self.builder.build_int_mul(l, r, "mul"),
                    "/" => self.builder.build_int_signed_div(l, r, "div"),
                    "%" => self.builder.build_int_signed_rem(l, r, "rem"),
                    "==" => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                    "!=" => self.builder.build_int_compare(IntPredicate::NE, l, r, "ne"),
                    "<" => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                    ">" => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                    "<=" => self.builder.build_int_compare(IntPredicate::SLE, l, r, "le"),
                    ">=" => self.builder.build_int_compare(IntPredicate::SGE, l, r, "ge"),
                    _ => return Err(format!("Unsupported binary operator: {}", op)),
                };
                Ok(result.into())
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = match op {
                    "+" => self.builder.build_float_add(l, r, "fadd"),
                    "-" => self.builder.build_float_sub(l, r, "fsub"),
                    "*" => self.builder.build_float_mul(l, r, "fmul"),
                    "/" => self.builder.build_float_div(l, r, "fdiv"),
                    "==" => self.builder.build_float_compare(FloatPredicate::OEQ, l, r, "feq"),
                    "!=" => self.builder.build_float_compare(FloatPredicate::ONE, l, r, "fne"),
                    "<" => self.builder.build_float_compare(FloatPredicate::OLT, l, r, "flt"),
                    ">" => self.builder.build_float_compare(FloatPredicate::OGT, l, r, "fgt"),
                    "<=" => self.builder.build_float_compare(FloatPredicate::OLE, l, r, "fle"),
                    ">=" => self.builder.build_float_compare(FloatPredicate::OGE, l, r, "fge"),
                    _ => return Err(format!("Unsupported binary operator: {}", op)),
                };
                Ok(result.into())
            }
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }
    
    /// Generate IR for unary operation
    pub fn generate_unary_op(&mut self, op: &str, operand: &Expression) -> Result<BasicValueEnum<'ctx>, String> {
        let operand_val = self.generate_expression(operand)?;
        
        match (op, operand_val) {
            ("-", BasicValueEnum::IntValue(val)) => {
                let zero = self.context.i64_type().const_int(0, false);
                Ok(self.builder.build_int_sub(zero, val, "neg").into())
            }
            ("-", BasicValueEnum::FloatValue(val)) => {
                Ok(self.builder.build_float_neg(val, "fneg").into())
            }
            ("!", BasicValueEnum::IntValue(val)) => {
                // Assuming boolean is represented as i1 or i64
                let zero = self.context.bool_type().const_int(0, false);
                Ok(self.builder.build_int_compare(IntPredicate::EQ, val, zero, "not").into())
            }
            _ => Err(format!("Unsupported unary operator: {}", op)),
        }
    }
    
    /// Generate IR for function call
    pub fn generate_function_call(&mut self, name: &str, args: &[Expression]) -> Result<BasicValueEnum<'ctx>, String> {
        // Look up function in module
        if let Some(function) = self.module.get_function(name) {
            let mut arg_values = Vec::new();
            for arg in args {
                arg_values.push(self.generate_expression(arg)?);
            }
            
            let call_result = self.builder.build_call(function, &arg_values, "call");
            if let Some(result) = call_result.try_as_basic_value().left() {
                Ok(result)
            } else {
                // Void function, return dummy value
                Ok(self.context.i64_type().const_int(0, false).into())
            }
        } else {
            Err(format!("Undefined function: {}", name))
        }
    }
    
    /// Get the generated module
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
}
