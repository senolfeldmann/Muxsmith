# Task 1 report: Fold the four owner rulings into the design document

## Status: DONE

## What was changed, per step

**Step 1 (D41 writer signature and error currency).** In the D41 section, both
writer signatures (`to_string`, `to_file`) now name `SaveError` instead of
`Diagnostic`. Immediately after that paragraph, inserted the new subsection:
the `SaveError` enum (`Io`, `Serialize` variants with rustdoc), the shell
mapping note (`SaveError::Io` -> `profile-save-io-failed`,
`SaveError::Serialize` -> `profile-save-failed`), and the "Why not a
`Diagnostic`" rationale block citing `core-124-error-currency-split`. Landed
before the pre-existing "**Rationale.**" (YAML comments) paragraph, as
specified.

**Step 2 (save-surface note key count).** Replaced "Two new Fluent keys, en+de
(D47's catalog table)." with "One new Fluent key, en+de (section 2's catalog
table). Owner ruling 2026-07-16: the note is a single message; `gui-editor.ftl`
carries 43 keys, as section 2 already states." Removes the dangling D47
cross-reference (D47 is the schema ADR, has no catalog table).

**Step 3 (section 2 D41 gui-common.ftl row).** Replaced the row's code column
(previously the placeholder word "codes") with the two named codes
(`profile-save-io-failed`, `profile-save-failed`) and count `2`.

**Step 4 (section 2 D43 ApplyError row -> D49's three codes).** Replaced the
row that still named the single `suggestion-rule-not-found` code with the
three codes D49 introduced (`apply-unparsable-config-path`,
`apply-rule-index-out-of-range`, `apply-edit-changed-nothing`) and count `3`.
Verified `docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md` exists (D49's
source document, cited as authority, not re-specified here).

**Step 5 (D42 `ProfileDocument` -> owner resolution, three sites).**
- The `:341-349` paragraph (bespoke `{ profile: Option<Profile>, diagnostics:
  Vec<Diagnostic> }` struct) replaced with the `config_only_document` envelope
  description (owner decision 2026-07-16, `core-85-report-json-dry`).
- The `load_profile` command-table row's return type changed from
  `Result<ProfileDocument, IpcError>` to `Result<serde_json::Value, IpcError>`,
  with a note pointing at the envelope-plus-`"profile"`-key shape.
- The `:404` interface-changes sentence's "new `ProfileDocument` wire shape;"
  phrase replaced with "the `load_profile` document shape (the
  `config_only_document` envelope plus a `"profile"` key);".

Verified against the actual core source
(`crates/muxsmith-core/src/report/json.rs:78`) that
`config_only_document(config_diags: &[Diagnostic], mkvmerge_found:
Option<bool>, renderer: &dyn DiagnosticRenderer) -> serde_json::Value` is the
real signature, and that `ShellRenderer` is a real `DiagnosticRenderer` impl
used the same way elsewhere in `src-tauri/src/run.rs` (e.g.
`config_only_document(&[d], None, &ShellRenderer)`), so the inserted prose is
grounded, not invented.

