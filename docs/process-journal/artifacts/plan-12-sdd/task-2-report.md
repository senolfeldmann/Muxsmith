# Task 2 report: the settings language control becomes three-state (W1)

**Status: DONE.** All seven steps complete, full gate green, committed as
`ea39d88`, not pushed.

This task returned one NEEDS_CONTEXT mid-flight with two forks in Step 5.
Both were routed and settled by the controller in
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-12/task-2-ruling-1.md`, which
sits below the plan and above the brief for exactly those two questions and is
silent on everything else. **That file is the authority for the two places
where the shipped work departs from the brief's literal text** - the
interaction locators in Step 5, and case 3 being two cases instead of one.
Neither is an unrouted decision; both are tabulated with their authority in
section 5.

> **FIX ROUND 1, 2026-07-30, read before section 3.** The review
> (`task-2-verdict.md`, findings I1 and M1) found two defects in this report's
> own REASONING. Nothing in the shipped work changes: `ea39d88` stands,
> and no file in `src/`, `locales/` or `e2e/` was touched by this round. Three
> passages in section 3 carry the wrong claims; each is left standing with a
> marked correction beside it, so a later reader can see both what was asserted
> and what was measured. **Section 9 is the re-measurement**, with every command
> and its pasted output; its 9.7 names two further sites of the same wrong claim
> that lie outside this round's scope, one of them in committed source.

---

## 1. What was done, per step

### Step 1: the seam (`src/i18n/index.ts`) - DONE

`effectiveLocale(saved: string | null): string` added, exported, placed
directly above `primarySubtag` (the resolution chain reads `effectiveLocale`
-> `primarySubtag` -> `buildBundles`, so it now reads in that order in the
file). Body is the fenced one, character for character:

```ts
export function effectiveLocale(saved: string | null): string {
  return saved ?? navigator.language;
}
```

The doc comment states the rule and names both callers by symbol
(`resolveLocale` in `src/main.ts`, `save` in
`src/components/SettingsDialog.vue`), per
`comments-locate-by-symbol-never-by-line-number`.

**One constraint the brief imposes implicitly and I honoured deliberately:**
the doc comment does NOT contain the token `navigator.language`, because Step
6's soundness control fences the end-state occurrence count in this file at
exactly 2 and enumerates its two members (the `primarySubtag` doc comment plus
this function's `return`). A third mention in the new doc comment would have
broken that fence. The comment therefore says "the language the browser
reports for the platform" in prose. Ruling 1 records this as a harvest item.

### Step 2: `src/main.ts` - DONE

`resolveLocale` is now `effectiveLocale((await getSettings()).locale)` in the
try and `effectiveLocale(null)` in the catch; `import { effectiveLocale } from
"./i18n";` added (placed before the `./i18n/fluent` import, keeping the local
imports in path order as the file already had them). `eslint.config.js`
carries no import-ordering plugin at all - it composes
`tseslint.configs.recommended`, `pluginVue.configs["flat/recommended"]` and
the single `@intlify/vue-i18n/no-raw-text` rule, read from the file, not
recalled - so the ordering is convention, not enforcement.

The doc comment keeps both facts it carried:

- why the locale resolves before mount (the spec 8.4 quote and the
  no-English-flash rationale, unchanged);
- why a `get_settings` failure is not a startup blocker - reworded so it stays
  true under the seam: an unreadable settings file carries no override either,
  so the catch resolves exactly as an unset setting does, and `buildBundles`
  still falls back to `"en"` for any locale it cannot resolve.

The inline `navigator.language` description is replaced by a sentence naming
`effectiveLocale` as the owner of the rule and naming the dialog's live switch
as its second caller.

### Step 3: `src/components/SettingsDialog.vue` - DONE, all four edits

1. Module-level `const SYSTEM_LOCALE = "";` with a comment stating why the
   empty string is the sentinel (a `<select>` cannot bind `null`) and naming
   the sibling field that already uses it (`mkvmergePath` loads as
   `baseline.mkvmerge_path ?? ""` and saves `""` back as `null`), plus the
   second ground from D106 decision 1 (no BCP-47 tag can be empty).
2. `form`'s initial `locale` is `SYSTEM_LOCALE`; `open()`'s init is
   `form.locale = baseline.locale ?? SYSTEM_LOCALE;`.
3. `save()`'s `next` carries
   `locale: form.locale === SYSTEM_LOCALE ? null : form.locale,` and the live
   switch is exactly the fenced two lines. The `!== null` narrowing
   explanation is gone (the guard itself too - `effectiveLocale` returns
   `string`, so vue-tsc no longer needs it); the replacement comment names the
   seam and states that the comparison stays on the raw nullable values so
   both directions across the sentinel fire. The D56 live-switch rationale and
   the v-show-keeps-state sentence are kept verbatim.
4. The `<select>` gains the system option as its FIRST child; `en` and `de`
   keep their explicit values and their order.

**One structural conformance note (no round trip taken, under the dispatch's
existing-pattern grant):** the brief renders the new option inline on one
line. The file's two existing `<option>`s are three-line, so the new one
follows the file's shape. Same element, same attributes, same interpolation;
zero outward effect.

Measured rather than assumed - the brief's one-line rendering was put in place
and linted before being restored:

```
/home/senol/Git/Muxsmith/src/components/SettingsDialog.vue
  159:42  warning  Expected 1 line break after opening tag (`<option>`), but no line breaks found    vue/singleline-html-element-content-newline
  159:83  warning  Expected 1 line break before closing tag (`</option>`), but no line breaks found  vue/singleline-html-element-content-newline

