# Verify-29: profile_load.rs fully-qualified paths in last two tests

**Finding:** `crates/muxsmith-core/tests/profile_load.rs:131` (idiom, slice F2b) - the last two tests spell out `muxsmith_core::profile::load::from_str` / `...::Format::Yaml` / `...::model::KeepDrop` although lines 1-5 already import `from_str`, `Format`, and `KeepDrop`.

**Verdict: CONFIRMED**

## (a) Code says what the finding claims

Read at HEAD. Lines 1-4 import `Format`, `from_str` (from `profile::load`) and `KeepDrop` (from `profile::model`). Every test through line 120 uses the bare imported names (e.g. line 11 `from_str(REFERENCE, Format::Yaml)`, line 19 `assert_eq!(p.tags.global, KeepDrop::Drop)`). Only `tracks_block_parses_and_unmatched_defaults_to_drop` (lines 131-137) and `tracks_unmatched_keep_parses` (lines 151-157) use the fully-qualified spellings, each forcing rustfmt into a 3-line `let` and a 4-line `assert_eq!`. Claim accurate.

## (b) Replacement is current idiom

The Rust Reference (use-declarations, checked via context7 at rust-lang/reference) states the purpose of `use` directly: "A use declaration creates local name bindings synonymous with other paths, typically to shorten the path required to refer to a module item." Nothing in edition 2024 or Rust 1.96 changes path/import semantics relevant here. The strongest evidence is intra-file: the proposed replacement expressions are byte-identical in shape to what the file's own first five tests already write. `let p = from_str(yaml, Format::Yaml).expect(...)` and `assert_eq!(p.tracks.unmatched, KeepDrop::Drop)` each fit rustfmt's default 100-char width, so the multi-line qualified expressions collapse to one line each as claimed. (Exact count: 10 lines cut, not the finding's 8 - immaterial.)

## (c) Load-bearing difference

Not a duplication finding; no shadowing or name conflict exists that would justify the qualification (the imports are in scope and unambiguous throughout the file).

## (d) Tag

`idiom`, not `yagni`; concrete construct and replacement are named regardless.

## Decision guard

- Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `profile_load`, "qualified", the two test names: no design memo (D1-D35), no IDEAS entry, no ROADMAP entry covers this. Cosmetic-cleanup group K (ROADMAP.md:260-267) and the test-hygiene collection (ROADMAP.md:307-321) list other items in other files; neither mentions this construct.
- Provenance found (not a decision): the qualified paths were copy-pasted verbatim from the plan snippet in `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md:52-70`. That plan itself adds "If `from_str`/`Format`/`load` paths differ, mirror the existing helper calls already in `profile_load.rs`" (line 74), i.e. the plan pointed *toward* the file's import style; the executor kept the snippet's defensive spelling instead. Supports the finding rather than conflicting with it.

Not tracked, not decided against: CONFIRMED.
