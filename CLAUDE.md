# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build

```sh
# Compile extension to WebAssembly (required target)
cargo build --target wasm32-wasip1 --release

# Add the target if missing
rustup target add wasm32-wasip1
```

Requires Rust ≥ 1.85.0 (also set as `rust-version` in Cargo.toml).

## Checks

CI runs `make check` (`cargo fmt --check` and clippy with `-D warnings`). Run it before pushing.

## Tests

Unit tests live in `src/lib.rs` and run on the host target (not WASM):

```sh
cargo test
```

## Testing in Zed

1. Open Zed → **Extensions** → **Install Dev Extension** → select this directory
2. Add to Zed settings to activate on workflow files:
   ```json
   { "file_types": { "GitHub Actions": ["**/.github/workflows/*.yml", "**/.github/workflows/*.yaml"] } }
   ```
3. Open any `.github/workflows/*.yml` file to trigger the language server

## Architecture

This is a Zed extension compiled to `wasm32-wasip1`. The crate type is `cdylib` — no binary, just the library exported as a WASM component.

**`src/lib.rs`** — the entire Rust implementation. Implements `zed::Extension` with three methods:
- `language_server_command`: resolves the language server binary in priority order: user-configured path → PATH → npm install. User `arguments` apply only to a configured path; the other two use `--stdio`. Uses Zed's managed Node via `zed::node_binary_path()` to run the npm-installed server.
- `language_server_initialization_options`: passes GitHub token (`sessionToken`) and enables all experimental features.
- `language_server_workspace_configuration`: forwards the user's LSP settings JSON directly to the server.

**Language server**: `@actions/languageserver` (npm). Binary entry point is `node_modules/@actions/languageserver/bin/actions-languageserver`. Installed and version-managed via `zed::npm_install_package` / `zed::npm_package_latest_version`.

**Token resolution** (checked in order):
1. `lsp.gh-actions-language-server.settings.token` in Zed settings
2. `GITHUB_TOKEN` env var
3. `GH_TOKEN` env var

**`languages/github-actions/`** — language definition using the `yaml` tree-sitter grammar. The grammar is downloaded by Zed at runtime, pinned in `extension.toml` (`grammars.yaml`) to a specific commit of `zed-industries/tree-sitter-yaml`. `highlights.scm` adds workflow-specific keyword captures on top of standard YAML highlights. Strings containing `${{` are captured as `@string.special`.

## Git

No `Co-Authored-By` or "Generated with Claude Code" lines in commits or PRs.
