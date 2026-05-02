use std::{collections::HashMap, fs};
use zed_extension_api::{
    self as zed,
    settings::{CommandSettings, LspSettings},
    LanguageServerId, Result,
};

struct HledgerExtension {
    cached_binary_path: Option<String>,
}

impl HledgerExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let (platform, arch) = zed::current_platform();
        let binary_name = Self::binary_name(platform, arch)?;

        match Self::latest_release_binary_path(language_server_id, &binary_name) {
            Ok(binary_path) => {
                self.cached_binary_path = Some(binary_path.clone());
                Ok(binary_path)
            }
            Err(err) => {
                if let Some(path) = self.github_error_fallback_path_for_worktree(worktree) {
                    return Ok(path);
                }
                Err(format!(
                    "failed to install latest hledger-lsp release and no cached binary available: {err}"
                ))
            }
        }
    }

    fn latest_release_binary_path(
        language_server_id: &LanguageServerId,
        binary_name: &str,
    ) -> Result<String> {
        let release = zed::latest_github_release(
            "juev/hledger-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == binary_name)
            .ok_or_else(|| format!("no asset found for {binary_name}"))?;

        let (version_dir, binary_path) = Self::release_paths(&release.version, binary_name);

        if !Self::path_is_file(&binary_path) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            fs::create_dir_all(&version_dir)
                .map_err(|e| format!("failed to create directory '{version_dir}': {e}"))?;

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download hledger-lsp: {e}"))?;

            zed::make_file_executable(&binary_path)?;
        }

        Ok(binary_path)
    }

    fn command_parts(
        binary_settings: Option<CommandSettings>,
        shell_env: zed::EnvVars,
    ) -> (Option<String>, Vec<String>, zed::EnvVars) {
        if let Some(settings) = binary_settings {
            (
                settings.path,
                settings.arguments.unwrap_or_default(),
                Self::merge_env(shell_env, settings.env),
            )
        } else {
            (None, Vec::new(), shell_env)
        }
    }

    fn merge_env(
        mut shell_env: zed::EnvVars,
        settings_env: Option<HashMap<String, String>>,
    ) -> zed::EnvVars {
        let Some(settings_env) = settings_env else {
            return shell_env;
        };

        let mut settings_env: Vec<_> = settings_env.into_iter().collect();
        settings_env.sort_by(|(left, _), (right, _)| left.cmp(right));

        for (key, value) in settings_env {
            if let Some((_, existing_value)) = shell_env
                .iter_mut()
                .find(|(existing_key, _)| existing_key == &key)
            {
                *existing_value = value;
            } else {
                shell_env.push((key, value));
            }
        }

        shell_env
    }

    fn release_paths(release_version: &str, binary_name: &str) -> (String, String) {
        let version_dir = format!("hledger-lsp-{release_version}");
        let binary_path = format!("{version_dir}/{binary_name}");
        (version_dir, binary_path)
    }

    fn github_error_fallback_path(
        cached_binary_path: Option<&str>,
        cached_binary_path_exists: bool,
        path_binary: Option<String>,
    ) -> Option<String> {
        if cached_binary_path_exists {
            cached_binary_path.map(str::to_string)
        } else {
            path_binary
        }
    }

    fn github_error_fallback_path_for_worktree(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Option<String> {
        let cached_binary_path = self.cached_binary_path.as_deref();
        let cached_binary_path_exists = cached_binary_path.map(Self::path_is_file).unwrap_or(false);
        let fallback_path = Self::github_error_fallback_path(
            cached_binary_path,
            cached_binary_path_exists,
            worktree.which("hledger-lsp"),
        );

        if let Some(path) = &fallback_path {
            self.cached_binary_path = Some(path.clone());
        }

        fallback_path
    }

    fn path_is_file(path: &str) -> bool {
        fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
    }

    fn binary_name(platform: zed::Os, arch: zed::Architecture) -> Result<String> {
        let os = match platform {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };

        let arch = match arch {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "amd64",
            zed::Architecture::X86 => "386",
        };

        let extension = match platform {
            zed::Os::Windows => ".exe",
            _ => "",
        };

        Ok(format!("hledger-lsp_{os}_{arch}{extension}"))
    }
}

