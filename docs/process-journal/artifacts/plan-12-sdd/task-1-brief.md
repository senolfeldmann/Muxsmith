## Task 1: the normative documents - two spec amendments and D106-D110

Read first: this plan's Decision register in full; `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` sections 8.2, 8.3, 8.4 and 11; one existing decisions file (`docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md`) for the ADR house form; Tier-2 `proc-04-spec-wins`, `a-document-never-cites-a-line-number-inside-itself`. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (two regions only: the tail of section 8.2's numbered item 1, and section 8.2's app-settings paragraph)
- Create: `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`

Section 8.4 is deliberately NOT edited: its locale sentence is already true, and the control's surface belongs to 8.2, which is where it was missing.

**Interfaces:**
- Consumes: nothing.
- Produces: the amended spec every later task reads as ground truth, and the five decision records (D106-D110).

- [ ] **Step 1: section 8.2's editor item.** Replace exactly the sentence

```
detail editor per rule, panels for attachments/chapters/tags/title, open/save YAML, recent profiles.
```

  with exactly

```
detail editor per rule, panels for attachments/chapters/tags/title, create/open/save YAML, recent profiles.
```

  and append, after the item's existing final sentence "Inline validation markers from core diagnostics.", exactly:

```
New creates a blank profile in the editor and touches no file: the seed carries the format version, one candidate extension and one empty track rule, so it is incomplete-until-filled and announced by a validation warning exactly as Add's empty rule is, never by an error that would disable Save. A profile created this way has no path yet; Save opens a save dialog and the picked path becomes the profile's path from then on. Undo and redo cover every model mutation - field edits, rule add and remove including the unconfirmed delete, drag-reorder, and every list or map widget mutation - at one step per editing burst, where a burst ends at a focus change or a grid operation; saving marks a position in that history rather than clearing it, and the history is what the editor derives "has unsaved changes" from. The editor holds at most one profile. Replacing it - by creating another, or by opening one - warns first and only while unsaved changes exist, naming what would be overwritten; switching views never touches it; and closing the app with unsaved changes warns as well, in one prompt that also covers a running batch when both hold.
```

- [ ] **Step 2: section 8.2's app-settings paragraph.** Replace exactly

```
App settings (not profile data): mkvmerge path override, default parallelism. Stored in the platform config directory.
```

  with exactly

```
App settings (not profile data): mkvmerge path override, default parallelism, interface language. Stored in the platform config directory. The language control is three-state - follow the system language, English, German - where following the system IS the absence of a stored override, is preselected until the user chooses otherwise, and stays reachable afterwards, so saving without touching the control stores no override (8.4).
```

- [ ] **Step 3: the decisions file.** Create `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` with an H1 `# Plan 12 decisions` and one section per decision, in the house form measured from the plan-5.7 file: `## D106: <title>` then the bold slots **Decision**, **Rationale**, **Rejected alternatives** (each alternative with its steelman stated at its strongest, not as a strawman), and, where one exists, **Triggers created**. Content comes from this plan's Decision register: D106 the locale control, D107 blank profile creation, D108 undo/redo and the derived save state, D109 the discard guards, D110 the shell's localized dialogs and the parity check over them. Four fixed properties of the file:
  - **D108 is recorded as a REVERSAL**, naming the owner ruling it reverses (S22, 2026-07-22, undo/redo wholesale in 1.x), the old reasoning (at 1.0 the explicit-save model bounds the loss, and undo/redo rather than a confirmation dialog is the durable answer to accidental destruction), and the new reason (change tracking is being built anyway). It also records that D66's no-confirmation-for-Remove premise is CONSUMED, not reopened.
  - **D109 records the superseded controller reading** (an unconditional warning independent of save state) as superseded by the owner's save-state gate, not as an open option, and its rejected alternative **shipping the shell's dialogs in English with a recorded reason** - named rather than numbered, since an ordinal into that list stales the moment one is inserted - as OVERRULED by the owner rather than as a live tradeoff.
  - **D110 records the ruling in the general form the owner gave it** (German translations always ship in the same change, without exception), not as a decision about one dialog, and states the residual it does not close (a non-literal `ftl_message` argument; the CLI's identical unserved-locale gap, surfaced not fixed).
  - **Every rejected alternative in the register appears**, including the ones whose steelman is strong enough to be mistaken for the winning argument. A caricatured rejection is a defect here.
  - No line-number citations, in either direction: not into the spec, not into this plan, not inside the file itself. Name sections, symbols and decisions.

- [ ] **Step 4: the self-contradiction sweep, as an enumeration with a fired control** (`proc-04-spec-wins`'s corollary). Run, and paste, all three:
  - `grep -nEi 'locale|language' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
  - `grep -nEi 'undo|redo|unsaved|discard|confirm' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
  - `grep -nEi 'create|new profile|open/save' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

  Read every hit and classify it as consistent or contradicting; a contradiction is a finding to report, never a silent second edit. **Fired control for the sweep, because an empty or thin result must be distinguishable from a broken pattern:** the `locale|language` expression must return section 8.4's own locale bullets and section 8.2's amended paragraph, and the `create|new profile|open/save` one must return the amended editor item - **named by what they search for rather than by position**, since an ordinal into a bullet list stales the moment one is inserted. If any expression returns nothing at all, it is malformed and the step is not done.
  - Two hits are known in advance and are consistent, named so they are not reported as findings: section 8.3's help-mode Escape sentence (a different keyboard channel), and section 11's non-goals, which name neither creation nor undo.

- [ ] **Step 5: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green (this task changes only documents, so every part is behaviour-preserving by construction; a failure is a real finding -> NEEDS_CONTEXT). `git diff --stat` covers exactly the two files in the Files list.

- [ ] **Step 6: commit.**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-30-plan-12-decisions.md
git -c commit.gpgsign=false commit -m "spec+adr: the editor creates, undoes and guards its unsaved profile; app settings carry a three-state language" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the two fenced spec replacements; that section 8.4 is not edited; the five decision numbers (D106-D110, the next free numbers, measured); which alternatives are recorded as rejected and that each carries a steelman; that the sweep is an enumeration with a fired control rather than a reading; that no file outside the Files list is touched.

---

