# Task 6 report - Plan 9 (D104; amendment 2)

**Status: NEEDS_CONTEXT.** Six of the seven Files-list entries are complete and
verified. The seventh, `e2e/jobsview-reset.spec.ts`, is written in full and
three of its four ruled tests pass; the fourth - D104 item 2, "fresh dispatch
rejection renders the error and clears runActive" - fails on its **second**
assertion, and it fails because D104's chosen observable cannot exist in that
end state, not because the adjudicated behaviour differs. The adjudicated
premise is confirmed: `runActive` IS cleared, and clearing it is exactly what
removes the button the assertion targets. The decision memo is section 4.
Nothing is committed (section 8, with the reason).

---

## 1. Per-file changes against the Files list (EXHAUSTIVE: six modified, one created)

| File | Status | What changed |
|---|---|---|
| `e2e/mount-entry.ts` | done | D104's fence transcribed: glob gains `"../src/views/JobsView.vue"`, `resolvePath` becomes the three-branch `if` form. `spec.props` moved into a `ref`, the render closure spreads `...props.value`, `window.__muxsmithSetProps__(partial)` merges into it (assigned inside `mount()`, mirroring `__muxsmithModel__`'s existing per-mount assignment). One doc comment stating D104's reason. |
| `e2e/global.d.ts` | done | `__muxsmithSetProps__: (partial: Record<string, unknown>) => void;` declared immediately after `__muxsmithMount__`, with its doc. |
| `e2e/jobsview-reset.spec.ts` | created; **1 of 4 tests red** | The four ruled tests, names verbatim from the plan, in one describe. D104's composition assembled spec-locally (`setContent` -> `addScriptTag` tauri-mock-harness -> `evaluate(installMockIPC \| installSoftOutcomeIPC)` -> `addScriptTag` mount-harness -> `__muxsmithMount__`). Item 2's `cancel-batch` assertion is red; see section 4. |
| `e2e/i18n-en.ts` | done | `export function name(...)` added directly after `en()`, carrying smoke's rationale comment verbatim. Nothing else; `FluentVariable` was already imported at `:61`. |
| `e2e/smoke.spec.ts` | done | Local copy + doc comment deleted; `import { en, enAttr }` -> `import { en, enAttr, name }`; `import type { FluentVariable } from "@fluent/bundle";` dropped (0 remaining uses). |
| `e2e/editor-markers.spec.ts` | done | Same deletion; `import { en }` -> `import { en, name }`; `FluentVariable` type import dropped. |
| `e2e/editor-rule-add-remove.spec.ts` | done | Same deletion; `import { en }` -> `import { en, name }`; `FluentVariable` type import dropped. |

`e2e/mount.ts` untouched (`git diff --stat -- e2e/mount.ts` prints nothing). No
Vitest, no `tauri::test`, no `src-tauri/tests/`, no IpcError funnel, no house
YAML edited, no ledger entry written or touched.

```
$ git diff --stat
 e2e/editor-markers.spec.ts         |  9 +--------
 e2e/editor-rule-add-remove.spec.ts |  9 +--------
 e2e/global.d.ts                    |  5 +++++
 e2e/i18n-en.ts                     | 11 +++++++++++
 e2e/mount-entry.ts                 | 26 +++++++++++++++++++++-----
 e2e/smoke.spec.ts                  | 14 +-------------
 6 files changed, 40 insertions(+), 34 deletions(-)
$ git status --porcelain
 M e2e/editor-markers.spec.ts
 M e2e/editor-rule-add-remove.spec.ts
 M e2e/global.d.ts
 M e2e/i18n-en.ts
 M e2e/mount-entry.ts
 M e2e/smoke.spec.ts
?? e2e/jobsview-reset.spec.ts
```

---

## 2. The brief's anchors, re-derived before editing

All reproduced; none failed.

```
$ grep -rn "^function name(" e2e/*.spec.ts
e2e/editor-markers.spec.ts:29:function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } {
e2e/editor-rule-add-remove.spec.ts:41:function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } {
e2e/smoke.spec.ts:60:function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } {
$ grep -c "export function name(" e2e/i18n-en.ts
0
$ ls e2e/*.spec.ts | wc -l
9
$ for f in smoke editor-markers editor-rule-add-remove; do grep -c FluentVariable e2e/$f.spec.ts; done
2
2
2
```

