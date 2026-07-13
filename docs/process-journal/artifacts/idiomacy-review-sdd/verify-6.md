# Verify-6: scan_primaries triple regex execution (discovery.rs:76)

**Verdict: CONFIRMED**

Finding: `scan_primaries` runs the regex three times per file and carries a cross-call
`expect("first match implies captures")` invariant; replacement is a single
`captures_iter` pass. Slice F1a, tag idiom.

## (a) Code says what the finding claims

Verified at HEAD `2f17880`, `crates/muxsmith-core/src/discovery.rs:76-88`:

- L76-80: `re.find_iter(name)` + `matches.next()` — regex search 1 (first match).
- L81: `matches.next().is_some()` — regex search 2 (multiplicity, scans the remainder).
- L88: `let caps = re.captures(name).expect("first match implies captures");` — regex
  search 3, a full re-scan from position 0, redundant with search 1, held together only
  by the cross-call invariant in the `expect`.

Three regex executions per matching file, one fully redundant. Claim accurate.

## (b) Replacement is current idiom for the pinned toolchain

regex crate pinned at `1.12.4` (muxsmith-core/Cargo.toml:11, Cargo.lock agrees).
Checked against current official docs (context7, /rust-lang/regex):

- `captures_iter` is the documented standard for iterating matches when capture groups
  are needed (README examples use exactly this shape). Captures ARE needed here (the
  groups loop at L90-100), so a captures-based search is unavoidable; the current code
  pays for it *and* a preceding `find_iter` scan.
- `Captures::get(0)` / `&caps[0]` is the documented whole-match access; group 0 is
  guaranteed present on any `Captures` value (documented within-value invariant), so
  the cross-call `expect` disappears rather than moving.
- Borrowck: `Captures<'h>` borrows the haystack, not the iterator, so
  `it.next()` for the multiplicity check after binding the first `caps` is valid.

Semantics preserved: `captures_iter` yields the same non-overlapping leftmost-first
match sequence as `find_iter`, so the IgnoredFile (no match), MultipleIdentifierMatches
(second `next()` is some), and "first match used" behaviors are identical. Regex
searches drop from 3 to 2 in the matching path (the second `next()` computing captures
for a potential second match is marginally pricier than a plain find, but strictly
cheaper than the extra full `re.captures` re-scan it replaces).

## (c) Duplication difference — n/a

No duplication claim in this finding.

## (d) yagni gate — n/a

Tag is `idiom`, and both concrete construct and concrete replacement are named anyway.

## Decision guard

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for
`scan_primaries`, `find_iter`, `captures`, `discovery.rs`, `MultipleIdentifierMatches`,
plus broader `regex|discovery|primar`:

- Spec hit: `2026-07-08-muxsmith-v1-design.md:274` — the diagnostics table defining
  `MultipleIdentifierMatches` ("first match used"). Defines semantics only; the
  replacement preserves them. Not a conflict.
- ROADMAP regex entries are all matcher.rs-scoped: compile cache (matcher.rs:74,
  residue R4, v1.x) and subsumption-lint (D2 scope cut). Different file, different
  construct (compilation caching / value linting, not per-file re-matching in
  discovery). Not tracked, no conflict.
- IDEAS.md hit is a feature idea (forced-flag recognition regex), unrelated.
