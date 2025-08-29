use anyhow::Result;
use clap::{Parser, Subcommand};
use unified_test_framework::{TestOrchestrator, LanguageLoader, IntegrationTestGenerator};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::io::{self, Write};
use git2::Repository;
use walkdir::WalkDir;

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
    /// Generate tests for all supported files in a directory
    Dir {
        /// Path to the directory to analyze
        path: String,
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
    },
    /// Generate integration tests for a file
    IntegrationTest {
        /// Path to the file to analyze for integration patterns
        path: String,
        /// Output directory for generated tests
        #[arg(short, long, default_value = "integration-tests/")]
        output: String,
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
    },
    /// Generate tests for all supported files in a Git repository
    GitRepo {
        /// Git repository URL
        url: String,
        /// Custom language configs directory
        #[arg(short, long, default_value = "./language_configs")]
        config_dir: String,
        /// Branch to checkout (default: main)
        #[arg(short, long, default_value = "main")]
        branch: String,
        /// Write tests to repository's standard test directories instead of separate folder
        #[arg(long, default_value = "true")]
        in_repo: bool,
    },
    /// Install and configure uft for system-wide use
    Install {
        /// Skip shell configuration (only install configs)
        #[arg(long)]
        skip_shell: bool,
        /// Force reinstall even if already configured
        #[arg(long)]
        force: bool,
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
            
            // Determine the proper test file path based on language conventions
            let source_path = Path::new(&path);
            let current_dir = std::env::current_dir()?;
            let output_file = get_test_file_path(&current_dir, source_path, &test_suite.language, &test_suite.framework)?;
            
            // Create output directory
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Generate test file content based on language
            let test_content = generate_test_file_content(&test_suite)?;
            
            fs::write(&output_file, test_content)?;
            println!("Tests written to: {}", output_file.display());
        }
        Commands::IntegrationTest { path, output, config_dir } => {
            // Load languages dynamically
            let mut loader = LanguageLoader::new(config_dir.clone());
            let adapters = loader.load_all_languages()?;
            
            let mut orchestrator = TestOrchestrator::new();
            for (lang, adapter) in adapters {
                orchestrator.register_adapter(lang, adapter);
            }
            println!("Generating integration tests for: {path}");
            
            let content = fs::read_to_string(&path)?;
            let language = orchestrator.detect_language(&path)?;
            
            // Check if the adapter supports integration tests
            if language == "javascript" {
                let js_adapter = unified_test_framework::JavaScriptAdapter::new();
                let patterns = js_adapter.analyze_integration_patterns(&content, &path).await?;
                
                if patterns.is_empty() {
                    println!("No integration patterns found in the file");
                    return Ok(());
                }
                
                println!("Found {} integration patterns", patterns.len());
                for pattern in &patterns {
                    println!("- {:?} (confidence: {:.2})", pattern.pattern_type, pattern.confidence);
                }
                
                let test_suite = js_adapter.generate_integration_tests(patterns).await?;
                println!("Generated {} integration test cases", test_suite.test_cases.len());
                
                // Create output directory
                let output_path = Path::new(&output);
                fs::create_dir_all(output_path)?;
                
                // Generate integration test file
                let source_path = Path::new(&path);
                let file_stem = source_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("test");
                
                let integration_test_file = output_path.join(format!("{}.integration.test.js", file_stem));
                let test_content = generate_integration_test_content(&test_suite)?;
                
                fs::write(&integration_test_file, test_content)?;
                
                println!("Integration tests written to: {}", integration_test_file.display());
                println!("\nSetup requirements:");
                for req in &test_suite.setup_requirements {
                    println!("  - {}", req);
                }
                println!("\nCleanup requirements:");
                for req in &test_suite.cleanup_requirements {
                    println!("  - {}", req);
                }
            } else {
                println!("Integration test generation not yet supported for language: {}", language);
                println!("Currently supported: JavaScript");
            }
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
            
            let builtin_languages = loader.list_builtin_languages();
            let dynamic_languages = loader.list_dynamic_languages();
            let extensions = loader.get_supported_extensions();
            
            // Collect language data for tabular display
            let mut language_data = Vec::new();
            
            // Add built-in languages
            for language in &builtin_languages {
                let frameworks = get_available_frameworks_for_display(&language);
                let test_ext = loader.get_test_file_extension(&language);
                language_data.push((
                    get_language_with_symbol(&language.to_uppercase()),
                    "Built-in".to_string(),
                    frameworks,
                    test_ext,
                ));
            }
            
            // Add dynamic languages
            for language in &dynamic_languages {
                let frameworks = get_available_frameworks_for_display(&language);
                let test_ext = loader.get_test_file_extension(&language);
                language_data.push((
                    get_language_with_symbol(&language.to_uppercase()),
                    "Dynamic".to_string(),
                    frameworks,
                    test_ext,
                ));
            }
            
            // Use fixed column widths optimized for ASCII content
            let lang_width = 16;      // Enough for "[JS] JAVASCRIPT"
            let type_width = 10;      // Enough for "Built-in"
            let framework_width = 24; // Enough for "cargo-test, nextest"
            let test_width = 12;      // Enough for "Tests.swift"
            
            // Print header
            println!("🚀 Supported Languages & Testing Frameworks");
            println!("═{:═<width$}═", "", width = lang_width + type_width + framework_width + test_width + 9);
            
            // Table header
            println!(
                "│ {:^lang_width$} │ {:^type_width$} │ {:^framework_width$} │ {:^test_width$} │",
                "LANGUAGE", "TYPE", "FRAMEWORKS", "TEST FORMAT",
                lang_width = lang_width,
                type_width = type_width,
                framework_width = framework_width,
                test_width = test_width
            );
            
            // Header separator
            println!(
                "├─{:─<lang_width$}─┼─{:─<type_width$}─┼─{:─<framework_width$}─┼─{:─<test_width$}─┤",
                "", "", "", "",
                lang_width = lang_width,
                type_width = type_width,
                framework_width = framework_width,
                test_width = test_width
            );
            
            // Print data rows
            for (lang, lang_type, frameworks_str, test_format) in language_data {
                println!(
                    "│ {:<lang_width$} │ {:<type_width$} │ {:<framework_width$} │ {:<test_width$} │",
                    lang, lang_type, frameworks_str, test_format,
                    lang_width = lang_width,
                    type_width = type_width,
                    framework_width = framework_width,
                    test_width = test_width
                );
            }
            
            // Bottom border
            println!("└─{:─<lang_width$}─┴─{:─<type_width$}─┴─{:─<framework_width$}─┴─{:─<test_width$}─┘",
                "", "", "", "",
                lang_width = lang_width,
                type_width = type_width,
                framework_width = framework_width,
                test_width = test_width
            );
            
            // Summary and help
            let total_builtin = builtin_languages.len();
            let total_dynamic = dynamic_languages.len();
            let total_languages = total_builtin + total_dynamic;
            
            println!();
            println!("📊 Summary: {} total languages ({} built-in, {} dynamic)", 
                total_languages, total_builtin, total_dynamic);
            println!("💡 Add new languages: Create JSON files in {}/", config_dir);
        }
        Commands::Install { skip_shell, force } => {
            println!("🚀 Installing Unified Test Framework...");
            
            // Install language configurations
            install_language_configs(force)?;
            
            // Configure shell if not skipped
            if !skip_shell {
                configure_shell_integration(force)?;
            }
            
            println!("✅ Installation completed successfully!");
            println!("\n📋 What was installed:");
            println!("   • Language configurations: ~/.config/uft/language_configs/");
            
            if !skip_shell {
                println!("   • Shell integration: Added to your shell profile");
                println!("\n🔄 Please restart your terminal or run:");
                println!("   source ~/.bashrc    # for bash");
                println!("   source ~/.zshrc     # for zsh");
            }
            
            println!("\n✨ You can now run 'uft languages' from anywhere!");
        }
        Commands::Dir { path, config_dir } => {
            let target_dir = Path::new(&path);
            
            if !target_dir.exists() {
                return Err(anyhow::anyhow!("Directory does not exist: {}", path));
            }
            
            if !target_dir.is_dir() {
                return Err(anyhow::anyhow!("Path is not a directory: {}", path));
            }
            
            println!("🔍 Scanning directory: {}", target_dir.display());
            
            // Load language adapters
            let mut loader = LanguageLoader::new(config_dir.clone());
            let adapters = loader.load_all_languages()?;
            let supported_extensions = get_supported_extensions(&loader);
            
            // Detect project languages and let user choose frameworks
            let project_languages = detect_project_languages(&target_dir, &supported_extensions)?;
            println!("🔍 Detected languages: {:?}", project_languages);
            
            let framework_choices = prompt_framework_choices(&project_languages)?;
            
            let mut orchestrator = TestOrchestrator::new();
            for (lang, adapter) in adapters {
                orchestrator.register_adapter(lang, adapter);
            }
            
            // Find all source files
            let source_files = find_source_files_excluding_tests(&target_dir, &supported_extensions)?;
            println!("📝 Found {} source files to test", source_files.len());
            
            let mut total_tests = 0;
            let mut processed_files = 0;
            let mut skipped_files = 0;
            
            // Process each file
            for file_path in source_files {
                let relative_path = file_path.strip_prefix(&target_dir)
                    .unwrap_or(&file_path)
                    .to_string_lossy();
                
                println!("🔍 Processing: {}", relative_path);
                
                // Determine language and framework
                let language = detect_file_language(&file_path, &supported_extensions, &loader)?;
                let framework = framework_choices.get(&language).cloned().unwrap_or_else(|| {
                    get_default_framework(&language)
                });
                
                // Check if test already exists
                let test_file_path = get_test_file_path(&target_dir, &file_path, &language, &framework)?;
                
                if test_file_path.exists() {
                    println!("  ⏭️  Test already exists: {}", test_file_path.display());
                    skipped_files += 1;
                    continue;
                }
                
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        match orchestrator.generate_tests_for_file(
                            &file_path.to_string_lossy(), 
                            &content
                        ).await {
                            Ok(mut test_suite) => {
                                if !test_suite.test_cases.is_empty() {
                                    // Update test suite with chosen framework
                                    test_suite.framework = framework.clone();
                                    
                                    let test_content = generate_test_file_content_with_framework(&test_suite, &framework)?;
                                    
                                    // Create test directory if needed
                                    if let Some(parent) = test_file_path.parent() {
                                        fs::create_dir_all(parent)?;
                                    }
                                    
                                    fs::write(&test_file_path, test_content)?;
                                    
                                    println!("  ✅ Generated {} tests -> {}", 
                                        test_suite.test_cases.len(), 
                                        test_file_path.strip_prefix(&target_dir)
                                            .unwrap_or(&test_file_path)
                                            .display()
                                    );
                                    
                                    total_tests += test_suite.test_cases.len();
                                    processed_files += 1;
                                } else {
                                    println!("  ⚠️  No testable patterns found");
                                }
                            }
                            Err(e) => println!("  ❌ Error generating tests: {}", e),
                        }
                    }
                    Err(e) => println!("  ❌ Error reading file: {}", e),
                }
            }
            
            println!("\n🎉 Test generation complete!");
            println!("📊 Summary:");
            println!("   • Processed files: {}", processed_files);
            println!("   • Skipped files (tests exist): {}", skipped_files);
            println!("   • Total test cases: {}", total_tests);
            println!("   • Directory: {}", target_dir.display());
            println!("\n💡 Next steps:");
            println!("   1. Review and implement test logic in generated files");
            println!("   2. Run tests with your project's test command");
        }
        Commands::GitRepo { url, config_dir, branch, in_repo } => {
            println!("🔄 Cloning repository: {}", url);
            
            // Clone repository to working directory (not temp)
            let repo_name = url.split('/').last().unwrap_or("repo").replace(".git", "");
            let repo_dir = Path::new(&repo_name);
            
            if repo_dir.exists() {
                fs::remove_dir_all(&repo_dir)?;
            }
            
            let _repo = Repository::clone(&url, &repo_dir)?;
            
            // Checkout specified branch if not main
            if branch != "main" {
                let repo = Repository::open(&repo_dir)?;
                let (object, reference) = repo.revparse_ext(&format!("origin/{}", branch))?;
                repo.checkout_tree(&object, None)?;
                match reference {
                    Some(gref) => repo.set_head(gref.name().unwrap())?,
                    None => repo.set_head_detached(object.id())?,
                }
            }
            
            println!("✅ Repository cloned to: {}", repo_dir.display());
            
            // Load language adapters
            let mut loader = LanguageLoader::new(config_dir.clone());
            let adapters = loader.load_all_languages()?;
            let supported_extensions = get_supported_extensions(&loader);
            
            // Detect project languages and let user choose frameworks
            let project_languages = detect_project_languages(&repo_dir, &supported_extensions)?;
            println!("🔍 Detected languages: {:?}", project_languages);
            
            let framework_choices = prompt_framework_choices(&project_languages)?;
            
            let mut orchestrator = TestOrchestrator::new();
            for (lang, adapter) in adapters {
                orchestrator.register_adapter(lang, adapter);
            }
            
            // Find all source files
            let source_files = find_source_files_excluding_tests(&repo_dir, &supported_extensions)?;
            println!("📝 Found {} source files to test", source_files.len());
            
            let mut total_tests = 0;
            let mut processed_files = 0;
            let mut skipped_files = 0;
            
            // Process each file
            for file_path in source_files {
                let relative_path = file_path.strip_prefix(&repo_dir)
                    .unwrap_or(&file_path)
                    .to_string_lossy();
                
                println!("🔍 Processing: {}", relative_path);
                
                // Determine language and framework
                let language = detect_file_language(&file_path, &supported_extensions, &loader)?;
                let framework = framework_choices.get(&language).cloned().unwrap_or_else(|| {
                    get_default_framework(&language)
                });
                
                // Check if test already exists
                let test_file_path = get_test_file_path(&repo_dir, &file_path, &language, &framework)?;
                
                if test_file_path.exists() {
                    println!("  ⏭️  Test already exists: {}", test_file_path.display());
                    skipped_files += 1;
                    continue;
                }
                
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        match orchestrator.generate_tests_for_file(
                            &file_path.to_string_lossy(), 
                            &content
                        ).await {
                            Ok(mut test_suite) => {
                                if !test_suite.test_cases.is_empty() {
                                    // Update test suite with chosen framework
                                    test_suite.framework = framework.clone();
                                    
                                    let test_content = generate_test_file_content_with_framework(&test_suite, &framework)?;
                                    
                                    // Create test directory if needed
                                    if let Some(parent) = test_file_path.parent() {
                                        fs::create_dir_all(parent)?;
                                    }
                                    
                                    fs::write(&test_file_path, test_content)?;
                                    
                                    println!("  ✅ Generated {} tests -> {}", 
                                        test_suite.test_cases.len(), 
                                        test_file_path.strip_prefix(&repo_dir)
                                            .unwrap_or(&test_file_path)
                                            .display()
                                    );
                                    
                                    total_tests += test_suite.test_cases.len();
                                    processed_files += 1;
                                } else {
                                    println!("  ⚠️  No testable patterns found");
                                }
                            }
                            Err(e) => println!("  ❌ Error generating tests: {}", e),
                        }
                    }
                    Err(e) => println!("  ❌ Error reading file: {}", e),
                }
            }
            
            println!("\n🎉 Test generation complete!");
            println!("📊 Summary:");
            println!("   • Processed files: {}", processed_files);
            println!("   • Skipped files (tests exist): {}", skipped_files);
            println!("   • Total test cases: {}", total_tests);
            println!("   • Repository: {}", repo_dir.display());
            println!("\n💡 Next steps:");
            println!("   1. cd {}", repo_dir.display());
            println!("   2. Review and implement test logic in generated files");
            println!("   3. Run tests with your project's test command");
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

