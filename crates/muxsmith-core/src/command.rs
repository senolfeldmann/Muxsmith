//! Renders a resolved [`Plan`] into the mkvmerge argument vector (spec 4.9).
//! `command` produces argv only: no process invocation, no filesystem
//! access beyond the paths already carried in the `Plan`. The canonical
//! ordering (global section, then one input group per distinct source, then
//! `--track-order`) is locked by the Task 9-11 golden tests: the global
//! section, multi-group input handling, track selection, and per-track
//! property options (Tasks 9-10), plus the per-group
//! attachment-filter/`--no-chapters`/`--no-*-tags` flags (Task 11).

use std::path::{Path, PathBuf};

use crate::planner::{AppliedChange, ChapterSource, Plan, PrimaryAttachments, TitleAction};
use crate::profile::match_expr::Scalar;

/// One track-selection category: the `-J` `type` string it matches, and the
/// pair of mkvmerge flags that select it or exclude it entirely. The
/// no-selection flag name does not mechanically derive from the category
/// name (`--no-video`/`--no-subtitles`/`--no-buttons` are singular-vs-plural
/// inconsistent with their select-flag counterparts), so both are spelled
/// out rather than templated.
struct Category {
    kind: &'static str,
    select_flag: &'static str,
    no_flag: &'static str,
}

/// Track categories in mkvmerge's fixed selection order (spec 4.9 item 2d).
const CATEGORIES: [Category; 4] = [
    Category {
        kind: "video",
        select_flag: "--video-tracks",
        no_flag: "--no-video",
    },
    Category {
        kind: "audio",
        select_flag: "--audio-tracks",
        no_flag: "--no-audio",
    },
    Category {
        kind: "subtitles",
        select_flag: "--subtitle-tracks",
        no_flag: "--no-subtitles",
    },
    Category {
        kind: "buttons",
        select_flag: "--button-tracks",
        no_flag: "--no-buttons",
    },
];

/// Renders `plan` into the mkvmerge argument vector, without the leading
/// `mkvmerge` program name (spec 4.9). Deterministic: the global section,
/// then one input group per distinct source (the primary always first),
/// then `--track-order`.
pub fn command(plan: &Plan) -> Vec<String> {
    let mut argv: Vec<String> = Vec::new();
    let groups = input_groups(plan);

    push_global(&mut argv, plan);
    for source in &groups {
        push_group(&mut argv, plan, source);
    }
    push_track_order(&mut argv, plan, &groups);

    argv
}

// Computes the input groups: the primary is always group 0, then every
// distinct `Assignment::source` with a resolved `track_id` not already
// present, in first-appearance order (spec 4.9 item 2). A source whose only
// assignment(s) have `track_id: None` (an external rule that matched no
// track) contributes no group: it would carry no kept track into the output
// (and its attachments are dropped regardless, per D10), so opening it as an
// input at all would just be dead weight in the command.
fn input_groups(plan: &Plan) -> Vec<PathBuf> {
    let mut groups = vec![plan.source.clone()];
    for a in &plan.assignments {
        if a.track_id.is_some() && !groups.iter().any(|g| g.as_path() == a.source.as_path()) {
            groups.push(a.source.clone());
        }
    }
    groups
}

// Index of `path` within `groups`. Every assignment source is always one of
// the groups `input_groups` computed (it is built from those same sources),
// so a lookup miss here would mean `input_groups` itself is wrong, not a
// legitimate "unknown source" case worth propagating as an error.
fn group_index(groups: &[PathBuf], path: &Path) -> usize {
    groups
        .iter()
        .position(|g| g.as_path() == path)
        .expect("assignment source is always one of the computed input groups")
}

// The global section: `--output`, title, external chapters, `--attach-file`
// per add (spec 4.9 item 1).
fn push_global(argv: &mut Vec<String>, plan: &Plan) {
    argv.push("--output".to_string());
    argv.push(plan.output.display().to_string());

    match &plan.title {
        TitleAction::Keep => {}
        TitleAction::Clear => {
            argv.push("--title".to_string());
            argv.push(String::new());
        }
        TitleAction::Set(s) => {
            argv.push("--title".to_string());
            argv.push(s.clone());
        }
    }

    if let ChapterSource::External(path) = &plan.chapters {
        argv.push("--chapters".to_string());
        argv.push(path.display().to_string());
    }

    for add in &plan.attachments.add_files {
        argv.push("--attach-file".to_string());
        argv.push(add.display().to_string());
    }
}

// One input group's argv (spec 4.9 item 2), in canonical order: chapters
// (a), tags (b), attachments (c), track selection (d), per-track properties
// (e), then the bracketed source (f).
fn push_group(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    push_group_chapters(argv, plan);
    push_group_tags(argv, plan);
    push_group_attachments(argv, plan, source);
    push_track_selection(argv, plan, source);
    push_track_properties(argv, plan, source);

    argv.push("(".to_string());
    argv.push(source.display().to_string());
    argv.push(")".to_string());
}

