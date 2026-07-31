# Task 3 verdict (Plan 12): New creates a blank profile

**Reviewer:** independent, did not author the change. Read-only on the repo
except this file. Every quotation below is copied from the artifact; every
number is one I measured on this machine, at `2cc0650`, with the tree clean
before and after.

**Spec compliance: APPROVED.** All thirteen requirements T3-a through T3-m are
met, including the ruled deviation, which is implemented exactly as ruled and no
wider.

**Task quality: APPROVED WITH REQUIRED FIXES.** Three Important findings, seven
Minor, no Critical. Two of the Importants are cheap and belong to a fix round
(I-1, I-3); the third (I-2) is a defect in text the plan fenced, so it needs a
controller ruling rather than an implementer edit.

The execution is unusually disciplined: the fork was routed rather than patched,
the seed was re-measured on two instruments, the recount was recomputed from the
file with a fired control, and five of the six new cases defeat at least one
mutation I built against them. The findings below are what survived that.

---

## What I ran

Baseline, on the committed tree: `pnpm build` exit **0**; `pnpm lint` exit
**0**; `pnpm check:i18n` exit **0**; `pnpm test:e2e` exit **0**, **78 passed**;
`python3 scripts/ledger-lint.py` exit **0** (`568 entries across 4 files plus
BUILDING.md's gate enumeration, all invariants hold`). Exit codes captured with
`$?` on the command itself, never through a pipeline (zsh).

Seventeen mutations, each applied to the working tree, each followed by a full
`pnpm build` **and** `pnpm test:e2e`, each reverted with `git checkout --` and
confirmed by an empty `git status --porcelain`. Two throwaway probe specs, at
paths the implementer could not have used (`e2e/zzz-reviewer-probe.spec.ts`,
`e2e/zzz-reviewer-probe2.spec.ts`), both deleted. Restoration confirmed: `pnpm
build` exit 0 then `pnpm test:e2e` exit 0, **78 passed**, tree clean at
`2cc065077ca3597e90d0f81c9cfc9a5ad7523bcd`.

---

## Findings

### Important

#### I-1. `createBlank`'s comment asserts a statement ordering that is measurably not load-bearing

**Symbol:** `createBlank`, `src/views/EditorView.vue`.

The comment says:

```
// The statement ORDER is load-bearing. `diagnostics` is cleared before the
// model is replaced, so a previous profile's findings can never render
// against the new one; `sessionActive` is set BEFORE `model`, so the
// `watch(model)` above -- which fires on that very assignment -- validates
// the seed instead of returning early on a still-false gate; ...
```

Both halves are false, and the first one is falsified by the suite's own case 1,
which asserts the `validate_profile_model` invocation exists:

- Swapping `sessionActive.value = true;` and `model.value = blankProfile();`
  (`pnpm build` exit 0, `pnpm test:e2e` exit 0): **78 passed**. Under the
  comment's stated mechanism the watcher would have returned early on a
  still-false gate and case 1 would have failed. It did not.
- Moving `diagnostics.value = [];` after `model.value = blankProfile();`
  (`pnpm build` exit 0, `pnpm test:e2e` exit 0): **78 passed**.

The mechanism is Vue's default `flush: "pre"` on `watch(model, ...)`. The
callback is queued, not run at the assignment; by the time it reads
`sessionActive.value` the whole synchronous body of `createBlank` has completed,
so both writes are already visible to it whichever order they were made in. The
same fact answers the `diagnostics` half: no render happens between two
synchronous ref writes, so the previous profile's findings cannot paint against
the new model regardless of position.

This is not a coverage gap — it is a comment that teaches a wrong timing model,
in the exact function Tasks 4 and 5 both build on (a history over this funnel,
then guards in front of it). A later reader who believes the watcher fires
synchronously at the assignment will get the next change wrong.

