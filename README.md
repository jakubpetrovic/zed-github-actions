# GitHub Actions — Zed Extension

GitHub Actions workflow support for [Zed](https://zed.dev), providing intelligent editing features powered by the official [GitHub Actions Language Server](https://github.com/actions/languageservices).

## Features

- **Code completion** — actions, event names, expression functions, job outputs, step IDs
- **Validation** — real-time diagnostics for invalid workflow syntax and schema errors
- **Hover documentation** — inline docs for workflow keys, expression functions, and event payloads
- **Code actions** — quickfixes like adding missing required action inputs
- **Syntax highlighting** — workflow-specific keywords and `${{ }}` expression markers, bash in step `run:` scripts, JavaScript in `actions/github-script` `script:` blocks
- **Outline** — top-level keys, jobs and named steps in the symbol outline

## Installation

1. Open the Zed Extensions menu
2. Select `Install Dev Extension`
3. Select this repository as the extension directory

Zed builds the extension itself. It needs Rust 1.85 or newer with the `wasm32-wasip1` target (`rustup target add wasm32-wasip1`).

The language server (`@actions/languageserver`) is downloaded automatically on first use via Zed's built-in Node.js. No manual install needed.

## Setup

### File type association (required)

Zed needs to know which files are GitHub Actions workflows. Add this to your Zed settings (`Zed → Settings → Open Settings`):

```json
{
  "file_types": {
    "GitHub Actions": [
      "**/.github/workflows/*.yml",
      "**/.github/workflows/*.yaml"
    ]
  }
}
```

### GitHub token (optional)

A token enables completions that use live data from the GitHub API. Create a personal access token with read access to the repositories your workflows use.

The extension looks for a token in this order:

1. `lsp.gh-actions-language-server.settings.token` in Zed settings
2. `GITHUB_TOKEN` environment variable
3. `GH_TOKEN` environment variable

`gh auth login` does not set `GH_TOKEN`. It stores the token in the system keychain. To reuse it, export it in your shell profile: `export GH_TOKEN=$(gh auth token)`. Zed only sees it when started from that shell (for example `zed .`). Launching from the Dock or Spotlight does not pass it. In that case, set `token` in Zed settings instead.

```json
{
  "lsp": {
    "gh-actions-language-server": {
      "settings": { "token": "ghp_..." }
    }
  }
}
```

### Custom language server binary (optional)

The extension uses the first match:

1. `binary.path` in Zed settings
2. `actions-languageserver` on `PATH`
3. `@actions/languageserver` installed from npm

```json
{
  "lsp": {
    "gh-actions-language-server": {
      "binary": {
        "path": "/usr/local/bin/actions-languageserver",
        "arguments": ["--stdio"],
        "env": { "NODE_OPTIONS": "--max-old-space-size=512" }
      }
    }
  }
}
```

## License

MIT
