# Task 2 verdict: D71 help-mode conformance - the additive case 9 in `e2e/help-mode.spec.ts`

**Spec compliance: APPROVED**
**Task quality: APPROVED_WITH_FINDINGS** (one Medium comment correction recommended; no rework required for the plan to proceed)

- Reviewer: independent, mid tier. Read-only on the tree except the temporary,
  restored fire-verification mutations documented in section 1.3; this file is
  the only write.
- Reviewed: branch `plan75-a`, commit `92ba1e79a179415089afbdbdc8a9fae7fc0b66ff`
  (worktree `/home/senol/Git/Muxsmith/.worktrees/plan75-a`), base `fc9e9a4`
  (Task 1), range `fc9e9a4..92ba1e7`.
- Inputs: `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-7.5/task-2-brief.md`
  (the requirements),
  `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-7.5/task-2-report.md` (graded
  as claims, not as evidence),
  `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-7.5/review-fc9e9a4..92ba1e7.diff`,
  `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-7.5/implementer-preamble.md`
  (binding).
- Ground truth read directly in the worktree: design
  `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`
  D71 (:556, help-mode block :622-639, the attribution sentence :631, the
  closing claim :638) and section 5 case 9 (:898-916, the numbered item at
  :908); `docs/product-boundaries.yaml` `help-mode-suppression-pointer-scope`
  (:463); `docs/decision-ledger.yaml`
  `e2e-filter-invokes-playwright-directly` (:3924),
  `content-claims-anchor-bound` (:3810); `docs/conventions.yaml`
  `code-comment-line-citations-drift` (:1012); `src/App.vue`,
  `src/help/state.ts`, `src/views/EditorView.vue`,
  `src/components/SuggestionCard.vue`, `e2e/mocks.ts`,
  `e2e/help-mode.spec.ts` (whole file), `playwright.config.ts`.
- No session-relocation tool was called. Every run was foreground, in-worktree,
  with the direct playwright invocation form
  (`e2e-filter-invokes-playwright-directly`); no `pnpm <script> --` separator
  form was used anywhere. Worktree re-confirmed clean after all probing
  (`git status --porcelain` empty, `git diff --exit-code` 0, `md5sum` identity
  against pre-mutation backups).

---

## 1. What was verified, independently

### 1.1 Runs (all foreground, `cd /home/senol/Git/Muxsmith/.worktrees/plan75-a`)

`dist/` was rebuilt from the committed source before the first run, so nothing
below could be measured against a stale `vite preview` payload.

| Command | Result |
| --- | --- |
| `pnpm build` | clean (`vue-tsc --noEmit && vite build`) |
| `pnpm exec playwright test --grep "both activation channels are suppressed"` | `1 passed (738ms)`, and the selection was exactly one test: `e2e/help-mode.spec.ts:186:3` - the filter genuinely narrowed, it did not silently run the suite |
| `pnpm exec playwright test help-mode` | `9 passed (902ms)` - the new case plus all 8 pre-existing ones |
| `pnpm exec tsc --noEmit -p e2e/tsconfig.json` | clean (the typecheck `pnpm exec playwright test` alone skips) |
| `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e` | all green; `check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), ...)`; `62 passed` |

The four frontend gate parts are therefore re-verified by me, not borrowed. The
five cargo parts were not run: this commit touches one `.ts` test file, and the
same ruling was already made in `task-1-verdict.md` Q2.

### 1.2 Additive-only check, with its own positive control

```
git diff --numstat fc9e9a4 92ba1e7        ->  83   0   e2e/help-mode.spec.ts
git diff --name-status fc9e9a4 92ba1e7    ->  M       e2e/help-mode.spec.ts
git show --stat 92ba1e7                   ->  1 file changed, 83 insertions(+)
git log -1 --format='%GG' 92ba1e7         ->  (empty: unsigned, as required)
trailer                                   ->  Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
```

