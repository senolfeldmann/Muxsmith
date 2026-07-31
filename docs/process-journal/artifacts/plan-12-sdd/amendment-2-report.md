# Amendment 2 report: a criterion rewritten against its mandate, and one stale comment placed

**Status:** DONE_WITH_CONCERNS (three concerns, section 8; two are controller items
outside a documents-only edit surface, one is a finding against the brief's own
corroboration).
**Commit:** `dceca9b5fa5368c22186c06a2254835213cdb6cf`, **one** tracked document,
one trailer, unsigned, not pushed.

```
$ git show --stat --format= HEAD
 .../2026-07-30-plan-12-qa-round-3-findings.md      | 64 ++++++++++++++++++----
 1 file changed, 52 insertions(+), 12 deletions(-)

$ git log -1 --format='%(trailers)'
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format='%G?'
N
```

**The tree was not mine and it survived.** Task 5's uncommitted work was in place
before I started and is in place after the commit, unstaged and unmodified. I
staged one path explicitly and committed with that same pathspec; nothing was
stashed, checked out, reset, restored or cleaned, and no path outside `docs/` was
written at any point.

```
$ git status --porcelain          # after my commit
 M e2e/editor-undo-redo.spec.ts
 M e2e/smoke.spec.ts
 M locales/de/gui-editor.ftl
 M locales/en/gui-editor.ftl
 M src/views/EditorView.vue
?? src/components/ConfirmDialog.vue

$ git diff --cached --stat        # the index, immediately before the commit
 .../2026-07-30-plan-12-qa-round-3-findings.md      | 64 ++++++++++++++++++----
 1 file changed, 52 insertions(+), 12 deletions(-)
```

**The decisions document was not edited, and that is a decision rather than an
omission.** Nothing normative changed: D109's guard shape, D112's condition and the
catalog budget all stand exactly as they are. What this amendment corrects is a
plan-level membership rule, a task-scope disagreement and one stale in-tree
comment - none of them an architectural decision, so none of them belongs in the
append-only ADR file.

---

## 1. B-1: the criterion, and why it is deterministic

**What was wrong.** Step 4b's membership rule was "builds history, then opens
again". It named a *control* (a second Open) where the mandate is a pair of
*functions*. `createBlank` is guarded by the same Step 2 and is reached through a
different control, so a case that replaces the editor's content through New was
outside what the rule could express - not overlooked, but structurally invisible.

**The criterion now written into Step 4b**, derived from what Step 2 guards:

> A case is a member iff it activates a control bound to `pickAndOpen` or
> `createBlank` at a moment when `dirty` is true.

Three supporting facts make it decidable by reading, and each is measured rather
than asserted:

1. **The controls are exactly two**, read from the template rather than recalled:

```
$ grep -nE 'data-testid="editor-(new|open|recent-profile)"|@click="(createBlank|pickAndOpen|openPath)' src/views/EditorView.vue
893:      data-testid="editor-new"
895:      @click="createBlank"
901:      data-testid="editor-open"
904:      @click="pickAndOpen"
955:            data-testid="editor-recent-profile"
957:            @click="openPath(path)"
```

   `editor-recent-profile` is bound to `openPath`, which Task 5 deliberately does
   not guard, so a recents click is never a member.

2. **`dirty` is true at a click iff a model mutation lies between it and the most
   recent baseline** (a successful open, a create, or a successful save, per D108
   decisions 3 and 8).

3. **The derivation is therefore mechanical:** list every activation of the two
   controls, read backwards to the nearest baseline, ask whether a mutation lies
   between.

**Why this is deterministic where the old rule was not:** a third party runs one
search for the two controls and then reads a fixed question at each hit. There is
no step where a reader has to decide which entry points "count" - the guarded
functions decide it, and the plan states them. A criterion narrower than the
mandate it serves regenerates the defect on the next addition, which is what
`a-normative-claim-is-scoped-down-to-its-producers-reach` is about one level up,
and it is the same defect shape amendment 1's fix round already corrected once
(I-1: a claim wider than its producer; this is its mirror, a producer's scope
standing in for the claim).

## 2. My own re-derivation of the set

**Measured against the file as it stands** (the working tree, which carries Task
5's uncommitted repairs to two of the three - the criterion classifies a case by
what it does, not by whether the repair is already applied).

