# Contributing to ctx-budget

Contributions are welcome. This guide covers how to set up your environment, follow project standards, and submit pull requests.

## Project Principles

- **Minimal dependencies**: Prefer the Rust standard library when possible.
- **Test-driven**: Add failing tests before fixing bugs or building features.
- **Static single binary**: Keep compilation output portable and self-contained.

## Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/tamaraw01/ctx-budget
   cd ctx-budget
   ```

2. Run the test suite:
   ```bash
   cargo test --release
   ```

## Development Workflow

1. Open an issue to discuss proposed changes or features.
2. Create a feature branch:
   ```bash
   git checkout -b feat/your-feature-name
   ```
3. Write unit or integration tests for your change.
4. Implement the minimal code needed for tests to pass.
5. Verify formatting, linting, and tests:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --release
   cargo test --release
   ```
6. Commit using conventional commit format:
   ```bash
   git commit -m "feat: add support for Svelte files"
   ```
7. Push your branch and submit a pull request.

## Testing Guidelines

- Keep tests isolated: one test verifies one behavior.
- Use clear test function names describing expected outcomes.
- Include edge cases: empty files, invalid encodings, symlinks, permission errors.

Example:

```rust
#[test]
fn tokenize_mixed_code_and_comment() {
    let code = "fn main() {\n    // TODO: implement\n}";
    let tokens = count_tokens(code);
    assert!(tokens >= 5 && tokens <= 15, "expected 5-15 tokens, got {}", tokens);
}
```

## Commit Message Format

Use standard commit prefixes:

- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code restructuring without functional changes
- `docs`: Documentation updates
- `test`: Test suite additions or fixes
- `chore`: Build or tooling updates

## Code Style

- Format all code with `cargo fmt`.
- Address all warnings from `cargo clippy --release`.
- Public functions and structs require doc comments (`///`).

## Security

Report security issues through GitHub Security Advisories or privately to repository maintainers. Do not post unpatched vulnerabilities in public issues.

## License

By contributing to `ctx-budget`, you agree that your contributions will be licensed under the GNU General Public License v3.0.
