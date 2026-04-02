# GitHub Actions — Zed Extension

GitHub Actions workflow support for [Zed](https://zed.dev), providing intelligent editing features powered by the official [GitHub Actions Language Server](https://github.com/actions/languageservices).

## Features

- **Code completion** — actions, event names, expression functions, job outputs, step IDs
- **Validation** — real-time diagnostics for invalid workflow syntax and schema errors
- **Hover documentation** — inline docs for workflow keys, expression functions, and event payloads
- **Code actions** — quickfixes like adding missing required action inputs
- **Syntax highlighting** — workflow-specific keywords and `${{ }}` expression markers

## Installation

1. Build the extension using `make build` or `cargo build --target wasm32-wasip1 --release` 
2. Open Zed Extensions menu
3. Select `Install Dev Extension`
4. Select this repository as teh extension directory

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

## License

MIT