**Origin:** the plan's Step 2 prose, "The order is load-bearing and is commented
as such", so the correction is owed in the plan document as well as in the code.
**Handle:** state what is actually true (the funnel establishes gate and model in
one synchronous block; the watcher observes both afterwards) or drop the
load-bearing claim. The third clause of the same comment, about index 0 and the
detail panel, is true — see I-3.

#### I-2. The new empty state contradicts the open-path line in the one state D107 decision 7 reasons about

**Symbols:** the `v-if="!model"` `editor-empty` paragraph and the `v-if="!model
&& recents.length"` recents section, both in `EditorView.vue`'s template.

`load_profile_body` returns `profile: null` with the parse diagnostic when a
profile fails to load (`src-tauri/src/lib.rs`), and `openPath` sets
`currentPath.value = path` and `sessionActive.value = true` before assigning
`model.value = doc.profile ?? undefined`. So after a failed load the editor holds
a path and no model.

Measured with a reviewer-built probe (mock `load_profile` returning the parse
envelope, open it in the editor, count the surfaces):

```
PROBE-RESULT empty=1 recents=1 diagSection=1 unsaved=0
PROBE-EMPTY-TEXT "No profile open. Create one with New profile, or choose an existing profile file."
```

alongside a visible `Selected profile: /profiles/reviewer-broken.yaml`. The user
is told, simultaneously, which profile is selected and that no profile is open;
and the recents list — hidden after any open before this task, when the gate was
`!currentPath` — comes back underneath both.

The diagnostics section behaves correctly here (`diagSection=1`), which is the
half D107 decision 7 explicitly protects: "keeps the failed-load case visible (a
load that returns `profile: null` still carries its parse diagnostic)". The
decision reasoned about that state for the section and did not reason about it
for the two surfaces it introduced in the same breath.

**Both gates are fenced** — Step 4 prescribes `v-if="!model"` for the empty state
and the `!model` recents gate, and D107 decision 3(f) fixes the latter — so this
is a controller ruling, not an implementer error. The obvious repair (gating both
on `!sessionActive`, or on `!model && !currentPath`) leaves every assertion in
this task green: cases 1, 3, 4 and 5 never enter the failed-load state. Nothing
in the suite covers the state today.

#### I-3. D107 decision 9 ships with no producer, and the owner-ruled precedence clause required one

**Symbol:** `selectedIndex.value = 0` in `createBlank`.

Removing that line and running the full suite: **78 passed**, exit 0. No
assertion in the repository observes it.

The behaviour itself is correct — a second probe measured `PROBE2-RESULT
detailPanel=1 diagMarkers=1` after clicking New — so this is a coverage defect,
not a behaviour defect. That is what makes it worth a finding: the seeded
selection is what puts the user on the field the warning names, and the next
touch of `createBlank` (Task 4 clears `selectedIndex` on every history apply) can
silently drop it against a green suite.

`tests-ship-with-the-feature-never-after` (Tier 2, `docs/process-conventions.yaml`)
carries an owner-ruled precedence clause of 2026-07-28 for exactly this: at
execution time the rule wins over a plan's test enumeration, and the implementer
**builds** the missing producer, when four conditions hold — additive, existing
infrastructure, a consequence this package's own diff creates, and named in the
report. All four hold here. The assertion is one line against the existing
`editor-rule-detail` testid, already asserted in the `editor view: rule detail
editor (Task 13b, D45 / spec 8.2)` describe in the same file. The plan names no
producer anywhere: the acceptance map has no row for it, and the paragraph
enumerating the three observables covered without a row of their own does not
name it either.

**Handle:** add `await expect(page.getByTestId("editor-rule-detail")).toBeVisible();`
to case 1 (or its own two-line case).

### Minor

- **M-1. `plugin:dialog|save`'s arguments are never read.** Case 4 asserts only
  that the call happened. Changing `defaultPath: "profile.yaml"` to
  `"not-a-profile.txt"` leaves the whole suite green (measured). `defaultPath`
  and the reused `batch-profile-filter-name` filter are both fixed by D107
  decision 5, and `recorded` already carries the args, so the assertion is one
  line. `save_profile` and `set_settings` *are* asserted on their arguments,
  which is the important half; this is the third recorded call in the same test
  and the only one read as a bare counter.

