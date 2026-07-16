# Task 6 report: D43 + D49 - apply_suggestion in core, and the typed StructuredEdit seam

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan6-b`, branch `plan6-b`.
Commit: `bf4515c` "core: StructuredEdit carries the typed Scalar; apply_suggestion splices through the engine's own helper (D43, D49)"

## What was implemented

In `crates/muxsmith-core/src/planner.rs`, exactly per D49:

- `:10` import changed to `use serde::{Deserialize, Serialize};`.
- `StructuredEdit` reshaped to D49's wire shape: `AddExact`/`AddNotExact` now carry `value: Scalar` (was `String`); `AddSubstring`/`AddNotSubstring` keep `value: String` (the asymmetry D49 mandates, since `MatchExpr.exact` holds `Scalar` and `MatchExpr.substring` holds `String`). Derives `Debug, Clone, PartialEq, Serialize, Deserialize`, still `#[serde(tag = "kind", rename_all = "snake_case")]`. **No `ts` derive and no `ts_rs` import** - Task 5's, not this task's, per the brief's binding constraint.
- `delta_for` lost its `scalar: &Scalar` parameter and reads the edit's own typed `value` field on the two exact arms; stays private, unchanged visibility.
- The four engine call sites: `:1746`/`:1753` (line numbers as in the pre-edit tree) now build `value: scalar.clone()` instead of `value: display.clone()`; the two `delta_for` call sites (`:1762`, `:1791`) drop their second argument, and the synthetic `Scalar::Str(tok.to_string())` at `:1791` is gone. `prop_value_as`'s `(String, Scalar)` return is unchanged; `display` still keys `seen` and `rank`.
- New `pub fn apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>`, inserted directly after `with_rule_match` (which it calls, alongside `delta_for` and the new private `edit_key`). Does not validate, does not re-plan; detects the `core-44` no-clobber no-op via one `PartialEq` comparison of the spliced model against the input.
- New private `fn edit_key(edit: &StructuredEdit) -> &str`, total over the four variants.
- New `pub enum ApplyError { UnparsableConfigPath(String), RuleIndexOutOfRange { index, rules }, EditChangedNothing { index, property } }`, deriving `Debug, Clone, PartialEq` only (no `Deserialize`).

In `crates/muxsmith-core/tests/suggestions.rs`:

