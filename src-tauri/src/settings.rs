//! App settings persistence (D27): a JSON file under the platform config
//! directory, entirely separate from a user's profile YAML (`dir_memory`
//! and friends are never written into it).
//!
//! **Location choice.** `dirs::config_dir()` is used instead of Tauri's own
//! `app.path().app_config_dir()`. `app_config_dir()` needs a live
//! `AppHandle`/`Manager`, which means every settings-touching function
//! would need a Tauri runtime (or a mocked one) just to be called; a plain
//! `dirs::config_dir()` join keeps [`load`]/[`save`] pure `fn(&Path) -> ...`
//! that a unit test drives directly against a tempdir, which is exactly
//! this task's testability requirement. It also matches the convention
//! `muxsmith-core` already established for its own platform directory
//! (`executor::joblog::default_runs_root`, D26, joins `dirs::data_dir()`
//! with `"muxsmith"`); reusing the same crate and the same
//! `"muxsmith"`-subdirectory shape keeps one platform-directory strategy
//! across the whole workspace instead of introducing a second one just for
//! the shell.

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Maximum number of entries [`save`] keeps in `recent_profiles` (D27).
/// Callers are expected to keep the list newest-first, so capping drops the
/// oldest entries from the tail, not the newest.
const RECENT_PROFILES_CAP: usize = 10;

/// Per-profile source/output directory memory (D27): the batch view
/// remembers the last source/output directories picked for a given
/// profile without ever writing into the user's profile YAML (spec 8.2's
/// "persisted per profile" requirement, delivered app-side; profile
/// mutation itself is Plan 6's territory).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DirMemory {
    /// Last source directory used with this profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Last output directory used with this profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// App-level settings (D23, D27, D28), persisted as JSON at
/// [`settings_path`]. Distinct from a user's profile YAML; never mixed
/// with it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    /// Explicit mkvmerge path override (spec 8.2, D28); takes priority
    /// over PATH and platform candidates in
    /// [`muxsmith_core::capability::runtime::Mkvmerge::detect`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mkvmerge_path: Option<String>,
    /// Default parallelism for `run_queue` (spec 8.2); `1` = sequential.
    #[serde(default = "default_jobs")]
    pub default_jobs: usize,
    /// UI locale override; `None` falls back to the system locale (spec
    /// 8.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Most-recently-used profile paths, newest first, capped at
    /// [`RECENT_PROFILES_CAP`] entries by [`save`].
    #[serde(default)]
    pub recent_profiles: Vec<String>,
    /// Per-profile source/output directory memory, keyed by profile path
    /// (D27).
    #[serde(default)]
    pub dir_memory: HashMap<String, DirMemory>,
}

fn default_jobs() -> usize {
    1
}

impl Default for AppSettings {
    fn default() -> AppSettings {
        AppSettings {
            mkvmerge_path: None,
            default_jobs: default_jobs(),
            locale: None,
            recent_profiles: Vec::new(),
            dir_memory: HashMap::new(),
        }
    }
}

/// A settings I/O failure. Data only (no prose); `crate::error` maps this
/// to an [`crate::error::IpcError`]. `detail` fields are the one accepted
/// prose-passthrough exception (third-party I/O/serde error text, spec
/// 8.4), never text this crate composes itself.
///
/// No "config directory unresolvable" variant: [`load`]/[`save`] take an
/// already-resolved `&Path`, so that question is answered once, by
/// [`settings_path`], and consumed by the sole caller (`AppState`) before
/// this module is ever invoked; a variant this module can never construct
/// would be a dead one.
#[derive(Debug, Clone)]
pub enum SettingsError {
    /// Reading or writing the settings file failed; `detail` is the
    /// underlying OS error text.
    Io(String),
    /// The settings file exists but is not valid JSON in the expected
    /// shape; `detail` is the underlying serde error text.
    Parse(String),
}

