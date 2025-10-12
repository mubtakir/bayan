//! # Logic Programming Analysis
//!
//! This module implements analysis for logic programming constructs in AlBayan.
//! It validates relations, rules, facts, and queries for correctness and safety.

use crate::parser::ast::*;
use super::{ResolvedType, SemanticError, RelationInfo};
use std::collections::{HashMap, HashSet};

/// Logic analyzer for validating logic programming constructs
#[derive(Debug)]
pub struct LogicAnalyzer {
    /// Known relations and their signatures
    relations: HashMap<String, RelationInfo>,
    /// Rules in the knowledge base
    rules: Vec<ValidatedRule>,
    /// Facts in the knowledge base
    facts: Vec<ValidatedFact>,
}

/// A validated rule with type information
#[derive(Debug, Clone)]
pub struct ValidatedRule {
    pub head: ValidatedTerm,
    pub body: Vec<ValidatedTerm>,
    pub variables: HashSet<String>,
}

/// A validated fact with type information
#[derive(Debug, Clone)]
pub struct ValidatedFact {
    pub term: ValidatedTerm,
}

/// A validated logic term with type information
#[derive(Debug, Clone)]
pub struct ValidatedTerm {
    pub relation: String,
    pub args: Vec<ValidatedArg>,
    pub arg_types: Vec<ResolvedType>,
}

/// A validated logic argument with type information
#[derive(Debug, Clone)]
pub enum ValidatedArg {
    Variable { name: String, var_type: ResolvedType },
    Constant { value: ConstantValue, const_type: ResolvedType },
}

/// Constant values in logic programming
#[derive(Debug, Clone)]
pub enum ConstantValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Symbol(String),
}

