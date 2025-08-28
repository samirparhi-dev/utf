use crate::core::{TestablePattern, PatternType, TestCase, TestSuite, TestGenerator, SourceLocation, Context, FunctionPattern};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct JavaAdapter;

impl JavaAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_patterns(content: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();

        // Detect Java methods (public/private/protected static? returnType methodName(...))
        let method_regex = Regex::new(r"(?m)^\s*(?:public|private|protected)?\s*(?:static\s+)?(?:final\s+)?(\w+(?:<[^>]*>)?)\s+(\w+)\s*\([^)]*\)\s*(?:throws\s+[^{]*)?(?:\s*\{|;)").unwrap();
        for cap in method_regex.captures_iter(content) {
            if let (Some(return_type), Some(method_name)) = (cap.get(1), cap.get(2)) {
                // Skip constructors (method name matches class name pattern)
                if !Self::is_constructor_pattern(method_name.as_str(), content) {
                    let line_num = content[..cap.get(0).unwrap().start()].matches('\n').count() + 1;
                    let parameters = Self::extract_method_parameters(&cap[0]);
                    
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Function(FunctionPattern {
                            name: method_name.as_str().to_string(),
                            parameters,
                            return_type: Some(return_type.as_str().to_string()),
                        }),
                        location: SourceLocation {
                            file: "".to_string(),
                            line: line_num,
                            column: cap.get(0).unwrap().start() + 1,
                        },
                        context: Context {
                            function_name: Some(method_name.as_str().to_string()),
                            class_name: Self::extract_class_name(content),
                            module_name: None,
                        },
                        confidence: 0.9,
                    });
                }
            }
        }

        patterns
    }

    fn is_constructor_pattern(name: &str, content: &str) -> bool {
        // Check if there's a class with the same name
        let class_regex = Regex::new(&format!(r"class\s+{}\s*(?:\{{|extends|implements)", name)).unwrap();
        class_regex.is_match(content)
    }

    fn extract_method_parameters(method_def: &str) -> Vec<String> {
        let param_regex = Regex::new(r"\(([^)]*)\)").unwrap();
        if let Some(cap) = param_regex.captures(method_def) {
            let params_str = cap.get(1).unwrap().as_str().trim();
            if params_str.is_empty() {
                return vec![];
            }
            
            params_str
                .split(',')
                .map(|p| {
                    // Handle Java parameter syntax: Type name
                    let parts: Vec<&str> = p.trim().split_whitespace().collect();
                    if parts.len() >= 2 {
                        parts[parts.len() - 1].to_string() // parameter name is last
                    } else if parts.len() == 1 {
                        format!("param_{}", parts[0].to_lowercase()) // just type, generate name
                    } else {
                        "param".to_string()
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn extract_class_name(content: &str) -> Option<String> {
        let class_regex = Regex::new(r"class\s+(\w+)").unwrap();
        if let Some(cap) = class_regex.captures(content) {
            Some(cap.get(1).unwrap().as_str().to_string())
        } else {
            None
        }
    }
}

#[async_trait]
impl TestGenerator for JavaAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(Self::detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        let class_name = patterns.iter()
            .find_map(|p| p.context.class_name.as_ref())
            .unwrap_or(&"TestClass".to_string())
            .clone();

        for pattern in &patterns {
            match &pattern.pattern_type {
                PatternType::Function(func) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("test{}", func.name),
                        description: format!("Test for method {}", func.name),
                        input: serde_json::json!({
                            "method": func.name,
                            "parameters": func.parameters
                        }),
                        expected_output: serde_json::json!({
                            "type": func.return_type.as_ref().unwrap_or(&"void".to_string())
                        }),
                    });
                }
                _ => {} // Skip other pattern types for now
            }
        }

        Ok(TestSuite {
            name: format!("{}Test", class_name),
            language: "java".to_string(),
            framework: "junit".to_string(),
            test_cases,
            imports: vec![
                "import org.junit.*;".to_string(),
                "import static org.junit.Assert.*;".to_string(),
            ],
        })
    }

    fn get_language(&self) -> &str {
        "java"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["junit"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_method() {
        let content = r#"
public class Calculator {
    public int add(int a, int b) {
        return a + b;
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "add");
            assert_eq!(func.parameters, vec!["a", "b"]);
            assert_eq!(func.return_type.as_ref().unwrap(), "int");
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_method_no_params() {
        let content = r#"
public class Helper {
    public String getCurrentTime() {
        return new Date().toString();
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "getCurrentTime");
            assert!(func.parameters.is_empty());
            assert_eq!(func.return_type.as_ref().unwrap(), "String");
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_extract_method_parameters() {
        let method_def = "public int calculate(int value, String name)";
        let params = JavaAdapter::extract_method_parameters(method_def);
        assert_eq!(params, vec!["value", "name"]);
    }

    #[test]
    fn test_extract_method_parameters_empty() {
        let method_def = "public void doSomething()";
        let params = JavaAdapter::extract_method_parameters(method_def);
        assert!(params.is_empty());
    }

    #[test]
    fn test_is_constructor_pattern() {
        let content = r#"
public class Person {
    public Person(String name) {
        this.name = name;
    }
}
"#;
        assert!(JavaAdapter::is_constructor_pattern("Person", content));
        assert!(!JavaAdapter::is_constructor_pattern("getName", content));
    }

    #[test]
    fn test_extract_class_name() {
        let content = r#"
public class Calculator {
    // class content
}
"#;
        let class_name = JavaAdapter::extract_class_name(content);
        assert_eq!(class_name, Some("Calculator".to_string()));
    }

    #[tokio::test]
    async fn test_generate_tests_for_method() {
        let adapter = JavaAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "calculate".to_string(),
                parameters: vec!["value".to_string()],
                return_type: Some("int".to_string()),
            }),
            location: SourceLocation {
                file: "Calculator.java".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("calculate".to_string()),
                class_name: Some("Calculator".to_string()),
                module_name: None,
            },
            confidence: 0.9,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.language, "java");
        assert_eq!(test_suite.framework, "junit");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert_eq!(test_suite.test_cases[0].name, "testcalculate");
        assert_eq!(test_suite.name, "CalculatorTest");
    }

    #[tokio::test]
    async fn test_get_language() {
        let adapter = JavaAdapter::new();
        assert_eq!(adapter.get_language(), "java");
    }

    #[tokio::test]
    async fn test_get_supported_frameworks() {
        let adapter = JavaAdapter::new();
        assert_eq!(adapter.get_supported_frameworks(), vec!["junit"]);
    }
}