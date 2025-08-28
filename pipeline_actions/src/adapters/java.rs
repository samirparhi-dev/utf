use crate::core::{TestablePattern, PatternType, TestCase, TestSuite, TestGenerator};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;

pub struct JavaAdapter;

impl JavaAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_patterns(content: &str) -> Result<Vec<TestablePattern>> {
        let mut patterns = Vec::new();

        // Detect Java methods (public/private/protected static? returnType methodName(...))
        let method_regex = Regex::new(r"(?m)^\s*(?:public|private|protected)?\s*(?:static\s+)?(?:final\s+)?(\w+(?:<[^>]*>)?)\s+(\w+)\s*\([^)]*\)\s*(?:throws\s+[^{]*)?(?:\s*\{|;)")?;
        for cap in method_regex.captures_iter(content) {
            if let (Some(return_type), Some(method_name)) = (cap.get(1), cap.get(2)) {
                // Skip constructors (method name matches class name pattern)
                if !Self::is_constructor_pattern(method_name.as_str(), content) {
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Function,
                        name: method_name.as_str().to_string(),
                        parameters: Self::extract_method_parameters(&cap[0]),
                        return_type: return_type.as_str().to_string(),
                        line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                        complexity: Self::calculate_complexity(&cap[0]),
                    });
                }
            }
        }

        // Detect Java classes
        let class_regex = Regex::new(r"(?m)^\s*(?:public|private|protected)?\s*(?:abstract\s+)?(?:final\s+)?class\s+(\w+)(?:\s+extends\s+\w+)?(?:\s+implements\s+[^{]+)?\s*\{")?;
        for cap in class_regex.captures_iter(content) {
            if let Some(class_name) = cap.get(1) {
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Class,
                    name: class_name.as_str().to_string(),
                    parameters: vec![],
                    return_type: "class".to_string(),
                    line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                    complexity: 1,
                });
            }
        }

        // Detect Java interfaces
        let interface_regex = Regex::new(r"(?m)^\s*(?:public|private|protected)?\s*interface\s+(\w+)(?:\s+extends\s+[^{]+)?\s*\{")?;
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

        // Detect constructors
        let constructor_regex = Regex::new(r"(?m)^\s*(?:public|private|protected)?\s+(\w+)\s*\([^)]*\)\s*(?:throws\s+[^{]*)?(?:\s*\{)")?;
        for cap in constructor_regex.captures_iter(content) {
            if let Some(constructor_name) = cap.get(1) {
                if Self::is_constructor_pattern(constructor_name.as_str(), content) {
                    patterns.push(TestablePattern {
                        id: uuid::Uuid::new_v4().to_string(),
                        pattern_type: PatternType::Constructor,
                        name: constructor_name.as_str().to_string(),
                        parameters: Self::extract_method_parameters(&cap[0]),
                        return_type: constructor_name.as_str().to_string(),
                        line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                        complexity: 1,
                    });
                }
            }
        }

        // Detect email validation patterns
        let email_regex = Regex::new(r#"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"#)?;
        if email_regex.is_match(content) {
            patterns.push(TestablePattern {
                id: uuid::Uuid::new_v4().to_string(),
                pattern_type: PatternType::Validation,
                name: "email_validation".to_string(),
                parameters: vec!["email".to_string()],
                return_type: "boolean".to_string(),
                line_number: None,
                complexity: 2,
            });
        }

        // Detect exception handling patterns
        let exception_regex = Regex::new(r"throws\s+(\w+(?:Exception|Error))")?;
        for cap in exception_regex.captures_iter(content) {
            if let Some(exception_name) = cap.get(1) {
                patterns.push(TestablePattern {
                    id: uuid::Uuid::new_v4().to_string(),
                    pattern_type: PatternType::Exception,
                    name: format!("{}_handling", exception_name.as_str().to_lowercase()),
                    parameters: vec!["input".to_string()],
                    return_type: "void".to_string(),
                    line_number: Some(content[..cap.get(0).unwrap().start()].matches('\n').count() + 1),
                    complexity: 2,
                });
            }
        }

        Ok(patterns)
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

    fn calculate_complexity(method_def: &str) -> u32 {
        let complexity_patterns = [
            r"if\s*\(", r"else\s+if", r"while\s*\(", r"for\s*\(", r"switch\s*\(",
            r"case\s+", r"catch\s*\(", r"throw\s+", r"synchronized\s*\(", r"try\s*\{",
            r"\?\s*:", r"&&", r"\|\|",
        ];
        
        let mut complexity = 1; // Base complexity
        for pattern in &complexity_patterns {
            let regex = Regex::new(pattern).unwrap();
            complexity += regex.find_iter(method_def).count() as u32;
        }
        complexity
    }

    fn generate_java_test(pattern: &TestablePattern, class_name: &str) -> String {
        match pattern.pattern_type {
            PatternType::Function => {
                format!(
                    r#"@Test
public void test{}() {{
    // Arrange
    {} instance = new {}();
    {} expected = {}; // TODO: Set expected value
    {} {} = {}; // TODO: Set test parameters
    
    // Act
    {} actual = instance.{}({});
    
    // Assert
    assertEquals(expected, actual);
    
    // TODO: Add more test cases
    // TODO: Test edge cases and boundary conditions
}}"#,
                    pattern.name,
                    class_name,
                    class_name,
                    if pattern.return_type == "void" { "// No return value" } else { &pattern.return_type },
                    Self::generate_default_value(&pattern.return_type),
                    Self::generate_parameter_declarations(&pattern.parameters),
                    Self::generate_parameter_names(&pattern.parameters),
                    Self::generate_default_parameter_values(&pattern.parameters),
                    if pattern.return_type == "void" { "" } else { &pattern.return_type },
                    pattern.name,
                    Self::generate_parameter_names(&pattern.parameters)
                )
            }
            PatternType::Constructor => {
                format!(
                    r#"@Test
public void test{}Constructor() {{
    // Arrange
    {} {} = {}; // TODO: Set constructor parameters
    
    // Act
    {} instance = new {}({});
    
    // Assert
    assertNotNull(instance);
    // TODO: Assert on instance state/properties
    
    // TODO: Test constructor with different parameter combinations
}}"#,
                    pattern.name,
                    Self::generate_parameter_declarations(&pattern.parameters),
                    Self::generate_parameter_names(&pattern.parameters),
                    Self::generate_default_parameter_values(&pattern.parameters),
                    pattern.name,
                    pattern.name,
                    Self::generate_parameter_names(&pattern.parameters)
                )
            }
            PatternType::Class => {
                format!(
                    r#"@Test
public void test{}Creation() {{
    // Test {} class creation and basic operations
    {} instance = new {}();
    
    // Assert
    assertNotNull(instance);
    assertEquals("{}", instance.getClass().getSimpleName());
    
    // TODO: Test class methods and properties
    // TODO: Test class invariants
}}

@Test
public void test{}Methods() {{
    // Test {} class methods
    {} instance = new {}();
    
    // TODO: Test public methods
    // TODO: Test method combinations
    // TODO: Test state changes
}}"#,
                    pattern.name, pattern.name, pattern.name, pattern.name, pattern.name,
                    pattern.name, pattern.name, pattern.name, pattern.name
                )
            }
            PatternType::Interface => {
                format!(
                    r#"@Test
public void test{}Implementation() {{
    // Test {} interface implementation
    {} mock = Mockito.mock({}.class);
    
    // TODO: Setup mock behavior
    // when(mock.someMethod()).thenReturn(expectedValue);
    
    // TODO: Test interface contract
    // TODO: Verify method calls
}}

// Mock implementation for testing
private static class Mock{} implements {} {{
    // TODO: Implement interface methods for testing
}}"#,
                    pattern.name, pattern.name, pattern.name, pattern.name,
                    pattern.name, pattern.name
                )
            }
            PatternType::Exception => {
                format!(
                    r#"@Test(expected = {}.class)
public void test{}ThrowsException() {{
    // Arrange
    {} instance = new {}();
    
    // Act & Assert
    instance.methodThatThrows{}(); // TODO: Replace with actual method name
    
    // TODO: Test exception message and state
}}

@Test
public void test{}HandlesException() {{
    // Test proper exception handling
    {} instance = new {}();
    
    try {{
        instance.methodThatHandles{}(); // TODO: Replace with actual method name
        // TODO: Assert expected behavior when exception is handled
    }} catch (Exception e) {{
        fail("Exception should be handled gracefully");
    }}
}}"#,
                    pattern.name.replace("_handling", "").to_uppercase(),
                    pattern.name.replace("_handling", ""),
                    class_name, class_name,
                    pattern.name.replace("_handling", "").to_uppercase(),
                    pattern.name.replace("_handling", ""),
                    class_name, class_name,
                    pattern.name.replace("_handling", "").to_uppercase()
                )
            }
            PatternType::Validation => {
                r#"@Test
public void testEmailValidation() {
    // Test valid emails
    assertTrue(isValidEmail("test@example.com"));
    assertTrue(isValidEmail("user.name+tag@domain.co.uk"));
    
    // Test invalid emails
    assertFalse(isValidEmail("invalid-email"));
    assertFalse(isValidEmail("@domain.com"));
    assertFalse(isValidEmail("user@"));
    assertFalse(isValidEmail(""));
    assertFalse(isValidEmail(null));
    
    // TODO: Replace isValidEmail with actual validation method
}"#.to_string()
            }
            _ => format!("// TODO: Add test for {}", pattern.name),
        }
    }

    fn generate_parameter_declarations(params: &[String]) -> String {
        if params.is_empty() {
            "".to_string()
        } else {
            params.iter()
                .map(|p| format!("Object {}", p)) // Generic type for simplicity
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn generate_parameter_names(params: &[String]) -> String {
        params.join(", ")
    }

    fn generate_default_parameter_values(params: &[String]) -> String {
        if params.is_empty() {
            "".to_string()
        } else {
            params.iter()
                .map(|_| "null") // Default to null for simplicity
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    fn generate_default_value(return_type: &str) -> String {
        match return_type {
            "void" => "// No return value".to_string(),
            "boolean" => "false".to_string(),
            "int" | "Integer" => "0".to_string(),
            "long" | "Long" => "0L".to_string(),
            "double" | "Double" => "0.0".to_string(),
            "float" | "Float" => "0.0f".to_string(),
            "String" => r#""""#.to_string(),
            "char" | "Character" => "'a'".to_string(),
            _ => "null".to_string(),
        }
    }

    fn extract_class_name(content: &str) -> String {
        let class_regex = Regex::new(r"class\s+(\w+)").unwrap();
        if let Some(cap) = class_regex.captures(content) {
            cap.get(1).unwrap().as_str().to_string()
        } else {
            "TestClass".to_string()
        }
    }
}

#[async_trait]
impl TestGenerator for JavaAdapter {
    async fn detect_patterns(&self, content: &str) -> Result<Vec<TestablePattern>> {
        Self::detect_patterns(content)
    }

    async fn generate_tests(&self, patterns: Vec<TestablePattern>) -> Result<TestSuite> {
        let mut test_cases = Vec::new();
        let class_name = Self::extract_class_name(&patterns.iter()
            .find(|p| p.pattern_type == PatternType::Class)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "TestClass".to_string()));

        for pattern in patterns {
            let test_content = Self::generate_java_test(&pattern, &class_name);
            test_cases.push(TestCase {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("test{}", pattern.name),
                test_function: test_content,
                assertions: vec![],
                setup: None,
                teardown: None,
            });
        }

        Ok(TestSuite {
            id: uuid::Uuid::new_v4().to_string(),
            name: format!("{}Test", class_name),
            language: "java".to_string(),
            framework: "junit".to_string(),
            test_cases,
            setup_code: Some(format!(
                "import org.junit.*;\nimport static org.junit.Assert.*;\nimport org.mockito.Mockito;\n\npublic class {}Test {{\n",
                class_name
            )),
            imports: vec![
                "org.junit.*".to_string(),
                "org.junit.Assert.*".to_string(),
                "org.mockito.Mockito".to_string(),
            ],
        })
    }

    fn get_language(&self) -> &str {
        "java"
    }

    fn get_file_extensions(&self) -> Vec<&str> {
        vec!["java"]
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
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let method_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Function)
            .collect();
        assert_eq!(method_patterns.len(), 1);
        assert_eq!(method_patterns[0].name, "add");
        assert_eq!(method_patterns[0].parameters, vec!["a", "b"]);
        assert_eq!(method_patterns[0].return_type, "int");
    }

    #[test]
    fn test_detect_class() {
        let content = r#"
public class User {
    private String name;
    private int age;
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let class_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Class)
            .collect();
        assert_eq!(class_patterns.len(), 1);
        assert_eq!(class_patterns[0].name, "User");
    }

    #[test]
    fn test_detect_interface() {
        let content = r#"
public interface Repository {
    void save(Object entity);
    Object findById(Long id);
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let interface_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Interface)
            .collect();
        assert_eq!(interface_patterns.len(), 1);
        assert_eq!(interface_patterns[0].name, "Repository");
        
        // Should also detect interface methods
        let method_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Function)
            .collect();
        assert_eq!(method_patterns.len(), 2);
    }

    #[test]
    fn test_detect_constructor() {
        let content = r#"
public class Person {
    private String name;
    
    public Person(String name) {
        this.name = name;
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let constructor_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Constructor)
            .collect();
        assert_eq!(constructor_patterns.len(), 1);
        assert_eq!(constructor_patterns[0].name, "Person");
        assert_eq!(constructor_patterns[0].parameters, vec!["name"]);
    }

    #[test]
    fn test_detect_static_method() {
        let content = r#"
public class MathUtils {
    public static double sqrt(double value) {
        return Math.sqrt(value);
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let method_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Function && p.name == "sqrt")
            .collect();
        assert_eq!(method_patterns.len(), 1);
        assert_eq!(method_patterns[0].return_type, "double");
        assert_eq!(method_patterns[0].parameters, vec!["value"]);
    }

    #[test]
    fn test_detect_generic_method() {
        let content = r#"
public class GenericService {
    public <T> List<T> processItems(List<T> items) {
        return items;
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let method_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Function && p.name == "processItems")
            .collect();
        assert_eq!(method_patterns.len(), 1);
        assert!(method_patterns[0].return_type.contains("List"));
    }

    #[test]
    fn test_detect_exception_handling() {
        let content = r#"
public class FileService {
    public String readFile(String path) throws IOException {
        return Files.readString(Paths.get(path));
    }
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let exception_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Exception)
            .collect();
        assert_eq!(exception_patterns.len(), 1);
        assert_eq!(exception_patterns[0].name, "ioexception_handling");
    }

    #[test]
    fn test_detect_email_validation() {
        let content = r#"
public class Validator {
    private static final String EMAIL = "user@example.com";
    private static final String INVALID = "not-an-email";
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        let validation_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Validation)
            .collect();
        assert_eq!(validation_patterns.len(), 1);
        assert_eq!(validation_patterns[0].name, "email_validation");
    }

    #[test]
    fn test_detect_multiple_patterns() {
        let content = r#"
package com.example;

import java.util.List;
import java.io.IOException;

public class UserService implements UserRepository {
    
    public UserService() {
        // Constructor
    }
    
    public User createUser(String name, String email) throws ValidationException {
        if (!isValidEmail(email)) {
            throw new ValidationException("Invalid email: " + email);
        }
        return new User(name, email);
    }
    
    private boolean isValidEmail(String email) {
        return email.contains("@") && email.contains(".");
    }
    
    @Override
    public List<User> findAll() {
        return repository.findAll();
    }
}

interface UserRepository {
    List<User> findAll();
}
"#;
        let patterns = JavaAdapter::detect_patterns(content).unwrap();
        
        // Should find class, interface, constructor, methods, exception handling
        let class_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Class)
            .collect();
        assert!(!class_patterns.is_empty());
        
        let interface_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Interface)
            .collect();
        assert!(!interface_patterns.is_empty());
        
        let method_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Function)
            .collect();
        assert!(method_patterns.len() >= 2); // createUser, isValidEmail, findAll
        
        let constructor_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Constructor)
            .collect();
        assert!(!constructor_patterns.is_empty());
        
        let exception_patterns: Vec<_> = patterns.iter()
            .filter(|p| p.pattern_type == PatternType::Exception)
            .collect();
        assert!(!exception_patterns.is_empty());
    }

    #[test]
    fn test_extract_method_parameters_empty() {
        let method_def = "public void doSomething()";
        let params = JavaAdapter::extract_method_parameters(method_def);
        assert!(params.is_empty());
    }

    #[test]
    fn test_extract_method_parameters_single() {
        let method_def = "public int calculate(int value)";
        let params = JavaAdapter::extract_method_parameters(method_def);
        assert_eq!(params, vec!["value"]);
    }

    #[test]
    fn test_extract_method_parameters_multiple() {
        let method_def = "public String format(String template, Object... args)";
        let params = JavaAdapter::extract_method_parameters(method_def);
        assert_eq!(params, vec!["template", "args"]);
    }

    #[test]
    fn test_calculate_complexity_simple() {
        let method_def = "public int add(int a, int b) { return a + b; }";
        let complexity = JavaAdapter::calculate_complexity(method_def);
        assert_eq!(complexity, 1); // base complexity
    }

    #[test]
    fn test_calculate_complexity_with_conditionals() {
        let method_def = r#"
public int processValue(int value) {
    if (value < 0) {
        throw new IllegalArgumentException("Negative value");
    } else if (value == 0) {
        return 0;
    }
    
    while (value > 100) {
        value /= 2;
    }
    
    switch (value) {
        case 1:
            return 1;
        case 2:
            return 4;
        default:
            return value * 2;
    }
}
"#;
        let complexity = JavaAdapter::calculate_complexity(method_def);
        assert!(complexity > 5); // should detect multiple control structures
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

    #[tokio::test]
    async fn test_generate_tests_for_method() {
        let adapter = JavaAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Function,
            name: "calculate".to_string(),
            parameters: vec!["value".to_string()],
            return_type: "int".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.language, "java");
        assert_eq!(test_suite.framework, "junit");
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].test_function.contains("@Test"));
        assert!(test_suite.test_cases[0].test_function.contains("testcalculate"));
        assert!(test_suite.test_cases[0].test_function.contains("assertEquals"));
    }

    #[tokio::test]
    async fn test_generate_tests_for_constructor() {
        let adapter = JavaAdapter::new();
        let pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Constructor,
            name: "User".to_string(),
            parameters: vec!["name".to_string()],
            return_type: "User".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![pattern]).await.unwrap();
        assert_eq!(test_suite.test_cases.len(), 1);
        assert!(test_suite.test_cases[0].test_function.contains("testUserConstructor"));
        assert!(test_suite.test_cases[0].test_function.contains("new User"));
        assert!(test_suite.test_cases[0].test_function.contains("assertNotNull"));
    }

    #[tokio::test]
    async fn test_generate_tests_for_class() {
        let adapter = JavaAdapter::new();
        let class_pattern = TestablePattern {
            id: "test".to_string(),
            pattern_type: PatternType::Class,
            name: "Calculator".to_string(),
            parameters: vec![],
            return_type: "class".to_string(),
            line_number: Some(1),
            complexity: 1,
        };
        
        let test_suite = adapter.generate_tests(vec![class_pattern]).await.unwrap();
        assert!(test_suite.test_cases.len() >= 1);
        assert!(test_suite.test_cases[0].test_function.contains("testCalculatorCreation"));
        assert!(test_suite.setup_code.is_some());
        assert!(test_suite.setup_code.as_ref().unwrap().contains("public class CalculatorTest"));
    }

    #[tokio::test]
    async fn test_get_language() {
        let adapter = JavaAdapter::new();
        assert_eq!(adapter.get_language(), "java");
    }

    #[tokio::test]
    async fn test_get_file_extensions() {
        let adapter = JavaAdapter::new();
        assert_eq!(adapter.get_file_extensions(), vec!["java"]);
    }
}