- **M-2. The recents-gate change has no producer in this task.** Reverting
  `v-if="!model && recents.length"` to `!currentPath && recents.length` leaves
  the suite green (measured). The acceptance map assigns W4-f to Task 5. Recorded
  as a carry-forward rather than a defect: if Task 5's dispatch does not produce
  it, the change ships uncovered, and I-2 above shows the gate has a live edge
  case worth pinning.

- **M-3. A quoted phrase in the report carries a qualifier from a different
  row.** The report writes: "the model-tier ground for this task is 'seven new
  tests' whose fixtures are composed (the same framing Task 2 ran under)".
  Task 3's tier row reads "... and seven new tests"; "whose fixtures are
  composed" belongs to Task 2's row ("a new test whose locale mechanism is
  prescribed but whose fixtures are composed"). The composition of `OPENED_PATH`,
  `openedDoc` and `openedProfile` is nonetheless correct, on the sufficient
  ground the report also gives: case 6's own prescribed text ("Open a profile,
  edit, Save") cannot execute without an opened-profile fixture, so Step 7's
  fixture list is demonstrably a floor.

- **M-4. A reflow artifact in the rewritten Task-13 doc block.** The line ending
  `` `batch-profile-pick`/`batch-profile- `` now runs about half the width of its
  neighbours and breaks the identifier mid-word before `// current`. The break
  predates the task; the rewrite shortened the line carrying it and left it
  ragged.

- **M-5. One ledger-worthy item is missing from the report's surfacing list.**
  The report closes with "Nothing ledger-worthy beyond the above was found". The
  task's own diff falsifies the number in `editor-generic-action-keys` (Tier 2,
  `docs/product-boundaries.yaml`), whose statement reads "REVISED AGAIN 45 -> 46
  by the plan-7 design (D59)" against a catalog that now carries 49. The plan
  makes that Tier-2 update a controller close action and forbids the task from
  editing the YAML, so this is a surfacing miss and not an edit miss — but Tasks
  4 and 5 raise the number twice more, so it wants tracking now rather than at
  the close.

- **M-6. `OPENED_PATH` is declared twice in `e2e/smoke.spec.ts`** with different
  values: `/profiles/freshly-opened.yaml` in the recents describe and
  `/profiles/already-pathed.yaml` in the new one. Both values are distinct from
  every other path literal in the file, so `echo-mock-distinct-fixture-values`
  holds on the property that matters; the collision is in the NAME and costs a
  reader one extra step.

- **M-7. `createBlank`'s `if (opening.value || saving.value)` re-guard has no
  test.** The button's own `:disabled="opening || saving"` is the covered half.
  Mirrors `pickAndOpen`'s existing untested guard, so this is consistency rather
  than a new gap.

---

## Adjudication verdicts

### Q1. The ruled deviation

**Implemented exactly as ruled and no wider — CONFIRMED.** I extracted the
shipped `doSave` and the plan's fenced block, stripped comment lines from both,
and diffed the executable text. The entire difference:

```
--- FENCE
+++ CODE
   try {
-    if (needsPath) {
+    if (path === null) {
       const picked = await saveDialog({
```

`const needsPath = path === null;` is present unchanged and the second
`if (needsPath)` still gates the recents write. The owed comment is at the site
and locates by symbol: a line-number grep over `src/views/EditorView.vue` returns
nothing, and I fired that same pattern against known-present cases in `docs/`
(`decision-ledger.yaml` "Style note line 13", "progress.md line 10") so the empty
result is a measured absence rather than a broken expression. The comment is
true, with one overstatement worth noting and not fixing: "only a direct null
test makes `path` a `string`" — a cast would also satisfy the checker; the ruling
rejected casts on different grounds.

