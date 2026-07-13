# Task 2 report: planner.rs + report/mod.rs + ADR D36 (Stream B, first)

**Status: DONE**

Worktree `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b` (branch `plan-5.6-b`).
All 11 checkbox items plus the ADR implemented, mechanically, with zero
intended behavior change except the named `Diagnostic.claimants` wire
extension. Nine-part gate green before each commit.

## Commits

- `403e573` refactor(planner): idiom/dup/yagni/stdlib cleanups (plan 5.6 T2)
- `89f346b` feat(report): structural claimants on OverlappingRules (D36)

Both unsigned (`git -c commit.gpgsign=false`), explicit staging, no `git add -A`,
no push. Commit 1 stages `planner.rs` only; commit 2 stages `report/mod.rs`,
`planner.rs` (the two claimants hunks), and the ADR. The refactor commit was
made buildable-in-isolation by temporarily reverting the two claimants-feature
hunks in planner.rs (the file cannot be interactively hunk-split), then
reapplying them for the feature commit; each committed snapshot builds standalone.

## Per-item implementation

Anchors re-verified against the code before each edit; line numbers below are
pre-edit.

1. **planner.rs:526 dup** - added `impl Assignment { fn unmatched(rule_index,
   source) -> Self }`; the six placeholder literals (526/563/583/603 with
   `primary.path.clone()`, 646/675 with `source_path`) are each now a one-line
   `assignments.push(Assignment::unmatched(ri, <source>))`.
2. **planner.rs:714 idiom** - `matches!(profile.tracks.unmatched,
   crate::profile::model::KeepDrop::Keep)` -> `profile.tracks.unmatched ==
   KeepDrop::Keep` (import already at line 19; `KeepDrop: PartialEq`, same form
   already used in `resolve_tags`).
3. **planner.rs:1312, :1565 yagni** - deleted the two dead
   `#[allow(clippy::too_many_arguments)]` on `suggest` (6 args) and
   `partition_for_overlap` (7 args); `partition_for_rule`'s (9 args) kept.
   Verified empirically: clippy `-D warnings` on pinned 1.96.1 stays green
   (lint fires at 8+).
4. **planner.rs:1646ff idiom** - added `PropValue, Track` to the
   `crate::identify` import; collapsed every `std::collections::BTreeSet/BTreeMap`
   (1646, 1825, 1846, 1903, 1952) to the already-imported bare names and every
   `crate::identify::Track/PropValue` (1653, 1665-1680, 1717, 1998-2003) to
   bare `Track`/`PropValue`. The wrapped `seen` decl and the three `props.push`
   calls collapse to single lines (rustfmt).
5. **planner.rs:886/:971 dup** - extracted `fn render_ctx(primary:
   &PrimaryFile) -> Ctx` (identifier fields + `source_stem`); `render_output`
   and `resolve_title` both call it. `resolve_title`'s lockstep comment now
   points at the shared fn instead of re-describing the mirror ("by
   construction"). Added `Ctx` to the `crate::template` import.
6. **planner.rs:368 stdlib rider** - `known.contains(&ext.to_ascii_lowercase())`
   -> `known.iter().any(|k| k.eq_ignore_ascii_case(ext))`. Pure refactor: the
   `known` set is already lowercased at `runtime.rs:343`
   (`parse_list_types`), so case-insensitive comparison against a lowercase set
   is behaviorally identical, drops the per-entry `String` allocation, and
   matches the house `eq_ignore_ascii_case` form (matcher.rs, strip_mkv_suffix).
7. **planner.rs:1971 stdlib** - `rule_index_of` now
   `config_path.split_once("tracks[")?.1.split_once(']')?.0.parse().ok()`
   (rustfmt wrapped to a method-chain block). Same first-`tracks[`/first-`]`
   semantics as the `find`-based original.
8. **planner.rs:1965 idiom** - `diag_signature` keys on a
   `BTreeMap<(String, String, String), usize>` tuple `(code.key().to_string(),
   config_path.clone(), file)` instead of the `format!("{}|{}|{}")` string.
   The four `base_sig` param types (1491, 1825, 1846, 1903) propagate to the
   tuple key; `no_regression`'s body needed no change (Borrow reflexive on the
   tuple). See the '|'-collision closure note below.
9. **planner.rs:1521 doc (seed T13-m1)** - inserted the verbatim invariant
   comment above the `if let Some(cand) = best` skip in `partition_for_rule`.
10. **report/mod.rs + planner.rs:691-696, :1881 idiom (seed M2)** - added
    `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub claimants:
    Vec<usize>` to `Diagnostic`, initialized in `new`; added builder
    `Diagnostic::with_claimants(&[usize])` that sets BOTH the structural field
    and the rendered `rules` display param from one slice (cannot diverge). The
    OverlappingRules production site calls `.with_claimants(rules)`;
    `overlap_conflicts` reads `d.claimants.clone()` instead of splitting/
    re-parsing. `rule_index_of` stays for the AmbiguousRule config_path sites.
