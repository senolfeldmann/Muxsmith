# Plan-9 close pass - implementer report

**Commit:** `9dc3a4d` on `master`, main worktree, nine files, tree clean after.
No behaviour change; no file outside the brief's Files list touched. All runs
foreground. No session-relocation tool called.

Baseline before any mutation: `git status --porcelain` empty at `7aa9f99`
(pasted in the item-6 fire below, where the pre-fix doc run was taken on that
clean tree). No `cp`, no restore needed - nothing was mutated and reverted.

## Per item

### Item 1 - `src/views/BatchView.vue`, the `!doc.profile` else arm

Before:

```
        // Contract violation (D42's `load_profile` envelope): `profile:
        // null` is documented to always pair with a lead diagnostic
        // explaining why. An empty `config_diagnostics` here means core
        // broke that contract -- there is no diagnostic to surface through
        // the shared alert line, so at minimum this stops being a silent
        // no-op.
        console.error(
          "[batch] load_profile returned profile: null with no diagnostics",
```

After:

```
        // Contract violation (D42's `load_profile` envelope): `profile:
        // null` is documented to always pair with a lead `parse-error`
        // diagnostic explaining why. The fetch above is a code-keyed
        // `find`, so two shapes reach this arm and both are that
        // violation: an empty `config_diagnostics`, and a non-empty one
        // carrying no `parse-error` entry. Neither leaves a `parse-error`
        // diagnostic to surface through the shared alert line, so at
        // minimum this stops being a silent no-op.
        console.error(
          "[batch] load_profile returned profile: null with no parse-error diagnostic",
```

Logic untouched. Checked that no test or source consumes the old string:
`git grep -n "with no diagnostics|load_profile returned profile"` over `src/
e2e/ crates/ src-tauri/ docs/` hits only dated journal artifacts
(`plan-6-sdd/fix-wave-report.md`, a plan-6 review diff, the plan-9 task-5
report and verdict) plus the changed line itself. The Task-5 verdict had run
the same grep over `e2e/ src/` for the same reason.

Contract check behind the wording: `load_profile_body`
(`src-tauri/src/lib.rs:286-300`) puts exactly one diagnostic into the
`profile: null` document, the `Err(d)` from `load::from_file`, so "lead
`parse-error` diagnostic" is what the envelope promises and both shapes that
reach the else arm are violations of it.

### Item 2 - `crates/muxsmith-cli/tests/dry_run_cli.rs`, LOW-4's three texts

Applied verbatim from the Task-5 delta verdict's LOW-4 "Exact required
change" block (`.superpowers/sdd/plan-9/task-5-verdict.md:906-919`). Comment
before/after and both messages:

- comment: "Planning ran, so this document came from `batch_document`, not
  the config-only shape: `files` is present and `mkvmerge_found` absent." ->
  the verdict's four-line "Shape guards." comment.
- new message: `"expected a planned batch document with a files array, got:
  {report}"` -> `"expected a report document carrying a files array, got:
  {report}"`.
- pre-existing message: `"expected a planned batch document, got: {report}"`
  -> `"expected a planned batch document, not the mkvmerge-missing
  config-only shape, got: {report}"` (this pass carries the licence the fix
  round lacked).

Assertions unchanged. `dry_run_cli.rs` still reports **13 passed**, the
test-count invariant the verdict named.

One factual wrinkle in the fenced text, raised as concern 1 below rather than
edited: I re-measured the real profile-load-failure document myself
(`./target/debug/muxsmith dry-run /nonexistent-profile.yaml --json`, exit 2)
and it carries `files`:

```
keys: ['batch_diagnostics', 'config_diagnostics', 'files', 'suggestions']
files present: True value: []
mkvmerge_found present: False
codes: ['parse-error']
```

### Item 3 - `crates/muxsmith-core/src/identify.rs`, the `IdentifyCache` doc

Before: "In-memory identification cache, constructed per planning call and
dropped with it (spec 5.5)."

After: "In-memory identification cache, constructed per call and dropped with
it (per planning call in the pipeline seam, per invocation on the CLI and GUI
identify surfaces; spec 5.5)." Rest of the doc unchanged.

Shape follows `core-docs-name-callers-illustratively-never-exclusively`'s own
handle, read at `docs/decision-ledger.yaml:4715`: state the property, then
illustrate per surface in a parenthetical.

**Construction sites, measured myself** (`git grep -nE
"IdentifyCache::(new|default)"` plus `git grep -n "LiveIdentifier"` and `git
grep -n "cache:"` over `*.rs`, then read of each enclosing function):

| Site | Enclosing function | Lifetime |
|---|---|---|
| `crates/muxsmith-core/src/pipeline.rs:127` | `plan_pipeline` (`:97`), via `LiveIdentifier` | per planning call, stack-local, dropped on return |
| `crates/muxsmith-cli/src/commands/identify.rs:21` | `commands::identify::run` (`:13`) | per `muxsmith identify` invocation |
| `src-tauri/src/lib.rs:255` | `identify_body` (`:250`) | per `identify` IPC invocation |

Three production sites, not four (concern 2). Test constructions, for
completeness: `crates/muxsmith-core/tests/command_integration.rs:232`,
`:494`, `crates/muxsmith-core/tests/identify_live.rs:42`. The only other
occurrence, `identify.rs:327`, is `new()`'s own `default()` body.
`plan_pipeline` has three production callers (CLI dry-run, CLI run, GUI
`dry_run` body) but constructs the cache once per call, so it is one site
with three entry paths.

### Item 4 - `e2e/jobsview-reset.spec.ts`, the spec-local IPC installer's doc

Added as the doc comment's closing paragraph, one sentence:

```
 * Relative to `installMockIPC` it deliberately answers a narrower surface
 * -- no `__TAURI_OS_PLUGIN_INTERNALS__.platform` global, no forwarding to
 * the Node-side `__muxsmithRecordInvoke__` log, and no `get_settings` /
 * `set_settings` / `plugin:fs|write_text_file` answers -- which is safe
 * today because this spec mounts `JobsView` alone on a blank page instead
 * of driving the served app: `platform()` is `FirstRun.vue`'s, the
 * settings pair belongs to `main.ts`'s locale bootstrap and
 * `SettingsDialog.vue`, the file write sits behind `RunHistory.vue`'s
 * user-triggered log export, no mount here reaches any of them, and every
 * test in this file asserts DOM state rather than a recorded call log --
 * with the unmocked-command throw below as the backstop the day one of
 * those stops holding.
```

**Omission list measured against `e2e/mocks.ts`, not copied from the brief.**
`installMockIPC` (`mocks.ts:84-133`) does five things; `installSoftOutcomeIPC`
(`jobsview-reset.spec.ts:99-120` before my edit) does two of them:

| `installMockIPC` | spec-local |
|---|---|
| `mockWindows("main")` | same |
| `mockIPC(..., { shouldMockEvents: true })`, unmocked command throws | same |
| sets `window.__TAURI_OS_PLUGIN_INTERNALS__ = { platform: ... }` (`:98`) | absent |
| forwards every call to `window.__muxsmithRecordInvoke__` (`:102`) | absent |
| answers `list_runs`, `get_settings`, `set_settings`, `plugin:fs|write_text_file` (`:114-128`) | answers `start_run` and `list_runs` only |

Safety verified at the consumers, not assumed: `platform()` is imported only
in `src/views/FirstRun.vue:3`; `getSettings`/`setSettings` only in
`src/components/SettingsDialog.vue:4` (plus `main.ts`'s locale bootstrap,
which the mount harness does not run); `writeTextFile` only in
`src/components/RunHistory.vue:17,125`, behind the save-dialog log export.
`e2e/mount-entry.ts`'s glob mounts editor widgets, `EditorView` and
`JobsView` only, on a `page.setContent` blank page - no `page.goto` of the
served app - and `RunHistory` is mounted as `JobsView`'s child
(`JobsView.vue:28,333`), which is why `list_runs` is the one incidental
command the spec-local handler does answer.

### Item 5 - `src-tauri/src/lib.rs`, two ambiguous intra-doc links

- `:54` `[`run`]'s `start_run`` -> `[`mod@run`]'s `start_run``. `start_run` is
  defined in `src-tauri/src/run.rs:400`, so the sentence points at the module.
- `:87` "`.manage`d once in [`run`]" -> "[`run()`]". `.manage(AppState::default())`
  is at `src-tauri/src/lib.rs:543`, inside `pub fn run()` at `:537`, and it is
  the only `.manage(` in `src-tauri/` - so the sentence points at the function.

### Item 6 - the gate's own definition, both consuming sites

`BUILDING.md`:

- gate block line: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`
  -> `... --no-deps --document-private-items`.
- the rustdoc paragraph gains a sentence on what the flag buys (private items
  are neither rendered nor carried into link resolution without it), naming
  the `run` module/function ambiguity as the case it caught.
- new `### House-knowledge check` subsection after `### Frontend checks`, with
  `python3 scripts/ledger-lint.py` and its binding statement (pre-push, house
  rule `ledger-lint-runs-before-every-push`, PyYAML, CI runs it as its own
  job).

`.github/workflows/ci.yml`:

- `run: cargo doc --workspace --no-deps` -> `... --no-deps --document-private-items`
  (the single `cargo doc` step, confirmed by parsing the workflow: `yaml.safe_load`
  over all jobs' steps finds exactly one step whose `run` contains `cargo doc`,
  and it carries the flag plus `RUSTDOCFLAGS: "-D warnings"`).
- a comment above the step naming the flag's purpose and that it is kept
  identical to BUILDING.md's block.

**Both-sites check, fired:** `grep -rnP "cargo doc --workspace(?!.*--document-private-items)"`
over both files returns nothing (exit 1); the same pattern against a control
line `run: cargo doc --workspace --no-deps` returns 1 hit. The two remaining
`cargo doc` mentions in those files (`BUILDING.md:85` prose, `ci.yml:99` step
name) are labels, not invocations.

**Count wording:** neither file states a gate total, so nothing was
recomputed there. `BUILDING.md`'s "The Rust gate (six parts ...)" heading and
its "Rust-gate parts 1-5" / "part 6" references stay correct - the flag
modifies an existing part rather than adding one. I deliberately did **not**
write "the eleventh part" into BUILDING.md; see concern 4 for why its own
enumeration would not support that number today.

**The flag's fire, measured before and after item 5** (this item's required
verification):

Before, on the clean tree at `7aa9f99` (`git status --porcelain` empty):

```
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:54:21
error: `run` is both a function and a module
  --> src-tauri/src/lib.rs:87:15
error: could not document `muxsmith-gui`
EXIT=101
```

Exactly two ambiguity errors, at exactly the two sites item 5 names (the
third `error:` line is the resulting "could not document" for the crate).

After item 5, same command: `EXIT=0`, zero lines matching `^(error|warning)`.

### Item 7 - D64's snapshot claim, count and kind

**Recounted from the tree at `7aa9f99`, not transcribed from the ROADMAP.**

- Snapshot files, `git ls-files | grep '\.snap$'`: **13**, all under
  `crates/muxsmith-cli/tests/snapshots/`. Per file: `cli_validate` **5**,
  `dry_run_cli` **3**, `run_cli` **4**, `run_live` **1**, `cli_schema` **0**.
- Cross-check on the asserting side, `grep -c assert_snapshot` per test file:
  `cli_validate.rs` 5, `dry_run_cli.rs` 3, `run_cli.rs` 4, `run_live.rs` 1 =
  13. The two counts agree, so no inline snapshot exists without a file and
  no orphan file exists without an assert.
- This equals the ROADMAP's recount at `3412fcc` (13; 5/3/4/1), so the tree
  has not moved on this measure since.

**Kind, verified before writing the restated sentence:**

- Helper call sites: `cli_validate.rs` 6 `support::muxsmith(` + **1**
  `support::muxsmith_localized(`; `dry_run_cli.rs` 16; `run_cli.rs` 11;
  `run_live.rs` 6; `cli_schema.rs` 0 funnel sites and 2 `support::muxsmith_bare(`.
- The de snapshot test is `bare_raw_property_renders_german_with_locale_flag`
  (`cli_validate.rs:94`); it invokes `support::muxsmith_localized(&["validate",
  ...], "de")` at `:105`, i.e. it does not ride the en funnel.
- So of the 13 snapshots, **12 ride `support::muxsmith` and 1 rides
  `support::muxsmith_localized`**; `cli_schema.rs`'s two bare-helper callers
  carry no snapshot. The shape the ROADMAP suggested holds and is what I
  wrote: every CLI-invoking snapshot test rides a pinned helper, the en funnel
  or its locale-parameterized construction site.
- D64's own invariant re-measured: `Command::cargo_bin("muxsmith")` appears in
  exactly **one file**, `crates/muxsmith-cli/tests/support/mod.rs`, at **two
  call sites** (`:105` in `muxsmith_localized`, `:125` in `muxsmith_bare`).
  The invariant is stated at file level and holds.

Edits, four sites:

1. `specs/2026-07-21-plan7-help-i18n-design.md:1505` - "No new snapshot files:
   the 11 insta snapshots stay en-pinned (D64) and de rendering is covered
   here plus by the parity gates." -> records that amendment 4 later added
   exactly one file (the German `cli_validate` case, named), then the Plan-9
   recount: 13 snapshots, 12 en-pinned through the funnel, that one de-pinned
   through `support::muxsmith_localized`.
2. `:1556` - "`cli_validate.rs` (1 constructor, 3 snapshots)" -> "(1
   constructor, 3 snapshots - 5 since amendment 4 added the German case)".
   The enumeration declares itself "measured: `cargo_bin` grep, 2026-07-21",
   so the dated figure is kept and annotated rather than overwritten (see
   concern 6).
