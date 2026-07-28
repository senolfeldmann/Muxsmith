# Plan 9 design review, round 1

Reviewer: independent, fresh eyes; did not author the design. Artifact:
`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
(D91-D105, 1397 lines at cd8a27a per `wc -l`; this header first said 1398, a
reader-tool count - the same defect class as finding M-1, corrected in the
round-2 delta below). Reviewed 2026-07-28 against the working tree
and the ground-truth hierarchy of the review brief. All probes below were
built and run by this reviewer at
`/tmp/claude-1000/-home-senol-agents-peter/6556b0df-2581-4c8d-8a85-e1f1b567eb55/scratchpad/p9rev/`
(own CARGO_TARGET_DIR build of the CLI, own fixture profile
`rev-probe-empty-raw.yaml`, own srt/mkv fixtures) - no author instrument was
re-run, and every negative grep reported here was fire-verified against a
named known-present case.

## Verdict: NEEDS FIXES

The design is structurally strong: all nineteen forks are closed, all nine
recon OPEN-QUESTION sections are resolved or ruled out, the seven deliberate
divergences are individually dispositioned, every user-visible string is
written out in both locales, and the section-0 refutations 2-5 all verify at
the tree. What blocks approval is one unsound completion check inside D91,
two acceptance observables whose claimed producers do not exist (the exact
"the work already exists" shape the brief warns has recurred in this
project), and one explicit latitude clause.

---

## Important

### I-1. D91's inline funnel-migration check has an unreachable green state

Location: D91, the S2 parenthetical ("after this change `git grep -n
"lint::provable_overlaps" -- crates src-tauri` must hit only
`profile/validate.rs` and `lint.rs` themselves, a check fire-verified per
section 8's rules").

What is wrong, established empirically:

```
$ grep -rn "lint::provable_overlaps" crates src-tauri
crates/muxsmith-core/src/profile/validate.rs:189   (doc comment)
crates/muxsmith-core/src/profile/validate.rs:195   (the funnel's call)
crates/muxsmith-cli/src/commands/dry_run.rs:21     (doc comment)
crates/muxsmith-cli/src/commands/dry_run.rs:60     (copy 1)
crates/muxsmith-cli/src/commands/run.rs:85         (copy 2)
src-tauri/src/lib.rs:211                           (copy 3)
src-tauri/src/run.rs:263                           (copy 4)

$ grep -rn "lint::provable_overlaps" crates/muxsmith-core/src/profile/lint.rs
(no hits; and the file is profile/lint.rs, not lint.rs)
```

Two defects in the predicted survivor set: (a) `lint.rs` contributes zero
hits today and can contribute none after the change - the qualified string
never appears in the module that defines the function - so the expected set
names a member that cannot occur; (b) `dry_run.rs:21` is a doc-comment hit
that is NOT in the recon A-2 consolidation set D91 migrates
(A-2 = `dry_run.rs:41-46/64-68/85-94`), so it survives the hoist and the
check as stated is red on a correct implementation. This is precisely the
defect class the house records as `proc-check-green-state-reachable`.
Additionally, "per section 8's rules" dangles: the design's own sections run
0-7 plus the amendment log (verified against its `^##` headings); if the
brief's section 8 is meant, say so.

Note the design already contains the sound form of this same check:
acceptance observable 1 uses the pattern
`config_diags.extend(lint::provable_overlaps`, which I ran on the unmodified
tree and which hits exactly the four copies and nothing else (the funnel's
own line is `diags.extend(...)` and does not match). Fix: delete or rewrite
D91's inline grep to the acceptance-1 form (or state the true expected
survivor set: `validate.rs:189`, `:195`, and `dry_run.rs`'s fn doc or its
rewrite), and repair the section reference.

### I-2. Acceptance 5's GUI-Run-gate observable has no existing producer

Location: section 7, item 5: "the GUI Run gate disables on the same profile
(covered by the existing `hasErrors` path, whose e2e already exercises
error-severity config diagnostics)".

The premise was run, not weighed, and it is false on both halves:

