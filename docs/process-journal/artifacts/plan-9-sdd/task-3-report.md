# Task 3 report - Plan 9 (worker-panic payload, end to end)

**Status: DONE_WITH_CONCERNS.** Every step 1-11 executed, the full verification
bar green and pasted below, committed as `9e5e112`. Three of the four concerns
below are boundary calls a reviewer should rule on yes/no; the fourth is a
process hazard surfaced for the controller. No design question was resolved at
the keyboard: no fork was found that the plan or the design had not already
closed (see "Forks looked for and not found").

---

## 1. What changed, per file, against the plan's Files list

16 files, all inside the Files list. 13 are named entries; 3
(`tests/executor_events.rs`, `tests/joblog.rs`, `tests/report_json.rs`) enter
through the entry the plan marks EXEMPLARY ("any other file the compiler flags
for a `JobOutcome` literal"), which the compiler enumerated (section 2.3).

| File | Change |
|---|---|
| `crates/muxsmith-core/src/executor/job.rs` | `JobOutcome.panic: Option<String>` with D98's doc comment verbatim, placed after `duration_ms`; `panic: None` at the three flagged constructors (`run_job`'s pre-spawn cancel, its spawn-failure arm, `finish`) |
| `crates/muxsmith-core/src/executor/queue.rs` | `recover_panicked_worker` sets `panic: Some(message)` and its `eprintln!` is deleted; its licence doc block rewritten to D98 fork 12's rationale; the `run_batch` `///` block replaced by D96's amendment-3 rider fence (Step 2); `panic: None` at the `run_queue` fallback constructor (`outcome.unwrap_or(...)`); `worker_panic_is_reported_as_failed_not_cancelled` extended with the payload assertion (and its doc's enumeration of what it pins gained the payload clause) |
| `crates/muxsmith-core/src/executor/joblog.rs` | `JobRecord` gains `panic: Option<&'a str>`, assigned `outcome.panic.as_deref()` |
| `crates/muxsmith-core/tests/executor_events.rs` | flagged constructor gains `panic: None`; the exact serialized-string expectation gains `,"panic":null` (concern 1) |
| `crates/muxsmith-core/tests/joblog.rs` | `outcome()` helper gains `panic: None` |
| `crates/muxsmith-core/tests/report_json.rs` | `outcome()` helper gains `panic: None`; the exact-JSON expectation's four job objects each gain `"panic": null` (concern 1) |
| `crates/muxsmith-cli/src/commands/run.rs` | `render_finished` returns `Vec<String>` and its `Failed` arm branches on `outcome.panic` (concern 2); `render`'s `Finished` arm forwards the vector; the method doc restated; test helper `outcome()` gains `panic: None`; new unit test `finished_panicked_renders_two_lines_without_na` |
| `crates/muxsmith-cli/tests/catalog_completeness.rs` | `"run-job-panicked"` added to `ALLOWLISTED_CLI_KEYS` after `"run-job-failed"`; it joins the existing `"run-job-start" \| "run-job-cancelled"` fixture arm, whose `vec!` is exactly D99's prescribed args (concern 3); `DiagCode::WorkerPanicked => vec![("detail", "queue worker thread panicked")]` |
| `locales/en/diagnostics.ftl` | `worker-panicked` line replaced (D99 fence 1) |
| `locales/de/diagnostics.ftl` | `worker-panicked` line replaced (D99 fence 2) |
| `locales/en/cli.ftl` | `run-job-panicked` inserted after `run-job-failed` (D99 fence 3) |
| `locales/de/cli.ftl` | `run-job-panicked` inserted after `run-job-failed` (D99 fence 4) |
| `src/ipc.ts` | `JobOutcome.panic: string \| null` (required, so `RunJobEntry` inherits it) and `JobLogRecord.panic: string \| null` |
| `src/components/JobRow.vue` | `panicDetail` computed + D100's span in the state cell |
| `e2e/smoke.spec.ts` | `panic: null` on the four live-run fixture literals the type-check enumerated |
| `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | S-1's `WorkerPanicked` row replacement (spec `:289`) and S-2's 8.4 fragment extension (spec `:410`) |

**Nothing the Files list did not predict.** No `src-tauri` file was touched, per
the rider's no-src-tauri-sentence decision. No `Cargo.toml`/`package.json`
change (no logging facade, no `log`/`tracing`).

---

## 2. Evidence

### 2.1 Brief anchors, re-verified (located by content, not line number)

```
$ grep -n "fn recover_panicked_worker\|eprintln!\|fn worker_panic_is_reported_as_failed_not_cancelled\|pub fn run_batch" crates/muxsmith-core/src/executor/queue.rs
348:pub fn run_batch(
424:fn recover_panicked_worker(
441:    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");
783:    fn worker_panic_is_reported_as_failed_not_cancelled() {
```

The brief's three anchors (424 / 441 / 783) match the tree exactly. The brief's
correction to Task 2's claim also holds: neither moved test constructs a
`JobOutcome`, and the compiler flagged neither (section 2.3).

One stale line citation noted, not load-bearing (the entry instructs
locate-by-content, which is what I did): D98 says the `eprintln!` is at
`queue.rs:396`, pre-Task-2 numbering; it was at `:441`. D96's rider's
`:327-347` for the doc block was still exact - my splice measured the block at
lines 327-347 before replacing it.

### 2.2 Step 2: the rustdoc transcription

Extracted programmatically from D96's amendment-3 rider (25 lines, max width 75,
every line starts `///`), spliced in above the `pub fn run_batch` anchor, then
checked against the **committed** state:

```
A) installed /// block == D96 rider fence (committed state): True
   diff: (empty)
B) run_batch signature+body byte-identical to HEAD~1: True
   body lines: 22
C) old doc block replaced (not merely reflowed): True | old lines: 21 new lines: 25
```

(B) is the "rest of the function is byte-identical" half: the function from
`pub fn run_batch(` to its closing `}` is byte-for-byte equal between `HEAD~1`
and `HEAD`. Task 1's re-wrapping allowance was not applied - the fence's own
wrapping is what landed.

### 2.3 Step 3: the compiler sweep (the compiler enumerated the set)

Pass 1 (after the field was added):

```
error[E0063]: missing field `panic` in initializer of `JobOutcome`
   --> crates/muxsmith-core/src/executor/job.rs:108:16
   --> crates/muxsmith-core/src/executor/job.rs:129:20
   --> crates/muxsmith-core/src/executor/job.rs:192:5
   --> crates/muxsmith-core/src/executor/queue.rs:316:31
```

Pass 2 (once `muxsmith-core`'s lib compiled and the test/dependent targets could
be checked):

```
error[E0063]: missing field `panic` in initializer of `JobOutcome`
  --> crates/muxsmith-core/tests/executor_events.rs:55:18
  --> crates/muxsmith-core/tests/joblog.rs:24:5
  --> crates/muxsmith-core/tests/report_json.rs:13:5
   --> crates/muxsmith-cli/src/commands/run.rs:468:9
```

Pass 3: `cargo build --workspace --all-targets` -> no output (clean).

**Eight compiler-flagged sites in six files, all set `panic: None`.** Adding
`recover_panicked_worker`'s own literal (edited in Step 1, so never flagged) the
task touches nine constructors; it is the only `Some`-setter in non-test code,
and the CLI unit test is this task's one deliberately-`Some` fixture (Task 6's
e2e event is the other, per design round-2 note 1). No `src-tauri` site was
flagged.

### 2.4 Step 4: the fixture sweep enumerated by the type-check, not by eye

```
$ npx tsc --noEmit -p e2e/tsconfig.json
e2e/smoke.spec.ts(554,7): error TS2741: Property 'panic' is missing in type '{ state: "ok"; ... }' but required in type 'JobOutcome'.
e2e/smoke.spec.ts(559,7): error TS2741: Property 'panic' is missing in type '{ state: "cancelled"; ... }' but required in type 'JobOutcome'.
e2e/smoke.spec.ts(569,9): error TS2741: Property 'panic' is missing in type '{ index: number; ... }' but required in type 'RunJobEntry'.
e2e/smoke.spec.ts(570,9): error TS2741: Property 'panic' is missing in type '{ index: number; ... }' but required in type 'RunJobEntry'.
```

Exactly four, all in the live-run scenario (design amendment M-4's claim holds).
`vue-tsc --noEmit` over `src/` produced no output in the same run - no
`JobOutcome`-shaped literal exists in `src/`. After the edit:
`npx tsc --noEmit -p e2e/tsconfig.json` -> `TSC OK`.

### 2.5 Acceptance observable 4's Task-3 emitters (design section 7 item 4)

Named in the `cargo test --workspace` output:

```
test executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled ... ok
test commands::run::tests::finished_panicked_renders_two_lines_without_na ... ok
test every_diag_code_renders_without_leftover_placeholders ... ok
test every_cli_ftl_key_is_a_diag_code_or_allowlisted ... ok
```

(The e2e `job-panic` emitter rides Task 6, per the plan.)

**The placeholder-leak guard is fire-verified**, because its passing result is
an absence (no leaked `{$param}`). Mutation: the `DiagCode::WorkerPanicked`
fixture row put back to `vec![]`, everything else untouched:

```
test every_diag_code_renders_without_leftover_placeholders ... FAILED
---- every_diag_code_renders_without_leftover_placeholders stdout ----
thread '...' panicked at crates/muxsmith-cli/tests/catalog_completeness.rs:169:5:
DiagCode message(s) with an unresolved placeholder:
worker-panicked: A worker thread panicked while running this job: {$detail}. This is a Muxsmith bug, not an mkvmerge failure; the run's persisted job log carries the full record.
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Restored (`diff -q` against the pre-mutation copy -> `IDENTICAL`) and green
again:

```
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

So the guard really does prove template and params agree for `worker-panicked`;
it is not passing vacuously.

### 2.6 Core-stdio absence check, with its fire and its known-present control

```
=== POST-STATE (working tree) ===
$ grep -rn "eprintln!\|println!\|print!(" crates/muxsmith-core/src
crates/muxsmith-core/src/lib.rs:24:// literal at ~21 `eprintln!` sites across muxsmith-core, muxsmith-cli and

=== FIRE: same pattern+pathspec on the committed pre-state HEAD=a54a04a ===
$ git grep -n "eprintln!\|println!\|print!(" HEAD -- crates/muxsmith-core/src
HEAD:crates/muxsmith-core/src/executor/queue.rs:441:    eprintln!("muxsmith: worker thread panicked while running job {index}: {message}");
HEAD:crates/muxsmith-core/src/lib.rs:24:// literal at ~21 `eprintln!` sites across muxsmith-core, muxsmith-cli and

=== CONTROL: known-present case, same pattern, CLI pathspec ===
$ grep -rln "eprintln!\|println!\|print!(" crates/muxsmith-cli/src
crates/muxsmith-cli/src/commands/identify.rs
crates/muxsmith-cli/src/commands/validate.rs
crates/muxsmith-cli/src/commands/dry_run.rs
crates/muxsmith-cli/src/commands/mod.rs
crates/muxsmith-cli/src/commands/run.rs
crates/muxsmith-cli/src/main.rs
control file count: 6
```

Post-state exactly 1 hit, the `lib.rs` comment, zero call sites - as the plan
predicts. That comment stays true and untouched: its "~21 `eprintln!` sites"
counts the skip-marker sites in the three crates' **test** files, which this
grep's `src` pathspec does not reach.

### 2.7 Rust bar (final tree state, foreground, no subsets)

```
$ cargo fmt --all --check
fmt exit: 0
$ cargo clippy --workspace --all-targets -- -D warnings
clippy exit: 0
$ cargo test --workspace
test exit: 0
test-binary results ok: 39
failed lines: 0
```

**No insta churn:** `find . -name "*.snap.new"` -> 0 files, and
`git status --porcelain -- '*.snap'` -> 0 modified snapshots. The `panic: None`
rendering path is byte-unchanged, exactly as the plan requires.

Extra, beyond the task's bar (run because Step 5's new doc comment introduces a
cross-crate intra-doc field link `[`JobOutcome::panic`]`):

```
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo doc exit: 0
```

### 2.8 Frontend bar

```
$ pnpm lint            -> exit 0
$ pnpm build           -> exit 0  (vue-tsc --noEmit && vite build; "✓ built in 151ms")
$ pnpm check:i18n      -> exit 0
   check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated,
   22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for
   parity against 7 en/ catalog(s)).
$ pnpm test:e2e        -> exit 0
   62 passed (2.9s)
```

`check:i18n` is the guard that both new `run-job-panicked` lines exist in
lockstep and that `JobRow.vue`'s literal `$t("worker-panicked", ...)` resolves.

### 2.9 Fence fidelity (the character-for-character contracts)

- The four D99 Fluent texts were extracted from the design's own fences and
  written without retyping. Verified after the fact: each fence string is
  present verbatim as a whole line - `locales/en/diagnostics.ftl:80`,
  `locales/de/diagnostics.ftl:87`, `locales/en/cli.ftl:36`,
  `locales/de/cli.ftl:43`. German orthography (`ä`, `ü`, `ß`) copied exactly.
- D100's span: dedenting the installed block yields the fence exactly
  (`D100 fence == installed span (indentation removed): True`). Only the
  6-space indentation differs, which D100 rules implementer-owned placement.
- S-1/S-2 were applied by locating the design's "current text verbatim" fence in
  the spec (`count == 1` asserted for each) and substituting the replacement
  fence; S-1's `EmptyRawProperty` row was deliberately **not** inserted (Task 4
  owns it).

### 2.10 Things the "must not decide" list forbids, shown untouched

```
$ git show HEAD | grep -cE '^[+-].*delete_partial'    -> 0
$ git show HEAD | grep -cE '^[+-].*create_dir_all'    -> 0
$ git show HEAD | grep -cE '^[+-].*remove_dir_all'    -> 0
$ git show --name-only --format= HEAD | grep -c 'executor/spawn.rs'  -> 0
$ git show HEAD | grep -cE '^[+-].*DiagCode::WorkerPanicked.key\(\)' -> 0
```

Each of these is an absence, so each pattern was fired on a synthetic
known-present diff line first (`delete_partial -> 2`, `create_dir_all -> 2`,
`remove_dir_all -> 2`, the `errors`-token pattern `-> 1`). The `errors` token
itself is still in the tree, byte-identical:

```
$ grep -n 'errors: vec!\[format!("{}: job {index}", DiagCode::WorkerPanicked.key())\]' crates/muxsmith-core/src/executor/queue.rs
461:        errors: vec![format!("{}: job {index}", DiagCode::WorkerPanicked.key())],
```

So: fork 9 (`delete_partial_failed`) untouched, the four ruled-discarded
executor failures (`job.rs` create_dir_all, `joblog.rs` remove_dir_all,
`spawn.rs` kill, `spawn.rs` wait) untouched - `spawn.rs` is not in the commit at
all - and the token format byte-identical.

### 2.11 The import-removal doc-link sweep: trigger did not fire

```
$ git diff -U0 -- '*.rs' | grep -E "^-[^-]" | grep -cE '^\-\s*(pub )?use '
0
```

Fired on a known-present case so the zero is a real zero: the same pattern over
Task 2's commit `9b2843f` returns **7** removed `use` lines (e.g.
`-use std::sync::mpsc;`, `-use muxsmith_core::executor::queue::{JobEvent, QueueControl, QueueOpts, run_queue};`).
This task's diff deletes no `use` line, so the sweep has no subject. The
neighbouring risk was checked anyway: `DiagCode` is still used in `queue.rs`
(the `errors` token) and every intra-doc link in the rewritten blocks resolves
(section 2.7's rustdoc run).

---

## 3. Divergences and judgment calls under the grant, each named

1. **`render_finished` returns `Vec<String>` instead of `String`** (see concern
   2). Forced by D99, not chosen.
2. **Field position.** D98 fixes the field's name, type, doc and
   always-serialized form but not its position. Placed last in `JobOutcome`,
   and directly after `duration_ms` in `JobRecord` and in `ipc.ts`'s
   `JobLogRecord`, so the outcome-field block stays contiguous and Rust and TS
   declaration orders keep mirroring each other (the file's existing pattern -
   `ipc.ts` mirrors the Rust field order exactly). See concern 4.
3. **`run-job-panicked` joined an existing fixture arm** rather than getting its
   own (see concern 3).
4. **Two doc comments restated** because my own enumerated edit falsified them,
   inside listed files, zero outward effect: `render_finished`'s ("One
   `run-job-{ok,warning,failed,cancelled}` line ..." became false once the arm
   can return two lines and a fifth key), and
   `worker_panic_is_reported_as_failed_not_cancelled`'s doc, whose enumeration
   of what the test pins gained the payload clause
   (`proc-normative-count-recomputed` trigger 2: a member joined the set the
   sentence enumerates).
5. **The `recover_panicked_worker` licence-doc rewrite is composed prose.** The
   plan states its content requirements (D98 fork 12's rationale: payload
   carried as data, rendered through the catalog at presentation time, the
   spec's normal path rather than an exception, core still authors no
   user-facing prose, `core-37-prose-free-core`); no character-for-character
   fence exists for it, unlike Step 2's. The wording is mine, the content is
   D98's; it keeps the pre-existing `[`JobOutcome`]` / `[`DiagCode`]` links.

## 4. Forks looked for and not found

Checked deliberately, since the brief instructs a stop on any of them:

- **A hidden `JobOutcome` consumer.** The compiler enumerated the constructor
  set (2.3) and `vue-tsc`/`tsc` the TS side (2.4); no `src-tauri` site, no
  `src/` literal. `RunHistory.vue` consumes `JobLogRecord` read-only
  (`record.lines`), so the required TS field breaks nothing there and the
  RunHistory export stays raw-output-only, as carried into Task 6.
- **A design statement the code refutes.** None found. D98's `queue.rs:396` and
  D96's `:327-347` are stale numbers, not refuted statements - both entries
  instruct locate-by-content.
- **A ripple the plan did not predict.** The one runtime ripple (2.10's sibling:
  the two exact-serialization expectations) is predicted in kind by design
  section 7 item 1 - "the existing ... tests pass unchanged **except where
  D102's ordering and D98's field are the designed deltas**" - and its files are
  in the Files list via the EXEMPLARY compiler entry. See concern 1 for why I
  treated it as mechanically forced rather than routing it.

---

## 5. Numbered concerns a reviewer can rule on yes/no

**Concern 1 - updating two existing serialization expectations.**
`report_json.rs`'s exact-JSON equality and `executor_events.rs`'s exact
serialized-string equality both went red once the field was on the wire (pasted
failure, `report_json.rs:87`: left carried `"panic": Null`, right did not). I
added `"panic": null` / `,"panic":null` to the expected values rather than
returning NEEDS_CONTEXT. Reasoning: (a) both files are Files-list members
through the EXEMPLARY compiler entry, which carries no within-file qualifier, so
the file-vs-within-file boundary puts work inside them in scope; (b) the edits
are strictly additive - every previously asserted value is byte-identical and
both assertions are *stronger* afterwards, so this is not "weakening, deleting,
skipping or rewording an existing assertion" nor "mutating existing fixture
values"; (c) D98 forbids `skip_serializing_if` and mandates the field always on
the wire, so exactly one action is consistent with the named invariants - there
was no option set to weigh, hence no decision memo to write. **Rule: was that
the right call, or should the two expectation updates have been routed?**

**Concern 2 - `render_finished`'s return type.** D99 puts the panic branch
inside `render_finished` ("`render_finished`'s `JobState::Failed` arm branches
on `outcome.panic`") *and* requires two lines ("`MilestoneState::render`'s
`Finished` arm returns two lines instead of one"). A `-> String` return cannot
express both, so the private method now returns `Vec<String>` and `render`'s
`Finished` arm forwards it instead of wrapping it. One call site, no public API
change, no other arm's rendered bytes changed. **Rule: correct reading of D99?**

**Concern 3 - `run-job-panicked`'s fixture arm.** D99 prescribes fixture args
`[("index", "1"), ("total", "3"), ("output", "/out/movie.mkv")]`. The file
already has an arm producing exactly that `vec!` for
`"run-job-start" | "run-job-cancelled"`, and the file's house pattern groups
identical fixture sets into one arm (`"identify-failed" | "identify-not-media"`,
`"run-joblog-written" | "run-joblog-incomplete"`). I extended that arm rather
than adding a fourth duplicate. Still three edits, per D99's obligations list.
**Rule: acceptable, or should it have been its own arm to keep the D99 fence
textually visible at its own key?**

**Concern 4 - field position, hence JSON key order.** See section 3 item 2. The
serialized order is now `..., "duration_ms": N, "panic": null` on every
`JobOutcome` surface, and `..., "duration_ms": N, "panic": ..., "lines": [...]`
on `job-<index>.json`. No consumer depends on key order (`serde_json::Value`
equality is order-independent; `ipc.ts` interfaces are unordered), and the one
order-sensitive assertion was updated to match. **Rule: is the position the one
intended, given D98 fixes everything about the field except this?**

---

## 6. For the controller

1. **Coverage observation, deliberately not acted on.** Design acceptance 4
   names as user-observable that "its `job-<index>.json` contains
   `"panic": "<payload>"`". That is now structurally true (`JobRecord` carries
   it; the key is on every persisted record), but **no test asserts the
   persisted `panic` key.** `crates/muxsmith-core/tests/joblog.rs` asserts every
   other `JobRecord` key individually (`index`, `output`, `argv`, `state`,
   `exit_code`, `warnings`, `errors`, `duration_ms`, `lines`, `started_at`,
   `finished_at`) and did not gain a `panic` line. I did not add one: the plan's
   Step 7 ("the pinned tests") is an enumeration in a normative position - (a)
   the queue assertion, (b) the CLI unit test - and the structural grant fills
   silence only, so an enumeration wins over it. Recommendation: a one-line
   additive `assert_eq!(job["panic"], serde_json::Value::Null);` beside its
   siblings in `joblog.rs`, routed as a controller decision (either a Task-3 fix
   round or folded into Task 6's dispatch, which already owns the panic-render
   coverage). Cost of leaving it: the persisted half of acceptance 4 has no
   emitter, so a future change that drops the field from `JobRecord` is caught
   only by `ipc.ts`'s type, not by a test.
2. **Process hazard for Task 7 (D105), which is a mutate-measure-restore
   experiment.** A bare `cp` in this shell is aliased interactive: my restore
   step (`cp <backup> <file>`) blocked on `cp: overwrite '...'?`, which turned a
   foreground run into a hung 600s job **with the tree left in the mutated
   state**. Recovered with `command cp -f` and verified `diff -q` IDENTICAL, and
   the file's committed content is byte-identical to the pre-mutation copy - but
   a mutate-restore step whose restore can block interactively is a real risk of
   committing a mutated tree. Handle: every restore in a mutate-measure-restore
   step uses a non-interactive form (`command cp -f`, `git checkout --`,
   `install`), and the step ends with an explicit `diff -q`/`git status` proof
   of restoration. Worth a house-knowledge line before Task 7 runs.
3. **One stale line citation in the design** (not load-bearing; the entry
   instructs locate-by-content): D98's "deletes the `eprintln!` at
   `queue.rs:396`" - it was at `:441` after Task 2's hoist. D96's rider's
   `:327-347` was still exact at Step-2 time, measured before the splice.
4. **Task 2's report claim was refuted, as the brief said.** Confirmed
   independently by the compiler: neither moved test constructs a `JobOutcome`,
   so neither was flagged (2.3). The brief's correction is right.

---

## 7. Commit

```
$ git log -1 --format='%B'
executor+cli+gui: worker-panic payload travels as JobOutcome.panic and renders on both surfaces (D98-D100, S-1/S-2)

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format='%G?'
N
```

One trailer, no `Claude-Session` line, unsigned, not pushed. Staged by explicit
pathspec, each of the 16 files by name; the commit itself is pathspec-scoped
(`git -c commit.gpgsign=false commit -m ... -- <16 paths>`). Working tree clean
afterwards (`git status --short` empty; `e2e/.generated/` bundles are
gitignored, `.gitignore:20`).

```
$ git show --stat HEAD
commit 9e5e112f4560c00db5b28d8a71bfa3b3a099c152
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Tue Jul 28 19:21:29 2026 +0200

    executor+cli+gui: worker-panic payload travels as JobOutcome.panic and renders on both surfaces (D98-D100, S-1/S-2)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/src/commands/run.rs            | 97 ++++++++++++++++------
 crates/muxsmith-cli/tests/catalog_completeness.rs  |  5 +-
 crates/muxsmith-core/src/executor/job.rs           |  8 ++
 crates/muxsmith-core/src/executor/joblog.rs        |  2 +
 crates/muxsmith-core/src/executor/queue.rs         | 48 +++++++----
 crates/muxsmith-core/tests/executor_events.rs      |  3 +-
 crates/muxsmith-core/tests/joblog.rs               |  1 +
 crates/muxsmith-core/tests/report_json.rs          |  9 +-
 .../specs/2026-07-08-muxsmith-v1-design.md         |  4 +-
 e2e/smoke.spec.ts                                  | 23 ++++-
 locales/de/cli.ftl                                 |  1 +
 locales/de/diagnostics.ftl                         |  2 +-
 locales/en/cli.ftl                                 |  1 +
 locales/en/diagnostics.ftl                         |  2 +-
 src/components/JobRow.vue                          | 13 +++
 src/ipc.ts                                         |  4 +
 16 files changed, 168 insertions(+), 55 deletions(-)
```

---

# Fix round 1 (against `task-3-verdict.md`)

**Status: FIXED.** F1 and F2 applied exactly as the verdict writes them, both
fences verbatim; F3 accepted after independent measurement, not borrowed.
Nothing disputed. Committed as `4e73739`, two files, additive-only.

## F1 - BLOCKING, applied

The verdict's ruling overturns my §6.1 routing, and it is right on the
mechanism: my stated residual net does not exist. I had written "caught only by
`ipc.ts`'s type"; `ipc.ts` is a hand-written mirror with no generation link to
the Rust producer, so nothing caught it. That was a mechanism assertion I did
not run - `feedback_zitat_und_zahl_pruefen`'s class exactly.

Two additions to `crates/muxsmith-core/tests/joblog.rs`, both transcribed from
the verdict's fences and verified present verbatim, once each:

- Required: `#[test] fn panicked_outcome_persists_its_payload_on_the_job_record`,
  inserted immediately before the `/// Step 1, case 2: ...` doc comment (the
  verdict's line 138 anchor; the closing `}` of
  `full_lifecycle_writes_job_and_summary_files` and its blank line precede it).
- Recommended: the `contains_key` presence assertion, inserted directly after
  `assert_eq!(job["duration_ms"], 42);` (the verdict's line 116 anchor), matching
  `JobRecord`'s own field order.

```
$ python3 (fence extractor over the verdict, compared to the installed files)
F1 required test fence present verbatim: True | occurrences: 1
F1 recommended fence present verbatim: True | occurrences: 1
F2 fence present verbatim in run.rs: True | occurrences: 1
```

**Both new checks fired under the verdict's own mutation D** (`JobRecord.panic`
field and its assignment deleted), which is what proves the gap is now closed
rather than merely papered over:

```
test full_lifecycle_writes_job_and_summary_files ... FAILED
  joblog.rs:117: `panic` is always on the wire (D98), null for a job that did not panic:
  {"argv":[...],"duration_ms":42,"errors":[],"exit_code":0,...,"state":"ok","warnings":[]}
test panicked_outcome_persists_its_payload_on_the_job_record ... FAILED
  joblog.rs:165: the persisted record must carry the panic payload (D98), got: {...}
    left: Null
   right: "scripted worker panic for job 0"
```

Restored non-interactively per the hazard this task recorded, with proof in the
same command:

```
$ git checkout -- crates/muxsmith-core/src/executor/joblog.rs
$ sha256sum -c f1-mutD-baseline.sha
crates/muxsmith-core/src/executor/joblog.rs: OK
$ git status --porcelain -- crates/muxsmith-core/src/executor/joblog.rs
[]
$ cargo test -p muxsmith-core --test joblog
test panicked_outcome_persists_its_payload_on_the_job_record ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

13 tests, up from 12 (the sha baseline was taken *before* mutating, so the
restore proof has something to compare against).

## F2 - MEDIUM, applied

`crates/muxsmith-cli/src/commands/run.rs`, `MilestoneState::render`'s doc.

Before:

```rust
    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`,
    /// `Warning`/`Error` (both -> `run-job-notice`), and `Finished` each
    /// render exactly one line; `Progress` renders one line per newly
    /// crossed [`MILESTONES`] threshold (often zero, when still strictly
    /// between two of them).
```

After (the verdict's replacement, verbatim, `render_finished` as a plain code
span per the `proc-48-docsurface-delink` precedent):

```rust
    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`
    /// and `Warning`/`Error` (both -> `run-job-notice`) each render exactly
    /// one line; `Finished` renders one, or two when the outcome carries a
    /// worker panic (D99, see `render_finished`); `Progress` renders one
    /// line per newly crossed [`MILESTONES`] threshold (often zero, when
    /// still strictly between two of them).
```

Conceded without reservation: I restated two doc comments in this same file for
exactly this reason (report §3 item 4) and missed the caller's, three lines above
the call site I changed. `proc-normative-count-recomputed` trigger 2 fired and I
did not sweep the callers.

## F3 - LOW, accepted after measuring it myself

The verdict's mechanism claim decides which assertion shape F1 ships, so it is a
load-bearing borrowed claim and I ran it rather than accepting it (standalone
crate, `serde_json` 1):

```
with["panic"]    == Null : true
without["panic"] == Null : true
with.contains_key    : true
without.contains_key : false
```

Confirmed: my §6.1 recommendation `assert_eq!(job["panic"], serde_json::Value::Null)`
is vacuous - `serde_json::Value`'s `Index` returns `Null` for a missing key, so
it passes identically whether the key is present-and-null or gone. Had it
shipped, F1 would have closed on paper only. The verdict's `contains_key` form
is the discriminating one, and mutation D above shows it discriminating.

## Verification (fix round)

Full Rust bar, foreground, on the committed state:

```
$ cargo fmt --all --check                                       -> exit 0
$ cargo clippy --workspace --all-targets -- -D warnings         -> exit 0
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps     -> exit 0   (F2's gate)
$ cargo test --workspace                                        -> exit 0
   ok binaries: 39   FAILED: 0
   test full_lifecycle_writes_job_and_summary_files ... ok
   test panicked_outcome_persists_its_payload_on_the_job_record ... ok
   test commands::run::tests::finished_panicked_renders_two_lines_without_na ... ok
   test executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled ... ok
$ find . -name '*.snap.new'  -> 0     $ git status --porcelain -- '*.snap'  -> 0
```

Frontend bar not re-run, and the reason is measurable rather than assumed: this
round changed no file the frontend bar reads -
`git diff --name-only -- src e2e locales docs src-tauri` (excluding the other
writer's `decision-ledger.yaml`) returns nothing.

**Additive-only, proved with a fired control.** Removed assertion-shaped lines
in `4e73739`: **0**; the same pattern over Task 2's commit `9b2843f` returns
**12**, so the zero is a fired zero. The commit's only removed lines are the five
lines of F2's old doc block:

```
$ git show 4e73739 -U0 | grep -E "^-[^-]"
-    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`,
-    /// `Warning`/`Error` (both -> `run-job-notice`), and `Finished` each
-    /// render exactly one line; `Progress` renders one line per newly
-    /// crossed [`MILESTONES`] threshold (often zero, when still strictly
-    /// between two of them).
```

No existing assertion, fixture value, helper, import or dependency touched.

## Commit

```
commit 4e73739fbfff9cc094c66997fa0aef3112c7109c
    executor+cli: pin the persisted panic payload; correct render's line-count doc (Task 3 fix round, F1/F2)

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 crates/muxsmith-cli/src/commands/run.rs | 11 ++++++-----
 crates/muxsmith-core/tests/joblog.rs    | 33 +++++++++++++++++++++++++++++++++
 2 files changed, 39 insertions(+), 5 deletions(-)
```

`%G?` = `N`, one trailer, no `Claude-Session`, not pushed. Pathspec-scoped
(`git commit -- <the two paths>`) because another writer holds
`docs/decision-ledger.yaml` in this shared index; that file is still unstaged and
unmodified by me after the commit (`git status --short` shows it alone).

## Disputes

None. All three findings hold; F1's and F3's mechanisms were re-measured on my
own instruments and both confirmed.

## For the controller (unchanged from §6, plus one)

The verdict's HARVEST item 2 - the precedence gap between a task step's
pinned-test enumeration and the Tier-2 `tests-ship-with-the-feature-never-after`
rule - is the thing that actually produced F1 on my side. I weighed those two at
the keyboard and resolved toward the plan. Tasks 4, 5 and 6 each carry a
per-task pinned-test enumeration of the same shape, so a controller ruling
before Task 4 would remove the same judgment call from three more dispatches.
