# 🧪 **AlBayan Language Test Suite**

## 📋 **Overview**

This directory contains a comprehensive test suite for the AlBayan programming language, designed to verify the complete compilation pipeline from source code to execution, as recommended by our expert advisor.

## 🗂️ **Directory Structure**

```
tests/
├── programs/           # Test programs (.ab files)
├── output/            # Compiled executables (generated)
├── run_tests.sh       # Shell script test runner
├── compiler_integration.rs  # Rust integration tests
└── README.md          # This file
```

## 🎯 **Test Categories**

### **Basic Functionality Tests**
- `01_simple_main.ab` - Simple main function returning a value
- `02_arithmetic.ab` - Basic arithmetic operations
- `03_if_else.ab` - If-else control flow
- `04_while_loop.ab` - While loop with variables
- `05_function_call.ab` - Function with parameters

### **Advanced Feature Tests**
- `06_nested_if.ab` - Nested if statements
- `07_complex_arithmetic.ab` - Complex arithmetic expressions
- `08_multiple_functions.ab` - Multiple functions calling each other
- `09_variable_scope.ab` - Variable scoping rules
- `10_comparison_operators.ab` - Comparison and logical operators

## 🚀 **Running Tests**

### **Method 1: Shell Script (Recommended)**
```bash
# Run all tests
./tests/run_tests.sh

# Make executable if needed
chmod +x tests/run_tests.sh
```

### **Method 2: Rust Integration Tests**
```bash
# Run Rust integration tests
cargo test --test compiler_integration

# Run specific test
cargo test --test compiler_integration test_simple_main

# Run all tests with output
cargo test --test compiler_integration -- --nocapture
```

### **Method 3: Individual Test**
```bash
# Compile and run a single test (when compiler is ready)
albayan compile tests/programs/01_simple_main.ab -o test_output
./test_output
```

## 📊 **Test Format**

Each test file follows this format:

```albayan
// Test N: Description
// Expected output: EXPECTED_VALUE

fn main() -> int {
    // Test code here
    return EXPECTED_VALUE;
}
```

The expected output is extracted automatically by the test runners.

## 🔧 **Test Implementation Status**

### **Current Status (Placeholder Phase)**
- ✅ Test files created with proper syntax
- ✅ Shell script test runner implemented
- ✅ Rust integration test framework ready
- ⏳ Using placeholder C compilation for now
- ⏳ Waiting for AlBayan compiler completion

### **Next Phase (Compiler Integration)**
- 🔄 Replace placeholder with actual AlBayan compiler
- 🔄 Add LLVM IR generation tests
- 🔄 Add semantic analysis error tests
- 🔄 Add performance benchmarks

## 📈 **Test Coverage**

### **Language Features Covered**
- ✅ Function declarations and calls
- ✅ Variable declarations and assignments
- ✅ Arithmetic operations (+, -, *, /, %)
- ✅ Comparison operations (<, >, <=, >=, ==, !=)
- ✅ Logical operations (&&, ||)
- ✅ If-else statements
- ✅ While loops
- ✅ Variable scoping
- ✅ Return statements

### **Planned Coverage**
- 🔄 Struct declarations and usage
- 🔄 Tuple types
- 🔄 Array operations
- 🔄 String operations
- 🔄 Error handling
- 🔄 Module system
- 🔄 Logic programming features

## 🎯 **Expert Recommendations Implemented**

This test suite implements the expert's key recommendations:

1. **Complete Pipeline Testing**: Each test verifies the full path from source to execution
2. **Automated Verification**: Scripts automatically compile, run, and verify outputs
3. **Comprehensive Coverage**: Tests cover all basic language features
4. **Easy Expansion**: Simple format for adding new tests
5. **Multiple Test Methods**: Both shell and Rust-based testing

## 📝 **Adding New Tests**

To add a new test:

1. **Create test file**: `tests/programs/NN_test_name.ab`
2. **Add expected output comment**: `// Expected output: VALUE`
3. **Write test code**: Implement the test logic
4. **Run tests**: Execute `./tests/run_tests.sh` to verify

Example:
```albayan
// Test 11: String operations
// Expected output: Hello

fn main() -> string {
    let greeting: string = "Hello";
    return greeting;
}
```

## 🔍 **Debugging Failed Tests**

When a test fails:

1. **Check compilation**: Verify the source compiles without errors
2. **Check output**: Compare expected vs actual output
3. **Check logic**: Verify the test logic is correct
4. **Check environment**: Ensure all dependencies are available

## 📊 **Test Results Format**

The test runners provide colored output:
- 🟢 **[PASS]** - Test passed successfully
- 🔴 **[FAIL]** - Test failed (compilation or output mismatch)
- 🟡 **[WARN]** - Warning (missing expected output, etc.)
- 🔵 **[INFO]** - Informational message

## 🎯 **Future Enhancements**

1. **Performance Tests**: Benchmark compilation and execution times
2. **Memory Tests**: Verify memory usage and leak detection
3. **Error Tests**: Test error handling and reporting
4. **Stress Tests**: Large programs and edge cases
5. **Cross-Platform**: Test on different operating systems

---

**🧬 This test suite ensures AlBayan language quality and reliability! 🚀**
