use unified_test_framework::templates::{TemplateEngine, TestPattern};
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize the template engine
    let template_engine = TemplateEngine::new()?;
    
    println!("Available templates: {:?}", template_engine.get_available_templates());
    
    // Create a sample function pattern for JavaScript
    let js_function = TestPattern::Function {
        name: "validateEmail".to_string(),
        params: vec!["email".to_string()],
        return_type: Some("boolean".to_string()),
    };
    
    // Create a JavaScript adapter instance (normally we'd use the actual adapter)
    let js_adapter = unified_test_framework::JavaScriptAdapter::new();
    
    // Generate test using template
    match js_adapter.generate_test_with_template(&js_function, &template_engine) {
        Ok(test_code) => {
            println!("\n=== Generated JavaScript Test ===");
            println!("{}", test_code);
        }
        Err(e) => {
            println!("Error generating test: {}", e);
        }
    }
    
    // Test async function pattern
    let async_function = TestPattern::AsyncFunction {
        name: "fetchUserData".to_string(),
        params: vec!["userId".to_string()],
        return_type: Some("Promise<User>".to_string()),
    };
    
    match js_adapter.generate_test_with_template(&async_function, &template_engine) {
        Ok(test_code) => {
            println!("\n=== Generated Async JavaScript Test ===");
            println!("{}", test_code);
        }
        Err(e) => {
            println!("Error generating async test: {}", e);
        }
    }
    
    Ok(())
}