`e2e/i18n-en.ts`: `export function en(` at `:155`, `import type { FluentVariable }`
at `:61` - both read at the line. `data-testid="cancel-batch"` at
`src/views/JobsView.vue:263`, `data-testid="job-panic"` at
`src/components/JobRow.vue:63` - both read at the line.

---

## 3. Evidence

### 3.1 Verification bar (plan Step 4, foreground, no subsets)

```
$ pnpm lint
$ eslint .
(exit 0, no output)
```

```
$ pnpm test:e2e
vite v8.1.4 building client environment for production...
✓ built in 8ms
vite v8.1.4 building client environment for production...
✓ built in 128ms
  ✓  31 [chromium] › e2e/jobsview-reset.spec.ts:138:3 › jobs view: dispatch reset and run-active gating (D23/D104) › fresh dispatch with a soft outcome keeps the finished summary (191ms)
  ✓  35 [chromium] › e2e/jobsview-reset.spec.ts:219:3 › jobs view: dispatch reset and run-active gating (D23/D104) › a finished event with a panic renders the worker-panicked message (160ms)
  ✓  34 [chromium] › e2e/jobsview-reset.spec.ts:179:3 › jobs view: dispatch reset and run-active gating (D23/D104) › double dispatch against an active run does not wipe the live row (178ms)
  ✘  33 [chromium] › e2e/jobsview-reset.spec.ts:167:3 › jobs view: dispatch reset and run-active gating (D23/D104) › fresh dispatch rejection renders the error and clears runActive (5.2s)
  1 failed
  67 passed (6.2s)
```

Both `vite build` steps are in that output, so the harness change under test is
the one in the bundles (`frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`).

**Baseline and delta, both measured on this machine today:** `pnpm test:e2e` at
HEAD before any edit -> `64 passed (2.8s)`. After the change -> `67 passed`
plus the one described failure = 68 total. 68 - 4 (the new spec) = 64
pre-existing, all green: **no pre-existing suite moved**, which is the
no-regression evidence for the harness hook (`e2e/mount.ts` untouched, existing
mount specs never call `__muxsmithSetProps__`) and for the hoist (the three
migrated files behave identically).

Rust control, run because a probe mutation touched a file outside `e2e/`
(section 7, D-6):

```
$ cargo test --workspace 2>&1 | grep -c "^test result:"
39
$ cargo test --workspace 2>&1 | grep "^test result:" | grep -vc "^test result: ok"
0
```

### 3.2 The absence check and its fire (Step 4)

Fire, pasted from the pre-edit run in section 2: the same grep hits exactly the
three copies. Post-hoist:

```
$ grep -rn "^function name(" e2e/*.spec.ts
(no output, exit 1)
$ grep -c "export function name(" e2e/i18n-en.ts
1
```

Green state argued member-by-member as the plan requires: the one surviving
definition reads `export function name(` and lives in `e2e/i18n-en.ts` -
excluded twice over, by the `^function` anchor and by the `*.spec.ts` glob.
Two controls that the zero is a real absence and not a malformed invocation
(the ugrep hazard the brief names):

```
$ grep -rn "^function " e2e/*.spec.ts | wc -l
16
$ grep -rn "^export function name(" e2e/*.ts
e2e/i18n-en.ts:175:export function name(id: string, args?: Record<string, FluentVariable>): { name: string; exact: true } {
```

Same anchor and same glob still produce 16 hits, and the survivor is findable
one glob over. A third control against a differently-shaped local copy:

```
$ grep -rn "const name = \|const name=" e2e/*.spec.ts
(no output, exit 1)
```

`ls e2e/*.spec.ts | wc -l` -> `10` after creation (the new spec is the tenth,
as amendment 2 states). `FluentVariable` count in each of the three migrated
files -> `0`, `0`, `0`.

### 3.3 The pure-move claim

The deleted smoke copy (from the git object, not the working tree) against the
new home:

