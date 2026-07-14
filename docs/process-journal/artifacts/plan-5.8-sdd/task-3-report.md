# Task 3 report - D39: catalog selector, allowed-param removal, coupled-comment sweep

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-b`, branch `plan58-b`. All work done there; main tree and other worktrees untouched. Not pushed.

## Step 1: turn the leak test into the failing spec

Extended `invalid_changes_language_diagnostic_renders_without_placeholder_leak` in `crates/muxsmith-cli/tests/catalog_completeness.rs` with two new assertions after the existing no-`{$`-leak check: the rendered en message contains `must be a valid ISO 639 or BCP-47 language code`, and does NOT contain `Allowed values include`.

## Step 2: run to verify failure (foreground)

```
cargo test -p muxsmith-cli --test catalog_completeness invalid_changes_language -- --nocapture
```

Result: FAILED, as expected. Real output:

```
thread 'invalid_changes_language_diagnostic_renders_without_placeholder_leak' (1063620) panicked at crates/muxsmith-cli/tests/catalog_completeness.rs:450:5:
the changes.language InvalidPropertyValue did not render the [language] arm's registry wording: [error] /tmp/.tmpyeYDYE/Show.S01E01.mkv tracks[0].changes.language: Value "zz!" is not valid for property "language". Allowed values include: a valid ISO 639/BCP-47 language code.
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

## Step 3: catalog selector bodies

Replaced `invalid-property-value` in both `locales/en/diagnostics.ftl:43` and `locales/de/diagnostics.ftl:50` with the Fluent select on `$property` (`[language]` arm + `*[other]` arm), verbatim from the brief. The de body is Şenol-reviewed wording; changed nothing else in that file.

## Step 4: remove the prose param, sweep the two emitter-site comments

- `walk_exact_languages` (planner.rs): deleted `.with("allowed", "a valid ISO 639/BCP-47 language code")`.
- `resolve_changes` (planner.rs): deleted the same `.with("allowed", ...)` line and replaced the stale comment with the brief's D39 explanation.
- `report/mod.rs`: updated the `InvalidPropertyValue` rustdoc line inside `diag_codes!` to the brief's verbatim text (closed-domain emitters pass `allowed`, language emissions don't, selector renders registry wording).

**Deviation surfaced (formatting-driven, not content):** the brief placed the new `resolve_changes` comment inline in the method chain, right after `.with("value", scalar_display(value))` (where `.with("allowed", ...)` used to sit) - mirroring the original layout. Applied that way first: it's syntactically valid Rust, but `cargo fmt --all --check` (a mandatory pre-commit gate per `docs/process-conventions.yaml` id `194`) reformats a comment sitting between the last chain call and the closing paren of `diags.push(...)` into a trailing line-comment glued to that line with odd continuation-line indentation - ugly and not what the brief intended visually. Moved the identical comment text to stand alone directly above `diags.push(` (still inside the `if !valid {` block, before the diagnostic construction) instead. `cargo fmt --all --check` is clean with this placement; content and wording are unchanged from the brief, only the anchor line moved. Flagging per the brief's "surface any new pattern/deviation" instruction, not resolving it silently.

## Step 5: fixture switch + narration sweep

- `fixture_args` `InvalidPropertyValue` entry switched to the `*[other]` arm: `("property", "type"), ("value", "text"), ("allowed", "video, audio, subtitles")` - verbatim from the brief.
- Rewrote the `fixture_args` doc comment (:43-47 originally) to describe the selector split: the fixture now exercises the `*[other]` list arm; the `[language]` arm (no `allowed` param) is pinned separately by the site-level leak test.
- Rewrote the leak-test doc comment (:387-395 originally) to state the inverse of the old "before the fix" framing: since D39 both language emitters deliberately carry no `allowed`; the test pins that the `[language]` arm renders complete registry wording with no placeholder leak.

Both rewrites keep the codebase's established `--` (spaced double-hyphen) aside-dash convention already used throughout this file and elsewhere in the Rust sources (planner.rs, command.rs), consistent with the pre-existing local pattern - not the general single-hyphen typography convention. Not flagging this as new; it's the file's/codebase's existing habit, reused as-is.

**Minor note, not a deviation:** the brief's verbatim text for the `report/mod.rs` rustdoc line uses a single spaced hyphen (` - `) for its aside ("language emissions do not - the catalog's..."), which differs from the `--` convention used elsewhere in that same file (e.g. the `QueueWorkerPanic` doc comment two lines below) and in this crate generally. Applied verbatim as instructed since the brief gave exact text for this line; surfacing the inconsistency rather than silently "fixing" it either direction.

## Step 6: diagnosticFluentParams.ts doc comment

Replaced the overclaiming strictness sentence with the brief's verbatim replacement (accepts `Number()`-normalizable spellings like `"1e3"` -> `1000`; safe because wire values are plain-digit Rust `usize` serializations). Code unchanged, per the brief (no unit-test vehicle for a stricter regex until Plan 6's GUI test-harness block).

## Step 7: run the affected layers (foreground, real output)

