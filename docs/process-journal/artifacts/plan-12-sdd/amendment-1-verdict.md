# Amendment 1 verdict (independent review)

**Requirement compliance: MET on all ten (A-1 to A-10).**
**Quality: CHANGES REQUIRED before dispatch.** 0 Critical, 2 Important, 6 Minor.

The substance is right. The gate condition is correct, its state walk holds against
the tree, and every figure the amendment states that I could reproduce reproduced.
The two Important findings are both statements *about* artifacts rather than defects
in the design: one normative sentence that the same task falsifies, and one
no-work-needed conclusion that is false when run.

---

## 1. Requirement compliance, A-1 to A-10

| # | Verdict | Evidence I used |
|---|---|---|
| A-1 | **MET** | D112's opening paragraph in `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` and in the plan's Decision register states it in the ruling's own terms (kept: path line + parse error; hidden: empty-state paragraph + recents; "only before anything has been opened or created at all"), plus the accepted loss. Carried into the requirement table as **R43** with source `owner ruling 2026-07-31 (amendment 1)`. |
| A-2 | **MET** | One `computed` `nothingOpenedOrCreated = !model.value && currentPath.value === null`, fenced in Task 4 Step 4b and stated in D112 decision 1. I re-derived its ground: `openPath` sets `currentPath.value = path` and then `model.value = doc.profile ?? undefined` in one synchronous block after the awaited load; `createBlank` sets `currentPath.value = null` and then the seed in one synchronous block; those are the only two sites that write either. The report's 12-row state walk covers all five states the brief names as the minimum, plus IPC-reject, save-failure, post-failed-load New, undo/redo, and the bare mount harness. I checked the inverse direction independently: `model` is a `defineModel` with no parent binding (`App.vue` mounts `<EditorView v-show=... />` with no `v-model`), so no external writer can produce `model === undefined && currentPath === null` after a funnel ran. |
| A-3 | **MET** | New numbered decision **D112**, not an in-place edit of D107; D107 decisions 3(f) and 7 keep their text and gain dated supersession pointers in both documents. `proc-supersede-never-overwrite` is quoted verbatim - I diffed the quote against `docs/decision-ledger.yaml` and it matches word for word. Number re-derived independently by me at the parent commit with a *wider* pattern than the author's (`\bD(11[2-9]\|1[2-9][0-9]\|[2-9][0-9][0-9])\b` over the whole tracked tree at `411122f`): no hits, and my control line `ADR D112 and D250 and D999` matches. Slots match the file's own shape (Decision / Rationale / Rejected alternatives); I reproduced the omission ground myself: 110 D-sections across `docs/superpowers/specs/*.md`, 8 carrying `**Triggers created`. Supersession is expressed in the house form (inline dated pointer on the loser, successor names what it supersedes), same as plan-8's "Superseded in part (2026-07-27, ...)" and D111's "Supersedes:". |
| A-4 | **MET** | Task 4. The memo's "Task 5 is a natural vehicle" is refuted against Task 5's own EXHAUSTIVE Files list, which I confirmed lists `src/views/EditorView.vue (the dialog mount, the two guarded call sites)` and nothing about the recents surface. Task 4's `Read first`, Files list, new Step 4b, Step 6 case, Step 7 check P1 and `Must not decide` are all updated. The commit-block claim is a measurement I re-ran: Task 4's pathspec already contains `src/views/EditorView.vue` and `e2e/editor-undo-redo.spec.ts`, and D112 adds no catalog id, so neither `.ftl` moves. |
| A-5 | **MET** (see Minor 3) | Four rows, W2-m to W2-p. The rendered/hidden pair is graded on both sides: W2-n is the hidden side in the failed-load state, **W2-o is the rendered side in the pre-session state and is its own row**, which is exactly the half an implementation that hides the surfaces everywhere would otherwise pass without. P1 and P2 each carry the three parts: expression, a fire with an exact non-zero result (P1: the pre-state, which I reproduced at exactly 2 lines; P2: leg 1 at `editor-empty` visible and `editor-recents` count 1), and an end-state zero. |
| A-6 | **MET** | The producer ships in Task 4, the same task as the behaviour, with no infrastructure exemption claimed. I swept the 129 added lines for a deferral (`coverage follows`, `follows in a later`, `TBD`, `TODO`): no hits. |
| A-7 | **MET** | The amendment renders no new text. `editor-empty` (`locales/en/gui-editor.ftl`) and `batch-recents-heading` (`locales/en/gui-batch.ftl`) both already exist and keep their values. The budget arithmetic is byte-identical across the diff: `46 -> 54`, the running totals `49 / 51 / 54`, `15 new catalog ids across 3 catalogs` and `30 new catalog lines` all appear only as unchanged context or in lines whose only edit is elsewhere. |
| A-8 | **MET** | Conclusion confirmed with my own instrument rather than by re-running the author's. I extracted section 8.2 as its own region (`### 8.2 GUI` to `### 8.3`, 13 lines) and enumerated **the section's own vocabulary** instead of probing it with a recalled word list. Every hit: "recent profiles" as a bare capability in the editor item's list; "empty rule"/"empty track rule" about Add and the seed; "only while" governing the discard warning; "error" about the seed's severity and about the mkvtoolnix detection error; "render" about canonical YAML. No rendering condition on either surface and no mention of a failed or unparseable load anywhere in the section. **No spec edit is owed and none was made.** |
| A-9 | **MET** (the miss it did not cover is Important 1) | I recomputed both moved counts from the tables themselves with my own expressions: **43** requirement rows, 43 distinct ids, highest `R43`; **73** acceptance rows split `W1 10 / W2 16 / W3 22 / W4 23 / W5 2`, no duplicate ids. Both match. The `!model` sweep figures reproduce exactly: 11 lines across the two documents at `411122f` (9 plan + 2 ADR), 24 after (19 + 5). The typography zero reproduces with my own character class and my own fired control. S7 reproduces: no dangling reference to the parked decision exists in either document. |
| A-10 | **MET** | Seven tasks, unchanged in number and cut. The commit touches exactly two files, both under `docs/`, unsigned (`%G?` = `N`), one trailer. No file was added to any Files list and no path to any commit block. |

