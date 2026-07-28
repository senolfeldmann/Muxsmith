# Amendment 4, plan side - author report

Author: the plan's amendment author (Fable 5, same agent as amendments 3 and
its fix round). One file edited and committed:
`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`,
commit `ba69c36`, pathspec-scoped, not pushed. **Report location, decided:**
this NEW file (`amendment-4-plan-report.md`) rather than an append - the
amendment-3 report is a closed, reviewed artifact whose name binds to its
amendment; per-amendment files match the sdd directory's naming.

## 1. Status: DONE

All four dispatch items carried; the design untouched (and explicitly
recorded as needing no amendment); D64 preserved with its mechanism named;
every consumer surface swept with a per-surface verdict. Typography over the
whole plan: zero banned glyphs (exit 1; fire control on an em-dash sample: 1).

## 2. The exact diff (git show -U1 ba69c36)

```diff
diff --git a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
index 036cccd..0948308 100644
--- a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
+++ b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
@@ -280,2 +280,3 @@ Read first: design D101 in full (both forks, the boundary paragraph, the accepte
 - Modify: `crates/muxsmith-cli/tests/cli_validate.rs` (the two pinned subprocess tests)
+- Modify: `crates/muxsmith-cli/tests/support/mod.rs` (amendment 4: the new locale-parameterized pinned helper `muxsmith_localized`, the `muxsmith` funnel becoming a delegation to it with `"en"`, and the funnel's rustdoc updated to match that delegation - nothing else in this file; `muxsmith_bare` and its closed two-caller exception doc stay byte-identical)
 - Create: the two insta snapshot files those tests accept, named by the directory's existing `<test_file>__<test_fn>.snap` convention: `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap` and `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`
@@ -296,6 +297,7 @@ Read first: design D101 in full (both forks, the boundary paragraph, the accepte
   - `bare_raw_property_exits_two_and_renders_the_message`: the exact profile of the authoring probe (`profile_version: 1`, `input: { pattern: 'E(\d+)', extensions: [mkv] }`, one rule `- match: { exact: { 'raw:': eng } }`); assert `.code(2)`; snapshot the stdout (the snapshot must contain the en text of D101's fence).
-  - `bare_raw_property_renders_german_with_locale_flag`: same profile, args plus `--locale de`; `.code(2)`; snapshot (must contain the de text of D101's fence).
+  - **The invocation vehicle (amendment 4).** The originally pinned form, "args plus `--locale de`" through `support::muxsmith`, is impossible: the funnel appends `--locale en` AFTER the caller's args and clap rejects a repeated `--locale` (measured at amendment time: exit 2, `error: the argument '--locale <LOCALE>' cannot be used multiple times` on stderr - which also means the mis-invoked test PASSES `.code(2)` on clap's usage error while snapshotting empty stdout; the de-fence content assertion is what unmasks it). Instead, `crates/muxsmith-cli/tests/support/mod.rs` gains `pub fn muxsmith_localized(args: &[&str], locale: &str) -> Command` - today's funnel body with the locale parameterized: `cargo_bin("muxsmith")`, then the caller's args, then `--locale <locale>` appended LAST (preserving the after-the-subcommand guarantee the funnel's doc records) - and `muxsmith(args)`'s body becomes exactly `muxsmith_localized(args, "en")`. The funnel's rustdoc is updated in the same edit (the delegation falsifies its "builds its `Command` here" and appending description): it keeps the D64 contract statements (explicit locale pinning on the CLI's own surface, never environment variables) and states the delegation plus the file-level invariant, `cargo_bin("muxsmith")` confined to this file. `muxsmith_bare` and its closed two-caller exception doc are untouched.
+  - `bare_raw_property_renders_german_with_locale_flag`: same profile, invoked as `support::muxsmith_localized(args, "de")` (amendment 4); `.code(2)`; snapshot (must contain the de text of D101's fence - the content assertion discriminates a real German render from clap's stderr-only usage error, which also exits 2).
 - [ ] **Step 5: the Run-gate e2e scenario** (amendment 1, ruling A - the feature's GUI consequence ships with its test). Implement D101's amendment-1 producer paragraph exactly as enumerated there: a new scenario in `e2e/smoke.spec.ts`'s `batch view: dry run` describe (`:140`), mocking `detect_mkvmerge`, the dialog open, and `validate_profile` resolving the design-fixed document (one `empty-raw-property` error-severity diagnostic, `mkvmerge_found: true`); assert `data-testid="batch-run"` disabled AND its `title` equals the localized `batch-run.tooltip-errors` text (`gui-batch.ftl:69`), which discriminates the errors reason from the missing-profile and missing-mkvmerge gates by construction. It is the paired negative of the enabled assertion at `smoke.spec.ts:511` (locator `:510`) - paired by assertion, not by location: that assertion sits in the `jobs view: live run` flow (`:477` describe, test `:491`). **What this test establishes (`a-new-test-says-whether-the-behavior-or-the-assertion-is-new`): the ASSERTION is new, not the behavior.** `hasErrors` gating exists today, and an error-severity document already reaches BatchView in the plural-counts test (`pluralReport`, `severity: "error"` at `:272-277`, fed via `dry_run` at `:315`) - but nothing anywhere asserts `batch-run` disabled (in `smoke.spec.ts` the only `toBeDisabled` is the editor Save, `:1241`; suite-wide the count is three, adding `editor-rule-add-remove.spec.ts:151`/`:317`, none of them on `batch-run`). The scenario therefore PASSES on today's tree; no red-today claim attaches to it. The ruled consequence is covered end to end by a two-link chain whose links are separate tests by construction, because the e2e mock supplies the severity by hand: this task's core and CLI tests prove a bare `raw:` now yields error severity (new behavior, red today), and this scenario proves an error-severity document disables the Run button (existing behavior, newly asserted). Flow, mocked commands, document contents and assertion targets are design-fixed - no implementer choices (design section 5, the two-scenarios bullet).
 - [ ] **Step 6: spec amendments** S-1 (the new `EmptyRawProperty` row only), S-3, S-5, S-6, exactly as the design's section 3 fences write them.
-- [ ] **Step 7: verification.** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (new tests green; B-2/B-3 controls green; the catalog macro equality test and the placeholder-leak guard cover the new key/row); `pnpm check:i18n` (both locales stay in lockstep); `pnpm lint`; `pnpm test:e2e` (the new Run-gate scenario green, every pre-existing e2e suite unchanged). No absence-shaped check exists in this task; the exit-code flip that D101 accepts is asserted positively by the two `.code(2)` tests, whose red state is today's tree (the authoring probe measured exit 0 with an info diagnostic on the identical profile). The Run-gate scenario carries NO red-today claim: it passes on today's tree (Step 5 records why), and its value is the previously missing assertion plus regression protection on the gate.
+- [ ] **Step 7: verification.** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace` (new tests green; B-2/B-3 controls green; the catalog macro equality test and the placeholder-leak guard cover the new key/row); `pnpm check:i18n` (both locales stay in lockstep); `pnpm lint`; `pnpm test:e2e` (the new Run-gate scenario green, every pre-existing e2e suite unchanged). One invariant check exists since amendment 4: `grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates` -> exactly `crates/muxsmith-cli/tests/support/mod.rs` (D64's file-level invariant, held through the helper edit). Fire: the same grep at amendment time already returns exactly that one file, so pattern and pathspec demonstrably hit; the red state is reachable by construction - a `cargo_bin` call written into `cli_validate.rs` instead of riding the pinned helper adds a second file to the output. Otherwise no absence-shaped check exists in this task; the exit-code flip that D101 accepts is asserted positively by the two `.code(2)` tests, whose red state is today's tree (the authoring probe measured exit 0 with an info diagnostic on the identical profile). The Run-gate scenario carries NO red-today claim: it passes on today's tree (Step 5 records why), and its value is the previously missing assertion plus regression protection on the gate.
 - [ ] **Step 8: commit.**
@@ -303,3 +305,3 @@ Read first: design D101 in full (both forks, the boundary paragraph, the accepte
 ```bash
-git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/profile/validate.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-cli/tests/catalog_completeness.rs crates/muxsmith-core/tests/validate_semantics.rs crates/muxsmith-cli/tests/cli_validate.rs crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap e2e/smoke.spec.ts docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
+git add crates/muxsmith-core/src/report/mod.rs crates/muxsmith-core/src/profile/validate.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-cli/tests/catalog_completeness.rs crates/muxsmith-core/tests/validate_semantics.rs crates/muxsmith-cli/tests/cli_validate.rs crates/muxsmith-cli/tests/support/mod.rs crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap e2e/smoke.spec.ts docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
 git -c commit.gpgsign=false commit -m "validate: bare raw: with an empty property name is an error, own DiagCode + Run-gate e2e (D101, S-1/S-3/S-5/S-6)"
@@ -309,3 +311,3 @@ git -c commit.gpgsign=false commit -m "validate: bare raw: with an empty propert
 
-**Must not decide** (design section 5): the three-branch funnel form; no per-call-site checks; no matcher/planner change; the two locale texts character for character; the Run-gate scenario exactly as enumerated in D101 - no scenario beyond it and no new test infrastructure (the amendment-1 boundary in Global Constraints).
+**Must not decide** (design section 5): the three-branch funnel form; no per-call-site checks; no matcher/planner change; the two locale texts character for character; the Run-gate scenario exactly as enumerated in D101 - no scenario beyond it and no new test infrastructure (the amendment-1 boundary in Global Constraints); the German test's invocation vehicle exactly as amendment 4 fixes it - `muxsmith_localized` in the support module, the funnel delegating with `"en"`, `muxsmith_bare`'s closed two-caller set untouched, no environment-variable pinning in any form (D64 preserved, not reopened).
 
@@ -465 +467,9 @@ Routing: `.superpowers/sdd/plan-9/amendment-3-brief.md` (design half) and `.supe
 - **Task 2 Step 1's "rustdoc moved with it" gained a historical qualifier** - decided, not left open: the clause stays verbatim as the record of Task 2's order, with an amendment-3 parenthetical pointing at Task 3 Step 2, because the design side closed the identical misreading by qualifying its own executed "moves as-is" sentence in place, and a silent leave-as-is would re-open for every plan reader exactly the reading amendment 3 exists to close.
+
+## Amendment 4 (2026-07-28, owner-ruled fix of an impossible pinned invocation, mid-execution during Task 4)
+
+Routing: the decision memo in `.superpowers/sdd/plan-9/task-4-report.md` section 4 (Task 4 returned NEEDS_CONTEXT); the owner ruled its option B with the recommended sharpening. **This is a plan-only amendment: the design is NOT amended, and D101 needs no amendment** - its pinned German test is unchanged in substance (name, profile, `.code(2)`, the de-fence content in the snapshot); only this plan's invocation form was impossible, so the design log gains no round (the logs' deliberate numbering offset stays as amendment 3 recorded it). The blocker, re-measured at amendment time: every CLI subprocess test rides `crates/muxsmith-cli/tests/support/mod.rs`'s `muxsmith` funnel, which appends `--locale en` AFTER the caller's args (D64, plan 7), and clap rejects a repeated `--locale` (exit 2, usage error on stderr) - so "args plus `--locale de`" cannot reach the renderer, and the mis-invoked test would PASS its `.code(2)` assertion on clap's own exit 2 while snapshotting empty stdout: a green test proving nothing, unmasked only by the de-fence content assertion. What moved in this plan:
+
+- **Task 4's Files list gained `crates/muxsmith-cli/tests/support/mod.rs`** (recounted: ten entries now) with a within-file-qualified work description: the new locale-parameterized pinned helper `muxsmith_localized(args, locale)`, the `muxsmith(args)` funnel becoming exactly `muxsmith_localized(args, "en")`, and the funnel's rustdoc updated - the delegation falsifies its "builds its `Command` here" and appending description - with `muxsmith_bare` and its closed two-caller exception doc byte-identical. Step 4 gained the invocation-vehicle instruction and its second bullet now invokes `support::muxsmith_localized(args, "de")`; the Step-8 `git add` line stages the file.
+- **D64 is preserved, not reopened - the mechanism by name:** every invocation stays explicitly locale-pinned on the CLI's own `--locale` surface, never environment variables (D64's rejected alternative stays rejected); `cargo_bin("muxsmith")` stays confined to `support/mod.rs` (the file-level greppable invariant, `grep -rln` measured at amendment time: exactly that one file - now a Step-7 check); the pinned path keeps a single construction site (the helper, with the funnel delegating), and the bare unpinned helper's closed two-caller exception set is untouched, which is why option B won over a third bare caller. The helper is also outside the amendment-1 infrastructure boundary, whose OUT set is enumerated (Vitest, `tauri::test`/`mock_builder`, `src-tauri/tests/`, IpcError funnel) and does not contain a support-module function.
+- **Consumer sweep, each surface named with its verdict:** the `git add` block and the must-not-decide list gained the amendment (swept); Step 7's "no absence-shaped check exists in this task" sentence was falsified by the new invariant check and is now scoped with it (swept); the coverage map's D101 row stays "Task 4" unchanged (the design is unamended, so the map has no new obligation to place); the sequencing 4-5 edge stays unchanged (its "share ... the CLI test crate with their neighbours" already covers `support/mod.rs`, and the funnel's behavior for every existing caller is byte-identical under the delegation, so no new edge arises). **No step renumbering, deliberately and unlike amendment 3:** the vehicle instruction answers the same design entry as Step 4's own pinned test (D101) rather than a different one, and Task 4's executed report and review cite the current step numbers, which an insertion would dangle mid-fix-round.
```

## 3. Premise checks (all reproduced, one wording refined)

- **The funnel and its append order:** `crates/muxsmith-cli/tests/support/mod.rs`
  `pub fn muxsmith` at `:89`; body is `cargo_bin("muxsmith")` -> `cmd.args(args)`
  -> `cmd.args(["--locale", "en"])` (`:90-92`) - the en pin comes AFTER the
  caller's args, exactly as the Task-4 memo states. Its rustdoc (`:80-88`)
  carries the D64 contract ("every integration test that runs the muxsmith
  binary builds its `Command` here", the after-the-subcommand rationale, the
  no-env-var rationale, "Post-sweep invariant: `cargo_bin(\"muxsmith\")`
  appears nowhere outside this function") - the delegation edit falsifies the
  builds-here and appending sentences, so the rustdoc update the dispatch
  names is real.
- **The clap rejection, re-measured myself** (it carries the whole ruling):
  `./target/debug/muxsmith validate /tmp/nonexistent.yaml --locale de --locale en`
  -> `error: the argument '--locale <LOCALE>' cannot be used multiple times`
  (stderr), `EXIT=2`. Both halves of the danger confirmed: exit 2 satisfies
  `.code(2)`, and stdout is empty.
- **`cargo_bin("muxsmith")` in exactly one file today:**
  `grep -rln 'cargo_bin("muxsmith")' --include='*.rs' crates` ->
  `crates/muxsmith-cli/tests/support/mod.rs` only. Reproduced.
- **One precision refinement to the dispatch's sharpening, carried into the
  plan as the measured truth:** "exactly one `cargo_bin` call site remains,
  in one function" is not literally true of the file - `muxsmith_bare`
  (`:109-111`) has its own `cargo_bin` call inside its closed two-caller
  exception, untouched by this ruling (`grep -c` in the file: 3 = funnel
  `:90` + bare `:110` + the doc mention `:88`). D64's actual greppable
  invariant is FILE-level (`grep -rln` -> one file), and that is what the
  plan now states and checks; the "one call site" truth is scoped to the
  PINNED path (the helper constructs, the funnel delegates). Written that
  way in Step 4, Step 7 and the log entry so the Task-4 review's grep
  reproduces rather than refutes.
- **Plan surfaces before editing:** Task 4's Files list had nine entries,
  none `support/mod.rs` (`grep -c "support/mod.rs"` over the plan -> 0,
  exit 1; fire control: `cli_validate.rs` -> 4); the old invocation wording
  existed exactly once (`:297`); the `git add` line lacked the file; the
  must-not list carried no vehicle constraint.

## 4. Decisions recorded (dispatch item 4 and the two structural calls)

- **Consumer sweep, per surface:** Files list EXTENDED (tenth entry,
  within-file-qualified so the file-vs-within-file ruling's constraint form
  is deliberate: `muxsmith_bare` byte-identical); `git add` block EXTENDED;
  must-not list EXTENDED (the vehicle exactly as amendment 4 fixes it, D64
  preserved); Step 7 SWEPT - my own walk found a consumer the dispatch did
  not list: the sentence "No absence-shaped check exists in this task" was
  falsified by the new invariant grep and is now scoped with it. Coverage
  map: NO CHANGE - the design is unamended, the D101 row already reads
  Task 4, no new design obligation exists to place. Sequencing 4-5 edge: NO
  CHANGE - "share ... the CLI test crate with their neighbours" already
  covers `support/mod.rs`, and the funnel's behavior for every existing
  caller is byte-identical under the delegation, so no new edge arises.
- **No step renumbering, deliberately and unlike amendment 3** (recorded in
  the log entry): the vehicle instruction answers the same design entry as
  Step 4's own pinned test (D101), not a different one - amendment 3's
  ground for a dedicated step - and Task 4's executed report and review cite
  the current step numbers, which a mid-fix-round insertion would dangle.
  The instruction lives inside Step 4 as its own bold-led bullet.
- **The Step-7 invariant check added with the amendment** (tests ship with
  the package that introduces the behavior): the ruling's load-bearing claim
  is greppable, the Task-4 review will run the same walk, and the check's
  red state is reachable by construction (a `cargo_bin` call in
  `cli_validate.rs` instead of the helper adds a second file to the output).

## 5. Surfaced for the controller

- **Concurrent writer observed, handled by scoping:** at commit time the
  shared tree carried uncommitted modifications to `docs/decision-ledger.yaml`
  and `docs/process-conventions.yaml` (not mine; house YAML is
  controller-written). The pathspec-scoped commit left them untouched -
  `git status` after `ba69c36` still shows both files modified, unstaged.
- **The dispatch's "exactly one call site, in one function" wording** (see
  section 3): worth a precise formulation wherever the ruling is ledgered -
  the file-level grep is the invariant that holds verbatim; the
  single-call-site claim holds for the pinned path only, since
  `muxsmith_bare` keeps its own construction inside the closed exception.
- **The Task-4 report's section-6 items stand unhandled by this amendment**
  (correctly, they are controller calls): the stale D64 snapshot counts in
  the closed plan-7 design (`:1556`/`:1563`), and the proposed ledger rule
  about plan-pinned invocation forms against a closed funnel (probe the
  invocation once at authoring). Neither belongs in the Plan-9 plan file.

## 6. Commit

Hash: `ba69c36`, pathspec-scoped to the plan file, unsigned, single trailer,
not pushed. `git show --stat --no-color ba69c36`, pasted:

```
commit ba69c36c7422252b8fe138a04c263374b413e3c7
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 20:29:53 2026 +0200

    plan: amendment 4, the German subprocess test rides a locale-parameterized pinned helper (D64 preserved)
    
    Task 4's pinned invocation "args plus --locale de" was impossible through
    the en-appending funnel (clap rejects a repeated --locale, and the
    mis-invoked test passes .code(2) on clap's usage error with an empty
    snapshot). Owner ruling, option B of the Task-4 memo: support/mod.rs gains
    muxsmith_localized(args, locale), the muxsmith funnel delegates with "en",
    the funnel rustdoc follows, muxsmith_bare's closed two-caller set is
    untouched. Plan-only: D101 is unchanged in substance, the design gains no
    round. Files list, Step 4, Step 7 invariant check, git add, must-not list
    and the amendment log carry it.
    
    Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>

 .../2026-07-28-plan-9-core-hoists-planner-seam.md      | 18 ++++++++++++++----
 1 file changed, 14 insertions(+), 4 deletions(-)
```


---

# Fix round (2026-07-28, against amendment-4-plan-verdict.md, APPROVED_WITH_MINORS)

Both findings verified at the source and ACCEPTED; the verdict's exact
wordings taken verbatim (both were precise; no improvement needed). Commit
`4e5daa6`, pathspec-scoped to the plan file, not pushed. Three changed lines
= MEDIUM-1's two mandated edits plus LOW-1's one.

## MEDIUM-1 accepted: the helper's own rustdoc pinned

Premise verified before editing: every pub helper in
`crates/muxsmith-cli/tests/support/mod.rs` carries a contract-grade doc
(module doc `:1-7`, `insta_settings`, `insta_settings_with_tmp`,
`fake_mkvmerge_that_fails_queries`, the funnel `:80-88`, `muxsmith_bare`
`:96-107` - each confirmed by read), so an undocumented new pub fn is a
house-pattern outlier, and my own Files-entry qualifier "nothing else in
this file" would literally forbid writing the doc - a genuine omission-form
fork my amendment left open, in exactly the amendment whose purpose was
closing one. Step 4 gained the verdict's sentence (the helper doc states
the pinned-path construction-site fact and the append-after rationale, and
points at the funnel's rustdoc for the D64 contract and file-level
invariant rather than restating them - say-once preserved, consistent with
the single-homing choice already settled); the Files entry gained "with its
own rustdoc".

