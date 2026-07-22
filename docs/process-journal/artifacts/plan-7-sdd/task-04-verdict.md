# Task 4 verdict (D53: helpId on EditableField + $ta tooltip bindings + completeness e2e)

**VERDICT: APPROVED**

Commit d194588 on plan7-b (parent 923b049 = approved Task 3). 14-file diff, +80/-3.
Graded against plan Task 4 + Global Constraints, design D53/D54, the Tier-2 ledger
note `gui-helpid-equals-labelkey`, and the real tree. Verification reproduced in the
worktree; one RED fire-verify run and restored byte-identically.

---

## Findings (all clear)

**F1 - `helpId?` scoping is exact (PASS).** `src/editor/fieldSpec.ts` adds
`helpId?: string` to `EditableField` only, with the literal-value comment D53 mandates
("value always === labelKey when present ... never derived"). `FixedField` untouched.
No new `FieldSpec` variant. No registry `helpId:` lines (Task 13 scope) - `registries.ts`
is not in the diff. Matches D53's decision block verbatim in intent and shape.

**F2 - all 10 widgets bind on the correct labelled control (PASS).** Walked each `.vue`:
- Value widgets on their input/select/textarea: BoolWidget `<input checkbox>`,
  OptionalFlagWidget `<input checkbox>`, SelectWidget `<select>`, KeywordOrBlockWidget
  `<select>` (the `<label :for=id>` target), DirectoryPathWidget `<input>`,
  StringListWidget `<input>`.
- TextWidget: BOTH branches bound (`<textarea>` multiline + `<input>` else) - the plan's
  explicit requirement; a single-branch bind would miss Meta.description vs Meta.name.
- The three legend-carrying widgets on `<fieldset>` (the element the legend names):
  ListWidget, PropertyMapWidget, SectionWidget.
All use `:title="$ta(spec.labelKey).tooltip"` verbatim, native `$ta`, no wrapper, no
`tooltipKey`, no per-widget conditional - exactly D53's convention.

**F3 - completeness spec covers all 42 x their widgets, not a sample (PASS).**
`e2e/editor-tooltips.spec.ts` iterates all 13 registries, skips the sole FixedField via
`"fixed" in spec`, and mounts every one of the 42 editable fields through the real
`FieldWidgetDispatcher` routing (`mountWidget`), asserting the rendered DOM `[title]` set
contains the en `.tooltip`. All 10 widget kinds occur in the registries (bool 4,
directoryPath 2, keywordOrBlock 4, list 4, optionalFlag 1, propertyMap 4, section 11,
select 5, stringList 2, text 5 = 42), so every kind is exercised through a real field.
The spec is byte-verbatim to the plan Step 3 block (imports, SAMPLE_VALUE map, REGISTRIES
order, test body identical).

**F4 - `enAttr` and `mountWidget` per the explicit authorization (PASS).** `enAttr` in
`e2e/i18n-en.ts` matches the plan Step 2 code verbatim and reuses the file's existing
module-level `bundle` (house pattern, mirrors the existing `en()` helper). `mountWidget`
in `e2e/mount.ts` is a thin typed wrapper over the existing `mountComponent`; it does not
rebuild the plan-6 harness. Both authorized (Step 2/Step 3), staged in Step 7's explicit
`git add`.

**F5 - nothing extra in the diff (PASS).** 14 files = fieldSpec.ts + 10 widgets +
i18n-en.ts + editor-tooltips.spec.ts (plan Files block) + mount.ts (Step 3/Step 7
authorized). No stray files; ListWidget/PropertyMapWidget/SectionWidget show `-3` only
from the `<fieldset>` -> `<fieldset :title=...>` line rewrites.

**F6 - commit discipline (PASS).** Message matches plan Step 7; trailer
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present; exactly the 14 intended
files staged (no `git add -A` residue).

---

## Verification runs (reproduced in .worktrees/plan7-b)

| Gate | Result |
|---|---|
| `pnpm lint` | clean |
| `pnpm build` (vue-tsc + vite) | clean - typechecks helpId? and mountWidget |
| `pnpm check:i18n` | ok (17 pre-existing gui-*/jobs error-key warnings, no failures) |
| `pnpm test:e2e --grep "renders its label message's .tooltip"` (corrected form) | 1 test, passed - filter confirmed |
| `pnpm test:e2e` (full) | **32 passed** (matches expected) |

