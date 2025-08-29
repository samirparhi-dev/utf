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

    fn detect_integration_patterns(&self, source: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();
        
        // Detect API calls (fetch, axios, http requests)
        if let Ok(api_regex) = Regex::new(r#"(?:fetch|axios\.(?:get|post|put|delete))\s*\(\s*[`'""]([^`'""]+)[`'""]"#) {
            for captures in api_regex.captures_iter(source) {
                if let Some(endpoint) = captures.get(1) {
                    let method = if source.contains("axios.post") || source.contains("POST") {
                        HttpMethod::Post
                    } else if source.contains("axios.put") || source.contains("PUT") {
                        HttpMethod::Put
                    } else if source.contains("axios.delete") || source.contains("DELETE") {
                        HttpMethod::Delete
                    } else {
                        HttpMethod::Get
                    };

                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::ApiIntegration(ApiIntegrationPattern {
                            endpoint: endpoint.as_str().to_string(),
                            method,
                            request_body: None,
                            response_type: None,
                            authentication_required: source.contains("Authorization") || source.contains("Bearer"),
                        }),
                        location: SourceLocation {
                            file: "unknown".to_string(),
                            line: 1,
                            column: endpoint.start(),
                        },
                        context: Context {
                            function_name: None,
                            class_name: None,
                            module_name: None,
                        },
                        confidence: 0.85,
                    });
                }
            }
        }

        // Detect React/Vue component integrations
        if let Ok(component_regex) = Regex::new(r"(?:export\s+(?:default\s+)?(?:function|const)\s+(\w+)|class\s+(\w+)\s+extends\s+(?:React\.)?Component)") {
            for captures in component_regex.captures_iter(source) {
                let component_name = captures.get(1).or(captures.get(2)).unwrap().as_str();
                let is_class = captures.get(2).is_some();
                
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::ComponentIntegration(ComponentPattern {
                        component_name: component_name.to_string(),
                        component_type: if source.contains("React") || source.contains("jsx") {
                            ComponentType::ReactComponent
                        } else if source.contains("Vue") {
                            ComponentType::VueComponent
                        } else {
                            ComponentType::Module
                        },
                        dependencies: self.extract_imports(source),
                        props_or_params: self.extract_props(source),
                    }),
                    location: SourceLocation {
                        file: "unknown".to_string(),
                        line: 1,
                        column: 1,
                    },
                    context: Context {
                        function_name: Some(component_name.to_string()),
                        class_name: if is_class { Some(component_name.to_string()) } else { None },
                        module_name: None,
                    },
                    confidence: 0.90,
                });
            }
        }

        // Detect database operations (mongoose, sequelize, prisma)
        if let Ok(db_regex) = Regex::new(r"(?:Model|model)\.(?:(create|find|update|delete|save|remove)\w*)") {
            for captures in db_regex.captures_iter(source) {
                if let Some(operation) = captures.get(1) {
                    let db_operation = match operation.as_str() {
                        "create" | "save" => DatabaseOperation::Create,
                        "find" => DatabaseOperation::Read,
                        "update" => DatabaseOperation::Update,
                        "delete" | "remove" => DatabaseOperation::Delete,
                        _ => DatabaseOperation::Query,
                    };

                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::DatabaseOperation(DatabasePattern {
                            operation_type: db_operation,
                            table_name: "unknown".to_string(),
                            method_name: operation.as_str().to_string(),
                            has_transaction: source.contains("transaction") || source.contains("Transaction"),
                        }),
                        location: SourceLocation {
                            file: "unknown".to_string(),
                            line: 1,
                            column: operation.start(),
                        },
                        context: Context {
                            function_name: None,
                            class_name: None,
                            module_name: None,
                        },
                        confidence: 0.80,
                    });
                }
            }
        }

        patterns
    }

    fn extract_imports(&self, source: &str) -> Vec<String> {
        let mut imports = Vec::new();
        if let Ok(import_regex) = Regex::new(r#"import.*from\s+[`'""]([^`'""]+)[`'""]"#) {
            for captures in import_regex.captures_iter(source) {
                if let Some(module) = captures.get(1) {
                    imports.push(module.as_str().to_string());
                }
            }
        }
        imports
    }

    fn extract_props(&self, source: &str) -> Vec<String> {
        let mut props = Vec::new();
        // Extract props from function parameters or PropTypes
        if let Ok(props_regex) = Regex::new(r"(?:function\s+\w+\s*\(\s*\{([^}]+)\}|propTypes\s*=\s*\{([^}]+)\})") {
            for captures in props_regex.captures_iter(source) {
                if let Some(props_str) = captures.get(1).or(captures.get(2)) {
                    let prop_names: Vec<String> = props_str.as_str()
                        .split(',')
                        .map(|p| p.trim().split(':').next().unwrap_or("").trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect();
                    props.extend(prop_names);
                }
            }
        }
        props
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
            test_type: crate::core::TestType::Unit,
            setup_requirements: vec![],
            cleanup_requirements: vec![],
        })
    }

    fn get_language(&self) -> &str {
        "javascript"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["jest", "mocha", "vitest"]
    }
}

