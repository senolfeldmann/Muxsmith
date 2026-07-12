# Task 19 report: Fluent plural selectors replace the "error(s)" provisional (#17 step 1)

Worktree `.worktrees/wave3`, branch `plan55-wave3`, based on post-wave-2 master (T8/T9/T16/T18 all present).

## Scope actually covered

The brief named two sites (`cli.ftl:2` `validate-summary`, `gui-batch.ftl:31`
`batch-diagnostics-summary`) plus "any `(s)`-pattern key added by Waves 1-2"
and the two deferred wave-1 bare counts. Grepping `(s)` across `locales/en/`
turned up two more, both in `diagnostics.ftl`: `suggestions-capped` and
`suggestion-partition`. `run-summary`/`jobs-summary-line`/
`jobs-history-run-label` ("N ok, N warning, N failed, N cancelled") do NOT
match `(s)` and are a different, deliberate, pre-existing style (status
labels, not counted nouns - matches build-tool "N passed, N failed"
convention, consistent across both CLI and GUI) - confirmed out of scope
per the task's own conditional ("if the grep turns them up"); left
untouched.

## The type-flow problem this task's context flagged, and how it forced scope

The named sites and `dry-run-summary` are all rendered from a CLI call site
that owns a real `usize` right before stringifying it - trivial to fix by
routing that value through as a number instead. `suggestions-capped` and
`suggestion-partition` are different: they are **diagnostic-based**
messages, rendered via `Renderer::diagnostic()` from `Diagnostic.params`,
which is `BTreeMap<String, String>` by design (spec 5.2/8.4: "core stays
prose- and type-free on the wire"). Adding a Fluent `[one]`/`*[other]`
selector to their templates without also fixing this would have been
**cosmetic and silently broken**: a `FluentValue::String` selector always
falls through to `*[other]`, so `dropped`/`count` would forever render as
plural even when the emitter genuinely produces 1 (confirmed reachable in
production: `planner.rs:1554` emits `.with("count", "1")` literally for the
isolated-overlap partition case, and `SuggestionsCapped`'s `dropped` is 1
whenever a rule collects exactly 4 candidates). This is exactly the Plan-4
lesson the task context named. Rather than declare it out of scope or ship
a message with a selector that can never select `[one]`, I extended the
renderer with a small, targeted numeric-promotion mechanism on both the CLI
(Rust) and GUI (TS) sides (both surfaces share `diagnostics.ftl`, so both
needed the fix - `DiagnosticsPanel.vue` renders `d.params` from the same
`BTreeMap<String,String>` wire shape over IPC/JSON). This is a real,
mirrored architecture addition beyond a literal 4-file edit; flagging it
here since it goes past what the brief's "Files" list named, per the go-gate
discipline of surfacing scope decisions rather than deciding them silently.

Core's `Diagnostic.params` itself was **not** touched (no wire-type
redesign) - that would be a disproportionate, cross-cutting change (every
`.with()` call site, JSON schema, TS types) for two params on two codes,
and is exactly the kind of premature abstraction
`feedback_scale_appropriate_design` warns against. Instead:

- **Rust** (`crates/muxsmith-cli/src/i18n.rs`): `render_diagnostic_message`
  (new, called from `render_diagnostic`) consults `numeric_diagnostic_params(DiagCode) -> &[&str]`
  (currently `SuggestionsCapped -> ["dropped"]`,
  `SuggestionPartition -> ["dropped", "count"]`) and parses just those
  named params back to `usize` before handing them to Fluent via
  `msg_with_counts`; anything else (including a listed param that fails to
  parse) stays a string, so a surprise value degrades to `*[other]`/
  `*[group]` instead of leaking `{$name}`.
- **TypeScript** (`src/diagnosticFluentParams.ts`, new file): mirrors the
  Rust list 1:1 (same two codes, same param names, comment says to keep
  them in lockstep). `DiagnosticsPanel.vue` calls
  `diagnosticFluentParams(d.code, d.params)` instead of passing `d.params`
  straight through; `@fluent/bundle`'s resolver only resolves a plural
  selector against a JS `number`, and `Diagnostic.params` arrives over IPC
  as `Record<string, string>` exactly like the Rust side.

`msg_with_count` (single count) was generalized to `msg_with_counts` (a
`&[(&str, usize)]` slice), since `validate-summary`/`batch-diagnostics-summary`
each need three independent selectors (errors/warnings/infos) in one
message - one call, not three. The sole prior caller (`run.rs`'s
`run-job-warning`) was updated to the new signature; behavior unchanged
(verified by its existing T9 regression test, still green).

