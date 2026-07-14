### Task 1: D38 core - conditional NoTrackRules, PassthroughProfile diagnostic, catalogs, e2e

**Files:**
- Modify: `crates/muxsmith-core/src/profile/validate.rs:61-63`
- Modify: `crates/muxsmith-core/src/report/mod.rs` (diag_codes! block: NoTrackRules rustdoc + new variant after it)
- Modify: `locales/en/diagnostics.ftl:7` (+ insert after), `locales/de/diagnostics.ftl:14` (+ insert after)
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs` (fixture_args match)
- Test: `crates/muxsmith-core/tests/validate_semantics.rs`, `crates/muxsmith-cli/tests/run_live.rs`

**Interfaces:**
- Produces: `DiagCode::PassthroughProfile` (wire key `"passthrough-profile"`, info severity, config_path `tracks.rules`, no params) - Task 2's spec row and README name it.
- Consumes: nothing from other tasks.

- [ ] **Step 1: Write the failing validate tests**

In `crates/muxsmith-core/tests/validate_semantics.rs`, directly after `empty_tracks_list_is_rejected` (line ~55):

```rust
#[test]
fn zero_rules_with_unmatched_keep_is_a_passthrough_info() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  unmatched: keep
  rules: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    let diags = validate(&p);
    assert!(
        !diags.iter().any(|d| d.code == DiagCode::NoTrackRules),
        "keep + zero rules is a legal passthrough (D38), not NoTrackRules"
    );
    let d = diags
        .iter()
        .find(|d| d.code == DiagCode::PassthroughProfile)
        .expect("passthrough must be announced");
    assert_eq!(d.severity, Severity::Info);
    assert_eq!(d.config_path, "tracks.rules");
}

#[test]
fn zero_rules_with_unmatched_drop_stays_an_error() {
    let y = r#"
profile_version: 1
input: { pattern: 'E(\d+)', extensions: [mkv] }
tracks:
  unmatched: drop
  rules: []
"#;
    let p = from_str(y, Format::Yaml).unwrap();
    assert!(codes(&p).contains(&DiagCode::NoTrackRules));
    assert!(!codes(&p).contains(&DiagCode::PassthroughProfile));
}
```

The existing `empty_tracks_list_is_rejected` (no `unmatched` key, default drop) stays untouched - it pins the default-drop shape.

- [ ] **Step 2: Run to verify failure**

Run: `cargo test -p muxsmith-core --test validate_semantics`
Expected: compile error - `PassthroughProfile` not found in `DiagCode`.

- [ ] **Step 3: Add the DiagCode variant and reword the NoTrackRules rustdoc**

In `crates/muxsmith-core/src/report/mod.rs`, inside `diag_codes!`, replace

```rust
    /// `tracks` is empty: a profile must select at least one track to produce any output.
    NoTrackRules => "no-track-rules",
```

with

```rust
    /// `tracks.rules` is empty while `tracks.unmatched: drop` would discard every track; a pure passthrough remux instead wants `unmatched: keep` (D38).
    NoTrackRules => "no-track-rules",
    /// Zero track rules with `tracks.unmatched: keep`: a legal pure-passthrough remux (D38) - every primary track is copied unchanged (config-time info, emitted so an accidental delete-all-rules edit stays visible).
    PassthroughProfile => "passthrough-profile",
```

- [ ] **Step 4: Make validate conditional**

In `crates/muxsmith-core/src/profile/validate.rs`, replace lines 61-63

```rust
    if profile.tracks.rules.is_empty() {
        diags.push(Diagnostic::error(DiagCode::NoTrackRules, "tracks.rules"));
    }
```

with

```rust
    if profile.tracks.rules.is_empty() {
        match profile.tracks.unmatched {
            // Discarding everything with no rule selecting anything is a
            // profile that can never produce output.
            KeepDrop::Drop => {
                diags.push(Diagnostic::error(DiagCode::NoTrackRules, "tracks.rules"));
            }
            // Legal pure passthrough (D38): announce it so an accidental
            // delete-all-rules edit stays visible.
            KeepDrop::Keep => diags.push(Diagnostic::info(
                DiagCode::PassthroughProfile,
                "tracks.rules",
            )),
        }
    }
```

Add `KeepDrop` to the existing `use` of the model types at the top of validate.rs (it already imports `SourceCfg` from the same module).

- [ ] **Step 5: Add the bilingual catalog entries**

`locales/en/diagnostics.ftl` line 7, replace

```
no-track-rules = The profile defines no track rules; at least one is required.
```

with

```
no-track-rules = The profile defines no track rules; add at least one rule, or set tracks.unmatched: keep for a pure passthrough remux.
passthrough-profile = This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.
```

`locales/de/diagnostics.ftl` line 14, replace

```
no-track-rules = Das Profil definiert keine Spurregeln; mindestens eine ist erforderlich.
```

with

```
no-track-rules = Das Profil definiert keine Spurregeln; füge mindestens eine hinzu oder setze tracks.unmatched: keep für einen reinen Passthrough-Remux.
passthrough-profile = Dieses Profil definiert keine Spurregeln und tracks.unmatched ist keep: ein reiner Passthrough-Remux; alle Spuren der Primärquelle werden unverändert übernommen. Falls das nicht beabsichtigt ist, füge Spurregeln hinzu.
```

- [ ] **Step 6: Extend the catalog_completeness fixture**

In `crates/muxsmith-cli/tests/catalog_completeness.rs`, the exhaustive `fixture_args` match: after the `DiagCode::NoTrackRules => vec![],` arm add

```rust
        DiagCode::PassthroughProfile => vec![],
