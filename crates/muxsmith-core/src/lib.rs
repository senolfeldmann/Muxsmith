#![deny(missing_docs)]

//! Muxsmith core: all rule evaluation, profile validation, diagnostics and
//! template rendering live here (spec 7). This crate emits no user-facing
//! prose; it produces diagnostic codes and structured params only (spec
//! 5.2, 8.4), for the CLI and GUI renderers to turn into localized text.

pub mod capability;
pub mod identify;
pub mod profile;
pub mod report;
pub mod template;