```
$ git show HEAD:e2e/smoke.spec.ts | sed -n '53,62p' > old-helper.txt
$ sed -n '168,177p' e2e/i18n-en.ts > new-helper.txt
$ diff <(sed 's/^function name(/export function name(/' old-helper.txt) new-helper.txt
(no output)
```

Byte-identical apart from the added `export` keyword, doc comment included.
The comment that travelled is smoke's - the one carrying the "Run" / "Dry run"
/ `run-demo.yaml` substring-collision reason for `exact: true`. The two shorter
mirror comments were deleted with their copies and are recorded here so the
choice is checkable:

```
$ git show HEAD:e2e/editor-markers.spec.ts | sed -n '27,28p'
/** `getByRole(role, name(id))` with exact matching -- mirrors smoke.spec's
 *  own helper (Playwright's default role-name match is a loose substring). */
$ git show HEAD:e2e/editor-rule-add-remove.spec.ts | sed -n '39,40p'
/** `getByRole(role, name(id))` with exact matching -- mirrors the sibling
 *  specs' own helper (Playwright's default role-name match is a substring). */
```

Behavioural half of "pure move": all 64 pre-existing tests, including every
`name()` call site in the three migrated files, pass unchanged in 3.1.

### 3.4 The three green tests, against D104's numbered list

- **Item 1** (`fresh dispatch with a soft outcome keeps the finished summary`):
  spec-local page handler emits a zero-jobs `run-finished` through
  `window.__muxsmithE2E__.emit` and only then returns `StartedRun { total_jobs: 0 }`,
  reproducing the command's emit-before-resolve ordering. Asserts the summary
  line, then re-asserts it after an explicit post-resolve barrier, then
  `cancel-batch` disabled. Green.
- **Item 3** (`double dispatch against an active run does not wipe the live row`):
  `start_run: [resolveWith(StartedRun{total_jobs:2}), rejectWith("run-already-active")]`,
  a `started` event fills row 0, then `__muxsmithSetProps__({ pendingRun: R2 })`.
  Asserts the alert, two rows still present, row 0 still carrying its output,
  and `cancel-batch` **enabled** (`runActive` stayed true). Green - and this is
  the test that exercises the new harness hook end to end.
- **Item 4** (`a finished event with a panic renders the worker-panicked message`):
  a `finished` job-event with `state: "failed"`, `panic: "boom"`; asserts row
  0's `job-panic` testid renders `en("worker-panicked", { detail: "boom" })`.
  Green. This is acceptance observable 4's e2e half.

---

## 4. Decision memo: D104 item 2's second assertion is unsatisfiable (BLOCKING FORK)

### 4.1 The fact, pasted

```
  1) [chromium] › e2e/jobsview-reset.spec.ts:167:3 › ... › fresh dispatch rejection renders the error and clears runActive

    Error: expect(locator).toBeDisabled() failed

    Locator: getByTestId('view-jobs').getByTestId('cancel-batch')
    Expected: disabled
    Timeout: 5000ms
    Error: element(s) not found

    Call log:
      - Expect "toBeDisabled" with timeout 5000ms
      - waiting for getByTestId('view-jobs').getByTestId('cancel-batch')

      175 |     await expect(jobs.getByRole("alert")).toHaveText(en("run-already-active"));
    > 176 |     await expect(jobs.getByTestId("cancel-batch")).toBeDisabled();
```

The FIRST assertion of item 2 - the error alert renders `run-already-active` -
**passes**; the failure is on line 176, the line after it.

### 4.2 Why it is unsatisfiable, and why that does NOT contradict the premise

`cancel-batch` (`JobsView.vue:263`) sits inside
`<template v-if="jobs.length > 0 || runActive || finishedSummary">`
(`:258`). A dispatch is FRESH exactly when `startingFresh = !runActive.value`
is true, and the fresh branch resets `jobs = []`, `logLines = []`,
`finishedSummary = null` before invoking (`:177-182`). On rejection the catch
arm sets `runActive = false` (`:194-196`) and `ensureJobsLength(started.total_jobs)`
is never reached. All three disjuncts are therefore false in **every** end
state of item 2's scenario, so the button is not rendered and "disabled" has no
bearer. This is a property of the scenario, not of my fixture: no
predecessor state survives the unconditional reset into the assertion window.

