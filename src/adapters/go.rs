use crate::core::{TestablePattern, PatternType, TestCase, TestSuite, TestGenerator, SourceLocation, Context, FunctionPattern};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct GoAdapter;

impl GoAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_patterns(content: &str) -> Vec<TestablePattern> {
        let mut patterns = Vec::new();

        // Detect Go functions (func name(...) returnType)
        let func_regex = Regex::new(r"func\s+(\w+)\s*\([^)]*\)(?:\s*[^{]*)?(?:\s*\{|$)").unwrap();
        for cap in func_regex.captures_iter(content) {
            if let Some(func_name) = cap.get(1) {
                let line_num = content[..cap.get(0).unwrap().start()].matches('\n').count() + 1;
                let parameters = Self::extract_function_parameters(&cap[0]);
                
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Function(FunctionPattern {
                        name: func_name.as_str().to_string(),
                        parameters,
                        return_type: Some(Self::extract_return_type(&cap[0])),
                    }),
                    location: SourceLocation {
                        file: "".to_string(),
                        line: line_num,
                        column: cap.get(0).unwrap().start() + 1,
                    },
                    context: Context {
                        function_name: Some(func_name.as_str().to_string()),
                        class_name: None,
                        module_name: None,
                    },
                    confidence: 0.9,
                });
            }
        }

        patterns
    }

    fn extract_function_parameters(func_def: &str) -> Vec<String> {
        let param_regex = Regex::new(r"\(([^)]*)\)").unwrap();
        if let Some(cap) = param_regex.captures(func_def) {
            let params_str = cap.get(1).unwrap().as_str().trim();
            if params_str.is_empty() {
                return vec![];
            }
            
            params_str
                .split(',')
                .map(|p| {
                    // Handle Go parameter syntax: name type or type
                    let parts: Vec<&str> = p.trim().split_whitespace().collect();
                    if parts.len() >= 2 {
                        parts[0].to_string() // parameter name
                    } else if parts.len() == 1 {
                        format!("param_{}", parts[0]) // just type, generate name
                    } else {
                        "param".to_string()
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn extract_return_type(func_def: &str) -> String {
        // Match return type after parameters
        let return_regex = Regex::new(r"\)[^{]*?(\w+)(?:\s*\{|$)").unwrap();
        if let Some(cap) = return_regex.captures(func_def) {
            cap.get(1).unwrap().as_str().to_string()
        } else {
            "void".to_string()
        }
    }
}

#[async_trait]
impl TestGenerator for GoAdapter {
    async fn analyze_code(&self, source: &str, _file_path: &str) -> Result<Vec<TestablePattern>> {
        Ok(Self::detect_patterns(source))
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            match &pattern.pattern_type {
                PatternType::Function(func) => {
                    test_cases.push(TestCase {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: format!("Test{}", func.name),
                        description: format!("Test for function {}", func.name),
                        input: serde_json::json!({
                            "function": func.name,
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
            name: "Go Tests".to_string(),
            language: "go".to_string(),
            framework: "testing".to_string(),
            test_cases,
            imports: vec![
                "import (".to_string(),
                "\t\"testing\"".to_string(),
                ")".to_string(),
            ],
        })
    }

    fn get_language(&self) -> &str {
        "go"
    }

    fn get_supported_frameworks(&self) -> Vec<&str> {
        vec!["testing"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_simple_function() {
        let content = r#"
func Add(a int, b int) int {
    return a + b
}
"#;
        let patterns = GoAdapter::detect_patterns(content);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "Add");
            assert_eq!(func.parameters, vec!["a", "b"]);
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_detect_function_no_params() {
        let content = r#"
func GetCurrentTime() time.Time {
    return time.Now()
}
"#;
        let patterns = GoAdapter::detect_patterns(content);
        assert_eq!(patterns.len(), 1);
        
        if let PatternType::Function(func) = &patterns[0].pattern_type {
            assert_eq!(func.name, "GetCurrentTime");
            assert!(func.parameters.is_empty());
        } else {
            panic!("Expected Function pattern");
        }
    }

    #[test]
    fn test_extract_function_parameters() {
        let func_def = "func Add(a int, b int) int";
        let params = GoAdapter::extract_function_parameters(func_def);
        assert_eq!(params, vec!["a", "b"]);
    }

    #[test]
    fn test_extract_function_parameters_empty() {
        let func_def = "func GetTime() time.Time";
        let params = GoAdapter::extract_function_parameters(func_def);
        assert!(params.is_empty());
    }

    #[tokio::test]
    async fn test_generate_tests_for_function() {
        let adapter = GoAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Function(FunctionPattern {
                name: "Add".to_string(),
                parameters: vec!["a".to_string(), "b".to_string()],
                return_type: Some("int".to_string()),
            }),
            location: SourceLocation {
                file: "test.go".to_string(),
                line: 1,
                column: 1,
            },
            context: Context {
                function_name: Some("Add".to_string()),
                class_name: None,
                module_name: None,
            },
            confidence: 0.9,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.language, "go");
        assert_eq!(test_suite.framework, "testing");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert_eq!(test_suite.test_cases[0].name, "TestAdd");
    }

    #[tokio::test]
    async fn test_get_language() {
        let adapter = GoAdapter::new();
        assert_eq!(adapter.get_language(), "go");
    }

    #[tokio::test]
    async fn test_get_supported_frameworks() {
        let adapter = GoAdapter::new();
        assert_eq!(adapter.get_supported_frameworks(), vec!["testing"]);
    }
}