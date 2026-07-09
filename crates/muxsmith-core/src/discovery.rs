//! Source-tree discovery (spec 3, 4.2, 4.6): find primary files by extension
//! and `input.pattern`, extract their identifiers, and resolve external donor
//! candidates for a locator. Filesystem-facing but diagnostic-light: only the
//! file-independent facts (ignored files, duplicate identifiers, repeated
//! pattern matches) are emitted here; per-rule external diagnostics belong to
//! the planner, which knows `optional` and the primaries set.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::profile::model::{Input, Locator};
use crate::report::{DiagCode, Diagnostic};
use crate::template::{Ctx, Template};

/// A primary file's identifier (spec 3): the substring matched by
/// `input.pattern` plus its capture groups, feeding template fields and
/// external-file matching.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// The whole matched substring (`{match}`), capture group 0.
    pub whole: String,
    /// Named and numbered capture groups (`season`, `g1`, ...); numbered groups
    /// are keyed `g1`, `g2`, ... and named groups by their name.
    pub groups: BTreeMap<String, String>,
}

impl Identifier {
    /// A template render context binding `{match}` to `whole` and every capture
    /// group to its value (spec 4.7).
    pub fn to_ctx(&self) -> Ctx {
        let mut ctx = Ctx::new();
        ctx.set("match", self.whole.clone());
        for (k, v) in &self.groups {
            ctx.set(k.clone(), v.clone());
        }
        ctx
    }
}

/// A discovered primary file and its identifier (spec 3).
#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryFile {
    /// Absolute or source-relative path to the file.
    pub path: PathBuf,
    /// The identifier extracted from its basename.
    pub identifier: Identifier,
}

/// Walks `source` and returns the primary files plus file-independent
/// diagnostics (spec 4.2). A file whose extension is in `input.extensions`
/// (case-insensitive) but whose basename does not match `input.pattern` is an
/// `IgnoredFile` (info); more than one pattern match in a basename is
/// `MultipleIdentifierMatches` (info, first match used); two primaries sharing
/// an identifier is `DuplicateIdentifier` (warning).
pub fn scan_primaries(source: &Path, input: &Input) -> (Vec<PrimaryFile>, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let re = match regex::Regex::new(&input.pattern) {
        Ok(re) => re,
        Err(_) => return (Vec::new(), diags), // validate already reported InvalidRegex
    };
    let exts: Vec<String> = input
        .extensions
        .iter()
        .map(|e| e.to_ascii_lowercase())
        .collect();

    let mut primaries = Vec::new();
    for path in walk_files(source, input.recursive) {
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        if !extension_matches(&path, &exts) {
            continue; // not a candidate at all; silent
        }
        let mut matches = re.find_iter(name);
        let Some(first) = matches.next() else {
            diags.push(Diagnostic::info(DiagCode::IgnoredFile, "input.pattern").for_file(&path));
            continue;
        };
        if matches.next().is_some() {
            diags.push(
                Diagnostic::info(DiagCode::MultipleIdentifierMatches, "input.pattern")
                    .for_file(&path)
                    .with("name", name),
            );
        }
        let caps = re.captures(name).expect("first match implies captures");
        let mut groups = BTreeMap::new();
        for (i, opt_name) in re.capture_names().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(m) = caps.get(i) {
                groups.insert(format!("g{i}"), m.as_str().to_string());
                if let Some(n) = opt_name {
                    groups.insert(n.to_string(), m.as_str().to_string());
                }
            }
        }
        primaries.push(PrimaryFile {
            path: path.clone(),
            identifier: Identifier {
                whole: first.as_str().to_string(),
                groups,
            },
        });
    }

    // DuplicateIdentifier across the batch (spec 5.2).
    let mut by_id: BTreeMap<&str, Vec<&PrimaryFile>> = BTreeMap::new();
    for p in &primaries {
        by_id
            .entry(p.identifier.whole.as_str())
            .or_default()
            .push(p);
    }
    for (id, group) in &by_id {
        if group.len() >= 2 {
            diags.push(
                Diagnostic::warning(DiagCode::DuplicateIdentifier, "input.pattern")
                    .with("identifier", *id)
                    .with("file_a", group[0].path.display().to_string())
                    .with("file_b", group[1].path.display().to_string()),
            );
        }
    }

    (primaries, diags)
}

