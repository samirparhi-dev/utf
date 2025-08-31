use crate::core::*;
use crate::templates::{TemplateEngine, TestTemplateData, TestPattern};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct JavaScriptAdapter;

impl JavaScriptAdapter {
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
                    imports: vec!["const { expect } = require('@jest/globals');".to_string()],
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
                    imports: vec!["const { expect } = require('@jest/globals');".to_string()],
                    setup_code: None,
                    teardown_code: None,
                }
            },
            TestPattern::Class { name, methods: _methods } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}_class", name.to_lowercase()),
                    description: format!("Test {} class", name),
                    inputs: vec![],
                    expected_outputs: vec![],
                    test_category: "class".to_string(),
                    imports: vec!["const { expect } = require('@jest/globals');".to_string()],
                    setup_code: Some(format!("const instance = new {}();", name)),
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
                        "const { expect } = require('@jest/globals');".to_string(),
                        "const fetch = require('node-fetch');".to_string(),
                    ],
                    setup_code: None,
                    teardown_code: None,
                }
            },
        };
        
        let template_name = match pattern {
            TestPattern::Function { .. } => "jest/function_test",
            TestPattern::AsyncFunction { .. } => "jest/async_test",
            TestPattern::Class { .. } => "jest/class_test",
            TestPattern::ApiEndpoint { .. } => "jest/api_test",
        };
        
        template_engine.render_test(template_name, &template_data)
    }
    
    fn generate_inputs_for_params(&self, params: &[String]) -> Vec<serde_json::Value> {
        params.iter().enumerate().map(|(i, param)| {
            match param.to_lowercase().as_str() {
                p if p.contains("email") => serde_json::json!("test@example.com"),
                p if p.contains("id") => serde_json::json!(i + 1),
                p if p.contains("name") => serde_json::json!(format!("test_name_{}", i)),
                p if p.contains("count") || p.contains("number") => serde_json::json!(42),
                p if p.contains("bool") => serde_json::json!(true),
                p if p.contains("array") || p.contains("list") => serde_json::json!([1, 2, 3]),
                _ => serde_json::json!(format!("test_value_{}", i)),
            }
        }).collect()
    }
    
    fn generate_outputs_for_return_type(&self, return_type: &Option<String>) -> Vec<serde_json::Value> {
        match return_type {
            Some(t) if t.contains("boolean") || t.contains("bool") => {
                vec![serde_json::json!(true), serde_json::json!(false)]
            },
            Some(t) if t.contains("number") || t.contains("int") => {
                vec![serde_json::json!(42)]
            },
            Some(t) if t.contains("string") => {
                vec![serde_json::json!("expected_result")]
            },
            Some(t) if t.contains("array") || t.contains("Array") => {
                vec![serde_json::json!([1, 2, 3])]
            },
            Some(t) if t.contains("object") || t.contains("Object") => {
                vec![serde_json::json!({"key": "value"})]
            },
            _ => vec![serde_json::json!(null)],
        }
    }
    
    fn determine_test_category(&self, name: &str, params: &[String]) -> String {
        let name_lower = name.to_lowercase();
        
        if name_lower.contains("email") || params.iter().any(|p| p.contains("email")) {
            "email_validation".to_string()
        } else if name_lower.contains("calculate") || name_lower.contains("compute") {
            "numeric".to_string()
        } else if name_lower.contains("validate") || name_lower.contains("verify") {
            "validation".to_string()
        } else if name_lower.contains("parse") || name_lower.contains("format") {
            "string".to_string()
        } else if name_lower.contains("async") || name_lower.contains("promise") {
            "async".to_string()
        } else {
            "general".to_string()
        }
    }

    fn generate_email_validation_tests(&self, field: &FormField) -> Vec<TestCase> {
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_validate_correct_{}_format", field.name),
                description: format!("Test valid {} input formats", field.name),
                input: serde_json::json!({"email": "user@example.com"}),
                expected_output: serde_json::json!(true),
                test_body: "    expect(validateEmail('user@example.com')).toBe(true);\n    expect(validateEmail('test.email+tag@example.co.uk')).toBe(true);\n    expect(validateEmail('user.name@domain.org')).toBe(true);\n".to_string(),
                assertions: vec![
                    "expect(validateEmail('user@example.com')).toBe(true);".to_string(),
                    "expect(validateEmail('test.email+tag@example.co.uk')).toBe(true);".to_string(),
                ],
                test_category: TestCategory::HappyPath,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_reject_invalid_{}_formats", field.name),
                description: format!("Test invalid {} input formats", field.name),
                input: serde_json::json!({"email": "invalid-email"}),
                expected_output: serde_json::json!(false),
                test_body: "    expect(validateEmail('invalid-email')).toBe(false);\n    expect(validateEmail('@example.com')).toBe(false);\n    expect(validateEmail('user@')).toBe(false);\n    expect(validateEmail('')).toBe(false);\n".to_string(),
                assertions: vec![
                    "expect(validateEmail('invalid-email')).toBe(false);".to_string(),
                    "expect(validateEmail('@example.com')).toBe(false);".to_string(),
                ],
                test_category: TestCategory::EdgeCase,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_handle_{}_boundary_conditions", field.name),
                description: format!("Test {} boundary conditions", field.name),
                input: serde_json::json!({"email": "a@b.co"}),
                expected_output: serde_json::json!(true),
                test_body: "    expect(validateEmail('a@b.co')).toBe(true);\n    expect(validateEmail('verylongusernamepart@verylongdomainname.verylongtld')).toBe(true);\n    expect(validateEmail('user@domain')).toBe(false);\n".to_string(),
                assertions: vec![],
                test_category: TestCategory::BoundaryCondition,
            },
        ]
    }

    fn generate_function_tests(&self, func: &FunctionPattern, source: &str) -> Vec<TestCase> {
        let mut tests = Vec::new();
        
        match func.name.as_str() {
            "calculateSum" | "add" | "sum" => {
                tests.extend(self.generate_math_function_tests(func, "addition"));
            },
            "multiply" | "product" => {
                tests.extend(self.generate_math_function_tests(func, "multiplication"));
            },
            "validateEmail" => {
                tests.extend(self.generate_email_function_tests(func));
            },
            _ => {
                tests.extend(self.generate_generic_function_tests(func, source));
            }
        }
        
        tests
    }

    fn generate_math_function_tests(&self, func: &FunctionPattern, operation: &str) -> Vec<TestCase> {
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_perform_{}_correctly", operation),
                description: format!("Test {} with positive numbers", func.name),
                input: serde_json::json!({"a": 2, "b": 3}),
                expected_output: serde_json::json!(if operation == "addition" { 5 } else { 6 }),
                test_body: if operation == "addition" {
                    "    expect(calculateSum(2, 3)).toBe(5);\n    expect(calculateSum(10, 15)).toBe(25);\n    expect(calculateSum(0, 0)).toBe(0);\n".to_string()
                } else {
                    "    expect(multiply(2, 3)).toBe(6);\n    expect(multiply(4, 5)).toBe(20);\n    expect(multiply(1, 1)).toBe(1);\n".to_string()
                },
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_handle_negative_numbers_in_{}", operation),
                description: format!("Test {} with negative numbers", func.name),
                input: serde_json::json!({"a": -2, "b": 3}),
                expected_output: serde_json::json!(if operation == "addition" { 1 } else { -6 }),
                test_body: if operation == "addition" {
                    "    expect(calculateSum(-2, 3)).toBe(1);\n    expect(calculateSum(-5, -3)).toBe(-8);\n    expect(calculateSum(5, -2)).toBe(3);\n".to_string()
                } else {
                    "    expect(multiply(-2, 3)).toBe(-6);\n    expect(multiply(-4, -2)).toBe(8);\n    expect(multiply(0, -5)).toBe(0);\n".to_string()
                },
                assertions: vec![],
                test_category: TestCategory::EdgeCase,
            },
        ]
    }

    fn generate_email_function_tests(&self, func: &FunctionPattern) -> Vec<TestCase> {
        vec![
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: "should_validate_correct_email_formats".to_string(),
                description: "Test email validation with valid formats".to_string(),
                input: serde_json::json!({"email": "user@example.com"}),
                expected_output: serde_json::json!(true),
                test_body: "    expect(validateEmail('user@example.com')).toBe(true);\n    expect(validateEmail('test.email@example.co.uk')).toBe(true);\n    expect(validateEmail('user+tag@domain.org')).toBe(true);\n".to_string(),
                assertions: vec![],
                test_category: TestCategory::HappyPath,
            },
            TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: "should_reject_invalid_email_formats".to_string(),
                description: "Test email validation with invalid formats".to_string(),
                input: serde_json::json!({"email": "invalid"}),
                expected_output: serde_json::json!(false),
                test_body: "    expect(validateEmail('invalid')).toBe(false);\n    expect(validateEmail('@example.com')).toBe(false);\n    expect(validateEmail('user@')).toBe(false);\n    expect(validateEmail('')).toBe(false);\n".to_string(),
                assertions: vec![],
                test_category: TestCategory::EdgeCase,
            },
        ]
    }

    fn generate_generic_function_tests(&self, func: &FunctionPattern, source: &str) -> Vec<TestCase> {
        let mut tests = Vec::new();
        
        // Generate basic functionality test with real assertions
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("should_execute_{}_with_valid_input", func.name),
            description: format!("Test {} function with valid input", func.name),
            input: self.generate_sample_inputs(func),
            expected_output: self.generate_expected_output(func),
            test_body: self.generate_basic_test_body(func),
            assertions: vec![],
            test_category: TestCategory::HappyPath,
        });

        // Generate boundary condition tests
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("should_handle_{}_boundary_conditions", func.name),
            description: format!("Test {} function boundary conditions", func.name),
            input: serde_json::json!({}),
            expected_output: serde_json::json!(null),
            test_body: self.generate_boundary_test_body(func),
            assertions: vec![],
            test_category: TestCategory::BoundaryCondition,
        });

        // Generate error handling tests
        tests.push(TestCase {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("should_handle_{}_error_cases", func.name),
            description: format!("Test {} function error handling", func.name),
            input: serde_json::json!({}),
            expected_output: serde_json::json!(null),
            test_body: self.generate_error_test_body(func),
            assertions: vec![],
            test_category: TestCategory::ErrorHandling,
        });

        // Generate type validation tests for functions with parameters
        if !func.parameters.is_empty() {
            tests.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_validate_{}_input_types", func.name),
                description: format!("Test {} function input type validation", func.name),
                input: serde_json::json!({}),
                expected_output: serde_json::json!(null),
                test_body: self.generate_type_validation_test_body(func),
                assertions: vec![],
                test_category: TestCategory::EdgeCase,
            });
        }

        // Generate async tests if function appears to be async
        if self.is_async_function(source, &func.name) {
            tests.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("should_handle_{}_async_execution", func.name),
                description: format!("Test {} async function execution", func.name),
                input: serde_json::json!({}),
                expected_output: serde_json::json!(null),
                test_body: self.generate_async_test_body(func),
                assertions: vec![],
                test_category: TestCategory::Integration,
            });
        }

        tests
    }

    fn detect_patterns(&self, source: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();
        
        // Detect email form fields
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

        // Detect function declarations: function name(params)
        if let Ok(function_regex) = Regex::new(r"function\s+(\w+)\s*\(([^)]*)\)") {
            for captures in function_regex.captures_iter(source) {
                if let (Some(name), Some(params)) = (captures.get(1), captures.get(2)) {
                    let line_num = source[..captures.get(0).unwrap().start()].matches('\n').count() + 1;
                    let params_list: Vec<String> = if params.as_str().trim().is_empty() {
                        vec![]
                    } else {
                        params.as_str().split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
                    };
                    
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Function(FunctionPattern {
                            name: name.as_str().to_string(),
                            parameters: params_list,
                            return_type: self.infer_return_type(source, name.as_str()),
                        }),
                        location: SourceLocation {
                            file: "unknown".to_string(),
                            line: line_num,
                            column: name.start(),
                        },
                        context: Context {
                            function_name: Some(name.as_str().to_string()),
                            class_name: self.extract_containing_class(source, name.start()),
                            module_name: None,
                        },
                        confidence: 0.9,
                    });
                }
            }
        }

        // Detect arrow functions: const name = (params) => {}
        if let Ok(arrow_regex) = Regex::new(r"(?:const|let|var)\s+(\w+)\s*=\s*(?:async\s+)?\([^)]*\)\s*=>\s*\{") {
            for captures in arrow_regex.captures_iter(source) {
                if let Some(name) = captures.get(1) {
                    let line_num = source[..captures.get(0).unwrap().start()].matches('\n').count() + 1;
                    let params = self.extract_arrow_function_params(&captures[0]);
                    
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Function(FunctionPattern {
                            name: name.as_str().to_string(),
                            parameters: params,
                            return_type: self.infer_return_type(source, name.as_str()),
                        }),
                        location: SourceLocation {
                            file: "unknown".to_string(),
                            line: line_num,
                            column: name.start(),
                        },
                        context: Context {
                            function_name: Some(name.as_str().to_string()),
                            class_name: self.extract_containing_class(source, name.start()),
                            module_name: None,
                        },
                        confidence: 0.9,
                    });
                }
            }
        }

        // Detect class methods: methodName(params) { or async methodName(params) {
        if let Ok(method_regex) = Regex::new(r"(?:async\s+)?(\w+)\s*\([^)]*\)\s*\{") {
            for captures in method_regex.captures_iter(source) {
                if let Some(name) = captures.get(1) {
                    // Skip constructors and common keywords
                    if name.as_str() != "constructor" && name.as_str() != "function" && name.as_str() != "if" && name.as_str() != "for" && name.as_str() != "while" {
                        let line_num = source[..captures.get(0).unwrap().start()].matches('\n').count() + 1;
                        let params = self.extract_method_params(&captures[0]);
                        
                        patterns.push(TestablePattern {
                            id: uuid::Uuid::new_v4().to_string(),
                            pattern_type: PatternType::Function(FunctionPattern {
                                name: name.as_str().to_string(),
                                parameters: params,
                                return_type: self.infer_return_type(source, name.as_str()),
                            }),
                            location: SourceLocation {
                                file: "unknown".to_string(),
                                line: line_num,
                                column: name.start(),
                            },
                            context: Context {
                                function_name: Some(name.as_str().to_string()),
                                class_name: self.extract_containing_class(source, name.start()),
                                module_name: None,
                            },
                            confidence: 0.85,
                        });
                    }
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

    fn infer_return_type(&self, source: &str, function_name: &str) -> Option<String> {
        // Look for return statements in the function
        if let Ok(function_regex) = Regex::new(&format!(r"(?:function\s+{}|{}\s*=.*?)\s*\([^)]*\)\s*\{{([^}}]*)}}", function_name, function_name)) {
            if let Some(captures) = function_regex.captures(source) {
                if let Some(body) = captures.get(1) {
                    let body_str = body.as_str();
                    if body_str.contains("return true") || body_str.contains("return false") {
                        return Some("boolean".to_string());
                    } else if body_str.contains("return \"") || body_str.contains("return '") || body_str.contains("return `") {
                        return Some("string".to_string());
                    } else if body_str.contains("return [") {
                        return Some("array".to_string());
                    } else if body_str.contains("return {") {
                        return Some("object".to_string());
                    } else if body_str.contains(r"return \d") {
                        return Some("number".to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_containing_class(&self, source: &str, position: usize) -> Option<String> {
        // Find if this function is inside a class
        let before_position = &source[..position];
        if let Ok(class_regex) = Regex::new(r"class\s+(\w+)") {
            for captures in class_regex.captures_iter(before_position) {
                if let Some(class_name) = captures.get(1) {
                    return Some(class_name.as_str().to_string());
                }
            }
        }
        None
    }

    fn extract_arrow_function_params(&self, function_def: &str) -> Vec<String> {
        if let Ok(param_regex) = Regex::new(r"\(([^)]*)\)\s*=>") {
            if let Some(captures) = param_regex.captures(function_def) {
                if let Some(params_str) = captures.get(1) {
                    let params = params_str.as_str().trim();
                    if params.is_empty() {
                        return vec![];
                    }
                    return params.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect();
                }
            }
        }
        vec![]
    }

    fn extract_method_params(&self, method_def: &str) -> Vec<String> {
        if let Ok(param_regex) = Regex::new(r"\(([^)]*)\)") {
            if let Some(captures) = param_regex.captures(method_def) {
                if let Some(params_str) = captures.get(1) {
                    let params = params_str.as_str().trim();
                    if params.is_empty() {
                        return vec![];
                    }
                    return params.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect();
                }
            }
        }
        vec![]
    }

    fn generate_sample_inputs(&self, func: &FunctionPattern) -> serde_json::Value {
        let mut inputs = serde_json::Map::new();
        for (i, param) in func.parameters.iter().enumerate() {
            let value = self.get_sample_value_for_param(param, i);
            inputs.insert(param.clone(), value);
        }
        serde_json::Value::Object(inputs)
    }

    fn get_sample_value_for_param(&self, param: &str, index: usize) -> serde_json::Value {
        let param_lower = param.to_lowercase();
        match param_lower.as_str() {
            p if p.contains("email") => serde_json::json!("test@example.com"),
            p if p.contains("id") => serde_json::json!(index + 1),
            p if p.contains("name") => serde_json::json!(format!("TestName{}", index + 1)),
            p if p.contains("count") || p.contains("number") || p.contains("age") => serde_json::json!(42),
            p if p.contains("price") || p.contains("amount") => serde_json::json!(19.99),
            p if p.contains("bool") || p.contains("flag") || p.contains("enabled") => serde_json::json!(true),
            p if p.contains("array") || p.contains("list") => serde_json::json!([1, 2, 3]),
            p if p.contains("url") || p.contains("link") => serde_json::json!("https://example.com"),
            p if p.contains("date") => serde_json::json!("2023-12-01"),
            p if p.contains("phone") => serde_json::json!("+1-234-567-8900"),
            p if p.contains("a") || p.contains("b") || p.contains("x") || p.contains("y") => serde_json::json!(5),
            _ => serde_json::json!(format!("testValue{}", index + 1)),
        }
    }

    fn generate_expected_output(&self, func: &FunctionPattern) -> serde_json::Value {
        match &func.return_type {
            Some(return_type) => match return_type.as_str() {
                "boolean" => serde_json::json!(true),
                "string" => serde_json::json!("expected_result"),
                "number" => serde_json::json!(42),
                "array" => serde_json::json!([]),
                "object" => serde_json::json!({}),
                _ => serde_json::json!(null),
            },
            None => {
                // Infer based on function name
                let name_lower = func.name.to_lowercase();
                if name_lower.contains("validate") || name_lower.contains("check") || name_lower.contains("is") {
                    serde_json::json!(true)
                } else if name_lower.contains("calculate") || name_lower.contains("add") || name_lower.contains("sum") {
                    serde_json::json!(42)
                } else if name_lower.contains("format") || name_lower.contains("get") || name_lower.contains("to") {
                    serde_json::json!("result")
                } else {
                    serde_json::json!(null)
                }
            }
        }
    }

    fn generate_basic_test_body(&self, func: &FunctionPattern) -> String {
        let mut test_body = String::new();
        
        if func.parameters.is_empty() {
            // No parameters
            test_body.push_str(&format!("    const result = {}();\n", func.name));
            test_body.push_str("    expect(result).toBeDefined();\n");
            
            // Add return type specific assertions
            match &func.return_type {
                Some(return_type) => {
                    match return_type.as_str() {
                        "boolean" => {
                            test_body.push_str("    expect(typeof result).toBe('boolean');\n");
                            test_body.push_str(&format!("    expect({}()).toBe(true);\n", func.name));
                        },
                        "string" => {
                            test_body.push_str("    expect(typeof result).toBe('string');\n");
                            test_body.push_str("    expect(result.length).toBeGreaterThanOrEqual(0);\n");
                        },
                        "number" => {
                            test_body.push_str("    expect(typeof result).toBe('number');\n");
                            test_body.push_str("    expect(result).not.toBeNaN();\n");
                        },
                        "array" => {
                            test_body.push_str("    expect(Array.isArray(result)).toBe(true);\n");
                        },
                        "object" => {
                            test_body.push_str("    expect(typeof result).toBe('object');\n");
                            test_body.push_str("    expect(result).not.toBeNull();\n");
                        },
                        _ => test_body.push_str("    expect(result).toBeDefined();\n"),
                    }
                },
                None => {
                    // Infer based on function name
                    let name_lower = func.name.to_lowercase();
                    if name_lower.contains("validate") || name_lower.contains("check") || name_lower.contains("is") {
                        test_body.push_str("    expect(typeof result).toBe('boolean');\n");
                    } else if name_lower.contains("calculate") || name_lower.contains("add") || name_lower.contains("sum") {
                        test_body.push_str("    expect(typeof result).toBe('number');\n");
                        test_body.push_str("    expect(result).not.toBeNaN();\n");
                    } else {
                        test_body.push_str("    expect(result).toBeDefined();\n");
                    }
                }
            }
        } else {
            // With parameters - generate specific test cases
            let sample_params = self.generate_sample_parameters(func);
            test_body.push_str(&format!("    const result = {}({});\n", func.name, sample_params));
            test_body.push_str("    expect(result).toBeDefined();\n");
            
            // Add specific assertions based on function name patterns
            let name_lower = func.name.to_lowercase();
            if name_lower.contains("add") || name_lower.contains("sum") {
                test_body.push_str("    expect(typeof result).toBe('number');\n");
                test_body.push_str("    expect(result).toBeGreaterThan(0);\n");
            } else if name_lower.contains("multiply") {
                test_body.push_str("    expect(typeof result).toBe('number');\n");
            } else if name_lower.contains("validate") {
                test_body.push_str("    expect(typeof result).toBe('boolean');\n");
            }
        }
        
        test_body
    }

    fn generate_boundary_test_body(&self, func: &FunctionPattern) -> String {
        let mut test_body = String::new();
        
        if func.parameters.is_empty() {
            test_body.push_str(&format!("    // Test {} with no parameters\n", func.name));
            test_body.push_str(&format!("    expect(() => {}()).not.toThrow();\n", func.name));
        } else {
            test_body.push_str("    // Test boundary conditions\n");
            
            // Test with zero values
            let zero_params = func.parameters.iter().map(|_| "0").collect::<Vec<_>>().join(", ");
            test_body.push_str(&format!("    expect(() => {}({})).not.toThrow();\n", func.name, zero_params));
            
            // Test with empty strings if applicable
            let empty_params = func.parameters.iter().map(|p| {
                if p.to_lowercase().contains("string") || p.to_lowercase().contains("name") || p.to_lowercase().contains("email") {
                    "\"\""
                } else {
                    "0"
                }
            }).collect::<Vec<_>>().join(", ");
            test_body.push_str(&format!("    expect(() => {}({})).not.toThrow();\n", func.name, empty_params));
            
            // Test with large numbers if numeric function
            let name_lower = func.name.to_lowercase();
            if name_lower.contains("add") || name_lower.contains("multiply") || name_lower.contains("calculate") {
                let large_params = func.parameters.iter().map(|_| "Number.MAX_SAFE_INTEGER").collect::<Vec<_>>().join(", ");
                test_body.push_str(&format!("    expect(() => {}({})).not.toThrow();\n", func.name, large_params));
            }
        }
        
        test_body
    }

    fn generate_error_test_body(&self, func: &FunctionPattern) -> String {
        let mut test_body = String::new();
        
        if !func.parameters.is_empty() {
            test_body.push_str("    // Test error handling with invalid inputs\n");
            
            // Test with null values
            let null_params = func.parameters.iter().map(|_| "null").collect::<Vec<_>>().join(", ");
            test_body.push_str(&format!("    expect(() => {}({})).toThrow();\n", func.name, null_params));
            
            // Test with undefined values
            let undefined_params = func.parameters.iter().map(|_| "undefined").collect::<Vec<_>>().join(", ");
            test_body.push_str(&format!("    expect(() => {}({})).toThrow();\n", func.name, undefined_params));
            
            // Test with wrong types
            let wrong_type_params = func.parameters.iter().map(|p| {
                let p_lower = p.to_lowercase();
                if p_lower.contains("number") || p_lower.contains("count") {
                    "\"not_a_number\""
                } else if p_lower.contains("string") || p_lower.contains("name") {
                    "123"
                } else {
                    "\"invalid_input\""
                }
            }).collect::<Vec<_>>().join(", ");
            test_body.push_str(&format!("    expect(() => {}({})).toThrow();\n", func.name, wrong_type_params));
        } else {
            test_body.push_str("    // Test function execution doesn't throw\n");
            test_body.push_str(&format!("    expect(() => {}()).not.toThrow();\n", func.name));
        }
        
        test_body
    }

    fn generate_type_validation_test_body(&self, func: &FunctionPattern) -> String {
        let mut test_body = String::new();
        
        test_body.push_str("    // Test input type validation\n");
        
        // Test each parameter type
        for (i, param) in func.parameters.iter().enumerate() {
            let param_lower = param.to_lowercase();
            
            if param_lower.contains("number") || param_lower.contains("count") || param_lower.contains("age") {
                test_body.push_str(&format!("    expect(() => {}()).toThrow(); // Invalid number type\n", 
                    self.create_invalid_call(func, i, "\"not_a_number\"")));
            } else if param_lower.contains("string") || param_lower.contains("name") {
                test_body.push_str(&format!("    expect(() => {}()).toThrow(); // Invalid string type\n", 
                    self.create_invalid_call(func, i, "123")));
            } else if param_lower.contains("bool") || param_lower.contains("flag") {
                test_body.push_str(&format!("    expect(() => {}()).toThrow(); // Invalid boolean type\n", 
                    self.create_invalid_call(func, i, "\"not_boolean\"")));
            }
        }
        
        test_body
    }

    fn generate_async_test_body(&self, func: &FunctionPattern) -> String {
        let mut test_body = String::new();
        
        test_body.push_str("    // Test async function execution\n");
        
        if func.parameters.is_empty() {
            test_body.push_str(&format!("    await expect({}()).resolves.toBeDefined();\n", func.name));
            test_body.push_str(&format!("    await expect({}()).resolves.not.toThrow();\n", func.name));
        } else {
            let sample_params = self.generate_sample_parameters(func);
            test_body.push_str(&format!("    await expect({}({})).resolves.toBeDefined();\n", func.name, sample_params));
            test_body.push_str(&format!("    await expect({}({})).resolves.not.toThrow();\n", func.name, sample_params));
        }
        
        test_body
    }

    fn generate_sample_parameters(&self, func: &FunctionPattern) -> String {
        func.parameters.iter().enumerate().map(|(i, param)| {
            let param_lower = param.to_lowercase();
            match param_lower.as_str() {
                p if p.contains("email") => "\"test@example.com\"".to_string(),
                p if p.contains("name") => format!("\"TestName{}\"", i + 1),
                p if p.contains("count") || p.contains("number") || p.contains("age") => "42".to_string(),
                p if p.contains("price") || p.contains("amount") => "19.99".to_string(),
                p if p.contains("bool") || p.contains("flag") => "true".to_string(),
                p if p.contains("array") || p.contains("list") => "[1, 2, 3]".to_string(),
                p if p.contains("a") || p.contains("b") || p.contains("x") || p.contains("y") => "5".to_string(),
                _ => format!("\"testValue{}\"", i + 1),
            }
        }).collect::<Vec<_>>().join(", ")
    }

    fn create_invalid_call(&self, func: &FunctionPattern, invalid_param_index: usize, invalid_value: &str) -> String {
        let params: Vec<String> = func.parameters.iter().enumerate().map(|(i, _)| {
            if i == invalid_param_index {
                invalid_value.to_string()
            } else {
                "null".to_string()
            }
        }).collect();
        
        format!("{}({})", func.name, params.join(", "))
    }

    fn is_async_function(&self, source: &str, function_name: &str) -> bool {
        source.contains(&format!("async function {}", function_name)) ||
        source.contains(&format!("async {}", function_name)) ||
        source.contains(&format!("{} = async", function_name)) ||
        source.contains("await ") && source.contains(function_name)
    }
}

#[async_trait]
impl TestGenerator for JavaScriptAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(self.detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        self.generate_comprehensive_tests(patterns, "").await
    }

    async fn generate_comprehensive_tests(&self, patterns: Vec<TestablePattern>, source: &str) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::FormValidation(field) => {
                    if field.field_type == FieldType::Email {
                        test_cases.extend(self.generate_email_validation_tests(field));
                    }
                }
                PatternType::Function(func) => {
                    test_cases.extend(self.generate_function_tests(func, source));
                }
                _ => {}
            }
        }

        let mut test_suite = TestSuite {
            name: "Generated JavaScript Tests".to_string(),
            language: "javascript".to_string(),
            framework: "jest".to_string(),
            test_cases,
            imports: vec![
                "const { expect } = require('@jest/globals');".to_string(),
                "const { describe, it, beforeEach, afterEach } = require('@jest/globals');".to_string(),
            ],
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
        crate::core::CoverageStandards::get_coverage_target("javascript")
    }

    fn generate_test_code(&self, test_suite: &TestSuite) -> Result<String> {
        let mut code = String::new();
        
        for import in &test_suite.imports {
            code.push_str(&format!("{}
", import));
        }
        code.push_str("\n");
        
        code.push_str(&format!("describe('{}', () => {{\n", test_suite.name));
        
        for test_case in &test_suite.test_cases {
            code.push_str(&format!("  it('{}', () => {{\n", test_case.name));
            code.push_str(&format!("    // {}\n", test_case.description));
            code.push_str(&test_case.test_body);
            code.push_str("  });\n\n");
        }
        
        code.push_str("});\n");
        Ok(code)
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
                        test_body: "        // TODO: Implement integration test logic".to_string(),
                        assertions: vec![],
                        test_category: crate::core::TestCategory::Integration,
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
                        test_body: "        // TODO: Implement component integration test logic".to_string(),
                        assertions: vec![],
                        test_category: crate::core::TestCategory::Integration,
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
                        test_body: "        // TODO: Implement database integration test logic".to_string(),
                        assertions: vec![],
                        test_category: crate::core::TestCategory::Integration,
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
            coverage_target: self.get_coverage_target(),
            test_code: None,
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