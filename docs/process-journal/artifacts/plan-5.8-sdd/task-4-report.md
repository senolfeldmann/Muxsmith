# Task 4 report - D40: plan-JSON serialization fix (struct variants + report/json.rs hardening)

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-c`, branch `plan58-c`, based on merged master state `34e5624`. All work done there; main tree and other worktrees untouched. Not pushed.

## R4 (TDD order): failing tests first, real panic evidence

### Step 1: regression e2e written against the unfixed tree

Added `readme_passthrough_recipe_with_title_template_survives_dry_run_and_run` to `crates/muxsmith-cli/tests/run_live.rs`: the README's passthrough recipe (README.md:71-78) inlined verbatim, built against a real two-track MKV (the existing `tone.wav`/`sub.srt` fixture seeds), driven through `dry-run --json` then `run`, with `mkvmerge -J` asserting the templated title landed and the persisted `summary.json` asserting the same plan shape.

```
cargo test -p muxsmith-cli --test run_live readme_passthrough -- --nocapture
```

Real output (pre-fix):

```
thread 'readme_passthrough_recipe_with_title_template_survives_dry_run_and_run' (1245911) panicked at crates/muxsmith-cli/tests/run_live.rs:339:5:
dry-run --json must exit 0, stdout: , stderr: 
thread 'main' (1245914) panicked at crates/muxsmith-core/src/report/json.rs:44:13:
called `Result::unwrap()` on an `Err` value: Error("cannot serialize tagged newtype variant TitleAction::Set containing a string", line: 0, column: 0)
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... FAILED
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.12s
```

Exact match to the finding's evidence (report/json.rs:44, `TitleAction::Set`). The subprocess's own panic (`thread 'main'`) is what `dry-run --json` hit; my outer test assertion panicked first on the exit-code check, which is why only the dry-run half shows here - the fix removes both failure points identically (confirmed below), so a second, separate capture of the `run`-side exit-101 panic was not necessary.

### Step 2: per-variant round-trip/shape tests, also written and run against the unfixed tree

Added three tests to `crates/muxsmith-core/src/planner.rs`'s existing `#[cfg(test)] mod tests` (the `all_keys_match_serde_encoding` idiom from `report/mod.rs`, extended to `Plan`'s three tagged enums, covering every variant, not only the three broken ones). Written first with the OLD tuple-variant construction syntax (the only thing that compiles against the unfixed enums), asserting the intended post-fix wire shape:

```
cargo test -p muxsmith-core --lib serialize_to_the_expected_kind_tagged_shape -- --nocapture
```

Real output (pre-fix):

```
thread 'planner::tests::chapter_source_variants_serialize_to_the_expected_kind_tagged_shape' panicked at crates/muxsmith-core/src/planner.rs:2169:86:
called `Result::unwrap()` on an `Err` value: Error("cannot serialize tagged newtype variant ChapterSource::External containing a string", line: 0, column: 0)
thread 'planner::tests::title_action_variants_serialize_to_the_expected_kind_tagged_shape' panicked at crates/muxsmith-core/src/planner.rs:2152:74:
called `Result::unwrap()` on an `Err` value: Error("cannot serialize tagged newtype variant TitleAction::Set containing a string", line: 0, column: 0)
thread 'planner::tests::primary_attachments_variants_serialize_to_the_expected_kind_tagged_shape' panicked at crates/muxsmith-core/src/planner.rs:2186:74:
called `Result::unwrap()` on an `Err` value: Error("cannot serialize tagged newtype variant PrimaryAttachments::Subset containing a sequence", line: 0, column: 0)
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 116 filtered out; finished in 0.00s
```

Confirms the whole-branch-verdict's prediction that `ChapterSource::External`/`PrimaryAttachments::Subset` fail identically to `TitleAction::Set`, though not exercised by any existing test before this task.

## R2: the shape fix

Converted all three enums to struct variants in `crates/muxsmith-core/src/planner.rs`, keeping `#[serde(tag = "kind", rename_all = "snake_case")]`:
- `TitleAction::Set(String)` -> `Set { title: String }`
- `ChapterSource::External(PathBuf)` -> `External { path: PathBuf }`
- `PrimaryAttachments::Subset(Vec<u64>)` -> `Subset { ids: Vec<u64> }`