✖ 2 problems (0 errors, 2 warnings)
```

These are WARNINGS: `pnpm lint` is plain `eslint .` with no `--max-warnings`,
so the one-line form would still have exited 0 and passed the gate. The
three-line form is house conformance, not a gate rescue.

### Step 4: the catalogs - DONE

Both fenced replacements applied byte-for-byte, including the German
orthography (`Oberfläche`, `fällt`, `zurück`). `settings-locale-option-en` and
`-de` are unchanged and in place in both files.

### Step 5: the tests - DONE, four cases, under ruling 1

`e2e/locale-switch.spec.ts` carries
`test.describe("system-locale default (D106)")` with
`test.use({ locale: "de-DE" })` as its first statement. The file header gained
the prescribed paragraph naming the new subject and stating why the
describe-level override is safe (config untouched, siblings stay on `en-US`,
plan-5 D29's pinning undisturbed), plus a second paragraph recording ruling
1's locator rule and its two qualifiers, so a future reader does not read
`de("settings-save")` as a violation of the file's own en()-only line.

| Case | Scenario | Asserts |
| --- | --- | --- |
| first run | `locale: null` | de batch heading, `lang === "de"`, control holds `""`, selected option's text is the de `settings-locale-option-system` |
| saving without touching the language | `locale: null` | exactly one `set_settings`, its `locale` is `null` (+ a control that the save carried the jobs edit) |
| leaving the system language | `locale: null` | `set_settings.locale === "en"`, de heading gone, en heading present, `lang === "en"` |
| returning to the system language | `locale: "en"` | `set_settings.locale === null`, de heading back, `lang === "de"` |

The last two are the split of the brief's case 3, per ruling 2. Every
assertion the brief prescribed for that case survives; what changed is which
case reaches it. No reload anywhere; `e2e/mocks.ts`, `playwright.config.ts`
and `e2e/smoke.spec.ts` untouched.

**Both qualifiers in ruling 1 are respected, and I checked them against the
shipped code rather than assuming my memo matched:**

- **The assertion rule is untouched.** Every ASSERTED German string in the
  four cases is `batch-view-heading` (`Batch` / `Stapel`) or
  `settings-locale-option-system` (`System language` / `Systemsprache`), both
  differing across catalogs. `de("settings-save")` appears only as an
  interaction locator, never inside an assertion.
- **No locator is counted as a witness.** The comments beside the two
  locale-dependent Save locators say only which catalog applies at that point.
  The "second witness that the German bundle is live" framing from my memo is
  in neither the code nor this report's evidence.

Four things inside the describe are mine rather than the brief's, all
strengthening and none user-visible:

- `installStored(page, settings)`, one helper taking the stored-settings
  state, since the four cases differ only in that argument (reuse before
  writing, rather than near-duplicate helpers);
- a local `saved(recorded)` accessor for the `set_settings` payloads;
- in the second case, `expect(writes[0].default_jobs).toBe(3)`, a soundness
  control: without it that case's real assertion (`locale` is `null`) would
  read green even if the jobs edit had never landed and nothing had been saved
  at all;
- in the fourth case, a red-state control before the interaction (en heading
  visible, de heading count 0, `lang === "en"`, control holds `"en"`), so "the
  German heading is back" cannot be green on a page that was German all along.

### Step 6: verification - DONE (sections 2 to 4, and 7)

### Step 7: commit - DONE (section 6)

---

## 2. Absence check L1, the single resolution rule

Expression: `grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"`

### RED, run FIRST on the pre-state, before any edit

```
$ grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"
src/main.ts:17:    return (await getSettings()).locale ?? navigator.language;
src/main.ts:19:    return navigator.language;

$ grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts" | wc -l
2
```

Exactly 2 lines, both in `src/main.ts`, one in `resolveLocale`'s try branch
and one in its catch branch. This reproduces the plan's fenced pre-state
figure at the current head; no fence was adjusted anywhere.

Pre-state unfiltered, for completeness - this one pre-existing occurrence is
what makes the control's end-state figure 2 rather than 1:

```
$ grep -rn "navigator.language" src/
src/main.ts:17:    return (await getSettings()).locale ?? navigator.language;
src/main.ts:19:    return navigator.language;
src/i18n/index.ts:33: * lowercased. A saved setting or `navigator.language` is often
```

### GREEN on the FINAL end state

Re-run after Step 5 was rewritten under the ruling and after every mutation in
section 3 was reverted, so this is the committed tree, not a mid-task state:

```
$ grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"

$ grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts" | wc -l
0
```

### Soundness control, on the FINAL end state

The same expression WITHOUT the `-v` filter, because an empty grep and a
broken grep look identical:

```
$ grep -rn "navigator.language" src/
src/i18n/index.ts:46:  return saved ?? navigator.language;
src/i18n/index.ts:51: * lowercased. A saved setting or `navigator.language` is often

$ grep -rn "navigator.language" src/ | wc -l
2
```

Exactly 2, both in `src/i18n/index.ts`, and they are the two enumerated
members: Step 1's `return saved ?? navigator.language;` and the pre-existing
occurrence in `primarySubtag`'s doc comment. The pattern still matches where
the token survives, so the zero above is an absence and not a broken
instrument.

---

## 3. The new cases are falsifiable: three mutations, fired

The four new cases pass. Passing is not evidence by itself, and the defect
ruling 2 repaired was precisely a case that would have shipped green while
measuring nothing on the live path. So each mechanism the cases claim to
measure was broken deliberately, the suite re-run, and the tree restored.
`pnpm build` was re-run before each run, since `vite preview` serves `dist/`
and `pnpm test:e2e` does not rebuild it.

> **CORRECTION 1, 2026-07-30, fix round 1 (finding I1).** The sentence above is
> wrong in its consequence and is left standing rather than edited into
> agreement, because the wrong version is what a later reader would otherwise
> reconstruct. The brief's prescribed single-case form is **always RED against a
> correct implementation, not a false-green.** I rebuilt it - one case, two
> legs, one page, the same `locale: null` scenario, ruling-1 locators so that
> only the split fork is under measurement - and ran it against the committed
> tree at `ea39d88`. Leg 1's four prescribed assertions all pass; leg 2's
> persisted assertion (`set_settings.locale === null`) passes; leg 2's other two
> prescribed assertions, "the de heading back" and
> `documentElement.lang === "de"`, are **false against correct code**. The run
> and its output are in section 9.
>
> The mechanism this report describes in section 5 is right - `open()` re-reads
> the baseline, the mock's exhausted queue repeats `locale: null`, the guard
> sees `null` on both sides and `applyLocale` never fires. Only the consequence
> drawn from it was inverted. **So this was never an instance of the
> assertion-below-a-fallback class**, which is the class this plan's Global
> Constraints guard hardest, and which the sentence above invokes to justify the
> mutation work that follows. The real shape is narrower and still justifies the
> split: a fixture that cannot reach the state the assertion needs, so the case
> blocks on the fixture rather than on the product - an always-red case that
> cannot be made green without either a reload or new test infrastructure, and
> therefore one that must be re-cut rather than retried. The mutation work below
> keeps its value regardless; what it does NOT rest on is a false-green that
> never existed. The controller carries the same correction beside the same
> wrong sentence in `task-2-ruling-1.md`, where the claim originated.

| Mutation | Cases that went RED | Cases that stayed green |
| --- | --- | --- |
| **A.** `effectiveLocale` returns `saved ?? "en"` (the seam stops following the system language) | all four new cases | the D56 live-switch case |
| **B.** `save()`'s live switch no longer calls `applyLocale` | leaving the system language; returning to the system language; the D56 case | first run; saving without touching the language |
| **C.** `save()` drops the sentinel mapping (`locale: form.locale`) | saving without touching the language; returning to the system language | first run; leaving the system language; the D56 case |

Mutation A:

