//! Spec 10: every `DiagCode` must have a message template in the English
//! catalog, and every message must render without leaking an unresolved
//! `{$param}` placeholder (the param names an emitter actually sets must
//! match the template's placeholders -- drift here once reached a user as
//! a literal `{$property}` in output). A second guard enumerates every key
//! in `locales/en/cli.ftl` and requires each to be either a `DiagCode`
//! message or on the explicit allowlist of directly-rendered CLI strings
//! below, so a new catalog entry can never go silently unwired.

use std::collections::BTreeSet;

use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use fluent_syntax::ast::Entry;
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
/// guard stays green (known case found in review: `planner.rs:600` emits
/// `InvalidPropertyValue` without `allowed`, caught separately).
fn fixture_args(code: DiagCode) -> FluentArgs<'static> {
    let mut args = FluentArgs::new();
    match code {
        DiagCode::UnsupportedProfileVersion => {
            args.set("found", "2");
            args.set("supported", "1");
        }
        DiagCode::ParseError => {
            args.set("detail", "unexpected end of input");
            args.set("at", "tracks[0].match");
        }
        DiagCode::NoTrackRules => {}
        DiagCode::EmptyMatchExpression => {}
        DiagCode::EmptyExtensions => {}
        DiagCode::InvalidRegex => {
            args.set("detail", "unclosed group");
        }
        DiagCode::UnknownProperty => {
            args.set("property", "bogus_property");
        }
        DiagCode::CodecKindExactOnly => {
            args.set("condition", "substring");
        }
        DiagCode::InvalidPropertyValue => {
            args.set("property", "language");
            args.set("value", "xx-not-a-code");
            args.set("allowed", "a valid ISO 639/BCP-47 language code");
        }
        DiagCode::EmptyMatchList => {}
        DiagCode::NotStringProperty => {
            args.set("property", "channels");
            args.set("actual_type", "integer");
            args.set("condition", "substring");
        }
        DiagCode::ValueTypeMismatch => {
            args.set("property", "default");
            args.set("found", "string");
            args.set("expected", "boolean");
        }
        DiagCode::UnknownSettableProperty => {
            args.set("property", "bogus_settable");
        }
        DiagCode::InvalidKeyword => {
            args.set("found", "bogus");
            args.set("allowed", "primary");
        }
        DiagCode::LocatorConflict => {}
        DiagCode::InvalidTemplate => {
            args.set("kind", "unclosed-brace");
            args.set("pos", "12");
        }
        DiagCode::UnknownTemplateField => {
            args.set("field", "bogus_field");
            args.set("allowed", "match, g1, source_stem");
        }
        DiagCode::UnknownTemplateFilter => {
            args.set("name", "bogus_filter");
        }
        DiagCode::PathSeparatorInTemplate => {}
        DiagCode::AttachmentRuleShape => {
            args.set("found", "0");
        }
        DiagCode::ProvableOverlap => {
            args.set("rule_a", "0");
            args.set("rule_b", "1");
        }
        DiagCode::AmbiguousRule => {
            args.set("count", "2");
        }
        DiagCode::OverlappingRules => {
            args.set("rule_a", "tracks[0]");
            args.set("rule_b", "tracks[1]");
            args.set("track", "3");
        }
        DiagCode::MissingTrack => {}
        DiagCode::MissingExternal => {}
        DiagCode::AmbiguousExternal => {
            args.set("count", "2");
        }
        DiagCode::UnidentifiableSource => {
            args.set("detail", "mkvmerge exited with status 2");
        }
        DiagCode::UnsupportedSource => {}
        DiagCode::EmptyPlan => {}
        DiagCode::OutputCollision => {
            args.set("path", "/out/movie.mkv");
        }
        DiagCode::PathSeparatorInRenderedName => {
            args.set("name", "sub/dir");
        }
        DiagCode::EmptyRenderedName => {
            args.set("name", "");
        }
        DiagCode::SourceOverwrite => {
            args.set("path", "/in/movie.mkv");
        }
        DiagCode::DuplicateIdentifier => {
            args.set("identifier", "movie");
            args.set("file_a", "/in/movie.1080p.mkv");
            args.set("file_b", "/in/movie.720p.mkv");
        }
        DiagCode::DonorIsPrimary => {
            args.set("donor", "/in/movie.donor.mkv");
        }
        DiagCode::IgnoredFile => {}
        DiagCode::MultipleIdentifierMatches => {
            args.set("name", "movie.1080p.1080p.mkv");
        }
        DiagCode::UnknownPropertySkew => {
            args.set("version", "42");
        }
        DiagCode::UnknownExtension => {
            args.set("extension", "avi");
            args.set("known", "mkv, mp4, ac3");
        }
        DiagCode::SuggestionsCapped => {
            args.set("dropped", "2");
        }
        // `suggestion-partition`'s message selects on `kind` (planner.rs
        // `partition_for_rule`): the `*[group]` default branch (kind other
        // than "overflow") uses count/fix/files, the `[overflow]` branch
        // uses only `dropped`. This guard renders one fixture per code, so
        // only the default `group` branch is exercised here; the overflow
        // branch's `dropped` param is not proven to match the template by
        // this test (same single-fixture-per-code limitation the doc
        // comment above already calls out for `InvalidPropertyValue`).
        DiagCode::SuggestionPartition => {
            args.set("kind", "group");
            args.set("count", "2");
            args.set("fix", "tracks[0].match.exact.codec_id: A_AC3");
            args.set("files", "/in/a.mkv, /in/b.mkv");
        }
        DiagCode::WorkerPanicked => {}
    }
    args
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
    "dry-run-summary",
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
];

