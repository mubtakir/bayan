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
    /// First-argument indexing for performance (Expert recommendation: Priority 2 - Indexing)
    fact_index: HashMap<String, HashMap<String, Vec<usize>>>, // relation -> first_arg -> fact_indices
    rule_index: HashMap<String, HashMap<String, Vec<usize>>>, // relation -> first_arg -> rule_indices
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
        Self {
            facts: IndexMap::new(),
            rules: IndexMap::new(),
            relations: HashMap::new(),
            fact_index: HashMap::new(),
            rule_index: HashMap::new(),
        }
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

        let fact = Fact { relation: relation.clone(), args: args.clone() };

        // Add to main facts storage
        let facts_vec = self.facts
            .entry(relation.clone())
            .or_insert_with(Vec::new);
        let fact_index = facts_vec.len();
        facts_vec.push(fact);

        // Update first-argument index for performance (Expert recommendation: Priority 2)
        if !args.is_empty() {
            let first_arg_key = format!("{:?}", args[0]); // Simplified key generation
            self.fact_index
                .entry(relation)
                .or_insert_with(HashMap::new)
                .entry(first_arg_key)
                .or_insert_with(Vec::new)
                .push(fact_index);
        }

        Ok(())
    }

    /// Register a rule in the knowledge base (Expert recommendation: API function)
    pub fn register_rule(&mut self, head: Term, body: Vec<Term>) -> Result<(), String> {
        // Extract relation name from head
        let relation_name = match &head {
            Term::Compound { functor, .. } => functor.clone(),
            _ => return Err("Rule head must be a compound term".to_string()),
        };

        let rule = Rule { head: head.clone(), body };

        // Add to main rules storage
        let rules_vec = self.rules
            .entry(relation_name.clone())
            .or_insert_with(Vec::new);
        let rule_index = rules_vec.len();
        rules_vec.push(rule);

        // Update first-argument index for rules (Expert recommendation: Priority 2)
        if let Term::Compound { args, .. } = &head {
            if !args.is_empty() {
                let first_arg_key = format!("{:?}", args[0]); // Simplified key generation
                self.rule_index
                    .entry(relation_name)
                    .or_insert_with(HashMap::new)
                    .entry(first_arg_key)
                    .or_insert_with(Vec::new)
                    .push(rule_index);
            }
        }

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

    /// Get facts with indexed first argument (Expert recommendation: Priority 2 - Performance)
    pub fn get_facts_by_first_arg(&self, relation: &str, first_arg: &Value) -> Vec<&Fact> {
        let first_arg_key = format!("{:?}", first_arg);

        if let Some(relation_index) = self.fact_index.get(relation) {
            if let Some(fact_indices) = relation_index.get(&first_arg_key) {
                if let Some(facts_vec) = self.facts.get(relation) {
                    return fact_indices.iter()
                        .filter_map(|&idx| facts_vec.get(idx))
                        .collect();
                }
            }
        }

        Vec::new()
    }

    /// Get rules with indexed first argument (Expert recommendation: Priority 2 - Performance)
    pub fn get_rules_by_first_arg(&self, relation: &str, first_arg: &Term) -> Vec<&Rule> {
        let first_arg_key = format!("{:?}", first_arg);

        if let Some(relation_index) = self.rule_index.get(relation) {
            if let Some(rule_indices) = relation_index.get(&first_arg_key) {
                if let Some(rules_vec) = self.rules.get(relation) {
                    return rule_indices.iter()
                        .filter_map(|&idx| rules_vec.get(idx))
                        .collect();
                }
            }
        }

        Vec::new()
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