## Touched keys inventory (before -> after)

| Key | File | Before | After (EN) |
|---|---|---|---|
| `validate-summary` | cli.ftl | `{ $errors } error(s), { $warnings } warning(s), { $infos } info(s).` | 3 independent `[one]`/`*[other]` selectors: `"1 error, 2 warnings, 0 infos."` |
| `dry-run-summary` | cli.ftl | `{ $count } files matched (searched ...)` | `[one] 1 file matched` / `*[other] { $count } files matched` (searched ...) |
| `batch-diagnostics-summary` | gui-batch.ftl | `{ $errors } error(s), { $warnings } warning(s), { $infos } info notice(s).` | 3 selectors, `info notice`/`info notices` noun kept from the original wording |
| `suggestions-capped` | diagnostics.ftl | `{ $dropped } further suggestion(s) ... were capped ...` | `[one] 1 further suggestion ... was capped ...` / `*[other] { $dropped } further suggestions ... were capped ...` (verb agreement fixed too) |
| `suggestion-partition` | diagnostics.ftl | `[overflow] { $dropped } further resolution group(s) were capped ...` / `*[group] These { $count } file(s) need ...` | nested selector inside each existing `kind` variant: overflow's `dropped` and group's `count` each get their own `[one]`/`*[other]` |

Not touched (confirmed out of scope): `run-summary` (cli.ftl),
`jobs-summary-line`/`jobs-history-run-label` (gui-jobs.ftl) - bare
"N ok, N warning, N failed, N cancelled" status-label style, not a `(s)`
provisional, consistent on both surfaces. `jobs-row-warning-count`
(gui-jobs.ftl) - already a correct `[one]`/`*[other]` selector from T11,
fed a real JS number at its call site (`JobRow.vue`'s `job.warningCount`);
untouched, already correct.

## Arg-type verification per touched call site

| Call site | Arg source | Type reaching Fluent | Fix needed? |
|---|---|---|---|
| `validate.rs::render_summary` | `diagnostics.iter().filter(...).count()` (real `usize`) | `FluentValue::Number` via `msg_with_counts` | Extracted `render_summary` (mirrors `run.rs`'s own precedent), routed through `msg_with_counts` |
| `commands/mod.rs::batch_human_report` (`dry-run-summary`) | `batch.files.len()` (real `usize`) | `FluentValue::Number` via `msg_with_counts` | Was `renderer.msg` (string); switched to `msg_with_counts` |
| `run.rs::render_finished` (`run-job-warning`) | `outcome.warnings.len()` (real `usize`) | `FluentValue::Number` (unchanged behavior) | Already correct since T9 (`msg_with_count`); call-site renamed to the generalized `msg_with_counts` |
| `BatchView.vue` (`batch-diagnostics-summary`) | `diagnosticCounts.value.{error,warning,info}` (JS `number`, `computed<{error:number,...}>`) | `FluentNumber` (fluent-vue/`@fluent/bundle` auto-wrap a JS `number`) | None - already numbers, confirmed by reading `@fluent/bundle`'s resolver (`typeof arg === "number" -> FluentNumber`) and fluent-vue's `formatPattern` (passes `value` through unmodified) |
| `i18n.rs::render_diagnostic_message` (`suggestions-capped`, `suggestion-partition`) | `Diagnostic.params: BTreeMap<String,String>` (e.g. `.with("dropped", dropped.to_string())` in `planner.rs`) | Was always `FluentValue::String` -> always `*[other]`. Now: `numeric_diagnostic_params` + `.parse::<usize>()` promotes named params to `FluentValue::Number` | **Root fix** (this is the Plan-4-lesson case the task context named) |
| `DiagnosticsPanel.vue` (`$t(d.code, d.params)`) | `d.params: Record<string,string>` (JSON over IPC, same wire shape) | Was always JS `string` -> `@fluent/bundle` never promotes -> always `*[other]` | **Root fix**, mirrored: `diagnosticFluentParams()` promotes the same two codes' named params |

## TDD evidence

Genuine RED before GREEN for the one place the fix was non-trivial
(diagnostic-based numeric promotion): added
`suggestions_capped_renders_singular_and_plural`,
`suggestion_partition_group_branch_renders_singular_and_plural`,
`suggestion_partition_overflow_branch_renders_singular_and_plural` to
`i18n.rs` BEFORE implementing `render_diagnostic_message`/
`numeric_diagnostic_params`; ran `cargo test -p muxsmith-cli --lib
i18n::tests::suggestion` and confirmed all 3 failed with the exact
predicted wrong-plural text (e.g. `"1 further suggestions ... were
capped"` instead of `"1 further suggestion ... was capped"`), then
implemented and reran to green. For the three direct-call-site keys
(`validate-summary`, `dry-run-summary`) I added the extracted, directly
unit-testable functions (`validate.rs::render_summary`,
`commands/mod.rs::batch_human_report`) with singular+plural assertions,
mirroring `run.rs`'s own established `render_summary` test precedent
(same file already had `finished_warning_with_exactly_one_warning_renders_singular`
from T9). GUI: e2e test in `smoke.spec.ts` exercises the same RED path
first-hand during development (an initial `toHaveText` literal assertion
failed against the real rendered DOM because of Fluent's directional-isolate
marks, `useIsolating: true`, the GUI catalog loader's default, unlike the
CLI's explicit `set_use_isolating(false)`) - fixed by stripping
U+2066-U+2069 before comparing (`visibleText()` helper), not by touching the
pre-existing isolating setting itself (out of scope: a GUI-wide behavior
predating T19, orthogonal to plural selectors, with real product
implications - RTL/bidi mixing - that is Şenol's call, not mine to change
as a side effect of a test).

All new Rust tests, by file:
- `crates/muxsmith-cli/src/i18n.rs`: 3 new tests (singular+plural per
  diagnostic-based key, 6 assertions total).
- `crates/muxsmith-cli/src/commands/validate.rs`: new `#[cfg(test)] mod
  tests` (file had none before), 2 tests.
- `crates/muxsmith-cli/src/commands/mod.rs`: 2 new tests in the existing
  module (+ a `file_report` test helper).
- `crates/muxsmith-cli/src/commands/run.rs`: unchanged (T9's existing
  singular test still covers the renamed call site).

GUI: 1 new e2e test in `e2e/smoke.spec.ts` (`batch view: dry run`
describe block), covering both `batch-diagnostics-summary` (1 error
singular / 2 warnings plural / 1 info notice singular, all three selectors
in one message) and `suggestions-capped` end-to-end through
`DiagnosticsPanel.vue` (dropped=1, singular).

## Files changed

- `locales/en/cli.ftl` - `validate-summary`, `dry-run-summary`.
- `locales/en/gui-batch.ftl` - `batch-diagnostics-summary`.
- `locales/en/diagnostics.ftl` - `suggestions-capped`, `suggestion-partition`.
- `crates/muxsmith-cli/src/i18n.rs` - `msg_with_count` -> `msg_with_counts`
  (generalized to N counts); new `render_diagnostic_message` +
  `numeric_diagnostic_params`; 3 new tests.
- `crates/muxsmith-cli/src/commands/validate.rs` - extracted `render_summary`,
  new test module.
- `crates/muxsmith-cli/src/commands/mod.rs` - `dry-run-summary` call site
  now `msg_with_counts`; 2 new tests.
- `crates/muxsmith-cli/src/commands/run.rs` - call-site rename only
  (`msg_with_count` -> `msg_with_counts`), no behavior change.
- `src/diagnosticFluentParams.ts` (new) - GUI-side mirror of
  `numeric_diagnostic_params`.
- `src/components/DiagnosticsPanel.vue` - routes `d.params` through
  `diagnosticFluentParams` before `$t()`.
- `e2e/smoke.spec.ts` - new test + `visibleText()` helper.

Untouched by design: `crates/muxsmith-core` (no `Diagnostic.params` type
change), `crates/muxsmith-cli/tests/catalog_completeness.rs` (its
placeholder-leak guard renders every fixture, including the two
diagnostic-based keys, via string args through `renderer.msg` regardless of
selector shape - a `*[other]` fallback still resolves cleanly, so it stays
green without needing to know about the new selectors; it was never a
plural-correctness guard, only a leak guard, confirmed by reading it before
relying on that).

## Gate results (all nine parts, run in this worktree)

1. `cargo test --workspace` - all green (no failures; workspace-wide
   count in the 400s across all crates+integration suites).
2. `cargo fmt --check` - clean (one auto-fix round needed on `i18n.rs`,
   applied via `cargo fmt`).
3. `cargo clippy --workspace --all-targets -- -D warnings` - clean (one
   `collapsible_if` finding in `render_diagnostic_message`, fixed with
   an `if let` chain).
4. `cargo deny check` - `advisories ok, bans ok, licenses ok, sources ok`.
5. `pnpm lint` (eslint) - clean.
6. `pnpm build` (vue-tsc --noEmit + vite build) - clean.
7. `pnpm check:i18n` - `ok (17 source files scanned, 177 catalog ids, 12
   unused warning(s))` - the 12 warnings are the script's own documented
   pre-existing false positives (IpcError codes reached only via
   `$t(err.code, err.params)`), unrelated to this task.
8. `pnpm test:e2e` (tsc + vite harness build + playwright) - 4/4 passed.
9. `cargo doc --workspace --no-deps` with `RUSTDOCFLAGS="-D warnings"` -
   clean.

`pnpm install --frozen-lockfile` ran once at the start (fresh worktree, no
`node_modules`).

## Self-review

- Grammar: fixed the `suggestions-capped` singular's verb agreement
  ("was capped" vs. plural "were capped") even though the brief didn't
  call it out specifically - it falls directly out of doing the plural
  split properly and would otherwise ship a new, more visible inconsistency
  ("1 further suggestion ... were capped").
- Nested Fluent selectors (`suggestion-partition`'s `kind` selector now
  containing per-variant `dropped`/`count` selectors) were verified to
  parse correctly against the real `fluent_syntax` parser (not just
  reasoned about) before wiring any Rust code around them - ran the
  existing `i18n::tests` suite immediately after the catalog edit alone
  and confirmed `Renderer::new`'s `.expect("embedded catalog must parse")`
  didn't panic.
- Did not touch the GUI's `useIsolating` default, even though it made my
  own e2e assertions harder to write - a real, but orthogonal, pre-existing
  gap (CLI disables it for grep-ability, GUI never has) that belongs to
  whoever owns GUI i18n polish, not a silent side effect of a plural-selector
  task.
- `run-summary`/`jobs-summary-line`/`jobs-history-run-label` deliberately
  left as bare counts - re-checked against the task's own conditional
  ("if the brief's grep turns them up") rather than assuming pluralization
  is universally correct; these are categorical status labels (mirroring
  "N passed, N failed" test-runner idiom), not counted nouns, and are
  consistent as such on both CLI and GUI.

## Concerns / residual items (not blocking, flagged for T20/T21/T23)

- `numeric_diagnostic_params` (Rust) and `NUMERIC_DIAGNOSTIC_PARAMS` (TS)
  are two hand-maintained lists that must stay in lockstep; both carry a
  comment saying so, but nothing enforces it mechanically. At the current
  scale (2 codes, 1-2 params each) a shared-source-of-truth mechanism
  would be over-engineering; worth revisiting only if this list grows
  materially (e.g. if T21 or later work adds more plural-selected
  diagnostic params).
- T21 (de catalogs) will need to translate the now-nested
  `suggestion-partition` selector correctly (German's plural rule differs
  in shape from English's but Fluent's CLDR `[one]`/`*[other]` mechanism
  is locale-generic - de's own `[one]` semantics apply automatically once
  the catalog exists; no code change needed, just translated variant text).
- `catalog_completeness.rs`'s fixtures for `suggestions-capped`/
  `suggestion-partition` still pass string values (`"2"`, `"1"`) since
  that guard only checks for leaked placeholders, not plural correctness;
  I did not extend it to also verify plural selection, since that would
  duplicate exactly what the new `i18n.rs` unit tests already assert more
  precisely (with the real numeric-promotion path exercised) - flagging in
  case T23's whole-branch review wants belt-and-suspenders coverage there
  too.

## Wave 3 follow-up fixes (T19 review)

**Commit 0d19dc4** (`i18n(fix): numeric-promotion parity + mirrored-list pin (T19 review)`):

1. **TS numeric-promotion parity:** Tightened `src/diagnosticFluentParams.ts`'s
   `Number(raw) + Number.isFinite(n)` check to mirror Rust's `parse::<usize>()`
   strictness. Now rejects negatives, floats, empty strings, and whitespace-only
   strings; accepts only non-negative integers. Added explicit `raw.trim() === ""`
   guard since `Number("") === 0`. Updated docstring to clarify mirrored Rust parity.

2. **Mirrored-list pin test:** Added `numeric_diagnostic_params_list_is_mirrored_to_ts_side`
   test in `crates/muxsmith-cli/src/i18n.rs` (since no TS unit-test runner was found).
   Test asserts the exact `(DiagCode, param_names)` tuples match between Rust
   `numeric_diagnostic_params()` and TS `NUMERIC_DIAGNOSTIC_PARAMS`. Comments in both
   locations name the mirror file so changing one side alone will fail the test.
   Also verifies no other DiagCodes unexpectedly gain numeric params.

**Test results:**
- `pnpm lint`: passed (no TS lint errors)
- `cargo test -p muxsmith-cli --lib i18n`: 12 tests passed (including the new pin test)
- `cargo fmt --all --check`: passed (formatting clean)