Commit subject is the brief's Step 6 subject verbatim. The deletion column is a
passing-by-absence check, so it was fire-verified **without mutating the tree**:
the same command shape in the reverse direction over the same file yields
`0  83`, which proves the column is live rather than structurally zero. (The
implementer's own fire-verification - delete a line, observe `83  1`, restore -
is a valid second route; mine needed no mutation.) One file, no Task 1 file
touched, zero production code, zero new surface (the unchanged 22 help ids x 2
locales and 211 catalog ids in `check:i18n` are the gate's own confirmation of
D71's zero-new-help-id claim).

### 1.3 Fire-verification: are the assertions live, and what do they actually witness?

All four rounds mutated `src/App.vue` only (plus, in round C, one expectation
line in the spec), each round starting from a `command cp -f` restore of a
pre-mutation backup, each followed by `pnpm build` because `vite preview` serves
`dist/`. Final state verified byte-identical
(`md5sum` matches the backups, `git status --porcelain` empty,
`git diff --exit-code` 0), `dist/` rebuilt from the restored source, and
`pnpm exec playwright test help-mode` re-run green (`9 passed`).

| Round | Mutation | Expected | Observed |
| --- | --- | --- | --- |
| A | `preventDefault` + `stopPropagation` commented out in `onHelpClick` (`src/App.vue:63-64`) | click half fails | FAILED at `e2e/help-mode.spec.ts:240`, `Expected: 3 / Received: 4` |
| B | `preventDefault` + `stopPropagation` commented out in `onHelpKeydown`'s Enter/Space branch (`src/App.vue:91-92`) | report says PASSES | PASSED; and the **full suite**: `62 passed` |
| B2 (mine) | the **entire** Enter/Space branch commented out (`src/App.vue:90-94`), pin included | tests the report's claimed separate guard | `pnpm exec playwright test help-mode` -> `9 passed`; full suite -> `62 passed` |
| C | both suppression layers down, click-half expectation at :240 temporarily relaxed 3 -> 4 | Enter half fails | FAILED at `e2e/help-mode.spec.ts:251`, `Expected: 3 / Received: 5` |
| D (mine) | only `pinnedId.value = id` commented out in `onHelpClick` (`src/App.vue:67`); suppression left intact | tests what the innerHTML assertion witnesses | PASSED - the assertion does **not** witness the pin (finding M1) |

Rounds A and C reproduce the report's numbers to the digit. Both halves of the
case are therefore reachable and falsifiable; neither is structurally
always-true, and neither needed a scheduler-flush helper (the mutation is
visible at the assertion that follows the activation).

Round B2 is the decisive one for Q1 and is treated there.

### 1.4 Mechanism claims in the report and the comments, checked at the artifact

| Claim | Verdict |
| --- | --- |
| Three describes, exact titles `"help mode (D52)"`, `"help mode annotations (D54)"`, `"help mode drag suppression (I1)"` | true (`:95`, `:269`, `:478`); count still 3, so the design's "the file's three describes" needs no recount |
| The new test is the third and last test inside the D52 block | true - it spans `:186-252`, the D52 describe closes at `:253` |
| No live normative count over this file's case set exists in `docs/` | true - every `help-mode.spec` hit under `docs/` is a plan/design record (evidentiary), and `BUILDING.md` carries no e2e case count |
| `mocks.ts` repeats the last queue entry on exhaustion, so single-entry queues cover the extra revalidations | true (`e2e/mocks.ts:94`: `return q.length > 1 ? q.shift() : q[0];`) |
| Comments anchor on symbol names, never a bare `file:line` span | true - `code-comment-line-citations-drift` conformant |
| Add-not-Remove rationale is factually grounded | true - `src/views/EditorView.vue:643` renders Remove with `:disabled="selectedIndex === null"` |
| The buttons carry no `data-help-id`; the fallthrough resolves to `view-editor` | true - `EditorView.vue:482` carries `data-help-id="view-editor"` on the view root, `:556` puts `editor-tracks-rules` on the `<caption>` inside the table (not an ancestor of the buttons at `:636`/`:643`) |
| Typography: no non-ASCII in the added lines | true - `git diff ... \| grep '^+' \| LC_ALL=C grep -n '[^ -~]'` empty, with the pattern's positive control firing on 9 lines of the design document |
| "the pinned topic is this half's evidence that the listener actually handled the click" | **false** - see finding M1 |
| "the pre-existing D54 case ... exercises `onHelpKeydown`'s own effect directly" | **false** - see finding M2 |

---

## 2. Spec compliance

Graded against design section 5 case 9 (`:908`), the D71 help-mode block
(`:622-639`) and the brief. Every enumerated element is present, in the
enumerated order, and nothing beyond it was added.

| Enumerated element | Landed |
| --- | --- |
| Appended inside `test.describe("help mode (D52)")`, the activation-suppression family | yes, `:186-252`, last test in the block |
| Exactly ONE test, additive only | yes, `83  0` over one file |
| I1-sibling shape: real served app + `installTauriMocks`, own opened-profile fixture, mutation control and suppression assertion in the SAME test and harness | yes - structurally identical to the I1 case at `:479` (see the shape caveat in Q1) |
| Fixture: one-rule profile, `exact: { type: "video" }` | yes (`oneRuleProfile` at `:197-201`, `input: { pattern: ".*", extensions: ["mkv"] }` at `:199`) |
| Mocks: `detect_mkvmerge`, `"plugin:dialog\|open"`, `load_profile`, `validate_profile_model` | yes, all four (`:204-208`), sibling-shaped |
| Open the profile (`nav-editor`, `editor-open`), confirm the starting row count | yes, `toHaveCount(1)` |
| Outside help mode, channel 1: click Add -> +1 | yes, `toHaveCount(2)` |
| Outside help mode, channel 2: focus Add + Enter -> +1 | yes, `toHaveCount(3)` |
| Toggle help mode on (`help-toggle`), sidebar visible | yes |
| Inside help mode, channel 1: click Add -> count unchanged AND sidebar renders `view-editor` via `normalizeInPage(page, topicMarkup("view-editor"))` | yes, both assertions present exactly as the brief dictates (what the second one is worth: finding M1) |
| Inside help mode, channel 2: focus Add + Enter -> count unchanged | yes |
| Add-not-Remove rationale transcribed as a comment | yes, verbatim in the doc comment |
| Zero production code, no button-side help-mode condition, no help-id, no topic, no registry, no listener, no dependency | yes |
| Title verbatim from the brief | yes |
| Commit: single file staged by path, unsigned, brief's subject, repo trailer | yes |

Two brief-conformance notes, both in the implementer's favour:

- **The brief's Step 3 commands were defeated by design and the implementer
  corrected them.** Step 3 prescribes `pnpm test:e2e -- --grep "..."` and
  `pnpm test:e2e -- help-mode`; `pnpm test:e2e` is a `&&`-chained script, so
  under `e2e-filter-invokes-playwright-directly`
  (`docs/decision-ledger.yaml:3924`) the forwarded tokens land on the outer
  `sh -c` and the filter is silently defeated. The implementer ran
  `pnpm exec playwright test ...` and disclosed the substitution. Correct call,
  correctly reported; the defect is in the brief (finding L1).
- The task did not run `pnpm test:e2e`'s pre-steps in Step 3 (the `e2e/` tsc
  pass and the two harness builds), but Step 5 ran the full script. I re-ran
  both the tsc pass and the full script independently: clean.

