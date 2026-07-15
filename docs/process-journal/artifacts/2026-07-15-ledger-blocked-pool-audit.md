# Blocked-pool audit of the decision ledger (2026-07-15)

Read-only audit of every `status: blocked` entry in `docs/decision-ledger.yaml`,
asking one question per entry: **has its `blocked_on` reactivation condition
already cleared, judged against the real state of the repository rather than
against what the entry says about itself?**

Motivation: the blocked pool has no sweep step. An entry is written when work is
deferred, its condition is named, and nothing re-reads the pool when that
condition later clears. One confirmed instance
(`i18n-17-mixed-language-allowed-deferred`, resolved by ADR D39 on 2026-07-14
while still standing at `blocked_on: "pre-1.0 polish / Plan 6"`, since closed in
commit `0b59d42`) prompted this pass. That entry is excluded here.

## 1. Summary

**27 entries audited** (enumerated from the file: 274 total entries, 27 with
`status: blocked`; n-in 27, n-out 27).

| Verdict | Count |
| --- | --- |
| **ALREADY-DONE** | 12 |
| **STILL-BLOCKED** | 12 |
| **UNCLEAR** | 3 |
| **FIRED** (condition cleared, work not done) | 0 |
| **Total** | **27** |

The headline result: **12 of 27 blocked entries are stale** - the deferred work
is visibly finished in the tree, most of it landed by Plan 5.5 (2026-07-12),
Plan 5.6 (2026-07-13) and Plan 5.7 (2026-07-14). That is 44% of the pool. The
i18n-17 case was not an outlier; it was the first one anybody happened to look
at.

No entry landed in FIRED. The pool splits cleanly: either the work rode along
with a plan that happened to cover it (ALREADY-DONE), or the condition names a
future vehicle that genuinely has not started (STILL-BLOCKED), or it names no
observable event at all (UNCLEAR). Nothing sits in the intermediate state of
"condition cleared, work still outstanding" - which is itself a finding: the
reactivation conditions are not what actually drives this work to completion.
Plans do, and the ledger learns about it afterwards, or never.

The 12 STILL-BLOCKED entries are correctly blocked, and five of them
(`exec-36`, `exec-37`, `cli-08`, `core-121`, `gui-26`) were re-pointed from
"Plan 6" to Plans 7/9 earlier today by the scope re-cut (`0b59d42`), so they are
already swept and current.

## 2. Verdict table