```
  ✓  1 › live locale switch (D56) › selecting German in settings swaps the catalog in place: no reload, view state survives, html lang follows (197ms)
  ✘  5 › system-locale default (D106) › first run: the interface follows the system language and the control says so (5.1s)
  ✘  2 › system-locale default (D106) › returning to the system language: removing the override stores null and switches live (5.2s)
  ✘  3 › system-locale default (D106) › leaving the system language: picking English stores the override and switches live (30.1s)
  ✘  4 › system-locale default (D106) › saving without touching the language writes no override (30.1s)
```

> **CORRECTION 2, 2026-07-30, fix round 1 (finding M1).** The row for mutation A
> in the table above, and the "kills exactly" sentence at the end of this
> section, both read as four equivalent kills. They are not equivalent, and the
> row is left standing with this correction beside it. My own full-suite run
> under A gives **4 failed, 68 passed** - the four new cases red, the D56 case
> green, nothing else touched - and it separates the four deaths into two kinds
> (the line numbers are `e2e/locale-switch.spec.ts` at `ea39d88`, printed by the
> failure output itself):
>
> | Case | Where it dies under A | What kind of death |
> | --- | --- | --- |
> | first run | `expect(...de("batch-view-heading")...).toBeVisible()`, spec line 201 | a German rendering assertion below the `[requested, en]` fallback: a real kill |
> | returning to the system language | the same assertion after its save, spec line 303 | the same |
> | saving without touching the language | `getByRole("button", { name: de("settings-save") }).click()`, spec line 227, 30-second timeout | an interaction LOCATOR, which ruling 1 explicitly forbids counting as a witness |
> | leaving the system language | the same click, spec line 258, 30-second timeout | the same |
>
> So A discriminates the seam for two of the four cases. For the other two it
> proves nothing about `effectiveLocale`, and this is measured rather than
> argued: I re-ran both cases with the ONE change of an `en()` Save locator
> instead of `de()`, still under A, and **every assertion in both cases passed**
> (section 9). Their deaths under A are collateral of running the interface in
> German, which is a property of the describe rather than of the case. This goes
> one step further than the verdict, which named only *saving without touching
> the language* as having no seam-dependent claim; measured, *leaving the system
> language* has none either, because its post-save assertions (English heading,
> `lang === "en"`, persisted `"en"`) all hold on an app that was English from the
> start.
>
> What this does NOT change: the two cases carrying the seam claim are killed
> genuinely, and the D56 case is spared correctly (its scenario stores a concrete
> `"en"`, so the seam's `??` never yields). A remains the right mutation for the
> seam; its reach is two cases, not four.

Mutation B:

```
  ✓  3 › system-locale default (D106) › first run: the interface follows the system language and the control says so (162ms)
  ✓  2 › system-locale default (D106) › saving without touching the language writes no override (200ms)
  ✘  5 › system-locale default (D106) › leaving the system language: picking English stores the override and switches live (5.2s)
  ✘  1 › live locale switch (D56) › selecting German in settings swaps the catalog in place: no reload, view state survives, html lang follows (5.2s)
  ✘  4 › system-locale default (D106) › returning to the system language: removing the override stores null and switches live (5.2s)
```

Mutation C:

```
  ✓  3 › system-locale default (D106) › first run: the interface follows the system language and the control says so (162ms)
  ✓  5 › system-locale default (D106) › leaving the system language: picking English stores the override and switches live (205ms)
  ✓  1 › live locale switch (D56) › selecting German in settings swaps the catalog in place: no reload, view state survives, html lang follows (217ms)
  ✘  4 › system-locale default (D106) › saving without touching the language writes no override (221ms)
  ✘  2 › system-locale default (D106) › returning to the system language: removing the override stores null and switches live (224ms)
```

Each mutation kills exactly the cases whose claims depend on it and spares the
rest, so no case is passing for an unrelated reason and none is tautological.
**Mutation B is the one that matters most for ruling 2:** under it, "returning
to the system language" goes red. That is the assertion the brief's
single-case form could not make fail, which is exactly why the split was
ruled. The split is not a workaround for a fixture limitation - it is what
restored the assertion's power.

> **CORRECTION 3, 2026-07-30, fix round 1 (findings M1 and I1).** Three claims in
> the paragraph above are wrong; all are left standing.
>
> - **"Each mutation kills exactly the cases whose claims depend on it and spares
>   the rest"** holds for mutations B and C, not for A. Two of A's four deaths are
>   locator timeouts, and the assertion sets of exactly those two cases pass under
>   A. Measured; see correction 2 beside mutation A's own output.
> - **"That is the assertion the brief's single-case form could not make fail"**
>   inverts the prescribed form's failure mode. It could not make that assertion
>   PASS. The prescribed case is always red against correct code (correction 1,
>   measured in section 9), so there was no green to lose. Mutation B's own
>   standing is undisturbed: it is still the mutation showing that the shipped
>   split reaches the live path in the direction the prescribed form could not
>   reach at all.
> - **"The split is not a workaround for a fixture limitation - it is what
>   restored the assertion's power"** is exactly backwards. The split IS a re-cut
>   around a fixture limitation, and that is its correct and sufficient
>   justification: the mock has no stateful settings store, the prescribed case
>   therefore cannot reach the state its own assertion needs, and no amount of
>   strengthening makes an unexecutable case executable. That is why the ruling
>   re-cut it rather than retrying it, and why `mocks.ts` gaining a stateful
>   settings store is carried as a close action (section 8, item 3) rather than
>   dismissed.
>
> The sentence the record should have carried: *the prescribed case cannot pass
> on a correct implementation, so it had to be replaced rather than merely
> strengthened.*

Restoration verified byte-for-byte against the pre-mutation copies before the
gate was run:

```
$ diff -q src/i18n/index.ts <backup> && diff -q src/components/SettingsDialog.vue <backup>
identical
```

---

## 4. The full gate, foreground, no subsets

`BUILDING.md`'s own canonical sentence, read from the file: "The pre-push gate
is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge." All 11 were run on the
final tree, in order, in the foreground.

| # | Command | Result |
| --- | --- | --- |
| 1 | `cargo fmt --all --check` | green (silent, exit 0) |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | green |
| 3 | `cargo test --workspace` | green, **507 passed, 0 failed** across 39 suites, exit 0 |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | green |
| 5 | `cargo deny check` | green |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | green, exit 0 |
| 7 | `pnpm lint` | green (silent, exit 0) |
| 8 | `pnpm build` | green |
| 9 | `pnpm check:i18n` | green |
| 10 | `pnpm test:e2e` | green, **72 passed** |
| 11 | `python3 scripts/ledger-lint.py` | green |

Pasted, the load-bearing ones:

```
$ cargo test --workspace   (aggregated over the "test result" lines)
SUITES: 39 | PASSED: 507 | FAILED: 0
cargo test exit code: 0

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok

$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 213 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).

$ pnpm test:e2e
  72 passed (3.0s)

$ python3 scripts/ledger-lint.py
ledger-lint: 568 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

`pnpm check:i18n` is the hard cross-locale parity gate for the new id
`settings-locale-option-system`; it is present in both catalogs with no
placeable, so the D55 rule-5 pattern-structure check is satisfied by
construction and the attribute-name sets stay equal. `pnpm lint` is the D27
no-raw-text check for the new option, and it is silent.

Two notes on the Rust block, so neither reads as something hidden:

- Part 6 emits `warning: muxsmith-gui@0.1.0: GNU compiler is not supported for
  this target` twice. That is a cargo build-script warning from the Tauri
  build when cross-checking Windows from Linux, not a clippy lint; the step
  finishes with exit 0. It cannot be attributable to this change, which
  touches no Rust file at all (see the diffstat below).
- I skipped the Rust block in the mid-task NEEDS_CONTEXT report and said so
  there. The controller ruled that the reasoning does not survive the commit,
  because the gate's contract is no subsets. It was run in full here.

`git diff --stat` before the commit covered exactly the six files in the Files
list, nothing else, and `git status` showed no untracked strays:

```
 e2e/locale-switch.spec.ts         | 192 +++++++++++++++++++++++++++++++++++++-
 locales/de/gui-settings.ftl       |   3 +-
 locales/en/gui-settings.ftl       |   3 +-
 src/components/SettingsDialog.vue |  32 +++++--
 src/i18n/index.ts                 |  18 ++++
 src/main.ts                       |  17 ++--
 6 files changed, 247 insertions(+), 18 deletions(-)
```

---

## 5. The two deviations from the brief's literal text, and their authority

Both are ruled in
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-12/task-2-ruling-1.md`.
Neither was decided at the keyboard: each was returned as a NEEDS_CONTEXT with
a decision memo before any code was written for it, and each waited for the
ruling.

| Brief text | Shipped | Authority |
| --- | --- | --- |
| Step 5: "Every interaction string comes from `en()`" | Interaction LOCATORS resolve through the catalog of the locale rendering at that point: `de("settings-save")` while the interface is German, `en("settings-save")` once it has switched. Assertions unchanged. | ruling 1, "Ruling 1: interaction locators follow the locale that is RENDERING" |
| Step 5 case 3: one case with two legs, "same scenario" | Two cases, one per direction; the second starts from its own `locale: "en"` scenario. Every prescribed assertion preserved, both directions live, no reload. | ruling 1, "Ruling 2: case 3 splits into its two directions" |

The measurements that produced those forks are in the ruling's own
verification section and were re-measured by the controller at the source
before deciding. For the record, the two mechanisms:

- **The locator fork.** With the interface rendering German, a locator for
  `Save` cannot match a button labelled `Speichern`; both affected cases timed
  out at that click.
- **The split fork.** `open()` reassigns `baseline = await getSettings();`,
  and `mocks.ts`'s queue repeats its last entry forever, so a second save
  inside one page compared `null` against `null`, the live-switch guard stayed
  false and `applyLocale` never fired. Confirmed at the time by a temporary
  probe of the Node-side call log:

```
PROBE cmd sequence: ["get_settings","detect_mkvmerge","get_settings","list_runs","get_settings","get_settings","set_settings","get_settings","set_settings"]
PROBE set_settings payloads: ["en",null]
```

  A `get_settings` sits between the two `set_settings` calls - that is the
  second `open()`. The probe was removed; it is in no committed file, and
  section 3's mutation B is the permanent, committed replacement for that
  one-off measurement.

Ruling 1 also records the acceptance-map consequence: **W1-f and W1-g's
producer is now the second case of the split**, not "same test, second leg".
The plan document is not edited; the ruling is the overlay, and a coverage
walk reads both.

---

## 6. The commit

Pathspec-scoped, unsigned, exactly one trailer, staged explicitly (never
`git add -A`), not pushed.

```
$ git log -1 --format="%H%n%s%n%b" ; git log -1 --format="signed: %G?"
ea39d88e61c99bfe7ee3b5d0ba3e3e8491f013a6
settings: a third system-language option, so the effective locale and the shown value agree and the override stays removable
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

signed: N
```

`%G?` is `N` (deliberately unsigned, per SI-4). The trailer count is 1,
measured: `git log -1 --format="%b" | grep -c "Co-Authored-By"` returns `1`.

```
$ git show --stat --format="" HEAD
 e2e/locale-switch.spec.ts         | 192 +++++++++++++++++++++++++++++++++++++-
 locales/de/gui-settings.ftl       |   3 +-
 locales/en/gui-settings.ftl       |   3 +-
 src/components/SettingsDialog.vue |  32 +++++--
 src/i18n/index.ts                 |  18 ++++
 src/main.ts                       |  17 ++--
 6 files changed, 247 insertions(+), 18 deletions(-)

$ git status --short      (after the commit)
                          (empty)
```

Six files, the exact Files list, and a clean tree afterwards. This report is
not in the commit: `.superpowers/` is gitignored (`git check-ignore -v` ->
`.gitignore:2:.superpowers/`). The single push is a controller action at the
plan close.

---

## 7. The two existing `smoke.spec.ts` locale-control assertions

Both are inside the `german locale` describe's settings-save case, `selecting
German in the settings dialog saves it, and it renders the German catalog on
the next start`. Both stay valid, unchanged, and green.

| Assertion | Locator | Why shape A does not change it |
| --- | --- | --- |
| `toHaveValue("en")` | `localeSelect`, the pre-save read | Its scenario mocks no `get_settings`, so it inherits `mocks.ts`'s default fallback, whose `locale` is the concrete string `"en"` (`e2e/mocks.ts`, the `get_settings` branch of `installMockIPC`'s fallback). `open()` now does `baseline.locale ?? SYSTEM_LOCALE`; with a concrete `"en"` the `??` never yields, so the control holds `"en"` exactly as before. Shape A changed only what an ABSENT value displays as. |
| `toHaveValue("de")` | `reloadedLocaleSelect`, the post-reload read | Runs under the case's own `DE_SETTINGS`, whose `locale` is the concrete string `"de"` (`e2e/smoke.spec.ts`, the `DE_SETTINGS` fixture). Same argument: a concrete value, `??` never yields, display unchanged. |

Both figures are the fixtures' own literal values, read from the two files
named, not recalled. Re-run on the final tree:

```
Running 1 test using 1 worker
  ✓  1 [chromium] › e2e/smoke.spec.ts:810:3 › german locale › selecting German in the settings dialog saves it, and it renders the German catalog on the next start (250ms)
  1 passed (681ms)
```

This is also the measured confirmation of D106 decision 7 (both assertions
stay unchanged) and decision 6 (the default `locale: "en"` is untouched): had
shape A disturbed either, this case would be red.

---

## 8. Surfaced, not resolved

Nothing in this list is a change I made; each is for the controller's
disposition.

1. **A new pattern this task establishes, for the ledger's attention** (no
   task edits house-knowledge YAML): the resolution seam `effectiveLocale`.
   The rule it instantiates is that a rule read by more than one module lives
   in the module that owns the domain and is never restated per caller - the
   D106 defect is what happens when it is restated, and it stayed invisible
   for two plans. Worth an entry only if the controller judges it not already
   covered.
2. **A latent constraint the brief encodes implicitly**, worth making explicit
   if this brief is ever reused: the soundness control's end-state figure of 2
   silently forbids the new doc comment from mentioning `navigator.language`,
   a constraint derivable only from the control's own enumeration and never
   stated in Step 1. Complied with. Ruling 1 already carries it as a harvest
   item.
3. **`e2e/mocks.ts` has no stateful settings store.** Ruling 1 records the
   close-action tracker item: when a test needs save-then-reopen without a
   reload, build one. Repeated here only so this report is self-contained; the
   ruling is where it lives.
4. **A gate-shape observation, not a request.** `pnpm lint` runs plain
   `eslint .` with no `--max-warnings`, so eslint warnings - including the
   `vue/singleline-html-element-content-newline` pair measured in section 1 -
   pass the gate silently. Whether that is intended is a house question well
   outside this task; recorded because I tripped it deliberately and watched
   it not fail.

---

## 9. Fix round 1, 2026-07-30: the re-measurement

Written by a fresh implementer, not the author of sections 1 to 8. Scope: this
file. `ea39d88` stands, nothing in `src/`, `locales/` or `e2e/` was touched by
this round, and nothing was committed. The three wrong passages above are left
standing with corrections 1 to 3 beside them.

**Nothing below is restated on the verdict's authority.** Every figure comes
from a run I made myself, in the main working tree on `master`, in the
foreground. Where my measurement goes beyond the verdict, it says so.

**How to read the fenced blocks below.** They are the runs' own output. Two
mechanical shortenings, both marked with `...` where they occur and neither
touching a value: Playwright's repeated `[chromium] › <path> › <describe> ›`
prefixes are elided, and a few over-long source lines quoted inside a failure
block are wrapped. Every count, hash, boolean and file name is as printed.

### 9.1 The instruments, and why they sit outside `e2e/`

Two probe specs and one mutation script, all under
`/home/senol/Git/Muxsmith/.superpowers/fixround-scratch/` - inside the repo so
that `@playwright/test` and the real `e2e/mocks.ts` resolve exactly as they do
for the suite, but under a gitignored path (`.gitignore:2:.superpowers/`) so
they enter neither the tree, nor `pnpm test:e2e`'s typecheck
(`tsc -p e2e/tsconfig.json`, whose `include` is `**/*.ts` relative to `e2e/`
plus `../playwright.config.ts`), nor the suite itself. They run under their own Playwright config whose `testMatch` is
`**/*.probe.ts` and whose `webServer` reuses the same `vite preview` over
`dist/`. The prior implementer worked from `e2e/`; nothing here was inherited
from it, and none of its files were executed.

The mutation script refuses to run unless its anchor string occurs **exactly
once**, and restores from a hash-verified snapshot - a silently missed mutation
looks identical to a mutation that failed to discriminate.

### 9.2 The baseline, before anything was touched

```
$ git status --porcelain
                          (empty)

$ pnpm build
dist/assets/index-DGn2eD1R.css    1.31 kB │ gzip:   0.49 kB
dist/assets/index-CjJCKxvO.js   326.45 kB │ gzip: 106.09 kB
✓ built in 155ms

$ pnpm test:e2e
  72 passed (3.0s)
```

The built bundle name `index-CjJCKxvO.js` is content-hashed; it reappears
below as the restoration control.

### 9.3 Finding I1: the prescribed case 3, rebuilt and run

Reconstructed from the brief's own Step 5 text: one case, two legs, one page,
the same `locale: null` scenario, under `test.use({ locale: "de-DE" })`, with
interaction locators per ruling 1 (`de("settings-save")` while German,
`en("settings-save")` once English) so that the ONLY thing under measurement is
the split fork. Run against the committed tree.

```
$ pnpm exec playwright test --config=.superpowers/fixround-scratch/probe.config.ts
Running 3 tests using 1 worker

PROBE-LEG1-ALL-FOUR-PRESCRIBED-ASSERTIONS: passed
PROBE-LEG2-PERSISTED-ASSERTION: passed
  ✘  1 [chromium] › prescribed-case-3.probe.ts:77:3 › brief Step 5 case 3, as PRESCRIBED › case 3, the round trip (verbatim, hard assertions) (5.2s)
PROBE-AT-LOAD {"deHeading":1,"enHeading":0,"lang":"de"}
PROBE-AFTER-LEG1 {"persisted":["en"],"deHeading":0,"enHeading":1,"lang":"en"}
PROBE-CONTROL-VALUE-ON-REOPEN ""
PROBE-AFTER-LEG2 {"persisted":["en",null],"deHeading":0,"enHeading":1,"lang":"en"}
PROBE-LEG2-PRESCRIBED-ASSERTIONS {"recorded set_settings.locale === null":true,"the de heading back":false,"documentElement.lang === \"de\"":false}
  ✓  2 [chromium] › ... › case 3, instrumented: every prescribed leg-2 assertion evaluated (226ms)
PROBE-CONTROL: de heading + lang=de assert GREEN with this instrument
  ✓  3 [chromium] › ... › CONTROL: the same instrument goes green where it must (first run, no save) (57ms)

  1) case 3, the round trip (verbatim, hard assertions)

    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-batch').getByRole('heading', { name: 'Stapel', exact: true })
    Expected: visible
    Timeout: 5000ms
    Error: element(s) not found

    > 118 |     ).toBeVisible();

  1 failed
  2 passed (6.4s)
