#![deny(missing_docs)]

//! Muxsmith CLI: a thin clap shell over `muxsmith-core` (spec 7). No
//! validation or planning logic lives here; this crate only parses
//! arguments, calls core, and renders core's diagnostic data through
//! Fluent catalogs (spec 8.4).

pub mod cli;
pub mod commands;
pub mod i18n;