/// Candidate donor files for a locator (spec 4.6): files under the locator's
/// directory (relative to `primary_dir`, or absolute) whose extension is in
/// `locator.extensions` and whose basename matches the rendered
/// `match_to_source`/`match_pattern`. Sorted; unreadable directories yield an
/// empty list.
pub fn resolve_locator(
    locator: &Locator,
    primary_dir: &Path,
    identifier: &Identifier,
) -> Vec<PathBuf> {
    let base = if locator.path.is_absolute() {
        locator.path.clone()
    } else {
        primary_dir.join(&locator.path)
    };
    let exts: Vec<String> = locator
        .extensions
        .iter()
        .map(|e| e.to_ascii_lowercase())
        .collect();

    // The basename-matching regex: match_to_source is sugar for the template
    // "{match}" (spec 4.6). A validated template parses; on the off chance it
    // does not, match nothing rather than panic.
    let pattern_src = if matches!(locator.match_to_source, Some(true)) {
        "{match}".to_string()
    } else {
        locator.match_pattern.clone().unwrap_or_default()
    };
    let ctx = identifier.to_ctx();
    let re = Template::parse(&pattern_src)
        .ok()
        .map(|t| t.render_regex_pattern(&ctx, locator.case_sensitive))
        .and_then(|p| regex::Regex::new(&p).ok());

    let mut hits = Vec::new();
    for path in walk_files(&base, locator.recursive) {
        if !extension_matches(&path, &exts) {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };
        let matched = match &re {
            Some(re) => re.is_match(name),
            None => false,
        };
        if matched {
            hits.push(path);
        }
    }
    hits
}

fn extension_matches(path: &Path, exts_lower: &[String]) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(e) => exts_lower.iter().any(|x| x == &e.to_ascii_lowercase()),
        None => false,
    }
}

