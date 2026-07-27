# Task 1 verdict: D65-D70/D72 Add/Remove affordance + `e2e/editor-rule-add-remove.spec.ts` (cases 1-8)

**Spec compliance: APPROVED**
**Task quality: APPROVED**

Reviewed at `/home/senol/Git/Muxsmith/.worktrees/plan75-a`, branch `plan75-a`,
HEAD `fc9e9a41dcbeabdbbd34f1b6a8518e525710679c`, tree clean, range
`1d82179..fc9e9a4`. Every run below was foreground and in-worktree; no
session-relocation tool was called; nothing outside this verdict file was
written, and the worktree was re-confirmed clean (`git status --porcelain`
empty) after all probing.

---

## 1. What was verified, independently

Each check below was run by the reviewer, not read from the report. Every
check whose passing result is an absence was fire-verified against a
deliberately broken copy or a known-present control (`proc-verification-step-
must-be-falsifiable`). All fire-verification probing happened on scratchpad
copies, never in the worktree.

### 1.1 D67/D70 transcription fidelity (the report's `diff -wB` method, re-run)

Extraction was rebuilt from scratch rather than reusing the implementer's
script: each function by its own `^function <name>() {` / `^}` range (so both
comment blocks fall outside by construction), the template block by the
`</table>` / `</fieldset>` delimiters, and the design side from the worktree
copy of D67's fenced `ts` block and D70's fenced `html` block.

| Comparison | Result |
| --- | --- |
| `design addRule` vs landed | `diff -wB` IDENTICAL (exit 0), 9 lines both sides |
| `design removeSelectedRule` vs landed | `diff -wB` IDENTICAL (exit 0), 10 lines both sides |
| D70 template block vs landed | `diff -wB` IDENTICAL (exit 0), 15 lines both sides |

**Fire-verification of the extraction plus comparison, all three.** A probe
copy of `EditorView.vue` was mutated four ways and re-run through the same
extraction: skeleton value changed to `{ match: { exact: { type: "video" } } }`,
an `el.focus()` line inserted, `title="Add a rule"` added to the Add button,
`:disabled` widened to `selectedIndex === null || saving`, and (second probe
round) `selectedIndex.value = null` changed to `= 0` in `removeSelectedRule`.
Every check fired and named the exact delta. Note the first probe round left
`removeSelectedRule` untouched and its diff was correctly empty; that empty
result was NOT accepted as evidence until the second round mutated it and the
check fired. The three clean diffs are therefore evidence, not a malformed
comparison.

No attribute delta, no order swap, no changed `:disabled` expression, no
prefilled skeleton. The two permitted deltas (template indentation to 8
spaces, guard-brace formatting) are the only ones present, and the guard
braces were not reformatted at all (repo has no `curly` rule and no prettier
config; `pnpm lint` green confirms none was demanded).

### 1.2 The eight new e2e cases

```
$ pnpm exec playwright test --grep "editor rule add/remove"
  8 passed (704ms)
```

All eight listed individually, under the one describe titled
`editor rule add/remove (D65-D70, D72)`. Every assertion the brief enumerates
for cases 1-8 is present, including the drop-prone ones: case 1's `readModel`
anti-vacuity (`tracks.rules` length 3, member [2] `toEqual({ match: {} })`),
case 4's right-rule pair (removed summary gone AND the other still present),
case 6's (a) wire-truth check on the LAST recorded `validate_profile_model`
payload, (c) the bare-`tracks[1]` count-0 negative, (d) the panel line via
`en("empty-match-expression")`, (e) `editor-save` still enabled, and cases 7/8's
`drop`-error / `keep`-info pair with opposite Save gating.

`.only` / `.skip` / `fixme`: none (fire-verified pattern; control returns 1).

**Red-reachability**, since the red run cannot be re-run against landed code:
all 8 test bodies hard-depend on `editor-rule-add` or `editor-rule-remove`
(mechanically checked, 8/8), so the red run is red by construction, not by
report assertion.

### 1.3 The full frontend gate

```
$ pnpm lint      -> eslint . clean, no output
$ pnpm build     -> vue-tsc --noEmit && vite build, built in 152ms
$ pnpm check:i18n-> check-i18n: ok (41 source files scanned, 211 catalog ids,
                    19 IpcError code(s) gated, 22 help id(s) x 2 help
                    locale(s), 0 unused warning(s), 1 other locale(s) checked
                    for parity against 7 en/ catalog(s)).
$ pnpm test:e2e  -> 61 passed (2.7s)
=== pipeline exit: 0 ===
```

