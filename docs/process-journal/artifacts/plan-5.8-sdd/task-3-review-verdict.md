# Task 3 review verdict - D39: catalog selector, allowed-param removal, coupled-comment sweep

Reviewer: independent task reviewer (fresh context, no implementation involvement).
Commit reviewed: `d0a51a7` (single commit, `ed727af..d0a51a7`).
Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-b`, branch `plan58-b`.

## Verification performed (not taken from the report)

- `git log -1 --format="%GG / %G?"` on `d0a51a7`: empty signature, `%G?=N` -> genuinely unsigned. Trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present. Matches the binding constraint.
- Character-by-character diff of both catalog bodies (en `locales/en/diagnostics.ftl:43-46`, de `locales/de/diagnostics.ftl:50-53`) against the decisions doc's PROPOSAL blocks (`docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md:170-178`), including indentation (`cat -A`): byte-identical modulo the doc's extra 6-space nesting offset (a constant +4/+3 delta preserved in both). Exact match.
- `report/mod.rs:88` rustdoc line vs. the brief's Step-4 text: exact Python string-equality check, `True`.
- Ran `cargo test -p muxsmith-cli --test catalog_completeness` myself: 4/4 pass, output matches the report verbatim.
- Ran `pnpm check:i18n` myself from worktree root: `ok`, id parity holds, same 12 pre-existing gui-* warnings as reported.
- Ran `cargo test -p muxsmith-core` myself: all binaries `0 failed` (116+15+... all green).
- Ran `cargo fmt --all --check`: clean, no diff.
- Grepped all `InvalidPropertyValue` emitter sites in `crates/`: exactly three - `profile/validate.rs:283` (closed-domain, `allowed` via `domain_hint`, **unchanged**, confirmed), `planner.rs:442` (`walk_exact_languages`, `allowed` removed, confirmed), `planner.rs:816` (`resolve_changes`, `allowed` removed, confirmed). No other emitter site exists that the task could have missed.
- Confirmed `.for_file()` sets a separate `Diagnostic.file` field, not a `params` entry (`report/mod.rs:191-266`), so the language-arm params truly are `{property, value}` only, nothing extra smuggled in via the builder chain.
- Grepped the commit diff for em-dash/en-dash/curly-quotes/ellipsis: none. ASCII-only holds (German umlauts in the de catalog are expected content, not a typography violation).
- Grepped tracked `.snap` files for any reference to this diagnostic: zero hits, confirming no snapshot update was needed (matches the report's claim).
- Reconstructed the brief's literal in-chain comment placement in an isolated scratch file and ran `rustfmt --edition 2024` on it: **empirically reproduces** the claimed mangling - the comment gets glued as a trailing line-comment on `.with("value", scalar_display(value)),` with badly misaligned continuation lines. The implementer's claim is not an assertion, it's a checked fact.
- Quantified the codebase's aside-dash convention: doc-comment (`///`) lines using a spaced double-hyphen aside (`... -- ...`) occur **43** times across `crates/`; genuine single-hyphen asides (filtered for arithmetic/literal false positives) occur **12** times (13 including the new line itself). Within `report/mod.rs` alone, prior to this diff there was exactly **one** double-hyphen aside (`QueueWorkerPanic`, line 183) and zero single-hyphen asides - a 1-0 sample, not the "otherwise-consistent" pattern the report describes for that file in isolation, but codebase-wide the double-hyphen convention is unambiguous (43:12, roughly 3.6:1).

## Findings

