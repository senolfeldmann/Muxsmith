use std::fs;
use std::path::Path;

use crate::report::{DiagCode, Diagnostic};

use super::model::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Yaml,
    Json,
}

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
