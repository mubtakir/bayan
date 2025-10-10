//! # البيان (AlBayan) Programming Language
//!
//! A modern programming language that integrates logic programming, AI capabilities,
//! and traditional programming paradigms with high performance and memory safety.
//!
//! ## Architecture
//!
//! The compiler is structured in several phases:
//! 1. **Lexical Analysis** - Convert source text into tokens
//! 2. **Parsing** - Build Abstract Syntax Tree (AST) from tokens
//! 3. **Semantic Analysis** - Type checking, scope resolution, ownership analysis
//! 4. **Code Generation** - Generate LLVM IR
//! 5. **Runtime** - Provide logic engine, AI support, and system integration
//!
//! ## Modules

pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod codegen;
pub mod runtime;
pub mod cli;
pub mod modules;
pub mod tools;
pub mod lsp;
pub mod ai;
pub mod builtin_libraries;

// Re-export commonly used types
pub use lexer::{Token, TokenType, Lexer};
pub use parser::Parser;
pub use semantic::{SemanticAnalyzer, TypeChecker};
pub use codegen::{CodeGenerator};
pub use runtime::{Runtime, LogicEngine};
pub use builtin_libraries::{
    BuiltinLibraryManager, ThinkingCore, ExpertExplorer, GeneralShapeEquation,
    LayerType, OperationMode, AdaptationType, quick_access,
    ArtisticRenderer, ShapeInference, BasicShape, ShapeProperty, RenderedImage
};

/// Main compiler error type
#[derive(Debug, thiserror::Error)]
pub enum CompilerError {
    #[error("Lexical error: {0}")]
    LexicalError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Semantic error: {0}")]
    SemanticError(String),

    #[error("Code generation error: {0}")]
    CodeGenError(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Result type for compiler operations
pub type CompilerResult<T> = Result<T, CompilerError>;

/// Main compiler struct that orchestrates the compilation process
pub struct Compiler {
    /// Source file path
    pub source_path: Option<std::path::PathBuf>,
    /// Compilation options
    pub options: CompilerOptions,
}

/// Compiler configuration options
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Optimization level (0-3)
    pub optimization_level: u8,
    /// Target triple (e.g., "x86_64-unknown-linux-gnu")
    pub target_triple: Option<String>,
    /// Enable debug information
    pub debug_info: bool,
    /// Output file path
    pub output_path: Option<std::path::PathBuf>,
    /// Enable logic programming features
    pub enable_logic: bool,
    /// Enable AI features
    pub enable_ai: bool,
    /// Use LLVM backend for code generation
    pub use_llvm: bool,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            optimization_level: 0,
            target_triple: None,
            debug_info: true,
            output_path: None,
            enable_logic: true,
            enable_ai: true,
            use_llvm: false,
        }
    }
}

impl Compiler {
    /// Create a new compiler instance
    pub fn new() -> Self {
        Self {
            source_path: None,
            options: CompilerOptions::default(),
        }
    }

    /// Create a compiler with custom options
    pub fn with_options(options: CompilerOptions) -> Self {
        Self {
            source_path: None,
            options,
        }
    }

    /// Set the source file path
    pub fn source_file<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.source_path = Some(path.into());
        self
    }

    /// Compile a source string directly
    pub fn compile_string(&self, source: &str) -> CompilerResult<Vec<u8>> {
        // Phase 1: Lexical Analysis
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()
            .map_err(|e| CompilerError::LexicalError(e.to_string()))?;

        // Phase 2: Parsing
        let mut parser = Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| CompilerError::ParseError(e.to_string()))?;

        // Phase 3: Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new(&self.options);
        let analyzed_ast = analyzer.analyze(ast)
            .map_err(|e| CompilerError::SemanticError(e.to_string()))?;

        // Phase 4: Code Generation
        let mut codegen = codegen::SimpleCodeGenerator::new(&self.options);
        let object_code = codegen.generate(analyzed_ast)
            .map_err(|e| CompilerError::CodeGenError(e.to_string()))?;

        Ok(object_code)
    }

    /// Compile a source file
    pub fn compile_file(&mut self) -> CompilerResult<Vec<u8>> {
        let source_path = self.source_path.as_ref()
            .ok_or_else(|| CompilerError::SemanticError("No source file specified".to_string()))?;

        let source = std::fs::read_to_string(source_path)?;
        self.compile_string(&source)
    }

    /// Run the compiler in JIT mode (for REPL and quick execution)
    pub fn run_jit(&self, source: &str) -> CompilerResult<()> {
        // For now, just compile and return success
        // In a real implementation, this would use LLVM JIT
        let _object_code = self.compile_string(source)?;
        println!("JIT execution completed (placeholder)");
        Ok(())
    }
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const LANGUAGE_NAME: &str = "البيان (AlBayan)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new();
        assert!(compiler.source_path.is_none());
        assert_eq!(compiler.options.optimization_level, 0);
    }

    #[test]
    fn test_compiler_with_options() {
        let options = CompilerOptions {
            optimization_level: 2,
            debug_info: false,
            ..Default::default()
        };
        let compiler = Compiler::with_options(options);
        assert_eq!(compiler.options.optimization_level, 2);
        assert!(!compiler.options.debug_info);
    }
}
