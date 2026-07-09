//! Process execution (spec 6, D13): spawning mkvmerge behind a testable
//! seam, per-job state, and the FIFO queue. Prose-free like the rest of
//! core; all human text lives in the CLI's Fluent catalogs.

pub mod job;
pub mod spawn;
