//! Batch planning (spec 5): resolve every track rule against each primary
//! file's tracks (and located donors) under strict independent uniqueness,
//! render output paths, and collect diagnostics into a batch report. No
//! filesystem mutation and no mux invocations (dry-run, spec 5.5); the only
//! external work is identification, driven through the injected [`Identify`].

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION;
use crate::capability::runtime::LanguageIndex;
use crate::discovery::{self, PrimaryFile};
use crate::identify::{Identification, Identify};
use crate::matcher;
use crate::profile::match_expr::{MatchExpr, Scalar};
use crate::profile::model::{CollisionPolicy, FilenameCfg, Profile, SourceCfg};
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
    /// Index into `profile.tracks`.
    pub rule_index: usize,
    /// The source file the track comes from (the primary, or a donor).
    pub source: PathBuf,
    /// The resolved `-J` track id, or `None` for an unmatched optional rule.
    pub track_id: Option<u64>,
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

    let primary_paths: Vec<PathBuf> = primaries.iter().map(|p| p.path.clone()).collect();
    let output_dir = run
        .output
        .clone()
        .or_else(|| profile.output.directory.clone())
        .unwrap_or_else(|| run.source.clone());
    let policy = run.on_collision.unwrap_or(profile.output.on_collision);

    let mut files: Vec<FileReport> = Vec::new();
    for primary in primaries {
        files.push(resolve_file(
            profile,
            primary,
            &primary_paths,
            &output_dir,
            id,
            lang,
        ));
    }

    // Drop plans with resolution errors first, so collision detection only
    // considers files that will actually produce output; then re-drop any plan
    // that a collision error just invalidated.
    finalize_plans(&mut files);
    detect_output_collisions(&mut files, policy);
    finalize_plans(&mut files);

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
    batch.suggestions = suggest(profile, run, &primaries, id, lang, &batch);
    batch
}

