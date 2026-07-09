### Task 1: Restructure `tracks` into a `{ unmatched, rules }` block

Behavior-preserving structural change. `tracks` moves from a bare `Vec<TrackRule>` to a `TracksCfg { unmatched: KeepDrop (default drop), rules: Vec<TrackRule> }` block, matching the existing `AttachmentsCfg` and the profile's "policies live in their section" language (`output.on_collision`, `tags.global`). The `unmatched` field is added here but not yet consumed (Task 2 wires it). Default `drop` reproduces today's behavior, so the whole suite stays green after the mechanical YAML migration.

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs:34` (the `pub tracks:` field) and add `TracksCfg` next to `AttachmentsCfg` (~model.rs:272-300)
- Modify (consumers, `profile.tracks` -> `profile.tracks.rules`): `crates/muxsmith-core/src/planner.rs:286,364,983,1031,1169`; `crates/muxsmith-core/src/profile/validate.rs:59,63`; `crates/muxsmith-core/src/profile/lint.rs:21`
- Modify (fixtures): `crates/muxsmith-core/tests/fixtures/reference.yaml`, `crates/muxsmith-cli/tests/fixtures/good.yaml`, `crates/muxsmith-cli/tests/fixtures/bad.yaml`
- Modify (inline test YAML, all `tracks:` sites): `crates/muxsmith-core/tests/{planner_resolution.rs,profile_load.rs,suggestions.rs,validate_semantics.rs,validate_structure.rs,command_integration.rs,validate_hardening.rs}`, `crates/muxsmith-cli/tests/{dry_run_cli.rs,cli_validate.rs}`, and `#[cfg(test)]` modules in `crates/muxsmith-core/src/profile/lint.rs`, `crates/muxsmith-core/src/{identify.rs,command.rs}`, `crates/muxsmith-cli/src/commands/identify.rs`
- Modify (spec text): `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` sections 4.1 (reference example) and 4.5 (`tracks` description)

**Interfaces:**
- Produces: `pub struct TracksCfg { pub unmatched: KeepDrop, pub rules: Vec<TrackRule> }`; `Profile.tracks` is now `TracksCfg`. Rule access is `profile.tracks.rules` (was `profile.tracks`).

- [ ] **Step 1: Write the failing test** (new profile block shape parses; `unmatched` defaults to `drop`)

In `crates/muxsmith-core/tests/profile_load.rs` add:

```rust
#[test]
fn tracks_block_parses_and_unmatched_defaults_to_drop() {
    let yaml = r#"
profile_version: 1
input: { pattern: 'x', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }
"#;
    let p = muxsmith_core::profile::load::from_str(yaml, muxsmith_core::profile::load::Format::Yaml)
        .expect("block-form tracks must parse");
    assert_eq!(p.tracks.unmatched, muxsmith_core::profile::model::KeepDrop::Drop);
    assert_eq!(p.tracks.rules.len(), 1);
}

#[test]
fn tracks_unmatched_keep_parses() {
    let yaml = r#"
profile_version: 1
input: { pattern: 'x', extensions: [mkv] }
tracks:
  unmatched: keep
  rules:
    - match: { exact: { type: video } }
"#;
    let p = muxsmith_core::profile::load::from_str(yaml, muxsmith_core::profile::load::Format::Yaml)
        .expect("keep must parse");
    assert_eq!(p.tracks.unmatched, muxsmith_core::profile::model::KeepDrop::Keep);
}
```

(If `from_str`/`Format`/`load` paths differ, mirror the existing helper calls already in `profile_load.rs`.)

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p muxsmith-core --test profile_load tracks_block -- --nocapture`
Expected: FAIL to compile (`p.tracks.unmatched` / `p.tracks.rules` do not exist; `tracks` is still `Vec<TrackRule>`).

- [ ] **Step 3: Add `TracksCfg` and change the field**

In `crates/muxsmith-core/src/profile/model.rs`, change the field at line 34 from `pub tracks: Vec<TrackRule>,` (drop its doc-comment update accordingly) to:

```rust
    /// Track selection/change rules plus the unmatched-track policy
    /// (spec 4.5). Restructured into a block so the policy lives with its
    /// rules, matching `attachments` and `output`/`tags`.
    pub tracks: TracksCfg,
