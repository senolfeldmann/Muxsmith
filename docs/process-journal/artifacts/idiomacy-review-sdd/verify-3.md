# Verify-3: extension_matches hand-rolls eq_ignore_ascii_case (discovery.rs:187)

**Verdict: CONFIRMED**

## Finding under test

`extension_matches` (crates/muxsmith-core/src/discovery.rs:187) hand-rolls case-insensitive comparison by allocating `to_ascii_lowercase` per candidate, against pre-lowered `Vec<String>` copies built in both callers; `str::eq_ignore_ascii_case` is the stdlib API. Same pattern once at planner.rs:368.

## Checks

### (a) Cited code says what the finding claims — yes

- discovery.rs:189: `exts_lower.iter().any(|x| x == &e.to_ascii_lowercase())`. The `e.to_ascii_lowercase()` sits inside the `any` closure, so it allocates a fresh `String` per list element per file, exactly as claimed.
- Both cited pre-lowering collects exist verbatim: `scan_primaries` (discovery.rs:61-65) and `resolve_locator` (discovery.rs:147-151), each `.iter().map(|e| e.to_ascii_lowercase()).collect()` over a `Vec<String>` field (`Input.extensions` model.rs:79, `Locator.extensions` model.rs:259).
- Only these two call sites exist (grep over crates/, non-test).
- planner.rs:368-369 (`validate_extension_list`): `let normalized = ext.to_ascii_lowercase(); if !known.contains(&normalized)` — same pattern, single line, as the finding states.

### (b) Replacement is current idiom for Rust 1.96.1 / edition 2024 — yes, verified against the pinned toolchain, not memory

Empirical probe compiled and run under the exact pinned toolchain (`rust-toolchain.toml` pins channel 1.96.1; `rustc 1.96.1 (31fca3adb 2026-06-26)`):

1. **Semantic equivalence**: old (`pre-lowered list == lowered candidate`) and new (`exts.iter().any(|x| x.eq_ignore_ascii_case(e))` over the raw list) produce identical results across mixed-case ASCII, non-ASCII (`aç`/`AÇ`), and empty candidates. `eq_ignore_ascii_case` is defined as `to_ascii_lowercase(a) == to_ascii_lowercase(b)` without the allocations, so the equivalence is by construction; the probe confirms it.
2. **The toolchain's own clippy prescribes exactly this**: clippy 0.1.96 ships `manual_ignore_case_cmp` inside `clippy::all`, and its machine suggestion on the direct shape is literally `consider using .eq_ignore_ascii_case() instead` (help link: rust-clippy/rust-1.96.0/#manual_ignore_case_cmp). The lint does not fire on Muxsmith's current code only because the shape is indirect (comparison against a pre-collected Vec element), which is why it survives the `-D warnings` gate — that is a lint-detection gap, not an endorsement.
3. **The codebase already uses the proposed idiom** in three places: matcher.rs:164, matcher.rs:166, planner.rs:949. The finding brings discovery.rs in line with the repo's own established style.

### (c) Load-bearing difference between duplication sites — none

- The two discovery.rs collects are byte-identical in structure and feed the same helper. No difference.
- planner.rs:368 side note: `known` comes from `parse_list_types` (capability/runtime.rs:333), which lowercases every token at the source (runtime.rs:343, doc comment says "collected, lowercased, deduped"). With a guaranteed-lowercase `known`, `known.contains(&ext.to_ascii_lowercase())` and `known.iter().any(|k| k.eq_ignore_ascii_case(ext))` are equivalent; `Vec::contains` is linear anyway, so no perf regression from `any`. No load-bearing difference.

### (d) tag=yagni check — n/a (tag is `stdlib`; concrete construct and replacement are named regardless)

## Decision guard

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `extension_matches`, `eq_ignore_ascii`, `to_ascii_lower`, `case-insensitiv`, `discovery.rs`, `pre-lower`, `allocat`:

- **v1 design spec** (2026-07-08-muxsmith-v1-design.md:122): "extensions: list, matched case-insensitively" — a behavior mandate the replacement preserves exactly. No conflict.
- **Design memo Plan 3.5** (2026-07-09-plan-3.5-design-decisions.md:40): references the matcher.rs case-insensitive compare, which already uses `eq_ignore_ascii_case` — supports, not contradicts.
- **ROADMAP sweep group K** (cosmetic cleanup) enumerates its items explicitly (dead `at` param, mislabel trap, TracksCfg placement, stale module doc, Plan-1 remnants, eager resolve on discarded-plan path); this construct is not among them. Groups D / BCP-47 entries unrelated. Not tracked.
- **IDEAS.md**: no hits.

Neither DECISION_CONFLICT nor TRACKED.

## Notes on the proposed change

- Signature: `extension_matches(path: &Path, exts: &[String])` keeps working with `&input.extensions` / `&locator.extensions` passed directly (`Vec<String>` derefs). Both 5-line collects delete cleanly; `lines_cut: 10` checks out.
- Net effect: zero allocations on the extension-match path (previously one `String` per list element per walked file, plus the two per-call collects), behavior identical.