```
cargo test -p muxsmith-cli --test catalog_completeness
```
```
running 4 tests
test every_diag_code_has_a_catalog_message ... ok
test every_diag_code_renders_without_leftover_placeholders ... ok
test every_cli_ftl_key_is_a_diag_code_or_allowlisted ... ok
test invalid_changes_language_diagnostic_renders_without_placeholder_leak ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
cargo test -p muxsmith-core
```
All 12 test binaries green, 0 failures (unit-test lib binary 116 passed, remaining integration binaries summing to 190 more, all `0 failed`).

```
pnpm check:i18n
```
```
check-i18n: ok (17 source files scanned, 180 catalog ids, 12 unused warning(s), 1 other locale(s) checked for parity against 6 en/ catalog(s)).
```
id parity between en/de holds (the 12 "unused" warnings are pre-existing gui-* keys unrelated to this change). No dead reference or parity breakage from the new select-arm structure.

**Snapshot check:** grepped `crates/muxsmith-cli/tests/snapshots/` (11 `.snap` files) for `Allowed values include`, `allowed.*ISO 639`, `invalid-property-value`, `InvalidPropertyValue` - zero hits. Also ran the full `cargo test -p muxsmith-cli` (67 tests across 9 binaries incl. the insta-backed CLI/dry-run/run snapshot tests) to be certain: all green, no insta snapshot mismatch. **No snapshot file needed updating** - none of the 11 tracked snapshots render this diagnostic.

**Additional gate run (house rule, `docs/process-conventions.yaml` id 194 - "must all pass before every commit, never skipped"):**
- `cargo fmt --all --check`: clean after the comment-placement fix in Step 4 (see deviation above).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `cargo test --workspace`: every binary `0 failed` (re-run after the fmt fix, workspace-wide, includes muxsmith-gui's `src-tauri` crate).

## Step 8: commit

```
git add locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-core/src/planner.rs crates/muxsmith-core/src/report/mod.rs crates/muxsmith-cli/tests/catalog_completeness.rs src/diagnosticFluentParams.ts
git -c commit.gpgsign=false commit -m "fix: language diagnostics render locale-pure via catalog selector, allowed param off the wire (D39)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

Commit: `d0a51a7`. 6 files changed, 43 insertions(+), 22 deletions(-). Working tree clean after commit. Not pushed.

## Summary of surfaced items (not silently resolved)

1. Comment placement in `resolve_changes` moved from mid-chain (as the brief showed) to standalone above `diags.push(`, forced by the mandatory `cargo fmt --check` gate reformatting the mid-chain position into an ugly trailing comment. Same text, different anchor line.
2. `report/mod.rs`'s new rustdoc line uses a single spaced hyphen per the brief's verbatim text, diverging from this file's/crate's otherwise-consistent `--` aside-dash convention. Applied as given; not changed either direction.

## Fix round 1 (review verdict: two Minor findings)

Reviewer adjudication of the surfaced items: (1) the `resolve_changes` comment move stands (rustfmt mangling reproduced empirically, content byte-identical), no change needed; (2) the crate's dominant rustdoc aside convention is the double hyphen `--` (43:12 codebase-wide), the brief's ` - ` spelling was brief-drafting, not an owner decision, so house convention governs.

Fixes applied, one character each, nothing else touched:

1. `crates/muxsmith-core/src/report/mod.rs:88` - the new rustdoc aside: `language emissions do not - the catalog's` -> `language emissions do not -- the catalog's`.
2. `src/diagnosticFluentParams.ts:26` - the new JSDoc sentence: `(e.g. "1e3" -> 1000) -` -> `(e.g. "1e3" -> 1000) --`, matching the two existing `--` asides eight lines above in the same doc block.

Covering checks, run foreground:

```
cargo fmt --all --check
```
Clean, no diff, exit 0.

```
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.93s
   Generated /home/senol/Git/Muxsmith/.worktrees/plan58-b/target/doc/muxsmith_cli/index.html and 5 other files
```
Exit 0; zero warnings in the full log (grep count 0).

```
pnpm lint
```
```
$ eslint .
```
Exit 0, no findings.

Commit (unsigned, staging exactly the two files):

```
git add crates/muxsmith-core/src/report/mod.rs src/diagnosticFluentParams.ts
git -c commit.gpgsign=false commit -m "style: rustdoc/JSDoc asides use the crate's dominant \"--\" spelling (task-3 review fix round 1)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

Commit: `a9915f3`. 2 files changed, 2 insertions(+), 2 deletions(-). Working tree clean after commit. Not pushed.

## Status

DONE

Commits: d0a51a7 (task), a9915f3 (fix round 1)

Test summary: `cargo test -p muxsmith-cli --test catalog_completeness` 4/4 pass, `cargo test -p muxsmith-core` all binaries 0 failed, `pnpm check:i18n` ok (id parity holds), `cargo test --workspace` all binaries 0 failed, `cargo fmt --all --check` / `cargo clippy --workspace --all-targets -- -D warnings` / `cargo deny check` all clean; fix round 1: fmt + `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` + `pnpm lint` all clean; no insta snapshot needed updating.
