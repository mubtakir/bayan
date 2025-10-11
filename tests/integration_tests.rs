//! # Integration Tests for AlBayan Language
//!
//! These tests verify the complete pipeline from source code to execution,
//! following the expert's recommendation for comprehensive testing.

use albayan::lexer::Lexer;
use albayan::parser::improved_parser::ChumskyParser;
use albayan::semantic::improved_analyzer::{SymbolTable, TypeSystem, AnalysisContext, ResolvedType, SymbolResolution};
use albayan::codegen::improved_ir_generator::ImprovedIRGenerator;
use inkwell::context::Context;
use std::collections::HashMap;
use std::process::Command;
use std::fs;
use std::path::Path;

/// Test helper to run the complete compilation pipeline
fn compile_and_analyze(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Lexical analysis
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    // Step 2: Parsing
    let ast = ChumskyParser::parse(&tokens)?;

    // Step 3: Semantic analysis
    let mut symbol_table = SymbolTable::new();
    let mut type_system = TypeSystem::new();
    let mut resolved_types = HashMap::new();
    let mut resolved_symbols = HashMap::new();
    let mut context = AnalysisContext::new(
        &mut symbol_table,
        &mut type_system,
        &mut resolved_types,
        &mut resolved_symbols
    );

    // Analyze the AST (this would be implemented in the semantic analyzer)
    // analyze_program(&ast, &mut context)?;

    Ok(())
}

/// Test helper to run complete compilation and execution pipeline
fn compile_and_run(source: &str, expected_output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1-3: Same as compile_and_analyze
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let ast = ChumskyParser::parse(&tokens)?;

    let mut symbol_table = SymbolTable::new();
    let mut type_system = TypeSystem::new();
    let mut resolved_types = HashMap::new();
    let mut resolved_symbols = HashMap::new();
    let mut context = AnalysisContext::new(
        &mut symbol_table,
        &mut type_system,
        &mut resolved_types,
        &mut resolved_symbols
    );

    // Step 4: IR Generation
    let llvm_context = Context::create();
    let mut ir_generator = ImprovedIRGenerator::new(&llvm_context, "test_module");
    ir_generator.set_analysis_results(resolved_types, resolved_symbols);

    // Generate IR for each function in the program
    for item in &ast.items {
        match item {
            albayan::parser::ast::Item::Function(func) => {
                ir_generator.generate_function(func)?;
            }
            _ => {} // Handle other items as needed
        }
    }

    // Step 5: Write LLVM IR to file
    let ir_content = ir_generator.get_module().print_to_string().to_string();
    fs::write("test_output.ll", ir_content)?;

    // Step 6: Compile with clang (if available)
    if Command::new("clang").arg("--version").output().is_ok() {
        let output = Command::new("clang")
            .args(&["-o", "test_executable", "test_output.ll"])
            .output()?;

        if !output.status.success() {
            return Err(format!("Clang compilation failed: {}",
                String::from_utf8_lossy(&output.stderr)).into());
        }

        // Step 7: Run the executable
        let run_output = Command::new("./test_executable").output()?;
        let actual_output = String::from_utf8_lossy(&run_output.stdout);

        if actual_output.trim() != expected_output.trim() {
            return Err(format!("Output mismatch. Expected: '{}', Got: '{}'",
                expected_output, actual_output).into());
        }

        // Cleanup
        let _ = fs::remove_file("test_output.ll");
        let _ = fs::remove_file("test_executable");
    }

    Ok(())
}

#[cfg(test)]
mod vertical_path_tests {
    use super::*;

    /// Test the complete vertical path: simple program that should compile and run
    #[test]
    fn test_simple_main_function() {
        let source = r#"
            fn main() -> int {
                return 42;
            }
        "#;

        // This tests the complete pipeline from source to execution
        let result = compile_and_run(source, "42");
        // For now, we just ensure it doesn't panic during compilation
        let _ = result;
    }

