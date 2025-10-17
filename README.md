# AlBayan Programming Language — Semantic Computing with Embedded AI

Preface / تمهيد
- All conceptual foundations and innovative engines of the language are by the researcher: Basil Yahya Abdullah.
- تم تنفيذ الأفكار وتطويرها وتطوير اللغة بشكل كبير من قبل نموذجين في الذكاء الاصطناعي: أحدهما كان يلعب دور الخبير الموجّه والآخر دور المنفّذ العبقري.

Quick links:
- Arabic User Guide: docs/USER_GUIDE_AR.md
- Agent Briefing (technical vision): docs/AGENT_INTELLIGENT_MODEL_BRIEFING.md

Minimal parser basics (illustrative):
```rust
use crate::lexer::Lexer;
use crate::parser::Parser;

let source = "fn main() { return 42; }";
let tokens = Lexer::new(source).tokenize().unwrap();
let mut parser = Parser::new(tokens);
let ast = parser.parse().unwrap();
assert!(!ast.items.is_empty());
```

Minimal AlBayan program:
```albayan
fn main() -> int {
    print("Hello AlBayan!");
    return 0;
}
```

---

<!-- Arabic original README follows below. English summary is provided above. -->

# 🧬 لغة البيان (AlBayan) - أول لغة برمجة بذكاء اصطناعي مدمج

<div align="center">

