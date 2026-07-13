# Verify-40 (slice F4): settings.rs dead Windows `ends_with` fallback

**Verdict: CONFIRMED**

## Finding

`src-tauri/src/settings.rs` — the test
`settings_path_lives_under_a_muxsmith_subdirectory` (tag `native`) asserts
`path.ends_with("muxsmith/settings.json") || path.to_string_lossy().ends_with("muxsmith\\settings.json")`,
the second branch commented "Windows path separator." Claim: `Path::ends_with`
matches whole components and `std::path` treats `/` as a separator on Windows
too, so branch 1 already covers Windows and the fallback is dead on all
platforms.

**Proposed replacement:** `assert!(path.ends_with("muxsmith/settings.json"));`

## (a) Cited code matches — YES

HEAD `2f17880`, lines 316-326:

```rust
#[test]
fn settings_path_lives_under_a_muxsmith_subdirectory() {
    if let Some(path) = settings_path() {
        assert!(
            path.ends_with("muxsmith/settings.json") || {
                // Windows path separator.
                path.to_string_lossy().ends_with("muxsmith\\settings.json")
            }
        );
    }
}
```

Verbatim the described `||` fallback. `settings_path()` (line 112-114):
`dirs::config_dir().map(|dir| dir.join("muxsmith").join("settings.json"))`, so
every `Some` value has exact trailing components `["muxsmith", "settings.json"]`.

## (b) Replacement is current idiom / dead-branch claim holds — YES

Load-bearing std semantics verified against current docs (WebSearch →
doc.rust-lang.org stable), not memory:

- On Windows `std::path` recognizes **both** `\` and `/` as separators
  (`SEPARATORS = &['\\', '/']`; `/` primary only on Unix).
- `Path::ends_with` "only considers whole path components to match", case
  sensitive on every platform.

So on Windows the argument `"muxsmith/settings.json"` parses to components
`["muxsmith", "settings.json"]` (the `/` is a recognized separator), and the
real `\`-joined path's trailing components are `[..., "muxsmith", "settings.json"]`.
Branch 1 is **true on Windows**, so the `||` fallback never evaluates — dead on
all platforms. `Path::ends_with` is stable since Rust 1.0, valid under the
pinned 1.96.1 / edition 2024 toolchain, and is the std/native primitive the
`native` tag calls for. Replacement is exactly right.

## (c) No load-bearing difference saving the fallback

For the fallback to be live, branch 1 would need to be false while branch 2 is
true on the genuine value. It cannot: the real path is literally
`.../muxsmith/settings.json`, so component matching always succeeds regardless
of the OS-rendered separator. The string branch is in fact *laxer* (a
`foomuxsmith\settings.json` would false-pass it while failing component
matching) but never runs because branch 1 already matched, and
`settings_path()` constructs the component as exactly `muxsmith` via
`join("muxsmith")`. The fallback adds zero coverage.

## (d) N/A

Tag is `native`, not `yagni`; a concrete construct and concrete replacement are
both named anyway.

## Decision guard — not tracked, no conflict

Grepped `docs/superpowers/specs/*.md` (D1-D35, 8 files), `docs/IDEAS.md`,
`docs/ROADMAP.md` for `ends_with` / `settings_path` / `muxsmith_subdirectory` /
`settings.rs` / windows separator / this test name.

- Only spec separator hit is the v1-design filename invariant
  (`PathSeparatorInRenderedName`) — a different construct (rendered output
  names), unrelated to this test.
- ROADMAP **Test-hygiene collection (B-minors, 307-321)**: B1,B2,B4,B6,B7,B8,
  B9,B10,B11 (+ discarded B3,B13) — none is this test. B8 touches settings.rs
  *naming*, not this dead branch.
- ROADMAP **Cosmetic cleanup group K (260-267)**: `at` param, template
  mislabel, TracksCfg placement, stale module doc, Plan-1 remnants, eager
  chapters/attachments resolve — not this branch.
- No design decision marks the Windows fallback as deliberate.

Genuinely untracked. No DECISION_CONFLICT, no TRACKED.
