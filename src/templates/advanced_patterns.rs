use serde_json::Value;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AdvancedPattern {
    pub name: String,
    pub pattern_type: AdvancedPatternType,
    pub complexity_score: f32,
    pub test_variants: Vec<TestVariant>,
}

#[derive(Debug, Clone)]
pub enum AdvancedPatternType {
    DatabaseOperation {
        operation: String, // CREATE, READ, UPDATE, DELETE
        entity: String,
    },
    FileSystem {
        operation: String, // read, write, delete, create
        path_pattern: String,
    },
    NetworkRequest {
        method: String,
        endpoint_pattern: String,
        auth_required: bool,
    },
    DataValidation {
        field_type: String,
        validation_rules: Vec<String>,
    },
    ErrorHandling {
        error_types: Vec<String>,
        recovery_strategies: Vec<String>,
    },
    ConcurrentOperation {
        operation_type: String,
        thread_safety: bool,
    },
    CachingOperation {
        cache_type: String,
        eviction_policy: String,
    },
    SecurityOperation {
        operation: String,
        sensitive_data: bool,
    },
}

#[derive(Debug, Clone)]
pub struct TestVariant {
    pub name: String,
    pub scenario: TestScenario,
    pub inputs: Vec<Value>,
    pub expected_outcomes: Vec<Value>,
    pub setup_requirements: Vec<String>,
    pub cleanup_requirements: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TestScenario {
    HappyPath,
    EdgeCase,
    ErrorCondition,
    SecurityTest,
    PerformanceTest,
    ConcurrencyTest,
    IntegrationTest,
}

pub struct AdvancedPatternDetector {
    database_regex: Regex,
    filesystem_regex: Regex,
    network_regex: Regex,
    validation_regex: Regex,
    error_handling_regex: Regex,
    concurrency_regex: Regex,
    caching_regex: Regex,
    security_regex: Regex,
}

impl AdvancedPatternDetector {
    pub fn new() -> Self {
        Self {
            database_regex: Regex::new(r"(?i)(select|insert|update|delete|create|drop|alter|database|table|query|sql)").unwrap(),
            filesystem_regex: Regex::new(r"(?i)(file|path|directory|folder|read|write|delete|create|exists|fs\.|filesystem)").unwrap(),
            network_regex: Regex::new(r"(?i)(http|https|api|request|response|fetch|axios|curl|endpoint|url)").unwrap(),
            validation_regex: Regex::new(r"(?i)(validate|verify|check|sanitize|clean|format|parse|email|phone|url|regex)").unwrap(),
            error_handling_regex: Regex::new(r"(?i)(error|exception|throw|catch|try|fail|panic|result|option|unwrap)").unwrap(),
            concurrency_regex: Regex::new(r"(?i)(thread|async|await|promise|concurrent|parallel|mutex|lock|atomic|sync)").unwrap(),
            caching_regex: Regex::new(r"(?i)(cache|redis|memcache|storage|session|store|retrieve|expire|ttl)").unwrap(),
            security_regex: Regex::new(r"(?i)(auth|token|jwt|password|encrypt|decrypt|hash|salt|security|permission|role)").unwrap(),
        }
    }
    
    pub fn detect_advanced_patterns(&self, source_code: &str, function_name: &str) -> Vec<AdvancedPattern> {
        let mut patterns = Vec::new();
        
        // Database operations
        if self.database_regex.is_match(source_code) {
            patterns.push(self.create_database_pattern(function_name, source_code));
        }
        
        // File system operations
        if self.filesystem_regex.is_match(source_code) {
            patterns.push(self.create_filesystem_pattern(function_name, source_code));
        }
        
        // Network requests
        if self.network_regex.is_match(source_code) {
            patterns.push(self.create_network_pattern(function_name, source_code));
        }
        
        // Data validation
        if self.validation_regex.is_match(source_code) {
            patterns.push(self.create_validation_pattern(function_name, source_code));
        }
        
        // Error handling
        if self.error_handling_regex.is_match(source_code) {
            patterns.push(self.create_error_handling_pattern(function_name, source_code));
        }
        
        // Concurrency operations
        if self.concurrency_regex.is_match(source_code) {
            patterns.push(self.create_concurrency_pattern(function_name, source_code));
        }
        
        // Caching operations
        if self.caching_regex.is_match(source_code) {
            patterns.push(self.create_caching_pattern(function_name, source_code));
        }
        
        // Security operations
        if self.security_regex.is_match(source_code) {
            patterns.push(self.create_security_pattern(function_name, source_code));
        }
        
        patterns
    }
    
