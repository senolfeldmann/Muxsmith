//! Profile file writing (spec 8.2, D41): serializes the in-memory
//! [`Profile`] model back to YAML or JSON. Saving is a canonical rewrite
//! from the model, mirroring [`super::load`]'s read path in reverse:
//! comments, key order and flow/block formatting in the user's original
//! file are not preserved, and this is by design, not a defect (D41's
//! rationale: YAML has no concept of comment attachment, so any
//! "preserving" writer invents an association the spec disclaims).

use std::fs;
use std::path::Path;

use super::load::Format;
use super::model::Profile;

/// A failure of the profile writer. Not a [`crate::report::Diagnostic`]: a
/// `Diagnostic` describes a problem with the profile or the plan, and a
/// write failure leaves a valid model and, say, a full disk
/// (`core-124-error-currency-split`). Data only (no prose); the shell maps
/// this to an `IpcError`, mirroring `SettingsError` in `src-tauri`.
#[derive(Debug, Clone, PartialEq)]
pub enum SaveError {
    /// The file could not be written (permissions, full disk, bad path);
    /// `detail` is the underlying I/O error text.
    Io(String),
    /// The model could not be serialized to the target format; `detail` is
    /// the underlying serde error text.
    Serialize(String),
}

/// Serializes `profile` to `format`'s canonical text: `yaml_serde::to_string`
/// for [`Format::Yaml`], `serde_json::to_string_pretty` for
/// [`Format::Json`]. Both are the crate's ordinary serde writers, no
/// hand-rolled formatting; a serialization failure (neither writer can fail
/// on this model today, but the model derives `Serialize` generically)
/// surfaces as [`SaveError::Serialize`] rather than a panic.
pub fn to_string(profile: &Profile, format: Format) -> Result<String, SaveError> {
    match format {
        Format::Yaml => {
            yaml_serde::to_string(profile).map_err(|e| SaveError::Serialize(e.to_string()))
        }
        Format::Json => {
            serde_json::to_string_pretty(profile).map_err(|e| SaveError::Serialize(e.to_string()))
        }
    }
}

/// Writes `profile` to `path`, picking [`Format`] from the path extension
/// exactly as [`super::load::from_file`] does (`Some("json")` ->
/// [`Format::Json`], everything else -> [`Format::Yaml`]), so a `.json`
/// profile saves as JSON and a `.yaml`/`.yml`/extension-less path saves as
/// YAML: the format never silently changes underfoot. Serialization
/// failures and I/O failures ([`fs::write`]: missing parent directory,
/// permissions, full disk) are both reported as a [`SaveError`], never a
/// panic.
pub fn to_file(profile: &Profile, path: &Path) -> Result<(), SaveError> {
    let format = match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Format::Json,
        _ => Format::Yaml, // .yaml, .yml, and anything else writes YAML
    };
    let text = to_string(profile, format)?;
    fs::write(path, text).map_err(|e| SaveError::Io(e.to_string()))
}
