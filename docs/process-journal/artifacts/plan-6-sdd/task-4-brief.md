### Task 4: D48 - a canonical save omits default-valued fields

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs` (17 fields x 2 attributes, 4 predicates)
- Create: `crates/muxsmith-core/tests/fixtures/all-non-default.yaml`
- Test: `crates/muxsmith-core/tests/profile_save.rs` (extend Task 2's file)

The spec 8.2 amendment is **not** in this task - it lives in Task 7 (stream C, the single v1-spec owner), so stream A touches no shared file (F3).

**Interfaces:**
- Consumes: Task 2's `save::to_string`.
- Produces: `Profile`'s serialized form omits default-valued fields. No API change.

**Read first:** design D48 (`:1336-1722`) in full, especially:
- the **17-row table** at `:1517-1535` - location, field, serde default, predicate. This is the authoritative enumeration; work from it row by row.
- the mechanism at `:1442-1508`: **every predicate calls the very function the field's own `default` attribute names**.
- the schemars interaction at `:1557-1637`: each of the 17 also carries `#[schemars(extend("default" = <derived>))]`, derived from that same function.

Binding points, each one a place where getting it wrong loses user data silently:
- **A generic `is_default` is correct for 13 of the 17 and silently destroys data on the other 4.** Two of those four fail to compile (`FilenameCfg` and `SourceCfg` have no `Default` impl, so `is_default<T: Default>` cannot instantiate - `E0277`). **Two compile and are the hazard**: `TracksCfg.unmatched` (default `drop_policy()`, so a naive predicate omits `unmatched: keep` and it reloads as `drop` - destroying the owner-ruled-legal `core-83` passthrough profile) and `Input.recursive` (default `default_true()`, so a naive predicate omits `recursive: false` and it reloads inverted).
- Omission is implemented with `skip_serializing_if` **on the derives**, not by post-processing the tree in `save::to_string` (D48's rejected alternative: a walker cannot tell `tracks.unmatched` from `attachments.unmatched`, whose defaults are opposite).
- The three struct-valued fields derive to `"default": {}` and **that is accepted, not patched with a literal**.
- **Both guards ship with the serializer, not after it.** Guard 2 is not optional and is **not** to be argued out at the keyboard on the grounds that the derivation makes it vacuous - that argument is already recorded and answered in D48 `:1657-1701`, and `proc-proposed-safeguard-stays` holds the guard in until it exists and can be measured. If you believe it cannot fail, that belief is the trigger for design trigger 2, not for deleting the test.

- [ ] **Step 1: Write guard 1 - round-trip fidelity on an all-non-default fixture**

Create `crates/muxsmith-core/tests/fixtures/all-non-default.yaml`: a profile setting **every one of the 17 fields** to a value that is **not** its default, per the table at `:1517-1535`. This is what catches a predicate that skips a non-default value, and it catches it for all 17 at once. Then extend `crates/muxsmith-core/tests/profile_save.rs`:

```rust
const ALL_NON_DEFAULT: &str = include_str!("fixtures/all-non-default.yaml");

/// D48 guard 1: every one of the 17 defaulted fields set to a NON-default
/// value must survive a save/load round trip. A predicate that skips a value
/// which is not the default silently destroys it - the core-83 passthrough
/// class of bug (`unmatched: keep` reloading as `drop`).
#[test]
fn all_non_default_fields_survive_the_round_trip() {
    let p = from_str(ALL_NON_DEFAULT, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    let p2 = from_str(&text, Format::Yaml).expect("re-parses");
    assert_eq!(p, p2, "a non-default value must never be omitted (D48 guard 1)");
}

/// The sharpest instance, called out because it is an owner-ruled-legal
/// profile (`core-83`) that a naive `is_default` turns into a NoTrackRules
/// error: zero rules plus `unmatched: keep` is a pure-passthrough remux.
#[test]
fn the_core83_passthrough_profile_survives_a_save() {
    let y = "profile_version: 1\ninput: { pattern: 'E(\\d+)', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\n";
    let p = from_str(y, Format::Yaml).expect("parses");
    let text = to_string(&p, Format::Yaml).expect("serializes");
    assert!(
        text.contains("unmatched: keep"),
        "tracks.unmatched defaults to DROP, so `keep` is not a default and must be written: {text}"
    );
    assert_eq!(from_str(&text, Format::Yaml).unwrap(), p);
}
```

- [ ] **Step 2: Baseline - observe guard 1 green on the unmodified serializer**

```bash
cargo test -p muxsmith-core --test profile_save all_non_default
```
Expected: **PASS.** This is *not* a TDD red step and cannot be one: before D48's `skip_serializing_if` attributes exist nothing is omitted, so a save/load round-trip must pass (the design records this at `:90-92`, "Canonical round-trip is exact ... it holds today"). Step 1 already created the fixture, so there is no "fixture missing" arm. The real red proof is **step 4**, where a deliberately-naive predicate makes guard 1 go red. Record the baseline PASS in the log.

- [ ] **Step 3: Add the predicates and the 17 field attributes**

Work the table at `:1517-1535` row by row. Four predicates beyond the generic one, per `:1447-1457`. Every row gets `#[serde(default...)]`, `skip_serializing_if`, and `#[schemars(extend("default" = ...))]`, and **all three name the same function**.

- [ ] **Step 4: Prove guard 1 catches the naive predicate (the real red proof)**

Before moving on, deliberately break one row and confirm the guard fires - this is the evidence that guard 1 works, and it costs thirty seconds:

```bash
# temporarily change TracksCfg.unmatched's skip_serializing_if to the generic "is_default"
cargo test -p muxsmith-core --test profile_save
# Expected: FAIL - the_core83_passthrough_profile_survives_a_save goes red.
# Then revert to is_drop_policy and confirm green again.
```
Record the observed failure text in your report. If it does **not** go red, guard 1 is not testing what D48 says it tests - stop and return NEEDS_CONTEXT.

- [ ] **Step 5: Write guard 2 - schema-default honesty**

A table test asserting, for each of the 17 fields, that the schema's `default` equals `serde_json::to_value` of that field's serde default. Follow the house's existing table-test shape - `capability/mod.rs`'s `settable_maps_to_mkvmerge_options` asserts a `const EXPECTED` table against the real thing, length first, then row by row - rather than inventing a pattern. The three struct-valued fields (`Profile.output`, `Profile.attachments`, `Profile.tags`) expect `{}`, per `:1612-1637`.

- [ ] **Step 6: Run everything**

```bash
cargo test -p muxsmith-core --test profile_save
cargo test --workspace
git diff --exit-code crates/muxsmith-cli/tests/snapshots/
```
Expected: all green; snapshots unmoved.

- [ ] **Step 7: Full gate, then commit**

```bash
git add crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/tests/profile_save.rs crates/muxsmith-core/tests/fixtures/all-non-default.yaml
git -c commit.gpgsign=false commit -m "core: a canonical save omits default-valued fields, with both guards (D48)"
```

---

