//! Debugger for AlBayan language
//! 
//! Provides debugging capabilities including breakpoints, step execution,
//! variable inspection, and call stack analysis

use std::collections::{HashMap, HashSet};
use anyhow::{Result, anyhow};

/// Debugger for AlBayan programs
#[derive(Debug)]
pub struct Debugger {
    /// Breakpoints set by line number
    breakpoints: HashMap<String, HashSet<u32>>,
    /// Current execution state
    state: DebugState,
    /// Call stack
    call_stack: Vec<StackFrame>,
    /// Variable values in current scope
    variables: HashMap<String, DebugValue>,
    /// Execution history
    execution_history: Vec<ExecutionStep>,
    /// Debug configuration
    config: DebugConfig,
}

/// Debug state
#[derive(Debug, Clone)]
pub enum DebugState {
    /// Program is not running
    Stopped,
    /// Program is running
    Running,
    /// Program is paused at a breakpoint
    Paused {
        file: String,
        line: u32,
        column: u32,
    },
    /// Program has finished execution
    Finished,
    /// Program crashed with an error
    Error(String),
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Function name
    pub function_name: String,
    /// File name
    pub file_name: String,
    /// Line number
    pub line_number: u32,
    /// Local variables
    pub local_variables: HashMap<String, DebugValue>,
}

/// Debug value representation
#[derive(Debug, Clone)]
pub enum DebugValue {
    /// Integer value
    Int(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// Array value
    Array(Vec<DebugValue>),
    /// Object value
    Object(HashMap<String, DebugValue>),
    /// Null value
    Null,
    /// Reference to another value
    Reference(Box<DebugValue>),
}

/// Execution step in debug history
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    /// Step number
    pub step: u64,
    /// File name
    pub file: String,
    /// Line number
    pub line: u32,
    /// Instruction executed
    pub instruction: String,
    /// Variables changed
    pub variables_changed: Vec<String>,
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    /// Break on exceptions
    pub break_on_exceptions: bool,
    /// Break on function entry
    pub break_on_function_entry: bool,
    /// Maximum call stack depth to track
    pub max_stack_depth: usize,
    /// Maximum execution history to keep
    pub max_history_size: usize,
}

impl Debugger {
    /// Create a new debugger
    pub fn new() -> Self {
        Self {
            breakpoints: HashMap::new(),
            state: DebugState::Stopped,
            call_stack: Vec::new(),
            variables: HashMap::new(),
            execution_history: Vec::new(),
            config: DebugConfig::default(),
        }
    }
    
    /// Set a breakpoint
    pub fn set_breakpoint(&mut self, file: &str, line: u32) -> Result<()> {
        self.breakpoints
            .entry(file.to_string())
            .or_insert_with(HashSet::new)
            .insert(line);
        
        println!("Breakpoint set at {}:{}", file, line);
        Ok(())
    }
    
    /// Remove a breakpoint
    pub fn remove_breakpoint(&mut self, file: &str, line: u32) -> Result<()> {
        if let Some(file_breakpoints) = self.breakpoints.get_mut(file) {
            file_breakpoints.remove(&line);
            if file_breakpoints.is_empty() {
                self.breakpoints.remove(file);
            }
        }
        
        println!("Breakpoint removed from {}:{}", file, line);
        Ok(())
    }
    
    /// List all breakpoints
    pub fn list_breakpoints(&self) -> Vec<(String, u32)> {
        let mut breakpoints = Vec::new();
        for (file, lines) in &self.breakpoints {
            for &line in lines {
                breakpoints.push((file.clone(), line));
            }
        }
        breakpoints.sort();
        breakpoints
    }
    
    /// Start debugging a program
    pub fn start_debug(&mut self, program_path: &str) -> Result<()> {
        println!("Starting debug session for: {}", program_path);
        self.state = DebugState::Running;
        self.call_stack.clear();
        self.variables.clear();
        self.execution_history.clear();
        
        // TODO: Initialize program execution
        Ok(())
    }
    
    /// Continue execution
    pub fn continue_execution(&mut self) -> Result<()> {
        match &self.state {
            DebugState::Paused { .. } => {
                println!("Continuing execution...");
                self.state = DebugState::Running;
                // TODO: Resume program execution
                Ok(())
            }
            _ => Err(anyhow!("Cannot continue: program is not paused")),
        }
    }
    
    /// Step to next line
    pub fn step_over(&mut self) -> Result<()> {
        match &self.state {
            DebugState::Paused { .. } | DebugState::Running => {
                println!("Stepping over...");
                // TODO: Execute single step
                self.record_execution_step("step_over", "next_line", Vec::new());
                Ok(())
            }
            _ => Err(anyhow!("Cannot step: program is not running or paused")),
        }
    }
    