## LOW-1 accepted: the log cites only the existing report

Premise verified: `.superpowers/sdd/plan-9/` contains `task-4-brief.md` and
`task-4-report.md` but no task-4 verdict (fire control: the same ls pattern
finds `task-2-verdict.md`), and the execution order is amendment -> fix
round -> review, so a future review reads the amended numbering and cannot
dangle. My sentence had enumerated a consumer that does not exist - a
prediction cited as a citation. Replaced with the verdict's clause; the
non-renumbering decision itself stands on the report alone.

## The exact diff (git show -U1 4e5daa6)

```diff
diff --git a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
index 0948308..f2b83ee 100644
--- a/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
+++ b/docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md
@@ -280,3 +280,3 @@ Read first: design D101 in full (both forks, the boundary paragraph, the accepte
 - Modify: `crates/muxsmith-cli/tests/cli_validate.rs` (the two pinned subprocess tests)
-- Modify: `crates/muxsmith-cli/tests/support/mod.rs` (amendment 4: the new locale-parameterized pinned helper `muxsmith_localized`, the `muxsmith` funnel becoming a delegation to it with `"en"`, and the funnel's rustdoc updated to match that delegation - nothing else in this file; `muxsmith_bare` and its closed two-caller exception doc stay byte-identical)
+- Modify: `crates/muxsmith-cli/tests/support/mod.rs` (amendment 4: the new locale-parameterized pinned helper `muxsmith_localized` with its own rustdoc, the `muxsmith` funnel becoming a delegation to it with `"en"`, and the funnel's rustdoc updated to match that delegation - nothing else in this file; `muxsmith_bare` and its closed two-caller exception doc stay byte-identical)
 - Create: the two insta snapshot files those tests accept, named by the directory's existing `<test_file>__<test_fn>.snap` convention: `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap` and `crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`
@@ -297,3 +297,3 @@ Read first: design D101 in full (both forks, the boundary paragraph, the accepte
   - `bare_raw_property_exits_two_and_renders_the_message`: the exact profile of the authoring probe (`profile_version: 1`, `input: { pattern: 'E(\d+)', extensions: [mkv] }`, one rule `- match: { exact: { 'raw:': eng } }`); assert `.code(2)`; snapshot the stdout (the snapshot must contain the en text of D101's fence).
-  - **The invocation vehicle (amendment 4).** The originally pinned form, "args plus `--locale de`" through `support::muxsmith`, is impossible: the funnel appends `--locale en` AFTER the caller's args and clap rejects a repeated `--locale` (measured at amendment time: exit 2, `error: the argument '--locale <LOCALE>' cannot be used multiple times` on stderr - which also means the mis-invoked test PASSES `.code(2)` on clap's usage error while snapshotting empty stdout; the de-fence content assertion is what unmasks it). Instead, `crates/muxsmith-cli/tests/support/mod.rs` gains `pub fn muxsmith_localized(args: &[&str], locale: &str) -> Command` - today's funnel body with the locale parameterized: `cargo_bin("muxsmith")`, then the caller's args, then `--locale <locale>` appended LAST (preserving the after-the-subcommand guarantee the funnel's doc records) - and `muxsmith(args)`'s body becomes exactly `muxsmith_localized(args, "en")`. The funnel's rustdoc is updated in the same edit (the delegation falsifies its "builds its `Command` here" and appending description): it keeps the D64 contract statements (explicit locale pinning on the CLI's own surface, never environment variables) and states the delegation plus the file-level invariant, `cargo_bin("muxsmith")` confined to this file. `muxsmith_bare` and its closed two-caller exception doc are untouched.
+  - **The invocation vehicle (amendment 4).** The originally pinned form, "args plus `--locale de`" through `support::muxsmith`, is impossible: the funnel appends `--locale en` AFTER the caller's args and clap rejects a repeated `--locale` (measured at amendment time: exit 2, `error: the argument '--locale <LOCALE>' cannot be used multiple times` on stderr - which also means the mis-invoked test PASSES `.code(2)` on clap's usage error while snapshotting empty stdout; the de-fence content assertion is what unmasks it). Instead, `crates/muxsmith-cli/tests/support/mod.rs` gains `pub fn muxsmith_localized(args: &[&str], locale: &str) -> Command` - today's funnel body with the locale parameterized: `cargo_bin("muxsmith")`, then the caller's args, then `--locale <locale>` appended LAST (preserving the after-the-subcommand guarantee the funnel's doc records) - and `muxsmith(args)`'s body becomes exactly `muxsmith_localized(args, "en")`. The funnel's rustdoc is updated in the same edit (the delegation falsifies its "builds its `Command` here" and appending description): it keeps the D64 contract statements (explicit locale pinning on the CLI's own surface, never environment variables) and states the delegation plus the file-level invariant, `cargo_bin("muxsmith")` confined to this file. `muxsmith_localized` carries its own rustdoc (every helper in this file is documented): it states that it is the pinned path's construction site - the locale appended AFTER the caller's args, so it follows the subcommand - and that `muxsmith` is its `"en"` delegation; the D64 contract rationale and the file-level invariant stay stated once, in the funnel's rustdoc, which the helper's doc points at rather than restates. `muxsmith_bare` and its closed two-caller exception doc are untouched.
   - `bare_raw_property_renders_german_with_locale_flag`: same profile, invoked as `support::muxsmith_localized(args, "de")` (amendment 4); `.code(2)`; snapshot (must contain the de text of D101's fence - the content assertion discriminates a real German render from clap's stderr-only usage error, which also exits 2).
@@ -474,2 +474,2 @@ Routing: the decision memo in `.superpowers/sdd/plan-9/task-4-report.md` section
 - **D64 is preserved, not reopened - the mechanism by name:** every invocation stays explicitly locale-pinned on the CLI's own `--locale` surface, never environment variables (D64's rejected alternative stays rejected); `cargo_bin("muxsmith")` stays confined to `support/mod.rs` (the file-level greppable invariant, `grep -rln` measured at amendment time: exactly that one file - now a Step-7 check); the pinned path keeps a single construction site (the helper, with the funnel delegating), and the bare unpinned helper's closed two-caller exception set is untouched, which is why option B won over a third bare caller. The helper is also outside the amendment-1 infrastructure boundary, whose OUT set is enumerated (Vitest, `tauri::test`/`mock_builder`, `src-tauri/tests/`, IpcError funnel) and does not contain a support-module function.
-- **Consumer sweep, each surface named with its verdict:** the `git add` block and the must-not-decide list gained the amendment (swept); Step 7's "no absence-shaped check exists in this task" sentence was falsified by the new invariant check and is now scoped with it (swept); the coverage map's D101 row stays "Task 4" unchanged (the design is unamended, so the map has no new obligation to place); the sequencing 4-5 edge stays unchanged (its "share ... the CLI test crate with their neighbours" already covers `support/mod.rs`, and the funnel's behavior for every existing caller is byte-identical under the delegation, so no new edge arises). **No step renumbering, deliberately and unlike amendment 3:** the vehicle instruction answers the same design entry as Step 4's own pinned test (D101) rather than a different one, and Task 4's executed report and review cite the current step numbers, which an insertion would dangle mid-fix-round.
+- **Consumer sweep, each surface named with its verdict:** the `git add` block and the must-not-decide list gained the amendment (swept); Step 7's "no absence-shaped check exists in this task" sentence was falsified by the new invariant check and is now scoped with it (swept); the coverage map's D101 row stays "Task 4" unchanged (the design is unamended, so the map has no new obligation to place); the sequencing 4-5 edge stays unchanged (its "share ... the CLI test crate with their neighbours" already covers `support/mod.rs`, and the funnel's behavior for every existing caller is byte-identical under the delegation, so no new edge arises). **No step renumbering, deliberately and unlike amendment 3:** the vehicle instruction answers the same design entry as Step 4's own pinned test (D101) rather than a different one, and Task 4's executed report cites the current step numbers, which an insertion would dangle mid-fix-round (the review runs after the fix round, against the amended numbering).
```

## Commit

`git show --stat --no-color 4e5daa6`, pasted:

```
commit 4e5daa6fd45a3f50f545381663e0f7b98e128038
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 20:42:51 2026 +0200

    plan: amendment-4 fix round - pin the helper's own rustdoc, cite only the existing report
    
    MEDIUM-1: muxsmith_localized's own doc comment is now specified (pinned
    path's construction site, append-after rationale, muxsmith as the "en"
    delegation; D64 contract and file-level invariant stay single-homed in the
    funnel's rustdoc, pointed at, not restated) and the Files entry names it.
    LOW-1: the non-renumbering rationale cites only Task 4's report - the
    review does not exist yet and will run against the amended numbering.
    Nothing else changed.
    
    Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>

 .../superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md | 6 +++---
 1 file changed, 3 insertions(+), 3 deletions(-)
```

Typography over the whole plan after the edits: zero banned glyphs (exit 1;
fire control on an em-dash sample: 1). Nothing disputed.
