use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    TestablePattern, PatternType, TestCase, TestSuite, TestGenerator, 
    SourceLocation, Context, FunctionPattern
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub extensions: Vec<String>,
    pub framework: String,
    pub patterns: Vec<PatternConfig>,
    pub test_template: TestTemplate,
    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    pub name: String,
    pub pattern_type: String, // "function", "class", "interface", etc.
    pub regex: String,
    pub capture_groups: CaptureGroups,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureGroups {
    pub name: Option<usize>,          // Which capture group contains the name
    pub return_type: Option<usize>,   // Which capture group contains return type
    pub parameters: Option<usize>,    // Which capture group contains parameters
    pub parameter_separator: String,  // How parameters are separated (e.g., ",")
    pub parameter_format: String,     // Parameter format: "name_type", "type_name", "name_only"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTemplate {
    pub setup: Option<String>,        // Setup code for the test file
    pub test_function: String,        // Template for individual test functions
    pub teardown: Option<String>,     // Teardown code for the test file
    pub file_extension: String,       // Extension for generated test files
    pub placeholders: HashMap<String, String>, // Placeholder mappings
}

pub struct DynamicLanguageAdapter {
    config: LanguageConfig,
}

impl DynamicLanguageAdapter {
    pub fn new(config: LanguageConfig) -> Self {
        Self { config }
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        let config: LanguageConfig = serde_json::from_str(json_str)?;
        Ok(Self::new(config))
    }

    pub fn from_file(file_path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        Self::from_json(&content)
    }