| Entry id | `blocked_on` (quoted) | Verdict | Evidence |
| --- | --- | --- | --- |
| `core-56-batch-level-language-fold` | "optional refinement, not required for v1 correctness" | UNCLEAR | Names no event. Work not done: `planner.rs:798-802` still records the per-file scope choice |
| `core-64-donor-group-ordering` | "no golden arbitrates the ordering; pinning left pending" | **ALREADY-DONE** | Golden exists: `crates/muxsmith-core/tests/command.rs:209-291` (`543259b`) |
| `core-66-eager-resolution-discarded-plan` | "tied to detect_source_overwrites pre-finalize design; low priority" | UNCLEAR | Names no event. Still open; tracked as ROADMAP v1.x cosmetic item (`ROADMAP.md:434-435`) |
| `core-115-partition-unresolvable-file` | "idiomacy review (internal)" | **ALREADY-DONE** | Invariant comment landed: `planner.rs:1580-1583` (`403e573`, Plan 5.6 T2) |
| `exec-23-loop-resweep-watcher` | "internal: design change if stronger cancel ever wanted" | UNCLEAR | Condition is a hypothetical want, not an event |
| `exec-36-core-stderr-logging` | "Plan 9 (internal: core/orchestration hoists; re-pointed from Plan 6 by the 2026-07-15 scope re-cut)" | STILL-BLOCKED | Plan 9 not started (`ROADMAP.md:76-99`); re-pointed today (`0b59d42`) |
| `exec-37-panicked-msg-catalog` | "Plan 9 (internal: core/orchestration hoists; re-pointed from Plan 6 by the 2026-07-15 scope re-cut)" | STILL-BLOCKED | Plan 9 not started (`ROADMAP.md:97-99`); re-pointed today (`0b59d42`) |
| `cross-08-minor-findings-deferred` | "internal: pending dev work / owner disposition" | **ALREADY-DONE** | All 6 minors resolved: `3081fc6`, `99b2e34`, `65eef3c`, `0bce722`, + 2 accepted-as-designed |
| `proc-55-correctness-gate-deferred` | "internal: the dedicated correctness/security/perf review not yet run" | **ALREADY-DONE** | Review ran + Plan 5.7 closed 2026-07-14 (`ROADMAP.md:346-357`, `af00947`, `f7a6a01`) |
| `proc-56-idiomacy-fix-wave-deferred` | "internal: triage of the 70 confirmed findings with Şenol" | **ALREADY-DONE** | Triaged 2026-07-12, fix wave closed 2026-07-13 (`ROADMAP.md:299-308`, `0b3149a..a5d506b`) |
| `i18n-12-de-placeable-parity-deferred` | "future check:i18n extension (internal)" | STILL-BLOCKED | `scripts/check-i18n.mjs:52-54,117` still id-set-only; tracked (`ROADMAP.md:499-508`) |
| `i18n-13-cli-key-rename-deferred` | "idiomacy review (internal)" | **ALREADY-DONE** | Key renamed: `locales/en/cli.ftl:23`, `locales/de/cli.ftl:30` (`c877e4f`) |
| `gui-26` | "Plan 7 (internal: help mode + i18n cluster; re-pointed from Plan 6 by the 2026-07-15 scope re-cut)" | STILL-BLOCKED | Plan 7 not started (`ROADMAP.md:44-60`); re-pointed today (`0b59d42`) |
| `ci-09-skipmarker-contract` | "internal - idiomacy review" | **ALREADY-DONE** | Shared const landed: `crates/muxsmith-core/src/lib.rs:28` (`616778d`) |
| `ci-16-mise-not-ci` | "internal - kept until the release stabilizes; removal tracked in ROADMAP v1.x" | STILL-BLOCKED | `ci.yml:82` still uses `jdx/mise-action` unpinned-binary; v1.x item open (`ROADMAP.md:386-392`) |
| `testing-coverage-tooling` | "no measured coverage need in v1 (revisit at v1.x planning)" | STILL-BLOCKED | No coverage tooling in tree; `BUILDING.md:111` still lists it; v1.x planning not started |
| `testing-warning-path-coverage` | "internal: v1.x test work" | **ALREADY-DONE** | Both halves covered: `executor/job.rs:281-285` (`543259b`) + `tests/executor_events.rs:83-113` (`d6725c8`) |
| `testing-concurrencytracker-hygiene` | "internal: before go-public" | **ALREADY-DONE** | `#[doc(hidden)]` at `executor/spawn.rs:257` (`7a2bc15`) |
| `testing-cli-helper-dedup` | "internal: next CLI test file" | STILL-BLOCKED | No new CLI test file since 2026-07-10. **Premise decayed** - see note 5 |
| `testing-comment-fixture-convention` | "second use (trigger-gated, internal)" | STILL-BLOCKED | Exactly one `_comment` fixture (`tests/fixtures/identify/with-attachments.json`) |
| `testing-proptest-assert-locality` | "idiomacy review (internal)" | **ALREADY-DONE** | Zero `prop_assume` in tree; converted in `cc49337` |
| `testing-check-i18n-self-test` | "v1.x test-hardening (internal)" | STILL-BLOCKED | No self-test in `scripts/check-i18n.mjs`; v1.x item open (`ROADMAP.md:472-475`) |
| `cli-08-config-diags-json-ordering` | "Plan 9 (internal: core/orchestration hoists; re-pointed from Plan 6 by the 2026-07-15 scope re-cut)" | STILL-BLOCKED | Plan 9 not started (`ROADMAP.md:102-103`); re-pointed today (`0b59d42`) |
| `cli-16-ctrlc-pin-deferred` | "internal: next Cargo.toml edit" | **ALREADY-DONE** | `ctrlc = "3.5.2"` at `crates/muxsmith-cli/Cargo.toml:14` (`45e941a`) |
| `cli-22-jobs-index-doc-deferred` | "internal: before Plan 5 consumption" | **ALREADY-DONE** | Doc paragraph at `crates/muxsmith-core/src/report/json.rs:106-111` (`42ecc34`) |
| `core-121-planner-seam-and-hoist` | "Plan 9 (internal): the vehicle half is settled ...; the seam INTERFACE stays open as Plan 9's design question" | STILL-BLOCKED | Plan 9 not started (`ROADMAP.md:81-86`); entry already carries its 2026-07-15 occurrence |
| `proc-subagent-commit-trailer-set` | "owner/controller convention call on the next commit-policy touch" | STILL-BLOCKED | No commit-policy content in `docs/conventions.yaml` / `docs/process-conventions.yaml`; no such touch found |

