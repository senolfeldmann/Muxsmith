//! Profile file loading (spec 4): picks YAML vs JSON from the file
//! extension and deserializes into the [`Profile`] model, turning any I/O
//! or deserialization failure into a `ParseError` [`Diagnostic`].

use std::fs;
use std::path::Path;

use crate::report::{DiagCode, Diagnostic};

use super::model::Profile;

/// Serialization format of a profile file, selected by [`from_file`] from
/// the path's extension (`.json` -> Json, anything else including
/// `.yaml`/`.yml` -> Yaml); both formats deserialize into the identical
/// [`Profile`] model (spec 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// YAML source text.
    Yaml,
    /// JSON source text.
    Json,
}

/// Parses profile text already read into memory. Deserialization errors are
/// wrapped through `serde_path_to_error` so the returned `Diagnostic` (code
/// `ParseError`) carries the failing field's path in `config_path` and the
/// underlying serde message in the `detail` param; unknown keys are
/// rejected because every profile struct derives `deny_unknown_fields`
/// (spec 4: "unknown keys are errors, not warnings").
pub fn from_str(text: &str, format: Format) -> Result<Profile, Diagnostic> {
    match format {
        Format::Yaml => {
            let de = yaml_serde::Deserializer::from_str(text);
            serde_path_to_error::deserialize(de).map_err(|e| parse_error(&e))
        }
        Format::Json => {
            let mut de = serde_json::Deserializer::from_str(text);
            serde_path_to_error::deserialize(&mut de).map_err(|e| parse_error(&e))
        }
    }
}

/// Reads and parses a profile file, picking [`Format`] from the file
/// extension. Both an I/O failure and a parse failure surface as a
/// `ParseError` diagnostic; either way `file` is set to `path` via
/// [`Diagnostic::for_file`].
pub fn from_file(path: &Path) -> Result<Profile, Diagnostic> {
    let format = match path.extension().and_then(|e| e.to_str()) {
        Some("json") => Format::Json,
        _ => Format::Yaml, // .yaml, .yml, and anything else tries YAML
    };
    let text = fs::read_to_string(path).map_err(|e| {
        Diagnostic::error(DiagCode::ParseError, "")
            .for_file(path)
            .with("detail", e.to_string())
            .with("at", "")
    })?;
    from_str(&text, format).map_err(|d| d.for_file(path))
}

fn parse_error<E: std::fmt::Display>(err: &serde_path_to_error::Error<E>) -> Diagnostic {
    Diagnostic::error(DiagCode::ParseError, err.path().to_string())
        .with("detail", err.inner().to_string())
        .with("at", err.path().to_string())
}
