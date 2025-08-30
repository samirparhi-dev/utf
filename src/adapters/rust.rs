use crate::core::*;
use crate::templates::{TemplateEngine, TestTemplateData, TestPattern};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct RustAdapter;

impl RustAdapter {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_test_with_template(&self, pattern: &TestPattern, template_engine: &TemplateEngine) -> Result<String> {
        let template_data = match pattern {
            TestPattern::Function { name, params, return_type } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}", name.to_lowercase()),
                    description: format!("Test {} function", name),
                    inputs: self.generate_inputs_for_params(params),
                    expected_outputs: self.generate_outputs_for_return_type(return_type),
                    test_category: self.determine_test_category(name, params),
                    imports: vec!["use super::*;".to_string()],
                    setup_code: None,
                    teardown_code: None,
                }
            },
            TestPattern::AsyncFunction { name, params, return_type } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}_async", name.to_lowercase()),
                    description: format!("Test async {} function", name),
                    inputs: self.generate_inputs_for_params(params),
                    expected_outputs: self.generate_outputs_for_return_type(return_type),
                    test_category: self.determine_test_category(name, params),
                    imports: vec![
                        "use super::*;".to_string(),
                        "use tokio;".to_string(),
                        "use std::time::Duration;".to_string(),
                    ],
                    setup_code: None,
                    teardown_code: None,
                }
            },
            TestPattern::Class { name, methods: _methods } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}_struct", name.to_lowercase()),
                    description: format!("Test {} struct", name),
                    inputs: vec![],
                    expected_outputs: vec![],
                    test_category: "struct".to_string(),
                    imports: vec!["use super::*;".to_string()],
                    setup_code: Some(format!("let instance = {}::new();", name)),
                    teardown_code: None,
                }
            },
            TestPattern::ApiEndpoint { path, method, params } => {
                TestTemplateData {
                    function_name: format!("{}_{}", method.to_lowercase(), path.replace("/", "_")),
                    test_name: format!("test_api_{}_{}", method.to_lowercase(), path.replace("/", "_")),
                    description: format!("Test {} {} API endpoint", method, path),
                    inputs: self.generate_inputs_for_params(params),
                    expected_outputs: vec![serde_json::json!({"status": 200})],
                    test_category: "api".to_string(),
                    imports: vec![
                        "use super::*;".to_string(),
                        "use tokio;".to_string(),
                        "use std::collections::HashMap;".to_string(),
                    ],
                    setup_code: None,
                    teardown_code: None,
                }
            },
        };
        
        let template_name = match pattern {
            TestPattern::Function { .. } => "cargo/function_test",
            TestPattern::AsyncFunction { .. } => "cargo/async_test",
            TestPattern::Class { .. } => "cargo/struct_test",
            TestPattern::ApiEndpoint { .. } => "cargo/function_test", // Use function template for API tests
        };
        
        template_engine.render_test(template_name, &template_data)
    }
    
    fn generate_inputs_for_params(&self, params: &[String]) -> Vec<serde_json::Value> {
        params.iter().enumerate().map(|(i, param)| {
            match param.to_lowercase().as_str() {
                p if p.contains("string") || p.contains("str") => serde_json::json!("test_string"),
                p if p.contains("id") => serde_json::json!(i + 1),
                p if p.contains("name") => serde_json::json!(format!("test_name_{}", i)),
                p if p.contains("count") || p.contains("number") || p.contains("i32") => serde_json::json!(42),
                p if p.contains("bool") => serde_json::json!(true),
                p if p.contains("vec") || p.contains("array") => serde_json::json!([1, 2, 3]),
                p if p.contains("option") => serde_json::json!(null),
                _ => {
                    // Try to infer from Rust types
                    if param.contains("i32") || param.contains("u32") || param.contains("usize") {
                        serde_json::json!(42)
                    } else if param.contains("&str") || param.contains("String") {
                        serde_json::json!("test_value")
                    } else if param.contains("bool") {
                        serde_json::json!(true)
                    } else {
                        serde_json::json!(format!("test_value_{}", i))
                    }
                },
            }
        }).collect()
    }
    
    fn generate_outputs_for_return_type(&self, return_type: &Option<String>) -> Vec<serde_json::Value> {
        match return_type {
            Some(t) if t.contains("bool") => {
                vec![serde_json::json!(true), serde_json::json!(false)]
            },
            Some(t) if t.contains("i32") || t.contains("u32") || t.contains("usize") || t.contains("f32") || t.contains("f64") => {
                vec![serde_json::json!(42)]
            },
            Some(t) if t.contains("String") || t.contains("&str") => {
                vec![serde_json::json!("expected_result")]
            },
            Some(t) if t.contains("Vec") => {
                vec![serde_json::json!([1, 2, 3])]
            },
            Some(t) if t.contains("Option") => {
                vec![serde_json::json!(null), serde_json::json!("Some(value)")]
            },
            Some(t) if t.contains("Result") => {
                vec![serde_json::json!("Ok(value)"), serde_json::json!("Err(error)")]
            },
            Some(t) if t.contains("()") => {
                vec![serde_json::json!(null)]
            },
            _ => vec![serde_json::json!(0)], // Default for Rust
        }
    }
    
    fn determine_test_category(&self, name: &str, params: &[String]) -> String {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("email") || params.iter().any(|p| p.contains("email")) {
            "email_validation".to_string()
        } else if name_lower.contains("add") || name_lower.contains("calculate") || name_lower.contains("compute") {
            "numeric".to_string()
        } else if name_lower.contains("validate") || name_lower.contains("verify") {
            "validation".to_string()
        } else if name_lower.contains("parse") || name_lower.contains("format") {
            "string".to_string()
        } else if name_lower.contains("async") || name_lower.contains("await") {
            "async".to_string()
        } else {
            "general".to_string()
        }
    }

    fn generate_function_tests(&self, func: &FunctionPattern, source: &str) -> Vec<TestCase> {
        let mut tests = Vec::new();
        
        match func.name.as_str() {
            "add" | "sum" | "calculate_sum" => {
                tests.extend(self.generate_math_function_tests(func, "addition"));
            },
            "multiply" | "mul" | "product" => {
                tests.extend(self.generate_math_function_tests(func, "multiplication"));
            },
            "divide" | "div" => {
                tests.extend(self.generate_division_tests(func));
            },
            "main" => {
                tests.extend(self.generate_main_function_tests(func));
            },
            _ => {
                tests.extend(self.generate_generic_function_tests(func, source));
            }
        }
        
        tests
    }

    fn generate_math_function_tests(&self, func: &FunctionPattern, operation: &str) -> Vec<TestCase> {
        let func_name = &func.name;
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_positive_numbers", func_name),
                description: format!("Test {} with positive numbers", func_name),
                input: serde_json::json!({"a": 5, "b": 3}),
                expected_output: serde_json::json!(if operation == "addition" { 8 } else { 15 }),
                test_body: if operation == "addition" {
                    format!("        assert_eq!({}(5, 3), 8);\n        assert_eq!({}(10, 15), 25);\n        assert_eq!({}(0, 0), 0);\n        assert_eq!({}(1, 1), 2);\n", func_name, func_name, func_name, func_name)
                } else {
                    format!("        assert_eq!({}(5, 3), 15);\n        assert_eq!({}(4, 7), 28);\n        assert_eq!({}(1, 1), 1);\n        assert_eq!({}(2, 0), 0);\n", func_name, func_name, func_name, func_name)
                },
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_negative_numbers", func_name),
                description: format!("Test {} with negative numbers", func_name),
                input: serde_json::json!({"a": -5, "b": 3}),
                expected_output: serde_json::json!(if operation == "addition" { -2 } else { -15 }),
                test_body: if operation == "addition" {
                    format!("        assert_eq!({}(-5, 3), -2);\n        assert_eq!({}(-10, -5), -15);\n        assert_eq!({}(5, -3), 2);\n", func_name, func_name, func_name)
                } else {
                    format!("        assert_eq!({}(-5, 3), -15);\n        assert_eq!({}(-4, -2), 8);\n        assert_eq!({}(0, -5), 0);\n", func_name, func_name, func_name)
                },
                assertions: vec![],
                test_category: TestCategory::EdgeCase,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_boundary_values", func_name),
                description: format!("Test {} with boundary values", func_name),
                input: serde_json::json!({"a": 0, "b": 1}),
                expected_output: serde_json::json!(if operation == "addition" { 1 } else { 0 }),
                test_body: if operation == "addition" {
                    format!("        assert_eq!({}(0, 1), 1);\n        assert_eq!({}(i32::MAX, 0), i32::MAX);\n        assert_eq!({}(i32::MIN, 0), i32::MIN);\n", func_name, func_name, func_name)
                } else {
                    format!("        assert_eq!({}(0, 100), 0);\n        assert_eq!({}(100, 1), 100);\n        assert_eq!({}(i32::MAX, 1), i32::MAX);\n", func_name, func_name, func_name)
                },
                assertions: vec![],
                test_category: TestCategory::BoundaryCondition,
            },
        ]
    }

    fn generate_division_tests(&self, func: &FunctionPattern) -> Vec<TestCase> {
        let func_name = &func.name;
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_normal_division", func_name),
                description: format!("Test {} with normal cases", func_name),
                input: serde_json::json!({"a": 10, "b": 2}),
                expected_output: serde_json::json!(5),
                test_body: format!("        assert_eq!({}(10.0, 2.0), 5.0);\n        assert_eq!({}(15.0, 3.0), 5.0);\n        assert_eq!({}(1.0, 1.0), 1.0);\n", func_name, func_name, func_name),
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_division_by_zero", func_name),
                description: format!("Test {} division by zero handling", func_name),
                input: serde_json::json!({"a": 10, "b": 0}),
                expected_output: serde_json::json!("infinity"),
                test_body: format!("        // Division by zero should be handled appropriately\n        let result = {}(10.0, 0.0);\n        assert!(result.is_infinite() || result.is_nan());\n        \n        // Test with different numerators\n        assert!({}(5.0, 0.0).is_infinite());\n", func_name, func_name),
                assertions: vec![],
                test_category: TestCategory::ErrorHandling,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_negative_division", func_name),
                description: format!("Test {} with negative numbers", func_name),
                input: serde_json::json!({"a": -10, "b": 2}),
                expected_output: serde_json::json!(-5),
                test_body: format!("        assert_eq!({}(-10.0, 2.0), -5.0);\n        assert_eq!({}(10.0, -2.0), -5.0);\n        assert_eq!({}(-10.0, -2.0), 5.0);\n", func_name, func_name, func_name),
                assertions: vec![],
                test_category: TestCategory::EdgeCase,
            },
        ]
    }

    fn generate_main_function_tests(&self, _func: &FunctionPattern) -> Vec<TestCase> {
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: "test_main_execution".to_string(),
                description: "Test main function executes without panicking".to_string(),
                input: serde_json::json!({}),
                expected_output: serde_json::json!({}),
                test_body: "        // Test main function execution\n        // Note: main() typically doesn't return a value we can test\n        // This test ensures main doesn't panic\n        main();\n        // If we reach here, main() executed successfully\n        assert!(true);\n".to_string(),
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
        ]
    }

    fn generate_generic_function_tests(&self, func: &FunctionPattern, source: &str) -> Vec<TestCase> {
        let func_name = &func.name;
        let return_type = self.infer_return_type(func, source);
        
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_basic_functionality", func_name),
                description: format!("Test {} basic functionality", func_name),
                input: serde_json::json!({}),
                expected_output: serde_json::json!({}),
                test_body: if return_type == "bool" {
                    format!("        let result = {}();\n        assert!(result == true || result == false);\n        // Add more specific assertions based on expected behavior\n", func_name)
                } else if return_type.contains("i32") || return_type.contains("i64") {
                    format!("        let result = {}();\n        assert!(result >= i32::MIN as i64);\n        assert!(result <= i32::MAX as i64);\n        // Add specific value assertions\n", func_name)
                } else if return_type.contains("String") {
                    format!("        let result = {}();\n        assert!(!result.is_empty() || result.is_empty()); // Handle both cases\n        // Add specific string content assertions\n", func_name)
                } else {
                    format!("        // Test {} function\n        let result = {}();\n        // Add assertions based on expected function behavior\n        // assert_eq!(result, expected_value);\n", func_name, func_name)
                },
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
        ]
    }

    fn infer_return_type(&self, func: &FunctionPattern, source: &str) -> String {
        // Try to infer return type from function signature in source
        if let Some(return_type) = &func.return_type {
            return_type.clone()
        } else {
            // Parse source to find return type
            let function_pattern = format!("fn\\s+{}\\s*\\([^)]*\\)\\s*->\\s*([^\\s{{]+)", regex::escape(&func.name));
            if let Ok(regex) = regex::Regex::new(&function_pattern) {
                if let Some(captures) = regex.captures(source) {
                    if let Some(return_type) = captures.get(1) {
                        return return_type.as_str().to_string();
                    }
                }
            }
            "()".to_string() // Default to unit type
        }
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
        self.generate_comprehensive_tests(patterns, "").await
    }

    async fn generate_comprehensive_tests(&self, patterns: Vec<TestablePattern>, source: &str) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            if let PatternType::Function(func) = &pattern.pattern_type {
                test_cases.extend(self.generate_function_tests(func, source));
            }
        }

        let mut test_suite = TestSuite {
            name: "Generated Rust Tests".to_string(),
            language: "rust".to_string(),
            framework: "cargo-test".to_string(),
            test_cases,
            imports: vec![],
            test_type: crate::core::TestType::Unit,
            setup_requirements: vec![],
            cleanup_requirements: vec![],
            coverage_target: self.get_coverage_target(),
            test_code: None,
        };

        test_suite.test_code = Some(self.generate_test_code(&test_suite)?);
        Ok(test_suite)
    }

    fn get_coverage_target(&self) -> f32 {
        crate::core::CoverageStandards::get_coverage_target("rust")
    }

    fn generate_test_code(&self, test_suite: &TestSuite) -> Result<String> {
        let mut code = String::new();
        
        code.push_str("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");
        
        for test_case in &test_suite.test_cases {
            code.push_str(&format!("    #[test]\n    fn {}() {{\n", test_case.name));
            code.push_str(&format!("        // {}\n", test_case.description));
            code.push_str(&test_case.test_body);
            code.push_str("    }\n\n");
        }
        
        code.push_str("}\n");
        Ok(code)
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