# Task 3 verdict - Plan 9 (worker-panic payload, end to end)

**Reviewer:** independent, did not write this code. Graded against the v1 spec,
the Plan-9 design (D98, D99, D100, D96's amendment-3 rider, section 0 notes 4/5,
section 2, section 5, section 7 item 4, the amendment log), the plan's Task 3
(eleven steps) and Global Constraints, and the four house-knowledge YAMLs.
Commit under review: `9e5e112`, 16 files. Every claim below is measured with my
own instruments (appendix); every mutation was restored non-interactively and
proved.

## 1. Verdict

**NEEDS_FIXES** - one BLOCKING, one MEDIUM, one LOW.

The implementation itself is accurate work: every character-for-character
contract in this task is byte-exact (I diffed all seven of them mechanically),
the compiler sweep is provably complete, the two extended expectations are
provably strictly stronger, no assertion anywhere was weakened, and the full
verification bar reproduces green on my own runs with every aggregate matching.
The blocking item is a coverage gap the implementer surfaced instead of closing,
which the owner's standing Tier-2 ruling does not permit; the medium item is a
rustdoc the task's own edit falsified three lines above its call site, of exactly
the class that produced amendment 3 in this same plan.

## 2. Findings

### F1 - BLOCKING: the persisted half of acceptance 4 ships with no producer

**Site:** `crates/muxsmith-core/tests/joblog.rs` (no `panic` assertion anywhere);
producer line `crates/muxsmith-core/src/executor/joblog.rs:292`
(`panic: outcome.panic.as_deref(),`).

**Premise, verified myself (not taken from the report):**

- `JobRecord` (`crates/muxsmith-core/src/executor/joblog.rs:160-173`) has 12
  fields. `full_lifecycle_writes_job_and_summary_files` asserts 11 of them
  individually (`joblog.rs:106-127`: `index`, `output`, `argv`, `state`,
  `exit_code`, `warnings`, `errors`, `duration_ms`, `lines`, `started_at`,
  `finished_at`). `panic` is the only field with no assertion.
- No other test in the tree asserts the persisted key. My grep for `"panic"`
  over `crates src-tauri src e2e scripts` (`*.rs`, `*.ts`, `*.vue`) returns
  exactly five lines, all in `executor_events.rs:66` and `report_json.rs:92-95` -
  the `JobEvent` wire and the `run_document`, neither of which is
  `job-<index>.json`. Control for the same grep on a key that *is* asserted
  (`"duration_ms"`) returns hits, so the empty persisted-side result is a real
  absence, not a malformed pattern.
- No type-level backstop exists either. `get_job_log` returns a raw
  `serde_json::Value` (`src-tauri/src/run.rs:510-511`); `RunHistory.vue` reads
  only `record.lines`; the `ts-rs` binding surface covers the profile model only
  (`grep 'feature = "ts"' crates/muxsmith-core/src/executor/` -> 0; control over
  `src/profile/` -> 22), so `ipc.ts` is a hand-written mirror that verifies
  nothing about the Rust producer.

**Empirical proof of the gap (my mutation D):** I deleted the `panic` field from
`JobRecord` *and* its assignment, then ran the full suite:

```
cargo test --workspace exit: 0
ok binaries: 39  FAILED: 0
```

The workspace stays entirely green with the persisted payload gone. Restored with
`git checkout --`; `sha256sum -c` -> `OK`, `git status --porcelain` -> 0 lines.

**Why this is not acceptable as a disclosed concern.** Tier-2
`tests-ship-with-the-feature-never-after` (`docs/process-conventions.yaml:660`)
states it directly: *"Deferring the test to a later item is not an acceptable
resolution of a missing producer, and neither is recording the gap honestly:
honesty about an uncovered consequence closes a reporting defect, not the
coverage defect underneath it."* Its readable trigger is the sentence the report
actually wrote (report §6.1: "no test asserts the persisted `panic` key" plus a
recommendation to route it to Task 6 or a fix round). Its only carve-out is new
test *infrastructure*, which does not apply: `joblog.rs` already writes records
to a tempdir with an `outcome()` helper and asserts individual keys, so the
existing infrastructure expresses this scenario with no new machinery, no new
dependency and no new file.

Design acceptance 4 names the consequence as user-observable in its own words:
*"its `job-<index>.json` contains `"panic": "<payload>"`"*. The implementer's
counter-argument - that the plan's Step 7 enumerates the pinned tests and an
enumeration in a normative position beats the structural grant - is a real
reading, but it loses here: the plan's own Global Constraints make the four
house-knowledge files ground truth alongside the plan, a Tier-2 standing rule is
not the *silence* the latitude grant fills, and this particular Tier-2 entry was
created by the owner **at this plan's own approval gate**, overturning a
controller routing of precisely this shape (amendment 1, ruling A). Nor is the
addition a "new test scenario" the plan bars: an additive assertion, and a block
in an existing test file, are named in scope by
`latitude-carveout-zero-content-structural-forks`.

**The exact addition, additive-only.**

