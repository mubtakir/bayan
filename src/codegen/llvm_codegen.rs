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
    
    /// Generate function body
    fn generate_function(&mut self, func: &AnnotatedFunction) -> Result<()> {
        let function = self.functions[&func.name];
        self.current_function = Some(function);
        
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        
        // Create allocas for parameters
        for (i, param) in func.parameters.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let alloca = self.create_entry_block_alloca(&param.name, &param.param_type)?;
            self.builder.build_store(alloca, param_value)?;
            self.variables.insert(param.name.clone(), alloca);
        }
        
        // Generate function body
        self.generate_statement(&func.body)?;
        
        // Add return if missing
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            if func.return_type == Type::Void {
                self.builder.build_return(None)?;
            } else {
                // Return default value for non-void functions
                let default_val = self.get_default_value(&func.return_type)?;
                self.builder.build_return(Some(&default_val))?;
            }
        }
        
        self.current_function = None;
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
                
                self.variables.insert(name.clone(), alloca);
            }
            AnnotatedStatement::Assignment { target, value } => {
                let value_result = self.generate_expression(value)?;
                
                if let AnnotatedExpression::Variable(var_name) = target.as_ref() {
                    if let Some(&alloca) = self.variables.get(var_name) {
                        self.builder.build_store(alloca, value_result)?;
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
                if let Some(&alloca) = self.variables.get(name) {
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

    /// Generate code for function calls
    fn generate_function_call(
        &mut self,
        name: &str,
        arguments: &[AnnotatedExpression]
    ) -> Result<BasicValueEnum<'ctx>> {
        if let Some(&function) = self.functions.get(name) {
            let args: Vec<BasicValueEnum> = arguments
                .iter()
                .map(|arg| self.generate_expression(arg))
                .collect::<Result<Vec<_>>>()?;

            let call_result = self.builder.build_call(function, &args, "calltmp")?;

            if let Some(result) = call_result.try_as_basic_value().left() {
                Ok(result)
            } else {
                Err(anyhow!("Function call returned void"))
            }
        } else {
            Err(anyhow!("Undefined function: {}", name))
        }
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
