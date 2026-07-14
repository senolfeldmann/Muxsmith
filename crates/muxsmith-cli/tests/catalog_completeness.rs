//! Spec 10: every `DiagCode` must have a message template in the English
//! catalog, and every message must render without leaking an unresolved
//! `{$param}` placeholder (the param names an emitter actually sets must
//! match the template's placeholders -- drift here once reached a user as
//! a literal `{$property}` in output). A second guard enumerates every key
//! in `locales/en/cli.ftl` and requires each to be either a `DiagCode`
//! message or on the explicit allowlist of directly-rendered CLI strings
//! below, so a new catalog entry can never go silently unwired.

use std::collections::BTreeSet;
use std::path::Path;

use fluent_bundle::FluentResource;
use fluent_syntax::ast::Entry;
use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};
use muxsmith_core::planner::{RunInputs, plan_batch};
use muxsmith_core::profile::load::{Format, from_str};
use muxsmith_core::report::DiagCode;

#[test]
fn every_diag_code_has_a_catalog_message() {
    let renderer = muxsmith_cli::i18n::Renderer::new(Some("en"));
    let missing: Vec<&str> = DiagCode::ALL
        .iter()
        .filter(|code| renderer.msg(code.key(), &[]) == code.key())
        .map(|code| code.key())
        .collect();
    assert_eq!(missing, Vec::<&str>::new(), "missing catalog entries");
}

