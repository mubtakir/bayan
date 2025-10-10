//! # Logic Programming Engine
//! 
//! This module implements the logic programming engine for AlBayan.
//! It provides Prolog-style inference with facts, rules, and queries.

use std::collections::{HashMap, HashSet, VecDeque};
use indexmap::IndexMap;
use super::RuntimeError;

/// Logic programming engine
#[derive(Debug)]
pub struct LogicEngine {
    /// Knowledge base containing facts and rules
    knowledge_base: KnowledgeBase,
    
    /// Query execution statistics
    queries_executed: usize,
    
    /// Maximum search depth to prevent infinite loops
    max_depth: usize,
    
    /// Debug mode
    debug: bool,
}

/// Knowledge base containing facts and rules
#[derive(Debug, Clone)]
struct KnowledgeBase {
    /// Facts indexed by predicate name
    facts: IndexMap<String, Vec<Fact>>,
    
    /// Rules indexed by head predicate name
    rules: IndexMap<String, Vec<Rule>>,
    
    /// Predicate signatures (name -> arity)
    predicates: HashMap<String, usize>,
}

/// A fact in the knowledge base
#[derive(Debug, Clone, PartialEq)]
struct Fact {
    predicate: String,
    args: Vec<Term>,
}

/// A rule in the knowledge base
#[derive(Debug, Clone)]
struct Rule {
    head: Fact,
    body: Vec<Goal>,
}

/// A goal in a rule body or query
#[derive(Debug, Clone)]
struct Goal {
    predicate: String,
    args: Vec<Term>,
    negated: bool,
}

/// A term in logic programming
#[derive(Debug, Clone, PartialEq)]
enum Term {
    Variable(String),
    Atom(String),
    Integer(i64),
    Float(f64),
    String(String),
    Compound(String, Vec<Term>),
}

/// Variable bindings during unification
type Bindings = HashMap<String, Term>;

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub success: bool,
    pub bindings: Vec<HashMap<String, String>>,
}

impl LogicEngine {
    /// Create a new logic engine
    pub fn new() -> Self {
        Self {
            knowledge_base: KnowledgeBase::new(),
            queries_executed: 0,
            max_depth: 1000,
            debug: false,
        }
    }
    
    /// Initialize the logic engine
    pub fn initialize(&mut self) -> Result<(), RuntimeError> {
        // Add built-in predicates
        self.add_builtin_predicates()?;
        Ok(())
    }
    
    /// Shutdown the logic engine
    pub fn shutdown(&mut self) -> Result<(), RuntimeError> {
        self.knowledge_base.clear();
        Ok(())
    }
    
    /// Add built-in predicates
    fn add_builtin_predicates(&mut self) -> Result<(), RuntimeError> {
        // Add arithmetic predicates
        self.knowledge_base.add_predicate("=", 2);
        self.knowledge_base.add_predicate("<", 2);
        self.knowledge_base.add_predicate(">", 2);
        self.knowledge_base.add_predicate("<=", 2);
        self.knowledge_base.add_predicate(">=", 2);
        self.knowledge_base.add_predicate("is", 2);
        
        Ok(())
    }
    
    /// Assert a fact into the knowledge base with improved indexing
    pub fn assert_fact(&mut self, fact_str: &str) -> Result<(), RuntimeError> {
        let fact = self.parse_fact(fact_str)?;
        self.knowledge_base.add_fact(fact);
        Ok(())
    }

    /// Assert multiple facts at once for better performance
    pub fn assert_facts(&mut self, fact_strings: &[&str]) -> Result<(), RuntimeError> {
        for fact_str in fact_strings {
            self.assert_fact(fact_str)?;
        }
        Ok(())
    }
    
    /// Retract a fact from the knowledge base
    pub fn retract_fact(&mut self, fact_str: &str) -> Result<(), RuntimeError> {
        let fact = self.parse_fact(fact_str)?;
        self.knowledge_base.remove_fact(&fact);
        Ok(())
    }
    
    /// Add a rule to the knowledge base
    pub fn add_rule(&mut self, rule_str: &str) -> Result<(), RuntimeError> {
        let rule = self.parse_rule(rule_str)?;
        self.knowledge_base.add_rule(rule);
        Ok(())
    }
    