```

Read off that run:

- Leg 1's four prescribed assertions all pass.
- Leg 2's persisted assertion passes: the second `set_settings` carries
  `locale: null` (`"persisted":["en",null]`).
- Leg 2's other two prescribed assertions are **false**: no German heading is
  present (`deHeading: 0`), and `documentElement.lang` is still `"en"`.
- The prescribed case therefore FAILS on a correct implementation. Always red,
  never green.

**The fired control, because a red probe and a broken probe look identical.**
Test 3 runs the same helpers, the same mock wiring and the same
`de("batch-view-heading")` assertion in a shape that must pass, and it passes.
`PROBE-AT-LOAD {"deHeading":1,...,"lang":"de"}` shows the same assertion holding
inside the failing case itself, before leg 1. So the red at leg 2 is the
product's correct behaviour meeting an unreachable assertion, not a
misconfigured instrument.

`PROBE-CONTROL-VALUE-ON-REOPEN ""` is the mechanism in one value: after leg 1
saved `"en"`, the reopened dialog reads the baseline back from the mock and gets
`null` again, so the control shows the sentinel and the second save compares
`null` against `null`.

### 9.4 Finding M1: mutation A re-run, and what it actually discriminates

```
$ python3 .superpowers/fixround-scratch/mutate.py snapshot
snapshot taken: d7c2ab8e0e52d4578a3120a68798aef0c63485ad6887c5c27c9508a7e7bdc649

