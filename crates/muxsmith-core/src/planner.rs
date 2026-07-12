//! Batch planning (spec 5): resolve every track rule against each primary
//! file's tracks (and located donors) under strict independent uniqueness,
//! render output paths, and collect diagnostics into a batch report. No
//! filesystem mutation and no mux invocations (dry-run, spec 5.5); the only
//! external work is identification, driven through the injected [`Identify`].

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION;
use crate::capability::runtime::LanguageIndex;
use crate::discovery::{self, PrimaryFile};
use crate::identify::{Attachment, Identification, Identify};
use crate::matcher;
use crate::profile::match_expr::{MatchExpr, Scalar};
use crate::profile::model::{
    ChaptersCfg, CollisionPolicy, FilenameCfg, KeepDrop, Profile, SourceCfg, TitleCfg, TrackRule,
};
use crate::report::{DiagCode, Diagnostic, Severity};
use crate::template::Template;

/// Run inputs, separable from the profile (spec 3): overrides for source and
/// output directories and the collision policy. `None` falls back to the
/// profile's stored value (and, for output, ultimately to the source dir).
#[derive(Debug, Clone, PartialEq)]
pub struct RunInputs {
    /// Directory scanned for primary files.
    pub source: PathBuf,
    /// Output directory; falls back to `profile.output.directory`, then to
    /// `source` (output beside the source, which surfaces `SourceOverwrite`
    /// for a keep-name `.mkv` source).
    pub output: Option<PathBuf>,
    /// Collision policy override; falls back to `profile.output.on_collision`.
    pub on_collision: Option<CollisionPolicy>,
}

/// One resolved rule-to-track assignment. `track_id` is `None` for a satisfied
/// `optional` rule that matched nothing (spec 5.1).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Assignment {
    /// Index into `profile.tracks.rules`.
    pub rule_index: usize,
    /// The source file the track comes from (the primary, or a donor).
    pub source: PathBuf,
    /// The resolved `-J` track id, or `None` for an unmatched optional rule.
    pub track_id: Option<u64>,
    /// The matched track's `-J` type (`video`/`audio`/`subtitles`/`buttons`),
    /// needed by `command` to pick `--audio-tracks` vs `--video-tracks` etc.
    /// `None` exactly when `track_id` is `None`.
    pub track_kind: Option<String>,
    /// Settable changes to apply to the resolved track; empty when the rule has
    /// no `changes` or matched nothing.
    pub changes: Vec<AppliedChange>,
}

/// A resolved settable change on an assignment (spec 4.4). Format-neutral: the
/// property name and value, not an mkvmerge flag; `command` maps the property
/// to its option via `capability::settable`.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AppliedChange {
    /// Settable property name (spec 4.4 table key), e.g. `language`,
    /// `track_name`.
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
/// files always get `--no-attachments` (D10), so this concerns the primary
/// only.
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

/// The fully resolved plan for one primary (spec 3). Present only when the file
/// has no error-severity diagnostic.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Plan {
    /// The primary source file.
    pub source: PathBuf,
    /// The rendered absolute output path.
    pub output: PathBuf,
    /// One entry per track rule, in profile order (also the output track order,
    /// spec 4.5).
    pub assignments: Vec<Assignment>,
    /// Resolved attachment disposition (spec 4.9).
    pub attachments: AttachmentPlan,
    /// Resolved chapters disposition (spec 4.9).
    pub chapters: ChapterSource,
    /// Resolved tag flags (spec 4.9).
    pub tags: TagFlags,
    /// Resolved output title (spec 4.9).
    pub title: TitleAction,
    /// When true (`tracks.unmatched: keep`), the primary's unmatched tracks
    /// pass through: `command` emits no track-selection flags for the primary
    /// group. Donor groups are unaffected. Default construction is `false`
    /// (drop).
    pub keep_unmatched: bool,
    /// The primary file's full track-id list, in source order (as `-J`
    /// reports them). Under `keep_unmatched`, `command` lists these first in
    /// `--track-order` (group 0), ahead of any donor track (D20): a
    /// kept-but-unmatched primary track counts as matched ("keep = match to
    /// what is already there"), and the primary leads, donors trail.
    /// Unused when `keep_unmatched` is `false`.
    pub primary_track_ids: Vec<u64>,
}

/// Per-file result: the plan (if any) and every diagnostic about the file.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FileReport {
    /// The primary source file.
    pub source: PathBuf,
    /// The identifier matched from its basename.
    pub identifier: String,
    /// The plan, or `None` if any diagnostic is error-severity (spec 5.1).
    pub plan: Option<Plan>,
    /// All diagnostics about this file (per-file and rule-level).
    pub diagnostics: Vec<Diagnostic>,
}

/// The closed grammar of suggestion edits (spec 5.3, D6); only ever narrows a
/// rule. Populated by the engine (see `suggest`).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StructuredEdit {
    /// Add `property: value` to the rule's `exact` map.
    AddExact {
        /// The matchable property to constrain.
        property: String,
        /// The value to require.
        value: String,
    },
    /// Add `{ exact: { property: value } }` to the rule's `not` list.
    AddNotExact {
        /// The matchable property to exclude on.
        property: String,
        /// The value to exclude.
        value: String,
    },
    /// Add `track_name: value` to the rule's `substring` map.
    AddSubstring {
        /// The `track_name` substring to require.
        value: String,
    },
    /// Add `{ substring: { track_name: value } }` to the rule's `not` list.
    AddNotSubstring {
        /// The `track_name` substring to exclude.
        value: String,
    },
}

/// A structured, batch-validated suggested edit (spec 5.3, D6).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Suggestion {
    /// The diagnostic code this refinement resolves.
    pub resolves: DiagCode,
    /// Config path of the rule being edited.
    pub config_path: String,
    /// The structured edit.
    pub edit: StructuredEdit,
    /// The exact YAML fragment the CLI prints / the GUI applies.
    pub yaml_fragment: String,
}

/// The whole batch report (spec 3, 5): per-file results, batch-level
/// diagnostics (config/runtime/cross-file), and suggestions.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Batch {
    /// One entry per discovered primary file.
    pub files: Vec<FileReport>,
    /// Diagnostics not tied to a single primary: runtime config checks (bad
    /// `language` value) and cross-file facts (`DuplicateIdentifier`).
    pub batch_diagnostics: Vec<Diagnostic>,
    /// Batch-validated suggested refinements.
    pub suggestions: Vec<Suggestion>,
}

