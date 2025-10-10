//! Code formatter for AlBayan language
//! 
//! Provides automatic code formatting with configurable style options

use anyhow::Result;

/// Code formatter for AlBayan
#[derive(Debug)]
pub struct CodeFormatter {
    /// Formatting configuration
    config: FormatterConfig,
}

/// Formatting configuration
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Number of spaces for indentation
    pub indent_size: usize,
    /// Use tabs instead of spaces
    pub use_tabs: bool,
    /// Maximum line length
    pub max_line_length: usize,
    /// Insert final newline
    pub insert_final_newline: bool,
    /// Trim trailing whitespace
    pub trim_trailing_whitespace: bool,
    /// Space before function parentheses
    pub space_before_function_paren: bool,
    /// Space inside parentheses
    pub space_inside_parens: bool,
    /// Space around operators
    pub space_around_operators: bool,
    /// Brace style
    pub brace_style: BraceStyle,
    /// Line ending style
    pub line_ending: LineEnding,
}

/// Brace placement style
#[derive(Debug, Clone)]
pub enum BraceStyle {
    /// Same line as declaration
    SameLine,
    /// Next line
    NextLine,
    /// Next line indented
    NextLineIndented,
}

/// Line ending style
#[derive(Debug, Clone)]
pub enum LineEnding {
    /// Unix style (\n)
    Unix,
    /// Windows style (\r\n)
    Windows,
    /// Mac style (\r)
    Mac,
}

impl CodeFormatter {
    /// Create a new code formatter
    pub fn new() -> Self {
        Self {
            config: FormatterConfig::default(),
        }
    }
    
    /// Create formatter with custom configuration
    pub fn with_config(config: FormatterConfig) -> Self {
        Self { config }
    }
    
    /// Format source code
    pub fn format(&self, source: &str) -> Result<String> {
        let mut formatted = String::new();
        let lines: Vec<&str> = source.lines().collect();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut in_comment = false;
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Skip empty lines but preserve them
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            // Handle comments
            if trimmed.starts_with("//") {
                formatted.push_str(&self.format_comment(trimmed, indent_level));
                formatted.push('\n');
                continue;
            }
            
            // Handle multi-line comments
            if trimmed.starts_with("/*") {
                in_comment = true;
            }
            if in_comment {
                formatted.push_str(&self.format_comment(trimmed, indent_level));
                formatted.push('\n');
                if trimmed.ends_with("*/") {
                    in_comment = false;
                }
                continue;
            }
            
            // Adjust indentation for closing braces
            if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
            }
            
            // Format the line
            let formatted_line = self.format_line(trimmed, indent_level, &mut in_string)?;
            formatted.push_str(&formatted_line);
            
            // Adjust indentation for opening braces
            if trimmed.ends_with('{') || trimmed.contains(" {") {
                indent_level += 1;
            }
            
