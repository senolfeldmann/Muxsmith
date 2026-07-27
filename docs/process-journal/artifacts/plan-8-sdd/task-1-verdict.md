# Task 1 verdict (independent review): D87 version sync

Reviewed commit `7e36f96` on branch `plan8-a`, worktree
`/home/senol/Git/Muxsmith/.worktrees/plan8-a`, base `aec4cef`.
Inputs: `task-1-brief.md`, `task-1-report.md`, `review-aec4cef..7e36f96.diff`,
`implementer-preamble.md`, design
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(D87 lines 1016-1078, section 3.1 lines 1478-1528, section 3.3 lines 1539-1567,
section 8 lines 1849-1936, section 11 lines 1989-2029).

**Citation pin.** The design file was amended by the controller during this review
(amendment A1, Task 4's YAML-quoting fork, 2026-07-27 11:40) and grew one line above
section 1, shifting every later line by +1. All design line numbers in this verdict
are anchored to the post-A1 file, sha256
`734490f8d7186391949cd22ef875e69bc3e41f502645287fc6b8d03c27638af3`
(as committed in master `d21a19f`)
(`code-comment-line-citations-drift`). A1 touches nothing Task 1 transcribes: the 3.3
script block is byte-unchanged by it, verified by re-diffing after the amendment.
Plan citations are to `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`,
which is unmodified.

## Verdict 1 - spec compliance: APPROVED

- **Transcription is byte-faithful.** `sed -n '1542,1566p'` of the design (3.3 body,
  fences excluded) and the landed `scripts/check-version-sync.sh` share
  sha256 `c90cf5bd92859dbe4590279d7d7f09a62b2b16f8eefc9d1a7b1a5a076213d82c`;
  `diff` reports zero differences (run both pre- and post-A1). The brief's carried
  block is identical to the design's, so both comparisons pass against the same
  bytes. Shebang present, mode `100755` in the tree object (load-bearing: the guard
  job invokes the path directly, design line 1279).
- **tauri.conf.json: the key is deleted, not moved** (section 11 line 2008).
  `git diff --name-status aec4cef..HEAD` = exactly `A scripts/check-version-sync.sh`,
  `M src-tauri/tauri.conf.json`; the config diff is the single line
  `"version": "0.1.0",` removed, nothing else. `jq 'has("version")'` -> `false`;
  `jq -r 'keys_unsorted | join(",")'` -> `$schema,productName,identifier,build,app,bundle`,
  matching the brief's expected string exactly. `Cargo.toml` untouched (the value
  was not relocated).
- **Nothing extra.** No BUILDING.md/README mention, no ledger entry, no ci.yml
  contact, no manifest change, no new dependency. The bundle surface of design 3.1
  is Task 2's transcription target (plan lines 57, 71, 204-218), so its absence here
  is the plan's own stream-A serialization, not an omission.
- **Scope guards clean.** No non-ASCII byte in either file (`grep -nP '[^\x00-\x7F]'`
  empty over both), no CRLF, no trailing whitespace. Commit unsigned (`%G?` = `N`),
  trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present, message
  verbatim as the brief prescribes.
- **Review package integrity.** `review-aec4cef..7e36f96.diff` body is exactly
  `git diff -U10 aec4cef..7e36f96` (byte-identical; the default -U3 rendering differs
  only in context width).
- **Brief fidelity.** `task-1-brief.md` equals plan lines 119-202 with one trailing
  blank line added; no drift between plan and dispatch.

## Verdict 2 - task quality: APPROVED

All four load-bearing verification claims reproduce independently against the
committed text, worktree clean before and after every run.

| Check | Reviewer result | Report claim |
|---|---|---|
| G1 red (`... v9.9.9`) | `version-sync: tag v9.9.9 != v0.1.0`, exit 1 | matches |
| G1 green (plain) | `version-sync: OK (0.1.0)`, exit 0 | matches |
| G2 (version key reinjected) | `version-sync: src-tauri/tauri.conf.json declares 'version'; it must inherit from Cargo.toml (D87)`, exit 1 | matches |
| G3 (package.json 0.1.1) | `version-sync: Cargo.toml (0.1.0) != package.json (0.1.1)`, exit 1 | matches |
| jq key checks post-revert | `false` / `$schema,productName,identifier,build,app,bundle` | matches |
| `cargo check -p muxsmith-gui` | exit 0, `Compiling muxsmith-gui` + `Finished`, zero warning/error lines | true, but see m1 |

