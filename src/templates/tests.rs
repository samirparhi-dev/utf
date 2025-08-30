#[cfg(test)]
mod tests {
    use super::*;
    use crate::templates::{TemplateEngine, TestPattern};
    
    #[test]
    fn test_template_engine_creation() {
        let engine = TemplateEngine::new();
        assert!(engine.is_ok());
        
        if let Ok(engine) = engine {
            let templates = engine.get_available_templates();
            assert!(!templates.is_empty());
            
            // Check that we have the expected templates
            assert!(templates.iter().any(|t| t.contains("jest/function_test")));
            assert!(templates.iter().any(|t| t.contains("pytest/function_test")));
            assert!(templates.iter().any(|t| t.contains("cargo/function_test")));
        }
    }
    
    #[test]
    fn test_javascript_function_template() {
        let engine = TemplateEngine::new().unwrap();
        
        let pattern = TestPattern::Function {
            name: "validateEmail".to_string(),
            params: vec!["email".to_string()],
            return_type: Some("boolean".to_string()),
        };
        
        let template_data = pattern.generate_template_data("validation");
        let result = engine.render_test("jest/function_test", &template_data);
        
        assert!(result.is_ok());
        
        if let Ok(test_code) = result {
            assert!(test_code.contains("validateEmail"));
            assert!(test_code.contains("describe"));
            assert!(test_code.contains("test"));
            assert!(test_code.contains("expect"));
        }
    }
    
    #[test]
    fn test_python_function_template() {
        let engine = TemplateEngine::new().unwrap();
        
        let pattern = TestPattern::Function {
            name: "calculate_sum".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            return_type: Some("int".to_string()),
        };
        
        let template_data = pattern.generate_template_data("numeric");
        let result = engine.render_test("pytest/function_test", &template_data);
        
        assert!(result.is_ok());
        
        if let Ok(test_code) = result {
            assert!(test_code.contains("calculate_sum"));
            assert!(test_code.contains("class Test"));
            assert!(test_code.contains("def test_"));
            assert!(test_code.contains("assert"));
        }
    }
    
    #[test]
    fn test_rust_function_template() {
        let engine = TemplateEngine::new().unwrap();
        
        let pattern = TestPattern::Function {
            name: "add_numbers".to_string(),
            params: vec!["x".to_string(), "y".to_string()],
            return_type: Some("i32".to_string()),
        };
        
        let template_data = pattern.generate_template_data("numeric");
        let result = engine.render_test("cargo/function_test", &template_data);
        
        assert!(result.is_ok());
        
        if let Ok(test_code) = result {
            assert!(test_code.contains("add_numbers"));
            assert!(test_code.contains("#[cfg(test)]"));
            assert!(test_code.contains("#[test]"));
            assert!(test_code.contains("assert"));
        }
    }
}