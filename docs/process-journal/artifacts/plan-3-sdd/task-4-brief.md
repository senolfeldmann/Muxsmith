### Task 4: enrich `Plan` and `Assignment` (types + construction defaults)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (types + the single `Plan`/`Assignment` construction site + `resolve_file` signature stays)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs` (update expectations)

**Interfaces:**
- Consumes: `Scalar`, `PathBuf`.
- Produces: the new public types from the "enriched `Plan`" reference block above (`AppliedChange`, `ChapterSource`, `TagFlags`, `TitleAction`, `PrimaryAttachments`, `AttachmentPlan`), the widened `Assignment` (`track_kind`, `changes`) and `Plan` (`attachments`, `chapters`, `tags`, `title`). This task wires **defaults only** (KeepAll / Keep / all-keep / Keep / empty changes / track_kind from the matched track), so behavior is unchanged; Tasks 5-8 fill real resolution.

- [ ] **Step 1: Write the failing test.** Extend an existing `planner_resolution` case to assert the defaults on a plan:

```rust
// after obtaining a `plan` for a simple single-video profile:
assert_eq!(plan.attachments, muxsmith_core::planner::AttachmentPlan {
    primary: muxsmith_core::planner::PrimaryAttachments::KeepAll,
    add_files: vec![],
});
assert_eq!(plan.chapters, muxsmith_core::planner::ChapterSource::Keep);
assert_eq!(plan.tags, muxsmith_core::planner::TagFlags { global_keep: true, track_keep: true });
assert_eq!(plan.title, muxsmith_core::planner::TitleAction::Keep);
assert_eq!(plan.assignments[0].track_kind.as_deref(), Some("video"));
assert!(plan.assignments[0].changes.is_empty());
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution` -> FAIL (fields/types missing).
- [ ] **Step 3: Implement.** Add the new types (each field documented). Add `track_kind`/`changes` to `Assignment`; add `attachments`/`chapters`/`tags`/`title` to `Plan`. In `resolve_file`, populate:
  - `track_kind`: when an assignment resolves to a track, set `Some(track.kind.clone())`; for the matched-track path, the kind comes from the matched `Track`. Because the current code maps `matched: Vec<u64>` (ids only), also capture the kind: change the match collection to keep the matched track's `kind`, e.g. collect `Vec<(u64, String)>` of `(t.id, t.kind.clone())` and use `.0`/`.1`. For unmatched/optional (`track_id: None`), `track_kind: None`.
  - `changes`: empty `vec![]` for now (Task 5 fills).
  - The `Plan { ... }` literal at the construction site gets `attachments: AttachmentPlan { primary: PrimaryAttachments::KeepAll, add_files: vec![] }, chapters: ChapterSource::Keep, tags: TagFlags { global_keep: true, track_keep: true }, title: TitleAction::Keep`.
  - Every other `Assignment { ... }` literal in `resolve_file` (the several early-`continue` construction sites for missing/ambiguous cases) gets `track_kind: None, changes: vec![]`.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS (suggestion-engine code that clones profiles/replans is unaffected; it reads diagnostics, not the new Plan fields).
- [ ] **Step 5: Gate + commit.** `feat(planner): enrich Plan and Assignment with resolution fields (defaults)`.

---


---
## Appended: enriched Plan type reference (from the plan's shared reference block; these are the exact definitions Task 4 creates)

## Reference: the enriched `Plan` (types introduced in Task 4, consumed throughout)

All new public types live in `crates/muxsmith-core/src/planner.rs` unless noted. These signatures are the contract every later task builds against.

```rust
/// A resolved settable change on an assignment (spec 4.4). Format-neutral: the
/// property name and value, not an mkvmerge flag; `command` maps the property to
/// its option via `capability::settable`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AppliedChange {
    /// Settable property name (spec 4.4 table key), e.g. `language`, `track_name`.
    pub property: String,
    /// The value to set.
    pub value: Scalar,
}

/// What happens to the output's chapters (spec 4.9).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ChapterSource {
    /// mkvmerge default (no `--no-chapters`).
    Keep,
    /// `--no-chapters` on every input group.
    Drop,
    /// `--chapters <path>` globally, `--no-chapters` on every input group.
    External(PathBuf),
}

/// Output tag handling (spec 4.9).
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct TagFlags {
    /// Keep global (container) tags; `false` -> `--no-global-tags`.
    pub global_keep: bool,
    /// Keep per-track tags; `false` -> `--no-track-tags`.
    pub track_keep: bool,
}

/// Output title handling (spec 4.9).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TitleAction {
    /// mkvmerge default (no `--title`).
    Keep,
    /// `--title ""` (force empty).
    Clear,
    /// `--title <s>` (rendered template).
    Set(String),
}

/// How the primary file's existing attachments are treated (spec 4.9). Donor
/// files always get `--no-attachments` (D10), so this concerns the primary only.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PrimaryAttachments {
    /// Keep all (no attachment filter on the primary group).
    KeepAll,
    /// Keep exactly these attachment ids (`--attachments id,id`); non-empty.
    Subset(Vec<u64>),
    /// Keep none (`--no-attachments`).
    DropAll,
}

/// Resolved attachment disposition for one plan (spec 4.9, D10).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AttachmentPlan {
    /// Disposition of the primary's own attachments.
    pub primary: PrimaryAttachments,
    /// External files to attach via `--attach-file`, from `add` locators, in
    /// resolution order.
    pub add_files: Vec<PathBuf>,
}
```

`Assignment` gains two fields; `Plan` gains four:

```rust
pub struct Assignment {
    pub rule_index: usize,
    pub source: PathBuf,
    pub track_id: Option<u64>,
    /// The matched track's `-J` type (`video`/`audio`/`subtitles`/`buttons`),
    /// needed by `command` to pick `--audio-tracks` vs `--video-tracks` etc.
    /// `None` exactly when `track_id` is `None`.
    pub track_kind: Option<String>,
    /// Settable changes to apply to the resolved track; empty when the rule has
    /// no `changes` or matched nothing.
    pub changes: Vec<AppliedChange>,
}

pub struct Plan {
    pub source: PathBuf,
    pub output: PathBuf,
    pub assignments: Vec<Assignment>,
    /// Resolved attachment disposition (spec 4.9).
    pub attachments: AttachmentPlan,
    /// Resolved chapters disposition (spec 4.9).
    pub chapters: ChapterSource,
    /// Resolved tag flags (spec 4.9).
    pub tags: TagFlags,
    /// Resolved output title (spec 4.9).
    pub title: TitleAction,
}
```
