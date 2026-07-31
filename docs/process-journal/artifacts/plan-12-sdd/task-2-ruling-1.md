# Controller ruling 1 on Task 2 (Plan 12)

**Order of authority.** This file sits directly below the plan and above the
task brief for the two questions it settles, and touches nothing else. Where it
is silent, the brief and the plan govern unchanged. It answers a NEEDS_CONTEXT
the implementer returned after completing Steps 1 to 4; both forks were
discovered on code contact, both were measured, and neither was resolved at the
keyboard - which is the behaviour the latitude ban asks for.

**Routing.** Both forks are internal test mechanics: nothing user-visible, no
wire format, no product boundary. By the doctrine's routing matrix they are the
controller's to decide and record, not the owner's. No plan amendment is owed:
no task is added, removed or re-cut, and every assertion the plan prescribes
survives intact. What changes is how two of them are reached.

## The controller's own verification, before deciding

Both mechanism claims in the implementer's memo were re-measured at the source
rather than taken on trust:

- `SettingsDialog.vue`'s `open()` does reassign the baseline: `baseline = await
  getSettings();`.
- `e2e/mocks.ts` documents its own queue behaviour: "once exhausted, the last
  entry repeats for any further call". A single-entry `get_settings` queue
  therefore returns `locale: null` forever, which is precisely the mechanism the
  memo describes.
- The interaction ids at issue differ across locales (`settings-save` = `Save` /
  `Speichern`), while the two language-option labels are identical in both
  catalogs (`English`, `Deutsch`) - the plan's own identical-value measurement
  reproduces.

## Ruling 1: interaction locators follow the locale that is RENDERING

The brief's Step 5 says every interaction string comes from `en()`. That
convention encodes **"resolve through the catalog, never hardcode a literal"**;
its premise is that the app renders English, and this describe deliberately
removes that premise. Read as "English specifically", it makes its own cases
unexecutable - a locator for `Save` cannot find a button labelled `Speichern`.

**Decided: an interaction locator resolves through the catalog of the locale
that is actually rendering at that point in the case.** `de("settings-save")`
while the interface is German, `en("settings-save")` once it has switched to
English. Both helpers already exist in this file and are already used side by
side.

Two qualifiers, so this is not read wider than it is:

- **The assertion rule is untouched and still binds.** Every ASSERTED German
  string must be one whose German value differs from its English value, because
  `buildBundles` negotiates `[requested, en]` per message and an assertion on an
  identical-valued id passes even when the interface fell back to English
  entirely. This ruling governs locators, not assertions.
- **An interaction locator is a witness only where the two values differ**, and
  it is never counted as one of the case's assertions. Where they are identical
  the locator still finds the control, which is all it is there to do.

## Ruling 2: case 3 splits into its two directions

The prescribed case 3 cannot exercise its second leg. After the first save,
`open()` re-reads the baseline from the mock, whose exhausted queue repeats
`locale: null`; at the second save `next.locale` and `baseline.locale` are both
`null`, the live-switch guard is false, and `applyLocale` never fires. The
persisted half is nonetheless exactly right, which is what makes this dangerous:
**as written, the case would have shipped green while measuring nothing on the
live path** - the same shape as an assertion made below a fallback, which this
plan's own Global Constraints warn about three times.

This is not a product defect. The real backend persists, so a second `open()`
would read `"en"` and the switch would fire. The defect is in the fixture.

> **CORRECTION, 2026-07-30, controller.** The bolded sentence above is wrong in
> its consequence and is left standing rather than edited away, because the
> wrong version is what a later reader would otherwise reconstruct. The
> mechanism it describes is right - the guard sees `null` on both sides and
> `applyLocale` never fires - but the result is an **always-RED case, not a
> false-green one**. The Task-2 reviewer rebuilt the prescribed case 3 against
> the committed implementation and measured it: leg 2's persisted assertion
> passes, while "the de heading back" and `documentElement.lang === "de"` are
> false against correct code. The implementer's own mid-flight gate run says the
> same thing and I read past it - 69 passed, 2 failed, the two new prescribed
> cases among the failures.
>
> **So this was never an instance of the assertion-below-a-fallback class**, and
> invoking that class here was mine, not the implementer's. The real shape is
> narrower and still worth the split: a fixture that cannot reach the state the
> assertion needs, so the case blocks on the fixture rather than on the product.
> The ruling does not change - the split is correct either way, and the
> corrected reason is the stronger argument for it, because an always-red test
> that cannot be made green without either a reload or new test infrastructure
> is a case that must be re-cut rather than retried.

**Decided: split case 3 into two cases, one per direction.**

- **Direction 1, system to English**, from the existing `locale: null` scenario:
  every assertion the prescribed first leg names - the recorded
  `set_settings.locale === "en"`, the German heading gone, the English heading
  present, `documentElement.lang === "en"`.
- **Direction 2, English to system**, from its own scenario with a stored
  `locale: "en"`: the recorded `set_settings.locale === null`, the German
  heading back, `documentElement.lang === "de"`.

Ground: it preserves every prescribed assertion, keeps both directions on the
LIVE path with no reload anywhere, and stays inside every fence the task
carries - `playwright.config.ts` untouched, `e2e/mocks.ts` untouched, no counted
queue, no new test infrastructure. It also covers strictly more than the
prescribed form, which never reached the seam with a stored override at all.

**Rejected: making the mock stateful for settings.** It is the more general
answer and it is new test infrastructure, which is the one exemption
`tests-ship-with-the-feature-never-after` names - so it would be a scope
addition decided inside a task rather than routed. Recorded instead as a
close-action tracker candidate: **when a test needs save-then-reopen without a
reload, build a stateful settings store into `e2e/mocks.ts`.** That is an
observable event, so it is a trigger and not a wish.

## Consequence for the plan's acceptance map, stated so the coverage walk holds

The acceptance map names W1-f and W1-g's producer as "same test, second leg".
After this ruling their producer is **the second case of the split**. The plan
document is not edited; this file is the overlay, and the reviewer's coverage
walk reads both. Every other row is unaffected.

## Also surfaced by the implementer, and neither is a fix in this task

- The soundness control's end-state figure of 2 silently forbids the new doc
  comment from containing the token `navigator.language` - a constraint derivable
  only from the control's own enumeration, never stated. Complied with. Harvest
  item for the ledger, not a change.
- `e2e/mocks.ts` has no stateful settings store; later packages asserting
  save-then-reopen without a reload will meet this again. Carried as the close
  action named above.