    fn detect_patterns(&self, content: &str) -> Result<Vec<TestablePattern>> {
        let mut patterns = Vec::new();

        for pattern_config in &self.config.patterns {
            let regex = Regex::new(&pattern_config.regex)?;
            
            for cap in regex.captures_iter(content) {
                let name = self.extract_capture_group(&cap, &pattern_config.capture_groups.name, "unknown");
                let return_type = self.extract_capture_group(&cap, &pattern_config.capture_groups.return_type, "void");
                let parameters = self.extract_parameters(&cap, &pattern_config.capture_groups)?;
                
                let line_num = content[..cap.get(0).unwrap().start()].matches('\n').count() + 1;
                
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: self.create_pattern_type(&pattern_config.pattern_type, &name, &parameters, &return_type)?,
                    location: SourceLocation {
                        file: "".to_string(),
                        line: line_num,
                        column: cap.get(0).unwrap().start() + 1,
                    },
                    context: Context {
                        function_name: if pattern_config.pattern_type == "function" { Some(name.clone()) } else { None },
                        class_name: self.extract_class_name(content),
                        module_name: None,
                    },
                    confidence: pattern_config.confidence,
                });
            }
        }

        Ok(patterns)
    }

    fn extract_capture_group(&self, cap: &regex::Captures, group_index: &Option<usize>, default: &str) -> String {
        if let Some(index) = group_index {
            cap.get(*index)
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| default.to_string())
        } else {
            default.to_string()
        }
    }

    fn extract_parameters(&self, cap: &regex::Captures, capture_groups: &CaptureGroups) -> Result<Vec<String>> {
        if let Some(param_index) = capture_groups.parameters {
            if let Some(params_match) = cap.get(param_index) {
                let params_str = params_match.as_str().trim();
                if params_str.is_empty() {
                    return Ok(vec![]);
                }

                let parameters = params_str
                    .split(&capture_groups.parameter_separator)
                    .map(|p| self.parse_parameter(p.trim(), &capture_groups.parameter_format))
                    .collect();
                
                return Ok(parameters);
            }
        }
        Ok(vec![])
    }

    fn parse_parameter(&self, param: &str, format: &str) -> String {
        match format {
            "name_type" => {
                // e.g., "x int" -> "x"
                param.split_whitespace().next().unwrap_or(param).to_string()
            }
            "type_name" => {
                // e.g., "int x" -> "x" 
                param.split_whitespace().last().unwrap_or(param).to_string()
            }
            "name_only" => {
                // e.g., "x" -> "x"
                param.to_string()
            }
            _ => param.to_string()
        }
    }

    fn create_pattern_type(&self, pattern_type: &str, name: &str, parameters: &[String], return_type: &str) -> Result<PatternType> {
        match pattern_type {
            "function" => Ok(PatternType::Function(FunctionPattern {
                name: name.to_string(),
                parameters: parameters.to_vec(),
                return_type: Some(return_type.to_string()),
            })),
            _ => {
                // For now, default to function pattern
                // In the future, we could extend PatternType enum or make it more flexible
                Ok(PatternType::Function(FunctionPattern {
                    name: name.to_string(),
                    parameters: parameters.to_vec(),
                    return_type: Some(return_type.to_string()),
                }))
            }
        }
    }

    fn extract_class_name(&self, content: &str) -> Option<String> {
        // Simple heuristic - look for common class patterns
        let class_patterns = vec![
            r"class\s+(\w+)",
            r"struct\s+(\w+)",
            r"interface\s+(\w+)",
            r"type\s+(\w+)\s+struct",
        ];

        for pattern in class_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(cap) = regex.captures(content) {
                    return Some(cap.get(1).unwrap().as_str().to_string());
                }
            }
        }
        None
    }

    pub fn generate_test_content(&self, test_cases: &[TestCase]) -> String {
        let mut content = String::new();

        // Add setup code
        if let Some(setup) = &self.config.test_template.setup {
            content.push_str(&self.replace_placeholders(setup, &HashMap::new()));
            content.push_str("\n\n");
        }

        // Add test functions
        for test_case in test_cases {
            let mut placeholders = HashMap::new();
            placeholders.insert("TEST_NAME".to_string(), test_case.name.clone());
            placeholders.insert("TEST_DESCRIPTION".to_string(), test_case.description.clone());
            
            let test_function = self.replace_placeholders(
                &self.config.test_template.test_function, 
                &placeholders
            );
            content.push_str(&test_function);
            content.push_str("\n\n");
        }

        // Add teardown code
        if let Some(teardown) = &self.config.test_template.teardown {
            content.push_str(&self.replace_placeholders(teardown, &HashMap::new()));
            content.push('\n');
        }

        content
    }

    fn replace_placeholders(&self, template: &str, additional: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        // Replace built-in placeholders
        for (key, value) in &self.config.test_template.placeholders {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        // Replace additional placeholders
        for (key, value) in additional {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        
        result
    }
}

#[async_trait]
impl TestGenerator for DynamicLanguageAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        self.detect_patterns(source)
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in &patterns {
            match &pattern.pattern_type {
                PatternType::Function(func) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test{}", func.name),
                        description: format!("Test for {} {}", 
                            if self.config.name == "go" || self.config.name == "rust" { "function" } else { "method" }, 
                            func.name
                        ),
                        input: serde_json::json!({
                            "function": func.name,
                            "parameters": func.parameters
                        }),
                        expected_output: serde_json::json!({
                            "type": func.return_type.as_ref().unwrap_or(&"void".to_string())
                        }),
                        test_body: "        // TODO: Implement test logic".to_string(),
                        assertions: vec![],
                        test_category: crate::core::TestCategory::HappyPath,
                    });
                }
                _ => {} // Skip other pattern types for now
            }
        }

        let class_name = patterns.iter()
            .find_map(|p| p.context.class_name.as_ref())
            .unwrap_or(&format!("{}Test", self.config.name.to_title_case()))
            .clone();

        Ok(TestSuite {
            name: format!("{}Test", class_name.replace("Test", "")),
            language: self.config.name.clone(),
            framework: self.config.framework.clone(),
            test_cases,
            imports: self.config.imports.clone(),
            test_type: crate::core::TestType::Unit,
            setup_requirements: vec![],
            cleanup_requirements: vec![],
            coverage_target: 70.0,
            test_code: None,
        })
    }

    fn get_language(&self) -> &str {
        &self.config.name
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec![&self.config.framework]
    }

    async fn generate_comprehensive_tests(&self, patterns: Vec<TestablePattern>, _source: &str) -> Result<TestSuite> {
        self.generate_tests(patterns).await
    }

    fn get_coverage_target(&self) -> f32 {
        70.0
    }

    fn generate_test_code(&self, _test_suite: &TestSuite) -> Result<String> {
        Ok("// Dynamic adapter tests - TODO: implement code generation".to_string())
    }
}

