# Task 1 review verdict: D38 core - conditional NoTrackRules, PassthroughProfile diagnostic, catalogs, e2e

Reviewer worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-a` (branch `plan58-a`), commit `f8e863e`, review range `ed727af..f8e863e`. Independent review; did not implement.

## Findings

None. No Critical, Important, or Minor findings.

Everything checked below was verified directly, not taken from the implementer report:

- **Files touched == brief's file list, exactly.** `git show f8e863e --stat --name-only` lists precisely the 7 files the brief names, nothing more, nothing less. No scope creep (no README.md, no spec.md - both explicitly Task 2's job per the brief's Interfaces note).
- **`validate.rs`** (`crates/muxsmith-core/src/profile/validate.rs:61-73`): the unconditional `NoTrackRules` push is now a `match profile.tracks.unmatched { Drop => ..., Keep => ... }`, `KeepDrop` added to the existing `use super::model::{...}` line. Matches the brief verbatim and mirrors the existing `match profile.tracks.unmatched`/`match profile.attachments.unmatched` shape already used in `planner.rs` (house pattern, not a new idiom).
- **`report/mod.rs`**: `PassthroughProfile => "passthrough-profile"` inserted directly after `NoTrackRules`, `NoTrackRules`'s rustdoc reworded per the brief. `Diagnostic::info(code, config_path)` (used by the new arm) already exists with the matching two-argument signature (`report/mod.rs:254`) and is already used elsewhere for other info-severity codes (e.g. `RawProperty`) - no new builder needed.
- **Catalog strings verified programmatically, character-for-character, against the decisions document's PROPOSAL blocks** (not eyeballed): extracted the `no-track-rules`/`passthrough-profile` PROPOSAL text from `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md` D38 and diffed against the actual `locales/en/diagnostics.ftl` and `locales/de/diagnostics.ftl` lines with a normalizing script. All four (en/de x two keys) matched exactly.
- **De catalog register**: `tracks.unmatched`, `keep` stay literal per the de file's own header convention (config-field names/keywords stay literal); no quotes in either new de string so the straight-ASCII-quotes rule is vacuously satisfied.
- **Typography**: `grep -P` for em/en dash, curly quotes, ellipsis, NBSP, Unicode minus over the full diff - zero hits.
- **Commit**: unsigned (`git log --show-signature` shows no signature block), message matches the brief's exact text, `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer present. Files were staged explicitly (the commit's file list matches the brief's `git add` list exactly - no stray files that an `-A`/`.` add would have picked up).
- **The three self-reported adaptations in Step 8, checked against the actual neighboring idiom, not just the report's argument:**
  1. `config_diagnostics` instead of the brief draft's `diagnostics`: confirmed correct against `dry_run_cli.rs` (`dry_run_surfaces_config_time_invalid_regex`, `dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing`, and others) - `config_diagnostics` is the real top-level field for `validate()`-produced (config-time) diagnostics; `files[i]["diagnostics"]` is the per-file field. `PassthroughProfile`/`NoTrackRules` are both `validate()`-only, so `config_diagnostics` is right.
  2. Direct `output_dir.join("Show.S01E01.mkv")` instead of a directory glob: confirmed correct against `live_run_muxes_two_sources_and_reports_exit_zero` in the same file, which computes `output_dir.join(...)` directly with the identical comment citing spec 4.8's `keep` filename default. Matches the local idiom exactly, including the comment style.
  3. `.env("MUXSMITH_RUNS_ROOT", dir.path().join("runs"))` on the `run` invocation: confirmed present on both pre-existing `run_live.rs` tests, same comment citing Task 6 (D26). Its absence in the new test would have been the one live-run test in the file polluting the real platform data dir; adding it is correct and consistent.
  - The brief's fixed contract (exit codes for both `dry-run` and `run`, `passthrough-profile` present, `no-track-rules` absent, recognized container, track count 2) is intact word-for-word in the diff; none of the three adaptations touch it.
