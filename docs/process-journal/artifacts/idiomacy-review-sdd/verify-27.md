# verify-27: arb_track manual BTreeMap insert loop (prop_matcher.rs:179)

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-core/tests/prop_matcher.rs:179` — `arb_track` builds the `properties` BTreeMap with a manual `for`-loop insert over the generated `Vec<(String, PropValue)>`; replacement `properties: entries.into_iter().collect()` via `FromIterator for BTreeMap`. Tag: stdlib, slice F2b.

## Gate (a): does the code say what the finding claims?

Yes. At HEAD (2f17880), lines 179-182:

```rust
let mut properties = BTreeMap::new();
for (k, v) in entries {
    properties.insert(k, v);
}
```

followed by `properties,` in the `Track` struct literal (line 187). Exactly the claimed construct at the claimed line.

## Gate (b): is the replacement current idiom?

Yes. Checked against current std docs (doc.rust-lang.org `std::collections::BTreeMap`, `std::iter::FromIterator`): `BTreeMap<K, V>` implements `FromIterator<(K, V)>` for `K: Ord`; collecting an iterator of pairs is the documented, idiomatic construction. `String: Ord` holds; `PropValue` is unconstrained as the value type. In the struct-literal position `properties: entries.into_iter().collect(),` the target type is inferred from the `Track::properties` field, so no turbofish is needed. Semantics are identical to the loop (sequential insert; a later duplicate key overwrites the earlier value in both forms), and `entries` is not used after the loop, so the move is safe. Pure stdlib change; proptest =1.11.0 is untouched (the edit is inside a `prop_map` closure body). Lines cut: 4, matching the finding.

## Gate (c): duplication with load-bearing difference?

Not applicable — no duplication claim.

## Gate (d): yagni without concrete construct/replacement?

Not applicable — tag is stdlib, and both construct and replacement are concrete anyway.

## Decision guard

Grepped `docs/superpowers/specs/*.md` (8 memo files, D1-D35), `docs/IDEAS.md`, `docs/ROADMAP.md` for `arb_track`, `prop_matcher`, `BTreeMap`, `FromIterator`, `collect()`: zero hits. Read the ROADMAP cosmetic-cleanup group K and the test-hygiene collection (B-minors) in full: group K enumerates load.rs, model.rs, command_integration.rs, planner.rs and Plan-1 archive remnants; the B-minors cover yaml_fragment, D6 memo, Plan-3 ledger nits, B6-B11. Neither lists prop_matcher.rs or this construct. No recorded decision conflicts with the finding and it is not already tracked.