/// Get supported file extensions from the language loader
fn get_supported_extensions(loader: &LanguageLoader) -> Vec<String> {
    loader.get_supported_extensions()
        .keys()
        .cloned()
        .collect()
}

/// Find all source files with supported extensions in a directory
fn find_source_files(dir: &Path, supported_extensions: &[String]) -> Result<Vec<std::path::PathBuf>> {
    let mut source_files = Vec::new();
    
    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories, hidden files, and common non-source directories
        if path.is_dir() || is_ignored_path(path) {
            continue;
        }
        
        // Check if file has supported extension
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if supported_extensions.contains(&ext_str) {
                source_files.push(path.to_path_buf());
            }
        }
    }
    
    Ok(source_files)
}

/// Check if a path should be ignored (common non-source directories)
fn is_ignored_path(path: &Path) -> bool {
    let ignored_dirs = [
        "node_modules", "target", "build", "dist", "out", ".git", 
        ".svn", ".hg", "__pycache__", ".pytest_cache", "vendor",
        "deps", "_build", ".gradle", ".mvn", "bin", "obj"
    ];
    
    let ignored_files = [
        ".gitignore", ".dockerignore", "Dockerfile", "README.md",
        "LICENSE", "CHANGELOG.md", "package-lock.json", "Cargo.lock"
    ];
    
    // Check if any parent directory is in ignored list
    for ancestor in path.ancestors() {
        if let Some(name) = ancestor.file_name() {
            let name_str = name.to_string_lossy();
            if ignored_dirs.iter().any(|&ignored| name_str == ignored) {
                return true;
            }
        }
    }
    
    // Check if filename is in ignored list
    if let Some(filename) = path.file_name() {
        let filename_str = filename.to_string_lossy();
        if ignored_files.iter().any(|&ignored| filename_str == ignored) {
            return true;
        }
    }
    
    false
}