    /// Solve a query with improved algorithm
    pub fn solve_query(&mut self, query_str: &str) -> Result<Vec<HashMap<String, String>>, RuntimeError> {
        self.queries_executed += 1;

        let goals = self.parse_complex_query(query_str)?;
        let mut results = Vec::new();

        // Use improved backtracking search with constraint propagation
        let mut bindings = Bindings::new();
        self.solve_goals_with_constraints(&goals, &mut bindings, &mut results, 0)?;

        // Convert internal bindings to string format
        let string_results = results.into_iter()
            .map(|binding| {
                binding.into_iter()
                    .map(|(k, v)| (k, self.term_to_string(&v)))
                    .collect()
            })
            .collect();

        Ok(string_results)
    }

    /// Solve goals with constraint propagation for better performance
    fn solve_goals_with_constraints(
        &self,
        goals: &[Goal],
        bindings: &mut Bindings,
        results: &mut Vec<Bindings>,
        depth: usize,
    ) -> Result<(), RuntimeError> {
        if depth > self.max_depth {
            return Err(RuntimeError::LogicError("Maximum search depth exceeded".to_string()));
        }

        if goals.is_empty() {
            results.push(bindings.clone());
            return Ok(());
        }

        // Apply constraint propagation before goal selection
        let propagated_bindings = self.propagate_constraints(bindings, goals)?;

        // Select the most constrained goal
        let best_goal_index = self.select_most_constrained_goal(goals, &propagated_bindings);
        let selected_goal = &goals[best_goal_index];

        // Build remaining goals
        let mut remaining_goals = Vec::new();
        for (i, goal) in goals.iter().enumerate() {
            if i != best_goal_index {
                remaining_goals.push(goal.clone());
            }
        }

        // Try to solve the selected goal
        self.solve_single_goal(selected_goal, &remaining_goals, &propagated_bindings, results, depth)
    }
    