/// One fixture value per param an emitter actually sets for `code`, sourced
/// by grepping every `Diagnostic::error`/`warning`/`info` + `.with(...)`
/// call site across `muxsmith-core` (`planner.rs`, `discovery.rs`,
/// `profile/{validate,load,lint}.rs`). Exhaustive match: a new `DiagCode`
/// variant fails to compile here until its fixture is added, so the guard
/// grows with the enum instead of silently missing new codes.
///
/// **Limitation:** this guard is exhaustive over `DiagCode` variants, not
/// over emitter sites per variant. It renders one fixture per code, which
/// proves the fixture matches the template; a single emitter site that omits
/// a param its siblings set will leak `{$param}` in production while this
/// guard stays green. `InvalidPropertyValue` now selects on `$property` in
/// the catalog (D39): this fixture exercises the `*[other]` list arm (a
/// closed-domain property with a real `allowed` list), while the
/// `[language]` arm -- whose emitters carry no `allowed` param -- is pinned
/// separately by
/// [`invalid_changes_language_diagnostic_renders_without_placeholder_leak`],
/// which renders the real emitter-site diagnostic rather than a fixture.
fn fixture_args(code: DiagCode) -> Vec<(&'static str, &'static str)> {
    match code {
        DiagCode::UnsupportedProfileVersion => vec![("found", "2"), ("supported", "1")],
        DiagCode::ParseError => vec![
            ("detail", "unexpected end of input"),
            ("at", "tracks[0].match"),
        ],
        DiagCode::NoTrackRules => vec![],
        DiagCode::EmptyMatchExpression => vec![],
        DiagCode::EmptyExtensions => vec![],
        DiagCode::InvalidRegex => vec![("detail", "unclosed group")],
        DiagCode::UnknownProperty => vec![("property", "bogus_property")],
        DiagCode::RawProperty => vec![("property", "dolby_complexity_index")],
        DiagCode::RawOnKnownProperty => vec![("property", "language")],
        DiagCode::CodecKindExactOnly => vec![("condition", "substring")],
        DiagCode::InvalidPropertyValue => vec![
            ("property", "type"),
            ("value", "text"),
            ("allowed", "video, audio, subtitles"),
        ],
        DiagCode::EmptyMatchList => vec![],
        DiagCode::NotStringProperty => vec![
            ("property", "channels"),
            ("actual_type", "integer"),
            ("condition", "substring"),
        ],
        DiagCode::ValueTypeMismatch => vec![
            ("property", "default"),
            ("found", "string"),
            ("expected", "boolean"),
        ],
        DiagCode::UnknownSettableProperty => vec![("property", "bogus_settable")],
        DiagCode::InvalidKeyword => vec![("found", "bogus"), ("allowed", "primary")],
        DiagCode::LocatorConflict => vec![],
        DiagCode::InvalidTemplate => vec![("kind", "unclosed-brace"), ("pos", "12")],
        DiagCode::UnknownTemplateField => vec![
            ("field", "bogus_field"),
            ("allowed", "match, g1, source_stem"),
        ],
        DiagCode::UnknownTemplateFilter => vec![("name", "bogus_filter")],
        DiagCode::PathSeparatorInTemplate => vec![],
        DiagCode::AttachmentRuleShape => vec![("found", "0")],
        DiagCode::ProvableOverlap => vec![("rule_a", "tracks[0]"), ("rule_b", "tracks[1]")],
        DiagCode::AmbiguousRule => vec![("count", "2")],
        DiagCode::OverlappingRules => {
            vec![("rules", "tracks[0], tracks[1], tracks[2]"), ("track", "3")]
        }
        DiagCode::MissingTrack => vec![],
        DiagCode::MissingExternal => vec![],
        DiagCode::AmbiguousExternal => vec![("count", "2")],
        DiagCode::UnidentifiableSource => vec![("detail", "mkvmerge exited with status 2")],
        // `unsupported-source`'s message selects on `kind` (planner.rs
        // `resolve_file`): the `*[primary]` default branch (kind other than
        // "donor") needs no further params, the `[donor]` branch also
        // needs `donor`. Only the donor branch has a placeholder to leak,
        // so it is the one exercised here (same single-fixture-per-code
        // limitation the doc comment above calls out for
        // `InvalidPropertyValue`).
        DiagCode::UnsupportedSource => vec![("kind", "donor"), ("donor", "/in/movie.donor.srt")],
        DiagCode::EmptyPlan => vec![],
        DiagCode::OutputCollision => vec![("path", "/out/movie.mkv")],
        DiagCode::PathSeparatorInRenderedName => vec![("name", "sub/dir")],
        DiagCode::EmptyRenderedName => vec![("name", "")],
        DiagCode::SourceOverwrite => vec![("path", "/in/movie.mkv")],
        // `non-utf8-path`'s message selects on `role` (planner.rs
        // `detect_non_utf8_paths`; values: output/chapters/attachment/
        // primary/donor, all sharing the one `{ $path }` tail). `path`
        // carries the lossy `display()` rendering, hence the U+FFFD. The
        // non-default `donor` branch is exercised here (same
        // single-fixture-per-code limitation as `UnsupportedSource`).
        DiagCode::NonUtf8Path => vec![("role", "donor"), ("path", "/in/b\u{FFFD}d/movie.srt")],
        DiagCode::DuplicateIdentifier => vec![
            ("identifier", "movie"),
            ("file_a", "/in/movie.1080p.mkv"),
            ("file_b", "/in/movie.720p.mkv"),
        ],
        DiagCode::DonorIsPrimary => vec![("donor", "/in/movie.donor.mkv")],
        DiagCode::IgnoredFile => vec![],
        DiagCode::MultipleIdentifierMatches => vec![("name", "movie.1080p.1080p.mkv")],
        DiagCode::UnknownPropertySkew => vec![
            ("property", "new_prop"),
            ("found_version", "21"),
            ("pinned", "20"),
        ],
        DiagCode::SchemaDrift => vec![("found_version", "21"), ("pinned", "20")],
        DiagCode::UnknownExtension => vec![("extension", "avi"), ("known", "mkv, mp4, ac3")],
        DiagCode::SuggestionsCapped => vec![("dropped", "2")],
        // `suggestion-partition`'s message selects on `kind` (planner.rs
        // `partition_for_rule`): the `*[group]` default branch (kind other
        // than "overflow") uses count/fix/files, the `[overflow]` branch
        // uses only `dropped`. This guard renders one fixture per code, so
        // only the default `group` branch is exercised here; the overflow
        // branch's `dropped` param is not proven to match the template by
        // this test (same single-fixture-per-code limitation the doc
        // comment above already calls out for `InvalidPropertyValue`).
        DiagCode::SuggestionPartition => vec![
            ("kind", "group"),
            ("count", "2"),
            ("fix", "tracks[0].match.exact.codec_id: A_AC3"),
            ("files", "/in/a.mkv, /in/b.mkv"),
        ],
        DiagCode::WorkerPanicked => vec![],
    }
}

