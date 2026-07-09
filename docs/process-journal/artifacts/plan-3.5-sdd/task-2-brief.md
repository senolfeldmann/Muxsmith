### Task 2: `keep` semantics in the planner and `command`

`tracks.unmatched: keep` keeps all of the PRIMARY's tracks. The `Plan` gains a boolean, populated from the policy; `command` skips the primary group's track-selection flags when set, so mkvmerge passes unmatched primary tracks through. Matched tracks still get their property options and `--track-order` position; donor groups are unchanged.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (add `Plan.keep_unmatched` field at the struct ~130-146; populate at the construction site ~528-536)
- Modify: `crates/muxsmith-core/src/command.rs:185` (`push_track_selection`)
- Test: `crates/muxsmith-core/tests/command.rs` (golden argv)

**Interfaces:**
- Consumes: `Profile.tracks.unmatched: KeepDrop` (Task 1); `Plan.source: PathBuf` (the primary path); `Assignment { source, track_id, track_kind, changes }`.
- Produces: `Plan.keep_unmatched: bool`.

- [ ] **Step 1: Write the failing test** (primary selection flags suppressed under keep)

In `crates/muxsmith-core/tests/command.rs` add a test that builds a `Plan` with `keep_unmatched: true`, one primary source, one matched audio assignment (track_id 1) among a source that also has unmatched tracks, and asserts the argv contains NO `--no-video`/`--no-subtitles`/`--audio-tracks`/`--no-buttons` for the primary, but DOES contain the matched track's property option and `--track-order`:

```rust
#[test]
fn keep_unmatched_suppresses_primary_selection_flags() {
    let plan = Plan {
        source: PathBuf::from("/m/show.mkv"),
        output: PathBuf::from("/out/show.mkv"),
        keep_unmatched: true,
        assignments: vec![Assignment {
            rule_index: 0,
            source: PathBuf::from("/m/show.mkv"),
            track_id: Some(1),
            track_kind: Some("audio".into()),
            changes: vec![AppliedChange { property: "default_track".into(), value: Scalar::Bool(true) }],
        }],
        attachments: AttachmentPlan { primary: PrimaryAttachments::KeepAll, add_files: vec![] },
        chapters: ChapterSource::Keep,
        tags: TagFlags { global_keep: true, track_keep: true },
        title: TitleAction::Keep,
    };
    let argv = command(&plan);
    assert!(!argv.iter().any(|a| a == "--no-video" || a == "--no-subtitles"
        || a == "--no-buttons" || a == "--audio-tracks"),
        "keep must emit no primary selection flags, got {argv:?}");
    assert!(argv.windows(2).any(|w| w[0] == "--default-track-flag" && w[1] == "1:1"));
    assert!(argv.iter().any(|a| a == "--track-order"));
}
```

(Match the exact constructor field set to the current `Plan`/`Assignment` definitions; the drop-path golden tests already in this file show the shapes.)

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p muxsmith-core --test command keep_unmatched -- --nocapture`
Expected: FAIL to compile (`Plan` has no `keep_unmatched` field).

- [ ] **Step 3: Add the `Plan` field and populate it**

In `planner.rs`, add to `Plan` (after `title`, ~line 145):

```rust
    /// When true (`tracks.unmatched: keep`), the primary's unmatched tracks
    /// pass through: `command` emits no track-selection flags for the primary
    /// group. Donor groups are unaffected. Default construction is `false`
    /// (drop).
    pub keep_unmatched: bool,
```

At the construction site (~planner.rs:528), add the field (the fn has `profile` in scope):

```rust
        keep_unmatched: matches!(profile.tracks.unmatched, crate::profile::model::KeepDrop::Keep),
```

- [ ] **Step 4: Skip primary selection under keep in `command`**

In `command.rs`, at the top of `push_track_selection` (line 185):

```rust
fn push_track_selection(argv: &mut Vec<String>, plan: &Plan, source: &Path) {
    // tracks.unmatched: keep -> pass all PRIMARY tracks through (no selection
    // flags); mkvmerge keeps every track by default. Donor groups still get
    // their normal per-category selection.
    if plan.keep_unmatched && source == plan.source.as_path() {
        return;
    }
    for cat in &CATEGORIES {
        // ... unchanged ...
```

- [ ] **Step 5: Run the test, then the suite**

Run: `cargo test -p muxsmith-core --test command keep_unmatched`
Expected: PASS.
Run: `cargo test --workspace`
Expected: PASS (drop-path goldens unaffected; `keep_unmatched: false` everywhere else).

- [ ] **Step 6: Gate and commit**

Run: `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace && cargo deny check`

```bash
git add -A
git -c commit.gpgsign=false commit -m "$(cat <<'EOF'
feat(planner,command): tracks.unmatched keep passes primary tracks through (D20)

Plan carries keep_unmatched from the policy; command skips the primary
group's selection flags under keep, so mkvmerge keeps unmatched primary
tracks. Matched tracks still get property options and track-order.

Co-Authored-By: <session model> <noreply@anthropic.com>
EOF
)"
```

---

