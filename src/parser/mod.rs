//! # Parser Module
//! 
//! This module implements the parser for the AlBayan programming language.
//! It converts a stream of tokens into an Abstract Syntax Tree (AST).

pub mod ast;

use crate::lexer::{Token, TokenType};
use ast::*;
use std::collections::HashMap;

/// Parser for the AlBayan language
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Create a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }
    
    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue; // Skip newlines at top level
            }
            
            let item = self.parse_item()?;
            items.push(item);
        }
        
        Ok(Program { items })
    }
    
    /// Parse a top-level item (function, struct, relation, etc.)
    fn parse_item(&mut self) -> Result<Item, ParseError> {
        match &self.peek().token_type {
            TokenType::Fn => self.parse_function(),
            TokenType::Struct => self.parse_struct(),
            TokenType::Enum => self.parse_enum(),
            TokenType::Class => self.parse_class(),
            TokenType::Interface => self.parse_interface(),
            TokenType::Relation => self.parse_relation(),
            TokenType::Rule => self.parse_rule(),
            TokenType::Fact => self.parse_fact(),
            TokenType::Module => self.parse_module(),
            TokenType::Using => self.parse_using(),
            _ => Err(ParseError::UnexpectedToken {
                expected: "item declaration".to_string(),
                found: self.peek().clone(),
            }),
        }
    }
    
    /// Parse a function declaration
    fn parse_function(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Fn, "Expected 'fn'")?;
        
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let param_name = self.consume_identifier("Expected parameter name")?;
                self.consume(&TokenType::Colon, "Expected ':' after parameter name")?;
                let param_type = self.parse_type()?;
                
                parameters.push(Parameter {
                    name: param_name,
                    param_type: param_type,
                });
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(&TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let body = self.parse_block()?;
        
        Ok(Item::Function(FunctionDecl {
            name,
            parameters,
            return_type,
            body,
        }))
    }
    
    /// Parse a struct declaration
    fn parse_struct(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Struct, "Expected 'struct'")?;
        let name = self.consume_identifier("Expected struct name")?;
        
        self.consume(&TokenType::LeftBrace, "Expected '{' after struct name")?;
        
        let mut fields = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_identifier("Expected field name")?;
            self.consume(&TokenType::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type()?;
            self.consume(&TokenType::Semicolon, "Expected ';' after field")?;
            
            fields.push(StructField {
                name: field_name,
                field_type: field_type,
            });
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after struct fields")?;
        
        Ok(Item::Struct(StructDecl { name, fields }))
    }
    
    /// Parse a relation declaration
    fn parse_relation(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Relation, "Expected 'relation'")?;
        let name = self.consume_identifier("Expected relation name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after relation name")?;
        
        let mut arg_types = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let arg_type = self.parse_type()?;
                arg_types.push(arg_type);
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after relation arguments")?;
        self.consume(&TokenType::Semicolon, "Expected ';' after relation declaration")?;
        
        Ok(Item::Relation(RelationDecl { name, arg_types }))
    }
    
    /// Parse a rule declaration
    fn parse_rule(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Rule, "Expected 'rule'")?;
        
        let head = self.parse_logic_term()?;
        self.consume(&TokenType::Implies, "Expected ':-' in rule")?;
        
        let mut body = Vec::new();
        loop {
            let term = self.parse_logic_term()?;
            body.push(term);
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }
        
        self.consume(&TokenType::Semicolon, "Expected ';' after rule")?;
        
        Ok(Item::Rule(RuleDecl { head, body }))
    }
    
    /// Parse a logic term (for relations, rules, queries)
    fn parse_logic_term(&mut self) -> Result<LogicTerm, ParseError> {
        let name = self.consume_identifier("Expected term name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after term name")?;
        
        let mut args = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                let arg = self.parse_logic_arg()?;
                args.push(arg);
                
                if !self.match_token(&TokenType::Comma) {
                    break;
                }
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after term arguments")?;
        
        Ok(LogicTerm { name, args })
    }
    
    /// Parse a logic argument (variable or constant)
    fn parse_logic_arg(&mut self) -> Result<LogicArg, ParseError> {
        match &self.peek().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // Variables start with uppercase, constants with lowercase
                if name.chars().next().unwrap().is_uppercase() {
                    Ok(LogicArg::Variable(name))
                } else {
                    Ok(LogicArg::Constant(name))
                }
            }
            TokenType::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                Ok(LogicArg::StringConstant(s))
            }
            TokenType::IntegerLiteral(Some(n)) => {
                let n = *n;
                self.advance();
                Ok(LogicArg::IntConstant(n))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "logic argument".to_string(),
                found: self.peek().clone(),
            }),
        }
    }
    
    /// Parse a type annotation
    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match &self.peek().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Type::Named(name))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "type".to_string(),
                found: self.peek().clone(),
            }),
        }
    }
    
    /// Parse a block statement
    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.consume(&TokenType::LeftBrace, "Expected '{'")?;
        
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}'")?;
        
        Ok(Block { statements })
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match &self.peek().token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::If => self.parse_if_statement(),
            _ => {
                // Check if this is an assignment
                if self.check_assignment() {
                    self.parse_assignment_statement()
                } else {
                    let expr = self.parse_expression()?;
                    self.consume(&TokenType::Semicolon, "Expected ';' after expression")?;
                    Ok(Statement::Expression(expr))
                }
            }
        }
    }

    /// Check if the current position is an assignment
    fn check_assignment(&mut self) -> bool {
        if let TokenType::Identifier(_) = &self.peek().token_type {
            let saved_pos = self.current;
            self.advance(); // Skip identifier
            let is_assignment = matches!(
                &self.peek().token_type,
                TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign |
                TokenType::MultiplyAssign | TokenType::DivideAssign
            );
            self.current = saved_pos; // Restore position
            is_assignment
        } else {
            false
        }
    }

    /// Parse assignment statement
    fn parse_assignment_statement(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume_identifier("Expected variable name")?;

        let operator = match &self.peek().token_type {
            TokenType::Assign => {
                self.advance();
                BinaryOperator::Assign
            }
            TokenType::PlusAssign => {
                self.advance();
                BinaryOperator::AddAssign
            }
            TokenType::MinusAssign => {
                self.advance();
                BinaryOperator::SubtractAssign
            }
            TokenType::MultiplyAssign => {
                self.advance();
                BinaryOperator::MultiplyAssign
            }
            TokenType::DivideAssign => {
                self.advance();
                BinaryOperator::DivideAssign
            }
            _ => return Err(ParseError::UnexpectedToken {
                expected: "assignment operator".to_string(),
                found: self.peek().clone(),
            }),
        };

        let value = self.parse_expression()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after assignment")?;

        Ok(Statement::Expression(Expression::Binary(BinaryExpression {
            left: Box::new(Expression::Identifier(name)),
            operator,
            right: Box::new(value),
        })))
    }
    
    /// Parse a let statement
    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(&TokenType::Let, "Expected 'let'")?;
        let name = self.consume_identifier("Expected variable name")?;
        
        let var_type = if self.match_token(&TokenType::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let initializer = if self.match_token(&TokenType::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume(&TokenType::Semicolon, "Expected ';' after let statement")?;
        
        Ok(Statement::Let(LetStatement {
            name,
            var_type,
            initializer,
        }))
    }
    
    /// Parse a return statement
    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(&TokenType::Return, "Expected 'return'")?;
        
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume(&TokenType::Semicolon, "Expected ';' after return")?;
        
        Ok(Statement::Return(ReturnStatement { value }))
    }
    
    /// Parse an if statement
    fn parse_if_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(&TokenType::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;
        
        let else_block = if self.match_token(&TokenType::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(Statement::If(IfStatement {
            condition,
            then_block,
            else_block,
        }))
    }
    
    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_logical_or()
    }

    /// Parse logical OR expressions
    fn parse_logical_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_logical_and()?;

        while self.match_token(&TokenType::Or) {
            let right = self.parse_logical_and()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::Or,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// Parse logical AND expressions
    fn parse_logical_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_equality()?;

        while self.match_token(&TokenType::And) {
            let right = self.parse_equality()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: BinaryOperator::And,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }
    
    /// Parse equality expressions (==, !=)
    fn parse_equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        
        while self.match_tokens(&[TokenType::Equal, TokenType::NotEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.parse_comparison()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: match operator {
                    TokenType::Equal => BinaryOperator::Equal,
                    TokenType::NotEqual => BinaryOperator::NotEqual,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    /// Parse comparison expressions (<, <=, >, >=)
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_term()?;
        
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.parse_term()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: match operator {
                    TokenType::Greater => BinaryOperator::Greater,
                    TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                    TokenType::Less => BinaryOperator::Less,
                    TokenType::LessEqual => BinaryOperator::LessEqual,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    /// Parse term expressions (+, -)
    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_factor()?;
        
        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().token_type.clone();
            let right = self.parse_factor()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: match operator {
                    TokenType::Plus => BinaryOperator::Add,
                    TokenType::Minus => BinaryOperator::Subtract,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            });
        }
        
        Ok(expr)
    }
    
    /// Parse factor expressions (*, /, %)
    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_unary()?;

        while self.match_tokens(&[TokenType::Divide, TokenType::Multiply, TokenType::Modulo]) {
            let operator = self.previous().token_type.clone();
            let right = self.parse_unary()?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: match operator {
                    TokenType::Multiply => BinaryOperator::Multiply,
                    TokenType::Divide => BinaryOperator::Divide,
                    TokenType::Modulo => BinaryOperator::Modulo,
                    _ => unreachable!(),
                },
                right: Box::new(right),
            });
        }

        Ok(expr)
    }
    
    /// Parse unary expressions (!, -)
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.match_tokens(&[TokenType::Not, TokenType::Minus]) {
            let operator = self.previous().token_type.clone();
            let right = self.parse_unary()?;
            return Ok(Expression::Unary(UnaryExpression {
                operator: match operator {
                    TokenType::Not => UnaryOperator::Not,
                    TokenType::Minus => UnaryOperator::Negate,
                    _ => unreachable!(),
                },
                operand: Box::new(right),
            }));
        }
        
        self.parse_primary()
    }
    
    /// Parse primary expressions (literals, identifiers, parentheses)
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let mut expr = match &self.peek().token_type {
            TokenType::True => {
                self.advance();
                Expression::Literal(Literal::Boolean(true))
            }
            TokenType::False => {
                self.advance();
                Expression::Literal(Literal::Boolean(false))
            }
            TokenType::Null => {
                self.advance();
                Expression::Literal(Literal::Null)
            }
            TokenType::IntegerLiteral(Some(n)) => {
                let n = *n;
                self.advance();
                Expression::Literal(Literal::Integer(n))
            }
            TokenType::FloatLiteral(Some(f)) => {
                let f = *f;
                self.advance();
                Expression::Literal(Literal::Float(f))
            }
            TokenType::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                Expression::Literal(Literal::String(s))
            }
            TokenType::CharLiteral(Some(c)) => {
                let c = *c;
                self.advance();
                Expression::Literal(Literal::Char(c))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Expression::Identifier(name)
            }
            TokenType::LeftParen => {
                self.advance();
                let inner_expr = self.parse_expression()?;
                self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                inner_expr
            }
            TokenType::LeftBracket => {
                // Array literal
                self.advance();
                let mut elements = Vec::new();

                if !self.check(&TokenType::RightBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }

                self.consume(&TokenType::RightBracket, "Expected ']' after array elements")?;
                Expression::Array(ArrayExpression { elements })
            }
            _ => return Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: self.peek().clone(),
            }),
        };

        // Handle postfix operations (function calls, field access, indexing)
        loop {
            match &self.peek().token_type {
                TokenType::LeftParen => {
                    // Function call
                    self.advance();
                    let mut arguments = Vec::new();

                    if !self.check(&TokenType::RightParen) {
                        loop {
                            arguments.push(self.parse_expression()?);
                            if !self.match_token(&TokenType::Comma) {
                                break;
                            }
                        }
                    }

                    self.consume(&TokenType::RightParen, "Expected ')' after function arguments")?;
                    expr = Expression::Call(CallExpression {
                        callee: Box::new(expr),
                        arguments,
                    });
                }
                TokenType::Dot => {
                    // Field access
                    self.advance();
                    let field = self.consume_identifier("Expected field name after '.'")?;
                    expr = Expression::FieldAccess(FieldAccessExpression {
                        object: Box::new(expr),
                        field,
                    });
                }
                TokenType::LeftBracket => {
                    // Index access
                    self.advance();
                    let index = self.parse_expression()?;
                    self.consume(&TokenType::RightBracket, "Expected ']' after index")?;
                    expr = Expression::Index(IndexExpression {
                        object: Box::new(expr),
                        index: Box::new(index),
                    });
                }
                _ => break,
            }
        }

        Ok(expr)
    }
    
    // Helper methods for parsing
    
    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: message.to_string(),
                found: self.peek().clone(),
            })
        }
    }
    
    fn consume_identifier(&mut self, message: &str) -> Result<String, ParseError> {
        match &self.peek().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: message.to_string(),
                found: self.peek().clone(),
            }),
        }
    }
    
    // Placeholder implementations for missing items
    fn parse_enum(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Enum, "Expected 'enum'")?;
        let name = self.consume_identifier("Expected enum name")?;

        self.consume(&TokenType::LeftBrace, "Expected '{' after enum name")?;

        let mut variants = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let variant_name = self.consume_identifier("Expected variant name")?;

            let fields = if self.match_token(&TokenType::LeftParen) {
                let mut variant_fields = Vec::new();
                if !self.check(&TokenType::RightParen) {
                    loop {
                        let field_type = self.parse_type()?;
                        variant_fields.push(field_type);

                        if !self.match_token(&TokenType::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::RightParen, "Expected ')' after variant fields")?;
                Some(variant_fields)
            } else {
                None
            };

            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });

            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after enum variants")?;

        Ok(Item::Enum(EnumDecl { name, variants }))
    }

    fn parse_class(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Class, "Expected 'class'")?;
        let name = self.consume_identifier("Expected class name")?;

        self.consume(&TokenType::LeftBrace, "Expected '{' after class name")?;

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.check(&TokenType::Fn) {
                // Parse method
                if let Item::Function(func) = self.parse_function()? {
                    methods.push(func);
                }
            } else {
                // Parse field
                let field_name = self.consume_identifier("Expected field name")?;
                self.consume(&TokenType::Colon, "Expected ':' after field name")?;
                let field_type = self.parse_type()?;
                self.consume(&TokenType::Semicolon, "Expected ';' after field")?;

                fields.push(StructField {
                    name: field_name,
                    field_type,
                });
            }
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after class body")?;

        Ok(Item::Class(ClassDecl { name, fields, methods }))
    }

    fn parse_interface(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Interface, "Expected 'interface'")?;
        let name = self.consume_identifier("Expected interface name")?;

        self.consume(&TokenType::LeftBrace, "Expected '{' after interface name")?;

        let mut methods = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            self.consume(&TokenType::Fn, "Expected 'fn' in interface")?;
            let method_name = self.consume_identifier("Expected method name")?;

            self.consume(&TokenType::LeftParen, "Expected '(' after method name")?;

            let mut parameters = Vec::new();
            if !self.check(&TokenType::RightParen) {
                loop {
                    let param_name = self.consume_identifier("Expected parameter name")?;
                    self.consume(&TokenType::Colon, "Expected ':' after parameter name")?;
                    let param_type = self.parse_type()?;

                    parameters.push(Parameter {
                        name: param_name,
                        param_type,
                    });

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }
            }

            self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;

            let return_type = if self.match_token(&TokenType::Arrow) {
                Some(self.parse_type()?)
            } else {
                None
            };

            self.consume(&TokenType::Semicolon, "Expected ';' after method signature")?;

            methods.push(FunctionSignature {
                name: method_name,
                parameters,
                return_type,
            });
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after interface methods")?;

        Ok(Item::Interface(InterfaceDecl { name, methods }))
    }

    fn parse_fact(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Fact, "Expected 'fact'")?;
        let term = self.parse_logic_term()?;
        self.consume(&TokenType::Semicolon, "Expected ';' after fact")?;

        Ok(Item::Fact(FactDecl { term }))
    }

    fn parse_module(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Module, "Expected 'module'")?;
        let name = self.consume_identifier("Expected module name")?;

        self.consume(&TokenType::LeftBrace, "Expected '{' after module name")?;

        let mut items = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenType::Newline) {
                continue;
            }
            let item = self.parse_item()?;
            items.push(item);
        }

        self.consume(&TokenType::RightBrace, "Expected '}' after module body")?;

        Ok(Item::Module(ModuleDecl { name, items }))
    }

    fn parse_using(&mut self) -> Result<Item, ParseError> {
        self.consume(&TokenType::Using, "Expected 'using'")?;

        let mut path = Vec::new();
        path.push(self.consume_identifier("Expected module path")?);

        while self.match_token(&TokenType::DoubleColon) {
            path.push(self.consume_identifier("Expected module path component")?);
        }

        let alias = if self.check(&TokenType::Identifier("as".to_string())) {
            self.advance();
            Some(self.consume_identifier("Expected alias name")?)
        } else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expected ';' after using declaration")?;

        Ok(Item::Using(UsingDecl { path, alias }))
    }
}

/// Parser error types
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken {
        expected: String,
        found: Token,
    },
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid syntax: {message}")]
    InvalidSyntax { message: String },
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_parse_simple_function() {
        let source = "fn main() { return 42; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        assert_eq!(ast.items.len(), 1);
        assert!(matches!(ast.items[0], Item::Function(_)));
    }
}