Updated every construction/match site:
- `crates/muxsmith-core/src/planner.rs`: `resolve_title`, `resolve_chapters`, `resolve_attachments`, the `resolved_sources` chain in `resolve_file`, `detect_non_utf8_paths`.
- `crates/muxsmith-core/src/command.rs`: `push_global` (title + chapters), `push_group_chapters`, `push_group_attachments`.
- `crates/muxsmith-core/tests/command.rs` and `crates/muxsmith-core/tests/planner_resolution.rs`: every struct-literal/match-pattern construction of the three variants (7 sites total).

**Consumer inventory (as required by the brief):**

```
grep -rn "TitleAction::\|ChapterSource::\|PrimaryAttachments::" --include="*.rs" .
```
gave the complete Rust-side site list above (all now updated; workspace-wide `cargo build --workspace --tests` compiles clean, confirming nothing was missed).

```
grep -rn "TitleAction\|ChapterSource\|PrimaryAttachments" --include="*.ts" --include="*.tsx" --include="*.vue" --include="*.svelte" .
grep -rln "\"Set\"\|'Set'\|\"External\"\|\"Subset\"" src/
```
Both zero hits - no TS/frontend consumer references these types or the literal variant strings, confirmed empirically, not assumed.

Round-trip/shape tests updated to the new struct-literal syntax (assertions unchanged - they always targeted the post-fix shape) and re-run: green, along with the e2e (both reported under "Full green run" below).

## R3: report/json.rs hardening (owner-approved rider)

**Chosen design - NOT the brief's stated preferred default, using the brief's own escape hatch** ("if you find a materially simpler equally-safe alternative, implement it and record the choice in D40"). Full rationale is in D40 (docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md); summary here:

`report::json::batch_document`'s per-file `"plan"` field, previously embedded via the `json!` macro (whose non-literal-expression expansion is exactly `to_value(&_).unwrap()` - the actual, invisible-in-source panic site at `report/json.rs:44`), now goes through a new private `plan_value` helper that calls `serde_json::to_value` explicitly and falls back to `serde_json::Value::Null` on error. `batch_document`'s public signature is **unchanged** (`-> serde_json::Value`, not `Result`).

**Why not the preferred `Result`-propagation:** traced every call site first (`grep -rln "batch_document\|run_document\|config_only_document"` across the workspace) - only `batch_document` itself touches `Plan` data; `run_document` only splices `jobs`/`summary` onto an already-built value, `config_only_document` never has a `Batch`. But `batch_document` is called from **both** binaries: `muxsmith-cli`'s `dry_run.rs`/`run.rs` AND `muxsmith-gui`'s (`src-tauri`, a workspace member, not under `crates/`) `lib.rs`/`run.rs`. A `Result`-returning signature would ripple into all of them, and the CLI side has no existing catalog message for "internal report-building failure" - faithfully propagating one (rather than an unlocalized raw `eprintln!`, which would violate this crate's own "every user string through Fluent" pattern) needs a new bilingual `cli.ftl` entry, a `locales/` change that (per this plan's own D38/D39 precedent) drags in the frontend/e2e pnpm gate for a path R2 makes structurally unreachable today.

