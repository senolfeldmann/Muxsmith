//! Source-file identification via `mkvmerge -J` (spec 5.5, 9). Wraps the
//! external process, parses its JSON into a track model, and caches results
//! in memory keyed on path + mtime + size so dry-run and run never re-identify
//! an unchanged file (spec 5.5). Attachments/chapters parsing arrives in
//! Plan 3 where command generation consumes them.

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::capability::runtime::{Mkvmerge, RuntimeError};

/// A scalar track-property value from `-J` output. Non-scalar JSON (arrays,
/// objects, null) is not matchable and is dropped during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum PropValue {
    /// A boolean flag (`default_track`, `forced_track`, ...).
    Bool(bool),
    /// A whole-number value (`audio_channels`, `number`, ...).
    Int(i64),
    /// A floating-point value (`min_luminance`, ...).
    Float(f64),
    /// A string value (`codec_id`, `language`, `track_name`, ...).
    Str(String),
}

impl PropValue {
    /// Converts a JSON scalar into a [`PropValue`]; returns `None` for arrays,
    /// objects, and null. An integral JSON number becomes `Int`, a
    /// non-integral one `Float` (mirrors the profile `Scalar` rule).
    pub fn from_json(v: &Value) -> Option<PropValue> {
        match v {
            Value::Bool(b) => Some(PropValue::Bool(*b)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Some(PropValue::Int(i))
                } else {
                    n.as_f64().map(PropValue::Float)
                }
            }
            Value::String(s) => Some(PropValue::Str(s.clone())),
            _ => None,
        }
    }
}

/// One track from `-J` output. `kind` is the `-J` `type` (`video`/`audio`/
/// `subtitles`/`buttons`), a top-level field alongside `codec` and `id`;
/// everything else lives in `properties` (`codec_id`, `language`, flags, ...).
#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    /// The `-J` track `id` (mkvmerge's per-file track index).
    pub id: u64,
    /// The `-J` track `type`: `video`, `audio`, `subtitles`, or `buttons`.
    pub kind: String,
    /// The `-J` `codec` human-readable name (distinct from the nested
    /// `codec_id`, e.g. `AAC` vs `A_AAC`).
    pub codec: String,
    /// The nested `properties` object, scalar values only.
    pub properties: BTreeMap<String, PropValue>,
}

impl Track {
    /// Looks up a matchable property by name, unifying the top-level
    /// `type`/`codec`/`id` fields with the nested `properties` map so the
    /// matcher sees one flat namespace (spec 4.4). Returns `None` if the track
    /// carries no such property.
    pub fn get(&self, name: &str) -> Option<PropValue> {
        match name {
            "type" => Some(PropValue::Str(self.kind.clone())),
            "codec" => Some(PropValue::Str(self.codec.clone())),
            "id" => Some(PropValue::Int(self.id as i64)),
            other => self.properties.get(other).cloned(),
        }
    }
}

/// The result of identifying one source file (spec 5.5). Carries the track
/// model plus enough container status to tell an identifiable media file from
/// a non-media file (mkvmerge exits 0 either way).
#[derive(Debug, Clone, PartialEq)]
pub struct Identification {
    /// The `file_name` mkvmerge echoed back.
    pub file_name: String,
    /// `identification_format_version`; compared against
    /// [`crate::capability::PINNED_IDENTIFICATION_FORMAT_VERSION`] to detect
    /// schema skew (spec 9.2).
    pub format_version: u64,
    /// `container.recognized`.
    pub container_recognized: bool,
    /// `container.supported`.
    pub container_supported: bool,
    /// The parsed tracks (empty for a non-media file).
    pub tracks: Vec<Track>,
}

impl Identification {
    /// Whether the file is a usable media container: recognized, supported, and
    /// carrying at least one track. A file matching an extension list but not
    /// identifiable is handled by discovery, not treated as a source.
    pub fn is_identifiable(&self) -> bool {
        self.container_recognized && self.container_supported && !self.tracks.is_empty()
    }