    fn create_database_pattern(&self, function_name: &str, source_code: &str) -> AdvancedPattern {
        let operation = if source_code.contains("insert") || source_code.contains("create") {
            "CREATE".to_string()
        } else if source_code.contains("select") || source_code.contains("find") {
            "READ".to_string()
        } else if source_code.contains("update") || source_code.contains("modify") {
            "UPDATE".to_string()
        } else if source_code.contains("delete") || source_code.contains("remove") {
            "DELETE".to_string()
        } else {
            "QUERY".to_string()
        };
        
        AdvancedPattern {
            name: format!("{}_database_operation", function_name),
            pattern_type: AdvancedPatternType::DatabaseOperation {
                operation: operation.clone(),
                entity: "generic_entity".to_string(),
            },
            complexity_score: 0.8,
            test_variants: vec![
                TestVariant {
                    name: format!("test_{}_success", operation.to_lowercase()),
                    scenario: TestScenario::HappyPath,
                    inputs: vec![serde_json::json!({"valid_data": true})],
                    expected_outcomes: vec![serde_json::json!({"success": true})],
                    setup_requirements: vec!["database_connection".to_string(), "test_data".to_string()],
                    cleanup_requirements: vec!["cleanup_test_data".to_string()],
                },
                TestVariant {
                    name: format!("test_{}_connection_failure", operation.to_lowercase()),
                    scenario: TestScenario::ErrorCondition,
                    inputs: vec![serde_json::json!({"connection": "invalid"})],
                    expected_outcomes: vec![serde_json::json!({"error": "connection_failed"})],
                    setup_requirements: vec!["mock_database_failure".to_string()],
                    cleanup_requirements: vec!["restore_database_connection".to_string()],
                },
                TestVariant {
                    name: format!("test_{}_invalid_data", operation.to_lowercase()),
                    scenario: TestScenario::EdgeCase,
                    inputs: vec![serde_json::json!({"data": null})],
                    expected_outcomes: vec![serde_json::json!({"error": "invalid_data"})],
                    setup_requirements: vec!["database_connection".to_string()],
                    cleanup_requirements: vec![],
                },
            ],
        }
    }
    