Required. Insert into `crates/muxsmith-core/tests/joblog.rs` immediately before
line 138 (the doc comment `/// Step 1, case 2: a collision on ...`), i.e. after
the closing `}` of `full_lifecycle_writes_job_and_summary_files` at line 136 and
its blank line 137, followed by one blank line:

```rust
/// Acceptance 4's persisted half (D98): the panic payload a recovered
/// outcome carries reaches `job-<index>.json` as the `panic` key, so a
/// panicked run stays triageable from the record once the run is over -
/// the record is what replaced the deleted stderr line.
#[test]
fn panicked_outcome_persists_its_payload_on_the_job_record() {
    let dir = tempfile::tempdir().unwrap();
    let runs_root = dir.path().join("runs");
    let specs = vec![spec(&["--output", "out.mkv", "a.srt"], "out.mkv")];

    let mut logger = RunLogger::create(&runs_root, "20260710-120000Z", &specs).unwrap();
    let mut panicked = outcome(JobState::Failed, None);
    panicked.panic = Some("scripted worker panic for job 0".to_string());
    logger.on_event(&JobEvent::Finished {
        index: 0,
        outcome: panicked,
    });
    let written_dir = logger.finish(&serde_json::json!({})).unwrap();

    let job: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(written_dir.join("job-0.json")).unwrap())
            .unwrap();
    assert_eq!(
        job["panic"], "scripted worker panic for job 0",
        "the persisted record must carry the panic payload (D98), got: {job}"
    );
}
```

I ran this addition myself, both ways, so the fix round does not have to:

```
A) on the correct tree:
   test panicked_outcome_persists_its_payload_on_the_job_record ... ok
   test result: ok. 13 passed; 0 failed; ...
B) with mutation D applied (JobRecord.panic removed):
   test panicked_outcome_persists_its_payload_on_the_job_record ... FAILED
     left: Null
    right: "scripted worker panic for job 0"
   test result: FAILED. 12 passed; 1 failed; ...
```

Uses only what the file already imports (`JobOutcome`/`JobState`/`JobEvent`,
`RunLogger`, `tempfile`, `serde_json`); `serde_json::json!({})` as the finish
document matches the sibling at `joblog.rs:179`. **Additive-only: no existing
assertion, fixture value, helper or test is touched.**

Recommended in the same round (one line, pins D98's *always on the wire* half for
the null case at the site that asserts every other key). Insert after
`crates/muxsmith-core/tests/joblog.rs:116` (`assert_eq!(job["duration_ms"], 42);`),
matching `JobRecord`'s own field order:

```rust
    assert!(
        job.as_object().unwrap().contains_key("panic"),
        "`panic` is always on the wire (D98), null for a job that did not \
         panic: {job}"
    );
```

`contains_key` is load-bearing here - see F3.

### F2 - MEDIUM: `MilestoneState::render`'s rustdoc states a count this task falsified

**Site:** `crates/muxsmith-cli/src/commands/run.rs:341-345`, current text:

```rust
    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`,
    /// `Warning`/`Error` (both -> `run-job-notice`), and `Finished` each
    /// render exactly one line; `Progress` renders one line per newly
    /// crossed [`MILESTONES`] threshold (often zero, when still strictly
    /// between two of them).
```

`Finished` no longer renders exactly one line: D99 makes it render two whenever
the outcome carries a panic, and this commit is what made that true (the arm at
`:385-387` now forwards `render_finished`'s vector instead of wrapping a single
`String`). The doc block was not touched by the commit - `git show 9e5e112 --
crates/muxsmith-cli/src/commands/run.rs` opens its first hunk at `@@ -383,7
+383,7 @@`, four lines below.

This is the same defect class as Task-2 review MEDIUM-1, which the owner
escalated into amendment 3 and Tier-1
`core-docs-name-callers-illustratively-never-exclusively`: a doc passage
falsified by the very change that documents it. It is also
`proc-normative-count-recomputed`'s trigger 2 (a count-word describing a set the
change extended), and the implementer applied exactly this reasoning to two other
doc comments in the same file (report §3 item 4) while missing the caller's,
three lines above the call site it edited. In a **listed** file, so
`latitude-carveout-zero-content-structural-forks`'s named-in-scope clause
("repairing a reference which the task's OWN enumerated edit invalidated") covers
the repair without routing.

**The exact fix** - replace `run.rs:341-345` with:

```rust
    /// Renders zero or more human-mode lines for one [`JobEvent`]. `Started`
    /// and `Warning`/`Error` (both -> `run-job-notice`) each render exactly
    /// one line; `Finished` renders one, or two when the outcome carries a
    /// worker panic (D99, see `render_finished`); `Progress` renders one
    /// line per newly crossed [`MILESTONES`] threshold (often zero, when
    /// still strictly between two of them).
