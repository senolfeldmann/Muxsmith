//! Task 12: the whole pure layer end to end, plus real-mkvmerge acceptance
//! (spec 4.1, 6, 10).
//!
//! Two tests:
//! - `reference_example_end_to_end`: the spec 4.1 reference example (the
//!   full, German-completed profile checked in at `fixtures/reference.yaml`,
//!   see `docs/superpowers/plans/2026-07-08-plan-1-core-foundations-validate-cli.md`
//!   task 4) driven through `plan_batch` with a fake `Identify`, asserting
//!   the FULL `command::command` argv. Pure: no mkvmerge needed, always runs.
//! - `live_mkvmerge_accepts_planned_command`: spawns real mkvmerge on the
//!   argv a tiny real plan produces, gated on `Mkvmerge::locate()` (mirrors
//!   `identify_live.rs` / `mkvmerge_runtime.rs`'s self-skip pattern).
//! - `live_keep_donor_trails_primary` (Plan 3.5 Task 7): confirms the
//!   resolved D20 track order against real mkvmerge rather than from memory
//!   (SI-3): under `keep_unmatched`, `--track-order` lists every primary
//!   track first, in source order, before any donor track -- a primary +
//!   external-donor cross-file case, the whole-branch-review-flagged
//!   scenario D20 exists to fix (this test replaces the Task 3 version,
//!   which locked the superseded matched-first/unmatched-appended order).
//!
//! The parenthesized `( file )` input-group syntax `command.rs` emits for
//! every input source (spec 4.9 item 2f) was confirmed by hand against the
//! installed mkvmerge v100 before writing these tests: per `man mkvmerge`,
//! `( file1 file2 )` concatenates multiple files into one logical segment
//! and is documented as unusable for stand-alone self-contained containers
//! (it explicitly cannot be used for formats "which contains its own set of
//! headers", e.g. AVI/MP4 -- MKV likewise). A single-file group is the
//! degenerate n=1 case, which the same manual page states is equivalent to
//! prefixing the file with `=` (disables VOB-sibling auto-detection): a
//! harmless no-op for a non-VOB source. Manually running
//! `mkvmerge -o out.mkv --audio-tracks 0 ... ( a.mkv ) --subtitle-tracks 0 ... ( b.mkv ) --track-order 0:0,1:0`
//! against two real single-track MKVs produced exit 0 and the correctly
//! combined two-track output, confirming the existing `command.rs` argv
//! shape needs no change for Task 12.

use std::collections::HashMap;
use std::process::Command;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::command::command;
use muxsmith_core::identify::{Identification, IdentifyCache, LiveIdentifier, PropValue, Track};
use muxsmith_core::planner::{
    Assignment, AttachmentPlan, ChapterSource, Plan, PrimaryAttachments, RunInputs, TagFlags,
    TitleAction, plan_batch,
};
use muxsmith_core::profile::load::{Format, from_str};

mod support;
use support::{FakeIdent, lang};

// ---------------------------------------------------------------------------
// Pure golden: the spec 4.1 reference example, end to end.
// ---------------------------------------------------------------------------

// The full reference profile (spec 4.1, completed with the German subtitle
// trio the spec elides "for brevity"; see fixtures/reference.yaml's own
// history). Ten rules in profile order: video, audio en, audio de, subtitle
// en forced/plain/SDH, subtitle de forced/plain/SDH, external Turkish
// subtitle donor.
const REFERENCE_PROFILE: &str = include_str!("fixtures/reference.yaml");

// Nine primary tracks, one per non-external rule (ids 0-8, in rule order),
// each carrying exactly the properties that rule's match expression needs to
// resolve unambiguously: e.g. track 4 (subtitle, en, not forced, no SDH
// marker) matches only the "English" plain rule, never the forced or SDH
// rules, and track 5 (subtitle, en, not forced, flag_hearing_impaired) is
// excluded from the plain rule's `not` clause and falls to the SDH rule.
const REFERENCE_PRIMARY: &str = include_str!("fixtures/identify/reference-primary.json");