**The ruling's own claim is CORRECT.** Between `const needsPath = path === null;`
and the first guard the block writes only `saving.value` and `ipcErrorCode.value`
and opens the `try`. `path` is a function-local `let` that nothing outside the
invocation can reach, so the two conditions are equal at that point by
construction, exactly as the ruling states. I also confirmed the ruling's premise
independently: reverting the one token reproduces the failure at exit **2**, the
same two call sites,

```
src/views/EditorView.vue(361,23): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
src/views/EditorView.vue(366,53): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
```

(the line numbers differ from the ruling's 354/359 because the committed file
carries the owed comment; the call sites are `saveProfile(path, profile)` and
`rememberRecentProfile(path)` in both).

One property worth the controller's attention rather than a finding: with the
token reverted, `pnpm test:e2e` still exits **0**. `vite build` does not
type-check `src/`, so exactly one of the eleven gate parts — `pnpm build` — sees
this class of defect.

### Q2. The two divergent counts

**Both resolutions were right.**

*Five seeds against "the four candidate seeds".* Running all five is a superset
that cannot under-satisfy the step, and the authoring section it points at
enumerates five while D107's rejected-alternatives list reasons about five (S1,
S2 and S5 rejected; S3 and S4 both diagnostic-free, S4 chosen). Correct handling,
and the brief's count is the thing that is wrong against its own source.

*Six cases against "seven new tests".* The enumeration governs. "Seven" sits in a
table headed `## Model tiers` whose column is labelled "ground" — sizing
rationale for a model assignment, not a normative test count; Step 7 is the
normative text. Correct.

**And six cases do cover the acceptance halves this task owns.** Walking the map:
case 1 produces W2-a and W2-b; case 2 produces W2-c and W2-d; case 3 produces
W2-e and W2-f; case 4 produces W2-g, W2-h, W2-i and W2-j; case 5 produces W2-k;
Step 1's re-measurement produces W2-l. That is all twelve W2 halves. Case 6 is a
seventh producer answering no acceptance half — it is the branch regression guard
that `proc-proposed-safeguard-stays` protects, and it earns its place (it is the
only case that defeats the recents-write gate, measured below). So the
six-versus-seven resolution costs no acceptance coverage. The three coverage gaps
I found (I-3, M-1, M-2) all sit outside the acceptance map, which is where they
have to be argued.

### Q3. The recount

**49 in both locales — REPRODUCED independently.**

```
$ command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl
49
$ command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/de/gui-editor.ftl
49
```

**The corrected decomposition is true, not merely arithmetically consistent.** I
enumerated the 49 ids rather than subtracting: removing `editor-save-note` (the
save-surface note), `editor-action-add` and `editor-action-remove` (the two
generic action keys), `editor-track-rule-order` (the rule-grid ordinal, D59) and
this task's three leaves exactly 42 ids, and every one of those 42 is a registry
field label (`editor-profile-*`, `editor-meta-*`, `editor-input-*`,
`editor-output-*`, `editor-template-block-template`,
`editor-external-block-external`, `editor-track-rule-*`, `editor-locator-*`,
`editor-attachments-*`, `editor-tracks-*`, `editor-attachment-rule-*`,
`editor-tags-*`, `editor-match-expr-*`). The term that was missing from the file
before this task, the rule-grid ordinal, is a real id present in both catalogs
and is now named in the comment.

The fenced catalog blocks land byte-exact. I compared the plan's two fenced
blocks against the two shipped catalogs programmatically: both present verbatim,
both at end of file, German orthography intact (`Öffnen`, `geöffnet`, `wähle`),
both files ending in a newline.

---

## Can each new test fail?

Each answer below rests on a mutation I built, applied, rebuilt (`pnpm build`)
and ran (`pnpm test:e2e`), then reverted.