/// Install language configurations to user config directory
fn install_language_configs(force: bool) -> Result<()> {
    let home = std::env::var("HOME")?;
    let config_dir = format!("{}/.config/uft", home);
    let target_dir = format!("{}/language_configs", config_dir);
    
    // Create config directory
    std::fs::create_dir_all(&config_dir)?;
    
    if Path::new(&target_dir).exists() && !force {
        println!("   ⚠️  Language configs already exist (use --force to reinstall)");
        return Ok(());
    }
    
    // Remove existing if force is true
    if Path::new(&target_dir).exists() {
        std::fs::remove_dir_all(&target_dir)?;
    }
    
    // Find source configs - look in multiple locations
    let possible_sources = vec![
        "./language_configs",
        "../language_configs",
        "/usr/local/share/uft/language_configs",
    ];
    
    let mut source_dir = None;
    for src in possible_sources {
        if Path::new(src).exists() {
            source_dir = Some(src);
            break;
        }
    }
    
    match source_dir {
        Some(src) => {
            // Copy configs
            copy_dir_all(src, &target_dir)?;
            println!("   ✅ Language configurations installed");
        }
        None => {
            println!("   ⚠️  Language configs not found, creating basic structure");
            std::fs::create_dir_all(&target_dir)?;
        }
    }
    
    Ok(())
}

