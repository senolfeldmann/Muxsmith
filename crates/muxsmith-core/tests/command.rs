//! Golden tests for `command::command` (Task 9-11 progressively lock the
//! canonical argv contract; Task 9 covered the global section, single
//! primary input group with track selection, and `--track-order`; this file
//! now also covers Task 10's slice: multi-group input handling and
//! per-track property options).

use muxsmith_core::planner::*;
use muxsmith_core::profile::match_expr::Scalar;
use std::path::PathBuf;

fn p(s: &str) -> PathBuf {
    PathBuf::from(s)
}

fn change(property: &str, value: Scalar) -> AppliedChange {
    AppliedChange {
        property: property.to_string(),
        value,
    }
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

#[test]
fn per_track_properties_and_multi_group() {
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
                source: p("/m/e.mkv"),
                track_id: Some(1),
                track_kind: Some("audio".into()),
                changes: vec![
                    change("language", Scalar::Str("de".into())),
                    change("default_track", Scalar::Bool(true)),
                ],
            },
            Assignment {
                rule_index: 2,
                source: p("/m/e.tr.srt"),
                track_id: Some(0),
                track_kind: Some("subtitles".into()),
                changes: vec![
                    change("track_name", Scalar::Str("Turkce".into())),
                    change("language", Scalar::Str("tr".into())),
                ],
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
        title: TitleAction::Keep,
    };

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--video-tracks",
            "0",
            "--audio-tracks",
            "1",
            "--no-subtitles",
            "--no-buttons",
            "--default-track-flag",
            "1:1",
            "--language",
            "1:de",
            "(",
            "/m/e.mkv",
            ")",
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "--language",
            "0:tr",
            "--track-name",
            "0:Turkce",
            "(",
            "/m/e.tr.srt",
            ")",
            "--track-order",
            "0:0,0:1,1:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn boolean_and_string_value_encoding() {
    let plan = Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        assignments: vec![Assignment {
            rule_index: 0,
            source: p("/m/e.mkv"),
            track_id: Some(0),
            track_kind: Some("video".into()),
            changes: vec![
                change("forced_track", Scalar::Bool(false)),
                change("language", Scalar::Str("en".into())),
            ],
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
        title: TitleAction::Keep,
    };

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "--forced-display-flag",
            "0:0",
            "--language",
            "0:en",
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
