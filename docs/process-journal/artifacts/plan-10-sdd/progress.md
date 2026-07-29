# Plan 10 - progress tracker

SDD ledger, plan: `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`

The plan document carries no progress (house deviation stated in its header);
this file is the tracker. Controller-written. A task's state is its row here,
never a ticked box in the plan.

Execution session: session 28, 2026-07-29. Base at kickoff: `754cb73`
(master == origin/master, clean tree).

| # | Task | State | Commits | Verdict |
|---|---|---|---|---|
| 1 | W3: `BUILDING.md` states the gate total once, `ledger-lint` checks it | DONE | `ddb8f42` (base `754cb73`) | `task-1-verdict.md` APPROVED_WITH_MINORS, no fix round; 5 minors routed below |
| 2 | W1: the D102 preserved-order producers, selected by measurement | DONE | `35bc363` (base `39a9055`) | `task-2-verdict.md` APPROVED_WITH_MINORS, no fix round; measurement reproduced independently |
| 3 | W4: `renovate.jsonc` at the repo root | DONE | `630d418` (base `9ff4173`) | `task-3-verdict.md` APPROVED_WITH_MINORS, no fix round; the one MEDIUM is against the plan's Step 3, not the artifact |
| 4 | W5: the user-facing documentation pass (README + INSTALL.md) | DONE | `e657263` + fix round `845cf89` (base `f2e9d75`) | `task-4-verdict.md` APPROVED_WITH_MINORS -> fix round -> delta: all three ADDRESSED, one new NIT deferred to the whole-branch review |
| 5 | W2: the comment line-citation sweep | DONE | `1a23283` (base `44f1c8e`) | `task-5-verdict.md` APPROVED_WITH_MINORS, no fix round; both absence checks independently fired by the controller (A: 0 / 20, B: 0 / 4) |

## Deferred minors and parked findings

**Task 1** (`task-1-verdict.md`, APPROVED_WITH_MINORS, none requiring a tree
change; all five disposed here at verdict arrival, none carried silently):