The adjudicated premise is **confirmed, not contradicted**. The button's
absence is a direct consequence of `runActive` having gone back to false; if it
had stayed true, the button would be present. Measured, by mutating the catch
arm to `if (false && startingFresh)`, rebuilding both harness bundles and
re-running the same test with an absence-shaped assertion in place of the
disabled one:

```
    Error: expect(locator).toHaveCount(expected) failed
    Locator:  getByTestId('view-jobs').getByTestId('cancel-batch')
    Expected: 0
    Received: 1
    ...
        14 × locator resolved to 1 element
           - unexpected value "1"
```

So the flag under test is exactly what decides whether the control exists. The
defect is in the assertion VEHICLE D104 picked, in the same class as amendment
4's impossible pinned invocation: the design's substance is right, its stated
form cannot execute.

D104's own wording carries the tell - "cancel-batch is disabled **again**". At
mount, before any dispatch, the button does not exist either (same `v-if`, all
disjuncts false), so the "again" refers to a state that is also buttonless. The
entry models `cancel-batch` as always rendered and bound to `!runActive`, which
holds for items 1 and 3 and not for item 2.

### 4.3 Why I did not resolve it at the keyboard

Three separate standing statements each forbid it, and they agree:

- The brief: "The three ordering tests codify already-adjudicated behaviour. If
  one fails against the unmodified views ... returns as NEEDS_CONTEXT with the
  failure pasted. It is never 'fixed' at the keyboard - not in the view, not in
  the test."
- Global Constraints / `proc-latitude-clause-boundary`: a fork found on code
  contact returns with a decision memo, never resolved at the keyboard.
- `latitude-carveout-zero-content-structural-forks`: the grant "fills SILENCE
  only - an explicit enumeration in brief, design or spec always wins over it",
  and rewording an assertion is on its stop list. D104 enumerates this
  assertion explicitly.

The escape hatch a reader might expect - Playwright's negated form - was
measured and does not exist:

```
    Error: expect(locator).not.toBeEnabled() failed
    Locator: getByTestId('view-jobs').getByTestId('cancel-batch')
    Expected: not enabled
    Timeout: 5000ms
    Error: element(s) not found
```

`not.toBeEnabled()` also requires the element to be attached, so it does not
pass on a detached one. There is no wording-preserving fix.

### 4.4 Options, with costs against the named invariants

**A. Replace item 2's second assertion with the absence-shaped equivalent,
keyed on the same button, paired with the placeholder.** Concretely:

```ts
await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
```

Measured green (pasted below), and fire-verified red under the 4.2 mutation.
`jobs-empty` (`JobsView.vue:326-331`) is the `v-else` of the very same
condition, so with `jobs` empty and `finishedSummary` null its visibility is
logically equivalent to `runActive === false` - the exact proposition D104
wants asserted. The pairing matters: `toHaveCount(0)` alone would also pass if
the view had failed to mount at all, and `jobs-empty` visible is the positive
bearer that rules that out (the alert assertion above it independently proves
the view rendered).

```
$ pnpm exec playwright test e2e/jobsview-reset.spec.ts -g "fresh dispatch rejection"
  ✓  1 [chromium] › ... › fresh dispatch rejection renders the error and clears runActive (130ms)
  1 passed (509ms)
```

Cost: D104's numbered item 2 needs a rider recording that the vehicle is the
control's ABSENCE, not its disabled state, and why; the plan's Step-2 sentence
"the internal transitions are asserted through the cancel-batch button's
disabled state, as D104 fixes" needs the same qualifier for item 2 (it stays
literally true for items 1 and 3). Behaviour asserted is unchanged. No code
change, no new infrastructure, `runActive`-not-a-prop rationale untouched.

**B. Change `JobsView` so the batch header renders while `startError` is set,
making the button exist.** Rejected on its face: D104 opens with "**No code
fix**", the brief says no code fix is in scope, and it would change
user-visible behaviour (an empty jobs table and a cancel button appearing after
a failed start).