// The external Turkish-subtitle donor: one subtitle track, matched by the
// reference profile's final rule (`exact: { type: subtitles }`, no further
// constraint) regardless of its own language/name, since that rule's
// `changes` overwrite both anyway.
const REFERENCE_DONOR: &str = include_str!("fixtures/identify/reference-donor.json");

#[test]
fn reference_example_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    // The primary must match `input.pattern` (`S(?<season>\d{2})E(?<episode>\d{2})`)
    // and the external locator's `match_to_source` donor must share its
    // `{match}` identifier ("S01E01") as a basename substring (spec 4.6).
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    std::fs::write(dir.path().join("Show.S01E01.srt"), b"x").unwrap();

    let profile = from_str(REFERENCE_PROFILE, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut by_name = HashMap::new();
    by_name.insert(
        "Show.S01E01.mkv".to_string(),
        Identification::from_json(REFERENCE_PRIMARY).unwrap(),
    );
    by_name.insert(
        "Show.S01E01.srt".to_string(),
        Identification::from_json(REFERENCE_DONOR).unwrap(),
    );
    let mut ident = FakeIdent { by_name };

    let batch = plan_batch(&profile, &run, &mut ident, &lang());
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();

    // Sanity on the resolution itself before locking the argv: ten
    // assignments (nine primary tracks plus the external donor), all
    // resolved (no MissingTrack/AmbiguousRule -- the fixture tracks were
    // built to be unambiguous under the profile's match expressions).
    assert_eq!(plan.assignments.len(), 10);
    assert!(plan.assignments.iter().all(|a| a.track_id.is_some()));

    let primary_disp = plan.source.display().to_string();
    let output_disp = plan.output.display().to_string();
    let donor_disp = plan.assignments[9].source.display().to_string();
    assert_ne!(donor_disp, primary_disp);
    assert!(donor_disp.ends_with("Show.S01E01.srt"));

    assert_eq!(
        command(plan),
        [
            "--output",
            &output_disp,
            "--title",
            "",
            // --- primary group (spec 4.9 item 2, D10) ---
            "--no-global-tags", // tags.global: drop
            "--video-tracks",
            "0",
            "--audio-tracks",
            "1,2",
            "--subtitle-tracks",
            "3,4,5,6,7,8",
            "--no-buttons",
            "--default-track-flag",
            "1:1", // audio en: changes.default_track
            "--default-track-flag",
            "3:1", // subtitle en forced: changes (default_track < track_name)
            "--track-name",
            "3:English forced",
            "--track-name",
            "4:English", // subtitle en plain
            "--hearing-impaired-flag",
            "5:1", // subtitle en SDH (flag_hearing_impaired < track_name)
            "--track-name",
            "5:English SDH",
            "--default-track-flag",
            "6:1", // subtitle de forced
            "--track-name",
            "6:German forced",
            "--track-name",
            "7:German", // subtitle de plain
            "--hearing-impaired-flag",
            "8:1", // subtitle de SDH
            "--track-name",
            "8:German SDH",
            "(",
            &primary_disp,
            ")",
            // --- donor group: external Turkish subtitle ---
            "--no-global-tags",
            "--no-attachments", // donor attachments always dropped (D10)
            "--no-video",
            "--no-audio",
            "--subtitle-tracks",
            "0",
            "--no-buttons",
            "--language",
            "0:tr", // changes (language < track_name)
            "--track-name",
            "0:Türkçe",
            "(",
            &donor_disp,
            ")",
            "--track-order",
            "0:0,0:1,0:2,0:3,0:4,0:5,0:6,0:7,0:8,1:0",
        ]
    );
}

// ---------------------------------------------------------------------------
// Live acceptance: real mkvmerge, gated + self-skipping (spec 10).
// ---------------------------------------------------------------------------

fn mkvmerge() -> Option<Mkvmerge> {
    Mkvmerge::locate().ok()
}