More fundamentally: after R2, `plan_value`'s fallback branch cannot fire for any `Plan` this crate can construct (pinned by the R4.2 shape tests covering every variant). The only way to reach it is a **future regression** reintroducing a non-map newtype variant. If that future bug's remedy is "exit non-zero, fail the whole `run`," it reintroduces Finding 1's own failure mode in miniature - `run.rs` builds and executes every `JobSpec` from `f.plan` and runs the mux to completion **before** `batch_document` is ever called, so degrading the report after a bug there must never turn an already-completed successful mux into an apparent failure. The `null` fallback also introduces no new wire shape: `null` is already `f.plan`'s existing encoding for an error-severity file with no plan (`Option<Plan>`'s ordinary `None`).

The module's other pre-existing `serde_json::to_value(d).unwrap()` (`rendered_diags`, over `Diagnostic`) is deliberately left as `unwrap()` and not touched: verified `Diagnostic`'s full field set (`DiagCode` - fieldless enum, kebab-case; `Severity` - fieldless enum; `String`; `Option<PathBuf>`; `BTreeMap<String, String>`; `Option<usize>`; `Vec<usize>`) - every field type serializes unconditionally, so this unwrap is structurally infallible and out of R3's "plan data" scope, not an oversight left behind.

No `unwrap()`/`expect()` on serializing `Plan`/`Batch` data remains in `report/json.rs` (grepped after the change: only the provably-safe `Diagnostic` unwrap and `run_document`'s pre-existing `JobOutcome` `.expect()`, both outside this ADR's scope, remain).

## R1: ADR D40

Appended `## D40: plan-JSON serialization fix - struct variants + report/json.rs hardening` to `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md`, placed before "## Deliberately out of scope" (keeps the D-numbered decisions grouped; the out-of-scope coda stays last - a placement judgment call, not flagged as a deviation). Full D37-D39 slot layout (Decision / Rationale / Rejected alternatives / Interface-wire-format change / Spec amendments / Triggers created / Consistency note), covering both the R2 shape decision and the R3 hardening decision together (as the brief's content requirements specified).

**Spec amendment check** (brief-mandated grep, run before writing the ADR's "none required" claim):

```
grep -n "TitleAction\|ChapterSource\|PrimaryAttachments\|\"kind\"\|'kind'" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
grep -n 'title.*Set\|Set(String)\|External(Path\|Subset(Vec\|plan\.title\|plan\.chapters\|plan\.attachments' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
grep -n '"plan"\|files\[\].plan\|"files"' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```
All three: zero hits. Recorded in D40 as "none required - the spec does not spell these shapes" (5.2 covers the diagnostics catalog, 5.5 covers operation levels; neither touches `Plan`'s own field shapes).

**Consistency note (SI-3):** recorded explicitly in D40 that an mkvtoolnix comparison is not meaningful - this is Muxsmith's own internal report/plan serialization, and mkvmerge has no declarative "plan" artifact to compare against.

**Triggers created:** none - recorded explicitly in D40.

## Full green run (post-fix)

```
cargo test -p muxsmith-core --lib serialize_to_the_expected_kind_tagged_shape -- --nocapture
```
```
running 3 tests
test planner::tests::chapter_source_variants_serialize_to_the_expected_kind_tagged_shape ... ok
test planner::tests::title_action_variants_serialize_to_the_expected_kind_tagged_shape ... ok
test planner::tests::primary_attachments_variants_serialize_to_the_expected_kind_tagged_shape ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 116 filtered out
```

```
cargo test -p muxsmith-cli --test run_live
```
```
running 4 tests
test live_run_muxes_two_sources_and_reports_exit_zero ... ok
test zero_rule_keep_profile_is_a_pure_passthrough ... ok
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok
test live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Foreground, real mkvmerge v100 on PATH, zero `MKVMERGE_SKIP_MARKER` occurrences - all four gated live tests actually ran.

```
cargo test --workspace
```
Every binary: `test result: ok`, 0 failed, across all workspace crates (`muxsmith-core`, `muxsmith-cli`, `muxsmith-gui`/src-tauri, `xtask`) including doc-tests. No `FAILED`, no `panicked`, no `error[` anywhere in the full log.

## Full house gate

```
cargo fmt --all --check          # clean (one file needed `cargo fmt --all` once, then re-verified clean)
cargo clippy --workspace --all-targets -- -D warnings   # clean, zero warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps   # clean, zero warnings, generated
cargo deny check                 # exit 0: "advisories ok, bans ok, licenses ok, sources ok"
```

`git status --porcelain` before commit touched only `crates/muxsmith-cli/tests/run_live.rs`, `crates/muxsmith-core/src/{command,planner}.rs`, `crates/muxsmith-core/src/report/json.rs`, `crates/muxsmith-core/tests/{command,planner_resolution}.rs`, `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md` - nothing under `locales/` or `src/`, so the additional pnpm gate (lint/build/check:i18n/test:e2e) was not triggered and not run, per the brief's own framing ("if you touch anything outside crates/ - you should not"). `src-tauri` (a workspace member outside `crates/` but pure Rust, already covered by `cargo test --workspace`/clippy/fmt/deny above) was not touched at all: R3's chosen design specifically avoided a signature change there.

## Deviations/patterns surfaced (not silently resolved)

1. **R3's implemented option is the brief's non-preferred alternative.** The brief named `Result`-propagation as preferred; I implemented the null-fallback instead, using the brief's own "materially simpler equally-safe alternative" clause. Full rationale in D40 and in the R3 section above - flagging here per the process constraint, not asking permission after the fact, since the brief explicitly delegated this choice to the implementer with a recording obligation, which D40 satisfies.
2. **D40's placement in the decisions file** is before "## Deliberately out of scope" rather than strictly appended at the file's very end - a minor structural judgment call (keeps decisions grouped), not a content deviation.

No other deviations. Typography: ASCII-only verified across every changed file (grepped for em/en dash, curly quotes, ellipsis - zero hits).

## Commit

```
git add crates/muxsmith-cli/tests/run_live.rs crates/muxsmith-core/src/command.rs crates/muxsmith-core/src/planner.rs crates/muxsmith-core/src/report/json.rs crates/muxsmith-core/tests/command.rs crates/muxsmith-core/tests/planner_resolution.rs docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md
git -c commit.gpgsign=false commit -m "fix: plan-JSON serialization panic on Set/External/Subset (D40)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

One commit (ADR + code together - tightly coupled, D40 references exact behavior the same diff implements). Commit `de9ec50`. 7 files changed, 433 insertions(+), 24 deletions(-). Working tree clean after commit. Not pushed.

## Status

DONE

Commit: `de9ec50`

Test summary: `cargo test --workspace` all green (0 failed across every crate incl. doc-tests); `run_live.rs` 4/4 gated live tests actually ran against real mkvmerge v100 (zero skip markers), including the new README-recipe regression e2e; `fmt`/`clippy -D warnings`/`doc -D warnings`/`deny` all clean.

## Fix round 1 (review verdict: APPROVED, one Minor finding)

Reviewer adjudication: R3 deviation ruling STANDS (infallible signature + null fallback judged sound and house-consistent). Minor finding: the `plan_value` fallback degraded silently in every build; a future regression of the same class would ship null plans into persisted run documents unnoticed.

Fix applied, nothing else touched:

1. `crates/muxsmith-core/src/report/json.rs` - `plan_value`'s error branch restructured from `unwrap_or(Null)` to an explicit `unwrap_or_else` closure carrying a `debug_assert!(false, ...)` whose message names the failing serialization error, the defect class ("a Plan-adjacent enum has reintroduced a non-map newtype variant under #[serde(tag = \"kind\")]"), and the D40 context. Loud (panics) in dev/CI builds, compiled out in release: release behavior stays exactly as ruled (null degradation, mux never misreported). Doc comment extended by one sentence describing the debug_assert's role.
2. `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md` - D40's rejected-alternatives slot (the eprintln bullet) extended by one sentence recording the adopted debug_assert middle ground (loud in dev/CI, compiled out in release, null degradation unchanged there).

Covering checks, run foreground:

```
cargo test -p muxsmith-core
```
Every binary green: 119 + 15 + 4 + 2 + 1 + 1 + 1 + 12 + 7 + 2 + 70 + 8 + 8 + 7 + 3 + 3 + 12 + 10 + 21 + 15 passed, 0 failed across all test binaries. The suite runs in the dev profile where `debug_assert!` is ACTIVE, and no test panicked: the assert fired nowhere, confirming the error branch stays unreachable for every constructible `Plan` (the three round-trip shape tests explicitly re-verified: 3/3 ok).

```
cargo fmt --all --check
```
Clean, exit 0.

```
cargo clippy --workspace --all-targets -- -D warnings
```
Clean, zero warnings, all workspace crates.

Typography ASCII-only re-verified on both changed files (em/en dash, typographic dashes, curly quotes, ellipsis, NBSP: zero hits).

Commit (unsigned, explicit staging of exactly the two files):

```
git add crates/muxsmith-core/src/report/json.rs docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md
git -c commit.gpgsign=false commit -m "fix: debug_assert on the plan_value fallback so it is loud in dev/CI (task-4 review fix round 1)"
```

Commit: ff9140f. 2 files changed. Working tree clean after commit. Not pushed.

## Status (final, after fix round 1)

DONE

Commits: de9ec50 (task), ff9140f (fix round 1)