## 3. ALREADY-DONE findings (full reasoning)

These 12 are the actionable set: the deferred work is finished in the tree and
the entry no longer describes reality.

### 3.1 `ci-09-skipmarker-contract`

**Statement:** the skip-marker string is an unenforced cross-file contract
(19 call sites + 1 CI grep, no shared const); hardening with a shared const was
deferred.

**Both readings converge.** The entry's condition ("idiomacy review") is
satisfiable as "the review ran" (it did, 2026-07-12) *and* as "the review's
triage of this item" (it was triaged: the item is a named input in the review
dispatch list at `ROADMAP.md:278`, "skip-marker shared const (T2-m1)"). Either
way the item was carried through, and the fix landed.

**Evidence:** commit `616778d` *"refactor: single source for the mkvmerge skip
marker (T2-m1)"* introduced the const:

- `crates/muxsmith-core/src/lib.rs:28` -
  `pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";`
- All call sites now reference it (`identify_live.rs:36`,
  `mkvmerge_runtime.rs:112,206`, `command_integration.rs:202,288,459`,
  `executor_live.rs:19`, `run_cli.rs:132,284,331`,
  `dry_run_cli.rs:22,85,128,215,337,681`, `run_live.rs:90,161,292,438`,
  `src-tauri/src/lib.rs:646,705,789`).
- The remaining CI-side coupling is now explicit rather than silent:
  `.github/workflows/ci.yml:112` carries
  `# This literal must match muxsmith_core::MKVMERGE_SKIP_MARKER (crates/muxsmith-core/src/lib.rs) byte-for-byte.`

The false-negative risk the entry describes (a reword silently defeating ci-08's
assertion) is closed on the Rust side by construction and documented at the one
remaining string boundary.

### 3.2 `core-115-partition-unresolvable-file`

**Statement:** a fixless affected file is silently dropped from the partition
(`best=None` skip); hardening with an invariant comment or an explicit
"unresolvable" group deferred.

**Evidence:** the invariant comment exists at
`crates/muxsmith-core/src/planner.rs:1580-1583`, inside `partition_for_rule`:

```rust
// `best` is `None` only for a file no candidate resolves even in
// isolation - unreachable in v1: `id` is unique per track, so the id
// discriminator always resolves a single file (task-13 report, D6).
// Skipped defensively rather than fabricating a group without a fix;
// if id-uniqueness ever relaxes, this needs an "unresolvable" group.
```

Landed in `403e573` *"refactor(planner): idiom/dup/yagni/stdlib cleanups (plan
5.6 T2)"*. This is exactly the first of the two options the entry named ("an
invariant comment"), including the forward note about the second option. Named
input at `ROADMAP.md:282` ("partition best=None invariant comment (T13-m1)").