```

`render_finished` deliberately as a plain code span rather than an intra-doc
link, per the `proc-48-docsurface-delink` precedent the amendment-3 rider cites,
so the pending private-items rustdoc flag cannot turn it into a new link surface.
Doc-only; no test, no string, no behavior.

### F3 - LOW: the report's recommended assertion would have been vacuous

**Site:** report `.superpowers/sdd/plan-9/task-3-report.md` §6.1, which
recommends `assert_eq!(job["panic"], serde_json::Value::Null);`.

`serde_json::Value`'s `Index` impl returns `Null` for a **missing** key on an
object, so that assertion passes identically whether the key is present-and-null
or absent entirely. Measured, not recalled (standalone crate, appendix):

```
with["panic"]    == Null : true
without["panic"] == Null : true   <-- the assertion cannot fail
with.contains_key    : true
without.contains_key : false
```

Had the fix round executed the recommendation verbatim, it would have shipped a
green test that does not detect the removal it exists to detect - closing F1 on
paper only. F1's addition above uses a payload equality (and `contains_key` for
the null-side line) for this reason. No code defect; recorded because a borrowed
recommendation was about to be executed as fact.

## 3. The eight dimensions

**1. Contract compliance - clean.** Every fence in this task is byte-exact
against the design, checked by extraction and comparison, not by eye:

| Contract | Result |
|---|---|
| D98 field + doc fence -> `job.rs:55-59` | identical |
| D96 amendment-3 rider fence (25 lines) -> `queue.rs:328-352` | identical; fire: the same comparator on `HEAD~1`'s 21-line block -> False |
| D99 en `worker-panicked` -> `locales/en/diagnostics.ftl:80` | identical, 1 hit |
| D99 de `worker-panicked` -> `locales/de/diagnostics.ftl:87` | identical, 1 hit (`ä`/`ü`/`ß` exact) |
| D99 en `run-job-panicked` -> `locales/en/cli.ftl:36` | identical, 1 hit, after `run-job-failed` |
| D99 de `run-job-panicked` -> `locales/de/cli.ftl:43` | identical, 1 hit, same position |
| D100 span fence -> `JobRow.vue:61-64` | identical after dedent (indentation is the ruled-implementer-owned part) |
| S-1 WorkerPanicked replacement -> spec `:289` | identical, 1 hit; the `EmptyRawProperty` row correctly **absent** (0 in spec, control: 1 in the design) |
| S-2 8.4 fragment -> spec `:410` | present verbatim, exactly 1 occurrence; the un-extended predecessor gone |

Fire for the fence comparator: a one-character perturbation of the en fence
compares False against the installed line. The `errors` token is byte-identical
to `HEAD~1` (`queue.rs:461` vs pre-state `:453`, string compare True).
`delete_partial_failed` and the four ruled-discarded executor failures are
untouched: my count of `delete_partial|create_dir_all|remove_dir_all` on
`git show 9e5e112` is 0, with `job.rs` carrying 9 live `delete_partial`
occurrences as the known-present control, and `executor/spawn.rs` is not in the
commit. `panic` carries no `skip_serializing_if`. Catalog obligations complete
and recounted from D99's enumeration: 3 of 3 (allowlist entry, fixture args,
`WorkerPanicked` row), matching the 3 hunks in `catalog_completeness.rs`.

**2. The Step-2 transcription - clean.** My own extractor (locating the
contiguous `///` run above the sole `pub fn run_batch(` anchor, and the fence
after the rider's transcription sentence) reports installed == fence, 25 lines,
max width 75 on both sides, every line starting `///`. The rest of the function
is byte-identical: `pub fn run_batch(` through its closing brace, 22 lines,
sha256 `ddc5bd94b5ac1196` on both `HEAD~1` and `HEAD`; injecting one trailing
space makes the comparator report False. No `src-tauri` file entered the commit
on the amendment's account, as the rider decided.

