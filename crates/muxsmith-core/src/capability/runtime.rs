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
    /// The `(major, minor)` pair already parsed by [`Mkvmerge::detect`]'s
    /// floor check (`enforce_floor`), cached so a caller that follows
    /// `detect` with [`Mkvmerge::version_pair`] -- the GUI's
    /// `detect_mkvmerge` command does exactly that, on every startup --
    /// does not spawn `mkvmerge --version` a second time for a value the
    /// ladder already obtained. `None` on handles from [`Mkvmerge::at`]
    /// and [`Mkvmerge::locate`], whose behavior is unchanged: they never
    /// enforce the floor, so `version_pair` spawns per call there. The
    /// cache is a snapshot from detection time; a caller that needs to
    /// re-probe a possibly-replaced binary uses `at(path)` for a fresh,
    /// uncached handle.
    cached_version_pair: Option<(u64, u64)>,
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
    /// A located mkvmerge answered `--version` below [`MIN_SUPPORTED`].
    TooOld {
        /// The raw `mkvmerge --version` first line that was found.
        found: String,
        /// [`MIN_SUPPORTED`], formatted as `"major.minor"`.
        minimum: String,
    },
}

/// Minimum supported mkvmerge version (D28): the release that introduced
/// identification schema v20, the schema this build's capability table is
/// generated against ([`crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION`],
/// spec 9). Below this floor, `-J` output cannot be trusted to match the
/// generated table.
///
/// Evidence (checked in `~/Downloads/mkvtoolnix` source, not from memory):
/// `src/merge/id_result.h` pins `ID_JSON_FORMAT_VERSION = 20`, matching the
/// schema linked from `doc/man/mkvmerge.xml`
/// (`mkvmerge-identification-output-schema-v20.json`). `NEWS.md` never
/// spells out "bumped to 20" verbatim, so the release is derived from the
/// schema diff: `doc/json-schema/...-v19.json` vs `...-v20.json` differ only
/// in replacing five enumerated `tag_*` track properties with an open
/// `patternProperties: { "^tag_": ... }` (`additionalProperties: true`).
/// `NEWS.md`, "Version 86.0 'Winter' 2024-07-13", records exactly that
/// change: "mkvmerge: Matroska reader: track statistics tags are included
/// in the JSON identification output ... as part of the track properties,
/// prefixed with `tag_`." No schema-affecting entry exists between v82.0
/// (which explicitly bumped to schema v19, `NEWS.md` line ~633-634) and
/// v86.0, so v86.0 is the release that moved the schema from 19 to 20.
pub const MIN_SUPPORTED: (u64, u64) = (86, 0);

impl Mkvmerge {
    /// Uses the executable at `path` without searching PATH (an app-settings
    /// or `--mkvmerge` override; spec 8.2). The path is not probed here; the
    /// first query surfaces a `Spawn`/`NotFound` error if it is wrong.
    pub fn at(path: impl Into<PathBuf>) -> Mkvmerge {
        Mkvmerge {
            path: path.into(),
            cached_version_pair: None,
        }
    }

    /// Locates mkvmerge on PATH by spawning `mkvmerge --version`. Returns
    /// `NotFound` if the spawn fails with a not-found OS error. Does not
    /// check the version floor (spec 8.2); the CLI has relied on PATH plus
    /// the explicit override since Plan 2 and that behavior is unchanged.
    /// [`Mkvmerge::detect`] adds platform-candidate probing and the floor
    /// check on top of this, for the GUI's first-run detection (Plan 5).
    pub fn locate() -> Result<Mkvmerge, RuntimeError> {
        let m = Mkvmerge {
            path: PathBuf::from("mkvmerge"),
            cached_version_pair: None,
        };
        match m.version() {
            Ok(_) => Ok(m),
            Err(RuntimeError::Spawn(_)) => Err(RuntimeError::NotFound),
            Err(e) => Err(e),
        }
    }