```

Add near `AttachmentsCfg` (after its `impl Default`, ~model.rs:300):

```rust
/// Track handling: the unmatched-track policy plus the ordered rules
/// (spec 4.5). Parallel in shape to [`AttachmentsCfg`], but `unmatched`
/// defaults to `drop`: only rule-matched tracks survive unless the profile
/// opts into `keep` (spec 4.9 asymmetry note).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TracksCfg {
    /// Policy for PRIMARY-file tracks no `rules` entry matches. Defaults to
    /// `drop` (the declarative default). `keep` passes them through
    /// untouched; consumed by `command` (Task 2). Donor tracks are
    /// unaffected: a donor contributes only its rule-selected track.
    #[serde(default = "drop_policy")]
    pub unmatched: KeepDrop,
    /// Ordered track rules; list order defines the output `--track-order`
    /// (spec 4.5). Uniqueness-constrained (spec 2): each rule resolves to
    /// exactly one track.
    pub rules: Vec<TrackRule>,
}

fn drop_policy() -> KeepDrop {
    KeepDrop::Drop
}
```

- [ ] **Step 4: Update the non-test consumers**

Mechanical `profile.tracks` -> `profile.tracks.rules` at the load-bearing sites:
- `planner.rs:286`: `for (i, rule) in profile.tracks.rules.iter().enumerate() {`
- `planner.rs:364`: `for (ri, rule) in profile.tracks.rules.iter().enumerate() {`
- `planner.rs:983`: `let Some(rule) = profile.tracks.rules.get(ri) else {`
- `planner.rs:1031`: `let rule = &profile.tracks.rules[ri];`
- `planner.rs:1169`: `let expr = &mut p.tracks.rules[ri].match_expr;`
- `validate.rs:59`: `if profile.tracks.rules.is_empty() {`
- `validate.rs:63`: `for (i, rule) in profile.tracks.rules.iter().enumerate() {`
- `lint.rs:21`: the `.tracks` access becomes `.tracks.rules`

Leave `ident.tracks` / `source_ident.tracks` (planner.rs:448,1041; identify.rs) untouched: those are `Identification`, not the profile.

- [ ] **Step 5: Migrate every fixture and inline test YAML**

Transformation rule for every `tracks:` sequence: replace

```yaml
tracks:
  - match: ...
```

with

```yaml
tracks:
  rules:
    - match: ...
```

(indent the former list two spaces under a new `rules:` key). Apply to the three fixture files and every inline `tracks:` occurrence listed in **Files** above. For the `validate_semantics.rs::profile()` helper (validate_semantics.rs:5-16) that prepends a `tracks:` header, update the helper to prepend `tracks:\n  rules:` and re-indent the body it wraps, covering all its callers at once.

- [ ] **Step 6: Run the failing test, then the whole suite**

Run: `cargo test -p muxsmith-core --test profile_load tracks_block`
Expected: PASS.
Run: `cargo test --workspace`
Expected: PASS (all migrated). Fix any remaining un-migrated `tracks:` site the compiler/parse errors point to.

- [ ] **Step 7: Update the spec text**

In `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`: in the 4.1 reference example, nest the `tracks:` list under `rules:` and add an `unmatched: drop` line with a comment (`# keep | drop; default drop`); in 4.5, note `tracks` is a `{ unmatched, rules }` block.

- [ ] **Step 8: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`
Expected: all clean.

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
refactor(profile)!: tracks becomes a { unmatched, rules } block (D20)

Restructure tracks from a bare rule list into a block carrying the new
unmatched policy (default drop, behavior-preserving), matching attachments
and the output/tags policy-in-block pattern. Field not yet consumed; Task 2
wires keep semantics.

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

