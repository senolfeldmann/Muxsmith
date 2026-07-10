//! The shell's IPC error contract (D23, spec 7/8.4): every `#[tauri::command]`
//! that can fail returns `Result<_, IpcError>` instead of a string. `IpcError`
//! carries a stable, kebab-case `code` plus structured `params` -- no prose.
//! This mirrors core's own diagnostic-code philosophy
//! (`muxsmith_core::report::DiagCode`, spec 5.2) at the shell layer: the
//! frontend looks `code` up in its own Fluent catalog and interpolates
//! `params` (spec 8.4), exactly as it does for a `Diagnostic`'s `code`/
//! `params` inside a report document. Distinct from a core
//! [`muxsmith_core::report::Diagnostic`] (which describes a profile/plan
//! problem): an `IpcError` describes an IPC-protocol-level failure the
//! caller cannot proceed past (a state conflict, a bad argument, an
//! unreadable path) rather than plan content, which the shell's `Ok`
//! payloads carry as data instead (spec 7). The one accepted exception
//! (spec 8.4): a third-party error's own message text, passed through
//! verbatim as a `detail` param.

use std::collections::HashMap;

use muxsmith_core::capability::runtime::RuntimeError;
use muxsmith_core::identify::IdentifyError;
use serde::Serialize;

use crate::settings::SettingsError;

/// A command failure: a stable code plus structured params, never prose
/// (spec 8.4). Implements `Serialize` directly (not `std::error::Error`)
/// because that is all a `#[tauri::command]` return type's error half
/// requires; the frontend receives exactly this shape as the rejected
/// promise's payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct IpcError {
    /// A stable, kebab-case identifier for the failure condition (a Fluent
    /// message id), in the same style as `DiagCode::key()` so the whole
    /// app's error-code vocabulary stays uniform between core diagnostics
    /// and shell-level command failures.
    pub code: String,
    /// Structured values a renderer interpolates into the message template
    /// selected by `code`; e.g. `detail` carries a passed-through
    /// third-party error string (the one prose exception, spec 8.4),
    /// `found`/`minimum` carry a too-old mkvmerge version pair, `run_id`/
    /// `index` identify a run-lifecycle target.
    pub params: HashMap<String, String>,
}

impl IpcError {
    /// Builds a code-only error with no params.
    pub fn new(code: impl Into<String>) -> IpcError {
        IpcError {
            code: code.into(),
            params: HashMap::new(),
        }
    }

    /// Builds an [`IpcError`] with no params. Same as [`IpcError::new`],
    /// kept as its own name alongside it: the read-only/settings commands
    /// call `new`, the run-lifecycle commands call `code` -- both
    /// constructors survive the merge so neither call site needs touching.
    pub fn code(code: impl Into<String>) -> IpcError {
        IpcError::new(code)
    }

    /// Attaches one param, builder-style, overwriting any prior value for
    /// `key`. Mirrors [`muxsmith_core::report::Diagnostic::with`].
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> IpcError {
        self.params.insert(key.into(), value.into());
        self
    }
}

/// Maps a runtime mkvmerge failure ([`Mkvmerge::detect`]/`locate`/any
/// query, `muxsmith_core::capability::runtime`) to an [`IpcError`].
/// `TooOld` is kept distinguishable from a plain not-found (D28, this
/// task's brief): the frontend needs `found`/`minimum` to render "upgrade
/// mkvtoolnix" guidance rather than "install mkvtoolnix" guidance.
///
/// [`Mkvmerge::detect`]: muxsmith_core::capability::runtime::Mkvmerge::detect
impl From<RuntimeError> for IpcError {
    fn from(e: RuntimeError) -> IpcError {
        match e {
            RuntimeError::NotFound => IpcError::new("mkvmerge-not-found"),
            RuntimeError::TooOld { found, minimum } => IpcError::new("mkvmerge-too-old")
                .with("found", found)
                .with("minimum", minimum),
            RuntimeError::Spawn(detail) => {
                IpcError::new("mkvmerge-spawn-failed").with("detail", detail)
            }
            RuntimeError::NonZero { code, stderr } => IpcError::new("mkvmerge-query-failed")
                .with("detail", stderr)
                .with(
                    "code",
                    code.map(|c| c.to_string())
                        .unwrap_or_else(|| "signal".into()),
                ),
            RuntimeError::Parse(detail) => {
                IpcError::new("mkvmerge-query-failed").with("detail", detail)
            }
        }
    }
}

