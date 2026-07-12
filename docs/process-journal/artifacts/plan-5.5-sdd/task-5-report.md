# Task 5 report: Wire `--list-types` extension validation (#3)

Worktree: `.worktrees/stream-b`, branch `plan55-stream-b`.
Commit: `d9db161 feat(planner): validate profile extensions against mkvmerge --list-types (#3)`.

## What was implemented

`profile.input.extensions` is now checked once per batch against the local
mkvmerge's `--list-types` output; an entry not on that list still gets used
for file matching (so a typo silently excludes candidates, unchanged) but
now raises a warning-severity `UnknownExtension` diagnostic naming the
offending extension and the accepted set. When the runtime capability is
unavailable (mkvmerge absent, or the `--list-types` query fails), the check
degrades to a silent no-op rather than blocking planning.

### Design decision: no new `plan_core`/`plan_batch` parameter

The brief's file list (planner.rs, capability/runtime.rs, model.rs,
report/mod.rs, diagnostics.ftl, "planner batch-validation tests" only) does
not include any CLI or GUI production file (`dry_run.rs`, `run.rs`,
`src-tauri/src/{lib,run}.rs`), unlike Tasks 8/9 in the same plan, which
explicitly list them. Those files also aren't touched by any later task in
this stream or a parallel one for this purpose. I read that omission as a
deliberate scope boundary (also avoids stream-conflict risk: T8/T9 in a
different stream edit the exact same CLI files at different lines).

A naive mirror of `lang: &LanguageIndex` would add a new mandatory
parameter to `plan_core`/`plan_batch`, breaking ~20 existing call sites
across core tests, `muxsmith-cli`, and `src-tauri` for compilation alone,
and would still need those 4 production files to actually call
`mkv.list_types()` for the feature to do anything in production.

Instead, I threaded the capability through the **existing** `id: &mut dyn
Identify` parameter `plan_core` already takes:

- `Mkvmerge::known_extensions()` (`capability/runtime.rs`): `list_types()`
  degraded to `Option` (`Err` -> `None`).
- `Identify::known_extensions(&mut self) -> Option<Vec<String>>`
  (`identify.rs`): new trait method with a **default `None`** impl, so
  every existing `Identify` fake (`FakeIdent` etc.) keeps compiling and
  keeps behaving exactly as before (degraded/no-op) with zero changes.
- `IdentifyCache` gained a memoized field so the query runs at most once
  per cache instance, not once per `resolve_file` call and not once per
  `suggest`'s repeated `plan_core` re-simulation passes (which reuse the
  same `id`/cache instance) - satisfies "once per batch, not per file"
  without adding an explicit signature-level cache parameter.
- `LiveIdentifier` (constructed identically at all 4 production call
  sites, unchanged) overrides the method to return real data.

Net effect: **zero production call sites needed editing**; `cargo build
--workspace` was clean immediately after the core-only change, confirming
the design. `plan_core` calls `id.known_extensions()` once and passes it to
the new `validate_extension_values`, whose own signature
(`profile, known: Option<&[String]>, diags`) mirrors
`validate_language_values`'s shape.

## TDD evidence

RED (implementation call temporarily replaced with a no-op):

```
$ cargo test -p muxsmith-core --test planner_resolution unknown_extension -- --nocapture
...
thread 'unknown_extension_is_batch_warning_naming_the_extension' panicked at
crates/muxsmith-core/tests/planner_resolution.rs:1126:5:
assertion `left == right` failed: batch diags: []
  left: 0
 right: 1
test unknown_extension_check_degrades_when_runtime_unavailable ... ok
test unknown_extension_is_batch_warning_naming_the_extension ... FAILED
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 50 filtered out
```

(The degrade test passes trivially either way since it asserts absence of
a diagnostic; the positive test is the one that proves the walk is wired.)

GREEN (implementation restored):

```
$ cargo test -p muxsmith-core --test planner_resolution extension -- --nocapture
running 4 tests
test known_extension_case_insensitive_is_not_flagged ... ok
test unknown_extension_is_batch_warning_naming_the_extension ... ok
test unknown_extension_check_degrades_when_runtime_unavailable ... ok
test keep_filename_on_mp4_source_replaces_extension_with_mkv ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 48 filtered out
```

Three tests added to `crates/muxsmith-core/tests/planner_resolution.rs`,
right after `bad_language_value_is_batch_invalid_property_value` (the
batch-level language-validation test it mirrors):

- `unknown_extension_is_batch_warning_naming_the_extension`: brief's Step 1
  scenario verbatim (`extensions: [mkv, mp4a]` against a known list of
  `mkv, mp4, avi`); asserts exactly one warning-severity `UnknownExtension`
  naming `mp4a`, and that the file still resolves to a plan (batch
  continues).
- `unknown_extension_check_degrades_when_runtime_unavailable`: `known
  extensions = None`; asserts no `UnknownExtension` diagnostic and the
  batch still plans normally (the degrade path, Step 2).
- `known_extension_case_insensitive_is_not_flagged`: `extensions: [MKV]`
  against a lowercase known list; locks in the doc comment's
  case-insensitivity claim now that it is enforced.

