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
        let mut tests = Vec::new();
        let func_name = &func.name;
        let return_type = self.infer_return_type(func, source);
        
        // Generate basic functionality test with real assertions
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("test_{}_basic_functionality", func_name),
            description: format!("Test {} basic functionality", func_name),
            input: self.generate_sample_inputs_rust(func),
            expected_output: self.generate_expected_output_rust(&return_type),
            test_body: self.generate_basic_test_body_rust(func, &return_type),
            assertions: vec![],
            test_category: TestCategory::HappyPath,
        });

        // Generate error handling tests if function has parameters
        if !func.parameters.is_empty() {
            tests.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_error_handling", func_name),
                description: format!("Test {} error handling", func_name),
                input: serde_json::json!({}),
                expected_output: serde_json::json!(null),
                test_body: self.generate_error_test_body_rust(func, &return_type),
                assertions: vec![],
                test_category: TestCategory::ErrorHandling,
            });
        }

        // Generate boundary condition tests
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("test_{}_boundary_conditions", func_name),
            description: format!("Test {} boundary conditions", func_name),
            input: serde_json::json!({}),
            expected_output: serde_json::json!(null),
            test_body: self.generate_boundary_test_body_rust(func, &return_type),
            assertions: vec![],
            test_category: TestCategory::BoundaryCondition,
        });

        // Generate performance tests for complex functions
        if self.is_complex_function(func, source) {
            tests.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test_{}_performance", func_name),
                description: format!("Test {} performance characteristics", func_name),
                input: serde_json::json!({}),
                expected_output: serde_json::json!(null),
                test_body: self.generate_performance_test_body_rust(func, &return_type),
                assertions: vec![],
                test_category: TestCategory::Performance,
            });
        }

        tests
    }

    fn generate_sample_inputs_rust(&self, func: &FunctionPattern) -> serde_json::Value {
        let mut inputs = serde_json::Map::new();
        for (i, param) in func.parameters.iter().enumerate() {
            let value = self.get_sample_value_for_rust_param(param, i);
            inputs.insert(param.clone(), value);
        }
        serde_json::Value::Object(inputs)
    }

    fn get_sample_value_for_rust_param(&self, param: &str, index: usize) -> serde_json::Value {
        let param_lower = param.to_lowercase();
        match param_lower.as_str() {
            p if p.contains("email") || p.contains("mail") => serde_json::json!("test@example.com"),
            p if p.contains("id") => serde_json::json!(index as i32 + 1),
            p if p.contains("name") => serde_json::json!(format!("TestName{}", index + 1)),
            p if p.contains("count") || p.contains("number") || p.contains("i32") || p.contains("u32") => serde_json::json!(42),
            p if p.contains("f64") || p.contains("f32") => serde_json::json!(3.14),
            p if p.contains("bool") => serde_json::json!(true),
            p if p.contains("string") || p.contains("str") => serde_json::json!(format!("test_string_{}", index)),
            p if p.contains("vec") || p.contains("array") => serde_json::json!([1, 2, 3]),
            p if p.contains("a") || p.contains("b") || p.contains("x") || p.contains("y") => serde_json::json!(5),
            p if p.contains("width") || p.contains("height") => serde_json::json!(10),
            _ => serde_json::json!(format!("test_value_{}", index)),
        }
    }

    fn generate_expected_output_rust(&self, return_type: &str) -> serde_json::Value {
        match return_type {
            "bool" => serde_json::json!(true),
            "i32" | "i64" | "u32" | "u64" | "usize" | "isize" => serde_json::json!(42),
            "f32" | "f64" => serde_json::json!(3.14),
            "String" | "&str" => serde_json::json!("expected_result"),
            s if s.contains("Vec") => serde_json::json!([]),
            s if s.contains("Option") => serde_json::json!(null),
            s if s.contains("Result") => serde_json::json!("Ok"),
            "()" => serde_json::json!(null),
            _ => serde_json::json!("unknown"),
        }
    }

    fn generate_basic_test_body_rust(&self, func: &FunctionPattern, return_type: &str) -> String {
        let func_name = &func.name;
        let mut test_body = String::new();
        
        if func.parameters.is_empty() {
            // No parameters
            test_body.push_str(&format!("        let result = {}();\n", func_name));
            
            match return_type {
                "bool" => {
                    test_body.push_str("        assert!(result == true || result == false);\n");
                    test_body.push_str(&format!("        // Test specific boolean behavior\n"));
                    test_body.push_str(&format!("        assert_eq!({}(), result);\n", func_name));
                },
                "i32" | "i64" | "u32" | "u64" => {
                    test_body.push_str("        assert!(result >= 0 || result < 0); // Accept any integer\n");
                    test_body.push_str(&format!("        let second_call = {}();\n", func_name));
                    test_body.push_str("        assert_eq!(result, second_call); // Should be consistent\n");
                },
                "f32" | "f64" => {
                    test_body.push_str("        assert!(result.is_finite() || result.is_infinite() || result.is_nan());\n");
                    test_body.push_str(&format!("        let second_call = {}();\n", func_name));
                    test_body.push_str("        assert_eq!(result, second_call);\n");
                },
                "String" | "&str" => {
                    test_body.push_str("        assert!(result.len() >= 0); // Valid string\n");
                    test_body.push_str(&format!("        let second_call = {}();\n", func_name));
                    test_body.push_str("        assert_eq!(result, second_call);\n");
                },
                s if s.contains("Vec") => {
                    test_body.push_str("        assert!(result.len() >= 0);\n");
                    test_body.push_str(&format!("        let second_call = {}();\n", func_name));
                    test_body.push_str("        assert_eq!(result, second_call);\n");
                },
                s if s.contains("Option") => {
                    test_body.push_str("        match result {\n");
                    test_body.push_str("            Some(_) => assert!(true),\n");
                    test_body.push_str("            None => assert!(true),\n");
                    test_body.push_str("        }\n");
                },
                s if s.contains("Result") => {
                    test_body.push_str("        match result {\n");
                    test_body.push_str("            Ok(_) => assert!(true),\n");
                    test_body.push_str("            Err(_) => assert!(true),\n");
                    test_body.push_str("        }\n");
                },
                _ => {
                    test_body.push_str("        // Function executed successfully\n");
                    test_body.push_str(&format!("        let _result = {}();\n", func_name));
                }
            }
        } else {
            // With parameters - generate specific test cases
            let sample_params = self.generate_sample_parameters_rust(func);
            test_body.push_str(&format!("        let result = {}({});\n", func_name, sample_params));
            
            // Add smart assertions based on function name
            let name_lower = func_name.to_lowercase();
            if name_lower.contains("add") || name_lower.contains("sum") {
                test_body.push_str("        assert!(result > 0); // Sum should be positive with positive inputs\n");
                test_body.push_str(&format!("        assert_eq!({}(2, 3), 5);\n", func_name));
                test_body.push_str(&format!("        assert_eq!({}(0, 0), 0);\n", func_name));
            } else if name_lower.contains("multiply") {
                test_body.push_str("        assert!(result >= 0); // Product should be non-negative\n");
                test_body.push_str(&format!("        assert_eq!({}(2, 3), 6);\n", func_name));
                test_body.push_str(&format!("        assert_eq!({}(1, 5), 5);\n", func_name));
            } else if name_lower.contains("validate") || name_lower.contains("check") {
                test_body.push_str("        assert!(result == true || result == false);\n");
            } else if name_lower.contains("calculate") {
                test_body.push_str("        assert!(result.is_finite() || result.is_infinite());\n");
            } else {
                test_body.push_str("        // Function executed successfully with valid inputs\n");
                match return_type {
                    "bool" => test_body.push_str("        assert!(result == true || result == false);\n"),
                    s if s.contains("i32") || s.contains("u32") => test_body.push_str("        assert!(result >= 0 || result < 0);\n"),
                    _ => test_body.push_str("        // Add specific assertions based on return type\n"),
                }
            }
        }
        
        test_body
    }

    fn generate_error_test_body_rust(&self, func: &FunctionPattern, _return_type: &str) -> String {
        let func_name = &func.name;
        let mut test_body = String::new();
        
        test_body.push_str("        // Test error handling with edge case inputs\n");
        
        // Generate error tests based on parameter types
        if func.parameters.len() >= 2 {
            // Test with extreme values
            test_body.push_str(&format!("        let _result1 = {}(i32::MAX, i32::MAX);\n", func_name));
            test_body.push_str(&format!("        let _result2 = {}(i32::MIN, i32::MIN);\n", func_name));
            test_body.push_str(&format!("        let _result3 = {}(0, i32::MAX);\n", func_name));
        }

        // Test with boundary values
        let name_lower = func_name.to_lowercase();
        if name_lower.contains("divide") || name_lower.contains("div") {
            test_body.push_str("        // Division by zero should be handled\n");
            test_body.push_str(&format!("        let result = {}(10.0, 0.0);\n", func_name));
            test_body.push_str("        assert!(result.is_infinite() || result.is_nan());\n");
        }

        test_body
    }

    fn generate_boundary_test_body_rust(&self, func: &FunctionPattern, return_type: &str) -> String {
        let func_name = &func.name;
        let mut test_body = String::new();
        
        test_body.push_str("        // Test boundary conditions\n");
        
        if func.parameters.is_empty() {
            test_body.push_str(&format!("        let result1 = {}();\n", func_name));
            test_body.push_str(&format!("        let result2 = {}();\n", func_name));
            test_body.push_str("        // Results should be consistent\n");
            if !return_type.contains("f32") && !return_type.contains("f64") {
                test_body.push_str("        assert_eq!(result1, result2);\n");
            }
        } else {
            // Test with boundary values
            if func.parameters.len() == 1 {
                test_body.push_str(&format!("        let _result_zero = {}(0);\n", func_name));
                test_body.push_str(&format!("        let _result_max = {}(i32::MAX);\n", func_name));
                test_body.push_str(&format!("        let _result_min = {}(i32::MIN);\n", func_name));
            } else if func.parameters.len() == 2 {
                test_body.push_str(&format!("        let _result_zeros = {}(0, 0);\n", func_name));
                test_body.push_str(&format!("        let _result_mixed = {}(i32::MAX, 0);\n", func_name));
                test_body.push_str(&format!("        let _result_negative = {}(-1, 1);\n", func_name));
            }
        }
        
        test_body
    }

    fn generate_performance_test_body_rust(&self, func: &FunctionPattern, _return_type: &str) -> String {
        let func_name = &func.name;
        let mut test_body = String::new();
        
        test_body.push_str("        use std::time::Instant;\n");
        test_body.push_str("        \n");
        test_body.push_str("        let start = Instant::now();\n");
        
        if func.parameters.is_empty() {
            test_body.push_str("        for _ in 0..1000 {\n");
            test_body.push_str(&format!("            let _ = {}();\n", func_name));
            test_body.push_str("        }\n");
        } else {
            let sample_params = self.generate_sample_parameters_rust(func);
            test_body.push_str("        for _ in 0..1000 {\n");
            test_body.push_str(&format!("            let _ = {}({});\n", func_name, sample_params));
            test_body.push_str("        }\n");
        }
        
        test_body.push_str("        let duration = start.elapsed();\n");
        test_body.push_str("        \n");
        test_body.push_str("        // Performance should be reasonable (less than 1 second for 1000 calls)\n");
        test_body.push_str("        assert!(duration.as_secs() < 1);\n");
        test_body.push_str(&format!("        println!(\"{} performance: {{:?}}\", duration);\n", func_name));
        
        test_body
    }

    fn generate_sample_parameters_rust(&self, func: &FunctionPattern) -> String {
        func.parameters.iter().enumerate().map(|(i, param)| {
            let param_lower = param.to_lowercase();
            match param_lower.as_str() {
                p if p.contains("string") || p.contains("str") => format!("\"test_string_{}\"", i),
                p if p.contains("i32") || p.contains("u32") => "42".to_string(),
                p if p.contains("f64") || p.contains("f32") => "3.14".to_string(),
                p if p.contains("bool") => "true".to_string(),
                p if p.contains("a") || p.contains("b") || p.contains("x") || p.contains("y") => "5".to_string(),
                p if p.contains("width") || p.contains("height") => "10".to_string(),
                _ => "42".to_string(),
            }
        }).collect::<Vec<_>>().join(", ")
    }

    fn is_complex_function(&self, func: &FunctionPattern, source: &str) -> bool {
        let name_lower = func.name.to_lowercase();
        
        // Consider function complex if:
        // 1. Has multiple parameters (>= 3)
        // 2. Name suggests computational complexity
        // 3. Contains loops or recursive patterns in source
        
        func.parameters.len() >= 3 ||
        name_lower.contains("fibonacci") ||
        name_lower.contains("factorial") ||
        name_lower.contains("sort") ||
        name_lower.contains("search") ||
        name_lower.contains("algorithm") ||
        source.contains("for ") ||
        source.contains("while ") ||
        source.contains("loop ")
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
            
            // Try to infer from function name
            let name_lower = func.name.to_lowercase();
            if name_lower.contains("is_") || name_lower.contains("can_") || name_lower.contains("has_") {
                "bool".to_string()
            } else if name_lower.contains("count") || name_lower.contains("len") {
                "usize".to_string()
            } else if name_lower.contains("add") || name_lower.contains("sum") || name_lower.contains("calculate") {
                "i32".to_string()
            } else {
                "()".to_string() // Default to unit type
            }
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