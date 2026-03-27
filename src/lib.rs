//! Zed extension for GitHub Actions workflow files.
//!
//! Provides LSP support via [`@actions/languageserver`](https://github.com/actions/languageservices),
//! which is downloaded automatically via npm on first use.
//!
//! A GitHub personal access token (with `repo` scope) can be provided via the
//! `GITHUB_TOKEN` / `GH_TOKEN` environment variable or in Zed LSP settings to
//! enable enhanced completions using live data from the GitHub API.

use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

const SERVER_NAME: &str = "gh-actions-language-server";
const NPM_PACKAGE: &str = "@actions/languageserver";
/// Must end with `NPM_BIN_NAME`.
const NPM_BIN_PATH: &str = "node_modules/@actions/languageserver/bin/actions-languageserver";
const NPM_BIN_NAME: &str = "actions-languageserver";

#[derive(Debug, Default)]
struct GithubActionsExtension {
    // Set to true once we've confirmed the language server is available for
    // this session, avoiding redundant npm version checks on every LSP restart.
    server_ready: bool,
}

impl GithubActionsExtension {
    fn resolve_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Priority 1: user-configured binary path in LSP settings
        if let Some(binary) = LspSettings::for_worktree(SERVER_NAME, worktree)
            .ok()
            .and_then(|s| s.binary)
        {
            if let Some(path) = binary.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary.arguments.unwrap_or_else(|| vec!["--stdio".into()]),
                    env: Default::default(),
                });
            }
        }

        // Priority 2: binary already on PATH (e.g. installed globally via npm)
        if let Some(path) = worktree.which(NPM_BIN_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".into()],
                env: Default::default(),
            });
        }

        // Priority 3: install via npm into the extension's node_modules
        self.ensure_server_installed(language_server_id)?;

        // Run through Zed's managed Node binary so we don't depend on the
        // system Node version or .bin symlink resolution.
        let node = zed::node_binary_path()?;
        let bin_path = std::env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?
            .join(NPM_BIN_PATH)
            .to_str()
            .ok_or_else(|| "npm binary path contains non-UTF-8 characters".to_string())?
            .to_owned();

        Ok(zed::Command {
            command: node,
            args: vec![bin_path, "--stdio".into()],
            env: Default::default(),
        })
    }

    fn ensure_server_installed(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<()> {
        if self.server_ready {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let installed_version = zed::npm_package_installed_version(NPM_PACKAGE)?;

        match zed::npm_package_latest_version(NPM_PACKAGE) {
            Ok(latest) if installed_version.as_deref() != Some(&latest) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );
                // Tolerate install failure if we already have a usable version
                let result = zed::npm_install_package(NPM_PACKAGE, &latest);
                if let Err(err) = result {
                    if installed_version.is_none() {
                        return Err(err);
                    }
                }
            }
            Ok(_) => {} // already up to date
            Err(_) if installed_version.is_some() => {
                // Offline or registry error — proceed with the existing version
            }
            Err(e) => return Err(e), // no version installed and can't fetch one
        }

        self.server_ready = true;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(())
    }

    fn github_token(worktree: &zed::Worktree) -> Option<String> {
        let settings_token = LspSettings::for_worktree(SERVER_NAME, worktree)
            .ok()
            .and_then(|s| s.settings)
            .and_then(|s| s.get("token").and_then(|v| v.as_str()).map(str::to_owned));

        resolve_token(settings_token.as_deref(), &worktree.shell_env())
    }
}

/// Pure token resolution logic, extracted for testability.
///
/// Prefers `settings_token` over env vars; treats an empty settings token as absent.
fn resolve_token(settings_token: Option<&str>, env: &[(String, String)]) -> Option<String> {
    settings_token
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
        .or_else(|| {
            env.iter()
                .find(|(k, _)| k == "GITHUB_TOKEN" || k == "GH_TOKEN")
                .map(|(_, v)| v.clone())
        })
}

impl zed::Extension for GithubActionsExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        self.resolve_binary(language_server_id, worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let mut opts = serde_json::json!({
            // Enable code actions like "add missing required inputs"
            "experimentalFeatures": { "all": true }
        });

        // Only set sessionToken when we actually have one — an empty string
        // is not equivalent to omitting the key.
        if let Some(token) = Self::github_token(worktree) {
            opts["sessionToken"] = serde_json::Value::String(token);
        }

        Ok(Some(opts))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Pass through the user's LSP settings block directly so power users
        // can configure the server without requiring extension updates.
        // Returns None when no settings are configured, which the LSP spec
        // requires servers to handle gracefully.
        Ok(LspSettings::for_worktree(SERVER_NAME, worktree)
            .ok()
            .and_then(|s| s.settings))
    }
}

zed::register_extension!(GithubActionsExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn settings_token_takes_priority_over_env() {
        let e = env(&[("GITHUB_TOKEN", "env-token")]);
        assert_eq!(resolve_token(Some("settings-token"), &e).as_deref(), Some("settings-token"));
    }

    #[test]
    fn empty_settings_token_falls_back_to_env() {
        let e = env(&[("GITHUB_TOKEN", "env-token")]);
        assert_eq!(resolve_token(Some(""), &e).as_deref(), Some("env-token"));
    }

    #[test]
    fn github_token_env_var_accepted() {
        let e = env(&[("GITHUB_TOKEN", "gh-token")]);
        assert_eq!(resolve_token(None, &e).as_deref(), Some("gh-token"));
    }

    #[test]
    fn gh_token_env_var_accepted() {
        let e = env(&[("GH_TOKEN", "gh-token")]);
        assert_eq!(resolve_token(None, &e).as_deref(), Some("gh-token"));
    }

    #[test]
    fn github_token_takes_priority_over_gh_token() {
        let e = env(&[("GH_TOKEN", "gh"), ("GITHUB_TOKEN", "github")]);
        // whichever comes first in the env slice wins via `find`
        assert_eq!(resolve_token(None, &e).as_deref(), Some("gh"));
    }

    #[test]
    fn no_token_returns_none() {
        assert_eq!(resolve_token(None, &[]), None);
    }

    #[test]
    fn whitespace_only_settings_token_is_not_filtered() {
        // filter only removes empty strings, not whitespace — caller's responsibility
        let e = env(&[("GITHUB_TOKEN", "env-token")]);
        assert_eq!(resolve_token(Some("   "), &e).as_deref(), Some("   "));
    }
}