---

## 2. Quality findings

### Important

**I-1. D112 decision 1's closing sentence is falsified by the same task, two steps earlier.**

The ADR carries, unqualified:

> No gate in the view reads a bare `!model` afterwards.

and the plan's register carries "No other gate in the view reads a bare `!model`."

Task 4 Step 4, in the same task and immediately before Step 4b, prescribes:

> `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"` (the model term per D108 decision 10 ...)

and D108 decision 10, in both documents, is headed:

> **Undo and redo are gated on `model`**, in the functions and not only in the buttons. ... the buttons' `:disabled` carries the same term.

So the plan's own vocabulary calls exactly those two bindings a *gate* that reads `model`,
and D107 3(a) puts a third bare `!model.value` in `saveDisabled`. Under the document
family's own usage the sentence is false at the end of Task 4.

A narrow reading ("gate" = template render gate) rescues the plan-side sentence, because
"other" scopes it to the two surfaces just discussed and P1's zero covers that. It does not
rescue the ADR sentence, which drops "other" and is the one that ships in the append-only
normative file.

*Failure scenario:* Task 4's implementer reads Step 4, Step 4b and D112 decision 1 in one
sitting and hits a direct contradiction in normative text. The plan's Global Constraints
oblige a NEEDS_CONTEXT return rather than silent absorption, so the most likely cost is a
round-trip on a task that has not started. The worse branch is an implementer who takes the
sentence literally and tries to route the undo/redo `:disabled` through
`nothingOpenedOrCreated`, which is false in the failed-load state and would leave Undo
enabled over an empty editor - the exact state D108 decisions 9 and 10 exist to guard.

*Fix:* scope the claim to what P1 measures, in both documents - "no `v-if` in the view reads
a bare `!model`" - so the sentence and its producer have the same reach.

**I-2. The no-work-needed conclusion "there is no existing check part this could join
without adding a gate part" is false. I ran it.**

Report section 11 item 5 declines a standing guard for the single-definition property on
that ground, and D112's "What this leaves uncovered" paragraph settles for "the honest catch
for that is the diff review". The premise is refutable at one config line:

- `pnpm lint` (`eslint .`) is an existing gate part.
- `eslint-plugin-vue` is already a direct devDependency (installed 10.9.2) and
  `eslint.config.js` already carries a per-`.vue` `rules:` block.
- `vue/no-restricted-syntax` ships in that installed version
  (`dist/rules/no-restricted-syntax.js`).

Built outside the repository, with a selector targeting a `v-if` whose expression is a
unary `!` over `model`:

