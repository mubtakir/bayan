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
        match &expr.expr {
            AnnotatedExpressionKind::Literal(lit) => self.generate_literal(lit),
            AnnotatedExpressionKind::Identifier(name) => {
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
            "struct_value"
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
                field_name
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
                field_name
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

    /// Get LLVM struct type by name
    fn get_llvm_struct_type(&self, struct_name: &str) -> Result<inkwell::types::StructType<'ctx>> {
        // For now, create a simple struct type
        // This should be improved to use actual struct definitions
        match struct_name {
            "Point" => {
                // Example: Point { x: int, y: int }
                let i64_type = self.context.i64_type();
                Ok(self.context.struct_type(&[i64_type.into(), i64_type.into()], false))
            }
            _ => {
                // Generic struct with i64 fields for now
                let i64_type = self.context.i64_type();
                Ok(self.context.struct_type(&[i64_type.into()], false))
            }
        }
    }

    /// Get field index in struct
    fn get_field_index(&self, struct_name: &str, field_name: &str) -> Result<u32> {
        // For now, hardcode some common field mappings
        // This should be improved to use actual struct definitions
        match (struct_name, field_name) {
            ("Point", "x") => Ok(0),
            ("Point", "y") => Ok(1),
            _ => Ok(0), // Default to first field
        }
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

    /// Extract an LLVM value from an AlbayanValue
    /// This function takes an AlbayanValue and extracts the underlying LLVM value
    fn build_llvm_value_from_albayan(&self, albayan_value: BasicValueEnum<'ctx>, expected_type: &ResolvedType) -> Result<BasicValueEnum<'ctx>> {
        let albayan_value_type = self.get_albayan_value_llvm_type();
        let struct_type = albayan_value_type.into_struct_type();

        // Allocate space and store the AlbayanValue
        let alloca = self.builder.build_alloca(struct_type, "temp_albayan_value")?;
        self.builder.build_store(alloca, albayan_value)?;

        // Extract the tag for runtime type checking (optional - could be used for safety)
        let tag_ptr = self.builder.build_struct_gep(struct_type, alloca, 0, "tag_ptr")?;
        let _tag_value = self.builder.build_load(self.context.i32_type(), tag_ptr, "tag")?;

        // Extract the payload
        let payload_ptr = self.builder.build_struct_gep(struct_type, alloca, 1, "payload_ptr")?;
        let payload_value = self.builder.build_load(self.context.i64_type(), payload_ptr, "payload")?
            .into_int_value();

        // Convert payload back to the expected type
        match expected_type {
            ResolvedType::Int => {
                // Payload is already i64
                Ok(payload_value.into())
            },
            ResolvedType::Float => {
                // Convert i64 bits back to float
                let float_val = self.builder.build_bitcast(payload_value, self.context.f64_type(), "bits_to_float")?;
                Ok(float_val)
            },
            ResolvedType::Bool => {
                // Convert i64 back to bool (i1)
                let bool_val = self.builder.build_int_truncate(payload_value, self.context.bool_type(), "i64_to_bool")?;
                Ok(bool_val.into())
            },
            ResolvedType::String => {
                // Convert i64 back to string pointer
                let string_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, string_ptr_type, "int_to_string_ptr")?;
                Ok(ptr_val.into())
            },
            ResolvedType::List(_) => {
                // Convert i64 back to list pointer
                let list_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default()); // Generic pointer
                let ptr_val = self.builder.build_int_to_ptr(payload_value, list_ptr_type, "int_to_list_ptr")?;
                Ok(ptr_val.into())
            },
            ResolvedType::Struct(_) => {
                // Convert i64 back to struct pointer
                let struct_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, struct_ptr_type, "int_to_struct_ptr")?;
                Ok(ptr_val.into())
            },
            ResolvedType::Tuple(_) => {
                // Convert i64 back to tuple pointer
                let tuple_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let ptr_val = self.builder.build_int_to_ptr(payload_value, tuple_ptr_type, "int_to_tuple_ptr")?;
                Ok(ptr_val.into())
            },
            _ => {
                Err(anyhow!("Unsupported type for AlbayanValue extraction: {:?}", expected_type))
            }
        }
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

        // Store function references for later use
        self.functions.insert("albayan_rt_list_create".to_string(), list_create_fn);
        self.functions.insert("albayan_rt_list_push".to_string(), list_push_fn);
        self.functions.insert("albayan_rt_list_get".to_string(), list_get_fn);
        self.functions.insert("albayan_rt_list_len".to_string(), list_len_fn);

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
}
