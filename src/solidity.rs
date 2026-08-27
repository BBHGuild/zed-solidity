use std::{env, fs};
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/@nomicfoundation/solidity-language-server/out/index.js";
const PACKAGE_NAME: &str = "@nomicfoundation/solidity-language-server";

struct SolidityExtension {
    did_find_server: bool,
}

impl SolidityExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<String> {
        let server_exists = self.server_exists();

        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if !server_exists || installed_version.as_ref() != Some(&version) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for SolidityExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() == "solar" {
            if let Ok(lsp_settings) =
                zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            {
                if let Some(binary) = lsp_settings.binary {
                    let forge_cmd = if let Some(path) = binary.path {
                        path
                    } else {
                        worktree
                            .which("forge")
                            .or_else(|| {
                                let home = env::var("HOME").ok()?;
                                let default_path = format!("{home}/.foundry/bin/forge");
                                if fs::metadata(&default_path).map_or(false, |m| m.is_file()) {
                                    Some(default_path)
                                } else {
                                    None
                                }
                            })
                            .ok_or_else(|| {
                                "forge executable not found in PATH or ~/.foundry/bin/forge".to_string()
                            })?
                    };

                    let args = binary.arguments.unwrap_or_else(|| vec!["lsp".to_string()]);
                    let env = binary.env.unwrap_or_default().into_iter().collect();

                    return Ok(zed::Command {
                        command: forge_cmd,
                        args,
                        env,
                    });
                }
            }

            let forge_path = worktree
                .which("forge")
                .or_else(|| {
                    let home = env::var("HOME").ok()?;
                    let default_path = format!("{home}/.foundry/bin/forge");
                    if fs::metadata(&default_path).map_or(false, |m| m.is_file()) {
                        Some(default_path)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| "forge executable not found in PATH or ~/.foundry/bin/forge".to_string())?;

            return Ok(zed::Command {
                command: forge_path,
                args: vec!["lsp".to_string()],
                env: Default::default(),
            });
        }

        let server_path = self.server_script_path(language_server_id, worktree)?;

        let node_path = zed::node_binary_path()?;

        let server_full_path = env::current_dir()
            .unwrap()
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: node_path,
            args: vec![server_full_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SolidityExtension);
