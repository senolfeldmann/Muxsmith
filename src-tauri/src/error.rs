//! The shell's IPC error contract (D23, spec 8.4): every command returns
//! `Result<_, IpcError>`. `IpcError` carries a stable, kebab-case `code`
//! plus structured `params` -- no prose. This mirrors core's own
//! diagnostic-code philosophy (`muxsmith_core::report::DiagCode`, spec 5.2)
//! at the shell layer: the frontend looks `code` up in its own Fluent
//! catalog and interpolates `params` (spec 8.4), exactly as it does for a
//! `Diagnostic`'s `code`/`params` inside a report document. The one
//! accepted exception (spec 8.4): a third-party error's own message text,
//! passed through verbatim as a `detail` param.

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
    /// A stable, kebab-case identifier for the failure condition, in the
    /// same style as `DiagCode::key()` so the whole app's error-code
    /// vocabulary stays uniform between core diagnostics and shell-level
    /// command failures.
    pub code: String,
    /// Structured values for the frontend's message template; e.g.
    /// `detail` carries a passed-through third-party error string (the one
    /// prose exception, spec 8.4), `found`/`minimum` carry a too-old
    /// mkvmerge version pair.
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

    /// Attaches one param, builder-style, overwriting any prior value for
    /// `key`.
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
}
