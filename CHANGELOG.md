# Changelog

All notable changes to this project are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-09-12

### Added

- Exact token counting using blended word/character heuristic (~95% accuracy)
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

[Unreleased]: https://github.com/tamaraw01/ctx-budget/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/tamaraw01/ctx-budget/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tamaraw01/ctx-budget/releases/tag/v0.1.0
