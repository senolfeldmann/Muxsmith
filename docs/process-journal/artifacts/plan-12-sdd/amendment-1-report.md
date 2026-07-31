# Amendment 1 report: the owner's failed-load ruling, recorded as D112

**Status:** DONE_WITH_CONCERNS (two concerns, both pre-existing and neither
created by this amendment; section 12).
**Commit:** `bf857ed75b403ddc2805ee02c29d0479d4ce9e4f`, two tracked files, one
trailer, unsigned.

```
$ git show --stat --format= HEAD
 .../2026-07-30-plan-12-qa-round-3-findings.md      | 117 +++++++++++++++++----
 .../specs/2026-07-30-plan-12-decisions.md          |  31 +++++-
 2 files changed, 127 insertions(+), 21 deletions(-)

$ git log -1 --format='%(trailers)'
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format='%G?'
N
```

Nothing outside `docs/` was touched: no product source, no test, no catalog, no
`ROADMAP.md`, no `process-journal.md`, no house-knowledge YAML.

---

## 1. What changed, and where

### `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` (the shipped ADR file)

| Where | Change |
|---|---|
| D107 decision 3(f) | supersession pointer appended; the clause keeps its text as the record of the shipped gate |
| D107 decision 7 | supersession pointer appended for the paragraph's gate; the diagnostics half explicitly untouched |
| D109 decision 1 | the `!model` cross-reference now names the pre-session condition and states that D112 only narrows it |
| new `## D112` section | the ruling, the condition, six decisions, rationale, an uncovered-residual note, three rejected alternatives with steelmen |

### `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`

| Where | Change |
|---|---|
| header note on the decisions file | now names D106-D110 (Task 1) **and** D112 (amendment 1) as the file's contents |
| requirement table | **R43** added, source `owner ruling 2026-07-31 (amendment 1)`, implemented by `D112, Task 4` |
| Decision register intro | records that D112 arrived after Task 1 closed and is rendered by the amendment, not by a task |
| register D107 3(f), D107 7, D109 1 | the same three pointers/reconciliations as in the ADR file |
| register, new `### D112` | the plan-side decision entry (same substance as the ADR, register form) |
| work-item coverage map | W2's row gains `Task 4 (the pre-session gate condition and its tests, D112 - amendment 1)` |
| acceptance map | **W2-m, W2-n, W2-o, W2-p** added, each with a named producer |
| sequencing | the `4 before 5 and 6` bullet gains the second edge amendment 1 creates |
| Task 4 `Read first` | D112 in full, plus `smoke.spec.ts`'s recents describe (the `settingsWith` fixture shape and the two locators the new case needs) |
| Task 4 Files list | the two new named regions in `EditorView.vue`, the new case in `editor-undo-redo.spec.ts`, and an explicit statement that no file and no commit path is added |
| Task 4, new **Step 4b** | the fenced `nothingOpenedOrCreated` computed and the two fenced template gates, plus what deliberately does not move |
| Task 4 Step 6 | the new test case (three legs, fenced mock set, the non-vacuity argument) |
| Task 4 Step 7 | **absence check P1** with its measured pre-state, its green state, and what it does not cover |
| Task 4 `Must not decide` | six new items |
| Task 5 `Read first` and Step 2 | D112 named; the `!model` gate reference reconciled |
| plan close, verdict-harvest inputs | amendment 1's residue added to the known-inputs enumeration |
| self-review: Coverage, Halves, Counts, Absence-checks, Safeguards | all five recomputed or extended (section 9) |

---

## 2. A-1: the behaviour, stated normatively

D112's opening paragraph in both documents states it in the ruling's own terms:
after a profile fails to load the editor renders the "Selected profile" line and
the parse error, and renders neither the empty-state paragraph nor the recents
section; those two show only before anything has been opened or created at all.
The accepted loss (no recents shortcut in that one state) is named there rather
than left to be discovered, and the paragraph says explicitly that the option is
the owner's and is not re-argued.

---

## 3. A-2: the gate condition, and why it is these two terms

**The condition, defined once:**

```ts
const nothingOpenedOrCreated = computed(
  () => !model.value && currentPath.value === null,
);
```

**Design reasoning.** The fact the two surfaces need is "nothing has been opened
or created at all", and the tree already holds it in two refs, one per half of
the question: `model` answers "does the editor hold something", `currentPath`
answers "has a file been bound to the editor". The failed load is precisely the
state where those two answers differ, because `openPath` sets the path and then
clears the model:

```
$ sed -n '/const doc = await loadProfile(path);/,/model.value = doc.profile/p' src/views/EditorView.vue
    const doc = await loadProfile(path);
    currentPath.value = path;
    sessionActive.value = true;
    diagnostics.value = doc.config_diagnostics;
    model.value = doc.profile ?? undefined;
```

Three properties follow, and each is measured rather than argued:

1. **No intermediate frame.** The five statements above are synchronous after
   the awaited load, so no render falls between the path assignment and the
   model clear. In the other direction, the only site that assigns `null` to
   `currentPath` assigns the seed in the same synchronous body:

   ```
   $ grep -nE 'currentPath\.value *=' src/views/EditorView.vue
   247:    currentPath.value = path;
   352:  currentPath.value = null;
   391:    currentPath.value = path;
   ```

   Read by enclosing symbol, those three are `openPath` (a resolved load),
   `createBlank` (the only `null` assignment, followed synchronously by the
   seed) and `doSave` (a newly established path). Fire control for the
   expression: the same pattern against a synthetic
   `  currentPath.value = somethingElse;` matches it, so the three hits are the
   set rather than a pattern accident.

