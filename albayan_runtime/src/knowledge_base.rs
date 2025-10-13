//! Knowledge Base for storing facts and rules
//! Expert recommendation: Priority 2 - Logic Core Implementation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use indexmap::IndexMap;

/// A logical term (Expert recommendation: Core logic representation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Term {
    /// Variable: ?X, ?Y
    Variable(String),
    /// Constant: "Ahmed", 42, true
    Constant(Value),
    /// Compound term: parent(?X, ?Y)
    Compound {
        functor: String,
        args: Vec<Term>,
    },
}

/// Values in the logic system (Expert recommendation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Int(i64),
    Float(String), // Store as string to avoid floating point equality issues
    Bool(bool),
}

/// A fact in the knowledge base (Expert recommendation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fact {
    pub relation: String,
    pub args: Vec<Value>,
}

/// A rule in the knowledge base (Expert recommendation)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    pub head: Term,
    pub body: Vec<Term>,
}

/// Knowledge base for storing facts and rules (Expert recommendation: Priority 2)
#[derive(Debug, Clone, Default)]
pub struct KnowledgeBase {
    /// Facts indexed by relation name for fast lookup
    facts: IndexMap<String, Vec<Fact>>,
    /// Rules indexed by head relation name for fast lookup
    rules: IndexMap<String, Vec<Rule>>,
    /// Relation metadata (arity, types)
    relations: HashMap<String, RelationInfo>,
}

/// Information about a relation (Expert recommendation)
#[derive(Debug, Clone)]
pub struct RelationInfo {
    pub name: String,
    pub arity: usize,
    pub arg_types: Vec<String>, // Type names
}

impl KnowledgeBase {
    /// Create a new empty knowledge base
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a relation with its metadata (Expert recommendation)
    pub fn register_relation(&mut self, name: String, arity: usize, arg_types: Vec<String>) {
        self.relations.insert(name.clone(), RelationInfo {
            name,
            arity,
            arg_types,
        });
    }

    /// Assert a fact into the knowledge base (Expert recommendation: API function)
    pub fn assert_fact(&mut self, relation: String, args: Vec<Value>) -> Result<(), String> {
        // Validate relation exists and arity matches
        if let Some(rel_info) = self.relations.get(&relation) {
            if args.len() != rel_info.arity {
                return Err(format!(
                    "Arity mismatch for relation {}: expected {}, got {}",
                    relation, rel_info.arity, args.len()
                ));
            }
        } else {
            return Err(format!("Unknown relation: {}", relation));
        }

        let fact = Fact { relation: relation.clone(), args };

        self.facts
            .entry(relation)
            .or_insert_with(Vec::new)
            .push(fact);

        Ok(())
    }

    /// Register a rule in the knowledge base (Expert recommendation: API function)
    pub fn register_rule(&mut self, head: Term, body: Vec<Term>) -> Result<(), String> {
        // Extract relation name from head
        let relation_name = match &head {
            Term::Compound { functor, .. } => functor.clone(),
            _ => return Err("Rule head must be a compound term".to_string()),
        };

        let rule = Rule { head, body };

        self.rules
            .entry(relation_name)
            .or_insert_with(Vec::new)
            .push(rule);

        Ok(())
    }

    /// Get all facts for a relation
    pub fn get_facts(&self, relation: &str) -> Option<&Vec<Fact>> {
        self.facts.get(relation)
    }

    /// Get all rules for a relation
    pub fn get_rules(&self, relation: &str) -> Option<&Vec<Rule>> {
        self.rules.get(relation)
    }

    /// Get relation info
    pub fn get_relation_info(&self, relation: &str) -> Option<&RelationInfo> {
        self.relations.get(relation)
    }

    /// Get all relation names
    pub fn get_all_relations(&self) -> Vec<&String> {
        self.relations.keys().collect()
    }
}

impl Term {
    /// Check if term is a variable
    pub fn is_variable(&self) -> bool {
        matches!(self, Term::Variable(_))
    }

    /// Get all variables in the term
    pub fn get_variables(&self) -> Vec<&String> {
        match self {
            Term::Variable(name) => vec![name],
            Term::Constant(_) => vec![],
            Term::Compound { args, .. } => {
                args.iter().flat_map(|arg| arg.get_variables()).collect()
            }
        }
    }

    /// Get the functor name if this is a compound term
    pub fn get_functor(&self) -> Option<&String> {
        match self {
            Term::Compound { functor, .. } => Some(functor),
            _ => None,
        }
    }

    /// Get the arity (number of arguments) if this is a compound term
    pub fn get_arity(&self) -> Option<usize> {
        match self {
            Term::Compound { args, .. } => Some(args.len()),
            _ => None,
        }
    }
}

impl Value {
    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
        }
    }

    /// Convert from string representation (Expert recommendation: API support)
    pub fn from_string(s: &str, type_name: &str) -> Result<Self, String> {
        match type_name {
            "string" => Ok(Value::String(s.to_string())),
            "int" => s.parse::<i64>()
                .map(Value::Int)
                .map_err(|_| format!("Cannot parse '{}' as int", s)),
            "float" => s.parse::<f64>()
                .map(|f| Value::Float(f.to_string()))
                .map_err(|_| format!("Cannot parse '{}' as float", s)),
            "bool" => match s {
                "true" => Ok(Value::Bool(true)),
                "false" => Ok(Value::Bool(false)),
                _ => Err(format!("Cannot parse '{}' as bool", s)),
            },
            _ => Err(format!("Unknown type: {}", type_name)),
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.clone(),
            Value::Bool(b) => b.to_string(),
        }
    }
}
