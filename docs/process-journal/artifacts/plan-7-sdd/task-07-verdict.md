# Task 7 (D56) verdict - independent SDD review

**VERDICT: APPROVED**

Commit `72c07ee` on `plan7-b` (parent `2422a58` = approved Task 6). D56: live
in-session locale switch. `src/i18n/fluent.ts` (new) owns the single fluent-vue
instance and the reactive `currentLocale`; `applyLocale` reassigns
`fluent.bundles` to a fresh array, updates `currentLocale`, and sets
`<html lang>`; `main.ts` re-points to the owned module; `SettingsDialog.save()`
calls `applyLocale` post-save on locale change; the `settings-locale-label.hint`
is rewritten evergreen (en+de), dropping the now-false restart claim; a new
`e2e/locale-switch.spec.ts` proves the live swap. 7-file diff, full GUI gate
green.

The implementer shipped DONE with one flagged type-narrow deviation (the plan's
Step-4 literal fails `vue-tsc`) and two out-of-scope stale-comment finds
surfaced-not-edited. I rule the deviation a **grant-covered mechanical repair of
a non-compiling plan line** (the T6 precedent, and pre-authorized by T6 harvest
H3 naming T7 explicitly), correctly executed and disclosed; the two stale
comments are genuinely stale and correctly left for the whole-branch review. No
fixes required.

Every load-bearing claim was re-derived from the tree. The plan-literal type
error was reproduced by running `vue-tsc` against it in-tree; the "main.ts no
longer creates an instance" absence was fire-verified against the parent commit
as a known-present control; the probed file was restored byte-identically
(`command cp -f`; `cmp` identical; `git status` clean).

---

## Findings

### 1. Spec compliance (D56 mechanism + owner surface) - PASS

- **`src/i18n/fluent.ts` owns the single instance.** The module exports
  `currentLocale = shallowRef("en")`, `fluent = createFluentVue({ bundles:
  buildBundles("en") })`, and `applyLocale(locale: string)` doing exactly the
  three D56 lines: `currentLocale.value = locale`; `fluent.bundles =
  buildBundles(locale)` (fresh array, never mutation, so the shallowRef fires);
  `document.documentElement.lang = primarySubtag(locale)`. Matches the design's
  code block verbatim plus the imports the plan Step 2 mandates.
- **`main.ts` no longer creates an instance.** Diff removes the
  `createFluentVue` import and the inline `const fluent = createFluentVue(...)`,
  now `import { applyLocale, fluent } from "./i18n/fluent"` and `applyLocale(locale)`
  before `createApp(App).use(fluent).mount("#app")`. Bootstrap behavior
  unchanged (first paint in the resolved locale). **Fire-verified** (see
  section below): parent `2422a58:src/main.ts` HAS `createFluentVue` at lines
  2/26 (control fires); current `src/main.ts` has none (`grep` exit 1).
- **No second production `createFluentVue`.** `grep -rln createFluentVue src/`
  returns exactly one file: `src/i18n/fluent.ts`. The only other repo sites are
  `e2e/mount-entry.ts` (a pre-existing, test-only component-mount harness, NOT
  touched by this commit) and its gitignored generated bundle - not the
  production app instance (see H4).
- **`primarySubtag` exported, not duplicated.** `src/i18n/index.ts` flips
  `function primarySubtag` to `export function primarySubtag`; the one-line diff
  is an export keyword only, the existing implementation reused. Correct per
  plan Step 2 / D56 ("exported rather than duplicated").
- **`SettingsDialog` hook is post-save AND on-change only.** The call sits after
  `await setSettings(next)` (so a failed save never switches - the `catch`
  short-circuits it) and inside `if (next.locale !== baseline.locale && ...)`
  before `baseline = next`. Both D56 conditions honored.
- **Hint rewrite, en+de, evergreen, no enumeration.** en: `Which language the
  Muxsmith interface uses.`; de: `In welcher Sprache Muxsmith seine Oberflaeche
  anzeigt.` [umlaut rendered in file]. The stale "takes effect after restarting
  Muxsmith" / "A changed language takes effect after restarting" sentence is
  gone in both; no locale is enumerated (`i18n-15-settings-hint-evergreen`
  satisfied). `check:i18n` id-parity green confirms both landed bilingual.
- **v1-spec amendment 5 correctly NOT touched.** The plan's dependency graph
  (line 48) assigns amendment 5 ("takes effect live, without restart") to
  **wave-4 Task 21** ("amendment 5 needs D56"); Task 7 produces the mechanism
  amendment 5 later asserts. The 7-file diff touches no spec/design file. Right
  boundary - editing the v1 spec here would be a wave-4 job done early.