**3. The compiler sweep - provably complete.** 9 `JobOutcome` struct literals
exist in the tree (`job.rs:108/130/194`, `queue.rs:316/457`,
`tests/report_json.rs:13`, `tests/joblog.rs:24`, `tests/executor_events.rs:55`,
`cli/commands/run.rs:486`); 8 were compiler-flagged and set `panic: None`,
`recover_panicked_worker` (`queue.rs:457`) is the ninth and the only `Some`-setter
in non-test code (`grep 'panic: Some'` -> exactly `queue.rs:465`; the one test-side
`.panic = Some(...)` is this task's licensed CLI fixture at `run.rs:736`). No
constructor can hide from the debug/linux build: `JobOutcome` derives no
`Default` and no `impl Default` exists, so no `..Default::default()` construction
is possible; the only OS-gated code in the workspace is in
`capability/runtime.rs` (`target_os` windows/macos/linux, 6 sites) and touches no
`JobOutcome`; the only other non-test `cfg` in a constructor file
(`run.rs:275/279`, `debug_assertions`) is a two-line `runs_root` binding. Every
`JobOutcome` mention in the workspace (7 files, 38 lines) was enumerated and
accounted for. No `src-tauri` site exists. TS side independently confirmed: my own
`npx tsc --noEmit -p e2e/tsconfig.json` exits 0, exactly 4 outcome-shaped
literals exist in `e2e/` and all 4 carry `panic: null`, and no
`as JobOutcome`/`as RunJobEntry`/`as unknown as` cast exists that could bypass
the check. No `ts-rs` regeneration was owed (executor types are outside that
surface).

**4. Test integrity - clean, both extensions strictly stronger.** Zero removed
assertion-shaped lines in the whole commit (`git show 9e5e112 -U0 | grep '^-[^-]'`
filtered for `assert|#[test]|#[ignore]|expect(|panic!|should_panic`); the same
pattern over Task 2's commit `9b2843f` returns 12, so the zero is a fired zero.
`executor_events.rs`: 5 raw-string literals before and after, only the fifth
changed, and `post.replace(',"panic":null','') == pre` is True - the previously
asserted bytes survive exactly and the assertion now pins one more key.
`report_json.rs`: 4 removed lines, and every one is an added line minus
`, "panic": null` (True); `assert_eq!`/`assert!`/`#[test]` counts identical
before and after (5/4/3). The queue test's pre-existing prefix-token assertion
(`.starts_with(DiagCode::WorkerPanicked.key())`, `queue.rs:817-824`) is intact,
which is what Step 7(a)'s "AND the unchanged prefix token" half asks for; the
payload assertion was added beside it.

**5. Latitude, both forms.** No forward violation found: nothing in the commit
resolves a closed fork, and the four adjudications below all land in scope. The
inverse form found one instance, and it is F1: the implementer routed a
one-assertion coverage addition that the owner's own Tier-2 ruling says must ship
with the feature. The three other disclosed concerns were correctly *not* routed.
Two further judgment calls are also in scope: the two doc restatements of report
§3.4 (`render_finished`'s and the queue test's, both references the task's own
edit invalidated, both in listed files), and the composed licence-doc prose,
which has no fence and therefore licensed wording.

**6. House dimension - conformant except F2.** `latitude-carveout-zero-content-structural-forks`'s
file-vs-within-file boundary is applied correctly: the three EXEMPLARY-entry files
carry no within-file qualifier, so work inside them is in scope, and the four
zero-outward-effect conditions hold for what was done there (no API/symbol
surface, the data-format change is the design's own mandate, verification
strengthened not weakened, nothing user-visible). The `recover_panicked_worker`
licence-doc rewrite (`queue.rs:423-432`) carries all four content requirements
D98 fork 12 states - payload passed through as data, rendered at presentation
time through the catalog's `$detail`, the spec's normal path rather than an
exception, core still authoring no user-facing prose with the
`core-37-prose-free-core` citation - and adds the token-unchanged fact. Its form
matches its neighbours (same `--` dash convention as the block's own line 413,
same `[`JobOutcome`]`/`[`DiagCode`]` link style), and it satisfies
`core-docs-name-callers-illustratively-never-exclusively`: "each surface renders
it at presentation time" names no caller exclusively. F2 is the one house miss.
No house YAML was edited by the commit (0 of 16 files); the ledger modification
in the working tree is the controller's own in-flight write and I left it
untouched. Typography clean: 0 AI-tell glyphs across the commit's 166 added lines
(control: injecting one em dash makes the same pattern report 1), German
orthography present on 2 added lines. Commit hygiene: one `Co-Authored-By`
trailer, no `Claude-Session`, `%G?` = `N`, no manifest touched.

**7. The no-work-needed check - every premise run, all hold.**

- Report §2.6, "the `lib.rs:24` comment stays true": run. 23 `eprintln!` sites
  print `MKVMERGE_SKIP_MARKER` (7 core tests + 13 cli + 3 src-tauri) against the
  comment's "~21"; none is in `crates/muxsmith-core/src`, so the deleted call was
  never one of them and the comment's historical claim is unaffected.
- Report §2.11, "the import-removal sweep has no subject": run. 0 removed `use`
  lines in this commit; the same pattern over `9b2843f` returns 7.
- Report §2.10, the must-not-touch absences: re-run with my own counts and a
  known-present control for each (above).
- Report §5 concern 4, "no consumer depends on key order": run. The only
  order-sensitive JSON assertion touching a `JobOutcome` is
  `executor_events.rs:66` (updated); `report_json.rs` compares
  `serde_json::Value` (order-independent); `src-tauri/src/run.rs:1476` writes a
  synthetic fixture; the frontend has no `JSON.stringify` equality on outcome
  shapes.
- Report §6.1, "caught only by `ipc.ts`'s type": **refuted**. Nothing catches it
  (F1, mutation D).
- One further absence I checked because D99 removes the thing it refers to: no
  residual reference to the deleted stderr line survives anywhere -
  `application log|Anwendungsprotokoll|stderr line|logged here for triage` over
  `crates src src-tauri locales e2e` and the v1 spec returns only an unrelated
  `pipeline.rs:27` hit, with the design file as the known-present control.

**8. Verification quality - reproduces green, every aggregate recomputed.** My
own foreground runs on the committed tree: `cargo fmt --all --check` 0;
`cargo clippy --workspace --all-targets -- -D warnings` 0; `cargo test
--workspace` 0 with **39** `test result: ok` binaries and **0** FAILED, the four
acceptance-4 Task-3 emitters named in the output; `pnpm lint` 0; `pnpm build` 0;
`pnpm check:i18n` 0 (41 source files / 211 catalog ids / 19 IpcError codes / 22
help ids x 2 locales / 0 unused / 1 other locale vs 7 en catalogs - identical to
the report); `pnpm test:e2e` 0, **62 passed**. Insta churn: 0 `*.snap.new`, 0
modified snapshots, against 77 snapshots present. Core-stdio absence check with
my own fire and control: post-state exactly **1** hit (the `lib.rs:24` comment,
zero call sites); fire on the committed pre-state via `git grep` -> **2** hits
including the deleted `queue.rs:441` call; control over `crates/muxsmith-cli/src`
-> **6** files. File-count aggregate recomputed from the plan's Files list: 13
named entries + 3 through the EXEMPLARY compiler entry = 16, matching the commit.

Three guards whose pass is an absence I fired myself, with my own mutations, and
restored each with `git checkout --` plus a `sha256sum -c` proof:

- **A** (fixture param renamed `detail` -> `detai`):
  `every_diag_code_renders_without_leftover_placeholders` FAILED with
  `worker-panicked: ... {$detail}. ...` leaked. Restored `OK`.
- **B** (`"run-job-panicked"` dropped from `ALLOWLISTED_CLI_KEYS`):
  `every_cli_ftl_key_is_a_diag_code_or_allowlisted` FAILED with
  `cli.ftl key(s) wired to neither a DiagCode nor the allowlist:
  ["run-job-panicked"]`. Restored `OK`.
- **C** (`run-job-panicked` split into its own arm missing the `output` arg):
  the same test FAILED with
  `run-job-panicked: [1/3] {$output} ... failed (worker panicked)`. This is the
  one that settles adjudication 3 empirically - the joined arm's args really are
  consumed for this key and the leak check really renders it. Restored `OK`.

Final tree state: `git status --short` shows only `docs/decision-ledger.yaml`
(the controller's pre-existing 14-line insertion, untouched by me, verified
identical at start and end); all four files I mutated report 0 lines modified;
`HEAD` is still `9e5e112`; the Rust bar re-runs green after restoration (fmt 0,
clippy 0, test 0 / 39 ok / 0 failed).

## 4. The four adjudication questions

**Q1 - extending the two existing serialization expectations: IN SCOPE, correctly
not routed.** Three independent grounds, all verified. (a) The edits are provably
strictly additive, not a weakening: every previously asserted byte survives
(`post.replace(',"panic":null','') == pre` True for the string literal; every
removed JSON line equals an added line minus `, "panic": null`), and both
assertions pin strictly more afterwards - so the carve-out's "verification never
weakened" condition is satisfied in the direction it was written to protect.
(b) There was no option set to route. D98 forbids `skip_serializing_if` and
mandates the field always on the wire, so an exact-serialization assertion
*must* change, and exactly one value is consistent. `proc-latitude-clause-boundary`
asks for a memo of options with costs and a recommendation; a fork with one
member produces no such memo. (c) The design itself anticipates existing tests
changing on this account: acceptance observable 1 says the existing suites "pass
unchanged **except where D102's ordering and D98's field are the designed
deltas**". The files are Files-list members through the EXEMPLARY entry, which
carries no within-file qualifier, so the file-level boundary puts work inside
them in scope. Disclosing it was still right.

**Q2 - `render_finished` returning `Vec<String>`: correct reading of D99.** D99
fixes both properties simultaneously - the branch sits *inside* `render_finished`
("`render_finished`'s `JobState::Failed` arm branches on `outcome.panic`") and
the `Finished` arm yields two lines ("returns two lines instead of one"). No
`-> String` shape satisfies both: joining with `\n` yields one element, not two
lines by the milestone renderer's own contract (and the pinned test asserts
`lines.len() == 2`); branching in `render` instead would duplicate the same
condition and contradict D99's placement sentence. The chosen shape is the
minimal one and it matches `render`'s own return type, which was already
`Vec<String>`. No sanctioning was needed: `render_finished` is private with
exactly one call site (`grep render_finished` -> definition `run.rs:402`, call
`run.rs:386`, nothing else in the workspace), so the carve-out's first
zero-outward-effect condition - nothing outside the change can see, import or
call it - holds. Not an interface change in the sense the question asks about.