**Step 6 (section 6 editor tooltip statement).** Replaced "The spec 8.3
**tooltip/inline-explanation baseline still applies to the editor's views**
(D22's 'NOT deferred' clause); only the sidebar machinery waits." with the
owner-ruled defer-to-Plan-7 text (42 controls, tooltip keys join help-ids in
Plan 7, `gui-editor.ftl` stays at 43 keys in Plan 6). Cross-checked against
`docs/ROADMAP.md:74-84` (Plan 7 section), which already independently records
the same 2026-07-16 owner ruling in near-identical language - no contradiction
between the design doc and the roadmap.

**Step 7 (section 8, four bullets appended).** Appended, after the existing
D48-guard bullet (the section's last item), four new bullets recording: (1)
`SaveError` not `Diagnostic` + the two IpcError codes + no new DiagCode; (2)
the one-key save-surface note / 43-key `gui-editor.ftl` total; (3)
`load_profile`'s envelope-plus-`"profile"`-key return, not a bespoke struct;
(4) editor ships without tooltips in Plan 6, defers to Plan 7.

All seven steps used the brief's replacement text verbatim.

## Step 8 grep outputs (verbatim, run post-edit and re-confirmed post-commit
against the committed file; both runs identical)

```
$ grep -n "Result<String, Diagnostic>\|Result<(), Diagnostic>" "$D"
(no output, exit 1)
# Expected: no output. PASS.

$ grep -n "Two new Fluent keys" "$D"
(no output, exit 1)
# Expected: no output. PASS.

$ grep -n "D47's" "$D"
(no output, exit 1)
# Expected: no output. PASS.

$ grep -n "suggestion-rule-not-found" "$D"
(no output, exit 1)
# Expected: no output. PASS.

$ grep -cn "SaveError" "$D"
6
# Expected: 6 or more. PASS (exactly 6: the enum name, its two variants in
# the mapping-note prose, the "Why not a Diagnostic" paragraph, and section 8).

$ grep -n "carries \*\*43\*\*" "$D"
1790:why `gui-editor.ftl` carries **43**: the 42 registry labels plus the save-surface
# Expected: one hit (brief said :1749; line shifted to 1790 because earlier
# steps inserted ~41 lines above it - text matches, brief flagged line numbers
# as approximate). PASS.

$ grep -n "profile: Option<Profile>, diagnostics: Vec<Diagnostic>" "$D"
(no output, exit 1)
# Expected: no output. PASS.

$ grep -c "config_only_document" "$D"
4
# Expected: 1 or more. PASS.

$ grep -n "still applies to the editor" "$D"
(no output, exit 1)
# Expected: no output. PASS.
```

All nine checks match their stated expectation. No fix loop was needed.

## Self-review findings

- Re-read `git diff HEAD~1` in full: 80 insertions, 21 deletions, single file.
  Each of the seven steps' edits is present exactly where intended and nowhere
  else.
- Re-read the amended D41 section (lines ~120-194) and the amended D42 section
  (lines ~375-414) in full for coherence: both read as continuous prose, no
  duplicated sentences, no orphaned fence markers, the pre-existing
  "**Rationale.**" (YAML comments) paragraph in D41 and the "**Rejected: one
  command...**" paragraph in D42 are untouched and still follow naturally
  after the new content.
- `ProfileDocument` now appears exactly twice in the whole document, both as
  explicit "this was superseded" references (the owner-decision parenthetical
  in the `load_profile` paragraph, and section 8's new bullet) - never as a
  current type description. Confirmed via `grep -n "ProfileDocument"`.
- Typography: grepped every added line (`git diff | grep '^+'`) for em-dash,
  en-dash, curly quotes, Unicode ellipsis, non-breaking space. None found; all
  new prose uses ASCII hyphens and straight quotes, matching the brief's
  source text.
- Tier-2 house files (`docs/conventions.yaml`, `docs/process-conventions.yaml`)
  checked for `core-124-error-currency-split` and `core-85-report-json-dry`:
  both ledger entries exist and their `statement`/`occurrences` text is
  consistent with what the design doc now says (no conflict, no new pattern
  introduced by this task - it only records rulings already in the ledger).
  `docs/product-boundaries.yaml` has no related entries (expected; this is a
  technical-code ledger concern, not a product-boundary one).
- `docs/ROADMAP.md:74-84` (Plan 7 section) independently states the same
  2026-07-16 owner ruling on editor tooltips deferring to Plan 7; the design
  doc's new section-6 and section-8 text is consistent with it, not
  contradictory.

## Issues or concerns

None. All nine verification greps passed on the first run with no fix loop.
The one line-number drift noted above (grep 6's expected `:1749` landing at
`:1790` post-edit) is exactly the kind of drift the brief itself anticipated
("the line numbers are from the plan's authoring and should be close but the
text is the anchor") and does not affect the check's validity - the text
matched and the count of hits (one) matched.

## Commit

`0922df9` - "plan 6 design: fold the four owner rulings (SaveError currency,
one-key save note, load_profile envelope, editor tooltips to Plan 7) and
repoint section 2's ApplyError row to D49"

Committed unsigned (`git -c commit.gpgsign=false commit`) on `master`, staged
explicitly (`git add docs/superpowers/specs/2026-07-15-plan-6-design.md`), not
pushed (per instructions, the controller pushes).