### 2. The e2e method - genuinely proves live switching - PASS (one coverage note)

`e2e/locale-switch.spec.ts` drives the real served app with mocked IPC
(`installTauriMocks`, `smoke.spec.ts` discipline). It:

- asserts the app starts English (`en("batch-view-heading")` visible, `lang ==
  "en"`);
- sets `document.body.dataset.localeSwitchAlive = "yes"` **before** the switch;
- opens settings, `selectOption("de")`, clicks Save (`en("settings-save")`),
  waits for the dialog hidden;
- **(a)** asserts the batch heading now renders `de("batch-view-heading")`
  **and** the en heading is `toHaveCount(0)` - a real in-place re-render, not an
  addition;
- **(b)** asserts the body marker still reads `"yes"` - no reload happened;
- **(c)** asserts `documentElement.lang == "de"`.

The expected German is derived from the real `locales/de` catalog through a
local `buildDeBundle`/`de()` helper over `@fluent/bundle` (mirrors
`i18n-en.ts`), never a hardcoded string - so the assertion is byte-identical to
what the app itself renders. The marker sits on `document.body`, **outside**
`#app`, which is the correct witness: a reactive bundle swap never touches
`document.body`, a `page.reload()` recreates the document and wipes it.

**Ran it** (`pnpm test:e2e --grep "locale"`, correct no-`--` form per the
stream-B constraint): 3 passed (the D56 spec + the two german-locale smoke
tests). **Ran the full suite** (`pnpm test:e2e`): **33 passed** (the expected
count). Notably `smoke.spec.ts:685` (the T21.5 reload-based german test) still
passes post-D56 - the live switch does not break it.

Coverage note (not a defect, plan-conformant): the marker distinguishes
reload from non-reload but does not by itself distinguish a reactive swap from
an unmount+remount (a remount would also preserve a `body`-scoped marker while
losing `#app` view state). View-state retention therefore rests on the
mechanism (reactive `fluent.bundles` reassignment + `v-show`-mounted views in
`App.vue:117-137`, corroborated by `smoke.spec.ts:1251` proving the `v-show`
tab retention), not on a dedicated state-survival assertion across the switch.
This is exactly what the plan specifies - Step 1(b) asks only for the no-reload
marker and Step 6 designates "the spec's marker assertion" as the view-state
guard - so it is plan-conformant, not an implementer miss. Carried to H1.

### 3. THE DEVIATION - the `!== null` type-narrow - ADJUDICATED grant-covered repair

**The deviation.** Plan Step 4 hands over:
```ts
if (next.locale !== baseline.locale) applyLocale(next.locale);
```
The implementer shipped:
```ts
if (next.locale !== baseline.locale && next.locale !== null) {
  applyLocale(next.locale);
}
```
with a site comment stating the `!== null` narrows `AppSettings.locale`
(`string | null`) to the `string` `applyLocale` takes, that `next.locale` is
always set from `form.locale` at runtime, and that the guard only satisfies the
type. Disclosed in the report ("1 anticipated type-narrow").

**Reproduction (this reviewer, not borrowed).** I confirmed the shipped code
type-checks (`pnpm exec vue-tsc --noEmit` -> exit 0), then applied the plan's
literal in-tree and re-ran:
```
src/components/SettingsDialog.vue(71,54): error TS2345: Argument of type
'string | null' is not assignable to parameter of type 'string'.
  Type 'null' is not assignable to type 'string'.
vue-tsc exit: 2
```
Then restored (`command cp -f` from a pre-edit backup; `cmp` byte-identical;
sha `110a92e4...`; `git status` clean). The type facts, all read from the tree:
- `AppSettings.locale: string | null` (`src/ipc.ts:33`), mirroring the Rust
  `settings.rs::AppSettings`.
- `next` is declared `const next: AppSettings` (`SettingsDialog.vue:58`), so
  `next.locale` has static type `string | null` even though the assigned value
  is `form.locale`; `!==` against another `string | null` does not narrow it.
- `applyLocale(locale: string)` (`src/i18n/fluent.ts:16`).
- `tsconfig` strict; `pnpm build` runs `vue-tsc --noEmit` as a mandatory gate.

So the plan's literal is genuinely non-compiling and would break the build gate;
transcribing it verbatim is not an option.