3. `:1563` - "The funnel covers all **11 insta snapshots** ... and every
   locale-sensitive stdout/stderr assertion" -> the coverage sentence
   restated in kind ("Every CLI-invoking snapshot test rides a pinned helper
   - the en funnel or its locale-parameterized construction site
   `support::muxsmith_localized` - as does every locale-sensitive
   stdout/stderr assertion, including `--json` assertions ...") plus
   "Recounted at the Plan-9 close: **13 insta snapshots** ..., 12 of them
   through the en funnel and the German `cli_validate` case through
   `muxsmith_localized`".
4. `plans/2026-07-21-plan-7-help-i18n.md:80` - "The funnel covers all 11 insta
   snapshots (3+3+4+1+0 = 11, `tests/snapshots/` counted)" -> the same
   restatement with the arithmetic recomputed: "5+3+4+1+0 = 13 insta
   snapshots (`tests/snapshots/` counted), 12 of them through the en funnel
   and the German `cli_validate` case through `muxsmith_localized`
   (amendment 4)"; the same "- 5 since amendment 4 added the German case"
   annotation on that file's `cli_validate.rs` parenthetical.

## Verification - the eleven-part gate, foreground, no subsets

Run on the final tree (all nine edits in place). Exit codes captured
directly, not through a pipe.

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | exit 0, no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, 0 lines matching `^(warning|error)` |
| 3 | `cargo test --workspace` | exit 0, **39** `test result:` lines, all `ok`, 0 failed, 0 ignored; `dry_run_cli.rs` **13 passed** |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | exit 0, 0 lines matching `^(warning|error)` |
| 5 | `cargo deny check` | exit 0, `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | exit 0; the only two `^warning` lines are the pre-existing build-script notices `muxsmith-gui@0.1.0: GNU compiler is not supported for this target` |
| 7 | `pnpm lint` | exit 0, `$ eslint .` and nothing further |
| 8 | `pnpm build` | exit 0, `✓ built in 158ms` |
| 9 | `pnpm check:i18n` | exit 0, `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | exit 0, `68 passed (3.0s)` |
| 11 | `python3 scripts/ledger-lint.py` | exit 0, `ledger-lint: 516 entries across 4 files, all invariants hold` |

**Baselines, all held, none moved:** `cargo test --workspace` 39 `test
result:` lines all ok (baseline 39); `pnpm test:e2e` 68 passed (68);
`check:i18n` 212 catalog ids (212); `ledger-lint` 516 entries (516).

**Skip markers: 0.** `grep -cE "SKIP|skipping"` over the test log returns 0;
the 13 case-insensitive `skip` hits are all test NAMES
(`..._on_collision_skip_...`, `..._skips_subdirs`, ...), listed and checked
individually. This matters for item 2: the assertion it re-words lives in a
`have_mkvmerge()`-gated test, so a green run with a self-skipped test would
prove nothing about it.

**Typography check, fired:** `grep -nP "[\x{2013}\x{2014}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]"`
over every added diff line returns nothing (exit 1); the same pattern against
a control line containing an em dash and a Unicode ellipsis returns 1 hit.
ASCII hyphens and straight quotes throughout.

## Divergences and judgment calls

1. **Item 2 applied verbatim despite a wrinkle in its comment text.** The
   brief fences item 2's prose to the verdict, so I applied the block as
   written and raised the wrinkle as concern 1 rather than editing it.
2. **Item 3's clause names three production construction sites, not four.**
   I could not reproduce a fourth; concern 2 carries the measurement.
3. **No gate total written into BUILDING.md.** The item's semantics required
   ledger-lint's appearance in the list and a recomputation of any "ten parts"
   count; the file has no such count, and asserting "eleven" would exceed what
   its own enumeration shows. Concern 4.
4. **The dated enumerations in the two plan-7 documents were annotated, not
   overwritten**, where they declare their own measurement date. Concern 6.
5. **Nothing else in `identify.rs` touched**, including the module doc that
   carries the same phrase. Concern 3.

## Numbered concerns a reviewer can rule on yes/no

1. **The LOW-4 comment's "carries neither key" clause is inaccurate, and it is
   now in the tree verbatim.** The verdict's fenced comment says "The
   profile-load-failure shape carries neither key and is ruled out by the code
   sequence below". My own measurement of that exact document (pasted under
   item 2) shows it carries `files: []` - as the verdict's own LOW-4
   measurement did, and as `config_only_document`
   (`crates/muxsmith-core/src/report/json.rs:95-115`) emits unconditionally.
   Only `mkvmerge_found` is absent. Ruling wanted: leave verbatim (the fenced
   text stands), or narrow the clause to "carries no `mkvmerge_found`, and its
   `files` is an empty array, so it satisfies both assertions and is ruled out
   by the code sequence below".
