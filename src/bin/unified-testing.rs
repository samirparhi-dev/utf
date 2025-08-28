use anyhow::Result;
use clap::{Parser, Subcommand};
use unified_test_framework::{TestOrchestrator, LanguageLoader};
use std::fs;

#[derive(Parser)]
#[command(name = "unified-testing")]
#[command(about = "A unified test generation framework for multiple languages")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate tests for a file
    Generate {
        /// Path to the file to analyze
        path: String,
        /// Output directory for generated tests
        #[arg(short, long, default_value = "tests/")]
        output: String,
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
    },
    /// Analyze code patterns in a file
    Analyze {
        /// Path to the file to analyze
        path: String,
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
    },
    /// Build IDE plugins
    Plugin {
        /// Plugin type to build
        #[arg(value_enum)]
        plugin_type: PluginType,
        /// Output directory for built plugins
        #[arg(short, long, default_value = "target/plugins")]
        output: String,
    },
    /// List all available languages (built-in and dynamically loaded)
    Languages {
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum PluginType {
    Zed,
    Vscode,
    Spring,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { path, output, config_dir } => {
            // Load languages dynamically
            let mut loader = LanguageLoader::new(config_dir.clone());
            let adapters = loader.load_all_languages()?;
            
            let mut orchestrator = TestOrchestrator::new();
            for (lang, adapter) in adapters {
                orchestrator.register_adapter(lang, adapter);
            }
            println!("Generating tests for: {path}");
            
            let content = fs::read_to_string(&path)?;
            let test_suite = orchestrator.generate_tests_for_file(&path, &content).await?;
            
            println!("Generated {} test cases", test_suite.test_cases.len());
            
            // Create output directory
            fs::create_dir_all(&output)?;
            
            // Generate test file content based on language
            let test_content = generate_test_file_content(&test_suite)?;
            
            let output_file = format!("{}/test_{}.{}", 
                output, 
                test_suite.name.replace(" ", "_").to_lowercase(),
                get_test_file_extension(&test_suite.language)
            );
            
            fs::write(&output_file, test_content)?;
            println!("Tests written to: {output_file}");
        }
        Commands::Analyze { path, config_dir } => {
            // Load languages dynamically
            let mut loader = LanguageLoader::new(config_dir.clone());
            let adapters = loader.load_all_languages()?;
            
            let mut orchestrator = TestOrchestrator::new();
            for (lang, adapter) in adapters {
                orchestrator.register_adapter(lang, adapter);
            }
            
            println!("Analyzing patterns in: {path}");
            
            let content = fs::read_to_string(&path)?;
            let patterns = orchestrator.analyze_file(&path, &content).await?;
            
            println!("Found {} patterns:", patterns.len());
            for pattern in patterns {
                println!("- {} ({:?}) at line {}", pattern.id, pattern.pattern_type, pattern.location.line);
                println!("  Context: {:?}", pattern.context);
                println!("  Confidence: {:.2}", pattern.confidence);
            }
        }
        Commands::Plugin { plugin_type, output } => {
            println!("Building {:?} plugin...", plugin_type);
            
            // Create output directory
            fs::create_dir_all(&output)?;
            
            match plugin_type {
                PluginType::Zed => build_zed_plugin(&output).await?,
                PluginType::Vscode => build_vscode_plugin(&output).await?,
                PluginType::Spring => build_spring_plugin(&output).await?,
            }
            
            println!("Plugin built successfully in: {output}");
        }
        Commands::Languages { config_dir } => {
            // Load languages dynamically
            let mut loader = LanguageLoader::new(config_dir.clone());
            loader.load_all_languages()?;
            
            let languages = loader.list_available_languages();
            let extensions = loader.get_supported_extensions();
            
            println!("Available Languages:");
            println!("===================");
            
            for language in languages {
                let lang_extensions: Vec<String> = extensions
                    .iter()
                    .filter(|(_, lang)| *lang == &language)
                    .map(|(ext, _)| format!(".{}", ext))
                    .collect();
                
                let test_ext = loader.get_test_file_extension(&language);
                
                println!("📝 {} ", language.to_uppercase());
                println!("   Extensions: {}", lang_extensions.join(", "));
                println!("   Test files: {}", test_ext);
                println!();
            }
            
            println!("💡 To add a new language, create a JSON config file in: {}", config_dir);
            println!("   Example: {}/mylang.json", config_dir);
        }
    }

    Ok(())
}

