# Task 2 report: `tracks.unmatched: keep` semantics in the planner and `command`

(Plan 3.5, Task 2. This overwrites a stale `task-2-report.md` left over from
Plan 3's Task 2, "generalize the matcher over a `Matchable` trait" - that
content belonged to a different plan's task numbering and is superseded.)

## Implemented

- `Plan.keep_unmatched: bool` added to the struct (`crates/muxsmith-core/src/planner.rs`, after `title`), with a doc comment satisfying `#![deny(missing_docs)]`.
- Populated at the `Plan` construction site in `resolve_file` (`planner.rs`, the `output.map(|output| Plan { ... })` closure): `keep_unmatched: matches!(profile.tracks.unmatched, crate::profile::model::KeepDrop::Keep)`.
- `push_track_selection` in `crates/muxsmith-core/src/command.rs` now returns immediately, before touching `CATEGORIES`, when `plan.keep_unmatched && source == plan.source.as_path()`. No track-selection flags (`--video-tracks`/`--no-video`/etc.) are emitted for the primary group in that case; mkvmerge's own default (keep everything) then applies. The check is scoped to the primary source only, so donor groups keep their normal per-category selection.
- Property options (`push_track_properties`) and `--track-order` (`push_track_order`) are untouched by this change: they iterate `plan.assignments` directly and don't go through `push_track_selection`, so a matched track on the primary still gets its property flags and its `--track-order` position, exactly as before.

## TDD evidence

**RED** (Step 1-2 of the brief): added `keep_unmatched_suppresses_primary_selection_flags` to `crates/muxsmith-core/tests/command.rs` (uses the file's existing `p()` helper instead of `PathBuf::from` directly, for consistency with the rest of the file; otherwise matches the brief's test verbatim in intent). First run:

```
$ cargo test -p muxsmith-core --test command keep_unmatched -- --nocapture
error[E0560]: struct `muxsmith_core::planner::Plan` has no field named `keep_unmatched`
```

confirmed the field didn't exist yet.

**GREEN** (Step 3-5): after adding the `Plan` field, the construction-site wiring, and the early return in `push_track_selection`:

```
$ cargo test -p muxsmith-core --test command keep_unmatched -- --nocapture
test keep_unmatched_suppresses_primary_selection_flags ... ok
```

## Ripple (Plan struct-literal breakage)

Adding a new field without a `Default` broke every existing `Plan { ... }` literal, all in `crates/muxsmith-core/tests/command.rs` (6 sites: `global_and_single_video_group`, `unmatched_donor_rule_opens_no_input_group`, `per_track_properties_and_multi_group`, `boolean_and_string_value_encoding`, and the two shared builders `single_group_plan`/`multi_group_plan`). Added `keep_unmatched: false` to each (drop-path unchanged). Verified there are no other `Plan { ... }` literals in the workspace (`tests/planner_resolution.rs` and `tests/command_integration.rs` both construct `Plan` only indirectly, via the planner, not as a struct literal) with `grep -rn '\bPlan {' --include='*.rs' crates/`.

`cargo test --workspace`: all suites green (muxsmith-core lib/unit tests, `command` golden tests including the new one, `command_integration`, `planner_resolution`, validate_* suites, xtask, doctests). No failures anywhere in the workspace.

## Gate

All four green:
- `cargo test --workspace`: pass (0 failed across every crate/test binary).
- `cargo fmt --all --check`: initially flagged the multi-line `matches!` call in `planner.rs` and the `assert!` formatting in the new test; ran `cargo fmt --all` to apply, then `--check` passed clean. (rustfmt also reordered `keep_unmatched` ahead of `assignments` in the six struct literals; harmless, struct-literal field order is not declaration order.)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: `Plan.keep_unmatched` field + doc comment; populated at the `resolve_file` construction site from `profile.tracks.unmatched`.
- `crates/muxsmith-core/src/command.rs`: early-return guard at the top of `push_track_selection`.
- `crates/muxsmith-core/tests/command.rs`: new golden test `keep_unmatched_suppresses_primary_selection_flags`; `keep_unmatched: false` added to the six pre-existing `Plan { ... }` literals.

Commit: `b57abf4` "feat(planner,command): tracks.unmatched keep passes primary tracks through (D20)".

## Self-review

- Scope check: the guard in `push_track_selection` compares `source == plan.source.as_path()`, i.e. it fires only for the primary's own input group, never for a donor whose path happens to equal the primary's (paths are canonical file paths from discovery, so this can't alias in practice, and it's the same comparison `push_group_attachments` already uses for the primary/donor distinction).
- Confirmed by construction, not just by the golden test, that property options and `--track-order` are wired from `plan.assignments` directly (`push_track_properties`, `push_track_order` in `command.rs`) and never call `push_track_selection`, so suppressing primary selection flags cannot suppress those.
- Did not touch `resolve_file`'s diagnostics or any other Task 1 behavior; `keep_unmatched` is purely additive to `Plan` and consumed only in the one guard.
- Left `HANDOFF.md` and `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` (untracked, pre-existing, unrelated to this task) out of the commit; staged and committed only the three files this task actually touched, not `git add -A`.

## Concerns

None outstanding. One judgment call worth flagging: the brief's Step 1 test snippet uses `PathBuf::from(...)` inline; I used the file's own `p()` helper for the same paths instead, since every other test in the file does. Semantically identical, just idiomatic-for-the-file rather than a literal copy of the brief's snippet.
