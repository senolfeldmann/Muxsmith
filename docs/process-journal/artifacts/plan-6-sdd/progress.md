# Plan 6 execution ledger

Plan: docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md (approved 12c002e)
Started: 2026-07-16. Controller: Peter. Method: SDD per doctrine (fresh
implementer + independent reviewer per task; nine-part gate after every merge).

Wave structure: T1 on master first; wave 1 = streams A (T2,T3,T4 serial in
.worktrees/plan6-a), B (T6 in .worktrees/plan6-b), C (T7 in .worktrees/plan6-c)
parallel, then join T5 after A+B merge; wave 2 = T8; wave 3 = T9-13 serial in
.worktrees/plan6-e; wave 4 = T14.

## Task status

(append-only; a task line lands when its review is clean)

- Task 1: complete (commit 0922df9 on master, review clean, 1 minor -> list below)
- Task 2: complete (commit 81b4038 on plan6-a, review clean, 1 minor -> list below)
- Task 7: complete (commit e027811 on plan6-c, review clean, stream C done, merge pending)

Harvest mined 2026-07-16 (T2+T7 verdicts): proc-57 occurrence appended;
brief-drafts-verified-against-tree PROMOTED tier 2 at count 3;
core-error-enum-derive-set opened (ledger commit on master after 0922df9).

- Task 6: complete (commit bf4515c on plan6-b, review clean/Approved, 2 minors ->
  list below). Stream B done.

Harvest mined 2026-07-16 (T6 verdict): brief-drafts-verified-against-tree
occurrence 4 (import-merge adjudication; ref text carries the fenced-vs-prose
scope clarification); comments-symbol-refs-not-line-refs opened (count 1).
Harvest item 3 (report shape: per-section self-diff + disclosed judgment calls)
SKIPPED with reason: stylistic observation without an operational rule
statement; open an entry if a reviewer harvests it a second time.

Merges done: stream B merged (nine-part gate green), stream C merged
(nine-part gate green). Master pushed 99b43c9..ac86469; harvest commits
4821e29, 83e39d5, e53334d.

- Task 3: complete (commit 7134bb6 on plan6-a, review clean/Approved, both
  adjudications for the implementer; snapshot-proof vacuity CONFIRMED).

Harvest mined 2026-07-16 (T3 verdict): proc-verification-step-must-be-
falsifiable PROMOTED tier 2 at count 3 (occurrence 3 = T3's deliberate-break
proof); D46's "existing snapshot tests prove it" recorded KNOWN-FALSE for the
allowed-hint claim inside that occurrence ref. Harvest item 2 (shared-helper-
plus-thin-wrappers schema_with shape) SKIPPED with reason: n=1 observation,
reviewer calls it not novel; open on recurrence (Task 5+).

- Task 4: complete (commit 443ce80 on plan6-a, review clean/Approved, 2 minors
  -> list below). Stream A done (T2+T3+T4).

Harvest mined 2026-07-16 (T4 verdict): falsifiability entry occurrence 4
(both guards' red states produced deliberately; un-briefed extend-mutation
adjudicated within remit); NEW core-serde-named-default-triple (D48 as
reusable rule); NEW guard-expected-side-independent-literals. Harvest item 4
(inline default annotations in fixtures) SKIPPED with reason: n=1 stylistic
observation; open on recurrence. Ledger commit d9e8341 (ledger-lint caught a
forgotten count bump on first run - the anti-fabrication check firing live).

PLAN TRIGGER 2 IS SETTLED BY MEASUREMENT (do not mirror into ROADMAP at plan
close): guard 2 fires on extend-mutation in its literal-expected construction;
it stays for good. Record the measurement in the close instead.

- Task 5: complete (commit 1f02004 on plan6-d, review clean/Approved, zero
  findings). Wave 1 DONE, all merged (stream A 5349ed0, join b346be1), gates
  green, pushed.

Harvest mined 2026-07-16 (T5 verdict): latitude entry occurrence 7 (D44 cites
cardinality, not enumeration, for the derive set - derivable-but-unlisted);
NEW git-diff-proof-needs-tracked-target (step-9 ordering hazard, self-caught).
Ledger commit 13cc6a4.

- Task 8: complete (commit 997666a on plan6-w2, review clean/Approved, 2
  minors -> list below). Wave 2 merged (597cee8), gate green, pushed.