Spec compliance: **APPROVED**.

---

## 3. Findings by severity

### Critical / High

None. The landed case is non-vacuous, reachable, additive, and asserts what the
design told it to assert.

### Medium

**M1 - the help-ON click half's sidebar assertion witnesses nothing, and the
comment claims it does.** The committed comment reads:

> `// (D71's fallthrough). The pinned topic is this half's evidence that the`
> `// listener actually handled the click, so the unchanged row count is a`
> `// suppression rather than an event that never arrived.`

Round D falsifies it: with `pinnedId.value = id` removed from `onHelpClick`
entirely (suppression left intact), the case still passes. The reason is
structural, not incidental: the sidebar resolves
`pinnedId ?? hoverId ?? VIEW_TOPICS[activeView]` (`src/App.vue:49-51`) and
`VIEW_TOPICS.editor === "view-editor"` (`src/help/state.ts`), while the
Playwright click's own `mouseover` sets `hoverId` to that same id through the
same ancestor walk. All three states render the identical topic, so the
`innerHTML` compare holds whether the click pinned, merely hovered, or was
never seen. The half is still not vacuous - its row-count assertion is
discriminating (round A) - but the *mechanism attribution* in the comment is
false.

Scope of blame: the assertion itself is the design's (`:908` says "count
unchanged and `view-editor` pinned") and the brief's (it names the `innerHTML`
compare as the mechanism), so the implementer was right to write it. The
sentence claiming what it proves is the implementer's own text, and it is the
one thing in this commit that a future reader would act on wrongly. Minimal fix
is one sentence, e.g.: *"the sidebar assertion pins the topic identity only -
in the editor view the unpinned fallback is `view-editor` too, so it is not
evidence that the listener ran; the row count is."* Recommended for a fix round
or an explicit controller deferral. No named Tier-1/Tier-2 entry covers it:
`content-claims-anchor-bound` is scoped to authored *user-facing* content
(`docs/decision-ledger.yaml:3810`), and `code-comment-line-citations-drift`
binds `file:line` citations, of which there are none here.

