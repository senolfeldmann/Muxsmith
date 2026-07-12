# Task 10 Report: Catalog param-drift guard + full-key coverage (#16)

## What was implemented

`crates/muxsmith-cli/tests/catalog_completeness.rs` grew from one test to
three, closing the blindspot named in the task: every prior test rendered
`DiagCode` messages with empty args, so an emitter's `.with("param", ...)`
drifting away from its message template's `{ $param }` placeholder was
invisible until it reached a user as literal `{$property}` output.

1. **`fixture_args(code: DiagCode) -> FluentArgs<'static>`**: an exhaustive
   match over all 38 `DiagCode` variants, one arm per code, setting exactly
   the params that code's real emitter site(s) in `muxsmith-core` set (see
   table below). No wildcard arm, so a new `DiagCode` variant is a compile
   error here until it gets a fixture -- the guard grows with the enum by
   construction, not by discipline.
2. **`every_diag_code_renders_without_leftover_placeholders`**: renders
   every `DiagCode` message with its fixture and asserts the output
   contains no `{$` substring.
3. **`ALLOWLISTED_CLI_KEYS` + `allowlisted_cli_key_args`**: the 24 keys in
   `locales/en/cli.ftl` that are rendered directly (not through a
   `DiagCode`), each with its own fixture sourced from its real call site.
