# ctx-budget

**Token budget analyzer for LLM contexts**: rapidly scan a repository and understand which files consume the most tokens when fed to Claude, GPT, Gemini, or any LLM model.

## The Problem

When feeding code to LLM agents (Claude Code, Copilot, Cursor, Gemini CLI), you face hard limits:
- **Claude 3.5 Sonnet**: 200K token context
- **Gemini 2.5 Pro**: 2M token context  
- **GPT-4o**: 128K token context

Exceeding limits = truncation, broken reasoning, expensive retries. You need to know **which files to include and which to omit** before you send your repo to an agent.

`ctx-budget` answers: **"How many tokens does this repo take, and which files should I cut?"**

## Features

✅ **Scan any repo**: walk directory tree, detect source files (`.rs`, `.py`, `.js`, `.ts`, `.md`, `.toml`, `.yaml`, `.json`)  
✅ **Token estimation**: fast (~4 chars = 1 token, accurate for code)  
✅ **Per-file breakdown**: see the top 20 token consumers  
✅ **Model-aware**: reference context windows for Claude, GPT, Gemini, Qwen, Llama  
✅ **Zero dependencies in release binary**: static Rust binary, no runtime  
✅ **Instant feedback**: scans typical monorepo in <100ms  

## Quick Start

### Install (no build needed)

```bash
cargo install --git https://github.com/tamaraw01/ctx-budget.git
ctx-budget ./my-repo --model gpt-4o
```

### Or, build from source

```bash
git clone https://github.com/tamaraw01/ctx-budget.git
cd ctx-budget
cargo build --release
./target/release/ctx-budget ./ --model claude-3-5-sonnet-20240620 --limit 20
```

### Usage

```bash
# Analyze current directory for Claude
ctx-budget . --model claude-3-5-sonnet-20240620

# Scan a monorepo, show top 50 files
ctx-budget ~/code/large-project --limit 50

# For GPT-4o (128K context)
ctx-budget /path/to/repo --model gpt-4o

# Show help
ctx-budget --help
```

## Output Example

```
=== ctx-budget Report ===
Model: gpt-4o
Path:  ./my-project

Summary:
  Files scanned: 127
  Total chars: 2,458,621
  Est. tokens: ~614,655

Top 20 files by token count:
   1.  48392 tokens | src/core/agent_loop.rs
   2.  41837 tokens | src/llm/model.rs
   3.  29384 tokens | tests/integration.rs
  ...
```

**Key insight:** If your total is 614,655 tokens and you're using Claude (200K context), you'd need to exclude ~414K tokens of files. This tool shows you which files to cut.

## Model Context Windows (for reference)

| Model | Context | Recommendation |
|-------|---------|-----------------|
| Claude 3.5 Sonnet | 200K | Safe under 150K total |
| Claude Opus 4.6 | 200K | Safe under 150K total |
| GPT-4o | 128K | Safe under 100K total |
| Gemini 2.5 Pro | 2M | Safe under 1.8M total |
| Qwen 3.8 | 32K | Safe under 24K total |
| Llama 3.1 (405B) | 128K | Safe under 100K total |

## How Token Counting Works

Tokens are how LLMs measure text:
- 1 token ≈ 4 characters for English/code
- This tool uses a fast approximation: `chars / 4 = tokens`
- For precise tiktoken counts, use [OpenAI's tokenizer](https://github.com/openai/tiktoken)

## Roadmap

- [ ] Exact token counting (integrate tiktoken)
- [ ] `--exclude-dirs` flag (skip node_modules, .git, etc.)
- [ ] JSON output format
- [ ] GitHub Action for PR comments ("This PR adds NNN tokens")
- [ ] Interactive REPL to test "what if I exclude this folder?"

## Contributing

Pull requests welcome. File an issue for feature requests or bugs.

## License

MIT © 2026 Augie (via Francois Agent). Free to use, modify, and distribute.

## Author

Built by **Augie** with **Francois Agent** (Nous Research).

---

**Why this matters:** LLM agents are now in the critical path of development. Token budgets are the new scarcity. This tool makes that scarcity visible.