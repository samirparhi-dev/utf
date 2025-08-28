use crate::core::{TestablePattern, PatternType, TestCase, TestSuite, TestGenerator};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct GoAdapter;

impl GoAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_patterns(content: &str) -> Result<Vec<TestablePattern>> {
        let mut patterns = Vec::new();

        // Detect Go functions (func name(...) returnType)
        let func_regex = Regex::new(r"func\s+(\w+)\s*\([^)]*\)(?:\s*[^{]*)?(?:\s*\{|$)")?;
        for cap in func_regex.captures_iter(content) {
            if let Some(func_name) = cap.get(1) {
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Function,
                    name: func_name.as_str().to_string(),
                    parameters: Self::extract_function_parameters(&cap[0]),
                    return_type: Self::extract_return_type(&cap[0]),
                    line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                    complexity: Self::calculate_complexity(&cap[0]),
                });
            }
        }

        // Detect Go structs
        let struct_regex = Regex::new(r"type\s+(\w+)\s+struct\s*\{")?;
        for cap in struct_regex.captures_iter(content) {
            if let Some(struct_name) = cap.get(1) {
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Class,
                    name: struct_name.as_str().to_string(),
                    parameters: vec![],
                    return_type: "struct".to_string(),
                    line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                    complexity: 1,
                });
            }
        }

        // Detect Go interfaces
        let interface_regex = Regex::new(r"type\s+(\w+)\s+interface\s*\{")?;
        for cap in interface_regex.captures_iter(content) {
            if let Some(interface_name) = cap.get(1) {
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Interface,
                    name: interface_name.as_str().to_string(),
                    parameters: vec![],
                    return_type: "interface".to_string(),
                    line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                    complexity: 1,
                });
            }
        }

        // Detect email validation patterns (common in Go applications)
        let email_regex = Regex::new(r#"[^@\s]+@[^@\s]+\.[^@\s]+"#)?;
        if email_regex.is_match(content) {
            patterns.push(TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Validation,
                name: "email_validation".to_string(),
                parameters: vec!["email".to_string()],
                return_type: "bool".to_string(),
                line_number: None,
                complexity: 2,
            });
        }

        Ok(patterns)
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

    fn calculate_complexity(func_def: &str) -> u32 {
        let complexity_patterns = [
            r"if\s+", r"for\s+", r"switch\s+", r"case\s+", r"select\s+",
            r"go\s+", r"defer\s+", r"chan\s+", r"make\s*\(", r"new\s*\(",
        ];
        
        let mut complexity = 1; // Base complexity
        for pattern in &complexity_patterns {
            let regex = Regex::new(pattern).unwrap();
            complexity += regex.find_iter(func_def).count() as u32;
        }
        complexity
    }

    fn generate_go_test(pattern: &TestablePattern) -> String {
        match pattern.pattern_type {
            PatternType::Function => {
                format!(
                    r#"func Test{}(t *testing.T) {{
    // Test cases for {}
    tests := []struct {{
        name string
        args {}
        want {}
    }}{{
        {{
            name: "test case 1",
            args: {}, // TODO: Add test arguments
            want: {}, // TODO: Add expected result
        }},
        // TODO: Add more test cases
    }}

    for _, tt := range tests {{
        t.Run(tt.name, func(t *testing.T) {{
            got := {}({})
            if got != tt.want {{
                t.Errorf("{} = %v, want %v", got, tt.want)
            }}
        }})
    }}
}}"#,
                    pattern.name,
                    pattern.name,
                    Self::generate_args_struct(&pattern.parameters),
                    if pattern.return_type == "void" { "()" } else { &pattern.return_type },
                    Self::generate_default_args(&pattern.parameters),
                    Self::generate_default_return(&pattern.return_type),
                    pattern.name,
                    Self::generate_function_call_args(&pattern.parameters),
                    pattern.name
                )
            }
            PatternType::Class => {
                format!(
                    r#"func Test{}Creation(t *testing.T) {{
    // Test {} struct creation and basic operations
    instance := {}{{}}
    
    // TODO: Add field assignments and validations
    // TODO: Test struct methods if any
    
    if reflect.TypeOf(instance).Name() != "{}" {{
        t.Errorf("Expected struct type {}, got %T", instance)
    }}
}}"#,
                    pattern.name, pattern.name, pattern.name, pattern.name, pattern.name
                )
            }
            PatternType::Interface => {
                format!(
                    r#"func Test{}Interface(t *testing.T) {{
    // Test {} interface implementation
    // TODO: Create a mock implementation
    
    var _ {} = (*Mock{})(&Mock{}{{}})
    
    // TODO: Add interface method tests
}}

