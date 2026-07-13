# Verify-7: matches! with fully-qualified KeepDrop path (planner.rs:714)

**Verdict: CONFIRMED** (at HEAD 2f17880)

## Finding under test

`crates/muxsmith-core/src/planner.rs:714` — `matches!(profile.tracks.unmatched, crate::profile::model::KeepDrop::Keep)` uses a fully-qualified variant path although `KeepDrop` is imported (line 19) and derives `PartialEq`; siblings `resolve_tags` (:987) and `resolve_attachments` (:1084) use `== KeepDrop::Keep`. Proposed replacement: `let keep_unmatched = profile.tracks.unmatched == KeepDrop::Keep;`

## Checks

### (a) Cited code says what the finding claims — PASS

- planner.rs:714-717 reads exactly as claimed: `matches!` over the fully-qualified path `crate::profile::model::KeepDrop::Keep`, spanning 4 lines.
- planner.rs:19 imports `KeepDrop` from `crate::profile::model` (same `use` group as `ChaptersCfg`, `CollisionPolicy`, ...), so the short path is in scope at line 714.
- `KeepDrop` (profile/model.rs:181-190) derives `PartialEq, Eq, Copy` and is a fieldless two-variant enum; `TracksCfg.unmatched` is a plain `KeepDrop` (model.rs:310), not an `Option`. The replacement compiles and is semantically identical.
- Sibling sites confirmed: planner.rs:987/:988 (`resolve_tags`) and :1084 (`resolve_attachments`) both compare with `== KeepDrop::Keep` using the short path. 2 of 3 comparison sites in the same file already use the `==` form; line 714 is the outlier.
- `lines_cut: 3` is accurate (4 lines -> 1).

### (b) Replacement is current idiom for the pinned toolchain (Rust 1.96.1 / edition 2024) — PASS

Checked against current sources, not training memory:

- The std docs for `matches!` (doc.rust-lang.org/std/macro.matches.html) present it as a pattern-testing macro (ranges, bindings, `if` guards); nothing positions it as the equality idiom for `PartialEq` types.
- Clippy's `equatable_if_let` lint direction (rust-lang/rust-clippy PR #7777, issue #1716) is explicit: when the type implements `PartialEq`, the equality form is the preferred wording; pattern forms (`if let` / `matches!`) are the fallback for types *without* `PartialEq` or for real patterns. `KeepDrop` derives `PartialEq, Eq, Copy`, so `==` is the idiomatic form here.
- The fully-qualified `crate::profile::model::` path with the item already imported is unidiomatic regardless of the macro-vs-operator question.
- Verified installed toolchain matches the pin: rustc 1.96.1, rust-toolchain.toml `channel = "1.96.1"`.

### (c) Load-bearing difference between the sites — NONE

The three sites are behaviorally parallel `KeepDrop -> bool` mappings. The differing serde defaults (tracks defaults to `Drop`, attachments to `Keep`) are a config-schema concern and irrelevant to how the comparison expression is written.

### (d) yagni gate — N/A

Tag is `idiom`, and a concrete construct plus concrete replacement are named anyway.

## Decision guard — no conflict, not tracked

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `KeepDrop`, `keep_unmatched`, `matches!`, `planner.rs`, and qualified-path wording:

- Plan-3.5 memo addendum R3 mentions `keep_unmatched` only as a serialized plan wire-format field (D20/D21). The replacement keeps the variable and the serialized field untouched; only the bool computation changes. No conflict.
- ROADMAP cosmetic-cleanup group K's planner.rs entry targets the eager chapters/attachments resolve on the discarded-plan path (planner.rs:541ff) — different construct, different location. This finding is not tracked there or anywhere else.

Sources:
- [std::matches! docs](https://doc.rust-lang.org/std/macro.matches.html)
- [rust-clippy PR #7777 — upgrade equatable_if_let to style](https://github.com/rust-lang/rust-clippy/pull/7777)
- [rust-clippy issue #1716 — lint if-lets better worded as equality](https://github.com/rust-lang/rust-clippy/issues/1716)