```
$ grep -rn "batch-run" e2e/           -> one functional hit: smoke.spec.ts:510
$ sed -n '510,512p' e2e/smoke.spec.ts -> await expect(runButton).toBeEnabled();
$ grep -n "severity" e2e/smoke.spec.ts (batch describes, :140-476)
   -> "info" and "warning" only; no error-severity config diagnostic is ever
      fed to BatchView by any e2e scenario
$ grep -n "toBeDisabled" e2e/smoke.spec.ts
   -> :1241 only, and that is the EDITOR Save gate (":1180 Save is disabled
      while an error diagnostic exists"), a different view and a different
      gate
```

Fire control: the same greps produce the hits quoted above, so the absences
are real. No e2e drives `BatchView.vue`'s `hasErrors` (`:282`) with an
error-severity diagnostic, and none asserts `batch-run` disabled. An
acceptance item whose observable has no producer is not acceptance (brief
dimension 12), and "covered by Y" is the no-work-needed shape dimension 8
exists for. Fix: either add the producer to the design's pinned tests (a
mock document with one error-severity config diagnostic asserting
`batch-run` disabled plus its `.tooltip-errors` reason - the key already
exists in `gui-batch.ftl`), or strike the claim and let ruling 5's
acceptance rest on the emitters that exist.

### I-3. Acceptance 6's "apply-suggestion parse-failure e2e path" does not exist

Location: section 7, item 6: "the apply-suggestion parse-failure e2e path
still surfaces the parse diagnostic".

```
$ grep -rn "profile: null" e2e/
e2e/help-mode.spec.ts:363   (help-mode view; also built from emptyReport, so
                             config_diagnostics is EMPTY there - it would
                             exercise the console.error contract-violation
                             branch, not the diagnostic fetch)
$ grep -n "load_profile" e2e/smoke.spec.ts
   -> :415 resolveWith(loadedForApply) - the apply e2e (:406) covers the
      SUCCESS path only; no scenario resolves load_profile with a
      parse-error document
```

Fire control: the same `profile: null` grep finds the help-mode hit, so the
absence in the apply path is real. The only test of the branch D103 edits is
therefore the one D103's change itself must not break - there is no e2e that
"still surfaces the parse diagnostic". Fix: same two options as I-2 (pin a
test, or restate the emitter honestly - e.g. a mount/mock assertion that a
`profile: null` document whose `config_diagnostics` contains `parse-error`
surfaces it through the alert line).

### I-4. Latitude clause in D102: "a one-line delegate to it (or a re-export)"

Location: D102, the sentence "the CLI's `pub(crate) severity_sorted`
(`commands/mod.rs:21-25`) becomes a one-line delegate to it (or a
re-export)". This is the explicit either/or the brief's section 5 bans in
both forms and dimension 5 hunts; every other decision in the document picks
one shape. The observable difference is nil, which makes the fix one word:
pick one (recommend the re-export - it is the "exactly one ordering
definition" form `core-derive-dont-restate` wants with zero wrapper code)
and delete the parenthetical.

---

## Minor

### M-1. Section 0 note 1 mis-corrects an accurate figure

The note claims "`wc -l` on the file reports **1120**". It does not:

```
$ wc -l .superpowers/sdd/plan-9/recon-inventory.md   -> 1119
$ tail -c 1 ... | xxd                                -> 0a (trailing newline present)
$ awk 'END{print NR}' ...                            -> 1119
```

The brief's and ROADMAP's "1119 lines" was correct; the design's correction
is the error (likely a reader-tool line count attributed to `wc -l`). Drop
the note or invert it. Piquant because the note's only purpose is figure
hygiene, and the house rule is that a cited command's output must reproduce.

### M-2. D98's stdio-grep control count is wrong for the quoted pattern

D98: "control: the same grep over `crates/muxsmith-cli/src` returns hits in
three files". With the pattern actually quoted
(`eprintln!\|println!\|print!(`) it returns SIX files (main.rs, identify.rs,
dry_run.rs, validate.rs, mod.rs, run.rs); three is the `eprintln!`-only
count (identify.rs, dry_run.rs, run.rs). The control still fires and the
core-side result (exactly one call at `queue.rs:396` plus the `lib.rs:23`
comment) reproduces, so the conclusion stands; the recorded evidence line
does not. Fix the count or the pattern.

### M-3. D98's JobOutcome-constructor enumeration is muddled and incomplete

"(the three job.rs producers, finish, the CLI test helper, the e2e fixtures
via the TS type below)": `job.rs:187` IS `finish`'s constructor (job.rs has
exactly three literal constructors, `:103`, `:124`, `:187`), so "finish" is
a double-count of the third; and the list omits `run_queue`'s Cancelled
fallback constructor (`queue.rs:315`) and the two core test constructors
(`tests/report_json.rs:13`, `tests/executor_events.rs:55`). The compiler
enforces completeness so nothing breaks, but a wrong enumeration in a
normative position is the defect class the house count-rule records. Fix the
parenthetical or replace it with "every constructor the compiler flags".