**C. Drop item 2's second assertion, keep only the alert.** Cost: the test then
asserts nothing about `runActive`, which is the entire reason the ordering is in
the list; it collapses into a plain error-render check that the existing suite
already shows elsewhere. This is the "record the gap honestly" shape
`tests-ship-with-the-feature-never-after` names as not a resolution.

**D. Re-shape item 2's scenario so the block is already rendered when the fresh
rejection lands.** Impossible, per 4.2: the fresh branch's reset runs
unconditionally before the invoke, so no predecessor state survives.

### 4.5 Recommendation

**Option A**, routed as a D104 rider (design-side, mirroring amendment 4's
shape: the substance of the ruled assertion is unchanged, only its stated form
was impossible). If the owner prefers the plan to carry it instead, the rider
text is two assertions and one sentence of reason, and the fix round is a
four-line edit to a file that is already written.

---

## 5. Test-coverage precedence check (four conditions), stated even though it did not fire

The task's own diff touches `e2e/` only - test code and the test harness - plus
nothing under `src/`, `crates/`, `src-tauri/` or `locales/` (`git diff --stat`
in section 1; `src/views/JobsView.vue` was probe-mutated and restored, see D-6).
Condition 3 ("the consequence comes from THIS diff") therefore cannot hold for
any candidate consequence: this diff creates no user-visible behaviour at all.
Conditions 1, 2 and 4 are moot. **The check does not fire; no producer was
built under it.** The one behaviour this diff does add - the
`__muxsmithSetProps__` hook - is test infrastructure, not user-visible, and it
is exercised by item 3.

---

## 6. Surfaced for the controller (no task edits house YAMLs; I edited none)

1. **Ledger-worthy, design-authoring handle.** "Assert the control's disabled
   state" presumes the control is rendered. When the flag under test is ALSO a
   disjunct of the control's own render condition, the disabled-state vehicle
   collapses in exactly the branch where the flag is false - the branch usually
   worth asserting. Trigger is readable: you are writing "assert X is
   disabled/enabled" about a control whose enclosing `v-if` mentions the same
   state. Handle: read the control's render condition before choosing the
   vehicle; if the flag appears there, the observable is presence/absence.
   Same family as amendment 4's impossible invocation, one artifact class over
   (design assertion rather than plan invocation).
2. **Empirical Playwright fact, measured here** (4.3): `expect(locator).not.toBeEnabled()`
   does NOT pass on a detached element - it reports "element(s) not found" and
   times out, same as the positive form. Anyone reaching for the negated form
   as a missing-element escape hatch is reaching for nothing.
3. **D104's minimality claim held.** The reactive-props hook required no
   `mount.ts` change and moved no pre-existing test: 64 pre-existing tests green
   in the same run that exercises the hook.
4. **Item 3 is the only consumer of `__muxsmithSetProps__` in the repo.** If the
   fork's resolution ever removed item 3, the hook would be dead code.
5. The ROADMAP `name()` trigger's NOT FIRED / consumed-early record stands as
   written: the new spec consumes no `name()` (it asserts through
   `data-testid` and `getByRole("alert")` only), so no fourth consumer arose.
   Verified: `grep -n 'from "./i18n-en"' e2e/jobsview-reset.spec.ts` ->
   `47:import { en } from "./i18n-en";` - only `en`, no `name`.

---

## 7. Divergences and judgment calls, each named

**D-1: test 1 carries an explicit post-resolve barrier.** D104 says "assert the
finished summary is displayed AFTER the promise resolves". A retrying assertion
alone cannot distinguish "displayed and kept" from "displayed in flight", so
after the first assertion the test does one round trip through the page
(`await page.evaluate(() => document.readyState)`) and re-asserts. The
dispatch's own post-await continuation and Vue's flush are both microtasks
queued while the mocked `invoke` returned, so the round trip is strictly later
than both. Mechanics, not fenced by D104; a reviewer may rule it unnecessary.

**D-2: the panic fixture's non-fenced fields are copied from the real emitter.**
D104 fixes `state: "failed"` and `panic: "boom"`. The rest -
`exit_code: null`, `errors: ["worker-panicked: job 0"]`, `duration_ms: 0` -
is `recover_panicked_worker`'s actual construction
(`crates/muxsmith-core/src/executor/queue.rs:457-466`), read at the source
rather than invented.