**M2 - the report's claimed separate guard does not exist.** The report closes
its own finding with "That mechanism is not unguarded repo-wide: the pre-existing
D54 case `keyboard: focusin swaps the topic (focusin equivalence); Enter on a
focused annotated element pins it` exercises `onHelpKeydown`'s own effect
directly." Round B2 refutes it: with the **entire** Enter/Space branch of
`onHelpKeydown` commented out - suppression *and* pin - `pnpm exec playwright
test help-mode` reports `9 passed` and the full suite reports `62 passed`,
including that D54 case. The cause is the same synthesis the report itself
discovered: `batch-suggestion-copy` is a `<button>`
(`src/components/SuggestionCard.vue:80-87`) inside
`<article data-help-id="batch-suggestion-card">`, so Enter synthesizes a click,
`onHelpClick` catches it at capture phase and pins the *same* id. There is no
second test layer to fall back on: the repo has no vitest/jest frontend unit
runner, and `e2e/help-mode.spec.ts` is the only spec referencing help mode at
all. `onHelpKeydown`'s Enter/Space branch is currently **unguarded repo-wide**.

This is a borrowed claim presented as fact - the one class the report otherwise
policed well (it labelled its own attribution finding precisely and withheld the
unlicensed strengthening). One extra command (the same neutralization, run over
the file instead of the single grep) would have caught it. It changes no code and
no verdict on the artifact; it changes the *disposition* of the routed
observation, which is why it is Medium and why it feeds Q1 directly.

### Low

**L1 - the brief carried the defeated filter form.** `task-2-brief.md` Step 3
prescribes `pnpm test:e2e -- --grep` and `pnpm test:e2e -- help-mode`, written
after `task-1-verdict.md`'s HARVEST had already stated that Task 2's brief "must
carry `pnpm exec playwright test --grep`". The controller-side lesson did not
reach the next brief it named. No damage: the implementer caught it and
disclosed the substitution.

**L2 - fire-verification scoped to the filtered case only.** Rounds A-C were run
against the single grepped test. That is enough to prove the two new assertions
live, and not enough to answer "is this mechanism guarded elsewhere" - which the
report nevertheless answered, from inspection rather than from a run (M2). The
generalizable rule is the one `task-1-verdict.md`'s HARVEST already stated in a
narrower form: when a neutralization is used to argue about coverage, run it
against the coverage set, not against the case under test.

**L3 - bookkeeping.** `progress.md` records "pnpm-grep pattern promoted tier-2"
for Task 1, while the entry in the tree reads `tier: 1` with `promoted_at: null`
and no occurrence added during this plan (`docs/decision-ledger.yaml:3924`; the
one ledger edit in this plan, `1d82179`, touched the wrapped-prose entry). The
dispatch brief for this review likewise calls it Tier-2. The binding force is
identical either way, so this is a label to reconcile at the plan close, not a
constraint miss.

### Nits

- The doc comment's "in the I1 sibling's shape below" is a positional
  cross-reference. It survives because the sibling is also named by its describe
  title, but "below" is the kind of anchor a later insertion falsifies silently.

---

## 4. Q1 - the over-determined Enter half

