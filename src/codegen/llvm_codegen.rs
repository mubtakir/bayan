use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, PointerValue, IntValue, FloatValue};
use inkwell::types::{BasicTypeEnum, IntType, FloatType, PointerType};
use inkwell::targets::{Target, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;
use inkwell::{AddressSpace, IntPredicate, FloatPredicate};
use std::collections::HashMap;
use std::path::Path;
use anyhow::{Result, anyhow};

use crate::parser::ast::*;
use crate::semantic::{AnnotatedProgram, AnnotatedExpression, AnnotatedExpressionKind, AnnotatedStatement, ResolvedType};
use crate::CompilerOptions;

/// LLVM Code Generator for AlBayan language
pub struct LLVMCodeGenerator<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    // Symbol tables
    variables: HashMap<String, PointerValue<'ctx>>,
    functions: HashMap<String, FunctionValue<'ctx>>,

    // Improved scope management (as recommended by expert)
    variable_scopes: Vec<HashMap<String, PointerValue<'ctx>>>,

    // Current function context
    current_function: Option<FunctionValue<'ctx>>,

    // Optimization settings
    optimization_level: OptimizationLevel,
    target_machine: Option<TargetMachine>,

    // Type mappings
    type_cache: HashMap<String, BasicTypeEnum<'ctx>>,

    // Struct field mappings (as recommended by expert)
    struct_field_indices: HashMap<String, HashMap<String, u32>>,

    // V-Table mappings (Expert recommendation: Priority 1 - Dynamic Dispatch)
    vtables: HashMap<String, PointerValue<'ctx>>, // trait_type_key -> vtable_global
}

impl<'ctx> LLVMCodeGenerator<'ctx> {
    /// Create a new LLVM code generator
    pub fn new(context: &'ctx Context, module_name: &str, options: &CompilerOptions) -> Result<Self> {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        // Initialize target
        Target::initialize_all(&Default::default());

        let optimization_level = match options.optimization_level {
            crate::codegen::OptimizationLevel::None => OptimizationLevel::None,
            crate::codegen::OptimizationLevel::Basic => OptimizationLevel::Less,
            crate::codegen::OptimizationLevel::Standard => OptimizationLevel::Default,
            crate::codegen::OptimizationLevel::Aggressive => OptimizationLevel::Aggressive,
        };

        Ok(Self {
            context,
            module,
            builder,
            variables: HashMap::new(),
            functions: HashMap::new(),
            variable_scopes: vec![HashMap::new()], // Global scope
            current_function: None,
            optimization_level,
            target_machine: None,
            type_cache: HashMap::new(),
            struct_field_indices: HashMap::new(),
            vtables: HashMap::new(),
        })
    }

    /// Generate LLVM IR for the entire program
    pub fn generate_program(&mut self, program: &AnnotatedProgram) -> Result<()> {
        // Generate global declarations
        for item in &program.items {
            match item {
                AnnotatedItem::Function(func) => {
                    self.declare_function(func)?;
                }
                AnnotatedItem::Struct(struct_def) => {
                    self.declare_struct(struct_def)?;
                }
                _ => {} // Handle other items as needed
            }
        }

        // Generate function bodies
        for item in &program.items {
            if let AnnotatedItem::Function(func) = item {
                self.generate_function(func)?;
            }
        }

        Ok(())
    }

    /// Declare a function signature
    fn declare_function(&mut self, func: &AnnotatedFunction) -> Result<FunctionValue<'ctx>> {
        let return_type = self.get_llvm_type(&func.return_type)?;
        let param_types: Vec<BasicTypeEnum> = func.parameters
            .iter()
            .map(|param| self.get_llvm_type(&param.param_type))
            .collect::<Result<Vec<_>>>()?;

        let fn_type = match return_type {
            Some(ret_type) => ret_type.fn_type(&param_types, false),
            None => self.context.void_type().fn_type(&param_types, false),
        };

        let function = self.module.add_function(&func.name, fn_type, None);
        self.functions.insert(func.name.clone(), function);