// Helper trait to convert strings to title case
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for ch in self.chars() {
            if ch.is_alphabetic() {
                if capitalize_next {
                    result.push(ch.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch.to_ascii_lowercase());
                }
            } else {
                result.push(ch);
                capitalize_next = true;
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_go_config() -> LanguageConfig {
        LanguageConfig {
            name: "go".to_string(),
            extensions: vec!["go".to_string()],
            framework: "testing".to_string(),
            patterns: vec![
                PatternConfig {
                    name: "function".to_string(),
                    pattern_type: "function".to_string(),
                    regex: r"func\s+(\w+)\s*\(([^)]*)\)(?:\s*([^{]*?))?(?:\s*\{|$)".to_string(),
                    capture_groups: CaptureGroups {
                        name: Some(1),
                        parameters: Some(2),
                        return_type: Some(3),
                        parameter_separator: ",".to_string(),
                        parameter_format: "name_type".to_string(),
                    },
                    confidence: 0.9,
                }
            ],
            test_template: TestTemplate {
                setup: Some("package main\n\nimport \"testing\"".to_string()),
                test_function: "func {{TEST_NAME}}(t *testing.T) {\n\t// {{TEST_DESCRIPTION}}\n\t// TODO: Implement test logic\n}".to_string(),
                teardown: None,
                file_extension: "_test.go".to_string(),
                placeholders: HashMap::new(),
            },
            imports: vec!["testing".to_string()],
        }
    }

    #[test]
    fn test_dynamic_adapter_creation() {
        let config = create_go_config();
        let adapter = DynamicLanguageAdapter::new(config);
        assert_eq!(adapter.get_language(), "go");
    }

    #[test]
    fn test_from_json() {
        let config = create_go_config();
        let json_str = serde_json::to_string(&config).unwrap();
        let adapter = DynamicLanguageAdapter::from_json(&json_str).unwrap();
        assert_eq!(adapter.get_language(), "go");
    }

    #[tokio::test]
    async fn test_detect_go_function() {
        let adapter = DynamicLanguageAdapter::new(create_go_config());
        let content = r#"
func Add(a int, b int) int {
    return a + b
}
"#;
        let patterns = adapter.analyze_code(content, "test.go").await.unwrap();
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "Add");
            assert_eq!(func.parameters, vec!["a", "b"]);
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[tokio::test]
    async fn test_generate_tests() {
        let adapter = DynamicLanguageAdapter::new(create_go_config());
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "Add".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                return_type: Some("int".to_string()),
            }),
            location: SourceLocation {
                file: "test.go".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("Add".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.language, "go");
        assert_eq!(test_suite.framework, "testing");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert_eq!(test_suite.test_cases[0].name, "testAdd");
    }

    #[test]
    fn test_parameter_parsing() {
        let adapter = DynamicLanguageAdapter::new(create_go_config());
        
        // Test name_type format (Go style)
        assert_eq!(adapter.parse_parameter("x int", "name_type"), "x");
        
        // Test type_name format (Java style)  
        assert_eq!(adapter.parse_parameter("int x", "type_name"), "x");
        
        // Test name_only format
        assert_eq!(adapter.parse_parameter("x", "name_only"), "x");
    }

    #[test]
    fn test_placeholder_replacement() {
        let mut placeholders = HashMap::new();
        placeholders.insert("LANGUAGE".to_string(), "Go".to_string());
        
        let config = LanguageConfig {
            test_template: TestTemplate {
                placeholders,
                ..create_go_config().test_template
            },
            ..create_go_config()
        };
        
        let adapter = DynamicLanguageAdapter::new(config);
        let result = adapter.replace_placeholders("Testing {{LANGUAGE}} function", &HashMap::new());
        assert_eq!(result, "Testing Go function");
    }
}