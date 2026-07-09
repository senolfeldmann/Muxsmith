# Muxsmith Plan 3: full resolution and command generation (pure layer)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn a validated profile + identification into a fully resolved, self-contained `Plan` for every primary, and a pure `command(&Plan) -> Vec<String>` that renders the mkvmerge argv. No process execution (that is Plan 4).

**Architecture:** Extend `identify` to parse attachments and chapters from `-J`. Generalize the `matcher` over a `Matchable` trait so the same match algebra evaluates track rules and attachment rules. Extend the `planner`'s per-file resolution to populate an enriched `Plan` (resolved `changes` per assignment; attachments, chapters, tags, title). Add a new `command` module that consumes only the `Plan`. See `docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md` (D7-D12) and the v1 spec (sections 4.9, 6). Spec wins on conflict.

**Tech Stack:** Rust 2024, workspace crates `muxsmith-core` (all logic) + `muxsmith-cli`. serde_json for `-J` parsing, `regex` for match evaluation (already deps). No new runtime dependencies.

## Global Constraints

Every task's requirements implicitly include these (verbatim from the spec / HANDOFF):

- **Per-commit gate, never skipped:** `cargo test --workspace` AND `cargo fmt --all --check` AND `cargo clippy --workspace --all-targets -- -D warnings` AND `cargo deny check` all pass before each commit.
- **Core is prose-free:** `muxsmith-core` emits diagnostic `code` + `params` only; no user-facing strings. `#![deny(missing_docs)]` on the lib crate: every new `pub` item gets a doc comment.
- **Confirm mkvmerge behavior by running the binary (v100 installed), never from memory.** The `-J` shape is pinned to identification schema v20 (`~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json`).
- **Typography:** ASCII punctuation in code and comments; no em-dashes, no smart quotes.
- **Commits:** GPG blocks agent commits, use `git -c commit.gpgsign=false commit ...`. Trailer final line: `Co-Authored-By: Claude Opus 4.8 (1M context) <noreply@anthropic.com>`. Commits/pushes authorized for this repo; log every push in `gh-log.md`.
- **Diagnostics are data:** construct via `Diagnostic::error|warning|info(code, config_path).with(key, val).for_file(path)`; reuse existing `DiagCode` variants, do not add new ones in this plan (all needed codes exist).

---

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

---

### Task 1: `identify` parses attachments and chapters

**Files:**
- Modify: `crates/muxsmith-core/src/identify.rs`
- Test: same file's `#[cfg(test)] mod tests` (add cases) plus `crates/muxsmith-core/tests/identify_live.rs` if a live case fits; unit tests suffice here.

