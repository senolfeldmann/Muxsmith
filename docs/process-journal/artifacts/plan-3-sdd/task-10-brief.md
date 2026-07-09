### Task 10: `command` - multi-input grouping + per-track property options

**Files:**
- Modify: `crates/muxsmith-core/src/command.rs`
- Test: `crates/muxsmith-core/tests/command.rs`

**Interfaces:**
- Consumes: `capability::settable` (property -> mkvmerge option), `Scalar`.
- Produces: per-track property options and correct multi-group ordering + `--track-order` across groups.

- [ ] **Step 1: Write the failing golden test.** Primary `/m/e.mkv` (video 0, audio 1 with `changes: language=de, default_track=true`) plus donor `/m/e.tr.srt` (subtitles 0 with `changes: language=tr, track_name="Turkce"`). Assert the argv: primary group is index 0, donor group index 1; property options `--language 1:de`, `--default-track-flag 1:1` in the primary group (property order: default_track before language? property-name ascending: `default_track` < `language`, so `--default-track-flag 1:1` then `--language 1:de`); donor group `--language 0:tr`, `--track-name 0:Turkce`; `--track-order 0:0,0:1,1:0`.

```rust
// expected (excerpt), primary group after selection flags:
// "--default-track-flag", "1:1", "--language", "1:de",
// donor group:
// "--subtitle-tracks", "0", "--no-video", "--no-audio", "--no-buttons",
// "--language", "0:tr", "--track-name", "0:Turkce",
// ...
// "--track-order", "0:0,0:1,1:0",
```

Include a boolean-value case asserting `1`/`0` encoding and a string case.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test command per_track` -> FAIL.
- [ ] **Step 3: Implement.** For each group, after the selection flags, iterate that group's assignments with `track_id = Some` (ascending by track_id), and for each `AppliedChange` (the `changes` vec is already property-ascending from Task 5, but sort defensively) emit `capability::settable(&c.property).expect("validated settable").1` then `format!("{tid}:{}", value_str(&c.value))` where `value_str` = `"1"`/`"0"` for `Bool`, the string for `Str`, `to_string()` otherwise. Ensure the multi-group `--track-order` maps each assignment to its group index.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core --test command` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(command): per-track property options and multi-input grouping`.

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

