//! # Runtime Module
//!
//! This module implements the runtime system for the AlBayan programming language.
//! It provides the logic engine, AI support, memory management, and system integration.

pub mod logic_engine;
pub mod memory;
pub mod ai_support;
pub mod system_interface;
pub mod dynamic_types;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub use logic_engine::LogicEngine;
pub use dynamic_types::{AlbayanValue, AlbayanList, AlbayanValueTag};

/// Main runtime system for AlBayan
pub struct Runtime {
    /// Logic programming engine
    logic_engine: Arc<Mutex<LogicEngine>>,

    /// Memory allocator
    memory_manager: Arc<Mutex<memory::MemoryManager>>,

    /// AI inference engine
    ai_engine: Option<Arc<Mutex<ai_support::AIEngine>>>,

    /// System interface for I/O operations
    system_interface: Arc<system_interface::SystemInterface>,

    /// Runtime configuration
    config: RuntimeConfig,
}

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Enable logic programming features
    pub enable_logic: bool,

    /// Enable AI features
    pub enable_ai: bool,

    /// Maximum memory allocation (in bytes)
    pub max_memory: usize,

    /// Enable garbage collection
    pub enable_gc: bool,

    /// Debug mode
    pub debug_mode: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            enable_logic: true,
            enable_ai: true,
            max_memory: 1024 * 1024 * 1024, // 1GB
            enable_gc: true,
            debug_mode: false,
        }
    }
}

impl Runtime {
    /// Create a new runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create a new runtime with custom configuration
    pub fn with_config(config: RuntimeConfig) -> Self {
        let logic_engine = Arc::new(Mutex::new(LogicEngine::new()));
        let memory_manager = Arc::new(Mutex::new(memory::MemoryManager::new(config.max_memory)));
        let system_interface = Arc::new(system_interface::SystemInterface::new());

        let ai_engine = if config.enable_ai {
            Some(Arc::new(Mutex::new(ai_support::AIEngine::new())))
        } else {
            None
        };

        Self {
            logic_engine,
            memory_manager,
            ai_engine,
            system_interface,
            config,
        }
    }

    /// Initialize the runtime
    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        if self.config.debug_mode {
            println!("Initializing AlBayan Runtime...");
        }

        // Initialize logic engine
        if self.config.enable_logic {
            let mut logic_engine = self.logic_engine.lock().unwrap();
            logic_engine.initialize()?;
        }

        // Initialize AI engine
        if let Some(ai_engine) = &self.ai_engine {
            let mut ai_engine = ai_engine.lock().unwrap();
            ai_engine.initialize()?;
        }

        if self.config.debug_mode {
            println!("Runtime initialized successfully");
        }

        Ok(())
    }

    /// Shutdown the runtime
    pub fn shutdown(&mut self) -> Result<(), RuntimeError> {
        if self.config.debug_mode {
            println!("Shutting down AlBayan Runtime...");
        }

        // Shutdown AI engine
        if let Some(ai_engine) = &self.ai_engine {
            let mut ai_engine = ai_engine.lock().unwrap();
            ai_engine.shutdown()?;
        }

        // Shutdown logic engine
        if self.config.enable_logic {
            let mut logic_engine = self.logic_engine.lock().unwrap();
            logic_engine.shutdown()?;
        }

        // Clean up memory
        let mut memory_manager = self.memory_manager.lock().unwrap();
        memory_manager.cleanup()?;

        if self.config.debug_mode {
            println!("Runtime shutdown complete");
        }

        Ok(())
    }

    /// Get the logic engine
    pub fn logic_engine(&self) -> Arc<Mutex<LogicEngine>> {
        self.logic_engine.clone()
    }

    /// Get the memory manager
    pub fn memory_manager(&self) -> Arc<Mutex<memory::MemoryManager>> {
        self.memory_manager.clone()
    }

    /// Get the AI engine (if enabled)
    pub fn ai_engine(&self) -> Option<Arc<Mutex<ai_support::AIEngine>>> {
        self.ai_engine.clone()
    }

    /// Get the system interface
    pub fn system_interface(&self) -> Arc<system_interface::SystemInterface> {
        self.system_interface.clone()
    }

    /// Execute a logic query
    pub fn query_solve(&self, query: &str) -> Result<Vec<HashMap<String, String>>, RuntimeError> {
        if !self.config.enable_logic {
            return Err(RuntimeError::FeatureDisabled("Logic programming".to_string()));
        }

        let mut logic_engine = self.logic_engine.lock().unwrap();
        logic_engine.solve_query(query)
    }

    /// Assert a fact into the knowledge base
    pub fn assert_fact(&self, fact: &str) -> Result<(), RuntimeError> {
        if !self.config.enable_logic {
            return Err(RuntimeError::FeatureDisabled("Logic programming".to_string()));
        }

        let mut logic_engine = self.logic_engine.lock().unwrap();
        logic_engine.assert_fact(fact)
    }

    /// Retract a fact from the knowledge base
    pub fn retract_fact(&self, fact: &str) -> Result<(), RuntimeError> {
        if !self.config.enable_logic {
            return Err(RuntimeError::FeatureDisabled("Logic programming".to_string()));
        }

        let mut logic_engine = self.logic_engine.lock().unwrap();
        logic_engine.retract_fact(fact)
    }

    /// Print a string (runtime function called from generated code)
    pub fn print_string(&self, s: &str) -> Result<(), RuntimeError> {
        self.system_interface.print(s)
    }

    /// Allocate memory
    pub fn allocate(&self, size: usize) -> Result<*mut u8, RuntimeError> {
        let mut memory_manager = self.memory_manager.lock().unwrap();
        memory_manager.allocate(size)
    }

    /// Deallocate memory
    pub fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<(), RuntimeError> {
        let mut memory_manager = self.memory_manager.lock().unwrap();
        memory_manager.deallocate(ptr, size)
    }

    /// Run garbage collection
    pub fn garbage_collect(&self) -> Result<(), RuntimeError> {
        if !self.config.enable_gc {
            return Ok(());
        }

        let mut memory_manager = self.memory_manager.lock().unwrap();
        memory_manager.garbage_collect()
    }

    /// Get runtime statistics
    pub fn get_stats(&self) -> RuntimeStats {
        let memory_manager = self.memory_manager.lock().unwrap();
        let logic_engine = self.logic_engine.lock().unwrap();

        RuntimeStats {
            memory_allocated: memory_manager.total_allocated(),
            memory_peak: memory_manager.peak_allocated(),
            facts_count: logic_engine.facts_count(),
            rules_count: logic_engine.rules_count(),
            queries_executed: logic_engine.queries_executed(),
        }
    }
}