A small test-local `FakeIdentWithExtensions` wrapper (delegates `identify`
to an inner `FakeIdent`, overrides `known_extensions`) and a
`plan_one_with_extensions` helper were added in the same file; the existing
`FakeIdent`/`plan_one` and every test using them are untouched.

## Files changed

- `crates/muxsmith-core/src/capability/runtime.rs`: `Mkvmerge::known_extensions()`.
- `crates/muxsmith-core/src/identify.rs`: `Identify::known_extensions` default method, `IdentifyCache` memoization field + method, `LiveIdentifier` override.
- `crates/muxsmith-core/src/planner.rs`: `validate_extension_values`, wired once into `plan_core`.
- `crates/muxsmith-core/src/profile/model.rs`: `Input.extensions` doc comment now states the real behavior (checked once per batch, still used for matching regardless, degrades silently when unavailable) instead of the previously-false claim.
- `crates/muxsmith-core/src/report/mod.rs`: new `DiagCode::UnknownExtension` ("unknown-extension"), placed next to `UnknownPropertySkew`.
- `locales/en/diagnostics.ftl`: `unknown-extension` message, `$extension`/`$known` params.
- `crates/muxsmith-core/tests/planner_resolution.rs`: three new tests + local test doubles, as above.

`identify.rs` is not in the brief's literal file list; it is the natural
home for the trait/cache plumbing the chosen design needed and is small
(2 methods, 1 field, both well within the existing memoization idiom
`get_or_identify` already uses).

## Gate results (from worktree root, all green)

- `cargo fmt --all --check`: clean (after running `cargo fmt --all` once to apply formatting the initial edit didn't match).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean (one `clippy::manual_contains` hit fixed: `known.iter().any(|k| *k == normalized)` -> `known.contains(&normalized)`).
- `cargo test --workspace`: all passing, no failures (core, cli, gui, xtask, doctests).
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint`: clean.
- `pnpm build`: `vue-tsc --noEmit && vite build` clean.
- `pnpm check:i18n`: `ok (16 source files scanned, 172 catalog ids, 12 unused warning(s))` - the 12 warnings are pre-existing `gui-*.ftl` keys, unrelated to this change.
- `pnpm test:e2e`: `3 passed` (Playwright smoke + a11y + i18n-completeness harness).
- `cargo test -p muxsmith-cli --test catalog_completeness`: explicitly re-checked, passes (proves the new `unknown-extension` Fluent key exists and is wired).

`pnpm install --frozen-lockfile` was run once first (`node_modules` was
missing); `node`/`pnpm` versions matched the pinned `mise.toml` versions
without needing `mise install`.

## Self-review findings

1. **`crates/muxsmith-core/src/profile/model.rs:254-256` (`Locator.extensions`) makes the same now-fixed-elsewhere false claim, unaddressed.** Its doc comment reads "Candidate extensions, validated against `mkvmerge --list-types` like `input.extensions`" - a direct claim of parity with the field I just fixed. `EmptyExtensions` (config-time, `validate.rs:52` and `:386`) already checks both `input.extensions` and every `Locator.extensions` symmetrically, so there is established precedent for treating them the same way. This task's brief and Step-1 test scope only `input.extensions`, so I left `Locator.extensions` untouched rather than unilaterally expanding scope - but its doc comment is now the *only* remaining false claim of this kind in the codebase, and closing it would need its own walk (locators live nested inside track rules, chapters, and attachment `add` entries, so it's a recursive walk like `walk_exact_languages`, not a flat list check) plus its own test. Recommend a follow-up task or an explicit decision to downgrade that doc comment's claim.
2. **Message wording.** The `unknown-extension` Fluent message originally said "Files with this extension will never be found," which is only true for the typo case; a real-but-unsupported extension (e.g. `wav`) *would* be discovered by `discovery.rs` and then fail later at identification/muxing, not "never be found." Caught and reworded during self-review to state both outcomes without overclaiming.
3. **Severity/param choice.** One `UnknownExtension` diagnostic per unmatched entry (not one aggregate diagnostic for the whole list), matching the brief's Step 1 wording ("one `UnknownExtension` warning for `mp4a`") and the existing `walk_exact_languages` per-occurrence style.
4. Verified the "once per batch" property empirically, not just by construction: `suggest()` re-invokes `plan_core` per simulated candidate edit over the *same* `id`/`IdentifyCache` instance passed into `plan_batch`, so the memoized `known_extensions` never re-queries mid-batch. `every_suggestion_survives_the_next_dry_run` (suggestions.rs, exercises exactly this repeated-`plan_core` path) still passes.

## Concerns

- Item 1 above (`Locator.extensions`) is the only open item; it's a scope question, not a defect in what was built. Flagging rather than deciding unilaterally.
- T10's exhaustive param-fixture guard (cross-task constraint C1) will need a fixture entry for `DiagCode::UnknownExtension` with `$extension`/`$known` params; not addressed here per the ledger (T10's job at merge time).
