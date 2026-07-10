# Task 3 report: mkvmerge detection ladder + version floor (D28)

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan5-t3`, branch `plan5-t3`.
Commit: `85cdc62` `feat(core): mkvmerge detection ladder + minimum version floor (D28)`.

(Note: this file previously held a stale report for an unrelated "FIFO queue" task from an earlier
plan iteration that reused the same filename. Replaced in full below.)

## What was implemented

`crates/muxsmith-core/src/capability/runtime.rs`:

- `RuntimeError::TooOld { found: String, minimum: String }` (new variant, data-only).
- `pub const MIN_SUPPORTED: (u64, u64) = (86, 0)` with the empirical-evidence doc comment (see below).
- `Mkvmerge::detect(override_path: Option<&Path>) -> Result<Mkvmerge, RuntimeError>`: ladder
  override (authoritative, no fallthrough on any error) -> PATH via `Mkvmerge::locate()` ->
  `platform_candidates()` in order. A found mkvmerge below `MIN_SUPPORTED` stops the ladder
  immediately with `TooOld` rather than being silently skipped for another candidate; every other
  error at the PATH/platform-candidate rungs falls through to the next rung; exhausting all rungs
  is `NotFound`.
- `Mkvmerge::version_pair(&self) -> Result<(u64, u64), RuntimeError>`, backed by a private
  `parse_version_pair(raw: &str)` pure parser (kept private; the brief's interface list only named
  the method, not a public parser, unlike the pre-existing `parse_list_types`/`parse_list_languages`
  which are `pub` for reasons not evidenced by any external caller either).
- `fn platform_candidates() -> Vec<PathBuf>` (private, `#[cfg(target_os = ...)]`-gated per OS),
  unit-tested directly via the in-file `#[cfg(test)] mod tests` seam.
- `fn enforce_floor(m: Mkvmerge) -> Result<Mkvmerge, RuntimeError>`: shared helper so the version
  query never runs twice for the same candidate.
- Updated `Mkvmerge::locate()`'s doc comment (it previously said platform-candidate probing was
  "deferred to Plan 4"; that's what this task now delivers via `detect`, so the comment was stale).