Step one, every activation of the two guarded controls:

```
$ grep -nE 'getByTestId\("editor-(new|open)"\)\.click\(\)' e2e/editor-undo-redo.spec.ts
99:  await editor.getByTestId("editor-open").click();
390:    await editor.getByTestId("editor-open").click();
400:    await editor.getByTestId("editor-open").click();
429:    await editor.getByTestId("editor-new").click();
472:    await editor.getByTestId("editor-open").click();
486:    await editor.getByTestId("editor-open").click();
630:    await editor.getByTestId("editor-open").click();
638:    await editor.getByTestId("editor-open").click();
```

Fire controls for that expression, both directions: it returns 0 against
`await editor.getByTestId("editor-save").click();` (it discriminates a guarded
control from an ordinary one) and 1 against
`await editor.getByTestId("editor-new").click();` (it fires on a real member).

Step two, mutations and activations interleaved in file order, so "is there a
mutation between this click and its baseline" is answered by reading one list:

```
$ grep -nE 'getByTestId\("editor-(new|open)"\)\.click\(\)|\.fill\(|getByTestId\("editor-rule-(add|remove)"\)\.click\(\)|^  test\(|^async function' e2e/editor-undo-redo.spec.ts
81  async function openProfile(
99  editor.getByTestId("editor-open").click();
111  test("setFieldValue: a top-level field edit is one undo step", ...
118  pattern.fill("^changed$");
...
369  test("open resets: opening a second profile clears both Undo and Redo", ...
390  editor.getByTestId("editor-open").click();
393  pattern.fill("edited-a");
400  editor.getByTestId("editor-open").click();
422  test("createBlank resets: New after edited history clears both Undo and Redo", ...
426  pattern.fill("edited");
429  editor.getByTestId("editor-new").click();
454  test("a failed open hides the editing surface and the Undo/Redo buttons read disabled ...
472  editor.getByTestId("editor-open").click();
475  pattern.fill("edited");
486  editor.getByTestId("editor-open").click();
598  test("three legs in one flow: nothing opened yet, a successful open, then a failing open", ...
630  editor.getByTestId("editor-open").click();
638  editor.getByTestId("editor-open").click();
```

(The elided rows are the mutation-path, granularity, truncation, save-marks,
depth-cap and U1 cases; none of them contains an activation of either guarded
control, which is what the full run shows and what puts them outside the set.)

Step three, the classification, one row per activation:

| site | enclosing case | mutation since the last baseline? | member |
|---|---|---|---|
| 99 | the shared `openProfile` helper | no model exists yet | no |
| 390 | open resets, first open | none since the test began | no |
| **400** | **open resets, second open** | the fill at 393 | **YES** |
| **429** | **createBlank resets, the New click** | the fill at 426 | **YES** |
| 472 | a failed open, first open | none since the test began | no |
| **486** | **a failed open, second open** | the fill at 475 | **YES** |
| 630 | D112 leg 2 | the case never mutates the model | no |
| 638 | D112 leg 3 | the case never mutates the model | no |

**Three members**, which confirms both the controller's corroboration and the
implementer's independent derivation, reached here from the rewritten criterion
rather than from either list. The plan now carries the criterion, the three
members with the reason each qualifies, and a per-case statement of why every
other case in the file fails it - stated per case rather than by exclusion, so a
reviewer can check it without re-reading the file.

## 3. B-2: the repair pattern for the added member

**Identical, and the ground is a property of Step 2 rather than of the two
controls.** Step 2 mounts **one** `ConfirmDialog` in `EditorView`, and both
guarded functions await that same instance's `ask()`. The dialog that appears
therefore carries the same `confirm-dialog` and `confirm-dialog-confirm` testids
whichever control opened it, so the two-line insertion is the same insertion.

**The one real difference between the entry points does not reach the repair**,
and the plan says so rather than leaving it to be discovered: after a confirmed
Open the flow continues into the file dialog, which those cases already mock,
while after a confirmed New `createBlank` completes with no further IPC. So the
New case needs no additional mock, no additional wait and no different locator.
If an implementer finds otherwise, the plan directs NEEDS_CONTEXT rather than an
invented locator.

## 4. B-3: no assertion weakened

