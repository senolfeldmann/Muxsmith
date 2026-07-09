### Task 5: resolve settable `changes` per assignment (+ plan-time language validation)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `rule.changes: Option<BTreeMap<String, Scalar>>`, `capability::settable`, `LanguageIndex::normalize`, `DiagCode::InvalidPropertyValue`.
- Produces: populated `Assignment.changes` (only when the rule resolved to a track, i.e. `track_id = Some`); a plan-time `InvalidPropertyValue` diagnostic for a `changes.language` value not in `--list-languages`.

- [ ] **Step 1: Write the failing tests.** A rule with `changes: { language: tr, track_name: X }` on a matched track yields two `AppliedChange`s in property-name order; an invalid `changes: { language: zzz }` yields an `InvalidPropertyValue` diagnostic at config_path `tracks[N].changes.language`. (Use the existing test harness that builds a profile + fake `Identify` + `LanguageIndex::from_rows`.)

```rust
assert_eq!(plan.assignments[i].changes, vec![
    AppliedChange { property: "language".into(), value: Scalar::Str("tr".into()) },
    AppliedChange { property: "track_name".into(), value: Scalar::Str("X".into()) },
]);
// invalid-language case:
assert!(batch.files[0].diagnostics.iter().any(|d|
    d.code == DiagCode::InvalidPropertyValue
    && d.config_path == "tracks[0].changes.language"));
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution changes` -> FAIL.
- [ ] **Step 3: Implement.** In `resolve_file`, in the `1 => { ... }` matched branch (where `track_id = Some(tid)`), build `changes` from `rule.changes`: iterate the `BTreeMap` (already sorted by key), push `AppliedChange { property, value }`. For `property == "language"`, validate `lang.normalize(value_as_str)`; on `None` push `Diagnostic::error(DiagCode::InvalidPropertyValue, format!("{base}.changes.language")).for_file(&primary.path).with("property", "language").with("value", v)`. Non-string language value: also `InvalidPropertyValue` (it must be a string). `sub_charset` and other settables are carried without plan-time validation (validate.rs already checked types/known-ness). A rule that did not resolve to a track (missing/ambiguous) carries `changes: vec![]` (nothing to apply).

  Note: emitting an error here means the file gets no plan (existing rule: any error-severity diagnostic -> `plan: None`), consistent with a config error surfacing at plan time.

  Deliberate scope choice: settable `language` is validated per-file at the point of application (the matched branch), not batch-wide like the existing `validate_language_values` walk over `exact` match conditions. An invalid settable language on an optional rule that matched nothing therefore goes uncaught, but it is inert (nothing is set). If the whole-branch review prefers batch-level consistency, fold it into `validate_language_values`; not required for v1 correctness.

- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve settable changes and validate plan-time language values`.

---

