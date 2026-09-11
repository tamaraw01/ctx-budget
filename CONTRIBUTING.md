# Contributing to ctx-budget

Thank you for your interest in contributing! This guide explains our workflow, standards, and how to get started.

## Our Philosophy

- **Simplicity first**: Delete > Add. If it can be stdlib, don't add a dependency.
- **Quality over speed**: We follow TDD (test-first) rigorously. Every feature starts with a failing test.
- **Zero dependencies**: Keep the binary lightweight and portable.
- **Production-ready**: All code ships production-quality. No shortcuts.

## Quick Start

### 1. Fork & Clone
```bash
git clone https://github.com/YOUR_USERNAME/ctx-budget
cd ctx-budget
```

### 2. Set Up Local Environment
```bash
# Verify Rust toolchain
rustc --version  # 1.98.0+
cargo --version  # 1.98.0+

# Run tests to verify setup
cargo test --release
```

### 3. Pick an Issue or Feature
- Start small: documentation, examples, minor optimizations
- Graduate to: new features, performance improvements
- Coordinate large changes by opening an issue first

## Development Workflow

### For Bug Fixes

1. **Open an issue** describing the bug (unless already reported)
2. **Create a branch**: `git checkout -b fix/issue-123-name`
3. **Write a failing test** that reproduces the bug
4. **Verify failure**: `cargo test -- --nocapture`
5. **Fix the bug** (minimal code)
6. **Verify tests pass**: `cargo test --release`
7. **Commit**: `git commit -m "fix: brief description"`
8. **Push & open PR**: Link the issue

### For New Features

1. **Discuss first**: Open an issue proposing the feature, get feedback
2. **Design**: Agree on scope and API in the issue thread
3. **Create branch**: `git checkout -b feat/feature-name`
4. **TDD Cycle**:
   - Write failing test(s) for the feature
   - Verify tests fail with clear error
   - Implement minimal code to pass
   - Verify all tests pass (no regressions)
   - Refactor if needed (keep tests green)
5. **Document**: Add doc comments, update README if user-facing
6. **Commit**: `git commit -m "feat: brief description"`
7. **Push & open PR**

## Testing Requirements

**All code must pass:**

```bash
# Unit + integration tests
cargo test --release

# Clippy (linter)
cargo clippy --all-targets --release

# Format check
cargo fmt --check

# Doc tests
cargo test --doc
```

### Test Guidelines

- **One test, one behavior**: Each test verifies one thing
- **Clear names**: `test_name` describes what is tested, not how
- **Real scenarios**: Use real code paths, not mocks (unless genuinely necessary)
- **Edge cases**: Test empty files, large files, mixed encodings, permissions

Example:

```rust
#[test]
fn tokenize_mixed_code_and_comment() {
    let code = "fn main() {\n    // TODO: implement\n}";
    let tokens = count_tokens(code);
    
    // Verify the estimate is reasonable (5-15 tokens for this snippet)
    assert!(tokens >= 5 && tokens <= 15, 
        "expected 5-15 tokens, got {}", tokens);
}
```

## Code Style

### Rust

- Follow `rustfmt` (automatic):
  ```bash
  cargo fmt
  ```

- Follow `clippy` (lint checks):
  ```bash
  cargo clippy --all-targets --release
  ```

- **No unsafe code** except in documented, safety-justified cases

- **Error handling**: Use `anyhow::Result` for errors, propagate with `?`
  ```rust
  fn read_file(path: &Path) -> Result<String> {
      fs::read_to_string(path)
          .map_err(|e| anyhow!("failed to read {}: {}", path.display(), e))
  }
  ```

- **Doc comments**: Public items must have `///` comments
  ```rust
  /// Count tokens in text using GPT-2 tokenizer approximation.
  /// Returns ~95% accurate estimate without external dependencies.
  pub fn count_tokens(text: &str) -> usize { ... }
  ```

### Documentation

- **README updates**: If user-visible behavior changes
- **Doc comments**: For public APIs
- **Examples**: Real-world usage in `examples/`
- **Comments**: Explain *why*, not *what* (code shows what)

## Commit Messages

Follow conventional commits:

```
<type>: <brief description>

<optional body with more detail>
```

Types:
- `feat`: new feature
- `fix`: bug fix
- `refactor`: code cleanup
- `docs`: documentation
- `test`: test additions
- `perf`: performance improvement
- `ci`: CI/CD changes
- `chore`: maintenance

Examples:
```
feat: add streaming support for files > 100MB

fix: handle invalid UTF-8 gracefully without panic

docs: add benchmarks section to README

test: add property-based tests for tokenizer accuracy
```

## Pull Request Process

1. **Push your branch**
2. **Open PR** with:
   - Clear title (follows commit message format)
   - Description of changes
   - Link to related issue (`Fixes #123`)
   - Evidence of testing (paste test output)
   
3. **Respond to feedback** (we'll review within 48h)
4. **Keep PR updated** if repo changes
5. **Squash/rebase** if requested for clean history

Example PR description:
```
## Changes

- Add streaming support for files > 100MB
- Reduce memory usage to O(1)
- Add 2 integration tests for large files

## Testing

```
cargo test --release
test result: ok. 12 passed; 0 failed
```

Fixes #45
```

## Common Issues

**"Clippy warnings fail the build"**
- Run `cargo clippy --fix --allow-dirty` to auto-fix
- Manual fixes: follow clippy's suggestions

**"Tests pass locally but fail in CI"**
- Ensure you ran `cargo test --release` (not debug)
- Check output encoding on your system (UTF-8 assumed)
- Run integration tests: `cargo test --test integration_tests --release`

**"My PR is blocked on style feedback"**
- Run `cargo fmt` to auto-format
- Run `cargo clippy --fix --allow-dirty` for linting
- Commit the changes and push

## Performance Considerations

ctx-budget prioritizes:
1. **Correctness**: Accurate token counts
2. **Speed**: Scan large repos in seconds
3. **Memory**: Constant memory (streaming)
4. **Simplicity**: Minimal dependencies

When optimizing:
- Profile first: `cargo build --release && time ./target/release/ctx-budget .`
- Benchmark after: Compare before/after
- Avoid premature optimization

## Security

- No network calls (except downloading crates.io)
- No shell escaping (we use `walkdir`, not `find`)
- All file I/O is validated
- Symlink loops are prevented

Report security issues privately: open an issue marked `[SECURITY]` (don't post exploits publicly).

## Licensing

By contributing, you agree your code is licensed under GPLv3 (same as ctx-budget).

---

**Questions?** Open an issue or comment on an existing one. We're here to help!
