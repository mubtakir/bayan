//! # Improved Parser with Chumsky
//! 
//! This module implements an improved parser using the chumsky library
//! with proper recursive parser setup using the .define() pattern.

use chumsky::prelude::*;
use crate::lexer::{Token, TokenType};
use crate::parser::ast::*;

/// Parser error type
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken {
        expected: String,
        found: Token,
    },
    UnexpectedEof,
    InvalidSyntax(String),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, found } => {
                write!(f, "Expected {}, found {:?}", expected, found)
            }
            ParserError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParserError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
        }
    }
}

impl std::error::Error for ParserError {}

/// Type alias for parser input
type ParserInput<'a> = &'a [Token];

/// Type alias for parser output with error recovery
type ParserOutput<T> = (Option<T>, Vec<Simple<String>>);

/// Main parser structure using chumsky
pub struct ChumskyParser;

impl ChumskyParser {
    /// Parse tokens into an AST using the improved recursive parser setup
    pub fn parse(tokens: &[Token]) -> Result<Program, Vec<ParserError>> {
        let parser = Self::setup_recursive_parsers();
        
        match parser.parse(tokens) {
            Ok(program) => Ok(program),
            Err(errors) => {
                let parser_errors = errors
                    .into_iter()
                    .map(|e| ParserError::InvalidSyntax(e.to_string()))
                    .collect();
                Err(parser_errors)
            }
        }
    }
    
    /// Setup recursive parsers using the .define() pattern for better mutual recursion
    fn setup_recursive_parsers() -> impl Parser<ParserInput<'_>, Program, Error = Simple<String>> + Clone {
        // --- Forward Declarations ---
        // Create recursive parser placeholders that will be defined later
        let expr = recursive(|_| empty().to(Expression::Literal(Literal::Integer(0))));
        let type_annot = recursive(|_| empty().to(Type::Named(Path::single("unit".to_string()))));
        let stmt = recursive(|_| empty().to(Statement::Expression(Expression::Literal(Literal::Integer(0)))));
        let block = recursive(|_| empty().to(Block { statements: vec![] }));
        let pattern = recursive(|_| empty().to(Pattern::Identifier("_".to_string())));
        let item = recursive(|_| empty().to(Item::Function(FunctionDecl {
            name: "dummy".to_string(),
            parameters: vec![],
            return_type: None,
            body: Block { statements: vec![] },
        })));
        
        // --- Token Parsers ---
        let identifier = filter_map(|span, token: &Token| {
            match &token.token_type {
                TokenType::Identifier(name) => Ok(name.clone()),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token.clone()))),
            }
        });
        
        let integer = filter_map(|span, token: &Token| {
            match &token.token_type {
                TokenType::Integer(n) => Ok(*n),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token.clone()))),
            }
        });
        
        let float_num = filter_map(|span, token: &Token| {
            match &token.token_type {
                TokenType::Float(f) => Ok(*f),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token.clone()))),
            }
        });
        
        let string_lit = filter_map(|span, token: &Token| {
            match &token.token_type {
                TokenType::String(s) => Ok(s.clone()),
                _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token.clone()))),
            }
        });
        
        let token = |token_type: TokenType| {
            filter(move |token: &Token| token.token_type == token_type)
        };
        
        // --- Implementation Parsers ---
        
        // Literal parser
        let literal = choice((
            integer.map(Literal::Integer),
            float_num.map(Literal::Float),
            string_lit.map(Literal::String),
            token(TokenType::True).to(Literal::Boolean(true)),
            token(TokenType::False).to(Literal::Boolean(false)),
        ));
        
        // Path parser (for qualified names like std::collections::HashMap)
        let path = identifier
            .separated_by(token(TokenType::DoubleColon))
            .at_least(1)
            .map(|segments| Path::from_segments(segments));
        
        // Type annotation parser implementation
        let type_impl = choice((
            path.map(Type::Named),
            // Add more type variants as needed
        ));
        
        // Pattern parser implementation
        let pattern_impl = choice((
            identifier.map(Pattern::Identifier),
            literal.map(Pattern::Literal),
            // Add more pattern variants as needed
        ));
        
        // Expression parser implementation
        let expr_impl = choice((
            literal.map(Expression::Literal),
            identifier.map(Expression::Identifier),
            // Add more expression variants as needed
        ));
        
        // Block parser implementation
        let block_impl = stmt
            .clone()
            .repeated()
            .delimited_by(token(TokenType::LeftBrace), token(TokenType::RightBrace))
            .map(|statements| Block { statements });
        
        // Statement parser implementation
        let stmt_impl = choice((
            expr.clone().map(Statement::Expression),
            // Add more statement variants as needed
        ));
        
        // Function parameter parser
        let parameter = identifier
            .then_ignore(token(TokenType::Colon))
            .then(type_annot.clone())
            .map(|(name, param_type)| Parameter { name, param_type });
        
        // Function declaration parser
        let function_decl = token(TokenType::Fn)
            .ignore_then(identifier)
            .then(
                parameter
                    .separated_by(token(TokenType::Comma))
                    .delimited_by(token(TokenType::LeftParen), token(TokenType::RightParen))
            )
            .then(
                token(TokenType::Arrow)
                    .ignore_then(type_annot.clone())
                    .or_not()
            )
            .then(block.clone())
            .map(|(((name, parameters), return_type), body)| {
                FunctionDecl {
                    name,
                    parameters,
                    return_type,
                    body,
                }
            });
        
        // Item parser implementation
        let item_impl = choice((
            function_decl.map(Item::Function),
            // Add more item variants as needed
        ));
        
        // Program parser
        let program = item
            .clone()
            .repeated()
            .map(|items| Program { items })
            .then_ignore(end());
        
        // --- Define the recursive parsers ---
        // This is where the magic happens - we define what each recursive parser actually is
        expr.define(expr_impl);
        type_annot.define(type_impl);
        stmt.define(stmt_impl);
        block.define(block_impl);
        pattern.define(pattern_impl);
        item.define(item_impl);
        
        program
    }
}

/// Helper function to create a simple parser for testing
pub fn create_simple_parser() -> impl Parser<ParserInput<'_>, Program, Error = Simple<String>> + Clone {
    ChumskyParser::setup_recursive_parsers()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_simple_function_parsing() {
        let source = "fn main() -> int { 42 }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        
        let result = ChumskyParser::parse(&tokens);
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.items.len(), 1);
        
        match &program.items[0] {
            Item::Function(func) => {
                assert_eq!(func.name, "main");
                assert_eq!(func.parameters.len(), 0);
                assert!(func.return_type.is_some());
            }
            _ => panic!("Expected function declaration"),
        }
    }
    
    #[test]
    fn test_path_parsing() {
        let path = Path::from_string("std::collections::HashMap");
        assert_eq!(path.segments, vec!["std", "collections", "HashMap"]);
        assert_eq!(path.to_string(), "std::collections::HashMap");
    }
}
