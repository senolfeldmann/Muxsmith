### Task 1: Fold the four owner rulings into the design document

**Files:**
- Modify: `docs/superpowers/specs/2026-07-15-plan-6-design.md`

**Interfaces:**
- Consumes: nothing.
- Produces: the amended design document every later task reads as ground truth.

This is a **documentation-only** task. It changes no code. Its purpose is that no implementer reads a superseded signature: the design as approved fixes the save writer at `Result<_, Diagnostic>`, describes a bespoke `ProfileDocument` struct, and records spec 8.3's tooltip baseline as applying to the editor - the three forks the owner ruled on, plus the save-note count. Do not re-argue the rulings; record them.

- [ ] **Step 1: Amend D41's writer signature and error currency**

At `:130-138` the document reads (in part) "`to_string(&Profile, Format) -> Result<String, Diagnostic>` and `to_file(&Profile, &Path) -> Result<(), Diagnostic>`". Replace the error type in both signatures with `SaveError` and add, immediately after that paragraph, a subsection recording the ruling:

```markdown
**Error currency: `SaveError`, mapped to `IpcError` at the shell** (owner
ruling 2026-07-16, superseding this ADR's original `Result<_, Diagnostic>`).

```rust
/// A failure of the profile writer. Not a `Diagnostic`: a `Diagnostic`
/// describes a problem with the profile or the plan, and a write failure
/// leaves a valid model and a full disk (`core-124-error-currency-split`).
pub enum SaveError {
    /// The file could not be written (permissions, full disk, bad path).
    Io(String),
    /// The model could not be serialized to the target format.
    Serialize(String),
}
```

The shell maps it in `src-tauri/src/error.rs`, mirroring `SettingsError`:
`SaveError::Io` -> `profile-save-io-failed`, `SaveError::Serialize` ->
`profile-save-failed`, both carrying a `detail` param (the spec 8.4
third-party-message exception). No new `DiagCode`; `diagnostics.ftl` is
untouched.

**Why not a `Diagnostic`.** The original signature was chosen for symmetry
with `profile::load`, which does return `Result<Profile, Diagnostic>`. The
symmetry does not carry: the loader's `Diagnostic` is right because a parse
failure IS a profile problem - the file's content is wrong - whereas a write
failure is not. `src-tauri/src/error.rs:8-15` already drew that line ("an
`IpcError` describes an IPC-protocol-level failure ... an unreadable path")
and this ADR contradicted it unnoticed through four review rounds, because
the boundary was written only in rustdoc and nowhere a reviewer checks. It
is now Tier-2 `core-124-error-currency-split`. Reusing `ParseError` was
rejected outright: its catalog prose is `parse-error = The profile could not
be parsed: { $detail }`, which is a false statement for a full disk. Adding
a new `DiagCode` was rejected because `catalog_completeness.rs` matches
`DiagCode` exhaustively, so it would force new user-facing bilingual prose
for a condition that is not a profile diagnostic at all.
```

- [ ] **Step 2: Correct the save-surface note's key count**

At `:278-279` the text reads "Two new Fluent keys, en+de (D47's catalog table)". Two defects: the count contradicts section 2, and the cross-reference is dangling - D47 is the schema ADR and has no catalog table. Replace with:

```markdown
One new Fluent key, en+de (section 2's catalog table). Owner ruling
2026-07-16: the note is a single message; `gui-editor.ftl` carries 43 keys,
as section 2 already states.
```

- [ ] **Step 3: Make section 2's `gui-common.ftl` row concrete**

Section 2's table (`:1737`) currently reads `| D41 | save-failure IpcError codes | gui-common.ftl | codes |`. Replace the row's code column with the two now-named codes:

```markdown
| D41 | save-failure `IpcError` codes (`profile-save-io-failed`, `profile-save-failed`) | `gui-common.ftl` | 2 |
```

- [ ] **Step 4: Point section 2's `ApplyError` row at D49's three codes**

Section 2's D43 row (`:1739`) still names the single `suggestion-rule-not-found` code that D49 superseded with three, so the approved design's own catalog table now contradicts D49 - the dangling-reference class step 2 also fixes. Replace the row:

```markdown
| D43 | `ApplyError` codes (`apply-unparsable-config-path`, `apply-rule-index-out-of-range`, `apply-edit-changed-nothing`) | `gui-common.ftl` | 3 |
```

