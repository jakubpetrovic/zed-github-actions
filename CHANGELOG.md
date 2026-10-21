# Changelog

## Unreleased

- Apply user arguments only to a configured binary path.
- Clean up workflow highlight keywords.
- Build the extension on Rust edition 2024 with a size-tuned release profile.

## 0.1.0

- Language server support via `@actions/languageserver`, installed through npm on first use.
- GitHub token from Zed settings, `GITHUB_TOKEN` or `GH_TOKEN`.
- Custom binary path through Zed LSP settings. Arguments apply only to a custom binary path. Env applies to all.
- Workflow-specific syntax highlighting.