$ python3 .superpowers/fixround-scratch/mutate.py apply
mutation A applied: e9c654b89c02be66b7ee24f9b4a9aa52490789f589f7a5d54c127b687b593532
  return saved ?? navigator.language;  ->  return saved ?? "en";

$ git diff --stat
 src/i18n/index.ts | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)

$ pnpm build
dist/assets/index-vFA2Btem.js   326.44 kB │ gzip: 106.08 kB
✓ built in 157ms

$ pnpm test:e2e
  4 failed
    e2e/locale-switch.spec.ts:190:3 › first run: the interface follows the system language and the control says so
    e2e/locale-switch.spec.ts:216:3 › saving without touching the language writes no override
    e2e/locale-switch.spec.ts:244:3 › leaving the system language: picking English stores the override and switches live
    e2e/locale-switch.spec.ts:270:3 › returning to the system language: removing the override stores null and switches live
  68 passed (31.2s)
```

**4 failed, 68 passed.** Section 3's table carries no count, but its row A
claim reproduces exactly as stated there: the four new cases go red, the D56
case stays green, and nothing else in the suite moves. The four death sites,
from the same run's failure blocks:

```
  1) first run
    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-batch').getByRole('heading', { name: 'Stapel', exact: true })
    > 201 |     await expect(batch.getByRole("heading", { name: de("batch-view-heading"), ... })).toBeVisible();

  2) saving without touching the language
    Test timeout of 30000ms exceeded.
    Error: locator.click: Test timeout of 30000ms exceeded.
    > 227 |     await dialog.getByRole("button", { name: de("settings-save"), exact: true }).click();

  3) leaving the system language
    Test timeout of 30000ms exceeded.
    Error: locator.click: Test timeout of 30000ms exceeded.
    > 258 |     await dialog.getByRole("button", { name: de("settings-save"), exact: true }).click();

  4) returning to the system language
    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-batch').getByRole('heading', { name: 'Stapel', exact: true })
    > 303 |     await expect(batch.getByRole("heading", { name: de("batch-view-heading"), ... })).toBeVisible();
