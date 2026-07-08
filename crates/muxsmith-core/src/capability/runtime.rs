//! Runtime queries against the external mkvmerge (spec 4.4, 9): version,
//! supported file types, and the language table used to normalize match
//! values. Core shells out via `std::process::Command`; it never links
//! mkvmerge. The text-parsing halves are pure and unit-tested; the spawning
//! halves are covered by an integration test gated on a real mkvmerge.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A resolved handle to the local mkvmerge executable.
#[derive(Debug, Clone)]
pub struct Mkvmerge {
    path: PathBuf,
}

/// Failure of a runtime mkvmerge query. Data only; call sites map these to
/// diagnostics (core stays prose-free).
#[derive(Debug, Clone)]
pub enum RuntimeError {
    /// No mkvmerge executable could be located (PATH and override both failed).
    NotFound,
    /// The process could not be spawned; the string is the OS error.
    Spawn(String),
    /// The process ran but exited non-zero.
    NonZero {
        /// Exit code, or `None` if the process was terminated by a signal.
        code: Option<i32>,
        /// Captured stderr, trimmed.
        stderr: String,
    },
    /// The output could not be parsed into the expected shape.
    Parse(String),
}

impl Mkvmerge {
    /// Uses the executable at `path` without searching PATH (an app-settings
    /// or `--mkvmerge` override; spec 8.2). The path is not probed here; the
    /// first query surfaces a `Spawn`/`NotFound` error if it is wrong.
    pub fn at(path: impl Into<PathBuf>) -> Mkvmerge {
        Mkvmerge { path: path.into() }
    }

    /// Locates mkvmerge on PATH by spawning `mkvmerge --version`. Returns
    /// `NotFound` if the spawn fails with a not-found OS error. Platform-
    /// standard install-location probing (spec 8.2) is a GUI/first-run concern
    /// deferred to Plan 4; the CLI relies on PATH plus the explicit override.
    pub fn locate() -> Result<Mkvmerge, RuntimeError> {
        let m = Mkvmerge {
            path: PathBuf::from("mkvmerge"),
        };
        match m.version() {
            Ok(_) => Ok(m),
            Err(RuntimeError::Spawn(_)) => Err(RuntimeError::NotFound),
            Err(e) => Err(e),
        }
    }

    /// The resolved executable path (PATH-relative `mkvmerge` or an override).
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn run(&self, args: &[&str]) -> Result<String, RuntimeError> {
        let out = Command::new(&self.path)
            .args(args)
            .output()
            .map_err(|e| RuntimeError::Spawn(e.to_string()))?;
        if !out.status.success() {
            return Err(RuntimeError::NonZero {
                code: out.status.code(),
                stderr: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            });
        }
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    }

    /// The raw first line of `mkvmerge --version`.
    pub fn version(&self) -> Result<String, RuntimeError> {
        let out = self.run(&["--version"])?;
        Ok(out.lines().next().unwrap_or("").trim().to_string())
    }

    /// Lowercase source-file extensions the local mkvmerge accepts, from
    /// `--list-types`, deduped and sorted (spec 4.2 validation input).
    pub fn list_types(&self) -> Result<Vec<String>, RuntimeError> {
        Ok(parse_list_types(&self.run(&["--list-types"])?))
    }

    /// The language normalization index from `--list-languages` (spec 4.4).
    pub fn list_languages(&self) -> Result<LanguageIndex, RuntimeError> {
        Ok(parse_list_languages(&self.run(&["--list-languages"])?))
    }

    /// Runs `mkvmerge -J <file>` and returns the raw JSON stdout (spec 5.5).
    /// mkvmerge exits 0 for a non-media file too (with
    /// `container.recognized: false`), so a non-zero exit here is a genuine
    /// invocation failure, not a "not media" signal.
    pub fn identify_json(&self, file: &Path) -> Result<String, RuntimeError> {
        let file = file.to_str().ok_or_else(|| {
            RuntimeError::Parse("non-UTF-8 path cannot be passed to mkvmerge".into())
        })?;
        self.run(&["-J", file])
    }
}

/// Extracts the bracketed extension lists from `mkvmerge --list-types` output.
/// Each supported-type line ends with `[ext1 ext2 ...]`; every token is
/// collected, lowercased, deduped, and sorted.
pub fn parse_list_types(output: &str) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    for line in output.lines() {
        let (Some(open), Some(close)) = (line.rfind('['), line.rfind(']')) else {
            continue;
        };
        if close <= open + 1 {
            continue;
        }
        for tok in line[open + 1..close].split_whitespace() {
            exts.push(tok.to_ascii_lowercase());
        }
    }
    exts.sort();
    exts.dedup();
    exts
}