async fn build_zed_plugin(output_dir: &str) -> Result<()> {
    let plugin_dir = format!("{}/zed-unified-testing", output_dir);
    fs::create_dir_all(&plugin_dir)?;
    
    // Create extension.toml
    let extension_toml = r#"id = "unified-testing"
name = "Unified Testing Framework"
description = "Automatic test generation for multiple languages"
version = "0.1.0"
schema_version = 1

[author]
name = "Unified Testing Team"
email = "team@unified-testing.dev"

[[grammars]]
name = "unified-testing"

[[languages]]
name = "JavaScript"
extensions = ["js", "jsx"]

[[languages]]  
name = "TypeScript"
extensions = ["ts", "tsx"]

[[languages]]
name = "Python" 
extensions = ["py"]

[[languages]]
name = "Rust"
extensions = ["rs"]
"#;
    
    fs::write(format!("{}/extension.toml", plugin_dir), extension_toml)?;
    
    // Create Cargo.toml for the plugin
    let cargo_toml = r#"[package]
name = "zed-unified-testing"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
"#;
    
    fs::write(format!("{}/Cargo.toml", plugin_dir), cargo_toml)?;
    
    // Create the plugin source
    let plugin_source = include_str!("../../plugins/zed/src/lib.rs");
    fs::create_dir_all(format!("{}/src", plugin_dir))?;
    fs::write(format!("{}/src/lib.rs", plugin_dir), plugin_source)?;
    
    // Build the plugin
    std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&plugin_dir)
        .output()?;
        
    println!("Zed plugin created at: {}", plugin_dir);
    Ok(())
}

async fn build_vscode_plugin(output_dir: &str) -> Result<()> {
    let plugin_dir = format!("{}/vscode-unified-testing", output_dir);
    fs::create_dir_all(&plugin_dir)?;
    
    // Create package.json
    let package_json = r#"{
  "name": "unified-testing",
  "displayName": "Unified Testing Framework", 
  "description": "Automatic test generation for multiple languages",
  "version": "0.1.0",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Testing", "Other"],
  "activationEvents": [
    "onLanguage:javascript",
    "onLanguage:typescript", 
    "onLanguage:python",
    "onLanguage:rust"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "unifiedTesting.generateTests",
        "title": "Generate Tests",
        "category": "Unified Testing"
      },
      {
        "command": "unifiedTesting.analyzeFile",
        "title": "Analyze File",
        "category": "Unified Testing"  
      }
    ],
    "menus": {
      "explorer/context": [
        {
          "command": "unifiedTesting.generateTests",
          "group": "unifiedTesting"
        }
      ]
    },
    "configuration": {
      "title": "Unified Testing",
      "properties": {
        "unifiedTesting.outputDirectory": {
          "type": "string",
          "default": "tests/",
          "description": "Output directory for generated tests"
        }
      }
    }
  },
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "package": "vsce package"
  },
  "devDependencies": {
    "@types/vscode": "^1.60.0",
    "@types/node": "16.x",
    "typescript": "^4.9.4",
    "vsce": "^2.15.0"
  }
}"#;
    
    fs::write(format!("{}/package.json", plugin_dir), package_json)?;
    
    // Create TypeScript config
    let tsconfig = r#"{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2020", 
    "outDir": "out",
    "lib": ["ES2020"],
    "sourceMap": true,
    "rootDir": "src",
    "strict": true
  },
  "exclude": ["node_modules", ".vscode-test"]
}"#;
    
    fs::write(format!("{}/tsconfig.json", plugin_dir), tsconfig)?;
    
    // Create the extension source
    let extension_source = include_str!("../../plugins/vscode/src/extension.ts");
    fs::create_dir_all(format!("{}/src", plugin_dir))?;
    fs::write(format!("{}/src/extension.ts", plugin_dir), extension_source)?;
    
    println!("VSCode extension created at: {}", plugin_dir);
    println!("Run 'npm install && npm run compile && npm run package' in the plugin directory to build");
    Ok(())
}