1. Report evidence line generalized past its measurement (alignment "reads off
   checks 1-6", true of two of six). Report-local. -> new ledger entry
   `an-evidence-line-names-the-set-it-actually-measured`.
2. `BUILDING.md:138` at 86 chars, the file's only over-80 non-fenced prose line,
   a consequence of a fenced edit. -> ROADMAP, routed onto the same vehicle as
   the positional ordinals; occurrence on
   `latitude-carveout-zero-content-structural-forks` (do-NOT-loosen datum).
3. `states 1 commands` unpluralized. Reproduces the plan's fenced example byte
   for byte; keeping it was right. No action.
4. The plan's five fires never reach the counting rule's comment/blank-line
   exclusion; the reviewer fired it with a tolerate-green / catch-red pair,
   behaviour correct. -> occurrence on
   `proc-verification-step-must-be-falsifiable` + new ledger entry
   `a-check-with-an-exclusion-rule-needs-a-probe-that-exercises-the-exclusion`.
5. The shipped check cannot see a FOURTH gate block (marker set fixed at three,
   sentence fixed at four numbers), so a future fourth section drifts silently
   green. Plan property, not an implementation defect. -> ROADMAP Triggers, with
   the readable event (you are adding a `gate-block` marker).

Harvest also mined at arrival: `proc-latitude-clause-boundary` gained the
fenced-strings edge case (one violation message fenced by example, five composed).

**Task 2** (`task-2-verdict.md`, APPROVED_WITH_MINORS, no fix round; the
reviewer reproduced all four mutations with its own function-scoped driver -
necessary because the sort block occurs twice in `json.rs` - and got the same
red/red/green/green pattern, plus 503 -> 505 recomputed from its own runs):

1. Report claim "a second `use` of the same module path is not this tree's
   pattern" refuted: 3 of 73 tracked `.rs` files carry it, so dominant rather
   than unbroken. Code stands, claim was overstated. -> new ledger entry
   `a-house-pattern-claim-is-counted-before-it-justifies-a-choice`.
2. Plan defect, measured: the fenced `mixed_severity()` body cannot survive
   `cargo fmt`, which the same plan makes a task exit bar (fenced line 96 cols,
   argument list 66 against rustfmt's `fn_call_width` 60). Committed form is
   rustfmt's output and differs from the fence only in line breaks, indentation
   and a trailing comma. -> new ledger entry
   `a-plan-that-fences-code-fences-the-formatters-output-form`.
3. The module doc's alleged staleness has no vehicle (the file is not in Task
   5's Files list). **Controller ruling: no vehicle is needed.** The comment is
   provenance-tagged "(Plan 5 Task 2)" and describes what THAT task added; a
   later additive test does not falsify it, and the reviewer's adjudication 2
   ruled the same way from the grant's side. Recorded rather than dropped.
4. INFO: this machine carries a second, shadowed `mkvmerge` at
   `/usr/bin/mkvmerge` besides the linuxbrew v100.0, which made a PATH-stripped
   control measure the opposite of its intent until the reviewer caught it.
   -> occurrence on `proc-verification-step-must-be-falsifiable`.
5. The ROADMAP's vehicle for the mkvmerge-gated coverage fact fired on the
   package the same paragraph exempts by name. -> reworded in the ROADMAP the
   same turn, and the coverage fact upgraded from claim to measurement (guard
   prints "mkvmerge not found; skipping" and exits 0 with the defect present).

**Task 3** (`task-3-verdict.md`, APPROVED_WITH_MINORS, no fix round; the
artifact is byte-perfect and all fifteen vendor premises held):

1. MEDIUM against the PLAN's Step 3: naming a file sets the validator's
   `configType` to `global`, so the fenced invocation is silently blind to
   repo-config defect classes. Proven with two probes that pass the fenced form
   and fail `--no-global`. Artifact clean under both modes. -> new ledger entry
   `a-validators-default-mode-is-part-of-the-commands-meaning`; `--no-global`
   written into the ROADMAP's Renovate entry for any later re-validation.
2. MINOR: the report's controller-facing section said the config "is on master"
   while the ordering rider is about what GitHub serves; measured 0 unpushed
   ahead-count and no remote branch containing it. -> new ledger entry
   `a-local-commit-is-not-on-master-for-a-claim-about-a-remote-service`.
3. MINOR: `ci-04-dependabot-cadence` never mentions runner images, though the
   shipped config disables `github-runner` on D85 grounds. -> occurrence on
   `ci-04` recording the exclusion and its origin (plan fence + D85, not a named
   owner ruling), **flagged for owner confirmation at the close.**
4. INFO: `EBADENGINE` is structural (renovate 44.0.1 declares the same node
   engine range), not a stale-pin artifact. No action.
5. Reviewer harvest: control-varies-the-dimension (value AND reorder; pattern
   AND selector). -> new ledger entry
   `a-control-varies-the-dimension-the-claim-is-about`.
6. Flagged by the reviewer and fixed the same turn: the ROADMAP's Renovate entry
   still said the cadence "is still open and is settled when the config is
   written" - falsified by Task 3. Now records the settled cadence and why the
   three-day window replaces the `schedule:monthly` preset.

**Task 4** (`task-4-verdict.md`, APPROVED_WITH_MINORS; the reviewer re-derived
the CLI surface from the binary itself, reproduced both counts with all three
boundary checks and the historical two-unit measurement at `62aaf61`, and
upheld all four anchor items against spec section AND core symbol):

1. MINOR: exit code 130 scoped to `run` on a measurement of EMITTING SITES,
   while the reader's observable is `$?` - a `dry-run` interrupted by SIGINT
   reports 130 too, by default signal disposition. **Routed to a fix round**
   (see the controller decision below). -> new ledger entry
   `an-evidence-line-naming-a-source-construct-cannot-carry-a-conclusion-about-a-runtime-observable`.
2. MINOR: `--on-collision`'s domain enumerated only in the `dry-run` subsection;
   a reader landing on `### muxsmith run` meets a bare `<policy>`. The
   PLACEMENT was adjudicated correct; the residual reader gap is real.
   **Routed to the same fix round** (a back-reference clause, not a second
   enumeration).
3. NIT: the 130 sentence describes only the first-SIGINT path; a second Ctrl-C
   force-exits without the cleanup the README attributes to it. Folded into the
   same fix.
4. NIT: "Two conventions that hold everywhere" - the three named flags exist on
   `dry-run`/`run` only. The reviewer deliberately did not press it (an idiom,
   not a quantifier over commands, unlike the two absolutes the task killed).
   **Controller ruling: not fixed.** Recorded here rather than dropped.
5. OBSERVATION: the plan-8 design spec carries a historical copy of the
   INSTALL.md file-top comment in its two-member form. Correctly untouched -
   a retired design document is history.
6. HARVEST 4, and it is the controller's own defect on both halves: the review
   brief named a commit as "the tree" and demanded byte-identity against it,
   and the controller then committed a ROADMAP disposition mid-review, moving
   HEAD. Nothing under review changed and the reviewer proved it at blob level.
   -> new ledger entry
   `byte-identity-is-proven-against-blobs-not-against-a-clean-git-status`.
7. HARVEST 6 and 7: the unit-claim entry gained its well-formedness half (a
   unit whose plausible re-readings converge), and the reviewer's own invented
   sha256 tail became
   `a-measurement-restated-in-shortened-form-invents-precision`.

**Task 4 fix round 1** (`845cf89`, `README.md` only, +2/-2; delta verdict
appended to `task-4-verdict.md`): findings 1, 2 and 3 all **ADDRESSED**, each
reproduced by the resumed original reviewer with its own instruments - a real
shell `$?` probe using `set -m` rather than the implementer's method, a SECOND
subcommand the fix never measured (`validate` on a 3000-rule profile: 130
interrupted, 0 in control), and a kernel-level instrument of a different kind
(`/proc/<pid>/status` giving `SigCgt=0x440`, so SIGINT is uncaught, independent
of any source search).

**FOR THE WHOLE-BRANCH REVIEW - one deferred NIT with its repair written out.**
The rewritten sentence opens "Interrupt any of them", and *them* has no
antecedent inside its paragraph: measured, the only `command` token before the
pronoun sits inside "command-line" as an adjective, and the nearest bindable
plural is "your scripts", which is a coherent but wrong reading. The referent
comes from the section opener several paragraphs up. Two-word repair:
"any subcommand". Deferred rather than routed into a second round on the
reviewer's own judgment and mine - it is a fresh finding on the fix diff, not an
unaddressed one, and the whole-branch fix wave can take it at zero marginal
cost.

**Controller decision on routing minors into a fix round.** The SDD skill keeps
Minor findings out of the loop and hands them to the whole-branch review to
triage. Findings 1, 2 and 3 were routed anyway, deliberately: they are
user-facing documentation accuracy in the one task whose entire purpose is that
accuracy, each is a one-clause repair whose content the reviewer measured, and
the implementer's context was still live so the round costs a round-trip rather
than a re-derivation. Recorded here because it is a deviation from the skill's
default, not an application of it.

**Controller process slip, recorded rather than smoothed over.** The fix round
was dispatched BEFORE the verdict's harvest was mined into the ledger; the
verdict-arrival gate puts the mining first, precisely so a harvest cannot be
overtaken by the next dispatch. Nothing was lost - the mining happened while the
fix round ran - but the order was wrong.

**Task 5** (`task-5-verdict.md`, APPROVED_WITH_MINORS, no fix round; all 21
rewrites verified against the cited code, comment-only proven twice - no
non-comment changed line, control firing at 8 on an adjacent commit, plus
comment-stripped blobs identical for all sixteen files):

1. MINOR, class-level and NOT this task's to fix: one member of the swept class
   survives in `.github/workflows/ci.yml`, outside the corpus's file selector.
   -> ROADMAP with its own vehicle; the class is NOT closed tree-wide, and any
   close statement must name the selector. **Open owner question recorded with
   it:** the ruling is scoped to source comments and its comment-form
   enumeration names `//`, `///`, `//!`, `/* */` - not `#` or `<!-- -->`.
   **-> FOR THE CLOSE REPORT.**
2. MINOR: two ragged mid-sentence re-wraps (`smoke.spec.ts`, `EditorView.vue`)
   against the report's claim that no line is left ragged; measured 2. Cosmetic.
   **-> deferred to the whole-branch review.**
3. NIT: one report row carries pre-edit line numbers unlabelled. Process
   artifact, permitted; no action.
4. HARVEST H1: the measuring expression carries TWO enumerations - what it READS
   and what it MATCHES - and only one was ever audited. Third instance on this
   same corpus, so `a-search-whose-terms-come-from-memory-produces-a-false-absence`
   **promotes to Tier 2** (agent-emergent + process at count 3), statement
   sharpened with the selector/pattern split.
5. HARVEST H2: `src-tauri/src/lib.rs`'s citation never once pointed at its
   target in a committed tree - the target moved in the same commit that wrote
   the comment. -> occurrence on
   `comments-locate-by-symbol-never-by-line-number`, which also gains the
   corrected corpus size (its prior occurrence recorded 20/13 where the swept
   union is 24/16).
6. Adjudications: all three of the implementer's judgment calls upheld, the
   recipe anchor verified by extracting the README block using the heading alone
   and comparing to the parsed Rust literal - 177 bytes each, byte-identical,
   with a fired mutation control.

## Cross-task constraints (travel verbatim in the dependent task's dispatch)

**For Task 5, from Task 4 (verified by the controller on the tree, not taken
from the report):**

1. `crates/muxsmith-cli/tests/run_live.rs` carries, at the comment above the
   `--json` assertion, `per README.md:91 "every command takes --json"`. Task 4
   corrected exactly that README claim, because it is false for `schema`. **The
   quotation is DROPPED, not re-anchored** - the Task-4 reviewer measured that
   the quoted phrase now exists nowhere in the README (`grep -c` returns 0,
   fired against a present phrase returning 1), and line 91 is a different
   bullet today. The rewritten comment names the "What you get" section's
   "Scriptable everything" bullet. This is the one Task-5 site where the
   ruling's mechanical handle (replace the number with the symbol) is not
   sufficient on its own.
2. The two `run_live.rs` comments citing `README.md:71-78` name the README's own
   anchor as Task 4 committed it, never a span. **Unit correction to my own
   earlier note here** (which took the implementer's report at face value): the
   report compared a FENCE span to a CONTENT span. `71-78` was the fence span;
   post-task the fence is 78-85 and the content is 79-84. Harmless for Task 5,
   which deletes the span outright, but wrong if quoted downstream. The recipe
   bytes are unchanged - 177 bytes, identical at both commits and to the test
   literal.
3. The corpus itself is unchanged by Tasks 1-4 (20 lines / 13 files under
   expression A, 4 / 4 under B, union 24 / 16), re-measured with fired controls
   in three consecutive reviews. Task 5 re-measures anyway - that is its Step 1.

## Controller-routed items, written at creation

- **Task 1, Step 1(f) surfacing, corrected in the ROADMAP the same turn.**
  `BUILDING.md` carries THREE positional gate ordinals, not the two the routing
  entry named: `:102`, `:134` and `:135`. The third was found by Task 1's
  implementer and hides in a known shape - it sits in the same paragraph as
  `:134`, hard-wrapped across the line break, so a by-paragraph reading sees one
  where there are two (`proc-wrapped-prose-quote-grep`). The ROADMAP entry now
  carries the measured enumeration and the command that produced it; the vehicle
  is unchanged. Ledger candidate at the close.
- **Shell hazard for Tasks 2-5:** this shell is zsh, where `${PIPESTATUS[0]}` is
  empty (bash-only; zsh spells it `$pipestatus[1]`). Surfaced by Task 1's
  implementer. Carried verbatim into every remaining task brief.

## Plan close, in progress

- **Entry condition MET.** Tasks 1-5 committed, tree clean; the full eleven-part
  gate per `BUILDING.md` run as one blocking chain with every part's exit code
  captured individually, all 0 (log: session scratchpad `ctrl-gate-close.log`);
  push `754cb73..80e5c19` done and logged in `gh-log.md`; the push-triggered CI
  run `30465581798` concluded **success** on all five jobs - the three OS legs,
  `deny`, and `ledger-lint`, which now also carries Task 1's new check.
- **Whole-branch review dispatched on the top tier** (`proc-03-model-assignment`,
  its only role), against the plan, the plan brief, the ROADMAP entries the
  brief cites and the spec, with the two deferred findings handed to it for an
  explicit ruling.
- Still to do after it returns: verdict harvest, blocked-pool sweep, ROADMAP
  dispositions, SDD salvage with its `diff -r` re-check and the file count
  verified IN THE COMMIT, journal entry, HANDOFF snapshot.

## Controller notes

- Strictly serial, one tree, no worktrees (plan ruling). No second writer is
  dispatched while a task is live (`a-serial-ruling-binds-dispatch-concurrency-too`).
- Task reviewers and implementers: mid tier (Opus 5). Whole-branch review at the
  close: top tier, the only role it serves.
- One push, at the close, after the full gate as `BUILDING.md` enumerates it.