**Q3 - `run-job-panicked` joining the existing fixture arm: acceptable, and the
D99 fence is not lost.** D99's obligation prescribes the *args*
(`[("index","1"),("total","3"),("output","/out/movie.mkv")]`), not the arm shape,
and the joined arm produces exactly that `vec!`. The file's own house pattern
groups identical fixture sets, with three prior instances (`"validate-ok" |
"mkvmerge-not-found" | "mkvmerge-query-failed" | "run-joblog-unavailable" |
"run-signal-handler-unavailable"` at `:221-225`, `"identify-failed" |
"identify-not-media"` at `:238`, `"run-joblog-written" | "run-joblog-incomplete"`
at `:290`), so this is `latitude-carveout-zero-content-structural-forks`'s
"extending an unbroken local pattern". Textual visibility is not lost in any way
that matters, because the args are not documentation here - they are executed:
`every_cli_ftl_key_is_a_diag_code_or_allowlisted` (`:347-358`) renders *every*
allowlisted key through `render_and_find_leaks` with `allowlisted_cli_key_args(k)`,
and the function is exhaustive-by-panic (`other => panic!` at `:291`), so a
missing arm cannot go silent. My mutation C proves the args are live for this key
specifically. A separate arm would have been a fourth verbatim duplicate of the
same three pairs - the deviation, not the conformance.