    fn create_filesystem_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_filesystem_operation", function_name),
            pattern_type: AdvancedPatternType::FileSystem {
                operation: "read_write".to_string(),
                path_pattern: "/tmp/test_*".to_string(),
            },
            complexity_score: 0.7,
            test_variants: vec![
                TestVariant {
                    name: "test_file_operations_success".to_string(),
                    scenario: TestScenario::HappyPath,
                    inputs: vec![serde_json::json!("/tmp/test_file.txt")],
                    expected_outcomes: vec![serde_json::json!({"file_processed": true})],
                    setup_requirements: vec!["create_temp_file".to_string()],
                    cleanup_requirements: vec!["delete_temp_file".to_string()],
                },
                TestVariant {
                    name: "test_file_not_found".to_string(),
                    scenario: TestScenario::ErrorCondition,
                    inputs: vec![serde_json::json!("/nonexistent/file.txt")],
                    expected_outcomes: vec![serde_json::json!({"error": "file_not_found"})],
                    setup_requirements: vec![],
                    cleanup_requirements: vec![],
                },
                TestVariant {
                    name: "test_permission_denied".to_string(),
                    scenario: TestScenario::SecurityTest,
                    inputs: vec![serde_json::json!("/root/protected_file.txt")],
                    expected_outcomes: vec![serde_json::json!({"error": "permission_denied"})],
                    setup_requirements: vec!["create_protected_file".to_string()],
                    cleanup_requirements: vec!["remove_protected_file".to_string()],
                },
            ],
        }
    }
    
    fn create_network_pattern(&self, function_name: &str, source_code: &str) -> AdvancedPattern {
        let method = if source_code.contains("POST") || source_code.contains("post") {
            "POST".to_string()
        } else if source_code.contains("PUT") || source_code.contains("put") {
            "PUT".to_string()
        } else if source_code.contains("DELETE") || source_code.contains("delete") {
            "DELETE".to_string()
        } else {
            "GET".to_string()
        };
        
        AdvancedPattern {
            name: format!("{}_network_request", function_name),
            pattern_type: AdvancedPatternType::NetworkRequest {
                method: method.clone(),
                endpoint_pattern: "/api/*".to_string(),
                auth_required: source_code.contains("auth") || source_code.contains("token"),
            },
            complexity_score: 0.9,
            test_variants: vec![
                TestVariant {
                    name: format!("test_{}_success", method.to_lowercase()),
                    scenario: TestScenario::HappyPath,
                    inputs: vec![serde_json::json!({"url": "https://api.example.com/test"})],
                    expected_outcomes: vec![serde_json::json!({"status": 200, "data": "success"})],
                    setup_requirements: vec!["mock_server".to_string()],
                    cleanup_requirements: vec!["stop_mock_server".to_string()],
                },
                TestVariant {
                    name: format!("test_{}_network_timeout", method.to_lowercase()),
                    scenario: TestScenario::ErrorCondition,
                    inputs: vec![serde_json::json!({"url": "https://timeout.example.com"})],
                    expected_outcomes: vec![serde_json::json!({"error": "timeout"})],
                    setup_requirements: vec!["mock_timeout_server".to_string()],
                    cleanup_requirements: vec![],
                },
                TestVariant {
                    name: format!("test_{}_rate_limiting", method.to_lowercase()),
                    scenario: TestScenario::PerformanceTest,
                    inputs: vec![serde_json::json!({"requests": 1000})],
                    expected_outcomes: vec![serde_json::json!({"rate_limited": true})],
                    setup_requirements: vec!["setup_rate_limiter".to_string()],
                    cleanup_requirements: vec!["reset_rate_limiter".to_string()],
                },
            ],
        }
    }
    
    fn create_validation_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_validation", function_name),
            pattern_type: AdvancedPatternType::DataValidation {
                field_type: "mixed".to_string(),
                validation_rules: vec!["required".to_string(), "format".to_string(), "length".to_string()],
            },
            complexity_score: 0.6,
            test_variants: vec![
                TestVariant {
                    name: "test_validation_success".to_string(),
                    scenario: TestScenario::HappyPath,
                    inputs: vec![serde_json::json!({"email": "test@example.com", "age": 25})],
                    expected_outcomes: vec![serde_json::json!({"valid": true})],
                    setup_requirements: vec![],
                    cleanup_requirements: vec![],
                },
                TestVariant {
                    name: "test_validation_failures".to_string(),
                    scenario: TestScenario::EdgeCase,
                    inputs: vec![
                        serde_json::json!({"email": "invalid-email", "age": -1}),
                        serde_json::json!({"email": "", "age": 150}),
                        serde_json::json!(null),
                    ],
                    expected_outcomes: vec![serde_json::json!({"valid": false, "errors": ["email_invalid", "age_invalid"]})],
                    setup_requirements: vec![],
                    cleanup_requirements: vec![],
                },
            ],
        }
    }
    
    fn create_error_handling_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_error_handling", function_name),
            pattern_type: AdvancedPatternType::ErrorHandling {
                error_types: vec!["RuntimeError".to_string(), "ValueError".to_string(), "TypeError".to_string()],
                recovery_strategies: vec!["retry".to_string(), "fallback".to_string(), "graceful_degradation".to_string()],
            },
            complexity_score: 0.8,
            test_variants: vec![
                TestVariant {
                    name: "test_error_recovery".to_string(),
                    scenario: TestScenario::ErrorCondition,
                    inputs: vec![serde_json::json!({"cause_error": true})],
                    expected_outcomes: vec![serde_json::json!({"recovered": true, "fallback_used": true})],
                    setup_requirements: vec!["setup_error_conditions".to_string()],
                    cleanup_requirements: vec!["reset_error_state".to_string()],
                },
                TestVariant {
                    name: "test_error_propagation".to_string(),
                    scenario: TestScenario::ErrorCondition,
                    inputs: vec![serde_json::json!({"unrecoverable_error": true})],
                    expected_outcomes: vec![serde_json::json!({"error_propagated": true})],
                    setup_requirements: vec![],
                    cleanup_requirements: vec![],
                },
            ],
        }
    }
    
    fn create_concurrency_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_concurrency", function_name),
            pattern_type: AdvancedPatternType::ConcurrentOperation {
                operation_type: "async_processing".to_string(),
                thread_safety: true,
            },
            complexity_score: 0.95,
            test_variants: vec![
                TestVariant {
                    name: "test_concurrent_access".to_string(),
                    scenario: TestScenario::ConcurrencyTest,
                    inputs: vec![serde_json::json!({"threads": 10, "operations": 100})],
                    expected_outcomes: vec![serde_json::json!({"all_completed": true, "data_consistent": true})],
                    setup_requirements: vec!["setup_concurrent_environment".to_string()],
                    cleanup_requirements: vec!["cleanup_threads".to_string()],
                },
                TestVariant {
                    name: "test_race_condition_prevention".to_string(),
                    scenario: TestScenario::ConcurrencyTest,
                    inputs: vec![serde_json::json!({"simultaneous_updates": true})],
                    expected_outcomes: vec![serde_json::json!({"race_condition_prevented": true})],
                    setup_requirements: vec!["setup_race_conditions".to_string()],
                    cleanup_requirements: vec![],
                },
            ],
        }
    }
    
    fn create_caching_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_caching", function_name),
            pattern_type: AdvancedPatternType::CachingOperation {
                cache_type: "memory".to_string(),
                eviction_policy: "lru".to_string(),
            },
            complexity_score: 0.7,
            test_variants: vec![
                TestVariant {
                    name: "test_cache_hit_miss".to_string(),
                    scenario: TestScenario::HappyPath,
                    inputs: vec![serde_json::json!({"key": "test_key", "value": "test_value"})],
                    expected_outcomes: vec![serde_json::json!({"cache_hit": true, "performance_improved": true})],
                    setup_requirements: vec!["setup_cache".to_string()],
                    cleanup_requirements: vec!["clear_cache".to_string()],
                },
                TestVariant {
                    name: "test_cache_eviction".to_string(),
                    scenario: TestScenario::EdgeCase,
                    inputs: vec![serde_json::json!({"cache_size_exceeded": true})],
                    expected_outcomes: vec![serde_json::json!({"oldest_evicted": true})],
                    setup_requirements: vec!["fill_cache_to_capacity".to_string()],
                    cleanup_requirements: vec!["clear_cache".to_string()],
                },
            ],
        }
    }
    
    fn create_security_pattern(&self, function_name: &str, _source_code: &str) -> AdvancedPattern {
        AdvancedPattern {
            name: format!("{}_security", function_name),
            pattern_type: AdvancedPatternType::SecurityOperation {
                operation: "authentication".to_string(),
                sensitive_data: true,
            },
            complexity_score: 0.9,
            test_variants: vec![
                TestVariant {
                    name: "test_security_valid_credentials".to_string(),
                    scenario: TestScenario::SecurityTest,
                    inputs: vec![serde_json::json!({"username": "test_user", "password": "valid_password"})],
                    expected_outcomes: vec![serde_json::json!({"authenticated": true, "token_issued": true})],
                    setup_requirements: vec!["setup_test_user".to_string()],
                    cleanup_requirements: vec!["cleanup_test_user".to_string()],
                },
                TestVariant {
                    name: "test_security_invalid_credentials".to_string(),
                    scenario: TestScenario::SecurityTest,
                    inputs: vec![serde_json::json!({"username": "test_user", "password": "wrong_password"})],
                    expected_outcomes: vec![serde_json::json!({"authenticated": false, "error": "invalid_credentials"})],
                    setup_requirements: vec![],
                    cleanup_requirements: vec![],
                },
                TestVariant {
                    name: "test_security_injection_attempts".to_string(),
                    scenario: TestScenario::SecurityTest,
                    inputs: vec![serde_json::json!({"username": "'; DROP TABLE users; --", "password": "password"})],
                    expected_outcomes: vec![serde_json::json!({"injection_prevented": true, "authenticated": false})],
                    setup_requirements: vec!["setup_injection_protection".to_string()],
                    cleanup_requirements: vec![],
                },
            ],
        }
    }
}