fn validate_language_values(profile: &Profile, lang: &LanguageIndex, diags: &mut Vec<Diagnostic>) {
    for (i, rule) in profile.tracks.iter().enumerate() {
        walk_exact_languages(&rule.match_expr, &format!("tracks[{i}].match"), lang, diags);
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
        && lang.normalize(v).is_none()
    {
        diags.push(
            Diagnostic::error(DiagCode::InvalidPropertyValue, format!("{path}.exact.language"))
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

fn resolve_file(
    profile: &Profile,
    primary: &PrimaryFile,
    primary_paths: &[PathBuf],
    output_dir: &Path,
    id: &mut dyn Identify,
    lang: &LanguageIndex,
) -> FileReport {
    let mut diagnostics = Vec::new();
    let primary_dir = primary.path.parent().unwrap_or(Path::new("."));

    let ident = match id.identify(&primary.path) {
        Ok(i) => i,
        Err(_) => {
            diagnostics.push(
                Diagnostic::error(DiagCode::MissingTrack, "input")
                    .for_file(&primary.path)
                    .with("detail", "file could not be identified"),
            );
            return FileReport {
                source: primary.path.clone(),
                identifier: primary.identifier.whole.clone(),
                plan: None,
                diagnostics,
            };
        }
    };
    if ident.format_version > PINNED_IDENTIFICATION_FORMAT_VERSION {
        diagnostics.push(
            Diagnostic::warning(DiagCode::UnknownPropertySkew, "input")
                .for_file(&primary.path)
                .with("version", ident.format_version.to_string()),
        );
    }

    let mut assignments = Vec::new();
    // (source_path, track_id) -> rule indices claiming it, for overlap checks.
    let mut claims: BTreeMap<(PathBuf, u64), Vec<usize>> = BTreeMap::new();

    for (ri, rule) in profile.tracks.iter().enumerate() {
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
                            Ok(di) => (donor, di),
                            Err(_) => {
                                diagnostics.push(
                                    Diagnostic::error(
                                        DiagCode::MissingExternal,
                                        format!("{base}.source.external"),
                                    )
                                    .for_file(&primary.path)
                                    .with("detail", "donor could not be identified"),
                                );
                                assignments.push(Assignment {
                                    rule_index: ri,
                                    source: primary.path.clone(),
                                    track_id: None,
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
                        });
                        continue;
                    }
                }
            }
        };

        let matched: Vec<u64> = source_ident
            .tracks
            .iter()
            .filter(|t| matcher::matches(&rule.match_expr, t, lang))
            .map(|t| t.id)
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
                });
            }
            1 => {
                let tid = matched[0];
                claims
                    .entry((source_path.clone(), tid))
                    .or_default()
                    .push(ri);
                assignments.push(Assignment {
                    rule_index: ri,
                    source: source_path,
                    track_id: Some(tid),
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
                });
            }
        }
    }

    // OverlappingRules: one track claimed by two or more rules (spec 5.2).
    for ((_src, tid), rules) in &claims {
        if rules.len() >= 2 {
            diagnostics.push(
                Diagnostic::error(DiagCode::OverlappingRules, format!("tracks[{}]", rules[0]))
                    .for_file(&primary.path)
                    .with("rule_a", format!("tracks[{}]", rules[0]))
                    .with("rule_b", format!("tracks[{}]", rules[1]))
                    .with("track", tid.to_string()),
            );
        }
    }

    let output = render_output(profile, primary, output_dir, &mut diagnostics);

    if let Some(out) = &output
        && (primary_paths.contains(out) || out == &primary.path)
    {
        diagnostics.push(
            Diagnostic::error(DiagCode::SourceOverwrite, "output")
                .for_file(&primary.path)
                .with("path", out.display().to_string()),
        );
    }

    let plan = output.map(|output| Plan {
        source: primary.path.clone(),
        output,
        assignments,
    });

    FileReport {
        source: primary.path.clone(),
        identifier: primary.identifier.whole.clone(),
        plan,
        diagnostics,
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
    let name = match &profile.output.filename {
        FilenameCfg::Keyword(_) => {
            let stem = primary
                .path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            format!("{stem}.mkv")
        }
        FilenameCfg::Template(block) => {
            let mut ctx = primary.identifier.to_ctx();
            if let Some(stem) = primary.path.file_stem().and_then(|s| s.to_str()) {
                ctx.set("source_stem", stem);
            }
            let mut rendered = Template::parse(&block.template)
                .map(|t| t.render_literal(&ctx))
                .unwrap_or_default();
            if !rendered.to_lowercase().ends_with(".mkv") {
                rendered.push_str(".mkv");
            }
            rendered
        }
    };

    if name.contains('/') || name.contains('\\') {
        diags.push(
            Diagnostic::error(DiagCode::PathSeparatorInRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", name.clone()),
        );
        return None;
    }
    let stem_only = name
        .strip_suffix(".mkv")
        .or_else(|| name.strip_suffix(".MKV"))
        .unwrap_or(&name);
    if stem_only.is_empty() || name == "." || name == ".." {
        diags.push(
            Diagnostic::error(DiagCode::EmptyRenderedName, "output.filename")
                .for_file(&primary.path)
                .with("name", name.clone()),
        );
        return None;
    }

    Some(output_dir.join(name))
}

// Two plans rendering to the same output path collide, as does an existing
// on-disk file (spec 4.8). Only surviving plans (post-finalize) are considered.
fn detect_output_collisions(files: &mut [FileReport], policy: CollisionPolicy) {
    let mut counts: BTreeMap<PathBuf, usize> = BTreeMap::new();
    for f in files.iter() {
        if let Some(p) = &f.plan {
            *counts.entry(p.output.clone()).or_default() += 1;
        }
    }
    for f in files.iter_mut() {
        let Some(plan) = &f.plan else { continue };
        let out = plan.output.clone();
        let planned_twice = counts.get(&out).copied().unwrap_or(0) >= 2;
        let exists_on_disk = out.exists();
        if !planned_twice && !exists_on_disk {
            continue;
        }
        // Two planned outputs to one path: always an error unless Skip. An
        // existing on-disk file: severity follows the policy.
        let severity = if planned_twice {
            match policy {
                CollisionPolicy::Skip => Severity::Warning,
                _ => Severity::Error,
            }
        } else {
            match policy {
                CollisionPolicy::Error => Severity::Error,
                CollisionPolicy::Skip => Severity::Warning,
                CollisionPolicy::Overwrite => Severity::Info,
            }
        };
        let mut d = Diagnostic::info(DiagCode::OutputCollision, "output")
            .for_file(&plan.source)
            .with("path", out.display().to_string());
        d.severity = severity;
        f.diagnostics.push(d);
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

// The suggestion engine is added in the next commit; until then it emits none.
#[allow(clippy::too_many_arguments)]
fn suggest(
    _profile: &Profile,
    _run: &RunInputs,
    _primaries: &[PrimaryFile],
    _id: &mut dyn Identify,
    _lang: &LanguageIndex,
    _baseline: &Batch,
) -> Vec<Suggestion> {
    Vec::new()
}