const LIVE_PROFILE: &str = r#"
profile_version: 1
input: { pattern: 'source', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles } }
"#;

#[test]
fn live_mkvmerge_accepts_planned_command() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };

    let dir = tempfile::tempdir().unwrap();
    let srt = dir.path().join("seed.srt");
    std::fs::write(&srt, "1\n00:00:00,000 --> 00:00:01,000\nHello\n").unwrap();

    // A minimal one-track MKV: an SRT needs no media libs to mux (task
    // brief), unlike audio/video which would need a real codec.
    let source = dir.path().join("source.mkv");
    let status = Command::new(m.path())
        .args(["-q", "-o"])
        .arg(&source)
        .arg(&srt)
        .status()
        .expect("spawn mkvmerge to build the fixture source");
    assert!(status.success(), "mkvmerge failed to build the source");

    let out_dir = dir.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let profile = from_str(LIVE_PROFILE, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(out_dir),
        on_collision: None,
    };
    let lang = m.list_languages().expect("list-languages");
    let mut identify = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &m,
    };

    let batch = plan_batch(&profile, &run, &mut identify, &lang);
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 1);
    assert_eq!(plan.assignments[0].track_id, Some(0));

    let argv = command(plan);
    let status = Command::new(m.path())
        .args(&argv)
        .status()
        .expect("spawn mkvmerge on the planned command");
    assert!(
        status.success(),
        "mkvmerge rejected the planned argv: {argv:?}"
    );
    assert!(plan.output.exists(), "output file was not created");

    let out_json = m
        .identify_json(&plan.output)
        .expect("re-identify the muxed output");
    let out_id = Identification::from_json(&out_json).expect("parse re-identification JSON");
    assert!(out_id.is_identifiable());
    assert_eq!(out_id.tracks.len(), 1);
    assert_eq!(out_id.tracks[0].kind, "subtitles");
}

// ---------------------------------------------------------------------------
// Live acceptance: the resolved D20 track order (Plan 3.5 Task 7, SI-3).
// Confirmed against real mkvmerge v100 (see this test's construction), not
// taken from memory: under `keep_unmatched`, `--track-order` lists every
// primary track first, in source order, before any donor track. This
// replaces the Task 3 version of this test, which locked the superseded
// matched-first/unmatched-appended order the whole-branch review flagged as
// producing a donor-FIRST result on exactly the additive use case `keep`
// exists for.
// ---------------------------------------------------------------------------

/// Looks up a track's `track_name`, panicking if absent (every track in this
/// test's fixture is named, so a miss means the fixture or the mux step
/// broke, not a legitimate "no name" case).
fn track_name(t: &Track) -> String {
    match t.get("track_name") {
        Some(PropValue::Str(s)) => s,
        other => panic!("expected a track_name property, got {other:?}"),
    }
}