### Finding 1 (Minor, code quality only - not spec compliance)
- **File:line:** `crates/muxsmith-core/src/report/mod.rs:88`
- **What's wrong:** the new rustdoc line's aside uses a single spaced hyphen (`... do not - the catalog's ...`). The crate's dominant aside-dash convention (43 vs 12 codebase-wide, see verification above) is a spaced double-hyphen (`... do not -- the catalog's ...`). This is the same line the implementer already flagged as a "minor note, not a deviation" - the review's job here is to give the ruling the report explicitly asked for.
- **Suggested fix:** change ` - the catalog's` to ` -- the catalog's` on that line. One-character-class edit, no retest needed (doc comment only).

### Finding 2 (Minor, code quality only - not spec compliance; not previously surfaced by the implementer)
- **File:line:** `src/diagnosticFluentParams.ts:26`
- **What's wrong:** the new sentence `... "1e3" -> 1000) - acceptable because ...` uses the same single-spaced-hyphen aside, while the *same JSDoc block*, eight lines above (`src/diagnosticFluentParams.ts:4` and `:7`), already uses this file's own established double-hyphen aside twice (`spec 5.2/8.4 -- core stays prose-...`, `the render boundary -- ...`). Same file, same comment block, same drift as Finding 1, just not flagged in the report.
- **Suggested fix:** change `1000) -` to `1000) --` on that line.

No Critical or Important findings. No other deviation from `docs/conventions.yaml` / `docs/process-conventions.yaml` found (checked `core-37-prose-free-core`, `ci-06-per-commit-gate`, `proc-05-commit-signing`, `proc-07-verify-against-source`, `testing-si3-run-binary` - all hold).

## The two flagged judgment calls - adjudicated

### 1. Comment moved from mid-chain to standalone above `diags.push(` in `resolve_changes`
**Verdict: placement is correct, content is verbatim-identical to the brief.**

Checked the pre-diff file (`git show ed727af:...planner.rs`): the *old* comment sat between `.with("value", ...)` and the now-deleted `.with("allowed", ...)` - a position that only existed because it was justifying the call right after it. That anchor is gone along with the deleted call. I reproduced the brief's literal in-chain placement in isolation and ran `rustfmt --edition 2024` on it: it independently confirms the exact mangling the report describes (trailing comment glued to the `.with("value", ...)` line, continuation lines mis-indented). The chosen fix - a standalone comment above the whole `diags.push(...)` statement, still inside `if !valid {` - is arguably *more* idiomatic Rust than the original (commenting on an entire statement from above beats embedding prose inside a builder chain), passes `cargo fmt --all --check` cleanly (verified), and the four-line text is byte-identical to the brief's replacement. No fix needed.

### 2. `report/mod.rs` rustdoc line's hyphen style
**Verdict: the line should carry `--`, not ` - `.**

The brief's verbatim text used a single spaced hyphen. Codebase-wide this file's crate uses double-hyphen asides at roughly 3.6:1 over single-hyphen ones (43:12); the single-hyphen instances that do exist are mostly arithmetic operators or literal strings misread as asides, not genuine prose asides. Within `report/mod.rs` itself the sample is too small (1 prior instance) to itself certify "this file's convention" one way or the other, so the correct authority is the crate-wide dominant pattern, which favors `--` decisively. Unlike the Fluent catalog bodies (explicitly marked PROPOSAL, "validated pre-merge against @fluent/bundle... during design review" - owner-approved, verbatim-sacrosanct), this rustdoc sentence carries no such provenance; it is ordinary brief-authored prose and should have been house-styled rather than typed literally. Correctly flagged by the implementer rather than silently resolved either way (per the "surface deviations, don't silently resolve" convention) - but flagging without ruling leaves the defect in the tree. Ruling: change to `--` (Finding 1).

## Verdicts

**(a) Spec compliance: PASS.** Both catalog bodies match the decisions doc's PROPOSAL strings character-for-character including indentation. The wire-format change is exactly as specified: `walk_exact_languages` and `resolve_changes` carry `property`+`value` only for property=language; `profile/validate.rs`'s closed-domain emitter is untouched and still carries `allowed`. Both locales changed in the same commit. The leak test asserts the three required conditions (no `{$` leak, contains the ISO-639/BCP-47 sentence, does not contain "Allowed values include") and passes for real against `plan_batch`'s actual emitter output, not a fixture. The fixture switched to the `*[other]` arm verbatim. The TS change is comment-only (verified: no line outside the JSDoc block changed). Commit is a single unsigned commit with the correct trailer. All required tests were re-run independently and are green (`cargo test -p muxsmith-cli --test catalog_completeness`, `pnpm check:i18n`, `cargo test -p muxsmith-core`, `cargo fmt --all --check`).

**(b) Code quality: PASS WITH TWO MINOR NITS.** Both nits (Findings 1 and 2) are the same class of drift - a brief-authored aside sentence defaulting to a single spaced hyphen where its host file's own established convention is a double hyphen - and both are single-character-class fixes with zero behavioral or test surface. Everything else (comment placement fix in `resolve_changes`, fixture switch, narration sweep, snapshot-none-needed check, gate discipline) is sound and, on the comment-placement question, arguably improves on the brief's original layout.

## Harvest (for the project's convention ledger, not acted on here)

- **rustfmt-vs-inline-chain-comment.** A bare comment sitting between a builder chain's last method call and the closing paren of the enclosing call/macro (e.g. `diags.push(...)`) gets reflowed by `rustfmt` into an ugly trailing line-comment with misaligned continuation lines (empirically reproduced this session). Emerging house pattern: put such a comment as a standalone line above the whole statement, never inline at the tail of a chain. One occurrence so far (agent-emergent, technical-code) - below the count-3 promotion threshold, but worth watching for a second instance.
- **Brief-authored replacement prose skips the target file's own dash convention.** Twice in this one task (`report/mod.rs:88`, `diagnosticFluentParams.ts:26`), plan-authored "replace this line with exactly this text" instructions used a single spaced hyphen in a file whose own established convention (checked empirically, not assumed) is a double hyphen. Candidate process-ledger note: task briefs that hand down verbatim replacement *prose* (as opposed to owner-approved catalog/spec strings) should either check the target file's existing aside convention first, or explicitly mark the text as house-style-adjustable rather than literal, so implementers don't have to choose between silently "fixing" a brief and shipping a known inconsistency.
- **Select-on-existing-discriminating-param as the default i18n technique.** D39's core move - branch catalog wording on `$property`, which already discriminates the emission sites, instead of adding a new wire element (`allowed-kind`) or a new `DiagCode` - is a clean, reusable principle beyond this one diagnostic. Worth promoting to its own `i18n-*` ledger entry (alongside `i18n-05-plural-selectors`) if the same shape recurs on another diagnostic.

## Round-1 verdict (superseded by re-review below)

NEEDS FIXES

Two Minor, trivial (single hyphen-style edit each, no retest required) findings: `crates/muxsmith-core/src/report/mod.rs:88` and `src/diagnosticFluentParams.ts:26`, both changing a spaced single hyphen to the crate's/file's established double-hyphen aside. Everything else - spec compliance, wire-format scope, bilingual sync, test coverage, gate discipline, both flagged judgment calls - checks out.

## Re-review round 1 (fix commit a9915f3)

Delta reviewed: `d0a51a7..a9915f3`, one commit. Independently verified, not taken from the fix report:

- **Both findings resolved.** `git show a9915f3` touches exactly two lines: `crates/muxsmith-core/src/report/mod.rs:88` (` do not - the catalog's` -> ` do not -- the catalog's`) and `src/diagnosticFluentParams.ts:26` (`1000) -` -> `1000) --`). These are precisely the two lines Findings 1 and 2 named, with precisely the suggested fix and no collateral edits on either line.
- **Nothing else changed.** The commit's full diff is 2 files, 2 insertions, 2 deletions - both hunks comment-only, zero behavioral surface. Working tree clean after the commit; `HEAD` = `a9915f3`.
- **Commit hygiene holds.** `%G?` = `N` (genuinely unsigned), trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present, message accurately scoped (`style: ... task-3 review fix round 1`).
- **Covering checks re-run myself:** `cargo fmt --all --check` clean; `RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-core --no-deps` clean (the changed rustdoc line compiles warning-free); `pnpm lint` (eslint) clean. Comment-only delta, so no test re-run required beyond these.

Both findings closed, no new findings.

## Final verdict

APPROVED