    /// Parses `mkvmerge -J` JSON into an [`Identification`]. Missing optional
    /// sections (a non-media file has no `tracks`) default to empty/false.
    pub fn from_json(text: &str) -> Result<Identification, IdentifyError> {
        let v: Value =
            serde_json::from_str(text).map_err(|e| IdentifyError::Json(e.to_string()))?;
        let container = v.get("container");
        let container_recognized = container
            .and_then(|c| c.get("recognized"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let container_supported = container
            .and_then(|c| c.get("supported"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let tracks = v
            .get("tracks")
            .and_then(Value::as_array)
            .map(|arr| arr.iter().filter_map(parse_track).collect())
            .unwrap_or_default();
        Ok(Identification {
            file_name: v
                .get("file_name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            format_version: v
                .get("identification_format_version")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            container_recognized,
            container_supported,
            tracks,
        })
    }
}

fn parse_track(v: &Value) -> Option<Track> {
    let id = v.get("id").and_then(Value::as_u64)?;
    let kind = v.get("type").and_then(Value::as_str)?.to_string();
    let codec = v
        .get("codec")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let mut properties = BTreeMap::new();
    if let Some(props) = v.get("properties").and_then(Value::as_object) {
        for (k, val) in props {
            if let Some(pv) = PropValue::from_json(val) {
                properties.insert(k.clone(), pv);
            }
        }
    }
    Some(Track {
        id,
        kind,
        codec,
        properties,
    })
}

/// Failure identifying a file. Data only; call sites render it to diagnostics.
#[derive(Debug, Clone)]
pub enum IdentifyError {
    /// The mkvmerge process failed (spawn, non-zero, not found).
    Runtime(RuntimeError),
    /// The `-J` output was not valid JSON in the expected shape.
    Json(String),
    /// The file could not be stat'd for the cache key.
    Stat(String),
}

impl From<RuntimeError> for IdentifyError {
    fn from(e: RuntimeError) -> Self {
        IdentifyError::Runtime(e)
    }
}

impl std::fmt::Display for IdentifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentifyError::Runtime(RuntimeError::NotFound) => {
                write!(f, "mkvmerge failed: executable not found")
            }
            IdentifyError::Runtime(RuntimeError::Spawn(e)) => write!(f, "mkvmerge failed: {e}"),
            IdentifyError::Runtime(RuntimeError::NonZero { code, stderr }) => {
                let code = code
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "signal".into());
                write!(f, "mkvmerge failed: exit {code}: {stderr}")
            }
            IdentifyError::Runtime(RuntimeError::Parse(e)) => write!(f, "mkvmerge failed: {e}"),
            IdentifyError::Json(e) => write!(f, "invalid identification JSON: {e}"),
            IdentifyError::Stat(e) => write!(f, "cannot read file: {e}"),
        }
    }
}

/// In-memory identification cache for one session (spec 5.5). Keyed on path
/// plus (mtime, size); a changed file re-identifies, so a dry run is never
/// stale. On-disk caching is a future candidate (spec non-goals).
#[derive(Debug, Default)]
pub struct IdentifyCache {
    entries: HashMap<PathBuf, (CacheKey, Identification)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CacheKey {
    mtime_ns: i128,
    size: u64,
}

impl IdentifyCache {
    /// A fresh, empty cache.
    pub fn new() -> IdentifyCache {
        IdentifyCache::default()
    }

    /// Returns the identification for `path`, running `mkvmerge -J` only if the
    /// file is absent from the cache or its (mtime, size) changed. Borrows the
    /// cached value for the caller's lifetime.
    pub fn get_or_identify(
        &mut self,
        mkv: &Mkvmerge,
        path: &Path,
    ) -> Result<&Identification, IdentifyError> {
        let key = cache_key(path)?;
        let fresh = matches!(self.entries.get(path), Some((k, _)) if *k == key);
        if !fresh {
            let out = mkv.identify_json(path)?;
            let id = Identification::from_json(&out)?;
            self.entries.insert(path.to_path_buf(), (key, id));
        }
        Ok(&self.entries.get(path).expect("just inserted").1)
    }
}

fn cache_key(path: &Path) -> Result<CacheKey, IdentifyError> {
    let meta = std::fs::metadata(path).map_err(|e| IdentifyError::Stat(e.to_string()))?;
    let mtime_ns = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_nanos() as i128)
        .unwrap_or(0);
    Ok(CacheKey {
        mtime_ns,
        size: meta.len(),
    })
}

/// Abstracts identification so the planner can be unit-tested against fixture
/// data without spawning mkvmerge. Returns an owned [`Identification`] (a clone
/// from the cache in the live impl) to keep the trait object simple.
pub trait Identify {
    /// Identifies `path`, or returns why it could not be identified.
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError>;
}

/// The production [`Identify`]: an [`IdentifyCache`] plus the resolved
/// mkvmerge, wired together so `plan_batch` can drive real identification.
pub struct LiveIdentifier<'a> {
    /// The per-session identification cache (spec 5.5).
    pub cache: IdentifyCache,
    /// The resolved external mkvmerge.
    pub mkv: &'a Mkvmerge,
}