impl LogicAnalyzer {
    /// Create a new logic analyzer
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
            rules: Vec::new(),
            facts: Vec::new(),
        }
    }

    /// Register a relation declaration
    pub fn register_relation(&mut self, relation: &RelationDecl) -> Result<(), SemanticError> {
        if self.relations.contains_key(&relation.name) {
            return Err(SemanticError::Redefinition(relation.name.clone()));
        }

        // Convert Type to ResolvedType (this should be done by type checker)
        let mut arg_types = Vec::new();
        for arg_type in &relation.arg_types {
            let resolved_type = self.resolve_type(arg_type)?;
            arg_types.push(resolved_type);
        }

        self.relations.insert(relation.name.clone(), RelationInfo {
            name: relation.name.clone(),
            arg_types,
        });

        Ok(())
    }

    /// Validate and register a rule
    pub fn validate_rule(&mut self, rule: &RuleDecl) -> Result<ValidatedRule, SemanticError> {
        // Validate the head term
        let validated_head = self.validate_term(&rule.head)?;

        // Validate body terms
        let mut validated_body = Vec::new();
        for term in &rule.body {
            let validated_term = self.validate_term(term)?;
            validated_body.push(validated_term);
        }

        // Extract all variables from the rule
        let mut variables = HashSet::new();
        self.extract_variables(&validated_head, &mut variables);
        for term in &validated_body {
            self.extract_variables(term, &mut variables);
        }

        // Check rule safety: all variables in head must appear in body
        self.check_rule_safety(&validated_head, &validated_body)?;

        let validated_rule = ValidatedRule {
            head: validated_head,
            body: validated_body,
            variables,
        };

        self.rules.push(validated_rule.clone());
        Ok(validated_rule)
    }

    /// Validate and register a fact
    pub fn validate_fact(&mut self, fact: &FactDecl) -> Result<ValidatedFact, SemanticError> {
        let validated_term = self.validate_term(&fact.term)?;

        // Facts should only contain constants, not variables
        self.check_fact_groundness(&validated_term)?;

        let validated_fact = ValidatedFact {
            term: validated_term,
        };

        self.facts.push(validated_fact.clone());
        Ok(validated_fact)
    }

    /// Validate a logic term
    fn validate_term(&self, term: &LogicTerm) -> Result<ValidatedTerm, SemanticError> {
        // Check if relation exists
        let relation_info = self.relations.get(&term.name)
            .ok_or_else(|| SemanticError::UndefinedRelation(term.name.clone()))?;

        // Check arity
        if term.args.len() != relation_info.arg_types.len() {
            return Err(SemanticError::ArityMismatch {
                expected: relation_info.arg_types.len(),
                found: term.args.len(),
            });
        }

        // Validate arguments
        let mut validated_args = Vec::new();
        for (arg, expected_type) in term.args.iter().zip(&relation_info.arg_types) {
            let validated_arg = self.validate_arg(arg, expected_type)?;
            validated_args.push(validated_arg);
        }

        Ok(ValidatedTerm {
            relation: term.name.clone(),
            args: validated_args,
            arg_types: relation_info.arg_types.clone(),
        })
    }

    /// Validate a logic argument
    fn validate_arg(&self, arg: &LogicArg, expected_type: &ResolvedType) -> Result<ValidatedArg, SemanticError> {
        match arg {
            LogicArg::Variable(name) => {
                // Variables can be of any type that matches the expected type
                Ok(ValidatedArg::Variable {
                    name: name.clone(),
                    var_type: expected_type.clone(),
                })
            }

            LogicArg::Constant(name) => {
                // Constants should be validated against a constant table
                // For now, assume they're symbols
                Ok(ValidatedArg::Constant {
                    value: ConstantValue::Symbol(name.clone()),
                    const_type: expected_type.clone(),
                })
            }

            LogicArg::StringConstant(s) => {
                if !matches!(expected_type, ResolvedType::String) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::String,
                        found: expected_type.clone(),
                    });
                }
                Ok(ValidatedArg::Constant {
                    value: ConstantValue::String(s.clone()),
                    const_type: ResolvedType::String,
                })
            }

            LogicArg::IntConstant(n) => {
                if !matches!(expected_type, ResolvedType::Int) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Int,
                        found: expected_type.clone(),
                    });
                }
                Ok(ValidatedArg::Constant {
                    value: ConstantValue::Integer(*n),
                    const_type: ResolvedType::Int,
                })
            }

            LogicArg::FloatConstant(f) => {
                if !matches!(expected_type, ResolvedType::Float) {
                    return Err(SemanticError::TypeMismatch {
                        expected: ResolvedType::Float,
                        found: expected_type.clone(),
                    });
                }
                Ok(ValidatedArg::Constant {
                    value: ConstantValue::Float(*f),
                    const_type: ResolvedType::Float,
                })
            }
        }
    }

    /// Extract all variables from a term
    fn extract_variables(&self, term: &ValidatedTerm, variables: &mut HashSet<String>) {
        for arg in &term.args {
            if let ValidatedArg::Variable { name, .. } = arg {
                variables.insert(name.clone());
            }
        }
    }

    /// Check rule safety: all head variables must appear in body
    fn check_rule_safety(&self, head: &ValidatedTerm, body: &[ValidatedTerm]) -> Result<(), SemanticError> {
        let mut head_vars = HashSet::new();
        self.extract_variables(head, &mut head_vars);

        let mut body_vars = HashSet::new();
        for term in body {
            self.extract_variables(term, &mut body_vars);
        }

        for head_var in &head_vars {
            if !body_vars.contains(head_var) {
                return Err(SemanticError::UnboundVariable(head_var.clone()));
            }
        }

        Ok(())
    }

    /// Check that a fact contains only constants (is ground)
    fn check_fact_groundness(&self, term: &ValidatedTerm) -> Result<(), SemanticError> {
        for arg in &term.args {
            if let ValidatedArg::Variable { name, .. } = arg {
                return Err(SemanticError::VariableInFact(name.clone()));
            }
        }
        Ok(())
    }

    /// Validate a query
    pub fn validate_query(&self, goals: &[LogicTerm]) -> Result<Vec<ValidatedTerm>, SemanticError> {
        let mut validated_goals = Vec::new();

        for goal in goals {
            let validated_goal = self.validate_term(goal)?;
            validated_goals.push(validated_goal);
        }

        // Check that all variables in the query are properly bound
        self.check_query_safety(&validated_goals)?;

        Ok(validated_goals)
    }

    /// Check query safety (variables should be properly constrained)
    fn check_query_safety(&self, goals: &[ValidatedTerm]) -> Result<(), SemanticError> {
        // For now, we allow any variables in queries
        // More sophisticated analysis could check for infinite search spaces
        Ok(())
    }

    /// Get all relations
    pub fn get_relations(&self) -> &HashMap<String, RelationInfo> {
        &self.relations
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[ValidatedRule] {
        &self.rules
    }

    /// Get all facts
    pub fn get_facts(&self) -> &[ValidatedFact] {
        &self.facts
    }

    /// Check if a relation exists
    pub fn relation_exists(&self, name: &str) -> bool {
        self.relations.contains_key(name)
    }

    /// Get relation info
    pub fn get_relation_info(&self, name: &str) -> Option<&RelationInfo> {
        self.relations.get(name)
    }

    /// Resolve a type (placeholder - should use type checker)
    fn resolve_type(&self, type_annotation: &Type) -> Result<ResolvedType, SemanticError> {
        match type_annotation {
            Type::Named(name) => {
                match name.to_string().as_str() {
                    "int" => Ok(ResolvedType::Int),
                    "float" => Ok(ResolvedType::Float),
                    "bool" => Ok(ResolvedType::Bool),
                    "string" => Ok(ResolvedType::String),
                    "char" => Ok(ResolvedType::Char),
                    _ => Ok(ResolvedType::Struct(name.to_string())),
                }
            }
            _ => todo!("Complex type resolution not yet implemented"),
        }
    }
}

