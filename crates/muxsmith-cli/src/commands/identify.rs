//! `muxsmith identify` (spec 8.1): identify one file via mkvmerge and print
//! its tracks (human-readable, or `--json` passing the structured data
//! through). Exit 0 on success, 2 on any failure to identify.

use std::path::Path;

use muxsmith_core::capability::runtime::Mkvmerge;
use muxsmith_core::identify::{Identification, IdentifyCache, PropValue};

use crate::i18n::Renderer;

/// Runs `muxsmith identify`. Returns the mkvmerge-style exit code.
pub fn run(file: &Path, json: bool, renderer: &Renderer) -> i32 {
    let mkv = match Mkvmerge::locate() {
        Ok(m) => m,
        Err(_) => {
            eprintln!("{}", renderer.msg("mkvmerge-not-found", &[]));
            return 2;
        }
    };
    let mut cache = IdentifyCache::new();
    let id = match cache.get_or_identify(&mkv, file) {
        Ok(id) => id.clone(),
        Err(_) => {
            eprintln!(
                "{}",
                renderer.msg("identify-failed", &[("file", &file.display().to_string())])
            );
            return 2;
        }
    };
    if json {
        print_identify_json(&id);
    } else {
        print_identify_human(&id, renderer);
    }
    0
}

fn print_identify_json(id: &Identification) {
    let tracks: Vec<serde_json::Value> = id
        .tracks
        .iter()
        .map(|t| serde_json::json!({ "id": t.id, "type": t.kind, "codec": t.codec }))
        .collect();
    println!(
        "{}",
        serde_json::json!({
            "file_name": id.file_name,
            "identification_format_version": id.format_version,
            "identifiable": id.is_identifiable(),
            "tracks": tracks,
        })
    );
}

fn print_identify_human(id: &Identification, renderer: &Renderer) {
    if !id.is_identifiable() {
        println!(
            "{}",
            renderer.msg("identify-not-media", &[("file", &id.file_name)])
        );
        return;
    }
    for t in &id.tracks {
        let lang = match t.get("language") {
            Some(PropValue::Str(s)) => s,
            _ => String::new(),
        };
        println!(
            "{}",
            renderer.msg(
                "identify-track-line",
                &[
                    ("id", &t.id.to_string()),
                    ("type", &t.kind),
                    ("codec", &t.codec),
                    ("language", &lang),
                ],
            )
        );
    }
}
