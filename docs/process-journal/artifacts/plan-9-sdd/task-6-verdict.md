# Task 6 review verdict - Plan 9 (D104; amendment 2; amendment 5)

**Verdict: APPROVED_WITH_MINORS.**

Graded against the current tree at `a2c1028` (HEAD, `master`, main worktree),
not against a hash quoted in a report. Every fenced contract reproduces
character for character under my own comparison; the hoist is a genuine byte
pure move; the absence check is real and I re-derived it with an instrument
that is not `grep`; the full bar is green under my own run (`pnpm lint` exit 0,
`pnpm test:e2e` 68 passed, `cargo test --workspace` 39 `test result:` lines all
ok). The four ruled tests are behavioural, not shape guards: I fired each one
with its own targeted source mutation and in every case the mutation reddened
exactly that test and nothing else in the 68-test suite - these four are the
sole coverage of the four behaviours, which is the strongest argument for the
D23 item existing at all.

One LOW finding and four INFO observations. No fix round is required to close
the task; LOW-1 may be discharged at the plan close.

---

## 1. Findings

### LOW-1: the spec-local IPC installer silently diverges from `installMockIPC`, and the report's divergence list does not name it

**Where:** `e2e/jobsview-reset.spec.ts:99-120` (`installSoftOutcomeIPC`),
against `e2e/mocks.ts:84-133` (`installMockIPC`).

**Evidence I ran.** `installMockIPC` performs three things the spec-local
handler does not: it sets `window.__TAURI_OS_PLUGIN_INTERNALS__ =
{ platform: scenario.platform ?? "linux" }` (`e2e/mocks.ts:99`), it forwards
every call to `window.__muxsmithRecordInvoke__?.()` (`:103`), and it answers
three further incidental commands - `get_settings`, `set_settings`,
`plugin:fs|write_text_file` (`:113-126`). `installSoftOutcomeIPC` answers
`start_run` and `list_runs` and throws on everything else
(`e2e/jobsview-reset.spec.ts:116`). The omission is currently harmless and I
measured why: the only `platform()` consumer in the frontend is
`src/views/FirstRun.vue:35`, which is not in `JobsView`'s subtree (its child
imports are `JobRow`, `LiveLog`, `RunHistory`, `src/views/JobsView.vue:26-28`).

The report's D-4 discloses the spec-local helper *names* but not this
behavioural delta, so a reader of the report cannot see that test 1 runs on a
narrower mock surface than tests 2-4. When it eventually bites - a new
`onMounted` read anywhere under `JobsView`, or a `platform()` call reaching
that subtree - test 1 fails with `jobsview-reset mock: unmocked command "X"`
while tests 2-4 stay green, and the asymmetry is what makes that confusing.

**Exact required change (either discharges it).** Extend the doc comment above
`installSoftOutcomeIPC` at `e2e/jobsview-reset.spec.ts:88-98` with one sentence
naming what it deliberately omits relative to `installMockIPC`
(`__TAURI_OS_PLUGIN_INTERNALS__`, the invoke recorder, and the `get_settings`
family) and why the omission is safe today; **or** answer the same incidental
set the shared installer answers. The comment is the cheaper and, given
D104's "spec-local mock composition" fence, the more honest of the two.
Acceptable to route to the plan close rather than a fix round: it changes no
observable and the current state is green and measured.

### INFO-1: "the same gating condition" is true only under one of its two readings

**Where:** `e2e/jobsview-reset.spec.ts:40-41` - "Test 2's transition is
asserted at the same gating condition from its other side".

Tests 1 and 3 assert through `:disabled="!runActive"`
(`src/views/JobsView.vue:264`); test 2 asserts through the `v-if` disjunction
`jobs.length > 0 || runActive || finishedSummary` (`:258`) and its `v-else`
(`:327`). Those are two different expressions sharing one flag. The sentence
is exactly true if "gating condition" means the `v-if` fork (items 1 and 3
assert with it true, item 2 with it false - literally its other side) and loose
if it means the disabled binding. The comment is not the implementer's
invention: it echoes the rider's own wording (design `:1336-1337`), and it
points at the rider rather than restating the replacement, which is what
amendment 5's MEDIUM-1 required. No change; carry the sharper phrasing if the
rider is ever touched, alongside the amendment-5 verdict's INFO-1.

