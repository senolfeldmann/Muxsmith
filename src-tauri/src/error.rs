//! The shell's IPC error contract (spec 7/8.4): every `#[tauri::command]`
//! that can fail returns `Result<_, IpcError>` instead of a string. Like
//! core's own [`muxsmith_core::report::Diagnostic`], an [`IpcError`] is a
//! stable `code` plus interpolation `params`, never prose -- the frontend
//! renders it through its own Fluent catalog. The shell itself never
//! formats a message string.

use std::collections::HashMap;

use serde::Serialize;

/// A structured IPC-level failure: a stable, kebab-case `code` (a Fluent
/// message id) plus the `params` a renderer interpolates into it. Distinct
/// from a core [`muxsmith_core::report::Diagnostic`] (which describes a
/// profile/plan problem): an `IpcError` describes an IPC-protocol-level
/// failure the caller cannot proceed past (a state conflict, a bad
/// argument, an unreadable path) rather than plan content, which the
/// shell's `Ok` payloads carry as data instead (spec 7).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IpcError {
    /// The stable failure code (a Fluent message id).
    pub code: String,
    /// Structured values a renderer interpolates into the message template
    /// selected by `code`.
    pub params: HashMap<String, String>,
}

impl IpcError {
    /// Builds an [`IpcError`] with no params.
    pub fn code(code: impl Into<String>) -> IpcError {
        IpcError {
            code: code.into(),
            params: HashMap::new(),
        }
    }

    /// Attaches one param key/value, overwriting any prior value for
    /// `key`. Builder-style, mirroring [`muxsmith_core::report::Diagnostic::with`].
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> IpcError {
        self.params.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_builds_with_empty_params() {
        let e = IpcError::code("run-already-active");
        assert_eq!(e.code, "run-already-active");
        assert!(e.params.is_empty());
    }

    #[test]
    fn with_attaches_and_overwrites_params() {
        let e = IpcError::code("job-log-not-found")
            .with("run_id", "20260710-153612Z")
            .with("index", "0")
            .with("index", "1");
        assert_eq!(e.params["run_id"], "20260710-153612Z");
        assert_eq!(e.params["index"], "1");
    }

    #[test]
    fn serializes_as_code_and_params() {
        let e = IpcError::code("invalid-run-id").with("run_id", "../etc");
        let json = serde_json::to_value(&e).unwrap();
        assert_eq!(json["code"], "invalid-run-id");
        assert_eq!(json["params"]["run_id"], "../etc");
    }
}
