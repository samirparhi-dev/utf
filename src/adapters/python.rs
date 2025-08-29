use crate::core::*;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct PythonAdapter;

impl PythonAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_patterns(&self, source: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();
        
        // Detect function definitions
        if let Ok(function_regex) = Regex::new(r"def\s+(\w+)\s*\(([^)]*)\):") {
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

        // Detect email validation patterns
        if source.contains("EmailField") || source.contains("email") {
            patterns.push(TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::FormValidation(FormField {
                    name: "email".to_string(),
                    field_type: FieldType::Email,
                    required: true,
                }),
                location: SourceLocation {
                    file: "unknown".to_string(),
                    line: 1,
                    column: 1,
                },
                context: Context {
                    function_name: None,
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.7,
            });
        }

        patterns
    }
}

#[async_trait]
impl TestGenerator for PythonAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(self.detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::Function(func) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test_{}", func.name),
                        description: format!("Test {} function", func.name),
                        input: serde_json::json!({}),
                        expected_output: serde_json::json!(null),
                    });
                }
                PatternType::FormValidation(field) => {
                    if field.field_type == FieldType::Email {
                        test_cases.push(TestCase {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: format!("test_{}_validation", field.name),
                            description: "Test email validation".to_string(),
                            input: serde_json::json!({"email": "test@example.com"}),
                            expected_output: serde_json::json!(true),
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(TestSuite {
            name: "Generated Python Tests".to_string(),
            language: "python".to_string(),
            framework: "pytest".to_string(),
            test_cases,
            imports: vec!["import pytest".to_string()],
            test_type: crate::core::TestType::Unit,
            setup_requirements: vec![],
            cleanup_requirements: vec![],
        })
    }

    fn get_language(&self) -> &str {
        "python"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["pytest", "unittest"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_adapter_new() {
        let adapter = PythonAdapter::new();
        assert_eq!(adapter.get_language(), "python");
    }

    #[test]
    fn test_get_language() {
        let adapter = PythonAdapter::new();
        assert_eq!(adapter.get_language(), "python");
    }

    #[test]
    fn test_get_supported_frameworks() {
        let adapter = PythonAdapter::new();
        let frameworks = adapter.get_supported_frameworks();
        assert_eq!(frameworks, vec!["pytest", "unittest"]);
    }

    #[test]
    fn test_detect_patterns_function_def() {
        let adapter = PythonAdapter::new();
        let source = "def calculate_area(length, width):\n    return length * width";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "calculate_area");
            assert_eq!(func.parameters, vec!["length".to_string(), "width".to_string()]);
            assert!(func.return_type.is_none());
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_patterns_function_no_params() {
        let adapter = PythonAdapter::new();
        let source = "def test():\n    pass";
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
    fn test_detect_patterns_email_validation() {
        let adapter = PythonAdapter::new();
        let source = "email = models.EmailField()";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::FormValidation(field) = &patterns[0].pattern_type {
            assert_eq!(field.name, "email");
            assert_eq!(field.field_type, FieldType::Email);
            assert!(field.required);
        } else {
            panic!("Expected FormValidation pattern");
        }
    }

    #[test]
    fn test_detect_patterns_email_keyword() {
        let adapter = PythonAdapter::new();
        let source = "def validate_email(email_address):\n    pass";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2); // Function + email validation
        
        // Check function pattern
        let function_pattern = patterns.iter().find(|p| matches!(p.pattern_type, PatternType::Function(_)));
        assert!(function_pattern.is_some());
        
        // Check email validation pattern
        let email_pattern = patterns.iter().find(|p| matches!(p.pattern_type, PatternType::FormValidation(_)));
        assert!(email_pattern.is_some());
    }

    #[test]
    fn test_detect_patterns_multiple_functions() {
        let adapter = PythonAdapter::new();
        let source = r#"
            def func1(x):
                return x * 2
            
            def func2(y, z):
                return y + z
        "#;
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_detect_patterns_no_matches() {
        let adapter = PythonAdapter::new();
        let source = "x = 42\nprint(x)";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn test_detect_patterns_mixed() {
        let adapter = PythonAdapter::new();
        let source = r#"
            def validate_user_email(email):
                return True
            
            class User:
                email = models.EmailField()
        "#;
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2); // Function + email field
    }

    #[tokio::test]
    async fn test_analyze_code() {
        let adapter = PythonAdapter::new();
        let source = "def test_function():\n    pass";
        let result = adapter.analyze_code(source, "test.py").await;
        assert!(result.is_ok());
        
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].confidence, 0.9);
    }

    #[tokio::test]
    async fn test_generate_tests_function_pattern() {
        let adapter = PythonAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "calculate_area".to_string(),
                parameters: vec!["length".to_string(), "width".to_string()],
                return_type: None,
            }),
            location: SourceLocation {
                file: "test.py".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("calculate_area".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.language, "python");
        assert_eq!(test_suite.framework, "pytest");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].name.contains("test_calculate_area"));
    }

    #[tokio::test]
    async fn test_generate_tests_email_validation() {
        let adapter = PythonAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::FormValidation(FormField {
                name: "email".to_string(),
                field_type: FieldType::Email,
                required: true,
            }),
            location: SourceLocation {
                file: "test.py".to_string(),
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
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].name.contains("test_email_validation"));
    }

    #[tokio::test]
    async fn test_generate_tests_non_email_field() {
        let adapter = PythonAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::FormValidation(FormField {
                name: "username".to_string(),
                field_type: FieldType::Text,
                required: false,
            }),
            location: SourceLocation {
                file: "test.py".to_string(),
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
        assert_eq!(test_suite.test_cases.len(), 0); // Non-email fields don't generate tests
    }

    #[tokio::test]
    async fn test_generate_tests_api_call_pattern() {
        let adapter = PythonAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::ApiCall(ApiEndpoint {
                method: HttpMethod::Post,
                path: "/api/data".to_string(),
                parameters: vec!["param1".to_string()],
            }),
            location: SourceLocation {
                file: "test.py".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: None,
                class_name: None,
                module_name: None,
            },
            confidence: 0.6,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0); // API calls don't generate tests in current impl
    }

    #[tokio::test]
    async fn test_generate_tests_empty_patterns() {
        let adapter = PythonAdapter::new();
        let result = adapter.generate_tests(vec![]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0);
        assert_eq!(test_suite.imports.len(), 1);
        assert_eq!(test_suite.imports[0], "import pytest");
    }

    #[tokio::test]
    async fn test_generate_tests_mixed_patterns() {
        let adapter = PythonAdapter::new();
        let patterns = vec![
            TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Function(FunctionPattern {
                    name: "process_data".to_string(),
                    parameters: vec!["data".to_string()],
                    return_type: None,
                }),
                location: SourceLocation {
                    file: "test.py".to_string(),
                    line: 1,
                    column: 1,
                },
                context: Context {
                    function_name: Some("process_data".to_string()),
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.9,
            },
            TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::FormValidation(FormField {
                    name: "email".to_string(),
                    field_type: FieldType::Email,
                    required: true,
                }),
                location: SourceLocation {
                    file: "test.py".to_string(),
                    line: 5,
                    column: 1,
                },
                context: Context {
                    function_name: None,
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.7,
            },
        ];

        let result = adapter.generate_tests(patterns).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 2); // 1 function + 1 email validation
    }
}