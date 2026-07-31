# Plan 12 review, round 1 - independent reviewer verdict

Reviewed artifact: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`
(untracked at review time).
Requirement set: `.superpowers/sdd/plan-12/plan-brief.md` plus
`.superpowers/sdd/plan-12/plan-brief-addenda.md`, assembled independently rather
than read off the plan's own requirement table.
Ground truth walked, in the plan's own precedence order: `docs/ROADMAP.md`'s
"OWNER QA PASS, round 3" entry in full (plus the Docs-accuracy entry it spawned
and the example-validation vehicle), the v1 spec, the four house-knowledge YAML
files.
Tree state at review: `148f19f`, working tree clean apart from the untracked plan
file. This is the same commit the plan cites, so every claimed measurement was
re-runnable against the identical source.
Independent instruments: all reproductions were built at
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr12-review-independent/`
(seed fixtures `s1.yaml`-`s5.yaml`, the in-place-mutation fire fixture
`synthetic.ts`, a self-contained Playwright project under `probe/` with its own
config). No instrument, fixture or config of the author's was executed.

---

## Overall verdict: NEEDS_FIXES

The plan is unusually strong on the dimensions it was warned about: every one of
the nineteen brief decisions and every one of the five addenda's additions has a
settled decision and a named implementer, the acceptance map's half count is
exactly right (63, recounted per work item: 10 + 12 + 20 + 19 + 2), the seed is
chosen on a measurement that reproduces digit for digit, the ruled explicit
`pattern: ".*"` form is used with no defaulted variant assumed anywhere, D66 and
spec 8.2's "Remove deletes the selected rule without confirmation" are left
intact and explicitly fenced, and the out-of-scope derivation package appears
only as two information duties with no seam, parameter, key or test prepared for
it. Fifteen of the seventeen claimed measurements I could re-run reproduced
exactly, including the ones that were easiest to fake. **What blocks approval is
concentrated in W4b, the shell half**, and it is the same defect class the addenda
file records the controller committing: the safeguard that replaced a
green-before-and-after check is itself green under both of its own prescribed red
states, because the assertion is made through a lookup that carries an English
fallback, so neither a de row pointed at the en catalog nor a deleted de key can
make it fire; alongside it, Task 6 Step 3 still carries a paragraph from the
plan's pre-ruling draft stating that the shell reads en only and a German user
sees English, which contradicts D110 and Step 1b inside the same task. Two
further structural gaps are independent of that cluster: no task step or Files
region ever writes `savedSnapshot` on a successful save, so the derived save state
never clears and the guard family stays armed after a save, and the history's
behaviour on a failed open is undefined while the Undo button sits outside the
model gate. The rest is count drift and two mis-stated enumerations, which matter
here only because this plan polices that class explicitly and its self-review
claims the sweep was done.

Findings: **7 major, 6 minor, 2 nits.**

---

## Findings

### F1 (MAJOR) Task 6 Step 3 contradicts D110 and Step 1b inside its own task

