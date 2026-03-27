# GitHub Actions — Zed Extension

GitHub Actions workflow support for [Zed](https://zed.dev), providing intelligent editing features powered by the official [GitHub Actions Language Server](https://github.com/actions/languageservices).

## Features

- **Code completion** — actions, event names, expression functions, job outputs, step IDs
- **Validation** — real-time diagnostics for invalid workflow syntax and schema errors
- **Hover documentation** — inline docs for workflow keys, expression functions, and event payloads
- **Code actions** — quickfixes like adding missing required action inputs
- **Syntax highlighting** — workflow-specific keywords and `${{ }}` expression markers

## Installation

Install via **Zed → Extensions** and search for "GitHub Actions".

The language server (`@actions/languageserver`) is downloaded automatically on first use via Zed's built-in Node.js. No manual install needed.

## Setup

### File type association (required)

Zed needs to know which files are GitHub Actions workflows. Add this to your Zed settings (`Zed → Settings → Open Settings`):

```json
{
  "file_types": {
    "GitHub Actions": [
      ".github/workflows/*.yml",
      ".github/workflows/*.yaml"
    ]
  }
}
```

### GitHub token (optional, recommended)

Providing a token enables enhanced completions using live data from GitHub (available actions, their inputs/outputs, etc.):

**Option 1 — Environment variable** (picked up automatically):
```sh
export GITHUB_TOKEN="ghp_your_token_here"
# or
export GH_TOKEN="ghp_your_token_here"  # set by `gh auth login`
```

**Option 2 — Zed settings** (per-project via `.zed/settings.json`):
```json
{
  "lsp": {
    "gh-actions-language-server": {
      "settings": {
        "token": "ghp_your_token_here"
      }
    }
  }
}
```

### Custom language server binary (optional)

If you want to manage the language server yourself:

```json
{
  "lsp": {
    "gh-actions-language-server": {
      "binary": {
        "path": "/usr/local/bin/actions-languageserver",
        "arguments": ["--stdio"]
      }
    }
  }
}
```

## License

MIT
