use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::core::dynamic_adapter::{DynamicLanguageAdapter, LanguageConfig};
use crate::core::{TestGenerator, TestCase};

pub struct LanguageLoader {
    config_dir: String,
    loaded_configs: HashMap<String, LanguageConfig>,
}

impl LanguageLoader {
    pub fn new(config_dir: String) -> Self {
        Self {
            config_dir,
            loaded_configs: HashMap::new(),
        }
    }

    pub fn load_all_languages(&mut self) -> Result<HashMap<String, Box<dyn TestGenerator + Send + Sync>>> {
        let mut adapters = HashMap::new();
        
        // First, load built-in hardcoded adapters for backward compatibility
        self.load_builtin_adapters(&mut adapters);
        
        // Then, load dynamic JSON-configured adapters
        self.load_dynamic_adapters(&mut adapters)?;
        
        Ok(adapters)
    }

    fn load_builtin_adapters(&self, adapters: &mut HashMap<String, Box<dyn TestGenerator + Send + Sync>>) {
        // Load existing hardcoded adapters for backward compatibility
        adapters.insert("javascript".to_string(), Box::new(crate::adapters::JavaScriptAdapter::new()));
        adapters.insert("python".to_string(), Box::new(crate::adapters::PythonAdapter::new()));
        adapters.insert("rust".to_string(), Box::new(crate::adapters::RustAdapter::new()));
        adapters.insert("go".to_string(), Box::new(crate::adapters::GoAdapter::new()));
        adapters.insert("java".to_string(), Box::new(crate::adapters::JavaAdapter::new()));
    }

