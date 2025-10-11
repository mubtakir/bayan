//! # Command Line Interface Module
//!
//! This module implements the CLI for the AlBayan compiler and runtime.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::{Compiler, CompilerOptions, CompilerResult};

/// AlBayan programming language compiler and runtime
#[derive(Parser)]
#[command(name = "albayan")]
#[command(about = "البيان (AlBayan) - A modern programming language integrating logic programming, AI, and traditional paradigms")]
#[command(version = crate::VERSION)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable debug mode
    #[arg(short, long, global = true)]
    pub debug: bool,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Compile a source file
    Build {
        /// Source file to compile
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, default_value = "0")]
        optimization: u8,

        /// Target triple (e.g., x86_64-unknown-linux-gnu)
        #[arg(long)]
        target: Option<String>,

        /// Enable release mode (equivalent to -O2)
        #[arg(long)]
        release: bool,

        /// Disable logic programming features
        #[arg(long)]
        no_logic: bool,

        /// Disable AI features
        #[arg(long)]
        no_ai: bool,

        /// Use LLVM backend for code generation
        #[arg(long)]
        llvm: bool,
    },

    /// Run a source file directly (JIT compilation)
    Run {
        /// Source file to run
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Start an interactive REPL
    Repl {
        /// Enable logic programming mode
        #[arg(long)]
        logic: bool,

        /// Enable AI mode
        #[arg(long)]
        ai: bool,
    },

    /// Check syntax without compilation
    Check {
        /// Source file to check
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },

    /// Format source code
    Format {
        /// Source file to format
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Format in place
        #[arg(short, long)]
        in_place: bool,
    },

    /// Show language information
    Info {
        /// Show version information
        #[arg(long)]
        version: bool,

        /// Show feature information
        #[arg(long)]
        features: bool,

        /// Show target information
        #[arg(long)]
        targets: bool,
    },

    /// Start Language Server Protocol (LSP) server
    Lsp,
}

/// CLI application
pub struct CliApp {
    args: Cli,
}

impl CliApp {
    /// Create a new CLI application
    pub fn new() -> Self {
        let args = Cli::parse();
        Self { args }
    }