2. **Minimum blast radius.** The condition differs from the shipped `!model`
   gate in exactly one combination, `model === undefined && currentPath !== null`,
   which only `openPath`'s failed branch produces. Every other state answers
   identically, including the bare mount-harness case (an injected `modelValue`,
   no path), where both gates say "hidden".

3. **It survives a funnel nobody has written yet.** Any funnel that puts content
   into the editor writes `model`; any that binds a file writes `currentPath`.
   D107-i's derivation-package funnel is the concrete case: it lands at "model
   set, path null" and both surfaces hide with nothing added for it.

### The state walk

Every state reachable after tasks 3, 4 and 5. `C` is the condition; "shown" and
"hidden" are the two surfaces (`editor-empty`, `editor-recents`).

| # | State | `model` | `currentPath` | `sessionActive` after T4 | `C` | Result |
|---|---|---|---|---|---|---|
| 1 | nothing opened or created yet | undefined | null | false | **true** | shown - required |
| 2 | a blank profile created, unsaved | seed | null | true | false | hidden; the unsaved line renders instead |
| 3 | a profile opened successfully | profile | path | true | false | hidden |
| 4 | an open that failed on a fresh editor | undefined | path | **false** | false | **hidden - the ruling's case**; path line + parse error render |
| 5 | an open that failed while a profile was held | undefined | new path | **false** | false | hidden; identical to 4 by construction, since `openPath` overwrites both |
| 6 | an open that REJECTED at the IPC layer (`loadProfile` threw) | unchanged | unchanged | unchanged | inherited from the prior state | well-defined; from state 1 the surfaces stay shown beside the error alert, which is correct - no path was ever bound |
| 7 | created, then saved through the save dialog | seed | picked path | true | false | hidden |
| 8 | created, then New again (T5 guard confirmed) | fresh seed | null | true | false | hidden; the path clear cannot flash the empty state back (property 1) |
| 9 | an undo or redo applied | profile | unchanged | true | false | hidden; history entries are always profiles |
| 10 | a save that failed | unchanged | unchanged | unchanged | inherited | unchanged state |
| 11 | failed load, then New | seed | null | true | false | hidden |
| 12 | bare mount harness (injected `modelValue`, no IPC) | injected | null | false | false | hidden - unchanged from the shipped `!model` gate, so no mount-harness spec moves |

States 4 and 5 are the two the ruling changes; every other row answers exactly as
the shipped gate does. **The inverse direction was walked too:** for `C` to be
true after a funnel ran, `model` would have to be undefined while `currentPath`
is null, which no site produces (the only model-clearing site sets the path
first, and the only path-clearing site assigns a seed).

**Why not `!sessionActive`** (the near-miss the memo warned about): Task 4 turns
`sessionActive` into a `computed` over `savedSnapshot`, and D108 decision 9 nulls
`savedSnapshot` on a failed load. Column 5 of the table shows the consequence -
`sessionActive` is **false** in rows 4 and 5, so `!sessionActive` would show both
surfaces in exactly the state they must be hidden in. Read against the tree as
Task 3 left it (`sessionActive` a never-cleared ref) that gate would have been
correct, which is why the memo called this out as the thing to decide with the
option: it would have inverted two tasks later.

Both near-misses and the third alternative (clearing `currentPath` on a failed
load, which deletes the only place the failing file is named) are recorded as
rejected alternatives with steelmen in D112.

---

## 4. A-3: where the record went, and why

**A new numbered decision, D112, in `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`,
with supersession pointers on D107 decisions 3(f) and 7.** Not an in-place
amendment of D107.

**The house rule, quoted from the artifact** (`docs/decision-ledger.yaml`,
`proc-supersede-never-overwrite`):

> When a recorded decision is reversed, the old entry is never rewritten into its
> opposite or deleted: the superseded entry keeps its statement and rationale
> with a supersession pointer, the successor entry carries the live rule plus the
> loser's steelman, and both events sit in the occurrence logs - the reversal
> stays reconstructible from the files alone (who ruled what, when, and why the
> first ruling lost).

The same rule stated for ADRs specifically, in `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`:
"makes ADRs append-only with `superseded by` links rather than edits", and its
own trigger T5: D32's B-7 row "needs a `superseded by D111` link, not an edit ...
append-only ADR record".

The substantive reason on top of the rule: D107 decisions 3(f) and 7 describe
what Task 3 built, measured and committed. Rewriting them would delete the
shipped state from the record and leave a dated owner ruling (2026-07-31) with no
dated entry of its own, which is the exact reconstructibility the house rule
protects.

### The number, re-derived rather than trusted

```
$ grep -rhoE '\bD[0-9]{1,4}\b' docs/ | sort -u | sed 's/^D//' | sort -n | tail -20
92 93 94 95 96 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111

$ grep -rnE '\bD1(1[2-9]|[2-9][0-9])\b' docs/ ; echo "exit=$?"
exit=1

$ git grep -nE '\bD1(1[2-9]|[2-9][0-9])\b' -- .   # whole tracked tree
(no output)  repo-wide exit=1

FIRE CONTROL, same expression against a line that carries the token:
$ printf 'this cites D112 and D999\n' | grep -nE '\bD1(1[2-9]|[2-9][0-9])\b'
1:this cites D112 and D999
```

