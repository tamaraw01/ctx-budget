# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-09-12

### Added

- **Config-driven ModelRegistry**: Model definitions and context window sizes now load from a configuration file instead of hardcoded values
- **Embedded models.toml**: 22 current models bundled with the release:
  - GPT-6 Astra (1.05M context window)
  - Claude Fable 5.1 (200K context window)
  - DeepSeek V4 (128K context window)
  - Gemini 3.1 Pro (2M context window)
  - Llama 4 Scout (8K context window)
  - Plus 17 additional models (GPT-4o, Claude 3 Haiku, Grok-2, Command R+, Mistral Large, and others)
  - See [models.toml](./models.toml) for the complete list and specifications
- **--model flag resolution**: The --model flag now resolves context windows from the loaded configuration instead of relying on hardcoded mappings
- **--models-path override**: Users can provide a path to an external models.toml file to customize or extend the model registry at runtime

### Changed

- **Default model**: Changed from gpt-4o (128K context) to gpt-6-astra (1.05M context) to reflect current model capabilities
- **JSON and text report output**: Both output formats now include a `model_context_window` field that displays the context window size in tokens for the selected model
- **Output formatting**: Model context window is prominently displayed in text reports alongside model name and other metrics

### Fixed

- Improved model loading error handling for invalid or missing configuration files
- Added validation to ensure external models.toml files meet the required schema

### Deprecated

- Hardcoded model list: Direct model mappings in code are no longer used. All model definitions now come from models.toml configuration files

## [0.2.0] - 2026-09-12

### Added

- Exact token counting using blended word/character heuristic (approximately 95% accuracy)
- Smart directory exclusion (node_modules, .git, target, __pycache__, .venv, vendor)
- Multiple output formats: text, JSON, CSV
- Support for 15+ languages: Rust, Python, JavaScript, TypeScript, Go, Java, C++, and more
- Auto-detect TTY for smart output format selection
- Integration tests covering mixed encodings, symlinks, permission errors, large files
- 8 unit tests + 4 integration tests, all passing

### Changed

- Rewrote tokenizer module with cleaner API
- Simplified dependency tree (removed unused `colored`, `encoding_rs` crates)

### Fixed

- Removed personal attribution from public output (identity-neutral by design)
- Fixed clippy warnings (unnecessary_sort_by, borrowed expressions)

## [0.1.0] - 2026-09-12

### Added

- Initial release: basic token budget CLI
- Directory scanning with walkdir
- Approximate token counting (chars / 4 heuristic)
- CLI argument parsing with clap
- MIT license (later changed to GPLv3 in 0.2.0)

[Unreleased]: https://github.com/tamaraw01/ctx-budget/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/tamaraw01/ctx-budget/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/tamaraw01/ctx-budget/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tamaraw01/ctx-budget/releases/tag/v0.1.0