D49 (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`, section "The catalog entries") is the authority on these three; this row records them, it does not re-specify them.

- [ ] **Step 5: Amend D42's `ProfileDocument` bullet to the owner's resolution**

The design describes `ProfileDocument` in three places, all now superseded by owner decision 2026-07-16 (`core-85-report-json-dry`). Amend all three so no implementer reads the bespoke-struct shape:

At `:341-349`, replace the `ProfileDocument` paragraph (from "`ProfileDocument` is `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }`," through "a second call would let them disagree.") with:

```markdown
`load_profile` returns **no bespoke struct**. It returns the existing
`report::json::config_only_document(&diags, None, &ShellRenderer)` envelope
(the same document machinery `validate_profile` uses) with one added key,
`"profile"`: the parsed model, or `null` on a `ParseError`. Its diagnostics
therefore live under `config_diagnostics`, carry the injected `rendered`
field, and are **byte-identical in shape** to what `validate_profile` already
returns (`core-85-report-json-dry`: neither surface owns document logic, and
no second document shape is invented). On a `ParseError` the `"profile"` value
is `null` and the single diagnostic explains why, mirroring
`config_diagnostics_from_file`'s own short-circuit
(`profile/validate.rs:203-208`). One round trip, because the editor needs
both and a second call would let them disagree. (Owner decision 2026-07-16,
superseding the original bespoke `ProfileDocument` struct.)
```

At `:311`, replace the `load_profile` command-table row's return type and note:

```markdown
| `load_profile` | `async fn load_profile(path: String) -> Result<serde_json::Value, IpcError>` | New. Returns the `config_only_document` envelope plus a `"profile"` key (the model, or `null` on `ParseError`); no bespoke struct (owner 2026-07-16, `core-85`). |
```

At `:404`, replace "new `ProfileDocument` wire shape;" in the interface-changes sentence with "the `load_profile` document shape (the `config_only_document` envelope plus a `"profile"` key);".

- [ ] **Step 6: Correct section 6's editor tooltip statement**

At `:1878-1880`, section 6 currently reads "The spec 8.3 **tooltip/inline-explanation baseline still applies to the editor's views** (D22's 'NOT deferred' clause); only the sidebar machinery waits." Owner ruling 2026-07-16: the editor's 8.3 tooltip baseline defers to Plan 7 with the sidebar. Replace those lines with:

```markdown
The editor's own spec 8.3 tooltip/inline-explanation baseline **defers to
Plan 7** with the sidebar (owner ruling 2026-07-16): the editor ships in
Plan 6 WITHOUT tooltips, and its 42 controls get their tooltip keys in the
Plan 7 pass, in the same pass as their help-ids, rather than as a retrofit -
the re-cut's own stated reason for sequencing Plan 7 after Plan 6. So
`gui-editor.ftl` carries 43 keys in Plan 6 (42 labels + 1 save-surface note)
and grows by the tooltip set in Plan 7 (`docs/ROADMAP.md:74-84`).
```

- [ ] **Step 7: Record all four rulings in section 8**

Section 8 ("What the implementer must not decide") is the list a dispatched implementer reads to know what is pre-decided. Append four bullets:

```markdown
- The writer returns `SaveError`, **not** a `Diagnostic`, and the shell maps
  it to `profile-save-io-failed` / `profile-save-failed` in `gui-common.ftl`.
  No new `DiagCode` and no `diagnostics.ftl` change (owner ruling 2026-07-16,
  `core-124-error-currency-split`).
- The save-surface note is **one** Fluent message, so `gui-editor.ftl` carries
  43 keys: 42 registry labels + 1 note (owner ruling 2026-07-16).
- `load_profile` returns the `config_only_document` envelope plus a `"profile"`
  key, **not** a bespoke `ProfileDocument` struct; its diagnostics are
  byte-identical in shape to `validate_profile`'s (owner decision 2026-07-16,
  `core-85-report-json-dry`).
- The editor ships in Plan 6 **without tooltips**; spec 8.3's editor tooltip
  baseline defers to Plan 7. `gui-editor.ftl` gets no tooltip budget here
  (owner ruling 2026-07-16, `docs/ROADMAP.md:74-84`).
```

- [ ] **Step 8: Verify no superseded text survives**

Run each and confirm the stated expectation. Each grep is scoped so it produces output for the defect it guards and none once fixed - the two split greps below replace one that spanned a line break and could never match its second alternative.

```bash
D=docs/superpowers/specs/2026-07-15-plan-6-design.md

grep -n "Result<String, Diagnostic>\|Result<(), Diagnostic>" "$D"
# Expected: no output. Both signatures now name SaveError (step 1).

grep -n "Two new Fluent keys" "$D"
# Expected: no output. The count was corrected in step 2.

grep -n "D47's" "$D"
# Expected: no output. The dangling cross-reference was the only "D47's" in the
# file (confirmed: it occurs once, at :278) and step 2 removed it.

grep -n "suggestion-rule-not-found" "$D"
# Expected: no output. Section 2's D43 row named it (once, at :1739) and step 4
# repointed it to D49's three codes; it survives nowhere else in the design.

grep -cn "SaveError" "$D"
# Expected: 6 or more (D41's enum, its two variants in prose, section 8, the
# mapping note). It was 0 before this task.

grep -n "carries \*\*43\*\*" "$D"
# Expected: one hit at :1749 - unchanged, and now consistent with D41.

grep -n "profile: Option<Profile>, diagnostics: Vec<Diagnostic>" "$D"
# Expected: no output. The bespoke ProfileDocument struct is gone (step 5).

grep -c "config_only_document" "$D"
# Expected: 1 or more. Step 5 introduced the envelope reference; it was 0 before.

grep -n "still applies to the editor" "$D"
# Expected: no output. The tooltip statement was corrected in step 6.
```

- [ ] **Step 9: Commit**

```bash
git add docs/superpowers/specs/2026-07-15-plan-6-design.md
git -c commit.gpgsign=false commit -m "plan 6 design: fold the four owner rulings (SaveError currency, one-key save note, load_profile envelope, editor tooltips to Plan 7) and repoint section 2's ApplyError row to D49"
```

---