async fn build_spring_plugin(output_dir: &str) -> Result<()> {
    let plugin_dir = format!("{}/spring-unified-testing", output_dir);
    fs::create_dir_all(&plugin_dir)?;
    
    // Create plugin.xml
    let plugin_xml = r#"<idea-plugin>
  <id>com.unified-testing.spring-plugin</id>
  <name>Unified Testing Framework</name>
  <version>0.1.0</version>
  <vendor email="team@unified-testing.dev" url="https://unified-testing.dev">Unified Testing Team</vendor>

  <description><![CDATA[
    Automatic test generation framework for multiple programming languages.
    Supports JavaScript, TypeScript, Python, and Rust with intelligent pattern detection.
  ]]></description>

  <depends>com.intellij.modules.platform</depends>
  <depends>com.intellij.modules.java</depends>

  <extensions defaultExtensionNs="com.intellij">
    <projectService serviceImplementation="com.unified.testing.services.UnifiedTestingService"/>
    <toolWindow id="Unified Testing" secondary="true" anchor="bottom"
                factoryClass="com.unified.testing.ui.UnifiedTestingToolWindowFactory"/>
  </extensions>

  <actions>
    <group id="UnifiedTesting.Menu" text="Unified Testing" description="Unified Testing Framework">
      <add-to-group group-id="EditorPopupMenu" anchor="last"/>
      <action id="UnifiedTesting.GenerateTests" class="com.unified.testing.actions.GenerateTestsAction"
              text="Generate Tests" description="Generate tests for current file"/>
      <action id="UnifiedTesting.AnalyzeFile" class="com.unified.testing.actions.AnalyzeFileAction"
              text="Analyze File" description="Analyze current file for testable patterns"/>
    </group>
  </actions>

</idea-plugin>"#;
    
    fs::write(format!("{}/plugin.xml", plugin_dir), plugin_xml)?;
    
    // Create build.gradle.kts
    let build_gradle = r#"plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.10"
    id("org.jetbrains.intellij") version "1.15.0"
}

group = "com.unified-testing"
version = "0.1.0"

repositories {
    mavenCentral()
}

intellij {
    version.set("2023.2")
    type.set("IC")
    
    plugins.set(listOf("java"))
}

dependencies {
    implementation("com.fasterxml.jackson.core:jackson-databind:2.15.2")
    implementation("com.fasterxml.jackson.module:jackson-module-kotlin:2.15.2")
}

tasks {
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    patchPluginXml {
        sinceBuild.set("232")
        untilBuild.set("241.*")
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("PUBLISH_TOKEN"))
    }
}
"#;
    
    fs::write(format!("{}/build.gradle.kts", plugin_dir), build_gradle)?;
    
    // Create source directories
    fs::create_dir_all(format!("{}/src/main/java/com/unified/testing", plugin_dir))?;
    fs::create_dir_all(format!("{}/src/main/resources/META-INF", plugin_dir))?;
    
    // Copy plugin.xml to META-INF
    let plugin_xml_source = include_str!("../../plugins/spring/src/main/resources/META-INF/plugin.xml");
    fs::write(
        format!("{}/src/main/resources/META-INF/plugin.xml", plugin_dir),
        plugin_xml_source
    )?;
    
    // Copy the main plugin class
    let main_plugin_source = include_str!("../../plugins/spring/src/main/java/com/unified/testing/UnifiedTestingPlugin.java");
    fs::write(
        format!("{}/src/main/java/com/unified/testing/UnifiedTestingPlugin.java", plugin_dir),
        main_plugin_source
    )?;
    
    println!("Spring IDE plugin created at: {}", plugin_dir);
    println!("Run './gradlew buildPlugin' in the plugin directory to build");
    Ok(())
}

