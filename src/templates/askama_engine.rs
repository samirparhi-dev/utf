use askama::Template;
use serde_json::Value;
use anyhow::Result;

// Template structs for each test type with Askama derive macro
#[derive(Template)]
#[template(path = "jest/function_test.html")]
pub struct JestFunctionTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "jest/async_test.html")]
pub struct JestAsyncTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "jest/class_test.html")]
pub struct JestClassTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "pytest/function_test.html")]
pub struct PytestFunctionTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "pytest/async_test.html")]
pub struct PytestAsyncTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "pytest/class_test.html")]
pub struct PytestClassTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "cargo/function_test.html")]
pub struct CargoFunctionTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "cargo/async_test.html")]
pub struct CargoAsyncTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "cargo/struct_test.html")]
pub struct CargoStructTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "go-testing/function_test.html")]
pub struct GoFunctionTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "go-testing/struct_test.html")]
pub struct GoStructTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "go-testing/interface_test.html")]
pub struct GoInterfaceTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "go-testing/benchmark_test.html")]
pub struct GoBenchmarkTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "junit/method_test.html")]
pub struct JunitMethodTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
}

#[derive(Template)]
#[template(path = "junit/class_test.html")]
pub struct JunitClassTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "junit/integration_test.html")]
pub struct JunitIntegrationTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

#[derive(Template)]
#[template(path = "junit/mock_test.html")]
pub struct JunitMockTemplate {
    pub function_name: String,
    pub test_name: String,
    pub description: String,
    pub inputs: Vec<Value>,
    pub expected_outputs: Vec<Value>,
    pub test_category: String,
    pub imports: Vec<String>,
    pub setup_code: String,
    pub teardown_code: String,
    pub methods: Vec<String>,
}

// Template engine that uses Askama
pub struct AskamaTemplateEngine;

impl AskamaTemplateEngine {
    pub fn new() -> Self {
        Self
    }
    
    pub fn render_jest_function_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = JestFunctionTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_jest_async_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = JestAsyncTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_jest_class_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = JestClassTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_pytest_function_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = PytestFunctionTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_pytest_async_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = PytestAsyncTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_pytest_class_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = PytestClassTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_cargo_function_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = CargoFunctionTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_cargo_async_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = CargoAsyncTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_cargo_struct_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = CargoStructTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_go_function_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = GoFunctionTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_go_struct_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = GoStructTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_go_interface_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = GoInterfaceTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_go_benchmark_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = GoBenchmarkTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_junit_method_test(&self, data: &crate::TestTemplateData) -> Result<String> {
        let template = JunitMethodTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
        };
        Ok(template.render()?)
    }
    
    pub fn render_junit_class_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = JunitClassTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_junit_integration_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = JunitIntegrationTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_junit_mock_test(&self, data: &crate::TestTemplateData, methods: Vec<String>) -> Result<String> {
        let template = JunitMockTemplate {
            function_name: data.function_name.clone(),
            test_name: data.test_name.clone(),
            description: data.description.clone(),
            inputs: data.inputs.clone(),
            expected_outputs: data.expected_outputs.clone(),
            test_category: data.test_category.clone(),
            imports: data.imports.clone(),
            setup_code: data.setup_code.clone().unwrap_or_default(),
            teardown_code: data.teardown_code.clone().unwrap_or_default(),
            methods,
        };
        Ok(template.render()?)
    }
    
    pub fn render_test(&self, template_name: &str, data: &crate::TestTemplateData) -> Result<String> {
        match template_name {
            "jest/function_test" => self.render_jest_function_test(data),
            "jest/async_test" => self.render_jest_async_test(data),
            "jest/class_test" => self.render_jest_class_test(data, vec![]), // Default empty methods
            "pytest/function_test" => self.render_pytest_function_test(data),
            "pytest/async_test" => self.render_pytest_async_test(data),
            "pytest/class_test" => self.render_pytest_class_test(data, vec![]), // Default empty methods
            "cargo/function_test" => self.render_cargo_function_test(data),
            "cargo/async_test" => self.render_cargo_async_test(data),
            "cargo/struct_test" => self.render_cargo_struct_test(data, vec![]), // Default empty methods
            "go-testing/function_test" => self.render_go_function_test(data),
            "go-testing/struct_test" => self.render_go_struct_test(data, vec![]), // Default empty methods
            "go-testing/interface_test" => self.render_go_interface_test(data, vec![]), // Default empty methods
            "go-testing/benchmark_test" => self.render_go_benchmark_test(data),
            "junit/method_test" => self.render_junit_method_test(data),
            "junit/class_test" => self.render_junit_class_test(data, vec![]), // Default empty methods
            "junit/integration_test" => self.render_junit_integration_test(data, vec![]), // Default empty methods
            "junit/mock_test" => self.render_junit_mock_test(data, vec![]), // Default empty methods
            _ => Err(anyhow::anyhow!("Unknown template: {}", template_name)),
        }
    }
}