### M-4. D98's fixture-site claim names a scenario with no such literal

"the `JobOutcome`/`RunJobEntry` object literals in `e2e/smoke.spec.ts`'s
live-run and german-locale scenarios": all JobOutcome-shaped literals sit in
the live-run describe (`:554`, `:559`, `:569-577`; `grep -n duration_ms`
returns nothing else in `e2e/`); the german-locale describe (`:610`)
contains none. Drop "and german-locale".

### M-5. Several D-sections carry no labeled rejected alternative

D97, D100, D101, D102, D103 have decision and rationale but no rejected
mechanics alternative (dimension 10 asks for one per Dn). The ruled arms
live in the ledger steelmen and must not be re-litigated - but the mechanics
each had a real alternative worth one line: D97 a trivial retained wrapper;
D100 a dedicated new key vs reusing `worker-panicked`; D101 per-arm checks
vs the shared funnel (currently argued inline but not framed as the rejected
shape); D102 sorting inside `rendered_diags`; D103 a severity-keyed or
`[0]`-fallback fetch. One sentence each closes the gap.

---

## Verified and held (so the fix round does not re-plow it)

- **Forks 1-19**: each closed by a named D-section; the D-1..D-7 divergence
  table is complete and none is flattened. D-2/D92: verified at
  `capability/runtime.rs:94-104` that `locate` maps only `Spawn`->`NotFound`
  and both CLI copies match `Err(_)`, so section 0 note 4 is right and D92's
  unified meaning changes no surface's behavior. D95: all four `"."`
  defaults confirmed at the sites. D96: tee-before-on_event order confirmed
  in both bodies; the two named tests exist (`run.rs:1228`, `:1334`) and use
  core-owned `FakeSpawner` (`spawn.rs:165`); `tempfile` is already a core
  dev-dependency.
- **Recon OPEN QUESTIONS** (1.4, 2.3, 3.4, 4.5, 5.4, 6.4, 7.4, 8.5, 10.4):
  every one closed by a decision or by a ruled OUT with the vehicle named.
- **Section 0 notes 2-5**: all verify at the tree (BatchView `!doc.profile`
  detection with singleton `[parse-error]` envelope - `load.rs:62`/`:71`
  both `ParseError`, `lib.rs:308-322`; the static-props mount harness;
  `jobRowState.ts` embedding the whole outcome). Note 1 is the exception
  (M-1).
