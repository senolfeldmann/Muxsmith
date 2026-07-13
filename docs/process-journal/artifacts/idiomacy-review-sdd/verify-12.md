# Verify-12: diag_signature format!-joined composite key (slice F1a)

**Finding:** `crates/muxsmith-core/src/planner.rs:1965` — `diag_signature` builds its `BTreeMap<String, usize>` key via `format!("{}|{}|{}", d.code.key(), d.config_path, file)`; the Rust-normal composite key is a tuple `(String, String, String)`.

**Verdict: CONFIRMED**

## (a) Code matches the claim

Verified at HEAD, planner.rs:1952-1969. The function signature is `fn diag_signature(batch: &Batch) -> std::collections::BTreeMap<String, usize>` and the entry key at line 1965 is exactly the claimed `format!("{}|{}|{}", ...)` join.

## (b) Replacement is current idiom

Checked against current std docs (doc.rust-lang.org/stable via context7, valid for Rust 1.96.1 / edition 2024):

- `impl Ord for (T1, ..., Tn)` exists for tuples up to twelve items, comparing lexicographically.
- `BTreeMap` requires only `Ord` on the key.

So `BTreeMap<(String, String, String), usize>` keyed on `(d.code.key().to_string(), d.config_path.clone(), file)` is directly supported and is the standard composite-key form: no separator convention, no formatting pass, and it removes a real (if contrived) collision hazard — `file` is a user-controlled path that may legally contain `|` on Linux, so the string join is not even injective.

## (c) No load-bearing string requirement

All consumers checked: `base_sig` (planner.rs:1321, 1579), `no_regression` (1846-1850), `resolves_without_regression` (1823-1836), `partition_for_rule` pass-through (1491, 1515), and the unit test at 2065. Keys are only used for `get` + count comparison; they are never displayed, logged, or serialized. The test-side `diag_sig` in `tests/prop_planner.rs:343` is a separate mirror with a load-bearing difference (basename-scoped, `BTreeSet`), not the same site; it can be migrated in the same pass but does not block the change.

## Decision guard

- `docs/superpowers/specs/*.md`: only signature-related decision is the D6 partition-key wording supersession (2026-07-11 plan-5.5 memo residues), which concerns suggestion-keyed *grouping of affected files*, not the key encoding inside `diag_signature`. No conflict.
- `docs/ROADMAP.md`: sweep group K (cosmetic cleanup) lists six items; none is this. No other entry tracks it.
- `docs/IDEAS.md`: no mention.

Not tracked, not decided against. Pure mechanical idiom fix, zero behavior change (both key forms produce the same multiset semantics; BTreeMap ordering differs but no consumer depends on iteration order across the two encodings).
