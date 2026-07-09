//! Shared test helpers for muxsmith-core's integration test suites.
//!
//! A `tests/` SUBDIRECTORY module (`tests/support/mod.rs`), not a
//! `tests/*.rs` file: Cargo's test-target autodiscovery only globs
//! `tests/*.rs`, so this file is never compiled as its own test binary. Each
//! consumer test file pulls it in with `mod support;`.

use std::collections::HashMap;
use std::path::Path;

use muxsmith_core::capability::runtime::LanguageIndex;
use muxsmith_core::identify::{Identification, Identify, IdentifyError};

/// A fake identifier backed by fixture JSON keyed on file name.
#[allow(dead_code)]
pub struct FakeIdent {
    pub by_name: HashMap<String, Identification>,
}

impl Identify for FakeIdent {
    fn identify(&mut self, path: &Path) -> Result<Identification, IdentifyError> {
        let name = path.file_name().unwrap().to_str().unwrap();
        self.by_name
            .get(name)
            .cloned()
            .ok_or_else(|| IdentifyError::Json(format!("no fixture for {name}")))
    }
}

/// The standard three-language (English/German/Turkish) index shared by the
/// planner/suggestions/command-integration suites.
#[allow(dead_code)]
pub fn lang() -> LanguageIndex {
    LanguageIndex::from_rows(&[
        ["English", "eng", "eng", "en"],
        ["German", "ger", "ger", "de"],
        ["Turkish", "tur", "tur", "tr"],
    ])
}