/// Configure shell integration (add uft to PATH if needed)
fn configure_shell_integration(force: bool) -> Result<()> {
    let home = std::env::var("HOME")?;
    
    // Check if uft is already in PATH
    if !force {
        if let Ok(_) = std::process::Command::new("which").arg("uft").output() {
            println!("   ✅ uft already available in PATH");
            return Ok(());
        }
    }
    
    // Detect shell and configure
    let shell = std::env::var("SHELL").unwrap_or_default();
    let shell_name = Path::new(&shell).file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("bash");
    
    let config_file = match shell_name {
        "zsh" => format!("{}/.zshrc", home),
        "fish" => format!("{}/.config/fish/config.fish", home),
        _ => format!("{}/.bashrc", home), // default to bashrc
    };
    
    // Add cargo bin to PATH if not already there
    let cargo_bin_path = format!("{}/.cargo/bin", home);
    let path_export = match shell_name {
        "fish" => format!("set -gx PATH $PATH {}", cargo_bin_path),
        _ => format!("export PATH=\"$PATH:{}\"", cargo_bin_path),
    };
    
    // Check if already configured
    if Path::new(&config_file).exists() {
        let content = std::fs::read_to_string(&config_file)?;
        if content.contains(".cargo/bin") && !force {
            println!("   ✅ Shell already configured");
            return Ok(());
        }
    }
    
    // Add to shell config
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config_file)?;
    
    writeln!(file, "")?;
    writeln!(file, "# Added by Unified Test Framework installer")?;
    writeln!(file, "{}", path_export)?;
    
    println!("   ✅ Shell configuration updated: {}", config_file);
    
    Ok(())
}