/// Every DiagCode message, rendered with the params its emitter actually
/// sets ([`fixture_args`]), must not leak a raw `{$name}` placeholder:
/// that would mean the message template and the emitter's `.with(...)`
/// calls have drifted apart (params renamed on one side, not the other).
/// This is what previously reached a user as a literal `{$property}` in
/// output.
#[test]
fn every_diag_code_renders_without_leftover_placeholders() {
    let renderer = muxsmith_cli::i18n::Renderer::new(Some("en"));
    let leaked = render_and_find_leaks(
        DiagCode::ALL.iter().map(|&c| (c.key(), fixture_args(c))),
        &renderer,
    );
    assert!(
        leaked.is_empty(),
        "DiagCode message(s) with an unresolved placeholder:\n{}",
        leaked.join("\n")
    );
}

/// Directly-rendered `cli.ftl` keys that are not `DiagCode` messages (they
/// have no corresponding `DiagCode` variant, e.g. the `run-*` progress
/// lines and the `validate`/`dry-run`/`identify` subcommand strings).
/// Every entry here must also exist in `cli.ftl` and gets its own fixture
/// in [`allowlisted_cli_key_args`], rendered by the same leak check as the
/// `DiagCode` table above.
const ALLOWLISTED_CLI_KEYS: &[&str] = &[
    "validate-ok",
    "validate-summary",
    "diagnostic-line",
    "diagnostic-line-file",
    "mkvmerge-not-found",
    "mkvmerge-query-failed",
    "identify-failed",
    "identify-not-media",
    "identify-track-line",
    "dry-run-file",
    "dry-run-assignment",
    "dry-run-output",
    "dry-run-suggestion",
    "batch-summary",
    "run-job-start",
    "run-job-progress",
    "run-job-notice",
    "run-job-ok",
    "run-job-warning",
    "run-job-failed",
    "run-job-cancelled",
    "run-summary",
    "run-joblog-unavailable",
    "run-joblog-written",
    "run-joblog-incomplete",
    "run-signal-handler-unavailable",
];

/// Fixture params for each [`ALLOWLISTED_CLI_KEYS`] entry, sourced from its
/// `renderer.msg(...)`/`msg_with_count(...)` call site in
/// `src/commands/{identify,validate,mod,run}.rs` and `src/i18n.rs`'s own
/// `diagnostic()` method. Exhaustive-by-panic (not by match-arm compile
/// error, since `&str` cannot be matched exhaustively): an allowlist entry
/// with no arm here panics loudly instead of silently rendering with an
/// empty arg list.
fn allowlisted_cli_key_args(key: &str) -> Vec<(&'static str, &'static str)> {
    match key {
        "validate-ok"
        | "mkvmerge-not-found"
        | "mkvmerge-query-failed"
        | "run-joblog-unavailable"
        | "run-signal-handler-unavailable" => vec![],
        "validate-summary" => vec![("errors", "1"), ("warnings", "2"), ("infos", "0")],
        "diagnostic-line" => vec![
            ("severity", "error"),
            ("config_path", "tracks[0].match"),
            ("message", "example message"),
        ],
        "diagnostic-line-file" => vec![
            ("severity", "warning"),
            ("file", "/in/movie.mkv"),
            ("config_path", "tracks[0].match"),
            ("message", "example message"),
        ],
        "identify-failed" | "identify-not-media" => vec![("file", "/in/movie.mkv")],
        "identify-track-line" => vec![
            ("id", "0"),
            ("type", "audio"),
            ("codec", "AC-3"),
            ("language", "eng"),
        ],
        "dry-run-file" => vec![("file", "/in/movie.mkv"), ("id", "movie")],
        "dry-run-assignment" => vec![("rule", "0"), ("track", "1")],
        "dry-run-output" => vec![("path", "/out/movie.mkv")],
        "dry-run-suggestion" => vec![("config_path", "tracks[0].match")],
        "batch-summary" => vec![("count", "3"), ("root", "/in"), ("extensions", "mkv, mp4")],
        "run-job-start" | "run-job-cancelled" => {
            vec![("index", "1"), ("total", "3"), ("output", "/out/movie.mkv")]
        }
        "run-job-progress" => vec![
            ("index", "1"),
            ("total", "3"),
            ("output", "/out/movie.mkv"),
            ("percent", "42"),
        ],
        "run-job-notice" => vec![
            ("index", "1"),
            ("total", "3"),
            ("output", "/out/movie.mkv"),
            ("text", "muxing"),
        ],
        "run-job-ok" => vec![
            ("index", "1"),
            ("total", "3"),
            ("output", "/out/movie.mkv"),
            ("seconds", "12.3"),
        ],
        "run-job-warning" => vec![
            ("index", "1"),
            ("total", "3"),
            ("output", "/out/movie.mkv"),
            ("count", "2"),
            ("seconds", "12.3"),
        ],
        "run-job-failed" => vec![
            ("index", "1"),
            ("total", "3"),
            ("output", "/out/movie.mkv"),
            ("code", "1"),
        ],
        "run-summary" => vec![
            ("ok", "2"),
            ("warning", "1"),
            ("failed", "0"),
            ("cancelled", "0"),
        ],
        "run-joblog-written" | "run-joblog-incomplete" => vec![("dir", "/data/logs")],
        other => panic!("allowlisted_cli_key_args: no fixture for {other:?}"),
    }
}