The plan's repair sentence now reads "no existing assertion in any of the three is
removed, weakened, reordered **or reworded**, and no case changes what it is
about", and it names the alternative repair that was rejected and why:
re-establishing a baseline before the guarded click would make the confirm never
fire, which dodges the mechanism by swapping the case's own subject.
`proc-proposed-safeguard-stays` is cited at that sentence, because the assertions
those cases already carry are themselves the safeguard.

## 5. B-4: the ownership question, settled

**Settlement: the correction lands in Task 5**, as a new fenced **Step 4c**, with
`src/views/EditorView.vue`'s Files-list entry widened to name that region and
Step 5 rewritten so the step and the Files list agree.

**The authority is Task 5's own, not Task 4's inherited duty.** Task 5 adds three
ids to `gui-editor.ftl`, which falsifies that file's own sentence about how many
the catalog carries - the identical ground Task 3's Files list gives for the two
comment regions it repaired, and what `proc-normative-count-recomputed` exists to
prevent. That matters because Task 4's duty cannot be delegated forward: a closed
task's step is a record, not a live instruction.

**What it becomes, measured first.** The count is recomputed from the catalog, not
copied from the plan:

```
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl
54
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/de/gui-editor.ftl
54
```

(Both figures include Task 5's three uncommitted ids, which is the state Step 4c
runs in.) The stale sentence, read from the tree:

```
$ grep -n "carries 49 ids" src/views/EditorView.vue
67:// packages did add to it: `gui-editor.ftl` carries 49 ids today, three of
```

Step 4c fences both the exact three lines to replace and the exact seven that
replace them, stating 54 and decomposing the eight ids this view's own affordances
contributed (3 profile-creation + 2 undo/redo + 3 discard), with 46 + 8 = 54
against the pre-package count the authoring section measured. A recount that
disagrees is a finding, not a fence to adjust.

**Task 4's miss is on the record rather than quietly repaired**, in two places and
in neither case by rewriting the closed step: Task 4's Step 7 gains a marked
"NOT DISCHARGED IN FULL" note stating what was required, what was done, and that
both its review and its delta review graded the step MET; and Task 5's Files-list
note carries the same fact where the authority is argued. The plan close's
verdict-harvest inputs gain it as a **review-instrument** finding rather than an
implementer one - both reviews graded the step and neither opened the file the
step named.

## 6. B-5: the sweep

Terms derived from the artifact - the enumeration's own words, the file it names,
the comment's own text - rather than from memory of what I wrote.

| # | Expression | Result and disposition |
|---|---|---|
| E1 | `two repaired\|two Task-4 cases\|exactly two\|the two cases\|both cases\|two of the three` | 7 hits. Two were mine and were corrected (Step 5's "two repaired cases", the safeguards paragraph's "each of the two repaired Task-4 cases"). One is mine and correct as written ("the controls are exactly two"). Four are D109 decision 7's "exactly two choices" in both documents and P1's "exactly two lines" - unrelated senses, left alone |
| E2 | `editor-undo-redo\.spec\.ts` | 11 hits, every one read: Task 4's Files list and commit block, Task 5's Files list and commit block, Step 4b, Step 5, the sequencing file-overlap sentence, Task 4's Steps 5 and 6. Two carried the two-case set and were corrected; the rest are unaffected by a change in the set's size |
| E3 | `budget comment\|carries 49\|stale .{0,20}(comment\|count)\|catalog-budget` | 11 hits: Task 4's Step 7 (marked, not rewritten), Task 5's Files list, its new note, Step 4c, Step 5, Task 3's and Task 4's `smoke.spec.ts` entries, correction 5 in the corrections table, the close list. All reconciled or correct as they stand |
| E4 | the counts my edit could move | **unchanged**, recomputed rather than assumed: 43 requirement rows, 73 acceptance rows, Task 4's Files list 7 files, Task 5's 6 files. Task 5 stays at six because B-4 widened an existing entry rather than adding a file, so Step 5's "exactly the six files" stays true |
| E5 | the plan's own gate-part-count audit, re-run because this round added prose | **1 line, the audit's own sentence.** Unlike amendment 1's fix round, this round's prose introduced no new match |