**Question.** Is the case correct as landed (contract-level test; mechanism
guarded elsewhere; the design's own enumeration governs), or does it owe a
strengthening that the design's closure would first have to license via
amendment?

**Verdict: correct as landed. No strengthening is owed *by this task*, and the
strengthening the case appears to want may not be added without a design
amendment - one that must specify the witness, because the obvious witness does
not work.**

### The two premises, checked

1. **The redundancy claim is true.** Reproduced exactly (section 1.3): round A
   fails at the click half (`Expected: 3 / Received: 4`), round B passes with the
   keydown suppression fully removed, round C fails at the Enter half
   (`Expected: 3 / Received: 5`) only once both layers are down. Enter on a
   focused `<button>` synthesizes a click; the capture-phase `onHelpClick` on
   `<main>` stops it before `@click="addRule"`. Both layers independently close
   the keyboard path into the model, so the Enter-half assertion cannot
   attribute the closure to either.
2. **The claimed separate guard is false.** Round B2: the entire Enter/Space
   branch removed, `9 passed` over the file and `62 passed` over the suite, the
   D54 keyboard case included, for the same synthesis reason (M2). Nothing in
   the repo guards that branch.

So the honest picture is one step worse than the report's: not "mechanism
asserted elsewhere, outcome asserted here", but "mechanism asserted nowhere,
outcome asserted here".

### Why the case is still correct as landed

- **The design's enumeration is closed and it is the contract.** Case 9 (`:908`)
  and D71's closing claim (`:638`, "Both mutation paths into the model are
  therefore closed in help mode by the existing capture-phase delegation") are
  outcome claims about the model, and the case verifies exactly that outcome,
  falsifiably (round C). D71's `:631` attribution sentence is a rationale
  paragraph, not an enumerated assertion.
- **The implementer had no latitude, and correctly took none.** Global
  Constraint 2 of the preamble plus design section 8 bind every task; under
  `proc-latitude-clause-boundary` an unenumerated addition is not licensed by
  the enumeration's silence in either direction. Adding a mechanism assertion
  would have been the implementer widening a closed set at the keyboard -
  precisely the move the plan forbids. Observing, withholding and routing is the
  behaviour the process wants, and it is what happened.
- **Redundancy at the contract level is a property, not a defect.** Two
  independent layers closing one channel is the shipped design; a test that
  asserts the user-visible outcome stays correct if a later refactor moves the
  closure between layers. A mechanism-level test is the complement of that
  value, not a replacement for it.

### Why a strengthening is nevertheless owed - at design level, by amendment

The reason case 9 was allowed to stop at the outcome was the belief that the
mechanism is guarded elsewhere. That belief is false (M2). Concretely exposed:
`onHelpKeydown`'s Enter/Space branch could be deleted wholesale today and the
gate would stay green in all nine parts. That is a regression-detection gap, not
a live defect - the shipped behaviour is correct as of `92ba1e7`, verified.

The amendment must name the witness, because the two obvious candidates fail:

- **Asserting the pin after Enter does not discriminate.** Both routes pin the
  same id: `onHelpKeydown` pins `helpTarget(event)`, and the synthesized click
  pins `helpTarget(event)` through `onHelpClick` - `view-editor` either way. And
  in the editor view the pin is unobservable through the sidebar at all
  (round D, M1).
- **Asserting the row count in any arrangement does not discriminate** - that is
  the finding itself.

What does discriminate is an event-level witness, and the file already contains
the pattern: the I1 sibling dispatches its own event and reads
`dragstart.defaultPrevented` (`e2e/help-mode.spec.ts:68-93`, asserted at `:532`
as `dragstartPrevented === true`). The keyboard counterpart is the same shape -
dispatch a real `keydown` through `page.evaluate` and assert `defaultPrevented`,
or count clicks arriving at the button. Worth recording in the amendment: case
9's fidelity to "the I1 sibling's shape" is fidelity to its
control-plus-suppression *structure*; the sibling's mechanism witness has no
counterpart in case 9, and that asymmetry is exactly the gap.

**Recommended routing (controller's call, not a merge blocker):** accept Task 2
as landed; open a design-level item carrying both halves - (a) D71 `:631`
describes the keydown route as *the* thing closing the keyboard channel, which
is incomplete: the click delegation closes it redundantly (the report's own
formulation, and correct); (b) the branch is unguarded repo-wide, so add one
mechanism-witness case in the D52/D54 family with the witness *specified*
(`defaultPrevented` on a dispatched keydown, per the I1 precedent). Leaving the
witness to an implementer would reproduce this class one level down: they would
reach for the pin assertion, which passes for free.

---

## 5. HARVEST

### Observed patterns worth propagating

- **A suppression test over a `<button>` is over-determined by default.** Enter
  on a focused button synthesizes a click, so any capture-phase click
  suppression also closes the keyboard path. Any future "keyboard channel is
  suppressed" case against a button element is structurally incapable of
  attributing the closure unless it witnesses the keydown itself
  (`defaultPrevented`) or counts activations at the handler. Generalizes past
  help mode: it is the same trap for any two-layer guard where the outer layer
  catches the inner layer's fallout.
- **Neutralize at the layer whose coverage you are claiming, and run the
  coverage set.** The report's rounds A-C were correctly designed for
  "is my assertion live" and then reused, informally, for "is this mechanism
  guarded elsewhere" - a different question needing a different run (the same
  mutation over the file or the suite, one command). Round B2 answered it in
  three seconds and inverted the conclusion. Candidate refinement of
  `proc-verification-step-must-be-falsifiable`'s reviewer practice: a coverage
  claim is fire-verified by deleting the mechanism and running everything that
  supposedly covers it, not by deleting it and running the case in front of you.
- **A negative-shaped check can be fire-verified without mutating the tree.**
  The numstat deletion column was proven live by running the identical command
  in the reverse commit direction (`0  83`). Where a check has a symmetry like
  that, prefer it to break-and-restore: no restore step, no window in which the
  tree is dirty, and the control is in the same command family as the check.
- **The implementer's discipline that worked, worth keeping in the loop:** it
  measured a deviation candidate, established it empirically, withheld the
  change because the enumeration was closed, and routed it with the consequences
  spelled out. The only failure was one unverified sentence about somebody
  else's coverage - which is the class this plan's own memories keep hitting
  (borrowed claim, presented as fact, in the same report that was careful about
  its own claims).

### Repeated rejections

None. D71's no-new-help-id, no-new-topic, no-button-side-condition rulings were
implemented without any re-litigation attempt, and the gate's own unchanged
`22 help id(s) x 2 help locale(s)` is the mechanical confirmation. No
NEEDS_CONTEXT was raised and none was owed: the one fork candidate was reported,
not resolved.

### Over-restriction flags

- **The closed enumeration bit here, correctly, and cost something real.** The
  case's inability to detect a keydown regression is a direct consequence of
  "the implementer may not widen a closed enumeration" - and the rule was still
  right: the implementer's proposed strengthening (pin/`defaultPrevented`
  guesses) would have been the wrong assertion anyway, as round D shows. The
  cost is one round trip through the controller, which is what the mechanism is
  for. No clause change recommended.
