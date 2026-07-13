# Verify-14: CODEC_KIND_NAMES hand-duplicates CODEC_KINDS keys (capability/mod.rs:126)

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-core/src/capability/mod.rs:126` — `CODEC_KIND_NAMES` hand-re-lists all 17 `CODEC_KINDS` keys; sync guaranteed only by the `codec_kind_domain_matches_kinds` test (mod.rs:220). Proposed: derive via `std::sync::LazyLock`, delete the sync test.

## Checks

### (a) Cited code says what the finding claims — yes

- mod.rs:103-121: `CODEC_KINDS` has exactly 17 entries.
- mod.rs:126-129: `CODEC_KIND_NAMES` hand-lists the same 17 names in the same order.
- mod.rs:123-125: the doc comment itself admits the arrangement: "Kept in sync with `CODEC_KINDS` by the `codec_kind_domain_matches_kinds` test".
- mod.rs:220: that test is the only sync mechanism; it asserts exact element-and-order equality, so the duplication is pure (no load-bearing difference, ruling out (c)).
- Consumers: mod.rs:64 (`matchable_domain`) and tests/prop_matcher.rs:143 (`.to_vec()`). No const-context consumer that a `LazyLock` would break.

### (b) Replacement is current idiom — yes, verified empirically on the pinned toolchain

`std::sync::LazyLock` confirmed in current stable std docs (context7, doc.rust-lang.org/stable): const-fn `new` (legal in a `static` item), `Deref` to the inner value. It is the std-native replacement for `lazy_static`/`once_cell::sync::Lazy` — exactly the repo's "native over dependency" directive.

Compile-tested in a scratch copy of the repo (pinned rustc 1.96.1, edition 2024), applying the replacement verbatim:

- `cargo build -p muxsmith-core`: clean.
- `cargo test -p muxsmith-core --lib`: 115 passed (sync test deleted).
- `cargo test -p muxsmith-core --test prop_matcher`: 7 passed — the external consumer `select(CODEC_KIND_NAMES.to_vec())` compiles unchanged via LazyLock -> Vec -> slice autoderef, as claimed.
- `cargo clippy -p muxsmith-core --all-targets -- -D warnings`: clean (matters: the repo gates on `-D warnings`).

**One-character caveat**: the finding says `matchable_domain` "keeps returning `Some(&CODEC_KIND_NAMES)`", but the current source is `Some(CODEC_KIND_NAMES)` (no `&`). The call site needs `&` added; deref coercion (`&'static LazyLock<Vec<&str>>` -> `&'static [&str]`) then works, return type unchanged. The finding's replacement text specifies the correct target form, so this is an imprecision in "keeps", not an error in the proposal. Confirmed by the compile: without `&` it is E0308, with `&` it builds.

### (c) Load-bearing difference — none

The sync test asserts exact equality (content and order); the derived version preserves iteration order of `CODEC_KINDS`. Diagnostics that render the domain list are unaffected.

### (d) tag=dup, not yagni — n/a.

## Decision guard — no conflict, not tracked

- Specs D1-D35: D1 (`codec_kind` exact-only) and D2 (closed-domain checks) govern semantics, not the representation of the name list. No decision mandates a hand-listed static.
- The Plan-2 plan doc (2026-07-09) contains the hand-listed static in its TDD steps — implementation history, not a decision memo.
- ROADMAP.md: not in cosmetic-cleanup group K, not in the test-hygiene collection (B11 is a different duplication: RECENT_PROFILES_CAP TS/Rust). The "Whole-codebase idiomacy review" entry is the umbrella commissioning this very review, not a prior tracking of this finding.
- IDEAS.md: no hit.

## Note (non-blocking)

An even stricter alternative exists: a const-evaluated `[&str; CODEC_KINDS.len()]` built with a `while` loop in a const block — zero runtime init, usable in const contexts. LazyLock is nonetheless squarely current idiom and the simpler diff; either kills the duplication and the sync test.