`crates/muxsmith-core/src/identify.rs`: added a `TooOld` arm to `IdentifyError`'s `Display` match
(pre-existing code, required for exhaustiveness once the new `RuntimeError` variant exists; this
`Display` impl already emitted prose for the other variants, so this isn't a new departure from
core's "no prose" policy, just keeping existing behavior compiling).

`crates/muxsmith-core/tests/mkvmerge_runtime.rs`: `fake_mkvmerge()` helper (`#[cfg(unix)]`, shell
script) plus `detect_prefers_override_over_path`, `detect_reports_too_old_with_found_and_minimum`,
and the gated `detect_none_finds_real_mkvmerge_meeting_the_version_floor`.

CLI call sites (`dry_run.rs`, `identify.rs`, `run.rs`) were **not touched** — they still call
`Mkvmerge::locate()` directly, per the brief.

## TDD evidence

**RED**: added all test code referencing `parse_version_pair`, `platform_candidates`,
`Mkvmerge::detect`, `MIN_SUPPORTED`, `RuntimeError::TooOld` before any of it existed. Compile
failure:

```
error[E0425]: cannot find function `parse_version_pair` in this scope
error[E0425]: cannot find function `platform_candidates` in this scope
error: could not compile `muxsmith-core` (lib test) due to 4 previous errors
```

**GREEN**: after implementation, `cargo test --workspace` passes in full (unit tests in
`runtime.rs` went from 17 to 21; the `mkvmerge_runtime.rs` integration binary from 3 to 6 tests).
Re-ran the full four-command gate 3x in a row with no failures, plus 15x isolated runs of the
`mkvmerge_runtime` test binary (see "flake" note below).

## Empirical evidence (SI-3)

**(a) MIN_SUPPORTED.** `~/Downloads/mkvtoolnix/src/merge/id_result.h` pins
`ID_JSON_FORMAT_VERSION = 20`, matching the schema linked from `doc/man/mkvmerge.xml`
(`mkvmerge-identification-output-schema-v20.json`). `NEWS.md` never states "bumped to 20"
verbatim (unlike v19, v12, v11, v8, v6, which all have an explicit "bumped to N" sentence), so the
release was derived from the schema diff: `doc/json-schema/...-v19.json` vs `...-v20.json` differ
only in replacing five enumerated `tag_*` track properties (`tag_artist`, `tag_bitsps`, `tag_bps`,
`tag_fps`, `tag_title`) with an open `patternProperties: { "^tag_": ... }` / `additionalProperties:
true`. `NEWS.md` under "Version 86.0 'Winter' 2024-07-13" records exactly that change: "mkvmerge:
Matroska reader: track statistics tags are included in the JSON identification output ... as part
of the track properties, prefixed with `tag_`." No schema-affecting NEWS entry exists between v82.0
(explicit bump to schema v19) and v86.0, so **v86.0 is MIN_SUPPORTED = (86, 0)**.

**(b) platform_candidates().** Verified against `~/Downloads/mkvtoolnix/packaging/`:
- Windows: `packaging/windows/installer/mkvtoolnix.nsi` — `PRODUCT_NAME = "MKVToolNix"`,
  `InstallDir` is `$PROGRAMFILES64\MKVToolNix` for the 64-bit installer, `$PROGRAMFILES\MKVToolNix`
  for the 32-bit one -> both `%ProgramFiles%\MKVToolNix\mkvmerge.exe` and
  `%ProgramFiles(x86)%\MKVToolNix\mkvmerge.exe` kept (env-var-driven, cfg'd to `target_os =
  "windows"`).
- macOS: `packaging/macos/config.sh` (`APP_BUNDLE_NAME="MKVToolNix.app"`, fixed name, no version
  suffix — the brief's glob guess `MKVToolNix-*.app` doesn't match reality, corrected) and
  `packaging/macos/build.sh` (`Contents/MacOS/{mkvmerge,...}` inside the bundle; its own
  `README.macOS.txt`, written by `build_dmg` and shipped in the DMG, explicitly tells users to copy
  the CLI tools to `/usr/local/bin`) -> kept `/Applications/MKVToolNix.app/Contents/MacOS/mkvmerge`
  and `/usr/local/bin/mkvmerge`.
- Linux: `packaging/debian/mkvtoolnix.install` and
  `packaging/centos-fedora-opensuse/mkvtoolnix.spec` (`%{_bindir}`) both place mkvmerge at
  `/usr/bin`; the project's own `INSTALL` (unmodified autotools boilerplate — `configure.ac` has no
  `AC_PREFIX_DEFAULT` override) documents `/usr/local` as the default `./configure` prefix -> kept
  `/usr/bin/mkvmerge` and `/usr/local/bin/mkvmerge`.
- **Dropped, with reasons in the doc comment**: Homebrew (`/opt/homebrew/bin`) — no Homebrew
  formula exists anywhere in mkvtoolnix's own source tree (that packaging lives in the separate
  `homebrew-core` repo, unverifiable from this checkout). Flatpak
  (`/var/lib/flatpak/exports/bin/org.bunkus.mkvtoolnix-gui`) — `packaging/` ships no Flatpak
  manifest, and that app ID names the GUI, not a standalone `mkvmerge` CLI binary anyway.

## Files changed

- `crates/muxsmith-core/src/capability/runtime.rs` (+228/-4)
- `crates/muxsmith-core/src/identify.rs` (+4)
- `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (+96 net, includes a flake-mitigation helper, see below)

## Self-review

- **Completeness**: every item in the brief's Interfaces list is present with the exact signatures
  requested. Step 1-4 all done.
- **YAGNI**: kept `parse_version_pair` and `platform_candidates` private (brief only asked for the
  `version_pair` method and a private-but-tested `platform_candidates`); didn't add a `NotFound`
  unit test for "override None + PATH empty + all platform candidates missing" — see limitation below.
- **Known minor inefficiency, not fixed**: the PATH rung in `detect()` calls `Mkvmerge::locate()`
  (which itself spawns `mkvmerge --version` once to check presence) and then `enforce_floor()`
  spawns a second time to get the parseable version string. This is a deliberate tradeoff: the
  brief's ladder spec explicitly routes the PATH rung "via `locate()`", and `locate()`'s signature
  can't change without touching CLI-visible behavior. The extra spawn only happens when PATH-located
  mkvmerge is real and healthy (the common case), costs a few ms, and avoids duplicating
  `locate()`'s Spawn->NotFound mapping logic in two places.
- **Design decision surfaced (not explicit in the brief)**: an explicit `override_path` is
  authoritative — any failure (spawn, non-zero, parse, too-old) is returned directly, never falls
  through to PATH/platform candidates. Rationale: a user-configured override is a directed choice;
  masking its failure by silently trying something else would hide a misconfiguration the GUI needs
  to surface. PATH and platform candidates, being automatic/best-effort rungs, do fall through on
  everything except `TooOld` (which is real, actionable signal worth stopping on immediately rather
  than potentially finding nothing further down the ladder). If this isn't the intended UX for T7,
  it's a one-line change (`detect`'s first `if let Some(path) = ...` block).
- **Real flake found and fixed, not brushed under the rug**: `detect_prefers_override_over_path`
  and `detect_reports_too_old_with_found_and_minimum` write a fresh executable script and
  immediately exec it. Under `cargo test --workspace`'s full parallel load this hit
  `ErrorKind::ExecutableFileBusy` ("Text file busy", errno 26) roughly 1 run in 3 — a documented
  Linux kernel race between a `close()`'s writecount release and a near-simultaneous `execve()` from
  another thread doing the same write-then-exec dance. Fixed with a bounded warm-up retry inside
  `fake_mkvmerge()` (not in production code — `detect`/`enforce_floor` are untouched by this).
  Verified: 0 failures in 15 isolated runs and 3 full-workspace runs after the fix, versus
  reproducible failures before it.
- **Test rigor gap, left open deliberately**: no unit test for "nothing found anywhere -> NotFound".
  Forcing that condition safely would require mutating the test process's global `PATH` env var
  in-process, which races other parallel tests in the same binary that also call `Mkvmerge::locate()`
  (the CLI integration tests avoid this by spawning the compiled `muxsmith` binary as a child
  process with a scoped env var — not available here since this is a direct in-process API test).
  The `NotFound` path itself is a single, simple `Err(RuntimeError::NotFound)` after the loop with
  no branching logic, so the risk of it being wrong is low, but it is not independently exercised.
- **Gate**: `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace
  --all-targets -- -D warnings`, `cargo deny check` all green, run in the foreground, re-verified 3x.
- **ASCII-only, `#![deny(missing_docs)]`**: confirmed via the clean clippy/build output (missing_docs
  is enforced at the lib crate root and would fail the build otherwise).

## Concerns for review

1. The override-hard-fail-vs-fallthrough design choice above is an interpretation, not literally
   spelled out in the brief; flagging it explicitly in case T7's GUI wants different semantics.
2. `platform_candidates()`'s Windows and macOS branches are untestable on this Linux dev machine;
   they compile and are reasoned through against the packaging sources but have not been executed.
   CI's 3-OS matrix (`ci.yml` matrix over `ubuntu-26.04`, `windows-2025`, `macos-15`) will be the
   first real exercise of those branches.
