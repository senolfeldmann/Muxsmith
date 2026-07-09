# Task 1 report: restructure `tracks` into `{ unmatched, rules }`

Plan 3.5 (mkvtoolnix parity), Task 1. Note: this file's slot was previously
used by an unrelated Plan 2 task also numbered 1 ("identify parses
attachments and chapters"); that content is superseded below.

## Status

DONE. Commit `91b19eb` on `master`: "refactor(profile)!: tracks becomes a { unmatched, rules } block (D20)".

## What was implemented

Followed the brief's TDD steps in order.

**RED (Step 1-2):** Added `tracks_block_parses_and_unmatched_defaults_to_drop` and
`tracks_unmatched_keep_parses` to `crates/muxsmith-core/tests/profile_load.rs`
(exact code from the brief). `cargo test -p muxsmith-core --test profile_load
tracks_block -- --nocapture` failed to compile with `no field 'unmatched'/'rules'
on type Vec<TrackRule>`, confirming RED before any implementation change.

**GREEN (Steps 3-4):**
- `crates/muxsmith-core/src/profile/model.rs`: `Profile.tracks` changed from
  `Vec<TrackRule>` to `TracksCfg`; added `TracksCfg { unmatched: KeepDrop
  (default via `drop_policy()`), rules: Vec<TrackRule> }` next to
  `AttachmentsCfg`, doc comments on the struct and both fields (satisfies
  `#![deny(missing_docs)]`; compile is proof, since a missing doc there is a
  hard compile error, not a lint warning).
- `crates/muxsmith-core/src/planner.rs`: `profile.tracks` -> `profile.tracks.rules`
  at all 5 sites the brief named (lines 286, 364, 983, 1031, 1169 pre-edit);
  also updated the `Assignment.rule_index` doc comment ("Index into
  `profile.tracks`" -> "`profile.tracks.rules`") since it referenced the field
  directly and would otherwise silently go stale.
- `crates/muxsmith-core/src/profile/validate.rs`: both sites (`is_empty()`,
  the rules loop) updated; the `format!("tracks[{i}]")` diagnostic-path string
  stays literal `tracks[i]` (brief did not ask to change it, and nothing reads
  `.rules` there).
- `crates/muxsmith-core/src/profile/lint.rs`: `.tracks` -> `.tracks.rules` at
  the one production site (line 21).

**YAML migration (Step 5) - exhaustive, verified against a fully green
`cargo test --workspace`:**
- Fixtures: `crates/muxsmith-core/tests/fixtures/reference.yaml`,
  `crates/muxsmith-cli/tests/fixtures/{good,bad}.yaml`.
- `crates/muxsmith-core/tests/profile_load.rs`: field-access fixes
  (`p.tracks.len()` -> `p.tracks.rules.len()` etc., 4 sites) plus inline YAML
  **and** the JSON literal in `json_profile_parses_identically_to_yaml`
  (`"tracks": [...]` -> `"tracks": { "rules": [...] }` - the brief's
  transformation rule is YAML-specific; the JSON side needed the equivalent
  structural change to keep asserting YAML/JSON parse to the same `Profile`).
- `crates/muxsmith-core/tests/{planner_resolution,suggestions,command_integration}.rs`,
  `crates/muxsmith-cli/tests/cli_validate.rs`, and the `#[cfg(test)]` module in
  `crates/muxsmith-core/src/profile/lint.rs`: column-0 `tracks:` blocks,
  migrated by inserting `  rules:` and reindenting the following list block by
  2 spaces (mechanical, done via a small Python pass for the two largest files
  - 38 blocks in `planner_resolution.rs`, 1 in `command_integration.rs` - then
  hand-verified; the rest by direct `Edit`).
- `crates/muxsmith-core/tests/validate_semantics.rs`: the `profile(tracks_yaml)`
  helper reindents its argument programmatically (`tracks_yaml.lines().map(|l|
  format!("  {l}"))`) rather than requiring every caller string to change, per
  the brief's instruction to "update the helper... covering all its callers
  together" - all ~12 callers were left untouched. Also fixed
  `empty_tracks_list_is_rejected`'s `tracks: []` (not covered by the brief's
  generic sequence-transform, since it's an inline empty list, not a
  sequence) to `tracks:\n  rules: []`, and the two raw `attachment_*` test
  blocks.
- `crates/muxsmith-core/tests/validate_hardening.rs`: `HEAD` constant gained
  `  rules:\n`; all 9 `format!("{HEAD}  - match: ...")` call sites bumped to
  `{HEAD}    - match: ...` (shared substring, `replace_all`).
- `crates/muxsmith-core/tests/validate_structure.rs` - the one file needing
  genuine care beyond mechanical reindent:
  - `BASE` constant and its 6 `format!("{BASE}  - source:\n      external:
    ...\n    match: ...")` call sites bumped uniformly (+2 to every line in
    the appended snippet, not just its head, since the snippet is itself a
    nested structure that must move as a unit).
  - `numbered_group_fields_are_accepted`'s own standalone block, reindented
    normally.
  - `unknown_keywords_are_flagged` does `BASE.replace("- match: {...}",
    "- source: secondary\n    match: {...}")`. After bumping `BASE`, the
    search substring still matches (leading whitespace isn't part of the
    matched text), but the replacement's **hardcoded** second-line indent
    (`"    match:"`, 4 spaces) was sibling-aligned to the *old* 2-space list
    nesting; left as-is it would misalign against the new 4-space list
    nesting. Bumped the replacement string's own indent to `"      match:"`
    (6 spaces) to keep `source:`/`match:` as siblings under the new depth.
    Confirmed by running the test (checks `InvalidKeyword`, which requires
    the YAML to actually parse as a well-formed track rule with an invalid
    `source` value - a silent misindent would have surfaced as a parse
    failure or a wrong diagnostic, not a false pass).
- `crates/muxsmith-cli/tests/dry_run_cli.rs`: 5 single-line
  (`"...\ntracks:\n  - match: ...\n"`) embedded profile strings, including
  one with 3 rules; edited directly (4 share an identical suffix, fixed via
  one `replace_all`; the 3-rule one by hand).

**Step 7 (spec):** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
4.1's reference example now nests the rule list under `rules:` with a new
`unmatched: drop # keep | drop; default drop` line; 4.5 gained a lead sentence
stating `tracks` is now a `{ unmatched, rules }` block and what `unmatched`
means, and the "List order in `tracks`" line now reads "`tracks.rules`".
Section 4.9's line describing "unmatched tracks are always dropped" was left
untouched - out of the brief's named scope (4.1, 4.5 only), and still
literally true after Task 1 since nothing consumes `keep` yet (Task 2's job).

**Step 8 (gate + commit):** All four gates green - `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test
--workspace` (208 tests across all binaries, 0 failed), `cargo deny check`
("advisories ok, bans ok, licenses ok, sources ok"). Committed as `91b19eb`
with the exact trailer requested.

## Files touched

- `crates/muxsmith-core/src/profile/model.rs` (structural change)
- `crates/muxsmith-core/src/planner.rs`, `src/profile/validate.rs`,
  `src/profile/lint.rs` (consumers + lint's own inline tests)
- `crates/muxsmith-core/tests/fixtures/reference.yaml`,
  `crates/muxsmith-cli/tests/fixtures/{good,bad}.yaml`
- `crates/muxsmith-core/tests/{profile_load,planner_resolution,suggestions,
  validate_semantics,validate_structure,validate_hardening,
  command_integration}.rs`
- `crates/muxsmith-cli/tests/{dry_run_cli,cli_validate}.rs`
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

**Explicitly untouched, verified by direct inspection (not just brief
citation):** `crates/muxsmith-core/src/identify.rs`,
`crates/muxsmith-cli/src/commands/identify.rs`, and
`crates/muxsmith-core/src/command.rs` - the brief listed these as "cfg(test)
modules" to check, but a targeted grep found zero `tracks:` YAML content in
any of them; their only `.tracks` occurrences are `Identification.tracks` /
`self.tracks` (an unrelated field the brief explicitly says to leave alone).
No edits were needed there.

## Self-review findings

- Confirmed via `git diff | grep` (Unicode ranges for em/en-dash, curly
  quotes, ellipsis, NBSP) that no forbidden typography glyphs entered the
  diff; umlauts/`Ş` were not touched (none were near the edited lines).
- Confirmed via `#![deny(missing_docs)]` (build succeeds) that `TracksCfg` and
  both its fields carry doc comments; a missing one would have been a hard
  compile error, so green build is direct proof, not just a visual check.
- Grepped the whole repo for any remaining un-migrated `tracks:\n  - `
  pattern after the fix: all remaining hits are in
  `docs/process-journal/artifacts/plan-1-sdd/*` (frozen historical task
  briefs for already-shipped work), `docs/superpowers/plans/2026-07-08-plan-1-*.md`
  and `2026-07-09-plan-2-*.md` (completed, historical plans), and the
  *current* Plan 3.5 document's own Step 5 description (its "before" example
  illustrating the transformation rule itself). None of these are code,
  fixtures, or the two named spec sections, so per repo convention (dated/
  historical artifacts are not retroactively rewritten) they were correctly
  left alone.
- Diagnostic `config_path` strings (`"tracks[0].match..."`,
  `"tracks[0].changes.language"`) were deliberately left as literal
  `tracks[i]`, matching that `validate.rs`'s and `planner.rs`'s `format!`
  calls still emit that literal prefix (the brief's Step 4 code didn't touch
  those format strings) - verified by running every test that asserts on
  `config_path`, all green.
- Untracked files `HANDOFF.md` and
  `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` sitting in
  the working tree are unrelated artifacts from a different plan/session;
  confirmed by reading their contents before deciding not to `git add -A`
  them - staged only the 17 Task 1 files by name instead.

## Concerns

None outstanding. The field is public and structurally in place but
inert (`unmatched` is parsed and defaulted, not yet read by `command`/
`planner` logic) exactly as scoped - that wiring is Task 2's job per the
brief.