```

Two die at a German rendering assertion, which is a real kill below the
`[requested, en]` fallback. Two die at an interaction locator, which ruling 1
forbids counting as a witness.

**Do those two cases have any assertion that discriminates the seam?** Asked as
a measurement rather than an argument: both were replicated with the single
change of an `en()` Save locator instead of `de()`, still under mutation A,
still with the mutated `dist/` in place.

```
$ pnpm exec playwright test --config=.superpowers/fixround-scratch/probe.config.ts mutation-a-locator-free
Running 3 tests using 1 worker

PROBE: 'saving without touching the language' assertions ALL PASS under mutation A
  ✓  1 › saving without touching the language: only its ASSERTIONS (137ms)
PROBE: 'leaving the system language' assertions ALL PASS under mutation A
  ✓  2 › leaving the system language: only its ASSERTIONS (115ms)
PROBE-CONTROL first run at load: deHeading=0 lang=en
  ✓  3 › CONTROL: 'first run' still dies on its assertion with the locator removed (51ms)

  3 passed (953ms)
```

Both cases pass in full. Neither has a single assertion that can tell a correct
seam from a broken one; under A they die only because the interface renders
English and the German Save label does not exist.

**The fired control for those two greens**, since two passes are exactly what a
run against a stale, unmutated `dist/` would also produce: test 3 asserts the
MUTATED state directly - `deHeading` is 0 and `lang` is `"en"` at page load,
which is *first run*'s own assertion inverted. It passes, so the mutant was live
in the bundle those two greens were measured against.

### 9.5 Restoration, verified

```
$ python3 .superpowers/fixround-scratch/mutate.py restore
restored: snapshot d7c2ab8e0e52d4578a3120a68798aef0c63485ad6887c5c27c9508a7e7bdc649 -> target d7c2ab8e0e52d4578a3120a68798aef0c63485ad6887c5c27c9508a7e7bdc649
IDENTICAL

$ git status --porcelain
                          (empty)

$ git diff --stat
                          (empty)

$ sha256sum src/i18n/index.ts
d7c2ab8e0e52d4578a3120a68798aef0c63485ad6887c5c27c9508a7e7bdc649  src/i18n/index.ts

$ pnpm build
dist/assets/index-CjJCKxvO.js   326.45 kB │ gzip: 106.09 kB
✓ built in 155ms

$ pnpm test:e2e
  72 passed (3.0s)
```

Three independent restoration witnesses: the source hash equals the
pre-mutation snapshot, `git status`/`git diff` are empty, and the rebuilt bundle
is byte-identical by content hash to the baseline of 9.2
(`index-CjJCKxvO.js`, against `index-vFA2Btem.js` while mutated). The empty
`git status` is not an untested absence: the same command printed
`src/i18n/index.ts | 2 +-` while the mutation was applied.

### 9.6 The sweep: how the sites were found, and one instrument blind spot

The claim was swept for, not the finding's list of locations. Method: this
report read end to end, then three greps whose patterns were derived from the
wording actually on the page rather than from recall. Final expression and its
result across the repo, run BEFORE the corrections above were inserted - so the
line numbers in this paste are the pre-correction ones, which is also how the
verdict cites them:

```
$ command grep -rn -E "measuring nothing|shipped green|could not make fail|restored the assertion|workaround for a fixture|kills exactly" \
    --include="*.md" --include="*.ts" --include="*.vue" --include="*.yaml" --include="*.ftl" .