type Mock{} struct {{
    // TODO: Add mock fields
}}

// TODO: Implement {} interface methods"#,
                    pattern.name, pattern.name, pattern.name, pattern.name, pattern.name, pattern.name, pattern.name
                )
            }
            PatternType::Validation => {
                r#"func TestEmailValidation(t *testing.T) {
    tests := []struct {
        name  string
        email string
        want  bool
    }{
        {"valid email", "test@example.com", true},
        {"invalid email no @", "testexample.com", false},
        {"invalid email no domain", "test@", false},
        {"empty email", "", false},
    }

    for _, tt := range tests {
        t.Run(tt.name, func(t *testing.T) {
            // TODO: Replace with actual validation function
            got := validateEmail(tt.email)
            if got != tt.want {
                t.Errorf("validateEmail(%s) = %v, want %v", tt.email, got, tt.want)
            }
        })
    }
}"#.to_string()
            }
            _ => format!("// TODO: Add test for {}", pattern.name),
        }
    }

    fn generate_args_struct(params: &[String]) -> String {
        if params.is_empty() {
            "struct{}".to_string()
        } else {
            format!("struct{{ {} }}", 
                params.iter()
                    .map(|p| format!("{} interface{{}}", p))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    fn generate_default_args(params: &[String]) -> String {
        if params.is_empty() {
            "struct{}{}".to_string()
        } else {
            format!("struct{{ {} }}{{{}}}", 
                params.iter()
                    .map(|p| format!("{} interface{{}}", p))
                    .collect::<Vec<_>>()
                    .join(", "),
                params.iter()
                    .map(|_| "nil")
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }

    fn generate_function_call_args(params: &[String]) -> String {
        if params.is_empty() {
            "".to_string()
        } else {
            params.iter()
                .map(|p| format!("tt.args.{}", p))
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn generate_default_return(return_type: &str) -> String {
        match return_type {
            "void" | "" => "nil".to_string(),
            "string" => r#""""#.to_string(),
            "int" | "int32" | "int64" => "0".to_string(),
            "bool" => "false".to_string(),
            "float32" | "float64" => "0.0".to_string(),
            _ => "nil".to_string(),
        }
    }
}

#[async_trait]
impl TestGenerator for GoAdapter {
    async fn detect_patterns(&self, content: &str) -> Result<Vec<TestablePattern>> {
        Self::detect_patterns(content)
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();

        for pattern in patterns {
            let test_content = Self::generate_go_test(&pattern);
            test_cases.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("Test{}", pattern.name),
                test_function: test_content,
                assertions: vec![],
                setup: None,
                teardown: None,
            });
        }

        Ok(TestSuite {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Go Tests".to_string(),
            language: "go".to_string(),
            framework: "testing".to_string(),
            test_cases,
            setup_code: Some("package main\n\nimport (\n    \"testing\"\n    \"reflect\"\n)".to_string()),
            imports: vec![
                "testing".to_string(),
                "reflect".to_string(),
            ],
        })
    }

    fn get_language(&self) -> &str {
        "go"
    }

    fn get_file_extensions(&self) -> Vec<&str> {
        vec!["go"]
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
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "Add");
        assert_eq!(patterns[0].pattern_type, PatternType::Function);
        assert_eq!(patterns[0].parameters, vec!["a", "b"]);
    }

    #[test]
    fn test_detect_function_no_params() {
        let content = r#"
func GetCurrentTime() time.Time {
    return time.Now()
}
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "GetCurrentTime");
        assert!(patterns[0].parameters.is_empty());
    }

    #[test]
    fn test_detect_function_multiple_return() {
        let content = r#"
func Divide(a, b float64) (float64, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "Divide");
        assert_eq!(patterns[0].parameters.len(), 2);
    }

    #[test]
    fn test_detect_struct() {
        let content = r#"
type Person struct {
    Name string
    Age  int
}
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "Person");
        assert_eq!(patterns[0].pattern_type, PatternType::Class);
    }

    #[test]
    fn test_detect_interface() {
        let content = r#"
type Writer interface {
    Write([]byte) (int, error)
}
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].name, "Writer");
        assert_eq!(patterns[0].pattern_type, PatternType::Interface);
    }

    #[test]
    fn test_detect_multiple_patterns() {
        let content = r#"
package main

type User struct {
    Email string
    Name  string
}

type Repository interface {
    Save(user User) error
    FindByEmail(email string) (User, error)
}

func ValidateEmail(email string) bool {
    // Simple email validation
    return strings.Contains(email, "@")
}

func CreateUser(name, email string) User {
    return User{Name: name, Email: email}
}
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        assert!(patterns.len() >= 4); // struct, interface, 2 functions, possibly email validation
        
        let struct_pattern = patterns.iter().find(|p| p.name == "User" && p.pattern_type == PatternType::Class);
        assert!(struct_pattern.is_some());
        
        let interface_pattern = patterns.iter().find(|p| p.name == "Repository" && p.pattern_type == PatternType::Interface);
        assert!(interface_pattern.is_some());
        
        let validate_func = patterns.iter().find(|p| p.name == "ValidateEmail");
        assert!(validate_func.is_some());
        
        let create_func = patterns.iter().find(|p| p.name == "CreateUser");
        assert!(create_func.is_some());
    }

    #[test]
    fn test_detect_email_validation() {
        let content = r#"
const validEmail = "user@example.com"
const invalidEmail = "not-an-email"
"#;
        let patterns = GoAdapter::detect_patterns(content).unwrap();
        let email_pattern = patterns.iter().find(|p| p.pattern_type == PatternType::Validation);
        assert!(email_pattern.is_some());
        assert_eq!(email_pattern.unwrap().name, "email_validation");
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

    #[test]
    fn test_extract_function_parameters_single() {
        let func_def = "func Square(x float64) float64";
        let params = GoAdapter::extract_function_parameters(func_def);
        assert_eq!(params, vec!["x"]);
    }

    #[test]
    fn test_calculate_complexity_simple() {
        let func_def = "func Add(a, b int) int { return a + b }";
        let complexity = GoAdapter::calculate_complexity(func_def);
        assert_eq!(complexity, 1); // base complexity
    }

    #[test]
    fn test_calculate_complexity_with_conditionals() {
        let func_def = r#"
func ProcessData(data []int) int {
    if len(data) == 0 {
        return 0
    }
    
    for i, val := range data {
        if val > 10 {
            switch val {
            case 20:
                return i
            default:
                continue
            }
        }
    }
    return -1
}
"#;
        let complexity = GoAdapter::calculate_complexity(func_def);
        assert!(complexity > 1); // should detect control structures
    }

    #[tokio::test]
    async fn test_generate_tests_for_function() {
        let adapter = GoAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Function,
            name: "Add".to_string(),
            parameters: vec!["a".to_string(), "b".to_string()],
            return_type: "int".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.language, "go");
        assert_eq!(test_suite.framework, "testing");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].test_function.contains("TestAdd"));
        assert!(test_suite.test_cases[0].test_function.contains("t.Run"));
    }

    #[tokio::test]
    async fn test_generate_tests_for_struct() {
        let adapter = GoAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Class,
            name: "User".to_string(),
            parameters: vec![],
            return_type: "struct".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].test_function.contains("TestUserCreation"));
        assert!(test_suite.test_cases[0].test_function.contains("reflect.TypeOf"));
    }

    #[tokio::test]
    async fn test_generate_tests_for_interface() {
        let adapter = GoAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Interface,
            name: "Writer".to_string(),
            parameters: vec![],
            return_type: "interface".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].test_function.contains("TestWriterInterface"));
        assert!(test_suite.test_cases[0].test_function.contains("MockWriter"));
    }

    #[tokio::test]
    async fn test_get_language() {
        let adapter = GoAdapter::new();
        assert_eq!(adapter.get_language(), "go");
    }

    #[tokio::test]
    async fn test_get_file_extensions() {
        let adapter = GoAdapter::new();
        assert_eq!(adapter.get_file_extensions(), vec!["go"]);
    }
}