/// Enumerates every message key in `locales/en/cli.ftl` (real Fluent
/// parsing, not a hand-rolled regex, so multi-line selector values like
/// `run-job-warning`'s plural branches are handled correctly) and asserts
/// each key is either a `DiagCode` message or on [`ALLOWLISTED_CLI_KEYS`].
/// A key that is neither fails the test by name, so a catalog entry can
/// never go unwired to either mechanism (spec 10).
#[test]
fn every_cli_ftl_key_is_a_diag_code_or_allowlisted() {
    let diag_keys: BTreeSet<&str> = DiagCode::ALL.iter().map(|c| c.key()).collect();
    let allowlist: BTreeSet<&str> = ALLOWLISTED_CLI_KEYS.iter().copied().collect();
    assert_eq!(
        allowlist.len(),
        ALLOWLISTED_CLI_KEYS.len(),
        "ALLOWLISTED_CLI_KEYS contains a duplicate"
    );

    let source = include_str!("../../../locales/en/cli.ftl");
    let resource =
        FluentResource::try_new(source.to_string()).expect("locales/en/cli.ftl must parse");

    let cli_keys: Vec<&str> = resource
        .entries()
        .filter_map(|entry| match entry {
            Entry::Message(message) => Some(message.id.name),
            _ => None,
        })
        .collect();

    let orphans: Vec<&str> = cli_keys
        .iter()
        .copied()
        .filter(|id| !diag_keys.contains(id) && !allowlist.contains(id))
        .collect();
    assert!(
        orphans.is_empty(),
        "cli.ftl key(s) wired to neither a DiagCode nor the allowlist: {orphans:?}"
    );

    // Catches a stale allowlist entry (renamed/removed key) the orphan
    // check above cannot see, since it only walks cli.ftl -> allowlist,
    // never the reverse direction.
    let present: BTreeSet<&str> = cli_keys.into_iter().collect();
    let stale: Vec<&str> = ALLOWLISTED_CLI_KEYS
        .iter()
        .copied()
        .filter(|k| !present.contains(k))
        .collect();
    assert!(
        stale.is_empty(),
        "allowlisted key(s) no longer present in cli.ftl: {stale:?}"
    );

    let renderer = muxsmith_cli::i18n::Renderer::new(Some("en"));
    let leaked = render_and_find_leaks(
        ALLOWLISTED_CLI_KEYS
            .iter()
            .map(|&k| (k, allowlisted_cli_key_args(k))),
        &renderer,
    );
    assert!(
        leaked.is_empty(),
        "allowlisted CLI message(s) with an unresolved placeholder:\n{}",
        leaked.join("\n")
    );
}