11. **ADR D36** - `docs/superpowers/specs/2026-07-13-plan-5.6-decisions.md`
    created, matching the plan-5.5 decisions-file structure (decision /
    rationale / rejected alternatives / interface-wire note). Records the
    structural-claimants decision, the display-re-parse-fragility rationale,
    the two rejected alternatives (keep re-parsing; shared const format), and
    the wire note (JSON gains `claimants` on overlapping-rules only, omitted
    elsewhere via skip_serializing_if).
12. **report/mod.rs:165 doc (seed M3)** - UnknownExtension rustdoc "Batch-wide,
    once per batch;" -> "Checked once per batch; emitted once per offending
    list entry at its own config path (no dedup by extension value);". Verified
    accurate against `validate_extension_list` (per-entry loop, one diagnostic
    per index, no value dedup).

## '|'-collision closure (routed-out correctness item, planner.rs:1965)

The tuple-key `diag_signature` fix closes the routed-out '|'-collision
correctness item at the same line. The old key `format!("{}|{}|{}",
d.code.key(), d.config_path, file)` could collapse two distinct diagnostics
into one signature if a `config_path` or file path itself contained a `|`
(a plausible character in a filename), corrupting the multiset counts the D6/
D33 acceptance criterion depends on. The `(String, String, String)` tuple key
has no delimiter and cannot collide. Stated here explicitly for the routed-items
review to check off.

## Gate results (nine parts, per BUILDING.md)

Run in the foreground. All green on the full feature state immediately before
commit 2; the Rust gate re-run green on the refactor-only state before commit 1
(JS/deny parts unaffected - this task touches zero JS/frontend/dependency files).

1. `cargo fmt --all --check` - OK
2. `cargo clippy --workspace --all-targets -- -D warnings` - OK (no warnings)
3. `cargo test --workspace` - OK: all suites pass, 0 failed, 0 ignored (no
   mkvmerge-gated test self-skipped)
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - OK
5. `cargo deny check` - advisories ok, bans ok, licenses ok, sources ok
6. `pnpm lint` - OK
7. `pnpm build` - OK
8. `pnpm check:i18n` - ok (179 catalog ids, parity checked)
9. `pnpm test:e2e` - 6 passed

## Tests/snapshots asserting OverlappingRules JSON

Searched `crates/ e2e/ src-tauri/ tests/` for `overlapping-rules` / `claimants`
in JSON goldens and insta snapshots: none exist. All insta snapshots are
human-rendered text (cli_validate, dry_run human, run_live summary); no JSON
document snapshot contains an OverlappingRules diagnostic. `Diagnostic` is
`Serialize`-only (no `Deserialize`), so `#[serde(default)]` is inert today and
no round-trip test is affected. Nothing needed updating. The
`overlapping_rules_names_every_claimant` test (planner_resolution.rs) still
passes: `with_claimants` sets the `rules` param byte-identically.

## Self-review

- **Completeness**: all 11 checkbox items + ADR done; grep confirms zero
  residual `crate::identify::PropValue/Track` or `std::collections::BTree*`
  fully-qualified paths in planner.rs.
- **Quality**: `render_ctx` doc and `resolve_title` doc state the lockstep is
  now by construction; `with_claimants` doc states it is the single point
  populating both representations; the `diag_signature` comment explains the
  tuple key's collision-safety.
- **Discipline**: no behavior change beyond the named claimants surface; each
  commit builds standalone; unsigned; explicit staging; report/json.rs (another
  stream's file) untouched - the claimants field surfaces through its existing
  `serde_json::to_value(d)` path automatically.
- **Pristine test output**: no warnings, no skips, no ignored.

## Surfaced patterns / deviations for the house ledger

- **New pattern (single-builder co-derived display+structural field).**
  `Diagnostic::with_claimants` sets a structural field and its rendered display
  param from one source so they cannot diverge. This is the concrete instance
  behind ADR D36 and generalizes core-37 (core emits codes + structured params;
  formatting derives from the structured data, machine-consumers read the
  structured data, never re-parse the rendered string). Candidate ledger entry
  under `core` if it recurs; flagged, not self-promoted.
- **No product-scope or process deviation.** All changes are technical-code
  idiomacy fixes plus one controller-sanctioned wire extension (D36) already
  authorized by the plan. No convention-file edits were needed; the eq_ignore /
  bare-import / builder choices all conform to existing house patterns
  (matcher.rs eq_ignore, top-of-file `use std::collections::{BTreeMap, BTreeSet}`,
  core-47 with_severity builder precedent).
- **Method note for the controller.** The refactor/feature split of a single
  file was achieved by revert-then-reapply of two small hunks (interactive
  `git add -p` is unavailable in this harness). Each commit was gated and builds
  in isolation. If the wave wants strictly-non-interactive per-file splitting as
  a standing method, this is the pattern used.