/// Runtime statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub memory_allocated: usize,
    pub memory_peak: usize,
    pub facts_count: usize,
    pub rules_count: usize,
    pub queries_executed: usize,
}

/// Runtime errors
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("Logic engine error: {0}")]
    LogicError(String),

    #[error("Memory error: {0}")]
    MemoryError(String),

    #[error("AI engine error: {0}")]
    AIError(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("Feature disabled: {0}")]
    FeatureDisabled(String),

    #[error("Runtime not initialized")]
    NotInitialized,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

// C-compatible runtime functions for LLVM-generated code
extern "C" {
    // These will be implemented in the runtime library
}

/// C-compatible runtime functions
#[no_mangle]
pub extern "C" fn albayan_rt_print_string(ptr: *const u8, len: usize) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        if let Ok(s) = std::str::from_utf8(slice) {
            print!("{}", s);
        }
    }
}

#[no_mangle]
pub extern "C" fn albayan_rt_alloc(size: usize) -> *mut u8 {
    // Simple allocation using system allocator
    // In a real implementation, this would use the runtime's memory manager
    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

#[no_mangle]
pub extern "C" fn albayan_rt_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }

    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) }
}

#[no_mangle]
pub extern "C" fn albayan_rt_assert_fact(ptr: *const u8, len: usize) -> i32 {
    // Placeholder implementation
    // In a real implementation, this would interact with the logic engine
    0 // Success
}

#[no_mangle]
pub extern "C" fn albayan_rt_query_solve(ptr: *const u8, len: usize) -> i32 {
    // Placeholder implementation
    // In a real implementation, this would execute the query and return results
    0 // Success
}

/// Global runtime instance (for C functions)
static mut GLOBAL_RUNTIME: Option<Runtime> = None;
static RUNTIME_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global runtime
pub fn init_global_runtime() -> Result<(), RuntimeError> {
    RUNTIME_INIT.call_once(|| {
        unsafe {
            GLOBAL_RUNTIME = Some(Runtime::new());
            if let Some(ref mut runtime) = GLOBAL_RUNTIME {
                runtime.initialize().unwrap();
            }
        }
    });
    Ok(())
}

/// Get the global runtime
pub fn get_global_runtime() -> Option<&'static Runtime> {
    unsafe { GLOBAL_RUNTIME.as_ref() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new();
        assert!(runtime.config.enable_logic);
        assert!(runtime.config.enable_ai);
    }

    #[test]
    fn test_runtime_initialization() {
        let mut runtime = Runtime::new();
        let result = runtime.initialize();
        assert!(result.is_ok());

        let shutdown_result = runtime.shutdown();
        assert!(shutdown_result.is_ok());
    }

    #[test]
    fn test_runtime_config() {
        let config = RuntimeConfig {
            enable_logic: false,
            enable_ai: false,
            max_memory: 1024,
            enable_gc: false,
            debug_mode: true,
        };

        let runtime = Runtime::with_config(config.clone());
        assert_eq!(runtime.config.enable_logic, false);
        assert_eq!(runtime.config.max_memory, 1024);
    }
}