4. **`every_cli_ftl_key_is_a_diag_code_or_allowlisted`**: parses
   `locales/en/cli.ftl` with the real Fluent parser (`fluent_syntax`, not a
   hand-rolled regex, so multi-line selector values like `run-job-warning`
   can't produce a false id), and asserts every message id is either a
   `DiagCode` key or on the allowlist; a third key fails the test by name.
   Also checks the allowlist has no stale entries (a key removed/renamed
   from `cli.ftl` that the orphan-scan direction alone can't see) and that
   every allowlisted key renders without a leaked placeholder.

Dependency added: `fluent-syntax = "0.12.0"` as a `muxsmith-cli`
dev-dependency (already resolved transitively via `fluent-bundle` in
`Cargo.lock` at that exact version, so this pins nothing new -- `Cargo.lock`
only gained one new edge, no version bump). Needed because `fluent-bundle`
does not re-export `fluent_syntax::ast`, and matching `Entry::Message` to
enumerate ids requires naming that type.

## DiagCode -> params table (fixture_args, doubles as documentation)

Sourced by grepping every `Diagnostic::error`/`warning`/`info` +
`.with(...)` call site in `crates/muxsmith-core/src/{planner,discovery}.rs`
and `profile/{validate,load,lint}.rs`, cross-checked against each
message's `{ $param }` placeholders in `locales/en/diagnostics.ftl`.

| DiagCode | params |
|---|---|
| UnsupportedProfileVersion | found, supported |
| ParseError | detail, at |
| NoTrackRules | (none) |
| EmptyMatchExpression | (none) |
| EmptyExtensions | (none) |
| InvalidRegex | detail |
| UnknownProperty | property |
| CodecKindExactOnly | condition |
| InvalidPropertyValue | property, value, allowed |
| EmptyMatchList | (none) |
| NotStringProperty | property, actual_type, condition |
| ValueTypeMismatch | property, expected, found |
| UnknownSettableProperty | property |
| InvalidKeyword | found, allowed |
| LocatorConflict | (none) |
| InvalidTemplate | kind, pos |
| UnknownTemplateField | field, allowed |
| UnknownTemplateFilter | name |
| PathSeparatorInTemplate | (none) |
| AttachmentRuleShape | found |
| ProvableOverlap | rule_a, rule_b |
| AmbiguousRule | count |
| OverlappingRules | rule_a, rule_b, track |
| MissingTrack | (none) |
| MissingExternal | (none) |
| AmbiguousExternal | count |
| UnidentifiableSource | detail |
| UnsupportedSource | (none) |
| OutputCollision | path |
| PathSeparatorInRenderedName | name |
| EmptyRenderedName | name |
| SourceOverwrite | path |
| DuplicateIdentifier | identifier, file_a, file_b |
| DonorIsPrimary | donor |
| IgnoredFile | (none) |
| MultipleIdentifierMatches | name |
| UnknownPropertySkew | version |
| SuggestionsCapped | dropped |

All 38 cross-checked one-for-one against their `diagnostics.ftl` template's
placeholder set; no mismatches found (this run of the guard is clean on
the current catalog, as the brief's TDD framing expects).

## `cli.ftl` allowlist scope decision

The brief's own text names "the 8 run-* keys today" as the allowlist. The
current `cli.ftl` (24 keys total) has grown since that line was drafted:
besides the 7 `run-job-*` + `run-summary` keys (8, matching the brief
exactly), it also carries `validate-ok`/`validate-summary`,
`diagnostic-line`/`diagnostic-line-file`, `mkvmerge-not-found`/
`mkvmerge-query-failed`, `identify-failed`/`identify-not-media`/
`identify-track-line`, `dry-run-file`/`dry-run-assignment`/
`dry-run-output`/`dry-run-suggestion`, and `run-joblog-unavailable`/
`-written`/`-incomplete`. None of these are `DiagCode` messages (grepped
and confirmed no key collision with the `DiagCode::ALL` set), so under the
brief's own rule ("either a DiagCode message... or in an explicit
allowlist... or the test fails naming the orphan key") every one of them
has to be allowlisted -- the brief's parenthetical "and future ones"
anticipates exactly this growth. Allowlisting only the original 8 would
make the other 16 fail the test as false-positive orphans on the very
first run, which cannot be the intent.

## TDD / guard evidence (RED, captured then reverted)

**Guard 1 (placeholder-leak detection):** temporarily removed
`args.set("property", "bogus_property")` from the `UnknownProperty` arm
of `fixture_args`, simulating an emitter/message param drift:

```
$ cargo test -p muxsmith-cli --test catalog_completeness
test every_diag_code_renders_without_leftover_placeholders ... FAILED
---- every_diag_code_renders_without_leftover_placeholders stdout ----
thread '...' panicked at crates/muxsmith-cli/tests/catalog_completeness.rs:168:5:
DiagCode message(s) with an unresolved placeholder:
unknown-property: Unknown property "{$property}". It is not part of the mkvmerge identification model.
```

This is the exact failure class the guard exists for (literal `{$property}`
in output). Fixture restored; re-ran green (see below).

**Guard 2 (orphan-key detection):** temporarily appended
`orphan-test-key = An orphan key nobody wired up.` to `locales/en/cli.ftl`:

```
$ cargo test -p muxsmith-cli --test catalog_completeness every_cli_ftl_key_is_a_diag_code_or_allowlisted
test every_cli_ftl_key_is_a_diag_code_or_allowlisted ... FAILED
thread '...' panicked at crates/muxsmith-cli/tests/catalog_completeness.rs:337:5:
cli.ftl key(s) wired to neither a DiagCode nor the allowlist: ["orphan-test-key"]
```

`cli.ftl` restored via `git checkout -- locales/en/cli.ftl` (it was the only
pending change to that file, confirmed via `git status` before reverting).

**GREEN (both fixes reverted):**

```
$ cargo test -p muxsmith-cli --test catalog_completeness
test every_diag_code_has_a_catalog_message ... ok
test every_diag_code_renders_without_leftover_placeholders ... ok
test every_cli_ftl_key_is_a_diag_code_or_allowlisted ... ok
test result: ok. 3 passed; 0 failed
```

## Files changed

- `crates/muxsmith-cli/tests/catalog_completeness.rs`: rewritten, +383/-3
  lines (kept the original `every_diag_code_has_a_catalog_message` test
  verbatim, added `fixture_args`, the placeholder-leak test, the allowlist
  + its fixtures, and the `cli.ftl` coverage test).
- `crates/muxsmith-cli/Cargo.toml`: `fluent-syntax = "0.12.0"` added to
  `[dev-dependencies]`.
- `Cargo.lock`: one new dependency edge (`muxsmith-cli` -> `fluent-syntax`,
  already at the version already resolved transitively).

## Gate results (full gate, from worktree root, all foreground)

- `cargo fmt --all --check`: clean (one arm-wrapping diff auto-fixed by
  `cargo fmt --all` before the final check; committed already formatted)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo test --workspace`: all suites `ok`, zero failures (32 `test
  result: ok` blocks, includes unit + integration + doctests across all
  four workspace crates)
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`
  (exit 0; the printed duplicate-version tree is pre-existing noise,
  unrelated to this change)
- `pnpm install --frozen-lockfile`: ran once (node_modules was missing in
  this worktree), lockfile unchanged
- `pnpm lint`: clean
- `pnpm build`: clean (`vue-tsc --noEmit && vite build`)
- `pnpm check:i18n`: `ok` (exit 0); 12 pre-existing "unused" warnings in
  `gui-*.ftl` are unrelated (`cli.ftl` is explicitly excluded from that
  script by design, per its own header comment)
- `pnpm test:e2e`: 3/3 Playwright specs pass

## Self-review findings

- Confirmed by whole-repo grep that no production code outside
  `muxsmith-core/src` constructs a `Diagnostic` (the two other
  `Diagnostic::error/warning/info` hits outside `muxsmith-core/src` are
  both inside `#[cfg(test)] mod tests` blocks in `i18n.rs` and
  `src-tauri/src/run.rs`), so the fixture table's source grep was
  exhaustive over the real emitter surface.
- Cross-checked every fixture's param names against its message's `{ $x }`
  placeholders in `diagnostics.ftl` by hand (table above); zero mismatches
  on the current catalog, matching the brief's "should pass on the current
  catalogs" TDD framing.
- `run-job-warning`'s plural selector (`{ $count -> [one] ... *[other]
  ... }`) resolves against a `FluentValue::String` fixture (not
  `FluentValue::Number`) since every fixture in this file is string-typed
  by design; this always falls to the `*[other]` branch regardless of the
  count value, which is irrelevant to what this guard checks (leftover
  `{$` placeholders) but means the guard would not itself notice a broken
  plural branch. Out of scope for this task (that's `msg_with_count`'s
  contract, exercised by `run.rs`'s own unit tests, not the catalog
  guard).
- Verified via `git status` that all three demonstrate-the-guard reverts
  (`catalog_completeness.rs`'s `UnknownProperty` arm, `cli.ftl`'s appended
  orphan line) left the working tree in exactly the intended final state
  before running the full gate and committing.
- Confirmed `Cargo.lock`'s diff is a single new dependency edge, not a
  version bump of `fluent-syntax` or anything else (it was already present
  in the lockfile transitively via `fluent-bundle` at 0.12.0).

## Concerns

- **Merge-time note, not a defect:** Task 8's report (already filed in
  this same plan directory) flags that its `dry-run-summary` key (three
  params: `count`, `root`, `extensions`) will need an allowlist entry and
  fixture once merged. That key does not exist in this worktree's
  `cli.ftl` yet (Task 8 lives on a different stream/branch), so this guard
  correctly has no arm for it today. At merge time,
  `every_cli_ftl_key_is_a_diag_code_or_allowlisted` will fail naming
  `dry-run-summary` as an orphan until the merge controller adds it to
  `ALLOWLISTED_CLI_KEYS` and gives it a fixture in
  `allowlisted_cli_key_args` -- this is the guard doing exactly its job
  (per the task brief: "at merge time the exhaustive match will force
  their fixtures - that is the guard working; the controller handles
  those merge-time additions"), not something to design around now. The
  same applies to any other in-flight task adding a new `DiagCode` variant
  (forces a new `fixture_args` arm) or a new `cli.ftl` key (forces a new
  allowlist entry).
- No other concerns; full gate is green.

## Correction (review finding)

The report's claim that "All 38 cross-checked one-for-one ... no mismatches found" and the method was "exhaustive over the real emitter surface" overclaimed the guard's scope. The per-`DiagCode` fixture design renders *one* fixture per code and verifies it matches the template; it does not verify *every* emitter site per code. A single emitter site that omits a param its siblings set remains invisible to this guard — it will leak `{$param}` in production while the guard passes. This divergence (InvalidPropertyValue at `planner.rs:600` emitting without the `allowed` param) was missed by the fixture-per-code method and surfaced only in code review. The guard's actual scope, documented in the `fixture_args` limitation note, is: proof that each DiagCode's fixture matches its template and that template-vs-fixture drift does not exist, not exhaustiveness over every site.

## Merge-reconciliation fixture wave

Post-merge reconciliation (master merging plan55-stream-d): four `DiagCode`
variants and one `cli.ftl` key landed via other streams since this guard
was written, exactly as anticipated in the Concerns section above. The
exhaustive match in `fixture_args` failed to compile
(`non-exhaustive patterns: EmptyPlan, UnknownExtension, SuggestionPartition,
WorkerPanicked`), which is the guard doing its job. Fixtures added, each
sourced from its actual emitter site:

- **`DiagCode::EmptyPlan`** -- `{}` (no params). Emitter:
  `crates/muxsmith-core/src/planner.rs:1118`,
  `Diagnostic::warning(DiagCode::EmptyPlan, "tracks").for_file(&f.source)`
  in `detect_empty_plans`. No `.with(...)` calls; `diagnostics.ftl`'s
  `empty-plan` message has no `{ $x }` placeholders.
- **`DiagCode::UnknownExtension`** -- `extension`, `known`. Emitter:
  `crates/muxsmith-core/src/planner.rs:368-371`, `validate_extension_list`:
  `.with("extension", ext.clone()).with("known", known.join(", "))`.
  Matches `diagnostics.ftl`'s `unknown-extension` placeholders.
- **`DiagCode::SuggestionPartition`** -- `kind`, `count`, `fix`, `files`
  (the `*[group]` default branch). Emitter:
  `crates/muxsmith-core/src/planner.rs:1290-1298`, `partition_for_rule`'s
  group loop: `.with("kind", "group").with("count",
  files.len().to_string()).with("fix", fragment).with("files",
  files.join(", "))`. The same function's overflow branch (lines
  1300-1305) sets only `kind="overflow"` + `dropped`; the guard renders one
  fixture per `DiagCode`, so that branch is not independently exercised
  here. Documented inline in the fixture's own code comment, following the
  file's existing single-fixture-per-code limitation convention
  (`InvalidPropertyValue`'s own noted gap).
- **`DiagCode::WorkerPanicked`** -- `{}` (no params). Emitter:
  `crates/muxsmith-core/src/executor/queue.rs:403`,
  `recover_panicked_worker`: the panic payload is only ever formatted into
  `JobOutcome.errors` as a plain string
  (`format!("{}: job {index}", DiagCode::WorkerPanicked.key())`), never
  passed through `Diagnostic::with(...)`; per its own doc comment the
  panic payload is "developer-diagnostic content ... never carried into
  this code's params." `diagnostics.ftl`'s `worker-panicked` message has
  no placeholders.
- **Allowlist: `dry-run-summary`** -- `count`, `root`, `extensions`. This
  is exactly the key the Concerns section above predicted (Task 8's
  addition). Emitter: `crates/muxsmith-cli/src/commands/mod.rs:107-117`,
  `renderer.msg("dry-run-summary", &[("count", ...), ("root", ...),
  ("extensions", ...)])`. Added to `ALLOWLISTED_CLI_KEYS` and given a
  fixture in `allowlisted_cli_key_args` following the existing pattern.
  Grepped `ALLOWLISTED_CLI_KEYS` against every `cli.ftl` message key by
  hand; `dry-run-summary` was the only key present in the file but absent
  from the array.

Scope: only `crates/muxsmith-cli/tests/catalog_completeness.rs` touched
(26 lines added, 0 removed, 0 changed elsewhere). No production code
needed changes to make any fixture faithful.

### Test results

- `cargo test -p muxsmith-cli --test catalog_completeness`: 3/3 pass
  (`every_diag_code_has_a_catalog_message`,
  `every_diag_code_renders_without_leftover_placeholders`,
  `every_cli_ftl_key_is_a_diag_code_or_allowlisted`).
- `cargo test --workspace`: all green, no failures (full run across every
  crate's unit/integration/doc tests).
- `cargo fmt --all --check`: **fails**, but not on the touched file --
  the only diff is a stray blank line before the closing brace at
  `crates/muxsmith-core/src/executor/queue.rs:1310`. Confirmed
  `catalog_completeness.rs` alone formats clean
  (`rustfmt --check --edition 2021` on just that file exits 0). `queue.rs`
  is one of this merge's still-unmerged ("both modified") paths and was
  not touched per this task's scope constraint.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`: **fails**, also
  outside the touched file -- `crates/muxsmith-core/src/executor/queue.rs:74`
  has a doc comment linking `[\`worker_count\`]`, a private item, which
  `-D rustdoc::private-intra-doc-links` rejects. Same unmerged file as the
  fmt failure above; not touched.

Both fmt and doc failures live entirely in `queue.rs`, one of the two
still-conflicted merge paths (`joblog.rs`, `queue.rs` per `git status`),
outside this task's file scope. Flagging for the merge controller rather
than fixing.
