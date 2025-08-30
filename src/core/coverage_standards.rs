use std::collections::HashMap;

/// Industry standard code coverage targets by programming language
/// Based on industry best practices and community standards
pub struct CoverageStandards;

impl CoverageStandards {
    /// Get the recommended coverage target for a given programming language
    pub fn get_coverage_target(language: &str) -> f32 {
        match language.to_lowercase().as_str() {
            // Rust: High standards due to strong type system and safety focus
            "rust" => 75.0,
            
            // JavaScript: Good coverage expected due to dynamic nature
            "javascript" | "js" | "typescript" | "ts" => 80.0,
            
            // Python: High coverage standard in the community
            "python" => 85.0,
            
            // Java: Enterprise-grade coverage expectations
            "java" => 80.0,
            
            // Go: Good coverage standard
            "go" => 70.0,
            
            // C#: Enterprise standards
            "csharp" | "c#" => 80.0,
            
            // Swift: iOS development standards
            "swift" => 75.0,
            
            // Kotlin: Modern JVM language standards
            "kotlin" => 80.0,
            
            // PHP: Web development standards
            "php" => 70.0,
            
            // Ruby: Community standards
            "ruby" => 80.0,
            
            // Scala: Functional programming standards
            "scala" => 75.0,
            
            // C++: Lower due to complexity of testing
            "cpp" | "c++" => 60.0,
            
            // C: Systems programming standards
            "c" => 65.0,
            
            // Default for unknown languages
            _ => 70.0,
        }
    }
    
    /// Get comprehensive coverage standards with explanations
    pub fn get_coverage_standards() -> HashMap<String, CoverageInfo> {
        let mut standards = HashMap::new();
        
        standards.insert("rust".to_string(), CoverageInfo {
            target: 75.0,
            reasoning: "Rust's strong type system catches many errors at compile time, but thorough testing is still crucial for business logic and edge cases.".to_string(),
            test_types: vec![
                "Unit tests for functions and methods".to_string(),
                "Integration tests for modules".to_string(),
                "Property-based tests for complex logic".to_string(),
                "Error handling tests".to_string(),
            ],
        });
        
        standards.insert("javascript".to_string(), CoverageInfo {
            target: 80.0,
            reasoning: "JavaScript's dynamic nature requires comprehensive testing to catch runtime errors and type-related issues.".to_string(),
            test_types: vec![
                "Unit tests for functions and classes".to_string(),
                "Integration tests for components".to_string(),
                "End-to-end tests for user workflows".to_string(),
                "Browser compatibility tests".to_string(),
            ],
        });
        
        standards.insert("python".to_string(), CoverageInfo {
            target: 85.0,
            reasoning: "Python's dynamic typing and extensive use in critical applications demands high test coverage.".to_string(),
            test_types: vec![
                "Unit tests with pytest".to_string(),
                "Integration tests for APIs and databases".to_string(),
                "Property-based tests with Hypothesis".to_string(),
                "Type checking with mypy".to_string(),
            ],
        });
        
        standards.insert("java".to_string(), CoverageInfo {
            target: 80.0,
            reasoning: "Java's enterprise usage patterns require high reliability and maintainability through comprehensive testing.".to_string(),
            test_types: vec![
                "JUnit unit tests".to_string(),
                "Integration tests with TestContainers".to_string(),
                "Mock-based testing".to_string(),
                "Spring Boot test slices".to_string(),
            ],
        });
        
        standards.insert("go".to_string(), CoverageInfo {
            target: 70.0,
            reasoning: "Go's simplicity and built-in testing tools make achieving good coverage straightforward.".to_string(),
            test_types: vec![
                "Table-driven tests".to_string(),
                "Benchmark tests".to_string(),
                "Example tests".to_string(),
                "Integration tests".to_string(),
            ],
        });
        
        standards
    }
    
    /// Generate test cases based on coverage requirements
    pub fn generate_comprehensive_test_cases(language: &str) -> Vec<TestCaseType> {
        match language.to_lowercase().as_str() {
            "rust" => vec![
                TestCaseType::HappyPath,
                TestCaseType::EdgeCase,
                TestCaseType::ErrorHandling,
                TestCaseType::BoundaryCondition,
                TestCaseType::PropertyBased,
            ],
            "javascript" | "typescript" => vec![
                TestCaseType::HappyPath,
                TestCaseType::EdgeCase,
                TestCaseType::ErrorHandling,
                TestCaseType::AsyncBehavior,
                TestCaseType::BrowserCompatibility,
            ],
            "python" => vec![
                TestCaseType::HappyPath,
                TestCaseType::EdgeCase,
                TestCaseType::ErrorHandling,
                TestCaseType::TypeValidation,
                TestCaseType::PropertyBased,
                TestCaseType::Performance,
            ],
            "java" => vec![
                TestCaseType::HappyPath,
                TestCaseType::EdgeCase,
                TestCaseType::ErrorHandling,
                TestCaseType::MockingIntegration,
                TestCaseType::ConcurrencyTesting,
            ],
            _ => vec![
                TestCaseType::HappyPath,
                TestCaseType::EdgeCase,
                TestCaseType::ErrorHandling,
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoverageInfo {
    pub target: f32,
    pub reasoning: String,
    pub test_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestCaseType {
    HappyPath,
    EdgeCase,
    ErrorHandling,
    BoundaryCondition,
    PropertyBased,
    AsyncBehavior,
    BrowserCompatibility,
    TypeValidation,
    Performance,
    MockingIntegration,
    ConcurrencyTesting,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_coverage_target() {
        assert_eq!(CoverageStandards::get_coverage_target("rust"), 75.0);
    }

    #[test]
    fn test_javascript_coverage_target() {
        assert_eq!(CoverageStandards::get_coverage_target("javascript"), 80.0);
        assert_eq!(CoverageStandards::get_coverage_target("typescript"), 80.0);
    }

    #[test]
    fn test_python_coverage_target() {
        assert_eq!(CoverageStandards::get_coverage_target("python"), 85.0);
    }

    #[test]
    fn test_unknown_language_default() {
        assert_eq!(CoverageStandards::get_coverage_target("unknown"), 70.0);
    }

    #[test]
    fn test_coverage_standards_exist() {
        let standards = CoverageStandards::get_coverage_standards();
        assert!(standards.contains_key("rust"));
        assert!(standards.contains_key("javascript"));
        assert!(standards.contains_key("python"));
        assert!(standards.contains_key("java"));
    }

    #[test]
    fn test_comprehensive_test_cases_generation() {
        let rust_cases = CoverageStandards::generate_comprehensive_test_cases("rust");
        assert!(rust_cases.contains(&TestCaseType::HappyPath));
        assert!(rust_cases.contains(&TestCaseType::ErrorHandling));
        assert!(rust_cases.contains(&TestCaseType::PropertyBased));
        
        let js_cases = CoverageStandards::generate_comprehensive_test_cases("javascript");
        assert!(js_cases.contains(&TestCaseType::AsyncBehavior));
        assert!(js_cases.contains(&TestCaseType::BrowserCompatibility));
    }
}