use serde_json::Value;
use serde::{Serialize, Deserialize};
use anyhow::Result;

pub mod javascript;
pub mod python;
pub mod rust;
pub mod advanced_patterns;
pub mod askama_engine;

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
    askama_engine: askama_engine::AskamaTemplateEngine,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let askama_engine = askama_engine::AskamaTemplateEngine::new();
        Ok(Self { askama_engine })
    }
    
    pub fn render_test(&self, template_name: &str, data: &TestTemplateData) -> Result<String> {
        self.askama_engine.render_test(template_name, data)
    }
    
    pub fn render_test_suite(&self, _language: &str, _framework: &str, _tests: Vec<TestTemplateData>) -> Result<String> {
        // Test suite rendering would be implemented based on requirements
        // For now, return a simple concatenation message
        Ok("Test suite rendering not yet implemented for Askama".to_string())
    }
    
    pub fn get_available_templates(&self) -> Vec<String> {
        vec![
            "jest/function_test".to_string(),
            "jest/async_test".to_string(), 
            "jest/class_test".to_string(),
            "pytest/function_test".to_string(),
            "pytest/async_test".to_string(),
            "pytest/class_test".to_string(),
            "cargo/function_test".to_string(),
            "cargo/async_test".to_string(),
            "cargo/struct_test".to_string(),
            "go-testing/function_test".to_string(),
            "go-testing/struct_test".to_string(),
            "go-testing/interface_test".to_string(),
            "go-testing/benchmark_test".to_string(),
            "junit/method_test".to_string(),
            "junit/class_test".to_string(),
            "junit/integration_test".to_string(),
            "junit/mock_test".to_string(),
        ]
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