2. **"All four sites" (item 3) does not match the tree.** I measure three
   production construction sites (table under item 3) and three test ones, six
   `IdentifyCache::new()` call sites in total. My clause names the three
   production contexts, which is what the doc must be true of. Ruling wanted:
   is three what the brief meant, or is there a fourth site I did not reach?
3. **The MODULE doc in the same file still says "constructed per planning
   call".** `crates/muxsmith-core/src/identify.rs:3-6`: "The cache is
   constructed per planning call and dropped with it, so separate calls
   re-identify." The whole-branch verdict scoped finding 3 to "the TYPE's
   doc" and the brief to "the `IdentifyCache` doc", so I left it. It is the
   same exclusive-form shape one level up, in a file this pass already
   touches. Ruling wanted: rides this pass, or a separate vehicle?
4. **BUILDING.md's gate blocks enumerate ten commands after my edit, not
   eleven, because `pnpm build` is not in them.** The `### Frontend checks`
   block lists `pnpm lint`, `pnpm check:i18n`, `pnpm test:e2e`; `pnpm build`
   is documented under `## Building and running` and named only in the CI
   paragraph. Every count outside the file (HANDOFF's "TEN parts per
   BUILDING.md", the brief's eleven-part list) counts it as a gate part. So
   the file's own enumeration is one short of the number the rest of the
   project derives "per BUILDING.md". Adding it is outside item 6's fixed
   semantics, so I did not. Ruling wanted: add `pnpm build` to the frontend
   block (making the file enumerate eleven), or leave the count implicit?
5. **`ci.yml`'s comment above the doc step still reads "rustdoc correctness as
   the ninth gate part"** (`.github/workflows/ci.yml:88`, a Plan 5.5 Task 12
   provenance note). It is a dated attribution, not a live count, so
   recomputing it to "eleventh" would falsify the history. Left untouched.
   Ruling wanted: fine as provenance, or re-word?