**D-3: test 1 asserts the joblog note in addition to the summary line.** D104
fixes `joblog_status: "unavailable"` in the fixture but enumerates no assertion
on the note it produces. Both strings render in the same `jobs-run-summary`
paragraph, so asserting only the summary line would leave a fixture-pinned
consequence unasserted. Additive, on existing infrastructure, no existing
assertion touched. A reviewer may rule it beyond the enumeration.

**D-4: names D104 does not fence.** The describe title
(`jobs view: dispatch reset and run-active gating (D23/D104)`), the spec-local
helper names (`preparePage`, `mountJobsView`, `installSoftOutcomeIPC`), the
fixture constants and their values (`RUN_ID`, profile paths, `JOB0_OUTPUT`).
The four TEST names are the plan's, character for character.

**D-5: the props ref lives inside `mount()`, not at module scope.** So
`__muxsmithSetProps__` is re-bound per mount and its state resets with the
component, mirroring `__muxsmithModel__`'s existing per-mount assignment two
lines above. D104 fixes the hook's name and merge semantics, not its lifetime.

**D-6: I probe-mutated a file outside the Files list and restored it.**
`src/views/JobsView.vue`, catch arm -> `if (false && startingFresh)`, to fire
the assertion option A recommends (4.2). Baseline taken BEFORE the mutation;
restored with `git checkout --` and proven:

```
$ sha256sum src/views/JobsView.vue      # before
ca18ea6349b3420789ae22d829b5f992d27ec360669f6db2a4b0542faf0d30cb  src/views/JobsView.vue
$ git checkout -- src/views/JobsView.vue && sha256sum -c jobsview.sha256
src/views/JobsView.vue: OK
$ git status --porcelain                # after restore - the path is absent
 M e2e/editor-markers.spec.ts
 M e2e/editor-rule-add-remove.spec.ts
 M e2e/global.d.ts
 M e2e/i18n-en.ts
 M e2e/mount-entry.ts
 M e2e/smoke.spec.ts
?? e2e/jobsview-reset.spec.ts
```

Both harness bundles were rebuilt after the restore, and the final
`pnpm test:e2e` in 3.1 rebuilt them again. The `cargo test --workspace` control
in 3.1 was run because of this mutation.

**D-7: no commit.** Section 8.

---

## 8. Commit: none

`a-returning-task-may-commit-the-subset-that-survives-every-option` (ledger,
Tier 1) permits committing a finished subset when three conditions hold. Here:

- (a) complete against its own Files-list entries: the six non-spec entries
  pass, `e2e/jobsview-reset.spec.ts` does not.
- (b) green on the task's full verification bar: **fails.** `pnpm test:e2e`
  is red on the tree as it stands, and it stays red after any partial commit,
  because the failing artifact is the untracked spec file. Measuring the
  six-file subset green would require physically removing the task's central
  artifact from the tree first, which is not the state I would be leaving.
- (c) survives every option: the six non-spec files do (options A-D touch only
  two assertion lines inside the new spec).

Failing (b), the entry's own instruction is that the work stays uncommitted.
A second reason, independent of (b): the plan's Task-6 commit message is a
fenced string that names "the ruled D23 reset tests + panic render", and the
only committable subset is precisely the one WITHOUT them - so committing it
under that message would make the message false, and editing the message is
itself an unrouted deviation from a fenced string.

The finished work is intact in the working tree and fully visible above. If the
controller judges the routing interval long enough that the single-index risk
the ledger entry names outweighs this, say so and I will stage the six by path
in one call - the content is final under every option.

`git log -1 --oneline` is unchanged: `dc7c605 house+roadmap: mine the Task-5
delta review; two text corrections routed to the close`. No `git show --stat`,
because no commit was made.

---

## 9. Numbered concerns a reviewer can rule on yes/no

1. **Is the fork correctly ROUTED rather than resolved?** My read: yes - three
   standing statements forbid the keyboard fix and none of them has an
   exception that fits. The counter-read a reviewer might hold: the design pins
   these tests by scenario, not by patch text, so "assert through the button's
   state" could be read as satisfied by asserting the button's absence, making
   this an implementation detail rather than a fork. I judged that a rewording
   of an enumerated assertion, which is on the carve-out's stop list.