/// Copy directory recursively
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Get available frameworks for display in table format
fn get_available_frameworks_for_display(language: &str) -> String {
    let frameworks = get_available_frameworks(language);
    frameworks.join(", ")
}

/// Get language name with appropriate symbol for better identification
fn get_language_with_symbol(language: &str) -> String {
    let symbol = match language.to_lowercase().as_str() {
        "java" => "[J]",
        "javascript" => "[JS]",
        "typescript" => "[TS]", 
        "python" => "[PY]",
        "rust" => "[RS]",
        "go" => "[GO]",
        "php" => "[PHP]",
        "csharp" => "[C#]",
        "kotlin" => "[KT]",
        "swift" => "[SW]",
        _ => "[ ]",
    };
    format!("{} {}", symbol, language)
}

/// Calculate visual width accounting for emojis (approximate)
fn visual_width(text: &str) -> usize {
    // Simple approximation: each emoji counts as 2 characters for alignment
    let emoji_count = text.chars().filter(|c| *c as u32 > 0x1F000).count();
    text.chars().count() + emoji_count
}

/// Detect which programming languages are present in the repository
fn detect_project_languages(repo_dir: &Path, supported_extensions: &[String]) -> Result<Vec<String>> {
    let mut languages = std::collections::HashSet::new();
    let source_files = find_source_files_excluding_tests(repo_dir, supported_extensions)?;
    
    for file_path in source_files {
        if let Some(extension) = file_path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if let Some(lang) = extension_to_language(&ext_str) {
                languages.insert(lang);
            }
        }
    }
    
    Ok(languages.into_iter().collect())
}