The controller's corroboration is confirmed: highest in use is **D111**, living in
`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`, and no D112 or
higher exists anywhere in `docs/` or in the tracked tree. **D112 is the next free
id.** The zero result is not trusted on its own - the fire control shows the
pattern matches the token when it is present.

### The slots

D112 carries **Decision / Rationale / Rejected alternatives** and omits
**Triggers created**, because it creates none. That omission is the dominant
house behaviour, re-measured rather than borrowed from the ledger's own note:

```
$ grep -rhcE '^## D[0-9]+'   docs/superpowers/specs/*.md | paste -sd+ | bc   ->  110
$ grep -rhcE '^### D[0-9]+'  docs/superpowers/specs/*.md | paste -sd+ | bc   ->    0
$ grep -rhciE '^\**Triggers created' docs/superpowers/specs/*.md | paste -sd+ | bc -> 8
```

110 D-sections across the D-sectioned spec files, 8 of them with the slot. (The
ledger records the same conclusion against a narrower denominator, 46 sections;
my denominator is the whole `specs/` set, so the two are not the same
measurement and I did not adopt its figure.)

---

## 5. A-4: which task builds it

**Task 4**, not Task 5. The memo's own view - "Task 5 already touches the same
recents surface, so it is a natural vehicle" - is **refuted by the plan's own
Files list**, which is the artifact to check it against:

```
Task 5, Files (EXHAUSTIVE):
- Modify: `src/views/EditorView.vue` (the dialog mount, the two guarded call sites)
```

Task 5 does not edit the recents surface at all; it only asserts on it (case 5,
and acceptance row W4-f). Task 4, by contrast:

1. **owns the state the ruling is about.** D108 decision 9 decides what happens
   to the history on a load that returns no profile, and Task 4 Step 6 already
   carries the only prescribed test that drives that state.
2. **is the task that redefines `sessionActive`** (Step 1: ref -> `computed` over
   `savedSnapshot`), which is the hazard the ruling's cost clause names. Landing
   the explicit condition in the same task means the two can never diverge, not
   even for one commit.
3. **is next in the serial order**, so the state the owner ruled against does not
   ship through another task's commit.
4. **must not be both producer and consumer.** D109 decision 1's argument for
   leaving `openPath` unguarded consumes this gate. Settling the gate in Task 4
   and consuming it in Task 5 keeps those two roles in different tasks; putting
   both in Task 5 would make that argument self-referential inside one review.

Task 4's sections were updated end to end: `Read first`, the exhaustive Files
list, a new fenced **Step 4b**, the new test case in Step 6, absence check **P1**
in Step 7, and six new `Must not decide` items.

**The commit block needed no edit, and that is a measurement rather than an
oversight.** Task 4's pathspec already stages both files this amendment writes
into:

```
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl \
        e2e/editor-rule-add-remove.spec.ts e2e/editor-undo-redo.spec.ts e2e/smoke.spec.ts
```

`src/views/EditorView.vue` (the computed and the two gates) and
`e2e/editor-undo-redo.spec.ts` (the case) are both members; D112 adds no catalog
string, so neither `.ftl` moves. The Files list now says this explicitly so a
reviewer can check it rather than infer it.

---

## 6. A-5 and A-6: the acceptance rows and their producers

Four rows, all machine-verifiable, all produced by Task 4 - the same task as the
behaviour, with no exemption claimed:

| Row | Half | Producer |
|---|---|---|
| W2-m | after a failed parse the editor still names the failing file and still renders the parse error | Step 6's D112 case, leg 3's two POSITIVE assertions |
| W2-n | ...and renders neither surface | same case, absence check P2's zero in leg 3, fire in leg 1 |
| W2-o | before anything is opened or created, both DO render | same case, leg 1 |
| W2-p | the condition exists in exactly one place | Step 7's absence check P1, its measured pre-state as fire |

**Why W2-o is a row and not only P2's fire:** an implementation that hid both
surfaces in every state passes W2-n and fails the product. The rendered side has
to be a graded half, which is exactly what A-5 demands of a rendered-versus-hidden
pair.

**The two absence checks carry all three parts.**

- **P1** - expression `grep -nE 'v-if="!model' src/views/EditorView.vue`;
  pre-state **measured now**, not predicted:

  ```
  $ grep -nE 'v-if="!model' src/views/EditorView.vue
  637:      v-if="!model"
  644:      v-if="!model && recents.length"
  ```

  exactly 2 lines, the two gates Step 4b replaces; end state 0. That non-zero
  pre-state run **is** its fire - same expression, same file, a result that
  cannot be a pattern matching nothing. Its two uncovered edges are stated in the
  plan rather than implied: it matches one exact spelling of the attribute, and
  it is a one-shot check at Task 4's verification rather than a standing guard.
- **P2** - the two locators `editor-empty` and `editor-recents`; fire is leg 1 of
  the same test with an exact expected result (visible, and count **1**); end
  state is leg 3 with the expected zero for both.

**Non-vacuity of P2's zero, which a count-0 assertion on a conjunction otherwise
lacks:** the recents gate is `condition && recents.length`, so a zero could mean
either term. The scenario keeps `recents` non-empty throughout (the `get_settings`
mock seeds one path, and each successful open writes another through
`rememberRecentProfile`), and leg 1 measures that same list through the same
locator at count 1 - so the term that is false in leg 3 is the gate.

**The fenced mock set is complete and rests on measured harness behaviour**
(`e2e/mocks.ts`): per-command queues are "consumed in call order; once exhausted,
the last entry repeats", the `get_settings` fallback returns `recent_profiles: []`
(so a seeded recents list must be mocked explicitly), `set_settings` has a
fallback that returns `null`, and an unmocked command throws in the page.