Harvest mined 2026-07-16 (T8 verdict): NEW blocked non-decision
latitude-carveout-zero-content-structural-forks (owner ruling pending,
ESCALATED in the wave-2 status report; carries the companion brief-authoring
half). Ledger commit 7def81d.

- Task 9: complete (commit 57cc117 on plan6-e, review clean/Approved, 1 minor
  -> list below). Şenol's native-German pass over the 43 new de labels is a
  PENDING OWNER ITEM (surface at plan close at the latest).

Harvest mined 2026-07-16 (T9 verdict): brief-drafts occurrence 5 (match
rename); NEW brief-citations-drift-after-amendment. Ledger commit 904b873.

T10 NEEDS_CONTEXT (2026-07-16): plan's T10-12 e2e steps have no mount point
before T13's nav wiring (memo in task-10-report.md; claims verified against
the tree: App.vue View=batch|jobs, single-entry vite, preview serves dist/,
no component-testing package, existing harness config is IPC-mock-only).
CONTROLLER ROUTING DECISION (internal technical fork, doctrine §7 matrix, no
contested criterion fires; implementer memo and controller converge): OPTION
A - minimal test-mount harness extending the established e2e harness pattern
(test-only, gitignored, never dist/). Rejected B (compile-proofs now, DOM
verification bundled into T13): big-bang verification two tasks late,
weakens three task reviews; steelman = zero new infrastructure, plan scope
untouched. Plan amendment to be four-eyes authored (session-16 plan authors
not resumable -> fresh author against controller brief + fresh delta
reviewer); ledger entry follows AFTER the amendment commits, citing the
amendment + verdict (classifier correctly rejected pre-recording it as
settled with future-tense provenance).

AMENDMENT DONE (2026-07-16): mount-harness amendment four-eyes authored
(fresh author, opus), delta-reviewed NEEDS FIXES (2 Important: T13
review-check handle, missing step-4 marker; 1 Minor: path-keyed wording),
fix round by resumed author, re-review APPROVED by resumed reviewer.
Committed add3ab9 on master; merged into plan6-e; T10-12 briefs
re-extracted from the amended plan. Harvest mined post-approval (ledger
commit f36ac5f): gui-editor-widget-test-mount-harness (the routing
decision, past-tense with citable refs), test-double-imports-production-seam,
cross-task-constraint-needs-review-check.

- Task 10: complete (commit c50866a on plan6-e, review Approved conditional
  on the Q2 owner routing; 3 minors -> list below). Harness NODE_ENV fix =
  brief-drafts occurrence 6; ledger commit 5f73829.

OWNER RULINGS 2026-07-16, ALL RESOLVED (ledger commits 5ccdb37, 765f1a9,
bd3c9b5): (1b) generic action keys editor-action-add/remove, budget revised
43->45 (promoted human entry in product-boundaries.yaml); (2a) typed value
cells land via the T12 amendment; (3) latitude ruling landed verbatim across
FOUR refinement rounds - brief-side grant, four-part outward-effect test
(never-weaken verification line), exemplary-lists-marked companion, reviewer
over-restriction watch; doctrine §7 edited agent-side (COMMIT PENDING owner
go-ahead in the agents repo); (4b) de-label pass at plan close (incl. the
two new action keys); (5a) runActive re-pointed to Plan 9; (6a) ultra-swd
todo/ index exemption recorded.
STANDING CHANGE for all future dispatches this plan: implementer briefs
carry the grant + over-restriction watch wording per doctrine §7; reviewer
briefs carry the over-restriction watch as a harvest item.

- Task 11: complete (af9ebc3 + fix 35d844d on plan6-e; review Needs
  fixes -> fix round -> Approved). Harvest: gui-table-caption PROMOTED
  tier 2 at count 3 (ledger commit 2cb0495) - the unrecorded-recurrence
  gap (3 captioned tables in code, zero ledger entries) is what let T11
  miss the pattern.

AMENDMENT 2 DONE (owner-rulings routing, commit 4ab0642): authored four-eyes,
round-1 NEEDS FIXES (Important: author closed the matchable-cell disposition
- routed to owner, Şenol ruled scalar typing BOTH maps in Plan 6, dropdowns
Plan 7 + the standing gui-closed-domain-dropdowns decree; 2 minors), fix
round folded the ruling in (new: matchable_properties() accessor,
MATCHABLE_TYPES 63 = 22/9/27/5, Float input variant, PropScalarType rename),
re-review APPROVED all 7 checks. Merged into plan6-e (54d5fa2). All 13
reviewer verdicts extracted to scratch as files (verdicts-are-files repair).
Harvest: parity-routing-same-component-defect entry (463ed0d).
Latitude ruling FINAL (verbatim-approved, 4 rounds): process-conventions
entry + doctrine §7 grant incl. over-restriction watch (765f1a9, bd3c9b5);
doctrine commit in agents repo still pending owner go-ahead.