/// Renders each `(id, args)` pair via `renderer.msg` and collects
/// `"id: rendered"` for any output that still contains a raw `{$`
/// placeholder marker (Fluent's syntax for an unresolved reference).
fn render_and_find_leaks<'a>(
    entries: impl Iterator<Item = (&'a str, Vec<(&'a str, &'a str)>)>,
    renderer: &muxsmith_cli::i18n::Renderer,
) -> Vec<String> {
    entries
        .filter_map(|(id, args)| {
            let rendered = renderer.msg(id, &args);
            rendered.contains("{$").then(|| format!("{id}: {rendered}"))
        })
        .collect()
}

/// Returns the same `Identification` for any path: enough to drive
/// `plan_batch` past identification so a rule resolves and its `changes`
/// apply.
struct OneIdent(Identification);

impl Identify for OneIdent {
    fn identify(&mut self, _path: &Path) -> Result<Identification, IdentifyError> {
        Ok(self.0.clone())
    }

    fn known_extensions(&mut self) -> Option<Vec<String>> {
        None
    }
}

/// Regression guard for the emitter-site divergence the two guards above are
/// structurally blind to (they render one hand-written fixture per code, so a
/// single emitter that omits a param its siblings set stays green there). This
/// drives the REAL `resolve_changes` emitter -- the plan-time invalid
/// `changes.language` path -- through `plan_batch`, then renders the diagnostic
/// it actually produced. Since D39, both `resolve_changes` and its
/// `walk_exact_languages` sibling deliberately carry no `allowed` param for
/// property=language emissions; the catalog's `invalid-property-value`
/// selects on `$property` and its `[language]` arm renders complete
/// registry-membership wording without one. This test pins that arm from
/// the emitter's own params rather than a fixture: no `{ $allowed }`
/// placeholder leak, and the ISO 639/BCP-47 wording rendered in full.
#[test]
fn invalid_changes_language_diagnostic_renders_without_placeholder_leak() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Show.S01E01.mkv"), b"x").unwrap();
    let profile = from_str(
        "profile_version: 1\n\
         input: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\n\
         tracks:\n  rules:\n\
         \x20   - match: { exact: { type: audio, language: en } }\n\
         \x20     changes: { language: 'zz!' }\n",
        Format::Yaml,
    )
    .unwrap();
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let series = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/identify/series-s01e01.json"
    ));
    let lang = LanguageIndex::from_rows(&[
        ["English", "eng", "eng", "en"],
        ["German", "ger", "ger", "de"],
        ["Turkish", "tur", "tur", "tr"],
    ]);
    let mut ident = OneIdent(Identification::from_json(series).unwrap());
    let batch = plan_batch(&profile, &run, &mut ident, &lang);

    let diag = batch
        .files
        .iter()
        .flat_map(|f| &f.diagnostics)
        .find(|d| {
            d.code == DiagCode::InvalidPropertyValue && d.config_path.ends_with(".changes.language")
        })
        .unwrap_or_else(|| {
            panic!(
                "expected an InvalidPropertyValue from the changes.language emitter site; diags: {:?}",
                batch
                    .files
                    .iter()
                    .flat_map(|f| &f.diagnostics)
                    .collect::<Vec<_>>()
            )
        });

    let renderer = muxsmith_cli::i18n::Renderer::new(Some("en"));
    let rendered = renderer.diagnostic(diag);
    assert!(
        !rendered.contains("{$"),
        "the changes.language InvalidPropertyValue leaked a placeholder: {rendered}"
    );
    assert!(
        rendered.contains("must be a valid ISO 639 or BCP-47 language code"),
        "the changes.language InvalidPropertyValue did not render the [language] arm's registry wording: {rendered}"
    );
    assert!(
        !rendered.contains("Allowed values include"),
        "the changes.language InvalidPropertyValue rendered the *[other] arm's allowed-list wording instead of the [language] arm: {rendered}"
    );
}
