//! Logic solver with backtracking
//! Expert recommendation: Priority 2 - Core solver implementation

use crate::knowledge_base::{KnowledgeBase, Term, Fact, Rule, Value};
use crate::unification::{Unifier, UnificationResult, Substitution};
use std::collections::VecDeque;

/// Query result (Expert recommendation: API result type)
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub success: bool,
    pub bindings: Vec<Substitution>,
}

/// Solution iterator for query_solve (Expert recommendation: Priority 1)
#[derive(Debug)]
pub struct SolutionIterator<'a> {
    solver: &'a LogicSolver<'a>,
    goal: Term,
    solutions: Vec<Substitution>,
    current_index: usize,
    exhausted: bool,
}

/// Solution handle for FFI (Expert recommendation: Priority 1)
#[derive(Debug, Clone)]
pub struct Solution {
    pub bindings: Substitution,
    pub variables: Vec<String>,
}

/// Goal in the resolution process (Expert recommendation)
#[derive(Debug, Clone)]
struct Goal {
    term: Term,
    substitution: Substitution,
}

/// Logic solver with SLD resolution and backtracking (Expert recommendation: Priority 2)
#[derive(Debug)]
pub struct LogicSolver<'a> {
    knowledge_base: &'a KnowledgeBase,
}

impl<'a> LogicSolver<'a> {
    /// Create a new solver with a knowledge base
    pub fn new(knowledge_base: &'a KnowledgeBase) -> Self {
        Self { knowledge_base }
    }

    /// Prove a query goal (Expert recommendation: Main API function)
    pub fn prove(&self, goal: &Term) -> QueryResult {
        let mut solutions = Vec::new();
        let initial_goal = Goal {
            term: goal.clone(),
            substitution: Substitution::new(),
        };

        self.solve_goal(&initial_goal, &mut solutions);

        QueryResult {
            success: !solutions.is_empty(),
            bindings: solutions,
        }
    }

    /// Solve a single goal using SLD resolution (Expert recommendation)
    fn solve_goal(&self, goal: &Goal, solutions: &mut Vec<Substitution>) {
        let goal_term = Unifier::apply_substitution(&goal.term, &goal.substitution);

        // Try to match against facts
        self.try_facts(&goal_term, &goal.substitution, solutions);

        // Try to match against rules
        self.try_rules(&goal_term, &goal.substitution, solutions);
    }

    /// Try to unify goal with facts (Expert recommendation - Enhanced with indexing)
    fn try_facts(&self, goal_term: &Term, substitution: &Substitution, solutions: &mut Vec<Substitution>) {
        if let Term::Compound { functor, args } = goal_term {
            // Use indexed lookup if first argument is a constant (Expert recommendation: Priority 2)
            let facts_to_check: Vec<&Fact> = if !args.is_empty() {
                match &args[0] {
                    Term::Constant(value) => {
                        // Use indexed lookup for performance
                        self.knowledge_base.get_facts_by_first_arg(functor, value)
                    }
                    _ => {
                        // Fall back to full scan for variables or complex terms
                        if let Some(facts) = self.knowledge_base.get_facts(functor) {
                            facts.iter().collect()
                        } else {
                            Vec::new()
                        }
                    }
                }
            } else {
                // No arguments, use full scan
                if let Some(facts) = self.knowledge_base.get_facts(functor) {
                    facts.iter().collect()
                } else {
                    Vec::new()
                }
            };

            for fact in facts_to_check {
                // Convert fact to term
                let fact_term = self.fact_to_term(fact);

                // Try to unify
                match Unifier::unify(goal_term, &fact_term) {
                    UnificationResult::Success(unifier) => {
                        // Compose with existing substitution
                        let final_substitution = Unifier::compose_substitutions(substitution, &unifier);
                        solutions.push(final_substitution);
                    }
                    UnificationResult::Failure => {
                        // Continue to next fact
                    }
                }
            }
        }
    }

