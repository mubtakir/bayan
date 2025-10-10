//! Code linter for AlBayan language
//! 
//! Provides static analysis and code quality checks

use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Code linter for AlBayan
#[derive(Debug)]
pub struct Linter {
    /// Linting rules configuration
    config: LinterConfig,
    /// Custom rules
    custom_rules: Vec<Box<dyn LintRule>>,
}

/// Linting configuration
#[derive(Debug, Clone)]
pub struct LinterConfig {
    /// Enable/disable specific rules
    pub enabled_rules: HashMap<String, bool>,
    /// Rule severity levels
    pub rule_severities: HashMap<String, Severity>,
    /// Maximum line length
    pub max_line_length: usize,
    /// Maximum function length
    pub max_function_length: usize,
    /// Maximum cyclomatic complexity
    pub max_complexity: usize,
    /// Naming conventions
    pub naming_conventions: NamingConventions,
}

/// Naming conventions
#[derive(Debug, Clone)]
pub struct NamingConventions {
    /// Function naming style
    pub function_style: NamingStyle,
    /// Variable naming style
    pub variable_style: NamingStyle,
    /// Constant naming style
    pub constant_style: NamingStyle,
    /// Type naming style
    pub type_style: NamingStyle,
}

/// Naming style
#[derive(Debug, Clone)]
pub enum NamingStyle {
    /// snake_case
    SnakeCase,
    /// camelCase
    CamelCase,
    /// PascalCase
    PascalCase,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnakeCase,
    /// kebab-case
    KebabCase,
}

/// Lint issue severity
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    /// Error - must be fixed
    Error,
    /// Warning - should be fixed
    Warning,
    /// Info - suggestion
    Info,
    /// Hint - minor suggestion
    Hint,
}

/// Lint issue
#[derive(Debug, Clone)]
pub struct LintIssue {
    /// Rule that triggered this issue
    pub rule: String,
    /// Issue severity
    pub severity: Severity,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Issue message
    pub message: String,
    /// Suggested fix
    pub suggestion: Option<String>,
}

/// Lint rule trait
pub trait LintRule: std::fmt::Debug {
    /// Rule name
    fn name(&self) -> &str;
    /// Rule description
    fn description(&self) -> &str;
    /// Check source code for issues
    fn check(&self, source: &str) -> Result<Vec<LintIssue>>;
}

impl Linter {
    /// Create a new linter
    pub fn new() -> Self {
        let mut linter = Self {
            config: LinterConfig::default(),
            custom_rules: Vec::new(),
        };
        
        // Add built-in rules
        linter.add_builtin_rules();
        linter
    }
    
    /// Create linter with custom configuration
    pub fn with_config(config: LinterConfig) -> Self {
        let mut linter = Self {
            config,
            custom_rules: Vec::new(),
        };
        
        linter.add_builtin_rules();
        linter
    }
    
    /// Add built-in lint rules
    fn add_builtin_rules(&mut self) {
        self.custom_rules.push(Box::new(LineLengthRule::new(self.config.max_line_length)));
        self.custom_rules.push(Box::new(UnusedVariableRule::new()));
        self.custom_rules.push(Box::new(NamingConventionRule::new(self.config.naming_conventions.clone())));
        self.custom_rules.push(Box::new(ComplexityRule::new(self.config.max_complexity)));
        self.custom_rules.push(Box::new(DeadCodeRule::new()));
    }
    
    /// Add a custom lint rule
    pub fn add_rule(&mut self, rule: Box<dyn LintRule>) {
        self.custom_rules.push(rule);
    }
    
    /// Analyze source code
    pub fn analyze(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        
        for rule in &self.custom_rules {
            if self.is_rule_enabled(rule.name()) {
                let rule_issues = rule.check(source)?;
                issues.extend(rule_issues);
            }
        }
        
        // Sort issues by line number
        issues.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        
        Ok(issues)
    }
    
    /// Check if a rule is enabled
    fn is_rule_enabled(&self, rule_name: &str) -> bool {
        self.config.enabled_rules.get(rule_name).copied().unwrap_or(true)
    }
    
    /// Get rule severity
    fn get_rule_severity(&self, rule_name: &str) -> Severity {
        self.config.rule_severities.get(rule_name).cloned().unwrap_or(Severity::Warning)
    }
}