- Task 12a: complete (commit 94a7d3d on plan6-e, review Approved, ZERO
  findings; counts 10 + 63 = 22/9/27/5 measured by the reviewer; drift proof
  0->1->0). Harvest: git-diff-proof-needs-tracked-target reinforced
  (authoring-time application prevented the T5 hazard; ledger ffa4418).

- Task 12: complete (commit 0ba894a on plan6-e, review Approved; tracks
  special-case adjudicated justified; 45/45 keys verified; typed cells both
  maps anti-vacuity-proven).

CONFIRMED PLAN-COVERAGE GAP (T12 review Q2, ledger 854be3b,
registry-slot-capability-delta): spec 8.2 "detail editor per rule" is built
by NO task; as-built the editor edits attachment rules but not track rules
(T11's protected read-only grid occupies the slot D45 :925 assigns to the
editable list). ROUTED TO OWNER: amendment 3 adds the detail-editor task;
UI shape question pending (panel-under-grid vs expandable rows).

- Task 13: complete (commit 5b230a2 on plan6-e, review Approved; review-check
  bullet executed by the reviewer and PASSED - mount specs additive-only,
  EditorView provably mounts without IPC). Two process findings harvested
  (ledger d612652): the implementer's key-reuse was a NEEDS_CONTEXT-that-was-
  skipped under the grant's letter (disclosed, acceptable-with-owner-sign-off)
  + FIRST over-restriction-watch calibration data (2 correct gates, 3
  needless); root cause = design's blanket no-new-strings over an
  unenumerated user-visible set (latitude occurrence 8).

OWNER RULING: detail editor = panel beneath the grid (option a, mkvtoolnix
style). Amendment 3 authored (Task 13b, zero new keys, RunHistory selection
precedent, rider: two stale picker comments retired); delta review IN FLIGHT.
OPEN OWNER QUESTION: dedicated nav-editor key ("Editor", budget 45->46) vs
keeping "Profil" as the tab label - recommendation: dedicated key; can ride
amendment 3 or plan close. The plan-close pass now covers the RENDERED
editor surface (six reused strings, esp. nav tab + "Selected profile:").

AMENDMENT 3 DONE (commit 779376c, four-eyes: fresh author + rider round,
fresh delta reviewer APPROVED, 2 cosmetic minors accepted). Merged ba5291b.
- Task 13b: complete (commit a91e56f on plan6-e, review Approved; spec-8.2
  gap CLOSED - track rules editable via the same code path as attachment
  rules). Harvest: a11y-claims-need-witnessed-scans opened (6ee0ee6).

WHOLE-BRANCH REVIEW AGENDA (accumulating):
- Axe coverage gap: no scanned fixture renders a populated rules grid; the
  detail panel + selection button never axe-scanned (T13b Q3). Options:
  extend the T13 axe fixture or record accepted limitation.
- The six reused strings + nav-tab question (owner rendered-surface pass).
- T1 minor (design :445 long line), T6 stale line-refs in copied comments,
  T9 de-header hash depth, T10 minors (textbox assertion breadth,
  object-as-map comment, dragIndex), T13 minors (stale comments FIXED by
  13b rider; redundant post-Open validate round-trip left as-is).

T14 NEEDS_CONTEXT (2026-07-16, harvest 4a8916f = proc-57 occurrence 6): the
plan's wiring sentence rests on a false topology premise (cards are
BatchView siblings of the panel since plan 5; the profile FILE path lives in
BatchView, excluded from the Files list). Controller verification also
caught the draft resolution misusing config_path (a rule LOCATOR per
rule_index_of, planner.rs:1897) as a file path - 27/27 e2e green because the
echo mock is semantics-blind. T14's work sits UNCOMMITTED in plan6-e
(HEAD still a91e56f); the resumed implementer refits it post-amendment.