    /// Try to unify goal with rule heads and solve rule bodies (Expert recommendation - Enhanced with indexing)
    fn try_rules(&self, goal_term: &Term, substitution: &Substitution, solutions: &mut Vec<Substitution>) {
        if let Term::Compound { functor, args } = goal_term {
            // Use indexed lookup if first argument is a constant (Expert recommendation: Priority 2)
            let rules_to_check: Vec<&Rule> = if !args.is_empty() {
                match &args[0] {
                    Term::Constant(_) => {
                        // Use indexed lookup for performance
                        self.knowledge_base.get_rules_by_first_arg(functor, &args[0])
                    }
                    _ => {
                        // Fall back to full scan for variables or complex terms
                        if let Some(rules) = self.knowledge_base.get_rules(functor) {
                            rules.iter().collect()
                        } else {
                            Vec::new()
                        }
                    }
                }
            } else {
                // No arguments, use full scan
                if let Some(rules) = self.knowledge_base.get_rules(functor) {
                    rules.iter().collect()
                } else {
                    Vec::new()
                }
            };

            for rule in rules_to_check {
                // Rename variables in rule to avoid conflicts
                let renamed_rule = self.rename_rule_variables(rule);

                // Try to unify goal with rule head
                match Unifier::unify(goal_term, &renamed_rule.head) {
                    UnificationResult::Success(head_unifier) => {
                        // Compose with existing substitution
                        let new_substitution = Unifier::compose_substitutions(substitution, &head_unifier);

                        // Solve rule body
                        self.solve_body(&renamed_rule.body, &new_substitution, solutions);
                    }
                    UnificationResult::Failure => {
                        // Continue to next rule
                    }
                }
            }
        }
    }

    /// Solve a rule body (conjunction of goals) (Expert recommendation)
    fn solve_body(&self, body: &[Term], substitution: &Substitution, solutions: &mut Vec<Substitution>) {
        if body.is_empty() {
            // Empty body means success
            solutions.push(substitution.clone());
            return;
        }

        // Solve first goal in body
        let first_goal = &body[0];
        let remaining_goals = &body[1..];

        let goal = Goal {
            term: first_goal.clone(),
            substitution: substitution.clone(),
        };

        let mut intermediate_solutions = Vec::new();
        self.solve_goal(&goal, &mut intermediate_solutions);

        // For each solution of the first goal, solve the remaining goals
        for solution in intermediate_solutions {
            self.solve_body(remaining_goals, &solution, solutions);
        }
    }

    /// Convert a fact to a term for unification (Expert recommendation)
    fn fact_to_term(&self, fact: &Fact) -> Term {
        let args = fact.args.iter()
            .map(|value| Term::Constant(value.clone()))
            .collect();

        Term::Compound {
            functor: fact.relation.clone(),
            args,
        }
    }

    /// Rename variables in a rule to avoid conflicts (Expert recommendation)
    fn rename_rule_variables(&self, rule: &Rule) -> Rule {
        // Simple renaming strategy: add a unique suffix
        let suffix = format!("_{}", self.generate_unique_id());

        Rule {
            head: self.rename_term_variables(&rule.head, &suffix),
            body: rule.body.iter()
                .map(|term| self.rename_term_variables(term, &suffix))
                .collect(),
        }
    }

    /// Rename variables in a term (Expert recommendation)
    fn rename_term_variables(&self, term: &Term, suffix: &str) -> Term {
        match term {
            Term::Variable(name) => Term::Variable(format!("{}{}", name, suffix)),
            Term::Constant(value) => Term::Constant(value.clone()),
            Term::Compound { functor, args } => {
                let renamed_args = args.iter()
                    .map(|arg| self.rename_term_variables(arg, suffix))
                    .collect();
                Term::Compound {
                    functor: functor.clone(),
                    args: renamed_args,
                }
            }
        }
    }