**Location.** Task 6, Step 3 ("the catalog, both locales, fenced, single-line by
the shell's own parser constraint"), the paragraph immediately after the two
fenced catalog blocks, beginning "These are read by the shell from the EN file
only".

**What is wrong.** The paragraph states that the six new strings "are read by the
shell from the EN file only, so a German user sees the English text - the
pre-existing, recorded limitation the de header already anticipates" and that "the
de values exist for parity and for a later shell i18n, exactly as the
`close-abort-*` ones do." Step 1b of the same task makes the lookup locale-aware,
D110 decision 1 and 2 make the shell render in the locale the frontend pushed, and
the owner's ruling of 2026-07-30 (addendum 5) forbids exactly the disposition this
paragraph asserts. It is a survival from the plan's own first draft, which D109's
fourth rejected alternative records as OVERRULED. An implementer reading its task
top to bottom meets the ruling in Step 1b and its reversal in Step 3.

**Evidence.** The two statements are in the same task, four steps apart. The
plan's own D109 rejected-alternatives list names this position as overruled, and
its corrections table row 6 records the reversal. The de catalog header the
paragraph appeals to (`locales/de/gui-common.ftl`, its `###` block) does say the
`close-abort-*` strings "are not yet shown to a de user ... kept single-line and
translated for parity and a later shell i18n" - which this package consumes rather
than continues, as the plan's own close-action list acknowledges ("the de catalog
header's 'a later shell i18n' note, now consumed rather than open").

**Ruling.** FIX. Delete the paragraph and replace it with what is now true: the six
strings are read through the locale-aware lookup, they must stay single-line and
column-0 in both locales because that parser constraint is unchanged, and the de
header's note is consumed by this task rather than extended. The Task 6 **Must not
decide** list already carries the correct position, so nothing else moves.

---

### F2 (MAJOR) Both prescribed red states of the shell parity test are green by construction

**Location.** D110 decision 4; Task 6 Step 5, third bullet ("Add the shell parity
test (D110 decision 4), derived rather than hand-listed"); acceptance rows W4-q
and W4-r.

**What is wrong.** Half (b) of the test asserts that each shell-consumed key
"resolves - through the shell's own lookup - to a non-empty value that is not the
key, in every shipped locale". Step 1b defines that lookup as: primary subtag, try
that row's line lookup, **then the `en` row's**, then the key. The en fallback is
inside the instrument being used to test. Therefore:

- Red state 1, "the `de` row's table entry pointing at the en catalog": every key
  resolves to the English value. Non-empty, not the key. **The test passes.**
- Red state 2, "one de key deleted": the de row's line lookup misses, the en row's
  hits, the key resolves to the English value. Non-empty, not the key. **The test
  passes.**

So the check cannot distinguish "the shell serves German" from "the shell falls
back to English for every string", which is precisely the state D110 exists to
end. The residual it claims to cover ("(b) catches a key added to the shell
without a German value or written multiline in de only") is not covered either,
for the same reason. Only half (a), the directory-versus-table row check, has a red
state that fires - and its red state (the missing-row mutation, W4-r) is a
different mutation from the two Step 5 prescribes.

This is the second instance of the class in two turns: the addenda file records the
controller demanding a check that "would have been green before AND after the fix",
and the replacement safeguard reproduces the property one layer down. The plan's own
Global Constraints bind it (`proc-verification-step-must-be-falsifiable`,
`proc-check-green-state-reachable`), and the plan asserts in its self-review that
the test carries "its two prescribed red states".

**Evidence.** `src-tauri/src/run.rs`'s existing `ftl_message` and its test
`close_abort_strings_resolve_from_the_ftl_catalog` show the assertion shape the new
test inherits verbatim: `assert_ne!(value, key, ...)` plus `assert!(!value.is_empty())`.
Step 1b's fallback chain is stated in the plan as `[requested, en]`. Both red states
resolve through that chain to the en value.

**Ruling.** FIX, and the fix is in the assertion, not in the red states. Two changes,
both keeping the derivation and neither removing a safeguard:

1. Half (b) asserts against the **single row**, not through the fallback chain:
   for every `(subtag, catalog)` row and every derived key, the line lookup **in
   that row's catalog alone** must yield a non-empty value. Red state 2 then fires.
2. Add the half that defeats red state 1: pin one de value the way the existing test
   pins the en one (`ftl_message("close-abort-title", "de")` must equal
   `Laufende Jobs abbrechen`), or assert each row's catalog content equals the file
   read from `locales/<subtag>/gui-common.ftl` at test time. Either makes a de row
   aimed at the en catalog fail.

Then re-run both prescribed red states and paste the failures, which is what Step 5
already demands and what would have caught this.

---

### F3 (MAJOR) The user-visible half of D110 has no producing check

**Location.** Acceptance map rows W4-n through W4-s; Task 6 Step 5 and Step 6.

**What is wrong.** D110's consequence for a user is one observable: a German user
who quits with unsaved changes reads a German dialog. Walked in halves, that
observable has a wire side and a rendered side. The wire side is covered twice
(W4-o startup, W4-p live switch, both asserting the recorded `set_shell_locale`
call). **The rendered side has no producer at all.** W4-n extends the existing
string test and Step 5 has it "passing `"en"` explicitly"; W4-q/W4-r are the parity
test of F2; nothing asserts that any key resolves to its German value. The plan's
brief is explicit that one producer named for the whole observable while covering
one side "is how a gap survived two review rounds on Plan 9".

**Evidence.** Task 6 Step 5's first bullet: the extended enumeration keeps "its
pinned reference-wording assertion and passing `"en"` explicitly now that the
lookup takes a locale". No row in the acceptance map names a de value. Searched the
plan for a de-side assertion on the shell: the only de assertions in the whole plan
are frontend ones (W1-a/b/e/g, through the `de()` helper and de-only literals).

**Ruling.** FIX. Add an acceptance half and its producer: a Rust unit case pinning at
least one German shell value through the locale-aware lookup. If F2's fix takes the
value-pinning form, that case discharges both and the acceptance map gains one row
naming it. Do not discharge this by pointing at `pnpm check:i18n`: that gate checks
id parity, which was already green while the German values shipped dead - the exact
finding the addenda file records.

---

### F4 (MAJOR) `savedSnapshot` is never written on a successful save, so the derived save state never clears

**Location.** D108 decision 3; Task 4 Files list and Steps 1-6; Task 6 Step 6 and
acceptance rows W3-q and W4-h.

**What is wrong.** D108 decision 3 decides that "A successful save sets
`savedSnapshot` to the current entry", and decision 4 derives `dirty` from it. No
task step prescribes that write, and no task's Files list admits the region that
would carry it:

- Task 4's Files entry for `src/views/EditorView.vue` enumerates "the history state,
  the push rule inside the existing watcher, the coalescing boundary, undo/redo
  functions, the keyboard handler, `sessionActive` becoming a computed, the two
  buttons". `doSave` is not in it, and the Files lists are declared EXHAUSTIVE.
- Task 4's steps never mention the save path except in a test.
- Task 6's Files entry for the same file is "(one watcher on `dirty`)".
- Task 3 rewrites `doSave` wholesale, but it runs before the history exists.

Consequence on the deliverable: after a save, `history[position]` still differs
from `savedSnapshot`, `dirty` stays true, and every guard in the D109 family fires
on a profile that was just saved - a false warning on the most common path in the
feature.

Consequence on coverage: W3-q ("Saving marks a position rather than clearing the
history") names one producer for a two-sided observable and covers the side that
passes anyway. Its producer is "Task 4, undo still available after a save", which is
green whether or not `savedSnapshot` moves. The side that would detect the omission
is W4-h ("the recorded `set_editor_dirty` calls, true then false across a save"),
which lands in a later task whose Files list also excludes the fix, so the first
signal is a red test in Task 6 against code Task 4 owns.

**Evidence.** Read `doSave` in the current tree: `if (saveDisabled.value || !model.value || !currentPath.value) { return; } ... await saveProfile(...)` and nothing else. Task 3's
fenced replacement adds the dialog branch and the recents write, no snapshot. Task 4
Step 6's own wording for the case is "open, edit, Save (mocked), then Undo is still
enabled and one Undo restores the pre-edit state" - both assertions hold with
`savedSnapshot` frozen at the load baseline. The parity precedent the plan adopts
does the write at the save site: `mkvtoolnix-gui`'s `Tab::onSaveConfig` sets
`p.savedState = currentState()` after `p.config.save()`.

**Ruling.** FIX, three parts. (a) Task 4's Files entry for `EditorView.vue` gains
`doSave`'s post-write region, and a step fences the assignment (set `savedSnapshot`
to `history[position]` after `saveProfile` resolves, inside the existing `try`, so a
failed save leaves the state dirty). (b) W3-q splits into its two halves, the second
naming a check that the save state clears - assert `dirty`'s observable proxy in the
same test rather than deferring it to Task 6. (c) Keep W4-h as the wire half.

---

### F5 (MAJOR) The history's behaviour on a failed open is undecided, and Undo is reachable with no model

**Location.** D108 decision 8; Task 4 Step 1's `resetHistory(profile: Profile)`
bullet and Step 4's button placement.

**What is wrong.** Two joined gaps, both latitude by omission.

1. `resetHistory` takes a `Profile`. `openPath` assigns
   `model.value = doc.profile ?? undefined`, so the profile is absent whenever a
   load succeeds at the IPC level but returns no profile (a parse failure carrying
   its diagnostic, which is a supported and tested state). Task 4 says the two
   funnels "drop their `sessionActive.value = true` in favour of establishing the
   baseline" and says nothing about the no-profile branch. The implementer must
   invent one of at least three behaviours: leave the previous history standing,
   clear it, or skip the call. D108 decision 8 covers only "Opening or creating a
   profile".
2. Whichever is invented, the second gap bites: Step 4 places Undo and Redo "in the
   action row after New and Open", and that row sits **outside** the
   `<template v-if="model">` gate that wraps the editing surface. So after a failed
   open over an existing history, Undo is enabled with `model` undefined and
   `currentPath` pointing at the file that failed to parse. One click restores the
   previous profile; Save then writes it to the broken file's path, since
   `saveDisabled` no longer consults anything but `model`, `hasErrors`, `saving` and
   `opening` after Task 3.

**Evidence.** `openPath` in the current tree: `currentPath.value = path;` then
`diagnostics.value = doc.config_diagnostics;` then
`model.value = doc.profile ?? undefined;`, all before the recents write. The
editing surface begins at `<template v-if="model">` in the template; the New/Open
button block and the recents section precede it, and Task 4 Step 4 puts the new
buttons in that preceding row. `saveDisabled` after D107 decision 3a is
`!model.value || hasErrors.value || saving.value || opening.value`.

**Ruling.** FIX. Decide the failed-open case explicitly in D108 (recommended: a
failed load resets the history to empty and `savedSnapshot` to `null`, which also
restores `sessionActive` to false and keeps the pre-existing bare-mount property),
and gate the two buttons - or the two functions - on `model` so neither can apply a
history entry while the editor holds nothing. Add the case to Task 4's step list
and one acceptance half for it.

---

### F6 (MAJOR) Absence check L1's prescribed red-state count and its soundness control are both off by one

**Location.** Task 2 Step 6, first bullet ("Absence check L1, the single resolution
rule"); acceptance row W1-i.

**What is wrong.** The step prescribes
`grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"` and states
"**RED, run FIRST on the pre-state: exactly 1 line**, in `src/main.ts`". Measured on
`148f19f`: **2 lines**, both in `src/main.ts` (`resolveLocale`'s try branch and its
catch branch). The soundness control fares the same way: it states that the
unfiltered expression "must return exactly 1 line, in `src/i18n/index.ts`", but
`src/i18n/index.ts` already contains the token in a doc comment, so after Step 1 adds
`return saved ?? navigator.language;` the unfiltered run returns **2** lines there,
not 1.

Both numbers are the kind an implementer is told to treat as a gate. Whoever runs
L1 will read a disagreement with the plan and either report NEEDS_CONTEXT on a
non-finding or, worse, adjust the fence.

**Evidence.**

```
$ grep -rn "navigator.language" src/
src/main.ts:17:    return (await getSettings()).locale ?? navigator.language;
src/main.ts:19:    return navigator.language;
src/i18n/index.ts:33: * lowercased. A saved setting or `navigator.language` is often
```

Filtered on `src/i18n/index.ts`: 2 lines. Unfiltered hits inside
`src/i18n/index.ts` today: 1 (a comment), which becomes 2 after Step 1's addition.

**Ruling.** FIX both figures: pre-state 2, control 2 (or scope the control to the
function body rather than the file). Since `resolveLocale`'s two branches are the
two occurrences and Step 2 rewrites both, the reachable-green argument the step
makes stays intact once the count is right.

---

### F7 (MAJOR) The string-surface enumeration presents a row its own expression cannot produce, and its blind spot is unprobed

**Location.** Authoring-time verification, "THE STRING-SURFACE SET, derived from the
tree"; R40; the self-review paragraph "The string surfaces, enumerated from the tree
rather than from recall".

**What is wrong.** The section states an expression, calls what follows "Full output,
six sites in five files", and lists among those sites
`src/i18n/index.ts:17 import.meta.glob(locales/*/gui-*.ftl, diagnostics.ftl) -> the frontend surface`.
That expression cannot return that line: the call's argument array is on the next
line, and the pattern requires `locales` or `help` on the same line as the opening
paren. Re-running the expression verbatim returns nine lines in four files, and
`src/i18n/index.ts` is not among them. The largest surface in the package was
recovered from the author's knowledge, not from the instrument - which is exactly
the failure mode `a-search-whose-terms-come-from-memory-produces-a-false-absence`
and `proc-sweep-surface-completeness` name, and the plan cites both. The fired
control (the same expression over `EditorView.vue` returning 0) proves the pattern
discriminates a loading site from an ordinary file; it cannot and does not prove
completeness, and the one blind spot it has - a multi-line invocation - is where the
miss happened.

The plan's Global Constraints also bind this directly: "every observed value in a
task report is pasted from the run that produced it, never recalled, and never
attributed to a command that was not the one run".

**Evidence.** My re-run of the stated expression:

```
crates/muxsmith-cli/src/i18n.rs:7,8,9,10   include_str! en/de {diagnostics,cli}.ftl
scripts/check-i18n.mjs:110,538,568         join(ROOT,"locales"|"help"), readFileSync
src-tauri/src/run.rs:519                   include_str! en/gui-common.ftl
src/help/topics.ts:7                       import.meta.glob("../../help/*/*.md", {
```

and the site the table claims:

```
src/i18n/index.ts:17:const catalogSources = import.meta.glob(
src/i18n/index.ts:18:  ["../../locales/*/gui-*.ftl", "../../locales/*/diagnostics.ftl"],
```

**The conclusion survives.** I ran a multi-line-tolerant sweep of my own over every
tracked `.rs`/`.ts`/`.mjs`/`.vue`/`.json`/`.js` outside tests
(`grep -nE '"[^"]*(locales|help)/'`) and it returns the same five surfaces and no
sixth. So the five-surface set and the "this package adds to exactly three" claim
are correct; only the evidence for them is misattributed.

**Ruling.** FIX the evidence, not the conclusion. Either widen the expression so it
actually returns the frontend site (drop the same-line coupling, or run a second
expression aimed at the multi-line form and paste it as the blind-spot probe the way
Task 4's mutation enumeration does), or mark the frontend row explicitly as recovered
outside the expression. Then state the blind spot in the sentence that presents the
set.

---

### F8 (MINOR) Four normative counts in the plan are stale, three of them contradicted inside the same document

**Location.** Task 1 Interfaces ("the four decision records"); the corrections
section's lead sentence ("None of the six changes a ruling"); the plan-close ROADMAP
disposition bullet ("the corrections table's six items"); the self-review counts
paragraph ("38 requirements (counted from the requirement table's rows)").

**What is wrong.** Measured against the enumerations they summarise:

| Claim | Site says | Measured | Contradicted at |
|---|---|---|---|
| ADR set | four decision records | 5 (D106-D110) | six other sites in the plan say five |
| corrections table | six | 7 rows | self-review says "Refutations. Seven" |
| corrections table | six items | 7 rows | same |
| requirement table | 38 | 41 rows, max `R41` | self-review's own coverage paragraph says "all 41" |

The self-review asserts these were "recomputed from their own enumerations at
authoring rather than recalled", and `proc-normative-count-recomputed` is in the
plan's own Global Constraints. The halves count (63) and the addenda count (five)
are correct at every site, so the sweep was partial rather than absent.

**Evidence.** `grep -cE '^\| R[0-9]+ \|'` -> 41, highest `R41`. Corrections rows
counted from the table -> 7. Acceptance rows per work item -> 10, 12, 20, 19, 2 = 63,
which reproduces the plan's figure exactly.

**Ruling.** FIX all four. Worth noting for the controller rather than only fixing:
the same class is live in the controller's own artifact - `plan-brief-addenda.md`
opens with "Four addenda reached the plan author" while carrying five numbered
addenda.

---

### F9 (MINOR) "Exactly one hit on the locale control" is two hits

**Location.** Authoring-time verification, "The existing locale assertion, and the
brief's premise about it refuted"; D106 decision 7; Task 2 Step 6's last bullet;
acceptance row W1-h; Task 2's **Must not decide** list.

**What is wrong.** The plan states that `grep -rn "toHaveValue" e2e/*.ts` "returns
exactly one hit on the locale control". It returns two, both inside
`test.describe("german locale")`'s settings-save case: `toHaveValue("en")` on
`localeSelect` before the save, and `toHaveValue("de")` on `reloadedLocaleSelect`
after the reload against the `DE_SETTINGS` mock. The second is never mentioned. Its
disposition happens to be the same (a stored `"de"` still displays as `"de"` under
shape A, so it stays valid), so nothing breaks - but the enumeration is in a
normative position: D106 decision 7, the acceptance row and the **Must not decide**
list all speak of "the existing assertion", singular, and Task 2's report duty names
one.

**Evidence.**

```
e2e/smoke.spec.ts:826:    await expect(localeSelect).toHaveValue("en");
e2e/smoke.spec.ts:848:    await expect(reloadedLocaleSelect).toHaveValue("de");
```

with `DE_SETTINGS.locale = "de"` supplied to the post-reload `get_settings` mock.

**Ruling.** FIX the count and name both assertions with their shared disposition.
The refutation of brief decision 6 itself stands and is correct.

---

### F10 (MINOR) The README example defect is already owner-ruled to a vehicle; the plan proposes to route it as new

**Location.** Corrections table row 7; plan-close verdict-harvest bullet.

**What is wrong.** The plan reports the README's first example failing to load as a
finding that is "out of this package's scope, SURFACED for routing, not fixed by any
task", and the close lists it among the harvest inputs. `docs/ROADMAP.md` at the
plan's own baseline already carries it in the Docs-accuracy section - recorded as
"surfaced by the Plan-12 author 2026-07-30", mechanism confirmed by the controller -
with its vehicle RULED: "Plan 11, folded into the fix round its plan review
produces", and with the owner's ruling attached (he rejected a serde default for
`Input::pattern` as magic; the example gains the line). A second ROADMAP entry from
this plan's close would duplicate a ruled disposition.

**Evidence.** `git show HEAD:docs/ROADMAP.md` contains both the
"THE README'S FIRST EXAMPLE PROFILE DOES NOT LOAD" entry and the
`Input::pattern` ruling, so the record predates the plan rather than following it.
The same section's neighbour, "Every documented example is validated against the
real binary (owner-approved 2026-07-30)", is the checker vehicle and is ruled
pre-1.0.

**Ruling.** FIX the disposition wording: cite the existing entry and its ruled
vehicle instead of surfacing the item, and drop it from the harvest list. The
measurement itself reproduces exactly (see reproduction R14) and is worth keeping as
a confirmation.

---

### F11 (MINOR) `ConfirmDiscard`'s confirm action is "an exit" with no exit code fenced

**Location.** Task 6 Step 2.

**What is wrong.** "on confirmation runs `abort_and_quit` for the two run-bearing
variants and an exit for `ConfirmDiscard` (no run exists, so there is nothing to
abort)". The exit code is not written down. The existing site passes a code through
(`abort_and_quit(&app.state::<AppState>(), |code| app.exit(code))`), so an
implementer must invent the literal. Small, but it is the omission form the plan bans
absolutely: a step requiring a value not written down.

**Ruling.** FIX: fence the call (`app.exit(0)`), or state that the discard-only path
routes through the same closure with a named code.

---

### F12 (MINOR) R28's "no second boolean" has no absence check, though its red state is already measured

**Location.** R28; D108 decision 4; the self-review's absence-check enumeration
(L1, E1, E2, U1, G1, G2, H1).

**What is wrong.** The requirement is absence-shaped ("the save-state gate is DERIVED
from the undo history, not maintained as a second boolean") and the plan already
holds its red state: the ROADMAP's and the plan's own measurement that
`grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue` returns nothing
today, with a control proving the pattern matches a real change-tracking
implementation (`mkvtoolnix-gui`'s `tab.cpp`). No end-state expression is prescribed,
so nothing detects a second flag introduced during a fix round.

**Ruling.** FIX by adding one, since a proposed safeguard stays and this one costs a
line: an end-state expression over `EditorView.vue` whose expected hits are exactly
the derived `dirty` computed and the `savedSnapshot` ref, fired against the pre-state
zero the plan already measured.

---

### F13 (MINOR) The 430-byte snapshot figure does not reproduce, and its method is unstated

**Location.** Authoring-time verification, "A snapshot is small"; D108 decision 5.

**What is wrong.** The seed's 101 bytes reproduce exactly. The companion figure, "the
README's four-rule example profile to 430 bytes", does not reproduce under the
obvious reading, and the plan does not say which serialization it measured. Compact
JSON of that YAML block as written: **492 bytes**; with the missing `pattern: ".*"`
added so it would actually load: **507 bytes**. 430 is consistent with the
serde-normalized wire form (defaults such as `recursive: true` and the `output`
block's two default fields omitted per D48), which is in fact the right form for the
claim, since a history entry is `JSON.stringify` of a model that arrived over the
wire. So the figure is probably correct and the method is missing.

**Evidence.** My run over `README.md`'s two fenced YAML blocks: block 0 (four rules)
-> 492 bytes compact JSON, 507 with `pattern` added; block 1 (passthrough) -> 186.

**Ruling.** FIX by stating the method in one clause ("compact JSON of the
serde-normalized model, defaults omitted per D48"), or re-measure and paste. The
memory statement R31 owes is unaffected either way.

---

### N1 (NIT) "Unreachable in a real session" overstates the pre-push dialog window

D110 decision 3 names the residual correctly (a dialog firing before the frontend
has pushed renders en) and then argues it is "unreachable in a real session (the
locale is applied before mount and the window shows the app)". The window is shown
by the shell, not by the webview's bootstrap, so a `CloseRequested` in the first
frames is reachable in principle. The direction is benign and the residual is named,
which is what matters. Soften the claim to "not reachable in practice" rather than
"unreachable".

### N2 (NIT) Two acceptance halves that exist in substance have no dedicated row

W2's three new editor ids and W4a's three discard ids get no catalog-parity row of
their own, where W1 (W1-j) and W3 (W3-t) do; both are in fact covered, by the gate
that every task runs and by rendered assertions (W2-e, W4-a). Likewise "New renders
immediately before Open" is an SI-3-derived rendered observable with no assertion.
Not worth a row each, but the asymmetry is worth one sentence in the coverage note so
a later reader does not read it as a gap.

---

## Reproductions

Every figure below was produced by an instrument built at the review path named in
the header. "Reproduced" means my run and the plan's stated value agree exactly.

| # | Claim in the plan | My measurement | Verdict |
|---|---|---|---|
| R1 | S1 schema minimum: `empty-extensions` + `no-track-rules`, both error, exit 2 | identical, exit 2 | reproduced |
| R2 | S2 minimum + empty rule: `empty-extensions` error + `empty-match-expression` warning, exit 2 | identical, exit 2 | reproduced |
| R3 | S3 `pattern ""` + `["mkv"]` + empty rule: one warning, exit 1 | identical, exit 1 | reproduced |
| R4 | S4 `pattern ".*"` + `["mkv"]` + empty rule: one warning on `tracks[0].match`, exit 1 | identical, exit 1 | reproduced |
| R5 | S5 passthrough: one `passthrough-profile` info, exit 0 | identical, exit 0 | reproduced |
| R5b | The seed choice S4 is what the measurement supports, and `pattern` is written explicitly | S3 and S4 are both diagnostic-free; the `.*`-over-`""` tiebreak is a stated design argument, not a measurement, and the plan says so. `Input::pattern` has no `#[serde(default)]` and `Input` carries `deny_unknown_fields`, so the explicit form is the only loadable one - matching the owner's ruling against a default | reproduced; ruled form used |
| R6 | `currentPath`: one write site (`openPath`), six read duties | one write at the `openPath` assignment; reads at `saveDisabled`, the `watch(model)` gate, `doSave`'s re-guard, `doSave`'s save target, the template path line, the recents gate = 6 | reproduced |
| R7 | Editing is gated on `model`, not `currentPath` (correction 1) | `<template v-if="model">` wraps the editing surface | reproduced |
| R8 | Seven whole-value `model.value =` assignments; six mutation functions plus `openPath`; no in-place mutation | 7 assignments; enclosing symbols `openPath`, `setFieldValue`, `setTracksUnmatched`, `setRuleValue`, `onDrop`, `addRule`, `removeSelectedRule`; in-place expression returns nothing, exit 1; my own fire fixture matches all three synthetic mutation forms and skips a read-only line | reproduced, fire independently confirmed |
| R9 | `model = defineModel<Profile>()`, no `v-model` on `<EditorView>` in `App.vue` | confirmed; `<EditorView v-show="activeView === 'editor'" />` | reproduced |
| R10 | Catalog counts: `gui-editor` 46, `gui-settings` 8, `gui-common` 38, identical en/de; 42 registry `labelKey` ids | identical, all four | reproduced |
| R11 | Two live comments still say 45 (`EditorView.vue`, `smoke.spec.ts`) | both found; the smoke comment's decomposition is `42 labels + 1 save-surface note + 2 generic action keys`, consistent with 46 = 42+1+2+1 | reproduced |
| R12 | Budget 46 -> 54 needs 8 new editor ids; 15 new ids across 3 catalogs; 30 lines | the two itemization tables list 8 and 7 rows = 15, one row per id, each strikeable; 46+8 = 54; `gui-batch` unchanged at 28; Tier-2 `editor-generic-action-keys` records 46 and its two prior revisions as owner ruling / owner-approved design | reproduced; itemization complete |
| R13 | Highest D-number in use is D105; D106-D110 free | highest D105; the D106-D110 pattern returns nothing outside plan-12 artifacts; control `D10[0-5]` matches 14 times in ROADMAP | reproduced |
| R14 | The README's first example fails: `input: missing field 'pattern' at line 4 column 3`, exit 2; the second validates clean | identical message and column, exit 2; second block clean | reproduced |
| R15 | `check-i18n.mjs` parity covers ALL `.ftl` in `locales/en/`; `RUST_ONLY_IDS` names the four `close-abort-*` keys; unused-id check is warning-only, parity is a hard failure | all four confirmed, including the script's own "Scope here is deliberately ALL `.ftl` files in locales/en/" comment and the `console.warn("... (warning only)")` branch | reproduced |
| R16 | `run.rs`: `include_str!` of en only, single-line `ftl_message`, two-variant `CloseDecision`, `close_decision` reads only the run slot, the two named unit tests, the pinned `close-abort-title` wording, the never-prefix-match companion, four literal `ftl_message` call sites | all confirmed | reproduced |
| R17 | `AppState` already carries an `AtomicBool` (`quit_after_finished`) | confirmed, with `Mutex` and `AtomicBool` already imported | reproduced |
| R18 | Capabilities grant `dialog:allow-open`, `dialog:allow-save`, `clipboard-manager:allow-write-text`, `os:default`, `fs:allow-write-text-file`, `core:default`, `core:event:default`; not `dialog:allow-message` | identical | reproduced |
| R19 | `@tauri-apps/plugin-dialog@2.7.1`'s `confirm` routes through `messageCommand` -> `invoke('plugin:dialog|message')` and compares against the ok label (correction 4) | identical, version 2.7.1 | reproduced |
| R20 | `applyLocale` has exactly two callers (`main.ts` bootstrap, `SettingsDialog.save()`); `currentLocale`'s only consumer is `HelpSidebar.vue` | identical | reproduced |
| R21 | A describe-level `test.use({ locale })` overrides `navigator.language` for that describe only | my own probe project: PRE-sibling `en-US`, inside override `de-DE`, POST-sibling `en-US`, 3 passed. I checked a sibling on **both** sides, which the plan's probe did not | reproduced, strengthened |
| R22 | `focusout` bubbles, `blur` does not; `fill()` dispatches `input` and `change` | my probe: `["a:input","a:change","root:focusout"]`, no `root:blur` | reproduced |
| R23 | mkvtoolnix: `actionMergeNew` first, before Open/Save, `&New`, `Ctrl+N`, wired to `Tool::appendNewTab`; empty state carries `newFileButton`/`openFileButton` under "No multiplex job has been opened yet." | all confirmed | reproduced |
| R24 | `Tab::onSaveConfig` delegates to `onSaveConfigAs` when the config filename is empty | confirmed verbatim | reproduced |
| R25 | No undo/redo anywhere in mkvtoolnix-gui; controls: 7 files match "multiplex" in `merge/`, 15 files match `QMessageBox` | both control figures identical; `QUndoStack|QUndoCommand|QUndoGroup` returns nothing; `hasBeenModified() { return currentState() != savedState; }` confirms the parity shape D108 adopts | reproduced |
| R26 | `crates/muxsmith-cli/src/i18n.rs`'s `LOCALES` table shape, its "no glob form" rationale, `Renderer::new`'s `--locale > sys_locale > en` cascade with primary-subtag collapse and a deduplicated `[requested, en]` chain; `sys-locale` is a CLI-only dependency | all confirmed | reproduced |
| R27 | The seed serializes to 101 bytes of compact JSON | 101 bytes | reproduced |
| R28 | The README's four-rule example serializes to 430 bytes | 492 as written, 507 with `pattern` added; 430 is consistent only with the serde-normalized wire form, which the plan does not name | **not reproduced as stated** (F13) |
| R29 | `grep -rn "toHaveValue" e2e/*.ts` returns exactly one hit on the locale control | **two** hits (`"en"` pre-save, `"de"` post-reload) | **different figure** (F9) |
| R30 | L1's pre-state: exactly 1 `navigator.language` line outside `src/i18n/index.ts` | **2** lines, both in `src/main.ts`; the control's expected post-state count is also 2, not 1 | **different figure** (F6) |
| R31 | The string-surface expression's full output is six sites in five files including `src/i18n/index.ts:17` | the expression returns nine lines in four files and cannot return that row; an independent multi-line-tolerant sweep confirms the five-surface conclusion | **evidence not reproduced, conclusion confirmed** (F7) |
| R32 | The gate part-count audit currently returns one line, the self-audit sentence | 1 line, that sentence | reproduced |
| R33 | 63 acceptance halves, 10 + 12 + 20 + 19 + 2 | identical per work item and in total | reproduced |
| R34 | The D62 help gate carries six conditions | the script's own numbered comment enumerates exactly those six | reproduced |
| R35 | H1's expression is clean on the two topics today and fires against a document carrying a URL and a pipe | topics: nothing, exit 1; `docs/INSTALL.md`: 9 hits | reproduced |
| R36 | `e2e/mocks.ts`'s default `get_settings` returns `locale: "en"` | confirmed | reproduced |
| R37 | The fenced spec anchors and the fenced catalog originals match the tree byte for byte | spec 8.2's editor sentence, its final sentence, the app-settings paragraph, and both `settings-locale-label`/`.hint` blocks all match exactly; both existing `<option>`s carry explicit values; `select#settings-locale` exists; the `mkvmerge_path` `?? ""` / `=== "" ? null` sibling pattern D106 cites is real | reproduced |
| R38 | `RunHistory.saveLog`'s capture-before-the-dialog-gap comment; `batch-profile-filter-name` reuse; `rememberRecentProfile` returning settings | all confirmed; `EditorView` already uses `fluent.$t("batch-profile-filter-name")` for its open dialog | reproduced |
| R39 | The three `editor-save` assertions in `e2e/editor-rule-add-remove.spec.ts` all run after an Open | all three are in the served-app cases, each preceded by an `editor-open` click | reproduced |
| R40 | `e2e/.generated/` would not disturb a task's `git diff --stat` | gitignored and untracked, 0 tracked files | reproduced (not claimed, checked) |
| R41 | Every Tier-2 id the plan cites exists | all 28 cited ids resolve in the four house YAML files | reproduced |

**Prescribed evidence graded as design rather than run.** L1 (F6, figures wrong,
mechanism sound), E1/E2 (fires named in the neighbouring state of the same test,
sound), U1 (run twice, once passing through and once firing - the strongest of the
set), G1/G2 (fires named on the same counter and locator, sound), H1 (fire verified
against a real document by me), Task 1's sweep (three expressions with a stated
must-return rather than a zero, sound), Task 6 Step 6b's pre/post `check:i18n` pair
(sound: the pre-state run must name exactly the six ids), Task 4's mutation
enumeration with its blind-spot expression (sound, and I fired the blind-spot
expression myself). **The one unsound prescribed check is the shell parity test's
half (b): both of its red states are green, see F2.**

---

## Harvest for the controller

Surfaced only; I wrote nothing to the house-knowledge files.

**Dominant patterns, each now at two or more instances.**

1. **The hand-written per-locale `include_str!` table.** `crates/muxsmith-cli/src/i18n.rs`
   carries it with a recorded rationale ("`include_str!` is compile-time and has no
   glob form"); Task 6 gives `run.rs` the same shape. That makes it a house pattern
   with two instances and one shared defect: a new `locales/<tag>/` directory is
   silently unserved by every such table. This plan closes the shell's half with a
   check and leaves the CLI's trigger-remembered, which is a deliberate controller
   routing decision - but the pattern plus its standing gap is now a Tier-2
   candidate in its own right, with the discriminator being "an embedded catalog
   table needs a directory-versus-table parity assertion", not "the shell needs one".
2. **Capture state before the dialog gap.** `RunHistory.saveLog` documents it in
   prose; Task 3's `doSave` conforms and says so. Second instance of a named pattern
   whose failure mode is silent (the model changes under an open native dialog).
3. **Tolerant background bookkeeping IPC writes.** `rememberRecentProfile`, the
   editor's validation watcher, BatchView's recents write, and now
   `set_editor_dirty` and `set_shell_locale`. The house form is settled: swallow,
   log, name the consequence. Worth recording as a pattern with its own boundary
   (never for a write whose failure the user must act on).
4. **The nullable-field-to-empty-string sentinel in a form.** `mkvmerge_path` already
   loads `?? ""` and saves `=== "" ? null`; D106 gives `locale` the same treatment
   with a named constant. Second instance, and the plan's reasoning for it (a BCP-47
   tag can never be the empty string) generalises to "the sentinel must be outside
   the field's value space".

**Repeated rejections.**

5. **Composing localized prose from fragments** is rejected again (D109 decision 5's
   combined close message gets its own key rather than a concatenation of the two
   single-fact ones). This is at least the second time the i18n architecture's ban
   has had to be restated in a design decision.
6. **A hand-written key list where a derivation is available** is rejected twice in
   one task: D110 decision 4 insists the shell's consumed key set be derived from
   source, and Step 5 explicitly forbids simplifying it into a list during a fix
   round. `RUST_ONLY_IDS` in `check-i18n.mjs` is the counter-example the rejection
   reacts to.
7. **A second JS check script** is rejected again (D110's third rejected alternative
   cites the D62 decision that killed `check-help.mjs`), with a new and sharper
   reason: a JS check would have to re-implement the shell's own lookup to test it.
8. **A serde default on a field that selects the input set** is now ruled twice over
   (`Input::pattern`, owner 2026-07-30) and cited by two vehicles - this plan's seed
   and the example-validation checker's marker-versus-heuristic question. The
   controller's recorded boundary reading (fields that decide what the bulk operation
   acts on stay explicit; fields that tune how it acts may default) held up against
   everything I read in this plan.

**Defect classes recurring despite an explicit standing rule, which is the part
worth the controller's attention.**

9. **A prescribed red state that is green.** The addenda file records the controller
   demanding a check that would have been green before and after the fix. Its
   replacement, authored by the party that refuted it, has the same property for
   both of its own red states (F2). Awareness of the class did not prevent the
   second instance, which is the signature of a rule that needs a handle rather than
   a warning. The handle this case suggests is mechanical and readable: **where the
   mechanism under test contains a fallback, a red state must defeat the fallback -
   test the single row, not the chain.** That is derivable from the artifact (does
   the lookup have a fallback clause?) rather than from noticing.
10. **Count drift in restated normative figures.** Four instances in this plan (F8),
    three of them contradicted by a correct statement of the same figure elsewhere
    in the same document, in a plan that carries
    `proc-normative-count-recomputed` in its constraints and asserts in its
    self-review that the counts were recomputed. One more instance in the
    controller's own `plan-brief-addenda.md` ("Four addenda" over five sections).
    The pattern across all five: the count is correct where it is computed and stale
    where it is restated. A handle that follows: **a restated count carries the
    expression that produces it, or it is replaced by a reference to the site that
    computes it.**
11. **An enumeration presented as a command's output that the command cannot
    produce** (F7). Distinct from a wrong count: the instrument ran, the conclusion
    was right, and the missing member was filled in from knowledge without the seam
    showing. `a-search-whose-terms-come-from-memory-produces-a-false-absence`
    covers the terms of a pattern; this is the complementary case, where the
    pattern's **shape** (same-line coupling) rather than its terms causes the miss,
    and the fired control cannot see it because a control proves discrimination, not
    completeness.

**One process observation.** The plan's `proc-08-parallel-worktrees` ruling, its
pathspec-scoped commits and its serial-dispatch constraint are all sound and
matched by the file-overlap measurement (three tasks write `EditorView.vue`, three
write the editor catalogs). The plan-close entry condition, the completeness-claim
boundary and the blocked-pool sweep are all present and correctly scoped. Nothing
in the sequencing or the close needs a fix.
