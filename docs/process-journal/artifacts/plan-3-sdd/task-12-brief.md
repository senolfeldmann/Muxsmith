### Task 12: integration - reference-example golden + real-mkvmerge acceptance

**Files:**
- Create: `crates/muxsmith-core/tests/command_integration.rs`
- Test fixtures: build tiny MKVs at test time via mkvmerge itself (as spec 10 prescribes), gated + self-skipping when mkvmerge is absent (mirror the existing `identify_live.rs` / `mkvmerge_runtime.rs` gating).

**Interfaces:**
- Consumes: `planner::plan_batch` (or `plan_core`), `command::command`, `capability::runtime::Mkvmerge`.
- Produces: proof that the argv `command` emits is accepted by mkvmerge v100 for a representative plan, and a full golden for the spec 4.1 reference example (pure, no binary needed).

- [ ] **Step 1: Write the tests.**
  - **Pure golden:** construct (or plan, via a fake `Identify` returning fixtures for the 4.1 reference example) a `Plan` and assert the full argv string. This locks the reference example end to end without a binary.
  - **Live acceptance (gated):** if `Mkvmerge::locate()` succeeds, generate a tiny source MKV (e.g. from a generated tone WAV or an existing fixture), run the planner to get a `Plan`, prepend `mkvmerge` to `command(&plan)`, spawn it, and assert exit code 0 and that the output file exists and re-identifies. Skip with an eprintln when mkvmerge is absent (CI has none yet, per HANDOFF follow-ups).
- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test command_integration` -> FAIL (pure golden mismatch until argv is right; live test skips if no binary).
- [ ] **Step 3: Implement / adjust.** Reconcile any gap between the emitted argv and what mkvmerge v100 actually accepts (e.g. exact `--*-tracks` spellings, boolean flag encoding, `--attach-file`/`--chapters` placement). If a discrepancy is found, fix `command.rs` AND update the Task 9-11 golden expectations to match, then re-run the whole `command` suite. Record any mkvmerge-behavior finding as a code comment citing the observed v100 behavior.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` (full) -> PASS (live test PASS locally where mkvmerge exists, SKIP in CI).
- [ ] **Step 5: Gate + commit.** `test(command): reference-example golden and live mkvmerge acceptance`.

---

## Post-plan: verification and handoff

- [ ] **Whole-branch review** on the most capable model (SI-1): diff `042a2c0..HEAD`, checked against this plan and the spec (sections 4.9, 6) and D7-D12. Focus: command argv correctness vs mkvmerge v100, attachment resolution edge cases (empty primary attachments, all-dropped, add-zero warning severity), title/language plan-time validation, `Matchable` generalization not regressing track semantics.
- [ ] **Full gate** on the final HEAD: `cargo test --workspace && cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check`.
- [ ] **Journal** per SI-2: append the Plan 3 entry to `docs/process-journal.md` following `docs/process-journal/PROMPT.md` verbatim; salvage any `.superpowers/sdd/` artifacts to `docs/process-journal/artifacts/plan-3-sdd/` and verify the file count in the commit.
- [ ] **HANDOFF** refresh: mark Plan 3 complete, set Plan 4 (executor + run + SIGINT) as next, reproduce SI-1 and SI-2 verbatim.
- [ ] **Push** and log in `gh-log.md`.

## Deferred to later plans (recorded so nothing is silently dropped)

- Executor, `run` subcommand, FIFO job queue, `--jobs N`, `--fail-fast`, SIGINT cleanup -> **Plan 4**.
- Persisted per-job logs in the app-data directory (spec 6) -> **Plan 5 (GUI)**.
- proptest coverage for the match algebra over attachments (spec 10) -> fold into Plan 4 or a cleanup pass; the Plan 2 follow-up list (`FINAL-review.md` minors, regex-recompile-per-`matches`, mkvmerge in CI) still stands.

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
## Appended reference 2: canonical argv contract

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