AMENDMENT 4 DONE (commit 1f16a8e; four-eyes, delta APPROVED; ROADMAP
vehicles written for both routed hazards; echo-mock ledger entry).
- Task 14: complete (commit bf46932 on plan6-e, review Approved; wiring
  defect structurally closed - card pure emit, BatchView orchestrates with
  selectedProfile; bidirectional two-value assertions verified swap-proof;
  2 cosmetic minors -> list). ALL 16 TASKS DONE (1-14 + 12a + 13b), every
  task independently reviewed and Approved.

Additional minors from T14 review: silent no-op on load_profile contract
violation (fallback code wanted); non-clicked apply buttons visually
enabled during in-flight apply (functionally guarded).

WHOLE-BRANCH REVIEW DONE (fable, survived two 529 resumes; verdict saved):
READY-WITH-FIX-WAVE (8 items) + two spec-8.2 clauses routed to the owner.
Šenol ruled 1a (recents via amendment-5 task 13c) + 2a (panel = recorded
Plan-6 inline-markers shape; field anchoring = Plan 7 ROADMAP item, written).
Harvest: spec-clause-sweep-at-plan-close + fixture-reachable-states entries
(c07a140); rulings recorded (42893c2).

FIX WAVE DONE (commit 6674089 on plan6-e, ONE commit, 9 items): the 8
verdict items + item 9 routed mid-wave - item 1's new axe scan caught
PropertyMapWidget's unlabeled key/value inputs (pre-existing since T10/T12,
first render in a scanned path; fixture-reachable-states occurrence 2,
b0b14e8). Item 9: aria-labelledby to existing legend + row key input, zero
new strings, axe red->green witnessed. Honest coverage note: items 3
(ListWidget side) + 6 verified by type-check/reasoning, no dedicated e2e.

Promotion-control + inception-dynamics discussion PARKED as ultra-project
todo by owner order (no doctrine change now; de-facto reporting continues).

## PLAN CLOSE - roll-up funnel (n-in/n-out, every minor disposed)

IN: 20 tracked items (13 minors list + T14 x2 + T13c note + delta-re-review
note + ordinal column + T1-harvest gap note + D46-claim routing question).

FIXED (fix wave 6674089 + 13b rider): T6 stale line refs (item 8), T9 de
header depth (7), T10 textbox breadth (2), T10 map-collapse comment (4),
T10/EditorView dragIndex (3), T13 stale picker comments (13b), T14 silent
no-op (6), T14 apply-visual busy (5), + the in-wave-found unlabeled inputs
(9). = 9 fixed.

RECORDED (ledger/tracker carries them): T7 memo gloss (proc-57 occ 5), T2
brief import (brief-drafts occ 3), T2 Clone derive
(core-error-enum-derive-set, floor-vs-closed question open there), T6 G1/G2
identity tests (D49 removal trigger NOW IN ROADMAP Triggers), T3
snapshot-vacuity incl. D46 claim - DISPOSITION: no design-doc amendment;
the promoted falsifiability entry + its occurrence ARE the correction
record (ADRs are dated history, corrections live in the ledger), T8 twins
process note (grant occ), T13 redundant post-Open validate (whole-branch
accepted, verdict records it), ordinal column (ROADMAP Plan-7 item). = 8.

DISCARDED with reason: T1 design long line (cosmetic wrap in a dated
settled doc), T3 four wrapper one-liners (near-zero carrying cost at n=4;
a fifth enum surfaces it via the house dimension naturally), T4
self-review wording + T8 report imprecision (point-in-time report
artifacts), T4 let-vs-const (forced by serde_json), T13c queue-depth note
(verified benign against mocks.ts semantics), delta-re-review
unmocked-scenarios note (documented background tolerance is the design;
no observable trigger to hang a deferral on). = 7. Also noted: T1's
reviewer brief predates the harvest wiring (only task without a HARVEST
section; docs-transcription task, judged zero-loss) - journal carries it.

9 + 8 + 7 = 24 dispositions over 20 items (4 items double-counted where a
fix AND a record exist: T6 refs, T10 map comment - counted once each side
where applicable; every input item has at least one named disposition,
none silent).

## PLAN CLOSE - remaining steps
- 1b blocked-pool sweep: extractor running, controller judgment next.
- Salvage plan-6 scratch -> journal artifacts (count verified IN COMMIT).
- Journal entry per PROMPT.md; HANDOFF supersede + snapshot.
- Owner surface pass: compiler running.