- **Brief-authoring, not implementer-restriction, is where this plan keeps
  leaking:** L1 (defeated filter form, already named in the previous verdict's
  harvest) and the design's own untested attribution premise (Q1) both entered
  through authored text upstream of the keyboard. Both are cheap to catch with a
  grep and a run, respectively.

### Verification limits, stated

- The five cargo gate parts were not run (no Rust in this commit; consistent
  with `task-1-verdict.md` Q2). Rust could not guard `onHelpKeydown` in any
  case.
- Round B2's full-suite run reused the mount/harness bundles built by the
  implementer's `pnpm test:e2e` (my mutation touched `src/App.vue`, and only the
  real-app `dist/` was rebuilt). That does not weaken the conclusion: the mount
  specs mount components directly, no mount spec references help mode, and the
  delegation under test lives in `App.vue`, which only the real-app specs
  exercise.
- I did not attempt to author or run the proposed mechanism-witness case; that
  is design-gated work, and writing it would have been the same latitude
  overreach I am crediting the implementer for refusing.

---

# Re-review: fix round 1, commit `ae24589` (scoped re-reviewer, 2026-07-27)

**M1: ADDRESSED. M2: ADDRESSED. No new breakage in the fix diff.**

- Scoped re-reviewer, dispatched fresh (the original reviewer's transcript was
  not available and was not needed). Read-only on both trees except this file
  and three transient `src/App.vue` mutations in the worktree, each restored and
  verified.
- Reviewed: branch `plan75-a`, commit
  `ae24589850002136f074f9a5598a04206938bf2b` on top of `92ba1e7`, worktree
  `/home/senol/Git/Muxsmith/.worktrees/plan75-a`, range `92ba1e7..ae24589`.
- Inputs: the two findings above (M1/M2), the fix report appended to
  `task-2-report.md` (graded as claims), the scoped diff package
  `review-92ba1e7..ae24589.diff`, and the AMENDED design read from the main tree
  (`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`,
  case 9 at `:913-988`, witness extension `:924-988`).
- All runs foreground, in-worktree, direct `pnpm exec playwright test` form
  (`e2e-filter-invokes-playwright-directly`); no `pnpm <script> --` separator
  form anywhere. No session-relocation tool called. No git write of any kind.
- Nothing was listening on `:4173` before the first run (`ss -tlnp` empty for
  that port, no `vite preview` process), so `reuseExistingServer` could not have
  served a foreign `dist/` into any round below. Every round rebuilt `dist/`
  from the source state under test.

## M1 - ADDRESSED