2. **Is option A the right resolution, and is the `jobs-empty` pairing the
   right sharpening?** (Without it, `toHaveCount(0)` passes vacuously on a view
   that never mounted.)
3. **Is D-1's post-resolve barrier warranted, or noise a reviewer would cut?**
4. **Is D-3 (`jobs-joblog-unavailable` asserted alongside the summary line)
   inside the enumeration or beyond it?**
5. **Is leaving all seven uncommitted right (section 8), or should the six
   complete files have been committed?** This is the same shape Task 4's
   concern 4 raised and the ledger entry answered; the new wrinkle is that here
   condition (b) cannot be met on the tree as left, and the fenced commit
   message names exactly the omitted artifact.
6. **Is D-2's fidelity choice right** (the panic fixture's non-fenced fields
   copied from `recover_panicked_worker`), or should unfenced fields be
   minimal placeholders?

---

## Fix round (after the amendment-5 ruling)

**Status: DONE.** The fork returned in section 4 was ruled by the controller as
option A and written into the design as D104's amendment-5 rider (commit
`1e0dbd8`, with the plan's Step-2 sentence gaining its pointer qualifier in the
same commit). Two edits to `e2e/jobsview-reset.spec.ts`, nothing else in that
file and no other file. All four ruled tests are green; the task is committed.

Sections 1-9 above are the pre-ruling record and stand as written; section 8's
"no commit" reasoning was superseded by the ruling, which made the plan's fenced
message accurate. Concerns 1, 2 and 5 of section 9 are answered by the ruling
itself; 3, 4 and 6 remain live for the reviewer.

### F.1 What changed, both edits

```
$ diff -u <pre-fix spec> e2e/jobsview-reset.spec.ts
@@ -36,8 +36,11 @@
  * `runActive` is deliberately not passed as a prop: an absent prop makes
  * `defineModel` fall back to local-ref semantics, which is the view's real
  * standalone behavior, and it keeps the internal transitions assertable
- * through the cancel-batch button's own disabled state instead of through
- * a value the test itself supplied.
+ * through the cancel-batch button's own disabled state -- for tests 1 and
+ * 3 -- instead of through a value the test itself supplied. Test 2's
+ * transition is asserted at the same gating condition from its other side;
+ * D104's amendment-5 rider is the single home for that vehicle and its
+ * reason.
  */
@@ -173,7 +176,8 @@
     const jobs = page.getByTestId("view-jobs");
     await expect(jobs.getByRole("alert")).toHaveText(en("run-already-active"));
-    await expect(jobs.getByTestId("cancel-batch")).toBeDisabled();
+    await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
+    await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
   });
```

Two hunks, exactly the two instructed sites. The header comment POINTS at the
rider rather than restating the replacement, so the rider stays the single home
for that vehicle and its reason.

**The replacement is the rider's fence verbatim.** Checked mechanically against
the design rather than by eye - the rider's fenced block (design `:1310-1311`)
against the committed lines (spec `:179-180`, de-indented by the file's
four-space test-body indent):

```
$ diff <rider fence> <committed lines, de-indented>
(no output)
VERBATIM MATCH (modulo the file's 4-space test-body indent)
```

**Items 1 and 3 keep the disabled-state vehicle**, as the rider's "Items 1 and 3
are untouched" paragraph and the dispatch both say; the surviving occurrences
map one-to-one to their tests:

```
$ grep -n 'test("\|toBeDisabled\|toBeEnabled\|toHaveCount(0)\|jobs-empty' e2e/jobsview-reset.spec.ts
141:  test("fresh dispatch with a soft outcome keeps the finished summary", async ({ page }) => {
167:    await expect(jobs.getByTestId("cancel-batch")).toBeDisabled();
170:  test("fresh dispatch rejection renders the error and clears runActive", async ({ page }) => {
179:    await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
180:    await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
183:  test("double dispatch against an active run does not wipe the live row", async ({ page }) => {
220:    await expect(jobs.getByTestId("cancel-batch")).toBeEnabled();
223:  test("a finished event with a panic renders the worker-panicked message", async ({ page }) => {
```