61 = 53 pre-existing + 8 new, matching the report. Zero existing spec files
edited: the range diff names exactly `e2e/editor-rule-add-remove.spec.ts` (new,
362 lines) and `src/views/EditorView.vue` (+50).

### 1.4 Zero new surface

| Constraint | Result |
| --- | --- |
| `gui-editor.ftl` id budget | 46 en / 46 de, recomputed from the enumeration (pattern fire-verified against real ids) |
| `locales/` `help/` `src/editor/` `scripts/` `src-tauri/` `crates/` | zero delta in the range; positive control over `src/` prints the one changed file |
| New help-ids / `data-help-id` | none added; the `data-help-id` set is unchanged (view-batch, view-jobs, view-editor, editor-tracks-rules, batch-suggestion-card, plus the registry's `spec.helpId`) |
| New topic files | none; 22 topics x 2 locales, matching check-i18n |
| New dependency (npm or cargo) | none; `package.json` and lockfiles untouched |
| Rust / `DiagCode` / eslint config | zero delta |
| `.tooltip` on either action key (D72) | absent in both locales, verified at the catalog |

### 1.5 D66 selection clearing and D69 zero-rule behavior, in the code

- `removeSelectedRule` sets `selectedIndex.value = null` (transcription-
  identical to D67), so the panel closes and no row stays `aria-current`;
  e2e case 4 asserts both.
- No zero-rule guard anywhere: the grep for a rules-length floor returns only
  two pre-existing `matchSummary` hits (`expr.any.length > 0`,
  `expr.not.length > 0`), never a guard on `rules` (pattern fire-verified
  against files that do test `.length`).
- Remove's ONLY disable condition is `selectedIndex === null`
  (`src/views/EditorView.vue:644`). The view's other three `:disabled`
  expressions are pre-existing (`opening || saving` twice, `saveDisabled` on
  Save). Add carries no `:disabled` at all, and the whole block sits inside
  `<template v-if="model">`, so D70's "never disabled while the grid renders"
  holds structurally.
- Cases 5/7/8 exercise zero rules end to end, including the sanctioned
  `keep`-passthrough path (`core-83-zero-rule-keep-passthrough`) staying
  saveable.

### 1.6 Help-mode suppression conformance of the new buttons

D71 claims conformance by construction, with zero new code. Verified against
the shipped delegation rather than taken on the design's word:

- `<main ref="mainEl">` (`src/App.vue:233`) wraps `<EditorView>` (`:254`), and
  the help-mode listeners register on that element in the `watch(helpMode)`
  block, all capture phase: `mouseover`, `focusin`, `click`, `dragstart` on
  `main`, `keydown` on `document`.
- `onHelpClick` calls `preventDefault()` then `stopPropagation()`
  unconditionally before resolving the id. A capture-phase `stopPropagation`
  on an ancestor halts propagation before the target phase, so the button's own
  bubble-phase Vue binding `@click="addRule"` never runs. Pointer channel
  killed, exactly the channel `help-mode-suppression-pointer-scope` closes.
- `onHelpKeydown` intercepts Enter and Space whenever `helpTarget(event)` is
  non-null and `preventDefault()`s, so no activation click is synthesized.
- `helpTarget` resolves `closest("[data-help-id]")`. The buttons sit as
  siblings of `</table>` inside the rules `<fieldset>`; the annotated
  `<caption data-help-id="editor-tracks-rules">` is INSIDE the table and
  therefore not an ancestor, so the nearest annotated ancestor is the view root
  `view-editor`. That is D71's stated fallthrough, not an accident of nesting.
- Zero button-side help-mode code: `helpMode` / `help-mode` / `helpTarget`
  appear nowhere in `EditorView.vue` (grep exit 1, pattern fire-verified
  against `App.vue`, 9 hits). Design section 8's "adding a button-side
  help-mode condition is a defect" is satisfied by absence, verified.
- Note for Task 2: a disabled `<button>` dispatches no click at all, so a
  suppression assertion against Remove would pass vacuously. The design's
  choice of Add as case 9's target is the non-vacuous one and the landed code
  preserves it (Add is never disabled).

The e2e proof of this behavior is Task 2's case 9 and is correctly NOT in this
task's scope.

### 1.7 Housekeeping claims

Commit `fc9e9a4`: unsigned (`%G?` = `N`), trailer
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, exactly the two
intended paths staged, no `git add -A` residue. Typography: both changed files
are pure ASCII and free of the AI-tell glyph class (em/en dash, smart quotes,
ellipsis, NBSP), both patterns fire-verified against known-present controls.
No line-number citation in any added line (`code-comment-line-citations-drift`),
pattern fire-verified against a real citation string.

Both landed comment blocks were checked for factual accuracy, since an
inaccurate comment would be a finding regardless of the Q1 adjudication:
`v-if="selectedRule"` is real, the no-focus claim is fire-verified, the
immutable-whole-model-rebuild claim matches `onDrop`/`setRuleValue`, and the
index-shift rationale matches the in-file `onDrop` precedent. All accurate,
all D-anchored, none carrying a line number.

---

## 2. Findings by severity

**Critical: none. Major: none. Minor: none.**

Informational (no fix owed by this task):

- **I1 - brief line-ref drift, confirmed.**
  `e2e-diagnostic-rendered-is-wire-ballast` is at
  `docs/decision-ledger.yaml:4111`, not `:4110` as `task-1-brief.md` line 32
  states. The report's concern 4 is correct. The landed spec cites the entry by
  bare id with no line number, so no code depends on the ref. Plan-artifact
  drift only.
- **I2 - design trigger 1 is still open (controller duty).**
  `editor-generic-action-keys` in `docs/product-boundaries.yaml` does not yet
  record the rule grid as the third render site of the generic pair (grep for
  "third consumer site" / "third render site" returns 0; pattern fire-verified
  against the design, which contains it). The design assigns this to the
  controller on consuming the design, so it is not a Task 1 obligation, but it
  is now due: the third site has landed.
- **I3 - the red-run evidence predates the disclosed amend.** The report's
  Step-4 output cites spec lines 96/126/144/156/179/209/277/319; the committed
  file's tests are at 97/127/145/157/180/210/278/320. The uniform +1 offset
  across all eight is exactly what one inserted line in the module doc header
  produces, which corroborates the disclosed single-line amend
  (concern 2) rather than contradicting it. Recorded so the controller does not
  read the offset as a discrepancy. The pre-amend hash `1933cf3` is unreachable
  by construction and was never pushed.
- **I4 - third local copy of the `name()` helper in `e2e/`.** `smoke.spec.ts:60`
  and `editor-markers.spec.ts:29` already define it identically and no shared
  module exports one. The third copy is the established house pattern, so this
  is explicitly NOT a finding; hoisting it would be a pattern change touching
  every site, out of scope here. Noted as a future shared-helper candidate
  only.

---

## 3. Adjudication questions

### Q1 - two comment blocks where Step 5 named one

**Verdict: grant-covered structural conformance. No fix owed.**

The landed file carries two blocks outside both function bodies: a region
header plus rationale above `addRule`
(`// --- Plan 7.5 (D65-D70, D72): Add/Remove for track rules ---` plus six
prose lines), and the D66 clearing note above `removeSelectedRule`.

**The case for calling it an enumeration violation, in its strongest form.**
Step 5's parenthetical is narrow twice over: it names ONE comment and it names
its SUBJECT MATTER ("may note the D66 clearing rationale"). The region header's
subject matter is D65/D67, not D66, so it falls outside the named permission
even on a generous count. And this seam is not accidental: the plan's own
round-1 review already worked it, recorded at
`proc-latitude-clause-boundary` occurrence 2026-07-23 ("Step 5 granted a comment
permission that Step 6's permitted-delta enumeration over the same artifact
omitted"), and the fix round harmonized the two steps. Deliberate language,
freshly reviewed, is language a reader is entitled to read as closed.

**Why it nonetheless falls to the grant.**

1. **The grant's own test is not "does a permission appear?"** Its statement
   reads: "The test is not 'does a permission appear?' but 'must the
   implementer invent something it is not allowed to invent?'" Nothing was
   invented. The header's content restates D65 and D67 in prose, both closed
   above; its form is mechanical.
2. **Step 6's closed enumeration is over deltas INSIDE the extracted blocks,
   not over comments.** Its text enumerates "the two permitted deltas: template
   indentation, guard-brace formatting" and then explains that "a Step-5
   comment sits above the functions, outside the extracted blocks, and
   therefore never appears in this diff". The comment clause is not a third
   delta; it is the reason comments are out of the diff's scope at all, and
   that reason holds identically for N comments. The indefinite article and the
   plural "functions" do not cap a count. Verified empirically: my per-function
   extraction is byte-identical to D67 with both blocks present, so neither
   comment touches any check the brief specifies.
3. **The convention is locally unbroken, 4 of 4.** `EditorView.vue` carries
   `// --- Task 13: ...`, `// --- Task 14 (D57): ...`,
   `// --- Tasks 11-12: ...`, `// --- Task 13b: ...` at lines 117/144/283/357,
   each followed by a bare `//` line and prose, each introducing one task's
   block. The landed header at 443 is the same shape at the same dash-fill
   width (76 chars, matching the 13b header exactly). This is the fifth
   instance of a five-instance pattern, not a novelty.
4. **The restrictive reading is a double bind.** `conventions.md` "Match the
   house pattern" makes the LONE DEVIATION the defect ("a lone deviation is a
   defect even when individually idiomatic"). A new script region in this file
   WITHOUT a region header would have been a legitimate finding under that
   rule. A reading that makes both the header and its absence findable is not a
   rule being applied; it is a rule being overfitted.
5. **Zero outward effect, and disclosed.** No runtime, DOM, catalog, test or
   API surface. The implementer flagged it rather than assuming it, which is
   the behavior the grant is designed to reward.

The preamble's grant covers exactly "zero-outward-effect structural
conformance", which is this, precisely.

### Q2 - the five cargo gate parts were not run

**Verdict: sanctioned by the step's own text. Not a gate cut.**

Verified at the brief: Step 10 is titled "Frontend gate, foreground" and its
run line is `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e`,
four parts, named explicitly and exhaustively. The nine-part obligation lives
in the preamble with a different trigger: "Nine-part gate green before any
push" plus "Full gate after every merge". Step 11 is a local commit; no push
occurred. The trigger did not fire.

The preamble's parenthetical ("the five cargo parts guard against accidental
drift, they do not become optional") governs the COMPOSITION of the nine-part
gate when it runs, not the composition of Step 10's separately named frontend
gate; "no subsets" likewise binds the nine-part gate at its own trigger. No
conflict between the two texts.

Independently checked, and it strengthens the position: the range
`1d82179..fc9e9a4` carries zero delta under `*.rs`, `*.toml`, `*.lock`,
`crates/`, `src-tauri/` (positive control over `*.vue`/`*.ts` prints the two
changed files). The cargo-side state at `fc9e9a4` is therefore byte-identical
to the base commit by construction, so running the five parts here would have
produced evidence about the base, not about this task. Not running them cost
nothing; the merge-time full gate remains owed and the report flagged it to
the controller rather than letting it pass silently. That is the correct
handling.

### Q3 - filtered runs satisfied by full-suite superset runs

**Verdict: acceptable satisfaction of both steps' intent. No finding against
the task; the defect is in the plan's command form, already harvested.**

Reproduced independently in this worktree, pnpm 11.10.0:
`pnpm test:e2e -- --grep "editor rule add/remove"` ran the full suite
(`61 passed`), while `pnpm exec playwright test --grep "editor rule add/remove"`
filtered correctly (`8 passed`). The report's tooling observation is true as
measured.

Both steps' intent survives the superset:

- **Step 4 (red).** Its expected result is "FAIL every case". The full-suite
  run reported all 8 individually red on locator timeouts for the not-yet-
  existing testids, plus 53 pre-existing green. That is strictly MORE evidence
  than the filtered run would have given, not less; nothing about the red
  signal is diluted by the passing neighbours. Red-reachability was separately
  confirmed structurally (8 of 8 cases hard-depend on a new testid).
- **Step 7 (green).** Satisfied twice over: the filtered `8 passed` via the
  working form, and the full `61 passed`, which is Step 7's second half anyway.

The residue is a plan-authoring defect, and it is already closed:
`e2e-filter-invokes-playwright-directly` (`docs/conventions.yaml:1049`) was
promoted to Tier 2 at count 3, and its third occurrence cites this very report
("the PLAN itself still carried the defeated form - the tier-1 entry was
invisible at plan authoring, which is the promotion's own argument").

One process credit worth recording: the report states the observation and
explicitly withholds a mechanism ("I state only the observation, not a
mechanism"). The Tier-2 entry supplies the mechanism (pnpm 11 forwards the `--`
separator literally; against a `&&`-chained script the args land on the outer
`sh -c` as positionals). Observation from the implementer, mechanism from the
entry, is the right division of a borrowed claim.

---

## 4. HARVEST

### Observed patterns worth propagating

- **The transcription-diff method is sound and reusable.** Extract each closed
  shape by its own syntactic range (a function by `^function name() {` / `^}`,
  a template block by its delimiters), compare `diff -wB` against the design's
  fenced block, then fire-verify by injecting an attribute into a COPY and
  watching the same comparison fire. Two independent implementations (the
  implementer's and mine, written without reading theirs) produced identical
  verdicts, which is the property a method wants. Cheap, and it catches exactly
  the class D70/D72 enforcement cares about (added `title`, changed
  `:disabled`, swapped order).
- **Fire-verify per assertion, not per script.** My first probe round mutated
  `addRule` and the template but not `removeSelectedRule`; that third diff came
  back empty and the empty result proved nothing until a second round mutated
  it. A fire-verification that exercises three checks with one probe silently
  leaves the unexercised ones unverified. Worth stating in the doctrine's
  falsifiability entry if it is ever touched: the probe must hit every check
  whose absence is being trusted.
- **Anti-vacuity controls in the new spec are the template to copy.** Three
  absence assertions, three controls: case 6(c)'s bare-`tracks[1]` count-0
  paired with 6(b) as the brief-designated in-test positive control on the same
  marker layer; case 4's "no row is current" preceded by a demonstrably
  matching selected row; the zero-count transitions each preceded by a nonzero
  count of the same locator.
- **Observation without mechanism, when the mechanism is borrowed.** See Q3.
  The implementer had a measured behavior and no verified cause, reported the
  first and withheld the second. Propagate.
- **Actionable for the remaining briefs:** Task 2 (help-mode case 9) and Task 4
  (amendment 1) both own e2e work and their briefs are not yet written. They
  must carry `pnpm exec playwright test --grep "<pattern>"`. `task-1-brief.md`
  is the only brief in this plan folder carrying the defeated form, and
  `task-3-brief.md` needs no filter form at all.

### Repeated rejections

None new. D66 (no confirmation dialog, no successor auto-select), D67 (no
programmatic focus), D68 (no dedicated keys), D69 (no last-rule floor) and D72
(no tooltips) all landed without a re-litigation attempt anywhere in the report,
which is `proc-proposed-safeguard-stays` running in the intended direction. No
NEEDS_CONTEXT was raised and, on review, none was owed: no fork surfaced on
code contact.

### Over-restriction flags

- **Q1 is the calibration datum, and it lands on the over-restriction side.**
  The seam is a phrasing artifact, not a discipline failure. Step 5's comment
  permission is scoped by SUBJECT MATTER ("the D66 clearing rationale") where
  the thing it needed to scope was PLACEMENT (outside the extracted blocks).
  The plan-review fix round closed Step 6's delta enumeration correctly and
  left that narrowness untouched, producing a clause under which the file's own
  unbroken 4-of-4 convention reads as an unpermitted addition. Since omitting
  the header would have been findable under "Match the house pattern", both
  branches were findable, which is the signature of an overfitted rule rather
  than a real defect.
- **No fourth clause is owed.** Per
  `feedback_regel_braucht_ausloeser_und_handgriff`, the honest answer to this
  class is the standing grant, which fired correctly and produced the right
  outcome with a disclosure attached. If the plan template is ever edited, the
  minimal fix is to rephrase by class rather than add a rule: "comments in the
  file's own register, placed outside the extracted blocks, are unrestricted in
  number and content-anchored to the design" replaces "may note the D66
  clearing rationale". That is a phrasing change to an existing clause, not new
  machinery.
- **Watch item for the next instance.** This is the first case in this plan
  where the enumeration discipline generated a candidate finding against
  zero-outward-effect house conformance. One instance is phrasing. A second in
  the same plan would argue that the comment-permission pattern itself, not the
  individual clause, is the defect, and would justify hoisting the class-level
  phrasing above into the brief template rather than fixing it per task.
- **I4 (`name()` helper, third copy) is deliberately not a finding.** Recording
  it here so a later reviewer does not mistake the omission for an oversight:
  three local copies is the house pattern, and the reuse rule does not reach a
  case where no shared export exists and hoisting would touch every site.

### Verification limits, stated

- The historical red run cannot be re-executed against landed code under the
  read-only mandate. Substitutes used: structural red-reachability (8 of 8),
  the report's quoted per-case output, and the +1 line offset corroborating the
  amend. The red claim is sound; it is not first-hand.
- The pre-amend commit `1933cf3` is unreachable by construction, so the
  amend's content is taken from the report plus the offset corroboration.
- The implementer's own probes were transient. Every one that mattered was
  re-run independently here rather than accepted.