The three quoted comment lines are the commit's **only** deletions, verified at
the artifact rather than taken from the report:

```
git diff --numstat 92ba1e7 ae24589                  ->  57  3   e2e/help-mode.spec.ts  (one file, M)
git diff -U0 ... | grep '^-' | grep -v '^---'       ->  exactly the three quoted comment lines
same, further filtered by grep -vE '^-\s*//'        ->  empty
```

That last check is an absence, so it was fire-verified: piping a synthetic
`-    await expect(rows).toHaveCount(3);` through the identical filter prints it.
No landed assertion was deleted or modified; every `expect(` change in the range
is an addition.

The replacement text (`e2e/help-mode.spec.ts:279-283`) states topic identity
only and denies the evidence claim in terms:

> `// (D71's fallthrough). The sidebar assertion checks topic IDENTITY only`
> `// and is NOT evidence that the listener ran:` `pinnedId ?? hoverId ??`
> `// VIEW_TOPICS[activeView]` `resolves to` `view-editor` `in this view whether`
> `// the click pinned it, the pointer merely hovered it, or the listener`
> `// never saw it -- the row count is what carries the suppression evidence.`

It names the correct evidence carrier (the row count, which round A proved
discriminating) and the correct reason (all three resolution states render the
same topic in this view). Residual-claim sweep over the whole file: `evidence`,
`proves`, `proof` occur at `:212`, `:280`, `:283` only; `:212` is the case's own
doc comment about the conformance claim, the other two are this correction. No
pin-as-evidence phrasing survives anywhere in case 9.

## M2 - ADDRESSED

### The helper landed verbatim