/// Map file extension to language name
fn extension_to_language(ext: &str) -> Option<String> {
    match ext {
        "java" => Some("java".to_string()),
        "js" | "jsx" => Some("javascript".to_string()),
        "ts" | "tsx" => Some("typescript".to_string()),
        "py" => Some("python".to_string()),
        "rs" => Some("rust".to_string()),
        "go" => Some("go".to_string()),
        "php" => Some("php".to_string()),
        "cs" => Some("csharp".to_string()),
        "swift" => Some("swift".to_string()),
        "kt" | "kts" => Some("kotlin".to_string()),
        _ => None,
    }
}

/// Prompt user to choose testing frameworks for each detected language
fn prompt_framework_choices(languages: &[String]) -> Result<HashMap<String, String>> {
    let mut choices = HashMap::new();
    
    for language in languages {
        let frameworks = get_available_frameworks(language);
        if frameworks.len() <= 1 {
            choices.insert(language.clone(), frameworks[0].clone());
            continue;
        }
        
        println!("\n📋 Choose testing framework for {}:", language.to_uppercase());
        for (i, framework) in frameworks.iter().enumerate() {
            println!("  {}. {}", i + 1, framework);
        }
        
        loop {
            print!("Enter choice (1-{}): ", frameworks.len());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice > 0 && choice <= frameworks.len() {
                    choices.insert(language.clone(), frameworks[choice - 1].clone());
                    break;
                }
            }
            println!("Invalid choice. Please enter a number between 1 and {}", frameworks.len());
        }
    }
    
    Ok(choices)
}

/// Get available testing frameworks for a language
fn get_available_frameworks(language: &str) -> Vec<String> {
    match language {
        "java" => vec!["junit5".to_string(), "testng".to_string()],
        "javascript" | "typescript" => vec!["jest".to_string(), "mocha".to_string()],
        "python" => vec!["pytest".to_string(), "unittest".to_string()],
        "rust" => vec!["cargo-test".to_string(), "nextest".to_string()],
        "go" => vec!["testing".to_string(), "testify".to_string()],
        "php" => vec!["phpunit".to_string(), "pest".to_string()],
        "csharp" => vec!["nunit".to_string(), "xunit".to_string()],
        "swift" => vec!["xctest".to_string(), "quick".to_string()],
        "kotlin" => vec!["junit5".to_string(), "kotest".to_string()],
        _ => vec!["default".to_string()],
    }
}

/// Get default framework for a language
fn get_default_framework(language: &str) -> String {
    get_available_frameworks(language)[0].clone()
}

