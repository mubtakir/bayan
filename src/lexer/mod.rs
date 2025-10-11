//! # Lexical Analysis Module
//!
//! This module implements the lexer (tokenizer) for the AlBayan programming language.
//! It converts source code text into a stream of tokens that can be processed by the parser.

use logos::Logos;
use std::fmt;

/// Token types for the AlBayan language
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords - Definitions
    #[token("fn")]
    Fn,
    #[token("class")]
    Class,
    #[token("interface")]
    Interface,
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("const")]
    Const,
    #[token("type")]
    Type,

    // Keywords - Control Flow
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("match")]
    Match,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("loop")]
    Loop,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,

    // Keywords - Logic Programming
    #[token("relation")]
    Relation,
    #[token("fact")]
    Fact,
    #[token("rule")]
    Rule,
    #[token("query_solve")]
    QuerySolve,
    #[token("query_prove")]
    QueryProve,
    #[token("assert")]
    Assert,
    #[token("retract")]
    Retract,

    // Keywords - AI
    #[token("model")]
    Model,
    #[token("tensor")]
    Tensor,

    // Keywords - Concurrency
    #[token("async")]
    Async,
    #[token("await")]
    Await,
    #[token("gpu")]
    Gpu,

    // Keywords - Modules
    #[token("module")]
    Module,
    #[token("using")]
    Using,
    #[token("pub")]
    Pub,
    #[token("priv")]
    Priv,

    // Keywords - Special
    #[token("self")]
    SelfKeyword,
    #[token("super")]
    Super,
    #[token("init")]
    Init,
    #[token("dynamic")]
    Dynamic,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,

    // Identifiers and Literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntegerLiteral(Option<i64>),

    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(Option<f64>),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        s[1..s.len()-1].to_owned() // Remove quotes
    })]
    StringLiteral(String),

    #[regex(r"'([^'\\]|\\.)'", |lex| {
        let s = lex.slice();
        s.chars().nth(1) // Get character between quotes
    })]
    CharLiteral(Option<char>),

    // Operators - Arithmetic
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("**")]
    Power,

    // Operators - Comparison
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,

    // Operators - Logical
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,

    // Operators - Assignment
    #[token("=")]
    Assign,
    #[token("+=")]
    PlusAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    MultiplyAssign,
    #[token("/=")]
    DivideAssign,

    // Operators - Logic Programming
    #[token(":-")]
    Implies, // For rules: head :- body

    // Delimiters
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,

    // Punctuation
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("_", priority = 3)]
    Underscore,

    // Special tokens
    #[regex(r"//[^\n]*", logos::skip)] // Skip line comments
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)] // Skip block comments
    #[regex(r"[ \t\f]+", logos::skip)] // Skip whitespace
    #[token("\n")]
    Newline,

    // Error token
    Error,

    // End of file
    Eof,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: std::ops::Range<usize>,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}:{}", self.token_type, self.line, self.column)
    }
}

/// Lexer for the AlBayan language
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut lexer = TokenType::lexer(self.input);

        while let Some(token_type) = lexer.next() {
            let span = lexer.span();
            let (line, column) = self.position_to_line_col(span.start);

            match token_type {
                Ok(token_type) => {
                    if !matches!(token_type, TokenType::Error) {
                        tokens.push(Token {
                            token_type,
                            span,
                            line,
                            column,
                        });
                    }
                }
                Err(_) => {
                    return Err(LexerError::InvalidToken {
                        position: span.start,
                        line,
                        column,
                    });
                }
            }
        }

        // Add EOF token
        tokens.push(Token {
            token_type: TokenType::Eof,
            span: self.input.len()..self.input.len(),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    /// Convert byte position to line and column
    fn position_to_line_col(&self, position: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.input.char_indices() {
            if i >= position {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        (line, column)
    }
}

/// Lexer error types
#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Invalid token at line {line}, column {column} (position {position})")]
    InvalidToken {
        position: usize,
        line: usize,
        column: usize,
    },

    #[error("Unexpected end of input")]
    UnexpectedEof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("fn main() { let x = 42; }");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Fn);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("main".to_string()));
        assert_eq!(tokens[2].token_type, TokenType::LeftParen);
        assert_eq!(tokens[3].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_logic_tokens() {
        let mut lexer = Lexer::new("relation Parent(string, string); rule Grandparent(GP, GC) :- Parent(GP, P), Parent(P, GC);");
        let tokens = lexer.tokenize().unwrap();

        // Print tokens for debugging
        for (i, token) in tokens.iter().enumerate() {
            println!("Token {}: {:?}", i, token.token_type);
        }

        assert_eq!(tokens[0].token_type, TokenType::Relation);
        // Find the Implies token
        let implies_pos = tokens.iter().position(|t| matches!(t.token_type, TokenType::Implies));
        assert!(implies_pos.is_some(), "Implies token not found");
    }
}