    /// Detection ladder for first-run/GUI use (spec 8.2, D28): an explicit
    /// `override_path` (if given) is authoritative, probed with
    /// `--version` and returned or failed outright, PATH and platform
    /// candidates are never consulted. Without an override, tries PATH via
    /// [`Mkvmerge::locate`], then each of [`platform_candidates`] in order;
    /// the first one that answers `--version` with a parseable version wins.
    /// A found mkvmerge below [`MIN_SUPPORTED`] stops the ladder immediately
    /// with `TooOld` rather than being silently skipped in favor of another
    /// candidate: that is real, actionable signal ("upgrade this install"),
    /// not a "not found here" result. Exhausting every rung without finding
    /// any usable mkvmerge is `NotFound`.
    pub fn detect(override_path: Option<&Path>) -> Result<Mkvmerge, RuntimeError> {
        if let Some(path) = override_path {
            return enforce_floor(Mkvmerge::at(path));
        }

        if let Ok(m) = Mkvmerge::locate() {
            match enforce_floor(m) {
                Ok(m) => return Ok(m),
                Err(e @ RuntimeError::TooOld { .. }) => return Err(e),
                Err(_) => {} // nothing usable on PATH; fall through
            }
        }

        for candidate in platform_candidates() {
            match enforce_floor(Mkvmerge::at(candidate)) {
                Ok(m) => return Ok(m),
                Err(e @ RuntimeError::TooOld { .. }) => return Err(e),
                Err(_) => continue,
            }
        }

        Err(RuntimeError::NotFound)
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

    /// The `(major, minor)` version pair parsed from [`Mkvmerge::version`]
    /// (D28), for comparison against [`MIN_SUPPORTED`]. On a handle
    /// returned by [`Mkvmerge::detect`], answers from the pair its floor
    /// check already parsed, spawning nothing (see the field doc on
    /// `cached_version_pair` for the exact contract); on an
    /// [`Mkvmerge::at`]/[`Mkvmerge::locate`] handle, spawns `--version`
    /// per call, unchanged.
    pub fn version_pair(&self) -> Result<(u64, u64), RuntimeError> {
        if let Some(pair) = self.cached_version_pair {
            return Ok(pair);
        }
        parse_version_pair(&self.version()?)
    }

    /// Lowercase source-file extensions the local mkvmerge accepts, from
    /// `--list-types`, deduped and sorted (spec 4.2 validation input).
    pub fn list_types(&self) -> Result<Vec<String>, RuntimeError> {
        Ok(parse_list_types(&self.run(&["--list-types"])?))
    }

    /// [`Mkvmerge::list_types`], degrading a query failure to `None` (spec
    /// 4.2). Unlike identification or [`Mkvmerge::list_languages`], whose
    /// failure blocks planning outright, `profile.input.extensions`
    /// validation is advisory: a missing or broken mkvmerge should not stop
    /// batch planning, only skip the typo check (walkthrough #3).
    pub fn known_extensions(&self) -> Option<Vec<String>> {
        self.list_types().ok()
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

/// Checks `m` against [`MIN_SUPPORTED`] (D28): probes `--version` once, and
/// either returns `m` (with the parsed pair cached on it, so a subsequent
/// [`Mkvmerge::version_pair`] answers without respawning) or a
/// `TooOld`/propagated query error. Shared by every rung of
/// [`Mkvmerge::detect`]'s ladder so the version query is never run twice
/// for the same candidate.
fn enforce_floor(m: Mkvmerge) -> Result<Mkvmerge, RuntimeError> {
    let raw = m.version()?;
    let pair = parse_version_pair(&raw)?;
    if pair < MIN_SUPPORTED {
        return Err(RuntimeError::TooOld {
            found: raw,
            minimum: format!("{}.{}", MIN_SUPPORTED.0, MIN_SUPPORTED.1),
        });
    }
    Ok(Mkvmerge {
        cached_version_pair: Some(pair),
        ..m
    })
}

/// Parses the `(major, minor)` pair out of an `mkvmerge --version` first
/// line, e.g. `"mkvmerge v100.0.0 ('Message') 64-bit"`. The locally
/// installed v100 actually reports the shorter `"mkvmerge v100.0 (...) ..."`
/// (no patch component), so only the first dot-separated component after
/// the leading `v` is required; a missing second component defaults to 0,
/// and any further components (patch, etc.) are ignored.
fn parse_version_pair(raw: &str) -> Result<(u64, u64), RuntimeError> {
    let token = raw
        .split_whitespace()
        .find(|tok| {
            tok.strip_prefix('v')
                .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        })
        .ok_or_else(|| RuntimeError::Parse(format!("no version token in {raw:?}")))?;
    let mut parts = token[1..].split('.');
    let major = parts
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| RuntimeError::Parse(format!("no major version in {raw:?}")))?;
    let minor = parts
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    Ok((major, minor))
}

/// Platform-standard mkvmerge install locations (D28, spec 8.2), probed as
/// the last rung of [`Mkvmerge::detect`] after an override and PATH both
/// come up empty. Verified against mkvtoolnix's own packaging
/// (`~/Downloads/mkvtoolnix/packaging/`) where a location is claimed to be
/// an mkvtoolnix installer/package default; the one exception is Homebrew
/// on macOS (see below), verified against Homebrew's own documentation
/// instead, since Homebrew's mkvtoolnix formula lives in the separate
/// `homebrew-core` repo, outside mkvtoolnix's own source tree.
///
/// - **Homebrew (macOS, `/opt/homebrew/bin`).** Homebrew's own installation
///   docs (<https://docs.brew.sh/Installation>) pin `/opt/homebrew` as the
///   default prefix on Apple Silicon (`/usr/local` on Intel, already listed
///   below). Sharpens why this rung matters specifically for a bundled
///   Tauri app, beyond the general override/PATH/candidate ladder: a GUI
///   app launched from Finder (or Spotlight, the Dock, etc.) does not
///   inherit the invoking shell's PATH the way a terminal-launched process
///   does, so [`Mkvmerge::locate`]'s PATH probe -- which covers a
///   Homebrew-on-Intel install via `/usr/local/bin` being conventionally on
///   PATH -- does not reliably cover an Apple-Silicon Homebrew install at
///   all for this app; this candidate rung is what closes that gap.
/// - Flatpak (`/var/lib/flatpak/exports/bin/org.bunkus.mkvtoolnix-gui`):
///   `packaging/` ships no Flatpak manifest either; that app ID also names
///   the GUI, not a standalone `mkvmerge` CLI binary. Still excluded.
fn platform_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // packaging/windows/installer/mkvtoolnix.nsi: PRODUCT_NAME is
        // "MKVToolNix"; InstallDir is `$PROGRAMFILES64\${PRODUCT_NAME}` for
        // the 64-bit installer target, `$PROGRAMFILES\${PRODUCT_NAME}` for
        // the 32-bit one. On a 64-bit process %ProgramFiles% resolves to the
        // former and %ProgramFiles(x86)% to the latter, so both cover
        // whichever installer variant was actually run.
        for var in ["ProgramFiles", "ProgramFiles(x86)"] {
            if let Ok(pf) = std::env::var(var) {
                candidates.push(PathBuf::from(pf).join("MKVToolNix").join("mkvmerge.exe"));
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // packaging/macos/config.sh: APP_BUNDLE_NAME="MKVToolNix.app" (fixed
        // name, no version suffix). packaging/macos/build.sh's build_dmg
        // places the CLI binaries at Contents/MacOS/{mkvmerge,...} inside
        // that bundle, and its own README.macOS.txt (written by build_dmg,
        // shipped in the DMG) tells users to copy them to /usr/local/bin.
        candidates.push(PathBuf::from(
            "/Applications/MKVToolNix.app/Contents/MacOS/mkvmerge",
        ));
        candidates.push(PathBuf::from("/opt/homebrew/bin/mkvmerge"));
        candidates.push(PathBuf::from("/usr/local/bin/mkvmerge"));
    }

    #[cfg(target_os = "linux")]
    {
        // packaging/debian/mkvtoolnix.install and
        // packaging/centos-fedora-opensuse/mkvtoolnix.spec (%{_bindir}) both
        // place mkvmerge at /usr/bin. mkvtoolnix's own INSTALL (the
        // unmodified generic autotools boilerplate: configure.ac has no
        // AC_PREFIX_DEFAULT override) documents /usr/local as the default
        // `./configure` prefix for a from-source build.
        candidates.push(PathBuf::from("/usr/bin/mkvmerge"));
        candidates.push(PathBuf::from("/usr/local/bin/mkvmerge"));
    }

    candidates
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

    /// Whether `token` is an acceptable language value: a recognized ISO
    /// 639-1/2/3 code (via [`normalize`](Self::normalize)) OR a well-formed
    /// IETF BCP 47 tag (region/script subtags, e.g. `pt-BR`, `sr-Latn`).
    /// Well-formedness only (RFC 5646 grammar); a grammatically valid but
    /// nonexistent tag is accepted here and left for mkvmerge to reject at
    /// mux time (D19).
    pub fn is_valid_value(&self, token: &str) -> bool {
        self.normalize(token).is_some() || language_tags::LanguageTag::parse(token).is_ok()
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

    #[test]
    fn version_pair_parses_three_component_version() {
        let raw = "mkvmerge v100.0.0 ('Message') 64-bit";
        assert_eq!(parse_version_pair(raw).unwrap(), (100, 0));
    }

    #[test]
    fn version_pair_parses_two_component_version() {
        // The locally installed v100 actually reports this shorter form
        // (no patch component), so the parser must not assume exactly three
        // dot-separated components.
        let raw = "mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit";
        assert_eq!(parse_version_pair(raw).unwrap(), (100, 0));
    }

    #[test]
    fn version_pair_rejects_unparseable_string() {
        assert!(matches!(
            parse_version_pair("not a version string"),
            Err(RuntimeError::Parse(_))
        ));
    }

    #[test]
    fn platform_candidates_are_verified_against_mkvtoolnix_packaging() {
        let candidates = platform_candidates();
        #[cfg(target_os = "linux")]
        {
            assert!(candidates.contains(&PathBuf::from("/usr/bin/mkvmerge")));
            assert!(candidates.contains(&PathBuf::from("/usr/local/bin/mkvmerge")));
        }
        #[cfg(target_os = "macos")]
        {
            assert!(candidates.contains(&PathBuf::from(
                "/Applications/MKVToolNix.app/Contents/MacOS/mkvmerge"
            )));
            assert!(candidates.contains(&PathBuf::from("/opt/homebrew/bin/mkvmerge")));
            assert!(candidates.contains(&PathBuf::from("/usr/local/bin/mkvmerge")));
        }
        #[cfg(target_os = "windows")]
        {
            assert!(
                candidates
                    .iter()
                    .any(|p| p.ends_with("MKVToolNix/mkvmerge.exe")
                        || p.ends_with("MKVToolNix\\mkvmerge.exe"))
            );
        }
    }
}
