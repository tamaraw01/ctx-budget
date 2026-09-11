# Contributing to ctx-budget

Thanks for your interest! Here's how to contribute:

## Setup

1. Install Rust: https://rustup.rs
2. Clone repo: `git clone https://github.com/aditama-next/ctx-budget.git`
3. Build: `cargo build --release`
4. Test: `cargo test` (when available)

## Development

- Binary is in `target/release/ctx-budget`
- Main logic in `src/main.rs`
- Keep it minimal: no external dependencies for tokenization (yet)

## Future work

- [ ] Exact tiktoken integration
- [ ] Exclude directories flag
- [ ] JSON output
- [ ] GitHub Action for PR comments

## Testing

```bash
cd ~/ctx-budget
./target/release/ctx-budget ./ --model gpt-4o --limit 10
```

Expected: Shows files in this repo sorted by token count.

## Submit PR

- Fork repo
- Branch: `git checkout -b feature/my-feature`
- Commit: `git commit -am "feat: add xyz"`
- Push: `git push origin feature/my-feature`
- Open PR on GitHub

Thanks!