            // Add line ending
            if line_num < lines.len() - 1 || self.config.insert_final_newline {
                formatted.push_str(&self.get_line_ending());
            }
        }
        
        // Trim trailing whitespace if configured
        if self.config.trim_trailing_whitespace {
            formatted = self.trim_trailing_whitespace(&formatted);
        }
        
        Ok(formatted)
    }
    
    /// Format a single line
    fn format_line(&self, line: &str, indent_level: usize, in_string: &mut bool) -> Result<String> {
        let mut formatted = String::new();
        
        // Add indentation
        formatted.push_str(&self.get_indentation(indent_level));
        
        // Process characters
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            match ch {
                '"' => {
                    *in_string = !*in_string;
                    formatted.push(ch);
                }
                ' ' => {
                    // Normalize whitespace
                    if !*in_string {
                        // Skip multiple spaces
                        while i + 1 < chars.len() && chars[i + 1] == ' ' {
                            i += 1;
                        }
                    }
                    formatted.push(' ');
                }
                '(' | ')' => {
                    if !*in_string {
                        if self.config.space_inside_parens {
                            if ch == '(' && i + 1 < chars.len() && chars[i + 1] != ')' {
                                formatted.push(ch);
                                formatted.push(' ');
                            } else if ch == ')' && i > 0 && chars[i - 1] != '(' {
                                formatted.push(' ');
                                formatted.push(ch);
                            } else {
                                formatted.push(ch);
                            }
                        } else {
                            formatted.push(ch);
                        }
                    } else {
                        formatted.push(ch);
                    }
                }
                '+' | '-' | '*' | '/' | '=' | '<' | '>' | '!' => {
                    if !*in_string && self.config.space_around_operators {
                        // Add space before operator
                        if !formatted.ends_with(' ') {
                            formatted.push(' ');
                        }
                        formatted.push(ch);
                        // Add space after operator
                        if i + 1 < chars.len() && chars[i + 1] != ' ' && chars[i + 1] != '=' {
                            formatted.push(' ');
                        }
                    } else {
                        formatted.push(ch);
                    }
                }
                '{' => {
                    if !*in_string {
                        match self.config.brace_style {
                            BraceStyle::SameLine => {
                                if !formatted.ends_with(' ') {
                                    formatted.push(' ');
                                }
                                formatted.push(ch);
                            }
                            BraceStyle::NextLine => {
                                formatted.push('\n');
                                formatted.push_str(&self.get_indentation(indent_level));
                                formatted.push(ch);
                            }
                            BraceStyle::NextLineIndented => {
                                formatted.push('\n');
                                formatted.push_str(&self.get_indentation(indent_level + 1));
                                formatted.push(ch);
                            }
                        }
                    } else {
                        formatted.push(ch);
                    }
                }
                _ => {
                    formatted.push(ch);
                }
            }
            
            i += 1;
        }
        
        Ok(formatted)
    }
    
    /// Format a comment line
    fn format_comment(&self, comment: &str, indent_level: usize) -> String {
        let mut formatted = String::new();
        formatted.push_str(&self.get_indentation(indent_level));
        formatted.push_str(comment);
        formatted
    }
    
    /// Get indentation string
    fn get_indentation(&self, level: usize) -> String {
        if self.config.use_tabs {
            "\t".repeat(level)
        } else {
            " ".repeat(level * self.config.indent_size)
        }
    }
    
    /// Get line ending string
    fn get_line_ending(&self) -> &'static str {
        match self.config.line_ending {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Mac => "\r",
        }
    }
    
    /// Trim trailing whitespace from all lines
    fn trim_trailing_whitespace(&self, text: &str) -> String {
        text.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join(&self.get_line_ending())
    }
    
    /// Format function declaration
    pub fn format_function(&self, func_text: &str) -> Result<String> {
        // TODO: Implement specific function formatting
        self.format(func_text)
    }
    
    /// Format struct declaration
    pub fn format_struct(&self, struct_text: &str) -> Result<String> {
        // TODO: Implement specific struct formatting
        self.format(struct_text)
    }
    
    /// Format expression
    pub fn format_expression(&self, expr_text: &str) -> Result<String> {
        // TODO: Implement specific expression formatting
        let mut formatted = String::new();
        let chars: Vec<char> = expr_text.chars().collect();
        let mut in_string = false;
        
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '"' => {
                    in_string = !in_string;
                    formatted.push(ch);
                }
                '+' | '-' | '*' | '/' | '=' if !in_string => {
                    if self.config.space_around_operators {
                        if !formatted.ends_with(' ') {
                            formatted.push(' ');
                        }
                        formatted.push(ch);
                        if i + 1 < chars.len() && chars[i + 1] != ' ' {
                            formatted.push(' ');
                        }
                    } else {
                        formatted.push(ch);
                    }
                }
                _ => {
                    formatted.push(ch);
                }
            }
        }
        
        Ok(formatted)
    }
    
    /// Check if code is properly formatted
    pub fn is_formatted(&self, source: &str) -> Result<bool> {
        let formatted = self.format(source)?;
        Ok(formatted == source)
    }
    
    /// Get formatting differences
    pub fn get_diff(&self, source: &str) -> Result<Vec<FormattingDiff>> {
        let formatted = self.format(source)?;
        let original_lines: Vec<&str> = source.lines().collect();
        let formatted_lines: Vec<&str> = formatted.lines().collect();
        
        let mut diffs = Vec::new();
        let max_lines = original_lines.len().max(formatted_lines.len());
        
        for i in 0..max_lines {
            let original = original_lines.get(i).unwrap_or(&"");
            let formatted_line = formatted_lines.get(i).unwrap_or(&"");
            
            if original != formatted_line {
                diffs.push(FormattingDiff {
                    line_number: i + 1,
                    original: original.to_string(),
                    formatted: formatted_line.to_string(),
                });
            }
        }
        
        Ok(diffs)
    }
}

/// Formatting difference
#[derive(Debug, Clone)]
pub struct FormattingDiff {
    /// Line number (1-based)
    pub line_number: usize,
    /// Original line content
    pub original: String,
    /// Formatted line content
    pub formatted: String,
}

impl Default for CodeFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            use_tabs: false,
            max_line_length: 100,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            space_before_function_paren: false,
            space_inside_parens: false,
            space_around_operators: true,
            brace_style: BraceStyle::SameLine,
            line_ending: LineEnding::Unix,
        }
    }
}