**Interfaces:**
- Consumes: existing `PropValue::from_json`, `Identification::from_json` (serde_json `Value`).
- Produces: `pub struct Attachment { pub id: u64, pub file_name: String, pub size: u64, pub content_type: Option<String>, pub description: Option<String>, pub uid: Option<u64> }` with `pub fn get(&self, name: &str) -> Option<PropValue>`; `Identification` gains `pub attachments: Vec<Attachment>` and `pub chapters: u64` (total entry count, summed over the `-J` `chapters` array's `num_entries`).

- [ ] **Step 1: Write failing tests.** In `identify.rs` tests, add fixture JSON and assertions:

```rust
#[test]
fn parses_attachments_with_optional_fields() {
    let json = r#"{
      "file_name": "e.mkv",
      "identification_format_version": 20,
      "container": { "recognized": true, "supported": true },
      "tracks": [],
      "attachments": [
        { "id": 1, "file_name": "font.ttf", "size": 1234,
          "content_type": "application/x-truetype-font",
          "description": "Main font", "properties": { "uid": 99 } },
        { "id": 2, "file_name": "cover.jpg", "size": 5678, "properties": {} }
      ],
      "chapters": [ { "num_entries": 12 } ]
    }"#;
    let id = Identification::from_json(json).unwrap();
    assert_eq!(id.attachments.len(), 2);
    let a = &id.attachments[0];
    assert_eq!(a.id, 1);
    assert_eq!(a.file_name, "font.ttf");
    assert_eq!(a.size, 1234);
    assert_eq!(a.content_type.as_deref(), Some("application/x-truetype-font"));
    assert_eq!(a.description.as_deref(), Some("Main font"));
    assert_eq!(a.uid, Some(99));
    assert_eq!(id.attachments[1].content_type, None);
    assert_eq!(id.attachments[1].description, None);
    assert_eq!(id.attachments[1].uid, None);
    assert_eq!(id.chapters, 12);
}

#[test]
fn absent_attachments_and_chapters_default_empty() {
    let json = r#"{ "file_name": "e.mkv", "identification_format_version": 20,
      "container": { "recognized": true, "supported": true }, "tracks": [] }"#;
    let id = Identification::from_json(json).unwrap();
    assert!(id.attachments.is_empty());
    assert_eq!(id.chapters, 0);
}

#[test]
fn attachment_get_exposes_match_properties() {
    let json = r#"{ "file_name": "e.mkv", "identification_format_version": 20,
      "container": { "recognized": true, "supported": true }, "tracks": [],
      "attachments": [ { "id": 3, "file_name": "f.otf", "size": 10,
        "content_type": "font/otf", "properties": {} } ] }"#;
    let a = &Identification::from_json(json).unwrap().attachments[0];
    assert_eq!(a.get("file_name"), Some(PropValue::Str("f.otf".into())));
    assert_eq!(a.get("content_type"), Some(PropValue::Str("font/otf".into())));
    assert_eq!(a.get("description"), None);
    assert_eq!(a.get("id"), Some(PropValue::Int(3)));
    assert_eq!(a.get("size"), Some(PropValue::Int(10)));
    assert_eq!(a.get("nope"), None);
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core identify` -> FAIL (`Attachment` undefined, no `attachments`/`chapters` field).

- [ ] **Step 3: Implement.** Add the `Attachment` struct with a doc comment on each field, a `parse_attachment(v: &Value) -> Option<Attachment>` mirroring `parse_track` (required: `id`, `file_name`, `size`; optional: `content_type`, `description`; `uid` nested under `properties.uid`), the `get` method (map `file_name`/`content_type`/`description` -> `Str`, `id`/`size` -> `Int`, else `None`), add `attachments` and `chapters` to `Identification` and to `from_json`:

```rust
let attachments = v.get("attachments").and_then(Value::as_array)
    .map(|arr| arr.iter().filter_map(parse_attachment).collect())
    .unwrap_or_default();
let chapters = v.get("chapters").and_then(Value::as_array)
    .map(|arr| arr.iter()
        .filter_map(|c| c.get("num_entries").and_then(Value::as_u64))
        .sum())
    .unwrap_or(0);
```

`parse_attachment`:

```rust
fn parse_attachment(v: &Value) -> Option<Attachment> {
    let id = v.get("id").and_then(Value::as_u64)?;
    let file_name = v.get("file_name").and_then(Value::as_str)?.to_string();
    let size = v.get("size").and_then(Value::as_u64)?;
    let content_type = v.get("content_type").and_then(Value::as_str).map(str::to_string);
    let description = v.get("description").and_then(Value::as_str).map(str::to_string);
    let uid = v.get("properties").and_then(|p| p.get("uid")).and_then(Value::as_u64);
    Some(Attachment { id, file_name, size, content_type, description, uid })
}
```

Update every `Identification { ... }` literal in the codebase (tests, fixtures) to include the two new fields; `cargo build` will point them out.

- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core identify` -> PASS.
- [ ] **Step 5: Gate + commit.** Run the full gate. `git -c commit.gpgsign=false commit` with message `feat(identify): parse attachments and chapters from -J`.

---

### Task 2: generalize the matcher over a `Matchable` trait

**Files:**
- Modify: `crates/muxsmith-core/src/matcher.rs`

**Interfaces:**
- Consumes: `identify::{Track, PropValue}`, `Track::get`.
- Produces: `pub trait Matchable { fn get(&self, prop: &str) -> Option<PropValue>; }` (impl'd for `Track`); `pub fn matches<M: Matchable>(expr: &MatchExpr, item: &M, lang: &LanguageIndex) -> bool` (signature widened from `&Track` to `&M`). No behavior change for tracks: all existing `matcher` tests must still pass unchanged.

- [ ] **Step 1: Write the failing test.** Add a test that pins the trait exists and `matches` is generic by calling it through a generic helper:

```rust
#[test]
fn matches_is_generic_over_matchable() {
    fn check<M: Matchable>(m: &M) -> bool {
        matches(&expr("exact: { type: audio }"), m, &lang())
    }
    let t = track("audio", &[]);
    assert!(check(&t));
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core matcher` -> FAIL (`Matchable` undefined).

- [ ] **Step 3: Implement.** Define the trait, impl it for `Track` (delegating to the existing `Track::get`), and make `matches`, `exact_matches`, and the `track_str` helper generic over `M: Matchable`. Rename `track_str` to `item_str<M: Matchable>(prop, item)`. The language and `codec_kind` special cases in `exact_matches` stay: they consult `item.get(...)` and are simply never triggered for property sets that lack those names. The boolean-absent-false branch keeps consulting `matchable_type` (a track-schema fact); for a non-track item whose property is absent and not in the track table it yields `false`, which is correct.

```rust
pub trait Matchable {
    /// The value of a match property, or `None` if absent.
    fn get(&self, prop: &str) -> Option<PropValue>;
}

impl Matchable for Track {
    fn get(&self, prop: &str) -> Option<PropValue> { Track::get(self, prop) }
}
```

- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> all matcher + planner tests PASS (planner calls `matcher::matches` with `&Track`, still resolves via type inference).
- [ ] **Step 5: Gate + commit.** `refactor(matcher): generalize matches over a Matchable trait`.

---

### Task 3: attachment matching (`Attachment: Matchable`)

**Files:**
- Modify: `crates/muxsmith-core/src/matcher.rs` (impl + tests) and `crates/muxsmith-core/src/identify.rs` (nothing new; `Attachment::get` from Task 1).

**Interfaces:**
- Consumes: `Matchable` (Task 2), `Attachment::get` (Task 1).
- Produces: `impl Matchable for Attachment`, so `matcher::matches(expr, attachment, lang)` evaluates attachment rules with the same algebra (spec 4.9).

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn attachment_matching_uses_the_same_algebra() {
    use crate::identify::Attachment;
    let font = Attachment {
        id: 1, file_name: "Roboto.ttf".into(), size: 100,
        content_type: Some("font/ttf".into()), description: None, uid: None,
    };
    assert!(matches(&expr("substring: { file_name: robot }"), &font, &lang()));
    assert!(matches(&expr("exact: { content_type: font/ttf }"), &font, &lang()));
    assert!(matches(
        &expr("any:\n  - substring: { file_name: .ttf }\n  - substring: { file_name: .otf }"),
        &font, &lang()));
    assert!(!matches(&expr("exact: { description: whatever }"), &font, &lang()));
    assert!(!matches(&expr("substring: { content_type: pdf }"), &font, &lang()));
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core attachment_matching` -> FAIL (`Attachment` does not implement `Matchable`).
- [ ] **Step 3: Implement.** `impl Matchable for Attachment { fn get(&self, prop: &str) -> Option<PropValue> { Attachment::get(self, prop) } }`.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core attachment_matching` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(matcher): match attachment rules via Matchable`.

---

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

### Task 6: resolve title and tags

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `profile.title: TitleCfg`, `profile.tags: TagsCfg`, `template::{Template, Ctx}`, `primary.identifier.to_ctx()`, `KeepDrop`.
- Produces: populated `Plan.title` and `Plan.tags`.

- [ ] **Step 1: Write the failing tests.** `title: clear` -> `TitleAction::Clear`; `title: { template: "Show S{season}" }` on a primary with `season=03` -> `TitleAction::Set("Show S03")` (raw capture; filters as spec 4.7); `title: keep` -> `TitleAction::Keep`. `tags: { global: drop, track: keep }` -> `TagFlags { global_keep: false, track_keep: true }`.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution title` -> FAIL.
- [ ] **Step 3: Implement.** Add a helper `resolve_title(profile, primary, diags) -> TitleAction`:
  - `TitleCfg::Keyword(k)` where `k == "keep"` -> `Keep`; `k == "clear"` -> `Clear`. (Validate.rs already rejects other keywords, so no diagnostic needed here; an unexpected keyword defaults to `Keep`.)
  - `TitleCfg::Template(block)` -> parse via `Template::parse`, render with `primary.identifier.to_ctx()` via `render_literal`, return `Set(rendered)`. A template that fails to parse cannot occur post-validate; on the off chance, fall back to `Keep` (no panic). Title has no path-separator / empty-name invariants (unlike filenames): an empty rendered title is a legitimate empty title.
  Add `resolve_tags(profile) -> TagFlags`: `TagFlags { global_keep: profile.tags.global == KeepDrop::Keep, track_keep: profile.tags.track == KeepDrop::Keep }`. Wire both into the `Plan { ... }` construction.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve title and tags`.

---

### Task 7: resolve chapters (keep / drop / external locator)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `profile.chapters: ChaptersCfg`, `discovery::resolve_locator`, `DiagCode::{MissingExternal, AmbiguousExternal}`.
- Produces: populated `Plan.chapters`. External chapters reuse the locator machinery and its diagnostics; a chapters file is NOT identified via mkvmerge (it is XML/simple, passed straight to `--chapters`).

- [ ] **Step 1: Write the failing tests.** `chapters: drop` -> `ChapterSource::Drop`; `chapters: keep` -> `Keep`; `chapters: { external: { path: ".", extensions: [xml], match_to_source: true } }` with exactly one matching `<id>.xml` beside the primary -> `ChapterSource::External(<that path>)`; zero matches -> `MissingExternal` diagnostic at `chapters.external` and the file gets no plan; two matches -> `AmbiguousExternal`.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution chapters` -> FAIL.
- [ ] **Step 3: Implement.** Add `resolve_chapters(profile, primary, primary_dir, diags) -> ChapterSource`:
  - `ChaptersCfg::Keyword(k)`: `"keep"` -> `Keep`, `"drop"` -> `Drop`.
  - `ChaptersCfg::External(block)`: `discovery::resolve_locator(&block.external, primary_dir, &primary.identifier)`; match `hits.len()`: `1` -> `External(path)`; `0` -> push `Diagnostic::error(MissingExternal, "chapters.external").for_file(&primary.path)`, return `Keep` (a placeholder; the error already suppresses the plan); `n` -> push `Diagnostic::error(AmbiguousExternal, "chapters.external").for_file(&primary.path).with("count", n)`, return `Keep`. Wire into `Plan`. Because an error diagnostic already forces `plan: None`, the returned placeholder is never emitted.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve chapters (keep/drop/external)`.

---

### Task 8: resolve attachments (rules + unmatched + external adds)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`; `crates/muxsmith-core/src/report.rs` (one doc-comment tweak on `MissingExternal`)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `ident.attachments: Vec<Attachment>` (primary's, Task 1), `profile.attachments: AttachmentsCfg` (`unmatched`, `rules`), `matcher::matches` for attachments (Task 3), `discovery::resolve_locator`, `KeepDrop`, `DiagCode::MissingExternal`.
- Produces: populated `Plan.attachments: AttachmentPlan`.

**Decisions locked here:**
- **Attachment scope (design-decisions D10):** rules and `unmatched` apply to the **primary file's** attachments only; donor attachments never flow in (command emits `--no-attachments` on donor groups, Task 11).
- **`add` cardinality and zero-match (design-decisions D12):** an `add` locator attaches **all** files it matches (a `Locator` is a query that populates the attachment collection, like `select`/`drop`; not a unique slot-filler like a track/chapters donor), appended to `add_files` in resolution order and **deduplicated by path** (two rules matching one file attach it once). An `add` that matches **zero** files emits a **warning** `MissingExternal` at `attachments.rules[i].add` (auxiliary payload, not an error: it does not suppress the plan). `content_type`/name are left to mkvmerge to infer.

- [ ] **Step 1: Write the failing tests.**

```rust
// Primary has attachments: id0 "a.ttf", id1 "b.otf", id2 "cover.jpg".
// rules: [ { select: { substring: { file_name: .ttf } } } ], unmatched: drop
// -> keep only id0:
assert_eq!(plan.attachments.primary, PrimaryAttachments::Subset(vec![0]));
// rules: [], unmatched: keep -> KeepAll
// rules: [], unmatched: drop -> DropAll
// rules: [ { drop: { substring: { file_name: cover } } } ], unmatched: keep
//   -> keep id0,id1 (all but the dropped) = Subset(vec![0,1])
// add locator matching two font files beside the primary -> add_files has both, sorted
// two add rules matching the same file -> that path appears once (dedup by path)
// add locator matching zero -> warning MissingExternal at attachments.rules[i].add,
//   plan still present
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution attachment` -> FAIL.
- [ ] **Step 3: Implement.** Add `resolve_attachments(profile, primary, primary_dir, primary_attachments: &[Attachment], diags) -> AttachmentPlan`:
  - **Existing attachments (select/drop/unmatched):** for each attachment of the primary, walk `profile.attachments.rules` in order; the **first** rule with a `select` expr that matches -> keep; the first with a `drop` expr that matches -> drop; `add` rules are skipped in this pass. If no `select`/`drop` rule matches, fall to `unmatched` (`Keep` -> keep, `Drop` -> drop). Collect kept ids. Then reduce to `PrimaryAttachments`: kept == all ids -> `KeepAll`; kept empty -> `DropAll`; else `Subset(sorted kept ids)`. (Reducing to `KeepAll` when everything is kept keeps the argv minimal; command emits no filter.)
  - **Adds:** for each rule with `add: Some(locator)`, `resolve_locator(locator, primary_dir, &primary.identifier)`; extend `add_files` with all hits (already sorted by `walk_files`); if that rule's hits are empty, push `Diagnostic::warning(MissingExternal, format!("attachments.rules[{i}].add")).for_file(&primary.path)`. After all rules, dedup `add_files` by path preserving first-seen order (two rules matching one file must attach it once).
  - Return `AttachmentPlan { primary, add_files }`. Wire into `Plan`. Update the `MissingExternal` doc comment in `report.rs` to read "track rule, chapters, or attachment add".
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve attachments (rules, unmatched, external adds)`.

---

### Task 9: `command` module - global section + track-order skeleton

**Files:**
- Create: `crates/muxsmith-core/src/command.rs`
- Modify: `crates/muxsmith-core/src/lib.rs` (add `pub mod command;`)
- Test: `crates/muxsmith-core/tests/command.rs` (new)

**Interfaces:**
- Consumes: the enriched `Plan` (Task 4).
- Produces: `pub fn command(plan: &Plan) -> Vec<String>`. This task implements the global section (output, title, chapters-external, attach-file) plus a single primary input group with track selection and `--track-order`; attachments and per-track props land in Tasks 10-11. Build incrementally but keep every committed state green.

- [ ] **Step 1: Write the failing golden test.** A minimal plan: one primary `/m/e.mkv`, output `/out/e.mkv`, one video assignment (track 0, kind video), `title: Clear`, `chapters: Keep`, `tags` all keep, `attachments KeepAll`, no changes:

```rust
use muxsmith_core::planner::*;
use std::path::PathBuf;

fn p(s: &str) -> PathBuf { PathBuf::from(s) }

#[test]
fn global_and_single_video_group() {
    let plan = Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        assignments: vec![Assignment {
            rule_index: 0, source: p("/m/e.mkv"),
            track_id: Some(0), track_kind: Some("video".into()), changes: vec![],
        }],
        attachments: AttachmentPlan { primary: PrimaryAttachments::KeepAll, add_files: vec![] },
        chapters: ChapterSource::Keep,
        tags: TagFlags { global_keep: true, track_keep: true },
        title: TitleAction::Clear,
    };
    assert_eq!(muxsmith_core::command::command(&plan), vec![
        "--output", "/out/e.mkv",
        "--title", "",
        "--video-tracks", "0",
        "--no-audio", "--no-subtitles", "--no-buttons",
        "(", "/m/e.mkv", ")",
        "--track-order", "0:0",
    ].into_iter().map(String::from).collect::<Vec<_>>());
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test command` -> FAIL (no `command`).
- [ ] **Step 3: Implement** the global section and the input-group scaffolding per the canonical-argv reference. Compute input groups: `let mut groups: Vec<PathBuf> = vec![plan.source.clone()];` then append distinct `assignment.source` values (in first appearance) not already present. A helper `group_index(groups, path) -> usize`. Track selection per group per the four categories (Task uses only video/audio/subtitles/buttons flag names from the reference). `--track-order` from assignments with `track_id = Some`. Emit as `Vec<String>` (use `push`/`extend`). Doc-comment the module and `command`.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core --test command` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(command): global section and track-order`.

---

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