6. **Other numbers in the same plan-7 enumerations are also stale at HEAD, and
   I left them.** The spec doc's `dry_run_cli.rs` "13 invocation sites" is 16
   today; `run_cli.rs` "1 constructor" has 11 funnel call sites, `run_live.rs`
   6. Both enumerations declare their measurement date ("measured:
   `cargo_bin` grep, 2026-07-21" / "measured 2026-07-21 ... re-verify by text
   at dispatch"), and the ROADMAP named only the `cli_validate` parenthetical,
   so I annotated that one and left the neighbours as the dated record they
   claim to be. Ruling wanted: correct reading?
7. **Item 4's addition is one sentence but a long one.** If the reviewer wants
   it split into two, the semantics do not change.

## For the controller - counts and statements in your files that this change moves

None of these were touched.

1. `HANDOFF.md:107-109`: "**The gate is TEN parts** per BUILDING.md ...
   `python3 scripts/ledger-lint.py` runs before every push as well and **is
   not one of the ten**. Two edits to that gate block are an open close
   action (below)." Both halves are now stale: eleven parts, ledger-lint is
   one of them, and `HANDOFF.md:155` lists the two edits as still open.
2. `docs/process-conventions.yaml:693`
   (`ledger-lint-runs-before-every-push`): "It is not one of the ten parts
   ... The gate block in BUILDING.md **gains** it as an eleventh part at the
   Plan 9 close, together with the rustdoc private-items flag". Future tense;
   both edits landed in `9dc3a4d`.
3. `docs/decision-ledger.yaml:4585`
   (`does-the-ten-part-gate-bind-doc-only-pushes`): "ledger-lint **is not one
   of the ten parts**, so the only check a docs-or-YAML push can turn red was
   not run before the push at all" - the gap that sentence records is now
   closed. Its 11-second measurement is explicitly a measurement of ten parts
   and reads fine as a dated figure.
4. `docs/ROADMAP.md:1178-1192`: the close-action entry itself (both edits
   done) and its instruction that "the plan's own 'ten-part gate' wording is
   updated in the same pass, and any count that says ten is recomputed".
5. **The plan-9 plan document quotes the ten-part gate verbatim, including
   the unflagged doc command**: `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
   at `:11` ("One ten-part gate run"), `:20` (the enumerated list, with
   `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` and no
   ledger-lint), `:117`, `:131`, `:431`. That is the file ROADMAP:1191 means.
6. `docs/ROADMAP.md`'s "Docs accuracy" D64 entry (`:1196-1223`): its vehicle
   fired; my independent recount agrees with its recorded 13 (5/3/4/1).
7. Pre-existing and older than this change, so only flagged: two Tier
   statements still say **nine**-part gate in live rule text, not as dated
   records - `docs/decision-ledger.yaml:4117` ("the full nine-part gate is
   owed at merge and at push") and `:4246` ("The local nine-part gate covers
   ONE platform").

## Commit

```
$ git show --stat HEAD
commit 9dc3a4d4bd30707ff5e89e5d322d8c0bb7670e66
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Wed Jul 29 03:16:30 2026 +0200

    close: plan-9 text corrections and the gate's own definition
    [message body: the seven items, then the eleven-part gate result]

 .github/workflows/ci.yml                           |  7 ++++-
 BUILDING.md                                        | 18 ++++++++++++-
 crates/muxsmith-cli/tests/dry_run_cli.rs           | 10 +++++---
 crates/muxsmith-core/src/identify.rs               |  9 ++++---
 .../plans/2026-07-21-plan-7-help-i18n.md           |  2 +-
 .../specs/2026-07-21-plan7-help-i18n-design.md     | 30 ++++++++++++++--------
 e2e/jobsview-reset.spec.ts                         | 13 ++++++++++
 src-tauri/src/lib.rs                               |  4 +--
 src/views/BatchView.vue                            | 14 +++++-----
 9 files changed, 78 insertions(+), 29 deletions(-)
```

Staged file by file, never `git add -A`. `git -c commit.gpgsign=false`,
exactly one trailer (`git log -1 --format='%(trailers)'` returns the single
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>` line), no
`Claude-Session` line. Not pushed.

