# Amendment 2 verdict (independent review)

**Requirement compliance: B-1, B-2, B-3, B-4, B-6 MET; B-5 NOT MET.**
**Quality: CHANGES REQUIRED. 0 Critical, 2 Important, 3 Minor.**

**The criterion derives.** That is the ruling B-1 asked for and it is earned, not
granted: I applied the rule cold against the file and reached the same three
members without reading the author's set. Its residual sits one level up and is
named below.

Instruments were built at `/tmp/.../scratchpad/amd2-werkbank/`, a path none of my
earlier passes used. Nothing in the repository was written except this verdict; no
stash, checkout, reset, restore or clean was run, and `git status --porcelain`
shows the same six paths of Task 5's uncommitted work, unchanged, after my pass.

---

## 1. Requirement compliance

| # | Verdict | Evidence |
|---|---|---|
| B-1 | **MET** | The criterion is written against the two functions Step 2 guards rather than against a control. I applied it cold (section 2) and derived the same three members. Its three supporting facts each check out: the guarded controls are exactly two in the template (`editor-new` -> `createBlank`, `editor-open` -> `pickAndOpen`, `editor-recent-profile` -> the unguarded `openPath`); the `dirty` rule matches D108 decision 4's formula; and the file contains exactly eight activations of the two controls, which I re-enumerated with a wider search than the author's (any mention of either testid, plus `.press`, `dispatchEvent` and variable-stored locators - the file does store locators in variables for `editor-undo` and `editor-rule-add`, so the risk was live, but not for these two). |
| B-2 | **MET** | The mechanism claim is true at the code, not just in the sentence. `src/views/EditorView.vue` mounts exactly one `<ConfirmDialog ref="confirmDialog">` behind one `useTemplateRef("confirmDialog")`, and both guard sites - inside `pickAndOpen` and inside `createBlank` - read `if (dirty.value && !(await confirmDialog.value?.ask()))`, i.e. the same instance. `src/components/ConfirmDialog.vue` carries `data-testid="confirm-dialog"` and `confirm-dialog-confirm`, so the locators are the same whichever control opened it. The "no additional mock" conclusion also holds: the `createBlank resets` case runs through the shared `openProfile` helper, which already mocks `validate_profile_model`. The stated ground for it is loose - see M-2. |
| B-3 | **MET** | The prescribed repair is purely additive - a visibility assertion and a confirm click inserted between the activation and the assertions that follow. The two repairs already applied in the working tree are insertions only (the diff against HEAD adds lines and removes none). The sentence now bans "removed, weakened, reordered **or reworded**", names the rejected alternative (re-establishing a baseline so the confirm never fires) and cites `proc-proposed-safeguard-stays` at the assertions themselves. |
| B-4 | **MET for the comment it names** | Step 4c fences the exact replacement, the Files list entry is widened to name that region, and Step 5 names both comments. I verified the fence executes: the OLD block appears verbatim in the file, the substitution applied to a copy produces a well-formed comment whose last fenced line hands the sentence back to the untouched "// currently-open-path line" continuation, and its figure is right (both catalogs measure 54 with Task 5's three ids present, 51 at HEAD, 46 + 3 + 2 + 3 = 54). The authority argument is sound and it is Task 5's own, not Task 4's inherited: Task 5 adds three ids to the catalog the sentence counts. Task 4's miss is marked rather than rewritten. **But the settlement created the mirror defect on the neighbouring comment - Important 2.** |
| B-5 | **NOT MET** | The five expressions are real, each carries a control, and the two instrument defects the author caught in itself (a backtick-blind pattern that missed the very site B-4 names, and a `&&` chain that swallowed a zero-match check) are exemplary self-reporting. But the sweep missed two statements the edit falsified and one it created: D109 decision 2's sensitivity clause in both documents, Task 5 Step 4's Case 4 parenthetical, and Step 5's own new cross-reference. B-5's entire content is "anything else your edit falsifies", and three sentences qualify. |
| B-6 | **MET** | Seven tasks, unchanged in number and cut. Task 5's Files list gained no file - one entry was widened, one entry's case list grew inside a file it already owned - and I measured it: six entries, six pathspec paths, identical, so Step 5's "exactly the six files" stays true. No design decision reopened; the implementer's Option C (narrowing `createBlank`'s guard) is not taken, and the implementer's own memo already classes it as "not a real option" for the same reason. |

---

## 2. The criterion, applied cold

I did not read the author's classification table before doing this.

**The rule as written:** a case is a member iff it activates a control bound to
`pickAndOpen` or `createBlank` at a moment when `dirty` is true.

**Step one, every activation, searched wider than the rule's own phrasing** - any
occurrence of `editor-new`, `editor-open` or `editor-recent-profile` anywhere in
the file, not only in the `getByTestId(...).click()` spelling:

```
99   editor.getByTestId("editor-open").click();      <- inside the shared openProfile helper
390  editor.getByTestId("editor-open").click();
400  editor.getByTestId("editor-open").click();
429  editor.getByTestId("editor-new").click();
472  editor.getByTestId("editor-open").click();
486  editor.getByTestId("editor-open").click();
630  editor.getByTestId("editor-open").click();
638  editor.getByTestId("editor-open").click();
```

Eight, and no other activation path exists: no `.press()` on either control, no
`dispatchEvent` on them, and no variable-stored locator for either - though the
file does store locators in variables for other controls (`const undoButton = ...`,
`const add = ...`), so the blind spot was real and simply unoccupied here.

**Step two, mutations:** `pattern.fill` at 118, 289, 303, 305, 339, 344, 359, 393,
426, 475; `selectOption` at 145; `dispatchEvent("drop")` at 207; `editor-rule-add`
at 234, 325-326, 524, 561; `editor-rule-remove` at 260.

**Step three, my classification:**

| site | case | mutation since the nearest baseline | member |
|---|---|---|---|
| 99 | `openProfile` helper | no model yet, `savedSnapshot` still null | no |
| 390 | open resets, first Open | none | no |
| **400** | **open resets, second Open** | the fill at 393 | **yes** |
| **429** | **createBlank resets, the New click** | the fill at 426 | **yes** |
| 472 | failed open, first Open | none | no |
| **486** | **failed open, second Open** | the fill at 475 | **yes** |
| 630 | D112 leg 2 | the case never mutates | no |
| 638 | D112 leg 3 | the case never mutates | no |

**Three, and the same three.** The plan's per-case exclusion statement is also
accurate against the file: the six mutation-path cases mutate after one helper
open and never activate a guarded control again; granularity, truncation, the
depth cap and U1 add no activation of their own; "save marks rather than clears"
fills at 359 and saves at 361, re-establishing the baseline, and stops there; the
D112 case contains no mutation between 598 and the end of the file.

### Does it derive, or does it list?

**It derives.** The old rule named a surface property (a second Open click) and had
no reference to the mandate at all, so a case entering through the other guarded
control was not overlooked but unexpressible. The new rule names the guarded
functions - what Step 2 actually does - and enumerates the current members of that
set as a measured fact beside it. Run it forward: a new case that clicks New after
edits is caught; one that clicks Open after edits is caught; one that clicks a
recents entry after edits is correctly excluded, because `openPath` is deliberately
unguarded and D112's condition makes that surface unreachable while a model is
held; one that activates either button by keyboard is caught, because the rule says
"activates a control", not "calls `.click()`". None of those requires a judgement
call. The one place it leaves method open - "list every activation of those two
controls in the file" - is a strength rather than a gap: fencing a single
expression there would rebuild the narrower-than-the-mandate defect inside the
instrument.

**Its residual is one level up, and it is not hypothetical.** The rule says "those
two functions" and asserts alongside it "and on neither `openPath` nor anything
else" - a closed-world claim about Step 2's guard set. If that set changes, the
criterion goes stale exactly as its predecessor did. The plan already contains the
one documented event that would do it: D109 decision 2's sensitivity clause, which
says that if the owner strikes it, `createBlank` is called unconditionally. Neither
sentence points at the other. That is Important 1 and Minor 1, which are the two
halves of that unwired seam.

---

## 3. Quality findings

### Important

**I-1. D109 decision 2's sensitivity clause is now false in both documents, and so
is Task 5 Step 4's Case 4 parenthetical.** The clause reads:

> Sensitivity, stated so it can be struck cleanly: if the owner strikes it, the
> only change is that `createBlank` is called unconditionally and one test moves
> from asserting the confirm to asserting its absence; nothing else in the package
> depends on it.

and Case 4 carries "(If the owner strikes D109 decision 2, this case inverts to
asserting no dialog; nothing else moves.)". After amendment 2, striking that
decision moves **two** tests, not one - Task 5's own case 4 in `e2e/smoke.spec.ts`
**and** the `createBlank resets` case in `e2e/editor-undo-redo.spec.ts`, whose
Step 4b repair exists only because `createBlank` is guarded - and it also moves
Step 4b's guarded-function pair from two to one and its derived set from three to
two. "Nothing else in the package depends on it" is now false: the criterion does.

*Why it matters, concretely:* the clause exists so the owner can strike a decision
cleanly. Acting on it as written leaves a test asserting a confirm that no longer
appears - a red gate met at code contact, which is the identical failure amendment
2 was written to end, one decision over. The clause also sits in the append-only
ADR, so the longer it stands the more expensive it is to correct.

*Why the sweep could not see it:* E1 keyed on "two"-shaped phrases and E2 on the
spec filename. This sentence carries neither - it says "one test" and "nothing
else" - so both expressions were structurally blind to it, which is the same
shape as the defect the amendment repairs.

**I-2. The settlement removed the only live instruction for the other budget
comment and replaced it with a citation to a step that does not contain it.**
Before this round, Task 5 Step 5 read "recompute and correct the budget comments" -
vague as to file scope, which is the defect B-4 named, but a live instruction
covering both. It now reads:

> **The `gui-editor.ftl` recount must be 54, and the budget comments it governs are
> exactly two, both of which this task corrects**: `e2e/smoke.spec.ts`'s (Step 4)
> and `src/views/EditorView.vue`'s header sentence (Step 4c).

Measured: Task 5 Step 4 contains no mention of a budget comment, a recount, or the
figure 54 - zero hits for `budget|comment|recount|recompute|54` over the whole
Step-4-to-Step-4b range. `src/views/EditorView.vue` now has a precise fenced step;
`e2e/smoke.spec.ts` has a Files-list entry that names the region ("**the
catalog-budget comment**, recomputed") and no step that instructs it. That is
exactly the pair B-4 identified as the cause, reproduced in mirror image on the
other file.

*What softens it and what does not:* Task 5's implementer already corrected that
comment in the working tree (51 -> 54 at `e2e/smoke.spec.ts`), so no stall follows
this run. The plan is the durable artifact, though, and on a re-read or a redo the
instruction is gone while Step 5 asserts it was carried out.

*Fix:* give Step 4 the one sentence Step 5 now credits it with, or point Step 5 at
itself for that half.

### Minor

**M-1. The criterion does not name its own staleness condition.** It fixes the
defect one level down and is silent about the level it still has: if Step 2's
guarded set changes, the enumeration inside the rule ("those two functions", "and
on neither `openPath` nor anything else") is falsified. One clause in Step 4b
pointing at D109 decision 2's sensitivity clause - and the reciprocal pointer there
- wires the seam shut. This is the forward half of I-1.

**M-2. B-2's stated ground is looser than its conclusion.** "After a confirmed New
`createBlank` completes with no further IPC" is not accurate: the model assignment
queues `watch(model)`, which invokes `validate_profile_model`. The conclusion
survives, because the operative word is *additional* and the shared `openProfile`
helper already mocks that command (measured). The risk is that the plan directs
NEEDS_CONTEXT "if the implementer finds that it does" need a mock - an implementer
reading the ground rather than the claim could raise one against a call that is
already covered.

**M-3. One per-case exclusion is imprecise.** "granularity, truncation, the depth
cap and U1 activate no guarded control at all" - each of the four calls
`openProfile`, which clicks `editor-open` at line 99. The helper's activation is
classified correctly in the same sentence immediately before, so a reader applying
the criterion lands right; but the per-case statement B-1 asked to be precise is
wrong as written on four of its members. "no guarded control beyond the shared
helper's own Open" is the accurate form.

---

## 4. The 43-figure enumeration

Derived from a bare `\b43\b` sweep over `src/`, `e2e/`, `crates/`, `src-tauri/` and
`scripts/` rather than from a vocabulary guess, with a fired control. **Four live
sites, and they count two different sets. Two hold; two do not.**

| # | Site | What the figure counts | Status |
|---|---|---|---|
| 1 | `src/editor/registries.ts`, header: "42 of the 43 fields are `EditableField`; the sole `FixedField` is `Profile.profile_version`" | **field specs across the 13 registries** | **HOLDS.** Measured: 42 `labelKey:` entries + exactly 1 `fixed: true` entry = 43, and the single fixed entry is `profile_version`. The "13 structs" claim in the same block also holds - 13 `Record<keyof T, FieldSpec>` exports. |
| 2 | `src/editor/registries.ts`, header: "Widget choices come straight from the 43-row table in D45" | **rows in D45's design table**, which is the same set as (1) by construction | **HOLDS** on the tree side: the figure it must agree with is 43 and the tree measures 43. The design table itself is a dated document I did not re-count row by row. |
| 3 | `src/editor/widgets/SelectWidget.vue`: "gui-editor.ftl stays at its 43 label keys (D45's own constraint)" | **keys in `gui-editor.ftl`**, phrased as label keys | **STALE, on either reading.** The catalog carries **54** today (51 at HEAD, 46 before Plan 12); the registry-referenced label keys number **42**. Its historical basis is genuine - the session-16 close handoff records "carries 43 keys in Plan 6 (42 labels + 1 save-surface note)" - so 43 was the whole-catalog figure then and the word "label" was already loose. Present tense in live source makes it a live claim. |
| 4 | `src/editor/widgets/StringListWidget.vue`: "no generic 'add'/'remove' chrome text exists in gui-editor.ftl's 43 keys (D45 forbids growing it)" | **keys in `gui-editor.ftl`** | **STALE, and wrong on its substance as well as its figure.** The catalog is 54, and the premise the sentence rests on is false: `editor-action-add` and `editor-action-remove` - the generic Add/Remove chrome - exist in `locales/en/gui-editor.ftl` today, added by D59's 45 -> 46 revision. The widget's design choice (a comma-separated textbox rather than per-item rows) may still be right, but its stated ground has been gone since plan 7.5. |

**Not live, correct as history, nothing owed:** `docs/ROADMAP.md`'s two 43-figures
sit inside one dated, closed entry that scopes itself explicitly ("carries 43 keys
in Plan 6 ... (2026-07-16, plan-6 plan review)"); `docs/process-journal.md` and the
two session-16 handoff artifacts carry the same figure as dated record.

**Routing note.** Site 4 is the one worth a vehicle rather than a correction: a
stale count is cosmetic, a stale premise under a design decision is not. Both
widget files lie outside every Files list in Plan 12, so neither is this
amendment's to fix, and the author was right to surface rather than to conclude no
vehicle exists.

---

## 5. Standing dimensions

- **Typography:** 0 hits over the 53 added lines with my own dash/quote/ellipsis/
  NBSP class; my synthetic control returns 1.
- **Latitude, both forms:** no explicit permission and no placeholder in the added
  lines (control fires). The omission form is clean too: Step 4b's criterion names
  a property rather than a spelling; Step 4c fences both the text to remove and the
  text to insert and I verified the removal target matches the file verbatim; the
  `Must not decide` list names the criterion, the three cases, the identical-repair
  ruling and the fenced replacement.
- **No-work-needed claims, run rather than weighed:** "the acceptance map gains no
  row" - W4-a to W4-c grade the Open path and W4-e grades "New over unsaved changes
  warns the same way", so the repair genuinely produces no new observable;
  "Task 5's Files list gained no file" - six entries, six pathspec paths, identical;
  "no additional mock" - `openProfile` already mocks `validate_profile_model`.
- **Counts:** 43 requirement rows, 73 acceptance rows, no duplicate ids, unchanged;
  the plan's own gate-part audit returns 1 line, its own sentence.
- **House conformance:** no self line-number citation in the added lines; the
  decisions document was correctly left untouched, since nothing normative changed
  and an append-only ADR is not where a plan-level membership rule belongs;
  `proc-supersede-never-overwrite`'s spirit is followed on Task 4's closed step,
  which is marked "NOT DISCHARGED IN FULL" rather than rewritten.
- **Commit hygiene:** one tracked document, unsigned, one trailer, and Task 5's six
  uncommitted paths intact before and after.

---

## 6. Harvest

**The round's pattern, and it is the plan's own defect class walking one seam
further each time.** Amendment 1 scoped a claim wider than its check. Its fix round
scoped a claim wider than its instrument's enumerated set. Amendment 2 repairs a
criterion scoped narrower than its mandate - and in the same edit leaves the one
sentence that describes how that mandate could change (D109's sensitivity clause)
pointing at a world with one guarded function fewer. The shape is constant: **two
sentences describe the same set, one is edited, and the edit does not walk to the
other.** What makes it survive review each time is that the two sentences share no
vocabulary - "the affected set is exactly two" and "one test moves from asserting
the confirm" are about the same fact and have no word in common, so every
term-derived sweep is structurally blind to the second.

**The reusable handle, because it is mechanical.** When you edit a set, do not only
grep the set's own words - **grep the set's dependents by their FUNCTION**: every
sentence that says what happens *if* the set changes (a sensitivity clause, a
strike-cleanly note, a "nothing else depends on this"), and every sentence that
counts a consequence of the set (how many tests move, how many files are touched).
Those are the sentences a term-derived sweep cannot reach, because they describe
the set without naming it. In this repo the readable trigger is the phrase family
"if the owner strikes", "nothing else moves", "the only change is" - three
expressions, all greppable, none of which any of the three amendment sweeps ran.

**Second item, on the shape of a good criterion.** The one thing amendment 2 got
exactly right is worth keeping as the model: it states the membership rule against
the *mandate* (the functions Step 2 guards), states the current members of that
mandate as a *measurement* beside it, and leaves the search *method* open. That is
the right factoring - the rule cannot go stale from a new case, only from a new
mandate, and the mandate is one screen away in the same task. The improvement it
still owes is one clause naming that last boundary, which is cheap and which this
plan has now paid for twice.

**Carried forward, outside this diff and still unrouted:** the validation-response
race (`watch(model)` does not increment `validationGeneration` on the `undefined`
branch), and the ROADMAP `currentPath` write-site correction from amendment 1's fix
round. Both are correctly reported as controller items in the author's concerns.