- **e2e test executed by the reviewer, foreground, on this machine** (`mkvmerge v100.0` confirmed on PATH at `/home/linuxbrew/.linuxbrew/bin/mkvmerge`):
  ```
  $ cargo test -p muxsmith-cli --test run_live zero_rule_keep -- --nocapture
  running 1 test
  test zero_rule_keep_profile_is_a_pure_passthrough ... ok
  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.29s
  ```
  No skip marker in stdout/stderr; 0.29s runtime is consistent with two real `mkvmerge` invocations (mux + `-J` identify), not a vacuous return.
- **Full verification suite re-run independently** (not trusted from the report): `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets -- -D warnings` zero warnings; `cargo test --workspace` all green (78 passed in the core/cli test binary that includes the new tests, full workspace otherwise green); `cargo deny check` advisories/bans/licenses/sources all ok; `node scripts/check-i18n.mjs` reports `ok` (181 catalog ids, en/de parity holds, only pre-existing unrelated GUI-catalog warnings).
- **House convention (core-83, `docs/product-boundaries.yaml:388-402`)**: the ruling's substance (`unmatched: keep` + zero rules is legal, `drop` stays an error, MUST be documented and hinted) is satisfied for the "hinted" half (the new info diagnostic); "documented" (README) is explicitly out of this task's scope per the brief.
- **No other exhaustive `match`/table over `DiagCode` was missed**: grepped the whole tree for `NoTrackRules` - the only sites needing an update were `report/mod.rs`'s `key()` (macro-generated, handled automatically) and `catalog_completeness.rs`'s `fixture_args` (updated). No separate severity table exists; severity is set at the `Diagnostic::info`/`::error` call site, already correct.

## Verdict A: spec compliance

**APPROVED.** Every brief requirement (Steps 1-10) is present and correct: failing tests written and shown to fail on the right error, `DiagCode::PassthroughProfile` added with the specified wire key/severity/config_path, `validate.rs` made conditional exactly as specified, both catalogs updated with the owner-approved PROPOSAL wording verbatim, `catalog_completeness.rs` fixture extended, e2e test added and passing for real, commit staged and worded exactly as specified. Nothing built beyond the brief's file list.

## Verdict B: code quality

**APPROVED.** Idiomatic: the `match` shape mirrors existing `unmatched` matches elsewhere in the codebase; the e2e test's three adaptations all resolve to the correct neighboring idiom, verified independently rather than accepted on the report's say-so; no new abstraction, no duplicated helper, no invented pattern. Rustdoc on the new `DiagCode` variant carries the same density and cross-referencing (D38, wire key rationale) as its neighbors.

## House dimension

No deviation found from `docs/conventions.yaml`, `docs/process-conventions.yaml`, or `docs/product-boundaries.yaml` core-83. The pre-commit gate (`process-conventions.yaml`'s fmt/clippy/test/deny quartet) was run and independently reproduced clean.

## Harvest (patterns/repeated behavior worth the convention ledger)

- **"Read the neighboring test file before adapting a brief's literal draft snippet" is working as designed and is now observed twice** (this task's three Step-8 adaptations, all correctly resolved against `dry_run_cli.rs`/`run_live.rs` idiom rather than the brief's literal-but-wrong field names). Worth promoting to Tier 2 in `conventions.yaml` (nature: process or technical-code) if it recurs on a third task: "when a brief's draft test snippet conflicts with the actual JSON/CLI shape the neighboring test file already establishes, the neighboring file wins, and the report calls out the field-name correction explicitly with a cross-reference to the sibling test that pins it."
- **`match profile.tracks.unmatched { Drop => .., Keep => .. }` as the house shape for KeepDrop-gated diagnostics** (this task) mirrors the same shape already at `planner.rs` for `attachments.unmatched`. Not yet a repeated *diagnostic-emission* case (only one instance so far here), but worth a note if D39 or a later task adds a third KeepDrop-gated diagnostic: the match-over-KeepDrop-with-a-comment-per-arm shape is the convention, not an ad hoc `if`.
- **Catalog PROPOSAL strings verified by exact program diff against the decisions doc, not eyeballing** - this review's own method, worth keeping as the reviewer-side default for any future "verbatim owner-approved wording" check: a script comparison catches whitespace/wrapping differences a visual scan would miss.

## APPROVED