./.superpowers/sdd/plan-12/task-2-report.md:240:ruling 2 repaired was precisely a case that would have shipped green while
./.superpowers/sdd/plan-12/task-2-report.md:241:measuring nothing on the live path. So each mechanism the cases claim to
./.superpowers/sdd/plan-12/task-2-report.md:282:Each mutation kills exactly the cases whose claims depend on it and spares the
./.superpowers/sdd/plan-12/task-2-report.md:286:single-case form could not make fail, which is exactly why the split was
./.superpowers/sdd/plan-12/task-2-report.md:287:ruled. The split is not a workaround for a fixture limitation - it is what
./.superpowers/sdd/plan-12/task-2-report.md:288:restored the assertion's power.
./.superpowers/sdd/plan-12/task-2-ruling-1.md:64:**as written, the case would have shipped green while measuring nothing on the
./.superpowers/sdd/plan-12/task-2-verdict.md:55:would have shipped green while measuring nothing on the live path") and the
./.superpowers/sdd/plan-12/task-2-verdict.md:95:mutation kills exactly the cases whose claims depend on it". For mutation A
./.superpowers/sdd/plan-12/task-2-verdict.md:341:   inverts easily.** The instance is finding I1: "would have shipped green" and
./.superpowers/sdd/plan-12/progress.md:171:case would have shipped GREEN while measuring nothing on the live path.
./.superpowers/sdd/plan-12/progress.md:218:"shipped green while measuring nothing" - a false-green, and an instance of the
./e2e/locale-switch.spec.ts:167: *  measuring nothing on the live path (controller ruling 1, Plan 12 Task 2). */
```

(Elided from the paste: matches in plan-5, plan-11 and their `.worktrees/`
copies where the same phrases carry an unrelated sense - "cancel_job(index)
kills exactly that job", "the shipped green state paired with a bounded diff".)

Three passages in THIS file, and they are the three this round corrected. Of
the rest, the ruling's line 64 is where the claim originated and already carries
the controller's correction beside it; the verdict's three lines are the
findings quoting the claim, which is correct usage. The two that remain are the
subject of 9.7. Section 1's opening carries no version of the claim, and neither
does section 5, whose mechanism description is correct as written.

**The blind spot, recorded because it nearly cost the sweep.** In this shell
`grep` is a wrapper around `ugrep` with `--ignore-files`, which honours
`.gitignore` - and `.superpowers/` IS gitignored. A recursive `grep` from the
repo root therefore **cannot see any SDD artifact at all**: the first repo-wide
sweep returned only the `e2e/` hit and looked like a clean result. Fired
control, run against a line proven present by a direct file grep:

```
$ grep -rn "shipped green" .superpowers/sdd/plan-12/task-2-report.md ; echo "exit=$?"
.superpowers/sdd/plan-12/task-2-report.md:240:ruling 2 repaired was precisely a case that would have shipped green while
exit=0

$ grep -rn --include="*.md" "shipped green" .superpowers/ ; echo "exit=$?"
exit=1

$ type grep
grep is a shell function from /home/senol/.claude/shell-snapshots/snapshot-zsh-...sh
```

Same string, same tool, one hit and one miss, decided only by whether the path
was named explicitly or reached by recursion. Any sweep over the SDD artifacts
must use `command grep` or name the paths.

### 9.7 Surfaced, not fixed: two further sites of the same wrong claim

Both are outside this round's scope (report file only), and both are for the
controller's disposition:

1. **`e2e/locale-switch.spec.ts`, the `EN_OVERRIDE_SETTINGS` doc comment**
   (lines 160 to 167 at `ea39d88`): "*a second save inside one page would
   compare `null` against `null`, the live-switch guard would stay false, and
   the case would pass while measuring nothing on the live path*". Same
   inversion, in **committed source**, where it will outlive every document in
   `.superpowers/`. The mechanism half of that comment is correct; only "would
   pass while measuring nothing" is wrong, and the sentence stays true with
   "would fail at its German rendering assertions, which no fixture in this file
   can satisfy". Untouched here because the round's scope is the report.
2. **`.superpowers/sdd/plan-12/progress.md:171`** carries the original wrong
   claim in the NEEDS_CONTEXT log entry. That file is an append-only controller
   log and its own later entry (around line 218) records the correction, so it
   is arguably self-correcting; named so the decision is made rather than
   assumed.

### 9.8 Where this round agrees and disagrees with the verdict

- **I1: agrees, independently.** My reconstruction reproduces the verdict's
  result value for value, including which one of leg 2's three prescribed
  assertions passes.
- **M1: agrees on the four death sites and extends the finding.** The verdict
  names only *saving without touching the language* as having no seam-dependent
  claim. Measured (9.4), *leaving the system language* has none either: with the
  locator death removed, its full assertion set passes under mutation A. So
  mutation A's reach is two of four cases, not three of four.
- **No disagreement was found.** One figure the correction blocks deliberately
  do NOT restate: the "69 passed, 2 failed" mid-flight gate line quoted in
  `task-2-ruling-1.md` and `progress.md`. It is attributed there to the
  implementer's mid-flight NEEDS_CONTEXT memo, which is not among the plan-12
  artifacts on disk and is not in this report; I did not measure it, so it is
  not carried here as if I had.

---

## 10. Fix round 2, 2026-07-31: the wrong clause removed from committed source

Scope: exactly the site section 9.7 item 1 surfaced and left for disposition -
the `EN_OVERRIDE_SETTINGS` doc comment in `e2e/locale-switch.spec.ts`. One file
touched, comment only. The ruling document itself was corrected by the
controller before this round started; this round carries the correction into the
tree, where it outlives every artifact under `.superpowers/`.

### 10.1 The sweep, and the instrument it was run with

`grep` in this shell is a wrapper function that silently skips gitignored paths
on a recursive search from the repo root, so the sweep deliberately avoided the
bare name. Two instruments, both stated:

- `command grep -nEi <pattern> e2e/locale-switch.spec.ts` for the in-file sweep
  (explicit path, wrapper bypassed).
- `git grep -nEi <pattern> -- .` for the repo sweep. `git grep` searches tracked
  files, and `git status --porcelain` returned empty both before and after, so
  there were no untracked-but-unignored files for it to miss: over the
  non-ignored tree, tracked coverage is total coverage here.

Four patterns, aimed at the four separable halves of the claim rather than at
one phrase:

```
git grep -nEi "measuring nothing|measures nothing|measure nothing" -- .
git grep -nEi "guard would stay false|live-switch guard|live switch guard" -- .
git grep -nEi "null\` against \`null|null against null" -- .
git grep -nEi "EN_OVERRIDE|two directions|opposite direction|two cases, one scenario" -- .
```

Result: **one site** carries the false consequence, `e2e/locale-switch.spec.ts`
lines 166 to 167. The first three patterns each return that one file and nothing
else. The fourth returns only unrelated matter (`broken_override` Rust test
names, prose about opposite directions in ledger and journal entries), i.e. the
split's reasoning is not restated anywhere else in the tracked tree.

Two in-file near-misses were read and left standing, because both are true and
neither is this claim: line 30 (an id whose de and en values are identical
"would assert green even if the interface had fallen back to English"), and the
two "not green merely because" control comments at lines 236 and 282. Line 242's
pointer - "see EN_OVERRIDE_SETTINGS for why one page cannot carry both" - stays
correct after the rewrite, since the rewritten comment still says one page
cannot carry both, for the corrected reason.

`.superpowers/sdd/plan-12/progress.md:171` (section 9.7 item 2) was **not**
touched: it is gitignored, outside this round's one-file scope, and its
disposition is the controller's.

### 10.2 The claim verified before the rewrite, not after

Two halves, each checked against its own source rather than taken on authority.

**The fixture mechanism, read at the source.** `e2e/mocks.ts` is stateless in
exactly the way the comment says: the response queue's `nextResult` returns
`q.length > 1 ? q.shift() : q[0]`, so a single-entry `get_settings` queue repeats
the same baseline forever, and `set_settings` is answered with `null` and never
feeds it (`mocks.ts` lines 89 to 95 and 126 to 128). `SettingsDialog.vue`
`open()` reassigns `baseline = await getSettings()` on every open (line 39), and
`save()` guards the live swap on `if (next.locale !== baseline.locale)` (line 82)
with `baseline = next` only afterwards (line 85). So a reopen inside one page
discards the in-memory `baseline` the first save had set and restores the
fixture's `null`.

**The red/green claim, measured by construction and run.** The merged
single-case form does not exist in the tree, so it was built as a temporary
probe inside `e2e/locale-switch.spec.ts` (first save: system -> en; second save
inside the SAME page: en -> system), run, and then reverted with
`git checkout --` before any edit was made. Verbatim from the run:

```
Running 1 test using 1 worker

