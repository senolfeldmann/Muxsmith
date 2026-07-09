//! Golden tests for `command::command` (Task 9-11 progressively lock the
//! canonical argv contract; Task 9 covered the global section, single
//! primary input group with track selection, and `--track-order`; Task 10
//! added multi-group input handling and per-track property options; this
//! file now also covers Task 11's slice: per-group attachment/chapter/tag
//! flags).

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
        keep_unmatched: false,
        primary_track_ids: vec![0],
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
fn keep_unmatched_suppresses_primary_selection_flags() {
    let plan = Plan {
        source: p("/m/show.mkv"),
        output: p("/out/show.mkv"),
        keep_unmatched: true,
        // video(0), audio(1, matched below), subtitle(2, kept-unmatched):
        // the full primary track-id list `-J` would report, source order.
        primary_track_ids: vec![0, 1, 2],
        assignments: vec![Assignment {
            rule_index: 0,
            source: p("/m/show.mkv"),
            track_id: Some(1),
            track_kind: Some("audio".into()),
            changes: vec![change("default_track", Scalar::Bool(true))],
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
    let argv = muxsmith_core::command::command(&plan);
    assert!(
        !argv.iter().any(|a| a == "--no-video"
            || a == "--no-subtitles"
            || a == "--no-buttons"
            || a == "--audio-tracks"),
        "keep must emit no primary selection flags, got {argv:?}"
    );
    assert!(
        argv.windows(2)
            .any(|w| w[0] == "--default-track-flag" && w[1] == "1:1")
    );
    // D20: --track-order lists every primary track (0,1,2), source order,
    // group 0 -- not just the matched (audio, id 1) assignment.
    let track_order = argv
        .iter()
        .position(|a| a == "--track-order")
        .map(|i| argv[i + 1].as_str());
    assert_eq!(track_order, Some("0:0,0:1,0:2"));
}

#[test]
fn unmatched_donor_rule_opens_no_input_group() {
    let plan = Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        keep_unmatched: false,
        primary_track_ids: vec![0],
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
        keep_unmatched: false,
        primary_track_ids: vec![0, 1],
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
            "--no-attachments",
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
        keep_unmatched: false,
        primary_track_ids: vec![0],
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

// A single-group plan: one primary video track, no donor. Used by the
// attachment/global-flag tests below where a second group is not needed to
// demonstrate the behavior.
fn single_group_plan(attachments: AttachmentPlan) -> Plan {
    Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        keep_unmatched: false,
        primary_track_ids: vec![0],
        assignments: vec![Assignment {
            rule_index: 0,
            source: p("/m/e.mkv"),
            track_id: Some(0),
            track_kind: Some("video".into()),
            changes: vec![],
        }],
        attachments,
        chapters: ChapterSource::Keep,
        tags: TagFlags {
            global_keep: true,
            track_keep: true,
        },
        title: TitleAction::Keep,
    }
}

// A two-group plan: primary video track plus a donor subtitle track, so
// "every group" assertions (chapters, tags) are actually exercised across
// both the primary and a donor input group.
fn multi_group_plan(attachments: AttachmentPlan, chapters: ChapterSource, tags: TagFlags) -> Plan {
    Plan {
        source: p("/m/e.mkv"),
        output: p("/out/e.mkv"),
        keep_unmatched: false,
        primary_track_ids: vec![0],
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
                track_id: Some(0),
                track_kind: Some("subtitles".into()),
                changes: vec![],
            },
        ],
        attachments,
        chapters,
        tags,
        title: TitleAction::Keep,
    }
}

#[test]
fn attachments_subset_on_primary_no_attachments_on_donor() {
    let plan = multi_group_plan(
        AttachmentPlan {
            primary: PrimaryAttachments::Subset(vec![0, 2]),
            add_files: vec![],
        },
        ChapterSource::Keep,
        TagFlags {
            global_keep: true,
            track_keep: true,
        },
    );

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--attachments",
            "0,2",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "(",
            "/m/e.mkv",
            ")",
            "--no-attachments",
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "(",
            "/m/e.tr.srt",
            ")",
            "--track-order",
            "0:0,1:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn attachments_drop_all_on_primary() {
    let plan = single_group_plan(AttachmentPlan {
        primary: PrimaryAttachments::DropAll,
        add_files: vec![],
    });

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--no-attachments",
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
fn attachments_keep_all_emits_no_flag_on_primary() {
    let plan = single_group_plan(AttachmentPlan {
        primary: PrimaryAttachments::KeepAll,
        add_files: vec![],
    });

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
fn add_files_emit_global_attach_file() {
    let plan = single_group_plan(AttachmentPlan {
        primary: PrimaryAttachments::KeepAll,
        add_files: vec![p("/m/x.ttf")],
    });

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--attach-file",
            "/m/x.ttf",
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
fn chapters_drop_emits_no_chapters_on_every_group_and_no_global_flag() {
    let plan = multi_group_plan(
        AttachmentPlan {
            primary: PrimaryAttachments::KeepAll,
            add_files: vec![],
        },
        ChapterSource::Drop,
        TagFlags {
            global_keep: true,
            track_keep: true,
        },
    );

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--no-chapters",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "(",
            "/m/e.mkv",
            ")",
            "--no-chapters",
            "--no-attachments",
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "(",
            "/m/e.tr.srt",
            ")",
            "--track-order",
            "0:0,1:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn chapters_external_emits_global_chapters_and_no_chapters_on_every_group() {
    let plan = multi_group_plan(
        AttachmentPlan {
            primary: PrimaryAttachments::KeepAll,
            add_files: vec![],
        },
        ChapterSource::External(p("/m/e.xml")),
        TagFlags {
            global_keep: true,
            track_keep: true,
        },
    );

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--chapters",
            "/m/e.xml",
            "--no-chapters",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "(",
            "/m/e.mkv",
            ")",
            "--no-chapters",
            "--no-attachments",
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "(",
            "/m/e.tr.srt",
            ")",
            "--track-order",
            "0:0,1:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}

#[test]
fn tags_dropped_emit_flags_on_every_group() {
    let plan = multi_group_plan(
        AttachmentPlan {
            primary: PrimaryAttachments::KeepAll,
            add_files: vec![],
        },
        ChapterSource::Keep,
        TagFlags {
            global_keep: false,
            track_keep: false,
        },
    );

    assert_eq!(
        muxsmith_core::command::command(&plan),
        vec![
            "--output",
            "/out/e.mkv",
            "--no-global-tags",
            "--no-track-tags",
            "--video-tracks",
            "0",
            "--no-audio",
            "--no-subtitles",
            "--no-buttons",
            "(",
            "/m/e.mkv",
            ")",
            "--no-global-tags",
            "--no-track-tags",
            "--no-attachments",
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "(",
            "/m/e.tr.srt",
            ")",
            "--track-order",
            "0:0,1:0",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}
