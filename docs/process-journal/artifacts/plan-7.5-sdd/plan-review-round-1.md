# Plan 7.5 plan review, round 1

Artifact: `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md`
(commit cfe10a4, 4 tasks). Ground truth walked independently: the design
(D65-D72, sections 0-9), the plan brief, the plan-7 house template, the
Tier-2 entries, and the actual tree at cfe10a4. Every load-bearing claim
below was re-verified at the tree, not believed from the plan.

## Verdict: NEEDS FIXES

One major (a step whose literal execution produces a failing assertion,
forcing keyboard invention), two minors. Everything else across the nine
dimensions verified clean - notably all 13 Tier-2 line refs exact, all
four transcriptions exact, all counts recomputed and confirmed.

---

## Findings

### M1 (major) - Task 1 Step 3, case 6(d): "lists the rendered text" asserts a mechanism the tree contradicts

Location: Task 1, Step 3, case 6, assertion (d), plus the step preamble
sentence "Diagnostics carry `rendered` strings as in the sibling
fixtures".

The panel never displays the `rendered` field. Verified:

- `src/components/DiagnosticsPanel.vue` renders each line as
  `$t("batch-diagnostic-line", { severity: $t(severity-*), message:
  $t(d.code, diagnosticFluentParams(...)) })` - Fluent over `code`/
  `params`, `d.rendered` unread.
- `src/ipc.ts`'s own `Diagnostic` doc says it explicitly: "the frontend
  renders `code`/`params` through its own Fluent bundle instead of
  trusting this field".
- `locales/en/diagnostics.ftl:9`: `empty-match-expression = This match
  expression is empty and would match every track.` - contains neither
  the mock's `rendered` value (`"empty-match-expression"`) nor the code
  string.
- No existing spec asserts on `rendered` anywhere (grep over `e2e/` and
  `src/`): the field is fixture ballast required by the type, nothing
  more. The sibling (`editor-markers.spec.ts`) asserts panel COUNT only,
  so it supplies no text-assertion pattern to fall back on.

Consequence: an implementer executing 6(d) literally writes
`toContainText("empty-match-expression")` (the step's preamble binds
"rendered text" to the fixture field, whose value is the code string) and
gets a red test against correct code - then must invent the right
assertion at the keyboard, the exact fork class the plan's own
every-fork-closed constraint bans. The design's case 6 phrase "lists the
code" carries the same imprecision, but the plan step is the executable
instruction and concretized it in the wrong direction.

Fix (one clause): reword 6(d) to assert via the house i18n-en convention,
e.g.: the diagnostics-panel `li` set contains a line whose text includes
`en("empty-match-expression")` (the Fluent-rendered en value; the message
has no placeables, so no args are needed). Optionally drop or reword the
preamble's "rendered strings" sentence to "the `rendered` field is
type-required fixture data the frontend never displays" so the next
reader is not steered at the field again.

### m2 (minor) - Task 1: Step 5's comment permission is not in Step 6's permitted-delta enumeration

Step 5 permits "A short comment in the file's register may note the D66
clearing rationale, mirroring the existing `onDrop` comment" - and the
`onDrop` precedent sits INSIDE the function body
(`EditorView.vue:429-432`). Step 6 then diffs the extracted functions
against D67's blocks "expecting semantic identity (the two permitted
deltas: template indentation, guard-brace formatting)". A comment placed
per Step 5 surfaces in Step 6's diff as an unenumerated third delta: the
two steps contradict each other on whether the diff may show it.

Fix: either scope Step 5's permission to a placement outside the
extracted blocks (a comment line above the function), or add "comment
lines per Step 5" to Step 6's permitted-delta enumeration. Either way the
enumeration and the permission must agree.

### m3 (minor) - coverage map, Section 7 row overstates its carrier

The row claims section 7 is carried by "the zero-new-surface global
constraint (binding negative scope)". That constraint covers section 7's
bullets on undo/redo, insert-at-position/up-down/multi-select, and
core/validate changes - but not the other three: attachment-rule
add/remove untouched, the spec-8.2 grid-column-order cosmetic, and the
plan-7 non-goals (tooltip-disable setting, help search, F1). Actual
coverage exists elsewhere: attachment rules are excluded by every task's
Files list; the column-order cosmetic - the one live temptation, since
Task 4 edits the very sentence carrying the stale column order - is
guarded by the verbatim transcription plus Step 5's unabbreviated-
transcription check (the design's amendment text deliberately preserves
"order, source, match summary, changes, optional"); the plan-7 non-goals
have no surface any task touches. No invention license results; the map
row is just inaccurate about the mechanism.

