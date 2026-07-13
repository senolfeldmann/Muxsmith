# verify-24: command_integration.rs:121 — Vec<String> map/collect boilerplate

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-core/tests/command_integration.rs:121-182`: `let expected: Vec<String> = [...].into_iter().map(String::from).collect();` followed by `assert_eq!(command(plan), expected)`. Proposed: `assert_eq!(command(plan), ["--output", &output_disp, ...])` directly, relying on LUB coercion of the mixed `&'static str` / `&String` elements to `[&str; N]`.

## Checks

### (a) Cited code says what the finding claims — yes

Read at HEAD (2f17880). Lines 121-180 are exactly the described binding (62-element array, mixed `"literal"` elements and `&output_disp` / `&primary_disp` / `&donor_disp` which are `&String`), line 182 is `assert_eq!(command(plan), expected);`. One site in this file; `tests/command.rs` carries the same `.into_iter().map(String::from).collect::<Vec<_>>()` tail at 10 sites (all-static-literal variants).

### (b) Replacement is valid current idiom on the pinned toolchain — yes, verified empirically

Not taken from memory: a minimal repro mirroring the exact shape (no type annotation, element type driven only by the `PartialEq` inference variable, mixed `&'static str` literals and `&String` references, including non-ASCII `"0:Türkçe"`) was compiled and run on **rustc 1.96.1 (31fca3adb 2026-06-26), `--edition 2024`** (matches `rust-toolchain.toml` channel 1.96.1 and workspace `edition = "2024"`):

- Compiles clean, assertion passes: the array elements LUB-coerce to `&str` (deref coercion `&String -> &str` inside `CoerceMany`), and `impl<T: PartialEq<U>, const N: usize> PartialEq<[U; N]> for Vec<T>` bridges `Vec<String> == [&str; N]`.
- `clippy-driver -W clippy::all -W clippy::pedantic` on the repro: zero diagnostics, so the repo's `-D warnings` gate is not at risk.

Comparing a `Vec<String>` against a `[&str; N]` literal in `assert_eq!` is the std-supported idiom these `PartialEq` impls exist for; the intermediate `expected` binding plus `map(String::from)` tail adds nothing.

### (c) Load-bearing difference between duplication sites — none

The `command.rs` sites are all-static (`vec![...]` of pure `&'static str`); this site interpolates runtime `String` bindings. That difference is precisely what the LUB-coercion claim covers, and it is verified above. Failure-message quality is equivalent (`Debug` of `[&str; N]` vs `Vec<String>` renders the same list).

### (d) yagni gate — n/a (tag is `idiom`, and a concrete construct + replacement are named anyway).

## Decision guard — no conflict, not tracked

- `docs/ROADMAP.md` sweep group K (cosmetic cleanup) names `command_integration.rs` only for the **stale "Two tests:" module doc at line 4** — a different construct; the line-121 boilerplate is not part of that entry.
- ROADMAP "Test-hygiene collection (docs-tree B-minors)" is a closed B1-B11 enumeration; none touches argv-assert conversion style.
- `docs/IDEAS.md` and `docs/superpowers/specs/*.md` (D-memos): greps for `String::from`, `map(String`, `Vec<String>`, `expected.*argv`, `command_integration` hit only the D-memo statements that `command` is `Plan -> Vec<String>` with golden tests — a statement about the function's signature and test strategy, not a prescription of the `expected`-binding style. No recorded decision mandates the current form.

## Notes for the fixer

The same simplification applies even more trivially to the 10 all-static sites in `tests/command.rs` (finding scope is this one site, per its own text). `lines_cut=4` is accurate for this site (drop the `let expected: Vec<String> =` head and the 3-line `.into_iter().map(String::from).collect();` tail, inline into the assert).
