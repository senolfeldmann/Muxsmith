#![deny(missing_docs)]

//! Muxsmith core: all rule evaluation, profile validation, diagnostics and
//! template rendering live here (spec 7). This crate emits no user-facing
//! prose; it produces diagnostic codes and structured params only (spec
//! 5.2, 8.4), for the CLI and GUI renderers to turn into localized text.

pub mod capability;
pub mod command;
pub mod discovery;
pub mod executor;
pub mod identify;
pub mod matcher;
pub mod pipeline;
pub mod planner;
pub mod profile;
pub mod report;
pub mod template;

// `#[doc(hidden)] pub`: the live-mkvmerge integration tests self-skip when
// mkvmerge is absent from PATH, and CI's no-silent-skip gate
// (.github/workflows/ci.yml) greps test output for this exact string to
// assert the count is zero. The marker text lived as a duplicated string
// literal at ~21 `eprintln!` sites across muxsmith-core, muxsmith-cli and
// muxsmith-gui (no per-crate tests/support module can span all three), so a
// single reworded copy would silently defeat the gate; one shared const
// keeps every site and the CI grep byte-identical by construction.
#[doc(hidden)]
pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";
