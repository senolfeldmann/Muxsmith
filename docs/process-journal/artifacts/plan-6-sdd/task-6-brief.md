### Task 6: D43 + D49 - apply_suggestion in core, and the typed `StructuredEdit` seam

**Stream B** (`.worktrees/plan6-b`), parallel with streams A and C - touches `planner.rs` and its test file only.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (`StructuredEdit` reshape + `Deserialize`; `delta_for` loses its scalar param; four engine call sites; new `apply_suggestion`, `ApplyError`, `edit_key`; the `:10` import)
- Test: `crates/muxsmith-core/tests/suggestions.rs`

**Interfaces:**
- Produces, for Task 5: `StructuredEdit` in its **final D49 shape** - `value: Scalar` on the two `Exact` variants, plus `Deserialize` - but **without** the `ts` derive, which Task 5 adds.
- Produces, for Task 8: `muxsmith_core::planner::apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>` and `muxsmith_core::planner::ApplyError` (three variants).
- Consumes: nothing from other tasks.

**Read first:** design D43 (`:410-495`) and **D49 in full** (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`). D49 supersedes D43 on the seam; where the two differ, D49 wins.

Binding points (all from D49; each is a place the design review found the original plan handed the implementer an uncloseable fork):
- **The seam D43 called "reuse" is resolved by D49, not by reconstruction.** `StructuredEdit`'s two `Exact` variants carry `value: Scalar` (not `String`), so the engine's own typed value rides on the edit and apply reconstructs nothing. The two substring variants keep `value: String` (their target map `MatchExpr.substring` holds `String`). This asymmetry is settled, not the implementer's to reconsider (D49 §"The wire shape").
- **`delta_for` loses its `scalar` parameter and stays private** (D49 §"delta_for"): it now reads the edit's own typed `value`. `apply_suggestion` reaches it, `with_rule_match` and `rule_index_of` by ordinary in-module visibility - no visibility change is needed anywhere (`with_rule_match`/`rule_index_of` are already `#[doc(hidden)] pub`; `delta_for` stays private).
- **`apply_suggestion` has exactly three exits, so `ApplyError` has exactly three variants** (D49 §"ApplyError"): `rule_index_of` returns `None` -> `UnparsableConfigPath`; the parsed index is past `tracks.rules.len()` -> `RuleIndexOutOfRange { index, rules }`; the spliced model equals the input (the `core-44` `or_insert` merge dropped the delta) -> `EditChangedNothing { index, property }`. The last is detected by one whole-model comparison (`applied == *profile`), which is a single `PartialEq`, not a re-plan and not a validation. `ApplyError` is `pub`, derives `Debug, Clone, PartialEq`, and does **not** derive `Deserialize`.
- **`StructuredEdit` gains `Deserialize`; `Suggestion` and `DiagCode` do not** (`core-37-prose-free-core`, the most-reinforced house entry: the shell must not be able to synthesize diagnostics).
- **`apply_suggestion` does not validate and does not re-plan.** It returns the mutated model; the caller validates through the normal `validate_profile_model` path (D43). `core-33` (narrow-only), `core-44` (no-clobber via `with_rule_match`) and `core-72` (typed equality) all hold unchanged (D49 §"Not changed by this ADR").
- **The `ts` derive on `StructuredEdit` is Task 5's, not this task's** (closed decision 7 / D49 "D49 cannot land before D44"). Task 6 lands `StructuredEdit` with `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]` and `#[serde(tag = "kind", rename_all = "snake_case")]` **only**; it does **not** add the `#[cfg_attr(feature = "ts", derive(TS), ...)]` line and does **not** add the `#[cfg(feature = "ts")] use ts_rs::TS;` import. Those need the `ts` feature and `Scalar: TS`, which Task 5 provides. Task 6 does change `planner.rs:10` to `use serde::{Deserialize, Serialize};` (the `Deserialize` derive needs it).

- [ ] **Step 1: Write the failing tests, fixture and helpers (D49 gives all bodies)**

Add to `crates/muxsmith-core/tests/suggestions.rs`, verbatim from D49's guard section (a second copy of settled test code would be a drift surface - copy from D49, do not paraphrase):

- The **harness split** (D49 §"Harness change the guard needs", `:815-857`): introduce `plan_model(profile: &Profile, files) -> (Batch, TempDir)` as today's `plan_multi` body with the `from_str` lifted to the caller; re-express `plan_multi` and `plan` on top of it (this removes an existing hand-rolled duplicate, it does not add one). Add the two imports D49 names: `use muxsmith_core::profile::model::Profile;` and `use muxsmith_core::planner::{ApplyError, apply_suggestion};`.
- The **`spliced_scalar` accessor** (D49 `:864-891`) and the **`yaml_scalar` helper** (D49 `:1264-1276`).
- The fixture **`P_ALREADY_CONSTRAINED`** (D49 `:1073-1079`).
- The **seven tests**, each copied verbatim from its D49 section:
  1. `apply_splices_the_simulated_scalar_for_a_bool_property` - G1, the Bool case (D49 `:895-933`), with its `checked > 0` anti-vacuity assertion.
  2. `apply_splices_the_simulated_scalar_for_an_int_property` - G2, the Int case (D49 `:943-980`), with its `checked > 0` assertion.
  3. `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` - G3, the `core-03` re-plan at the model level with no YAML laundering (D49 `:989-1019`).
  4. `apply_rejects_an_unparsable_config_path` (D49 `:1034-1045`).
  5. `apply_rejects_a_rule_index_past_the_end` (D49 `:1047-1058`).
  6. `apply_rejects_an_edit_the_no_clobber_merge_drops` - G4, the `EditChangedNothing` case (D49 `:1081-1095`).
  7. `apply_returns_ok_when_the_edit_reaches_the_model` - the control proving the no-op detection does not fire when the edit lands (D49 `:1100-1112`).

- [ ] **Step 2: Run to confirm it fails**

```bash
cargo test -p muxsmith-core --test suggestions apply
```
Expected: FAIL to compile - `apply_suggestion` and `ApplyError` are undefined, and the tests construct `StructuredEdit` with `Scalar`-typed values the current `String` field rejects. Both are the red. (The existing suggestion tests still bind `value` as `&str`; they stop compiling too, and step 4 updates them.)

- [ ] **Step 3: Implement the reshape and the applier (D49 gives the code)**

In `planner.rs`, exactly per D49:
- Change `:10` to `use serde::{Deserialize, Serialize};`.
- Reshape `StructuredEdit` to D49 §"The wire shape" (`:345-383`) **minus** the `#[cfg_attr(feature = "ts", ...)]` line (Task 5 adds it): `value: Scalar` on `AddExact`/`AddNotExact`, `value: String` on the two substring variants, deriving `Debug, Clone, PartialEq, Serialize, Deserialize`, tagged `kind`/`snake_case`.
- Rewrite `delta_for` to the single-argument, private form (D49 §"delta_for", `:415-448`).
- Apply the four engine call-site edits (D49 §"The engine call sites", `:454-459`): `:1746` and `:1753` become `value: scalar.clone()`; `:1762` and `:1791` become `delta_for(&edit)`, dropping the synthetic `Scalar::Str` at `:1791`. `prop_value_as` keeps its `(String, Scalar)` return - `scalar` now goes into the edit, `display` still keys the `seen` dedup and the `rank` tuple.
- Add `apply_suggestion` (`pub`), `ApplyError` (`pub`, three variants), and the private `edit_key` helper, all verbatim from D49 §"apply_suggestion" (`:465-519`) and §"ApplyError" (`:522-560`).

- [ ] **Step 4: Update the seven existing `value`-binding sites (D49 gives each)**

D49 §"Interface changes" > "Tests" (`:1252-1311`) enumerates all seven and gives the fix for each; apply them exactly:
- **Four template sites** (`:97`, `:100`, `:203`, `:206`) interpolate `yaml_scalar(value)` where they interpolated `value`. `yaml_scalar` (added in step 1) renders byte-identically to the old `display` string.
- **Three comparison sites**, each getting the typed literal from D49's table (`:1291-1295`), never derived from the old string: `:325` `track_name` -> `&Scalar::Str("Chapter 1: Intro".to_string())`; `:722` `forced_track` -> `&Scalar::Bool(true)` (this one does **not** follow `:325` - `forced_track` is Boolean); `:890` `language` -> `&Scalar::Str("eng".to_string())` (this is a **negative** assertion, so a wrong literal passes vacuously - it must be `Str` because `language` is String-typed).

- [ ] **Step 5: Run the tests**

```bash
cargo test -p muxsmith-core --test suggestions
```
Expected: PASS - the seven new tests plus the updated existing ones.

- [ ] **Step 6: Prove the no-clobber reuse is structural, not asserted**

`core-44` is the one thing a plausible reimplementation gets wrong silently, and G4 proves the behaviour. Confirm structurally that `apply_suggestion` splices through the engine's own helpers and merges nothing itself, scoping the grep to the function body so it actually fires (the earlier plan's grep opened its `sed` range on a line that never contains both patterns and so could never match):

```bash
awk '/pub fn apply_suggestion/,/^}/' crates/muxsmith-core/src/planner.rs \
  | grep -nE "with_rule_match|delta_for|extend\("
# Expected, read from the OUTPUT (not from its absence): at least one
# `with_rule_match` line and one `delta_for` line, and NO `extend(` line.
# A BTreeMap::extend inside the applier is the Bug C shape and fails review;
# the only legitimate `extend` is inside with_rule_match's `not` merge, which
# is a different function and outside this range.
```

- [ ] **Step 7: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/suggestions.rs
git -c commit.gpgsign=false commit -m "core: StructuredEdit carries the typed Scalar; apply_suggestion splices through the engine's own helper (D43, D49)"
```

---

