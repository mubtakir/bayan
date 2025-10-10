//! Documentation generator for AlBayan language
//! 
//! Generates documentation from source code comments and annotations

use anyhow::Result;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Documentation generator
#[derive(Debug)]
pub struct DocGenerator {
    /// Generator configuration
    config: DocConfig,
}

/// Documentation configuration
#[derive(Debug, Clone)]
pub struct DocConfig {
    /// Output format
    pub output_format: OutputFormat,
    /// Include private items
    pub include_private: bool,
    /// Include source code
    pub include_source: bool,
    /// Generate index
    pub generate_index: bool,
    /// Theme for HTML output
    pub theme: String,
}

/// Output format
#[derive(Debug, Clone)]
pub enum OutputFormat {
    /// HTML documentation
    Html,
    /// Markdown documentation
    Markdown,
    /// JSON documentation
    Json,
    /// Plain text documentation
    Text,
}

/// Generated documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    /// Module documentation
    pub modules: Vec<ModuleDoc>,
    /// Function documentation
    pub functions: Vec<FunctionDoc>,
    /// Struct documentation
    pub structs: Vec<StructDoc>,
    /// Enum documentation
    pub enums: Vec<EnumDoc>,
    /// Relation documentation
    pub relations: Vec<RelationDoc>,
    /// Examples
    pub examples: Vec<ExampleDoc>,
}

/// Module documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDoc {
    /// Module name
    pub name: String,
    /// Module description
    pub description: String,
    /// Module path
    pub path: String,
    /// Exported items
    pub exports: Vec<String>,
    /// Examples
    pub examples: Vec<String>,
}

/// Function documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDoc {
    /// Function name
    pub name: String,
    /// Function description
    pub description: String,
    /// Parameters
    pub parameters: Vec<ParameterDoc>,
    /// Return type
    pub return_type: Option<String>,
    /// Return description
    pub return_description: Option<String>,
    /// Examples
    pub examples: Vec<String>,
    /// Notes
    pub notes: Vec<String>,
    /// Visibility
    pub visibility: String,
}

/// Parameter documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDoc {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Parameter description
    pub description: String,
    /// Is optional
    pub optional: bool,
}

/// Struct documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDoc {
    /// Struct name
    pub name: String,
    /// Struct description
    pub description: String,
    /// Fields
    pub fields: Vec<FieldDoc>,
    /// Methods
    pub methods: Vec<String>,
    /// Examples
    pub examples: Vec<String>,
}

/// Field documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDoc {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Field description
    pub description: String,
    /// Visibility
    pub visibility: String,
}

/// Enum documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDoc {
    /// Enum name
    pub name: String,
    /// Enum description
    pub description: String,
    /// Variants
    pub variants: Vec<VariantDoc>,
    /// Examples
    pub examples: Vec<String>,
}

/// Enum variant documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDoc {
    /// Variant name
    pub name: String,
    /// Variant description
    pub description: String,
    /// Associated data
    pub data: Option<String>,
}

/// Relation documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationDoc {
    /// Relation name
    pub name: String,
    /// Relation description
    pub description: String,
    /// Arity
    pub arity: usize,
    /// Parameters
    pub parameters: Vec<String>,
    /// Examples
    pub examples: Vec<String>,
}

/// Example documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDoc {
    /// Example title
    pub title: String,
    /// Example description
    pub description: String,
    /// Example code
    pub code: String,
    /// Expected output
    pub output: Option<String>,
}

impl DocGenerator {
    /// Create a new documentation generator
    pub fn new() -> Self {
        Self {
            config: DocConfig::default(),
        }
    }
    
    /// Create generator with custom configuration
    pub fn with_config(config: DocConfig) -> Self {
        Self { config }
    }
    
    /// Generate documentation from source code
    pub fn generate(&self, source: &str) -> Result<Documentation> {
        let mut documentation = Documentation {
            modules: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            relations: Vec::new(),
            examples: Vec::new(),
        };
        
        let lines: Vec<&str> = source.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Parse documentation comments
            if line.starts_with("///") || line.starts_with("//!") {
                let (doc_comment, next_i) = self.parse_doc_comment(&lines, i);
                i = next_i;
                
                // Check what follows the documentation
                if i < lines.len() {
                    let next_line = lines[i].trim();
                    
                    if next_line.starts_with("fn ") {
                        let func_doc = self.parse_function_doc(&doc_comment, next_line)?;
                        documentation.functions.push(func_doc);
                    } else if next_line.starts_with("struct ") {
                        let struct_doc = self.parse_struct_doc(&doc_comment, next_line)?;
                        documentation.structs.push(struct_doc);
                    } else if next_line.starts_with("enum ") {
                        let enum_doc = self.parse_enum_doc(&doc_comment, next_line)?;
                        documentation.enums.push(enum_doc);
                    } else if next_line.starts_with("relation ") {
                        let relation_doc = self.parse_relation_doc(&doc_comment, next_line)?;
                        documentation.relations.push(relation_doc);
                    }
                }
            } else {
                i += 1;
            }
        }
        
