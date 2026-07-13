# Verify F1a: placeholder Assignment literal duplicated six times in resolve_file

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/src/planner.rs`, placeholder `Assignment` literal
(`track_id: None, track_kind: None, changes: vec![]`) duplicated six times in
`resolve_file`; replacement `Assignment::unmatched(rule_index, source)`.

## (a) Cited code matches the claim

Read at HEAD. All six sites exist and are structurally identical seven-line
literals pushing an unmatched-rule placeholder:

| Site | Lines | `source` argument | Context |
|---|---|---|---|
| 1 | 526-532 | `primary.path.clone()` | external locator: 0 hits |
| 2 | 563-569 | `primary.path.clone()` | donor container unsupported |
| 3 | 583-589 | `primary.path.clone()` | donor unidentifiable |
| 4 | 603-609 | `primary.path.clone()` | ambiguous external (n hits) |
| 5 | 646-652 | `source_path` | match count 0 (MissingTrack path) |
| 6 | 675-681 | `source_path` | match count n (AmbiguousRule path) |

Every site sets `rule_index: ri`, `track_id: None`, `track_kind: None`,
`changes: vec![]`. The finding's line numbers (526, 563, 583, 603, 646, 675)
are the literals' opening lines. Accurate.

## (b) Replacement is current idiom

Checked against the Rust API Guidelines via context7
(rust-lang.github.io/api-guidelines, predictability / C-CTOR): associated-function
constructors, including ones taking arguments (`Box::new`), are the documented
standard; a domain-named constructor (`Assignment::unmatched`) is the same
convention as `Duration::from_secs` / `Vec::with_capacity`. Nothing in edition
2024 / Rust 1.96 changes this. The codebase already uses exactly this pattern
at the same call sites: `Diagnostic::error(...)` / `Diagnostic::warning(...)`
are named associated constructors. The replacement is consistent with both
ecosystem and project idiom.

Side benefit, not load-bearing for the verdict: the field doc on `track_kind`
(line 51, "`None` exactly when `track_id` is `None`") becomes true by
construction on this path instead of by six-fold repetition.

## (c) No load-bearing difference between sites

The only variation across the six literals is the `source` value
(`primary.path.clone()` at sites 1-4, `source_path` at sites 5-6) and that is
precisely the second parameter of the proposed constructor. `rule_index` is
`ri` everywhere. The three placeholder fields are byte-identical. No site
carries a hidden semantic difference the constructor would flatten.

`Assignment` has all-public fields and is also literal-constructed in
`tests/command.rs` and `tests/command_integration.rs`; the constructor is
additive and breaks neither.

## (d) N/A

Tag is `dup`, not `yagni`.

## Decision guard

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for
`Assignment`, `planner.rs`, constructor/literal/placeholder decisions:

- ROADMAP cosmetic-cleanup group K names planner.rs:541ff, but for a different
  construct (eager chapters/attachments resolve on the discarded-plan path).
- The "Whole-codebase idiomacy review" ROADMAP entry is the parent of this very
  review pass, not a pre-existing tracker for this finding.
- Plan-3 design memo discusses `Assignment` gaining `changes`; no decision to
  keep literal construction or forbid a constructor.
- IDEAS.md is product-scope only; no hit.

No conflict, not already tracked.

## Estimate check (informational)

Six 7-line literals (42 lines) become six one-line pushes; a constructor of
roughly 8-10 lines including doc comment nets ~26 lines cut. The claimed
`lines_cut: 26` and `deps_cut: 0` are plausible.