/// Find source files excluding test directories
fn find_source_files_excluding_tests(dir: &Path, supported_extensions: &[String]) -> Result<Vec<std::path::PathBuf>> {
    let mut source_files = Vec::new();
    
    for entry in WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories, hidden files, and common non-source directories
        if path.is_dir() || is_ignored_path(path) || is_test_path(path) {
            continue;
        }
        
        // Check if file has supported extension
        if let Some(extension) = path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if supported_extensions.contains(&ext_str) {
                source_files.push(path.to_path_buf());
            }
        }
    }
    
    Ok(source_files)
}

/// Check if path is a test directory or file
fn is_test_path(path: &Path) -> bool {
    let test_indicators = [
        "test", "tests", "spec", "specs", "__tests__", "Test", "Tests"
    ];
    
    // Check if any part of the path contains test indicators
    for component in path.components() {
        if let Some(name) = component.as_os_str().to_str() {
            if test_indicators.iter().any(|&indicator| name.contains(indicator)) {
                return true;
            }
        }
    }
    
    false
}

/// Detect language of a specific file
fn detect_file_language(file_path: &Path, supported_extensions: &[String], loader: &LanguageLoader) -> Result<String> {
    if let Some(extension) = file_path.extension() {
        let ext_str = extension.to_string_lossy().to_lowercase();
        if let Some(language) = loader.get_supported_extensions().get(&ext_str) {
            return Ok(language.clone());
        }
    }
    Err(anyhow::anyhow!("Unsupported file type"))
}

/// Get the appropriate test file path for a source file
fn get_test_file_path(repo_dir: &Path, source_file: &Path, language: &str, framework: &str) -> Result<std::path::PathBuf> {
    let source_path = if source_file.is_absolute() {
        source_file.to_path_buf()
    } else {
        repo_dir.join(source_file)
    };
    
    let file_stem = source_path.file_stem().unwrap_or_default().to_string_lossy();
    
    match language {
        "java" => {
            // Java: src/test/java/... mirrors src/main/java/...
            let test_file_name = format!("{}Test.java", 
                file_stem.chars().next().unwrap().to_uppercase().collect::<String>() + 
                &file_stem[1..].to_string()
            );
            
            if let Some(parent) = source_path.parent() {
                Ok(parent.join("test").join(test_file_name))
            } else {
                Ok(repo_dir.join("test").join(test_file_name))
            }
        },
        "javascript" | "typescript" => {
            // JS/TS: __tests__ folder or .test.js alongside source
            let ext = if language == "typescript" { "ts" } else { "js" };
            let test_file_name = format!("{}.test.{}", file_stem, ext);
            
            if let Some(parent) = source_path.parent() {
                Ok(parent.join("__tests__").join(test_file_name))
            } else {
                Ok(repo_dir.join("__tests__").join(test_file_name))
            }
        },
        "python" => {
            // Python: tests/ folder or test_ prefix
            let test_file_name = format!("test_{}.py", file_stem);
            if let Some(parent) = source_path.parent() {
                Ok(parent.join("tests").join(test_file_name))
            } else {
                Ok(repo_dir.join("tests").join(test_file_name))
            }
        },
        "rust" => {
            // Rust: tests/ folder or inline tests
            let test_file_name = format!("test_{}.rs", file_stem);
            if let Some(parent) = source_path.parent() {
                Ok(parent.join("tests").join(test_file_name))
            } else {
                Ok(repo_dir.join("tests").join(test_file_name))
            }
        },
        "go" => {
            // Go: _test.go suffix in same directory
            let test_file_name = format!("{}_test.go", file_stem);
            if let Some(parent) = source_path.parent() {
                Ok(parent.join(test_file_name))
            } else {
                Ok(repo_dir.join(test_file_name))
            }
        },
        _ => {
            // Default: tests/ folder
            let test_file_name = format!("test_{}.test", file_stem);
            Ok(repo_dir.join("tests").join(test_file_name))
        }
    }
}

