### Task 11: `command` - attachments, chapters, tags argv

**Files:**
- Modify: `crates/muxsmith-core/src/command.rs`
- Test: `crates/muxsmith-core/tests/command.rs`

**Interfaces:**
- Consumes: `AttachmentPlan`, `PrimaryAttachments`, `ChapterSource`, `TagFlags`.
- Produces: attachment/chapter/tag flags placed per the canonical-argv reference (per-input `--no-chapters`/`--no-*-tags`/attachment flags; global `--chapters`/`--attach-file`).

- [ ] **Step 1: Write the failing golden tests.** Cases:
  - `PrimaryAttachments::Subset(vec![0,2])` -> primary group gets `--attachments 0,2`; a donor group gets `--no-attachments`.
  - `PrimaryAttachments::DropAll` -> primary group `--no-attachments`.
  - `PrimaryAttachments::KeepAll` -> no attachment flag on primary.
  - `add_files: [p("/m/x.ttf")]` -> global `--attach-file /m/x.ttf`.
  - `ChapterSource::Drop` -> every input group gets `--no-chapters`; no global `--chapters`.
  - `ChapterSource::External(p("/m/e.xml"))` -> global `--chapters /m/e.xml` and every group `--no-chapters`.
  - `TagFlags { global_keep: false, track_keep: false }` -> every group `--no-global-tags --no-track-tags`.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test command attach` (and the chapter/tag cases) -> FAIL.
- [ ] **Step 3: Implement.** Global: `--chapters <path>` when `External`; `--attach-file <p>` per `add_files`. Per group (in the per-group emission, before track selection per the canonical order): `--no-chapters` when `Drop|External`; `--no-global-tags`/`--no-track-tags` per `tags`; attachments: primary group per `PrimaryAttachments`, donor groups `--no-attachments`. Determine "is this group the primary" by `group_path == plan.source`.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core --test command` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(command): attachments, chapters, and tags argv`.

---


---
## Appended reference 1: enriched Plan types

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

---
## Appended reference 2: canonical argv contract (Tasks 9-11 golden tests lock this)

## Reference: the canonical argv `command` emits (locked by Task 9-11 golden tests)

`command(&Plan) -> Vec<String>` returns argv **without** the leading `mkvmerge` program name. Deterministic ordering:

1. **Global section:** `--output`, `<output>`; then title (`Clear` -> `--title`, ``; `Set(s)` -> `--title`, `s`; `Keep` -> nothing); then `--chapters`, `<path>` if `ChapterSource::External`; then `--attach-file`, `<path>` for each `add_files` entry in order.
2. **Input groups**, primary first, then donor sources in first-appearance order across `assignments`. The primary (`plan.source`) is always group 0 even if it contributes no tracks. Each group, in order:
   a. `--no-chapters` if chapters is `Drop` or `External`.
   b. `--no-global-tags` if `!tags.global_keep`; `--no-track-tags` if `!tags.track_keep`.
   c. attachments: primary group per `PrimaryAttachments` (`KeepAll` -> nothing; `Subset(ids)` -> `--attachments`, `id,id`; `DropAll` -> `--no-attachments`); every donor group -> `--no-attachments`.
   d. track selection, categories in fixed order video, audio, subtitles, buttons: if this group has assigned track ids of that category -> `--<cat>-tracks`, `id,id` (ids ascending); else -> `--no-<cat>`. Category-to-flag: video=`--video-tracks`/`--no-video`, audio=`--audio-tracks`/`--no-audio`, subtitles=`--subtitle-tracks`/`--no-subtitles`, buttons=`--button-tracks`/`--no-buttons`.
   e. per-track property options: for each assigned track (track_id ascending), for each `AppliedChange` (property name ascending): `<option>`, `<tid>:<value>`, where `<option>` = `capability::settable(property).unwrap().1`; value = `1`/`0` for `Scalar::Bool`, the raw string for `Scalar::Str`, `to_string()` otherwise.
   f. `(`, `<path>`, `)`.
3. **`--track-order`**, `<g:tid,...>`: one entry per assignment with `track_id = Some`, in profile (assignment) order, `g` = the input group index of that assignment's source. Omitted entirely if no assignment has a track.

Every "confirm against v100" note is discharged by Task 12's real-mkvmerge round trip; the golden tests here lock this exact string output.