---

## Fix round

Fresh implementer, did not write the pass. Six edits, all prose, no
behaviour change: no assertion, test or command semantics move. Every
finding read at the verdict rather than from the brief's summary.

**Note on the brief's own header:** it says "six edits across five files".
It is six files - `BUILDING.md` carries two of the six edits (F3 and F6),
and F5 is one edit applied to two documents. Nothing follows from it; the
edit set is unchanged.

### Edit 1 - F1, `crates/muxsmith-cli/tests/dry_run_cli.rs:443-446`

Before:

```rust
    // Shape guards. `mkvmerge_found` absent rules out the two config-only
    // shapes that carry it (mkvmerge missing, query failed). The
    // profile-load-failure shape carries neither key and is ruled out by the
    // code sequence below, where it would be a singleton `parse-error`.
```

After (the verdict's F1 text, verbatim):

```rust
    // Shape guards. `mkvmerge_found` absent rules out the two config-only
    // shapes that carry it (mkvmerge missing, query failed). The
    // profile-load-failure shape carries no `mkvmerge_found` and an empty
    // `files` array, so it satisfies both guards and is ruled out by the
    // code sequence below, where it would be a singleton `parse-error`.
```

Assertions untouched. The controller's lift of the Task-5 fence is what
licences this; I did not re-open the question, but I did check the claim at
its root rather than inherit it: `config_only_document`
(`crates/muxsmith-core/src/report/json.rs:104-112`) builds every document
with `"files": []` at `:106` unconditionally and adds `mkvmerge_found` only
inside `if let Some(found)`. So the profile-load-failure shape (the `None`
call) carries `files: []`, and "carries neither key" was false on `files`.

### Edit 2 - F2 + Q7, `e2e/jobsview-reset.spec.ts:99-110`

Before:

```
 * Relative to `installMockIPC` it deliberately answers a narrower surface
 * -- no `__TAURI_OS_PLUGIN_INTERNALS__.platform` global, no forwarding to
 * the Node-side `__muxsmithRecordInvoke__` log, and no `get_settings` /
 * `set_settings` / `plugin:fs|write_text_file` answers -- which is safe
 * today because this spec mounts `JobsView` alone on a blank page instead
 * of driving the served app: `platform()` is `FirstRun.vue`'s, the
 * settings pair belongs to `main.ts`'s locale bootstrap and
 * `SettingsDialog.vue`, the file write sits behind `RunHistory.vue`'s
 * user-triggered log export, no mount here reaches any of them, and every
 * test in this file asserts DOM state rather than a recorded call log --
 * with the unmocked-command throw below as the backstop the day one of
 * those stops holding.
```

After:

```
 * Relative to `installMockIPC` it deliberately answers a narrower surface:
 * no `__TAURI_OS_PLUGIN_INTERNALS__.platform` global, no forwarding to the
 * Node-side `__muxsmithRecordInvoke__` log, and no `get_settings` /
 * `set_settings` / `plugin:fs|write_text_file` answers. That is safe today
 * because no mount in this spec reaches any of them: it mounts `JobsView`
 * alone on a blank page instead of driving the served app, `platform()` is
 * `FirstRun.vue`'s, the settings pair is read across the app but by no
 * component this spec mounts (`main.ts`'s locale bootstrap,
 * `SettingsDialog.vue`, `BatchView.vue`, `EditorView.vue`, `FirstRun.vue`,
 * `recentProfiles.ts`), the file write sits behind `RunHistory.vue`'s
 * user-triggered log export, and every test in this file asserts DOM state
 * rather than a recorded call log -- with the unmocked-command throw below
 * as the backstop the day one of those stops holding.
```

Both halves of the finding are in: the settings clause states the property
(read across the app, by no component this spec mounts) and then
illustrates, which is the `core-docs-name-callers-illustratively-never-exclusively`
shape; and the paragraph is two sentences, the second opening on the
conclusion the old one buried at word sixty-odd. The `platform` and
`writeTextFile` attributions were verified correct by the reviewer and are
carried through unchanged.

### Edit 3 - F3, `BUILDING.md` `### Frontend checks`

One line added after `pnpm lint`, comment in the neighbours' shape (and
identical to the file's other `pnpm build` line at `:65`, whose column it
also matches):