/// Maps an identification failure to an [`IpcError`]. A `Runtime` failure
/// delegates to the [`RuntimeError`] mapping above (so a missing/too-old
/// mkvmerge surfaces the same granular code whether it was found via
/// `identify` or `dry_run`); a bad `-J` payload or an unreadable file both
/// fold into one `identify-failed` code with a `detail` param, mirroring
/// the CLI's single `identify-failed` message (`locales/en/cli.ftl`) --
/// finer-grained codes for these two would need `identify-failed`
/// subtypes the CLI's own catalog does not have, and the parse/stat
/// distinction is not user-actionable (spec 8.1 exposes no separate hint
/// for either) the way a too-old-vs-missing mkvmerge distinction is.
impl From<IdentifyError> for IpcError {
    fn from(e: IdentifyError) -> IpcError {
        match e {
            IdentifyError::Runtime(re) => IpcError::from(re),
            IdentifyError::Json(detail) | IdentifyError::Stat(detail) => {
                IpcError::new("identify-failed").with("detail", detail)
            }
        }
    }
}

/// Maps a settings I/O failure to an [`IpcError`]. Read vs write context is
/// left for the frontend to infer from which command failed (`get_settings`
/// vs `set_settings`); the error type itself carries no such distinction
/// (an `io::Error` does not know which caller produced it), so encoding it
/// into the code would be a guess, not a fact.
impl From<SettingsError> for IpcError {
    fn from(e: SettingsError) -> IpcError {
        match e {
            SettingsError::Io(detail) => IpcError::new("settings-io-failed").with("detail", detail),
            SettingsError::Parse(detail) => {
                IpcError::new("settings-parse-failed").with("detail", detail)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_found_has_no_params() {
        let err: IpcError = RuntimeError::NotFound.into();
        assert_eq!(err.code, "mkvmerge-not-found");
        assert!(err.params.is_empty());
    }

    #[test]
    fn too_old_carries_found_and_minimum() {
        let err: IpcError = RuntimeError::TooOld {
            found: "mkvmerge v50.0.0 ('Old') 64-bit".into(),
            minimum: "86.0".into(),
        }
        .into();
        assert_eq!(err.code, "mkvmerge-too-old");
        assert_eq!(err.params["found"], "mkvmerge v50.0.0 ('Old') 64-bit");
        assert_eq!(err.params["minimum"], "86.0");
    }

    #[test]
    fn too_old_is_distinguishable_from_not_found() {
        let not_found: IpcError = RuntimeError::NotFound.into();
        let too_old: IpcError = RuntimeError::TooOld {
            found: "x".into(),
            minimum: "y".into(),
        }
        .into();
        assert_ne!(not_found.code, too_old.code);
    }

    #[test]
    fn non_zero_maps_to_query_failed_with_detail_and_exit_code() {
        let err: IpcError = RuntimeError::NonZero {
            code: Some(2),
            stderr: "unsupported option".into(),
        }
        .into();
        assert_eq!(err.code, "mkvmerge-query-failed");
        assert_eq!(err.params["detail"], "unsupported option");
        assert_eq!(err.params["code"], "2");
    }

    #[test]
    fn non_zero_signal_termination_maps_code_param_to_signal() {
        let err: IpcError = RuntimeError::NonZero {
            code: None,
            stderr: "killed".into(),
        }
        .into();
        assert_eq!(err.code, "mkvmerge-query-failed");
        assert_eq!(err.params["code"], "signal");
    }

    #[test]
    fn parse_failure_maps_to_query_failed_with_detail() {
        let err: IpcError = RuntimeError::Parse("no version token in \"gibberish\"".into()).into();
        assert_eq!(err.code, "mkvmerge-query-failed");
        assert_eq!(err.params["detail"], "no version token in \"gibberish\"");
    }

    #[test]
    fn identify_runtime_failure_delegates_to_the_runtime_mapping() {
        let err: IpcError = IdentifyError::Runtime(RuntimeError::NotFound).into();
        assert_eq!(err.code, "mkvmerge-not-found");
    }

    #[test]
    fn identify_json_failure_carries_detail() {
        let err: IpcError = IdentifyError::Json("EOF while parsing".into()).into();
        assert_eq!(err.code, "identify-failed");
        assert_eq!(err.params["detail"], "EOF while parsing");
    }

    #[test]
    fn settings_errors_map_to_distinct_codes() {
        let io: IpcError = SettingsError::Io("permission denied".into()).into();
        let parse: IpcError = SettingsError::Parse("unexpected EOF".into()).into();
        assert_eq!(io.code, "settings-io-failed");
        assert_eq!(io.params["detail"], "permission denied");
        assert_eq!(parse.code, "settings-parse-failed");
        assert_ne!(io.code, parse.code);
    }

    #[test]
    fn with_overwrites_a_prior_value_for_the_same_key() {
        let err = IpcError::new("x").with("k", "a").with("k", "b");
        assert_eq!(err.params["k"], "b");
    }

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