/// Resolution-only planning (no suggestions), over an already-discovered
/// primaries list. The engine re-invokes this on edited profiles to simulate
/// candidate suggestions against the cached identification (spec 5.3, D6).
pub fn plan_core(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Batch {
    let mut batch_diagnostics = Vec::new();
    validate_language_values(profile, lang, &mut batch_diagnostics);
    let known_extensions = id.known_extensions();
    validate_extension_values(profile, known_extensions.as_deref(), &mut batch_diagnostics);

    let primary_paths: Vec<PathBuf> = primaries.iter().map(|p| p.path.clone()).collect();
    let output_dir = run
        .output
        .clone()
        .or_else(|| profile.output.directory.clone())
        .unwrap_or_else(|| run.source.clone());
    let policy = run.on_collision.unwrap_or(profile.output.on_collision);

    let mut files: Vec<FileReport> = Vec::new();
    let mut resolved_sources: Vec<PathBuf> = Vec::new();
    let mut found_versions: Vec<u64> = Vec::new();
    for primary in primaries {
        let (report, sources, format_version) =
            resolve_file(profile, primary, &primary_paths, &output_dir, id, lang);
        files.push(report);
        resolved_sources.extend(sources);
        found_versions.extend(format_version);
    }
    detect_schema_drift(&found_versions, &mut batch_diagnostics);

    // SourceOverwrite is batch-wide (spec 4.8, 5.2): it needs every file's
    // resolved donors, not just the current one's, so it runs as its own
    // pass before anything drops a plan. Then drop plans with resolution
    // errors, so collision detection only considers files that will actually
    // produce output; then re-drop any plan that a collision error just
    // invalidated.
    detect_source_overwrites(&mut files, &primary_paths, &resolved_sources);
    finalize_plans(&mut files);
    detect_output_collisions(&mut files, policy);
    finalize_plans(&mut files);
    detect_empty_plans(&mut files);

    Batch {
        files,
        batch_diagnostics,
        suggestions: Vec::new(),
    }
}

/// Plans the whole batch and attaches batch-validated suggestions (spec 5).
pub fn plan_batch(
    profile: &Profile,
    run: &RunInputs,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> Batch {
    let (primaries, discovery_diags) = discovery::scan_primaries(&run.source, &profile.input);
    let mut batch = plan_core(profile, run, &primaries, id, lang);
    batch.batch_diagnostics.extend(discovery_diags);
    let (suggestions, cap_diagnostics) = suggest(profile, run, &primaries, id, lang, &batch);
    batch.suggestions = suggestions;
    batch.batch_diagnostics.extend(cap_diagnostics);
    batch
}

fn validate_language_values(profile: &Profile, lang: &LanguageIndex, diags: &mut Vec<Diagnostic>) {
    for (i, rule) in profile.tracks.rules.iter().enumerate() {
        walk_exact_languages(&rule.match_expr, &format!("tracks[{i}].match"), lang, diags);
    }
}

// Batch-wide, once per plan_core call (spec 4.2, 4.6, walkthrough #3):
// checks `profile.input.extensions` and every locator's `extensions` (a
// track rule's external source, `chapters`, each `attachments.rules[i].add`)
// against the runtime's `--list-types` output, mirroring
// `validate_language_values`'s structure. `known` is `None` when the
// capability is unavailable (mkvmerge absent or the query failed); the
// check then degrades to a no-op rather than blocking planning, unlike
// `validate_language_values`'s `lang`, which callers must always resolve
// before planning can start at all. No dedup by extension value: the same
// unknown extension repeated across `input.extensions` and a locator (or
// across two locators) gets one diagnostic per occurrence, each at its own
// `config_path`, exactly as two repeated `input.extensions` entries always
// did.
fn validate_extension_values(
    profile: &Profile,
    known: Option<&[String]>,
    diags: &mut Vec<Diagnostic>,
) {
    let Some(known) = known else { return };
    validate_extension_list(known, "input.extensions", &profile.input.extensions, diags);

    for (i, rule) in profile.tracks.rules.iter().enumerate() {
        if let SourceCfg::External(block) = &rule.source {
            validate_extension_list(
                known,
                &format!("tracks[{i}].source.external.extensions"),
                &block.external.extensions,
                diags,
            );
        }
    }
    if let ChaptersCfg::External(block) = &profile.chapters {
        validate_extension_list(
            known,
            "chapters.external.extensions",
            &block.external.extensions,
            diags,
        );
    }
    for (i, rule) in profile.attachments.rules.iter().enumerate() {
        if let Some(locator) = &rule.add {
            validate_extension_list(
                known,
                &format!("attachments.rules[{i}].add.extensions"),
                &locator.extensions,
                diags,
            );
        }
    }
}

// Shared per-list core (Task 5.9 reuse): checks one `extensions` list
// against `known`, raising `UnknownExtension` at `{path_prefix}[i]` per
// offending entry. Backs both `input.extensions` and every locator
// position in `validate_extension_values`.
fn validate_extension_list(
    known: &[String],
    path_prefix: &str,
    extensions: &[String],
    diags: &mut Vec<Diagnostic>,
) {
    for (i, ext) in extensions.iter().enumerate() {
        let normalized = ext.to_ascii_lowercase();
        if !known.contains(&normalized) {
            diags.push(
                Diagnostic::warning(DiagCode::UnknownExtension, format!("{path_prefix}[{i}]"))
                    .with("extension", ext.clone())
                    .with("known", known.join(", ")),
            );
        }
    }
}

// SchemaDrift (D32 addendum, Task 16.5, Şenol 2026-07-12 ruling): once per
// batch, info severity, decoupled from `raw:` consumption entirely (that
// stays `UnknownPropertySkew`'s job, emitted per consumed property in
// `emit_raw_property_skew`). Rebuilds the general "your mkvmerge speaks a
// newer identification schema" notice T16 dropped when D32 repurposed
// `UnknownPropertySkew` to the per-property `raw:` path (that removal
// silenced the signal even for a batch with no `raw:` property in play at
// all). Fires when ANY identified file in `found_versions` (collected by
// `plan_core` across every primary `resolve_file` identified, regardless of
// container support) exceeds `PINNED_IDENTIFICATION_FORMAT_VERSION`;
// `found_version` carries the max seen, so a mixed batch reports the most
// skewed file rather than the first. `found_versions` is empty whenever no
// file in the batch identified at all (mkvmerge absent, every query
// failed), which degrades this to a no-op exactly like
// `validate_extension_values` degrades on a `None` capability query.
fn detect_schema_drift(found_versions: &[u64], diags: &mut Vec<Diagnostic>) {
    let max_found = found_versions
        .iter()
        .copied()
        .filter(|&v| v > PINNED_IDENTIFICATION_FORMAT_VERSION)
        .max();
    if let Some(found) = max_found {
        diags.push(
            Diagnostic::info(DiagCode::SchemaDrift, "input")
                .with("found_version", found.to_string())
                .with("pinned", PINNED_IDENTIFICATION_FORMAT_VERSION.to_string()),
        );
    }
}

// Recurses match expressions collecting exact `language` values, checking each
// against the runtime index (plan-time InvalidPropertyValue, D2).
fn walk_exact_languages(
    expr: &MatchExpr,
    path: &str,
    lang: &LanguageIndex,
    diags: &mut Vec<Diagnostic>,
) {
    if let Some(exact) = &expr.exact
        && let Some(Scalar::Str(v)) = exact.get("language")
        && !lang.is_valid_value(v)
    {
        diags.push(
            Diagnostic::error(
                DiagCode::InvalidPropertyValue,
                format!("{path}.exact.language"),
            )
            .with("property", "language")
            .with("value", v.clone())
            .with("allowed", "a valid ISO 639/BCP-47 language code"),
        );
    }
    if let Some(any) = &expr.any {
        for (i, sub) in any.iter().enumerate() {
            walk_exact_languages(sub, &format!("{path}.any[{i}]"), lang, diags);
        }
    }
    if let Some(not) = &expr.not {
        for (i, sub) in not.iter().enumerate() {
            walk_exact_languages(sub, &format!("{path}.not[{i}]"), lang, diags);
        }
    }
}

// Returns the file's report alongside every source path it resolved -
// assignment sources (primaries and track donors) plus attachment `add`
// donors (Task 7.5) - independent of whether the file's own plan ends up
// `Some`; `detect_source_overwrites` needs the latter even when this file's
// own output never renders (Plan-2 FINAL M2 / #7).
fn resolve_file(
    profile: &Profile,
    primary: &PrimaryFile,
    primary_paths: &[PathBuf],
    output_dir: &Path,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> (FileReport, Vec<PathBuf>, Option<u64>) {
    let mut diagnostics = Vec::new();
    let primary_dir = primary.path.parent().unwrap_or(Path::new("."));

    let ident = match id.identify(&primary.path) {
        Ok(i) => i,
        Err(e) => {
            diagnostics.push(
                Diagnostic::error(DiagCode::UnidentifiableSource, "input")
                    .for_file(&primary.path)
                    .with("detail", format!("{e}")),
            );
            return (
                FileReport {
                    source: primary.path.clone(),
                    identifier: primary.identifier.whole.clone(),
                    plan: None,
                    diagnostics,
                },
                Vec::new(),
                None,
            );
        }
    };
    // Every successful identification contributes its schema version to the
    // batch-level SchemaDrift walk (`detect_schema_drift` in `plan_core`),
    // independent of whether the container turns out supported/recognized -
    // mkvmerge already produced a genuine `identification_format_version`
    // for this file at this point (D32 addendum, Task 16.5).
    let format_version = Some(ident.format_version);
    if !ident.container_recognized || !ident.container_supported {
        diagnostics.push(
            Diagnostic::error(DiagCode::UnsupportedSource, "input")
                .for_file(&primary.path)
                .with("kind", "primary"),
        );
        return (
            FileReport {
                source: primary.path.clone(),
                identifier: primary.identifier.whole.clone(),
                plan: None,
                diagnostics,
            },
            Vec::new(),
            format_version,
        );
    }

    let mut assignments = Vec::new();
    // (source_path, track_id) -> rule indices claiming it, for overlap checks.
    let mut claims: BTreeMap<(PathBuf, u64), Vec<usize>> = BTreeMap::new();

    for (ri, rule) in profile.tracks.rules.iter().enumerate() {
        let base = format!("tracks[{ri}]");
        let (source_path, source_ident): (PathBuf, Identification) = match &rule.source {
            SourceCfg::Keyword(_) => (primary.path.clone(), ident.clone()),
            SourceCfg::External(block) => {
                let hits =
                    discovery::resolve_locator(&block.external, primary_dir, &primary.identifier);
                match hits.len() {
                    0 => {
                        if !rule.optional {
                            diagnostics.push(
                                Diagnostic::error(
                                    DiagCode::MissingExternal,
                                    format!("{base}.source.external"),
                                )
                                .for_file(&primary.path),
                            );
                        }
                        assignments.push(Assignment {
                            rule_index: ri,
                            source: primary.path.clone(),
                            track_id: None,
                            track_kind: None,
                            changes: vec![],
                        });
                        continue;
                    }
                    1 => {
                        let donor = hits.into_iter().next().unwrap();
                        if primary_paths.contains(&donor) {
                            diagnostics.push(
                                Diagnostic::warning(
                                    DiagCode::DonorIsPrimary,
                                    format!("{base}.source.external"),
                                )
                                .for_file(&primary.path)
                                .with("donor", donor.display().to_string()),
                            );
                        }
                        match id.identify(&donor) {
                            Ok(di) => {
                                // Same predicate as the primary branch: a donor
                                // that identifies but whose container is not a
                                // supported muxing source is UnsupportedSource,
                                // not a silent skip (spec 5.1).
                                if !di.container_recognized || !di.container_supported {
                                    diagnostics.push(
                                        Diagnostic::error(
                                            DiagCode::UnsupportedSource,
                                            format!("{base}.source.external"),
                                        )
                                        .for_file(&primary.path)
                                        .with("kind", "donor")
                                        .with("donor", donor.display().to_string()),
                                    );
                                    assignments.push(Assignment {
                                        rule_index: ri,
                                        source: primary.path.clone(),
                                        track_id: None,
                                        track_kind: None,
                                        changes: vec![],
                                    });
                                    continue;
                                }
                                (donor, di)
                            }
                            Err(e) => {
                                diagnostics.push(
                                    Diagnostic::error(
                                        DiagCode::UnidentifiableSource,
                                        format!("{base}.source.external"),
                                    )
                                    .for_file(&primary.path)
                                    .with("detail", format!("{e}")),
                                );
                                assignments.push(Assignment {
                                    rule_index: ri,
                                    source: primary.path.clone(),
                                    track_id: None,
                                    track_kind: None,
                                    changes: vec![],
                                });
                                continue;
                            }
                        }
                    }
                    n => {
                        diagnostics.push(
                            Diagnostic::error(
                                DiagCode::AmbiguousExternal,
                                format!("{base}.source.external"),
                            )
                            .for_file(&primary.path)
                            .with("count", n.to_string()),
                        );
                        assignments.push(Assignment {
                            rule_index: ri,
                            source: primary.path.clone(),
                            track_id: None,
                            track_kind: None,
                            changes: vec![],
                        });
                        continue;
                    }
                }
            }
        };

        // UnknownPropertySkew (spec 9.2, D32): one warning per `raw:` property
        // consumed while evaluating this rule against its resolved source,
        // carrying the source file's `identification_format_version`
        // (`found_version`) and this build's `pinned` schema. Emitted before
        // the match-count branch, so it fires regardless of whether the rule
        // resolves, matches nothing (still a MissingTrack, B-11), or is
        // ambiguous - the untyped match was attempted either way.
        emit_raw_property_skew(
            &rule.match_expr,
            &format!("{base}.match"),
            &primary.path,
            source_ident.format_version,
            &mut diagnostics,
        );

        let matched: Vec<(u64, String)> = source_ident
            .tracks
            .iter()
            .filter(|t| matcher::matches(&rule.match_expr, t, lang))
            .map(|t| (t.id, t.kind.clone()))
            .collect();

        match matched.len() {
            0 => {
                if !rule.optional {
                    diagnostics.push(
                        Diagnostic::error(DiagCode::MissingTrack, format!("{base}.match"))
                            .for_file(&primary.path),
                    );
                }
                assignments.push(Assignment {
                    rule_index: ri,
                    source: source_path,
                    track_id: None,
                    track_kind: None,
                    changes: vec![],
                });
            }
            1 => {
                let (tid, tkind) = matched[0].clone();
                claims
                    .entry((source_path.clone(), tid))
                    .or_default()
                    .push(ri);
                let changes = resolve_changes(rule, &base, &primary.path, lang, &mut diagnostics);
                assignments.push(Assignment {
                    rule_index: ri,
                    source: source_path,
                    track_id: Some(tid),
                    track_kind: Some(tkind),
                    changes,
                });
            }
            n => {
                diagnostics.push(
                    Diagnostic::error(DiagCode::AmbiguousRule, format!("{base}.match"))
                        .for_file(&primary.path)
                        .with("count", n.to_string()),
                );
                assignments.push(Assignment {
                    rule_index: ri,
                    source: source_path,
                    track_id: None,
                    track_kind: None,
                    changes: vec![],
                });
            }
        }
    }

    // OverlappingRules: one track claimed by two or more rules (spec 5.2).
    // Names every claimant, not just the first pair: three rules colliding on
    // one track is one diagnostic listing all three.
    for ((_src, tid), rules) in &claims {
        if rules.len() >= 2 {
            let refs: Vec<String> = rules.iter().map(|r| format!("tracks[{r}]")).collect();
            diagnostics.push(
                Diagnostic::error(DiagCode::OverlappingRules, format!("tracks[{}]", rules[0]))
                    .for_file(&primary.path)
                    .with("rules", refs.join(", "))
                    .with("track", tid.to_string()),
            );
        }
    }

    let output = render_output(profile, primary, output_dir, &mut diagnostics);
    let title = resolve_title(profile, primary);
    let tags = resolve_tags(profile);
    let chapters = resolve_chapters(profile, primary, primary_dir, &mut diagnostics);
    let attachments = resolve_attachments(
        profile,
        primary,
        primary_dir,
        &ident.attachments,
        lang,
        &mut diagnostics,
    );

    let keep_unmatched = matches!(
        profile.tracks.unmatched,
        crate::profile::model::KeepDrop::Keep
    );

    // Captured before `assignments`, `attachments` and `chapters` move into
    // `Plan` below: every source this file resolved is known at this point
    // already, regardless of whether `output` renders successfully.
    // Completeness (Task 7.6, #7 class closure): every donor kind reaches
    // this chain - track rules (`Assignment.source`), attachment `add`
    // donors (`AttachmentPlan.add_files`, Task 7.5), and now chapters
    // (`ChapterSource::External`, Task 7.6). `model.rs` has exactly two
    // `Locator` field sites feeding these three kinds:
    // `ExternalBlock.external` (shared by `SourceCfg::External` and
    // `ChaptersCfg::External`) and `AttachmentRule.add`. A future third
    // `Locator` field site must be chained in here too, or it silently
    // re-opens this class.
    let resolved_sources: Vec<PathBuf> = assignments
        .iter()
        .map(|a| a.source.clone())
        .chain(attachments.add_files.iter().cloned())
        .chain(match &chapters {
            ChapterSource::External(path) => Some(path.clone()),
            ChapterSource::Keep | ChapterSource::Drop => None,
        })
        .collect();

    let plan = output.map(|output| Plan {
        source: primary.path.clone(),
        output,
        assignments,
        attachments,
        chapters,
        tags,
        title,
        keep_unmatched,
        primary_track_ids: ident.tracks.iter().map(|t| t.id).collect(),
    });

    (
        FileReport {
            source: primary.path.clone(),
            identifier: primary.identifier.whole.clone(),
            plan,
            diagnostics,
        },
        resolved_sources,
        format_version,
    )
}

// Emits one UnknownPropertySkew warning (spec 9.2, D32) per distinct `raw:`
// property referenced anywhere in this rule's match expression, at the rule's
// `.match` path. `found_version` is the resolved source's schema version, so
// the message can show it against `pinned` whether the runtime schema is
// genuinely newer (B-9) or the same (B-10). Deduplicated per rule via the
// BTreeSet, which also fixes the emission order.
fn emit_raw_property_skew(
    expr: &MatchExpr,
    match_path: &str,
    file: &Path,
    found_version: u64,
    diags: &mut Vec<Diagnostic>,
) {
    let mut raw_props: BTreeSet<String> = BTreeSet::new();
    collect_raw_props(expr, &mut raw_props);
    for prop in raw_props {
        diags.push(
            Diagnostic::warning(DiagCode::UnknownPropertySkew, match_path.to_string())
                .for_file(file)
                .with("property", prop)
                .with("found_version", found_version.to_string())
                .with("pinned", PINNED_IDENTIFICATION_FORMAT_VERSION.to_string()),
        );
    }
}

// Collects the bare (prefix-stripped) names of every `raw:`-prefixed property
// used in `expr`'s exact/substring/regex maps, recursing into `any`/`not`.
fn collect_raw_props(expr: &MatchExpr, out: &mut BTreeSet<String>) {
    let exact_keys = expr.exact.iter().flat_map(|m| m.keys());
    let substring_keys = expr.substring.iter().flat_map(|m| m.keys());
    let regex_keys = expr.regex.iter().flat_map(|m| m.keys());
    for key in exact_keys.chain(substring_keys).chain(regex_keys) {
        if let Some(bare) = key.strip_prefix("raw:") {
            out.insert(bare.to_string());
        }
    }
    for sub in expr.any.iter().flatten().chain(expr.not.iter().flatten()) {
        collect_raw_props(sub, out);
    }
}

// Builds the AppliedChange list for a track a rule resolved to, from the
// rule's `changes` map (spec 4.4). Iterates the BTreeMap in key order, so the
// result is already property-name ascending. Validates a `language` value
// against the runtime LanguageIndex at the point of application (plan-time
// InvalidPropertyValue, D2); other settables are carried through unchecked,
// since validate.rs already checked their type and known-ness at config
// time (deliberate scope choice, see task brief: batch-wide language
// consistency with `walk_exact_languages` is not required for v1).
fn resolve_changes(
    rule: &TrackRule,
    base: &str,
    primary_path: &Path,
    lang: &LanguageIndex,
    diags: &mut Vec<Diagnostic>,
) -> Vec<AppliedChange> {
    let Some(changes) = &rule.changes else {
        return Vec::new();
    };
    changes
        .iter()
        .map(|(property, value)| {
            if property == "language" {
                let valid = matches!(value, Scalar::Str(s) if lang.is_valid_value(s));
                if !valid {
                    diags.push(
                        Diagnostic::error(
                            DiagCode::InvalidPropertyValue,
                            format!("{base}.changes.language"),
                        )
                        .for_file(primary_path)
                        .with("property", "language")
                        .with("value", scalar_display(value))
                        // `invalid-property-value` requires `allowed`; the
                        // sibling emitter in `walk_exact_languages` sets it, so
                        // this site must too or `{$allowed}` leaks to the user.
                        .with("allowed", "a valid ISO 639/BCP-47 language code"),
                    );
                }
            }
            AppliedChange {
                property: property.clone(),
                value: value.clone(),
            }
        })
        .collect()
}

// Renders a Scalar as the plain string a diagnostic's `value` param wants:
// no quoting, no type tag, just the value a user typed (or its literal
// rendering for non-string types, which only reach here via a mistyped
// `changes.language`, since the settable `language` is always a string).
fn scalar_display(value: &Scalar) -> String {
    match value {
        Scalar::Bool(b) => b.to_string(),
        Scalar::Int(i) => i.to_string(),
        Scalar::Float(f) => f.to_string(),
        Scalar::Str(s) => s.clone(),
    }
}

// Renders the output path, enforcing the D4 rendered-name invariants. Returns
// None (and pushes a diagnostic) if the rendered name is invalid.
fn render_output(
    profile: &Profile,
    primary: &PrimaryFile,
    output_dir: &Path,
    diags: &mut Vec<Diagnostic>,
) -> Option<PathBuf> {
    // Rendered BEFORE ".mkv" is appended: PathSeparatorInRenderedName must
    // see the actual rendered value, not one already carrying the enforced
    // extension (".mkv" never itself introduces a separator, but a
    // template's own output could).
    let rendered = match &profile.output.filename {
        FilenameCfg::Keyword(_) => primary
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string(),
        FilenameCfg::Template(block) => {
            let mut ctx = primary.identifier.to_ctx();
            if let Some(stem) = primary.path.file_stem().and_then(|s| s.to_str()) {
                ctx.set("source_stem", stem);
            }
            Template::parse(&block.template)
                .map(|t| t.render_literal(&ctx))
                .unwrap_or_default()
        }
    };

    if rendered.contains('/') || rendered.contains('\\') {
        diags.push(
            Diagnostic::error(DiagCode::PathSeparatorInRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", rendered.clone()),
        );
        return None;
    }

    // keep and template diverge here (spec 4.8): keep is file_stem +
    // ".mkv" unconditionally, since the source's own extension carries no
    // meaning to preserve, even if the stem happens to already end in
    // something that looks like ".mkv". A template's rendered value keeps
    // a trailing ".mkv" if already present (case-insensitively) instead of
    // doubling it. The two must not share one conditional.
    let name = match &profile.output.filename {
        FilenameCfg::Keyword(_) => format!("{rendered}.mkv"),
        FilenameCfg::Template(_) => {
            if rendered.to_lowercase().ends_with(".mkv") {
                rendered.clone()
            } else {
                format!("{rendered}.mkv")
            }
        }
    };

    // D4 / spec 4.8 empty-name invariant, enforced on the FINAL name's
    // stem, not the pre-append `rendered` value above: a template that
    // renders to exactly ".mkv" (or to an empty field followed by a
    // literal ".mkv" segment) is non-empty and not "."/".." before ".mkv"
    // is appended, since it already carries that extension, so a
    // pre-append check never sees the problem. Stripping ".mkv" back off
    // the final name catches this the same way as an explicit "." or "..".
    let stem = strip_mkv_suffix(&name);
    if stem.is_empty() || stem == "." || stem == ".." {
        diags.push(
            Diagnostic::error(DiagCode::EmptyRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", rendered.clone()),
        );
        return None;
    }

    Some(output_dir.join(name))
}

// Strips a trailing ".mkv" (case-insensitive) off `name`, or returns `name`
// unchanged if it does not end in ".mkv". `is_char_boundary` guards the
// slice: ".mkv" is ASCII, so a real match is always a safe boundary, but a
// non-matching multi-byte tail must never be sliced into.
fn strip_mkv_suffix(name: &str) -> &str {
    let len = name.len();
    if len >= 4 && name.is_char_boundary(len - 4) && name[len - 4..].eq_ignore_ascii_case(".mkv") {
        &name[..len - 4]
    } else {
        name
    }
}

// Resolves `profile.title` to a `TitleAction` (spec 4.9). Unlike
// `render_output`'s filename, a title has no path-separator or empty-name
// invariant: an empty rendered title is a legitimate `Set("")`, so a
// template's render always passes through unchecked. The Ctx mirrors
// `render_output`'s exactly, including `source_stem`: validate.rs allows
// `source_stem` in `title.template` identically to `output.filename.template`
// (see `validate::validate`), so the two templates' available fields must
// stay in lockstep. A parse failure or an unexpected keyword cannot occur
// post-validate (validate.rs rejects both at config time); the fallback to
// `Keep` is defensive only, never a panic.
fn resolve_title(profile: &Profile, primary: &PrimaryFile) -> TitleAction {
    match &profile.title {
        TitleCfg::Keyword(k) if k == "clear" => TitleAction::Clear,
        TitleCfg::Keyword(_) => TitleAction::Keep,
        TitleCfg::Template(block) => {
            let mut ctx = primary.identifier.to_ctx();
            if let Some(stem) = primary.path.file_stem().and_then(|s| s.to_str()) {
                ctx.set("source_stem", stem);
            }
            match Template::parse(&block.template) {
                Ok(t) => TitleAction::Set(t.render_literal(&ctx)),
                Err(_) => TitleAction::Keep,
            }
        }
    }
}

// Resolves `profile.tags` to a `TagFlags` (spec 4.9); a direct KeepDrop ->
// bool mapping, no plan-time validation needed.
fn resolve_tags(profile: &Profile) -> TagFlags {
    TagFlags {
        global_keep: profile.tags.global == KeepDrop::Keep,
        track_keep: profile.tags.track == KeepDrop::Keep,
    }
}

// Resolves `profile.chapters` to a `ChapterSource` (spec 4.9). The external
// case reuses the same locator machinery as a track rule's external source
// (`resolve_locator` plus the 0/1/n uniqueness split), but a chapters file is
// never run through `Identify`: it is XML/simple-format text consumed
// directly by `--chapters`, not an mkvmerge source with tracks of its own.
// `config_path` is the literal top-level `"chapters.external"`, never
// `tracks[i]`-scoped, since chapters is a singular profile-wide setting.
// Unlike a track rule's external source, there is no `optional` escape: zero
// matches is always `MissingExternal`. Both error branches return `Keep` as
// a placeholder; it never surfaces because the pushed error diagnostic
// already forces `plan: None` in the caller's later finalize step.
fn resolve_chapters(
    profile: &Profile,
    primary: &PrimaryFile,
    primary_dir: &Path,
    diags: &mut Vec<Diagnostic>,
) -> ChapterSource {
    match &profile.chapters {
        ChaptersCfg::Keyword(k) if k == "drop" => ChapterSource::Drop,
        ChaptersCfg::Keyword(_) => ChapterSource::Keep,
        ChaptersCfg::External(block) => {
            let hits =
                discovery::resolve_locator(&block.external, primary_dir, &primary.identifier);
            match hits.len() {
                1 => ChapterSource::External(hits.into_iter().next().unwrap()),
                0 => {
                    diags.push(
                        Diagnostic::error(DiagCode::MissingExternal, "chapters.external")
                            .for_file(&primary.path),
                    );
                    ChapterSource::Keep
                }
                n => {
                    diags.push(
                        Diagnostic::error(DiagCode::AmbiguousExternal, "chapters.external")
                            .for_file(&primary.path)
                            .with("count", n.to_string()),
                    );
                    ChapterSource::Keep
                }
            }
        }
    }
}

// Resolves `profile.attachments` against the primary's own attachments
// (spec 4.9, D10): donor attachments never flow in, so `primary_attachments`
// is always the primary's identification's attachments, never a donor's
// (the command drops donor attachments outright via `--no-attachments`,
// Task 11).
//
// Existing attachments (select/drop/unmatched): walks `rules` in order per
// attachment; the first matching `select` keeps it, the first matching
// `drop` drops it, `add` rules are skipped in this pass. An attachment no
// select/drop rule claims falls to `unmatched`. The kept id set then reduces
// to the most compact `PrimaryAttachments` the command can express: `KeepAll`
// when nothing was filtered out, `DropAll` when everything was, `Subset`
// otherwise (spec 4.9's minimal-argv intent).
//
// Adds (D12): each `add` locator is a query that populates the attachment
// collection, like `select`/`drop`, not a unique slot-filler like a donor
// source, so ALL its hits are attached, not just one. A rule whose locator
// matches zero files still gets a `MissingExternal` warning (not an error:
// an add is an auxiliary payload, so a miss must never suppress the plan) at
// `attachments.rules[i].add`. After every rule runs, `add_files` is deduped
// by path, keeping first-seen order, so two rules matching one file attach
// it once.
fn resolve_attachments(
    profile: &Profile,
    primary: &PrimaryFile,
    primary_dir: &Path,
    primary_attachments: &[Attachment],
    lang: &LanguageIndex,
    diags: &mut Vec<Diagnostic>,
) -> AttachmentPlan {
    let mut kept: Vec<u64> = Vec::new();
    for att in primary_attachments {
        let mut decision: Option<bool> = None;
        for rule in &profile.attachments.rules {
            if let Some(select) = &rule.select
                && matcher::matches(select, att, lang)
            {
                decision = Some(true);
                break;
            }
            if let Some(drop) = &rule.drop
                && matcher::matches(drop, att, lang)
            {
                decision = Some(false);
                break;
            }
        }
        if decision.unwrap_or(profile.attachments.unmatched == KeepDrop::Keep) {
            kept.push(att.id);
        }
    }

    let primary_disposition = if kept.len() == primary_attachments.len() {
        PrimaryAttachments::KeepAll
    } else if kept.is_empty() {
        PrimaryAttachments::DropAll
    } else {
        kept.sort_unstable();
        PrimaryAttachments::Subset(kept)
    };

    let mut add_files: Vec<PathBuf> = Vec::new();
    for (i, rule) in profile.attachments.rules.iter().enumerate() {
        let Some(locator) = &rule.add else { continue };
        let hits = discovery::resolve_locator(locator, primary_dir, &primary.identifier);
        if hits.is_empty() {
            diags.push(
                Diagnostic::warning(
                    DiagCode::MissingExternal,
                    format!("attachments.rules[{i}].add"),
                )
                .for_file(&primary.path),
            );
        }
        add_files.extend(hits);
    }
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    add_files.retain(|p| seen.insert(p.clone()));

    AttachmentPlan {
        primary: primary_disposition,
        add_files,
    }
}

// A rendered output must never equal an input path anywhere in the batch:
// every primary, plus every donor any file's rules resolved - track donors
// (`Assignment.source`), attachment donors (`AttachmentPlan.add_files`,
// Task 7.5), and chapters donors (`ChapterSource::External`, Task 7.6)
// alike (spec 4.8, 5.2). Batch-wide because one primary's output can equal
// a donor a *different* primary reads from; `resolved_sources` (built in
// `resolve_file`, before that file's own output ever renders) already
// carries every one of those source paths - donor or primary - independent
// of whether the file's own plan survives, so the union of
// `resolved_sources` plus the primaries is the complete input set. Runs
// before `finalize_plans` drops anything.
//
// Plan-2 FINAL M2 / #7: sources are known before rendering, so a file whose
// own output fails to render (`plan == None`) must still contribute its
// donors here - the previous version read `plan.assignments`, which does
// not exist once `plan` is `None`, so a donor referenced solely by such a
// file escaped protection and a colliding output could overwrite it
// silently. Task 7.5 closed the same gap for attachment donors, Task 7.6
// for chapters donors: neither was ever in `resolved_sources` at all
// (gathered only from `Assignment`), regardless of render outcome. #7 is
// now closed by construction (see the completeness comment at
// `resolved_sources`'s gathering site in `resolve_file`).
//
// S11 guard: `resolve_file`'s `AmbiguousExternal` branch (2+ candidate
// donors for one locator) deliberately pushes a placeholder assignment
// sourced at the primary path, not at any of the n candidates - which one
// is "the" donor is genuinely unknown, so none of them is protected here.
// Safe only because `AmbiguousExternal` is unconditionally Error-severity
// (that file's own plan never survives regardless, per `finalize_plans`);
// if it is ever downgraded to non-fatal, this function needs to start
// protecting all n candidates explicitly (F5 report).
fn detect_source_overwrites(
    files: &mut [FileReport],
    primary_paths: &[PathBuf],
    resolved_sources: &[PathBuf],
) {
    let mut inputs: BTreeSet<PathBuf> = primary_paths.iter().cloned().collect();
    inputs.extend(resolved_sources.iter().cloned());
    for f in files.iter_mut() {
        let Some(plan) = &f.plan else { continue };
        if inputs.contains(&plan.output) {
            let path = plan.output.display().to_string();
            f.diagnostics.push(
                Diagnostic::error(DiagCode::SourceOverwrite, "output")
                    .for_file(&f.source)
                    .with("path", path),
            );
        }
    }
}

// Two plans rendering to the same output path collide, as does an existing
// on-disk file (spec 4.8, decision #3). Only surviving plans (post the
// SourceOverwrite finalize pass) are considered.
fn detect_output_collisions(files: &mut [FileReport], policy: CollisionPolicy) {
    let mut counts: BTreeMap<PathBuf, usize> = BTreeMap::new();
    for f in files.iter() {
        if let Some(p) = &f.plan {
            *counts.entry(p.output.clone()).or_default() += 1;
        }
    }
    for f in files.iter_mut() {
        let Some(out) = f.plan.as_ref().map(|p| p.output.clone()) else {
            continue;
        };
        let planned_twice = counts.get(&out).copied().unwrap_or(0) >= 2;
        let exists_on_disk = out.exists();
        if !planned_twice && !exists_on_disk {
            continue;
        }
        // Two planned outputs to one path: the batch is internally
        // inconsistent, always an error, independent of `on_collision`
        // (decision #3) -- neither skip nor overwrite can pick a winner. A
        // pre-existing on-disk file that is not itself a batch input
        // (SourceOverwrite covers that case) follows the policy.
        let severity = if planned_twice {
            Severity::Error
        } else {
            match policy {
                CollisionPolicy::Error => Severity::Error,
                CollisionPolicy::Skip => Severity::Warning,
                CollisionPolicy::Overwrite => Severity::Info,
            }
        };
        f.diagnostics.push(
            Diagnostic::info(DiagCode::OutputCollision, "output")
                .for_file(&f.source)
                .with("path", out.display().to_string())
                .with_severity(severity),
        );
        // "skip" on an on-disk collision means the output is not produced
        // (bug E), even though the diagnostic above is only a Warning;
        // `finalize_plans` only drops Error-severity plans, so null it here
        // explicitly rather than relying on that pass.
        if !planned_twice && policy == CollisionPolicy::Skip {
            f.plan = None;
        }
    }
}

// EmptyPlan (spec 5.2, D18/#6): a plan that survived both finalize_plans
// passes above still resolved zero output tracks. Runs last, after the
// cross-file passes (`detect_source_overwrites`, `detect_output_collisions`)
// and their finalize calls, so `f.plan.is_some()` here already means "no
// error, local or cross-file, doomed this file's plan" - a file that
// resolves to zero tracks locally but then loses its plan to a cross-file
// error never gets this warning stacked on top of that error. `Plan`'s
// `assignments`/`keep_unmatched`/`primary_track_ids` are public, so
// has-tracks is fully recomputable from the surviving plan alone; no
// severity scan needed.
fn detect_empty_plans(files: &mut [FileReport]) {
    for f in files.iter_mut() {
        let Some(plan) = &f.plan else { continue };
        // Either a rule matched (`Assignment::track_id` is `Some`), or,
        // under `keep`, the primary's own tracks pass through untouched
        // regardless of rule matches (D20: "keep = match to what is
        // already there", so a non-empty primary counts as matched even
        // when no rule fired).
        let has_tracks = plan.assignments.iter().any(|a| a.track_id.is_some())
            || (plan.keep_unmatched && !plan.primary_track_ids.is_empty());
        if !has_tracks {
            f.diagnostics
                .push(Diagnostic::warning(DiagCode::EmptyPlan, "tracks").for_file(&f.source));
        }
    }
}

// Drop the plan for any file that has an error-severity diagnostic (spec 5.1).
fn finalize_plans(files: &mut [FileReport]) {
    for f in files.iter_mut() {
        if f.diagnostics.iter().any(|d| d.severity == Severity::Error) {
            f.plan = None;
        }
    }
}

// A candidate refinement plus the concrete delta to splice into the rule.
#[derive(Clone)]
struct Candidate {
    edit: StructuredEdit,
    apply: MatchExpr,
    rank: (u8, String, String),
}

// How `candidates_for_rule` seeds discriminators. The `AmbiguousRule` path
// needs a >=2 matched set and both polarities (a discriminator that includes
// one of the tied tracks resolves the ambiguity). The `OverlappingRules` path
// (D33) seeds from the single shared track (`matched.len() == 1`) and emits
// NOT-polarity only: a narrowing can only zero a claim, never redirect it, so
// the sole useful edit is one that EXCLUDES the contested track. A positive
// `AddExact` toward a value the track lacks would also zero the claim but is
// an oblique, near-tie-noise way to say "not this track" (and a no-op when it
// duplicates an existing key), so the overlap seed drops it.
#[derive(Clone, Copy, PartialEq)]
enum SeedMode {
    Ambiguous,
    Overlap,
}

impl SeedMode {
    fn min_matched(self) -> usize {
        match self {
            SeedMode::Ambiguous => 2,
            SeedMode::Overlap => 1,
        }
    }

    fn keeps_polarity(self, polarity: u8) -> bool {
        match self {
            SeedMode::Ambiguous => true,
            SeedMode::Overlap => polarity == 1,
        }
    }
}

/// Generates and validates suggestions for every `AmbiguousRule` and
/// `OverlappingRules` conflict (spec 5.3, D6/D33). For each ambiguous rule
/// (primary or external source): gather the matched (conflicting) tracks
/// across all affected files, derive discriminator candidates, simulate each
/// against the whole batch via [`plan_core`], and keep only those that resolve
/// the ambiguity everywhere with no new diagnostic. For each overlap conflict:
/// generate NOT-polarity candidates for ALL claimants symmetrically and keep
/// those where the target overlap instance disappears without regression -
/// feasibility (a claimant's `optional` flag / batch structure) selects the
/// rule, no precedence is assumed (D33). Deterministic; capped at 3 per
/// conflict, with a `SuggestionsCapped` diagnostic so the cap is never silent.
/// When no candidate resolves a conflict batch-wide, the no-single-fix
/// partition ([`partition_for_rule`] / [`partition_for_overlap`]) is reported;
/// an unresolvable overlap emits no partition and lets the standing
/// `OverlappingRules` diagnostic (naming every claimant) stand as the report.
#[allow(clippy::too_many_arguments)]
fn suggest(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
    baseline: &Batch,
) -> (Vec<Suggestion>, Vec<Diagnostic>) {
    let base_sig = diag_signature(baseline);

    let mut conflicted: Vec<usize> = baseline
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .filter(|d| d.code == DiagCode::AmbiguousRule)
        .filter_map(|d| rule_index_of(&d.config_path))
        .collect();
    conflicted.sort_unstable();
    conflicted.dedup();

    let mut out = Vec::new();
    let mut cap_diagnostics = Vec::new();
    for ri in conflicted {
        if profile.tracks.rules.get(ri).is_none() {
            continue;
        }
        let candidates = candidates_for_rule(profile, ri, primaries, id, lang, SeedMode::Ambiguous);
        let mut accepted: Vec<Candidate> = Vec::new();
        for cand in &candidates {
            let edited = with_rule_match(profile, ri, &cand.apply);
            let sim = plan_core(&edited, run, primaries, id, lang);
            if resolves_without_regression(&sim, ri, &base_sig) {
                accepted.push(cand.clone());
            }
        }
        // No single refinement resolves the rule batch-wide: report the
        // no-single-fix partition instead of empty suggestions (spec 5.3, D6
        // step 6).
        if accepted.is_empty() {
            cap_diagnostics.extend(partition_for_rule(
                profile,
                run,
                primaries,
                id,
                lang,
                ri,
                &candidates,
                baseline,
                &base_sig,
            ));
            continue;
        }
        accepted.sort_by(|a, b| a.rank.cmp(&b.rank));
        let total_accepted = accepted.len();
        accepted.truncate(3);
        let dropped = total_accepted - accepted.len();
        if dropped > 0 {
            cap_diagnostics.push(
                Diagnostic::info(DiagCode::SuggestionsCapped, format!("tracks[{ri}].match"))
                    .with("dropped", dropped.to_string()),
            );
        }
        for cand in accepted {
            let fragment = yaml_fragment(ri, &cand.apply);
            out.push(Suggestion {
                resolves: DiagCode::AmbiguousRule,
                config_path: format!("tracks[{ri}].match"),
                yaml_fragment: fragment,
                edit: cand.edit,
            });
        }
    }

    // OverlappingRules suggestions (D33): symmetric generation across ALL
    // claimants of each overlap, acceptance-filtered on the overlap instance
    // disappearing. Feasibility - a claimant's `optional` flag and its
    // batch-wide matches - selects which narrowing survives; no precedence is
    // assumed. Reuses candidates_for_rule (NOT-polarity, single-track seed),
    // resolves_overlap_without_regression, and yaml_fragment. The common
    // two-required-single-file case and the >=3-claimant case both survive
    // nothing here and fall through to partition_for_overlap, which for an
    // unresolvable overlap emits nothing: the standing OverlappingRules
    // diagnostic (T9's $rules list names every claimant) is the no-fix report.
    for conflict in overlap_conflicts(baseline) {
        // (rule index, candidate) tagged so each narrowing knows the rule it
        // edits - unlike the ambiguous path, candidates for one conflict span
        // several rules.
        let mut tagged: Vec<(usize, Candidate)> = Vec::new();
        for &ri in &conflict.claimants {
            if profile.tracks.rules.get(ri).is_none() {
                continue;
            }
            for cand in candidates_for_rule(profile, ri, primaries, id, lang, SeedMode::Overlap) {
                tagged.push((ri, cand));
            }
        }

        let mut accepted: Vec<(usize, Candidate)> = Vec::new();
        for (ri, cand) in &tagged {
            let edited = with_rule_match(profile, *ri, &cand.apply);
            let sim = plan_core(&edited, run, primaries, id, lang);
            if resolves_overlap_without_regression(&sim, &conflict, &base_sig) {
                accepted.push((*ri, cand.clone()));
            }
        }

        if accepted.is_empty() {
            cap_diagnostics.extend(partition_for_overlap(
                profile, run, primaries, id, lang, &conflict, &tagged,
            ));
            continue;
        }

        // Rank: the candidate's own dimension rank first; ties between
        // narrowings on DIFFERENT claimant rules break broader-rule-first
        // (larger match domain), then lower rule index (D33).
        let breadth: BTreeMap<usize, usize> = conflict
            .claimants
            .iter()
            .map(|&ri| (ri, rule_breadth(profile, ri, primaries, id, lang)))
            .collect();
        accepted.sort_by(|(ra, ca), (rb, cb)| {
            ca.rank
                .cmp(&cb.rank)
                .then_with(|| breadth[rb].cmp(&breadth[ra]))
                .then_with(|| ra.cmp(rb))
        });
        let total_accepted = accepted.len();
        accepted.truncate(3);
        let dropped = total_accepted - accepted.len();
        if dropped > 0 {
            // Keyed on the lowest claimant, mirroring the OverlappingRules
            // diagnostic's own `config_path` convention.
            let anchor = conflict.claimants.first().copied().unwrap_or(0);
            cap_diagnostics.push(
                Diagnostic::info(
                    DiagCode::SuggestionsCapped,
                    format!("tracks[{anchor}].match"),
                )
                .with("dropped", dropped.to_string()),
            );
        }
        for (ri, cand) in accepted {
            out.push(Suggestion {
                resolves: DiagCode::OverlappingRules,
                config_path: format!("tracks[{ri}].match"),
                yaml_fragment: yaml_fragment(ri, &cand.apply),
                edit: cand.edit,
            });
        }
    }

    (out, cap_diagnostics)
}

// The no-single-fix partition report cap (spec 5.3, D6 step 6): at most this
// many resolution groups are rendered per rule, the surplus recorded in an
// overflow note, mirroring `SuggestionsCapped`'s never-silent cap philosophy.
const PARTITION_GROUP_CAP: usize = 5;

// Reports the no-single-fix partition for rule `ri` (spec 5.3, D6 step 6):
// when no candidate resolves the rule batch-wide, group the affected files by
// the per-file refinement that WOULD resolve each in isolation (its
// top-ranked resolving candidate), so the report states "these files need one
// narrowing, those another". One `SuggestionPartition` info diagnostic per
// group, deterministically ordered and capped at [`PARTITION_GROUP_CAP`] with
// an overflow note. Reuses the discriminator candidates already generated for
// the batch-wide pass.
#[allow(clippy::too_many_arguments)]
fn partition_for_rule(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
    ri: usize,
    candidates: &[Candidate],
    baseline: &Batch,
    base_sig: &BTreeMap<String, usize>,
) -> Vec<Diagnostic> {
    // Files whose baseline report carries an AmbiguousRule for this rule.
    let affected: Vec<&PrimaryFile> = primaries
        .iter()
        .filter(|p| {
            baseline.files.iter().any(|f| {
                f.source == p.path
                    && f.diagnostics.iter().any(|d| {
                        d.code == DiagCode::AmbiguousRule
                            && rule_index_of(&d.config_path) == Some(ri)
                    })
            })
        })
        .collect();

    // group fragment (the per-file refinement, rendered once) -> member files.
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for primary in affected {
        let single = std::slice::from_ref(primary);
        let mut best: Option<&Candidate> = None;
        for cand in candidates {
            let edited = with_rule_match(profile, ri, &cand.apply);
            let sim = plan_core(&edited, run, single, id, lang);
            if resolves_without_regression(&sim, ri, base_sig)
                && best.is_none_or(|b| cand.rank < b.rank)
            {
                best = Some(cand);
            }
        }
        if let Some(cand) = best {
            groups
                .entry(yaml_fragment(ri, &cand.apply))
                .or_default()
                .push(primary.path.display().to_string());
        }
    }

    let total_groups = groups.len();
    let mut diagnostics = Vec::new();
    for (fragment, mut files) in groups.into_iter().take(PARTITION_GROUP_CAP) {
        files.sort();
        diagnostics.push(
            Diagnostic::info(DiagCode::SuggestionPartition, format!("tracks[{ri}].match"))
                .with("kind", "group")
                .with("count", files.len().to_string())
                .with("fix", fragment)
                .with("files", files.join(", ")),
        );
    }
    if total_groups > PARTITION_GROUP_CAP {
        diagnostics.push(
            Diagnostic::info(DiagCode::SuggestionPartition, format!("tracks[{ri}].match"))
                .with("kind", "overflow")
                .with("dropped", (total_groups - PARTITION_GROUP_CAP).to_string()),
        );
    }
    diagnostics
}

// The no-single-fix report for one overlap conflict (D33 touch-point 4), run
// when no narrowing resolves it batch-wide. An overlap is a single-file fact,
// so this partitions over exactly one file (the conflict's): it reports the
// top-ranked narrowing that resolves the conflict IN ISOLATION but was
// rejected batch-wide (a candidate that clears this file yet regresses another
// - e.g. empties a file whose only track it also matched), as one `kind=group`
// diagnostic rendered by the existing partition message.
//
// When no narrowing resolves the conflict even in isolation - the common
// two-required-single-file case and every >=3-claimant overlap (a single edit
// leaves a smaller overlap) - this emits NOTHING: the standing
// `OverlappingRules` diagnostic, which already names every claimant (T9's
// `$rules`), is the no-fix report. That keeps the "unresolvable overlap"
// branch inside the existing catalog voice with no new Fluent message.
#[allow(clippy::too_many_arguments)]
fn partition_for_overlap(
    profile: &Profile,
    run: &RunInputs,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
    conflict: &OverlapConflict,
    tagged: &[(usize, Candidate)],
) -> Vec<Diagnostic> {
    let Some(primary) = primaries.iter().find(|p| p.path == conflict.file) else {
        return Vec::new();
    };
    let single = std::slice::from_ref(primary);
    let iso_base_sig = diag_signature(&plan_core(profile, run, single, id, lang));

    let mut best: Option<&(usize, Candidate)> = None;
    for entry in tagged {
        let (ri, cand) = entry;
        let edited = with_rule_match(profile, *ri, &cand.apply);
        let sim = plan_core(&edited, run, single, id, lang);
        if resolves_overlap_without_regression(&sim, conflict, &iso_base_sig)
            && best.is_none_or(|(_, b)| cand.rank < b.rank)
        {
            best = Some(entry);
        }
    }

    match best {
        Some((ri, cand)) => vec![
            Diagnostic::info(DiagCode::SuggestionPartition, format!("tracks[{ri}].match"))
                .with("kind", "group")
                .with("count", "1")
                .with("fix", yaml_fragment(*ri, &cand.apply))
                .with("files", conflict.file.display().to_string()),
        ],
        None => Vec::new(),
    }
}

// The identification a rule reads for one primary: the primary itself for a
// keyword source, the single located+identified donor for an external source.
// `None` when no single donor resolves (zero, ambiguous, or unidentifiable),
// mirroring `resolve_file`'s source resolution so candidate generation draws
// discriminators from the same tracks the planner matched against. This is
// what makes the engine source-agnostic (spec 5.3, D6): an external rule's
// ambiguity lives in its donor's tracks, not the primary's.
fn rule_source_ident(
    rule: &TrackRule,
    primary: &PrimaryFile,
    id: &mut dyn Identify,
) -> Option<Identification> {
    match &rule.source {
        SourceCfg::Keyword(_) => id.identify(&primary.path).ok(),
        SourceCfg::External(block) => {
            let primary_dir = primary.path.parent().unwrap_or(Path::new("."));
            let hits =
                discovery::resolve_locator(&block.external, primary_dir, &primary.identifier);
            match hits.as_slice() {
                [donor] => id.identify(donor).ok(),
                _ => None,
            }
        }
    }
}

// Discriminator candidates for a rule, drawn from the property vectors of the
// tracks it matches across every affected file. `mode` selects the seed
// contract (see [`SeedMode`]): `Ambiguous` needs a >=2 matched set and both
// polarities; `Overlap` seeds from the single shared track and NOT-polarity
// only.
fn candidates_for_rule(
    profile: &Profile,
    ri: usize,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
    mode: SeedMode,
) -> Vec<Candidate> {
    let rule = &profile.tracks.rules[ri];
    let mut raw: Vec<Candidate> = Vec::new();
    let mut seen: std::collections::BTreeSet<(String, String, u8)> =
        std::collections::BTreeSet::new();

    for primary in primaries {
        let Some(ident) = rule_source_ident(rule, primary, id) else {
            continue;
        };
        let matched: Vec<&crate::identify::Track> = ident
            .tracks
            .iter()
            .filter(|t| matcher::matches(&rule.match_expr, t, lang))
            .collect();
        if matched.len() < mode.min_matched() {
            continue;
        }
        for t in &matched {
            // Own the property list, including the top-level `type`/`codec`/
            // `id` pseudo-props (spec 4.4 flattens these over `properties`;
            // R1 iv makes `codec` and `id` discriminator dimensions too).
            let mut props: Vec<(String, crate::identify::PropValue)> = t
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            props.push((
                "type".to_string(),
                crate::identify::PropValue::Str(t.kind.clone()),
            ));
            props.push((
                "codec".to_string(),
                crate::identify::PropValue::Str(t.codec.clone()),
            ));
            props.push((
                "id".to_string(),
                crate::identify::PropValue::Int(t.id as i64),
            ));
            for (prop, val) in &props {
                if crate::capability::matchable_type(prop).is_none() {
                    continue;
                }
                let Some((display, scalar)) = prop_value_as(val) else {
                    continue;
                };
                for (polarity, edit) in [
                    (
                        0u8,
                        StructuredEdit::AddExact {
                            property: prop.clone(),
                            value: display.clone(),
                        },
                    ),
                    (
                        1u8,
                        StructuredEdit::AddNotExact {
                            property: prop.clone(),
                            value: display.clone(),
                        },
                    ),
                ] {
                    if !mode.keeps_polarity(polarity) {
                        continue;
                    }
                    if seen.insert((prop.clone(), display.clone(), polarity)) {
                        raw.push(Candidate {
                            apply: delta_for(&edit, &scalar),
                            rank: (rank_of(prop, polarity), prop.clone(), display.clone()),
                            edit,
                        });
                    }
                }
            }
            if let Some(crate::identify::PropValue::Str(name)) = t.get("track_name") {
                for tok in name.split_whitespace() {
                    for (polarity, edit) in [
                        (
                            0u8,
                            StructuredEdit::AddSubstring {
                                value: tok.to_string(),
                            },
                        ),
                        (
                            1u8,
                            StructuredEdit::AddNotSubstring {
                                value: tok.to_string(),
                            },
                        ),
                    ] {
                        if !mode.keeps_polarity(polarity) {
                            continue;
                        }
                        let key = ("track_name~".to_string(), tok.to_string(), polarity);
                        if seen.insert(key) {
                            raw.push(Candidate {
                                apply: delta_for(&edit, &Scalar::Str(tok.to_string())),
                                rank: (
                                    rank_substring(polarity),
                                    "track_name".into(),
                                    tok.to_string(),
                                ),
                                edit,
                            });
                        }
                    }
                }
            }
        }
    }
    raw
}

// Builds the MatchExpr delta a candidate edit represents.
fn delta_for(edit: &StructuredEdit, scalar: &Scalar) -> MatchExpr {
    let mut m = MatchExpr::default();
    match edit {
        StructuredEdit::AddExact { property, .. } => {
            let mut map = BTreeMap::new();
            map.insert(property.clone(), scalar.clone());
            m.exact = Some(map);
        }
        StructuredEdit::AddNotExact { property, .. } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert(property.clone(), scalar.clone());
            inner.exact = Some(map);
            m.not = Some(vec![inner]);
        }
        StructuredEdit::AddSubstring { value } => {
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            m.substring = Some(map);
        }
        StructuredEdit::AddNotSubstring { value } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            inner.substring = Some(map);
            m.not = Some(vec![inner]);
        }
    }
    m
}

// Merges a delta into rule `ri`'s match expression, returning an edited
// profile. `exact`/`substring` use insert-only-if-absent semantics (bug C):
// a candidate whose key already exists on the rule must never overwrite the
// existing constraint, since a suggestion may only narrow a match, never
// relax it (D6). A key collision then makes the delta a no-op for that key,
// which `resolves_without_regression` correctly rejects for not resolving
// the ambiguity. `not` entries are always additive (appending a not-clause
// always narrows, never relaxes), so plain `extend` stays correct there.
fn with_rule_match(profile: &Profile, ri: usize, delta: &MatchExpr) -> Profile {
    let mut p = profile.clone();
    let expr = &mut p.tracks.rules[ri].match_expr;
    if let Some(add) = &delta.exact {
        let map = expr.exact.get_or_insert_with(BTreeMap::new);
        for (k, v) in add {
            map.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
    if let Some(add) = &delta.substring {
        let map = expr.substring.get_or_insert_with(BTreeMap::new);
        for (k, v) in add {
            map.entry(k.clone()).or_insert_with(|| v.clone());
        }
    }
    if let Some(add) = &delta.not {
        expr.not.get_or_insert_with(Vec::new).extend(add.clone());
    }
    p
}

// Accept iff rule `ri` has no AmbiguousRule anywhere in the simulation and no
// diagnostic in the simulation is absent from the baseline (no regression).
// The signature is a multiset (R1 v): a candidate that introduces a SECOND
// copy of an already-present (code, config_path, file) diagnostic is a
// regression, so containment compares counts, not mere membership.
fn resolves_without_regression(
    sim: &Batch,
    ri: usize,
    base_sig: &std::collections::BTreeMap<String, usize>,
) -> bool {
    let still_ambiguous = sim
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .any(|d| d.code == DiagCode::AmbiguousRule && rule_index_of(&d.config_path) == Some(ri));
    if still_ambiguous {
        return false;
    }
    no_regression(sim, base_sig)
}

// The "nothing new anywhere" half of the acceptance criterion (D6 (b), R1 v),
// shared by the AmbiguousRule and OverlappingRules paths: no diagnostic
// signature in the simulation exceeds its baseline count. Multiset-valued, so
// a candidate that introduces a SECOND copy of an already-present diagnostic
// (a new MissingTrack, a new EmptyPlan, a new OverlappingRules) is rejected
// even when that signature already existed once. Code-agnostic: it needs no
// per-code special case, which is exactly why generalizing the "target
// conflict gone" half to overlaps needed no change here.
fn no_regression(sim: &Batch, base_sig: &std::collections::BTreeMap<String, usize>) -> bool {
    diag_signature(sim)
        .iter()
        .all(|(sig, count)| base_sig.get(sig).is_some_and(|base| count <= base))
}

// A single overlap conflict instance, distilled from an `OverlappingRules`
// diagnostic (D33): the file and track it is on, its claimant rule indices
// (T9's `$rules` list), and the diagnostic's `config_path`. Acceptance and
// partitioning key on `(file, track)` - the identity the diagnostic reports
// and a fix resolves - not on the edited rule index (the diagnostic is filed
// under the lowest claimant, so keying on the edited index would misread
// "resolved" when a different claimant was narrowed).
struct OverlapConflict {
    file: PathBuf,
    track: String,
    claimants: Vec<usize>,
}

// Collects the overlap conflicts from a baseline, one per `OverlappingRules`
// diagnostic. Claimants are parsed back from the rendered `$rules` list
// ("tracks[0], tracks[1], ..."), so every claimant - not just the first pair -
// gets candidates generated for it (symmetric generation, D33).
fn overlap_conflicts(baseline: &Batch) -> Vec<OverlapConflict> {
    baseline
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .filter(|d| d.code == DiagCode::OverlappingRules)
        .filter_map(|d| {
            let file = d.file.clone()?;
            let track = d.params.get("track")?.clone();
            let claimants: Vec<usize> = d
                .params
                .get("rules")
                .map(|r| r.split(',').filter_map(rule_index_of).collect())
                .unwrap_or_default();
            Some(OverlapConflict {
                file,
                track,
                claimants,
            })
        })
        .collect()
}

// Accept iff the targeted overlap instance `(file, track)` is gone from the
// simulation and no diagnostic regressed (D33 touch-point 1). The "gone" half
// keys on the conflict identity, not on `rule_index == edited-rule`: the
// diagnostic is filed under the lowest claimant's `config_path`, so narrowing
// a higher-indexed claimant would otherwise spuriously read "resolved" while
// the diagnostic still sits under the lowest one. For a >=3-claimant overlap,
// dropping one claimant leaves a smaller overlap on the same `(file, track)`,
// which this check correctly still sees as unresolved.
fn resolves_overlap_without_regression(
    sim: &Batch,
    conflict: &OverlapConflict,
    base_sig: &std::collections::BTreeMap<String, usize>,
) -> bool {
    let still_overlapping = sim
        .files
        .iter()
        .flat_map(|f| f.diagnostics.iter())
        .any(|d| {
            d.code == DiagCode::OverlappingRules
                && d.file.as_deref() == Some(conflict.file.as_path())
                && d.params.get("track") == Some(&conflict.track)
        });
    if still_overlapping {
        return false;
    }
    no_regression(sim, base_sig)
}

// A rule's match-domain size across the batch: the total number of tracks it
// matches over every primary's source identification. The D33 rank tiebreak
// between surviving narrowings on DIFFERENT claimant rules is "broader rule
// first" (Policy 2), measured here; ties break on lower rule index (Policy 1).
// On the contested file every claimant matches exactly one track, so breadth
// only discriminates via the rest of the batch - deliberately, since the
// broader rule is the likely accidental over-claimer.
fn rule_breadth(
    profile: &Profile,
    ri: usize,
    primaries: &[PrimaryFile],
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> usize {
    let rule = &profile.tracks.rules[ri];
    primaries
        .iter()
        .filter_map(|p| rule_source_ident(rule, p, id))
        .map(|ident| {
            ident
                .tracks
                .iter()
                .filter(|t| matcher::matches(&rule.match_expr, t, lang))
                .count()
        })
        .sum()
}

// A comparable signature multiset of all diagnostics: (code + config_path +
// file) -> occurrence count. A multiset, not a set, so two diagnostics that
// share a signature but describe different tracks (e.g. two OverlappingRules
// on one rule) stay counted separately (R1 v, D6 acceptance criterion b).
fn diag_signature(batch: &Batch) -> std::collections::BTreeMap<String, usize> {
    let mut counts = std::collections::BTreeMap::new();
    let all = batch
        .batch_diagnostics
        .iter()
        .chain(batch.files.iter().flat_map(|f| f.diagnostics.iter()));
    for d in all {
        let file = d
            .file
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        *counts
            .entry(format!("{}|{}|{}", d.code.key(), d.config_path, file))
            .or_insert(0) += 1;
    }
    counts
}

fn rule_index_of(config_path: &str) -> Option<usize> {
    let start = config_path.find("tracks[")? + "tracks[".len();
    let end = config_path[start..].find(']')? + start;
    config_path[start..end].parse().ok()
}

// Rank: typed flags/booleans (0) < language (1) < other exact (2); positive
// exact before its negation at equal property rank.
fn rank_of(prop: &str, polarity: u8) -> u8 {
    let base = match prop {
        "forced_track"
        | "default_track"
        | "flag_hearing_impaired"
        | "flag_visual_impaired"
        | "flag_commentary"
        | "flag_original"
        | "enabled_track" => 0,
        "language" | "language_ietf" => 1,
        _ => 2,
    };
    base * 2 + polarity
}

fn rank_substring(polarity: u8) -> u8 {
    6 + polarity
}

fn prop_value_as(v: &crate::identify::PropValue) -> Option<(String, Scalar)> {
    match v {
        crate::identify::PropValue::Bool(b) => Some((b.to_string(), Scalar::Bool(*b))),
        crate::identify::PropValue::Int(i) => Some((i.to_string(), Scalar::Int(*i))),
        crate::identify::PropValue::Str(s) => Some((s.clone(), Scalar::Str(s.clone()))),
        crate::identify::PropValue::Float(_) => None,
    }
}

// Wrapper so the fragment renders as `match: <expr>`, the shape the CLI/GUI
// splice into a rule's `match:` key (spec 5.3, D6). The field cannot be
// named `match` (reserved keyword), so the wire name is set via `rename`.
#[derive(Serialize)]
struct MatchFragment<'a> {
    #[serde(rename = "match")]
    expr: &'a MatchExpr,
}

// Renders a suggestion's delta as a YAML fragment (bug D). Hand-formatting a
// value into a string template breaks for any value containing a
// YAML-significant character (`:`, `,`, `{`, `}`; e.g. a track_name like
// "Chapter 1: Intro"). Serializing the actual `MatchExpr` delta through
// yaml_serde instead guarantees valid, round-trippable YAML for every
// [`Scalar`] variant, since the serializer -- not this function -- decides
// quoting.
fn yaml_fragment(ri: usize, delta: &MatchExpr) -> String {
    let body = yaml_serde::to_string(&MatchFragment { expr: delta })
        .expect("a MatchExpr delta always serializes to YAML");
    format!("# tracks[{ri}] - add:\n{body}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn batch_with(diags: Vec<Diagnostic>) -> Batch {
        Batch {
            files: vec![FileReport {
                source: PathBuf::from("Show.S01E01.mkv"),
                identifier: "S01E01".to_string(),
                plan: None,
                diagnostics: diags,
            }],
            batch_diagnostics: vec![],
            suggestions: vec![],
        }
    }

    // R1 v: two diagnostics sharing (code, config_path, file) but differing
    // only in a param (here `track`) are distinct occurrences. A set-valued
    // signature collapses them, so a candidate that introduces a SECOND copy
    // of a pre-existing diagnostic reads as no regression and is wrongly
    // accepted. The signature must be a multiset.
    #[test]
    fn duplicate_signature_diagnostic_is_a_regression_not_a_collapse() {
        let one = || {
            Diagnostic::error(DiagCode::OverlappingRules, "tracks[1]")
                .for_file("Show.S01E01.mkv")
                .with("track", "5")
        };
        let two = || {
            Diagnostic::error(DiagCode::OverlappingRules, "tracks[1]")
                .for_file("Show.S01E01.mkv")
                .with("track", "6")
        };

        let base = batch_with(vec![one()]);
        let base_sig = diag_signature(&base);
        // The simulation grew a second overlap with the same signature; rule 0
        // carries no AmbiguousRule, so only the multiset check decides.
        let sim = batch_with(vec![one(), two()]);

        assert!(
            !resolves_without_regression(&sim, 0, &base_sig),
            "a newly duplicated diagnostic must count as a regression"
        );
    }
}
