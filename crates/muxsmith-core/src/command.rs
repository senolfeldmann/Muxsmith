//! Renders a resolved [`Plan`] into the mkvmerge argument vector (spec 4.9).
//! `command` produces argv only: no process invocation, no filesystem
//! access beyond the paths already carried in the `Plan`. The canonical
//! ordering (global section, then one input group per distinct source, then
//! `--track-order`) is locked by the Task 9-11 golden tests; this module
//! currently implements the global section and the input-group scaffolding
//! with track selection (Task 9). Per-track property options (Task 10) and
//! the attachment-filter/`--no-chapters`/`--no-*-tags` flags (Task 11) slot
//! into [`push_group`] without reordering what is already here.

use std::path::{Path, PathBuf};

use crate::planner::{ChapterSource, Plan, TitleAction};

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
// distinct `Assignment::source` not already present, in first-appearance
// order (spec 4.9 item 2).
fn input_groups(plan: &Plan) -> Vec<PathBuf> {
    let mut groups = vec![plan.source.clone()];
    for a in &plan.assignments {
        if !groups.iter().any(|g| g.as_path() == a.source.as_path()) {
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

// One input group's argv (spec 4.9 item 2). Task 11 adds the
// `--no-chapters`, `--no-global-tags`/`--no-track-tags`, and
// attachment-filter flags before track selection; Task 10 adds per-track
// property options after it; both slot in here without moving the
// selection call or the closing bracket.
fn push_group(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    push_track_selection(argv, plan, source);

    argv.push("(".to_string());
    argv.push(source.display().to_string());
    argv.push(")".to_string());
}

// Track selection for one group, categories in fixed order (spec 4.9 item
// 2d): assigned ids of that category in this group, ascending, if any;
// otherwise the category's `--no-*` flag.
fn push_track_selection(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
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