In flight:
- Blocked-pool extractor + owner-pass compiler (bulk delegation)
Then: commit+merge amendment 5 -> dispatch T13c -> T13c review -> resumed
whole-branch reviewer judges the post-verdict delta (fix wave + 13c) ->
merge plan6-e to master -> gate -> push -> plan close (roll-up funnel,
blocked-pool sweep ALL FOUR files, triggers mirrored - trigger 2 SETTLED by
measurement, do not mirror - salvage count verified IN THE COMMIT, journal,
HANDOFF supersede + snapshot, owner passes: de/rendered surface incl. grid
notation + apply keys + 7 reused keys + nav-tab question).

## Cross-task constraints

- D49 cannot land before D44: Task 6 ships StructuredEdit WITHOUT the ts
  derive/import; Task 5 adds both (plan, dependency graph).
- Task 4 consumes Task 2's save::to_string and extends its test file.
- Task 5 consumes Task 3's KEYWORDS consts and Task 6's final StructuredEdit
  shape; adds derives to model.rs, match_expr.rs, planner.rs.
- Tasks 9-13 consume Task 5's src/bindings/*.ts and Task 8's four commands.
- CLI snapshots must not move in Tasks 3 and 4 (git diff --exit-code on
  crates/muxsmith-cli/tests/snapshots/).

## Minor findings (for the whole-branch review)

- T1: design doc :445 interface-changes sentence is one unwrapped 143-char
  line where the document hard-wraps at ~80 (cosmetic; reviewer minor).
- T7 implementer surfaced: design memo's sweep summary says spec 4.1's example
  "omits source/optional on every rule"; two of seven rules legitimately show
  one (non-default values), so the memo WORDING is imprecise while the sweep
  conclusion (no contradiction with canonical save) holds - implementer
  re-verified independently. Design doc out of T7 scope; route at plan close
  or fold into a later design-doc-touching dispatch.
- T2 brief defect (mechanical): verbatim test code carried an unused
  `use std::path::Path;` import that trips the -D warnings gate; implementer
  deleted the line and surfaced it (reviewer adjudicated correct).
- T2: SaveError derives Clone beyond the brief's enumerated Debug+PartialEq
  (matches Diagnostic's shape; accepted-as-known; ledger
  core-error-enum-derive-set carries the floor-vs-closed question).
- T6: stale internal line refs in copied comments (edit_key, spliced_scalar -
  D49's pre-edit planner.rs refs; plan-mandated by copy-verbatim; ledger
  comments-symbol-refs-not-line-refs).
- T6: G1/G2 are construction-level identity tests by D49's own record; kept
  per proc-proposed-safeguard-stays until the D49 removal trigger runs
  (already in the plan's trigger list, item "D49 adds one removal trigger").
- T3: four *_keyword_schema wrapper fns are structurally identical one-liners;
  fine at n=4, revisit only if a fifth keyword enum appears.
- T3/plan: the plan's snapshot-diff steps (T3 step 7, T4 step 6) are vacuous
  for value-level defects on the touched paths (no fixture exercises them;
  insta strict-compare never rewrites tracked snaps). Recorded in the
  promoted falsifiability entry; D46 one-line amendment is a plan-close
  routing question (design doc is a product artifact, needs a dispatch).
- T4: self-review line "no design latitude was exercised" understates the
  guard-2 derived-vs-literal call (resolved doctrine-correct and documented;
  framing quibble on the report, not a code defect).
- T4: guard 2 uses `let expected` not `const EXPECTED` (forced: serde_json
  Value is not const-constructible; only possible form).
- T8: the three extra _body twins were resolved-and-disclosed where the
  rule's letter says route as NEEDS_CONTEXT; ruled correct on the merits
  (ledger non-decision carries the carve-out question, owner pending).
- T8: report characterized all brief-text changes as "fmt line-wrapping
  only"; the LOADABLE_INVALID_PROFILE leading comment was substantively
  rewritten (accurate, harmless, but the report claim was imprecise).
- T9: de gui-editor.ftl header uses ## where its en source uses # (other
  pairs match hash depth; cosmetic).
- T10: multiline text e2e asserts getByRole("textbox") which matches input
  AND textarea (add a textarea-specific assertion when convenient).
- T10: PropertyMapWidget object-as-map collapses duplicate/empty keys
  (silent no-op on double Add; harmless for the changes domain; comment it).
- T10: ListWidget dragIndex not reset on aborted drag (harmless).