### INFO-2: a third declaration site for the `.generated/` harness paths

`e2e/jobsview-reset.spec.ts:54-55` re-declares both bundle paths already held
privately in `e2e/mocks.ts:24` (`HARNESS_PATH`) and `e2e/mount.ts:14`
(`MOUNT_HARNESS_PATH`). Neither is exported, and both owning files are off
Task 6's Files list, so the copy was forced and the implementer was right not
to touch them. This is the same three-copy shape the `name()` helper just left.
Trigger candidate in the HARVEST.

### INFO-3 (controller bookkeeping): `progress.md` still shows Task 6 mid-flight

`.superpowers/sdd/plan-9/progress.md:14` reads `FIX ROUND (returned
NEEDS_CONTEXT, fork routed via amendment 5) | uncommitted in the tree`. The
task is committed at `a2c1028` with a clean tree. Successor state of the
amendment-5 verdict's INFO-3; for the close, not for the implementer.

### INFO-4 (this brief, `proc-57-briefs-not-ground-truth`): two commits landed mid-flight, not three

My review brief says "Three commits landed before it while the task was
mid-flight - the amendment (`1e0dbd8`) and two house commits". Measured:
`dc7c605` (2026-07-28T22:51:00) is the commit the report records as HEAD
*unchanged* at its NEEDS_CONTEXT return (`task-6-report.md:498-499`), so it
predates the task's start. The commits that landed in the routing interval are
`1e0dbd8` (23:25:56) and `18ef645` (23:47:48) - two, over a 56-minute interval
ending at `a2c1028` (23:49:48). Immaterial to the grade; recorded because it
feeds adjudication 5's risk weighing.

---

## 2. What I verified and found sound, by dimension

### D1. Contract compliance where D104 fences text - PASS, every fence diffed mechanically with its own fire

| Fence | Source | Tree | Result |
|---|---|---|---|
| `mount-entry.ts` glob + `resolvePath` | design `:1181-1194` | `e2e/mount-entry.ts:26-39` | `diff` -> no output; fire (one letter changed in the tree copy) -> 4 diff lines |
| The four test names, in order | plan `:388-391` | `e2e/jobsview-reset.spec.ts:141,170,183,223` | `diff` of extracted name lists -> no output; fire (`runActive` -> `runactive`) -> 1 diff line |
| Item 2's replacement pair | rider `:1310-1311` | spec `:179-180`, de-indented by 4 | `diff` -> no output; fire (`toHaveCount(0)` -> `(1)`) -> 1 diff line |
| Commit message | plan `:403` | `git log -1 --format=%s` | `diff` -> no output; fire -> differs |

The hook: name `__muxsmithSetProps__`, merge semantics
`props.value = { ...props.value, ...partial }` (`e2e/mount-entry.ts:71-73`),
`spec.props` moved into a `ref` (`:70`), render closure spreading
`...props.value` (`:78`), typed immediately after `__muxsmithMount__` in
`e2e/global.d.ts:43-47`. All four as D104 `:1197-1203` writes them.
`e2e/mount.ts` untouched (`git show --stat a2c1028` lists seven paths, all
under `e2e/`, and it is not among them); no `docs/` file touched, so
`gui-d23-reset-gating-form` (`docs/decision-ledger.yaml:4539`) was neither
rewritten nor duplicated.

### D2. The four tests against D104 items 1-4 - PASS, and each is a discriminator, measured

I built four targeted source mutations, rebuilt both harness bundles through
`pnpm test:e2e` between edit and run in every case
(`frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`), and recorded
which tests reddened.

| Mutation | Source edit | Red | Rest of the suite |
|---|---|---|---|
| M-a | catch arm `if (startingFresh)` -> `if (false)` (`JobsView.vue:194`) | test 2 only, at `:179`, `Expected: 0 / Received: 1` | 67 passed |
| M-b | `const startingFresh = !runActive.value` -> `= true` (`:176`) | test 3 only | 67 passed |
| M-c | the fresh reset moved from before the invoke to after the `await` (the literal "reset after resolve Ok" reading) | test 1 only, at `:153`, `element(s) not found` | 67 passed |
| M-d | `JobRow.vue:46` panic pass-through -> `null` | test 4 only | 67 passed |