/// Fixture params for each [`ALLOWLISTED_CLI_KEYS`] entry, sourced from its
/// `renderer.msg(...)`/`msg_with_count(...)` call site in
/// `src/commands/{identify,validate,mod,run}.rs` and `src/i18n.rs`'s own
/// `diagnostic()` method. Exhaustive-by-panic (not by match-arm compile
/// error, since `&str` cannot be matched exhaustively): an allowlist entry
/// with no arm here panics loudly instead of silently rendering with an
/// empty arg list.
fn allowlisted_cli_key_args(key: &str) -> FluentArgs<'static> {
    let mut args = FluentArgs::new();
    match key {
        "validate-ok"
        | "mkvmerge-not-found"
        | "mkvmerge-query-failed"
        | "run-joblog-unavailable" => {}
        "validate-summary" => {
            args.set("errors", "1");
            args.set("warnings", "2");
            args.set("infos", "0");
        }
        "diagnostic-line" => {
            args.set("severity", "error");
            args.set("config_path", "tracks[0].match");
            args.set("message", "example message");
        }
        "diagnostic-line-file" => {
            args.set("severity", "warning");
            args.set("file", "/in/movie.mkv");
            args.set("config_path", "tracks[0].match");
            args.set("message", "example message");
        }
        "identify-failed" | "identify-not-media" => {
            args.set("file", "/in/movie.mkv");
        }
        "identify-track-line" => {
            args.set("id", "0");
            args.set("type", "audio");
            args.set("codec", "AC-3");
            args.set("language", "eng");
        }
        "dry-run-file" => {
            args.set("file", "/in/movie.mkv");
            args.set("id", "movie");
        }
        "dry-run-assignment" => {
            args.set("rule", "0");
            args.set("track", "1");
        }
        "dry-run-output" => {
            args.set("path", "/out/movie.mkv");
        }
        "dry-run-suggestion" => {
            args.set("config_path", "tracks[0].match");
        }
        "dry-run-summary" => {
            args.set("count", "3");
            args.set("root", "/in");
            args.set("extensions", "mkv, mp4");
        }
        "run-job-start" | "run-job-cancelled" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
        }
        "run-job-progress" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
            args.set("percent", "42");
        }
        "run-job-notice" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
            args.set("text", "muxing");
        }
        "run-job-ok" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
            args.set("seconds", "12.3");
        }
        "run-job-warning" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
            args.set("count", "2");
            args.set("seconds", "12.3");
        }
        "run-job-failed" => {
            args.set("index", "1");
            args.set("total", "3");
            args.set("output", "/out/movie.mkv");
            args.set("code", "1");
        }
        "run-summary" => {
            args.set("ok", "2");
            args.set("warning", "1");
            args.set("failed", "0");
            args.set("cancelled", "0");
        }
        "run-joblog-written" | "run-joblog-incomplete" => {
            args.set("dir", "/data/logs");
        }
        other => panic!("allowlisted_cli_key_args: no fixture for {other:?}"),
    }
    args
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
    entries: impl Iterator<Item = (&'a str, FluentArgs<'a>)>,
    renderer: &muxsmith_cli::i18n::Renderer,
) -> Vec<String> {
    entries
        .filter_map(|(id, args)| {
            let pairs = string_pairs(&args);
            let rendered = renderer.msg(id, &pairs);
            rendered.contains("{$").then(|| format!("{id}: {rendered}"))
        })
        .collect()
}

/// Converts a [`FluentArgs`] built entirely from string values (true of
/// every fixture in this file) into the `&[(&str, &str)]` shape
/// [`muxsmith_cli::i18n::Renderer::msg`] takes.
fn string_pairs<'a>(args: &'a FluentArgs<'a>) -> Vec<(&'a str, &'a str)> {
    args.iter()
        .map(|(k, v)| match v {
            FluentValue::String(s) => (k, s.as_ref()),
            other => panic!("fixture value for {k:?} is not a string: {other:?}"),
        })
        .collect()
}
