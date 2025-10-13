//! # Abstract Syntax Tree (AST) Definitions
//!
//! This module defines the AST node types for the AlBayan programming language.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a qualified path (e.g., `std::collections::HashMap`)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Path {
    pub segments: Vec<String>,
}

impl Path {
    /// Create a new path from a single segment
    pub fn single(segment: String) -> Self {
        Self {
            segments: vec![segment],
        }
    }

    /// Create a new path from multiple segments
    pub fn from_segments(segments: Vec<String>) -> Self {
        Self { segments }
    }

    /// Parse a path from a string like "std::collections::HashMap"
    pub fn from_string(path_str: &str) -> Self {
        Self {
            segments: path_str.split("::").map(|s| s.to_string()).collect(),
        }
    }

    /// Convert path back to string representation
    pub fn to_string(&self) -> String {
        self.segments.join("::")
    }

    pub fn as_str(&self) -> String {
        self.to_string()
    }
}

/// Root node of the AST - represents a complete program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level items in a program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Function(FunctionDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Class(ClassDecl),
    Interface(InterfaceDecl),
    Trait(TraitDecl),        // NEWLY ADDED: Expert recommendation
    Impl(ImplDecl),          // NEWLY ADDED: Expert recommendation
    Relation(RelationDecl),
    Rule(RuleDecl),
    Fact(FactDecl),
    Module(ModuleDecl),
    Using(UsingDecl),
}

/// Function declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
    pub generic_params: Option<Vec<GenericParam>>,  // NEWLY ADDED: Expert recommendation
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Block,
}

/// Function parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

/// Struct declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDecl {
    pub name: String,
    pub generic_params: Option<Vec<GenericParam>>,  // NEWLY ADDED: Expert recommendation
    pub fields: Vec<StructField>,
}

/// Struct field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}

/// Enum declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<Type>>,
}

/// Class declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassDecl {
    pub name: String,
    pub fields: Vec<StructField>,
    pub methods: Vec<FunctionDecl>,
}

/// Interface declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDecl {
    pub name: String,
    pub methods: Vec<FunctionSignature>,
}

/// Trait declaration (Expert recommendation: Priority 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitDecl {
    pub name: String,
    pub generic_params: Option<Vec<GenericParam>>,
    pub methods: Vec<TraitMethod>,
}

/// Trait method (can be required or have default implementation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,  // None for required methods, Some for default implementations
}

/// Impl declaration (Expert recommendation: Priority 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplDecl {
    pub trait_name: Option<String>,  // None for inherent impl, Some for trait impl
    pub type_name: String,
    pub generic_params: Option<Vec<GenericParam>>,
    pub methods: Vec<FunctionDecl>,
}

/// Generic parameter (Expert recommendation: Priority 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TraitBound>,  // Trait bounds like T: Display + Clone
}

/// Trait bound (Expert recommendation: Priority 1)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitBound {
    pub trait_name: String,
}

/// Function signature (for interfaces)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
}

/// Relation declaration (for logic programming)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationDecl {
    pub name: String,
    pub arg_types: Vec<Type>,
}

/// Rule declaration (for logic programming)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleDecl {
    pub head: LogicTerm,
    pub body: Vec<LogicTerm>,
}

/// Fact declaration (for logic programming)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactDecl {
    pub term: LogicTerm,
}

/// Logic term (predicate with arguments)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogicTerm {
    pub name: String,
    pub args: Vec<LogicArg>,
}

/// Logic argument (variable or constant)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicArg {
    Variable(String),
    Constant(String),
    StringConstant(String),
    IntConstant(i64),
    FloatConstant(f64),
}

/// Module declaration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleDecl {
    pub name: String,
    pub items: Vec<Item>,
}

/// Using declaration (imports)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UsingDecl {
    pub path: Vec<String>,
    pub alias: Option<String>,
}