**Q4 - field position: right call, nothing constrains it.** Nothing in the v1
spec, the design or a consumer fixes the position: D98's fence writes the field
and its doc but never places it; the wire-contract memo names surfaces, not
order; the spec's execution section (`:319`) describes the persisted job log
without a field list. JSON objects are unordered by definition, `serde_json::Value`
equality is order-independent, `ipc.ts` interfaces are unordered, and the only
order-sensitive assertion in the tree touching a `JobOutcome`
(`executor_events.rs:66`) was updated additively. The position chosen is also the
locally-patterned one: `panic` last in `JobOutcome` (after `duration_ms`), after
`duration_ms` in `JobRecord` - which keeps the outcome-derived fields contiguous
ahead of the accumulator-derived `lines`/`started_at`/`finished_at` - and the
same spot in `ipc.ts`, preserving the file's stated Rust-field-order mirroring
(`src/ipc.ts:1-12`). Confirmed correct.

## 5. The required ruling

**Does the persisted half of acceptance 4 need an assertion in THIS task? YES.**

Premise verified independently and stated in F1: `joblog.rs` asserts 11 of
`JobRecord`'s 12 keys and `panic` is the sole exception; no other test in the
tree covers the persisted key; and mutation D proves the entire workspace stays
green (39 ok, 0 failed) with the persisted field deleted outright. The
implementer's stated residual safety net - "caught only by `ipc.ts`'s type" - does
not exist: `ipc.ts` is a hand-written mirror with no generation or verification
link to the Rust producer (`ts-rs` covers the profile model only, 0 attrs in
`executor/`), and `get_job_log` returns an untyped `serde_json::Value`.

The plan's Step-7 enumeration does not carry this. Tier-2
`tests-ship-with-the-feature-never-after` is ground truth alongside the plan by
the plan's own Global Constraints, its trigger is the sentence the report wrote,
its only carve-out (new test *infrastructure*) does not apply because
`joblog.rs`'s existing harness expresses the scenario as-is, and the owner
created it at this plan's approval gate by overturning exactly this routing
shape. The addition is not a "new test scenario" under the plan's test-scope
constraint either - an additive assertion and a new block in an existing test
file are named in scope by `latitude-carveout-zero-content-structural-forks`.

**Exact addition and site:** given in full in F1 -
`crates/muxsmith-core/tests/joblog.rs`, new `#[test] fn
panicked_outcome_persists_its_payload_on_the_job_record`, inserted immediately
before line 138, plus the recommended one-line `contains_key` presence assertion
after line 116. **Additive-only: yes** - no existing assertion, fixture value,
helper, import or dependency is touched, in either part. I executed the addition
against both the correct tree (green) and the field-removed tree (red, with the
pasted failure) before naming it, so the fix round can apply it as written.

**Do not use the report's `assert_eq!(job["panic"], serde_json::Value::Null)`** -
it is vacuous (F3, measured).

## 6. Evidence appendix

Scratch root, all instruments mine, none the implementer's:
`/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t3rev-independent/`

| Instrument | Purpose |
|---|---|
| `extract_rustdoc.py` | locates the `///` run above the `pub fn run_batch(` anchor and the rider fence, diffs them; fired against `HEAD~1` |
| `fluent_fences.py` | extracts D99's four `ftl` fences, D98's field fence and D100's `html` fence from the design and compares to the installed lines; carries its own perturbation fire |
| `queue_pre.rs` | `HEAD~1` copy of `queue.rs` for the signature+body byte comparison (sha256 `ddc5bd94b5ac1196` both sides) |
| `stdio_post.txt`, `stdio_pre.txt`, `stdio_ctl.txt` | core-stdio absence check, its fire and its known-present control |
| `clippy.log`, `test.log`, `test_final.log`, `lint.log`, `build.log`, `i18n.log`, `e2e.log` | my own foreground bar runs, before and after the mutations |
| `test_mutD.log` | the workspace suite under mutation D (persisted field removed): 39 ok, 0 failed |
| `vac/` | standalone crate proving `serde_json::Value` index-on-missing-key returns `Null` (F3) |
| `catalog_before.sha`, `joblog_before.sha`, `ruling_before.sha` | restoration proofs for mutations A/B/C, D and the proposed-fix trial |
| `added.txt` | the commit's 166 added lines, for the typography sweep and its em-dash fire |