Fix: amend the Section 7 map row to name the real carriers ("the
zero-new-surface constraint + the tasks' Files lists + Task 4's verbatim
transcription"), or fold the three residual bullets into the constraint.

### Observations (no fix owed)

- The review brief's dimension 2 cites "Design section 9's
  implementer-must-not-decide list"; in the design that list is section
  8 (section 9 is "Open items: none"). Graded against the actual design;
  the plan cites section 8 correctly throughout. The brief template
  should be corrected for future rounds.
- D70's keyboard-reachability claim is exercised by Task 2's case 9
  (Enter activation outside help mode), which the map's D70 row does not
  name (it names Task 1 only). The decision itself is fully implemented
  by Task 1's block; sub-minor map nuance, fold into the m3 edit if
  convenient.

---

## Dimension record

**1. COVERAGE (independent walk).** Walked D65-D72 and sections 0-9
element by element against the tasks. Every D-entry has implementing
task(s); every "no implementation surface" verdict in the map checked
true (section 0: consumed corrections; section 2: rationale folded into
D66/D67/D70/D72, nothing buildable; section 9: empty). Section 1's
execution-tree re-verifications exist (T1 Steps 1, 8). Section 3's
inventory = the four Files lists exactly; the confirmed-pure-frontend
wire finding correctly yields no task. Section 4 -> Task 4; section 5's
nine cases -> Tasks 1-2 with the untouched-existing-specs invariant
carried and its gate-ripple items each landed (D62 bans -> T3 Step 3;
46/46 -> T1 Step 10; h1 -> T3 Step 3; bilingual -> T3 Step 5;
no-raw-text -> T1 Step 10; catalog_completeness unaffected). Section 6:
all six triggers routed at plan close (1 consumed, 2 with declined-
fallback, 3-6 mirrored). Section 8 -> global constraint 2 + per-task
restatements. Map graded against the walk: accurate except m3 and the
D70 observation. No design element lacks an implementing task.

**2. Latitude, both forms.** Per-task scan: fixtures, claim sets, disable
conditions, permitted deltas, and scope fences are enumerated; the
one-row scoping on amendment 2 ("adding any second row is a defect")
transmits the design's own fence. Task 3's authored-content latitude is
exactly the standing D54 rendered-surface carve-out, named in the plan
and the design. Section 8's list is transmitted undiluted (constraint +
task-local restatements: T2's no-button-side-condition, T3's
claims-only + h1 fence, T4's one-row fence). The only permission/
enumeration mismatch is m2. M1 is an involuntary latitude: a wrong
mechanics claim that forces invention.

**3. Template conformance.** Header note incl. house deviation, tracker
`.superpowers/sdd/plan-7.5/progress.md`, Goal/Architecture/Tech Stack,
Global Constraints (all brief-mandated members present incl. the
session-relocation ban and the seven minimum Tier-2 entries plus six
more), binding execution-method section (SDD, fresh implementer +
independent reviewer per task, whole-branch review), model-tier table
(brief-mandated addition over the plan-7 template), "How this plan cites
the design", dependency graph + coverage map, plan-close
pre-registrations: rendered-surface pass (complete set stated) and the
salvage re-pointing (verified registered, `docs/ROADMAP.md:295-298`).
Conforms.

**4. Transcription fidelity (re-diffed, not believed).** All four blocks
extracted from both files and diffed (`diff -wB`, amendments normalized
for blockquote wrap): D67's two functions - match; D70's template block -
match; D71's claim enumeration - match; amendment 1 (old fragment and
replacement) and amendment 2 - match, including the "- invalid until
filled, announced by validation -" parenthetical and the full
suppression clause. The T21 truncation class did not recur.

**5. Dependency graph vs reality.** Stream A files
(`src/views/EditorView.vue`, `e2e/editor-rule-add-remove.spec.ts`,
`e2e/help-mode.spec.ts`) and stream B files
(`help/{en,de}/editor-tracks-rules.md`) are disjoint; nothing else
touches either set. Edge 1->2 real (case 9 clicks Task 1's button, same
worktree, serial). Edge 1->4 real (amendment 1 asserts the affordance);
amendment 2's semantics verified already in the tree
(`validate.rs:79-91`, `EmptyMatchExpression` + `empty_list_here`).
Task 3's independence verified structurally: the D54 annotation spec and
the topic loader derive expected HTML from the same files at runtime
(`help-mode.spec.ts:35-41,211`; `help-topics.spec.ts` reads
`view-batch` only), so content edits cannot desync any spec. Merge order
A -> B -> Task 4 with full gates is collision-free.

