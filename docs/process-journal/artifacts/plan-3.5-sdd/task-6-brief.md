### Task 6: `UnsupportedSource` diagnostic (D21)

A primary file mkvmerge identifies (exit 0) but reports as unrecognized/unsupported container currently falls through to per-rule `MissingTrack` noise. Add a pre-resolution gate emitting one clear `UnsupportedSource` error and skipping the file. Per D21 open mechanic #5, the trigger is `!container_recognized || !container_supported` only; a recognized+supported container with zero tracks stays a `MissingTrack` case (not `UnsupportedSource`), so the gate does NOT use `is_identifiable()` (which also tests `!tracks.is_empty()`).

**Files:**
- Modify: `crates/muxsmith-core/src/report.rs` (add `UnsupportedSource => "unsupported-source"` in the planning-time section of `diag_codes!`, ~report.rs:108-140)
- Modify: `crates/muxsmith-core/src/planner.rs` (insert the gate after the skew check ~line 358, before `let mut assignments = Vec::new();` at 360)
- Modify: `locales/en/diagnostics.ftl` (add the `unsupported-source` message)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `Identification { container_recognized: bool, container_supported: bool, .. }`; `FileReport { source, identifier, plan, diagnostics }`; `Diagnostic::error(DiagCode, config_path).for_file(path)`.
- Produces: `DiagCode::UnsupportedSource`.

- [ ] **Step 1: Write the failing test**

Add to `planner_resolution.rs` a test using an identify fake/fixture whose `Identification` has `container_recognized: false` (or `container_supported: false`) with a non-empty pattern-matched primary; assert the file's diagnostics contain exactly one `UnsupportedSource` (error) and NO `MissingTrack`, and that `plan` is `None`. Use the file's existing fake-identify helper (`FakeIdent`) and construction style.

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p muxsmith-core --test planner_resolution unsupported_source -- --nocapture`
Expected: FAIL (no such `DiagCode`; today the file yields `MissingTrack` per rule).

- [ ] **Step 3: Add the diagnostic code**

In `crates/muxsmith-core/src/report.rs`, in the planning-time section of the `diag_codes! { ... }` block (near `MissingTrack`/`UnidentifiableSource`, ~line 114-120):

```rust
    /// A primary source mkvmerge identified (exit 0) whose container it does
    /// not recognize or support, so it cannot be muxed. Distinct from
    /// `UnidentifiableSource` (mkvmerge exited non-zero). A recognized,
    /// supported container with zero tracks is NOT this code (stays a
    /// per-rule `missing-track`).
    UnsupportedSource => "unsupported-source",
```

- [ ] **Step 4: Add the Fluent message**

In `locales/en/diagnostics.ftl` (near `unidentifiable-source`):

```
unsupported-source = mkvmerge identified this file but its container is not a supported muxing source.
```

- [ ] **Step 5: Insert the planner gate**

In `planner.rs`, immediately after the skew-check block (after line 358, before `let mut assignments = Vec::new();`):

```rust
    if !ident.container_recognized || !ident.container_supported {
        diagnostics.push(
            Diagnostic::error(DiagCode::UnsupportedSource, "input").for_file(&primary.path),
        );
        return FileReport {
            source: primary.path.clone(),
            identifier: primary.identifier.whole.clone(),
            plan: None,
            diagnostics,
        };
    }
```

(Mirror the exact `FileReport` field set used by the `UnidentifiableSource` early return at planner.rs:344-349.)

- [ ] **Step 6: Run the test, the completeness guard, and the suite**

Run: `cargo test -p muxsmith-core --test planner_resolution unsupported_source`
Expected: PASS.
Run: `cargo test -p muxsmith-cli --test catalog_completeness`
Expected: PASS (the new code has its Fluent message).
Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 7: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
feat(planner): clean UnsupportedSource diagnostic (D21)

A primary mkvmerge identifies but cannot mux (unrecognized/unsupported
container) now yields one UnsupportedSource error and skips the file,
instead of confusing per-rule missing-track noise. Zero-track containers
stay missing-track.

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

## Self-review (controller, after all tasks)

- **Spec coverage:** D19 -> Tasks 4 (validation) + 5 (canonical matching + the `exact` principle in spec 4.3); D20 (restructure) -> Task 1, (keep) -> Tasks 2-3; D21 -> Task 6. All three memo decisions covered.
- **README/guide content flagged:** the "`exact` is typed value-equality" principle (spec 4.3, Task 5 step 6) is a core semantic to surface in the public README/guide at 1.0; note it in the Plan 3.5 journal entry (SI-2) so the 1.0 doc pass picks it up.
- **Not in scope (deferred, by decision):** registry `validate()` on the accept side (well-formed tags accepted, mkvmerge rejects the pathological one); donor-side `UnsupportedSource` gate (primary-only per D21); the shelved ideas in `docs/IDEAS.md`.
- **Whole-branch review + journal:** after Task 6, run the SDD whole-branch review on the most capable model (per SI-1), then append the Plan 3.5 process-journal entry per SI-2 and salvage any `.superpowers/sdd/` artifacts to `docs/process-journal/artifacts/plan-3.5-sdd/` (verify file count in the commit).