        Ok(documentation)
    }
    
    /// Parse documentation comment
    fn parse_doc_comment(&self, lines: &[&str], start: usize) -> (String, usize) {
        let mut comment = String::new();
        let mut i = start;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                comment.push_str(&line[3..].trim());
                comment.push('\n');
            } else if line.starts_with("//!") {
                comment.push_str(&line[3..].trim());
                comment.push('\n');
            } else {
                break;
            }
            i += 1;
        }
        
        (comment.trim().to_string(), i)
    }
    
    /// Parse function documentation
    fn parse_function_doc(&self, doc_comment: &str, function_line: &str) -> Result<FunctionDoc> {
        let func_name = self.extract_function_name(function_line);
        let (description, sections) = self.parse_doc_sections(doc_comment);
        
        let mut parameters = Vec::new();
        let mut return_description = None;
        let mut examples = Vec::new();
        let mut notes = Vec::new();
        
        for (section_name, content) in sections {
            match section_name.as_str() {
                "param" | "parameter" => {
                    if let Some(param_doc) = self.parse_parameter_doc(&content) {
                        parameters.push(param_doc);
                    }
                }
                "return" | "returns" => {
                    return_description = Some(content);
                }
                "example" => {
                    examples.push(content);
                }
                "note" => {
                    notes.push(content);
                }
                _ => {}
            }
        }
        
        Ok(FunctionDoc {
            name: func_name,
            description,
            parameters,
            return_type: self.extract_return_type(function_line),
            return_description,
            examples,
            notes,
            visibility: "public".to_string(),
        })
    }
    
    /// Parse struct documentation
    fn parse_struct_doc(&self, doc_comment: &str, struct_line: &str) -> Result<StructDoc> {
        let struct_name = self.extract_struct_name(struct_line);
        let (description, _sections) = self.parse_doc_sections(doc_comment);
        
        Ok(StructDoc {
            name: struct_name,
            description,
            fields: Vec::new(), // TODO: Parse fields
            methods: Vec::new(), // TODO: Parse methods
            examples: Vec::new(),
        })
    }
    
    /// Parse enum documentation
    fn parse_enum_doc(&self, doc_comment: &str, enum_line: &str) -> Result<EnumDoc> {
        let enum_name = self.extract_enum_name(enum_line);
        let (description, _sections) = self.parse_doc_sections(doc_comment);
        
        Ok(EnumDoc {
            name: enum_name,
            description,
            variants: Vec::new(), // TODO: Parse variants
            examples: Vec::new(),
        })
    }
    
    /// Parse relation documentation
    fn parse_relation_doc(&self, doc_comment: &str, relation_line: &str) -> Result<RelationDoc> {
        let relation_name = self.extract_relation_name(relation_line);
        let (description, _sections) = self.parse_doc_sections(doc_comment);
        
        Ok(RelationDoc {
            name: relation_name,
            description,
            arity: 2, // TODO: Calculate actual arity
            parameters: Vec::new(), // TODO: Parse parameters
            examples: Vec::new(),
        })
    }
    
    /// Parse documentation sections
    fn parse_doc_sections(&self, doc_comment: &str) -> (String, HashMap<String, String>) {
        let mut description = String::new();
        let mut sections = HashMap::new();
        let mut current_section = None;
        let mut current_content = String::new();
        
        for line in doc_comment.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with('@') {
                // Save previous section
                if let Some(section_name) = current_section.take() {
                    sections.insert(section_name, current_content.trim().to_string());
                    current_content.clear();
                }
                
                // Start new section
                let parts: Vec<&str> = trimmed[1..].splitn(2, ' ').collect();
                if !parts.is_empty() {
                    current_section = Some(parts[0].to_string());
                    if parts.len() > 1 {
                        current_content.push_str(parts[1]);
                        current_content.push('\n');
                    }
                }
            } else if current_section.is_some() {
                current_content.push_str(trimmed);
                current_content.push('\n');
            } else {
                description.push_str(trimmed);
                description.push('\n');
            }
        }
        
        // Save last section
        if let Some(section_name) = current_section {
            sections.insert(section_name, current_content.trim().to_string());
        }
        
        (description.trim().to_string(), sections)
    }
    
    /// Parse parameter documentation
    fn parse_parameter_doc(&self, content: &str) -> Option<ParameterDoc> {
        let parts: Vec<&str> = content.splitn(3, ' ').collect();
        if parts.len() >= 2 {
            Some(ParameterDoc {
                name: parts[0].to_string(),
                param_type: parts[1].to_string(),
                description: parts.get(2).unwrap_or(&"").to_string(),
                optional: false,
            })
        } else {
            None
        }
    }
    
    /// Extract function name from function declaration
    fn extract_function_name(&self, line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "fn" {
            parts[1].trim_end_matches('(').to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    /// Extract return type from function declaration
    fn extract_return_type(&self, line: &str) -> Option<String> {
        if let Some(arrow_pos) = line.find("->") {
            let return_part = &line[arrow_pos + 2..];
            Some(return_part.trim().trim_end_matches('{').trim().to_string())
        } else {
            None
        }
    }
    
    /// Extract struct name
    fn extract_struct_name(&self, line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "struct" {
            parts[1].trim_end_matches('{').to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    /// Extract enum name
    fn extract_enum_name(&self, line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "enum" {
            parts[1].trim_end_matches('{').to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    /// Extract relation name
    fn extract_relation_name(&self, line: &str) -> String {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "relation" {
            parts[1].trim_end_matches('(').to_string()
        } else {
            "unknown".to_string()
        }
    }
    
    /// Generate HTML documentation
    pub fn generate_html(&self, documentation: &Documentation) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("<title>AlBayan Documentation</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 40px; }\n");
        html.push_str("h1, h2, h3 { color: #333; }\n");
        html.push_str("code { background-color: #f4f4f4; padding: 2px 4px; }\n");
        html.push_str("pre { background-color: #f4f4f4; padding: 10px; overflow-x: auto; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");
        
        html.push_str("<h1>AlBayan Documentation</h1>\n");
        
        // Functions
        if !documentation.functions.is_empty() {
            html.push_str("<h2>Functions</h2>\n");
            for func in &documentation.functions {
                html.push_str(&format!("<h3>{}</h3>\n", func.name));
                html.push_str(&format!("<p>{}</p>\n", func.description));
                
                if !func.parameters.is_empty() {
                    html.push_str("<h4>Parameters</h4>\n<ul>\n");
                    for param in &func.parameters {
                        html.push_str(&format!(
                            "<li><code>{}: {}</code> - {}</li>\n",
                            param.name, param.param_type, param.description
                        ));
                    }
                    html.push_str("</ul>\n");
                }
                
                if let Some(return_type) = &func.return_type {
                    html.push_str(&format!("<h4>Returns</h4>\n<p><code>{}</code>", return_type));
                    if let Some(return_desc) = &func.return_description {
                        html.push_str(&format!(" - {}", return_desc));
                    }
                    html.push_str("</p>\n");
                }
            }
        }
        
        // Structs
        if !documentation.structs.is_empty() {
            html.push_str("<h2>Structs</h2>\n");
            for struct_doc in &documentation.structs {
                html.push_str(&format!("<h3>{}</h3>\n", struct_doc.name));
                html.push_str(&format!("<p>{}</p>\n", struct_doc.description));
            }
        }
        
        html.push_str("</body>\n</html>");
        
        Ok(html)
    }
    
    /// Generate Markdown documentation
    pub fn generate_markdown(&self, documentation: &Documentation) -> Result<String> {
        let mut markdown = String::new();
        
        markdown.push_str("# AlBayan Documentation\n\n");
        
        // Functions
        if !documentation.functions.is_empty() {
            markdown.push_str("## Functions\n\n");
            for func in &documentation.functions {
                markdown.push_str(&format!("### {}\n\n", func.name));
                markdown.push_str(&format!("{}\n\n", func.description));
                
                if !func.parameters.is_empty() {
                    markdown.push_str("#### Parameters\n\n");
                    for param in &func.parameters {
                        markdown.push_str(&format!(
                            "- `{}`: `{}` - {}\n",
                            param.name, param.param_type, param.description
                        ));
                    }
                    markdown.push_str("\n");
                }
                
                if let Some(return_type) = &func.return_type {
                    markdown.push_str("#### Returns\n\n");
                    markdown.push_str(&format!("`{}`", return_type));
                    if let Some(return_desc) = &func.return_description {
                        markdown.push_str(&format!(" - {}", return_desc));
                    }
                    markdown.push_str("\n\n");
                }
            }
        }
        
        Ok(markdown)
    }
}

impl Default for DocGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DocConfig {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Html,
            include_private: false,
            include_source: false,
            generate_index: true,
            theme: "default".to_string(),
        }
    }
}
