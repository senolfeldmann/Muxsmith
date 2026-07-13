# Task 11 report: config_diagnostics helper

## Verified before writing

Both duplicated bodies are byte-identical (confirmed by reading both in
full before touching anything):

```rust
match load::from_file(path) {
    Err(d) => vec![d],
    Ok(profile) => {
        let mut diags = validate::validate(&profile);
        diags.extend(lint::provable_overlaps(&profile));
        diags
    }
}
```

No divergence found, so no BLOCKED report needed. `load::from_file(path:
&Path) -> Result<Profile, Diagnostic>` already picks the format from the
extension internally, so `config_diagnostics_from_file` needed no format
parameter (the brief's "adjust minimally if types demand" contingency
didn't fire).

## New helpers (crates/muxsmith-core/src/profile/validate.rs)

```rust
pub fn config_diagnostics(profile: &Profile) -> Vec<Diagnostic> {
    let mut diags = validate(profile);
    diags.extend(lint::provable_overlaps(profile));
    diags
}

pub fn config_diagnostics_from_file(path: &Path) -> Vec<Diagnostic> {
    match load::from_file(path) {
        Err(d) => vec![d],
        Ok(profile) => config_diagnostics(&profile),
    }
}
```

Both carry rustdoc stating contract/edge-case (not name-echo), per
BUILDING.md's documentation standard. Added imports: `std::path::Path`,
`super::{lint, load}`.

## Consumer diffs

**`crates/muxsmith-cli/src/commands/validate.rs`**: `collect()` body
replaced with `validate::config_diagnostics_from_file(profile_path)`;
import trimmed from `profile::{lint, load, validate}` to `profile::validate`
(neither `lint::` nor `load::` referenced elsewhere in the file).

**`src-tauri/src/lib.rs`**: `validate_profile_body()` body replaced with
`validate::config_diagnostics_from_file(path)`; doc comment updated to
reference the new helper instead of describing the inline steps. Import
line (`profile::{lint, load, validate}`) left untouched: `dry_run_body`
(one of the four planning-pipeline sites, lines ~204-215) still calls
`load::from_file` / `validate::validate` / `lint::provable_overlaps`
directly and was not touched, per the scope boundary.

## Scope boundary respected

Grepped all `provable_overlaps` call sites post-change: the four
planning-pipeline copies (`muxsmith-cli/src/commands/dry_run.rs`,
`muxsmith-cli/src/commands/run.rs`, `src-tauri/src/run.rs`,
`src-tauri/src/lib.rs::dry_run_body`) are untouched and still inline the
two-liner, confirmed by diff review before commit.

## ROADMAP rider

Added to the "Further named inputs (2026-07-12, idiomacy review triage)"
paragraph in `docs/ROADMAP.md`: "plan_pipeline consumes
profile::validate::config_diagnostics (landed Plan 5.6 T11)."

## Gate results (all nine parts, foreground, from repo root)

1. `cargo fmt --all --check` -- clean
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean
3. `cargo test --workspace` -- all green (78 in the CLI/core/gui suites
   relevant here, full workspace all green)
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- clean
5. `cargo deny check` -- advisories ok, bans ok, licenses ok, sources ok
6. `pnpm lint` -- clean (no frontend files touched by this task)
7. `pnpm build` -- clean
8. `pnpm check:i18n` -- ok (12 pre-existing unused-key warnings, unrelated
   to this change)
9. `pnpm test:e2e` -- 7 passed

No behavior change: both consumers produce the identical `Vec<Diagnostic>`
content and order as before (same function calls, same order, now behind
one call each).

## Files changed

- `crates/muxsmith-core/src/profile/validate.rs`
- `crates/muxsmith-cli/src/commands/validate.rs`
- `src-tauri/src/lib.rs`
- `docs/ROADMAP.md`

Commit: `6bd1f33` "refactor(core): shared config-diagnostics helper
(validate+lint funnel)"
