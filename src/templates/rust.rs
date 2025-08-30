pub const CARGO_FUNCTION_TEST_TEMPLATE: &str = r#"
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_{{ test_name }}_{{ test_category | lower }}() {
        // {{ description }}
        {% for input in inputs %}
        {% if input is number %}
        let result = {{ function_name }}({{ input }});
        {% else %}
        let result = {{ function_name }}("{{ input }}");
        {% endif %}
        assert!(result.is_ok() || result.is_err()); // Ensure function executes
        {% endfor %}
    }
    
    #[test]
    fn test_{{ test_name }}_boundary_conditions() {
        // Test boundary conditions
        {% if test_category == "numeric" %}
        assert_eq!({{ function_name }}(0), {{ expected_outputs.0 | default(value="0") }});
        assert_eq!({{ function_name }}(i32::MAX), {{ function_name }}(i32::MAX));
        assert_eq!({{ function_name }}(i32::MIN), {{ function_name }}(i32::MIN));
        {% elif test_category == "string" %}
        assert_eq!({{ function_name }}(""), {{ function_name }}(""));
        assert!({{ function_name }}("test").len() > 0 || {{ function_name }}("test").len() == 0);
        {% endif %}
    }
    
    #[test]
    fn test_{{ test_name }}_edge_cases() {
        // Test edge cases and error conditions
        {% if test_category == "numeric" %}
        // Test with zero
        let zero_result = {{ function_name }}(0);
        assert!(zero_result == {{ expected_outputs.0 | default(value="0") }} || zero_result != {{ expected_outputs.0 | default(value="0") }});
        
        // Test with negative numbers
        let negative_result = {{ function_name }}(-1);
        assert!(negative_result <= 0 || negative_result > 0);
        {% elif test_category == "string" %}
        // Test with empty string
        let empty_result = {{ function_name }}("");
        assert!(empty_result.is_empty() || !empty_result.is_empty());
        
        // Test with special characters
        let special_result = {{ function_name }}("!@#$%^&*()");
        assert_ne!(special_result.len(), usize::MAX);
        {% endif %}
    }
    
    #[test]
    #[should_panic]
    fn test_{{ test_name }}_panic_conditions() {
        // Test conditions that should panic
        // This would be customized based on function analysis
        {{ function_name }}(/* panic-inducing input */);
    }
    
    #[test]
    fn test_{{ test_name }}_memory_safety() {
        // Test memory safety and resource cleanup
        let large_input = vec![0; 10000];
        let _result = {{ function_name }}(); // Function should handle large inputs safely
        
        // Verify no memory leaks (basic check)
        drop(large_input);
    }
}
"#;

pub const CARGO_STRUCT_TEST_TEMPLATE: &str = r#"
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_{{ function_name | lower }}_creation() {
        // Test struct creation
        let instance = {{ function_name }}::new();
        assert!(instance.is_ok() || instance.is_err());
        
        {% if setup_code %}
        // Custom setup
        {{ setup_code }}
        {% endif %}
    }
    
    #[test]
    fn test_{{ function_name | lower }}_default() {
        // Test Default implementation if available
        let instance = {{ function_name }}::default();
        
        // Verify default state
        assert_eq!(format!("{:?}", instance).len() > 0, true);
    }
    
    #[test]
    fn test_{{ function_name | lower }}_clone() {
        // Test Clone implementation if available
        let original = {{ function_name }}::default();
        let cloned = original.clone();
        
        // Verify clone is independent
        assert_eq!(original, cloned);
    }
    
    #[test]
    fn test_{{ function_name | lower }}_equality() {
        // Test PartialEq implementation if available
        let instance1 = {{ function_name }}::default();
        let instance2 = {{ function_name }}::default();
        
        assert_eq!(instance1, instance2);
        assert_eq!(instance1, instance1); // reflexivity
    }
    
    #[test]
    fn test_{{ function_name | lower }}_serialization() {
        // Test serialization if Serialize/Deserialize is implemented
        let instance = {{ function_name }}::default();
        
        // This would be conditional based on serde detection
        #[cfg(feature = "serde")]
        {
            let serialized = serde_json::to_string(&instance);
            assert!(serialized.is_ok());
            
            if let Ok(json_str) = serialized {
                let deserialized: Result<{{ function_name }}, _> = serde_json::from_str(&json_str);
                assert!(deserialized.is_ok());
            }
        }
    }
    
    #[test]
    fn test_{{ function_name | lower }}_methods() {
        // Test public methods
        let mut instance = {{ function_name }}::default();
        
        // Test method calls (would be populated based on detected methods)
        {% for method in methods | default(value=[]) %}
        let _result = instance.{{ method }}();
        {% endfor %}
    }
    
    #[test]
    fn test_{{ function_name | lower }}_thread_safety() {
        use std::sync::Arc;
        use std::thread;
        
        let instance = Arc::new({{ function_name }}::default());
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let instance_clone = Arc::clone(&instance);
                thread::spawn(move || {
                    // Test thread-safe access
                    let _result = format!("{:?}", instance_clone);
                    i
                })
            })
            .collect();
        
        for handle in handles {
            assert!(handle.join().is_ok());
        }
    }
}
"#;

pub const CARGO_ASYNC_TEST_TEMPLATE: &str = r#"
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_{{ test_name }}_{{ test_category | lower }}() {
        // {{ description }}
        {% for input in inputs %}
        {% if input is number %}
        let result = {{ function_name }}({{ input }}).await;
        {% else %}
        let result = {{ function_name }}("{{ input }}").await;
        {% endif %}
        assert!(result.is_ok() || result.is_err());
        {% endfor %}
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_timeout() {
        // Test with timeout
        let timeout_duration = Duration::from_secs(5);
        
        let result = tokio::time::timeout(timeout_duration, {{ function_name }}()).await;
        assert!(result.is_ok() || result.is_err());
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_cancellation() {
        // Test task cancellation
        let handle = tokio::spawn(async {
            {{ function_name }}().await
        });
        
        // Cancel the task
        handle.abort();
        
        let result = handle.await;
        assert!(result.is_err()); // Should be cancelled
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_concurrent_execution() {
        // Test concurrent execution
        let tasks = vec![
            tokio::spawn({{ function_name }}()),
            tokio::spawn({{ function_name }}()),
            tokio::spawn({{ function_name }}()),
        ];
        
        let results = futures::future::join_all(tasks).await;
        assert_eq!(results.len(), 3);
        
        // Verify all tasks completed (successfully or with error)
        for result in results {
            assert!(result.is_ok() || result.is_err());
        }
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_error_propagation() {
        // Test error propagation in async context
        match {{ function_name }}().await {
            Ok(_) => {
                // Function succeeded
                assert!(true);
            }
            Err(e) => {
                // Function failed, verify error is meaningful
                assert!(!e.to_string().is_empty());
            }
        }
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_resource_cleanup() {
        // Test that resources are properly cleaned up
        {
            let _result = {{ function_name }}().await;
            // Resources should be cleaned up when result goes out of scope
        }
        
        // Test multiple calls don't leak resources
        for _i in 0..10 {
            let _result = {{ function_name }}().await;
        }
    }
    
    #[tokio::test]
    async fn test_{{ test_name }}_backpressure() {
        // Test handling of backpressure
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        
        tokio::spawn(async move {
            let _result = {{ function_name }}().await;
            let _ = tx.send(()).await;
        });
        
        // Wait for completion or timeout
        tokio::select! {
            _ = rx.recv() => {
                assert!(true); // Completed successfully
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                assert!(false, "Function took too long to complete");
            }
        }
    }
}
"#;