Mutations A, B, C, D and the proposed-fix trial were each restored with
`git checkout -- <path>` (non-interactive, per the hazard this task hit) and each
verified with `sha256sum -c` -> `OK` plus `git status --porcelain` -> 0 lines.
Final whole-tree proof in dimension 8.

## 7. HARVEST

**Patterns observed.**

1. **An absence-shaped assertion over a dynamic map needs `contains_key`, never
   `== Null`.** `serde_json::Value`'s `Index` returns `Null` for a missing key,
   so `assert_eq!(v["k"], Value::Null)` cannot fail on removal - the exact defect
   the assertion is written to catch. The same shape exists in every dynamic-map
   test idiom (`v["k"].is_null()`, JS `obj.k === null`, Python `d.get(k) is
   None`). This is a **new house-knowledge candidate** and it is readable-trigger
   shaped: you are asserting a key's *null* value on a parsed document. Handle:
   assert the key's presence separately, or assert a non-null value. It nearly
   shipped here as a controller-routed recommendation.
2. **A precedence gap the plan does not resolve, and it will recur.** Global
   Constraints name the four house YAMLs "ground truth alongside" the design and
   plan, without ordering them against each other. F1 is the first place they
   disagree: the plan's Step-7 enumeration reads as normative, Tier-2
   `tests-ship-with-the-feature-never-after` says the coverage ships regardless,
   and the implementer resolved it toward the plan. **Route a controller ruling
   before Task 4**, because Tasks 4-6 all carry per-task pinned-test enumerations
   with the same shape.
3. **Over-restriction watch (the carve-out asks for this explicitly): one stop
   the boundary forced that it should have covered.** The implementer stopped at
   a one-assertion additive test extension - precisely what the carve-out's
   "additive, pattern-conforming test extensions ARE covered" clause permits -
   because the plan's Step-7 enumeration read as the winning normative statement.
   The boundary text is right; what failed is that a *task step's* enumeration
   and a *Tier-2 rule* were weighed by the implementer at the keyboard. Fixing
   item 2 fixes this.
4. **The amendment-3 doc-truth class recurred inside the very task that fixed
   it** (F2): the caller's doc restating the callee's contract went stale while
   the callee's own doc was correctly restated. Handle for Tasks 4-7, and it is
   greppable: when a change alters what a function returns or how many outputs an
   arm produces, sweep the same file for count-words and "exactly"/"each"/"one
   line" near the changed symbol **and in its callers' docs**, not only the
   changed function's own block.
5. **The report's own numbers held up.** Every aggregate I recomputed matched
   (39 test binaries, 62 e2e, 6 control files, 1 stdio hit, 4 e2e fixture
   literals, 0 snapshot churn, 3 catalog edits, 16 files = 13 + 3). No fabricated
   figure, no unattributed quotation. The one factual error in the report is the
   coverage claim in §6.1 ("caught only by `ipc.ts`'s type"), which is a
   reasoning error about a mechanism, not a measurement error - and it is the
   class `feedback_zitat_und_zahl_pruefen` names: a mechanism assertion that was
   not run.

**What Tasks 4-7 must carry.**

- **Task 4** (`EmptyRawProperty`): S-1's `EmptyRawProperty` row is still absent
  from the spec (verified: 0 rows now, 1 in the design fence) - Task 3 correctly
  left it. `empty-raw-property` is a `DiagCode`, so its catalog obligation lands
  in `fixture_args` (`catalog_completeness.rs:~130-152`), **not** in
  `ALLOWLISTED_CLI_KEYS`; the placeholder-leak guard is fire-proven live
  (mutations A/C), so the fixture row is a real check, not a formality.
- **Tasks 4-6 writing `e2e/smoke.spec.ts`**: any new `JobOutcome`- or
  `RunJobEntry`-shaped literal must carry `panic`, and
  `npx tsc --noEmit -p e2e/tsconfig.json` is the enumerator (proven live: it is
  what produced the four sites here). Do not hunt by eye.
- **Task 6**: `data-testid="job-panic"` is installed at `JobRow.vue:63` and
  `worker-panicked` resolves through the bundled `diagnostics.ftl` (`pnpm
  check:i18n` green, 211 ids), so the panic-render test's two dependencies are in
  place. Its e2e event is the second licensed deliberately-`Some` fixture.
- **Task 7** (D49, mutate-measure-restore, the highest-risk inheritance):
  a bare `cp` in this shell is aliased interactive and hangs **with the tree
  mutated** - the failure mode is a hung foreground step that reads as slow, and
  a later staging command carrying the mutation into a commit. The controller has
  already mined this into `a-mutate-restore-step-restores-non-interactively-and-proves-it`
  (uncommitted in the working ledger as of this review). I ran five
  mutate-restore cycles this session under that handle and it costs nothing:
  restore with `git checkout -- <path>` (or `command cp -f`), then prove with
  `sha256sum -c` **and** `git status --porcelain -- <path>` in the same command.
  Take a `sha256sum` baseline *before* mutating so the proof has something to
  compare against; the exit status of the restore command alone is not evidence.