    fn load_dynamic_adapters(&mut self, adapters: &mut HashMap<String, Box<dyn TestGenerator + Send + Sync>>) -> Result<()> {
        if !Path::new(&self.config_dir).exists() {
            return Ok(()); // No config directory, skip dynamic loading
        }

        let entries = fs::read_dir(&self.config_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                match self.load_language_config(&path) {
                    Ok((language_name, adapter)) => {
                        println!("Loaded dynamic language adapter: {}", language_name);
                        adapters.insert(language_name, adapter);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to load language config from {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(())
    }

    fn load_language_config(&mut self, path: &Path) -> Result<(String, Box<dyn TestGenerator + Send + Sync>)> {
        let content = fs::read_to_string(path)?;
        let config: LanguageConfig = serde_json::from_str(&content)?;
        let language_name = config.name.clone();
        
        // Validate the configuration
        self.validate_config(&config)?;
        
        // Store for future reference
        self.loaded_configs.insert(language_name.clone(), config.clone());
        
        let adapter = DynamicLanguageAdapter::new(config);
        Ok((language_name, Box::new(adapter)))
    }

    fn validate_config(&self, config: &LanguageConfig) -> Result<()> {
        if config.name.is_empty() {
            return Err(anyhow::anyhow!("Language name cannot be empty"));
        }
        
        if config.extensions.is_empty() {
            return Err(anyhow::anyhow!("Language must have at least one file extension"));
        }
        
        if config.patterns.is_empty() {
            return Err(anyhow::anyhow!("Language must have at least one pattern configuration"));
        }
        
        // Validate regex patterns
        for pattern in &config.patterns {
            regex::Regex::new(&pattern.regex)
                .map_err(|e| anyhow::anyhow!("Invalid regex pattern '{}': {}", pattern.regex, e))?;
        }
        
        Ok(())
    }

    pub fn get_supported_extensions(&self) -> HashMap<String, String> {
        let mut extensions = HashMap::new();
        
        // Add built-in extensions
        extensions.insert("js".to_string(), "javascript".to_string());
        extensions.insert("jsx".to_string(), "javascript".to_string());
        extensions.insert("ts".to_string(), "javascript".to_string());
        extensions.insert("tsx".to_string(), "javascript".to_string());
        extensions.insert("py".to_string(), "python".to_string());
        extensions.insert("rs".to_string(), "rust".to_string());
        extensions.insert("go".to_string(), "go".to_string());
        extensions.insert("java".to_string(), "java".to_string());
        
        // Add dynamic extensions
        for config in self.loaded_configs.values() {
            for ext in &config.extensions {
                extensions.insert(ext.clone(), config.name.clone());
            }
        }
        
        extensions
    }

    pub fn get_test_file_extension(&self, language: &str) -> String {
        if let Some(config) = self.loaded_configs.get(language) {
            config.test_template.file_extension.clone()
        } else {
            // Fallback to built-in extensions
            match language {
                "javascript" => ".test.js".to_string(),
                "python" => ".py".to_string(),
                "rust" => ".rs".to_string(),
                "go" => "_test.go".to_string(),
                "java" => "Test.java".to_string(),
                _ => ".txt".to_string(),
            }
        }
    }

    pub fn generate_test_content(&self, language: &str, test_cases: &[TestCase]) -> Result<String> {
        if let Some(config) = self.loaded_configs.get(language) {
            let adapter = DynamicLanguageAdapter::new(config.clone());
            Ok(adapter.generate_test_content_from_cases(test_cases))
        } else {
            Err(anyhow::anyhow!("No dynamic configuration found for language: {}", language))
        }
    }

    pub fn list_available_languages(&self) -> Vec<String> {
        let mut languages = vec![
            "javascript".to_string(),
            "python".to_string(), 
            "rust".to_string(),
            "go".to_string(),
            "java".to_string(),
        ];
        
        for config in self.loaded_configs.values() {
            if !languages.contains(&config.name) {
                languages.push(config.name.clone());
            }
        }
        
        languages.sort();
        languages
    }
}

// Extension to DynamicLanguageAdapter to support test content generation
impl DynamicLanguageAdapter {
    pub fn generate_test_content_from_cases(&self, test_cases: &[TestCase]) -> String {
        self.generate_test_content(test_cases)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config() -> LanguageConfig {
        use crate::core::dynamic_adapter::*;
        use std::collections::HashMap;

        LanguageConfig {
            name: "kotlin".to_string(),
            extensions: vec!["kt".to_string()],
            framework: "junit".to_string(),
            patterns: vec![
                PatternConfig {
                    name: "function".to_string(),
                    pattern_type: "function".to_string(),
                    regex: r"fun\s+(\w+)\s*\(([^)]*)\)(?:\s*:\s*(\w+))?".to_string(),
                    capture_groups: CaptureGroups {
                        name: Some(1),
                        parameters: Some(2),
                        return_type: Some(3),
                        parameter_separator: ",".to_string(),
                        parameter_format: "name_type".to_string(),
                    },
                    confidence: 0.9,
                }
            ],
            test_template: TestTemplate {
                setup: Some("import org.junit.*\nimport org.junit.Assert.*".to_string()),
                test_function: "@Test\nfun {{TEST_NAME}}() {\n    // {{TEST_DESCRIPTION}}\n    // TODO: Implement test logic\n}".to_string(),
                teardown: None,
                file_extension: "Test.kt".to_string(),
                placeholders: HashMap::new(),
            },
            imports: vec!["org.junit.*".to_string()],
        }
    }

    #[test]
    fn test_language_loader_creation() {
        let loader = LanguageLoader::new("./configs".to_string());
        assert_eq!(loader.config_dir, "./configs");
    }

    #[test]
    fn test_load_builtin_adapters() {
        let mut loader = LanguageLoader::new("./nonexistent".to_string());
        let adapters = loader.load_all_languages().unwrap();
        
        // Should load all 5 built-in adapters
        assert!(adapters.contains_key("javascript"));
        assert!(adapters.contains_key("python"));
        assert!(adapters.contains_key("rust"));
        assert!(adapters.contains_key("go"));
        assert!(adapters.contains_key("java"));
    }

    #[test]
    fn test_load_dynamic_adapter() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("kotlin.json");
        
        let config = create_test_config();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).unwrap();
        
        let mut loader = LanguageLoader::new(temp_dir.path().to_string_lossy().to_string());
        let adapters = loader.load_all_languages().unwrap();
        
        // Should have built-ins plus the dynamic Kotlin adapter
        assert!(adapters.contains_key("kotlin"));
        assert_eq!(adapters.len(), 6); // 5 built-ins + 1 dynamic
    }

    #[test]
    fn test_get_supported_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("kotlin.json");
        
        let config = create_test_config();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).unwrap();
        
        let mut loader = LanguageLoader::new(temp_dir.path().to_string_lossy().to_string());
        loader.load_all_languages().unwrap();
        
        let extensions = loader.get_supported_extensions();
        assert!(extensions.contains_key("kt"));
        assert_eq!(extensions.get("kt"), Some(&"kotlin".to_string()));
    }

    #[test]
    fn test_validate_config() {
        let loader = LanguageLoader::new("./test".to_string());
        
        // Valid config
        let valid_config = create_test_config();
        assert!(loader.validate_config(&valid_config).is_ok());
        
        // Invalid config - empty name
        let mut invalid_config = create_test_config();
        invalid_config.name = "".to_string();
        assert!(loader.validate_config(&invalid_config).is_err());
        
        // Invalid config - no extensions
        let mut invalid_config = create_test_config();
        invalid_config.extensions = vec![];
        assert!(loader.validate_config(&invalid_config).is_err());
        
        // Invalid config - no patterns
        let mut invalid_config = create_test_config();
        invalid_config.patterns = vec![];
        assert!(loader.validate_config(&invalid_config).is_err());
    }

    #[test]
    fn test_get_test_file_extension() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("kotlin.json");
        
        let config = create_test_config();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).unwrap();
        
        let mut loader = LanguageLoader::new(temp_dir.path().to_string_lossy().to_string());
        loader.load_all_languages().unwrap();
        
        // Dynamic language
        assert_eq!(loader.get_test_file_extension("kotlin"), "Test.kt");
        
        // Built-in language
        assert_eq!(loader.get_test_file_extension("go"), "_test.go");
    }

    #[test]
    fn test_list_available_languages() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("kotlin.json");
        
        let config = create_test_config();
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::write(&config_path, config_json).unwrap();
        
        let mut loader = LanguageLoader::new(temp_dir.path().to_string_lossy().to_string());
        loader.load_all_languages().unwrap();
        
        let languages = loader.list_available_languages();
        assert!(languages.contains(&"kotlin".to_string()));
        assert!(languages.contains(&"go".to_string()));
        assert!(languages.contains(&"java".to_string()));
        assert_eq!(languages.len(), 6);
    }
}