// Add new error type for variables in facts
impl SemanticError {
    pub fn VariableInFact(var: String) -> Self {
        SemanticError::VariableInFact(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation_registration() {
        let mut analyzer = LogicAnalyzer::new();

        let relation = RelationDecl {
            name: "Parent".to_string(),
            arg_types: vec![
                Type::Named("string".to_string()),
                Type::Named("string".to_string()),
            ],
        };

        let result = analyzer.register_relation(&relation);
        assert!(result.is_ok());
        assert!(analyzer.relation_exists("Parent"));
    }

    #[test]
    fn test_fact_validation() {
        let mut analyzer = LogicAnalyzer::new();

        // Register relation first
        let relation = RelationDecl {
            name: "Parent".to_string(),
            arg_types: vec![
                Type::Named("string".to_string()),
                Type::Named("string".to_string()),
            ],
        };
        analyzer.register_relation(&relation).unwrap();

        // Create a fact
        let fact = FactDecl {
            term: LogicTerm {
                name: "Parent".to_string(),
                args: vec![
                    LogicArg::StringConstant("john".to_string()),
                    LogicArg::StringConstant("mary".to_string()),
                ],
            },
        };

        let result = analyzer.validate_fact(&fact);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rule_validation() {
        let mut analyzer = LogicAnalyzer::new();

        // Register relations
        let parent_relation = RelationDecl {
            name: "Parent".to_string(),
            arg_types: vec![
                Type::Named("string".to_string()),
                Type::Named("string".to_string()),
            ],
        };
        analyzer.register_relation(&parent_relation).unwrap();

        let grandparent_relation = RelationDecl {
            name: "Grandparent".to_string(),
            arg_types: vec![
                Type::Named("string".to_string()),
                Type::Named("string".to_string()),
            ],
        };
        analyzer.register_relation(&grandparent_relation).unwrap();

        // Create a rule: Grandparent(GP, GC) :- Parent(GP, P), Parent(P, GC)
        let rule = RuleDecl {
            head: LogicTerm {
                name: "Grandparent".to_string(),
                args: vec![
                    LogicArg::Variable("GP".to_string()),
                    LogicArg::Variable("GC".to_string()),
                ],
            },
            body: vec![
                LogicTerm {
                    name: "Parent".to_string(),
                    args: vec![
                        LogicArg::Variable("GP".to_string()),
                        LogicArg::Variable("P".to_string()),
                    ],
                },
                LogicTerm {
                    name: "Parent".to_string(),
                    args: vec![
                        LogicArg::Variable("P".to_string()),
                        LogicArg::Variable("GC".to_string()),
                    ],
                },
            ],
        };

        let result = analyzer.validate_rule(&rule);
        assert!(result.is_ok());
    }
}