```
+pnpm build            # vue-tsc type-check + production frontend build
```

No total added anywhere in the file. Nothing else in the block moved.

### Edit 4 - F4, `crates/muxsmith-core/src/identify.rs:4-5`

Before: `The cache is constructed per planning call and dropped with it, so
separate calls re-identify.`
After: `The cache is constructed per call and dropped with it, so separate
calls re-identify.`

Two words changed, nothing rewrapped, the rest of the paragraph untouched.
`LiveIdentifier.cache` at `:392` deliberately not touched.

### Edit 5 - F5, the two plan-7 documents

`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1561`, before:

```
  `cli_validate.rs` (1 constructor, 3 snapshots - 5 since amendment 4
  added the German case), ...
```

after:

```
  `cli_validate.rs` (1 constructor, 3 snapshots at this measurement; 5 at
  the Plan-9 close, the bare-raw case (`d768657`) and the German case
  (`3412fcc`, amendment 4) - the other figures in this enumeration are the
  2026-07-21 measurement, unrefreshed), ...
```

`docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80`, the same
substitution inside the single-line paragraph. Before:

```
3 snapshots - 5 since amendment 4 added the German case
```

after:

```
3 snapshots at this measurement; 5 at the Plan-9 close, the bare-raw case
(`d768657`) and the German case (`3412fcc`, amendment 4) - the other
figures in this enumeration are the 2026-07-21 measurement, unrefreshed
```