Answering the dimension's question per item: the reachable failing code path is
the pre-D23-fix implementation for tests 2 and 3, the literal reading the
plan-5 round-2 verdict rejected for test 1, and an unwired D100 for test 4.
Those are real implementations of the same feature, one of which shipped and
was corrected - a regression discriminator, not a shape guard under
`a-fired-check-still-needs-a-reachable-failing-input`. Each mutation is caught
by exactly one test and by nothing else in 68, so removing any one of the four
would leave its behaviour silently uncovered.

M-c also settles a premise nobody had measured: under a post-resolve reset the
summary is **absent and stays absent through a 5s retry**, which is only
possible if `onRunFinished` had already written it before the post-await
continuation ran. The spec's `installSoftOutcomeIPC` therefore genuinely
reproduces the Rust command's emit-before-resolve ordering, rather than merely
claiming to.

### D3. The hoist is a pure move - PASS, both halves

Helper half, against the git object rather than the working tree:
`git show HEAD~1:e2e/smoke.spec.ts | sed -n '53,62p'` versus
`sed -n '168,177p' e2e/i18n-en.ts`. After normalizing the single added `export`
keyword the two files have the **same md5** (`3c1d0af881a42db021ba9678d59e5c4b`)
and `diff` prints nothing; the un-normalized `diff` prints exactly one hunk,
the `export` line. Fire: one letter changed in the doc comment (`SUBSTRING` ->
`SUBSTRINg`) produces a diff line, so the comparison is live.