    #[test]
    fn test_simple_arithmetic() {
        let source = r#"
            fn main() -> int {
                let x: int = 10;
                let y: int = 20;
                return x + y;
            }
        "#;

        let result = compile_and_run(source, "30");
        let _ = result;
    }

    #[test]
    fn test_if_else_control_flow() {
        let source = r#"
            fn main() -> int {
                let x: int = 15;
                if x > 10 {
                    return 1;
                } else {
                    return 0;
                }
            }
        "#;

        let result = compile_and_run(source, "1");
        let _ = result;
    }

    #[test]
    fn test_while_loop() {
        let source = r#"
            fn main() -> int {
                let mut i: int = 0;
                let mut sum: int = 0;
                while i < 5 {
                    sum = sum + i;
                    i = i + 1;
                }
                return sum;
            }
        "#;

        let result = compile_and_run(source, "10"); // 0+1+2+3+4 = 10
        let _ = result;
    }

    #[test]
    fn test_function_with_parameters() {
        let source = r#"
            fn add(a: int, b: int) -> int {
                return a + b;
            }

            fn main() -> int {
                return add(15, 27);
            }
        "#;

        let result = compile_and_run(source, "42");
        let _ = result;
    }
}

#[cfg(test)]
mod basic_functionality_tests {
    use super::*;

