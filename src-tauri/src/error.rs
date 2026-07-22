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
use muxsmith_core::planner::ApplyError;
use muxsmith_core::profile::save::SaveError;
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
    /// `index` identify a run-lifecycle target. Each value is a
    /// [`ParamValue`]: numeric-semantic params (`index`, `rules`) serialize
    /// as JSON numbers so Fluent can apply CLDR plural rules; everything
    /// else stays a string (D61, i18n-05).
    pub params: HashMap<String, ParamValue>,
}

/// One `IpcError` param value: a number for numeric-semantic params
/// (`index`, `rules`), a string for everything else. `#[serde(untagged)]`
/// keeps string params serializing exactly as before while numeric params
/// become JSON numbers, so Fluent can apply CLDR plural rules (D61,
/// i18n-05).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// A numeric param, rendered by Fluent as a number.
    Num(u64),
    /// A string param (the default; includes third-party passthrough text).
    Str(String),
}

impl From<&str> for ParamValue {
    fn from(v: &str) -> ParamValue {
        ParamValue::Str(v.to_owned())
    }
}
impl From<String> for ParamValue {
    fn from(v: String) -> ParamValue {
        ParamValue::Str(v)
    }
}
impl From<usize> for ParamValue {
    fn from(v: usize) -> ParamValue {
        ParamValue::Num(v as u64)
    }
}

impl IpcError {
    /// Builds a code-only error with no params. (Cleanup: this used to have
    /// a duplicate `code(...)` constructor -- a merge artifact from two
    /// tasks each naming their own read-only/settings vs. run-lifecycle
    /// call sites differently -- collapsed into this single constructor;
    /// every call site now uses this name.)
    pub fn new(code: impl Into<String>) -> IpcError {
        IpcError {
            code: code.into(),
            params: HashMap::new(),
        }
    }