// `--no-chapters` on every input group when chapters are dropped or replaced
// by an external file (spec 4.9 item 2a); `Keep` emits nothing.
fn push_group_chapters(argv: &mut Vec<String>, plan: &Plan) {
    match &plan.chapters {
        ChapterSource::Keep => {}
        ChapterSource::Drop | ChapterSource::External(_) => {
            argv.push("--no-chapters".to_string());
        }
    }
}

// `--no-global-tags`/`--no-track-tags` on every input group per `plan.tags`
// (spec 4.9 item 2b).
fn push_group_tags(argv: &mut Vec<String>, plan: &Plan) {
    if !plan.tags.global_keep {
        argv.push("--no-global-tags".to_string());
    }
    if !plan.tags.track_keep {
        argv.push("--no-track-tags".to_string());
    }
}

// Attachment filter for one group (spec 4.9 item 2c, D10). The primary group
// (`source == plan.source`) follows `PrimaryAttachments`; every donor group
// always gets `--no-attachments` since donor attachments never flow into the
// output.
fn push_group_attachments(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    if source != plan.source.as_path() {
        argv.push("--no-attachments".to_string());
        return;
    }

    match &plan.attachments.primary {
        PrimaryAttachments::KeepAll => {}
        PrimaryAttachments::Subset(ids) => {
            argv.push("--attachments".to_string());
            argv.push(ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","));
        }
        PrimaryAttachments::DropAll => argv.push("--no-attachments".to_string()),
    }
}

// Track selection for one group, categories in fixed order (spec 4.9 item
// 2d): assigned ids of that category in this group, ascending, if any;
// otherwise the category's `--no-*` flag.
fn push_track_selection(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    // tracks.unmatched: keep -> pass all PRIMARY tracks through (no selection
    // flags); mkvmerge keeps every track by default. Donor groups still get
    // their normal per-category selection.
    if plan.keep_unmatched && source == plan.source.as_path() {
        return;
    }
    for cat in &CATEGORIES {
        let mut ids: Vec<u64> = plan
            .assignments
            .iter()
            .filter(|a| a.source.as_path() == source && a.track_kind.as_deref() == Some(cat.kind))
            .filter_map(|a| a.track_id)
            .collect();
        ids.sort_unstable();

        if ids.is_empty() {
            argv.push(cat.no_flag.to_string());
        } else {
            argv.push(cat.select_flag.to_string());
            argv.push(ids.iter().map(u64::to_string).collect::<Vec<_>>().join(","));
        }
    }
}

// Per-track property options for one group (spec 4.9 item 2e): for each
// assigned track in this group (track_id ascending), for each
// `AppliedChange` on that assignment (property name ascending; the
// `changes` vec is already property-ascending from the planner, sorted
// again here defensively), the mkvmerge option for the property followed
// by `<tid>:<value>`.
fn push_track_properties(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    let mut tracks: Vec<(u64, &[AppliedChange])> = plan
        .assignments
        .iter()
        .filter(|a| a.source.as_path() == source)
        .filter_map(|a| a.track_id.map(|tid| (tid, a.changes.as_slice())))
        .collect();
    tracks.sort_by_key(|(tid, _)| *tid);

    for (tid, changes) in tracks {
        let mut changes: Vec<&AppliedChange> = changes.iter().collect();
        changes.sort_by(|a, b| a.property.cmp(&b.property));

        for c in changes {
            let (_, option) = crate::capability::settable(&c.property)
                .expect("changes carry only validated settable properties");
            argv.push(option.to_string());
            argv.push(format!("{tid}:{}", value_str(&c.value)));
        }
    }
}

// Renders a Scalar as the plain value string a per-track property option
// takes: `1`/`0` for booleans (mkvmerge's flag encoding), the raw string
// for `Str` (no quoting), `to_string()` otherwise.
fn value_str(value: &Scalar) -> String {
    match value {
        Scalar::Bool(b) => (if *b { "1" } else { "0" }).to_string(),
        Scalar::Str(s) => s.clone(),
        Scalar::Int(i) => i.to_string(),
        Scalar::Float(f) => f.to_string(),
    }
}

// `--track-order g:tid,...`: one entry per assignment with a resolved
// track_id, in profile (assignment) order, `g` the input group index of
// that assignment's source (spec 4.9 item 3). Omitted entirely if no
// assignment has a track.
fn push_track_order(argv: &mut Vec<String>, plan: &Plan, groups: &[PathBuf]) {
    let entries: Vec<String> = plan
        .assignments
        .iter()
        .filter_map(|a| {
            a.track_id
                .map(|tid| format!("{}:{}", group_index(groups, &a.source), tid))
        })
        .collect();

    if entries.is_empty() {
        return;
    }
    argv.push("--track-order".to_string());
    argv.push(entries.join(","));
}