**RED fire-verify (falsifiability proven).** Removed BoolWidget's `:title` line, ran the
completeness spec: FAILED on `editor-input-recursive` (a `bool` field), `Received array:
[]` - the check genuinely detects a per-widget missing binding, not a no-op green.
Restored from scratchpad backup via `command cp -f`; `cmp` IDENTICAL, sha256 matches
(2ebf1ad6...), `git status` clean, `git diff HEAD` empty. Probe state fully reverted; the
verdict file is the only write.

---

## Adjudications

**(1) Controller-corrected command form - ACCEPTED.** The filtered form is
`pnpm test:e2e --grep ...` with NO `--` (controller-notes 119-122; pnpm 11 forwards `--`
literally and Playwright drops post-`--` tokens). Empirically confirmed here: the
corrected form ran exactly 1 test; the plan's `-- --grep` form would run the full suite.
Implementer used the corrected form. Correct.

**(2) Additive `check:i18n` gate step - ACCEPTED.** Plan Task 4 Step 6 omits check:i18n;
running it is strictly additive, read-only, artifact-neutral verification that aligns with
the nine-part merge gate (Global Constraint, line 18). Not a fork, not a latitude
decision. Result was clean. Positive.

**(3) mount.ts type-import under the structural grant - ACCEPTED.** `import type
{ EditableField } from "../src/editor/fieldSpec"` is the necessary consequence of the
authorized typed `mountWidget(spec: EditableField, ...)`. Matches the file's own
type-import precedent (`import type { Page } from "@playwright/test"`). Within the Step 3
grant; the harness mechanism is untouched.

**(4) KeywordOrBlockWidget nested-title - BENIGN, no routing.** The outer `<select>` (the
labelled control) carries the tooltip as instructed. KeywordOrBlockWidget also composes a
nested `<SectionWidget>` whose `blockSpec.labelKey === props.spec.labelKey`, and
SectionWidget generically binds `:title` on its `<fieldset>` - so a keywordOrBlock field
renders the SAME tooltip text on TWO elements. This is emergent from pre-existing plan-6
composition, not an extra binding the implementer wrote (KeywordOrBlockWidget's diff adds
only the select's title; the fieldset title is SectionWidget's own mandated binding). Same
text, no contradiction; the completeness spec's `toContain` accommodates it; the full
suite passes on all 4 keywordOrBlock fields. No plan violation, no fork opened. Recorded in
HARVEST so a future reviewer does not misread it as a duplicate defect.

**(5) No `EditableFieldOf` change - CORRECT scoping.** `EditableFieldOf<K>` (shared.ts) is
a separate narrowed type `{ labelKey; widget }` the leaf widgets prop against; none of them
read `helpId` (they render only `spec.labelKey`). `helpId` is consumed by the annotation/
help wiring at the full-`EditableField`/dispatcher level (Task 13). Adding helpId to the
narrowed type would inject an unused field. D53 scopes the change to `EditableField` alone.
Correct as-is. (The EditableFieldOf/EditableField structural near-duplication is a
pre-existing plan-6 DRY tension, out of scope for Task 4 and not introduced by it.)

---

## HARVEST

- **KeywordOrBlockWidget renders the tooltip twice** (outer select + nested SectionWidget
  fieldset, same labelKey). Benign structural fact for the whole-branch review and Tasks
  13+: not a duplicate-binding defect. If ever objectionable, the fix is a distinct block
  labelKey, not removing a mandated binding - defer as UX polish (1.x surface pass), do not
  route now.
- **Over-restriction watch (calibrated correctly).** The completeness assertion is
  `toContain(expected)`, NOT `toEqual([expected])`. This is the right calibration: a widget
  may legitimately render the tooltip on more than one element (keywordOrBlock does).
  A future "tightening" to exactly-one-title would over-restrict and false-fail
  keywordOrBlock's legitimate double render. Keep `toContain`.
- **`enAttr` fixture is now reusable** by Tasks 5/12/13 (planned) - throws loudly on a
  missing message/attribute, so a later misuse surfaces as a test defect, not a silent
  empty string.
- **Corrected e2e filter form** (`pnpm test:e2e --grep`, no `--`) re-confirmed empirically
  this session; already captured in controller-notes 119-122. No new house entry needed.