Restoration verified byte-exact after each fire-test: `src-tauri/tauri.conf.json`
back to sha256 `1651535c86a1bf6a60755923b2abfbde1147fe4ce9fe0e9e8cc590e9243019e3`,
`package.json` back to `51f215e69fda43a72ab4ce6b154be97a6a0c9417515821335e608e175f17602c`,
`git status --short` empty, `HEAD` still `7e36f96`.

### The Cargo-inheritance claim: what the landed state proves, and what is deferred

**Proved by the landed state.** (1) The version-less config parses and validates:
`cargo check -p muxsmith-gui` genuinely re-ran the build script in my run (the
config's mtime changed, and the script declares
`cargo:rerun-if-changed=.../src-tauri/tauri.conf.json`), recompiling the crate with
zero diagnostics. I fire-verified that this check can fail: with `"productName": 42`
injected, `cargo check` exits 101 with
`invalid type: integer 42, expected a string` from the build script - so "clean" is
a real signal, not a cached no-op. Restored, re-checked clean. (2) The absence itself
is guard-enforced and its red state fires (G2).

**Not proved by the landed state, and correctly so:** no artifact is built in
Task 1, so nothing here shows a *built* app self-reporting the workspace version.
That deferral is the design's own stated split: D87 lines 1049-1051 route
artifact-level confirmation to the rehearsal, section 8 R9 (lines 1926-1928) names
it as a Task 6 observable, and the plan records the dependency edge `1 -> 6`
(plan line 60). Brief Step 1 explicitly forbids re-verifying the schema fallback
(design 1.1, lines 62-64).

**Beyond the brief, for the controller's confidence:** the fallback is confirmed in
the pinned dependency sources, not only in the schema's prose description.
`tauri-codegen-2.6.3/src/context.rs:273-278` - when `config.version` is `None`,
`PackageInfo.version` becomes `env!("CARGO_PKG_VERSION")`, and `src-tauri/Cargo.toml:3`
carries `version.workspace = true`, so the app's runtime self-report is the workspace
version. `tauri-build-2.6.3/src/lib.rs:632-640` sets the Windows exe resource
version fields *only* when `config.version` is `Some`; with the key deleted that
block is skipped and `tauri-winres-0.3.6/src/lib.rs:148-153` (plus 174-189 for the
numeric FILEVERSION) supplies `CARGO_PKG_VERSION` as the default - i.e. still the
workspace version, so deleting the key opens no gap in exe metadata. This closes the
mechanism question at code level for the app; the *bundler's* package metadata
(msi/deb/rpm version fields) is produced by the npm-shipped tauri CLI and still rests
on the design-time schema evidence.

## Findings by severity

No blocking findings. Nothing here requires a fix round.

**m1 (minor, report only, no code defect) - `task-1-report.md:101-103`: the
zero-warnings evidence is vacuous as recorded.** Two problems in one block.
(a) `cargo check -p muxsmith-gui 2>&1 | grep -i "warning\|error"` on a warm cache
recompiles nothing, so no diagnostic can be emitted whatever the code's state - the
check passes for free (`proc-verification-step-must-be-falsifiable`; the house rule
on absence-shaped checks). (b) The line `NO WARNINGS OR ERRORS` sits inside a
transcript block, but no grep can emit it; it is the implementer's conclusion
typeset as command output. The underlying claim is nonetheless **true** - I
established it with a forced build-script rerun plus the `productName: 42` positive
control above. Owed: nothing in the code; the report line is the defect, and the
lesson generalizes (see HARVEST H1).

**i1 (informational, closes a preamble duty) - G1-G3 pre-merge re-run against the
committed text is now done.** Preamble line 17 and design section 11 line 2009
require the fire-tests re-run pre-merge against the *committed* text; the
implementer ran them pre-commit (report section "Step 3", before `git commit`).
My re-runs above executed against `7e36f96` with a clean tree, so the controller
should count this duty as satisfied by this review rather than scheduling a third
run.

**i2 (informational, design-settled, no action) - guard parse edge shapes all fail
closed.** Probed on scratch trees (worktree untouched): a trailing comment on the
version line yields a mangled value (`Cargo.toml (0.1.0 # bump me) != package.json
(0.1.0)`, exit 1); an unspaced `version="0.1.0"` does not match `/^version = /` and
trips the parse guard (exit 1); `version` absent from `[workspace.package]` but
present in a later section is *not* misread (block scoping holds, exit 1); a
`package.json` without `version` yields `!= package.json (null)`, exit 1. Every
deviation is a false *positive* (a blocked release), never a silent green. The text
is design-carried verbatim, so changing it would need a design amendment; recorded
as a known-shape list, not a defect.