#[test]
fn live_keep_donor_trails_primary() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };

    let dir = tempfile::tempdir().unwrap();
    let pa_srt = dir.path().join("pa.srt");
    let pb_srt = dir.path().join("pb.srt");
    let donor_srt = dir.path().join("donor.srt");
    std::fs::write(&pa_srt, "1\n00:00:00,000 --> 00:00:01,000\nPA\n").unwrap();
    std::fs::write(&pb_srt, "1\n00:00:00,000 --> 00:00:01,000\nPB\n").unwrap();
    std::fs::write(&donor_srt, "1\n00:00:00,000 --> 00:00:01,000\nDONOR\n").unwrap();

    // Primary: a real 2-track source (task brief: >= 2 tracks), built the
    // same way as the sibling golden fixture -- two single-track SRTs
    // merged as separate input files, sequential track ids in argument
    // order (confirmed below, not assumed).
    let primary = dir.path().join("primary.mkv");
    let status = Command::new(m.path())
        .args(["-q", "-o"])
        .arg(&primary)
        .args(["--track-name", "0:PA"])
        .arg(&pa_srt)
        .args(["--track-name", "0:PB"])
        .arg(&pb_srt)
        .status()
        .expect("spawn mkvmerge to build the primary fixture");
    assert!(status.success(), "mkvmerge failed to build the primary");

    // Donor: a real, separate external file with one subtitle track -- the
    // additive case D20 exists for ("add a German sub, keep the rest").
    let donor = dir.path().join("donor.mkv");
    let status = Command::new(m.path())
        .args(["-q", "-o"])
        .arg(&donor)
        .args(["--track-name", "0:DONOR"])
        .arg(&donor_srt)
        .status()
        .expect("spawn mkvmerge to build the donor fixture");
    assert!(status.success(), "mkvmerge failed to build the donor");

    let primary_json = m
        .identify_json(&primary)
        .expect("identify the primary fixture");
    let primary_id =
        Identification::from_json(&primary_json).expect("parse primary identification JSON");
    let mut primary_tracks = primary_id.tracks;
    primary_tracks.sort_by_key(|t| t.id);
    assert_eq!(
        primary_tracks.len(),
        2,
        "primary fixture must carry exactly 2 tracks, got {primary_tracks:?}"
    );
    assert_eq!(
        track_name(&primary_tracks[0]),
        "PA",
        "fixture sanity: primary id 0 is PA"
    );
    assert_eq!(
        track_name(&primary_tracks[1]),
        "PB",
        "fixture sanity: primary id 1 is PB"
    );

    let donor_json = m.identify_json(&donor).expect("identify the donor fixture");
    let donor_id = Identification::from_json(&donor_json).expect("parse donor identification JSON");
    assert_eq!(
        donor_id.tracks.len(),
        1,
        "donor fixture must carry exactly 1 track, got {:?}",
        donor_id.tracks
    );
    let donor_track = &donor_id.tracks[0];
    assert_eq!(
        track_name(donor_track),
        "DONOR",
        "fixture sanity: donor id 0 is DONOR"
    );

    let out_dir = dir.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();
    let output = out_dir.join("out.mkv");

    // `keep_unmatched: true`, a single donor rule adding the external
    // subtitle; neither primary track has an assignment (both pass through
    // as kept-unmatched). D20 says the primary still leads `--track-order`
    // (both PA and PB, source order) and the donor trails.
    let plan = Plan {
        source: primary.clone(),
        output: output.clone(),
        keep_unmatched: true,
        primary_track_ids: primary_tracks.iter().map(|t| t.id).collect(),
        assignments: vec![Assignment {
            rule_index: 0,
            source: donor.clone(),
            track_id: Some(donor_track.id),
            track_kind: Some(donor_track.kind.clone()),
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

    let argv = command(&plan);
    let status = Command::new(m.path())
        .args(&argv)
        .status()
        .expect("spawn mkvmerge on the planned command");
    assert!(
        status.success(),
        "mkvmerge rejected the planned argv: {argv:?}"
    );
    assert!(output.exists(), "output file was not created");

    let out_json = m
        .identify_json(&output)
        .expect("re-identify the muxed output");
    let out_id = Identification::from_json(&out_json).expect("parse re-identification JSON");
    let mut out_tracks = out_id.tracks;
    out_tracks.sort_by_key(|t| t.id);

    // (a) all three tracks survive: both kept-unmatched primary tracks plus
    // the donor's.
    assert_eq!(
        out_tracks.len(),
        3,
        "expected primary(2) + donor(1) tracks, got {out_tracks:?}"
    );

    // (b) D20, observed against real mkvmerge (not assumed, SI-3): the
    // primary's tracks lead in source order, the donor trails.
    let out_names: Vec<String> = out_tracks.iter().map(track_name).collect();
    assert_eq!(
        out_names,
        vec!["PA".to_string(), "PB".to_string(), "DONOR".to_string()],
        "donor must trail every primary track (D20)"
    );
}

// ---------------------------------------------------------------------------
// Live acceptance: attachment + `changes` round trip (Plan 4 Task 7, D18).
// Converts Plan 3's one-off manual mkvmerge-v100 attachment/changes
// validation into a standing guard. Probed against real mkvmerge v100 before
// writing this test (SI-3, not assumed): `--attach-file` with an explicit
// `--attachment-mime-type text/plain` accepts a plain `.txt` file (exit 0,
// re-`-J` reports `content_type: "text/plain"`) -- identical to a bare
// `--attach-file` with no mime-type flag, since mkvmerge already guesses
// `text/plain` from the `.txt` extension. The explicit flag is kept in the
// fixture build because it is what the task specifies, not because it
// changes observable behavior.
// ---------------------------------------------------------------------------

const ATTACHMENT_PROFILE: &str = r#"
profile_version: 1
input: { pattern: 'source', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles } }
      changes: { track_name: Renamed, default_track: true }
"#;

#[test]
fn live_attachment_and_changes_round_trip() {
    let Some(m) = mkvmerge() else {
        eprintln!("mkvmerge not found; skipping");
        return;
    };

    let dir = tempfile::tempdir().unwrap();
    let srt = dir.path().join("seed.srt");
    std::fs::write(&srt, "1\n00:00:00,000 --> 00:00:01,000\nHello\n").unwrap();
    let note = dir.path().join("note.txt");
    std::fs::write(&note, "attachment payload\n").unwrap();

    // The primary: one subtitle track plus one attached text file, built
    // directly with real mkvmerge (SI-3: probed first, see the block comment
    // above).
    let source = dir.path().join("source.mkv");
    let status = Command::new(m.path())
        .args(["-q", "-o"])
        .arg(&source)
        .args(["--attachment-mime-type", "text/plain", "--attach-file"])
        .arg(&note)
        .arg(&srt)
        .status()
        .expect("spawn mkvmerge to build the fixture source");
    assert!(status.success(), "mkvmerge failed to build the source");

    let out_dir = dir.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let profile = from_str(ATTACHMENT_PROFILE, Format::Yaml).unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(out_dir),
        on_collision: None,
    };
    let lang = m.list_languages().expect("list-languages");
    let mut identify = LiveIdentifier {
        cache: IdentifyCache::new(),
        mkv: &m,
    };

    let batch = plan_batch(&profile, &run, &mut identify, &lang);
    assert_eq!(batch.files.len(), 1);
    let fr = &batch.files[0];
    assert!(fr.plan.is_some(), "diags: {:?}", fr.diagnostics);
    let plan = fr.plan.as_ref().unwrap();
    assert_eq!(plan.assignments.len(), 1);
    assert_eq!(plan.assignments[0].track_id, Some(0));

    let argv = command(plan);
    let status = Command::new(m.path())
        .args(&argv)
        .status()
        .expect("spawn mkvmerge on the planned command");
    assert!(
        status.success(),
        "mkvmerge rejected the planned argv: {argv:?}"
    );
    assert!(plan.output.exists(), "output file was not created");

    let out_json = m
        .identify_json(&plan.output)
        .expect("re-identify the muxed output");
    let out_id = Identification::from_json(&out_json).expect("parse re-identification JSON");
    assert!(out_id.is_identifiable());

    // The `changes` round trip: the subtitle track carries the renamed
    // `track_name` and the newly-set `default_track` flag.
    assert_eq!(out_id.tracks.len(), 1);
    let track = &out_id.tracks[0];
    assert_eq!(track.kind, "subtitles");
    assert_eq!(track_name(track), "Renamed");
    assert_eq!(track.get("default_track"), Some(PropValue::Bool(true)));

    // The attachment round trip: `PrimaryAttachments::KeepAll` (default
    // `attachments.unmatched: keep`, no rules) carries the original
    // attachment through untouched.
    assert_eq!(
        out_id.attachments.len(),
        1,
        "expected the primary's attachment to survive: {:?}",
        out_id.attachments
    );
    assert_eq!(out_id.attachments[0].file_name, "note.txt");
}