**Fired controls.** E5's alternatives were fired individually (`11 parts` -> 1,
`an eleven-part gate` -> 1, `the gate runs in order` -> 0). E1 to E4 all returned
non-empty results, which is their own evidence that the patterns match. The
typography check is the only zero-result claim and carries its own control:

```
$ git diff -U0 -- docs/ | grep '^+' | grep -cP '<the dash/quote/ellipsis/NBSP class>'
0        (exit 1)
$ printf '<a line carrying an em-dash, curly quotes and an ellipsis>' | grep -cP '<same class>'
1        (exit 0)
```

(The class and the synthetic line are written out here rather than pasted, because
reproducing them literally would put the very glyphs this document forbids into
it. The expression is the same one amendment 1's rounds used.)

**One instrument defect of my own, caught and recorded rather than papered over.**
My first B-4 search used `gui-editor\.ftl (carries|stays)` and **missed the very
site B-4 names**, because the comment writes the filename in backticks
(`` `gui-editor.ftl` carries ``) and the pattern demanded a bare space. The
re-derivation used a backtick-tolerant expression, and the union of the two runs is
what section 8's concern 2 reports. The lesson is the same class this plan already
carries: an expression whose literal comes from how you would write the phrase,
rather than from how the artifact writes it, produces a false absence.

**A second, smaller one:** a `&&` chain of checks silently stopped at a
zero-match `grep -c`, which exits 1, so the typography check in that chain never
ran and its absence looked like a pass. Re-run standalone, with its exit status
printed. Recorded because "the check did not run" and "the check found nothing"
are indistinguishable in a chained shell command, which is the same failure mode
as an unfired absence check one level down.

## 7. B-6: no task added, removed or re-cut

Seven tasks, unchanged in number and in cut. Task 5's Files list gained no file -
one existing entry was widened with a named region, and one entry's case list grew
from two to three within a file it already owned. No work moved between tasks, and
no design decision was reopened: D109 decision 2 (the guard sits on `createBlank`
as well as `pickAndOpen`) is the premise this amendment serves, not one it
revisits. Option C in the implementer's memo - narrowing `createBlank`'s guard -
would have reopened it and is rejected in the plan text by construction, since the
criterion is written against the guard rather than against what would be
convenient for the tests.

## 8. Concerns

1. **The brief's second corroboration is not exact, and the difference is a
   controller item.** It states that a wide search for a catalog-budget figure
   "returns exactly the one site named in B-4". Mine returns that site plus a
   second live one with a *different* claim:

   ```
   $ grep -n . src/editor/widgets/SelectWidget.vue | sed -n '5p'
   5:// gui-editor.ftl stays at its 43 label keys (D45's own constraint).

   $ grep -oE 'labelKey:\s*"[a-z][a-z0-9-]*"' src/editor/registries.ts | sort -u | wc -l
   42
   ```

   That comment is about D45's *label-key* constraint rather than the catalog
   total, and it is off by one against the registry today. It is a Plan-6-era
   statement in a file no task in Plan 12 touches, so it is **outside this
   amendment's edit surface and outside every Files list in the plan** - reported
   here for routing, not converted into a claim that no vehicle exists. Two
   further hits are dated design-document statements ("carries 43 keys in Plan 6")
   in an archived spec, which are correct as history and need nothing.
2. **The validation-response race reported at amendment 1's fix round is still
   unrouted** (`watch(model)` returns early on the `undefined` branch without
   incrementing `validationGeneration`, so an in-flight response can overwrite a
   parse diagnostic). Unchanged by this amendment, still a product defect outside
   a documents-only surface, still owed a routing decision.
3. **The ROADMAP correction from amendment 1's fix round is still owed** as a
   close action, with the corrected figure: Task 3 added two `currentPath` write
   sites, so the tree carries three assignments in total.

---

# Fix round 1 (2026-07-31)

**Status:** DONE_WITH_CONCERNS.
**Commit:** `d4c8401fe7a33689a4fe69fb97a7914ed3bafeac`, two tracked documents, one
trailer, unsigned, not pushed, on top of `dceca9b`.

```
$ git show --stat --format= HEAD
 .../2026-07-30-plan-12-qa-round-3-findings.md      | 30 ++++++++++++++++------
 .../specs/2026-07-30-plan-12-decisions.md          |  2 +-
 2 files changed, 23 insertions(+), 9 deletions(-)

$ git log -1 --format='%(trailers)'
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
$ git log -1 --format='%G?'
N
```

**Task 5's uncommitted work is intact, again.** Six paths before, the same six
after; nothing stashed, checked out, reset, restored or cleaned; nothing outside
`docs/` written; the index held exactly the two documents at commit time.

```
$ git status --porcelain          # after the commit
 M e2e/editor-undo-redo.spec.ts
 M e2e/smoke.spec.ts
 M locales/de/gui-editor.ftl
 M locales/en/gui-editor.ftl
 M src/views/EditorView.vue
?? src/components/ConfirmDialog.vue
```

The verdict is at `.superpowers/sdd/plan-12/amendment-2-verdict.md`. B-5 came back
NOT MET; both Important findings are its consequences, and both are fixed here
together with the three Minors and the controller's ruling on the criterion's
staleness boundary.

## I-1: the neighbours that describe the set without naming it

**The three phrases the reviewer named, run as instructed rather than only my own
term-derived expressions.** This is the run that finds what three rounds of sweeps
could not, and it is pasted because the finding is that it was never made:

```
$ grep -rn 'if the owner strikes' docs/superpowers/plans/ docs/superpowers/specs/
docs/superpowers/specs/2026-07-30-plan-12-decisions.md:87:   D109 decision 2's sensitivity clause
docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md:371:   the same clause in the register

$ grep -rn 'nothing else moves' docs/superpowers/plans/ docs/superpowers/specs/
docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md:850:   (a plan-6 refactor note about `plan_model`; not a set sensitivity, not mine)
docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md:1178:   Task 5 Step 4's Case 4 parenthetical

$ grep -rn 'the only change is' docs/superpowers/plans/ docs/superpowers/specs/
docs/superpowers/specs/2026-07-30-plan-12-decisions.md:87
docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md:371
```

Three expressions, three hits inside this plan family plus one out-of-family hit
that I read rather than skipped (a plan-6 note about lifting a `from_str` line;
no set, nothing owed). **Exactly the two sentences the reviewer named**, and my
own five expressions of the previous round reached neither, because neither
carries a word from the set: one says "one test", the other says "nothing else
moves".

**Both are fixed, in all three places they live.**

- **D109 decision 2's sensitivity clause, in both documents**, now states that
  striking the decision moves **two** tests - Task 5's case 4 in
  `e2e/smoke.spec.ts` and `createBlank resets` in `e2e/editor-undo-redo.spec.ts` -
  and that a third dependent is not a test at all: Step 4b's criterion, whose
  guarded pair would shrink to one. The falsified claim is quoted and marked as
  falsified rather than deleted, which is the house form and is why the phrase
  "nothing else in the package depends on it" still returns a hit: it now appears
  inside the sentence that records its own staleness.
- **Task 5 Step 4's Case 4 parenthetical** now says the sibling case inverts with
  it and the criterion loses a guarded function, and points at the clause that
  carries the full list.

## I-2: the sibling budget comment has a live owner again

**The defect, re-measured before repairing it** (the reviewer's zero-hit claim,
with a fired control so the zero means something):

```
$ awk '/^- \[ \] \*\*Step 4: the tests, in `e2e\/smoke.spec.ts`/,/^- \[ \] \*\*Step 4b/' <plan> \
    | grep -cniE 'budget|comment|recount|recompute|\b54\b'
0        (exit 1)

FIRE CONTROL, same expression over Step 4c's range, which does carry them:
4        (exit 0)
```

So Step 5 credited Step 4 with an instruction Step 4 does not contain. Confirmed.

**The repair keeps the pair in ONE owner rather than splitting it again**, because
splitting is what produced both the original ambiguity and this mirror of it.
Step 4c is retitled to carry both halves and now reads as a pair:

- **Half 1, `e2e/smoke.spec.ts`.** An **end-state** fence rather than an
  old-to-new replacement, deliberately: Task 5's Step 4 edits that file and its
  implementer has already moved the figure from 51 to 54 in the working tree, so a
  replacement fence would not match for a resumed run while an end-state fence is
  checkable in both. The fenced text is the exact three comment lines, with the
  decomposition checked rather than copied (42 + 1 + 4 + 1 + 3 + 3 = 54, the four
  generic action keys being add, remove, undo and redo).
- **Half 2, `src/views/EditorView.vue`.** Unchanged from what amendment 2 fenced.
- **Step 5 now asserts and no longer instructs**, and says so: Step 4c is the only
  place that instructs the pair. The Files-list entry for `e2e/smoke.spec.ts`
  points at "Step 4c, half 1", so a third reader sees both halves covered from
  either direction.

## M-1 and the controller's ruling: the criterion states what it depends on

Step 4b gains a clause naming its one closure: the set of functions Step 2 guards,
fixed by D109 decisions 1 and 2. It states that a new test case can never falsify
the rule - which is the point of writing it against the mandate - that only a
change to the guarded pair can, that **D109 decision 2's sensitivity clause is the
one documented route by which that happens**, and that the rule and its
enumeration are re-derived from the new pair if it ever changes. **The pointer is
reciprocal**: that clause now names Step 4b in return, so the seam is wired from
both ends rather than from one.

## M-2: the ground corrected, the conclusion kept

The old ground - "after a confirmed New `createBlank` completes with no further
IPC" - was wrong about the mechanism. Measured at the artifact:

```
$ sed -n '81,100p' e2e/editor-undo-redo.spec.ts
async function openProfile(...)
  const recorded = await installTauriMocks(page, {
    commands: {
      detect_mkvmerge: [resolveWith(MKVMERGE_INFO)],
      "plugin:dialog|open": [resolveWith(path)],
      load_profile: [resolveWith(loadedDoc(profile))],
      validate_profile_model: [resolveWith(cleanReport)],
```

The seed assignment queues the validate-on-edit watcher, so `validate_profile_model`
**does** fire after a confirmed New. The conclusion survives on the correct ground:
both commands the two entry points reach (`plugin:dialog|open` for Open,
`validate_profile_model` for New) are already in the shared helper's mock set, so
what the New case needs is no *additional* mock. The plan now says that, names the
operative word, and warns that an implementer reading the old ground could raise a
question against a call that is already covered.

## M-3: the per-case exclusion made accurate

"granularity, truncation, the depth cap and U1 activate no guarded control at all"
became "no guarded control **beyond that shared helper's own Open**, which is the
clean activation classified in the clause before this one". Each of those four
calls `openProfile`, which clicks `editor-open`; the helper's activation is
classified in the preceding clause, so the two sentences now agree.

## The sweep for this round

Run in two shapes deliberately, because the round's whole finding is that one
shape is structurally blind:

| # | Shape | Expression | Result |
|---|---|---|---|
| F1 | **function-shaped** (the neighbour class) | `if the owner strikes`, `nothing else moves`, `the only change is`, `nothing else in the package depends` | the two target sentences plus one out-of-family hit, all read; after the fix the first three return only the rewritten sites and the quoted-as-falsified phrase |
| F2 | **consequence-counting** | `one test moves\|two tests move\|moves from asserting\|inverts to\|depends on it\|nothing else` | 16 hits, every one read. The three set-dependent ones are the sentences repaired above; the rest are unrelated senses (a Files-list "nothing else", D112's "read by both surfaces and by nothing else", Task 5 Step 2's "nothing else about it changes", which is about `createBlank`'s implementation and stays true) |
| F3 | **term-derived** (the set's own words) | `the three repaired\|three cases\|exactly two\|two functions\|guarded (pair\|functions)` | 18 hits, all consistent; the "exactly two" survivors are D109 decision 7's two dialog choices and my own "the controls are exactly two", both correct senses |
| F4 | counts | requirement rows, acceptance rows, per-task Files lists | **unchanged**: 43, 73, Task 4 seven, Task 5 six; Task 5's Files list and commit pathspec diff identical (exit 0) |
| F5 | the plan's own gate-part audit | its recorded expression | 1 line, the audit's own sentence - this round added no new match |
| F6 | typography | the dash/quote/ellipsis/NBSP class over the diff | 0 hits, exit 1; the synthetic control returns 1 |

Two house entries written from this review bind the F1/F2 shape and I read both
from the ledger rather than from the dispatch:
`an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations` (its
statement names the sensitivity-clause class explicitly, and its two occurrences
are I-1 and I-2 of this very verdict) and
`a-normative-sentence-naming-a-set-is-discharged-member-by-member`.

## Concerns

1. **The 43-figure sites are the controller's to route and I have not touched
   them**, per the instruction. Recorded here only so the trail is complete: the
   reviewer's enumeration is four live sites in two sets, two holding and two
   stale, and site 4's stale *premise* (that generic add/remove chrome does not
   exist in the catalog, when `editor-action-add`/`-remove` have existed since
   D59) is the one worth a vehicle rather than a figure correction.
2. **The validation-response race is still unrouted** (`watch(model)` does not
   increment `validationGeneration` on the `undefined` branch, so an in-flight
   response can overwrite a parse diagnostic). Unchanged by this round, still a
   product defect outside a documents-only surface.
3. **The ROADMAP `currentPath` write-site correction is still owed** as a close
   action, with the corrected figure of three assignments, two of them added by
   Task 3.
4. **One residual I am naming rather than closing, because closing it is not this
   amendment's to do.** The reciprocal pointer I wired between Step 4b's criterion
   and D109 decision 2's sensitivity clause is prose on both ends: nothing turns
   red if a future edit changes the guarded pair and updates only one of them. The
   honest catch is the review of that edit, and the two house entries now name
   the class, which is a stronger trigger than a sentence. I did not invent a
   mechanism for it, per the standing rule against building unasked.

---

# Fix round 2 (2026-07-31)

**Status:** DONE.
**Commit:** `9d95c5e6954a74855d2787e31cd7a1a0708ac142`, two tracked documents, one
trailer, unsigned, not pushed, on top of `d4c8401`.

```
$ git show --stat --format= HEAD
 docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md | 6 +++---
 docs/superpowers/specs/2026-07-30-plan-12-decisions.md           | 4 ++--
 2 files changed, 5 insertions(+), 5 deletions(-)

$ git log -1 --format='%(trailers)'
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
$ git log -1 --format='%G?'
N
```

Task 5's six uncommitted paths were present before staging and are present after
the commit, unchanged. Nothing stashed, checked out, reset, restored or cleaned;
nothing outside `docs/` written; the index held exactly the two documents.

## N-1: the twins now say the same thing

The decisions document's closing clause said "the second is the claim that went
stale" while its own opening said **two** tests move, and while its plan twin said
both claims were falsified. The plan was right. Both clauses now close with the
same sentence and the same per-claim breakdown, which also removes the
self-refutation:

> ... both were true when written and **both** were falsified by amendment 2 - the
> count by the case that amendment's repair added to the guarded set, and the
> closure claim by the membership criterion that now depends on this decision ...

The falsified original is still quoted rather than deleted, so the record stays
reconstructible, and the per-claim breakdown is what the append-only document owed:
a "why" that names which claim failed for which reason.

## N-2: both sibling pointers now name their owning step

```
- Modify: `src/views/EditorView.vue` (... **and the stale catalog-budget sentence in the file's own header doc block - Step 4c, half 2**)
- Modify: `e2e/smoke.spec.ts` (... **the catalog-budget comment, recomputed - Step 4c, half 1**)
```

The EditorView entry pointed at Step 5, which now explicitly disclaims instructing
anything. Both halves of the pair now point at the step that owns them, in the same
form, and the asymmetry the last round created is gone.

## N-3: the instrument fixed, and the figure it returns

The phrase expression was case-blind and could not match a sentence-initial "If".
Corrected and re-run; the figure, not the conclusion:

```
$ grep -rc 'if the owner strikes' <plan> <adr>          # as run last round
<adr>:1   <plan>:1                                       -> 2 sites

$ grep -rnci 'if the owner strikes' <plan> <adr>         # corrected instrument
<adr>:1   <plan>:2                                       -> 3 sites
```

**Three**, the third being Task 5 Step 4's Case 4 parenthetical, which opens its
sentence with "If". Last round's union with `nothing else moves` covered it, so the
conclusion held, but by luck rather than by design; the corrected expression finds
it on its own.

## The pair comparison, run as its own step

Not a sweep. I derived the pair inventory from my own four commits' ADR hunks
rather than from recall - the ADR side of each commit names exactly the statements
that have a twin - and then read each pair side by side, claim by claim.

| # | Pair | Result |
|---|---|---|
| A | D107 decision 3(f) supersession pointer | **AGREE.** Same four claims (what shipped, what replaces it, the kept consequence, the added state). The plan adds "the live gate is D112's"; no conflict |
| B | D107 decision 7 supersession pointer | **AGREE.** Same claims; the plan places the pointer before the decision's own diagnostics sentences and the ADR after them, and each reads correctly in its position ("the sentence below" vs "this decision's own reason") |
| C | D109 decision 1's recents-gate reconciliation | **AGREE.** Both: gated on the pre-session condition, D112 only narrows, the `!model` term alone carries the unreachability. The plan adds what the second term subtracts; the ADR cites "decision 3f above" where the plan cites "D107 decision 3f", each correct for its own document |
| D | D109 decision 2's sensitivity clause | **AGREE after N-1's fix.** Two tests named identically, the third non-test dependent named in both, and the closing clause now identical in substance |
| E | D112's opening ruling paragraph | **AGREE.** Same ruling, same kept and hidden surfaces, same accepted loss, same "not re-argued", same statement of what the record settles |
| F | D112's supersession paragraph (ADR) against the plan's record-placement decision | **AGREE.** Both: a new decision rather than an edit, `proc-supersede-never-overwrite`, D107 3f and 7 keep their text and gain pointers. The ADR adds that the rest of D106-D110 stands; the plan states that in its own D107 pointer |
| G | D112 decisions 1 to 6 | **AGREE, one by one.** The condition and its reach including the `:disabled` and `saveDisabled` exclusions; the two terms and the `openPath` ordering; the not-`!sessionActive` reason; no third flag; D107's duty split preserved; no new string and no other gate moved. Where one side is terser the other carries the same claim in its rationale or its rejected alternatives, and no claim contradicts |
| H | D112's standing-guard decision (plan 9 / ADR 7) | **AGREE** on every claim: one rule entry in the existing per-`.vue` block, the selector's shape, `pnpm lint` already a gate part, red on a re-inlined gate, scoped so the `:disabled` bindings stay legal |
| I | D112's "What this leaves uncovered" | **AGREE.** Two instruments, the lint rule durable and the grep the demonstration, the same uncovered class (the same question under another name), the same house-consistency argument |
| J | D112's four rejected alternatives | **AGREE on all four steelmen and all four rejections** - and this pair produced the round's one additional finding, below |
| K | D112's "Triggers created" | **ADR only, deliberately.** The plan carries the same fact as a verdict-harvest input in the close list. Asymmetry by design, no divergence |

**The finding the pair check produced beyond the three I was given.** Pair J's last
alternative ended "Decision 9 carries it" in the plan and "Decision 7 carries it"
in the ADR - each correct inside its own document, since the two number D112's
decisions differently, but a reader holding both sees two numbers for one thing.
It is the same class the earlier round already ruled on for cross-references
between the documents, and my rename pass then missed these two because they sit
inside a rejected alternative. Both now read "The standing-guard decision above
carries it", and a re-run for `Decision [0-9]+ carries it` returns **0** in both
documents.

**What the pair check could not have found by sweeping**, stated because it is the
point of the method: N-1 was two sentences this diff itself wrote, so no search for
falsified statements could reach them - both were new and their vocabulary matched
at both ends. Pair J's divergence was two correct sentences that disagree only in a
number each is entitled to use. Neither is expressible as an expression over the
corpus; both fall out of reading the two texts against each other.

## Standing checks

```
requirement rows: 43        acceptance rows: 73
Task 4 Files list: 7        Task 5 Files list: 6
gate-part audit: 1 line, the audit's own sentence
typography over this round's diff: 0 (exit 1)
non-ASCII over this round's diff, wider than the dash class: 0 (exit 1); control returns 1
```

## Concerns

None new. The three carried items - the validation-response race, the ROADMAP
`currentPath` write-site correction, and the 43-figure sites - are routed by the
controller and are not mine. My earlier concern 4 (the reciprocal pointer between
Step 4b and D109 decision 2 is prose at both ends, with nothing turning red if a
future edit updates only one side) is accepted as the honest residual and stands as
recorded rather than as a defect awaiting repair.
