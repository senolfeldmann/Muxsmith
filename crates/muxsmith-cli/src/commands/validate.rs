use std::path::Path;

use muxsmith_core::profile::{lint, load, validate};
use muxsmith_core::report::{Diagnostic, Severity, worst_severity};

use crate::i18n::Renderer;

pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
    let diagnostics = collect(profile_path);
    let exit = match worst_severity(&diagnostics) {
        Some(Severity::Error) => 2,
        Some(Severity::Warning) => 1,
        _ => 0,
    };

    if json {
        let entries: Vec<serde_json::Value> = diagnostics
            .iter()
            .map(|d| {
                let mut v = serde_json::to_value(d).unwrap();
                v["rendered"] = serde_json::Value::String(renderer.diagnostic(d));
                v
            })
            .collect();
        println!("{}", serde_json::json!({ "diagnostics": entries }));
    } else if diagnostics.is_empty() {
        println!("{}", renderer.msg("validate-ok", &[]));
    } else {
        let mut sorted = diagnostics.clone();
        sorted.sort_by_key(|d| std::cmp::Reverse(d.severity));
        for d in &sorted {
            println!("{}", renderer.diagnostic(d));
        }
        let count = |s| {
            diagnostics
                .iter()
                .filter(|d| d.severity == s)
                .count()
                .to_string()
        };
        println!(
            "{}",
            renderer.msg(
                "validate-summary",
                &[
                    ("errors", &count(Severity::Error)),
                    ("warnings", &count(Severity::Warning)),
                    ("infos", &count(Severity::Info)),
                ],
            )
        );
    }
    exit
}

fn collect(profile_path: &Path) -> Vec<Diagnostic> {
    match load::from_file(profile_path) {
        Err(d) => vec![d],
        Ok(profile) => {
            let mut diags = validate::validate(&profile);
            diags.extend(lint::provable_overlaps(&profile));
            diags
        }
    }
}