    /// Generate a unique ID for variable renaming (Expert recommendation)
    fn generate_unique_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }

    /// Check if a query has any solutions (Expert recommendation: Utility function)
    pub fn can_prove(&self, goal: &Term) -> bool {
        let result = self.prove(goal);
        result.success
    }

    /// Get the first solution for a query (Expert recommendation: Utility function)
    pub fn prove_once(&self, goal: &Term) -> Option<Substitution> {
        let result = self.prove(goal);
        result.bindings.into_iter().next()
    }

    /// Get all variables in a term that would be bound by a query (Expert recommendation)
    pub fn get_query_variables(&self, goal: &Term) -> Vec<String> {
        goal.get_variables().into_iter().cloned().collect()
    }

    /// Create a solution iterator for query_solve (Expert recommendation: Priority 1)
    pub fn solve_iter(&self, goal: &Term) -> SolutionIterator {
        let result = self.prove(goal);
        SolutionIterator {
            solver: self,
            goal: goal.clone(),
            solutions: result.bindings,
            current_index: 0,
            exhausted: false,
        }
    }
}

impl<'a> SolutionIterator<'a> {
    /// Get the next solution (Expert recommendation: Priority 1)
    pub fn next_solution(&mut self) -> Option<Solution> {
        if self.exhausted || self.current_index >= self.solutions.len() {
            return None;
        }

        let substitution = &self.solutions[self.current_index];
        self.current_index += 1;

        // Extract variable names from the goal
        let variables = self.solver.get_query_variables(&self.goal);

        Some(Solution {
            bindings: substitution.clone(),
            variables,
        })
    }

    /// Check if there are more solutions (Expert recommendation: Priority 1)
    pub fn has_more(&self) -> bool {
        !self.exhausted && self.current_index < self.solutions.len()
    }

    /// Reset the iterator (Expert recommendation: Priority 1)
    pub fn reset(&mut self) {
        self.current_index = 0;
        self.exhausted = false;
    }

    /// Get total number of solutions (Expert recommendation: Priority 1)
    pub fn solution_count(&self) -> usize {
        self.solutions.len()
    }
}

impl Solution {
    /// Get the value of a variable (Expert recommendation: Priority 1)
    pub fn get_variable_value(&self, var_name: &str) -> Option<&Term> {
        self.bindings.get(var_name)
    }

    /// Get all variable names (Expert recommendation: Priority 1)
    pub fn get_variable_names(&self) -> &Vec<String> {
        &self.variables
    }

    /// Check if a variable is bound (Expert recommendation: Priority 1)
    pub fn is_variable_bound(&self, var_name: &str) -> bool {
        self.bindings.contains_key(var_name)
    }

    /// Get all bindings as a map (Expert recommendation: Priority 1)
    pub fn get_all_bindings(&self) -> &Substitution {
        &self.bindings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge_base::*;

    #[test]
    fn test_simple_fact_query() {
        let mut kb = KnowledgeBase::new();

        // Register relation
        kb.register_relation("parent".to_string(), 2, vec!["string".to_string(), "string".to_string()]);

        // Add fact: parent("Ahmed", "Sara")
        kb.assert_fact("parent".to_string(), vec![
            Value::String("Ahmed".to_string()),
            Value::String("Sara".to_string()),
        ]).unwrap();

        let solver = LogicSolver::new(&kb);

        // Query: parent("Ahmed", "Sara")
        let query = Term::compound("parent", vec![
            Term::string("Ahmed"),
            Term::string("Sara"),
        ]);

        let result = solver.prove(&query);
        assert!(result.success);
    }

    #[test]
    fn test_variable_query() {
        let mut kb = KnowledgeBase::new();

        // Register relation
        kb.register_relation("parent".to_string(), 2, vec!["string".to_string(), "string".to_string()]);

        // Add fact: parent("Ahmed", "Sara")
        kb.assert_fact("parent".to_string(), vec![
            Value::String("Ahmed".to_string()),
            Value::String("Sara".to_string()),
        ]).unwrap();

        let solver = LogicSolver::new(&kb);

        // Query: parent("Ahmed", ?X)
        let query = Term::compound("parent", vec![
            Term::string("Ahmed"),
            Term::variable("X"),
        ]);

        let result = solver.prove(&query);
        assert!(result.success);
        assert_eq!(result.bindings.len(), 1);

        let binding = &result.bindings[0];
        assert_eq!(binding.get("X"), Some(&Term::string("Sara")));
    }
}
