use std::fs;
use zed_extension_api::{self as zed, LanguageServerId, Result};

struct HledgerExtension {
    cached_binary_path: Option<String>,
}

impl HledgerExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("hledger-lsp") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let (platform, arch) = zed::current_platform();
        let binary_name = Self::binary_name(platform, arch)?;

        match zed::latest_github_release(
            "juev/hledger-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => {
                let asset = release
                    .assets
                    .iter()
                    .find(|asset| asset.name == binary_name)
                    .ok_or_else(|| format!("no asset found for {binary_name}"))?;

                let version_dir = format!("hledger-lsp-{}", release.version);
                let binary_path = format!("{version_dir}/{binary_name}");

                if !fs::metadata(&binary_path)
                    .map(|m| m.is_file())
                    .unwrap_or(false)
                {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed::LanguageServerInstallationStatus::Downloading,
                    );

                    zed::download_file(
                        &asset.download_url,
                        &binary_path,
                        zed::DownloadedFileType::Uncompressed,
                    )
                    .map_err(|e| format!("failed to download hledger-lsp: {e}"))?;

                    zed::make_file_executable(&binary_path)?;
                }

                self.cached_binary_path = Some(binary_path.clone());
                Ok(binary_path)
            }
            Err(_) => {
                if let Some(path) = &self.cached_binary_path {
                    if fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                        return Ok(path.clone());
                    }
                }
                Err("failed to fetch latest hledger-lsp release and no cached binary available"
                    .into())
            }
        }
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

        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(HledgerExtension);