One `toBeDisabled` (test 1), one `toBeEnabled` (test 3), the replacement pair in
test 2, nothing in test 4.

### F.2 Verification bar (foreground, no subsets)

```
$ pnpm lint
$ eslint .
(exit 0, no output)
```

```
$ pnpm test:e2e 2>&1 | grep -E "jobsview-reset|passed|failed|built in|vite v"
vite v8.1.4 building client environment for production...
✓ built in 9ms
vite v8.1.4 building client environment for production...
✓ built in 126ms
  ✓  32 [chromium] › e2e/jobsview-reset.spec.ts:141:3 › jobs view: dispatch reset and run-active gating (D23/D104) › fresh dispatch with a soft outcome keeps the finished summary (205ms)
  ✓  33 [chromium] › e2e/jobsview-reset.spec.ts:170:3 › jobs view: dispatch reset and run-active gating (D23/D104) › fresh dispatch rejection renders the error and clears runActive (169ms)
  ✓  35 [chromium] › e2e/jobsview-reset.spec.ts:223:3 › jobs view: dispatch reset and run-active gating (D23/D104) › a finished event with a panic renders the worker-panicked message (159ms)
  ✓  34 [chromium] › e2e/jobsview-reset.spec.ts:183:3 › jobs view: dispatch reset and run-active gating (D23/D104) › double dispatch against an active run does not wipe the live row (180ms)
  68 passed (3.0s)
```

`68 passed`, zero failed, as predicted. Both `vite build` steps are in that
output, so the run measures the rebuilt bundles and not a stale
`e2e/.generated/`. All four new tests green; 68 - 4 = 64 pre-existing, matching
the pre-task baseline of `64 passed` exactly, so nothing moved. The bar ran on
content byte-identical to what was then committed (`git status --porcelain`
empty afterwards, F.3).

The absence check, the pure-move evidence and the Rust control (39 `test result:`
lines, all ok) are unchanged by this round - it touched one spec file - and stand
as pasted in sections 3.1-3.3.

### F.3 Commit

`a2c1028` on `master`, main worktree. Staged by name, seven paths, no
`git add -A`; unsigned (`git log -1 --format='%G?'` -> `N`); exactly one
trailer, no `Claude-Session` line; the plan's Step-5 message character for
character. Not pushed.

```
$ git show --stat HEAD
commit a2c1028d4ad98496c8bcfe57f7c0bf36f63b2659
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 23:49:48 2026 +0200

    e2e: JobsView mountable, reactive-props hook, the ruled D23 reset tests + panic render; hoist the shared name() helper (D104, amendment 2)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 e2e/editor-markers.spec.ts         |   9 +-
 e2e/editor-rule-add-remove.spec.ts |   9 +-
 e2e/global.d.ts                    |   5 +
 e2e/i18n-en.ts                     |  11 ++
 e2e/jobsview-reset.spec.ts         | 258 +++++++++++++++++++++++++++++++++++++
 e2e/mount-entry.ts                 |  26 +++-
 e2e/smoke.spec.ts                  |  14 +-
 7 files changed, 298 insertions(+), 34 deletions(-)
$ git status --porcelain
(empty)
```

Seven paths, matching the Files list exactly: six modified, one created.

### F.4 Surfaced from this round

1. **The rider's second half found a consumer my report did not.** My section 4
   named the two document sentences the ruling would falsify (D104's item-2 line
   and its `runActive`-not-a-prop paragraph) and missed the third instance of the
   same sentence in my own file's header - the reference sweep stopped at the
   documents and did not come back to the artifact. The amendment review caught
   it. Worth carrying as a handle: when a ruling falsifies a sentence that exists
   in a document AND in the code the document describes, the sweep is not done at
   the document boundary.
2. Sections 6.1-6.4's surfaced items are unaffected by this round and still
   stand; 6.5 (the `name()` trigger record) likewise - the fix round added no
   import to the new spec, which still imports only `en` from `./i18n-en`.
3. Section 9's concerns 3, 4 and 6 (D-1's post-resolve barrier, D-3's joblog-note
   assertion, D-2's panic-fixture fidelity) are untouched by the ruling and are
   the live ones for the reviewer.
