//! Golden tests for `command::command` (Task 9-11 progressively lock the
//! canonical argv contract; this file covers Task 9's slice: global section,
//! single primary input group with track selection, and `--track-order`).

use muxsmith_core::planner::*;
use std::path::PathBuf;

fn p(s: &str) -> PathBuf {
    PathBuf::from(s)
}

#[test]
fn global_and_single_video_group() {
    let plan = Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        assignments: vec![Assignment {
            rule_index: 0,
            source: p("/m/e.mkv"),
            track_id: Some(0),
            track_kind: Some("video".into()),
            changes: vec![],
        }],
        attachments: AttachmentPlan {
            primary: PrimaryAttachments::KeepAll,
            add_files: vec![],
        },
        chapters: ChapterSource::Keep,
        tags: TagFlags {
            global_keep: true,
            track_keep: true,
        },
        title: TitleAction::Clear,
    };
    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--title",
            "",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "(",
            "/m/e.mkv",
            ")",
            "--track-order",
            "0:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn unmatched_donor_rule_opens_no_input_group() {
    let plan = Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        assignments: vec![
            Assignment {
                rule_index: 0,
                source: p("/m/e.mkv"),
                track_id: Some(0),
                track_kind: Some("video".into()),
                changes: vec![],
            },
            Assignment {
                rule_index: 1,
                source: p("/m/e.tr.srt"),
                track_id: None,
                track_kind: None,
                changes: vec![],
            },
        ],
        attachments: AttachmentPlan {
            primary: PrimaryAttachments::KeepAll,
            add_files: vec![],
        },
        chapters: ChapterSource::Keep,
        tags: TagFlags {
            global_keep: true,
            track_keep: true,
        },
        title: TitleAction::Clear,
    };
    let argv = muxsmith_core::command::command(&plan);

    assert!(
        !argv.iter().any(|a| a == "/m/e.tr.srt"),
        "unmatched donor source must not open an input group: {argv:?}"
    );

    let track_order = argv
        .iter()
        .position(|a| a == "--track-order")
        .map(|i| argv[i + 1].as_str());
    assert_eq!(track_order, Some("0:0"));
}