The comment that travelled is **smoke's**, carrying the substring-collision
reason (`"Run"` inside `"Dry run"` and inside `run-demo.yaml`) that is the
whole reason `exact: true` exists. It is not either mirror: the two deleted
mirrors read "mirrors smoke.spec's own helper" and "mirrors the sibling specs'
own helper" (visible in `git show a2c1028 -- e2e/editor-markers.spec.ts
e2e/editor-rule-add-remove.spec.ts`), and neither string occurs anywhere in
`e2e/i18n-en.ts`. The one cost the plan forbade paying was not paid.

Behavioural half, measured rather than asserted: the three migrated diffs are
pure deletions plus the two enumerated import edits - nothing else. Call sites
are unchanged, decomposed with my own instrument rather than inferred from a
raw hit count:

| File | HEAD~1 total / definition / doc / **call sites** | HEAD | Call sites unchanged |
|---|---|---|---|
| `smoke.spec.ts` | 71 / 1 / 1 / **69** | 69 / 0 / 0 / **69** | yes |
| `editor-markers.spec.ts` | 8 / 1 / 1 / **6** | 6 / 0 / 0 / **6** | yes |
| `editor-rule-add-remove.spec.ts` | 6 / 1 / 1 / **4** | 4 / 0 / 0 / **4** | yes |

79 live `name()` call sites across three files, all green in my own run. The
`FluentVariable` drop is correct in all three: 2 occurrences each before, 0
after, and `e2e/i18n-en.ts:61` still imports the type for the two signatures
that need it (`:155`, `:175`).

### D4. The absence check and its green state - PASS, re-derived with a non-`grep` instrument

I did not re-run anyone's grep. `absence-probe.sh` walks `e2e/*.spec.ts` with
`awk` and reports every column-0 definition of a symbol named `name`, in the
`function`, `export function` and `const|let|var` forms. Current tree: no
output. Fire, same instrument against a reconstruction of the pre-commit spec
set from the git objects: it prints exactly the three copies at
`editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41`,
`smoke.spec.ts:60` - the brief's three anchors, reproduced.

Survivor findable one glob over: `e2e/i18n-en.ts:175:export function name(`.
Presence control in both states, counted with `awk`: pre-commit object -> `0`,
current tree -> `1`. Spec-file count `9` before, `10` after.

### D5. Latitude, both forms - PASS

Composed rather than fenced, each checked: the spec-local helper names
(`preparePage`, `mountJobsView`, `installSoftOutcomeIPC`), the describe title,
the fixture constants (`RUN_ID`, the two `RunRequest` literals, `JOB0_OUTPUT`),
and the post-resolve barrier. None of them is a string D104 or the plan fences,
and none invents a contract. Every fenced fixture value is present and correct:
item 1's empty `jobs`, zeroed `summary`, `joblog_status: "unavailable"`,
`StartedRun { total_jobs: 0 }`; item 2's `rejectWith("run-already-active")`;
item 3's `total_jobs: 2` plus the `started` event at index 0 and the second
dispatch through `__muxsmithSetProps__`; item 4's `state: "failed"`,
`panic: "boom"`.

The inverse form - stopping where it should have proceeded - is where the task
was most exposed, and it stopped correctly in the one place that mattered
(section 4's fork; adjudication 5 below). The one composed helper that silently
narrows an existing one is LOW-1.

D-6, the probe mutation of `src/views/JobsView.vue`, a file off the Files list:
correct as executed. Baseline taken first, restored with `git checkout --`,
proven with `sha256sum -c` and `git status --porcelain`, Rust control run
because the mutation left `e2e/`. My own independent baseline hash of that file
today is `ca18ea6349b3420789ae22d829b5f992d27ec360669f6db2a4b0542faf0d30cb`,
identical to the one the report pasted before its probe, so the restore is
confirmed a second time by an instrument the implementer did not run.

### D6. House dimension - PASS

- **`proc-57-briefs-not-ground-truth`:** the report re-derived every dispatch
  anchor before editing. I re-measured a sample independently and all of them
  reproduce: the three helper copies at `:29/:41/:60` (via my own fire),
  `export function en(` at `e2e/i18n-en.ts:155`, the `FluentVariable` import at
  `:61`, `export function name(` count `0` pre-hoist, `9` spec files pre /
  `10` post, 2 `FluentVariable` hits per migrated file pre / 0 post,
  `data-testid="cancel-batch"` at `src/views/JobsView.vue:263`,
  `data-testid="job-panic"` at `src/components/JobRow.vue:63`. Zero failures.
- **`proc-normative-count-recomputed`:** every stated aggregate recomputed from
  its enumeration, not taken from the report - 68 e2e passed and
  `Total: 68 tests in 10 files` from `playwright test --list`, of which
  `jobsview-reset.spec.ts` contributes exactly 4, so 64 pre-existing, matching
  the pre-task baseline; 39 `test result:` lines, 0 non-ok; 10 spec files; 3
  deleted copies; 2 -> 0 `FluentVariable` per file. All correct as stated.
- **`latitude-carveout-zero-content-structural-forks`:** the Files-list
  boundary held over files (seven paths, exactly the list). The import
  add/drop cases are the plan's own enumerated Step-3 instruction, not the
  grant, and were executed as written. Nothing on the stop list was touched:
  no existing assertion weakened, deleted, skipped or reworded, no existing
  fixture value mutated, no new test infrastructure beyond D104's fenced hook.
  The one addition that lives at the grant's edge is D-3, adjudicated below.
- **`tests-ship-with-the-feature-never-after`:** the four-condition check was
  stated even though it did not fire, as the brief required. Premise verified,
  not accepted: `git show --stat a2c1028` puts all seven paths under `e2e/`,
  so the diff creates no user-visible consequence and condition 3 cannot hold
  for any candidate. Correct.
- **Amendment-5 hygiene (D9):** the header comment at `:36-43` points at the
  rider and does not restate the replacement, which is what the amendment-5
  verdict's MEDIUM-1 required; its wording is discussed in INFO-1. My own
  tree-wide sweep (`sweep.py`, with an impossible fourth pattern as its
  control: 127 / 3 / 34 hits against 0) finds **no surviving live restatement**
  of the falsified vehicle. The four sites that state it are all qualified or
  pointing: design `:1233-1236`, design `:1259-1260`, plan `:392`, spec
  `:39-43`. Every other hit is a record of the episode itself (reports,
  verdicts, briefs, the ledger occurrence at `docs/decision-ledger.yaml:4922`,
  `progress.md:38`), not a live claim.

### D7. The no-work-needed check - all four named premises run, all four hold

1. *The test-coverage rule cannot fire because the diff creates no
   user-visible behaviour.* Verified at the diff, above. Holds.
2. *The `name()` trigger's NOT-FIRED record stands because the new spec
   consumes no `name()`.* Verified with my own instrument, not the report's
   grep: `e2e/jobsview-reset.spec.ts:50` imports `{ en }` only, and a regex for
   `name(` that excludes `.name(` and identifier suffixes returns **0** hits in
   that file against **2** in `e2e/i18n-en.ts` as its control. The ROADMAP
   record at `docs/ROADMAP.md:668-680` stands as written.
3. *The existing mount specs passing is sufficient evidence the harness hook
   regressed nothing.* Holds, and on stronger ground than the report claims.
   The report rests it on "existing mount specs never call the new hook", but
   the change is not confined to the hook: every mount's props now travel
   through a `ref`, which deep-reactifies them. That path IS exercised -
   `mountWidget` passes a non-`modelValue` prop (`e2e/mount.ts:45`,
   `props: { spec, modelValue: model }`), so the 20 widget/editor tests drive
   the changed spread with a structured object. `modelValue` itself is
   unaffected: it was already a separate `ref` (`mount-entry.ts:60`) before the
   change.
4. *`e2e/mount.ts` needed no change.* Holds, and forced rather than merely
   allowed: its `loadHarness` couples `setContent` with the mount-bundle
   injection (`e2e/mount.ts:25-28`), while D104's composition requires the
   Tauri mock and the IPC handler to land *between* those two steps. Reusing it
   was impossible without editing it, which D104 forbids.

### D8. Verification quality - PASS, full bar re-run by me

```
$ pnpm lint                       -> exit 0, output: "$ eslint ."
$ pnpm test:e2e                   -> exit 0
   vite v8.1.4 building client environment for production...  (x2, both builds present)
   68 passed (2.9s)
$ cargo test --workspace          -> exit 0; 39 "^test result:" lines, 0 non-ok
   (control: a pattern that cannot occur returns 0 against the real 39)
$ pnpm exec playwright test --list -> Total: 68 tests in 10 files
   jobsview-reset.spec.ts 4  ->  64 pre-existing, matching the stated baseline
```

Typography: zero non-ASCII characters in six of the seven touched files; the
three in `e2e/smoke.spec.ts` are pre-existing German fixture strings at
`:783/:786/:789`, untouched by this diff. Commit hygiene: unsigned
(`%G?` -> `N`), exactly one `Co-Authored-By` trailer, no `Claude-Session` line,
seven paths staged by name, message identical to the plan's Task-6 fence.

---

## 3. The six adjudications

### A1. D-1, the post-resolve barrier in test 1 - WARRANTED, keep it

`e2e/jobsview-reset.spec.ts:155-162`. Keep, with an honest note about what it
does and does not buy.

What it buys: `await page.evaluate(() => document.readyState)` forces a full
round trip through the page, after which every microtask queued before it has
run. The re-assertion at `:162` therefore reads a settled DOM, and a clobber
landing after the first assertion passed - a retrying assertion resolves on its
first match and never looks again - would leave the second one failing through
its whole 5s window. That is the only construct in the test that turns D104's
"after the promise resolves" from an assumption into an observable, and it
costs about a millisecond.

What it does not buy, measured: against the actual defect class the item exists
for, the barrier is not what catches the regression. Under M-c the **first**
assertion fails, at `:153`, not the re-assertion. So the barrier is insurance
against a narrower class (a clobber scheduled later than the first read) rather
than the discriminator for the ruled ordering. Not noise - noise is a construct
with no failure mode at all - but the comment above it slightly oversells its
role. No change required; if the comment is ever touched, "insurance against a
clobber landing after the first read" is the accurate version.

### A2. D-3, the joblog note asserted alongside the summary line in test 1 - INSIDE the enumeration

`e2e/jobsview-reset.spec.ts:163`. Ruled inside, as an additive consequence of a
fenced fixture value.

D104 fixes `joblog_status: "unavailable"` in item 1's fixture and enumerates no
assertion on it. Left unasserted, that fenced value is inert - the design would
have pinned a fixture field that nothing observes, which is the shape a review
normally flags, not the shape it protects. The assertion adds no new locator
(the same `jobs-run-summary` paragraph renders both strings,
`src/views/JobsView.vue:309-323`), touches no existing assertion, mutates no
fixture value, adds no infrastructure, and has zero outward effect - so it sits
inside `latitude-carveout-zero-content-structural-forks`'s positive half rather
than on its stop list. The must-not-decide bullet fences "the four assertions
of D104" against rewording, not against a fifth line; nothing in D104 says item
1's list is closed downward.

Two things make me comfortable ruling for the implementer rather than against:
it is the *only* observation of a value the design itself chose to fence, and
it was disclosed as D-3 for adjudication instead of taken silently. That is the
correct handling of a grant-edge case, and it is calibration data for the
over-restriction watch in the permitted direction.

### A3. D-2, the panic fixture's non-fenced fields copied from the real emitter - RIGHT CALL

`e2e/jobsview-reset.spec.ts:244-251`. Verified against the source rather than
against the report: `recover_panicked_worker`'s construction at
`crates/muxsmith-core/src/executor/queue.rs:457-466` is
`state: Failed, exit_code: None, warnings: [], errors: [format!("{}: job
{index}", DiagCode::WorkerPanicked.key())], duration_ms: 0, panic: Some(msg)`,
and `WorkerPanicked => "worker-panicked"`
(`crates/muxsmith-core/src/report/mod.rs:189`). The fixture is that shape
exactly, including `"worker-panicked: job 0"`.

"Minimal placeholders" is not the cheaper option here: `satisfies JobEvent`
forces every field to carry *some* value, so the choice is only which. Real
values cost nothing extra and document what the emitter produces. The one
residual is `errors`, which no assertion reads: if the Rust format ever
changes, this fixture goes stale while staying green. That risk is identical
under placeholders (a placeholder is stale from birth), and the honest fix if
it ever matters is an assertion on the token, not a weaker fixture. Keep.

### A4. D-5, the props ref's lifetime inside `mount()` - SOUND, and the better of the two options

`e2e/mount-entry.ts:70-73`. The alternative - a module-scope ref - would carry
props across mounts, so a second `__muxsmithMount__` would inherit the previous
test's merged partials unless explicitly reset. Per-mount binding makes the
reset structural instead of remembered, and it mirrors `__muxsmithModel__`
(`:60-61`) and `__muxsmithEmitted__` (`:51`), which are assigned in the same
place for the same reason. That is the house pattern in this file, and the
`global.d.ts` doc states the consequence accurately: "Reset with the props on
every `__muxsmithMount__` call".

The hazards per-mount binding could create are not reachable here. A caller
holding a stale function reference across a re-mount would write into the
unmounted app's ref - silent but harmless, and no spec does it; the repo has
exactly one call site (`e2e/jobsview-reset.spec.ts:210`, confirmed by a
tree-wide search: declaration, assignment, one consumer). The typing is
non-optional while the global is undefined before the first mount, which is a
lie the file already tells about `__muxsmithModel__` - a pre-existing pattern,
not something this change introduced. D104 fences name and merge semantics and
is silent on lifetime; the silence was filled with the neighbouring precedent,
which is the correct way to fill it.

### A5. The commit decision - the reading was RIGHT

Two independent reasons, and either alone carries it.

The entry's condition (b) is "green on the task's full verification bar"
(`docs/decision-ledger.yaml:4694`). The bar was red, and the implementer's
observation is the sharp one: it stays red after any partial commit, because
the artifact that reddens it is the untracked spec file. Measuring the six-file
subset green would have required deleting the task's central artifact from the
tree first - a state nobody intended to leave. The entry's own instruction on
failing any of the three is explicit: the work stays uncommitted.

The second reason is stronger and the entry does not anticipate it at all. The
plan's Task-6 commit message is a fenced string (`plan :403`) naming "the ruled
D23 reset tests + panic render". The only committable subset was precisely the
one without them. Committing under that message would have made the message
false; editing it would have been an unrouted deviation from a fenced string.
There is no third option.

Against that, the risk the entry exists to prevent was real and did partly
materialize: two commits landed in the routing interval (`1e0dbd8`, `18ef645`,
56 minutes between `dc7c605` and `a2c1028`; see INFO-4). Neither touched
`e2e/`, so nothing was lost - but that is luck, not design. The correct
response is not that the implementer should have committed; it is that the
entry has a gap, recorded in the HARVEST.

The report also offered to stage the six on request rather than treating its
own reading as final, which is the right posture for a judgment the controller
owns.

### A6. Test 2's replacement pair in situ - it asserts what item 2 needs, and no state satisfies it while `runActive` is true

Both halves, one by structure and one by measurement.

By structure: `jobs-empty` (`src/views/JobsView.vue:328`) is the `v-else`
(`:327`) of `v-if="jobs.length > 0 || runActive || finishedSummary"` (`:258`),
and it is the sole element carrying that testid anywhere in `src/`.
`toBeVisible()` requires the element attached, attachment requires the `v-else`
branch active, and that requires all three disjuncts false - `runActive`
included. So `jobs-empty` visible entails `runActive === false`
unconditionally, whatever the other two disjuncts do. There is no satisfying
state with `runActive` true.

By measurement, and this is the part that answers what the pairing buys *in
situ* rather than in the abstract - the amendment reviewer was right that the
rider's recorded reason ("`toHaveCount(0)` alone would pass against a view that
never mounted") is not it, since `:178`'s alert assertion already proves the
mount. I built M-e: the catch-arm guard neutralised (so `runActive` stays
**true**, the exact state item 2 must reject) **and** the `cancel-batch` testid
renamed. Result, with both bundles rebuilt:

```
  ✘ 33 [chromium] > ... > fresh dispatch rejection renders the error and clears runActive
    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-jobs').getByTestId('jobs-empty')
    Error: element(s) not found
      179 |     await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
    > 180 |     await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
```

`:179` **passed** - vacuously, the control it counts no longer exists under that
name - and `:180` caught it. The bearer is what stands between item 2 and a
silently vacuous assertion, exactly as
`the-absence-idiom-is-count-zero-paired-with-a-positive-bearer`
(`docs/decision-ledger.yaml:4894`) states as reasoning. That entry's clause
about surviving a rename is now measured rather than argued.

---

## 4. Evidence appendix

All instruments are mine, written for this review, under
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t6rev-independent/`.
No instrument another agent wrote was re-run, and no path any report names was
reused.

| Path | What it does | Its fire |
|---|---|---|
| `absence-probe.sh` | `awk`-only walk of `e2e/*.spec.ts` reporting every column-0 `name` definition in three forms; deliberately not `grep`, since the local one is ugrep 7.5.0 | run against a reconstruction of the pre-commit spec set from git objects (`fire-tree/`): prints exactly the three copies at `:29/:41/:60`; current tree prints nothing |
| `sweep.py` | tree-wide sweep for `cancel-batch`, `disabled again`, `jobs-empty` outside `.git`/`node_modules`/build dirs | a fourth, impossible pattern returns 0 while the three real ones return 127 / 3 / 34 |
| `callsites.py` | decomposes `name(` hits per file into definition / doc comment / real call sites, current tree versus the `HEAD~1` git object | control file with no helper returns 0 while `i18n-en.ts` returns 2 |
| `fence-mountentry.txt`, `tree-mountentry.txt` | D104 `:1181-1194` versus `e2e/mount-entry.ts:26-39` | one letter changed in the tree copy -> 4 diff lines |
| `old-smoke-helper.txt`, `new-helper.txt`, `old-normalized.txt` | the deleted smoke copy from the `HEAD~1` git object versus `e2e/i18n-en.ts:168-177` | `SUBSTRING` -> `SUBSTRINg` in the new copy -> 1 diff line |
| `rider-fence.txt`, `committed-pair.txt` | design `:1310-1311` versus spec `:179-180` de-indented | `toHaveCount(0)` -> `(1)` -> 1 diff line |
| `plan-names.txt`, `spec-names.txt` | the four fenced test names versus the four in the file | `runActive` -> `runactive` -> 1 diff line |
| `baseline.sha256` | pre-mutation hashes of `src/views/JobsView.vue` and `src/components/JobRow.vue` | `sha256sum -c` OK after every restore; `git status --porcelain` and `git diff --stat` both empty at the end |
| `e2e-baseline.log`, `e2e-Ma.log`, `e2e-Mb.log`, `e2e-Mc.log`, `e2e-Md.log`, `e2e-Me.log`, `e2e-restored.log`, `e2e-final.log` | the eight `pnpm test:e2e` runs (baseline, five mutations, two restores) | every log contains both `vite build` steps, so each measures its own rebuilt bundles |
| `cargo.log`, `lint-baseline.log`, `list.log` | the Rust control, lint, and the per-file test census | impossible-pattern control on the `test result:` count |

**Mutations applied and restored.** Five source mutations
(`src/views/JobsView.vue` x4, `src/components/JobRow.vue` x1), each with the
baseline taken before the edit, each restored with `git checkout --` (never a
bare `cp`), each proven with `sha256sum -c` -> OK. Final state:
`git status --porcelain` empty, `git diff --stat` empty, HEAD still `a2c1028`,
`pnpm test:e2e` 68 passed on the restored tree. I committed nothing and edited
no product file; this verdict file is my only write.

---

## 5. HARVEST

**For Task 7.** Nothing blocking. Its mutate-measure-restore protocol is the
same shape Task 6's D-6 probe executed correctly, and this review executed five
more times: baseline hash before the edit, `git checkout --` to restore,
`sha256sum -c` plus an empty `git status --porcelain` as the proof. Task 7's
Step 5 already demands exactly that, and the shape is now proven twice over on
this tree. One difference worth carrying: Task 7 mutates Rust, so the
frontend-rebuild rule does not apply to it - `cargo test` compiles what it
runs.

**For the plan close.**

1. `progress.md:14` still reads `FIX ROUND ... uncommitted in the tree`; the
   task is DONE and committed at `a2c1028`, tree clean, bar green. Also
   `progress.md` is where the amendment-5 verdict's INFO-3 already pointed.
2. The `name()` trigger's close action is a confirmation only, as the plan
   states, and it stands: `docs/ROADMAP.md:668-680` records NOT FIRED /
   CONSUMED EARLY, and I re-measured the premise - the new spec imports `{ en }`
   alone (`e2e/jobsview-reset.spec.ts:50`) and calls `name(` zero times, so no
   fourth consumer arose. Confirm and move on.
3. LOW-1 is the only open item from this review and may be discharged here
   rather than in a fix round.
4. The four-mutation measurement in section D2 is the acceptance evidence for
   the D23 item's coverage half, stronger than "the tests pass": each of the
   four ruled tests is the sole catcher of its own regression across the whole
   68-test suite. Worth citing when the ROADMAP D23 item is marked resolved.

**Ledger-worthy.**

1. **Occurrence for `the-absence-idiom-is-count-zero-paired-with-a-positive-bearer`
   (`docs/decision-ledger.yaml:4894`).** Its closing clause - the bearer "also
   survives someone deleting or renaming the absent control's testid, which the
   count alone accepts silently" - was reasoning when written and is now
   measured: M-e (flag left true **and** testid renamed) let `toHaveCount(0)`
   pass while `jobs-empty` failed, on the first real instance of the idiom in
   the tree. `kind: reinforced`.
2. **Occurrence, and a statement gap, for
   `a-returning-task-may-commit-the-subset-that-survives-every-option`
   (`docs/decision-ledger.yaml:4694`).** Task 6 declined to commit and the
   reading was right, but for a reason the entry does not carry: condition (b)
   is unmeasurable when the *omitted* artifact is what reddens the bar, and the
   entry is silent on a plan-fenced commit message that names the missing
   artifact - which is the decisive constraint here and cannot be satisfied by
   staging by path. Candidate extension: a fourth consideration, "the fenced
   commit message must stay true of what is committed; where it names the
   omitted artifact, the subset is uncommittable regardless of the other
   three". Its first two occurrences (Task 4 committed, Task 6 did not) are the
   two sides of the same rule, which is unusually good evidence for one plan.
3. **Trigger candidate (INFO-2).** A FOURTH declaration of a
   `.generated/` harness-bundle path constant -> export
   `HARNESS_PATH`/`MOUNT_HARNESS_PATH` from `e2e/mocks.ts:24` and
   `e2e/mount.ts:14` and import them. Three sites exist now
   (`mocks.ts`, `mount.ts`, `jobsview-reset.spec.ts:54-55`), the same count at
   which the `name()` helper's trigger was registered, and the trigger's event
   is readable from the text: you are typing
   `resolve(import.meta.dirname, ".generated/` in a file that owns neither
   constant. Registering it also avoids repeating the `name()` trigger's own
   recorded defect - its condition counted consumers while its prose said
   "spec file", so read this one as counting *declaration sites*.
4. **Observation, no entry proposed.** The one measurement in this review that
   nobody had made is that the spec's page-side handler genuinely reproduces
   the Rust command's emit-before-resolve ordering (M-c: under a post-resolve
   reset the summary is absent through a full 5s retry, which requires
   `onRunFinished` to have written it first). D104 and
   `gui-d23-reset-gating-form` both assert that ordering from the Rust doc
   comments; it is now observed from the frontend side too. If
   `gui-d23-reset-gating-form` is ever touched, that is worth a sentence -
   it converts a documented premise into a measured one.