/// Line length rule
#[derive(Debug)]
struct LineLengthRule {
    max_length: usize,
}

impl LineLengthRule {
    fn new(max_length: usize) -> Self {
        Self { max_length }
    }
}

impl LintRule for LineLengthRule {
    fn name(&self) -> &str {
        "line_length"
    }
    
    fn description(&self) -> &str {
        "Lines should not exceed maximum length"
    }
    
    fn check(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        
        for (line_num, line) in source.lines().enumerate() {
            if line.len() > self.max_length {
                issues.push(LintIssue {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: line_num + 1,
                    column: self.max_length + 1,
                    message: format!("Line too long ({} > {})", line.len(), self.max_length),
                    suggestion: Some("Consider breaking this line".to_string()),
                });
            }
        }
        
        Ok(issues)
    }
}

/// Unused variable rule
#[derive(Debug)]
struct UnusedVariableRule;

impl UnusedVariableRule {
    fn new() -> Self {
        Self
    }
}

impl LintRule for UnusedVariableRule {
    fn name(&self) -> &str {
        "unused_variable"
    }
    
    fn description(&self) -> &str {
        "Variables should be used after declaration"
    }
    
    fn check(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        let mut declared_vars = HashSet::new();
        let mut used_vars = HashSet::new();
        
        // Simple pattern matching for variable declarations and usage
        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            
            // Check for variable declarations (let keyword)
            if trimmed.starts_with("let ") {
                if let Some(var_name) = self.extract_variable_name(trimmed) {
                    declared_vars.insert((var_name.clone(), line_num + 1));
                }
            }
            
            // Check for variable usage
            for word in trimmed.split_whitespace() {
                if declared_vars.iter().any(|(name, _)| name == word) {
                    used_vars.insert(word.to_string());
                }
            }
        }
        
        // Find unused variables
        for (var_name, line_num) in declared_vars {
            if !used_vars.contains(&var_name) {
                issues.push(LintIssue {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: line_num,
                    column: 1,
                    message: format!("Variable '{}' is declared but never used", var_name),
                    suggestion: Some(format!("Remove unused variable '{}'", var_name)),
                });
            }
        }
        
        Ok(issues)
    }
}

impl UnusedVariableRule {
    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // Simple extraction: "let var_name = ..."
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "let" {
            Some(parts[1].trim_end_matches('=').to_string())
        } else {
            None
        }
    }
}

/// Naming convention rule
#[derive(Debug)]
struct NamingConventionRule {
    conventions: NamingConventions,
}

impl NamingConventionRule {
    fn new(conventions: NamingConventions) -> Self {
        Self { conventions }
    }
    
    fn check_naming_style(&self, name: &str, expected_style: &NamingStyle) -> bool {
        match expected_style {
            NamingStyle::SnakeCase => name.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '_'),
            NamingStyle::CamelCase => name.chars().next().map_or(false, |c| c.is_lowercase()) && !name.contains('_'),
            NamingStyle::PascalCase => name.chars().next().map_or(false, |c| c.is_uppercase()) && !name.contains('_'),
            NamingStyle::ScreamingSnakeCase => name.chars().all(|c| c.is_uppercase() || c.is_numeric() || c == '_'),
            NamingStyle::KebabCase => name.chars().all(|c| c.is_lowercase() || c.is_numeric() || c == '-'),
        }
    }
}

impl LintRule for NamingConventionRule {
    fn name(&self) -> &str {
        "naming_convention"
    }
    
    fn description(&self) -> &str {
        "Names should follow naming conventions"
    }
    
    fn check(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        
        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            
            // Check function names
            if trimmed.starts_with("fn ") {
                if let Some(func_name) = self.extract_function_name(trimmed) {
                    if !self.check_naming_style(&func_name, &self.conventions.function_style) {
                        issues.push(LintIssue {
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            line: line_num + 1,
                            column: 1,
                            message: format!("Function name '{}' doesn't follow naming convention", func_name),
                            suggestion: Some(format!("Use {:?} naming style", self.conventions.function_style)),
                        });
                    }
                }
            }
            
            // Check variable names
            if trimmed.starts_with("let ") {
                if let Some(var_name) = self.extract_variable_name(trimmed) {
                    if !self.check_naming_style(&var_name, &self.conventions.variable_style) {
                        issues.push(LintIssue {
                            rule: self.name().to_string(),
                            severity: Severity::Warning,
                            line: line_num + 1,
                            column: 1,
                            message: format!("Variable name '{}' doesn't follow naming convention", var_name),
                            suggestion: Some(format!("Use {:?} naming style", self.conventions.variable_style)),
                        });
                    }
                }
            }
        }
        