**Robustness against the rest of the plan, stated in the step:** the model is
never edited in that flow, so `dirty` stays false and the discard guard Task 5
adds to `pickAndOpen` cannot alter what the test does.

---

## 7. A-7: no new catalog string

The amendment changes two `v-if` expressions and adds one `computed`. It renders
no new text: `editor-empty` and the recents heading (`batch-recents-heading`)
already exist and keep their values. The editor catalog budget arithmetic in the
plan (46 -> 54, and the running totals 49 / 51 / 54) is untouched, as are the
"15 new catalog ids across 3 catalogs" and "30 new catalog lines" figures - all
verified unchanged in the diff. D112 decision 6 records this as part of the
decision, since it is part of what the owner bought with option A.

---

## 8. A-8: the spec question, answered with a measurement

**Spec 8.2, as Task 1 amended it, asserts nothing about the state this ruling
changes. No spec edit, and no addition to any task's Files list.**

```
$ S=docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md

# A8-E1, the two surfaces' own vocabulary
$ grep -nEi 'recent profiles|recents|empty state|no profile open|nothing (is )?open' "$S"
382: [the editor item] ... panels for attachments/chapters/tags/title, create/open/save
     YAML, recent profiles. ... Add appends an empty rule - incomplete until filled ...

# A8-E2, the failed-load state's vocabulary
$ grep -nEi 'fails to (load|parse)|failed (load|open|parse)|parse (error|failure)|does not parse|unparse' "$S"
(no output)   exit=1

# A8-E2 FIRE CONTROL, the same expression against documents that do carry it
$ grep -nEic '<same expression>' <plan> <owner memo>
.superpowers/sdd/plan-12/owner-decision-failed-load-empty-state.md:7
docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md:3

# A8-E3, any rendering condition on the editor's surfaces
$ grep -nEi 'only (shows|renders|appears|while|before)|shown only|renders only|hidden|hides' "$S"
382: [the editor item] ... warns first and only while unsaved changes exist ...

# A8-E4, every spec line that names the editor at all, so no editor statement is unreachable
$ grep -nEi 'editor' "$S"
44:  [YAML/JSON data model]     -- not the editor view
376: [`muxsmith schema` / CLI]  -- not the editor view
382: [the editor item]
```

Reading the three hits: 8.2 names "recent profiles" as an editor capability and
never states a condition under which the recents list or an explanatory paragraph
renders; its one "only while" clause governs the discard warning, not these
surfaces; and the failed-load state does not appear in the section at all. E4
proves the surface was whole rather than the expressions merely quiet - the spec
has exactly three lines mentioning the editor and only one is the editor view.

---

## 9. A-9: the self-contradiction sweep

Seven expressions before the edits, three after, each run over **both** documents
(`.../plans/2026-07-30-plan-12-qa-round-3-findings.md` and
`.../specs/2026-07-30-plan-12-decisions.md`). The terms were derived from the
artifact - the two gates' source text, the two surfaces' `data-testid`s, the term
whose meaning Task 4 changes, and the tables my edit moves - not from memory of
what I had written.

| # | Expression | Result and disposition |
|---|---|---|
| S1 | `!model` | 11 hits before the edit. Reconciled: D107 3(f) and 7 (pointers added), D109 dec 1 (rewritten), Task 5 Step 2 (rewritten). Left standing deliberately: the authoring section's pasted greps of the pre-Task-3 tree, Task 3's fenced steps, `doSave`'s fenced body, Task 4's `:disabled="!model \|\| !canUndo"`, D108 decision 10 |
| S2 | `recents\|recent-profile\|recent profiles\|editor-recents` | every hit read; W2-j, W4-f, Task 5 case 5 and the catalog rows all stay true under the narrowed gate, which only subtracts states |
| S3 | `empty state\|empty-state\|editor-empty\|pre-session` | every hit read; W2-e, W2-f, Task 3 Step 7 case 3, the budget table row and the catalog fences stay true |
| S4 | `failed (open\|load)\|profile: null\|parse error\|parse-error\|does not parse\|fails to load` | 6 hits: D107 dec 7, D108 dec 9, Task 4 Steps 1 and 6, plus two rationale lines. None contradicts the ruling; D108 dec 9 is its neighbour and is consistent (the history clears, the diagnostics stay) |
| S5 | `sessionActive` | every hit read; the term's two-step landing is what D112 decision 3 turns on, and no existing statement claims these two gates read it |
| S6 | `D106-D110\|D106 to D110\|five ADRs\|5 ADRs\|five decision\|R4[0-9]\|69 acceptance\|acceptance halves\|nineteen\|...` | the file-scoped statements (header note, register intro) now name both sets; the Task-1-scoped ones (its heading, Interfaces, Must-not-decide, the model-tier table and its ground) stay as written because they remain true **about Task 1**, which authored five |
| S7 | `parked\|owner decision\|owner-decision\|his call\|route to the owner\|amendment` | **no dangling cross-reference to the parked decision exists in either document** - the parking lived in the SDD scratch record, not in the plan. Nothing to reconcile |

**Post-edit re-runs** (the ones that matter, because my edits are the newest
source of contradiction): S1 re-run and every one of its 24 surviving hits
classified by reading (list above); a count/enumeration re-run confirming the
surviving `D106-D110` statements are all Task-1-scoped; and a "what renders while
the editor holds nothing" re-run over
`holds no model|holds nothing|shown when the editor|no profile open|editor holds (anything|a profile)|pre-session|pre-Open`,
whose every hit was read and found consistent.