        Ok(function)
    }

    /// Generate function body (improved as recommended by expert)
    fn generate_function(&mut self, func: &AnnotatedFunction) -> Result<()> {
        let function = self.functions[&func.name];
        self.current_function = Some(function);

        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        // Enter function scope (improved scope management)
        self.enter_scope();

        // Create allocas for parameters (improved parameter handling)
        for (i, param) in func.parameters.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let param_type = self.get_llvm_type(&param.param_type)?
                .ok_or_else(|| anyhow!("Cannot convert parameter type to LLVM type"))?;

            // Create alloca in entry block for better optimization
            let alloca = self.builder.build_alloca(param_type, &param.name)?;
            self.builder.build_store(alloca, param_value)?;

            // Store in current scope
            self.declare_variable(&param.name, alloca);
        }

        // Generate function body
        self.generate_block(&func.body)?;

        // Add return if missing (improved return handling as recommended by expert)
        if !self.is_terminated() {
            if let Some(ref ret_type) = func.return_type {
                if matches!(ret_type, ResolvedType::Void) {
                    self.builder.build_return(None)?;
                } else {
                    // Non-void function without return should have been caught by semantic analysis
                    // Add unreachable instruction to indicate this path should never be taken
                    self.builder.build_unreachable()?;
                }
            } else {
                // Void function, add implicit return
                self.builder.build_return(None)?;
            }
        }

        // Leave function scope
        self.leave_scope();
        self.current_function = None;
        Ok(())
    }

    /// Enter a new variable scope (as recommended by expert)
    fn enter_scope(&mut self) {
        self.variable_scopes.push(HashMap::new());
    }

    /// Leave the current variable scope (as recommended by expert)
    fn leave_scope(&mut self) {
        if self.variable_scopes.len() > 1 {
            self.variable_scopes.pop();
        }
    }

    /// Declare a variable in the current scope (as recommended by expert)
    fn declare_variable(&mut self, name: &str, alloca: PointerValue<'ctx>) {
        self.variable_scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), alloca);
    }

    /// Look up a variable in the scope chain (as recommended by expert)
    fn lookup_variable(&self, name: &str) -> Option<PointerValue<'ctx>> {
        for scope in self.variable_scopes.iter().rev() {
            if let Some(alloca) = scope.get(name) {
                return Some(*alloca);
            }
        }
        None
    }

    /// Check if current block is terminated (as recommended by expert)
    fn is_terminated(&self) -> bool {
        self.builder
            .get_insert_block()
            .and_then(|bb| bb.get_terminator())
            .is_some()
    }

    /// Generate code for a block with scope management
    fn generate_block(&mut self, block: &AnnotatedBlock) -> Result<()> {
        self.enter_scope();

        for stmt in &block.statements {
            self.generate_statement(stmt)?;

            // Stop if block is terminated
            if self.is_terminated() {
                break;
            }
        }

        // Generate automatic destroy calls using semantic analysis results (Expert recommendation: Priority 1)
        if let Some(ref variables_to_destroy) = block.variables_to_destroy {
            self.generate_destroy_calls_for_variables(variables_to_destroy)?;
        }

        self.leave_scope();
        Ok(())
    }

    /// Declare a struct type (enhanced as recommended by expert)
    fn declare_struct(&mut self, struct_def: &AnnotatedStruct) -> Result<()> {
        let field_types: Vec<BasicTypeEnum> = struct_def.fields
            .iter()
            .map(|field| self.get_llvm_type(&field.field_type))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter_map(|t| t)
            .collect();

        let struct_type = self.context.struct_type(&field_types, false);
        self.type_cache.insert(
            struct_def.name.clone(),
            struct_type.into()
        );

        // Build field index mapping (as recommended by expert)
        let mut field_indices = HashMap::new();
        for (i, field) in struct_def.fields.iter().enumerate() {
            field_indices.insert(field.name.clone(), i as u32);
        }
        self.struct_field_indices.insert(struct_def.name.clone(), field_indices);

        Ok(())
    }

    /// Generate code for a statement
    fn generate_statement(&mut self, stmt: &AnnotatedStatement) -> Result<()> {
        match stmt {
            AnnotatedStatement::Expression(expr) => {
                self.generate_expression(expr)?;
            }
            AnnotatedStatement::VariableDeclaration { name, var_type, initializer } => {
                let alloca = self.create_entry_block_alloca(name, var_type)?;

                if let Some(init_expr) = initializer {
                    let init_value = self.generate_expression(init_expr)?;
                    self.builder.build_store(alloca, init_value)?;
                }

                // Use improved scope management (as recommended by expert)
                self.declare_variable(name, alloca);
            }
            AnnotatedStatement::Assignment { target, value } => {
                let value_result = self.generate_expression(value)?;

                if let AnnotatedExpression::Variable(var_name) = target.as_ref() {
                    // Use improved scope lookup (as recommended by expert)
                    if let Some(alloca) = self.lookup_variable(var_name) {
                        self.builder.build_store(alloca, value_result)?;
                    } else {
                        return Err(anyhow!("Undefined variable: {}", var_name));
                    }
                }
            }
            AnnotatedStatement::Return(expr) => {
                if let Some(expr) = expr {
                    let return_value = self.generate_expression(expr)?;
                    self.builder.build_return(Some(&return_value))?;
                } else {
                    self.builder.build_return(None)?;
                }
            }
            AnnotatedStatement::If { condition, then_branch, else_branch } => {
                self.generate_if_statement(condition, then_branch, else_branch.as_deref())?;
            }
            AnnotatedStatement::While { condition, body } => {
                self.generate_while_loop(condition, body)?;
            }
            AnnotatedStatement::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
            }
            AnnotatedStatement::Match(match_stmt) => {
                self.generate_match_statement(match_stmt)?;
            }
        }

        Ok(())
    }

    /// Generate code for an expression
    fn generate_expression(&mut self, expr: &AnnotatedExpression) -> Result<BasicValueEnum<'ctx>> {
        match &expr.expr {
            AnnotatedExpressionKind::Literal(lit) => self.generate_literal(lit),
            AnnotatedExpressionKind::Identifier(name) => {
                // Use improved scope lookup (as recommended by expert)
                if let Some(alloca) = self.lookup_variable(name) {
                    Ok(self.builder.build_load(
                        alloca.get_type().get_element_type().into(),
                        alloca,
                        &format!("{}_load", name)  // Enhanced naming as recommended by expert
                    )?)
                } else {
                    Err(anyhow!("Undefined variable: {}", name))
                }
            }
            AnnotatedExpressionKind::Binary { left, operator, right } => {
                self.generate_binary_op(left, operator, right)
            }
            AnnotatedExpressionKind::StructLiteral { name, fields } => {
                self.generate_struct_literal(name, fields)
            }
            AnnotatedExpressionKind::FieldAccess { object, field } => {
                self.generate_field_access(object, field)
            }
            AnnotatedExpressionKind::Array { elements } => {
                self.generate_array_literal(elements, &expr.result_type)
            }
            AnnotatedExpressionKind::Index { object, index } => {
                self.generate_index_access(object, index)
            }
            AnnotatedExpressionKind::Match { expression, arms } => {
                self.generate_match_expression(expression, arms, &expr.result_type)
            }
            AnnotatedExpressionKind::Call { function, arguments } => {
                self.generate_function_call(function, arguments)
            }
            _ => Err(anyhow!("Expression type not yet implemented"))
        }
    }

    /// Generate code for a literal value
    fn generate_literal(&self, literal: &Literal) -> Result<BasicValueEnum<'ctx>> {
        match literal {
            Literal::Integer(value) => {
                Ok(self.context.i64_type().const_int(*value as u64, true).into())
            }
            Literal::Float(value) => {
                Ok(self.context.f64_type().const_float(*value).into())
            }
            Literal::String(value) => {
                let string_val = self.context.const_string(value.as_bytes(), true);
                Ok(string_val.into())
            }
            Literal::Boolean(value) => {
                Ok(self.context.bool_type().const_int(*value as u64, false).into())
            }
        }
    }

    /// Generate code for binary operations
    fn generate_binary_op(
        &mut self,
        left: &AnnotatedExpression,
        operator: &BinaryOperator,
        right: &AnnotatedExpression
    ) -> Result<BasicValueEnum<'ctx>> {
        let left_val = self.generate_expression(left)?;
        let right_val = self.generate_expression(right)?;

        match operator {
            BinaryOperator::Add => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_add(
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "addtmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_add(
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "addtmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in addition"))
                }
            }
            BinaryOperator::Subtract => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_sub(
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "subtmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_sub(
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "subtmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in subtraction"))
                }
            }
            BinaryOperator::Multiply => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_mul(
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "multmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_mul(
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "multmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in multiplication"))
                }
            }
            BinaryOperator::Divide => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_signed_div(
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "divtmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_div(
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "divtmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in division"))
                }
            }
            BinaryOperator::Equal => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_compare(
                        IntPredicate::EQ,
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "eqtmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_compare(
                        FloatPredicate::OEQ,
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "eqtmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in equality comparison"))
                }
            }
            BinaryOperator::LessThan => {
                if left_val.is_int_value() && right_val.is_int_value() {
                    Ok(self.builder.build_int_compare(
                        IntPredicate::SLT,
                        left_val.into_int_value(),
                        right_val.into_int_value(),
                        "lttmp"
                    )?.into())
                } else if left_val.is_float_value() && right_val.is_float_value() {
                    Ok(self.builder.build_float_compare(
                        FloatPredicate::OLT,
                        left_val.into_float_value(),
                        right_val.into_float_value(),
                        "lttmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in less than comparison"))
                }
            }
            _ => Err(anyhow!("Binary operator not yet implemented: {:?}", operator))
        }
    }

    /// Generate code for unary operations
    fn generate_unary_op(
        &mut self,
        operator: &UnaryOperator,
        operand: &AnnotatedExpression
    ) -> Result<BasicValueEnum<'ctx>> {
        let operand_val = self.generate_expression(operand)?;

        match operator {
            UnaryOperator::Minus => {
                if operand_val.is_int_value() {
                    Ok(self.builder.build_int_neg(
                        operand_val.into_int_value(),
                        "negtmp"
                    )?.into())
                } else if operand_val.is_float_value() {
                    Ok(self.builder.build_float_neg(
                        operand_val.into_float_value(),
                        "negtmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in negation"))
                }
            }
            UnaryOperator::Not => {
                if operand_val.is_int_value() {
                    Ok(self.builder.build_not(
                        operand_val.into_int_value(),
                        "nottmp"
                    )?.into())
                } else {
                    Err(anyhow!("Type mismatch in logical not"))
                }
            }
        }
    }

    /// Generate code for function calls (improved as recommended by expert)
    fn generate_function_call(
        &mut self,
        name: &str,
        arguments: &[AnnotatedExpression]
    ) -> Result<BasicValueEnum<'ctx>> {
        // Handle built-in functions first
        if name == "print" {
            return self.generate_print_call(arguments);
        }

        // Handle dynamic dispatch calls (Expert recommendation: Priority 1)
        if name.starts_with("dyn_") {
            return self.generate_dynamic_dispatch_call(name, arguments);
        }

        // Look up user-defined function
        if let Some(&function) = self.functions.get(name) {
            let mut args: Vec<BasicValueEnum> = Vec::new();

            // Generate arguments with proper struct handling (as recommended by expert)
            for arg in arguments {
                let arg_value = self.generate_expression(arg)?;

                // If argument is a struct, we need to pass by pointer
                match &arg.result_type {
                    ResolvedType::Struct(_) => {
                        // If arg_value is a struct value, we need to store it and pass pointer
                        if !arg_value.is_pointer_value() {
                            // Create temporary alloca for struct value
                            let struct_name = match &arg.result_type {
                                ResolvedType::Struct(name) => name,
                                _ => unreachable!(),
                            };
                            let struct_type = self.get_llvm_struct_type(struct_name)?;
                            let temp_alloca = self.builder.build_alloca(struct_type, "temp_struct_arg")?;
                            self.builder.build_store(temp_alloca, arg_value)?;
                            args.push(temp_alloca.into());
                        } else {
                            // Already a pointer, pass directly
                            args.push(arg_value);
                        }
                    }
                    _ => {
                        // Non-struct types, pass by value
                        args.push(arg_value);
                    }
                }
            }

            let call_result = self.builder.build_call(function, &args, &format!("{}_call", name))?;

            if let Some(result) = call_result.try_as_basic_value().left() {
                Ok(result)
            } else {
                // Function returns void, return a dummy value for expressions
                Ok(self.context.i64_type().const_int(0, false).into())
            }
        } else {
            Err(anyhow!("Undefined function: {}", name))
        }
    }

    /// Generate print function call (built-in)
    fn generate_print_call(&mut self, arguments: &[AnnotatedExpression]) -> Result<BasicValueEnum<'ctx>> {
        if arguments.len() != 1 {
            return Err(anyhow!("print() expects exactly one argument"));
        }

        let arg_value = self.generate_expression(&arguments[0])?;

        // Call appropriate print function based on type
        match arg_value {
            BasicValueEnum::IntValue(int_val) => {
                if let Some(print_fn) = self.functions.get("albayan_rt_print_int") {
                    self.builder.build_call(*print_fn, &[int_val.into()], "print_int")?;
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                if let Some(print_fn) = self.functions.get("albayan_rt_print_float") {
                    self.builder.build_call(*print_fn, &[float_val.into()], "print_float")?;
                }
            }
            _ => {
                return Err(anyhow!("Unsupported type for print()"));
            }
        }

        // Return dummy value
        Ok(self.context.i64_type().const_int(0, false).into())
    }

    /// Generate if statement
    fn generate_if_statement(
        &mut self,
        condition: &AnnotatedExpression,
        then_branch: &AnnotatedStatement,
        else_branch: Option<&AnnotatedStatement>
    ) -> Result<()> {
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = if condition_val.is_int_value() {
            let int_val = condition_val.into_int_value();
            self.builder.build_int_compare(
                IntPredicate::NE,
                int_val,
                self.context.i64_type().const_zero(),
                "ifcond"
            )?
        } else {
            return Err(anyhow!("Condition must be boolean"));
        };

        let function = self.current_function.unwrap();
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
        let merge_block = self.context.append_basic_block(function, "ifcont");

        self.builder.build_conditional_branch(condition_bool, then_block, else_block)?;

        // Generate then block
        self.builder.position_at_end(then_block);
        self.generate_statement(then_branch)?;
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block)?;
        }

        // Generate else block
        self.builder.position_at_end(else_block);
        if let Some(else_stmt) = else_branch {
            self.generate_statement(else_stmt)?;
        }
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_block)?;
        }

        // Continue with merge block
        self.builder.position_at_end(merge_block);

        Ok(())
    }

    /// Generate while loop
    fn generate_while_loop(
        &mut self,
        condition: &AnnotatedExpression,
        body: &AnnotatedStatement
    ) -> Result<()> {
        let function = self.current_function.unwrap();
        let loop_block = self.context.append_basic_block(function, "loop");
        let body_block = self.context.append_basic_block(function, "loopbody");
        let after_block = self.context.append_basic_block(function, "afterloop");

        self.builder.build_unconditional_branch(loop_block)?;

        // Generate loop condition
        self.builder.position_at_end(loop_block);
        let condition_val = self.generate_expression(condition)?;
        let condition_bool = if condition_val.is_int_value() {
            let int_val = condition_val.into_int_value();
            self.builder.build_int_compare(
                IntPredicate::NE,
                int_val,
                self.context.i64_type().const_zero(),
                "loopcond"
            )?
        } else {
            return Err(anyhow!("Loop condition must be boolean"));
        };

        self.builder.build_conditional_branch(condition_bool, body_block, after_block)?;

        // Generate loop body
        self.builder.position_at_end(body_block);
        self.generate_statement(body)?;
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(loop_block)?;
        }

        // Continue after loop
        self.builder.position_at_end(after_block);

        Ok(())
    }

    /// Generate match statement (Expert recommendation: Priority 1 - Complete match IR generation)
    fn generate_match_statement(&mut self, match_stmt: &AnnotatedMatchStatement) -> Result<()> {
        // Generate the matched expression
        let matched_val = self.generate_expression(&match_stmt.expression)?;
        let function = self.current_function.unwrap();

        // Create basic blocks
        let merge_bb = self.context.append_basic_block(function, "match_cont");
        let otherwise_bb = self.context.append_basic_block(function, "match_otherwise");

        // Create a basic block for each arm
        let arm_blocks: Vec<_> = match_stmt.arms.iter()
            .enumerate()
            .map(|(i, _)| self.context.append_basic_block(function, &format!("match_arm_{}", i)))
            .collect();

        // For simple types (int, bool), use switch instruction
        match &match_stmt.expression.result_type {
            ResolvedType::Int | ResolvedType::Bool => {
                self.generate_match_with_switch(matched_val, match_stmt, &arm_blocks, otherwise_bb, merge_bb)?;
            }
            _ => {
                // For complex types, use if/else chain
                self.generate_match_with_if_else(matched_val, match_stmt, &arm_blocks, otherwise_bb, merge_bb)?;
            }
        }

        // Position at merge block for continuation
        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    /// Generate match using LLVM switch instruction (Expert recommendation: For simple types)
    fn generate_match_with_switch(
        &mut self,
        matched_val: BasicValueEnum<'ctx>,
        match_stmt: &AnnotatedMatchStatement,
        arm_blocks: &[BasicBlock<'ctx>],
        otherwise_bb: BasicBlock<'ctx>,
        merge_bb: BasicBlock<'ctx>
    ) -> Result<()> {
        // Collect switch cases
        let mut switch_cases = Vec::new();
        let mut default_arm_index = None;

        for (i, arm) in match_stmt.arms.iter().enumerate() {
            match &arm.pattern {
                AnnotatedPattern::Literal(literal, _) => {
                    let case_val = match literal {
                        Literal::Integer(n) => self.context.i64_type().const_int(*n as u64, false),
                        Literal::Boolean(b) => self.context.bool_type().const_int(if *b { 1 } else { 0 }, false),
                        _ => continue, // Skip non-integer/bool literals for switch
                    };
                    switch_cases.push((case_val, arm_blocks[i]));
                }
                AnnotatedPattern::Wildcard | AnnotatedPattern::Identifier(_, _) => {
                    default_arm_index = Some(i);
                }
                _ => {
                    // Complex patterns - skip for switch, handle in if/else
                    continue;
                }
            }
        }

        // Build switch instruction
        let switch_val = if matched_val.is_int_value() {
            matched_val.into_int_value()
        } else {
            return Err(anyhow!("Switch requires integer value"));
        };

        let default_bb = if let Some(idx) = default_arm_index {
            arm_blocks[idx]
        } else {
            otherwise_bb
        };

        self.builder.build_switch(switch_val, default_bb, &switch_cases)?;

        // Generate code for each arm
        for (i, arm) in match_stmt.arms.iter().enumerate() {
            self.builder.position_at_end(arm_blocks[i]);

            // TODO: Bind pattern variables
            // TODO: Handle guard conditions

            // Generate arm body
            self.generate_block(&arm.body)?;

            // Branch to merge if not terminated
            if !self.is_terminated() {
                self.builder.build_unconditional_branch(merge_bb)?;
            }
        }

        // Generate otherwise block (for non-exhaustive matches)
        self.builder.position_at_end(otherwise_bb);
        // For now, just branch to merge (should be unreachable if exhaustive)
        self.builder.build_unconditional_branch(merge_bb)?;

        Ok(())
    }

    /// Generate match using if/else chain (Expert recommendation: For complex types)
    fn generate_match_with_if_else(
        &mut self,
        matched_val: BasicValueEnum<'ctx>,
        match_stmt: &AnnotatedMatchStatement,
        arm_blocks: &[BasicBlock<'ctx>],
        otherwise_bb: BasicBlock<'ctx>,
        merge_bb: BasicBlock<'ctx>
    ) -> Result<()> {
        // For complex patterns, generate if/else chain
        let mut current_bb = self.builder.get_insert_block().unwrap();

        for (i, arm) in match_stmt.arms.iter().enumerate() {
            let next_check_bb = if i < match_stmt.arms.len() - 1 {
                self.context.append_basic_block(self.current_function.unwrap(), &format!("match_check_{}", i + 1))
            } else {
                otherwise_bb
            };

            self.builder.position_at_end(current_bb);

            // Generate pattern matching condition
            let matches = self.generate_pattern_match(&matched_val, &arm.pattern)?;

            // Branch based on pattern match
            self.builder.build_conditional_branch(matches, arm_blocks[i], next_check_bb)?;

            // Generate arm body
            self.builder.position_at_end(arm_blocks[i]);
            self.generate_block(&arm.body)?;

            if !self.is_terminated() {
                self.builder.build_unconditional_branch(merge_bb)?;
            }

            current_bb = next_check_bb;
        }

        // Generate otherwise block
        self.builder.position_at_end(otherwise_bb);
        self.builder.build_unconditional_branch(merge_bb)?;

        Ok(())
    }

    /// Generate pattern matching condition (Expert recommendation: Pattern matching logic)
    fn generate_pattern_match(
        &mut self,
        matched_val: &BasicValueEnum<'ctx>,
        pattern: &AnnotatedPattern
    ) -> Result<IntValue<'ctx>> {
        match pattern {
            AnnotatedPattern::Literal(literal, _) => {
                let literal_val = self.generate_literal(literal)?;
                self.generate_equality_check(matched_val, &literal_val)
            }
            AnnotatedPattern::Wildcard => {
                // Wildcard always matches
                Ok(self.context.bool_type().const_int(1, false))
            }
            AnnotatedPattern::Identifier(_, _) => {
                // Identifier pattern always matches (binds the value)
                Ok(self.context.bool_type().const_int(1, false))
            }
            _ => {
                // TODO: Implement other pattern types
                Ok(self.context.bool_type().const_int(0, false))
            }
        }
    }

    /// Generate equality check between two values
    fn generate_equality_check(
        &mut self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>
    ) -> Result<IntValue<'ctx>> {
        match (left, right) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                Ok(self.builder.build_int_compare(IntPredicate::EQ, *l, *r, "eq_cmp")?)
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                Ok(self.builder.build_float_compare(FloatPredicate::OEQ, *l, *r, "eq_cmp")?)
            }
            _ => {
                // TODO: Handle other types
                Ok(self.context.bool_type().const_int(0, false))
            }
        }
    }

    /// Generate match expression (Expert recommendation: Match as expression with PHI nodes)
    fn generate_match_expression(
        &mut self,
        expression: &AnnotatedExpression,
        arms: &[AnnotatedMatchArm],
        result_type: &ResolvedType
    ) -> Result<BasicValueEnum<'ctx>> {
        // Generate the matched expression
        let matched_val = self.generate_expression(expression)?;
        let function = self.current_function.unwrap();

        // Create basic blocks
        let merge_bb = self.context.append_basic_block(function, "match_expr_cont");
        let otherwise_bb = self.context.append_basic_block(function, "match_expr_otherwise");

        // Create a basic block for each arm
        let arm_blocks: Vec<_> = arms.iter()
            .enumerate()
            .map(|(i, _)| self.context.append_basic_block(function, &format!("match_expr_arm_{}", i)))
            .collect();

        // Vector to collect incoming values for PHI node (Expert recommendation)
        let mut incoming_values_for_phi = Vec::new();

        // Generate pattern matching logic (similar to statement version)
        match &expression.result_type {
            ResolvedType::Int | ResolvedType::Bool => {
                self.generate_match_expr_with_switch_phi(matched_val, arms, &arm_blocks, otherwise_bb, merge_bb, &mut incoming_values_for_phi)?;
            }
            _ => {
                self.generate_match_expr_with_if_else_phi(matched_val, arms, &arm_blocks, otherwise_bb, merge_bb, &mut incoming_values_for_phi)?;
            }
        }

        // Position at merge block and create PHI node (Expert recommendation)
        self.builder.position_at_end(merge_bb);

        // Get the expected LLVM type for the match expression
        let expected_llvm_type = self.resolve_llvm_type(result_type)?;

        let phi = self.builder.build_phi(expected_llvm_type, "match_result")?;
        for (value, block) in incoming_values_for_phi {
            phi.add_incoming(&[(&value, block)]);
        }

        // The value of the match expression is the result of the PHI node
        Ok(phi.as_basic_value())
    }

    /// Generate match expression with switch using PHI nodes (Expert recommendation: Switch for simple types with PHI)
    fn generate_match_expr_with_switch_phi(
        &mut self,
        matched_val: BasicValueEnum<'ctx>,
        arms: &[AnnotatedMatchArm],
        arm_blocks: &[BasicBlock<'ctx>],
        otherwise_bb: BasicBlock<'ctx>,
        merge_bb: BasicBlock<'ctx>,
        incoming_values_for_phi: &mut Vec<(BasicValueEnum<'ctx>, BasicBlock<'ctx>)>
    ) -> Result<()> {
        // Similar to statement version but collect results for PHI
        let mut switch_cases = Vec::new();
        let mut default_arm_index = None;

        for (i, arm) in arms.iter().enumerate() {
            match &arm.pattern {
                AnnotatedPattern::Literal(literal, _) => {
                    let case_val = match literal {
                        Literal::Integer(n) => self.context.i64_type().const_int(*n as u64, false),
                        Literal::Boolean(b) => self.context.bool_type().const_int(if *b { 1 } else { 0 }, false),
                        _ => continue,
                    };
                    switch_cases.push((case_val, arm_blocks[i]));
                }
                AnnotatedPattern::Wildcard | AnnotatedPattern::Identifier(_, _) => {
                    default_arm_index = Some(i);
                }
                _ => continue,
            }
        }

        let switch_val = matched_val.into_int_value();
        let default_bb = if let Some(idx) = default_arm_index {
            arm_blocks[idx]
        } else {
            otherwise_bb
        };

        self.builder.build_switch(switch_val, default_bb, &switch_cases)?;

        // Generate code for each arm and collect results for PHI (Expert recommendation)
        for (i, arm) in arms.iter().enumerate() {
            self.builder.position_at_end(arm_blocks[i]);

            // Generate arm body and get result
            let arm_result = self.generate_block_expression(&arm.body)?;
            let last_block_of_arm = self.builder.get_insert_block().unwrap();

            // Collect value and block for PHI node (Expert recommendation)
            incoming_values_for_phi.push((arm_result, last_block_of_arm));

            if !self.is_terminated() {
                self.builder.build_unconditional_branch(merge_bb)?;
            }
        }

        // Generate otherwise block (for non-exhaustive matches)
        self.builder.position_at_end(otherwise_bb);
        self.builder.build_unconditional_branch(merge_bb)?;

        Ok(())
    }

    /// Generate match expression with if/else using PHI nodes (Expert recommendation: If/else for complex types with PHI)
    fn generate_match_expr_with_if_else_phi(
        &mut self,
        matched_val: BasicValueEnum<'ctx>,
        arms: &[AnnotatedMatchArm],
        arm_blocks: &[BasicBlock<'ctx>],
        otherwise_bb: BasicBlock<'ctx>,
        merge_bb: BasicBlock<'ctx>,
        incoming_values_for_phi: &mut Vec<(BasicValueEnum<'ctx>, BasicBlock<'ctx>)>
    ) -> Result<()> {
        let mut current_bb = self.builder.get_insert_block().unwrap();

        for (i, arm) in arms.iter().enumerate() {
            let next_check_bb = if i < arms.len() - 1 {
                self.context.append_basic_block(self.current_function.unwrap(), &format!("match_expr_check_{}", i + 1))
            } else {
                otherwise_bb
            };

            self.builder.position_at_end(current_bb);
            let matches = self.generate_pattern_match(&matched_val, &arm.pattern)?;
            self.builder.build_conditional_branch(matches, arm_blocks[i], next_check_bb)?;

            self.builder.position_at_end(arm_blocks[i]);
            let arm_result = self.generate_block_expression(&arm.body)?;
            let last_block_of_arm = self.builder.get_insert_block().unwrap();

            // Collect value and block for PHI node (Expert recommendation)
            incoming_values_for_phi.push((arm_result, last_block_of_arm));

            if !self.is_terminated() {
                self.builder.build_unconditional_branch(merge_bb)?;
            }

            current_bb = next_check_bb;
        }

        // Generate otherwise block (for non-exhaustive matches)
        self.builder.position_at_end(otherwise_bb);
        self.builder.build_unconditional_branch(merge_bb)?;

        Ok(())
    }

    /// Create a default value for a given type (Expert recommendation: For non-exhaustive matches)
    fn create_default_value_for_type(&self, result_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        let llvm_type = self.resolve_llvm_type(result_type)?;
        let default_value = match llvm_type {
            BasicTypeEnum::IntType(int_type) => int_type.const_zero().into(),
            BasicTypeEnum::FloatType(float_type) => float_type.const_zero().into(),
            BasicTypeEnum::PointerType(ptr_type) => ptr_type.const_null().into(),
            BasicTypeEnum::StructType(struct_type) => struct_type.const_zero().into(),
            _ => self.context.i64_type().const_zero().into(),
        };
        Ok(default_value)
    }

    /// Generate block as expression (returns the last expression value)
    fn generate_block_expression(&mut self, block: &AnnotatedBlock) -> Result<BasicValueEnum<'ctx>> {
        self.enter_scope();

        let mut last_value = None;
        for stmt in &block.statements {
            match stmt {
                AnnotatedStatement::Expression(expr) => {
                    last_value = Some(self.generate_expression(expr)?);
                }
                _ => {
                    self.generate_statement(stmt)?;
                }
            }

            if self.is_terminated() {
                break;
            }
        }

        // Generate automatic destroy calls using semantic analysis results (Expert recommendation: Priority 1)
        if let Some(ref variables_to_destroy) = block.variables_to_destroy {
            self.generate_destroy_calls_for_variables(variables_to_destroy)?;
        }

        self.leave_scope();

        // Return the last expression value or unit
        if let Some(value) = last_value {
            Ok(value)
        } else {
            // Return unit value (empty struct)
            Ok(self.context.struct_type(&[], false).const_zero().into())
        }
    }

    /// Generate struct literal (as recommended by expert)
    fn generate_struct_literal(
        &mut self,
        struct_name: &str,
        fields: &[(String, AnnotatedExpression)]
    ) -> Result<BasicValueEnum<'ctx>> {
        // Get LLVM struct type
        let struct_type = self.get_llvm_struct_type(struct_name)?;

        // Allocate memory for the struct
        let struct_alloca = self.builder.build_alloca(struct_type, "struct_tmp")?;

        // Initialize each field using build_struct_gep and build_store
        for (i, (field_name, field_expr)) in fields.iter().enumerate() {
            // Generate value for the field
            let field_value = self.generate_expression(field_expr)?;

            // Get pointer to the field using GEP
            let field_ptr = self.builder.build_struct_gep(
                struct_type,
                struct_alloca,
                i as u32,
                &format!("{}_field", field_name)
            )?;

            // Store the value in the field
            self.builder.build_store(field_ptr, field_value)?;
        }

        // Load the complete struct
        Ok(self.builder.build_load(
            struct_type.into(),
            struct_alloca,
            &format!("{}_struct_value", struct_name)  // Enhanced naming as recommended by expert
        )?)
    }

    /// Generate field access (as recommended by expert)
    fn generate_field_access(
        &mut self,
        object: &AnnotatedExpression,
        field_name: &str
    ) -> Result<BasicValueEnum<'ctx>> {
        // Generate the object expression
        let object_value = self.generate_expression(object)?;

        // Get struct type from the object's result type
        let struct_name = match &object.result_type {
            ResolvedType::Struct(name) => name,
            _ => return Err(anyhow!("Field access on non-struct type")),
        };

        // Get LLVM struct type
        let struct_type = self.get_llvm_struct_type(struct_name)?;

        // For now, we need to know the field index
        // This should be looked up from the struct definition
        let field_index = self.get_field_index(struct_name, field_name)?;

        // If object_value is a struct value, we need to extract the field
        // For simplicity, let's assume we have a pointer to the struct
        if object_value.is_pointer_value() {
            let struct_ptr = object_value.into_pointer_value();

            // Get pointer to the field using GEP
            let field_ptr = self.builder.build_struct_gep(
                struct_type,
                struct_ptr,
                field_index,
                &format!("{}_field_access", field_name)
            )?;

            // Load the field value
            Ok(self.builder.build_load(
                field_ptr.get_type().get_element_type().into(),
                field_ptr,
                &format!("{}_field_load", field_name)  // Enhanced naming as recommended by expert
            )?)
        } else {
            // If we have a struct value, we need to extract the field differently
            // This is more complex and would require storing the struct temporarily
            let temp_alloca = self.builder.build_alloca(struct_type, "temp_struct")?;
            self.builder.build_store(temp_alloca, object_value)?;

            let field_ptr = self.builder.build_struct_gep(
                struct_type,
                temp_alloca,
                field_index,
                &format!("{}_field_access", field_name)
            )?;

            Ok(self.builder.build_load(
                field_ptr.get_type().get_element_type().into(),
                field_ptr,
                &format!("{}_temp_field_load", field_name)  // Enhanced naming as recommended by expert
            )?)
        }
    }

    /// Create an alloca instruction in the entry block
    fn create_entry_block_alloca(
        &self,
        name: &str,
        var_type: &Type
    ) -> Result<PointerValue<'ctx>> {
        let builder = self.context.create_builder();

        let entry_block = self.current_function
            .unwrap()
            .get_first_basic_block()
            .unwrap();

        match entry_block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry_block),
        }

        let llvm_type = self.get_llvm_type(var_type)?
            .ok_or_else(|| anyhow!("Cannot create alloca for void type"))?;

        Ok(builder.build_alloca(llvm_type, name)?)
    }

    /// Get LLVM type from AlBayan resolved type (improved for struct support)
    fn get_llvm_type(&self, resolved_type: &ResolvedType) -> Result<Option<BasicTypeEnum<'ctx>>> {
        match resolved_type {
            ResolvedType::Int => Ok(Some(self.context.i64_type().into())),
            ResolvedType::Float => Ok(Some(self.context.f64_type().into())),
            ResolvedType::Bool => Ok(Some(self.context.bool_type().into())),
            ResolvedType::String => Ok(Some(self.context.i8_type().ptr_type(AddressSpace::default()).into())),
            ResolvedType::Struct(name) => {
                // For struct parameters, pass by pointer (as recommended by expert)
                let struct_type = self.get_llvm_struct_type(name)?;
                Ok(Some(struct_type.ptr_type(AddressSpace::default()).into()))
            }
            ResolvedType::TraitObject(_traits) => {
                // Trait object as fat pointer (Expert recommendation: Priority 1)
                let data_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let vtable_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());

                let fat_pointer_type = self.context.struct_type(
                    &[data_ptr_type.into(), vtable_ptr_type.into()],
                    false
                );
                Ok(Some(fat_pointer_type.into()))
            }
            ResolvedType::Reference(referenced_type, _is_mutable) => {
                // Reference types (&T, &mut T) - Expert recommendation: Priority 1
                // References are represented as pointers in LLVM
                if let Some(referenced_llvm_type) = self.get_llvm_type(referenced_type)? {
                    Ok(Some(referenced_llvm_type.ptr_type(AddressSpace::default()).into()))
                } else {
                    Ok(None)
                }
            }
            _ => Err(anyhow!("Type not yet supported: {:?}", resolved_type))
        }
    }

    /// Get LLVM type from AST type (legacy support)
    fn get_llvm_type_from_ast(&self, ast_type: &Type) -> Result<Option<BasicTypeEnum<'ctx>>> {
        match ast_type {
            Type::Named(path) => {
                match path.to_string().as_str() {
                    "int" => Ok(Some(self.context.i64_type().into())),
                    "float" => Ok(Some(self.context.f64_type().into())),
                    "bool" => Ok(Some(self.context.bool_type().into())),
                    "string" => Ok(Some(self.context.i8_type().ptr_type(AddressSpace::default()).into())),
                    name => {
                        // Custom struct type - pass by pointer
                        let struct_type = self.get_llvm_struct_type(name)?;
                        Ok(Some(struct_type.ptr_type(AddressSpace::default()).into()))
                    }
                }
            }
            _ => Err(anyhow!("AST type not yet supported: {:?}", ast_type))
        }
    }

    /// Get default value for a resolved type
    fn get_default_value(&self, resolved_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        match resolved_type {
            ResolvedType::Int => Ok(self.context.i64_type().const_zero().into()),
            ResolvedType::Float => Ok(self.context.f64_type().const_zero().into()),
            ResolvedType::Bool => Ok(self.context.bool_type().const_zero().into()),
            ResolvedType::Struct(name) => {
                // Return null pointer for struct types
                let struct_type = self.get_llvm_struct_type(name)?;
                let ptr_type = struct_type.ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            }
            ResolvedType::String => {
                let empty_str = self.context.const_string(b"", true);
                Ok(empty_str.into())
            }
            ResolvedType::TraitObject(_traits) => {
                // Default trait object is null fat pointer (Expert recommendation: Priority 1)
                let data_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let vtable_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());

                let fat_pointer_type = self.context.struct_type(
                    &[data_ptr_type.into(), vtable_ptr_type.into()],
                    false
                );

                // Create null fat pointer
                let null_data = data_ptr_type.const_null();
                let null_vtable = vtable_ptr_type.const_null();
                let null_fat_pointer = fat_pointer_type.const_named_struct(&[null_data.into(), null_vtable.into()]);

                Ok(null_fat_pointer.into())
            }
            ResolvedType::Reference(referenced_type, _is_mutable) => {
                // Default reference is null pointer (Expert recommendation: Priority 1)
                let referenced_llvm_type = self.resolve_llvm_type(referenced_type)?;
                let ptr_type = referenced_llvm_type.ptr_type(AddressSpace::default());
                Ok(ptr_type.const_null().into())
            }
            _ => Err(anyhow!("Cannot create default value for type: {:?}", resolved_type))
        }
    }

    /// Optimize the generated module
    pub fn optimize(&mut self) -> Result<()> {
        use inkwell::passes::PassManager;

        let pass_manager = PassManager::create(());

        match self.optimization_level {
            OptimizationLevel::None => {
                // No optimizations
            }
            OptimizationLevel::Less => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
            }
            OptimizationLevel::Default => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_gvn_pass();
                pass_manager.add_cfg_simplification_pass();
            }
            OptimizationLevel::Aggressive => {
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
                pass_manager.add_gvn_pass();
                pass_manager.add_cfg_simplification_pass();
                pass_manager.add_basic_alias_analysis_pass();
                pass_manager.add_promote_memory_to_register_pass();
                pass_manager.add_instruction_combining_pass();
                pass_manager.add_reassociate_pass();
            }
        }

        pass_manager.run_on(&self.module);
        Ok(())
    }

    /// Generate object file
    pub fn generate_object_file(&self, output_path: &Path) -> Result<()> {
        if let Some(target_machine) = &self.target_machine {
            target_machine.write_to_file(&self.module, FileType::Object, output_path)
                .map_err(|e| anyhow!("Failed to write object file: {}", e))?;
        } else {
            return Err(anyhow!("Target machine not initialized"));
        }
        Ok(())
    }

    /// Get the generated LLVM IR as string
    pub fn get_ir(&self) -> String {
        self.module.to_string()
    }

    /// Get LLVM struct type by name (as recommended by expert)
    fn get_llvm_struct_type(&self, struct_name: &str) -> Result<inkwell::types::StructType<'ctx>> {
        // Use the type cache that was populated during declare_struct
        if let Some(cached_type) = self.type_cache.get(struct_name) {
            if let BasicTypeEnum::StructType(struct_type) = cached_type {
                return Ok(*struct_type);
            }
        }

        // Fallback: create a generic struct type if not found in cache
        // This should not happen in normal operation
        eprintln!("Warning: Struct type '{}' not found in cache, creating fallback", struct_name);
        let i64_type = self.context.i64_type();
        Ok(self.context.struct_type(&[i64_type.into()], false))
    }

    /// Get field index in struct (enhanced as recommended by expert)
    fn get_field_index(&self, struct_name: &str, field_name: &str) -> Result<u32> {
        // Use the field index mapping built during declare_struct
        if let Some(field_indices) = self.struct_field_indices.get(struct_name) {
            if let Some(&index) = field_indices.get(field_name) {
                return Ok(index);
            }
        }

        // Error if field not found
        Err(format!("Field '{}' not found in struct '{}'", field_name, struct_name).into())
    }

    /// Verify the generated module
    pub fn verify(&self) -> Result<()> {
        if let Err(errors) = self.module.verify() {
            Err(anyhow!("LLVM module verification failed: {}", errors))
        } else {
            Ok(())
        }
    }

    // =============================================================================
    // AlbayanValue Integration Functions (Expert Recommendation)
    // =============================================================================

    /// Get the LLVM type for AlbayanValue struct
    /// This creates the struct type that matches the C representation
    fn get_albayan_value_llvm_type(&self) -> BasicTypeEnum<'ctx> {
        // AlbayanValue struct layout:
        // struct AlbayanValue {
        //     tag: i32,           // AlbayanValueTag (4 bytes)
        //     payload: union {    // 8 bytes (largest member)
        //         int_val: i64,
        //         float_val: f64,
        //         bool_val: bool,
        //         ptr_val: *mut T,
        //     }
        // }

        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();

        // Create the struct type with tag and payload
        let struct_type = self.context.struct_type(&[
            i32_type.into(),  // tag
            i64_type.into(),  // payload (using i64 as largest union member)
        ], false);

        struct_type.into()
    }

    /// Build an AlbayanValue from an LLVM value
    /// This function takes an LLVM value and wraps it in an AlbayanValue struct
    fn build_albayan_value_from_llvm(&self, value: BasicValueEnum<'ctx>, resolved_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        let albayan_value_type = self.get_albayan_value_llvm_type();
        let struct_type = albayan_value_type.into_struct_type();

        // Allocate space for the AlbayanValue
        let alloca = self.builder.build_alloca(struct_type, "albayan_value")?;

        // Determine the tag based on the resolved type
        let tag_value = match resolved_type {
            ResolvedType::Int => self.context.i32_type().const_int(0, false), // AlbayanValueTag::Int
            ResolvedType::Float => self.context.i32_type().const_int(1, false), // AlbayanValueTag::Float
            ResolvedType::Bool => self.context.i32_type().const_int(2, false), // AlbayanValueTag::Bool
            ResolvedType::String => self.context.i32_type().const_int(3, false), // AlbayanValueTag::String
            ResolvedType::List(_) => self.context.i32_type().const_int(4, false), // AlbayanValueTag::List
            ResolvedType::Struct(_) => self.context.i32_type().const_int(5, false), // AlbayanValueTag::Struct
            ResolvedType::Tuple(_) => self.context.i32_type().const_int(6, false), // AlbayanValueTag::Tuple
            _ => self.context.i32_type().const_int(7, false), // AlbayanValueTag::Null
        };

        // Store the tag
        let tag_ptr = self.builder.build_struct_gep(struct_type, alloca, 0, "tag_ptr")?;
        self.builder.build_store(tag_ptr, tag_value)?;

        // Convert and store the payload
        let payload_value = match resolved_type {
            ResolvedType::Int => {
                // Value should already be i64 or convertible
                if value.is_int_value() {
                    let int_val = value.into_int_value();
                    if int_val.get_type() == self.context.i64_type() {
                        int_val
                    } else {
                        // Extend to i64
                        self.builder.build_int_s_extend(int_val, self.context.i64_type(), "extend_to_i64")?
                    }
                } else {
                    return Err(anyhow!("Expected int value for Int type"));
                }
            },
            ResolvedType::Float => {
                // Convert float to i64 bits
                if value.is_float_value() {
                    let float_val = value.into_float_value();
                    self.builder.build_bitcast(float_val, self.context.i64_type(), "float_to_bits")?
                        .into_int_value()
                } else {
                    return Err(anyhow!("Expected float value for Float type"));
                }
            },
            ResolvedType::Bool => {
                // Convert bool to i64
                if value.is_int_value() {
                    let bool_val = value.into_int_value();
                    self.builder.build_int_z_extend(bool_val, self.context.i64_type(), "bool_to_i64")?
                } else {
                    return Err(anyhow!("Expected bool value for Bool type"));
                }
            },
            _ => {
                // For pointers (String, List, Struct, Tuple), cast to i64
                if value.is_pointer_value() {
                    let ptr_val = value.into_pointer_value();
                    self.builder.build_ptr_to_int(ptr_val, self.context.i64_type(), "ptr_to_int")?
                } else {
                    return Err(anyhow!("Expected pointer value for reference type"));
                }
            }
        };

        // Store the payload
        let payload_ptr = self.builder.build_struct_gep(struct_type, alloca, 1, "payload_ptr")?;
        self.builder.build_store(payload_ptr, payload_value)?;

        // Load and return the complete AlbayanValue
        let loaded_value = self.builder.build_load(struct_type, alloca, "albayan_value")?;
        Ok(loaded_value)
    }

    /// Extract an LLVM value from an AlbayanValue (Expert recommendation: Unboxing Safety)
    /// This function takes an AlbayanValue and extracts the underlying LLVM value with runtime type checking
    fn build_llvm_value_from_albayan(&self, albayan_value: BasicValueEnum<'ctx>, expected_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        let albayan_value_type = self.get_albayan_value_llvm_type();
        let struct_type = albayan_value_type.into_struct_type();

        // Allocate space and store the AlbayanValue
        let alloca = self.builder.build_alloca(struct_type, "temp_albayan_value")?;
        self.builder.build_store(alloca, albayan_value)?;

        // Extract the tag for runtime type checking (Expert recommendation: Unboxing Safety)
        let tag_ptr = self.builder.build_struct_gep(struct_type, alloca, 0, "tag_ptr")?;
        let tag_value = self.builder.build_load(self.context.i32_type(), tag_ptr, "tag")?
            .into_int_value();

        // Determine expected tag based on ResolvedType
        let expected_tag = match expected_type {
            ResolvedType::Int => 0,    // AlbayanValueTag::Int
            ResolvedType::Float => 1,  // AlbayanValueTag::Float
            ResolvedType::Bool => 2,   // AlbayanValueTag::Bool
            ResolvedType::String => 3, // AlbayanValueTag::String
            ResolvedType::List(_) => 4, // AlbayanValueTag::List
            ResolvedType::Struct(_) => 5, // AlbayanValueTag::Struct
            ResolvedType::Tuple(_) => 6, // AlbayanValueTag::Tuple
            _ => return Err(anyhow!("Unsupported type for AlbayanValue extraction: {:?}", expected_type)),
        };

        let expected_tag_val = self.context.i32_type().const_int(expected_tag, false);
        let is_correct_tag = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            tag_value,
            expected_tag_val,
            "tag_check"
        )?;

        // Create basic blocks for conditional execution (Expert recommendation: Unboxing Safety)
        let function = self.current_fn.ok_or_else(|| anyhow!("No current function"))?;
        let then_bb = self.context.append_basic_block(function, "tag_ok");
        let else_bb = self.context.append_basic_block(function, "tag_error");
        let cont_bb = self.context.append_basic_block(function, "cont");

        self.builder.build_conditional_branch(is_correct_tag, then_bb, else_bb)?;

        // Error path: call runtime panic function
        self.builder.position_at_end(else_bb);
        let panic_fn = self.functions.get("albayan_rt_panic")
            .ok_or_else(|| anyhow!("Panic function not found"))?;

        // Create error message
        let error_msg = format!("Type mismatch in AlbayanValue: expected tag {}, got different tag", expected_tag);
        let error_msg_global = self.builder.build_global_string_ptr(&error_msg, "error_msg")?;
        let error_msg_len = self.context.i64_type().const_int(error_msg.len() as u64, false);

        self.builder.build_call(*panic_fn, &[
            error_msg_global.as_pointer_value().into(),
            error_msg_len.into()
        ], "")?;
        self.builder.build_unreachable()?;

        // Success path: extract and convert payload
        self.builder.position_at_end(then_bb);

        // Extract the payload
        let payload_ptr = self.builder.build_struct_gep(struct_type, alloca, 1, "payload_ptr")?;
        let payload_value = self.builder.build_load(self.context.i64_type(), payload_ptr, "payload")?
            .into_int_value();

        // Convert payload back to the expected type
        let extracted_value = match expected_type {
            ResolvedType::Int => {
                // Payload is already i64
                payload_value.into()
            },
            ResolvedType::Float => {
                // Convert i64 bits back to float
                self.builder.build_bitcast(payload_value, self.context.f64_type(), "bits_to_float")?
            },
            ResolvedType::Bool => {
                // Convert i64 back to bool (i1)
                let bool_val = self.builder.build_int_truncate(payload_value, self.context.bool_type(), "i64_to_bool")?;
                bool_val.into()
            },
            ResolvedType::String => {
                // Convert i64 back to string pointer
                let string_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, string_ptr_type, "int_to_string_ptr")?;
                ptr_val.into()
            },
            ResolvedType::List(_) => {
                // Convert i64 back to list pointer
                let list_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default()); // Generic pointer
                let ptr_val = self.builder.build_int_to_ptr(payload_value, list_ptr_type, "int_to_list_ptr")?;
                ptr_val.into()
            },
            ResolvedType::Struct(_) => {
                // Convert i64 back to struct pointer
                let struct_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, struct_ptr_type, "int_to_struct_ptr")?;
                ptr_val.into()
            },
            ResolvedType::Tuple(_) => {
                // Convert i64 back to tuple pointer
                let tuple_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, tuple_ptr_type, "int_to_tuple_ptr")?;
                ptr_val.into()
            },
            _ => {
                return Err(anyhow!("Unsupported type for AlbayanValue extraction: {:?}", expected_type));
            }
        };

        // Branch to continuation block
        self.builder.build_unconditional_branch(cont_bb)?;

        // Position at continuation block and return the extracted value
        self.builder.position_at_end(cont_bb);
        Ok(extracted_value)
    }

    /// Declare runtime API functions for list operations
    /// This makes the runtime functions available for calling from generated code
    fn declare_runtime_api_functions(&mut self) -> Result<()> {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        let i64_type = self.context.i64_type();
        let usize_type = self.context.i64_type(); // Assuming 64-bit platform
        let albayan_value_type = self.get_albayan_value_llvm_type();

        // albayan_rt_list_create() -> *mut AlbayanList
        let list_create_fn_type = self.context.void_type().fn_type(&[], false);
        let list_create_fn = self.module.add_function("albayan_rt_list_create", list_create_fn_type, None);

        // albayan_rt_list_push(list_ptr: *mut AlbayanList, value: AlbayanValue) -> i32
        let list_push_fn_type = i32_type.fn_type(&[
            i8_ptr_type.into(), // list_ptr (generic pointer)
            albayan_value_type.into(), // value
        ], false);
        let list_push_fn = self.module.add_function("albayan_rt_list_push", list_push_fn_type, None);

        // albayan_rt_list_get(list_ptr: *const AlbayanList, index: usize) -> AlbayanValue
        let list_get_fn_type = albayan_value_type.fn_type(&[
            i8_ptr_type.into(), // list_ptr
            usize_type.into(),  // index
        ], false);
        let list_get_fn = self.module.add_function("albayan_rt_list_get", list_get_fn_type, None);

        // albayan_rt_list_len(list_ptr: *const AlbayanList) -> usize
        let list_len_fn_type = usize_type.fn_type(&[
            i8_ptr_type.into(), // list_ptr
        ], false);
        let list_len_fn = self.module.add_function("albayan_rt_list_len", list_len_fn_type, None);

        // albayan_rt_list_destroy(list_ptr: *mut AlbayanList) -> void (Expert recommendation: Automatic destroy calls)
        let list_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // list_ptr
        ], false);
        let list_destroy_fn = self.module.add_function("albayan_rt_list_destroy", list_destroy_fn_type, None);

        // albayan_rt_panic(message_ptr: *const u8, message_len: usize) -> ! (Expert recommendation: Unboxing Safety)
        let panic_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // message_ptr
            usize_type.into(),  // message_len
        ], false);
        let panic_fn = self.module.add_function("albayan_rt_panic", panic_fn_type, None);

        // Additional destroy functions for different types (Expert recommendation: Priority 1)

        // albayan_rt_string_destroy(string_ptr: *mut AlbayanString) -> void
        let string_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // string_ptr
        ], false);
        let string_destroy_fn = self.module.add_function("albayan_rt_string_destroy", string_destroy_fn_type, None);

        // albayan_rt_dict_destroy(dict_ptr: *mut AlbayanDict) -> void
        let dict_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // dict_ptr
        ], false);
        let dict_destroy_fn = self.module.add_function("albayan_rt_dict_destroy", dict_destroy_fn_type, None);

        // albayan_rt_struct_destroy(struct_ptr: *mut AlbayanStruct) -> void
        let struct_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // struct_ptr
        ], false);
        let struct_destroy_fn = self.module.add_function("albayan_rt_struct_destroy", struct_destroy_fn_type, None);

        // AI types destroy functions (Expert recommendation: Priority 1 - AI types)

        // albayan_rt_model_destroy(model_ptr: *mut u8) -> void
        let model_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // model_ptr
        ], false);
        let model_destroy_fn = self.module.add_function("albayan_rt_model_destroy", model_destroy_fn_type, None);

        // albayan_rt_tensor_destroy(tensor_ptr: *mut u8) -> void
        let tensor_destroy_fn_type = self.context.void_type().fn_type(&[
            i8_ptr_type.into(), // tensor_ptr
        ], false);
        let tensor_destroy_fn = self.module.add_function("albayan_rt_tensor_destroy", tensor_destroy_fn_type, None);

        // Store function references for later use
        self.functions.insert("albayan_rt_list_create".to_string(), list_create_fn);
        self.functions.insert("albayan_rt_list_push".to_string(), list_push_fn);
        self.functions.insert("albayan_rt_list_get".to_string(), list_get_fn);
        self.functions.insert("albayan_rt_list_len".to_string(), list_len_fn);
        self.functions.insert("albayan_rt_list_destroy".to_string(), list_destroy_fn);
        self.functions.insert("albayan_rt_string_destroy".to_string(), string_destroy_fn);
        self.functions.insert("albayan_rt_dict_destroy".to_string(), dict_destroy_fn);
        self.functions.insert("albayan_rt_struct_destroy".to_string(), struct_destroy_fn);
        self.functions.insert("albayan_rt_model_destroy".to_string(), model_destroy_fn);
        self.functions.insert("albayan_rt_tensor_destroy".to_string(), tensor_destroy_fn);
        self.functions.insert("albayan_rt_panic".to_string(), panic_fn);

        Ok(())
    }

    /// Resolve a ResolvedType to an LLVM BasicTypeEnum
    fn resolve_llvm_type(&self, resolved_type: &ResolvedType) -> Result<BasicTypeEnum<'ctx>> {
        match resolved_type {
            ResolvedType::Int => Ok(self.context.i64_type().into()),
            ResolvedType::Float => Ok(self.context.f64_type().into()),
            ResolvedType::Bool => Ok(self.context.bool_type().into()),
            ResolvedType::String => Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into()),
            ResolvedType::List(_) => {
                // Lists are represented as pointers to AlbayanList
                Ok(self.context.i8_type().ptr_type(AddressSpace::default()).into())
            }
            ResolvedType::Tuple(element_types) => {
                // Create a struct type for the tuple
                let mut field_types = Vec::new();
                for elem_type in element_types {
                    field_types.push(self.resolve_llvm_type(elem_type)?);
                }
                let tuple_type = self.context.struct_type(&field_types, false);
                Ok(tuple_type.into())
            }
            ResolvedType::Struct(name) => {
                // Get the struct type from cache or create it
                self.get_llvm_struct_type(name).map(|t| t.into())
            }
            ResolvedType::TraitObject(_traits) => {
                // Trait object as fat pointer (Expert recommendation: Priority 1)
                // Fat pointer = { *mut u8 (data), *const VTable (vtable) }
                let data_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let vtable_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());

                let fat_pointer_type = self.context.struct_type(
                    &[data_ptr_type.into(), vtable_ptr_type.into()],
                    false
                );
                Ok(fat_pointer_type.into())
            }
            ResolvedType::Reference(referenced_type, _is_mutable) => {
                // Reference types (&T, &mut T) - Expert recommendation: Priority 1
                // References are represented as pointers in LLVM
                let referenced_llvm_type = self.resolve_llvm_type(referenced_type)?;
                Ok(referenced_llvm_type.ptr_type(AddressSpace::default()).into())
            }
            _ => Err(anyhow!("Unsupported type for LLVM resolution: {:?}", resolved_type)),
        }
    }

    /// Generate code for an array literal (Expert recommendation: List<T> support)
    fn generate_array_literal(&mut self, elements: &[AnnotatedExpression], result_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        // Ensure runtime API functions are declared
        self.declare_runtime_api_functions()?;

        // Extract element type from List<T>
        let element_type = match result_type {
            ResolvedType::List(elem_type) => elem_type.as_ref(),
            _ => return Err(anyhow!("Expected List type for array literal")),
        };

        // Create a new list
        let create_fn = self.functions.get("albayan_rt_list_create")
            .ok_or_else(|| anyhow!("List create function not found"))?;
        let list_ptr = self.builder.build_call(*create_fn, &[], "new_list")?
            .try_as_basic_value().left()
            .ok_or_else(|| anyhow!("List create returned void"))?;

        // Push each element to the list
        let push_fn = self.functions.get("albayan_rt_list_push")
            .ok_or_else(|| anyhow!("List push function not found"))?;

        for element in elements {
            // Generate the element value
            let element_value = self.generate_expression(element)?;

            // Convert to AlbayanValue
            let albayan_value = self.build_albayan_value_from_llvm(element_value, element_type)?;

            // Push to list
            self.builder.build_call(*push_fn, &[list_ptr, albayan_value], "push_element")?;
        }

        // Return the list pointer
        Ok(list_ptr)
    }

    /// Generate code for index access (Expert recommendation: List<T> support)
    fn generate_index_access(&mut self, object: &AnnotatedExpression, index: &AnnotatedExpression) -> Result<BasicValueEnum<'ctx>> {
        // Generate the object and index values
        let object_value = self.generate_expression(object)?;
        let index_value = self.generate_expression(index)?;

        match &object.result_type {
            ResolvedType::List(element_type) => {
                // Ensure runtime API functions are declared
                self.declare_runtime_api_functions()?;

                // Call albayan_rt_list_get
                let get_fn = self.functions.get("albayan_rt_list_get")
                    .ok_or_else(|| anyhow!("List get function not found"))?;

                let albayan_value = self.builder.build_call(*get_fn, &[object_value, index_value], "list_get")?
                    .try_as_basic_value().left()
                    .ok_or_else(|| anyhow!("List get returned void"))?;

                // Convert AlbayanValue back to LLVM value
                self.build_llvm_value_from_albayan(albayan_value, element_type)
            }
            ResolvedType::Tuple(element_types) => {
                // For tuple indexing, we need to extract the field at compile time
                if let AnnotatedExpressionKind::Literal(Literal::Integer(idx)) = &index.expr {
                    let idx = *idx as u32;
                    if (idx as usize) < element_types.len() {
                        // Generate GEP for tuple field access
                        if object_value.is_pointer_value() {
                            let tuple_ptr = object_value.into_pointer_value();
                            let tuple_type = self.resolve_llvm_type(&object.result_type)?;

                            let field_ptr = self.builder.build_struct_gep(
                                tuple_type.into_struct_type(),
                                tuple_ptr,
                                idx,
                                &format!("tuple_field_{}", idx)
                            )?;

                            let field_type = self.resolve_llvm_type(&element_types[idx as usize])?;
                            Ok(self.builder.build_load(field_type, field_ptr, "tuple_element")?)
                        } else {
                            Err(anyhow!("Tuple indexing requires pointer value"))
                        }
                    } else {
                        Err(anyhow!("Tuple index out of bounds"))
                    }
                } else {
                    Err(anyhow!("Tuple indexing requires compile-time constant index"))
                }
            }
            _ => {
                Err(anyhow!("Index access not supported for type: {:?}", object.result_type))
            }
        }
    }

    /// Generate automatic destroy calls for variables going out of scope (Expert recommendation)
    fn generate_automatic_destroy_calls(&mut self) -> Result<()> {
        // Expert recommendation: "في نهاية generate_block، قم بالمرور على المتغيرات التي يجب إسقاطها.
        // إذا كان نوع المتغير List<T>، قم بتوليد استدعاء لـ albayan_rt_list_destroy(handle)"

        // For now, we implement a basic version that demonstrates the concept
        // In a full implementation, this would integrate with SemanticAnalyzer.get_variables_to_destroy()

        // Ensure runtime API functions are declared
        self.declare_runtime_api_functions()?;

        // Get the current scope variables
        if let Some(current_scope) = self.variable_scopes.last() {
            for (var_name, var_alloca) in current_scope {
                // Check if this variable needs destruction
                // For now, we'll check if the variable name suggests it's a list
                // In a real implementation, this would use type information from semantic analysis
                if var_name.contains("list") || var_name.contains("array") {
                    // Generate destroy call for List<T> variables (Expert recommendation)
                    if let Some(destroy_fn) = self.functions.get("albayan_rt_list_destroy") {
                        // Load the list handle
                        let list_handle = self.builder.build_load(
                            var_alloca.get_type().get_element_type().into(),
                            *var_alloca,
                            &format!("{}_destroy_load", var_name)
                        )?;

                        // Call albayan_rt_list_destroy(handle)
                        self.builder.build_call(
                            *destroy_fn,
                            &[list_handle.into()],
                            &format!("{}_destroy", var_name)
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate destroy calls for specific variables using semantic analysis results (Expert recommendation: Priority 1)
    fn generate_destroy_calls_for_variables(&mut self, variables_to_destroy: &[DestroyInfo]) -> Result<()> {
        // Expert recommendation: "جني ثمار التحليل - استخدام معلومات SemanticAnalyzer.get_variables_to_destroy()"

        // Ensure runtime API functions are declared
        self.declare_runtime_api_functions()?;

        for destroy_info in variables_to_destroy {
            // Look up the variable in current scopes
            if let Some(var_alloca) = self.lookup_variable(&destroy_info.name) {
                match &destroy_info.var_type {
                    ResolvedType::List(_) => {
                        // Generate destroy call for List<T> variables (Expert recommendation)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_list_destroy") {
                            // Load the list handle
                            let list_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_list_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[list_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::String => {
                        // Generate destroy call for String variables (Expert recommendation)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_string_destroy") {
                            // Load the string handle
                            let string_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_string_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[string_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::Dict(_, _) => {
                        // Generate destroy call for Dict variables (Expert recommendation)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_dict_destroy") {
                            // Load the dict handle
                            let dict_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_dict_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[dict_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::Struct(_) => {
                        // Generate destroy call for Struct variables (Expert recommendation)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_struct_destroy") {
                            // Load the struct handle
                            let struct_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_struct_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[struct_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::String => {
                        // Generate destroy call for String variables (Expert recommendation)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_string_destroy") {
                            // Load the string handle
                            let string_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_string_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[string_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::Model(_) => {
                        // Generate destroy call for Model types (Expert recommendation: Priority 1)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_model_destroy") {
                            // Load the model handle
                            let model_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_model_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[model_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    ResolvedType::Tensor(_) => {
                        // Generate destroy call for Tensor types (Expert recommendation: Priority 1)
                        if let Some(destroy_fn) = self.functions.get("albayan_rt_tensor_destroy") {
                            // Load the tensor handle
                            let tensor_handle = self.builder.build_load(
                                var_alloca.get_type().get_element_type().into(),
                                var_alloca,
                                &format!("{}_destroy_load", destroy_info.name)
                            )?;

                            // Call albayan_rt_tensor_destroy(handle)
                            self.builder.build_call(
                                *destroy_fn,
                                &[tensor_handle.into()],
                                &format!("{}_destroy", destroy_info.name)
                            )?;
                        }
                    }
                    // Copy types (Int, Float, Bool) don't need destruction
                    ResolvedType::Int | ResolvedType::Float | ResolvedType::Bool | ResolvedType::Char => {
                        // No destruction needed for Copy types
                    }
                    _ => {
                        // For other types, we might need custom destruction logic
                        // For now, skip unknown types
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate V-Table for trait implementation (Expert recommendation: Priority 1)
    fn generate_vtable(&mut self, trait_name: &str, type_name: &str, methods: &[String]) -> Result<PointerValue<'ctx>> {
        let vtable_key = format!("{}_{}_vtable", trait_name, type_name);

        // Check if V-Table already exists
        if let Some(existing_vtable) = self.vtables.get(&vtable_key) {
            return Ok(*existing_vtable);
        }

        // Create V-Table type (array of function pointers)
        let function_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let vtable_type = function_ptr_type.array_type(methods.len() as u32);

        // Create global V-Table
        let vtable_global = self.module.add_global(vtable_type, Some(AddressSpace::default()), &vtable_key);
        vtable_global.set_linkage(inkwell::module::Linkage::Internal);
        vtable_global.set_constant(true);

        // Create array of function pointers
        let mut function_pointers = Vec::new();
        for method_name in methods {
            let mangled_method_name = format!("{}::{}", type_name, method_name);

            // Get or create function declaration
            let function = if let Some(func) = self.functions.get(&mangled_method_name) {
                *func
            } else {
                // Create function declaration if it doesn't exist
                let func_type = self.context.void_type().fn_type(&[], false);
                let func = self.module.add_function(&mangled_method_name, func_type, None);
                self.functions.insert(mangled_method_name.clone(), func);
                func
            };

            // Cast function to generic function pointer
            let func_ptr = function.as_global_value().as_pointer_value();
            let generic_ptr = self.builder.build_bitcast(
                func_ptr,
                function_ptr_type,
                &format!("{}_cast", method_name)
            )?;
            function_pointers.push(generic_ptr.into());
        }

        // Set V-Table initializer
        let vtable_array = function_ptr_type.const_array(&function_pointers);
        vtable_global.set_initializer(&vtable_array);

        // Store V-Table reference
        let vtable_ptr = vtable_global.as_pointer_value();
        self.vtables.insert(vtable_key, vtable_ptr);

        Ok(vtable_ptr)
    }

    /// Create fat pointer for trait object (Expert recommendation: Priority 1)
    fn create_fat_pointer(&mut self, data_ptr: PointerValue<'ctx>, vtable_ptr: PointerValue<'ctx>) -> Result<BasicValueEnum<'ctx>> {
        // Create fat pointer type
        let data_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let vtable_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let fat_pointer_type = self.context.struct_type(
            &[data_ptr_type.into(), vtable_ptr_type.into()],
            false
        );

        // Allocate fat pointer
        let fat_pointer_alloca = self.builder.build_alloca(fat_pointer_type, "fat_pointer")?;

        // Cast data pointer to generic pointer
        let generic_data_ptr = self.builder.build_bitcast(
            data_ptr,
            data_ptr_type,
            "data_ptr_cast"
        )?;

        // Cast vtable pointer to generic pointer
        let generic_vtable_ptr = self.builder.build_bitcast(
            vtable_ptr,
            vtable_ptr_type,
            "vtable_ptr_cast"
        )?;

        // Store data pointer in fat pointer
        let data_field_ptr = self.builder.build_struct_gep(
            fat_pointer_type,
            fat_pointer_alloca,
            0,
            "data_field_ptr"
        )?;
        self.builder.build_store(data_field_ptr, generic_data_ptr)?;

        // Store vtable pointer in fat pointer
        let vtable_field_ptr = self.builder.build_struct_gep(
            fat_pointer_type,
            fat_pointer_alloca,
            1,
            "vtable_field_ptr"
        )?;
        self.builder.build_store(vtable_field_ptr, generic_vtable_ptr)?;

        // Load and return fat pointer
        let fat_pointer = self.builder.build_load(fat_pointer_type, fat_pointer_alloca, "fat_pointer_load")?;
        Ok(fat_pointer)
    }

    /// Generate dynamic dispatch call for trait objects (Expert recommendation: Priority 1)
    fn generate_dynamic_dispatch_call(
        &mut self,
        name: &str,
        arguments: &[AnnotatedExpression]
    ) -> Result<BasicValueEnum<'ctx>> {
        // Parse dynamic call name: "dyn_TraitName::method_name"
        let parts: Vec<&str> = name.strip_prefix("dyn_").unwrap().split("::").collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid dynamic dispatch call format: {}", name));
        }

        let trait_name = parts[0];
        let method_name = parts[1];

        // First argument should be the trait object (fat pointer)
        if arguments.is_empty() {
            return Err(anyhow!("Dynamic dispatch requires at least one argument (self)"));
        }

        let trait_object_expr = &arguments[0];
        let trait_object_value = self.generate_expression(trait_object_expr)?;

        // Extract fat pointer components (Expert recommendation: Priority 1)
        let fat_pointer_type = trait_object_value.get_type();

        // Create temporary alloca for fat pointer if it's not already a pointer
        let fat_pointer_ptr = if trait_object_value.is_pointer_value() {
            trait_object_value.into_pointer_value()
        } else {
            let temp_alloca = self.builder.build_alloca(fat_pointer_type, "temp_fat_pointer")?;
            self.builder.build_store(temp_alloca, trait_object_value)?;
            temp_alloca
        };

        // Extract data pointer (field 0)
        let data_field_ptr = self.builder.build_struct_gep(
            fat_pointer_type,
            fat_pointer_ptr,
            0,
            "data_field_ptr"
        )?;
        let data_ptr = self.builder.build_load(
            self.context.i8_type().ptr_type(AddressSpace::default()),
            data_field_ptr,
            "data_ptr"
        )?;

        // Extract vtable pointer (field 1)
        let vtable_field_ptr = self.builder.build_struct_gep(
            fat_pointer_type,
            fat_pointer_ptr,
            1,
            "vtable_field_ptr"
        )?;
        let vtable_ptr = self.builder.build_load(
            self.context.i8_type().ptr_type(AddressSpace::default()),
            vtable_field_ptr,
            "vtable_ptr"
        )?;

        // Calculate method offset in vtable (Expert recommendation: Priority 1)
        // For now, we'll use a simple mapping: method index based on trait definition
        let method_index = self.get_trait_method_index(trait_name, method_name)?;

        // Load function pointer from vtable
        let function_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let vtable_array_ptr = self.builder.build_bitcast(
            vtable_ptr.into_pointer_value(),
            function_ptr_type.ptr_type(AddressSpace::default()),
            "vtable_array_ptr"
        )?;

        let method_ptr_ptr = unsafe {
            self.builder.build_gep(
                function_ptr_type,
                vtable_array_ptr.into_pointer_value(),
                &[self.context.i32_type().const_int(method_index as u64, false)],
                "method_ptr_ptr"
            )?
        };

        let method_ptr = self.builder.build_load(
            function_ptr_type,
            method_ptr_ptr,
            "method_ptr"
        )?;

        // Prepare arguments for dynamic call
        let mut call_args = Vec::new();
        call_args.push(data_ptr); // First argument is always the data pointer (self)

        // Add remaining arguments (skip first argument which is the trait object)
        for arg in &arguments[1..] {
            let arg_value = self.generate_expression(arg)?;
            call_args.push(arg_value);
        }

        // Create function type for indirect call
        let arg_types: Vec<BasicMetadataTypeEnum> = call_args.iter()
            .map(|arg| arg.get_type().into())
            .collect();

        let return_type = self.context.i64_type(); // Simplified return type for now
        let function_type = return_type.fn_type(&arg_types, false);

        // Cast method pointer to correct function type
        let typed_method_ptr = self.builder.build_bitcast(
            method_ptr.into_pointer_value(),
            function_type.ptr_type(AddressSpace::default()),
            "typed_method_ptr"
        )?;

        // Perform indirect call (Expert recommendation: Priority 1)
        let call_result = self.builder.build_indirect_call(
            function_type,
            typed_method_ptr.into_pointer_value(),
            &call_args.iter().map(|v| (*v).into()).collect::<Vec<_>>(),
            &format!("dyn_call_{}_{}", trait_name, method_name)
        )?;

        if let Some(result) = call_result.try_as_basic_value().left() {
            Ok(result)
        } else {
            // Function returns void, return a dummy value
            Ok(self.context.i64_type().const_int(0, false).into())
        }
    }

    /// Get method index in trait vtable (Expert recommendation: Priority 1)
    fn get_trait_method_index(&self, trait_name: &str, method_name: &str) -> Result<usize> {
        // For now, return a simple index based on method name
        // In a full implementation, this would look up the trait definition
        // and return the actual method index
        match method_name {
            "to_string" => Ok(0),
            "display" => Ok(1),
            "clone" => Ok(2),
            _ => Ok(0), // Default to first method
        }
    }
}