fn generate_test_file_content(test_suite: &unified_test_framework::TestSuite) -> Result<String> {
    let mut content = String::new();
    
    // Add imports only for non-Rust languages
    if test_suite.language != "rust" {
        for import in &test_suite.imports {
            content.push_str(import);
            content.push('\n');
        }
        content.push('\n');
    }
    
    // Add test cases based on language
    match test_suite.language.as_str() {
        "javascript" => {
            content.push_str("describe('Generated Tests', () => {\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "  test('{}', () => {{\n    // {}\n    // TODO: Implement test logic\n  }});\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("});\n");
        }
        "python" => {
            content.push_str("class TestGenerated:\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    def {}(self):\n        \"\"\" {} \"\"\"\n        # TODO: Implement test logic\n        pass\n\n",
                    test_case.name, test_case.description
                ));
            }
        }
        "rust" => {
            content.push_str("#[cfg(test)]\nmod tests {\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    #[test]\n    fn {}() {{\n        // {}\n        // TODO: Implement test logic\n    }}\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("}\n");
        }
        "go" => {
            content.push_str("package main\n\nimport (\n\t\"testing\"\n)\n\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "func {}(t *testing.T) {{\n\t// {}\n\t// TODO: Implement test logic\n}}\n\n",
                    test_case.name, test_case.description
                ));
            }
        }
        "java" => {
            content.push_str("import org.junit.*;\nimport static org.junit.Assert.*;\n\n");
            content.push_str(&format!("public class {}Test {{\n\n", 
                test_suite.name.replace("Test", "")));
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    @Test\n    public void {}() {{\n        // {}\n        // TODO: Implement test logic\n    }}\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("}\n");
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported language: {}", test_suite.language));
        }
    }
    
    Ok(content)
}

fn get_test_file_extension(language: &str) -> &str {
    match language {
        "javascript" => "test.js",
        "python" => "py",
        "rust" => "rs",
        "go" => "_test.go",
        "java" => "Test.java",
        _ => "txt",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unified_test_framework::{TestSuite, TestCase};

    fn create_test_suite(language: &str, framework: &str, test_cases: Vec<TestCase>) -> TestSuite {
        TestSuite {
            name: format!("Test Suite for {}", language),
            language: language.to_string(),
            framework: framework.to_string(),
            test_cases,
            imports: match language {
                "javascript" => vec!["const { expect } = require('@jest/globals');".to_string()],
                "python" => vec!["import pytest".to_string()],
                _ => vec![],
            },
        }
    }

    fn create_test_case(id: &str, name: &str, description: &str) -> TestCase {
        TestCase {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            input: serde_json::json!({}),
            expected_output: serde_json::json!({}),
        }
    }

    #[test]
    fn test_generate_test_file_content_javascript() {
        let test_case = create_test_case("test-1", "test_function", "Test a function");
        let test_suite = create_test_suite("javascript", "jest", vec![test_case]);

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("const { expect } = require('@jest/globals');"));
        assert!(content.contains("describe('Generated Tests', () => {"));
        assert!(content.contains("test('test_function', () => {"));
        assert!(content.contains("// Test a function"));
        assert!(content.contains("// TODO: Implement test logic"));
    }

    #[test]
    fn test_generate_test_file_content_python() {
        let test_case = create_test_case("test-2", "test_calculate", "Test calculation");
        let test_suite = create_test_suite("python", "pytest", vec![test_case]);

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("import pytest"));
        assert!(content.contains("class TestGenerated:"));
        assert!(content.contains("def test_calculate(self):"));
        assert!(content.contains("\"\"\" Test calculation \"\"\""));
        assert!(content.contains("pass"));
    }

    #[test]
    fn test_generate_test_file_content_rust() {
        let test_case = create_test_case("test-3", "test_add", "Test addition");
        let test_suite = create_test_suite("rust", "cargo-test", vec![test_case]);

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(!content.contains("import")); // Rust has no imports
        assert!(content.contains("#[cfg(test)]"));
        assert!(content.contains("mod tests {"));
        assert!(content.contains("#[test]"));
        assert!(content.contains("fn test_add() {"));
        assert!(content.contains("// Test addition"));
    }

    #[test]
    fn test_generate_test_file_content_unsupported_language() {
        let test_case = create_test_case("test-4", "test_cpp", "Test C++");
        let test_suite = TestSuite {
            name: "Test Suite".to_string(),
            language: "cpp".to_string(),
            framework: "gtest".to_string(),
            test_cases: vec![test_case],
            imports: vec![],
        };

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported language: cpp"));
    }

    #[test]
    fn test_generate_test_file_content_empty_test_cases() {
        let test_suite = create_test_suite("javascript", "jest", vec![]);

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("describe('Generated Tests', () => {"));
        assert!(content.contains("});"));
        // Should not contain any test cases
        assert!(!content.contains("test('"));
    }

    #[tokio::test]
    async fn test_build_zed_plugin() {
        let temp_dir = std::env::temp_dir().join("test_zed_plugin");
        let result = build_zed_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        // The actual plugin directory is a subdirectory
        let plugin_dir = temp_dir.join("zed-unified-testing");
        assert!(plugin_dir.exists());
        
        // Check for key files
        let cargo_toml = plugin_dir.join("Cargo.toml");
        let extension_toml = plugin_dir.join("extension.toml");
        let lib_rs = plugin_dir.join("src").join("lib.rs");
        
        assert!(cargo_toml.exists());
        assert!(extension_toml.exists());
        assert!(lib_rs.exists());
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_build_vscode_plugin() {
        let temp_dir = std::env::temp_dir().join("test_vscode_plugin");
        let result = build_vscode_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        // The actual plugin directory is a subdirectory
        let plugin_dir = temp_dir.join("vscode-unified-testing");
        assert!(plugin_dir.exists());
        
        // Check for key files
        let package_json = plugin_dir.join("package.json");
        let tsconfig_json = plugin_dir.join("tsconfig.json");
        let extension_ts = plugin_dir.join("src").join("extension.ts");
        
        assert!(package_json.exists());
        assert!(tsconfig_json.exists());
        assert!(extension_ts.exists());
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_build_spring_plugin() {
        let temp_dir = std::env::temp_dir().join("test_spring_plugin");
        let result = build_spring_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        // The actual plugin directory is a subdirectory
        let plugin_dir = temp_dir.join("spring-unified-testing");
        assert!(plugin_dir.exists());
        
        // Check for key files
        let build_gradle = plugin_dir.join("build.gradle.kts");
        let plugin_xml = plugin_dir.join("src").join("main").join("resources").join("META-INF").join("plugin.xml");
        let main_class = plugin_dir.join("src").join("main").join("java").join("com").join("unified").join("testing").join("UnifiedTestingPlugin.java");
        
        assert!(build_gradle.exists());
        assert!(plugin_xml.exists());
        assert!(main_class.exists());
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_plugin_type_enum() {
        // Test PluginType enum variants
        let zed = PluginType::Zed;
        let vscode = PluginType::Vscode;
        let spring = PluginType::Spring;
        
        // Just verify the variants exist and can be matched
        match zed {
            PluginType::Zed => assert!(true),
            _ => assert!(false),
        }
        
        match vscode {
            PluginType::Vscode => assert!(true),
            _ => assert!(false),
        }
        
        match spring {
            PluginType::Spring => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_cli_plugin_command_parsing() {
        // Test parsing plugin commands
        let args = vec!["unified-testing", "plugin", "zed", "--output", "./test"];
        let cli = Cli::try_parse_from(args);
        
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        
        match cli.command {
            Commands::Plugin { plugin_type, output } => {
                assert!(matches!(plugin_type, PluginType::Zed));
                assert_eq!(output, "./test");
            }
            _ => panic!("Expected Plugin command"),
        }
    }

    #[test]
    fn test_cli_plugin_command_default_output() {
        // Test parsing plugin commands with default output
        let args = vec!["unified-testing", "plugin", "vscode"];
        let cli = Cli::try_parse_from(args);
        
        assert!(cli.is_ok());
        let cli = cli.unwrap();
        
        match cli.command {
            Commands::Plugin { plugin_type, output } => {
                assert!(matches!(plugin_type, PluginType::Vscode));
                assert_eq!(output, "target/plugins");
            }
            _ => panic!("Expected Plugin command"),
        }
    }

    #[test]
    fn test_cli_plugin_command_all_types() {
        // Test all plugin types
        let plugin_types = vec!["zed", "vscode", "spring"];
        
        for plugin_type in plugin_types {
            let args = vec!["unified-testing", "plugin", plugin_type];
            let cli = Cli::try_parse_from(args);
            
            assert!(cli.is_ok(), "Failed to parse plugin type: {}", plugin_type);
            
            let cli = cli.unwrap();
            match cli.command {
                Commands::Plugin { plugin_type: pt, .. } => {
                    match plugin_type {
                        "zed" => assert!(matches!(pt, PluginType::Zed)),
                        "vscode" => assert!(matches!(pt, PluginType::Vscode)),
                        "spring" => assert!(matches!(pt, PluginType::Spring)),
                        _ => panic!("Unexpected plugin type"),
                    }
                }
                _ => panic!("Expected Plugin command"),
            }
        }
    }

    #[tokio::test]
    async fn test_build_zed_plugin_file_contents() {
        let temp_dir = std::env::temp_dir().join("test_zed_content");
        let result = build_zed_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        let plugin_dir = temp_dir.join("zed-unified-testing");
        
        // Check Cargo.toml content
        let cargo_toml_content = std::fs::read_to_string(plugin_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_toml_content.contains("[package]"));
        assert!(cargo_toml_content.contains("name = \"zed-unified-testing\""));
        assert!(cargo_toml_content.contains("crate-type = [\"cdylib\"]"));
        
        // Check extension.toml content
        let extension_toml_content = std::fs::read_to_string(plugin_dir.join("extension.toml")).unwrap();
        assert!(extension_toml_content.contains("id = \"unified-testing\""));
        assert!(extension_toml_content.contains("name = \"Unified Testing Framework\""));
        
        // Check lib.rs content
        let lib_rs_content = std::fs::read_to_string(plugin_dir.join("src").join("lib.rs")).unwrap();
        assert!(lib_rs_content.contains("UnifiedTestingExtension"));
        assert!(lib_rs_content.contains("zed::Extension"));
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_build_vscode_plugin_file_contents() {
        let temp_dir = std::env::temp_dir().join("test_vscode_content");
        let result = build_vscode_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        let plugin_dir = temp_dir.join("vscode-unified-testing");
        
        // Check package.json content
        let package_json_content = std::fs::read_to_string(plugin_dir.join("package.json")).unwrap();
        assert!(package_json_content.contains("\"name\": \"unified-testing\""));
        assert!(package_json_content.contains("\"unifiedTesting.generateTests\""));
        
        // Check extension.ts content
        let extension_ts_content = std::fs::read_to_string(plugin_dir.join("src").join("extension.ts")).unwrap();
        assert!(extension_ts_content.contains("export function activate"));
        assert!(extension_ts_content.contains("unified-testing.generateTests"));
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_build_spring_plugin_file_contents() {
        let temp_dir = std::env::temp_dir().join("test_spring_content");
        let result = build_spring_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        let plugin_dir = temp_dir.join("spring-unified-testing");
        
        // Check plugin.xml content
        let plugin_xml_content = std::fs::read_to_string(
            plugin_dir.join("src").join("main").join("resources").join("META-INF").join("plugin.xml")
        ).unwrap();
        assert!(plugin_xml_content.contains("<name>Unified Testing Framework</name>"));
        assert!(plugin_xml_content.contains("com.unified.testing.plugin"));
        
        // Check Java class content
        let java_content = std::fs::read_to_string(
            plugin_dir.join("src").join("main").join("java").join("com").join("unified").join("testing").join("UnifiedTestingPlugin.java")
        ).unwrap();
        assert!(java_content.contains("class UnifiedTestingPlugin"));
        assert!(java_content.contains("GenerateTestsAction"));
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[tokio::test]
    async fn test_build_plugin_success() {
        // Test that the plugin build actually succeeds in normal cases
        let temp_dir = std::env::temp_dir().join("test_plugin_success");
        let result = build_zed_plugin(&temp_dir.to_string_lossy()).await;
        
        assert!(result.is_ok());
        
        // Clean up
        std::fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_generate_test_file_content_multiple_test_cases() {
        let test_cases = vec![
            create_test_case("test-1", "test_func1", "Test function 1"),
            create_test_case("test-2", "test_func2", "Test function 2"),
        ];
        let test_suite = create_test_suite("python", "pytest", test_cases);

        let result = generate_test_file_content(&test_suite);
        assert!(result.is_ok());

        let content = result.unwrap();
        assert!(content.contains("def test_func1(self):"));
        assert!(content.contains("def test_func2(self):"));
        assert!(content.contains("\"\"\" Test function 1 \"\"\""));
        assert!(content.contains("\"\"\" Test function 2 \"\"\""));
    }

    #[test]
    fn test_get_test_file_extension_javascript() {
        assert_eq!(get_test_file_extension("javascript"), "test.js");
    }

    #[test]
    fn test_get_test_file_extension_python() {
        assert_eq!(get_test_file_extension("python"), "py");
    }

    #[test]
    fn test_get_test_file_extension_rust() {
        assert_eq!(get_test_file_extension("rust"), "rs");
    }

    #[test]
    fn test_get_test_file_extension_unknown() {
        assert_eq!(get_test_file_extension("unknown"), "txt");
    }

    #[test]
    fn test_get_test_file_extension_empty() {
        assert_eq!(get_test_file_extension(""), "txt");
    }

    // Test CLI structure (these won't execute main but test the structure)
    #[test]
    fn test_cli_command_structure() {
        use clap::Parser;
        
        // Test Generate command parsing
        let args = vec!["unified-testing", "generate", "test.js"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        
        if let Ok(cli) = cli {
            match cli.command {
                Commands::Generate { path, output } => {
                    assert_eq!(path, "test.js");
                    assert_eq!(output, "tests/");
                }
                _ => panic!("Expected Generate command"),
            }
        }
    }

    #[test]
    fn test_cli_analyze_command() {
        use clap::Parser;
        
        let args = vec!["unified-testing", "analyze", "src/main.py"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        
        if let Ok(cli) = cli {
            match cli.command {
                Commands::Analyze { path } => {
                    assert_eq!(path, "src/main.py");
                }
                _ => panic!("Expected Analyze command"),
            }
        }
    }

    #[test]
    fn test_cli_generate_command_with_output() {
        use clap::Parser;
        
        let args = vec!["unified-testing", "generate", "test.rs", "--output", "my-tests/"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        
        if let Ok(cli) = cli {
            match cli.command {
                Commands::Generate { path, output } => {
                    assert_eq!(path, "test.rs");
                    assert_eq!(output, "my-tests/");
                }
                _ => panic!("Expected Generate command"),
            }
        }
    }

    #[test]
    fn test_cli_invalid_command() {
        use clap::Parser;
        
        let args = vec!["unified-testing", "invalid"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_err());
    }

    #[test]
    fn test_cli_no_command() {
        use clap::Parser;
        
        let args = vec!["unified-testing"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_err());
    }
}