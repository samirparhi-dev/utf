use crate::core::*;
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct JavaScriptAdapter;

impl JavaScriptAdapter {
    pub fn new() -> Self {
        Self
    }

    fn detect_patterns(&self, source: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();
        
        // Simple regex-based pattern detection for demo
        if let Ok(email_regex) = Regex::new(r#"type\s*=\s*["']email["']"#) {
            if email_regex.is_match(source) {
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
                    confidence: 0.8,
                });
            }
        }

        // Detect function patterns
        if let Ok(function_regex) = Regex::new(r"function\s+(\w+)\s*\(([^)]*)\)") {
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
impl TestGenerator for JavaScriptAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(self.detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::FormValidation(field) => {
                    if field.field_type == FieldType::Email {
                        test_cases.push(TestCase {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: format!("test_{}_valid_email", field.name),
                            description: "Test valid email input".to_string(),
                            input: serde_json::json!({"email": "test@example.com"}),
                            expected_output: serde_json::json!(true),
                        });
                        
                        test_cases.push(TestCase {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: format!("test_{}_invalid_email", field.name),
                            description: "Test invalid email input".to_string(),
                            input: serde_json::json!({"email": "invalid-email"}),
                            expected_output: serde_json::json!(false),
                        });
                    }
                }
                PatternType::Function(func) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test_{}", func.name),
                        description: format!("Test {} function", func.name),
                        input: serde_json::json!({}),
                        expected_output: serde_json::json!(null),
                    });
                }
                _ => {}
            }
        }

        Ok(TestSuite {
            name: "Generated JavaScript Tests".to_string(),
            language: "javascript".to_string(),
            framework: "jest".to_string(),
            test_cases,
            imports: vec!["const { expect } = require('@jest/globals');".to_string()],
        })
    }

    fn get_language(&self) -> &str {
        "javascript"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["jest", "mocha", "vitest"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_javascript_adapter_new() {
        let adapter = JavaScriptAdapter::new();
        assert_eq!(adapter.get_language(), "javascript");
    }

    #[test]
    fn test_get_language() {
        let adapter = JavaScriptAdapter::new();
        assert_eq!(adapter.get_language(), "javascript");
    }

    #[test]
    fn test_get_supported_frameworks() {
        let adapter = JavaScriptAdapter::new();
        let frameworks = adapter.get_supported_frameworks();
        assert_eq!(frameworks, vec!["jest", "mocha", "vitest"]);
    }

    #[test]
    fn test_detect_patterns_email_field() {
        let adapter = JavaScriptAdapter::new();
        let source = r#"<input type="email" name="userEmail" required />"#;
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
    fn test_detect_patterns_function() {
        let adapter = JavaScriptAdapter::new();
        let source = "function calculateSum(a, b) { return a + b; }";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "calculateSum");
            assert_eq!(func.parameters, vec!["a".to_string(), "b".to_string()]);
            assert!(func.return_type.is_none());
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_patterns_function_no_params() {
        let adapter = JavaScriptAdapter::new();
        let source = "function test() { return 'hello'; }";
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
        let adapter = JavaScriptAdapter::new();
        let source = r#"
            function add(x, y) { return x + y; }
            function subtract(a, b) { return a - b; }
        "#;
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_detect_patterns_no_matches() {
        let adapter = JavaScriptAdapter::new();
        let source = "const x = 42; console.log(x);";
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 0);
    }

    #[test]
    fn test_detect_patterns_both_email_and_function() {
        let adapter = JavaScriptAdapter::new();
        let source = r#"
            function validateEmail(email) { return true; }
            const emailInput = '<input type="email" name="userEmail" required />';
        "#;
        let patterns = adapter.detect_patterns(source);
        assert_eq!(patterns.len(), 2);
    }

    #[tokio::test]
    async fn test_analyze_code() {
        let adapter = JavaScriptAdapter::new();
        let source = "function test() {}";
        let result = adapter.analyze_code(source, "test.js").await;
        assert!(result.is_ok());
        
        let patterns = result.unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].confidence, 0.9);
    }

    #[tokio::test]
    async fn test_generate_tests_function_pattern() {
        let adapter = JavaScriptAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "testFunction".to_string(),
                parameters: vec!["param1".to_string()],
                return_type: None,
            }),
            location: SourceLocation {
                file: "test.js".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("testFunction".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };

        let result = adapter.generate_tests(vec![pattern]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.language, "javascript");
        assert_eq!(test_suite.framework, "jest");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].name.contains("test_testFunction"));
    }

    #[tokio::test]
    async fn test_generate_tests_email_validation() {
        let adapter = JavaScriptAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::FormValidation(FormField {
                name: "email".to_string(),
                field_type: FieldType::Email,
                required: true,
            }),
            location: SourceLocation {
                file: "test.js".to_string(),
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
        assert_eq!(test_suite.test_cases.len(), 2); // valid and invalid email tests
        
        let test_names: Vec<&String> = test_suite.test_cases.iter().map(|t| &t.name).collect();
        assert!(test_names.iter().any(|name| name.contains("valid_email")));
        assert!(test_names.iter().any(|name| name.contains("invalid_email")));
    }

    #[tokio::test]
    async fn test_generate_tests_api_call_pattern() {
        let adapter = JavaScriptAdapter::new();
        let pattern = TestablePattern {
            id: uuid::Uuid::new_v4().to_string(),
            pattern_type: PatternType::ApiCall(ApiEndpoint {
                method: HttpMethod::Get,
                path: "/api/users".to_string(),
                parameters: vec!["id".to_string()],
            }),
            location: SourceLocation {
                file: "test.js".to_string(),
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
        assert_eq!(test_suite.test_cases.len(), 0); // API calls don't generate tests in current impl
    }

    #[tokio::test]
    async fn test_generate_tests_empty_patterns() {
        let adapter = JavaScriptAdapter::new();
        let result = adapter.generate_tests(vec![]).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 0);
        assert_eq!(test_suite.imports.len(), 1);
        assert!(test_suite.imports[0].contains("@jest/globals"));
    }

    #[tokio::test] 
    async fn test_generate_tests_mixed_patterns() {
        let adapter = JavaScriptAdapter::new();
        let patterns = vec![
            TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Function(FunctionPattern {
                    name: "func1".to_string(),
                    parameters: vec![],
                    return_type: None,
                }),
                location: SourceLocation {
                    file: "test.js".to_string(),
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
                pattern_type: PatternType::FormValidation(FormField {
                    name: "email".to_string(),
                    field_type: FieldType::Email,
                    required: true,
                }),
                location: SourceLocation {
                    file: "test.js".to_string(),
                    line: 5,
                    column: 1,
                },
                context: Context {
                    function_name: None,
                    class_name: None,
                    module_name: None,
                },
                confidence: 0.8,
            },
        ];

        let result = adapter.generate_tests(patterns).await;
        assert!(result.is_ok());
        
        let test_suite = result.unwrap();
        assert_eq!(test_suite.test_cases.len(), 3); // 1 function + 2 email tests
    }
}