- **Controller item, carried from the report and confirmed:** D98's "deletes the
  `eprintln!` at `queue.rs:396`" is a stale citation (it was at `:441` after Task
  2's hoist); D96's rider's `:327-347` was still exact at Step-2 time. Neither is
  load-bearing - both entries instruct locate-by-content, which is what happened -
  but the design carries two line numbers that no longer hold.

---

# Delta verdict - Task 3 fix round (`4e73739`, 2 files, +39/-5)

Resumed original reviewer. Verified at the tree, not from the appended report;
the four adjudications already ruled stay ruled and no settled non-finding is
re-opened. Note on tree state: HEAD has since moved to `760c00a` (a controller
house commit touching only `docs/decision-ledger.yaml` and
`docs/process-conventions.yaml`); `4e73739` is an ancestor and
`git diff 4e73739 HEAD` over the two delta files is empty, so what I graded is
what is on disk. Working tree clean.

## Verdict: APPROVED

Both findings are closed at the file, the delta is in scope, and the new
assertions provably fire on the removal they exist to catch.

**F1 closed, verbatim, at the specified sites, additive-only.** I compared the
installed code against my own verdict's fences by block match, not by eye: the
required test is present exactly once at `crates/muxsmith-core/tests/joblog.rs:143-169`,
the `contains_key` line exactly once at `:117-121`. Both land where I named them -
the presence line immediately after `assert_eq!(job["duration_ms"], 42);` (`:116`),
in `JobRecord`'s own field order; the test immediately before the
`/// Step 1, case 2:` anchor, which the earlier five-line insertion shifted from
138 to 171. The comparator is live (a one-space perturbation of the fence finds
nothing). `joblog.rs` has **zero** removed lines; the whole commit's only removed
lines are the five F2 doc lines; zero removed assertion-shaped lines across the
commit, with the same pattern returning 12 on Task 2's `9b2843f` as its fire.

**Not the green-but-blind variant - satisfied with my own instrument.** I
re-applied my mutation D (delete `JobRecord.panic` and its assignment) and ran
the binary myself:

```
A) delivered tree:  13 passed; 0 failed
B) under mutation D:
   test panicked_outcome_persists_its_payload_on_the_job_record ... FAILED
     left: Null   right: "scripted worker panic for job 0"
   test full_lifecycle_writes_job_and_summary_files ... FAILED
     panicked at crates/muxsmith-core/tests/joblog.rs:117:5:
     `panic` is always on the wire (D98), null for a job that did not panic: {...}
```

Both new assertions fire, and I confirmed the second failure by line number
(`:117`, the `contains_key` line) rather than inferring it - that is precisely
the check F3 existed to force. Restored with `git checkout --`; `sha256sum -c`
-> `OK`, `git status --porcelain` -> 0 lines.

**F2 closed, verbatim, with the link form I specified.** The replacement doc is
present exactly once at `crates/muxsmith-cli/src/commands/run.rs:341-346`, byte-identical
to my fence, and the old five-line block is gone from the file entirely (block
search -> False). `render_finished` is a plain code span, not an intra-doc link,
so no new link surface is created for the pending private-items rustdoc flag.

**Scope: nothing else changed.** Two files, both listed in Task 3's Files list.
No `src-tauri`, no frontend path, no locale, no spec, no house YAML (0), no
manifest (0). One `Co-Authored-By` trailer, no `Claude-Session`, `%G?` = `N`.
Typography clean across the 36 added lines (0 AI-tell glyphs; control: injecting
one em dash makes the same pattern report 1).

**Bar re-run by me, foreground, on the delivered tree:** `cargo fmt --all --check`
0; `cargo clippy --workspace --all-targets -- -D warnings` 0; `cargo test
--workspace` 0 with **39** ok binaries and **0** FAILED; `RUSTDOCFLAGS="-D
warnings" cargo doc --workspace --no-deps` 0 (correctly run, since a rustdoc
block changed); insta churn 0 `*.snap.new` and 0 modified snapshots. The joblog
binary reports 13 passed, up from 12.

**The no-frontend-run reasoning is sound, including its one non-obvious
channel.** `eslint .`, `vue-tsc --noEmit && vite build` and the `test:e2e` chain
(`tsc -p e2e/tsconfig.json`, the two vite harness builds, playwright) read only
`src/`, `e2e/` and their configs. The channel worth checking is
`scripts/check-i18n.mjs`, which *does* parse Rust - but only `src-tauri/**/*.rs`
(`:292`), for the IpcError gating; it never reads `crates/`. The delta touches
`crates/` only and introduces no catalog key, no `$t` and no `renderer.msg` call,
so no frontend check has an input that changed. Skipping them was correct, not a
shortcut.

Task 3 is done.
