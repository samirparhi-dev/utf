use tera::{Tera, Context};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use anyhow::Result;

pub mod javascript;
pub mod python;
pub mod rust;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTemplateData {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: Option<String>,
    pub teardown_code: Option<String>,
}

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::default();
        
        // Register JavaScript templates
        tera.add_raw_template("jest/function_test", javascript::JEST_FUNCTION_TEST_TEMPLATE)?;
        tera.add_raw_template("jest/class_test", javascript::JEST_CLASS_TEST_TEMPLATE)?;
        tera.add_raw_template("jest/async_test", javascript::JEST_ASYNC_TEST_TEMPLATE)?;
        
        // Register Python templates
        tera.add_raw_template("pytest/function_test", python::PYTEST_FUNCTION_TEST_TEMPLATE)?;
        tera.add_raw_template("pytest/class_test", python::PYTEST_CLASS_TEST_TEMPLATE)?;
        tera.add_raw_template("pytest/async_test", python::PYTEST_ASYNC_TEST_TEMPLATE)?;
        
        // Register Rust templates
        tera.add_raw_template("cargo/function_test", rust::CARGO_FUNCTION_TEST_TEMPLATE)?;
        tera.add_raw_template("cargo/struct_test", rust::CARGO_STRUCT_TEST_TEMPLATE)?;
        tera.add_raw_template("cargo/async_test", rust::CARGO_ASYNC_TEST_TEMPLATE)?;
        
        Ok(Self { tera })
    }
    
    pub fn render_test(&self, template_name: &str, data: &TestTemplateData) -> Result<String> {
        let mut context = Context::new();
        
        context.insert("function_name", &data.function_name);
        context.insert("test_name", &data.test_name);
        context.insert("description", &data.description);
        context.insert("inputs", &data.inputs);
        context.insert("expected_outputs", &data.expected_outputs);
        context.insert("test_category", &data.test_category);
        context.insert("imports", &data.imports);
        context.insert("setup_code", &data.setup_code);
        context.insert("teardown_code", &data.teardown_code);
        
        let rendered = self.tera.render(template_name, &context)?;
        Ok(rendered)
    }
    
    pub fn render_test_suite(&self, language: &str, framework: &str, tests: Vec<TestTemplateData>) -> Result<String> {
        let template_name = format!("{}/{}_test_suite", framework, language);
        let mut context = Context::new();
        context.insert("tests", &tests);
        
        let rendered = self.tera.render(&template_name, &context)?;
        Ok(rendered)
    }
    
    pub fn get_available_templates(&self) -> Vec<String> {
        self.tera.get_template_names().map(|s| s.to_string()).collect()
    }
}

#[derive(Debug, Clone)]
pub enum TestPattern {
    Function { name: String, params: Vec<String>, return_type: Option<String> },
    Class { name: String, methods: Vec<String> },
    AsyncFunction { name: String, params: Vec<String>, return_type: Option<String> },
    ApiEndpoint { path: String, method: String, params: Vec<String> },
}

impl TestPattern {
    pub fn generate_template_data(&self, test_category: &str) -> TestTemplateData {
        match self {
            TestPattern::Function { name, params, return_type } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}", name.to_lowercase()),
                    description: format!("Test {} function", name),
                    inputs: self.generate_sample_inputs(params),
                    expected_outputs: self.generate_sample_outputs(return_type),
                    test_category: test_category.to_string(),
                    imports: vec![],
                    setup_code: None,
                    teardown_code: None,
                }
            }
            TestPattern::Class { name, methods: _methods } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}_class", name.to_lowercase()),
                    description: format!("Test {} class", name),
                    inputs: vec![],
                    expected_outputs: vec![],
                    test_category: test_category.to_string(),
                    imports: vec![],
                    setup_code: Some(format!("instance = {}()", name)),
                    teardown_code: None,
                }
            }
            TestPattern::AsyncFunction { name, params, return_type } => {
                TestTemplateData {
                    function_name: name.clone(),
                    test_name: format!("test_{}_async", name.to_lowercase()),
                    description: format!("Test async {} function", name),
                    inputs: self.generate_sample_inputs(params),
                    expected_outputs: self.generate_sample_outputs(return_type),
                    test_category: test_category.to_string(),
                    imports: vec![],
                    setup_code: None,
                    teardown_code: None,
                }
            }
            TestPattern::ApiEndpoint { path, method, params } => {
                TestTemplateData {
                    function_name: format!("{}_{}", method.to_lowercase(), path.replace("/", "_")),
                    test_name: format!("test_{}_{}", method.to_lowercase(), path.replace("/", "_")),
                    description: format!("Test {} {}", method, path),
                    inputs: self.generate_sample_inputs(params),
                    expected_outputs: vec![serde_json::json!({"status": 200})],
                    test_category: test_category.to_string(),
                    imports: vec![],
                    setup_code: None,
                    teardown_code: None,
                }
            }
        }
    }
    
    fn generate_sample_inputs(&self, params: &[String]) -> Vec<Value> {
        params.iter().enumerate().map(|(i, param)| {
            match param.as_str() {
                p if p.contains("email") => serde_json::json!("test@example.com"),
                p if p.contains("id") || p.contains("count") => serde_json::json!(i + 1),
                p if p.contains("name") => serde_json::json!(format!("test_name_{}", i)),
                p if p.contains("bool") => serde_json::json!(true),
                _ => serde_json::json!(format!("test_value_{}", i)),
            }
        }).collect()
    }
    
    fn generate_sample_outputs(&self, return_type: &Option<String>) -> Vec<Value> {
        match return_type {
            Some(t) if t.contains("bool") => vec![serde_json::json!(true), serde_json::json!(false)],
            Some(t) if t.contains("int") || t.contains("number") => vec![serde_json::json!(42)],
            Some(t) if t.contains("string") => vec![serde_json::json!("expected_result")],
            Some(t) if t.contains("array") || t.contains("list") => vec![serde_json::json!([1, 2, 3])],
            _ => vec![serde_json::json!(null)],
        }
    }
}