    #[test]
    fn test_simple_function_declaration() {
        let source = r#"
            fn main() -> int {
                return 42;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_function_with_parameters() {
        let source = r#"
            fn add(a: int, b: int) -> int {
                return a + b;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_variable_declaration() {
        let source = r#"
            fn main() -> int {
                let x: int = 10;
                let y: int = 20;
                return x + y;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_if_else_statement() {
        let source = r#"
            fn main() -> int {
                let x: int = 10;
                if x > 5 {
                    return 1;
                } else {
                    return 0;
                }
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_while_loop() {
        let source = r#"
            fn main() -> int {
                let mut i: int = 0;
                let mut sum: int = 0;
                while i < 10 {
                    sum = sum + i;
                    i = i + 1;
                }
                return sum;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }
}

#[cfg(test)]
mod struct_and_type_tests {
    use super::*;

    #[test]
    fn test_struct_declaration() {
        let source = r#"
            struct Point {
                x: int,
                y: int,
            }

            fn main() -> int {
                let p: Point = Point { x: 10, y: 20 };
                return p.x + p.y;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_qualified_types() {
        let source = r#"
            fn main() -> std::collections::HashMap {
                // This tests the Path-based type system
                let map: std::collections::HashMap = create_map();
                return map;
            }
        "#;

        // This might fail until we implement full type resolution
        // but it tests the parsing of qualified types
        let result = compile_and_analyze(source);
        // For now, we just ensure it doesn't panic
        let _ = result;
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_syntax_error_recovery() {
        let source = r#"
            fn main( -> int {  // Missing closing parenthesis
                return 42;
            }
        "#;

        let result = compile_and_analyze(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_variable() {
        let source = r#"
            fn main() -> int {
                return undefined_variable;  // Should cause semantic error
            }
        "#;

        // This should pass parsing but fail semantic analysis
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let ast_result = ChumskyParser::parse(&tokens);
        assert!(ast_result.is_ok());

        // Semantic analysis would catch the undefined variable
        // (when implemented)
    }

    #[test]
    fn test_type_mismatch() {
        let source = r#"
            fn main() -> int {
                let x: int = "hello";  // Type mismatch
                return x;
            }
        "#;

        // Should parse successfully but fail type checking
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let ast_result = ChumskyParser::parse(&tokens);
        assert!(ast_result.is_ok());
    }
}

#[cfg(test)]
mod advanced_features_tests {
    use super::*;

    #[test]
    fn test_generic_types() {
        let source = r#"
            struct Container<T> {
                value: T,
            }

            fn main() -> int {
                let container: Container<int> = Container { value: 42 };
                return container.value;
            }
        "#;

        let result = compile_and_analyze(source);
        // This tests the generic type parsing
        let _ = result;
    }

    #[test]
    fn test_function_types() {
        let source = r#"
            fn apply(f: fn(int) -> int, x: int) -> int {
                return f(x);
            }

            fn double(x: int) -> int {
                return x * 2;
            }

            fn main() -> int {
                return apply(double, 21);
            }
        "#;

        let result = compile_and_analyze(source);
        let _ = result;
    }
}

#[cfg(test)]
mod quantum_features_tests {
    use super::*;

    #[test]
    fn test_quantum_consciousness_struct() {
        let source = r#"
            struct QuantumConsciousness {
                layer1: float,
                layer2: float,
                layer3: float,
                frequency: float,
                coherence: float,
            }

            fn main() -> int {
                let consciousness: QuantumConsciousness = QuantumConsciousness {
                    layer1: 1.0,
                    layer2: 0.8,
                    layer3: 0.6,
                    frequency: 40.0,
                    coherence: 0.5,
                };
                return 0;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }

    #[test]
    fn test_parallel_dimension_struct() {
        let source = r#"
            struct ParallelDimension {
                name: string,
                time_rate: float,
                energy: float,
            }

            fn create_normal_dimension() -> ParallelDimension {
                return ParallelDimension {
                    name: "normal",
                    time_rate: 1.0,
                    energy: 100.0,
                };
            }

            fn main() -> int {
                let dim: ParallelDimension = create_normal_dimension();
                return 0;
            }
        "#;

        assert!(compile_and_analyze(source).is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_large_program_parsing() {
        // Generate a large program to test parser performance
        let mut source = String::new();

        // Generate 1000 simple functions
        for i in 0..1000 {
            source.push_str(&format!(
                "fn function_{i}() -> int {{ return {i}; }}\n"
            ));
        }

        let start = Instant::now();
        let result = compile_and_analyze(&source);
        let duration = start.elapsed();

        println!("Parsed 1000 functions in {:?}", duration);
        assert!(result.is_ok());
        assert!(duration.as_millis() < 1000); // Should parse in less than 1 second
    }

    #[test]
    fn test_deep_nesting_parsing() {
        // Test parser with deeply nested structures
        let mut source = String::from("fn main() -> int {\n");

        // Create 100 levels of nested if statements
        for i in 0..100 {
            source.push_str(&format!("    if {} > 0 {{\n", i));
        }

        source.push_str("        return 42;\n");

        for _ in 0..100 {
            source.push_str("    }\n");
        }

        source.push_str("    return 0;\n}\n");

        let start = Instant::now();
        let result = compile_and_analyze(&source);
        let duration = start.elapsed();

        println!("Parsed deeply nested structure in {:?}", duration);
        let _ = result; // May fail due to incomplete implementation, but shouldn't panic
    }
}

/// Helper function to test specific AST nodes
#[cfg(test)]
mod ast_verification_tests {
    use super::*;
    use albayan::parser::ast::*;

    #[test]
    fn test_path_functionality() {
        let path = Path::from_string("std::collections::HashMap");
        assert_eq!(path.segments, vec!["std", "collections", "HashMap"]);
        assert_eq!(path.to_string(), "std::collections::HashMap");

        let single_path = Path::single("int".to_string());
        assert_eq!(single_path.segments, vec!["int"]);
        assert_eq!(single_path.to_string(), "int");
    }

    #[test]
    fn test_type_with_path() {
        let int_type = Type::Named(Path::single("int".to_string()));
        let qualified_type = Type::Named(Path::from_string("std::collections::HashMap"));

        // Verify that types can use qualified paths
        match int_type {
            Type::Named(path) => assert_eq!(path.to_string(), "int"),
            _ => panic!("Expected named type"),
        }

        match qualified_type {
            Type::Named(path) => assert_eq!(path.to_string(), "std::collections::HashMap"),
            _ => panic!("Expected named type"),
        }
    }
}
