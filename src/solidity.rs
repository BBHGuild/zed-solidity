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

/// Locate `forge`: worktree `PATH` first, then the default foundryup install
/// location. The extension runs in a WASM sandbox where neither the host's
/// `HOME` nor host files are visible, so `HOME` is read from the worktree's
/// shell environment and the default path is returned without probing it.
fn find_forge(worktree: &zed::Worktree, shell_env: &[(String, String)]) -> Result<String> {
    if let Some(path) = worktree.which("forge") {
        return Ok(path);
    }
    shell_env
        .iter()
        .find(|(key, _)| key == "HOME")
        .map(|(_, home)| format!("{home}/.foundry/bin/forge"))
        .ok_or_else(|| {
            "forge not found in PATH; install Foundry or set `lsp.solar.binary.path`".to_string()
        })
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
        let binary = zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let (path, args, extra_env) = match binary {
            Some(b) => (b.path, b.arguments, b.env.unwrap_or_default()),
            None => (None, None, Default::default()),
        };

        // Forward the project shell env (PATH, FOUNDRY_*, direnv, ...) so the
        // server behaves like `forge` run from a terminal, with user overrides on top.
        let mut env = worktree.shell_env();
        env.retain(|(key, _)| !extra_env.contains_key(key));
        env.extend(extra_env);

        if language_server_id.as_ref() == "solar" {
            let command = match path {
                Some(path) => path,
                None => find_forge(worktree, &env)?,
            };
            return Ok(zed::Command {
                command,
                args: args.unwrap_or_else(|| vec!["lsp".to_string()]),
                env,
            });
        }

        // `solidity` (Nomic Foundation server): honor an explicit binary override,
        // otherwise install/update the npm package and run it with Zed's Node.
        if let Some(command) = path {
            return Ok(zed::Command {
                command,
                args: args.unwrap_or_else(|| vec!["--stdio".to_string()]),
                env,
            });
        }

        let server_path = self.server_script_path(language_server_id, worktree)?;
        let server_full_path = env::current_dir()
            .map_err(|e| format!("failed to resolve extension work dir: {e}"))?
            .join(&server_path)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_full_path, "--stdio".to_string()],
            env,
        })
    }
}

zed::register_extension!(SolidityExtension);