**6. Citations and counts (every numeral walked).** All 13 Tier-2
`:line` refs exact (grep-confirmed each id at its claimed line).
`EditorView.vue` `</table>`/`</fieldset>` at :598/:599, exactly one
`</table>` (Step 1's "exactly one hit" holds). Catalog ids at en
:138-139 / de :142-143, no `.tooltip` on either, 46/46 ids recounted
with the id-line regex. `help-mode.spec.ts` describes at :95/:186/:395;
I1 sibling title present at :396. Topic sections en :9/:17, de :9/:17.
Spec anchors :374 (single line, exact substring present), :283
(`EmptyMatchList` row), :141 (positive control); `EmptyMatchExpression`
absent from the spec (grep, exit 1). The 17-of-47 claim recomputed
independently: 47 `diag_codes!` members, 30 distinct codes with 5.2
rows, `comm` difference = 17, `EmptyMatchExpression` among them. The
9 = 5+3+1 recount, the 4/2/2/2 inventory, and the row-cell layout
(cells 0-4) all verified. `SOURCE_KEYWORDS[0]` = "primary" fallback
verified for the skeleton row.

**7. Model tiers (proc-03, read at :58).** Table conforms to the
owner-bound instantiation: mid for judgment implementers and all task
reviewers, cheap only for Task 4, top tier only for whole-branch review
and controller loop. The cheap-tier gate checked as the brief demands:
the plan carries both amendment texts completely; the steps are
quoted-text anchor greps with stated expected outputs, an exact
substring replacement with the unwrap instruction stated, a verbatim
row insert, and mechanical checks - zero judgment required (M1 does not
touch Task 4).

**8. e2e mapping.** Nine cases -> enumerated steps: 1-5 (T1 Step 2), 6-8
(T1 Step 3), 9 (T2 Step 2); each case's assertions checked against
design section 5 - consistent, with the plan adding only concretion
(fixture values, selector strings). Zero-new-help-ids/zero-new-topics
intact: no step creates a help-id, `data-help-id`, or topic file; D70's
block carries none; T3 is body-only with the h1 guard fire-verified.

**9. Implementability walk.** Every step read as the fresh implementer:
anchors exist at the tree (`onDragEnd` closes the script region; testids
`editor-open`/`editor-save`/`nav-editor`/`help-toggle` present; the
mount harness has no `currentPath` so the bare mount never fires IPC -
verified in `EditorView.vue:201-218`); the mock-queue claim
("consumed per call, last repeats") verified in `mocks.ts:89-95`;
recorded invokes carry `{ profile }` args (`ipc.ts:284-286`); the
two-entry validate queue is correct (open fires one watch invocation,
the Add/Remove swap the second; selection clicks do not mutate the
model); the `tracks[1].match` panel marker lands (nested `SectionWidget`
anchors `data-diag-path` at its legend, `SectionWidget.vue:69-72`, with
the `editor-markers` DETAIL_MARKERS precedent); save gating semantics
verified (`saveDisabled`, error-only). All runs foreground; gates named
per task; fire-verifications carried on every absence check with
positive controls; NEEDS_CONTEXT routing stated (Step 1's stop, global
constraint 2). `pnpm test:e2e -- --grep`/file-filter passthrough works
(`package.json:13`, plain `playwright test`); the red run compiles (the
new spec references only DOM testids, so `tsc -p e2e` passes before
implementation). Only M1 breaks literal executability.

---

## HARVEST

- **The `rendered`-field trap generalizes.** Every e2e `Diagnostic`
  fixture must carry `rendered` (type-required), yet no frontend surface
  displays it - `ipc.ts` documents this, but the trap just produced M1
  in a carefully authored plan. Worth a one-line test-convention note
  (conventions.yaml or the e2e doc header): "diagnostic text assertions
  go through `i18n-en`'s `en(code)`; `rendered` is wire ballast the
  frontend never renders." Cheap, prevents the whole class in every
  future e2e-writing plan.
- **Permitted-delta enumerations must absorb sibling-step permissions**
  (m2's class): when one step grants a permission, every later step that
  enumerates acceptable deltas over the same artifact must list it. This
  is the house dependency-sweep rule (an enumeration is a dependency)
  applied at plan-step scale - a candidate example for
  proc-latitude-clause-boundary's occurrence log rather than a new
  entry.
- **The `comm`-based diag_codes-vs-spec-table recount** (extract table
  codes, extract macro members, `comm -23`) is cheap and reusable for
  the standing spec-5.2 staleness watch item from the design review.
- **Positive signal worth recording:** the plan-7 review's repeated
  defect classes (stale counts, transcription truncation) did not recur
  - 13/13 line refs exact, 4/4 transcriptions exact, every recount
  confirmed. The measured-at-authoring + re-verify-at-execution-tree
  discipline held.
- **Brief nit for the template:** plan-review briefs should cite the
  implementer-must-not-decide list as design section 8 (this design's
  numbering), or by name rather than number - the round-1 brief said
  section 9.

---

# Delta review (round 2)

Scope: commit `1bb1e25` (6 insertions / 6 deletions on the plan), judged
per finding with the load-bearing claims re-run at the tree. No settled
non-finding re-litigated.

## Verdict: APPROVED

## Per-finding disposition

**M1 - RESOLVED.** Case 6(d) now asserts the diagnostics-panel line via
`en("empty-match-expression")` and names the mechanism correctly.
Re-verified: `locales/en/diagnostics.ftl:9` is placeable-free (0 braces,
0 `$`); the file-wide positive control fires (48 lines carry a
`$`-placeable, 66 `{ $` tokens), so the absence reading is fire-verified,
and the plan itself carries no control number that could go stale. The
cited ledger entry `e2e-diagnostic-rendered-is-wire-ballast` exists at
exactly `docs/decision-ledger.yaml:4110` (Tier 1, settled, occurrence =
this review's M1), and its statement matches what the step cites
(DiagnosticsPanel renders Fluent over `code`/`params`; `ipc.ts` documents
the non-display; assertions via `en(code)`). The Step 3 preamble no
longer steers at the field ("wire ballast no frontend surface
displays"). Sweep: every remaining `rendered` occurrence in the plan is
a fixture literal (cases 6-8 response objects, type-required) or the
unrelated "rendered-surface pass" - no assertion site references the
field.

**m2 - RESOLVED.** Both steps walked. Step 5 now scopes the reformat
permission to "inside the function bodies" and places the optional D66
comment "ABOVE `removeSelectedRule` and outside both function bodies";
Step 6's enumeration explicitly accounts for it ("sits above the
functions, outside the extracted blocks, and therefore never appears in
this diff"). Permission and enumeration agree in substance; no
unenumerated delta remains. Observation, no fix owed: Step 6 says "above
the functions" where Step 5 says "above `removeSelectedRule`" (i.e.
between the two) - under the natural per-function extraction the comment
is outside both extracted definitions either way, and both steps name
the comment as accounted-for, so no fork with a defect consequence
exists.

**m3 - RESOLVED.** Section-7 map row re-pointed to three carriers;
recounted the section's bullets myself against the design: six bullets -
(1) undo/redo, (2) insert-at-position/up-down/multi-select, (3)
attachment-rule add/remove, (4) the spec-8.2 grid-column-order cosmetic,
(5) tooltip-disable/help search/F1, (6) any core/validate change. The
row's grouping covers all six exactly once: constraint -> {1, 2, 6};
Files lists -> {3, 5}; Task 4's verbatim transcription + Step 5 check ->
{4}. Complete. The round-1 D70 observation was also addressed (the map
row now names Task 2's case-9 Enter activation).

## Transcription spot-check

Both fenced blocks re-diffed post-edit: D67's two functions match the
design with zero deltas (`diff -wB`); D70's template block is byte-exact
fence-to-fence (15/15 lines). The commit touched only prose outside the
fences.

## New findings

None.

## HARVEST additions

- Round 1's convention-note harvest item is consumed: it landed as the
  Tier-1 ledger entry at `docs/decision-ledger.yaml:4110` with the M1
  occurrence; nothing further owed.
- Reviewer self-lesson for re-diffs after an edit: extract fenced blocks
  by fence anchors, not by remembered line ranges - the fixed-range
  extraction produced a phantom one-line delta here (a swept-in closing
  fence) before the anchor-based re-run confirmed byte-exactness. Same
  family as the ledgered line-based-grep false-negative
  (`docs/decision-ledger.yaml:~4100`); worth an occurrence there rather
  than a new entry.