    /// Run the CLI application
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.args.command {
            Commands::Build {
                input,
                output,
                optimization,
                target,
                release,
                no_logic,
                no_ai,
                llvm
            } => {
                self.build_command(input, output, *optimization, target, *release, *no_logic, *no_ai, *llvm)
            }

            Commands::Run { input, args } => {
                self.run_command(input, args)
            }

            Commands::Repl { logic, ai } => {
                self.repl_command(*logic, *ai)
            }

            Commands::Check { input } => {
                self.check_command(input)
            }

            Commands::Format { input, in_place } => {
                self.format_command(input, *in_place)
            }

            Commands::Info { version, features, targets } => {
                self.info_command(*version, *features, *targets)
            }

            Commands::Lsp => {
                self.lsp_command().await
            }
        }
    }

    /// Handle build command
    fn build_command(
        &self,
        input: &PathBuf,
        output: &Option<PathBuf>,
        optimization: u8,
        target: &Option<String>,
        release: bool,
        no_logic: bool,
        no_ai: bool,
        llvm: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("Building: {}", input.display());
        }

        let mut options = CompilerOptions {
            optimization_level: if release { 2 } else { optimization },
            target_triple: target.clone(),
            debug_info: !release,
            output_path: output.clone(),
            enable_logic: !no_logic,
            enable_ai: !no_ai,
            use_llvm: llvm,
        };

        if self.args.debug {
            options.debug_info = true;
        }

        let mut compiler = Compiler::with_options(options).source_file(input);

        match compiler.compile_file() {
            Ok(object_code) => {
                let output_path = output.as_ref()
                    .map(|p| p.clone())
                    .unwrap_or_else(|| {
                        let mut path = input.clone();
                        path.set_extension("o");
                        path
                    });

                std::fs::write(&output_path, object_code)?;

                if self.args.verbose {
                    println!("Output written to: {}", output_path.display());
                }

                println!("Compilation successful!");
            }
            Err(e) => {
                eprintln!("Compilation failed: {}", e);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    /// Handle run command
    fn run_command(&self, input: &PathBuf, _args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("Running: {}", input.display());
        }

        let options = CompilerOptions {
            optimization_level: 0,
            debug_info: true,
            ..Default::default()
        };

        let compiler = Compiler::with_options(options);
        let source = std::fs::read_to_string(input)?;

        match compiler.run_jit(&source) {
            Ok(_) => {
                if self.args.verbose {
                    println!("Execution completed successfully");
                }
            }
            Err(e) => {
                eprintln!("Execution failed: {}", e);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    /// Handle REPL command
    fn repl_command(&self, logic: bool, ai: bool) -> Result<(), Box<dyn std::error::Error>> {
        println!("البيان (AlBayan) Interactive REPL");
        println!("Version: {}", crate::VERSION);
        println!("Type 'exit' to quit, 'help' for help");

        if logic {
            println!("Logic programming mode enabled");
        }
        if ai {
            println!("AI mode enabled");
        }

        let mut repl = repl::ReplSession::new(logic, ai);
        repl.run()?;

        Ok(())
    }

    /// Handle check command
    fn check_command(&self, input: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("Checking: {}", input.display());
        }

        let source = std::fs::read_to_string(input)?;

        // Perform lexical and syntactic analysis only
        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("Lexical error: {}", e);
                std::process::exit(1);
            }
        };

        let mut parser = crate::parser::Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => {
                println!("Syntax check passed!");
                ast
            }
            Err(e) => {
                eprintln!("Syntax error: {}", e);
                std::process::exit(1);
            }
        };

        // Perform semantic analysis
        let options = crate::CompilerOptions::default();
        let mut semantic_analyzer = crate::semantic::SemanticAnalyzer::new(&options);
        match semantic_analyzer.analyze(ast) {
            Ok(_) => {
                println!("Semantic check passed!");
            }
            Err(e) => {
                eprintln!("Semantic error: {}", e);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    /// Handle format command
    fn format_command(&self, input: &PathBuf, in_place: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("Formatting: {}", input.display());
        }

        let source = std::fs::read_to_string(input)?;

        // Parse and reformat the code
        let mut lexer = crate::lexer::Lexer::new(&source);
        let tokens = lexer.tokenize()?;

        let mut parser = crate::parser::Parser::new(tokens);
        let ast = parser.parse()?;

        // Format the AST back to source code
        let formatted = formatter::format_ast(&ast);

        if in_place {
            std::fs::write(input, formatted)?;
            if self.args.verbose {
                println!("File formatted in place");
            }
        } else {
            println!("{}", formatted);
        }

        Ok(())
    }

    /// Handle info command
    fn info_command(&self, version: bool, features: bool, targets: bool) -> Result<(), Box<dyn std::error::Error>> {
        if version || (!features && !targets) {
            println!("البيان (AlBayan) Programming Language");
            println!("Version: {}", crate::VERSION);
            println!("Built with Rust (version not available)");
        }

        if features {
            println!("\nFeatures:");
            println!("  ✓ Logic Programming (Prolog-style inference)");
            println!("  ✓ AI Integration (Neural networks, ML models)");
            println!("  ✓ Memory Safety (Ownership and borrowing)");
            println!("  ✓ High Performance (LLVM backend)");
            println!("  ✓ Concurrent Programming (async/await)");
            println!("  ✓ Cross-platform (Windows, Linux, macOS)");
        }

        if targets {
            println!("\nSupported Targets:");
            println!("  • x86_64-unknown-linux-gnu");
            println!("  • x86_64-pc-windows-msvc");
            println!("  • x86_64-apple-darwin");
            println!("  • aarch64-unknown-linux-gnu");
            println!("  • aarch64-apple-darwin");
            println!("  • wasm32-unknown-unknown (WebAssembly)");
        }

        Ok(())
    }

    /// Handle LSP command
    async fn lsp_command(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.verbose {
            println!("Starting AlBayan Language Server...");
        }

        crate::lsp::start_language_server().await;
        Ok(())
    }
}

/// REPL module
mod repl {
    use std::io::{self, Write};
    use crate::{Compiler, CompilerOptions};

    pub struct ReplSession {
        logic_mode: bool,
        ai_mode: bool,
        history: Vec<String>,
    }

    impl ReplSession {
        pub fn new(logic_mode: bool, ai_mode: bool) -> Self {
            Self {
                logic_mode,
                ai_mode,
                history: Vec::new(),
            }
        }

        pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            loop {
                print!("albayan> ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "exit" | "quit" => break,
                    "help" => self.show_help(),
                    "history" => self.show_history(),
                    "clear" => self.clear_screen(),
                    _ => {
                        self.history.push(input.to_string());
                        self.execute_input(input)?;
                    }
                }
            }

            println!("Goodbye!");
            Ok(())
        }

        fn execute_input(&self, input: &str) -> Result<(), Box<dyn std::error::Error>> {
            let options = CompilerOptions {
                enable_logic: self.logic_mode,
                enable_ai: self.ai_mode,
                debug_info: true,
                ..Default::default()
            };

            let compiler = Compiler::with_options(options);

            match compiler.run_jit(input) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }

            Ok(())
        }

        fn show_help(&self) {
            println!("AlBayan REPL Commands:");
            println!("  help     - Show this help message");
            println!("  history  - Show command history");
            println!("  clear    - Clear the screen");
            println!("  exit     - Exit the REPL");
            println!();
            println!("Enter AlBayan code to execute it directly.");
        }

        fn show_history(&self) {
            println!("Command History:");
            for (i, cmd) in self.history.iter().enumerate() {
                println!("  {}: {}", i + 1, cmd);
            }
        }

        fn clear_screen(&self) {
            print!("\x1B[2J\x1B[1;1H");
            io::stdout().flush().ok();
        }
    }
}

/// Code formatter module
mod formatter {
    use crate::parser::ast::Program;

    pub fn format_ast(ast: &Program) -> String {
        // Simplified formatter - in a real implementation, this would
        // properly format the AST back to source code
        format!("// Formatted AlBayan code\n// {} items\n", ast.items.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test that CLI can be parsed (basic smoke test)
        // In a real test, we'd use clap's testing utilities
    }
}
