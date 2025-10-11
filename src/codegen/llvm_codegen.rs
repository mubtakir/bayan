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
use crate::semantic::mod::AnnotatedProgram;
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

        // Add return if missing (improved return handling)
        if !self.is_terminated() {
            if let Some(ref ret_type) = func.return_type {
                if matches!(ret_type, ResolvedType::Void) {
                    self.builder.build_return(None)?;
                } else {
                    // Return default value for non-void functions
                    let default_val = self.get_default_value(ret_type)?;
                    self.builder.build_return(Some(&default_val))?;
                }
            } else {
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

        self.leave_scope();
        Ok(())
    }

    /// Declare a struct type
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
        }

        Ok(())
    }

    /// Generate code for an expression
    fn generate_expression(&mut self, expr: &AnnotatedExpression) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            AnnotatedExpression::Literal(lit) => self.generate_literal(lit),
            AnnotatedExpression::Variable(name) => {
                // Use improved scope lookup (as recommended by expert)
                if let Some(alloca) = self.lookup_variable(name) {
                    Ok(self.builder.build_load(
                        alloca.get_type().get_element_type().into(),
                        alloca,
                        name
                    )?)
                } else {
                    Err(anyhow!("Undefined variable: {}", name))
                }
            }
            AnnotatedExpression::BinaryOp { left, operator, right } => {
                self.generate_binary_op(left, operator, right)
            }
            AnnotatedExpression::UnaryOp { operator, operand } => {
                self.generate_unary_op(operator, operand)
            }
            AnnotatedExpression::FunctionCall { name, arguments } => {
                self.generate_function_call(name, arguments)
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

        // Look up user-defined function
        if let Some(&function) = self.functions.get(name) {
            let mut args: Vec<BasicValueEnum> = Vec::new();

            // Generate arguments with proper type checking
            for arg in arguments {
                let arg_value = self.generate_expression(arg)?;
                args.push(arg_value);
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

    /// Get LLVM type from AlBayan type
    fn get_llvm_type(&self, albayan_type: &Type) -> Result<Option<BasicTypeEnum<'ctx>>> {
        match albayan_type {
            Type::Int => Ok(Some(self.context.i64_type().into())),
            Type::Float => Ok(Some(self.context.f64_type().into())),
            Type::Bool => Ok(Some(self.context.bool_type().into())),
            Type::String => Ok(Some(self.context.i8_type().ptr_type(AddressSpace::default()).into())),
            Type::Void => Ok(None),
            Type::Custom(name) => {
                if let Some(&cached_type) = self.type_cache.get(name) {
                    Ok(Some(cached_type))
                } else {
                    Err(anyhow!("Unknown custom type: {}", name))
                }
            }
            _ => Err(anyhow!("Type not yet supported: {:?}", albayan_type))
        }
    }

    /// Get default value for a type
    fn get_default_value(&self, albayan_type: &Type) -> Result<BasicValueEnum<'ctx>> {
        match albayan_type {
            Type::Int => Ok(self.context.i64_type().const_zero().into()),
            Type::Float => Ok(self.context.f64_type().const_zero().into()),
            Type::Bool => Ok(self.context.bool_type().const_zero().into()),
            Type::String => {
                let empty_str = self.context.const_string(b"", true);
                Ok(empty_str.into())
            }
            _ => Err(anyhow!("Cannot create default value for type: {:?}", albayan_type))
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

    /// Verify the generated module
    pub fn verify(&self) -> Result<()> {
        if let Err(errors) = self.module.verify() {
            Err(anyhow!("LLVM module verification failed: {}", errors))
        } else {
            Ok(())
        }
    }
}