- **Empirical re-verification with own instruments**: `resolve_runs_root`
  has exactly the three call sites (`:326`, `:529`, `:535`) and
  `MUXSMITH_RUNS_ROOT` has exactly the two producers + five CLI-test
  consumers (own grep; fire control `default_runs_root` = 15 hits,
  matching the design's figure). `worker-panicked` exists in both locales
  (`en:80`, `de:87`, text defers to "the application log" as D99 claims)
  and is looked up by no runtime code (fire control: the same grep on
  `run-job-failed` hits the render site `run.rs:489`). Empty-`raw:`
  behavior reproduced end-to-end with an own-built CLI and own fixtures:
  validate emits info `RawProperty` with `property: ""` on BOTH arms, exit
  0; dry-run against a real mkv shows the rule never matching and
  `UnknownPropertySkew` rendering `Property ""` at plan time - D101's
  premises and its "matcher and planner unchanged" boundary hold.
  `BatchView.vue:225` is the only positional consumer of
  `config_diagnostics` (EditorView `:209`/`:232` and BatchView `:268` are
  display-order-only, and D102 names their user-visible change). Highest
  ADR is D90 (plan-8 `^## D` = D75-D90); the only `D91` string in the tree
  is the plan-8.5 hypothetical.
- **No-work-needed premises run**: the acceptance-1 grep hits exactly the
  four copies (quoted in I-1); core stdio grep = 1 call; spec contains no
  `mkvmerge_found` (control: 13 hits in `src-tauri/src/lib.rs`); CLI JSON
  tests assert membership, never position (`config_diags[` = no hits;
  control: the `.iter().any` shapes at `dry_run_cli.rs:115-118`); the
  `no-raw-text` lint exists (`eslint.config.js:61`);
  `locales/*/diagnostics.ftl` is globbed by `src/i18n/index.ts`, so the GUI
  needs no new key; `render_summary` renders counts only, so D99's two-line
  change covers the CLI's whole human failure surface. I-2/I-3 are the two
  premises that failed.
- **Strings/keys (dimension 6)**: complete. `EmptyRawProperty` variant,
  `empty-raw-property` key, S-1 severity row, fixture row `vec![]`
  (message has no placeholders - consistent), both locales verbatim;
  `run-job-panicked` matches the `run-job-*` house shape (leading `[` is
  already Fluent-legal per the existing keys); `catalog_completeness`
  structures (`ALLOWLISTED_CLI_KEYS`, `allowlisted_cli_key_args`,
  `WorkerPanicked => vec![]` at `:152`) are as the design says.
- **Spec amendments (dimension 11)**: all seven anchors quoted verbatim
  match the spec on disk (5.2 row at spec:289 character-for-character; 8.4
  fragment; 4.4/5.4/9.2 sentences; 5.2 preamble). The contradiction sweep's
  4.3 claim verified ("which opts out of the type check", spec:142). No
  collateral contradiction found in my own sweep of `raw`,
  `worker-panicked`, `config_diagnostics`, `mkvmerge_found` over the spec.
- **SI-3 (dimension 13)**: the complete set of two comparisons plus the
  explicit nothing-else statement is present. Source anchors verified at
  mkvtoolnix 100.0: `job.h:70-71` accessors, `model.cpp:212` exactly the
  quoted `setIcon(numErrors ? m_errorsIcon : ...)`, `tab.cpp:314-315`
  routing, `mux_job.cpp` `lineRead` emissions, `output.cpp:193-196`
  `default_mxerror` -> `mxexit(2)`. Behavior reproduced with own fixtures
  on the installed `mkvmerge v100.0`: warning inline + exit 1; first error
  terminal + exit 2. No wording adopted from mkvtoolnix text.
- **D104**: the round-2 quote verified verbatim at the primary artifact
  (deviation-judgment section); the five traced orderings match; the
  asserted subset (2, 3, 4 of 5 + the D100 rider) is licensed by the fork's
  own "which of the five" framing, with 1 covered by smoke `:477` and 5's
  observable core inside test 3; `installMockIPC` is exported and
  documented serializable; `run-already-active` is a real backend code;
  `cancel-batch` disabled-binding and the `role="alert"` sites make every
  assertion expressible; `defineModel` local-ref fallback claim is correct
  for the pinned Vue.
- **D105**: mutation site (`planner.rs:1820-1827`), `scalar_display`
  (`:856`), and all three guard names/lines (`suggestions.rs:1037`,
  `:1074`, `:1113`) verified; the decision rule matches the registered
  ROADMAP trigger verbatim; no guard is removed in-plan; the two new ledger
  ids collide with nothing.
- **Scope (dimension 14)**: no Vitest, no `tauri::test`, no
  `src-tauri/tests/`, no IpcError funnel work, no new dependency
  (the props-ref hook uses already-imported `vue` machinery), no
  product-boundary change; the reactive-props hook is the
  controller-sanctioned extension recorded in the ROADMAP anchor.
- **Safeguard survival (dimension 9)**: nothing proposed by the brief,
  recon or earlier rounds is argued away; the four discarded executor
  failures stay discarded per the ruled steelman, restated in section 2.

## HARVEST

- **Dominant pattern, positive**: the design's fire-verification habit is
  mostly genuine - of the ~10 verification claims I re-ran, all but the
  four findings above reproduced exactly, including counts (the 15-hit
  control, the four-site grep, the three `locate()` sites). The failures
  cluster where the design *narrates* a check instead of pasting its
  output: I-1 (predicted survivor set never run against the end state),
  M-1/M-2 (command named, output misreported). A house handle that would
  have caught all four: an evidence line in a design may only contain
  output that was pasted, not recalled.
- **Repeated rejection shape**: "unconsumed wire surface" (D92's dropped
  failure reason, D98's untouched `delete_partial_failed`), each paired
  with a section-6 trigger naming the reopening event and the designated
  mechanism. Good pattern; worth promoting to a convention if it recurs in
  a third design.
- **Duplicate-check hazard**: D91 and acceptance-1 carry two different
  greps for the same migration, one sound, one not (I-1). When a design
  states the same completion check twice, the copies drift exactly like
  the four pipeline copies this plan removes; state it once, in the
  acceptance section, and point at it.
- **Brief boundary, wanted feedback**: the brief's dimension-4 phrasing
  "BatchView's index-0 read is the only order-dependent consumer" is
  narrower than reality - EditorView `:209`/`:232` and BatchView `:268`
  are order-*sensitive* display consumers whose rendering changes under
  D102. The design surfaced them anyway; a re-run of this brief should say
  "only positional consumer" to keep the check honest.
- **No over-restriction stops**: the own-instruments rule cost one 4-second
  scratch-dir CLI build and forced no detour; the no-git rule never bound
  (all needed history facts were in the documents). Nothing in the brief
  blocked a check I judged necessary.

---

# Round 2 delta review (2026-07-28, same judge)

Scope: the nine routed fixes only, at commit `612eae9` (1494 lines,
`wc -l`, trailing newline present - measured fresh). Nothing from round 1's
"Verified and held" was re-opened. All instruments this round were run
fresh from
`/tmp/claude-1000/-home-senol-agents-peter/6556b0df-2581-4c8d-8a85-e1f1b567eb55/scratchpad/p9rev-r2/`
context; none of the round-1 probe artifacts were re-executed.

## Verdict: APPROVED

All nine fixes close their findings as routed; none introduces a defect.
Two non-blocking notes below for the controller's discretion.

## Per-finding verdicts

- **I-1: CLOSED.** D91's inline grep and the "per section 8's rules"
  reference are gone; the S2 parenthetical now points at acceptance
  observable 1 as the single stated completion check, citing
  `design-states-a-completion-check-once`. The new citation resolves: the
  entry exists at `docs/decision-ledger.yaml:4507` and its statement
  matches the use. `grep -n "provable_overlaps"` over the design confirms
  the check now exists exactly once as a check (acceptance 1, with its
  in-place control); `:248` and `:934` are descriptive prose. No "must hit
  only" or dangling section reference survives (sweep grep: the only
  "section 8" hits are S-2's legitimate "section 8.4" and the amendment
  log's description of the removal).
- **I-2: CLOSED as routed** (no new e2e; honest statement instead). D101's
  new consequence paragraph states the Run-gate behavior as real,
  user-visible and uncovered, names the evidence correctly (no e2e feeds
  BatchView an error-severity config diagnostic; the only error-gate
  `toBeDisabled` is the editor Save test), and routes coverage onto the
  FIRM v1.x "GUI test harness for the run path" vehicle - consistent with
  that OUT item's deliberate no-trigger ruling, so no notice-it-yourself
  shape was created. Acceptance 5 now says explicitly the consequence "is
  not claimed as acceptance".
- **I-3: CLOSED as routed.** Acceptance 6 and D103's new "Coverage, stated
  plainly" paragraph replace the phantom producer with the accurate
  statement (the only `profile: null` fixture, `help-mode.spec.ts`,
  carries empty diagnostics and exercises the `console.error` branch);
  D103's correctness rests on the singleton-envelope evidence round 1
  already verified at `load.rs:62`/`:71` and `lib.rs:308-322`.
- **I-4: CLOSED.** The re-export is picked and stated precisely
  (`pub(crate) use muxsmith_core::report::severity_sorted;`). Verified
  compilable against the real import forms: `validate.rs:11`,
  `run.rs:23`, `dry_run.rs:14` all import via `crate::commands::{...}`,
  and `mod.rs:117`/`:121` call it bare in-module - every form resolves
  through a `pub(crate) use` with the identical signature. Both downstream
  references were swept as claimed: the consumers paragraph ("now through
  the re-exported core `severity_sorted`") and section 5's bullet ("the
  CLI helper is deleted in favor of the D102 `pub(crate) use` re-export").
  A design-wide grep finds "delegate" only in the amendment log's
  historical description.
- **M-1: CLOSED.** Note 1 is inverted, keeps notes 2-5 numbered, and its
  pasted evidence reproduces exactly on a fresh run:
  `wc -l` = 1119, `awk 'END{print NR}'` = 1119, `tail -c 1 | xxd` = `0a`.
- **M-2: CLOSED.** The control line now claims six files for the quoted
  three-macro pattern. Fresh run of
  `grep -rl "eprintln!\|println!\|print!(" crates/muxsmith-cli/src`
  returns exactly the six named: `main.rs`,
  `commands/{mod,validate,identify,dry_run,run}.rs`.
- **M-3: CLOSED.** The muddled enumeration is replaced by the semantic
  statement ("`recover_panicked_worker` is the only constructor that sets
  `Some`; every other `JobOutcome` constructor the compiler flags sets
  `panic: None`"), which cannot go stale. Note 1 below records the one
  reading it requires.
- **M-4: CLOSED.** "and german-locale" dropped. Fresh
  `grep -rn duration_ms e2e/` (excluding `.generated`): all four
  JobOutcome-shaped literals sit at `smoke.spec.ts:554/:559/:569/:577`,
  inside the live-run describe.
- **M-5: CLOSED.** Each of D97/D100/D101/D102/D103 carries one labeled
  rejected-mechanics alternative, and each is a real steelman with a
  stated cost (trivial wrapper's false promise; duplicate catalog key;
  per-arm predicate drift; `rendered_diags`-wide scope creep;
  severity-fetch masquerade plus `[0]`-fallback regression). None
  re-litigates a ruled arm.

## Ripple sweep

The sweep the fixes owe was run over the whole document: no stale "or a
re-export"/"three files"/"german-locale"/"1120"/"must hit only"/dangling
"section 8" wording survives outside note 1 and the amendment log, where
the old text is quoted deliberately as history. The amendment log's nine
entries match the fixes as landed. Section 2's counts, section 6's
triggers and every unchanged D-section carry no reference to the reworded
passages that would now dangle. The second new ledger citation introduced
by the fix round resolves; both new entries carry no obligation on this
document beyond what the design already states.

## Notes, non-blocking

1. **M-3's replacement sentence needs its intended reading.** "Every other
   `JobOutcome` constructor the compiler flags sets `panic: None`" is true
   of every *existing* constructor the field addition breaks; the two NEW
   test fixtures this same design pins (D99's `panic: Some(..)` unit test,
   D104 item 4's `panic: "boom"` e2e event) are new code, not
   compiler-flagged sites, so the sentence holds - but a reader taking
   "every other constructor" without the "compiler flags" qualifier will
   see D99 contradict it. One word ("existing") would remove the wobble.
2. **The general clause "new GUI test scenarios are outside the ruled
   scope" (D101 `:890-891`, D103 `:1010`)** is, unscoped, in tension with
   D104's own three new scenarios in `e2e/jobsview-reset.spec.ts`, which
   are ruled IN. Context and section 5's explicit enumeration win over the
   general clause (the house's own latitude ruling: an explicit
   enumeration always beats a silence-filling grant), so no implementer
   latitude arises; "beyond the ruled D23 tests" would say it exactly.
   Acceptance 5's instance (`:1426`) is already correctly scoped.

## Correction to this file's own header

Applied above: the round-1 header stated 1398 lines at `cd8a27a`; the
controller's `wc -l`/`awk` measurement at that commit is 1397, and my 1398
was a reader-tool count - the identical defect class as M-1. Recorded here
so the figure's provenance reproduces; the controller has logged this
round's count failures (including this one) on the reproducible-claims
house entry.

---

# Amendment-1 delta review (2026-07-28, same judge)

Scope: the amendment-1 edit only, `612eae9 -> 8eb8799` (read via
`git diff`, now permitted read-only), against the routing in
`.superpowers/sdd/plan-9/amendment-1-brief.md`. The owner's two rulings
are inputs, not under review. Probes this pass ran fresh from
`/tmp/claude-1000/-home-senol-agents-peter/6556b0df-2581-4c8d-8a85-e1f1b567eb55/scratchpad/p9rev-r3/`
context; nothing from earlier rounds was re-executed.

## Verdict: NEEDS FIXES

One finding, one word: the D93 ripple missed D91's rejected alternative
(b), which still counts the seam's parameters at six after the amendment
removed one. Everything else - both scenarios' discriminating power, the
rest of the ripple, S-8's new product prose including its hardest clause,
and the latitude scan - passes.

## Finding (Minor, blocks only because the pass's question was sweep completeness)

**D91 rejected alternative (b) still says "a function with six explicit
parameters is the simplest mechanism" (design `:315`); the amended
signature four lines above it has FIVE** (`profile_path`, `source`,
`output`, `on_collision`, `resolve_mkvmerge` - the diff removes
`cache: &mut IdentifyCache` and adds nothing). At `612eae9` six was
correct; the amendment's sweep list (signature block, S6, mappings 1 and
3, interface memo, rejected alternative (c), section 5 bullet, section 3
sweep) never visited alternative (b), and this is exactly the
count-follows-enumeration class the house entry on recomputed counts
names as its trigger 2 (a member left an enumerated set; the numeral
describing it lives elsewhere). Fix: "six" -> "five".

## Per-item verdicts

1. **The two scenarios discriminate: PASS.**
   - *Scenario 1 (D101 Run gate).* Verified against `BatchView.vue`: the
     pick flow validates without a dry-run click (`selectProfile` awaits
     `runValidate()`, which stores the `validate_profile` document in
     `report.value`); `hasErrors` counts over that document's
     `config_diagnostics`; and `runDisabledReason`'s chain is
     `runActive` -> no-profile (`!selectedProfile || !report`) ->
     `mkvmerge_found === false` -> `hasErrors`. The author's construction
     claim is true: the completed pick closes the first two branches and
     `mkvmerge_found: true` closes the third (strict `=== false` test),
     so a disabled button with the asserted `title` can only mean the
     errors reason. Both assertion targets exist on the button
     (`:disabled="runDisabled"`, `:title="runTooltip"`, where
     `runTooltip = $ta("batch-run")[reason]`), and the quoted en
     `.tooltip-errors` text matches `gui-batch.ftl`. Red paths: gate
     regression fails the disabled assertion; wrong-reason disable fails
     the title assertion; a severity regression of `EmptyRawProperty` to
     warning empties `hasErrors` and fails both.
   - *Scenario 2 (D103 parse-failure apply).* Verified: `parse-error`'s
     en text is exactly `The profile could not be parsed: { $detail }`,
     so the asserted substring is the literal prefix before the first
     placeable and Fluent's isolate marks cannot break it; the alert is
     the `role="alert"` line rendering `$t(ipcErrorCode, ipcErrorParams)`
     that the parse branch fills from the found diagnostic. If the
     code-keyed `find` misses `parse-error`, `ipcErrorCode` stays null
     (the else branch only `console.error`s), no alert renders, and
     assertion (a) goes red - the discriminating half; assertion (b)
     (no `apply_suggestion`/`save_profile` in the recorded invoke log) is
     the existing `installTauriMocks` `recorded` pattern.
   - *Infrastructure boundary: HELD.* Both scenarios use only
     `installTauriMocks` + `resolveWith(<document>)` + the existing
     dialog mock and plain Playwright assertions
     (`toBeDisabled`/`toHaveAttribute`/text-contains); no new harness
     machinery appears, and the design states the scenarios-in /
     infrastructure-out boundary explicitly in D101 with D103 pointing
     at it.
2. **D93 ripple: FAIL by one site** (the finding above). Independently
   swept with my own greps over every `cache`/`IdentifyCache`/
   `LiveIdentifier`/`Arc<Mutex`/`borrow`/`session` mention in the
   document: all other hits are correctly dispositioned - D91's signature
   block (five params, no cache), the rewritten S6 step, call-site
   mappings 1 and 3, the interface memo ("`LiveIdentifier` is unchanged"),
   rejected alternative (c) (rewritten to the per-call construction, with
   the `Identify` seam staying on `plan_batch`), section 5's D93 bullet,
   the section 3 contradiction sweep, and the amendment log. Section 2's
   recount and section 7 contain no cache assumption (grep: zero hits in
   either). Leftover-clause sweep: the only surviving "rides the v1.x" /
   "outside the ruled scope" strings are historical quotations inside
   D101's replaces-the-routing sentence and the amendment log.
3. **S-8: PASS, including the unverified clause.** The clause is true in
   the code, mechanically: `plan_batch` (`planner.rs:324-336`) threads
   one `id: &mut dyn Identify` into `plan_core` (`:331`, the planning
   pass) and then into `suggest` (`:333`), and `suggest` re-simulates
   every candidate via `plan_core(&edited, run, primaries, id, lang)`
   with that same identifier - one cache serving the baseline pass and
   every re-simulation, which `plan_core`'s own doc states ("re-invokes
   this on edited profiles to simulate ... against the cached
   identification"). Suggestions run unconditionally in `plan_batch`, so
   the claim holds for `run` too. S-8's quoted current sentence matches
   spec `:311` verbatim. Neighbour check, done myself over every `cache`
   mention in the spec (`:293`, `:309`, `:311`, `:340`, `:446`): the run
   bullet stays true (identification is cached within each call;
   staleness protection is the path+mtime+size key, never cross-call
   reuse), 5.3's "cached identification data" is exactly the mechanism
   just verified, the module table's "wrapper + cache" and the non-goals'
   on-disk-cache line survive unchanged. No contradiction.
4. **Latitude: NONE introduced.** D93's rewrite enumerates its
   non-changes; both scenario paragraphs pin flow, mocked commands, full
   document contents and assertion targets; section 5's new bullet closes
   the scenario set ("no scenario beyond these and the D104 four"); S-8
   is definite product prose. The round-2 note fixes (the "existing"
   qualifier in D98; the scoped clause, now superseded by ruling A's
   rewrite) landed as described in the log.

## Notes, non-blocking

1. Scenario 1's fixture sets `mkvmerge_found: true` on a
   `validate_profile` document, a key the real command never emits
   (`validate_profile_body` builds `config_only_document(.., None, ..)`;
   its doc: the field "is always absent ... this command never touches
   mkvmerge"). For the gate logic today, absent and `true` behave
   identically (only `=== false` triggers the missing branch), and `true`
   is the *more* robust pin for the errors-reason assertion if that
   strictness ever changes - but the fixture is knowingly wire-unfaithful
   to its producer, worth one comment line in the spec when written.
2. The status line's review-history counts ("four blocking findings, five
   minor") reproduce against round 1 of this file.