PROBE select value on reopen: 
PROBE persisted locales: ["en",null]
PROBE documentElement.lang: en
PROBE de heading count: 0
PROBE en heading count: 1
  x  1 [chromium] > e2e/locale-switch.spec.ts:307:3 > system-locale default (D106) > TEMP PROBE merged single-case form -- NOT FOR COMMIT (2.3s)

    Error: expect(locator).toBeVisible() failed

    Locator: getByTestId('view-batch').getByRole('heading', { name: 'Stapel', exact: true })
    Expected: visible
    Timeout: 2000ms
    Error: element(s) not found

  1 failed
```

Reading it: the persisted assertions `expect(writes).toHaveLength(2)` and
`expect(writes[1].locale).toBeNull()` both **passed** - they precede the failure
and the persisted locales are `["en",null]`. The failure is the German heading,
which never returns (`de heading count: 0`, `en heading count: 1`), and
`documentElement.lang` is still `en`, so the second rendering assertion would
have failed too had execution reached it. The empty `select value on reopen`
is the mechanism made visible: the reopened dialog already shows the system
sentinel, because `open()` re-read the fixture's `null`, while the page is
rendering English.

**This round's measurement agrees with the controller's in every value**: the
merged form is an always-red case, not a false green. No disagreement to report.

### 10.3 The rewrite

`e2e/locale-switch.spec.ts`, the `EN_OVERRIDE_SETTINGS` doc comment. Everything
the comment got right is kept (the stateless mock, `set_settings` not feeding
`get_settings`, `open()` re-reading the baseline, both sides of the guard being
`null`), it stays a comment about why this constant exists, and the citation of
controller ruling 1 stays - the ruling now carries its own marked correction, so
the pointer remains useful. The new final clause, verbatim:

```
 *  live-switch guard would stay false, and the merged case would go RED
 *  against a correct implementation: its `set_settings` assertion still
 *  passes (`null` is persisted), while both rendering assertions fail --
 *  the German heading never comes back and `<html lang>` stays `en`. So the
 *  split is here because that direction cannot be made green inside one page
 *  without a reload or a second fixture state, NOT because a merged form
 *  would be a false green (controller ruling 1, Plan 12 Task 2). */
```

Why the last sentence is in it rather than only the corrected mechanism: a
false-green is an instance of the assertion-below-a-fallback class this project
guards hardest, and a reader who finds "vacuous green" in a comment about a test
split will file this case under that class. Saying explicitly that it is NOT one
keeps the distinction from having to be re-derived.

### 10.4 The gate, foreground, all 11 parts, no subsets

Run from `/home/senol/Git/Muxsmith` on the edited tree, in `BUILDING.md`'s own
enumeration order. Exit codes captured with `$?` per command, each written to its
own log: the shell here is zsh, where `PIPESTATUS` is empty and the real array is
`pipestatus`, so two earlier `${PIPESTATUS[0]}` readings came back blank and were
re-run rather than read as success.

| # | Part | Exit | Observed |
| - | - | - | - |
| 1 | `cargo fmt --all --check` | 0 | no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `Finished dev profile ... in 0.12s` |
| 3 | `cargo test --workspace` | 0 | 39 `test result: ok` lines, `total passed: 507`, no `test result: FAILED` |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 | `Generated .../muxsmith_cli/index.html and 5 other files` |
| 5 | `cargo deny check` | 0 | `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 | `Finished dev profile ... in 0.14s` (plus the standing `GNU compiler is not supported for this target` build-script warning) |
| 7 | `pnpm lint` | 0 | `$ eslint .`, no findings |
| 8 | `pnpm build` | 0 | `built in 158ms` |
| 9 | `pnpm check:i18n` | 0 | `check-i18n: ok (41 source files scanned, 213 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | 0 | `72 passed (3.0s)` |
| 11 | `python3 scripts/ledger-lint.py` | 0 | `ledger-lint: 568 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

The 72 e2e cases include all **five** cases of the edited file - the D56
live-switch case at line 104 plus the four D106 system-locale cases at 195, 221,
249 and 275 - counted off the run log, not off recall (`command grep -oE
"locale-switch\.spec\.ts:[0-9]+:[0-9]+ .*"` over the `pnpm test:e2e` log, five
distinct entries). The probe from 10.2 is not among them, having been reverted
before the gate ran.

### 10.5 The commit

`e778ddadc9eb5f2e8e5e1d6a247437a55a198f70`

```
e2e: the split's reason is an always-red case, not a vacuous green one

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

Staged pathspec-scoped (`git add e2e/locale-switch.spec.ts`, commit with a
`-- e2e/locale-switch.spec.ts` pathspec), unsigned (`commit.gpgsign=false`;
`git log -1 --format='%G?'` returns `N`), exactly one trailer (`git log -1
--format='%b' | command grep -c "Co-Authored-By"` returns `1`), on `master` in
the main working tree. Diff: 1 file changed, 7 insertions, 2 deletions, comment
only. **Not pushed.**