        Ok(issues)
    }
}

impl NamingConventionRule {
    fn extract_function_name(&self, line: &str) -> Option<String> {
        // Simple extraction: "fn function_name("
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "fn" {
            Some(parts[1].trim_end_matches('(').to_string())
        } else {
            None
        }
    }
    
    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // Simple extraction: "let var_name = ..."
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "let" {
            Some(parts[1].trim_end_matches('=').to_string())
        } else {
            None
        }
    }
}

/// Complexity rule
#[derive(Debug)]
struct ComplexityRule {
    max_complexity: usize,
}

impl ComplexityRule {
    fn new(max_complexity: usize) -> Self {
        Self { max_complexity }
    }
}

impl LintRule for ComplexityRule {
    fn name(&self) -> &str {
        "complexity"
    }
    
    fn description(&self) -> &str {
        "Functions should not be too complex"
    }
    
    fn check(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        let mut in_function = false;
        let mut function_start = 0;
        let mut complexity = 1; // Base complexity
        
        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("fn ") {
                in_function = true;
                function_start = line_num + 1;
                complexity = 1;
            } else if in_function && trimmed == "}" {
                if complexity > self.max_complexity {
                    issues.push(LintIssue {
                        rule: self.name().to_string(),
                        severity: Severity::Warning,
                        line: function_start,
                        column: 1,
                        message: format!("Function complexity ({}) exceeds maximum ({})", complexity, self.max_complexity),
                        suggestion: Some("Consider breaking this function into smaller functions".to_string()),
                    });
                }
                in_function = false;
            } else if in_function {
                // Count complexity-increasing constructs
                if trimmed.contains("if ") || trimmed.contains("else") ||
                   trimmed.contains("while ") || trimmed.contains("for ") ||
                   trimmed.contains("match ") || trimmed.contains("&&") ||
                   trimmed.contains("||") {
                    complexity += 1;
                }
            }
        }
        
        Ok(issues)
    }
}

/// Dead code rule
#[derive(Debug)]
struct DeadCodeRule;

impl DeadCodeRule {
    fn new() -> Self {
        Self
    }
}

impl LintRule for DeadCodeRule {
    fn name(&self) -> &str {
        "dead_code"
    }
    
    fn description(&self) -> &str {
        "Remove unreachable code"
    }
    
    fn check(&self, source: &str) -> Result<Vec<LintIssue>> {
        let mut issues = Vec::new();
        let mut after_return = false;
        
        for (line_num, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("return") {
                after_return = true;
            } else if after_return && !trimmed.is_empty() && !trimmed.starts_with('}') {
                issues.push(LintIssue {
                    rule: self.name().to_string(),
                    severity: Severity::Warning,
                    line: line_num + 1,
                    column: 1,
                    message: "Unreachable code after return statement".to_string(),
                    suggestion: Some("Remove unreachable code".to_string()),
                });
                after_return = false; // Only report once per block
            } else if trimmed.starts_with('}') {
                after_return = false;
            }
        }
        
        Ok(issues)
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LinterConfig {
    fn default() -> Self {
        let mut enabled_rules = HashMap::new();
        enabled_rules.insert("line_length".to_string(), true);
        enabled_rules.insert("unused_variable".to_string(), true);
        enabled_rules.insert("naming_convention".to_string(), true);
        enabled_rules.insert("complexity".to_string(), true);
        enabled_rules.insert("dead_code".to_string(), true);
        
        Self {
            enabled_rules,
            rule_severities: HashMap::new(),
            max_line_length: 100,
            max_function_length: 50,
            max_complexity: 10,
            naming_conventions: NamingConventions::default(),
        }
    }
}

impl Default for NamingConventions {
    fn default() -> Self {
        Self {
            function_style: NamingStyle::SnakeCase,
            variable_style: NamingStyle::SnakeCase,
            constant_style: NamingStyle::ScreamingSnakeCase,
            type_style: NamingStyle::PascalCase,
        }
    }
}
