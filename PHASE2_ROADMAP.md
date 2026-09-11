# ctx-budget Phase 2 — Execution Plan

## Feature 1: Exact Token Counting (GPT-2 Tokenizer)
- **Problem:** ~4 chars ≈ 1 token is approximate. Agents need exact counts.
- **Solution:** Implement BPE tokenizer or use `encoding_rs` crate for O(1) fast tokenization
- **Tests:** Read sample files → count tokens → assert against known-good values
- **Status:** BUILD NOW

## Feature 2: Smart Directory Exclusion
- **Problem:** node_modules, .git, target/ inflate results with build artifacts  
- **Solution:** `--exclude-dirs node_modules,target,.git,...` flag with smart defaults
- **Tests:** Scan repo with/without excludes → verify totals differ correctly
- **Status:** BUILD NOW

## Feature 3: JSON & CSV Output  
- **Problem:** Can't pipe results to jq, pandas, spreadsheets
- **Solution:** `--output json|csv|text` flag; auto-detect TTY for smart defaults
- **Tests:** Run with each format → verify output structure, no errors
- **Status:** BUILD NOW

---

## Definition of Done (per feature)
- ✅ RED test failing (watch it)
- ✅ GREEN code passing test
- ✅ REFACTOR cleanup  
- ✅ Manual smoke test
- ✅ humanizer v2.11.2 pass
- ✅ antislop-code pass
- ✅ Git push

---

## Tech Stack Chosen
- **Token counting:** encoding_rs (pure Rust, no external data)
- **Exclude logic:** regex + walkdir filtering
- **Output:** serde_json, csv crate
- **Tests:** `#[cfg(test)] mod tests` in same files
