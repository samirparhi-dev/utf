use crate::core::*;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct RustAdapter;

impl RustAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_patterns(&self, source: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();
        
        // Detect function definitions
        if let Ok(function_regex) = Regex::new(r"fn\s+(\w+)\s*\(([^)]*)\)") {
            for captures in function_regex.captures_iter(source) {
                if let (Some(name), Some(params)) = (captures.get(1), captures.get(2)) {
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Function(FunctionPattern {
                            name: name.as_str().to_string(),
                            parameters: params.as_str().split(',').map(|s| s.trim().to_string()).collect(),
                            return_type: None,
                        }),
                        location: SourceLocation {
                            file: "unknown".to_string(),
                            line: 1,
                            column: name.start(),
                        },
                        context: Context {
                            function_name: Some(name.as_str().to_string()),
                            class_name: None,
                            module_name: None,
                        },
                        confidence: 0.9,
                    });
                }
            }
        }

        patterns
    }
}

#[async_trait]
impl TestGenerator for RustAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(self.detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            if let PatternType::Function(func) = &pattern.pattern_type {
                test_cases.push(TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: format!("test_{}", func.name),
                    description: format!("Test {} function", func.name),
                    input: serde_json::json!({}),
                    expected_output: serde_json::json!(null),
                });
            }
        }

        Ok(TestSuite {
            name: "Generated Rust Tests".to_string(),
            language: "rust".to_string(),
            framework: "cargo-test".to_string(),
            test_cases,
            imports: vec![],
        })
    }

    fn get_language(&self) -> &str {
        "rust"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["cargo-test"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_adapter_new() {
        let adapter = RustAdapter::new();
        assert_eq!(adapter.get_language(), "rust");
    }

    #[test]
    fn test_get_language() {
        let adapter = RustAdapter::new();
        assert_eq!(adapter.get_language(), "rust");
    }

    #[test]
    fn test_get_supported_frameworks() {
        let adapter = RustAdapter::new();
        let frameworks = adapter.get_supported_frameworks();
        assert_eq!(frameworks, vec!["cargo-test"]);
    }

    #[test]
    fn test_detect_patterns_function_def() {
        let adapter = RustAdapter::new();
        let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "add");
            assert_eq!(func.parameters, vec!["a: i32".to_string(), "b: i32".to_string()]);
            assert!(func.return_type.is_none());
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_patterns_function_no_params() {
        let adapter = RustAdapter::new();
        let source = "fn test() { println!(\"hello\"); }";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "test");
            assert_eq!(func.parameters, vec![""]);
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_patterns_multiple_functions() {
        let adapter = RustAdapter::new();
        let source = r#"
            fn multiply(x: f64, y: f64) -> f64 {
                x * y
            }
            
            fn divide(a: f64, b: f64) -> f64 {
                a / b
            }
        "#;
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2);
        
        let function_names: Vec<String> = patterns.iter()
            .filter_map(|p| {
                if let PatternType::Function(func) = &p.pattern_type {
                    Some(func.name.clone())
                } else {
                    None
                }
            })
            .collect();
        
        assert!(function_names.contains(&"multiply".to_string()));
        assert!(function_names.contains(&"divide".to_string()));
    }

    #[test]
    fn test_detect_patterns_pub_function() {
        let adapter = RustAdapter::new();
        let source = "pub fn public_function(param: String) -> bool { true }";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "public_function");
            assert_eq!(func.parameters, vec!["param: String".to_string()]);
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_patterns_no_matches() {
        let adapter = RustAdapter::new();
        let source = "let x = 42;\nconst Y: i32 = 10;";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn test_detect_patterns_main_function() {
        let adapter = RustAdapter::new();
        let source = "fn main() { println!(\"Hello, world!\"); }";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "main");
            assert_eq!(func.parameters, vec![""]);
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[tokio::test]
    async fn test_analyze_code() {
        let adapter = RustAdapter::new();
        let source = "fn test_function() -> bool { true }";
        let result = adapter.analyze_code(source, "test.rs").await;
        assert!(result.is_ok());
        
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].confidence, 0.9);
    }

    #[tokio::test]
    async fn test_generate_tests_function_pattern() {
        let adapter = RustAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "calculate_sum".to_string(),
                parameters: vec!["a: i32".to_string(), "b: i32".to_string()],
                return_type: Some("i32".to_string()),
            }),
            location: SourceLocation {
                file: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("calculate_sum".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.language, "rust");
        assert_eq!(test_suite.framework, "cargo-test");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].name.contains("test_calculate_sum"));
    }

    #[tokio::test]
    async fn test_generate_tests_non_function_pattern() {
        let adapter = RustAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::FormValidation(FormField {
                name: "email".to_string(),
                field_type: FieldType::Email,
                required: true,
            }),
            location: SourceLocation {
                file: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: None,
                class_name: None,
                module_name: None,
            },
            confidence: 0.8,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0); // Non-function patterns don't generate tests
    }

    #[tokio::test]
    async fn test_generate_tests_api_call_pattern() {
        let adapter = RustAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::ApiCall(ApiEndpoint {
                method: HttpMethod::Get,
                path: "/api/test".to_string(),
                parameters: vec!["id".to_string()],
            }),
            location: SourceLocation {
                file: "test.rs".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: None,
                class_name: None,
                module_name: None,
            },
            confidence: 0.7,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0); // API calls don't generate tests
    }

    #[tokio::test]
    async fn test_generate_tests_empty_patterns() {
        let adapter = RustAdapter::new();
        let result = adapter.generate_tests(vec![]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0);
        assert_eq!(test_suite.imports.len(), 0); // Rust adapter has no imports
    }

    #[tokio::test]
    async fn test_generate_tests_multiple_functions() {
        let adapter = RustAdapter::new();
        let patterns = vec![
            TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Function(FunctionPattern {
                    name: "func1".to_string(),
                    parameters: vec!["x: i32".to_string()],
                    return_type: Some("i32".to_string()),
                }),
                location: SourceLocation {
                    file: "test.rs".to_string(),
                    line: 1,
                    column: 1,
                },
                context: Context {
                    function_name: Some("func1".to_string()),
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.9,
            },
            TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Function(FunctionPattern {
                    name: "func2".to_string(),
                    parameters: vec!["y: String".to_string()],
                    return_type: Some("bool".to_string()),
                }),
                location: SourceLocation {
                    file: "test.rs".to_string(),
                    line: 5,
                    column: 1,
                },
                context: Context {
                    function_name: Some("func2".to_string()),
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.9,
            },
        ];

        let result = adapter.generate_tests(patterns).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 2);
        
        let test_names: Vec<&String> = test_suite.test_cases.iter().map(|t| &t.name).collect();
        assert!(test_names.iter().any(|name| name.contains("test_func1")));
        assert!(test_names.iter().any(|name| name.contains("test_func2")));
    }
}