import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    // Language server setup
    const serverPath = vscode.workspace.getConfiguration('albayan').get<string>('languageServer.path', 'albayan');
    const serverArgs = vscode.workspace.getConfiguration('albayan').get<string[]>('languageServer.args', ['lsp']);

    const serverOptions: ServerOptions = {
        run: { command: serverPath, args: serverArgs, transport: TransportKind.stdio },
        debug: { command: serverPath, args: serverArgs, transport: TransportKind.stdio }
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'albayan' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.ab')
        }
    };

    client = new LanguageClient(
        'albayanLanguageServer',
        'AlBayan Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client
    client.start();

    // Register commands
    const buildCommand = vscode.commands.registerCommand('albayan.build', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'albayan') {
            const filePath = activeEditor.document.fileName;
            runAlbayanCommand('build', [filePath]);
        } else {
            vscode.window.showErrorMessage('No AlBayan file is currently open');
        }
    });

    const runCommand = vscode.commands.registerCommand('albayan.run', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'albayan') {
            const filePath = activeEditor.document.fileName;
            runAlbayanCommand('run', [filePath]);
        } else {
            vscode.window.showErrorMessage('No AlBayan file is currently open');
        }
    });

    const checkCommand = vscode.commands.registerCommand('albayan.check', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'albayan') {
            const filePath = activeEditor.document.fileName;
            runAlbayanCommand('check', [filePath]);
        } else {
            vscode.window.showErrorMessage('No AlBayan file is currently open');
        }
    });

    const formatCommand = vscode.commands.registerCommand('albayan.format', () => {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'albayan') {
            const filePath = activeEditor.document.fileName;
            runAlbayanCommand('format', [filePath, '--in-place']);
        } else {
            vscode.window.showErrorMessage('No AlBayan file is currently open');
        }
    });

    // Register document formatting provider
    const formattingProvider = vscode.languages.registerDocumentFormattingProvider('albayan', {
        provideDocumentFormattingEdits(document: vscode.TextDocument): vscode.TextEdit[] {
            // This will be handled by the language server
            return [];
        }
    });

    // Register hover provider
    const hoverProvider = vscode.languages.registerHoverProvider('albayan', {
        provideHover(document: vscode.TextDocument, position: vscode.Position): vscode.Hover | undefined {
            // This will be handled by the language server
            return undefined;
        }
    });

    // Register completion provider
    const completionProvider = vscode.languages.registerCompletionItemProvider('albayan', {
        provideCompletionItems(document: vscode.TextDocument, position: vscode.Position): vscode.CompletionItem[] {
            // This will be handled by the language server
            return [];
        }
    }, '.', ':');

    // Add status bar item
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBarItem.text = "$(check) AlBayan";
    statusBarItem.tooltip = "AlBayan Language Support";
    statusBarItem.show();

    // Register all disposables
    context.subscriptions.push(
        buildCommand,
        runCommand,
        checkCommand,
        formatCommand,
        formattingProvider,
        hoverProvider,
        completionProvider,
        statusBarItem
    );

    // Show welcome message
    vscode.window.showInformationMessage('AlBayan Language Support is now active!');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

function runAlbayanCommand(command: string, args: string[]) {
    const terminal = vscode.window.createTerminal('AlBayan');
    const serverPath = vscode.workspace.getConfiguration('albayan').get<string>('languageServer.path', 'albayan');
    
    terminal.sendText(`${serverPath} ${command} ${args.join(' ')}`);
    terminal.show();
}

// Utility functions for AlBayan development
export class AlBayanUtils {
    static async createNewProject(projectName: string, projectPath: string) {
        const fs = require('fs');
        const path = require('path');

        const projectDir = path.join(projectPath, projectName);
        
        // Create project directory
        if (!fs.existsSync(projectDir)) {
            fs.mkdirSync(projectDir, { recursive: true });
        }

        // Create main.ab file
        const mainContent = `// AlBayan Project: ${projectName}

fn main() -> int {
    println("Hello, AlBayan!");
    return 0;
}
`;
        fs.writeFileSync(path.join(projectDir, 'main.ab'), mainContent);

        // Create project.toml file
        const projectConfig = `[project]
name = "${projectName}"
version = "0.1.0"
author = "Your Name"

[dependencies]
# Add dependencies here

[build]
optimization = 0
target = "native"
`;
        fs.writeFileSync(path.join(projectDir, 'project.toml'), projectConfig);

        vscode.window.showInformationMessage(`AlBayan project '${projectName}' created successfully!`);
        
        // Open the project
        const uri = vscode.Uri.file(projectDir);
        vscode.commands.executeCommand('vscode.openFolder', uri);
    }

    static async runTests() {
        const activeEditor = vscode.window.activeTextEditor;
        if (activeEditor && activeEditor.document.languageId === 'albayan') {
            const workspaceFolder = vscode.workspace.getWorkspaceFolder(activeEditor.document.uri);
            if (workspaceFolder) {
                runAlbayanCommand('test', [workspaceFolder.uri.fsPath]);
            }
        }
    }

    static async showDocumentation() {
        const panel = vscode.window.createWebviewPanel(
            'albayanDocs',
            'AlBayan Documentation',
            vscode.ViewColumn.Two,
            {
                enableScripts: true
            }
        );

        panel.webview.html = getDocumentationHtml();
    }
}

function getDocumentationHtml(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AlBayan Documentation</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        h1, h2 { color: #2d3748; }
        code { background-color: #f7fafc; padding: 2px 4px; border-radius: 3px; }
        pre { background-color: #f7fafc; padding: 10px; border-radius: 5px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>AlBayan Programming Language</h1>
    
    <h2>Basic Syntax</h2>
    <pre><code>// Function definition
fn greet(name: string) -> string {
    return "Hello, " + name + "!";
}

// Variable declaration
let message = greet("World");
println(message);</code></pre>

    <h2>Logic Programming</h2>
    <pre><code>// Define relations
relation parent(string, string);
relation grandparent(string, string);

// Assert facts
fact parent("john", "mary").
fact parent("mary", "alice").

// Define rules
rule grandparent(X, Z) :- parent(X, Y), parent(Y, Z).

// Query the knowledge base
query grandparent("john", "alice").</code></pre>

    <h2>AI Features</h2>
    <pre><code>// Load an AI model
model classifier = load("model.onnx");

// Create tensors
tensor<float> input = [[1.0, 2.0, 3.0]];

// Make predictions
let result = classifier.predict(input);
println("Prediction: " + result);</code></pre>

    <h2>Concurrency</h2>
    <pre><code>// Async functions
async fn fetch_data(url: string) -> string {
    // Fetch data asynchronously
    return await http_get(url);
}

// Channels for communication
let (sender, receiver) = channel::<string>();

// Spawn async task
spawn async {
    let data = await fetch_data("https://api.example.com");
    sender.send(data);
};</code></pre>
</body>
</html>`;
}