| Case | Can fail? | Defeated by (measured) |
|---|---|---|
| 1, New creates and validates | **yes** | seed `extensions: []`; seed `pattern: ""`; seed `rules: []`; watcher gate back on `currentPath`; the unsaved line removed |
| 2, the seed is warned, not blocked | **yes, but not on the seed** | `saveDisabled` regains `!currentPath`; watcher gate back on `currentPath` |
| 3, pre-session empty state + E1 | **yes** | the diagnostics `<section>` un-gated (E1's zero becomes 1); watcher gate back on `currentPath` |
| 4, Save with no path | **yes** | seed `rules: []`; `saveDisabled` regains `!currentPath`; `doSave`'s re-guard regains `!currentPath` |
| 5, cancelled dialog + E2 | **yes** | the cancel branch made to write anyway; `saveDisabled` regains `!currentPath`; `doSave`'s re-guard regains `!currentPath`; the unsaved line removed |
| 6, already-pathed save | **yes** | the recents write un-gated (`if (needsPath)` -> `if (true)`) |

**Case 2 is the one that does not prove what its name says.** "The seeded rule is
warned, not blocked" reads as a claim about the seed; the mechanism is a claim
about the panel and the save gate. `validate_profile_model` is mocked with
`warnReport`, so no frontend test in this harness can observe the real
validator's verdict on the seed — a seed silently changed to one producing
`empty-extensions` at error severity would leave case 2 green.

The suite still catches that mutation, in case 1 rather than case 2: case 1 pins
all three measured seed dimensions on the wire and in the DOM
(`validated.input.extensions` equal to `["mkv"]`, `validated.tracks.rules` length
1, the pattern field holding `.*`), and each of the three error-producing seed
mutations I built fails it. **Unpinned residual, named rather than assumed away:**
`profile_version` and the CONTENT of `tracks.rules[0].match` are asserted by
nothing, so a seed change confined to those two fields would pass the whole
suite. I did not enumerate the validator's error set far enough to say whether
either can produce an error-severity diagnostic.

**E1 is sound.** The zero-assertion and its fire share one `DIAGNOSTICS_SECTION`
const, so they cannot drift; and the fire is real — un-gating the section turns
the zero into a 1 and the test fails.

**E2 is sound, by a weaker route that the brief anticipated.** Its absence
expression is `recorded.filter((r) => r.cmd === "save_profile")` at length 0; its
in-test fire is a poll on `plugin:dialog|save`, a **different** expression, which
proves the flow ran and reached the dialog but says nothing about whether the
`save_profile` filter can ever resolve. The same-expression fire is case 4's
`.toBe(1)` on the identical filter, one test away. That is the shape Step 7
prescribed, and the mutation closes the residual empirically: making the cancel
branch write anyway fails case 5. The test also carries two end-state assertions
(`editor-unsaved` still visible, no open-path line) that a successful write would
break, which is what keeps the synchronous absence assertion honest.

**`PICKED_PATH`'s distinctness property holds.** `/profiles/created-by-new.yaml`
occurs exactly once in `e2e/smoke.spec.ts`, at its own declaration; every other
path literal in the file is a different string. The same is true of the new
describe's `OPENED_PATH`. So the identity assertions in cases 4 and 6 cannot pass
on a shared value — and case 6 goes further by scripting `plugin:dialog|save` to
return `PICKED_PATH`, so an accidentally-opened dialog branch fails the path
assertion loudly instead of agreeing silently.

---

## The no-work-needed checks

Each passage concluding something was unnecessary, with the claim run rather than
weighed:

- *"No capability file changes (`dialog:allow-save` is already granted)."*
  `src-tauri/capabilities/default.json` carries `"dialog:allow-save"` on the line
  after `"dialog:allow-open"`. Correct.
- *"`saveDisabled`'s existing doc comment names only the error-severity rule and
  stays true, so it is untouched."* The comment reads "Save is disabled while any
  error-severity diagnostic exists" and describes a computed that still says
  exactly that. Correct.
- *"None is a registry `labelKey`, so D55 rule 3's tooltip duty does not reach
  them."* `git grep` over `src/` finds the three ids only in the template `$t()`
  calls and the doc comment — no registry entry. Correct.
- *"None of the six values carries a placeable or an attribute."* True by reading
  the shipped catalogs, and `pnpm check:i18n` exits 0 with parity checked.
- *"Removing `currentPath` from `saveDisabled` cannot change the three
  `editor-save` assertions in `e2e/editor-rule-add-remove.spec.ts`, because all
  of them run after an Open."* All three (`toBeEnabled`, `toBeDisabled`,
  `toBeEnabled`) follow an `editor-open` click in their own test. Correct, and
  the suite confirms it.

