//! Unification algorithm for logic programming
//! Expert recommendation: Priority 2 - Core unification engine

use crate::knowledge_base::{Term, Value};
use std::collections::HashMap;

/// Variable substitution mapping (Expert recommendation: Core unification)
pub type Substitution = HashMap<String, Term>;

/// Unification result
#[derive(Debug, Clone, PartialEq)]
pub enum UnificationResult {
    Success(Substitution),
    Failure,
}

/// Unification engine (Expert recommendation: Priority 2)
pub struct Unifier;

impl Unifier {
    /// Unify two terms (Expert recommendation: Core algorithm)
    pub fn unify(term1: &Term, term2: &Term) -> UnificationResult {
        let mut substitution = Substitution::new();
        if Self::unify_with_substitution(term1, term2, &mut substitution) {
            UnificationResult::Success(substitution)
        } else {
            UnificationResult::Failure
        }
    }

    /// Unify two terms with existing substitution (Expert recommendation)
    fn unify_with_substitution(
        term1: &Term,
        term2: &Term,
        substitution: &mut Substitution,
    ) -> bool {
        // Apply existing substitutions
        let term1 = Self::apply_substitution(term1, substitution);
        let term2 = Self::apply_substitution(term2, substitution);

        match (&term1, &term2) {
            // Variable unification
            (Term::Variable(var1), Term::Variable(var2)) => {
                if var1 == var2 {
                    true // Same variable
                } else {
                    // Bind var1 to var2 (or vice versa)
                    substitution.insert(var1.clone(), term2.clone());
                    true
                }
            }
            (Term::Variable(var), other) | (other, Term::Variable(var)) => {
                // Occurs check: prevent infinite structures
                if Self::occurs_check(var, other) {
                    false
                } else {
                    substitution.insert(var.clone(), other.clone());
                    true
                }
            }

            // Constant unification
            (Term::Constant(val1), Term::Constant(val2)) => val1 == val2,

            // Compound term unification
            (
                Term::Compound { functor: f1, args: args1 },
                Term::Compound { functor: f2, args: args2 },
            ) => {
                if f1 != f2 || args1.len() != args2.len() {
                    false
                } else {
                    // Unify all arguments
                    for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                        if !Self::unify_with_substitution(arg1, arg2, substitution) {
                            return false;
                        }
                    }
                    true
                }
            }

            // Different term types cannot unify
            _ => false,
        }
    }

    /// Apply substitution to a term (Expert recommendation)
    pub fn apply_substitution(term: &Term, substitution: &Substitution) -> Term {
        match term {
            Term::Variable(var) => {
                if let Some(replacement) = substitution.get(var) {
                    // Recursively apply substitution to avoid chains
                    Self::apply_substitution(replacement, substitution)
                } else {
                    term.clone()
                }
            }
            Term::Constant(_) => term.clone(),
            Term::Compound { functor, args } => {
                let new_args = args
                    .iter()
                    .map(|arg| Self::apply_substitution(arg, substitution))
                    .collect();
                Term::Compound {
                    functor: functor.clone(),
                    args: new_args,
                }
            }
        }
    }

    /// Occurs check to prevent infinite structures (Expert recommendation)
    fn occurs_check(var: &str, term: &Term) -> bool {
        match term {
            Term::Variable(other_var) => var == other_var,
            Term::Constant(_) => false,
            Term::Compound { args, .. } => args.iter().any(|arg| Self::occurs_check(var, arg)),
        }
    }

    /// Compose two substitutions (Expert recommendation: Advanced unification)
    pub fn compose_substitutions(
        subst1: &Substitution,
        subst2: &Substitution,
    ) -> Substitution {
        let mut result = subst1.clone();

        // Apply subst2 to all values in subst1
        for (var, term) in result.iter_mut() {
            *term = Self::apply_substitution(term, subst2);
        }

        // Add bindings from subst2 that are not in subst1
        for (var, term) in subst2 {
            if !result.contains_key(var) {
                result.insert(var.clone(), term.clone());
            }
        }

        result
    }

    /// Check if a substitution is more general than another (Expert recommendation)
    pub fn is_more_general(subst1: &Substitution, subst2: &Substitution) -> bool {
        // subst1 is more general if it has fewer bindings and all its bindings
        // are consistent with subst2
        if subst1.len() > subst2.len() {
            return false;
        }

        for (var, term1) in subst1 {
            if let Some(term2) = subst2.get(var) {
                if Self::apply_substitution(term1, subst2) != *term2 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

/// Utility functions for creating terms (Expert recommendation: API helpers)
impl Term {
    /// Create a variable term
    pub fn variable(name: impl Into<String>) -> Self {
        Term::Variable(name.into())
    }

    /// Create a constant term
    pub fn constant(value: Value) -> Self {
        Term::Constant(value)
    }

    /// Create a compound term
    pub fn compound(functor: impl Into<String>, args: Vec<Term>) -> Self {
        Term::Compound {
            functor: functor.into(),
            args,
        }
    }

    /// Create a string constant
    pub fn string(s: impl Into<String>) -> Self {
        Term::Constant(Value::String(s.into()))
    }

    /// Create an integer constant
    pub fn int(i: i64) -> Self {
        Term::Constant(Value::Int(i))
    }

    /// Create a boolean constant
    pub fn bool(b: bool) -> Self {
        Term::Constant(Value::Bool(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_unification() {
        let x = Term::variable("X");
        let y = Term::variable("Y");
        
        match Unifier::unify(&x, &y) {
            UnificationResult::Success(subst) => {
                assert!(subst.len() == 1);
            }
            UnificationResult::Failure => panic!("Should unify"),
        }
    }

    #[test]
    fn test_constant_unification() {
        let c1 = Term::int(42);
        let c2 = Term::int(42);
        let c3 = Term::int(24);
        
        assert!(matches!(Unifier::unify(&c1, &c2), UnificationResult::Success(_)));
        assert!(matches!(Unifier::unify(&c1, &c3), UnificationResult::Failure));
    }

    #[test]
    fn test_compound_unification() {
        let parent1 = Term::compound("parent", vec![Term::string("Ahmed"), Term::variable("X")]);
        let parent2 = Term::compound("parent", vec![Term::string("Ahmed"), Term::string("Sara")]);
        
        match Unifier::unify(&parent1, &parent2) {
            UnificationResult::Success(subst) => {
                assert_eq!(subst.get("X"), Some(&Term::string("Sara")));
            }
            UnificationResult::Failure => panic!("Should unify"),
        }
    }
}