```

(The match is exhaustive over `DiagCode`; the workspace does not compile until this arm exists.)

- [ ] **Step 7: Run the unit layers**

Run: `cargo test -p muxsmith-core --test validate_semantics && cargo test -p muxsmith-cli --test catalog_completeness`
Expected: PASS, including both new tests.

- [ ] **Step 8: Write the gated e2e passthrough test**

In `crates/muxsmith-cli/tests/run_live.rs`, add (same `have_mkvmerge` self-skip idiom as the existing tests in that file; reuse its `muxsmith()` helper and the tone.wav/sub.srt seeds referenced in dry_run_cli.rs):

```rust
/// D38 acceptance: a zero-rule `unmatched: keep` profile is a legal pure
/// passthrough - dry-run reports `passthrough-profile` (info) and no
/// `no-track-rules`, and `run` produces an identifiable Matroska output
/// carrying both source tracks unchanged.
#[test]
fn zero_rule_keep_profile_is_a_pure_passthrough() {
    if !have_mkvmerge() {
        eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let wav = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/tone.wav"
    );
    let srt = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../muxsmith-core/tests/fixtures/seeds/sub.srt"
    );
    let media = dir.path().join("Show.S01E01.mkv");
    assert!(
        std::process::Command::new("mkvmerge")
            .args(["-q", "-o"])
            .arg(&media)
            .arg(wav)
            .arg(srt)
            .status()
            .unwrap()
            .success()
    );

    let profile = dir.path().join("p.yaml");
    fs::write(
        &profile,
        "profile_version: 1\ninput: { pattern: 'S(?<s>\\d{2})E(?<e>\\d{2})', extensions: [mkv] }\ntracks:\n  unmatched: keep\n  rules: []\n",
    )
    .unwrap();
    let outdir = dir.path().join("out");

    // Dry run: exit 0, passthrough announced, no NoTrackRules.
    let dry = muxsmith()
        .args(["dry-run", "--json"])
        .arg(&profile)
        .arg("--source")
        .arg(dir.path())
        .arg("--output")
        .arg(&outdir)
        .output()
        .unwrap();
    assert!(dry.status.success(), "dry-run must accept the passthrough profile");
    let report: serde_json::Value = serde_json::from_slice(&dry.stdout).unwrap();
    let codes: Vec<&str> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["code"].as_str().unwrap())
        .collect();
    assert!(codes.contains(&"passthrough-profile"));
    assert!(!codes.contains(&"no-track-rules"));

    // Run: output exists and identifies with both tracks.
    let run = muxsmith()
        .arg("run")
        .arg(&profile)
        .arg("--source")
        .arg(dir.path())
        .arg("--output")
        .arg(&outdir)
        .output()
        .unwrap();
    assert!(run.status.success());
    let out_file = fs::read_dir(&outdir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .find(|p| p.extension().is_some_and(|e| e == "mkv"))
        .expect("run must produce an mkv output");
    let ident = std::process::Command::new("mkvmerge")
        .args(["-J"])
        .arg(&out_file)
        .output()
        .unwrap();
    let j: serde_json::Value = serde_json::from_slice(&ident.stdout).unwrap();
    assert_eq!(j["container"]["recognized"], true);
    assert_eq!(j["tracks"].as_array().unwrap().len(), 2, "both tracks pass through");
}
```

Adapt the exact CLI flag spelling and JSON report field names to what the neighboring tests in `run_live.rs` / `dry_run_cli.rs` actually use (they are the ground truth for the harness idiom); the assertions above (exit codes, the two diagnostic-code checks, recognized container, track count 2) are the contract and stay.

- [ ] **Step 9: Run the e2e test**

Run: `cargo test -p muxsmith-cli --test run_live zero_rule_keep -- --nocapture`
Expected: PASS with a real mkvmerge on PATH (this dev machine has v100).

- [ ] **Step 10: Commit**

```bash
git add crates/muxsmith-core/src/profile/validate.rs crates/muxsmith-core/src/report/mod.rs locales/en/diagnostics.ftl locales/de/diagnostics.ftl crates/muxsmith-cli/tests/catalog_completeness.rs crates/muxsmith-core/tests/validate_semantics.rs crates/muxsmith-cli/tests/run_live.rs
git -c commit.gpgsign=false commit -m "feat: legalize zero-rule keep passthrough with PassthroughProfile info (D38)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

