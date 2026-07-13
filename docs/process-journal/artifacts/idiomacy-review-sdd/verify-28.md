# Verify-28: prop_language.rs one-entry BTreeMap hand-rolls

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/tests/prop_language.rs` — `track_with_language` (128-129) and `language_expr` (139-140) hand-roll one-entry maps via `let mut map = BTreeMap::new(); map.insert(...)`; replacement `BTreeMap::from([(...)])`, tag `stdlib`, slice F2b.

## Gate checks

- **(a) Code says what the finding claims — PASS.** Read at HEAD (`2f17880`). Lines 128-129: `let mut properties = BTreeMap::new(); properties.insert("language".to_string(), PropValue::Str(value.to_string()));`. Lines 139-140: `let mut map = BTreeMap::new(); map.insert("language".to_string(), Scalar::Str(value.to_string()));`. Exactly the cited construct at both sites.
- **(b) Replacement is current idiom for the pinned toolchain — PASS.** Verified two ways, not from training memory:
  1. Official std docs (doc.rust-lang.org, fetched 2026-07-12): `impl<K, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V> where K: Ord`, stable since **Rust 1.56.0**, with a doc example using exactly this form (`BTreeMap::from([(1, 2), (3, 4)])`).
  2. Compiled and ran the concrete replacement (`BTreeMap::from([("language".to_string(), PropValue::Str(value.to_string()))])`) with the repo-pinned toolchain: `rustup run 1.96.1 rustc --edition 2024` — compiles clean, runs correctly.
- **(c) Load-bearing difference — N/A.** The finding does not propose merging the two sites; it flags the same unidiomatic construct twice, each inlined independently with its own value type (`PropValue::Str` vs `Scalar::Str`), which the replacement text already distinguishes.
- **(d) yagni completeness — N/A.** Tag is `stdlib`.

## Decision guard

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `prop_language`, `track_with_language`, `language_expr`, `BTreeMap::from`, one-entry/hand-rolled map construction: no hit. ROADMAP's cosmetic-cleanup group K and the test-hygiene collection (B-minors) do not contain this construct. The "Whole-codebase idiomacy review" ROADMAP entry is the umbrella for this pass itself; its NAMED INPUTS list does not include this finding, so it is not separately tracked.

## Notes for triage (not refutation)

- `lines_cut: 4` is approximately right: 4 hand-roll lines become 1-2 `from(...)` lines per site depending on rustfmt wrapping (the `PropValue::Str` line lands at exactly 100 chars); realistic net is 2-4 lines.
- The identical mut+insert one-entry pattern also exists in `tests/prop_matcher.rs` (84-85, 93-94, 102-103), `tests/prop_planner.rs` (42-43, 51-52, 106-107, 176-177, 245-246, 281-282), and `src/planner.rs` (1761-1762, 1767-1768, 1773ff). If this finding is applied, the fix should sweep those sites in the same pass for consistency; a lone converted site in one test file would leave the codebase style split.