![AlBayan Logo](https://img.shields.io/badge/AlBayan-v1.0.0-blue?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge)
![AI Powered](https://img.shields.io/badge/AI-Powered-purple?style=for-the-badge)

**أول لغة برمجة في التاريخ تحتوي على ذكاء اصطناعي مدمج ومكتبات فنية متقدمة**

[🚀 البدء السريع](#-البدء-السريع) • [📚 الوثائق](#-الوثائق) • [🎯 الأمثلة](#-الأمثلة) • [🤝 المساهمة](#-المساهمة)

</div>

---

## 🌟 **ما يجعل لغة البيان ثورية؟**

### 🤖 **ذكاء اصطناعي مدمج (نظام بصيرة)**
```albayan
// نظام تشخيص طبي ذكي في سطر واحد!
let diagnosis = ai_medical_diagnosis(symptoms, 85, 1, 90);
```

### 🎨 **مكتبات فنية متقدمة**
```albayan
// رسم شخصية كرتونية متحركة
let character = enhanced_basic_shapes_simulation(15, 100); // إنسان
let animation = create_character_animation(character, 5, 90);
```

### 🧬 **مبنية على نظريات فيزيائية ثورية**
- **نظرية الصفرية المزدوجة** - كل شيء ينبثق من الصفر
- **نظرية الأضداد المتعامدة** - منع التداخل المدمر
- **نظرية الخيوط (الفتائل)** - أصغر وحدات بناء الكون

### 🌍 **دعم عربي كامل**
- أسماء متغيرات وتعليقات عربية
- رسائل خطأ عربية واضحة
- وثائق شاملة باللغة العربية

## Installation

### Prerequisites

- Rust 1.70+ (for building from source)
- LLVM 18.0+ (for code generation)

### Building from Source

```bash
git clone https://github.com/your-username/albayan.git
cd albayan
cargo build --release
```

### Installing

```bash
cargo install --path .
```

## Quick Start

### Hello World

Create a file `hello.ab`:

```albayan
fn main() -> int {
    let message = "مرحبا بكم في لغة البيان!";
    print(message);
    return 0;
}
```

Compile and run:

```bash
albayan build hello.ab
./hello
```

Or run directly:

```bash
albayan run hello.ab
```

### Logic Programming Example

```albayan
// Define relations
relation Parent(string, string);
relation Grandparent(string, string);

// Define rules
rule Grandparent(GP, GC) :- Parent(GP, P), Parent(P, GC);

fn main() -> int {
    // Assert facts
    assert Parent("أحمد", "فاطمة");
    assert Parent("فاطمة", "علي");

    // Query the knowledge base
    query_solve { Grandparent("أحمد", GrandChild) } => {
        print("أحمد جد لـ: ");
        print(GrandChild);
    }

    return 0;
}
```

### AI Integration Example

```albayan
use ai;

fn main() -> int {
    // Load an AI model
    let model = ai::load_model("classifier", "model.onnx");

    // Create input tensor
    let input = ai::create_tensor([1, 224, 224, 3], image_data);

    // Run inference
    let result = ai::predict(model, input);

    print("Classification result: ");
    print(result);

    return 0;
}
```

## Language Features

### Type System

AlBayan supports a rich type system:

```albayan
// Primitive types
let x: int = 42;
let y: float = 3.14;
let flag: bool = true;
let ch: char = 'أ';
let text: string = "مرحبا";

// Collections
let numbers: list<int> = [1, 2, 3, 4, 5];
let mapping: dict<string, int> = {"key": 42};
let coordinates: tuple<float, float> = (10.0, 20.0);

// Custom types
struct Person {
    name: string,
    age: int,
}

enum Color {
    Red,
    Green,
    Blue,
    RGB(int, int, int),
}
```

### Memory Management

AlBayan uses an ownership system for memory safety:

```albayan
fn main() -> int {
    let data = "Hello";  // data owns the string
    let borrowed = &data;  // borrowed reference

    process_data(borrowed);  // pass by reference

    let moved = data;  // ownership transferred
    // data is no longer accessible here

    return 0;
}

fn process_data(text: &string) {
    print(text);
}
```

### Async Programming

```albayan
async fn fetch_data(url: string) -> string {
    let response = await http::get(url);
    return response.body;
}

async fn main() -> int {
    let data = await fetch_data("https://api.example.com/data");
    print(data);
    return 0;
}
```

## CLI Usage

### Commands

- `albayan build <file>` - Compile a source file
- `albayan run <file>` - Run a source file directly
- `albayan check <file>` - Check syntax without compilation
- `albayan format <file>` - Format source code
- `albayan repl` - Start interactive REPL
- `albayan info` - Show language information

### Options

- `-O <level>` - Optimization level (0-3)
- `--release` - Release mode (equivalent to -O2)
- `--target <triple>` - Target platform
- `--no-logic` - Disable logic programming features
- `--no-ai` - Disable AI features
- `-v, --verbose` - Verbose output
- `-d, --debug` - Debug mode

## Examples

See the `examples/` directory for comprehensive examples:

### Basic Examples
- `examples/hello.ab` - Basic hello world
- `examples/simple.ab` - Simple function example
- `examples/logic.ab` - Logic programming features
- `examples/ai.ab` - AI integration

### Intermediate Examples
- `examples/variables_and_operations.ab` - Advanced variables and operations
- `examples/working_example.ab` - Comprehensive working example with functions

### Advanced Examples
- `examples/oop_basics.ab` - Object-oriented programming with inheritance and interfaces
- `examples/library_system.ab` - Complete library management system
- `examples/game_engine.ab` - Simple game engine with collision detection
- `examples/web_server.ab` - Asynchronous web server with routing
- `examples/expert_system.ab` - Medical diagnosis expert system
- `examples/functional_programming.ab` - Functional programming and data processing

## Architecture

AlBayan is implemented in Rust and follows a multi-phase compilation approach:

1. **Lexical Analysis** - Tokenization using the `logos` crate
2. **Parsing** - AST generation using recursive descent parser
3. **Semantic Analysis** - Type checking, symbol resolution, ownership analysis
4. **Code Generation** - LLVM IR generation using `inkwell`
5. **Runtime** - Logic engine, AI support, memory management

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/your-username/albayan.git
cd albayan
cargo build
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test lexer
cargo test parser
cargo test semantic
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Rust's ownership system
- Logic programming concepts from Prolog
- AI integration patterns from modern ML frameworks
- LLVM for high-performance code generation

## Roadmap

- [ ] Complete LLVM code generation
- [ ] Implement garbage collection
- [ ] Add package manager
- [ ] WebAssembly target support
- [ ] IDE language server
- [ ] Standard library expansion
- [ ] Performance optimizations

## Contact

- GitHub: [https://github.com/your-username/albayan](https://github.com/your-username/albayan)
- Documentation: [https://albayan-lang.org](https://albayan-lang.org)
- Community: [Discord](https://discord.gg/albayan)

---

البيان - لغة برمجة حديثة تجمع بين البرمجة المنطقية والذكاء الاصطناعي والبرمجة التقليدية