    /// Attaches one param, builder-style, overwriting any prior value for
    /// `key`. Mirrors [`muxsmith_core::report::Diagnostic::with`].
    pub fn with(mut self, key: impl Into<String>, value: impl Into<ParamValue>) -> IpcError {
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

/// Maps a profile-save failure ([`muxsmith_core::profile::save::to_file`])
/// to an [`IpcError`] (D42). Same split as [`From<SettingsError>`] above,
/// same reason: `Io` (permissions, full disk, bad path) and `Serialize`
/// (the model could not be turned into text) are different failures, kept
/// distinguishable rather than folded into one code (`core-124`, Şenol
/// ruling 2026-07-16: `SaveError::{Io,Serialize}` maps to
/// `profile-save-io-failed`/`profile-save-failed`, never `ParseError` --
/// that catalog entry asserts a parse, which a write failure never had).
impl From<SaveError> for IpcError {
    fn from(e: SaveError) -> IpcError {
        match e {
            SaveError::Io(detail) => IpcError::new("profile-save-io-failed").with("detail", detail),
            SaveError::Serialize(detail) => {
                IpcError::new("profile-save-failed").with("detail", detail)
            }
        }
    }
}

/// Maps an [`ApplyError`] ([`muxsmith_core::planner::apply_suggestion`]) to
/// an [`IpcError`] (D43/D49 "The shell mapping", verbatim). Every variant
/// here is a frontend-side bug surfaced as data rather than silently
/// swallowed: an unparsable `config_path`, an index past the end of
/// `tracks.rules`, or a suggestion computed against a since-edited model
/// (`EditChangedNothing`) that would otherwise silently no-op.
impl From<ApplyError> for IpcError {
    fn from(e: ApplyError) -> IpcError {
        match e {
            ApplyError::UnparsableConfigPath(path) => {
                IpcError::new("apply-unparsable-config-path").with("path", path)
            }
            ApplyError::RuleIndexOutOfRange { index, rules } => {
                IpcError::new("apply-rule-index-out-of-range")
                    .with("index", index)
                    .with("rules", rules)
            }
            ApplyError::EditChangedNothing { index, property } => {
                IpcError::new("apply-edit-changed-nothing")
                    .with("index", index)
                    .with("property", property)
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
        assert_eq!(
            err.params["found"],
            ParamValue::Str("mkvmerge v50.0.0 ('Old') 64-bit".into())
        );
        assert_eq!(err.params["minimum"], ParamValue::Str("86.0".into()));
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
        assert_eq!(
            err.params["detail"],
            ParamValue::Str("unsupported option".into())
        );
        assert_eq!(err.params["code"], ParamValue::Str("2".into()));
    }

    #[test]
    fn non_zero_signal_termination_maps_code_param_to_signal() {
        let err: IpcError = RuntimeError::NonZero {
            code: None,
            stderr: "killed".into(),
        }
        .into();
        assert_eq!(err.code, "mkvmerge-query-failed");
        assert_eq!(err.params["code"], ParamValue::Str("signal".into()));
    }

    #[test]
    fn parse_failure_maps_to_query_failed_with_detail() {
        let err: IpcError = RuntimeError::Parse("no version token in \"gibberish\"".into()).into();
        assert_eq!(err.code, "mkvmerge-query-failed");
        assert_eq!(
            err.params["detail"],
            ParamValue::Str("no version token in \"gibberish\"".into())
        );
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
        assert_eq!(
            err.params["detail"],
            ParamValue::Str("EOF while parsing".into())
        );
    }

    #[test]
    fn settings_errors_map_to_distinct_codes() {
        let io: IpcError = SettingsError::Io("permission denied".into()).into();
        let parse: IpcError = SettingsError::Parse("unexpected EOF".into()).into();
        assert_eq!(io.code, "settings-io-failed");
        assert_eq!(
            io.params["detail"],
            ParamValue::Str("permission denied".into())
        );
        assert_eq!(parse.code, "settings-parse-failed");
        assert_ne!(io.code, parse.code);
    }

    #[test]
    fn save_errors_map_to_distinct_codes() {
        let io: IpcError = SaveError::Io("permission denied".into()).into();
        let ser: IpcError = SaveError::Serialize("bad float".into()).into();
        assert_eq!(io.code, "profile-save-io-failed");
        assert_eq!(
            io.params["detail"],
            ParamValue::Str("permission denied".into())
        );
        assert_eq!(ser.code, "profile-save-failed");
        assert_ne!(io.code, ser.code);
    }

    #[test]
    fn apply_errors_map_to_distinct_codes() {
        let unparsable: IpcError =
            ApplyError::UnparsableConfigPath("not-a-rule-path".into()).into();
        let oob: IpcError = ApplyError::RuleIndexOutOfRange { index: 7, rules: 1 }.into();
        let noop: IpcError = ApplyError::EditChangedNothing {
            index: 0,
            property: "forced_track".into(),
        }
        .into();

        assert_eq!(unparsable.code, "apply-unparsable-config-path");
        assert_eq!(
            unparsable.params["path"],
            ParamValue::Str("not-a-rule-path".into())
        );
        assert_eq!(oob.code, "apply-rule-index-out-of-range");
        assert_eq!(oob.params["index"], ParamValue::Num(7));
        assert_eq!(oob.params["rules"], ParamValue::Num(1));
        assert_eq!(noop.code, "apply-edit-changed-nothing");
        assert_eq!(noop.params["index"], ParamValue::Num(0));
        assert_eq!(
            noop.params["property"],
            ParamValue::Str("forced_track".into())
        );

        assert_ne!(unparsable.code, oob.code);
        assert_ne!(oob.code, noop.code);
        assert_ne!(unparsable.code, noop.code);
    }

    #[test]
    fn with_overwrites_a_prior_value_for_the_same_key() {
        let err = IpcError::new("x").with("k", "a").with("k", "b");
        assert_eq!(err.params["k"], ParamValue::Str("b".into()));
    }

    #[test]
    fn new_builds_with_empty_params() {
        let e = IpcError::new("run-already-active");
        assert_eq!(e.code, "run-already-active");
        assert!(e.params.is_empty());
    }

    #[test]
    fn with_attaches_and_overwrites_params() {
        let e = IpcError::new("job-log-not-found")
            .with("run_id", "20260710-153612Z")
            .with("index", "0")
            .with("index", "1");
        assert_eq!(
            e.params["run_id"],
            ParamValue::Str("20260710-153612Z".into())
        );
        assert_eq!(e.params["index"], ParamValue::Str("1".into()));
    }

    #[test]
    fn serializes_as_code_and_params() {
        let e = IpcError::new("invalid-run-id").with("run_id", "../etc");
        let json = serde_json::to_value(&e).unwrap();
        assert_eq!(json["code"], "invalid-run-id");
        assert_eq!(json["params"]["run_id"], "../etc");
    }

    #[test]
    fn params_serialize_untagged_numbers_and_strings() {
        let err = IpcError::new("apply-rule-index-out-of-range")
            .with("index", 3usize)
            .with("rules", 2usize)
            .with("path", "x.yaml");
        let v = serde_json::to_value(&err).unwrap();
        assert_eq!(v["params"]["index"], serde_json::json!(3));
        assert_eq!(v["params"]["path"], serde_json::json!("x.yaml"));
    }
}
