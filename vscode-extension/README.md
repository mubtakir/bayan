# AlBayan Language Support for VS Code

This extension provides comprehensive language support for the AlBayan programming language in Visual Studio Code.

## Features

### Syntax Highlighting
- Full syntax highlighting for AlBayan language constructs
- Support for traditional programming, logic programming, and AI features
- Proper highlighting for keywords, types, operators, and comments

### Language Server Protocol (LSP)
- Real-time error detection and diagnostics
- Intelligent code completion
- Hover information for symbols
- Document formatting
- Go to definition support

### Code Snippets
- Comprehensive snippet collection for common patterns
- Function definitions, control structures, and data types
- Logic programming constructs (relations, rules, facts, queries)
- AI-specific snippets (models, tensors, neural networks)
- Concurrency patterns (async functions, channels, mutexes)

### Commands
- **Build**: Compile AlBayan source files (`Ctrl+Shift+B`)
- **Run**: Execute AlBayan programs (`Ctrl+F5`)
- **Check**: Syntax checking without compilation
- **Format**: Code formatting (`Shift+Alt+F`)

## Installation

1. Install the AlBayan compiler from [GitHub](https://github.com/albayan-team/albayan)
2. Install this extension from the VS Code marketplace
3. Configure the path to the AlBayan executable in settings

## Configuration

Open VS Code settings and configure the following options:

```json
{
  "albayan.languageServer.path": "albayan",
  "albayan.languageServer.args": ["lsp"],
  "albayan.trace.server": "off"
}
```

### Settings

- `albayan.languageServer.path`: Path to the AlBayan executable
- `albayan.languageServer.args`: Arguments to pass to the language server
- `albayan.trace.server`: Trace communication with the language server

## Usage

### Creating a New File

1. Create a new file with the `.ab` extension
2. Start typing AlBayan code
3. Use `Ctrl+Space` for code completion
4. Use `Ctrl+Shift+B` to build your program

### Example Code

```albayan
// Traditional programming
fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

// Logic programming
relation parent(string, string);
fact parent("john", "mary").
fact parent("mary", "alice").

rule grandparent(X, Z) :- parent(X, Y), parent(Y, Z).
query grandparent("john", "alice").

// AI features
model classifier = load("model.onnx");
tensor<float> input = [[1.0, 2.0, 3.0]];
let prediction = classifier.predict(input);

fn main() -> int {
    let result = fibonacci(10);
    println("Fibonacci(10) = " + result);
    return 0;
}
```

## Keyboard Shortcuts

| Command | Shortcut | Description |
|---------|----------|-------------|
| Build | `Ctrl+Shift+B` | Compile the current file |
| Run | `Ctrl+F5` | Run the current file |
| Format | `Shift+Alt+F` | Format the current document |
| Toggle Comment | `Ctrl+/` | Toggle line comment |

## Snippets

Type the following prefixes and press `Tab` to expand:

### General
- `fn` - Function definition
- `main` - Main function
- `let` - Variable declaration
- `if` - If statement
- `while` - While loop
- `struct` - Struct definition

### Logic Programming
- `relation` - Relation declaration
- `rule` - Logic rule
- `fact` - Logic fact
- `query` - Logic query

### AI Features
- `model` - AI model loading
- `tensor` - Tensor declaration
- `neural` - Neural network definition
- `train` - Model training

### Concurrency
- `async` - Async function
- `channel` - Channel creation
- `mutex` - Mutex creation

## Troubleshooting

### Language Server Not Starting

1. Ensure the AlBayan compiler is installed and in your PATH
2. Check the `albayan.languageServer.path` setting
3. Try restarting VS Code
4. Check the Output panel for error messages

### Syntax Highlighting Not Working

1. Ensure the file has the `.ab` extension
2. Try reloading the window (`Ctrl+Shift+P` → "Developer: Reload Window")

### Code Completion Not Working

1. Ensure the language server is running (check status bar)
2. Try triggering completion manually with `Ctrl+Space`
3. Check for syntax errors that might prevent analysis

## Contributing

Contributions are welcome! Please see our [contributing guidelines](CONTRIBUTING.md) for details.

## License

This extension is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Support

- [GitHub Issues](https://github.com/albayan-team/vscode-albayan/issues)
- [Documentation](https://albayan-lang.org/docs)
- [Community Discord](https://discord.gg/albayan)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each version.