**Runtime claim verified at source.** `form` is `reactive({... locale: "en"})`
(`:19-23`); `open()` sets `form.locale = baseline.locale ?? "en"` (`:39`); the
`<select v-model="form.locale">` offers only `"en"`/`"de"` (`:143-154`).
`next.locale = form.locale` (`:62`). So `next.locale` is always a non-null
string at runtime; the `!== null` guard never fails in practice and only
satisfies the type checker. Claim holds.

**Adjudication.** Governing rule: `brief-drafts-verified-against-tree` (a plan
literal is verified against the tree, a non-compiling divergence is
adapted-and-surfaced, never transcribed). This is nearly isomorphic to the T6
precedent (`resolvedTrackLabel` literal failed `vue-tsc` TS2322; adapted with an
invariant-backed null narrowing, disclosed, reviewer-reproduced; ruled
grant-covered) and was **explicitly pre-authorized**: T6 harvest H3 named "T7
(D56)" as inheriting the same constraint, and the controller cross-task note
(controller-notes.md:144-147) travels it verbatim into the T7 dispatch. The
implementer discharged the obligation in full: reproducible compile failure, a
minimal type-satisfying repair, disclosed at the site and in the report. The
repair form here is actually cleaner than T6's - when `locale` is null
(impossible at runtime) the guard simply **skips** `applyLocale`, which is the
correct degenerate behavior (nothing to switch to), so no placeholder token is
needed (unlike T6's rendering case). **Not** a skipped NEEDS_CONTEXT: a defect
in the plan's own transcribed literal is keyboard-adaptation-plus-disclosure
territory, not a design-latitude fork. Ruling upheld.

### 4. Non-re-rendering surfaces - enumerated in D56, structurally unreachable - PASS

D56 (design lines 857-863) enumerates what does **not** re-render and says so
explicitly: the native close-abort dialog (`run.rs:543` en-only `include_str!`),
the CLI (locale resolved **per invocation** at process start - a GUI live switch
reaches no separate process), the static `index.html` window title, and native
OS chrome (file pickers). `applyLocale` touches exactly three things -
`fluent.bundles`, `currentLocale`, `document.documentElement.lang` - none of
which can reach a separate CLI process, the Rust-side native dialog, the static
HTML title (`index.html:6` is a literal `<title>Muxsmith</title>`;
`applyLocale` sets `documentElement.lang`, never `document.title`), or an
OS-native picker. No silent expectation gap: the design lists every one and the
mechanism structurally cannot touch them.

### 5. Quality - PASS

- **7-file diff exact.** Exactly the plan's Files list: create
  `src/i18n/fluent.ts`; modify `src/i18n/index.ts`, `src/main.ts`,
  `SettingsDialog.vue`, `locales/{en,de}/gui-settings.ftl`; add
  `e2e/locale-switch.spec.ts`. Nothing extra.
- **Commit discipline.** Message matches plan Step 7 verbatim
  ("gui: live locale switch via owned fluent module and bundles swap; evergreen
  locale hint (D56, gui-26 closure)"); `Co-Authored-By: Claude Fable 5` trailer
  present; unsigned; 7 files, no `git add -A` residue (tree clean).
- **Gate GREEN** (run foreground, this reviewer, exit codes captured):
  `pnpm lint` -> 0; `pnpm build` (`vue-tsc --noEmit && vite build`) -> 0;
  `pnpm check:i18n` -> 0 ("208 catalog ids, 17 unused warning(s), 1 other locale
  checked for parity" - the 17 unused are the pre-existing IpcError-code ids,
  unchanged from T6, non-fatal); `pnpm test:e2e` -> **33 passed**. (The task
  touches no Rust; the plan's Task-7 Step 6 specifies exactly these four GUI
  commands, and the cargo half of the nine-part gate is unaffected by a
  GUI-only diff.)
- **Two stale comments correctly surfaced-not-edited.** Both verified genuinely
  stale:
  1. `e2e/mocks.ts` (~71-77): "there is no live in-session catalog swap;
     `main.ts` resolves the locale once, before mount" - **now false** post-D56.
     Mechanism it documents (layered `installMockIPC` via `addInitScript`) still
     works.
  2. `e2e/smoke.spec.ts` T21.5 (~671-674): "`main.ts` resolves the locale
     exactly once, before mount, and nothing in the app swaps the live
     `FluentBundle`s afterwards -- a saved locale change takes effect on the
     next start, same as a restart" - the "nothing swaps afterwards" clause is
     **now false** post-D56. The test still passes (it asserts `set_settings`
     saved "de", then reloads with a de-seeded mock; the reload is now redundant
     for proving de, but the assertions hold).
  Neither file is in T7's Files list; editing them would be scope creep. Leaving
  them and routing to the whole-branch review (already captured in
  controller-notes.md:152-155) is correct.

---

## Fire-verification of my own absence checks

- **Absence claim: `main.ts` no longer creates a fluent-vue instance.** Control
  = parent `2422a58:src/main.ts`, which HAS `createFluentVue` at lines 2 and 26
  (`git show 2422a58:src/main.ts | grep createFluentVue` fires). Target =
  current `src/main.ts`: same pattern, `grep` exit 1, no output. The control
  proves the pattern is well-formed and fires on presence, so the empty target
  result is a true absence, not a malformed grep.
- Corroboration for "single production instance": the same `grep -rln
  createFluentVue src/` returns exactly `src/i18n/fluent.ts` - it does fire (one
  hit), and the known other sites (`e2e/mount-entry.ts`, the generated bundle)
  live outside `src/`, so the src/-scoped pattern is not silently missing them
  by path error.
- Restore discipline: pre-edit backup taken (`command cp -f` to scratchpad, sha
  `110a92e4...`); after the type-repro edit, restored with `command cp -f` from
  the backup; `cmp` byte-identical; final sha `110a92e4...` matches;
  `git status --porcelain` empty.

---

## HARVEST

**H1 - e2e "views keep state" guard is no-reload-only, not remount-proof
(coverage characteristic, not a T7 defect).** The spec's view-state guard is the
`document.body` marker, which distinguishes reload from non-reload but not a
reactive swap from an unmount+remount (a remount preserves a `body`-scoped
marker while wiping `#app` state). View-state retention is established by
mechanism-reading (reactive `fluent.bundles` reassignment + `v-show` views),
not by a behavioral assertion. The plan itself designates the marker as the
guard (Step 1b, Step 6), so the implementer is conformant. Durable wrinkle: if a
future refactor ever made `applyLocale` remount, this e2e would stay green while
silently regressing the design's central "views keep state" promise. Candidate
for the whole-branch review: a one-shot stateful-view-across-switch assertion
(set a value in a mounted view, switch locale, assert the value survives) would
close the gap; low cost, and it is the assertion that actually witnesses the
design's differentiator over the rejected remount alternative.

**H2 - over-restriction watch (compile-forced repair, second occurrence).** The
type-narrow is NOT governed by the zero-content structural grant's item-4
("nothing user-visible") test; it is governed by
`brief-drafts-verified-against-tree`. No stop was wrongly forced on the
implementer here. This is second-occurrence confirmation of T6's H2 ruling: do
**not** tighten the boundary to route compile-forced repairs of a plan's own
non-compiling literal. If anything T7 is a cleaner exemplar than T6 - the repair
introduces no user-visible behavior at all (the guard skips a call on an
unreachable state), so even the "some wire-representable state changes output"
objection that T6 had to answer does not arise. The boundary is drawn correctly.

**H3 - reinforce `brief-drafts-verified-against-tree` with the T7 occurrence,
and broaden the cross-task note's wording.** Occurrence: plan-7 T7 plan Step-4
literal `if (next.locale !== baseline.locale) applyLocale(next.locale)` fails
`vue-tsc` TS2345 (`AppSettings.locale` is `string | null`, `applyLocale` takes
`string`); adapted with `&& next.locale !== null`, disclosed at site + report,
reviewer-reproduced. Separately: the controller cross-task note (line 144-147)
frames the class as "any `$t`/`$ta` **placeable** fed an Option-derived
(`T | null`) field". T7 shows the class is broader - **any `string`-typed sink**
fed a `T | null` value (here a plain function argument, not a Fluent placeable),
`vue-tsc`-only catch. Candidate: widen that note's wording from "$t/$ta
placeable" to "any string-typed sink" for the remaining T12/T13/T20 dispatches,
so those implementers are not primed to look only at Fluent calls.

**H4 - `e2e/mount-entry.ts` is a legitimate second `createFluentVue` site
(reviewer false-positive guard).** It is test-only component-mount infrastructure
(pre-existing, untouched by T7), not the production app instance; the
single-instance invariant is a `src/` production invariant. Worth one line in
the whole-branch notes so a later reviewer's "no second `createFluentVue`
anywhere" grep does not flag the test harness or its gitignored generated
bundle.