#[async_trait]
impl IntegrationTestGenerator for JavaScriptAdapter {
    async fn analyze_integration_patterns(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(self.detect_integration_patterns(source))
    }

    async fn generate_integration_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::ApiIntegration(api) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test_api_integration_{}", api.endpoint.replace('/', "_").replace('-', "_")),
                        description: format!("Integration test for {} {}", api.method.to_string(), api.endpoint),
                        input: serde_json::json!({
                            "endpoint": api.endpoint,
                            "method": api.method,
                            "auth_required": api.authentication_required
                        }),
                        expected_output: serde_json::json!({
                            "status": "success",
                            "data": "mock_response"
                        }),
                    });
                }
                PatternType::ComponentIntegration(comp) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test_component_integration_{}", comp.component_name.to_lowercase()),
                        description: format!("Integration test for {} component", comp.component_name),
                        input: serde_json::json!({
                            "component": comp.component_name,
                            "props": comp.props_or_params,
                            "dependencies": comp.dependencies
                        }),
                        expected_output: serde_json::json!({
                            "rendered": true,
                            "interactions": "working"
                        }),
                    });
                }
                PatternType::DatabaseOperation(db) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test_database_{}_{}", db.operation_type.to_string().to_lowercase(), db.method_name),
                        description: format!("Integration test for database {} operation", db.operation_type.to_string().to_lowercase()),
                        input: serde_json::json!({
                            "operation": db.operation_type.to_string().to_lowercase(),
                            "table": db.table_name,
                            "has_transaction": db.has_transaction
                        }),
                        expected_output: serde_json::json!({
                            "success": true,
                            "affected_rows": 1
                        }),
                    });
                }
                _ => {}
            }
        }

        Ok(TestSuite {
            name: "Generated JavaScript Integration Tests".to_string(),
            language: "javascript".to_string(),
            framework: "jest".to_string(),
            test_cases,
            imports: vec![
                "const { expect } = require('@jest/globals');".to_string(),
                "const request = require('supertest');".to_string(),
                "const { setupTestDB, cleanupTestDB } = require('./test-helpers');".to_string(),
            ],
            test_type: TestType::Integration,
            setup_requirements: vec![
                "Start test database".to_string(),
                "Start test server".to_string(),
                "Setup test data".to_string(),
            ],
            cleanup_requirements: vec![
                "Clear test database".to_string(),
                "Stop test server".to_string(),
                "Reset mocks".to_string(),
            ],
        })
    }

    fn get_integration_frameworks(&self) -> Vec<&str> {
        vec!["jest", "cypress", "playwright", "supertest"]
    }

    fn get_setup_requirements(&self, patterns: &[TestablePattern]) -> Vec<String> {
        let mut requirements = Vec::new();
        
        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::ApiIntegration(_) => {
                    requirements.push("Mock external APIs".to_string());
                    requirements.push("Setup test server".to_string());
                }
                PatternType::DatabaseOperation(_) => {
                    requirements.push("Setup test database".to_string());
                    requirements.push("Run database migrations".to_string());
                }
                PatternType::ComponentIntegration(_) => {
                    requirements.push("Setup DOM environment".to_string());
                    requirements.push("Mock external dependencies".to_string());
                }
                _ => {}
            }
        }
        
        requirements.sort();
        requirements.dedup();
        requirements
    }

    fn get_cleanup_requirements(&self, patterns: &[TestablePattern]) -> Vec<String> {
        let mut requirements = Vec::new();
        
        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::ApiIntegration(_) => {
                    requirements.push("Reset API mocks".to_string());
                    requirements.push("Clear request logs".to_string());
                }
                PatternType::DatabaseOperation(_) => {
                    requirements.push("Truncate test tables".to_string());
                    requirements.push("Reset database state".to_string());
                }
                PatternType::ComponentIntegration(_) => {
                    requirements.push("Unmount components".to_string());
                    requirements.push("Clear DOM".to_string());
                }
                _ => {}
            }
        }
        
        requirements.sort();
        requirements.dedup();
        requirements
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