- Harness split: `plan_model(profile: &Profile, files)` is the old `plan_multi` body with `from_str` lifted to the caller; `plan_multi` and `plan` are re-expressed on top of it (removes the pre-existing hand-rolled `plan`/`plan_multi` duplication, per the brief). Added imports `ApplyError`, `apply_suggestion` (merged into the existing `muxsmith_core::planner::{...}` import rather than a second `use` line for the same module - idiomatic consolidation, not a content change) and `use muxsmith_core::profile::model::Profile;`.
- Added `yaml_scalar` (mirrors `scalar_display`, keeps the four template-site fixtures byte-identical) and `spliced_scalar` (reads back the spliced value from whichever arm the edit's variant targets), both verbatim from D49.
- Added fixture `P_ALREADY_CONSTRAINED` verbatim.
- Added the seven guard tests verbatim: `apply_splices_the_simulated_scalar_for_a_bool_property` (G1), `apply_splices_the_simulated_scalar_for_an_int_property` (G2), `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` (G3), `apply_rejects_an_unparsable_config_path`, `apply_rejects_a_rule_index_past_the_end`, `apply_rejects_an_edit_the_no_clobber_merge_drops` (G4), `apply_returns_ok_when_the_edit_reaches_the_model` (control).
- Updated the seven pre-existing `value`-binding sites: four template sites (in `apply_edit_to_first_rule`'s `AddExact`/`AddNotExact` arms and `apply_edit_to_no_clobber_rule`'s `AddExact`/`AddNotExact` arms) now interpolate `yaml_scalar(value)`; three comparison sites now compare against a typed `Scalar` literal - `track_name` -> `&Scalar::Str("Chapter 1: Intro".to_string())`, `forced_track` -> `&Scalar::Bool(true)` (the Boolean-typed one, does not follow the `track_name` shape), `language` -> `&Scalar::Str("eng".to_string())` (the negative assertion, where a wrong literal would have passed vacuously).

## TDD evidence

**RED** - test changes applied, implementation changes stashed (`git stash push -- crates/muxsmith-core/src/planner.rs`), then `cargo test -p muxsmith-core --test suggestions apply`:

```
error[E0432]: unresolved imports `muxsmith_core::planner::ApplyError`, `muxsmith_core::planner::apply_suggestion`
 --> crates/muxsmith-core/tests/suggestions.rs:6:30
  |
6 | use muxsmith_core::planner::{ApplyError, Batch, RunInputs, StructuredEdit, apply_suggestion, plan_batch};
  |                              ^^^^^^^^^^ no `ApplyError` in `planner`       ^^^^^^^^^^^^^^^^ no `apply_suggestion` in `planner`

error[E0277]: can't compare `std::string::String` with `Scalar`
   --> crates/muxsmith-core/tests/suggestions.rs:885:52
    |
885 |                 if property == "language" && value == &Scalar::Str("eng".to_string())
    |                                                    ^^ no implementation for `std::string::String == Scalar`

error[E0308]: mismatched types
    --> crates/muxsmith-core/tests/suggestions.rs:1054:29
     |
1054 |             matches!(value, Scalar::Bool(_)),
     |                      -----  ^^^^^^^^^^^^^^^ expected `String`, found `Scalar`

[... 13 more E0308/E0277 errors of the same two shapes, one per new-test Scalar construction/comparison site ...]

error: could not compile `muxsmith-core` (test "suggestions") due to 16 previous errors
```
16 errors total: the `E0432` unresolved-import pair (`ApplyError`, `apply_suggestion` undefined) plus 14 `E0308`/`E0277` type errors from the Scalar-typed constructions and comparisons the still-`String` field rejects. Matches the brief's expected RED exactly. Implementation changes restored via `git stash pop` immediately after capturing this.

**GREEN** - `cargo test -p muxsmith-core --test suggestions` (full file, not just the `apply` filter):
```
running 19 tests
test apply_rejects_an_edit_the_no_clobber_merge_drops ... ok
test apply_returns_ok_when_the_edit_reaches_the_model ... ok
test apply_rejects_a_rule_index_past_the_end ... ok
test apply_rejects_an_unparsable_config_path ... ok
test yaml_fragment_round_trips_a_value_containing_a_colon ... ok
test ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions ... ok
test apply_splices_the_simulated_scalar_for_an_int_property ... ok
test suggestion_cap_truncation_is_logged_not_silent ... ok
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok
test tc_b_two_required_overlap_yields_no_suggestion_and_names_all_claimants ... ok
test ambiguous_rule_gets_a_validated_suggestion ... ok
test no_single_fix_produces_a_two_group_partition ... ok
test tc_a_overlap_optional_rule_yields_not_narrowings_on_that_rule ... ok
test tc_d_three_claimant_overlap_yields_no_suggestion_and_names_all_three ... ok
test tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard ... ok
test with_rule_match_never_widens_an_existing_substring_constraint ... ok
test every_suggestion_survives_the_next_dry_run ... ok
test ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule ... ok
test every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
12 pre-existing tests + 7 new guard tests, all passing. Re-ran after `cargo fmt --all` reformatted the file (import wrapping, one collapsed match arm): still 19/19.

## Step 6: structural proof (verbatim output)

```
$ awk '/pub fn apply_suggestion/,/^}/' crates/muxsmith-core/src/planner.rs \
  | grep -nE "with_rule_match|delta_for|extend\("
12:    let applied = with_rule_match(profile, index, &delta_for(edit));
```
One line, containing both `with_rule_match` and `delta_for`, and no `extend(` line - matches the expected shape exactly (splices through the engine's own helpers, no `BTreeMap::extend` inside the applier). Re-ran post-`cargo fmt` to confirm the reformatting didn't move anything: identical result.

## Gate results (all nine parts, foreground, no subsets)

1. `cargo fmt --all --check` - initially failed (import-list wrapping in the test file); ran `cargo fmt --all`, re-ran tests (still 19/19 green), re-ran `--check`: clean, exit 0.
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean, exit 0, whole workspace including `muxsmith-gui`/tauri.
3. `cargo test --workspace` - 37 `test result: ok` blocks, zero `FAILED`/`error` lines (grepped the full log to confirm), covering `muxsmith-core` (including `suggestions.rs`'s 19), `muxsmith-cli`, `muxsmith-gui`/tauri (78 unit tests), `xtask`, `codegen`, and all doc-tests.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean, exit 0, no `missing_docs` violations, no broken intra-doc links.
5. `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`, exit 0.
6. `pnpm install` (node_modules was missing in the worktree; ran `mise install` first per the constraint) then `pnpm lint` (`eslint .`) - clean, exit 0.
7. `pnpm build` (`vue-tsc --noEmit && vite build`) - clean, exit 0, `built in 143ms`.
8. `pnpm check:i18n` - `check-i18n: ok (17 source files scanned, 181 catalog ids, 12 unused warning(s), 1 other locale(s) checked for parity against 6 en/ catalog(s))`. The 12 unused-key warnings are pre-existing shell-error catalog entries (`identify-failed`, `mkvmerge-spawn-failed`, etc.) unrelated to this task, which adds no Fluent keys (the D49 shell mapping and catalog entries are explicitly out of this task's scope - core-crate only). Exit 0.
9. `pnpm test:e2e` (`tsc --noEmit -p e2e/tsconfig.json && vite build ... && playwright test`) - Playwright chromium browser binaries were already cached (`~/.cache/ms-playwright/chromium-1228`), no install needed. `7 passed (1.0s)`, exit 0.

## Files changed

- `crates/muxsmith-core/src/planner.rs` (116 insertions, 18 deletions net of context)
- `crates/muxsmith-core/tests/suggestions.rs` (301 insertions, 49 deletions net of context)

No other files touched (`git status --short` before commit showed exactly these two).

## Self-review findings

- Diffed both files against D49's quoted code blocks line-by-line: `StructuredEdit` (D49 `:345-383` minus the `ts` line), `delta_for` (`:415-448`), the four engine call sites (`:454-459`), `apply_suggestion` (`:465-499`), `edit_key` (`:505-519`, including its stale internal line-reference comment `planner.rs:1824, :1829` - kept verbatim per the "copy, don't paraphrase" instruction even though those exact line numbers now point elsewhere post-edit), `ApplyError` (`:530-560`), `plan_model`/`plan_multi`/`plan` (`:819-846`), `spliced_scalar` (`:869-887`), `yaml_scalar` (`:1268-1276`), `P_ALREADY_CONSTRAINED` (`:1073-1079`), and all seven tests - all transcribed exactly, differing only in rustfmt's own whitespace/wrapping choices (which the fmt gate requires).
- The seven pre-existing sites match the brief's table exactly, including the two non-obvious ones: `:722`'s `forced_track` uses `&Scalar::Bool(true)`, not the `Str`-shaped pattern the `track_name` site uses; `:890`'s `language` negative assertion uses `&Scalar::Str("eng".to_string())`, not a value that would make the `any(...)` match nothing and pass vacuously.
- `grep -n "ts_rs\|feature = \"ts\"\|derive(TS)"` over both changed files: zero matches - no `ts` residue from Task 5's scope.
- `Suggestion` and `DiagCode` still derive only `Serialize` (confirmed via grep) - `core-37-prose-free-core` untouched. `ApplyError` derives `Debug, Clone, PartialEq` only, no `Deserialize`.
- `apply_suggestion`, `ApplyError` (and its three variants), `edit_key`'s absence of `pub`, and `delta_for`'s continued privacy all verified directly in the diff and by the passing `missing_docs`-denying doc build.
- Scope check: `git status --short` before staging showed only the two files the brief names; nothing in `src-tauri` (the D49 shell mapping / Fluent catalog entries) was touched, correctly out of this task's scope per the brief's file list.

## Concerns

One deliberate, disclosed judgment call, not a deviation from D49's content: the brief's prose says "Add `use muxsmith_core::profile::model::Profile;` and `use muxsmith_core::planner::{ApplyError, apply_suggestion};` to the file's imports" as two statements, but the test file already had a `use muxsmith_core::planner::{Batch, RunInputs, StructuredEdit, plan_batch};` import. I merged `ApplyError`/`apply_suggestion` into that existing import rather than adding a second, separate `use muxsmith_core::planner::{...}` line for the same module - ordinary import consolidation, not a content or behavioral difference, and `rustfmt`'s default `imports_granularity = Preserve` would not have merged two separate statements for me. Flagging it since the brief's wording could be read as literally two `use` statements.

No other concerns. No design latitude was exercised: every wire-shape, error-variant, and test-body decision was already settled in D49; the only choices left to me were rustfmt-governed formatting and the placement of the new `apply_suggestion`/`edit_key`/`ApplyError` block within the file (placed immediately after `with_rule_match`, which it calls, ahead of `resolves_without_regression` - D49 specifies content, not file position).