/// The D27 default location: `dirs::config_dir()?/muxsmith/settings.json`.
/// `None` when the platform config dir itself cannot be resolved, mirroring
/// `executor::joblog::default_runs_root`'s identically-shaped fallback.
pub fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("muxsmith").join("settings.json"))
}

/// Reads settings from `path`. A missing file returns
/// [`AppSettings::default`] (first run, D27) rather than an error; any
/// other I/O or parse failure is returned distinctly so the caller (an IPC
/// command) can surface it.
pub fn load(path: &Path) -> Result<AppSettings, SettingsError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(AppSettings::default()),
        Err(e) => return Err(SettingsError::Io(e.to_string())),
    };
    serde_json::from_str(&text).map_err(|e| SettingsError::Parse(e.to_string()))
}

/// Writes `settings` to `path` as pretty JSON, creating the parent
/// directory if needed. `recent_profiles` is capped to
/// [`RECENT_PROFILES_CAP`] entries at write time (a defensive invariant on
/// the persisted file itself, independent of what the caller handed in),
/// keeping the first `RECENT_PROFILES_CAP` entries under the newest-first
/// convention documented on the field.
///
/// **Atomic publish (fix).** Writing straight to `path` left a torn,
/// unparseable file behind if the process died mid-write (crash, power
/// loss, kill) -- and `load`'s [`SettingsError::Parse`] on that torn file
/// is exactly what a naive first-run recovery flow would loop on forever,
/// since the file exists and simply never parses. Instead: the bytes are
/// written to a same-directory temp file first, then [`fs::rename`]
/// publishes it onto `path` -- same-filesystem rename is atomic on Linux,
/// macOS, and Windows, so any reader of `path` always sees either the
/// previous complete file or the new complete one, never a partial write.
/// The temp file is flushed to disk ([`fs::File::sync_all`]) before the
/// rename: rename atomicity alone only covers process death, while under
/// delayed allocation (e.g. ext4, btrfs) the rename can reach the journal
/// before the temp file's data blocks reach the disk, turning a power loss
/// into exactly the torn/empty file this path exists to prevent. With the
/// fsync, a power cut yields the previous complete file (rename lost) or
/// the new complete one -- never a torn one.
/// `fs::rename` is the ONLY thing that ever touches the final name. On a
/// failed rename the temp file is removed rather than left behind (its
/// error is deliberately swallowed: the publish failure is what gets
/// reported, a second failure while cleaning up after it would not be more
/// actionable).
pub fn save(path: &Path, settings: &AppSettings) -> Result<(), SettingsError> {
    let parent = match path.parent() {
        Some(parent) => {
            fs::create_dir_all(parent).map_err(|e| SettingsError::Io(e.to_string()))?;
            parent
        }
        None => Path::new("."),
    };
    let mut capped = settings.clone();
    capped.recent_profiles.truncate(RECENT_PROFILES_CAP);
    let bytes =
        serde_json::to_vec_pretty(&capped).map_err(|e| SettingsError::Parse(e.to_string()))?;

    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "settings".to_string());
    let tmp_path = parent.join(format!(".{file_name}.tmp-{}", std::process::id()));

    let mut tmp_file = fs::File::create(&tmp_path).map_err(|e| SettingsError::Io(e.to_string()))?;
    tmp_file
        .write_all(&bytes)
        .map_err(|e| SettingsError::Io(e.to_string()))?;
    tmp_file
        .sync_all()
        .map_err(|e| SettingsError::Io(e.to_string()))?;
    // Drop closes the temp file before the publish rename, matching the
    // open-write-close behavior of the fs::write this replaced instead of
    // leaning on platform-specific rename-while-open semantics.
    drop(tmp_file);
    fs::rename(&tmp_path, path).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        SettingsError::Io(e.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_missing_file_returns_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");

        let loaded = load(&path).expect("missing file is not an error");
        assert_eq!(loaded, AppSettings::default());
        assert_eq!(loaded.default_jobs, 1);
    }

    #[test]
    fn save_then_load_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");

        let mut dir_memory = HashMap::new();
        dir_memory.insert(
            "/profiles/show.yaml".to_string(),
            DirMemory {
                source: Some("/media/in".to_string()),
                output: Some("/media/out".to_string()),
            },
        );
        let original = AppSettings {
            mkvmerge_path: Some("/usr/local/bin/mkvmerge".to_string()),
            default_jobs: 4,
            locale: Some("de".to_string()),
            recent_profiles: vec!["/profiles/show.yaml".to_string()],
            dir_memory,
        };

        save(&path, &original).expect("save");
        let loaded = load(&path).expect("load");
        assert_eq!(loaded, original);
    }

    #[test]
    fn save_creates_missing_parent_directories() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir
            .path()
            .join("nested")
            .join("config")
            .join("settings.json");

        save(&path, &AppSettings::default()).expect("save creates parents");
        assert!(path.is_file());
    }

    #[test]
    fn save_caps_recent_profiles_at_the_mru_limit() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");

        let all: Vec<String> = (0..15).map(|i| format!("/profiles/p{i}.yaml")).collect();
        let settings = AppSettings {
            recent_profiles: all.clone(),
            ..AppSettings::default()
        };

        save(&path, &settings).expect("save");
        let loaded = load(&path).expect("load");

        assert_eq!(loaded.recent_profiles.len(), RECENT_PROFILES_CAP);
        assert_eq!(loaded.recent_profiles, all[..RECENT_PROFILES_CAP]);
    }

    #[test]
    fn load_reports_corrupt_json_distinctly_from_a_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::write(&path, b"{ not json").unwrap();

        match load(&path) {
            Err(SettingsError::Parse(detail)) => assert!(!detail.is_empty()),
            other => panic!("expected Parse error, got {other:?}"),
        }
    }

    #[test]
    fn save_leaves_no_temp_file_behind_after_a_successful_write() {
        // Regression guard for the atomic-write fix (write-to-temp, then
        // rename onto the final path, the only publish point): after a
        // successful save the directory holds exactly the final
        // settings.json, no temp sibling.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");

        save(&path, &AppSettings::default()).expect("save");

        let entries: Vec<String> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            entries,
            vec!["settings.json".to_string()],
            "only the final settings.json may remain visible; no temp file leftover"
        );
    }

    #[test]
    fn save_cleans_up_its_temp_file_when_the_publish_rename_fails() {
        // A directory at the FINAL path (not a file) forces fs::rename to
        // fail (EISDIR/ENOTDIR depending on OS) -- root-safe, unlike a
        // permission-bit trick, mirroring executor::job's/run.rs's own
        // delete_partial/finalize_joblog test pattern. Asserts both halves
        // of "rename is the only publish point": the pre-existing directory
        // at the final name is untouched (the write never touched it), and
        // the temp file the atomic write staged its bytes in does not leak
        // -- a naive atomic-write implementation that skips cleanup on a
        // failed publish would otherwise accumulate temp files forever.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        fs::create_dir(&path).unwrap();

        let result = save(&path, &AppSettings::default());
        assert!(result.is_err(), "a directory at the final path must fail");
        assert!(
            path.is_dir(),
            "the pre-existing directory must be untouched"
        );

        let leftovers: Vec<String> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name != "settings.json")
            .collect();
        assert!(
            leftovers.is_empty(),
            "no temp file may survive a failed publish, found: {leftovers:?}"
        );
    }

    #[test]
    fn settings_path_lives_under_a_muxsmith_subdirectory() {
        if let Some(path) = settings_path() {
            // `Path::ends_with` matches whole components, and `std::path`
            // recognizes '/' as a separator on Windows too, so one
            // forward-slash literal covers every platform.
            assert!(path.ends_with("muxsmith/settings.json"));
        }
    }
}