### 3.3 `i18n-13-cli-key-rename-deferred`

**Statement:** the `dry-run-summary` cli.ftl key serves both run and dry-run (a
latent catalog-skimming trap); renaming it to `batch-summary` was deferred
(touches en+de+allowlist+2 snapshots).

**Evidence:** the rename is complete. `dry-run-summary` no longer exists
anywhere; `batch-summary` is live in every place the entry named:

- `locales/en/cli.ftl:23`, `locales/de/cli.ftl:30` (both catalogs)
- `crates/muxsmith-cli/src/commands/mod.rs:129` (call site)
- `crates/muxsmith-cli/tests/catalog_completeness.rs:196,248` (allowlist + param
  fixture)
- Test comments updated at `dry_run_cli.rs:243`, `run_cli.rs:358`

Commit `c877e4f` *"i18n(cli): rename dry-run-summary to batch-summary (T8-m2)"*,
merged via `82e58d8` *"merge: plan-5.6 stream D (T5 CLI idiomacy + batch-summary
rename)"*.

### 3.4 `testing-proptest-assert-locality`

**Statement:** D6 property guards use `prop_assume` where `prop_assert` would
localize a future generator regression; the switch was deferred.

**Evidence:** `prop_assume` does not occur anywhere in the tree (grepped all
`*.rs`; every hit in `prop_language.rs`, `prop_planner.rs`, `prop_matcher.rs` is
`prop_assert`/`prop_assert_eq`). Commit `cc49337` *"refactor(tests):
property-test idiom cleanup (T14 seeds + select/BTreeMap sweeps)"* states it
directly in its message:

> D6's `accepted_suggestion_survives_replan` converts its two `prop_assume!`
> [...]