---

## Harvest

- **An absence check whose in-test fire is necessarily a different expression.**
  E2 cannot fire its own expression in its own test: the whole point is that
  `save_profile` never happens there. The house discipline is satisfied by a
  same-expression fire in a neighbouring test plus an end-state assertion that a
  successful write would break. Worth recording as the accepted shape for this
  class so it is not re-litigated per task.
- **The count-against-its-own-enumeration class hit this brief twice** (four
  versus five seeds, seven versus six tests), after Tasks 1 and 2 hit the same
  class. Three consecutive tasks. The ledger already carries the pattern; the
  generator is the brief-writing step, not the implementers, and each instance
  has so far cost only a surfaced note because the implementers read the
  enumeration rather than the count.
- **`pnpm test:e2e` does not rebuild `dist/`, and `vite build` does not
  type-check `src/`.** Two consequences measured in this review. First, the ruled
  deviation's necessity is invisible to ten of the eleven gate parts: with the
  token reverted, `pnpm test:e2e` exits 0 and only `pnpm build` reports the
  TS2345 pair. Second, reverting a source mutation without re-running `pnpm
  build` leaves the previously built `dist/` in place, so the next `pnpm
  test:e2e` reports the OLD tree's result against a clean `git status` — this
  cost me one confused run before I spotted it. The handle: any revert-and-verify
  loop re-runs `pnpm build` first, and the gate's build-before-e2e ordering is
  load-bearing rather than cosmetic. Both are ledger candidates.
- **The precedence clause in `tests-ship-with-the-feature-never-after` fired for
  the first time here and was not applied** (I-3). Worth an occurrence on that
  entry: the clause was written on 2026-07-28 for exactly this shape — an
  additive test, on existing infrastructure, for a consequence this diff creates,
  that the plan's enumeration happens not to mention.

---

## What I could not verify, and why

- **The six Rust gate parts** (`cargo fmt --check`, `clippy`, `test --workspace`,
  `doc`, `deny check`, `clippy` for the Windows target). The diff touches no Rust
  and no input to it; I re-ran the four frontend parts and `ledger-lint` (all exit
  0) and rebuilt `muxsmith-cli` to re-measure the seed. **The report's "507 Rust
  tests across 39 suites" is not a number I measured.**
- **That the commit was staged pathspec-scoped rather than through `git add -A`.**
  Git history records the resulting tree, not the staging method. What I verified
  is the result: four files, `%G?` = `N` (unsigned), exactly one trailer.
- **The mkvtoolnix SI-3 parity claims behind D107 decisions 5 and 6**
  (`Tab::onSaveConfig` delegating to `onSaveConfigAs`, `actionMergeNew` preceding
  `actionMergeOpen`). Measured at plan authoring, out of this task's scope, and I
  did not open the reference source.
- **Whether the New-before-Open DOM ordering is what shipped.** It is, by reading
  the template, but no test asserts sibling order and the plan says so explicitly
  ("no test in this suite asserts sibling order"). My mutation in that area
  removed the button rather than reordering it, so it measured the button's
  existence, not its position.
- **Whether a seed mutation confined to `profile_version` or to the content of
  `tracks.rules[0].match` could produce an error-severity diagnostic.** Named
  above as the unpinned residual; I measured three error-producing mutations, not
  the validator's full error set.
