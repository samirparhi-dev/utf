use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod dynamic_adapter;
pub mod language_loader;

pub use dynamic_adapter::*;
pub use language_loader::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub function_name: Option<String>,
    pub class_name: Option<String>,
    pub module_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestablePattern {
    pub id: String,
    pub pattern_type: PatternType,
    pub location: SourceLocation,
    pub context: Context,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    FormValidation(FormField),
    ApiCall(ApiEndpoint),
    Function(FunctionPattern),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FieldType {
    Email,
    Password,
    Text,
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub method: HttpMethod,
    pub path: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionPattern {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: String,
    pub name: String,
    pub description: String,
    pub input: serde_json::Value,
    pub expected_output: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub language: String,
    pub framework: String,
    pub test_cases: Vec<TestCase>,
    pub imports: Vec<String>,
}

#[async_trait]
pub trait TestGenerator {
    async fn analyze_code(&self, source: &str, file_path: &str) -> Result<Vec<TestablePattern>>;
    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite>;
    fn get_language(&self) -> &str;
    fn get_supported_frameworks(&self) -> Vec<&str>;
}

pub struct TestOrchestrator {
    adapters: HashMap<String, Box<dyn TestGenerator + Send + Sync>>,
}

impl TestOrchestrator {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    pub fn register_adapter(&mut self, language: String, adapter: Box<dyn TestGenerator + Send + Sync>) {
        self.adapters.insert(language, adapter);
    }

    pub async fn analyze_file(&self, file_path: &str, content: &str) -> Result<Vec<TestablePattern>> {
        let language = self.detect_language(file_path)?;
        
        if let Some(adapter) = self.adapters.get(&language) {
            adapter.analyze_code(content, file_path).await
        } else {
            Err(anyhow::anyhow!("No adapter found for language: {}", language))
        }
    }

    pub async fn generate_tests_for_file(&self, file_path: &str, content: &str) -> Result<TestSuite> {
        let patterns = self.analyze_file(file_path, content).await?;
        let language = self.detect_language(file_path)?;
        
        if let Some(adapter) = self.adapters.get(&language) {
            adapter.generate_tests(patterns).await
        } else {
            Err(anyhow::anyhow!("No adapter found for language: {}", language))
        }
    }

    fn detect_language(&self, file_path: &str) -> Result<String> {
        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("Could not determine file extension"))?;

        // Check if we have an adapter registered for this extension
        // We need to check the registered adapters to see what languages we support
        for (language, _) in &self.adapters {
            match (language.as_str(), extension) {
                ("javascript", "js" | "jsx" | "ts" | "tsx") => return Ok("javascript".to_string()),
                ("python", "py") => return Ok("python".to_string()),
                ("rust", "rs") => return Ok("rust".to_string()),
                ("go", "go") => return Ok("go".to_string()),
                ("java", "java") => return Ok("java".to_string()),
                ("kotlin", "kt") => return Ok("kotlin".to_string()),
                ("swift", "swift") => return Ok("swift".to_string()),
                ("csharp", "cs") => return Ok("csharp".to_string()),
                ("php", "php") => return Ok("php".to_string()),
                _ => continue,
            }
        }

        Err(anyhow::anyhow!("Unsupported file extension: {}", extension))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdapter {
        language: String,
        patterns: Vec<TestablePattern>,
    }

    impl MockAdapter {
        fn new(language: &str) -> Self {
            Self {
                language: language.to_string(),
                patterns: vec![],
            }
        }

        fn with_patterns(language: &str, patterns: Vec<TestablePattern>) -> Self {
            Self {
                language: language.to_string(),
                patterns,
            }
        }
    }

    #[async_trait]
    impl TestGenerator for MockAdapter {
        async fn analyze_code(&self, _source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
            Ok(self.patterns.clone())
        }

        async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
            Ok(TestSuite {
                name: format!("Test Suite for {}", self.language),
                language: self.language.clone(),
                framework: "mock".to_string(),
                test_cases: patterns.into_iter().map(|p| TestCase {
                    id: p.id.clone(),
                    name: format!("test_{}", p.id),
                    description: "Mock test".to_string(),
                    input: serde_json::json!({}),
                    expected_output: serde_json::json!({}),
                }).collect(),
                imports: vec![],
            })
        }

        fn get_language(&self) -> &str {
            &self.language
        }

        fn get_supported_frameworks(&self) -> Vec<&str> {
            vec!["mock"]
        }
    }

    #[test]
    fn test_source_location_creation() {
        let location = SourceLocation {
            file: "test.rs".to_string(),
            line: 10,
            column: 5,
        };
        assert_eq!(location.file, "test.rs");
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
    }

    #[test]
    fn test_context_creation() {
        let context = Context {
            function_name: Some("test_function".to_string()),
            class_name: None,
            module_name: Some("test_module".to_string()),
        };
        assert_eq!(context.function_name, Some("test_function".to_string()));
        assert!(context.class_name.is_none());
        assert_eq!(context.module_name, Some("test_module".to_string()));
    }

    #[test]
    fn test_field_type_equality() {
        assert_eq!(FieldType::Email, FieldType::Email);
        assert_ne!(FieldType::Email, FieldType::Password);
        assert_eq!(FieldType::Text, FieldType::Text);
        assert_ne!(FieldType::Number, FieldType::Text);
    }

    #[test]
    fn test_testable_pattern_creation() {
        let pattern = TestablePattern {
            id: "test-id".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "test_func".to_string(),
                parameters: vec!["param1".to_string()],
                return_type: Some("String".to_string()),
            }),
            location: SourceLocation {
                file: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("test_func".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };
        assert_eq!(pattern.id, "test-id");
        assert_eq!(pattern.confidence, 0.9);
    }

    #[tokio::test]
    async fn test_orchestrator_new() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.adapters.len(), 0);
    }

    #[tokio::test]
    async fn test_orchestrator_register_adapter() {
        let mut orchestrator = TestOrchestrator::new();
        let adapter = MockAdapter::new("javascript");
        orchestrator.register_adapter("javascript".to_string(), Box::new(adapter));
        assert_eq!(orchestrator.adapters.len(), 1);
    }

    #[tokio::test]
    async fn test_detect_language_js() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.detect_language("test.js").unwrap(), "javascript");
        assert_eq!(orchestrator.detect_language("test.jsx").unwrap(), "javascript");
    }

    #[tokio::test]
    async fn test_detect_language_ts() {
        let orchestrator = TestOrchestrator::new();
        // TypeScript files now use the JavaScript adapter
        assert_eq!(orchestrator.detect_language("test.ts").unwrap(), "javascript");
        assert_eq!(orchestrator.detect_language("test.tsx").unwrap(), "javascript");
    }

    #[tokio::test]
    async fn test_detect_language_python() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.detect_language("test.py").unwrap(), "python");
    }

    #[tokio::test]
    async fn test_detect_language_rust() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.detect_language("test.rs").unwrap(), "rust");
    }

    #[tokio::test]
    async fn test_detect_language_go() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.detect_language("test.go").unwrap(), "go");
    }

    #[tokio::test]
    async fn test_detect_language_java() {
        let orchestrator = TestOrchestrator::new();
        assert_eq!(orchestrator.detect_language("Test.java").unwrap(), "java");
    }

    #[tokio::test]
    async fn test_detect_language_unsupported() {
        let orchestrator = TestOrchestrator::new();
        assert!(orchestrator.detect_language("test.cpp").is_err());
    }

    #[tokio::test]
    async fn test_detect_language_no_extension() {
        let orchestrator = TestOrchestrator::new();
        assert!(orchestrator.detect_language("test").is_err());
    }

    #[tokio::test]
    async fn test_analyze_file_success() {
        let mut orchestrator = TestOrchestrator::new();
        let pattern = TestablePattern {
            id: "test-pattern".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "test_func".to_string(),
                parameters: vec![],
                return_type: None,
            }),
            location: SourceLocation {
                file: "test.js".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("test_func".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.8,
        };
        let adapter = MockAdapter::with_patterns("javascript", vec![pattern.clone()]);
        orchestrator.register_adapter("javascript".to_string(), Box::new(adapter));

        let result = orchestrator.analyze_file("test.js", "function test_func() {}").await;
        assert!(result.is_ok());
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].id, "test-pattern");
    }

    #[tokio::test]
    async fn test_analyze_file_no_adapter() {
        let orchestrator = TestOrchestrator::new();
        let result = orchestrator.analyze_file("test.js", "code").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No adapter found"));
    }

    #[tokio::test]
    async fn test_generate_tests_for_file_success() {
        let mut orchestrator = TestOrchestrator::new();
        let pattern = TestablePattern {
            id: "test-pattern".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "test_func".to_string(),
                parameters: vec![],
                return_type: None,
            }),
            location: SourceLocation {
                file: "test.js".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("test_func".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.8,
        };
        let adapter = MockAdapter::with_patterns("javascript", vec![pattern.clone()]);
        orchestrator.register_adapter("javascript".to_string(), Box::new(adapter));

        let result = orchestrator.generate_tests_for_file("test.js", "function test_func() {}").await;
        assert!(result.is_ok());
        let test_suite = result.unwrap();
        assert_eq!(test_suite.language, "javascript");
        assert_eq!(test_suite.test_cases.len(), 1);
    }

    #[tokio::test]
    async fn test_generate_tests_for_file_no_adapter() {
        let orchestrator = TestOrchestrator::new();
        let result = orchestrator.generate_tests_for_file("test.js", "code").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No adapter found"));
    }

    #[test]
    fn test_test_case_creation() {
        let test_case = TestCase {
            id: "test-1".to_string(),
            name: "test_function".to_string(),
            description: "Test a function".to_string(),
            input: serde_json::json!({"param": "value"}),
            expected_output: serde_json::json!({"result": "success"}),
        };
        assert_eq!(test_case.id, "test-1");
        assert_eq!(test_case.name, "test_function");
        assert_eq!(test_case.description, "Test a function");
    }

    #[test]
    fn test_test_suite_creation() {
        let test_suite = TestSuite {
            name: "Test Suite".to_string(),
            language: "javascript".to_string(),
            framework: "jest".to_string(),
            test_cases: vec![],
            imports: vec!["import { expect } from 'jest';".to_string()],
        };
        assert_eq!(test_suite.name, "Test Suite");
        assert_eq!(test_suite.language, "javascript");
        assert_eq!(test_suite.framework, "jest");
        assert_eq!(test_suite.imports.len(), 1);
    }
}