/// A language-token normalizer built from `mkvmerge --list-languages`. Maps any
/// recognized ISO 639-1/639-2/639-3 token (case-insensitive) to a single
/// canonical key (the 639-3 code, always present upstream), so that profile
/// values like `de` and file values like `ger` compare equal.
#[derive(Debug, Clone, Default)]
pub struct LanguageIndex {
    /// lowercased token -> canonical key.
    to_canonical: BTreeMap<String, String>,
}

impl LanguageIndex {
    /// Builds an index from `[english_name, iso639_3, iso639_2, iso639_1]`
    /// rows (blank cells allowed). The canonical key is the 639-3 code, or the
    /// first non-empty code if 639-3 is blank. Every non-empty code in the row
    /// maps to that key.
    pub fn from_rows(rows: &[[&str; 4]]) -> LanguageIndex {
        let mut to_canonical = BTreeMap::new();
        for row in rows {
            let codes = [row[1], row[2], row[3]]; // 639-3, 639-2, 639-1
            let canonical = codes.iter().map(|c| c.trim()).find(|c| !c.is_empty());
            let Some(canonical) = canonical else { continue };
            let canonical = canonical.to_ascii_lowercase();
            for code in codes {
                let code = code.trim();
                if !code.is_empty() {
                    to_canonical.insert(code.to_ascii_lowercase(), canonical.clone());
                }
            }
        }
        LanguageIndex { to_canonical }
    }

    /// The canonical key for `token`, or `None` if unrecognized. Matching is
    /// case-insensitive. Two tokens are the same language iff their canonical
    /// keys are equal.
    pub fn normalize(&self, token: &str) -> Option<String> {
        self.to_canonical
            .get(&token.trim().to_ascii_lowercase())
            .cloned()
    }
}

/// Parses `mkvmerge --list-languages` table output into a [`LanguageIndex`].
/// The table is `name | 639-3 | 639-2 | 639-1`, pipe-separated, with a header
/// row and a `---+---` separator row that are skipped.
pub fn parse_list_languages(output: &str) -> LanguageIndex {
    let mut owned: Vec<[String; 4]> = Vec::new();
    for line in output.lines() {
        if !line.contains('|') {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 4 {
            continue;
        }
        // Skip the header (contains "639") and the separator (dashes/plus).
        if cols[1].contains("639") || cols[0].chars().all(|c| c == '-' || c == '+' || c == ' ') {
            continue;
        }
        owned.push([
            cols[0].to_string(),
            cols[1].to_string(),
            cols[2].to_string(),
            cols[3].to_string(),
        ]);
    }
    let rows: Vec<[&str; 4]> = owned
        .iter()
        .map(|r| [r[0].as_str(), r[1].as_str(), r[2].as_str(), r[3].as_str()])
        .collect();
    LanguageIndex::from_rows(&rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_list_types_extensions() {
        let sample = "Supported file types:\n  \
            Dolby Digital/Dolby Digital Plus (AC-3, E-AC-3) [ac3 eac3 eb3 ec3]\n  \
            AAC (Advanced Audio Coding) [aac m4a mp4]\n  \
            Matroska [mkv mka mks mk3d webm]\n";
        let exts = parse_list_types(sample);
        assert!(exts.contains(&"mkv".to_string()));
        assert!(exts.contains(&"ac3".to_string()));
        assert!(exts.contains(&"mp4".to_string()));
        let mut sorted = exts.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(exts, sorted);
    }

    #[test]
    fn parses_list_languages_into_normalizer() {
        let sample = "\
English language name | ISO 639-3 code | ISO 639-2 code | ISO 639-1 code\n\
----------------------+----------------+----------------+---------------\n\
English               | eng            | eng            | en\n\
German                | ger            | ger            | de\n\
Klingon               | tlh            |                |   \n";
        let idx = parse_list_languages(sample);
        let en = idx.normalize("en");
        assert!(en.is_some());
        assert_eq!(idx.normalize("eng"), en);
        assert_eq!(idx.normalize("EN"), en);
        let de = idx.normalize("de");
        assert_eq!(idx.normalize("ger"), de);
        assert_ne!(en, de);
        assert!(idx.normalize("tlh").is_some());
        assert_eq!(idx.normalize("zz-not-a-lang"), None);
    }

    #[test]
    fn language_index_from_rows_builds_directly() {
        let idx = LanguageIndex::from_rows(&[["English", "eng", "eng", "en"]]);
        assert_eq!(idx.normalize("en"), idx.normalize("eng"));
    }
}