    /// Step into function
    pub fn step_into(&mut self) -> Result<()> {
        match &self.state {
            DebugState::Paused { .. } | DebugState::Running => {
                println!("Stepping into...");
                // TODO: Step into function call
                self.record_execution_step("step_into", "function_call", Vec::new());
                Ok(())
            }
            _ => Err(anyhow!("Cannot step: program is not running or paused")),
        }
    }
    
    /// Step out of current function
    pub fn step_out(&mut self) -> Result<()> {
        match &self.state {
            DebugState::Paused { .. } | DebugState::Running => {
                println!("Stepping out...");
                // TODO: Step out of current function
                self.record_execution_step("step_out", "function_return", Vec::new());
                Ok(())
            }
            _ => Err(anyhow!("Cannot step: program is not running or paused")),
        }
    }
    
    /// Pause execution
    pub fn pause(&mut self) -> Result<()> {
        match &self.state {
            DebugState::Running => {
                self.state = DebugState::Paused {
                    file: "current_file.ab".to_string(),
                    line: 1,
                    column: 1,
                };
                println!("Execution paused");
                Ok(())
            }
            _ => Err(anyhow!("Cannot pause: program is not running")),
        }
    }
    
    /// Stop debugging
    pub fn stop(&mut self) -> Result<()> {
        self.state = DebugState::Stopped;
        self.call_stack.clear();
        self.variables.clear();
        println!("Debug session stopped");
        Ok(())
    }
    
    /// Get current debug state
    pub fn get_state(&self) -> &DebugState {
        &self.state
    }
    
    /// Get call stack
    pub fn get_call_stack(&self) -> &[StackFrame] {
        &self.call_stack
    }
    
    /// Get variable value
    pub fn get_variable(&self, name: &str) -> Option<&DebugValue> {
        self.variables.get(name)
    }
    
    /// Set variable value
    pub fn set_variable(&mut self, name: &str, value: DebugValue) -> Result<()> {
        self.variables.insert(name.to_string(), value);
        println!("Variable '{}' updated", name);
        Ok(())
    }
    
    /// List all variables in current scope
    pub fn list_variables(&self) -> &HashMap<String, DebugValue> {
        &self.variables
    }
    
    /// Evaluate expression in current context
    pub fn evaluate_expression(&self, expression: &str) -> Result<DebugValue> {
        // TODO: Implement expression evaluation
        println!("Evaluating expression: {}", expression);
        
        // Simple mock evaluation
        if let Some(value) = self.variables.get(expression) {
            Ok(value.clone())
        } else {
            Ok(DebugValue::String(format!("Result of: {}", expression)))
        }
    }
    
    /// Get execution history
    pub fn get_execution_history(&self) -> &[ExecutionStep] {
        &self.execution_history
    }
    
    /// Record an execution step
    fn record_execution_step(&mut self, instruction: &str, details: &str, variables_changed: Vec<String>) {
        let step = ExecutionStep {
            step: self.execution_history.len() as u64 + 1,
            file: "current_file.ab".to_string(),
            line: 1,
            instruction: format!("{}: {}", instruction, details),
            variables_changed,
        };
        
        self.execution_history.push(step);
        
        // Limit history size
        if self.execution_history.len() > self.config.max_history_size {
            self.execution_history.remove(0);
        }
    }
    
    /// Check if execution should break at current location
    pub fn should_break(&self, file: &str, line: u32) -> bool {
        if let Some(file_breakpoints) = self.breakpoints.get(file) {
            file_breakpoints.contains(&line)
        } else {
            false
        }
    }
    
    /// Handle breakpoint hit
    pub fn handle_breakpoint(&mut self, file: &str, line: u32, column: u32) {
        self.state = DebugState::Paused {
            file: file.to_string(),
            line,
            column,
        };
        
        println!("Breakpoint hit at {}:{}:{}", file, line, column);
        self.record_execution_step("breakpoint", &format!("{}:{}", file, line), Vec::new());
    }
    
    /// Push stack frame
    pub fn push_stack_frame(&mut self, frame: StackFrame) {
        if self.call_stack.len() < self.config.max_stack_depth {
            self.call_stack.push(frame);
        }
    }
    
    /// Pop stack frame
    pub fn pop_stack_frame(&mut self) -> Option<StackFrame> {
        self.call_stack.pop()
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            break_on_exceptions: true,
            break_on_function_entry: false,
            max_stack_depth: 100,
            max_history_size: 1000,
        }
    }
}

impl std::fmt::Display for DebugValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugValue::Int(i) => write!(f, "{}", i),
            DebugValue::Float(fl) => write!(f, "{}", fl),
            DebugValue::Bool(b) => write!(f, "{}", b),
            DebugValue::String(s) => write!(f, "\"{}\"", s),
            DebugValue::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            DebugValue::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, value)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
            DebugValue::Null => write!(f, "null"),
            DebugValue::Reference(val) => write!(f, "&{}", val),
        }
    }
}
