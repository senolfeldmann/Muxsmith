# Task 2 verdict (Plan 12): the three-state language control

**Reviewer:** independent, did not author the change. Read-only on the repo
except this file. Range reviewed: `0c001ee..ea39d88`, one commit, six files.
Every figure below is from a run I made in the main working tree at `ea39d88`;
every quotation is copied from the artifact it names.

**Spec compliance: PASS.** All thirteen requirements (T2-a to T2-m) are met,
each verified independently rather than read off the report.

**Task quality: PASS, strong.** The fenced code landed byte-exact, every
comment claim I checked is true against the code it describes, the two
deviations from the brief's literal text are the two the controller ruled and
nothing else deviates, and the four cases carry anti-vacuity controls the brief
did not ask for. The one substantive finding below is a mischaracterisation in
the record, not a defect in the shipped artifact.

**Findings:** 0 Critical, 1 Important, 3 Minor.

---

## 1. Spec compliance, per requirement

| # | Verdict | How I verified it |
|---|---|---|
| T2-a | met | The system option is the `<select>`'s first child with `:value="SYSTEM_LOCALE"`; case *first run* asserts both `toHaveValue("")` and the checked option's German label. Non-vacuity measured, see mutation D in section 4. |
| T2-b | met | `const SYSTEM_LOCALE = "";` at module level in `SettingsDialog.vue`; `open()` carries `form.locale = baseline.locale ?? SYSTEM_LOCALE;` and `save()`'s `next` carries `locale: form.locale === SYSTEM_LOCALE ? null : form.locale,`. Both are the prescribed expressions character for character. |
| T2-c | met | `effectiveLocale` is exported from `src/i18n/index.ts` and is the only implementation. Grep of every `.locale` read in `src/` returns four sites: `resolveLocale` (through the seam), `open()`'s display mapping, `save()`'s sentinel mapping, and the live-switch comparison. No second system-locale source exists anywhere in `src/`, `e2e/` or `scripts/` (`navigator.languages`, `Intl...resolvedOptions`, `sys_locale`, `systemLocale`: no hits). The Rust side defaults `locale: None` in `src-tauri/src/settings.rs`, so it is not a third implementation either. |
| T2-d | met | `settings-locale-option-system` is `System language` / `Systemsprache`. No language name, no `Intl.DisplayNames`. |
| T2-e | met | The guard is `if (next.locale !== baseline.locale)` on the raw nullable values. Both directions demonstrated live by the two split cases; the "unrelated save does not fire" half is demonstrated by mutation C, under which the untouched-language save begins writing `""` and its case goes red. |
| T2-f | met | Measured, not eyeballed: I extracted the four fenced blocks from `task-2-brief.md`'s own Step 4 programmatically and tested containment against the two catalogs. Both replacement blocks are byte-exact present; both `old` blocks are absent; `settings-locale-option-en`/`-de` survive in both files. German orthography intact (`Oberfläche`, `fällt`, `zurück`, `Übersetzung`). |
| T2-g | met | Both assertions are in `smoke.spec.ts`'s `german locale` describe, case *selecting German in the settings dialog saves it, and it renders the German catalog on the next start*: `toHaveValue("en")` on `localeSelect` and `toHaveValue("de")` on `reloadedLocaleSelect`. The report's measurement reproduces: `mocks.ts`'s `get_settings` fallback returns the concrete `locale: "en"`, and `DE_SETTINGS` in `smoke.spec.ts` carries the concrete `locale: "de"`, so `?? SYSTEM_LOCALE` never yields for either. Both green in my own full run. |
| T2-h | met | `git diff --stat 0c001ee..ea39d88 -- playwright.config.ts e2e/mocks.ts e2e/smoke.spec.ts` is empty; the commit's file list is exactly the six of the Files list. |
| T2-i | met, and measured rather than assumed | Nothing was added for an out-of-band value. I probed the case that the new option set could plausibly have changed (a stored `"de-AT"`, for which `smoke.spec.ts` has a fixture but no settings-dialog assertion): the control renders `selectedIndex: -1` with no checked option, and a save that does not touch the language persists `"de-AT"` unchanged. So D106 decision 8's description - "renders as an unmatched select" - still holds exactly, and the sentinel option does not capture the unmatched case. |
| T2-j | met | Reproduced end to end. RED on the pre-state at `0c001ee`: exactly 2, both in `src/main.ts` (`resolveLocale`'s try and catch). GREEN on the committed tree: 0. Soundness control unfiltered on the committed tree: exactly 2, both in `src/i18n/index.ts` - `effectiveLocale`'s `return` and `primarySubtag`'s doc comment, the two enumerated members. |
| T2-k | met | `%G?` is `N`; the body is one line and `grep -c "Co-Authored-By"` is 1; six files. |
| T2-l | met | See adjudication Q2. |
| T2-m | met | See adjudication Q1. |

Two further constraints the plan binds on every task, checked and clean:
**typography** (the only non-ASCII characters in the whole added diff are
`ä ü Ü`, which the constraint explicitly permits as orthography; no dash,
quote, ellipsis or NBSP tell), and **no line-number citations** in any added
comment. Both greps were fired against known-present controls before their
empty results were trusted.

---

## 2. Findings

### Important

**I1. The recorded reason the single-case form was replaced is wrong, in the
direction that matters.** Both `task-2-ruling-1.md` ("as written, the case
would have shipped green while measuring nothing on the live path") and the
report's section 3 ("That is the assertion the brief's single-case form could
not make fail") describe a case that PASSES vacuously. It does not. I rebuilt
the brief's prescribed case 3 - one page, two legs, same scenario, ruling-1
locators so that only the split fork is under measurement - and ran it against
the correct, committed implementation:

```
PROBE-LEG1 {"persisted":"en","enHeading":1,"deHeading":0,"lang":"en"}
PROBE-LEG2-CONTROL-ON-REOPEN ""
PROBE-LEG2 {"persisted":["en",null],"enHeading":1,"deHeading":0,"lang":"en"}
PROBE-LEG2-BRIEF-ASSERTIONS {"recorded set_settings.locale === null":true,"the de heading back":false,"documentElement.lang === \"de\"":false}
```

Leg 2's persisted assertion passes; its other two prescribed assertions are
**false against a correct implementation**. The prescribed case is therefore
not a false-green, it is an always-red: unexecutable, carrying zero
discriminating power because it cannot pass whatever the code does. The
mechanism both documents describe is right (`open()` reassigns `baseline =
await getSettings();`, `mocks.ts`'s `nextResult` repeats the last queue entry,
so the second save compares `null` against `null` and the guard stays false) -
only the consequence drawn from it is inverted.

Why this is graded Important although the decision it supports is correct: a
false-green in a fallback-bearing mechanism is precisely the class this plan
guards three times in its own Global Constraints, and an invented instance of
that class entering the durable record (ruling, report, and any harvest drawn
from them) is worse than no instance. The corrected sentence is available and
is a stronger argument for the same ruling: *the prescribed case cannot pass on
a correct implementation, so it had to be replaced rather than merely
strengthened.*

Ownership: the claim originates in the controller ruling; the report's fault is
the narrower one of carrying a load-bearing borrowed claim into its own
evidence section without measuring it. No change is owed to the shipped
artifact.

### Minor

**M1. Mutation A's discrimination is overstated.** The report concludes "Each
mutation kills exactly the cases whose claims depend on it". For mutation A
that is true of two cases and not of the other two. In my re-run, *first run*
dies at its `de("batch-view-heading")` visibility assertion and *returning to
the system language* dies at its `de("batch-view-heading")` assertion - both
genuine, both defeating the `[requested, en]` fallback. But *saving without
touching the language* and *leaving the system language* die at
`getByRole("button", { name: de("settings-save") }).click()`, a 30-second
locator timeout, not at any assertion. *Saving without touching the language*
has no claim that depends on `effectiveLocale` at all: it asserts a persisted
payload. Its death under A is collateral of running the interface in German,
which is a property of the describe rather than of the case. The finding is
about the report's summary sentence, not about the tests.

**M2. The seam's doc comment cannot name the API it wraps.** Step 6's soundness
control fences the end-state occurrence count in `src/i18n/index.ts` at exactly
2 and enumerates its members, which silently forbids the new doc comment from
containing the token `navigator.language`. The comment therefore says "the
language the browser reports for the platform". The implementer complied
deliberately and surfaced it; the ruling carries it as a harvest item. Recorded
here so it is fixed rather than re-encountered if this brief shape is reused:
an absence check whose control counts occurrences in the file the task is
editing constrains that task's prose, and the constraint has to be stated where
the prose is prescribed.

**M3. The header doc's isolation claim is true but unwitnessed.** The new header
paragraph states that `test.use({ locale: "de-DE" })` "leaves every sibling case
(this file's own D56 case included) on the config's `en-US`". That is documented
Playwright scoping and `playwright.config.ts` is untouched, so I have no reason
to doubt it - but no case in this suite could detect a leak, because every other
scenario supplies a concrete stored locale and therefore never reads
`navigator.language`. Named as a residual; no action owed, and building a test
for it would be new coverage of Playwright rather than of this product.

---

## 3. Adjudication questions

**Q1. The split's coverage: the pair preserves everything the prescribed case
asserted, and adds what the single case could not reach.** Walked assertion by
assertion.

| Prescribed (brief Step 5, case 3) | Where it lives now |
|---|---|
| leg 1: recorded `set_settings.locale === "en"` | *leaving the system language*, `saved(recorded)[0].locale` |
| leg 1: the de heading is gone | same case, `de("batch-view-heading")` `toHaveCount(0)` |
| leg 1: the en heading is present | same case, `en("batch-view-heading")` `toBeVisible` |
| leg 1: `documentElement.lang === "en"` | same case |
| leg 2: recorded `set_settings.locale === null` | *returning to the system language*, `saved(recorded)[0].locale` |
| leg 2: the de heading back | same case, `de("batch-view-heading")` `toBeVisible` |
| leg 2: `documentElement.lang === "de"` | same case |

Nothing prescribed is unasserted. The pair additionally carries: a per-case
`toHaveLength(1)` on the recorded writes (the single form could only count
cumulatively), an explicit `toHaveValue("en")` pre-read of the control under a
stored override - a state the prescribed case never reached, since its mock
returned `null` on every read - and a four-part red-state control opening
*returning to the system language*, so "the German heading is back" cannot be
green on a page that was German throughout. "No reload anywhere" holds: neither
case contains `page.reload`, and `page.goto` appears once per case. Both are on
the live path, which section 4's mutation B confirms by killing exactly them
plus the pre-existing D56 case.

The ruling's consequence for the acceptance map is correct and I applied it: W1-f
and W1-g's producer is the second case of the split, not "same test, second
leg". Correction to the ruling's *ground* is finding I1; it does not disturb
this verdict.

**Q2. The locator ruling's boundary is respected.** I derived the sets from the
file rather than from the report's list. Inside the new describe, `de(...)`
occurs seven times: five inside `expect(...)` and two as
`getByRole("button", { name: de("settings-save"), exact: true }).click()`.

- **Asserted German ids: exactly two.** `batch-view-heading` (en `Batch`, de
  `Stapel`) and `settings-locale-option-system` (en `System language`, de
  `Systemsprache`). Both pairs differ. Stronger than the rule requires, and
  checked because a differing value can still collide elsewhere: neither
  `Stapel` nor `Systemsprache` occurs anywhere in `locales/en/` (grep fired
  against `Batch`, which does occur, to prove the instrument). So neither
  assertion can be satisfied by an interface that fell back to English.
- **No German-valued locator does assertion work.** `de("settings-save")`
  (`Speichern` vs `Save`) appears only in the two `.click()` calls, never inside
  an `expect`. The third Save click, in *returning to the system language*, uses
  `en("settings-save")` because the interface is English at that point - the
  ruling's rule applied in the other direction.
- **No locator is counted as a witness.** The two comments beside the German
  Save locators say only which catalog applies at that point in the case; the
  "second witness that the German bundle is live" framing from the implementer's
  original memo appears in neither the code nor the report's evidence, exactly
  as the report claims.

**Q3. The `pnpm lint` warning observation affects no coverage claim this task
makes. Verdict: no.** Recorded rather than left silent, and fired in both
directions rather than read off the config:

- The one coverage claim routed through `pnpm lint` is W1-j's D27 half, the
  no-raw-text check on the new option. `eslint.config.js` sets
  `@intlify/vue-i18n/no-raw-text` to `"error"`, and I fired it: replacing the
  option's `{{ $t(...) }}` with the literal `System language` produces
  `error  raw text 'System language' is used  @intlify/vue-i18n/no-raw-text` and
  `pnpm lint` exits **1**.
- The implementer's observation reproduces exactly: restoring the brief's
  one-line `<option>` rendering produces
  `✖ 2 problems (0 errors, 2 warnings)` from
  `vue/singleline-html-element-content-newline`, and `pnpm lint` exits **0**.

So the gate's blindness to warnings is real, it belongs to preset-level rules,
and no claim this task makes depends on a warning failing. It is a house
question for the controller, not a finding against this task. Both mutations
were restored and the tree re-verified clean.

While there, I fired the other gate this task's coverage leans on, because the
same "an absent result proves nothing" logic applies: deleting
`settings-locale-option-system` from `locales/de/gui-settings.ftl` makes
`pnpm check:i18n` report
`missing id "settings-locale-option-system" (present in locales/en/gui-settings.ftl)`
and exit 1. W1-j's parity half is real coverage.

---

## 4. The mutation evidence, re-run

I did not execute anything the implementer left behind. My instrument is a
standalone script under my own session scratch path, which applies exactly one
mutation by exact-string replacement and aborts if the anchor is not found
exactly once (a silently-missed mutation being the failure mode that makes this
whole exercise worthless). Sources were snapshotted before the first run and
restored from the snapshot before each subsequent one; `git status --porcelain`
and `git diff` are both empty at the time of writing, and the full suite is
green on the restored tree.

My baseline on the committed tree: **72 passed**.

| Mutation | Report's claim | My run | Verdict |
|---|---|---|---|
| **A.** `effectiveLocale` returns `saved ?? "en"` | all four new cases red, D56 green | **4 failed, 68 passed** - the four new cases red, D56 green, no collateral | Reproduces. Red state genuinely defeats the fallback for two of the four; see below. |
| **B.** the live switch no longer calls `applyLocale` | *leaving*, *returning*, D56 red; *first run*, *saving without touching* green | **3 failed, 69 passed** - exactly those three red | Reproduces exactly. Discriminates precisely. |
| **C.** `save()` drops the sentinel mapping | *saving without touching*, *returning* red; the other three green | **2 failed, 70 passed** - exactly those two red | Reproduces exactly. Discriminates precisely. |

**Per mutation, the two questions the brief asks.**

- **A - does the red state defeat what it claims to defeat?** Partly, and the
  distinction matters. Under A the app resolves `"en"` where it should resolve
  the system language, so the interface renders English. *first run* dies at
  `expect(...de("batch-view-heading")...).toBeVisible()` and *returning to the
  system language* dies at the same assertion after its save. Those are real
  kills below the fallback: the assertion is on a German value the en catalog
  cannot produce, so the `[requested, en]` chain cannot rescue it. The other two
  cases die at a `de("settings-save")` click timeout, i.e. at an interaction
  locator, which the ruling explicitly forbids counting as a witness. **Spared
  correctly:** the D56 case, whose scenario carries a concrete stored `"en"`, so
  the seam's `??` never yields. Net: A proves the seam is load-bearing for two
  cases and proves nothing about the other two - which is finding M1.
- **B - does the red state defeat what it claims to defeat?** Yes, cleanly.
  The mutation removes the `applyLocale` call while leaving the guard, the
  persisted payload and the German rendering at page load untouched, so the
  three dying cases die at post-save rendering assertions
  (`expect(locator).toBeVisible()` in all three), not at a locator and not at a
  payload. **Spared correctly:** *first run* performs no save at all, and
  *saving without touching the language* asserts only the recorded payload,
  which the mutation does not touch. This is the mutation that carries ruling 2:
  the direction the prescribed single case could not reach is the one that goes
  red here.
- **C - does the red state defeat what it claims to defeat?** Yes. Writing
  `locale: form.locale` makes the sentinel leave the dialog as `""` instead of
  `null`, and both dying cases assert `toBeNull()` on the recorded payload -
  a concrete value on the far side of nothing, no chain in between. **Spared
  correctly:** *leaving the system language* sets `form.locale` to `"en"`, for
  which the mutation is a no-op; *first run* saves nothing; D56 saves `"de"`.

**A fourth mutation, mine, because the brief asks where a check does not look.**
T2-a's coverage rests on two assertions in *first run*, and `toHaveValue("")` is
the kind that can pass on an absent control - a `<select>` whose bound value
matches no option also reads `""`. So I removed the third `<option>` element
entirely and ran the file:

```
✓ leaving the system language: picking English stores the override and switches live
✓ saving without touching the language writes no override
✓ live locale switch (D56) ...
✘ first run: the interface follows the system language and the control says so
✘ returning to the system language: removing the override stores null and switches live
```

*first run* fails at
`expect(localeSelect.locator("option:checked")).toHaveText(...)` with "element(s)
not found" - and `toHaveValue("")` immediately above it **passed** under the same
mutation. The two assertions are therefore not redundant: the second is what
actually carries "the option exists and is preselected". That is a point in the
implementation's favour and is recorded as such.

---

## 5. Quality, house, latitude

**The seam's contract.** `effectiveLocale(saved: string | null): string` is
right for both callers: `resolveLocale` needs a string for `applyLocale`, and
the dialog's live switch needs the same. Returning `string` is what let the
`next.locale !== null` narrowing and its comment disappear, which is the D106
decision-4 outcome. Placement directly above `primarySubtag` puts the file in
resolution order (`effectiveLocale` -> `primarySubtag` -> `buildBundles`).

**Are the comments true?** Checked individually rather than skimmed.
`SYSTEM_LOCALE`'s comment claims the sibling field already uses the pattern:
`open()` does carry `form.mkvmergePath = baseline.mkvmerge_path ?? "";` and
`save()` does carry `mkvmerge_path: form.mkvmergePath.trim() === "" ? null :
...`, so the claim holds. The live-switch comment's parenthetical
`(null -> "en", "de" -> null)` describes exactly what the guard does on raw
values. `main.ts`'s reworded catch rationale - "an unreadable settings file
carries no override either, so the catch resolves exactly as an unset setting
does" - is true of `effectiveLocale(null)`. No comment names a line number.

**`house`.** `gui-closed-domain-dropdowns` (Tier 2, `product-boundaries.yaml`):
a closed value set renders as a select, satisfied - the third option extends the
existing select rather than introducing any other control.
`comments-locate-by-symbol-never-by-line-number`: satisfied throughout the added
comments. The one structural divergence from the brief's literal text - the
three-line `<option>` rendering instead of the brief's inline one - is house
conformance with the file's two existing options, it is reported, and Step 3
does not fence that element the way Step 4 fences the catalog values and the
live switch. Not a finding.

**Latitude, both forms.** No design-latitude clause survives into the shipped
work: the two open forks were returned as NEEDS_CONTEXT with a decision memo and
ruled, which is the behaviour the ban asks for. The four things the implementer
composed inside the describe - the `installStored` helper, the `saved()`
accessor, the `default_jobs` control and the fourth case's red-state control -
are fixtures and controls, which the plan's own model-tier table assigns to this
task ("a new test whose locale mechanism is prescribed but whose fixtures are
composed"), and none of them changes an assertion the plan prescribed. The one
omission-form latitude I found in the brief is M2: an unstated prose constraint
derivable only from Step 6's control enumeration.

**The no-work-needed check.** Three passages conclude something is unnecessary,
and I ran the claim rather than weighing it in each case: the `pnpm lint`
warning claim (fired both directions, Q3), the "no handling for an out-of-band
locale" claim (probed with a stored `"de-AT"`, T2-i), and the "both existing
`smoke.spec.ts` assertions stay valid" claim (fixture values read from the two
files named, plus a green run).

---

## 6. Harvest, surfaced not written

I write nothing to the ledger. Four candidates, in descending confidence:

1. **A negative claim about a test's failure mode is a measurement, and it
   inverts easily.** The instance is finding I1: "would have shipped green" and
   "always red" look the same from the memo that describes the mechanism, and
   only running the prescribed form separates them. The trigger is readable -
   you are about to write that a test *would have passed* while measuring
   nothing, about a form you did not execute.
2. **An absence check whose control counts occurrences in the edited file
   constrains that file's prose** (M2). Worth stating where the prose is
   prescribed rather than leaving it derivable from the control's enumeration.
3. **The resolution-seam pattern** the report surfaces: a rule read by more than
   one module lives in the module that owns the domain and is never restated per
   caller. The D106 defect is what the restatement produces, and it survived two
   plans unnoticed. Whether this is already covered by an existing entry is the
   controller's call.
4. **`e2e/mocks.ts` has no stateful settings store**, already carried by ruling 1
   as a close-action tracker item with an observable trigger. Repeated here only
   so this verdict is self-contained.

---

## 7. What I could not verify, and why

- **Gate parts 1 to 6 and 11** (the Rust block and `scripts/ledger-lint.py`) were
  not re-run by me. The commit touches no Rust file and no ledger YAML, so this
  change cannot have moved them; I ran the four frontend parts myself
  (`pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`) and all four are
  green on the committed tree. The report's Rust figures are therefore
  unre-measured rather than doubted.
- **Whether `Claude Opus 5` is the model the dispatch actually assigned.** Not
  visible from the repository. It matches every other commit in this plan
  (`b381675`, `0c001ee`, `f083bc2`, `bd3aa34`) and carries no context-window
  suffix, so it conforms to `agent-commit-trailer-set`'s shape rules; the
  derived-from-the-dispatch half is outside a reviewer's reach.
- **The temporary Node-side call-sequence probe** quoted in the report's section
  5 is not reproducible from the tree, as the report itself states. I verified
  its two underlying mechanism claims at the source instead (`open()`'s
  `baseline = await getSettings();`, and `mocks.ts`'s `nextResult` returning
  `q[0]` once a queue is down to one entry), and my own reconstruction of the
  prescribed case reproduces the consequence - with the correction in I1.