The scoping clause is the verdict's wording, kept verbatim in both files
rather than sharpened per file: it already covers the stale neighbour
(`cli_validate.rs`'s "5 helper call sites", 7 at HEAD) as one of "the other
figures in this enumeration", and naming that figure specifically would
have been a rewording the finding did not ask for. Neighbouring figures not
refreshed, per the brief.

### Edit 6 - F6, `BUILDING.md:120`

`CI ... runs Rust-gate parts 1-5 natively on all three OS legs` ->
`... parts 1-4 ...`. The rest of the sentence, including the part-6
parenthetical and the `cargo deny check` independent-job clause, unchanged.

### Measurements I took

| What | Instrument | Result |
|---|---|---|
| settings consumers (edit 2) | `git grep -n "getSettings\|setSettings" -- src/` | six consumers: `main.ts:4,17`, `components/SettingsDialog.vue:4,30,64`, `recentProfiles.ts:9,34,39`, `views/BatchView.vue:12,15,69,81,83`, `views/EditorView.vue:109,135`, `views/FirstRun.vue:5,50,74,75`; plus the definition site `ipc.ts:255,259` which is not a consumer. Matches the brief's six. |
| the spec's mount set (edit 2) | `grep -n "component:" e2e/jobsview-reset.spec.ts` | exactly one, `{ component: "JobsView", ... }` at `:148` |
| F1's claim at the emitter | read `crates/muxsmith-core/src/report/json.rs:95-114` | `"files": []` unconditional at `:106`; `mkvmerge_found` only under `if let Some(found)` at `:110` |
| the two commits (edit 5) | `git log --diff-filter=A` per `cli_validate__*.snap` | `..._bare_raw_property_exits_two_and_renders_the_message` -> `d768657` 2026-07-28 20:16; `..._bare_raw_property_renders_german_with_locale_flag` -> `3412fcc` 2026-07-28 20:50; the other three -> `aba7f4f` 2026-07-12 |
| the module doc's neighbours (edit 4) | `grep -n "per planning call\|per call" crates/muxsmith-core/src/identify.rs` | `:5` module doc (edited), `:304-305` type doc (already corrected, "per call ... (per planning call in the pipeline seam, per invocation on the CLI and GUI)"), `:392` `LiveIdentifier.cache` field doc (true, untouched) |
| BUILDING.md's enumeration after edit 3 | commands per `### ` check block | 6 (Rust gate) + 4 (Frontend checks) + 1 (House-knowledge) = **11** |
| gate total anywhere in the edited files | `grep -nEi "ten part\|ten-part\|10 part\|eleven" BUILDING.md .github/workflows/ci.yml` | exit 1, no hits - **fired against the control** `grep -nEi "six parts" BUILDING.md` -> `:74` |
| typography over the change | 24 added lines, `grep -P '[\x{2013}\x{2014}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]'` | exit 1, no hits - **fired**: the same command over the same file plus one synthetic em-dash/ellipsis line hits at that line |
| commit-trailer house pattern | `git log -20 --format='%b' \| grep -E '^(Co-Authored-By\|Claude-Session)' \| sort \| uniq -c` | 20x `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, zero `Claude-Session`. Checked rather than taken from the brief's restatement of SI-4. |

### Verification - the eleven-part gate, my own run, foreground, no subsets

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | exit 0, zero output lines |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, 0 lines matching `^(warning\|error)` |
| 3 | `cargo test --workspace` | exit 0, **39** `test result:` lines, 0 non-`ok`, 0 with non-zero failed, 0 with non-zero ignored; `dry_run_cli.rs` 13 tests |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | exit 0, 0 err/warn lines |
| 5 | `cargo deny check` | exit 0, `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | exit 0, 2 `^warning` lines, both the build-script notice `muxsmith-gui@0.1.0: GNU compiler is not supported for this target` |
| 7 | `pnpm lint` | exit 0, `$ eslint .` |
| 8 | `pnpm build` | exit 0, `built in 151ms` |
| 9 | `pnpm check:i18n` | exit 0, `41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s)` |
| 10 | `pnpm test:e2e` | exit 0, **68 passed (3.0s)** |
| 11 | `python3 scripts/ledger-lint.py` | exit 0, `516 entries across 4 files, all invariants hold` |

Baselines **39 / 68 / 212 / 516**: all held, none moved. Aggregates
recomputed from my own logs (`g1.log`..`g11.log` in this session's
scratchpad), not quoted from the report or the verdict.

**Nothing to fire in these edits, stated plainly rather than manufactured:**
all six are prose in comments, docs and Markdown. No assertion, command,
script or workflow changed, so no check's outcome depends on this diff and
there is no mutation that would make one go red. The two absence checks that
*were* load-bearing here (the gate-total grep and the typography scan) were
each fired against a known-present control, above.

### Found and not touched

1. **`docs/superpowers/specs/...-design.md:1505`** - "No new snapshot files,
   as designed here - amendment 4 later added exactly one". The verdict flags
   this as the softer instance of F5's first half (two commits added
   snapshots, not one), but the brief scopes edit 5 to the `cli_validate.rs`
   parenthetical in both files. Not touched; it is a live candidate for the
   controller if the same correction is wanted at that sentence.
