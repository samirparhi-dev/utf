use zed_extension_api::{self as zed, Result};
use std::fs;
use std::process::Command;

struct UnifiedTestingExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for UnifiedTestingExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "unified-testing" => {
                let binary_path = self.binary_path()?;
                Ok(zed::Command {
                    command: binary_path,
                    args: vec!["server".into()],
                    env: Default::default(),
                })
            }
            _ => Err("Unknown language server".into()),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() != "unified-testing" {
            return Ok(None);
        }

        Ok(Some(zed::serde_json::json!({
            "unified_testing": {
                "auto_generate": true,
                "output_directory": "tests/",
                "supported_languages": ["javascript", "typescript", "python", "rust"]
            }
        })))
    }
}

impl UnifiedTestingExtension {
    fn binary_path(&mut self) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &zed::LanguageServerId("unified-testing".into()),
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "unified-testing/unified-test-framework",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "unified-testing-{}-{}{}",
            platform,
            arch,
            if platform == "windows" { ".exe" } else { "" }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("unified-testing-{}", release.version);
        let binary_path = format!("{version_dir}/{asset_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &zed::LanguageServerId("unified-testing".into()),
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Gzip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        zed::set_language_server_installation_status(
            &zed::LanguageServerId("unified-testing".into()),
            &zed::LanguageServerInstallationStatus::None,
        );
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(UnifiedTestingExtension);

// Additional helper functions for the extension
impl UnifiedTestingExtension {
    fn execute_command(&self, command: &str, args: &[&str], working_dir: Option<&str>) -> Result<String> {
        let binary_path = self.cached_binary_path.as_ref()
            .ok_or("Binary path not cached")?;
        
        let mut cmd = Command::new(binary_path);
        cmd.args(args);
        
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        
        let output = cmd.output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }

    pub fn generate_tests_for_file(&self, file_path: &str, output_dir: Option<&str>) -> Result<String> {
        let mut args = vec!["generate", file_path];
        if let Some(dir) = output_dir {
            args.extend(&["--output", dir]);
        }
        self.execute_command("generate", &args, None)
    }

    pub fn analyze_file(&self, file_path: &str) -> Result<String> {
        self.execute_command("analyze", &["analyze", file_path], None)
    }
}

// Language server commands for Zed integration
pub fn register_commands() {
    // Generate tests command
    zed::register_command("unified-testing:generate-tests", |editor: &mut zed::Editor| {
        if let Some(buffer) = editor.buffer() {
            if let Some(file_path) = buffer.file_path() {
                let extension = UnifiedTestingExtension::new();
                match extension.generate_tests_for_file(&file_path, Some("tests/")) {
                    Ok(output) => {
                        zed::show_message(&format!("Tests generated successfully:\n{}", output));
                    }
                    Err(e) => {
                        zed::show_error(&format!("Failed to generate tests: {}", e));
                    }
                }
            } else {
                zed::show_error("No file is currently open");
            }
        }
    });

    // Analyze file command
    zed::register_command("unified-testing:analyze-file", |editor: &mut zed::Editor| {
        if let Some(buffer) = editor.buffer() {
            if let Some(file_path) = buffer.file_path() {
                let extension = UnifiedTestingExtension::new();
                match extension.analyze_file(&file_path) {
                    Ok(output) => {
                        zed::show_message(&format!("Analysis results:\n{}", output));
                    }
                    Err(e) => {
                        zed::show_error(&format!("Failed to analyze file: {}", e));
                    }
                }
            } else {
                zed::show_error("No file is currently open");
            }
        }
    });
}