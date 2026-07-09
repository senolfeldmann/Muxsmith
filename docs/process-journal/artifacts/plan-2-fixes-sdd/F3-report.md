# F3 report: validate rejects a present-but-empty any/not list (decision #2)

## Status

DONE

## Commit

a67879e -- fix(core): reject empty any/not match lists (F3, decision #2)

## What changed

`crates/muxsmith-core/src/profile/validate.rs`, in `validate_expr`: when
`expr.any` is `Some(v)` with `v.is_empty()`, push
`Diagnostic::error(DiagCode::EmptyMatchList, format!("{path}.any"))`; same
for `expr.not` at `{path}.not`. Placed immediately before the existing
recursion loops for `any`/`not`, so an empty list is flagged once (no
sub-expressions to recurse into) and a non-empty list is unaffected -- only
its elements get the usual recursive validation. `None` (omitted key) is
untouched, matching spec 4.3: absence is not the same as an empty list.

The `DiagCode::EmptyMatchList` variant, its `"empty-match-list"` wire key,
and the Fluent message (`locales/en/diagnostics.ftl`) already existed from
F2 (commit d9422b3); this task only wires the check into `validate_expr`.

## Test-first

Added to `crates/muxsmith-core/tests/validate_hardening.rs`:

- `empty_any_list_is_empty_match_list`: `match: { any: [] }` -> contains `EmptyMatchList`.
- `empty_not_list_is_empty_match_list`: `match: { not: [] }` -> contains `EmptyMatchList`.
- `populated_any_and_not_are_not_empty_match_list`: `any`/`not` each with one
  sub-expression -> does not contain `EmptyMatchList`.

Confirmed red first: ran `cargo test --workspace --test validate_hardening`
before implementing; the two empty-list tests failed
(`assertion failed: codes(&y).contains(&DiagCode::EmptyMatchList)`), the
populated-list test passed trivially (it asserts absence, which already
held pre-implementation). After implementing, all 9 tests in that file pass
(6 pre-existing + 3 new).

## Verification

- `cargo test --workspace`: all crates green (no failures in any test binary).
- `cargo fmt --all --check`: clean, exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no warnings.
- Diff is ASCII-only (`grep -n '[^\x00-\x7F]'` over both changed files: no matches).
- No new public items; `#![deny(missing_docs)]` unaffected.

## Scope notes

`HANDOFF.md` was present as an untracked file in the working tree at task
start (unrelated to F3) and was left untouched; only the two files above
were staged and committed.