with the diff showing `-prop_assume!(...)` / `+prop_assert!(...)` on both guards.
Named input at `ROADMAP.md:280-281` ("prop_assume->prop_assert in D6 property
(T14-m1)").

### 3.5 `testing-concurrencytracker-hygiene`

**Statement:** `ConcurrencyTracker` is pure test instrumentation in the public
lib surface; doc-hiding (`#[doc(hidden)]`) or relocating it is deferred until
before go-public.

**Condition fired:** the repo went public 2026-07-10.

**Work done:** commit `7a2bc15` *"docs(core): hide ConcurrencyTracker from
rustdoc (test instrumentation, pre-go-public)"* added, at
`crates/muxsmith-core/src/executor/spawn.rs:255-259`:

```rust
/// Test instrumentation only, not a supported API: hidden from rustdoc
/// (pre-go-public decision); kept `pub` because cross-crate tests consume it.
#[doc(hidden)]
#[derive(Default)]
pub struct ConcurrencyTracker {
```

The first of the two named options (doc-hiding) was taken, with the rationale for
not taking the second (relocation) recorded inline. The commit subject names the
triggering condition explicitly.

### 3.6 `testing-warning-path-coverage`

**Statement:** the exit-1 warning path lacks an output-kept assertion and
`WarningLine`/`ErrorLine` `on_progress` surfacing is untested; deferred to v1.x.

Both named gaps are closed. Note the condition itself ("v1.x test work") has
**not** fired - v1.x has not started. The work landed anyway, via pre-1.0 test
hardening. This is the same shape as i18n-17: the entry is stale regardless of
its condition.

**Gap 1, output-kept assertion** - `crates/muxsmith-core/src/executor/job.rs`
(exit-1 warning test), added by `543259b` *"test: close the audited coverage gaps
(group T, #21)"* (Plan 5.5 T11, `ROADMAP.md:211`). The test now writes the output
first and asserts it survives, and the comment names the exact gap the ledger
entry describes:

```rust
// A real mkvmerge exit-1 mux still writes the output (spec 6, job.rs
// module doc: "warning, output kept"); `finish` only deletes on
// Failed/Cancelled, so the earlier version of this test (no file
// ever written) could not tell "kept" apart from "never produced".
std::fs::write(&output, b"muxed with warnings").unwrap();
...
assert!(
    spec.output.exists(),
    "exit-1 output must be kept, not deleted"
);
```

**Gap 2, WarningLine surfacing** - `crates/muxsmith-core/tests/executor_events.rs`
`output_line_captures_every_non_tick_line_verbatim` (commit `d6725c8`) drives
`run_job` with a collecting `on_progress` closure and asserts the full delivered
sequence including `JobProgress::WarningLine("'x.srt': track ignored.")`.

### 3.7 `cli-16-ctrlc-pin-deferred`

**Statement:** `ctrlc` is pinned to major only (`"3"`) unlike siblings' full-patch
pins; folding it to a full pin is deferred to the next `Cargo.toml` touch.

**Condition fired and work done in the same commit.** `45e941a` *"build: pin rust
1.96.1 (was floating stable) + ctrlc 3.5.2"*, whose message closes the item in as
many words:

> ctrlc was the last major-only dep pin.

Diff: `-ctrlc = "3"` / `+ctrlc = "3.5.2"`. Current state:
`crates/muxsmith-cli/Cargo.toml:14` reads `ctrlc = "3.5.2"`, matching the
`Cargo.lock` pin the entry cited and the full-patch style of its siblings.

### 3.8 `cli-22-jobs-index-doc-deferred`

**Statement:** `jobs[].index` indexes the queue, not files; one doc sentence in
`run_json_document` prevents consumer misreads in Plan 5; deferred until Plan 5
consumes it.

**Condition fired:** Plan 5 ran and closed ~2026-07-10.

**Work done in the consuming commit.** `42ecc34` *"refactor(core): hoist
batch/config/run JSON documents into report::json"* (Plan 5 Task 2) both hoisted
the function (now `run_document` in core, not `run_json_document` in the CLI -
which is why the entry's function name no longer resolves) and wrote the doc
paragraph. At `crates/muxsmith-core/src/report/json.rs:106-111`:

```rust
/// `jobs[].index` indexes the QUEUE (the job spec slice `run_queue` was
/// given: only the files that planned cleanly enough to mux), not the
/// source-file list `batch.files` enumerates. A file skipped because one of
/// its diagnostics is error-severity has no queue entry, so it has no
/// `jobs[].index` at all; do not treat this index as a `batch.files`
/// offset.
```

This is the deferred sentence, and it covers the exact misread the entry
predicted.

### 3.9 `core-64-donor-group-ordering`

**Statement:** with a donor whose assignments mix `None`/`Some`, its group
position is set by its first `Some` (can differ from a literal first-appearance
reading); no golden arbitrates it, left as a conscious choice into Task 12.

**Condition was "no golden arbitrates the ordering" - a golden now does.**
`crates/muxsmith-core/tests/command.rs:209-291`,
`donor_ordering_drop_mode_with_mixed_none_and_some_assignments`, added by
`543259b` *"test: close the audited coverage gaps (group T, #21)"* (Plan 5.5
T11). Its header labels it a golden pin and states the exact semantics the ledger
entry left unpinned:

> Golden pin (gap T-i): drop-mode `--track-order` when `track_id: None`
> assignments are interleaved BETWEEN `Some` ones across the primary and two
> distinct donors [...] (2) a group's index is assigned by first-appearance of a
> `Some` assignment on that source, so a donor hit only by a later rule (donor A,
> rule 3) gets a HIGHER group index than one first hit earlier (donor B, rule 2),
> even though donor A is also referenced (unsuccessfully) by an earlier, `None`
> rule (rule 1).

The assertion pins it concretely:

```rust
assert_eq!(
    track_order,
    Some("0:0,1:0,2:0"),
    "None assignments must be skipped in place; groups indexed by \
     first-Some-appearance (primary=0, e.ac3=1, e.en.srt=2), got {argv:?}"
);
```

A keep-mode counterpart golden follows immediately after (`command.rs:297ff`).
The "conscious choice" is now an asserted contract, which is what the entry was
waiting for.

### 3.10 `cross-08-minor-findings-deferred`

**Statement:** the whole-branch review's 6 Minor items (double-report,
render-fail donor gap, IdentifyError English, TempDir leaks, double file print,
`mkvmerge_found` JSON asymmetry) were recorded for a follow-up rather than
blocking the fix-pass merge.

Checked all six against
`docs/process-journal/artifacts/plan-2-fixes-sdd/FINAL-review.md:83-160` (the
source the entry cites). **Four were fixed; two were "Accept" verdicts at
recording time, so they were never outstanding work.** Nothing in this entry is
still pending.

| Minor | Item | State |
| --- | --- | --- |
| M1 | `{ any: [] }` double-reports `EmptyMatchExpression` + `EmptyMatchList` | **Fixed** - `3081fc6` |
| M2 | donor of a render-failed file escapes `SourceOverwrite` | **Fixed** - `99b2e34` |
| M3 | `IdentifyError` Display emits core-authored English | Accepted as designed (FINAL-review: "Accept; be aware `detail` will always be English") |
| M4 | new tests leak their `TempDir` via `mem::forget` | **Fixed** - `65eef3c` |
| M5 | per-file diagnostics print the file twice in dry-run human mode | **Fixed** - `0bce722` |
| M6 | `mkvmerge_found` schema asymmetry | Accepted as designed (FINAL-review: "schema asymmetry only. Accept") |

Detail on the four fixes:

- **M1** - `3081fc6` *"fix(diag): suppress EmptyMatchExpression when
  EmptyMatchList fires for the node"*. `crates/muxsmith-core/src/profile/validate.rs:80-92`
  now guards the push with `empty_list_here`, implementing precisely the
  suppression the review suggested ("suppress `EmptyMatchExpression` when an
  `EmptyMatchList` already fired on the same rule").
- **M2** - `99b2e34` *"fix(planner): protect chapters donors of render-failed
  files (#7 class closure)"*, plus the completeness comment at
  `crates/muxsmith-core/src/planner.rs:710` ("Completeness (Task 7.6, #7 class
  closure): every donor kind reaches ...") and the back-reference at
  `planner.rs:1141`. Tracked to completion as the pre-1.0 gate "SourceOverwrite
  completeness (Plan-2 FINAL minor M2): DONE 2026-07-12" (`ROADMAP.md:222`).
- **M4** - `65eef3c` *"test: shared support module (FakeIdent, lang) and
  tempdir-leak fix (D18)"*. `mem::forget` no longer occurs in any `*.rs` file in
  the tree.
- **M5** - `0bce722` *"fix(diag): print each filename once in dry-run/run human
  output"*. `crates/muxsmith-cli/src/i18n.rs:98-112` introduces
  `diagnostic_no_file()` and a `show_file` selector choosing `diagnostic-line`
  (file-less) over `diagnostic-line-file`, with the rationale inline: "dry-run/
  run's per-file block prints the filename once in its `dry-run-file` line, so
  repeating it on every diagnostic under it is noise."

Since M3 and M6 were accepted-as-designed rather than deferred, the entry's own
framing ("recorded for a follow-up") over-counts what was ever open. The
follow-up is complete.

### 3.11 `proc-55-correctness-gate-deferred`

**Statement:** the 11 correctness/security/perf findings routed out of the
idiomacy pass are deferred to a dedicated normal-review gate.

**Condition:** "the dedicated correctness/security/perf review not yet run". It
ran on 2026-07-14, and the resulting fixes shipped.

**Evidence** (`ROADMAP.md:335-357`, the gate entry itself):

> **DONE 2026-07-14**: bug-hunt adjudication against current master (1 already
> fixed by Plan 5.6, 4 should-fix, 6 can-wait, 0 refuted, 0 release blockers;
> ci.yml token premise downgraded by live API check - repo default is read) +
> **Plan 5.7 executed and closed** [...] All four task reviews APPROVED,
> whole-branch READY, nine-part gate green after every merge. Archive:
> docs/process-journal/artifacts/plan-5.7-sdd/ (incl. the routed-items
> adjudication verdict).

Commits: `cd5e917` (plan), `9143866` / `6c0a720` / `17ae87c` / `1cf10f9` (the
fixes), `af00947` (close + salvage), `f7a6a01` (journal entry). Archive:
`docs/process-journal/artifacts/plan-5.7-sdd/`.

The deferral vehicle this entry created has been built, run and closed. The six
"can-wait" items were re-deferred to v1.x and the Triggers list as their own
tracked entries (`ROADMAP.md:357`, "Four re-deferrals below (v1.x + Triggers)"),
so they are carried elsewhere and do not keep this entry alive.

### 3.12 `proc-56-idiomacy-fix-wave-deferred`

**Statement:** the 70 confirmed idiomacy findings await triage with Şenol; the
accepted set becomes its own SDD-executed fix-wave plan, with the review scratch
salvaged at that close.

**Condition:** "triage of the 70 confirmed findings with Şenol". Done
2026-07-12. Every downstream commitment in the statement is also discharged.

**Evidence** (`ROADMAP.md:299-308`):

> **Whole-codebase idiomacy review - STATUS 2026-07-12 (EXECUTED, triaged with
> Şenol); FIX WAVE EXECUTED AND CLOSED 2026-07-13** (Plan 5.6, commits
> `0b3149a..a5d506b`: 12 tasks + final fix wave, 64 findings + 13 seeds applied,
> whole-branch verdict READY, zero behavior change except three sanctioned
> interface deltas recorded in ADR D36 [...])

- Triage: done, with the routing recorded at `ROADMAP.md:310-334`.
- Own SDD plan: `docs/superpowers/plans/2026-07-13-plan-5.6-idiomacy-fix-wave.md`.
- Executed and closed: `831fc5c` *"docs: plan-5.6 close - ledger harvest,
  core-90 promotion, ROADMAP consumption"*, journal entry `24d40d4`.
- Review scratch salvaged at the close, as the statement required: `04845df`
  *"salvage: plan-5.6 SDD scratch (briefs, reports, verdicts, review packages,
  progress ledger)"*, archive `docs/process-journal/artifacts/plan-5.6-sdd/`.

## 4. UNCLEAR entries (undecidable from the repo)

Three entries name no observable event, so no repository state can decide them.
All three describe work that is **not** done - they are not stale, they are
unfalsifiable as written.

### `core-56-batch-level-language-fold`

> `blocked_on: "optional refinement, not required for v1 correctness"`

This is a justification for deferring, not a reactivation condition. It names no
event, no vehicle and no date. Nothing that could happen in this repository would
make it evaluate to "cleared" - it is a permanent statement about the item's
value, so the entry can never leave the blocked pool by observation.

The work is not done: `crates/muxsmith-core/src/planner.rs:798-802` still
records the per-file scope choice explicitly ("batch-wide language consistency
with `walk_exact_languages` is not required for v1"), and `resolve_changes`
still validates `changes.language` only at the application point. The item is
carried forward as ROADMAP v1.x candidate "Batch-level settable-language check
(D18 remainder)" (`ROADMAP.md:437-441`), which is where its real tracking lives.

### `core-66-eager-resolution-discarded-plan`

> `blocked_on: "tied to detect_source_overwrites pre-finalize design; low priority"`

Two non-conditions joined. "Tied to [a] design" is a dependency statement, not an
event - the design is current and settled, so the tie will never "clear". "Low
priority" is a ranking, not an observable. Neither half can fire.

The work is not done (the eager resolve remains at `planner.rs:541ff`); it is
tracked as a ROADMAP v1.x cosmetic item, "eager chapters/attachments resolve on
the discarded-plan path (planner.rs:541ff)" (`ROADMAP.md:434-435`).

### `exec-23-loop-resweep-watcher`

> `blocked_on: "internal: design change if stronger cancel ever wanted"`

The condition is gated on a hypothetical preference ("if [...] ever wanted"),
which is exactly the prediction-shaped trigger that under-fires silently: nobody
observes a want. There is no event to watch for, no artifact that would record
it, and no party assigned to decide it. The one-shot watcher is unchanged in
`crates/muxsmith-core/src/executor/queue.rs`; the entry's own statement already
records the current behavior as deliberate and honest, so it is arguably a
settled restraint wearing a `blocked` status rather than a real deferral.

## 5. Note: one STILL-BLOCKED entry with a decayed premise

`testing-cli-helper-dedup` is correctly STILL-BLOCKED on its literal condition,
but its statement no longer describes the tree, so it warrants a look even though
it is not stale enough for ALREADY-DONE.

**Condition not fired:** "internal: next CLI test file". No CLI test file has
been created since the deferral. Creation dates (`git log --diff-filter=A`) for
every file in `crates/muxsmith-cli/tests/`: `catalog_completeness.rs`,
`cli_schema.rs`, `cli_validate.rs` (all 2026-07-08), `dry_run_cli.rs`,
`run_cli.rs` (2026-07-09), `run_live.rs` (2026-07-10, the file whose review
produced this deferral). The trigger has genuinely not fired.

**Premise decayed anyway.** The statement asserts "no shared common module" as
its reason. That is now false, and part of the consolidation happened without the
trigger:

- `crates/muxsmith-cli/tests/support/mod.rs` exists (created `aba7f4f`, extended
  by `0e8d048` *"refactor(cli): mechanical idiomacy fixes (plan 5.6 task 5,
  stream D)"* - Plan 5.6's "sharing the same-crate test helper" routing,
  `ROADMAP.md:317`).
- The fake-stub helper the entry names is consolidated there:
  `fake_mkvmerge_that_fails_queries()`, consumed by `run_cli.rs:498,559` and
  `dry_run_cli.rs:575,634`.
- Still duplicated: `muxsmith()` 3x (`cli_validate.rs:3`, `run_cli.rs:15`,
  `run_live.rs:27`) and `have_mkvmerge()` 3x (`dry_run_cli.rs:11`,
  `run_cli.rs:19`, `run_live.rs:31`).

So the entry describes a 3-way gap where a partially-closed 2-way gap now sits,
and its stated blocker ("no shared common module") has been removed. The
remaining work is smaller and cheaper than the entry implies.

## 6. Method and limits

- **Enumeration:** all 27 entries were extracted programmatically from
  `docs/decision-ledger.yaml` by locating each `status: blocked` line and walking
  back to its owning `- id:` anchor (274 entries total, 27 blocked). n-in 27,
  n-out 27, no count taken on trust.
- **Ground truth:** `docs/ROADMAP.md`; `git log`/`git show`/`git log -S`/
  `git log -G` for what landed and when; `docs/process-journal/` and its
  `artifacts/` archives; the source tree itself for every ALREADY-DONE claim.
- **Ambiguity handling:** for the four entries blocked on "idiomacy review", both
  readings ("blocked until the review runs" vs "blocked pending the review's
  triage of this item") were tested. All four converge: each is a named input in
  the review dispatch list (`ROADMAP.md:276-288`) *and* has a landed fix, so the
  verdict does not depend on the reading.
- **Not audited:** entries with any status other than `blocked`; whether the
  ALREADY-DONE work is itself correct (only that it exists); the six "can-wait"
  items re-deferred out of Plan 5.7, which are tracked as their own ROADMAP
  entries.
- **Read-only:** no ledger entry, ROADMAP line or tracker vehicle was created or
  modified. This report is the only file written.