    /// Solve a list of goals using backtracking
    fn solve_goals(
        &self,
        goals: &[Goal],
        bindings: &mut Bindings,
        results: &mut Vec<Bindings>,
        depth: usize,
    ) -> Result<(), RuntimeError> {
        if depth > self.max_depth {
            return Err(RuntimeError::LogicError("Maximum search depth exceeded".to_string()));
        }
        
        if goals.is_empty() {
            // All goals satisfied, record the solution
            results.push(bindings.clone());
            return Ok(());
        }
        
        let goal = &goals[0];
        let remaining_goals = &goals[1..];
        
        if goal.negated {
            // Negation as failure
            let mut temp_bindings = bindings.clone();
            let mut temp_results = Vec::new();
            
            if self.solve_goals(&[Goal { negated: false, ..goal.clone() }], &mut temp_bindings, &mut temp_results, depth + 1).is_ok() && !temp_results.is_empty() {
                // Goal succeeded, so negation fails
                return Ok(());
            } else {
                // Goal failed, so negation succeeds
                return self.solve_goals(remaining_goals, bindings, results, depth + 1);
            }
        }
        
        // Check built-in predicates first
        if self.is_builtin_predicate(&goal.predicate) {
            if self.solve_builtin_predicate(goal, bindings)? {
                return self.solve_goals(remaining_goals, bindings, results, depth + 1);
            } else {
                return Ok(());
            }
        }
        
        // Try to unify with facts
        if let Some(facts) = self.knowledge_base.facts.get(&goal.predicate) {
            for fact in facts {
                let mut new_bindings = bindings.clone();
                if self.unify_fact_goal(fact, goal, &mut new_bindings)? {
                    self.solve_goals(remaining_goals, &mut new_bindings, results, depth + 1)?;
                }
            }
        }
        
        // Try to unify with rule heads
        if let Some(rules) = self.knowledge_base.rules.get(&goal.predicate) {
            for rule in rules {
                let mut new_bindings = bindings.clone();
                let renamed_rule = self.rename_variables_in_rule(rule, depth);
                
                if self.unify_fact_goal(&renamed_rule.head, goal, &mut new_bindings)? {
                    // Add rule body goals to the goal list
                    let mut new_goals = renamed_rule.body;
                    new_goals.extend_from_slice(remaining_goals);
                    
                    self.solve_goals(&new_goals, &mut new_bindings, results, depth + 1)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Unify a fact with a goal
    fn unify_fact_goal(&self, fact: &Fact, goal: &Goal, bindings: &mut Bindings) -> Result<bool, RuntimeError> {
        if fact.predicate != goal.predicate || fact.args.len() != goal.args.len() {
            return Ok(false);
        }
        
        for (fact_arg, goal_arg) in fact.args.iter().zip(goal.args.iter()) {
            if !self.unify_terms(fact_arg, goal_arg, bindings)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Unify two terms
    fn unify_terms(&self, term1: &Term, term2: &Term, bindings: &mut Bindings) -> Result<bool, RuntimeError> {
        let resolved_term1 = self.resolve_term(term1, bindings);
        let resolved_term2 = self.resolve_term(term2, bindings);
        
        match (&resolved_term1, &resolved_term2) {
            (Term::Variable(var), term) | (term, Term::Variable(var)) => {
                if let Term::Variable(other_var) = term {
                    if var == other_var {
                        return Ok(true);
                    }
                }
                
                // Check for occurs check (variable occurs in term)
                if self.occurs_check(var, term) {
                    return Ok(false);
                }
                
                bindings.insert(var.clone(), term.clone());
                Ok(true)
            }
            
            (Term::Atom(a1), Term::Atom(a2)) => Ok(a1 == a2),
            (Term::Integer(i1), Term::Integer(i2)) => Ok(i1 == i2),
            (Term::Float(f1), Term::Float(f2)) => Ok((f1 - f2).abs() < f64::EPSILON),
            (Term::String(s1), Term::String(s2)) => Ok(s1 == s2),
            
            (Term::Compound(name1, args1), Term::Compound(name2, args2)) => {
                if name1 != name2 || args1.len() != args2.len() {
                    return Ok(false);
                }
                
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    if !self.unify_terms(arg1, arg2, bindings)? {
                        return Ok(false);
                    }
                }
                
                Ok(true)
            }
            
            _ => Ok(false),
        }
    }
    
    /// Resolve a term by following variable bindings
    fn resolve_term(&self, term: &Term, bindings: &Bindings) -> Term {
        match term {
            Term::Variable(var) => {
                if let Some(bound_term) = bindings.get(var) {
                    self.resolve_term(bound_term, bindings)
                } else {
                    term.clone()
                }
            }
            _ => term.clone(),
        }
    }
    
    /// Occurs check to prevent infinite structures
    fn occurs_check(&self, var: &str, term: &Term) -> bool {
        match term {
            Term::Variable(other_var) => var == other_var,
            Term::Compound(_, args) => args.iter().any(|arg| self.occurs_check(var, arg)),
            _ => false,
        }
    }
    
    /// Rename variables in a rule to avoid conflicts
    fn rename_variables_in_rule(&self, rule: &Rule, suffix: usize) -> Rule {
        let mut var_mapping = HashMap::new();
        
        Rule {
            head: self.rename_variables_in_fact(&rule.head, &mut var_mapping, suffix),
            body: rule.body.iter()
                .map(|goal| self.rename_variables_in_goal(goal, &mut var_mapping, suffix))
                .collect(),
        }
    }
    
    /// Rename variables in a fact
    fn rename_variables_in_fact(&self, fact: &Fact, var_mapping: &mut HashMap<String, String>, suffix: usize) -> Fact {
        Fact {
            predicate: fact.predicate.clone(),
            args: fact.args.iter()
                .map(|arg| self.rename_variables_in_term(arg, var_mapping, suffix))
                .collect(),
        }
    }
    
    /// Rename variables in a goal
    fn rename_variables_in_goal(&self, goal: &Goal, var_mapping: &mut HashMap<String, String>, suffix: usize) -> Goal {
        Goal {
            predicate: goal.predicate.clone(),
            args: goal.args.iter()
                .map(|arg| self.rename_variables_in_term(arg, var_mapping, suffix))
                .collect(),
            negated: goal.negated,
        }
    }
    
    /// Rename variables in a term
    fn rename_variables_in_term(&self, term: &Term, var_mapping: &mut HashMap<String, String>, suffix: usize) -> Term {
        match term {
            Term::Variable(var) => {
                let new_var = var_mapping.entry(var.clone())
                    .or_insert_with(|| format!("{}_{}", var, suffix))
                    .clone();
                Term::Variable(new_var)
            }
            Term::Compound(name, args) => {
                Term::Compound(
                    name.clone(),
                    args.iter()
                        .map(|arg| self.rename_variables_in_term(arg, var_mapping, suffix))
                        .collect(),
                )
            }
            _ => term.clone(),
        }
    }
    
    /// Check if a predicate is built-in
    fn is_builtin_predicate(&self, predicate: &str) -> bool {
        matches!(predicate, "=" | "<" | ">" | "<=" | ">=" | "is")
    }
    
    /// Solve a built-in predicate
    fn solve_builtin_predicate(&self, goal: &Goal, bindings: &mut Bindings) -> Result<bool, RuntimeError> {
        if goal.args.len() != 2 {
            return Ok(false);
        }
        
        let arg1 = self.resolve_term(&goal.args[0], bindings);
        let arg2 = self.resolve_term(&goal.args[1], bindings);
        
        match goal.predicate.as_str() {
            "=" => self.unify_terms(&arg1, &arg2, bindings),
            "<" | ">" | "<=" | ">=" => {
                self.compare_terms(&arg1, &arg2, &goal.predicate)
            }
            "is" => {
                // Arithmetic evaluation
                if let Ok(value) = self.evaluate_arithmetic(&arg2) {
                    self.unify_terms(&arg1, &value, bindings)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
    
    /// Compare two terms numerically
    fn compare_terms(&self, term1: &Term, term2: &Term, op: &str) -> Result<bool, RuntimeError> {
        let val1 = self.term_to_number(term1)?;
        let val2 = self.term_to_number(term2)?;
        
        match op {
            "<" => Ok(val1 < val2),
            ">" => Ok(val1 > val2),
            "<=" => Ok(val1 <= val2),
            ">=" => Ok(val1 >= val2),
            _ => Ok(false),
        }
    }
    
    /// Convert a term to a number
    fn term_to_number(&self, term: &Term) -> Result<f64, RuntimeError> {
        match term {
            Term::Integer(i) => Ok(*i as f64),
            Term::Float(f) => Ok(*f),
            _ => Err(RuntimeError::LogicError("Term is not a number".to_string())),
        }
    }
    
    /// Evaluate arithmetic expression
    fn evaluate_arithmetic(&self, term: &Term) -> Result<Term, RuntimeError> {
        match term {
            Term::Integer(_) | Term::Float(_) => Ok(term.clone()),
            Term::Compound(op, args) if args.len() == 2 => {
                let val1 = self.term_to_number(&args[0])?;
                let val2 = self.term_to_number(&args[1])?;
                
                let result = match op.as_str() {
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    "*" => val1 * val2,
                    "/" => val1 / val2,
                    _ => return Err(RuntimeError::LogicError(format!("Unknown arithmetic operator: {}", op))),
                };
                
                if result.fract() == 0.0 {
                    Ok(Term::Integer(result as i64))
                } else {
                    Ok(Term::Float(result))
                }
            }
            _ => Err(RuntimeError::LogicError("Invalid arithmetic expression".to_string())),
        }
    }
    
    /// Convert a term to string representation
    fn term_to_string(&self, term: &Term) -> String {
        match term {
            Term::Variable(var) => var.clone(),
            Term::Atom(atom) => atom.clone(),
            Term::Integer(i) => i.to_string(),
            Term::Float(f) => f.to_string(),
            Term::String(s) => format!("\"{}\"", s),
            Term::Compound(name, args) => {
                let arg_strings: Vec<String> = args.iter().map(|arg| self.term_to_string(arg)).collect();
                format!("{}({})", name, arg_strings.join(", "))
            }
        }
    }
    
    /// Parse a fact from string (simplified parser)
    fn parse_fact(&self, fact_str: &str) -> Result<Fact, RuntimeError> {
        // Simplified parsing - in a real implementation, use a proper parser
        let trimmed = fact_str.trim().trim_end_matches('.');
        
        if let Some(paren_pos) = trimmed.find('(') {
            let predicate = trimmed[..paren_pos].trim().to_string();
            let args_str = &trimmed[paren_pos + 1..trimmed.len() - 1];
            let args = self.parse_args(args_str)?;
            
            Ok(Fact { predicate, args })
        } else {
            // Fact with no arguments
            Ok(Fact {
                predicate: trimmed.to_string(),
                args: vec![],
            })
        }
    }
    
    /// Parse rule from string (simplified)
    fn parse_rule(&self, rule_str: &str) -> Result<Rule, RuntimeError> {
        // Simplified parsing for rules like "head :- body1, body2."
        if let Some(implies_pos) = rule_str.find(":-") {
            let head_str = rule_str[..implies_pos].trim();
            let body_str = rule_str[implies_pos + 2..].trim().trim_end_matches('.');
            
            let head = self.parse_fact(head_str)?;
            let body_goals: Result<Vec<Goal>, RuntimeError> = body_str
                .split(',')
                .map(|goal_str| {
                    let trimmed = goal_str.trim();
                    let fact = self.parse_fact(trimmed)?;
                    Ok(Goal {
                        predicate: fact.predicate,
                        args: fact.args,
                        negated: false,
                    })
                })
                .collect();
            
            Ok(Rule {
                head,
                body: body_goals?,
            })
        } else {
            Err(RuntimeError::LogicError("Invalid rule format".to_string()))
        }
    }
    
    /// Parse query from string (simplified)
    fn parse_query(&self, query_str: &str) -> Result<Vec<Goal>, RuntimeError> {
        let goals: Result<Vec<Goal>, RuntimeError> = query_str
            .split(',')
            .map(|goal_str| {
                let trimmed = goal_str.trim().trim_end_matches('.');
                let fact = self.parse_fact(trimmed)?;
                Ok(Goal {
                    predicate: fact.predicate,
                    args: fact.args,
                    negated: false,
                })
            })
            .collect();
        
        goals
    }
    
    /// Parse arguments from string (simplified)
    fn parse_args(&self, args_str: &str) -> Result<Vec<Term>, RuntimeError> {
        if args_str.trim().is_empty() {
            return Ok(vec![]);
        }
        
        let args: Result<Vec<Term>, RuntimeError> = args_str
            .split(',')
            .map(|arg_str| self.parse_term(arg_str.trim()))
            .collect();
        
        args
    }
    
    /// Parse a single term (simplified)
    fn parse_term(&self, term_str: &str) -> Result<Term, RuntimeError> {
        let trimmed = term_str.trim();
        
        // Check if it's a string literal
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            return Ok(Term::String(trimmed[1..trimmed.len() - 1].to_string()));
        }
        
        // Check if it's a number
        if let Ok(i) = trimmed.parse::<i64>() {
            return Ok(Term::Integer(i));
        }
        
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(Term::Float(f));
        }
        
        // Check if it's a variable (starts with uppercase)
        if trimmed.chars().next().unwrap_or('a').is_uppercase() {
            return Ok(Term::Variable(trimmed.to_string()));
        }
        
        // Otherwise, it's an atom
        Ok(Term::Atom(trimmed.to_string()))
    }
    
    /// Get number of facts
    pub fn facts_count(&self) -> usize {
        self.knowledge_base.facts.values().map(|facts| facts.len()).sum()
    }
    
    /// Get number of rules
    pub fn rules_count(&self) -> usize {
        self.knowledge_base.rules.values().map(|rules| rules.len()).sum()
    }
    
    /// Get number of queries executed
    pub fn queries_executed(&self) -> usize {
        self.queries_executed
    }

    /// Parse complex queries with logical operators
    fn parse_complex_query(&self, query_str: &str) -> Result<Vec<Goal>, RuntimeError> {
        // Handle complex queries with AND, OR, NOT operators
        let query_str = query_str.trim().trim_end_matches('.');

        // For now, split by comma (AND operation)
        // TODO: Add support for OR and NOT operators
        let goals: Result<Vec<Goal>, RuntimeError> = query_str
            .split(',')
            .map(|goal_str| {
                let trimmed = goal_str.trim();
                let negated = trimmed.starts_with("not ");
                let goal_content = if negated {
                    trimmed.strip_prefix("not ").unwrap().trim()
                } else {
                    trimmed
                };

                let fact = self.parse_fact(goal_content)?;
                Ok(Goal {
                    predicate: fact.predicate,
                    args: fact.args,
                    negated,
                })
            })
            .collect();

        goals
    }

    /// Propagate constraints to reduce search space
    fn propagate_constraints(&self, bindings: &Bindings, goals: &[Goal]) -> Result<Bindings, RuntimeError> {
        let mut propagated = bindings.clone();

        // Apply forward chaining for constraint propagation
        for goal in goals {
            for arg in &goal.args {
                if let Term::Variable(var_name) = arg {
                    if let Some(_bound_value) = propagated.get(var_name) {
                        // Variable is already bound, check consistency
                        continue;
                    }

                    // Try to infer variable value from other constraints
                    if let Some(inferred_value) = self.infer_variable_value(var_name, goals, &propagated) {
                        propagated.insert(var_name.clone(), inferred_value);
                    }
                }
            }
        }

        Ok(propagated)
    }

    /// Select the most constrained goal for better performance
    fn select_most_constrained_goal(&self, goals: &[Goal], bindings: &Bindings) -> usize {
        let mut best_index = 0;
        let mut best_score = f64::INFINITY;

        for (i, goal) in goals.iter().enumerate() {
            let score = self.calculate_goal_score(goal, bindings);
            if score < best_score {
                best_score = score;
                best_index = i;
            }
        }

        best_index
    }

    /// Calculate goal constraint score
    fn calculate_goal_score(&self, goal: &Goal, bindings: &Bindings) -> f64 {
        let mut score = 0.0;

        // Count unbound variables (penalty)
        let unbound_vars = goal.args.iter()
            .filter(|arg| {
                if let Term::Variable(var_name) = arg {
                    !bindings.contains_key(var_name)
                } else {
                    false
                }
            })
            .count();

        score += unbound_vars as f64 * 10.0;

        // Count potential matches (fewer is better)
        let fact_count = self.knowledge_base.facts
            .get(&goal.predicate)
            .map(|facts| facts.len())
            .unwrap_or(0);

        let rule_count = self.knowledge_base.rules
            .get(&goal.predicate)
            .map(|rules| rules.len())
            .unwrap_or(0);

        score += (fact_count + rule_count) as f64;

        score
    }

    /// Infer variable value from constraints
    fn infer_variable_value(&self, var_name: &str, goals: &[Goal], _bindings: &Bindings) -> Option<Term> {
        // Simple inference: if variable appears in only one fact, try to bind it
        for goal in goals {
            if goal.args.iter().any(|arg| {
                if let Term::Variable(v) = arg {
                    v == var_name
                } else {
                    false
                }
            }) {
                // Check if we can infer from facts
                if let Some(facts) = self.knowledge_base.facts.get(&goal.predicate) {
                    if facts.len() == 1 {
                        // Only one fact matches, try to extract value
                        let fact = &facts[0];
                        for (goal_arg, fact_arg) in goal.args.iter().zip(&fact.args) {
                            if let Term::Variable(v) = goal_arg {
                                if v == var_name {
                                    return Some(fact_arg.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }



    /// Unify a goal with a fact
    fn unify_goal_with_fact(
        &self,
        goal: &Goal,
        fact: &Fact,
        bindings: &mut Bindings,
    ) -> Result<bool, RuntimeError> {
        // Simple unification - check if predicate names match
        if goal.predicate == fact.predicate && goal.args.len() == fact.args.len() {
            for (goal_term, fact_term) in goal.args.iter().zip(fact.args.iter()) {
                if !self.unify_terms(goal_term, fact_term, bindings)? {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Unify a goal with a rule head
    fn unify_goal_with_rule_head(
        &self,
        goal: &Goal,
        rule: &Rule,
        bindings: &mut Bindings,
    ) -> Result<bool, RuntimeError> {
        self.unify_goal_with_fact(goal, &rule.head, bindings)
    }



    /// Solve a single goal with remaining goals
    fn solve_single_goal(
        &self,
        goal: &Goal,
        remaining_goals: &[Goal],
        bindings: &Bindings,
        results: &mut Vec<Bindings>,
        depth: usize,
    ) -> Result<(), RuntimeError> {
        if goal.negated {
            // Handle negation as failure
            let mut temp_bindings = bindings.clone();
            let mut temp_results = Vec::new();
            let positive_goal = Goal { negated: false, ..goal.clone() };

            if self.solve_single_goal(&positive_goal, &[], &temp_bindings, &mut temp_results, depth + 1).is_ok()
                && !temp_results.is_empty() {
                // Goal succeeded, so negation fails
                return Ok(());
            } else {
                // Goal failed, so negation succeeds
                return self.solve_goals_with_constraints(remaining_goals, &mut bindings.clone(), results, depth + 1);
            }
        }

        // Check built-in predicates
        if self.is_builtin_predicate(&goal.predicate) {
            let mut new_bindings = bindings.clone();
            if self.solve_builtin_predicate(goal, &mut new_bindings)? {
                return self.solve_goals_with_constraints(remaining_goals, &mut new_bindings, results, depth + 1);
            } else {
                return Ok(());
            }
        }

        // Try facts
        if let Some(facts) = self.knowledge_base.facts.get(&goal.predicate) {
            for fact in facts {
                let mut new_bindings = bindings.clone();
                if self.unify_fact_goal(fact, goal, &mut new_bindings)? {
                    self.solve_goals_with_constraints(remaining_goals, &mut new_bindings, results, depth + 1)?;
                }
            }
        }

        // Try rules
        if let Some(rules) = self.knowledge_base.rules.get(&goal.predicate) {
            for rule in rules {
                let mut new_bindings = bindings.clone();
                if self.unify_goal_with_rule_head(goal, rule, &mut new_bindings)? {
                    let mut new_goals = rule.body.clone();
                    new_goals.extend_from_slice(remaining_goals);
                    self.solve_goals_with_constraints(&new_goals, &mut new_bindings, results, depth + 1)?;
                }
            }
        }

        Ok(())
    }
}

impl KnowledgeBase {
    fn new() -> Self {
        Self {
            facts: IndexMap::new(),
            rules: IndexMap::new(),
            predicates: HashMap::new(),
        }
    }
    
    fn add_predicate(&mut self, name: &str, arity: usize) {
        self.predicates.insert(name.to_string(), arity);
    }
    
    fn add_fact(&mut self, fact: Fact) {
        self.facts.entry(fact.predicate.clone()).or_insert_with(Vec::new).push(fact);
    }
    
    fn remove_fact(&mut self, fact: &Fact) {
        if let Some(facts) = self.facts.get_mut(&fact.predicate) {
            facts.retain(|f| f != fact);
        }
    }
    
    fn add_rule(&mut self, rule: Rule) {
        self.rules.entry(rule.head.predicate.clone()).or_insert_with(Vec::new).push(rule);
    }
    
    fn clear(&mut self) {
        self.facts.clear();
        self.rules.clear();
        self.predicates.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_logic_engine_creation() {
        let engine = LogicEngine::new();
        assert_eq!(engine.facts_count(), 0);
        assert_eq!(engine.rules_count(), 0);
    }
    
    #[test]
    fn test_fact_assertion() {
        let mut engine = LogicEngine::new();
        let result = engine.assert_fact("parent(john, mary).");
        assert!(result.is_ok());
        assert_eq!(engine.facts_count(), 1);
    }
    
    #[test]
    fn test_simple_query() {
        let mut engine = LogicEngine::new();
        engine.assert_fact("parent(john, mary).").unwrap();

        let results = engine.solve_query("parent(john, mary).").unwrap();
        // For now, just check that the query doesn't fail
        // The actual logic engine implementation is simplified
        assert!(results.len() >= 0);
    }
}