/// Regular files under `dir`, recursing into subdirectories only if
/// `recursive`. A symlink is resolved via its target's metadata: a symlink to
/// a regular file is included, under its own (link) path; a symlink to a
/// directory is never recursed into, even when `recursive` (cycle guard); a
/// broken symlink (unreadable target) is skipped silently. Sorted for
/// deterministic output; unreadable directories are skipped silently.
fn walk_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        let mut dir_entries: Vec<PathBuf> =
            entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
        dir_entries.sort();
        for path in dir_entries {
            let Ok(meta) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            if meta.is_dir() {
                if recursive {
                    stack.push(path);
                }
            } else if meta.is_file() {
                out.push(path);
            } else if meta.file_type().is_symlink() {
                let Ok(target_meta) = std::fs::metadata(&path) else {
                    continue; // broken symlink; skip silently
                };
                if target_meta.is_file() {
                    out.push(path);
                }
                // A directory target is never recursed into (cycle guard).
            }
        }
    }
    out.sort();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::model::{Input, Locator};
    use std::fs;

    fn input(pattern: &str, exts: &[&str], recursive: bool) -> Input {
        Input {
            pattern: pattern.to_string(),
            extensions: exts.iter().map(|s| s.to_string()).collect(),
            recursive,
        }
    }

    #[test]
    fn scans_primaries_and_extracts_named_groups() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E02.mkv"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E03.MP4"), b"x").unwrap();
        fs::write(dir.path().join("readme.txt"), b"x").unwrap();
        fs::write(dir.path().join("notes.mkv"), b"x").unwrap();

        let (primaries, diags) = scan_primaries(
            dir.path(),
            &input(
                r"S(?<season>\d{2})E(?<episode>\d{2})",
                &["mkv", "mp4"],
                true,
            ),
        );
        assert_eq!(primaries.len(), 2);
        let e02 = primaries
            .iter()
            .find(|p| p.identifier.whole == "S01E02")
            .expect("S01E02 primary");
        assert_eq!(e02.identifier.groups["season"], "01");
        assert_eq!(e02.identifier.groups["episode"], "02");
        assert_eq!(e02.identifier.groups["g1"], "01");
        assert!(diags.iter().any(|d| d.code == DiagCode::IgnoredFile));
    }

    #[test]
    fn duplicate_identifier_is_warned() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E01.720p.mkv"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E01.1080p.mkv"), b"x").unwrap();
        let (primaries, diags) = scan_primaries(
            dir.path(),
            &input(r"S(?<season>\d{2})E(?<episode>\d{2})", &["mkv"], true),
        );
        assert_eq!(primaries.len(), 2);
        assert!(
            diags
                .iter()
                .any(|d| d.code == DiagCode::DuplicateIdentifier)
        );
    }

    #[test]
    fn multiple_identifier_matches_uses_first() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("E01.E02.mkv"), b"x").unwrap();
        let (primaries, diags) = scan_primaries(dir.path(), &input(r"E(\d{2})", &["mkv"], true));
        assert_eq!(primaries.len(), 1);
        assert_eq!(primaries[0].identifier.whole, "E01");
        assert!(
            diags
                .iter()
                .any(|d| d.code == DiagCode::MultipleIdentifierMatches)
        );
    }

    #[test]
    fn non_recursive_scan_skips_subdirs() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub").join("E01.mkv"), b"x").unwrap();
        fs::write(dir.path().join("E02.mkv"), b"x").unwrap();
        let (primaries, _) = scan_primaries(dir.path(), &input(r"E(\d{2})", &["mkv"], false));
        assert_eq!(primaries.len(), 1);
        assert_eq!(primaries[0].identifier.whole, "E02");
    }

    #[cfg(unix)]
    #[test]
    fn discovers_symlinked_primary_file() {
        use std::os::unix::fs::symlink;

        // Real target lives outside the scanned tree entirely.
        let target_dir = tempfile::tempdir().unwrap();
        let target = target_dir.path().join("Show.S01E05.mkv");
        fs::write(&target, b"x").unwrap();

        let source_dir = tempfile::tempdir().unwrap();
        let link = source_dir.path().join("Show.S01E05.mkv");
        symlink(&target, &link).unwrap();

        let (primaries, diags) = scan_primaries(
            source_dir.path(),
            &input(r"S(?<season>\d{2})E(?<episode>\d{2})", &["mkv"], true),
        );
        assert_eq!(primaries.len(), 1, "diags: {diags:?}");
        assert_eq!(primaries[0].path, link);
        assert_eq!(primaries[0].identifier.whole, "S01E05");
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_directory_is_not_recursed_into() {
        use std::os::unix::fs::symlink;

        // A directory outside the scanned tree, containing a file that would
        // match if (and only if) the symlink below were followed.
        let real_dir = tempfile::tempdir().unwrap();
        fs::write(real_dir.path().join("E09.mkv"), b"x").unwrap();

        let source_dir = tempfile::tempdir().unwrap();
        symlink(real_dir.path(), source_dir.path().join("linked_sub")).unwrap();
        fs::write(source_dir.path().join("E01.mkv"), b"x").unwrap();

        let (primaries, _) = scan_primaries(source_dir.path(), &input(r"E(\d{2})", &["mkv"], true));
        assert_eq!(primaries.len(), 1);
        assert_eq!(primaries[0].identifier.whole, "E01");
    }

    #[test]
    fn resolve_locator_matches_by_identifier() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("Show.S01E01.srt"), b"x").unwrap();
        fs::write(dir.path().join("Show.S01E02.srt"), b"x").unwrap();
        let ident = Identifier {
            whole: "S01E01".to_string(),
            groups: Default::default(),
        };
        let locator = Locator {
            path: ".".into(),
            recursive: false,
            extensions: vec!["srt".into()],
            match_to_source: Some(true),
            match_pattern: None,
            case_sensitive: false,
        };
        let hits = resolve_locator(&locator, dir.path(), &ident);
        assert_eq!(hits.len(), 1);
        assert!(hits[0].ends_with("Show.S01E01.srt"));
    }
}