**The counts my edit moves, recomputed from the tables themselves rather than by
adding one to the old figure:**

```
$ awk '/^\| # \| Requirement/,/^$/' <plan> | grep -coE '^\| R[0-9]+ '        -> 43   (highest R43)
$ awk '/^\| # \| Observable half/,/^---$/' <plan> | grep -oE '^\| W[0-9]+-[a-z0-9]+ '
    TOTAL 73;  W1 10, W2 16, W3 22, W4 23, W5 2;  duplicate ids: none
```

10 + 16 + 22 + 23 + 2 = 73. Five self-review statements were updated to match:
the Coverage paragraph (43 / 73), the halves walk (W2's four new rows and why),
the Counts paragraph (43, 73 with the per-item split, and "6 decision records in
the decisions file" replacing "5 ADRs"), the absence-check enumeration (P1 and P2
with their fires), and the safeguards enumeration (nine -> **eleven**, with both
new ids listed).

**Fired controls for the zero-result checks.** S7's own hits are non-empty, so it
needs none; A8-E2 and the D112+ number check each carry one (sections 8 and 4).
The typography check is the third zero result and carries its own:

```
$ git diff -U0 -- docs/ | grep '^+' | grep -nP '[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]'
(no output)   exit=1
```

Its fire control was run with the identical character class against a synthetic
line carrying an em-dash, a curly quote pair and an ellipsis, and returned `1`.
The literal characters are not reproduced in this sentence, because this document
is under the same typography rule the check enforces.

No em-dash, en-dash, figure dash, horizontal bar, Unicode minus, smart quote,
ellipsis or non-breaking space entered either document. No line number is cited
inside either document, and the fenced comment locates code by symbol only.

---

## 10. A-10: no task added, removed or re-cut

The seven tasks are unchanged in number and in cut. Amendment 1 changes what one
existing task (Task 4) contains, adds no file to any Files list, and adds no path
to any commit block. This is the same shape the previous mid-run amendment took
in this plan (the owner's 2026-07-30 re-read ruling: R42 in the requirement
table, D109 decision 9 in the register, Step 2b plus `Must not decide` items and
acceptance rows in Task 6) - followed deliberately, because it is the house
pattern for this document rather than a shape I chose.

---

## 11. What I deliberately did not do, with reasons

1. **Did not rewrite Task 3's steps**, though they fence the gates D112 replaces.
   Task 3 is closed and committed; its steps are the record of what it built.
   Step 4b names the state it starts from instead ("Task 3 wrote
   `v-if="!model"`"), so the sequence stays legible without falsifying the
   record. This is the same `proc-supersede-never-overwrite` reasoning that
   decided A-3, one layer down, and it is stated in the step.
2. **Did not edit the v1 spec.** A-8's measurement shows it asserts nothing about
   the state; adding a sentence would have been new normative surface the ruling
   did not ask for.
3. **Did not re-argue options B and C.** The ruling settles the option; D112's
   rejected alternatives concern only how the gate condition is expressed.
4. **Did not add a catalog string, a "could not be opened" sentence, or any
   change to the parse error and the path line.** All three are outside what was
   ruled, and the last two are what the ruling explicitly preserves.
5. **Did not add a standing guard for the single-definition property.** A grep in
   a task's verification cannot watch the tree afterwards, and there is no
   existing check part this could join without adding a gate part, which the plan
   forbids. The gap is named in D112 and in P1 rather than covered over.
6. **Did not touch `docs/ROADMAP.md`, the process journal or any house-knowledge
   YAML** (boundary), including one staleness I found there - see concern 2.
7. **Did not fix the Task-4/Task-5 test interaction** - see concern 1. It is
   pre-existing, it is not this amendment's to re-cut, and my own case is
   deliberately built so it is immune to it.

---

## 12. Concerns

1. **Pre-existing, not created here: several Task 4 cases open a profile after
   editing one, and Task 5 puts a confirm dialog in front of exactly that.**
   Task 4 Step 6's "Open resets", "A failed open clears rather than keeps" and
   the truncation case each build history (which makes `dirty` true) and then
   click Open. Once Task 5 lands the `pickAndOpen` guard, that click raises the
   confirm and the open never happens, so those cases would need the confirm
   answered. This is visible in the plan today and is independent of amendment 1;
   I flag it for Task 5's dispatch rather than re-cutting Task 4's prescribed
   cases. **Amendment 1's own case is immune**: it never edits the model, so
   `dirty` stays false through both opens, and the step says so.
2. **Ledger-worthy, ROADMAP-side and out of my boundary:** the round-3 finding-2
   entry states "`currentPath` has ONE write site (`openPath`)", which Task 3
   already falsified (`doSave` now writes it when a path is newly established).
   The same entry's four-duty split is a correct record of the pre-Task-3 tree
   and needs no change. Both are close-action dispositions, not amendment work.
3. **A note on D112's placement of `currentPath`**, raised here because a reviewer
   will reasonably ask it: D107 decision 3 moved several duties off `currentPath`,
   and this condition reads it again. It reads it for the one question that
   decision left with it - which file the editor is bound to - and never as a
   stand-in for "may I edit" or "is there content", which the `!model` term still
   carries. D112 decision 5 states this, and the state walk's row 2 is the proof:
   a created-and-unsaved profile hides both surfaces with no path in existence.

---

# Fix round 1 (2026-07-31)

**Status:** DONE_WITH_CONCERNS.
**Commit:** `7c80c400c7bf38282621c7ca30323d3e9e656bcb`, two tracked documents, one
trailer, unsigned, not pushed. Round-1 commit `bf857ed` stands unchanged beneath it.

The verdict is at `.superpowers/sdd/plan-12/amendment-1-verdict.md`. Requirement
compliance came back MET on A-1 to A-10; this round addresses the 2 Important and
6 Minor quality findings, plus the controller's authorized scope extension.

## I-1: the claim scoped to what its checks measure, in both documents

**Verified before narrowing, as instructed.** The reviewer's reading holds at the
artifact:

```
$ grep -n 'data-testid="editor-undo"' <plan>
Step 4 ... `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"`
        (the model term per D108 decision 10, ...)

$ grep -n "Undo and redo are gated on" <plan> <adr>
<adr>:68:  10. **Undo and redo are gated on `model`, ... the buttons' `:disabled` carries the same term.**
<plan>:354: 10. **Undo and redo are gated on `model`, ...**
```

So the document family's own vocabulary calls those two bindings a *gate* on
`model`, and `saveDisabled` (D107 decision 3a) carries a third bare `!model.value`.
The unqualified sentence was false at the end of Task 4.

**Narrowed to the check, not the reverse.** Both documents now read "No RENDER
gate in the view - no `v-if`, no `v-else-if` - reads a bare `!model` afterwards",
and both name why the `:disabled` bindings and `saveDisabled` are deliberately
outside that reach: they gate an action on whether there is content, which is a
different question from whether anything has been opened or created. The reach of
the sentence is now exactly the reach of the lint rule and of P1.

```
$ grep -nE 'No (other )?gate in the view reads' <plan> <adr>
(no output)   exit=1
FIRE CONTROL: the same expression against the sentence as it stood before this
round returns 1.
```

## I-2: the guard prescribed, after re-deriving the measurement

**The premise was refuted by running it, independently of the reviewer's run.**
Every input is on disk and none of it was checked in round 1 - that is the whole
defect, and the general handle now sits in D112's rejected alternative: a
no-work-needed conclusion whose enabling premise is a claim about what the
toolchain can or cannot do is settled by invoking the toolchain.

```
$ grep -n 'eslint' package.json
11:    "lint": "eslint .",
33:    "eslint": "10.6.0",
34:    "eslint-plugin-vue": "10.9.2",

$ node -p "require('./node_modules/eslint-plugin-vue/package.json').version"
10.9.2

$ ls node_modules/eslint-plugin-vue/dist/rules/no-restricted-syntax.js
node_modules/eslint-plugin-vue/dist/rules/no-restricted-syntax.js
```

`eslint.config.js` already carries a per-`.vue` `rules:` block (the
`@intlify/vue-i18n/no-raw-text` one), and `pnpm lint` is already a gate part.

**The probe was built outside the repository** (in the session scratchpad), using
the repo's installed toolchain through absolute import paths. Two probe configs
were used: a minimal one, and then a second that is the repo's own
`eslint.config.js` verbatim with the three imports made absolute and the
prescribed rule inserted into the existing rules block - so the run validates the
edit the plan prescribes rather than a sketch of it. `eslint.config.js` in the
repository was not touched; the working tree carried only the two documents at
commit time.

```
RED, repo-shape config, against src/views/EditorView.vue as Task 3 left it:

/.../EditorViewPre.vue
  637:14  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
  644:14  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax

✖ 2 problems (2 errors, 0 warnings)
exit=1

GREEN, repo-shape config, against Task 4's end state (both gates rewritten to
nothingOpenedOrCreated, and the two `:disabled="!model || !can*"` bindings
PRESENT as the over-match control):
exit=0
```

Two further runs, because the selector's own enumerated set is a claim and a
grep-shaped residual was open:

```
SELECTOR SET, member 2 fired on its own (member 1, `v-if`, fired in the RED run):
the file's existing `v-else-if` pointed at `!model` once ->
  652:19  error  A render gate must not read `!model` directly: ...  vue/no-restricted-syntax
✖ 1 problem (1 error, 0 warnings)

M-3's term-order case, `v-if="currentPath === null && !model"`:
  658:38  error  A render gate must not read `!model` directly: ...  vue/no-restricted-syntax
✖ 1 problem (1 error, 0 warnings)
$ grep -cE 'v-if="!model' EditorViewTermOrder.vue
0        exit=1        <- P1's grep is blind to it; the rule is not
```

**Prescribed, per the controller ruling, in Task 4 and not as a close action:**
D112 gains a standing-guard decision in both documents; Task 4's Files list gains
`eslint.config.js` as a named region; **Task 4's commit block gains the same
path**; new **Step 4c** carries the fenced rule entry with its comment, its
placement (first in the existing `**/*.vue` rules block), and its falsifiability -
the instrument-exists check, the RED pre-state with its exact expected result, the
GREEN end state with the over-match control, and the per-member selector fire.
Task 4's `Must not decide` gains the rule's scope and shape. The Tech Stack's "one
script this package edits" sentence is corrected to two tooling files, with the
no-new-gate-part claim restated for both.

**The red state is fenced by the two gates, not by line numbers**, which move as
Task 4's other steps add script above the template; the step says so.

**P1 is kept beside the rule, not replaced by it** (`proc-proposed-safeguard-stays`):
one line, an already-measured pre-state, and a grep and a lint rule fail for
different reasons. Its role is restated as the one-shot demonstration; the rule is
the guard.

## M-1: the Read-first pointer corrected

```
$ grep -rn "editor-recents" e2e/*.ts
(no output)   exit=1
$ grep -rc "editor-recent-profile" e2e/*.ts | grep -v ':0'
e2e/smoke.spec.ts:3
$ grep -n 'data-testid="editor-recents"\|data-testid="editor-recent-profile"' src/views/EditorView.vue
646:      data-testid="editor-recents"
658:            data-testid="editor-recent-profile"
FIRE CONTROL for the zero: the same expression where the string IS present ->
$ grep -rc "editor-recents" src/views/EditorView.vue
3
```

Task 4's `Read first` now names `editor-recent-profile` as the only one of the two
that any spec uses, states that `editor-recents` appears nowhere under `e2e/`, and
points at `src/views/EditorView.vue`'s template, where both are defined. It also
gains `eslint.config.js` for Step 4c.

## M-2, M-4, M-5: report defects corrected in place

House form: the wrong sentence stays and is marked, with its correction beside it.

**M-2 CORRECTION (fix round 1).** Section 6 states "**The fenced mock set is
complete** and rests on measured harness behaviour". **That sentence is wrong in
its first three words and stands here as the record of the error:** measured over
the Step-6-to-Step-7 range of the plan, that range contains **zero** fenced
blocks. The mock set is prose carrying four angle-bracket placeholders. The
specificity itself is correct and matches the plan's settled house level (Task 3
Step 7 and Task 4 Step 5 specify e2e fixtures the same way); only the word "fenced"
was false. Read that sentence as "the prescribed mock set is complete". The same
class applies to sections 4, 8 and 9 of the round-1 report, where code fences mix
genuine pasted output with authored elisions (`<same expression>`, `<plan>`,
`<owner memo>`, `[the editor item]`): **a fence carries only what a re-run
produces, and elisions belong outside it.** This fix round's own fences follow
that rule; where a line is not reproducible verbatim it sits outside the fence.

**M-4 CORRECTION (fix round 1), and it must not propagate.** Concern 2 states that
the ROADMAP's "`currentPath` has ONE write site (`openPath`)" was falsified by
Task 3 and names only `doSave`. **That under-enumerates by one.** Task 3 added
**two** write sites, and the correct statement for the ROADMAP close action is
that the tree now carries three assignments:

```
$ grep -nE 'currentPath\.value *=' src/views/EditorView.vue
247:    currentPath.value = path;
352:  currentPath.value = null;
391:    currentPath.value = path;
```

Read by enclosing symbol: `openPath` (pre-existing), **`createBlank`** (the `null`
assignment, added by Task 3) and **`doSave`** (the newly established path, added by
Task 3). The close action that corrects the ROADMAP entry takes this figure, not
concern 2's original one. The amendment's own reasoning never depended on the
ROADMAP sentence: D112's "nothing else can produce the combination" rests on this
same fresh grep, which is why the under-count damaged nothing inside the
amendment.

**M-5 CORRECTION (fix round 1).** Concern 1 names **three** Task-4 cases that
"build history and then click Open". **The truncation case does not**, and the
sentence stands here as the record of the error. Re-enumerated from Task 4's own
step text by the stated criterion (builds history, then opens again through
`pickAndOpen`), the affected set is exactly **two**: "Open resets" and "A failed
open clears rather than keeps". The truncation bullet reads in full "undo once,
then edit; Redo is disabled" - no second Open. The full enumeration and the reason
each other case fails the criterion are now in the plan itself, at Task 5 Step 4b.

## M-3 and M-6: confirmed resolved by the I-2 fix, at the artifact

**M-3 (W2-p's observable broader than its producer): resolved, and the resolution
is measured rather than assumed.** The term-order duplicate the row implied but P1
could not see is caught by the rule (run pasted above: 1 error from the rule, 0
hits from P1's grep). W2-p is also rewritten so the observable and its producers
have the same reach - "No render gate in the editor asks `!model` directly" - and
it now names both producers and states the one class still outside both: a second
computed recomputing the same expression under a different NAME. That residual is
carried in D112's uncovered paragraph in both documents rather than left implied.

**M-6 (the residual registered nowhere that will announce it): resolved.** The
residual the finding was about - "nothing turns red if a later round re-inlines a
bare gate" - no longer exists, because the rule turns `pnpm lint` red and names the
file and line. D112 now carries a **Triggers created** slot recording exactly that
self-firing property, which is the same form D110 used for its own surfaced gap,
and the plan close's known-inputs list names the new standing guard as a check
class the verdict harvest should see. The slot's presence is now warranted on the
same ground its earlier omission was: the house writes the slot when a trigger
exists.

## The authorized scope extension: the Task-4/Task-5 confirm collision

**The affected set, enumerated from the plan's own step text rather than from
either count offered to me.** Criterion: the case makes the model dirty (an edit
after the load baseline) and then performs a second Open through `pickAndOpen`.

| Task 4 case | dirty at a second Open? | affected |
|---|---|---|
| the six mutation-path cases (Step 5) | the open precedes the mutation; no second open | no |
| granularity, three halves | no Open at all | no |
| truncation ("undo once, then edit; Redo is disabled") | no Open at all | no |
| save marks rather than clears | saves before it ends; no second Open | no |
| **Open resets** | yes ("with history built", then opens another profile) | **YES** |
| **A failed open clears rather than keeps** | yes ("with history built and Undo enabled", then opens a second path) | **YES** |
| depth cap | no second Open | no |
| U1, the text-entry exemption | no Open | no |
| mount-harness property (other file) | not applicable | no |
| amendment 1's D112 case | opens twice, never edits, so `dirty` is false at both | no |

Exactly two, which confirms the reviewer's count and refutes my own three.

**The repair, and why this shape.** Task 5's Files list gains
`e2e/editor-undo-redo.spec.ts` as a **named two-case region**, its commit block
gains the same path, and new **Step 4b** prescribes: in each of the two, between
the Open click and the assertions that follow it, assert `confirm-dialog` is
visible and click `confirm-dialog-confirm`. No existing assertion is removed,
weakened or reordered, and the cases keep exercising a genuinely dirty editor.

Grounds, in order of weight:

1. **It is the plan's own house pattern.** Task 3's Files list carries "the two
   doc-comment regions this task's own change falsifies", and Task 4's carries
   `e2e/editor-rule-add-remove.spec.ts` for "the header doc sentence this package
   falsified". The task whose change falsifies something repairs it, as a named
   region in a file it does not otherwise own.
2. **The alternative repair was rejected on the no-weakening rule.** Task 4 could
   have saved before the second Open, which would leave the history intact
   (D108's save marks rather than clears) while making `dirty` false, so the guard
   would never fire. That swaps the case's own subject to dodge a mechanism and
   silently drops the dirty-editor path, which is the state a real user reaches.
3. **Task 4 cannot pre-adapt them:** `ConfirmDialog` does not exist until Task 5's
   Step 1, so those clicks would fail there. The step says so.
4. **No acceptance row is added:** W4-a to W4-c already grade the guard. The
   repair keeps two existing producers alive rather than producing a new
   observable. Task 5's verification step now states that the two repaired cases
   are inside the gate it runs, so a missed repair is a red gate at Task 5 rather
   than a surprise at the push.

No task was added, removed or re-cut: the change adds a named region to one
existing task's Files list, which is the same maintenance the plan already
performs twice.

## Consequential corrections this round's edits forced

Each is an enumeration or count my own edit moved, found by sweeping rather than
by recall.

- **Task 4's Files list is now seven files, not six**, so its Step 7 diff-scope
  sentence was updated, and its Files list and commit pathspec were re-diffed
  against each other and are identical. Task 5's is now six, and the same
  comparison passes.

```
$ Task 4 Files list vs its git add pathspec   -> identical (7 entries)
$ Task 5 Files list vs its git add pathspec   -> identical (6 entries)
$ per-task Files-list counts: 2, 6, 4, 7, 6, 9, 4
```

- **The file-overlap enumeration in the sequencing section was understated before
  this round, independently of my change, and is corrected rather than left
  standing beside a correct neighbour.** Measured from the seven Files lists:
  `src/views/EditorView.vue` is written by four tasks (3, 4, 5, 6) and
  `e2e/smoke.spec.ts` by four (3, 4, 5, 6), where the sentence said three and two;
  `locales/{en,de}/gui-editor.ftl` by three (3, 4, 5), which was right; and
  `e2e/editor-undo-redo.spec.ts` now by two (4, 5). The correction changes no
  ordering argument, since serial execution was already the conclusion.
- **The plan's own gate-part-count audit fired on this round's prose, exactly as
  that paragraph predicts, and the sentence was reworded rather than the pattern
  narrowed.** My Step 4c text said "in this plan's three parts"; the audit
  expression matched it on the next run. It now names the three things instead of
  counting them, the audit returns one line again (its own sentence), and the
  audit paragraph records this round's run beside the earlier ones.
- **The two documents number D112's decisions differently** (the plan register
  carries two plan-side items the ADR omits), so every ordinal cross-reference
  into that list introduced this round was renamed to name the decision instead -
  the plan's own recorded rule for D109's rejected alternative, "named rather than
  numbered, since an ordinal into that list stales the moment one is inserted".
  Eight cross-references, and a re-run for residual ordinals returns none.

## Verification of this round

```
$ requirement rows: 43        acceptance rows: 73        duplicate ids: 0
$ typography over this round's diff (em/en dash, smart quotes, ellipsis, NBSP,
  Unicode minus): 0 hits; the same class against a synthetic line: 1
$ gate-part audit: 1 line, the audit's own sentence
$ git status --porcelain after the commit: clean
$ git log -1 --format='%G?' -> N   (unsigned, as policy requires)
$ trailers -> Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>   (exactly one)
```

## Concerns

1. **The validation-response race the reviewer surfaced is real, pre-existing, and
   still unrouted.** `watch(model)` returns early on the `undefined` branch
   *without* incrementing `validationGeneration`, so a `validate_profile_model`
   response still in flight from an earlier successful open can resolve and
   overwrite `diagnostics.value` after a failed load - the state whose parse error
   D112 requires to render. It is not reachable in the prescribed mocked flows and
   it equally affects Task 4's pre-existing failed-open case. It is a product
   defect in shipped code, outside a documents-only round's edit surface, and it
   belongs to whoever routes Task 4's dispatch. **Recorded here as a controller
   item rather than resolved**, per this round's brief-level correction.
2. **The ROADMAP correction still owes a close action**, now with the corrected
   figure (M-4 above): two new write sites, three assignments in total.
3. **D112's remaining uncovered class is a duplicate definition under a different
   name.** No check in this repo detects that class for any derived value, so the
   depth is house-consistent; it is stated in both documents rather than implied,
   and the catch is the diff review.
