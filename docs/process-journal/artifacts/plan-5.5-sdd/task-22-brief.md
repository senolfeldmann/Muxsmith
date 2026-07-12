### Task 22: insta snapshots for CLI rendering (#2)

**Files:**
- Modify: `crates/muxsmith-cli/Cargo.toml` (dev-dep `insta`, exact-pin, registry-verified)
- Create: snapshot tests replacing the wording-coupled asserts in `crates/muxsmith-cli/tests/cli_validate.rs:18,52` and successors (grep `contains(` across cli tests for the class)
- Create: `tests/snapshots/` (committed)

- [ ] Step 1: Redaction settings first (`insta::with_settings!` filters for absolute paths, mkvmerge version strings, durations) - paths WILL differ per machine; this is load-bearing, not optional.
- [ ] Step 2: Convert the flagged assert sites to `assert_snapshot!` (EN locale pinned for snapshots); review each snapshot content before accepting (`cargo insta review` locally; snapshots are committed artifacts).
- [ ] Step 3: CI runs strict compare (default behavior; assert no INSTA_UPDATE in CI env). Full gate; commit `test(cli): insta snapshots replace wording-coupled asserts (spec §10, #2)`.

