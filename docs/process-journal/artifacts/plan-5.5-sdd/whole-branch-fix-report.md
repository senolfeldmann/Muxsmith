# Whole-branch fix wave report (T23)

Fixer dispatch for the five items the whole-branch verdict
(`whole-branch-verdict.md`) named FIX-NOW: C1 (critical), I1 (important), and
the three ledger FIX-NOW minors (T4-m2, T7.5-m1, T18-m1). Worked directly on
`master` (clean tree, standing authorization), one commit. Full nine-part
gate green (see end).

## C1 — SchemaDrift spec severity: warning -> info

`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`:
- §5.2 table row: `SchemaDrift | warning | ...` -> `SchemaDrift | info | ...`.
- §9.2 prose: "emits a `SchemaDrift` warning" -> "emits a `SchemaDrift` info
  notice".

Both now agree with the owner ruling (D32 addendum, Task 16.5) and the code
(`planner.rs`: `Diagnostic::info(DiagCode::SchemaDrift, ...)`; `report/mod.rs`
rustdoc: "info severity"; test
`schema_drift_fires_once_per_batch_with_the_max_found_version` asserts
`Severity::Info`).

## I1 — §5.2 catalog: missing EmptyPlan, UnknownExtension, WorkerPanicked rows

Added three rows to the §5.2 table, each verified against its actual emitter
site (not transcribed from the ledger's characterization alone):

- **`EmptyPlan`** (warning, no params) — inserted after `UnidentifiableSource`
  (same per-file resolution-outcome cluster). Verified against
  `planner.rs::detect_empty_plans` (`Diagnostic::warning(DiagCode::EmptyPlan,
  "tracks").for_file(&f.source)`) and its D20 keep-passthrough exception.
- **`UnknownExtension`** (warning; params `extension`, `known`) — inserted
  after `MultipleIdentifierMatches` (same input/extension-matching cluster).
  Verified against `planner.rs::validate_extension_list`. Deliberately did
  NOT claim "once per batch" in the new row: cross-checked against M3 in the
  verdict (report/mod.rs's own rustdoc overclaims batch-wide-once; actual
  emission is once per offending list entry) and wrote the spec row to match
  the real per-entry behavior instead of propagating that inaccuracy.
- **`WorkerPanicked`** (info) — inserted after `SuggestionPartition` (last
  row, matching `report/mod.rs`'s own "Run-time (executor)" placement after
  every planning-time code). Carries the honest channel note the task
  specified: it is not a batch `Diagnostic` at all — `queue.rs`'s
  `recover_panicked_worker` carries it as a `"worker-panicked: job N"` string
  token in `JobOutcome.errors` (and that struct's `--json` encoding), never
  through the `Diagnostic{code,severity,config_path,file,params,
  suggestion_ref}` structure the rest of the table describes. Severity
  recorded as info per the verdict's own classification; the code has no
  literal `Severity` value attached to `WorkerPanicked` since it never flows
  through `Diagnostic` at all — the row's prose makes that explicit rather
  than implying otherwise.

Re-read the full §5.2 section after editing: the three new rows are
consistent with each other, with the surrounding rows' format (code, params,
spec-section back-references), and with the `diagnostic := {code, severity,
config_path, file?, params, suggestion_ref?}` grammar line above the table
(the `WorkerPanicked` row is the one row that explicitly opts out of that
grammar, which is the point of the channel note).

`WorkerPanicked`'s optional §6 sentence was decided against: the §5.2 row's
channel note already carries the executor-scope clarification; a second
mention in §6 would be pure duplication with no added information.

## T4-m2 — child-process leak on post-spawn panic

`crates/muxsmith-core/src/executor/queue.rs`, `recover_panicked_worker`:
`ctl.killers().remove(&index)` discarded the removed `Killer` without
invoking it. A worker thread that panics after a successful spawn (mid
`run_job`, e.g. inside line-parsing) leaves its mkvmerge child running; the
function reports the job `Failed` and, until this fix, never told the child
to stop, so it kept writing to an output already reported failed.

Fix: `if let Some(killer) = ctl.killers().remove(&index) { killer(); }` —
invoke before dropping. One comment line notes the killer is
idempotent/best-effort (per its own doc comment in `spawn.rs`) and why the
invoke must happen before the removal is discarded.

`worker_panic_is_reported_as_failed_not_cancelled` still passes unchanged:
its `PanicOnIndexSpawner` panics inside `spawn()` itself, before any
`RunningJob`/`Killer` exists for that index, so `killers().remove(&index)`
returns `None` there and the new branch is a no-op for that test — confirmed
by running it directly (`cargo test -p muxsmith-core --lib
executor::queue::tests::worker_panic_is_reported_as_failed_not_cancelled`,
passes).

## T7.5-m1 — resolve_file top doc comment omits chapters donors

`crates/muxsmith-core/src/planner.rs`, `resolve_file`'s top doc comment named
only "assignment sources (primaries and track donors) plus attachment `add`
donors (Task 7.5)", stale since Task 7.6 added chapters donors
(`ChapterSource::External`) to the same `resolved_sources` chain. The
completeness comment lower in the function (around the `resolved_sources`
construction, "Completeness (Task 7.6, #7 class closure)") already names all
three donor kinds correctly and was left untouched. One-line fix: the top doc
comment now names all three kinds — track donors, attachment `add` donors
(Task 7.5), and chapters donors (Task 7.6) — matching the lower comment.

## T18-m1 — dead `edited` fixture in suggestions.rs TC-A

`crates/muxsmith-core/tests/suggestions.rs`: TC-A defined an `edited` YAML
string (the naive/literal splice of the accepted structured edit) that was
never read except by `let _ = edited;` to silence the unused-variable
warning; the test actually plans against `applied` (the corrected form) a
few lines later. Removed the dead `edited` string, its `let _ = edited;`, and
the transitional comment that existed solely to explain the discrepancy
between `edited` and `applied` (its "the `not` above is spliced under
`match`" wording had no referent once `edited` was gone); folded that
comment's remaining useful content into a one-line note directly above
`applied`.

## Anything unexpected during this wave

While confirming I1's row set (grepping `report/mod.rs`'s `DiagCode` list
against every code name already present in the §5.2 table), found that
`UnsupportedSource` is *also* absent from the table — a fourth missing row,
not named in the verdict's I1 finding or the fix dispatch. Left untouched:
out of the assigned scope for this wave (the dispatch named exactly
`EmptyPlan`/`UnknownExtension`/`WorkerPanicked`), flagged here for a
follow-up spec pass rather than fixed unilaterally.

## Gate results (foreground, full nine-part BUILDING.md gate)

1. `cargo fmt --all --check` — clean, no diff.
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
3. `cargo test --workspace` — all green (36 `test result: ok` blocks, 0
   failed, exit 0); `worker_panic_is_reported_as_failed_not_cancelled` and
   the touched `suggestions.rs` TC-A test both pass.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean.
5. `cargo deny check` — advisories/bans/licenses/sources all ok, exit 0.
6. `pnpm lint` — clean.
7. `pnpm build` — `vue-tsc --noEmit && vite build` clean.
8. `pnpm check:i18n` — ok (12 pre-existing "unused" warnings on
   backend-error keys, unrelated to this wave; 1 locale x 6 catalogs parity
   clean).
9. `pnpm test:e2e` — 6/6 Playwright specs passed.

All nine parts green. Working tree: five files touched (four source/test
files plus the spec), staged and committed in one unsigned commit, not
pushed.
