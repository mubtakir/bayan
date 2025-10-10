use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::CompilerOptions;

/// AlBayan Language Server
pub struct AlBayanLanguageServer {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentInfo>>>,
}

/// Information about an open document
#[derive(Debug, Clone)]
struct DocumentInfo {
    content: String,
    version: i32,
    diagnostics: Vec<Diagnostic>,
}

impl AlBayanLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Analyze document and return diagnostics
    async fn analyze_document(&self, uri: &Url, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Lexical analysis
        let mut lexer = Lexer::new(content);
        match lexer.tokenize() {
            Ok(tokens) => {
                // Parsing
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => {
                        // Semantic analysis
                        let options = CompilerOptions::default();
                        let mut analyzer = SemanticAnalyzer::new(&options);

                        if let Err(error) = analyzer.analyze(ast) {
                            // Handle single error instead of iterator
                            let diagnostic = Diagnostic {
                                range: Range {
                                    start: Position { line: 0, character: 0 },
                                    end: Position { line: 0, character: 0 },
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: None,
                                code_description: None,
                                source: Some("albayan".to_string()),
                                message: error.to_string(),
                                related_information: None,
                                tags: None,
                                data: None,
                            };
                            diagnostics.push(diagnostic);
                        }
                    }
                    Err(error) => {
                        let diagnostic = Diagnostic {
                            range: Range {
                                start: Position { line: 0, character: 0 },
                                end: Position { line: 0, character: 0 },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            code_description: None,
                            source: Some("albayan".to_string()),
                            message: format!("Parse error: {}", error),
                            related_information: None,
                            tags: None,
                            data: None,
                        };
                        diagnostics.push(diagnostic);
                    }
                }
            }
            Err(error) => {
                let diagnostic = Diagnostic {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 0 },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    code_description: None,
                    source: Some("albayan".to_string()),
                    message: format!("Lexer error: {}", error),
                    related_information: None,
                    tags: None,
                    data: None,
                };
                diagnostics.push(diagnostic);
            }
        }

        diagnostics
    }

    /// Get completions for the current position
    fn get_completions(&self, _position: Position) -> Vec<CompletionItem> {
        vec![
            // Keywords
            CompletionItem {
                label: "fn".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Function declaration".to_string()),
                documentation: Some(Documentation::String("Define a new function".to_string())),
                insert_text: Some("fn ${1:name}(${2:params}) -> ${3:return_type} {\n    ${4:body}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "let".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Variable declaration".to_string()),
                documentation: Some(Documentation::String("Declare a new variable".to_string())),
                insert_text: Some("let ${1:name} = ${2:value};".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Conditional statement".to_string()),
                documentation: Some(Documentation::String("Conditional execution".to_string())),
                insert_text: Some("if ${1:condition} {\n    ${2:body}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "while".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Loop statement".to_string()),
                documentation: Some(Documentation::String("Loop while condition is true".to_string())),
                insert_text: Some("while ${1:condition} {\n    ${2:body}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "struct".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Structure definition".to_string()),
                documentation: Some(Documentation::String("Define a new structure".to_string())),
                insert_text: Some("struct ${1:Name} {\n    ${2:fields}\n}".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            // Logic programming keywords
            CompletionItem {
                label: "relation".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Logic relation".to_string()),
                documentation: Some(Documentation::String("Define a logic relation".to_string())),
                insert_text: Some("relation ${1:name}(${2:args});".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "rule".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Logic rule".to_string()),
                documentation: Some(Documentation::String("Define a logic rule".to_string())),
                insert_text: Some("rule ${1:head} :- ${2:body}.".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "fact".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Logic fact".to_string()),
                documentation: Some(Documentation::String("Assert a logic fact".to_string())),
                insert_text: Some("fact ${1:predicate}(${2:args}).".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "query".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Logic query".to_string()),
                documentation: Some(Documentation::String("Query the knowledge base".to_string())),
                insert_text: Some("query ${1:goal}.".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            // AI keywords
            CompletionItem {
                label: "model".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("AI model".to_string()),
                documentation: Some(Documentation::String("Define an AI model".to_string())),
                insert_text: Some("model ${1:name} = load(\"${2:path}\");".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "tensor".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Tensor type".to_string()),
                documentation: Some(Documentation::String("Multi-dimensional array for AI".to_string())),
                insert_text: Some("tensor<${1:type}> ${2:name} = ${3:value};".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            // Types
            CompletionItem {
                label: "int".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Integer type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "float".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Floating point type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "bool".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("Boolean type".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "string".to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some("String type".to_string()),
                ..Default::default()
            },
        ]
    }

    /// Format document
    async fn format_document(&self, uri: &Url) -> Result<Vec<TextEdit>> {
        let documents = self.documents.read().await;
        if let Some(doc_info) = documents.get(uri) {
            // Simple formatting - add proper indentation
            let lines: Vec<&str> = doc_info.content.lines().collect();
            let mut formatted_lines = Vec::new();
            let mut indent_level: u32 = 0;

            for line in &lines {
                let trimmed = line.trim();
                
                // Decrease indent for closing braces
                if trimmed.starts_with('}') {
                    indent_level = indent_level.saturating_sub(1);
                }
                
                // Add indentation
                let indented_line = format!("{}{}", "    ".repeat(indent_level as usize), trimmed);
                formatted_lines.push(indented_line);
                
                // Increase indent for opening braces
                if trimmed.ends_with('{') {
                    indent_level += 1;
                }
            }

            let formatted_content = formatted_lines.join("\n");
            
            // Return a single edit that replaces the entire document
            Ok(vec![TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { 
                        line: lines.len() as u32, 
                        character: 0 
                    },
                },
                new_text: formatted_content,
            }])
        } else {
            Ok(vec![])
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for AlBayanLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                document_formatting_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "AlBayan Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let content = params.text_document.text;
        let version = params.text_document.version;

        let diagnostics = self.analyze_document(&uri, &content).await;

        let doc_info = DocumentInfo {
            content: content.clone(),
            version,
            diagnostics: diagnostics.clone(),
        };

        self.documents.write().await.insert(uri.clone(), doc_info);

        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        if let Some(change) = params.content_changes.into_iter().next() {
            let content = change.text;
            let diagnostics = self.analyze_document(&uri, &content).await;

            let doc_info = DocumentInfo {
                content: content.clone(),
                version,
                diagnostics: diagnostics.clone(),
            };

            self.documents.write().await.insert(uri.clone(), doc_info);

            self.client
                .publish_diagnostics(uri, diagnostics, Some(version))
                .await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = self.get_completions(params.text_document_position.position);
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let edits = self.format_document(&params.text_document.uri).await?;
        Ok(Some(edits))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // Simple hover information
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(
                "AlBayan Language - Hover information".to_string(),
            )),
            range: None,
        }))
    }
}

/// Start the language server
pub async fn start_language_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| AlBayanLanguageServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
