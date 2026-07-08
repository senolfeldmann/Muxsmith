#![deny(missing_docs)]

//! xtask: maintainer-only codegen tool (spec 9.1), never a runtime
//! dependency of `muxsmith-core` or `muxsmith-cli`. Regenerates
//! `capability::generated` from a pinned mkvmerge identification schema.

pub mod codegen;
