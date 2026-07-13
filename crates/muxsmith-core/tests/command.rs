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
        [
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

// Fast, unconditional counterpart to `command_integration.rs`'s
// `live_keep_donor_trails_primary` (Task 7 review: that test is gated on
// `Mkvmerge::locate()` and silently skips when mkvmerge is absent, e.g. in
// CI, leaving the keep-mode primary+donor `--track-order` branch with no
// deterministic regression guard). Two primary tracks plus one donor
// assignment on a different source: D20 says the primary leads in source
// order (group 0, ids 0 and 1), the donor trails (group 1, id 0).
#[test]
fn keep_unmatched_donor_trails_primary_track_order() {
    let plan = Plan {
        source: p("/m/show.mkv"),
        output: p("/out/show.mkv"),
        keep_unmatched: true,
        primary_track_ids: vec![0, 1],
        assignments: vec![Assignment {
            rule_index: 0,
            source: p("/m/donor.srt"),
            track_id: Some(0),
            track_kind: Some("subtitles".into()),
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
        title: TitleAction::Keep,
    };
    let argv = muxsmith_core::command::command(&plan);
    let track_order = argv
        .iter()
        .position(|a| a == "--track-order")
        .map(|i| argv[i + 1].as_str());
    assert_eq!(
        track_order,
        Some("0:0,0:1,1:0"),
        "keep-mode donor must trail every primary track (D20), got {argv:?}"
    );
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

// Golden pin (gap T-i): drop-mode `--track-order` when `track_id: None`
// assignments are interleaved BETWEEN `Some` ones across the primary and two
// distinct donors, not just trailing a single donor as in
// `unmatched_donor_rule_opens_no_input_group`. Pins two facts about
// `push_track_order`/`input_groups` together: (1) `None` assignments are
// skipped in place without shifting the ordering of the `Some` ones around
// them, wherever they sit in profile order; (2) a group's index is assigned
// by first-appearance of a `Some` assignment on that source, so a donor
// hit only by a later rule (donor A, rule 3) gets a HIGHER group index than
// one first hit earlier (donor B, rule 2), even though donor A is also
// referenced (unsuccessfully) by an earlier, `None` rule (rule 1). A wholly
// unreferenced-by-any-Some donor (rule 4) opens no group at all, confirming
// that holds even amid this denser mix.
#[test]
fn donor_ordering_drop_mode_with_mixed_none_and_some_assignments() {
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
                source: p("/m/e.en.srt"),
                track_id: None,
                track_kind: None,
                changes: vec![],
            },
            Assignment {
                rule_index: 2,
                source: p("/m/e.ac3"),
                track_id: Some(0),
                track_kind: Some("audio".into()),
                changes: vec![],
            },
            Assignment {
                rule_index: 3,
                source: p("/m/e.en.srt"),
                track_id: Some(0),
                track_kind: Some("subtitles".into()),
                changes: vec![],
            },
            Assignment {
                rule_index: 4,
                source: p("/m/e.commentary.ac3"),
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
        !argv.iter().any(|a| a == "/m/e.commentary.ac3"),
        "a source with only None assignments must never open an input group: {argv:?}"
    );

    let track_order = argv
        .iter()
        .position(|a| a == "--track-order")
        .map(|i| argv[i + 1].as_str());
    assert_eq!(
        track_order,
        Some("0:0,1:0,2:0"),
        "None assignments must be skipped in place; groups indexed by \
         first-Some-appearance (primary=0, e.ac3=1, e.en.srt=2), got {argv:?}"
    );
}

// Golden pin (gap T-i): keep-mode counterpart. D20's primary-then-donors
// order (`push_track_order_keep`) must also skip a `None` donor assignment
// in place rather than emitting a bogus entry for it, while still listing
// every REAL donor assignment that follows, in profile order.
#[test]
fn donor_ordering_keep_mode_with_mixed_none_and_some_assignments() {
    let plan = Plan {
        source: p("/m/show.mkv"),
        output: p("/out/show.mkv"),
        keep_unmatched: true,
        primary_track_ids: vec![0, 1],
        assignments: vec![
            Assignment {
                rule_index: 0,
                source: p("/m/missing.srt"),
                track_id: None,
                track_kind: None,
                changes: vec![],
            },
            Assignment {
                rule_index: 1,
                source: p("/m/donor.srt"),
                track_id: Some(0),
                track_kind: Some("subtitles".into()),
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
        title: TitleAction::Keep,
    };
    let argv = muxsmith_core::command::command(&plan);
    let track_order = argv
        .iter()
        .position(|a| a == "--track-order")
        .map(|i| argv[i + 1].as_str());
    assert_eq!(
        track_order,
        Some("0:0,0:1,1:0"),
        "keep-mode must skip the None donor assignment and still list the \
         real donor trailing every primary track (D20), got {argv:?}"
    );
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
        [
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
        [
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
        [
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
        [
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
        [
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
        [
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
        [
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
        [
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
        [
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
    );
}