2. **`cli_validate.rs`'s "5 helper call sites"** in the plan doc's same
   parenthetical, 7 at HEAD. Deliberately left at the dated measurement, per
   the brief; the new scoping clause is what tells a reader it is dated.
3. **`docs/decision-ledger.yaml:2329` (`ci-15-rustdoc-gate`) and `:4614`
   (`cargo-doc-is-no-evidence-...`)** - the verdict's HARVEST 6b items 1 and
   2, live Tier-1 rule text describing the flag change as pending and the
   step as "the ninth gate part". Outside the six edits; not touched.
4. **`BUILDING.md:74`'s "(six parts)"** - still correct after edit 3 (the
   Rust block is unchanged), so deliberately left.
5. **The `LiveIdentifier.cache` field doc** at `identify.rs:392` - true as
   written, explicitly fenced by the brief and the verdict. Untouched.
6. **No NEEDS_CONTEXT arose.** No edit presented a fork; every one had its
   text or its measurement fixed by the verdict.

### Commit

`c8dfc6d5ebf2f9dc62398e734e949a840aabcd3e` - `close: fix round for the
plan-9 close-pass findings (F1-F6)`

```
 BUILDING.md                                        |  3 ++-
 crates/muxsmith-cli/tests/dry_run_cli.rs           |  3 ++-
 crates/muxsmith-core/src/identify.rs               |  2 +-
 .../plans/2026-07-21-plan-7-help-i18n.md           |  2 +-
 .../specs/2026-07-21-plan7-help-i18n-design.md     |  8 ++++---
 e2e/jobsview-reset.spec.ts                         | 25 +++++++++++-----------
 6 files changed, 24 insertions(+), 19 deletions(-)
```

Six files staged by name, never `git add -A`; `git -c commit.gpgsign=false`;
exactly one trailer (`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`,
count 1), no `Claude-Session` line (count 0). One commit. Tree clean at
`c8dfc6d`, `git status --porcelain` empty, `master` **3 ahead of
`origin/master`**, not pushed. No session-relocation tool was called; every
run was foreground.
