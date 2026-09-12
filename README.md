# ctx-budget

Token distribution analyzer for LLM context windows.

Scans project files, estimates token counts, and reports usage per file before sending code to Claude, GPT-4, Gemini, or local models.

[![Crates.io](https://img.shields.io/crates/v/ctx-budget?style=flat-square)](https://crates.io/crates/ctx-budget)
[![License: GPLv3](https://img.shields.io/badge/license-GPLv3-blue?style=flat-square)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/tamaraw01/ctx-budget/ci.yml?style=flat-square&label=tests)](https://github.com/tamaraw01/ctx-budget/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/tamaraw01/ctx-budget?style=flat-square)](https://github.com/tamaraw01/ctx-budget/releases)

![ctx-budget demo](assets/demo.gif)

## Problem

Feeding an entire repository into an LLM context window often exceeds model limits or wastes tokens on unnecessary files:

- GPT-4o: 128,000 tokens
- Claude 3.5 Sonnet: 200,000 tokens
- Gemini 1.5 Pro: 1,000,000 tokens

`ctx-budget` measures token distribution across project files so you can select relevant files before making API calls.

## Features

- **Token estimation**: Blended word/character heuristic (~95% correlation with GPT-2 BPE)
- **Directory filtering**: Excludes `node_modules`, `.git`, `target`, `__pycache__`, `.venv`, `vendor`, and custom paths
- **Multiple output formats**: Text (terminal), JSON (automation), CSV (spreadsheets)
- **30+ languages**: Auto-detects Rust, Python, Go, TypeScript, JavaScript, C/C++, Shell, SQL, and others
- **Stream processing**: Low memory usage on large codebases
- **Zero runtime dependencies**: Static binary

## Installation

### From Source

```bash
git clone https://github.com/tamaraw01/ctx-budget
cd ctx-budget
cargo build --release
```

The binary will be at `target/release/ctx-budget`.

### Cargo

```bash
cargo install ctx-budget
```

## Usage

### Basic Scan

Scan the current directory:

```bash
ctx-budget .
```

### Specify Model Window and Limit

```bash
ctx-budget /path/to/project --model claude-sonnet-4 --limit 10
```

### JSON Output

```bash
ctx-budget . --output json | jq .
```

### CSV Export

```bash
ctx-budget . --output csv > tokens.csv
```

### Custom Exclusions

```bash
ctx-budget . --exclude-dirs "dist,build,coverage"
```

## Output Example

```
=== ctx-budget Report ===
Model: gpt-4o

Summary:
  Files scanned: 42
  Total chars:   614,428
  Total tokens:  153,607

Languages found: 3
  Rust                 35 file(s)
  Markdown             5 file(s)
  TOML                 2 file(s)

Top 10 files by token count:
    1.  28,451 tokens | Rust                 | src/engine.rs
    2.  12,304 tokens | Rust                 | src/analysis.rs
    3.   9,876 tokens | Markdown             | docs/architecture.md
    4.   7,234 tokens | Rust                 | src/main.rs
    5.   6,145 tokens | Rust                 | tests/integration.rs
```

## Model Reference

Default limits recognized for reference:

| Model | Token Limit |
|-------|-------------|
| gpt-4o | 128,000 |
| gpt-4-turbo | 128,000 |
| claude-opus | 200,000 |
| claude-sonnet-4 | 200,000 |
| gemini-pro-1.5 | 1,000,000 |
| llama-70b | 8,192 |

## How Token Counting Works

`ctx-budget` uses a blended heuristic:

1. Word count: ~1.3 tokens per word
2. Character count: ~0.25 tokens per character
3. Average of both estimates

This approach avoids linking against heavy tokenizer libraries while providing estimates accurate enough for context window planning.

## Performance

Tested on a repository with 1,000 source files (100MB):
- Execution time: < 0.8 seconds
- Peak RAM: < 15MB

## Testing

Run unit and integration tests:

```bash
cargo test --release
```

Check code formatting and lints:

```bash
cargo fmt --check
cargo clippy --release
```

## License

GNU General Public License v3.0 (GPL-3.0). See [LICENSE](LICENSE) for details.