impl Identify for LiveIdentifier<'_> {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        self.cache.get_or_identify(self.mkv, path).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../tests/fixtures/identify/series-s01e01.json");

    #[test]
    fn parses_tracks_and_container() {
        let id = Identification::from_json(SAMPLE).unwrap();
        assert_eq!(id.file_name, "Show.S01E01.mkv");
        assert_eq!(id.format_version, 20);
        assert!(id.container_recognized && id.container_supported);
        assert!(id.is_identifiable());
        assert_eq!(id.tracks.len(), 4);
        let audio = &id.tracks[1];
        assert_eq!(audio.id, 1);
        assert_eq!(audio.kind, "audio");
        assert_eq!(audio.codec, "AAC");
    }

    #[test]
    fn get_unifies_toplevel_and_nested_properties() {
        let id = Identification::from_json(SAMPLE).unwrap();
        let sub = &id.tracks[2];
        assert_eq!(sub.get("type"), Some(PropValue::Str("subtitles".into())));
        assert_eq!(sub.get("id"), Some(PropValue::Int(2)));
        assert_eq!(
            sub.get("codec_id"),
            Some(PropValue::Str("S_TEXT/UTF8".into()))
        );
        assert_eq!(sub.get("forced_track"), Some(PropValue::Bool(true)));
        assert_eq!(sub.get("language"), Some(PropValue::Str("eng".into())));
        assert_eq!(sub.get("no_such_prop"), None);
    }

    #[test]
    fn display_is_a_terse_phrase_not_the_debug_dump() {
        let e = IdentifyError::Runtime(RuntimeError::Spawn("No such file or directory".into()));
        assert_eq!(e.to_string(), "mkvmerge failed: No such file or directory");

        let e = IdentifyError::Runtime(RuntimeError::NonZero {
            code: Some(2),
            stderr: "unsupported container".into(),
        });
        assert_eq!(
            e.to_string(),
            "mkvmerge failed: exit 2: unsupported container"
        );

        let e = IdentifyError::Json("EOF while parsing a value".into());
        assert_eq!(
            e.to_string(),
            "invalid identification JSON: EOF while parsing a value"
        );

        let e = IdentifyError::Stat("permission denied".into());
        assert_eq!(e.to_string(), "cannot read file: permission denied");
    }

    #[test]
    fn unrecognized_container_is_not_identifiable() {
        let json = r#"{ "container": { "recognized": false, "supported": false },
                        "file_name": "notes.txt", "identification_format_version": 20 }"#;
        let id = Identification::from_json(json).unwrap();
        assert!(!id.is_identifiable());
        assert!(id.tracks.is_empty());
    }

    #[test]
    fn prop_value_from_json_scalars_only() {
        use serde_json::json;
        assert_eq!(
            PropValue::from_json(&json!(true)),
            Some(PropValue::Bool(true))
        );
        assert_eq!(PropValue::from_json(&json!(7)), Some(PropValue::Int(7)));
        assert_eq!(
            PropValue::from_json(&json!(1.5)),
            Some(PropValue::Float(1.5))
        );
        assert_eq!(
            PropValue::from_json(&json!("x")),
            Some(PropValue::Str("x".into()))
        );
        assert_eq!(PropValue::from_json(&json!([1, 2])), None);
        assert_eq!(PropValue::from_json(&json!(null)), None);
    }
}