/// Type annotations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Named type using qualified path (int, string, std::collections::HashMap, etc.)
    Named(Path),
    /// Generic type with parameters (Vec<T>, HashMap<K, V>, etc.)
    Generic(Path, Vec<Type>),
    /// Generic type parameter (T, U, K, V, etc.)
    GenericParam(String),
    /// Function type
    Function(Vec<Type>, Box<Type>),
    /// Tuple type
    Tuple(Vec<Type>),
    /// Array type with size
    Array(Box<Type>, Option<usize>),
    /// Trait object type (dyn Trait) - Expert recommendation: Priority 1
    TraitObject(Vec<Path>),
    /// Reference type (&T, &mut T) - Expert recommendation: Priority 1
    Reference(Box<Type>, bool), // bool: true for mutable (&mut), false for immutable (&)
    /// Multi-dimensional array
    Matrix(Box<Type>, Vec<usize>),
    /// Vector type with fixed size
    Vector(Box<Type>, usize),
    /// Set type for unique elements
    Set(Box<Type>),
    /// Map/Dictionary type
    Map(Box<Type>, Box<Type>),
    /// Queue type (FIFO)
    Queue(Box<Type>),
    /// Stack type (LIFO)
    Stack(Box<Type>),
    /// Tree structure
    Tree(Box<Type>),
    /// Graph with nodes and edges
    Graph(Box<Type>, Box<Type>),
    /// Union type (one of several types)
    Union(Vec<Type>),
    /// Result type for error handling
    Result(Box<Type>, Box<Type>),
    /// Channel for concurrent communication
    Channel(Box<Type>),
    /// Mutex for thread safety
    Mutex(Box<Type>),
    /// Atomic type for lock-free operations
    Atomic(Box<Type>),

    /// Optional type
    Optional(Box<Type>),
    /// Tensor type for AI operations
    Tensor(Vec<usize>),
    /// Dataset type for machine learning
    Dataset(Box<Type>),
    /// Model type for AI models
    Model(String),
}

/// Block of statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
}

/// Statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Let(LetStatement),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Match(MatchStatement),
    Block(Block),
    Query(QueryStatement),
    Assert(AssertStatement),
    Retract(RetractStatement),
}

/// Let statement (variable declaration)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LetStatement {
    pub name: String,
    pub var_type: Option<Type>,
    pub initializer: Option<Expression>,
}

/// Return statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

/// If statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

/// While statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}

/// For statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStatement {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
}

/// Match statement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
}

/// Match arm
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Block,
}

/// Pattern for match statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Identifier(String),
    Tuple(Vec<Pattern>),
    Struct(String, Vec<(String, Pattern)>),
    Enum(String, Option<Vec<Pattern>>),
}

/// Query statement (logic programming)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryStatement {
    pub query_type: QueryType,
    pub goals: Vec<LogicTerm>,
    pub handler: Option<Block>,
}

/// Type of logic query
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    Solve,  // query_solve
    Prove,  // query_prove
}

/// Assert statement (add fact to knowledge base)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssertStatement {
    pub fact: LogicTerm,
}

/// Retract statement (remove fact from knowledge base)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetractStatement {
    pub fact: LogicTerm,
}

/// Expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
    FieldAccess(FieldAccessExpression),
    Index(IndexExpression),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    Struct(StructExpression),
    Enum(EnumExpression),
    Lambda(LambdaExpression),
    Async(AsyncExpression),
    Await(AwaitExpression),
    Match(Box<MatchStatement>), // Match can be both statement and expression
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Null,
}

/// Binary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,

    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,

    // Assignment
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
}

/// Unary expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Negate,
    Reference,
    MutableReference,
    Dereference,
}

/// Function call expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
}

/// Field access expression (obj.field)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessExpression {
    pub object: Box<Expression>,
    pub field: String,
}

/// Index expression (arr[index])
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexExpression {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
}

/// Array literal expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayExpression {
    pub elements: Vec<Expression>,
}

/// Tuple literal expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleExpression {
    pub elements: Vec<Expression>,
}

/// Struct literal expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructExpression {
    pub name: String,
    pub fields: Vec<(String, Expression)>,
}

/// Enum variant construction expression (Expert recommendation)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumExpression {
    pub enum_name: String,
    pub variant_name: String,
    pub fields: Option<Vec<Expression>>, // For tuple-like variants: Some(Color::RGB(255, 0, 0))
}

/// Lambda expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaExpression {
    pub parameters: Vec<Parameter>,
    pub body: Box<Expression>,
}

/// Async expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsyncExpression {
    pub body: Block,
}

/// Await expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitExpression {
    pub expression: Box<Expression>,
}

/// Trait for AST node visitors
pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_item(&mut self, item: &Item) -> T;
    fn visit_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression(&mut self, expression: &Expression) -> T;
}

/// Trait for mutable AST node visitors
pub trait AstVisitorMut<T> {
    fn visit_program(&mut self, program: &mut Program) -> T;
    fn visit_item(&mut self, item: &mut Item) -> T;
    fn visit_statement(&mut self, statement: &mut Statement) -> T;
    fn visit_expression(&mut self, expression: &mut Expression) -> T;
}

// Re-export the main AST node type
pub use Program as AstNode;