/// Generate test file content with specific framework
fn generate_test_file_content_with_framework(test_suite: &unified_test_framework::TestSuite, framework: &str) -> Result<String> {
    let mut content = String::new();
    
    match (test_suite.language.as_str(), framework) {
        ("java", "junit5") => {
            content.push_str("import org.junit.jupiter.api.Test;\n");
            content.push_str("import static org.junit.jupiter.api.Assertions.*;\n\n");
            content.push_str(&format!("class {} {{\n\n", test_suite.name));
            
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    @Test\n    void {}() {{\n        // {}\n        // TODO: Implement test logic\n    }}\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("}\n");
        },
        ("java", "testng") => {
            content.push_str("import org.testng.annotations.Test;\n");
            content.push_str("import static org.testng.Assert.*;\n\n");
            content.push_str(&format!("public class {} {{\n\n", test_suite.name));
            
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    @Test\n    public void {}() {{\n        // {}\n        // TODO: Implement test logic\n    }}\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("}\n");
        },
        ("javascript" | "typescript", "jest") => {
            content.push_str("describe('Generated Tests', () => {\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "  test('{}', () => {{\n    // {}\n    // TODO: Implement test logic\n  }});\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("});\n");
        },
        ("javascript" | "typescript", "mocha") => {
            content.push_str("const { expect } = require('chai');\n\n");
            content.push_str("describe('Generated Tests', () => {\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "  it('{}', () => {{\n    // {}\n    // TODO: Implement test logic\n  }});\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("});\n");
        },
        ("python", "pytest") => {
            content.push_str("import pytest\n\n");
            content.push_str("class TestGenerated:\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    def {}(self):\n        \"\"\" {} \"\"\"\n        # TODO: Implement test logic\n        pass\n\n",
                    test_case.name, test_case.description
                ));
            }
        },
        ("python", "unittest") => {
            content.push_str("import unittest\n\n");
            content.push_str("class TestGenerated(unittest.TestCase):\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    def {}(self):\n        \"\"\" {} \"\"\"\n        # TODO: Implement test logic\n        pass\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("\nif __name__ == '__main__':\n    unittest.main()\n");
        },
        ("rust", _) => {
            content.push_str("#[cfg(test)]\nmod tests {\n    use super::*;\n\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "    #[test]\n    fn {}() {{\n        // {}\n        // TODO: Implement test logic\n    }}\n\n",
                    test_case.name, test_case.description
                ));
            }
            content.push_str("}\n");
        },
        ("go", _) => {
            content.push_str("package main\n\nimport (\n    \"testing\"\n)\n\n");
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "func {}(t *testing.T) {{\n    // {}\n    // TODO: Implement test logic\n}}\n\n",
                    test_case.name, test_case.description
                ));
            }
        },
        _ => {
            return generate_test_file_content(test_suite);
        }
    }
    
    Ok(content)
}

/// Generate integration test file content
fn generate_integration_test_content(test_suite: &unified_test_framework::TestSuite) -> Result<String> {
    let mut content = String::new();
    
    match test_suite.language.as_str() {
        "javascript" => {
            // Add imports
            for import in &test_suite.imports {
                content.push_str(&format!("{}\n", import));
            }
            content.push_str("\n");
            
            // Add setup/teardown hooks
            content.push_str("describe('Integration Tests', () => {\n");
            content.push_str("  beforeAll(async () => {\n");
            content.push_str("    // Setup requirements:\n");
            for req in &test_suite.setup_requirements {
                content.push_str(&format!("    // - {}\n", req));
            }
            content.push_str("  });\n\n");
            
            content.push_str("  afterAll(async () => {\n");
            content.push_str("    // Cleanup requirements:\n");
            for req in &test_suite.cleanup_requirements {
                content.push_str(&format!("    // - {}\n", req));
            }
            content.push_str("  });\n\n");
            
            // Add test cases
            for test_case in &test_suite.test_cases {
                content.push_str(&format!(
                    "  test('{}', async () => {{\n",
                    test_case.name.replace('_', " ")
                ));
                content.push_str(&format!("    // {}\n", test_case.description));
                content.push_str("    // TODO: Implement integration test logic\n");
                content.push_str(&format!("    // Input: {}\n", test_case.input));
                content.push_str(&format!("    // Expected: {}\n", test_case.expected_output));
                content.push_str("  });\n\n");
            }
            
            content.push_str("});\n");
        },
        _ => {
            content.push_str("// Integration test generation not yet implemented for this language\n");
            content.push_str(&format!("// Language: {}\n", test_suite.language));
            content.push_str(&format!("// Framework: {}\n", test_suite.framework));
        }
    }
    
    Ok(content)
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