impl zed::Extension for HledgerExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != "hledger-lsp" {
            return Err(format!("unknown language server: {language_server_id}"));
        }

        let binary_settings = LspSettings::for_worktree("hledger-lsp", worktree)
            .ok()
            .and_then(|s| s.binary);
        let (configured_path, args, env) =
            Self::command_parts(binary_settings, worktree.shell_env());
        let binary_path = match configured_path {
            Some(path) => path,
            None => self.language_server_binary_path(language_server_id, worktree)?,
        };

        Ok(zed::Command {
            command: binary_path,
            args,
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("hledger-lsp", worktree)
            .ok()
            .and_then(|s| s.initialization_options);
        Ok(settings)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree("hledger-lsp", worktree)
            .ok()
            .and_then(|s| s.settings);
        Ok(settings)
    }
}

zed::register_extension!(HledgerExtension);

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use zed::settings::CommandSettings;

    fn env_value<'a>(env: &'a [(String, String)], key: &str) -> Option<&'a str> {
        env.iter()
            .find(|(env_key, _)| env_key == key)
            .map(|(_, value)| value.as_str())
    }

    #[test]
    fn command_parts_preserve_explicit_path_args_and_env() {
        let mut settings_env = HashMap::new();
        settings_env.insert("PATH".to_string(), "/custom/bin".to_string());
        settings_env.insert("RUST_LOG".to_string(), "debug".to_string());

        let (path, args, env) = HledgerExtension::command_parts(
            Some(CommandSettings {
                path: Some("/custom/hledger-lsp".to_string()),
                arguments: Some(vec!["--stdio".to_string()]),
                env: Some(settings_env),
            }),
            vec![
                ("PATH".to_string(), "/usr/bin".to_string()),
                ("HOME".to_string(), "/home/test".to_string()),
            ],
        );

        assert_eq!(path.as_deref(), Some("/custom/hledger-lsp"));
        assert_eq!(args, vec!["--stdio"]);
        assert_eq!(env_value(&env, "PATH"), Some("/custom/bin"));
        assert_eq!(env_value(&env, "RUST_LOG"), Some("debug"));
        assert_eq!(env_value(&env, "HOME"), Some("/home/test"));
    }

    #[test]
    fn release_paths_use_versioned_directory() {
        let (version_dir, binary_path) =
            HledgerExtension::release_paths("v0.2.41", "hledger-lsp_darwin_arm64");

        assert_eq!(version_dir, "hledger-lsp-v0.2.41");
        assert_eq!(binary_path, "hledger-lsp-v0.2.41/hledger-lsp_darwin_arm64");
    }

    #[test]
    fn github_error_fallback_prefers_valid_cached_path_then_path_binary() {
        assert_eq!(
            HledgerExtension::github_error_fallback_path(
                Some("hledger-lsp-v0.2.41/hledger-lsp_darwin_arm64"),
                true,
                Some("/usr/local/bin/hledger-lsp".to_string()),
            ),
            Some("hledger-lsp-v0.2.41/hledger-lsp_darwin_arm64".to_string())
        );

        assert_eq!(
            HledgerExtension::github_error_fallback_path(
                Some("hledger-lsp-v0.2.40/hledger-lsp_darwin_arm64"),
                false,
                Some("/usr/local/bin/hledger-lsp".to_string()),
            ),
            Some("/usr/local/bin/hledger-lsp".to_string())
        );

        assert_eq!(
            HledgerExtension::github_error_fallback_path(
                Some("hledger-lsp-v0.2.40/hledger-lsp_darwin_arm64"),
                false,
                None,
            ),
            None
        );
    }

    #[test]
    fn binary_name_matches_release_asset_names() {
        assert_eq!(
            HledgerExtension::binary_name(zed::Os::Mac, zed::Architecture::Aarch64).unwrap(),
            "hledger-lsp_darwin_arm64"
        );
        assert_eq!(
            HledgerExtension::binary_name(zed::Os::Linux, zed::Architecture::X86).unwrap(),
            "hledger-lsp_linux_386"
        );
        assert_eq!(
            HledgerExtension::binary_name(zed::Os::Windows, zed::Architecture::X8664).unwrap(),
            "hledger-lsp_windows_amd64.exe"
        );
    }
}