```
RED, against the tree as Task 3 left it:
/home/senol/Git/Muxsmith/src/views/EditorView.vue
  637:14  error  D112 probe: a bare !model render gate  vue/no-restricted-syntax
  644:14  error  D112 probe: a bare !model render gate  vue/no-restricted-syntax
2 problems (2 errors, 0 warnings)

GREEN, against a copy mutated to Task 4's end state (both gates rewritten to
nothingOpenedOrCreated, and three `:disabled="!model || ..."` bindings present
as the over-match control):
(no output)   exit=0
```

Both states are reachable, and the rule does **not** over-match the `:disabled` bindings
D108 decision 10 requires - which is also the scoping I-1 asks for, in executable form.

*Failure scenario:* the amendment ships a residual it names as uncovered, with the reason
recorded as "no vehicle exists". A later round re-inlines a bare gate, nothing turns red,
and the record says the matter was considered and closed. `proc-proposed-safeguard-stays`
exists because the argument for skipping a guard always takes exactly this shape.

*Fix, and the honest scope:* the amendment's "Documents only" boundary correctly forbade the
author from editing `eslint.config.js`, so the owed action was to **surface** the vehicle
(the report's "something ledger-worthy goes in your report" route), not to assert that none
exists. Replace the assertion with the surfaced item; the controller decides whether the
one-line rule lands in Task 4 or as a close action. (See the harvest item on the boundary.)

### Minor

**M-1. Task 4's `Read first` points at a locator that is not in the named file.** The added
text claims `e2e/smoke.spec.ts`'s recents describe carries "its
`editor-recents`/`editor-recent-profile` locators". Measured across `e2e/*.ts`:
`editor-recent-profile` 3 hits, all in that describe; **`editor-recents` 0 hits anywhere in
`e2e/`**. The testid exists in `EditorView.vue`, so the case is writable, but the pointer is
wrong about where the implementer will find it. The report repeats the claim ("the two
locators the new case needs").

**M-2. The report describes the artifact wrongly, and fences authored text as pasted output.**
Report section 6 states "**The fenced mock set is complete** and rests on measured harness
behaviour". Measured over the Step-6-to-Step-7 range of the plan: **zero** fenced blocks. The
mock set is prose carrying four angle-bracket placeholders (`<that path>`, `<the failing
path>`, `<a document carrying a profile>`, `<a document whose profile is null ...>`). That
specificity is *not* a latitude defect - Task 3 Step 7 and Task 4 Step 5 specify e2e fixtures
at the same level ("a `warnReport` carrying the measured seed diagnostic", "a `PICKED_PATH`
distinct from every other path literal"), so the amendment matches the plan's settled house
level and the implementer has `settingsWith`, `MKVMERGE_INFO` and the existing parse-error
fixture at `e2e/smoke.spec.ts` to copy. The defect is the description. Same class in section
8, where fenced blocks that read as pasted runs contain `<same expression>`, `<plan>`,
`<owner memo>` and `[the editor item]`. A fence should carry only what a re-run produces.
(I re-ran section 8's elided fire control: the memo returns 7 and the plan returned 3 at
`411122f`, so the figures were right.)

**M-3. W2-p's observable is broader than its producer.** The row reads "The pre-session
condition exists in exactly one place"; P1 measures "no `v-if` whose expression begins
`!model`". An implementer who inlined `v-if="currentPath === null && !model"` twice would
satisfy P1 at zero while falsifying the row. The stated residual names only the
spacing-after-the-bang variant, not term order or a differently-spelled duplicate. The
eslint selector in I-2 closes the term-order case too.

**M-4. Report concern 2 under-enumerates the tree it reports on.** The ROADMAP's round-3
finding-2 entry says "`currentPath` has ONE write site (`openPath`)". Task 3 falsified it
with **two** new sites, not one: `createBlank` (the `null` assignment) and `doSave` (the
newly established path). The report names only `doSave`. The concern is routed to a ROADMAP
close action, so the under-count would propagate into the correction.

**M-5. Report concern 1 over-enumerates.** It names three Task-4 cases that "build history
and then click Open": "Open resets", "A failed open clears rather than keeps" and "the
truncation case". The truncation bullet reads in full "undo once, then edit; Redo is
disabled" - no second Open, so Task 5's `pickAndOpen` guard cannot reach it. Two cases are
affected, not three.

**M-6. The residual D112 states is registered nowhere that will announce it.** D112 omits
`Triggers created` on a ground I reproduced (8 of 110 D-sections carry the slot), but the
residual it *does* state is structurally the same thing D110 recorded **as** a trigger ("The
CLI's `LOCALES` table carries the same unserved-locale gap and gets no equivalent check here.
Surfaced for later disposition, not fixed."). The plan close's known-inputs list gained
"amendment 1's own residue - the owner's failed-load ruling ... and D112's supersession of
two D107 clauses", and does not name the unwatched single-definition property. Between the
omitted slot and the omitted close-list entry, nothing carries it forward. Resolving I-2
resolves this too.

---

## 3. Observations (pre-existing, not created here, non-blocking)

- **The Task-4/Task-5 confirm collision is real and the plan locks the repair out.** Task 5's
  Files list is EXHAUSTIVE and does not include `e2e/editor-undo-redo.spec.ts`, so the task
  whose guard breaks two Task-4 cases is forbidden to fix them. Foreseeable NEEDS_CONTEXT at
  Task 5's dispatch. The amendment did not create it and its own case is immune.
- **A validation response can clobber a parse diagnostic.** `watch(model)` returns early on
  the `undefined` branch *without* incrementing `validationGeneration`, so a
  `validate_profile_model` response still in flight from a previous successful open resolves
  with `generation === validationGeneration` and overwrites `diagnostics.value` with its own
  (empty) list. `openPath`'s `opening` flag does not cover this, because the watcher's await
  outlives `openPath`'s `finally`. Not reachable in the prescribed mocked flow (leg 2's
  assertions drain it), and it equally affects Task 4's pre-existing "A failed open clears
  rather than keeps" case, which also asserts the diagnostic renders.
- **Both D-number instruments have the same hole and it does not change the answer.** The
  author's whole-tree pattern `\bD1(1[2-9]|[2-9][0-9])\b` cannot match D200 or higher, and
  the `docs/`-scoped expression that could was never run over the tree. Separately, the
  110-section denominator counts `^## D` and misses whole-file ADRs headed with a single `#`
  (D111's own file) - my `^#{2,3} D` had the identical hole until I probed for it. My wider
  run at `411122f` closes the first: nothing above D111 exists anywhere in the tracked tree.

---

## 4. Adjudication

**Q1 - the Task-4 cases and Task 5's `pickAndOpen` guard.** The reading is correct for two of
the three cases named, not three: "Open resets" and "A failed open clears rather than keeps"
both build history (which makes `dirty` true under D108 decision 4's formula) and then
perform a second Open through `pickAndOpen`, which Task 5 fronts with
`if (dirty.value && !(await confirmEl.value?.ask())) { return; }`, so the confirm fires and
the open never happens; the truncation case performs no second Open and is unaffected
(M-5). Nothing in either task answers it - Task 5's EXHAUSTIVE Files list excludes
`e2e/editor-undo-redo.spec.ts`, so the task that breaks those cases may not repair them, and
the collision surfaces as a red gate at Task 5's own pre-commit run. It is **entirely
pre-existing**: it is visible in the plan at `411122f`, and the amendment neither added nor
worsened an instance - its own case opens twice but never edits the model, so `savedSnapshot`
equals `history[position]` at both clicks, `dirty` is false, and the guard cannot fire. The
step says so and the claim checks out.

**Q2 - the ROADMAP's `currentPath` write-site claim.** The claim about the tree is correct and
understated: `docs/ROADMAP.md`'s round-3 finding-2 entry still says "`currentPath` has ONE
write site (`openPath`)", and the tree now has three assignments - `openPath`, `createBlank`
(the `null`) and `doSave` - so **two** new sites falsify it, not the one the report names
(M-4). The amendment's reasoning does **not** depend on that sentence anywhere: D112 decision
2's "nothing else can produce the combination" rests on the author's own fresh grep of
`currentPath.value =`, which I reproduced at exactly three hits and read by enclosing symbol,
and on `openPath`'s statement order, which I read in source. The dependency is sound because
it does not exist; the ROADMAP staleness is a close action and correctly stayed outside the
amendment's boundary.

**Q3 - D112 gating on `currentPath` after D107 moved duties off it.** Consistent, and
recorded rather than glossed. D107 decision 3 unloads `currentPath` from three things by name
- validation permission (b, moved to `sessionActive`), save-enablement (a and c) and the
pre-session gate (f, moved to `!model`) - and keeps two: the save target (d) and the path
display (e). D112 reads it only for the fact those two kept duties are about, "which file is
the editor bound to", and never as a proxy for content or permission; the `!model` term still
carries the has-content half, which the state walk's row 2 demonstrates (created and unsaved:
model set, path null, both surfaces hidden) - and that is precisely the property (f) was
written to obtain, so the defect (f) removed is not reintroduced. It *is* a genuine re-entry
into gate (f) specifically, as a second conjunct rather than a replacement, and D112 says so
in the open: D107 3(f) keeps its text with a dated "superseded in part by D112" pointer, and
D112 decision 5 states the duty split is preserved. Re-loading a term D107 unloaded would
mean using `currentPath` again for a question D107 took away from it; this uses it for the
one question D107 left with it.

---

## 5. Harvest

**The dominant pattern, and it is a compression failure rather than a measurement failure.**
Every figure this amendment states, I could reproduce: 43 requirements, 73 halves split
10/16/22/23/2, the `!model` sweep at 11 then 24, P1's pre-state at exactly 2, the D-number
ceiling at D111, 110 D-sections with 8 carrying the trigger slot, the typography zero, the
`e2e/mocks.ts` doc-comment quote word for word, the `proc-supersede-never-overwrite`
quotation word for word. Not one was wrong. What *was* wrong, five times, is the sentence
written **about** an artifact after the instrument was put down: "the fenced mock set" (there
is no fence), "its `editor-recents`/`editor-recent-profile` locators" (one of the two is
nowhere in `e2e/`), "ONE write site ... `doSave` now writes it" (two new sites), "three cases
... and the truncation case" (two cases), "no existing check part this could join" (there is
one, and it runs in the gate today). The lesson for a future author of this artifact class:
a measurement protects the number it produced and nothing else. The sentence that
*characterises* the artifact - fenced, complete, exactly one, the only place - is a second
claim and needs its own run.

**Repeated defect shape: the universal claim with the narrow producer.** I-1 and M-3 are the
same defect at two altitudes. A normative sentence said "no gate reads a bare `!model`" while
its only producer greps `v-if="!model`; an acceptance row said "exists in exactly one place"
while its producer greps the same string. The handle is readable at the keyboard: when you
write the claim and the check in the same paragraph, make them the same sentence. Where the
check is narrower, the check wins and the claim gets scoped down to it - never the reverse,
because the claim is what a later reader trusts and the check is what actually runs.

**Repeated defect shape: the tooling premise, weighed instead of run.** "There is no existing
check part this could join without adding a gate part" is a claim about `package.json`,
`eslint.config.js` and an installed plugin's rule list. All three are on disk, and running
them cost me one config file and two commands. Tooling premises are the cheapest class of
claim to verify and, in this document, the only one that was not. Worth a house handle: a
no-work-needed conclusion whose enabling premise is about what the toolchain can or cannot do
is verified by invoking the toolchain, not by reasoning about it.

**Report hygiene: a fence is a promise.** Sections 4, 8 and 9 mix genuine pasted output with
authored placeholders (`<same expression>`, `<plan>`, `[the editor item]`) inside the same
code fences. A reviewer cannot tell by looking which lines a re-run would reproduce, and the
brief's "every empirical claim is pasted from the run that produced it" is silently weakened.
Elisions and summaries belong outside the fence.

**Over-restriction in the brief, reported as the brief asks.** The "Documents only" boundary
was right for the plan and the ADR, and it was also the reason the author could not act on
the single real fork in the whole round. The residual D112 declares uncovered has a
one-config-line standing guard on an existing gate part; the boundary forbade touching
`eslint.config.js`; and the brief named no route for "a safeguard exists but lies outside my
edit surface" beyond the general "something ledger-worthy goes in your report". The author
filled that gap the way agents fill gaps - by concluding no vehicle exists, which reads as
analysis and is unfalsifiable in a document-only round. **A boundary that forbids an edit
should name where the finding goes instead**, in the same sentence. That one addition would
have converted an incorrect no-work-needed conclusion into a surfaced controller item at no
cost to the boundary.

**Second, smaller over-restriction.** A-2 demanded a state *walk* over the reachable states
and named "an open that failed on a fresh editor" among the minimum. A-5 demanded acceptance
rows in halves. Nothing joined the two, so the prescribed test drives the failed open only
*after* a successful one (leg 2 to leg 3) and never from the pre-session state - which is the
state the owner actually reported. I could construct no implementation that passes leg 3 and
fails that state, so this is not a finding; but the two requirements met at the walk and not
at the producer, and a brief that says "the states you walked are the states the case drives"
closes it for free.