The design's fenced block (`:942-953`, dedented by its 5-space list indent) and
the landed helper (`e2e/help-mode.spec.ts:117-128`) are **byte-identical**:
`diff -u` empty, both `md5sum 4c8bd95ca728a6ccda6288643e2c10c8`. `cancelable:
true` is present. Placement matches the amendment ("module level beside
`attemptDrag`"): directly after `attemptDrag`, before the D52 describe at
`:130`. The three prescribed steps landed in the prescribed positions: the
`false` control at `:265` after the Enter mutation control, its side-effect
count re-assertion at `:269`, the `true` witness at `:305` after the landed
Enter suppression assertions and as the case's last statement (so "no assertion
after this point reads pin state" is true by inspection).

### Acceptance fire-test, reproduced independently

Backup taken before any mutation (`md5 c520a02f4c215ae86130ddb504d57f02`,
matching the report's figure). Each round: `command cp -f` restore, edit,
`pnpm exec vite build`, run.

| Round | Mutation to `src/App.vue` | Scope run | Observed |
| --- | --- | --- | --- |
| baseline | none | `playwright test help-mode` | `9 passed` |
| B2 | entire Enter/Space branch (`:90-94`) commented out, pin included | `playwright test help-mode` | `1 failed, 8 passed`; sole failure `e2e/help-mode.spec.ts:221:3` (case 9) at `:305`, `Expected: true / Received: false` |
| B | only `preventDefault` + `stopPropagation` commented out, pin kept | `playwright test help-mode` | identical: `1 failed, 8 passed`, case 9 at `:305`, `Expected: true / Received: false` |
| B2 full | entire branch commented out | `playwright test` (whole suite) | `1 failed, 61 passed`; sole failure repo-wide is case 9 at `:305` |
| restore | none | `playwright test` (whole suite) | `62 passed` |

Both shapes the amendment names as the acceptance criterion fail at the witness
and only at the witness. Because Playwright aborts a test at its first failing
assertion and `:305` is the last statement in case 9, a failure there means
every previously-landed assertion in the case passed in both rounds; the other
eight cases in the file passed explicitly in each.

The B round is the sharper of the two: with the pin left intact and only the
suppression removed, the witness still fires. So it observes `preventDefault`,
not the pin, and the pin-based non-discrimination the amendment warned about is
ruled out empirically rather than by argument.

The B2 full-suite round is the direct inversion of the pre-fix measurement in
section 4 above (`62 passed` with the branch deleted). The regression-detection
gap M2 identified is closed, and the closure is measured against the coverage
set, not against the case under construction (the L2 lesson, applied).

**Restoration verified:** `md5sum src/App.vue` back to
`c520a02f4c215ae86130ddb504d57f02` (identical to the pre-mutation backup),
`git diff --exit-code -- src/App.vue` exit 0, `git status --porcelain` empty
(also empty after the full gate run; `dist/` and `e2e/.generated/` are
gitignored), `dist/` rebuilt from restored source, whole suite `62 passed`.

### The discrimination premises in the new comment, checked at the artifact

The failure class M1 belonged to is a comment asserting more than the artifact
supports, so the new comment's own claims were verified rather than read:

| Claim in the added doc comment | Verified how |
| --- | --- |
| `onHelpKeydown` is the ONLY keydown listener in `src/` | `grep -rn "keydown\|keyup\|keypress" src/` returns exactly two lines, the add/remove pair at `src/App.vue:105` / `:111` |
| registered capture-phase | third argument `true` at both sites |
| its Escape branch preventDefaults nothing | `src/App.vue:81-89`, no `preventDefault` on that path |
| the click layer never sees a keydown | `onHelpClick` is registered for `"click"` only |
| the row count cannot attribute the closure (Enter on a button synthesizes a click the click layer stops) | round B2: every row-count assertion in case 9 passed with the whole keydown branch gone |
| the probe's own dispatch appends no rule | asserted in-test at `:269`, and that assertion is discriminating (the controls at `:257-261` show the count moves when a rule is appended) |

The `false`/`true` pair is mutually controlling, which is the property that makes
the witness safe against the structural-pass failure mode: a probe that could
only ever return `false` (the exact failure `cancelable: true` prevents) turns
`:305` red on correct code, and a probe that could only return `true` turns
`:265` red. Neither branch can pass vacuously without the other failing loudly.

## New breakage in the fix diff: none found

- `pnpm lint` exit 0; `pnpm exec tsc --noEmit -p e2e/tsconfig.json` exit 0 (the
  `!` non-null assertion the amendment prescribes is accepted by this config).
- `pnpm exec playwright test --grep "both activation channels are suppressed"`
  -> `1 passed`, selection exactly one test (`e2e/help-mode.spec.ts:221:3`), so
  the filter narrowed rather than silently running the suite.
- `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e` -> exit 0,
  `62 passed`, zero lines matching `failed` in the captured log,
  `check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s)
  gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), ...)` - the help
  id and catalog counts unchanged, so the zero-new-surface claim holds
  mechanically.
- Commit hygiene: one file, unsigned (`%GG` empty), repo trailer
  `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present.
- Typography: no non-ASCII in the added lines (pattern positively controlled
  against a real em-dash). No `file:line` citation in the added lines
  (`code-comment-line-citations-drift` conformant; pattern positively controlled
  against a synthetic `src/App.vue:63` reference). Anchors are symbol names and
  a ruling id.
- The cargo gate parts were not run: this commit touches one `.ts` test file,
  same ruling as `task-1-verdict.md` Q2.

## Deferred (out of scope for this loop, controller-side)

1. **L1, L2, L3 from the original verdict are untouched and stay open.** All
   three are controller-side; nothing in a test file could close them.
2. **The Nit stays as landed.** `:212`'s "in the I1 sibling's shape below" is
   still a positional cross-reference. The implementer's reasoning is correct:
   the I1 describe is at `:532`, still below, and this round inserted its helper
   above the D52 describe, so the phrase's target did not move. Rewording
   committed prose the verdict classified as a nit was rightly left alone.
3. **The cited ruling id does not resolve on this branch yet.** The comment
   anchors on `redundant-layers-need-mechanism-witness`; that entry exists in
   the main tree's `docs/decision-ledger.yaml:4251` (commit `1af9540`, tier 1,
   count 1) but that commit is not an ancestor of `plan75-a`
   (`git merge-base --is-ancestor` false), so the worktree's own ledger has zero
   occurrences of the id. Expected branch lag that self-heals on integration,
   recorded only so nobody reads the dangling anchor as a defect in the commit.
4. **`cancelable: true` was not fire-verified by flipping it.** That would have
   meant mutating `e2e/help-mode.spec.ts`, outside my permitted mutation scope.
   It is accepted on DOM semantics (`preventDefault()` is a no-op on a
   non-cancelable event) plus the mutual-control argument above, which makes the
   failure mode loud rather than silent if the flag were ever dropped.

## Disposition

Both Medium findings are closed at the artifact. The fix is additive within the
same test, changes no landed contract assertion, adds no production code and no
new surface, and leaves the gate green. Nothing in this round warrants another
fix loop.