**i3 (informational, for the controller, not for this task) - D87's bundler-level
claim has no acceptance observable.** D87 lines 1044-1051 assert that every
artifact's self-reported version IS the workspace version, with bundle metadata
coming from Cargo via the fallback. The plan's observables do not reach that:
R9 (design 1926-1928) reads `./muxsmith --version` from the tar.gz, which is the CLI
crate's clap version (design 1.7, `crates/muxsmith-cli/src/cli.rs:10`,
`version.workspace = true`) - a Cargo-native path that would report the workspace
version even if the Tauri fallback misbehaved. R6 (design 1904-1912) reads deb/rpm
Recommends, Depends and contents, not their Version fields. The rename step globs
`target/release/bundle/<format>/*.<ext>` (design 1398-1409), so the bundler-native
names - which do carry the bundler's version, per the design's own example
`Muxsmith_0.1.0_x64_en-US.msi` (design 1160) - are logged by `pick:` on stderr
(design 1391) but never asserted. Cheapest close, if the controller wants one:
have R1 assert that each `pick:` path contains the guard's `$version`. Task 4/6 or
owner scope; explicitly out of Task 1's frozen scope, and no fix is requested here.

**i4 (informational, no action) - the awk parser is restated three times** in the
plan's landed surface: the script (design 3.3 line 1553) and release.yml twice
(design lines 1380, 1448), where the guard job could instead export the parsed
version as a job output. This is a settled design decision that section 11 forbids
reopening; recorded only so the restatement is on the record rather than discovered
later as drift risk.

Note: `shellcheck` is not installed on this machine and neither CI nor the nine-part
gate lints shell, so the script carries no lint verdict. Not owed by the brief.

## HARVEST

**H1 (process, generalizes past this plan) - a re-run on a warm build cache is not
evidence of a clean compile.** `cargo check` (and any incremental build tool) emits
diagnostics only for units it actually rebuilds, so `build | grep -i warning` as a
*second* invocation is an absence check that cannot fail. The falsifiable form is:
force the unit to rebuild (touch a declared `rerun-if-changed` input or the source),
capture the full output, and pair it with one deliberate break that shows the check
firing. Worth a nature-file entry alongside
`proc-verification-step-must-be-falsifiable`, since the shape recurs in every
Rust/Node task brief that ends with "expected: clean".

**H2 (process) - transcript blocks in reports carry only real output.** Writing a
conclusion (`NO WARNINGS OR ERRORS`) inside a `$ command` block makes an unfalsifiable
claim look like a measurement. Either paste the actual output (including "no output")
or state the conclusion outside the block. Same class as the house rule on quoted
wording and measured numbers.

**H3 (plan mechanics, worked well - keep) - carrying the full artifact text in the
brief made this task verifiable in minutes.** Because the brief's block equals the
design's block, transcription review reduced to one sha256 comparison, and the
expected outputs recorded in the brief (`false`, the exact key list) made every step
a string equality rather than a judgment. The pattern to keep for cheap-tier tasks:
carried text plus recorded expected outputs plus named fire-tests.

**H4 (empirical, reusable) - the cheapest positive control for a Tauri config change
is a type error.** `"productName": 42` makes `cargo check -p <gui-crate>` exit 101
with `invalid type: integer 42, expected a string` out of `tauri_build::build()`,
proving the build script really parses and validates `tauri.conf.json` on every
compile. Cheaper and more precise than any `tauri build`, and reusable for Task 2's
much larger config rewrite (where "the JSON is valid" via `python3 -m json.tool` is
strictly weaker than "Tauri accepts the schema").

**H5 (empirical, tauri 2 version fallback, pinned versions)** -
`tauri-codegen-2.6.3/src/context.rs:273-278` (config version `None` ->
`env!("CARGO_PKG_VERSION")`), `tauri-build-2.6.3/src/lib.rs:632-640` (Windows
resource version set only when the config declares one) and
`tauri-winres-0.3.6/src/lib.rs:148-153`, `:174-189` (defaults from
`CARGO_PKG_VERSION*`) together confirm D87's mechanism in code for both the runtime
self-report and Windows exe metadata. Worth citing in the project record so the
claim no longer rests on the schema description alone. Line citations are pinned to
these exact crate versions from `Cargo.lock`
(`code-comment-line-citations-drift` applies if the pins move).

**H6 (environment, already ledgered for Task 3, confirmed again) - `cp` is
interactive in this shell.** A plain `cp` inside an `&&` chain prompts, gets EOF,
and silently leaves the file unrestored - a fire-test restore that appears to have
run but did not. Use `command cp -f` (or `git checkout --`) in restore steps, and
always verify restoration by hash plus `git status`, never by the absence of an
error message.
