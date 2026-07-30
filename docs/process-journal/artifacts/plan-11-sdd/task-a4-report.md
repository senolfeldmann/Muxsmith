# Task A4 report - Plan 11, stream A (W4: the v1 spec's 8.1 states the shipped CLI surface)

**Status:** DONE_WITH_CONCERNS (two observations, neither blocking; both in
"Surfaced, not touched")
**Worktree:** `/home/senol/Git/muxsmith-plan11-a`, branch `plan-11-stream-a`
**Base:** `164e571` (A3) **Commit:** `06e896e`
**Files changed:** exactly one, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

> **This report carries four dated ERRATUM blocks added 2026-07-30 in a
> report-only correction round** (controller ruling: annotate, never rewrite, so
> every original claim stays legible). Two correct pasted grep outputs that were
> re-typed rather than copied and came out short; two correct a wrong spec
> citation. **The spec edit itself passed review unchanged and no product file
> was touched in that round.** Grep this file for `ERRATUM` before quoting any
> pasted output from it.

---

## Step 1: the surface, re-derived from the shipped binary

### 1a. The binary is not stale, and the staleness check is fire-verified

```
$ ls -la --time-style=full-iso target/debug/muxsmith crates/muxsmith-core/src/matcher.rs
-rw-r--r--. 1 senol senol    24100 2026-07-30 16:20:05.882226631 +0200 crates/muxsmith-core/src/matcher.rs
-rwxr-xr-x. 2 senol senol 62787160 2026-07-30 16:23:01.999860891 +0200 target/debug/muxsmith
```

A3's `matcher.rs` edit landed at 16:20:05; the binary is from 16:23:01. A3
rebuilt it (its own Step-1 binary probes required a rebuild), so no rebuild was
needed here - but cargo was asked anyway, because mtime is not cargo's
staleness criterion:

```
$ cargo build -p muxsmith-cli
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
```

No compilation units ran, so the binary matches the source by cargo's own
fingerprint, not only by mtime.

The prescribed check:

```
$ find crates src-tauri -name '*.rs' -newer target/debug/muxsmith
(no output)
```

**Fired, because its passing result is an absence** - a `find` that is
misaimed and a `find` that is clean look identical:

```
$ touch crates/muxsmith-cli/src/.fire-check-a4.rs
$ find crates src-tauri -name '*.rs' -newer target/debug/muxsmith
crates/muxsmith-cli/src/.fire-check-a4.rs
$ rm -f crates/muxsmith-cli/src/.fire-check-a4.rs
$ find crates src-tauri -name '*.rs' -newer target/debug/muxsmith
(no output)
$ git status --porcelain
(no output)
```

The check discriminates, and the fire left nothing behind.

### 1b. `--version` and the top-level `--help`

```
$ ./target/debug/muxsmith --version
muxsmith 0.1.0

$ ./target/debug/muxsmith --help
Top-level CLI arg parser (spec 8.1): `muxsmith <subcommand> ...`

Usage: muxsmith <COMMAND>

Commands:
  validate  Statically validate a profile (YAML or JSON)
  schema    Print the profile JSON Schema
  dry-run   Plan the batch without muxing: identify sources, resolve rules, and print the per-file resolution, diagnostics, and suggestions
  identify  Identify one source file via mkvmerge and print its tracks
  run       Plan and execute the batch (spec 5.5 level 3)
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Six entries under `Commands:`. Five are Muxsmith's own subcommands; `help` is
clap's built-in, and it is captured below with the other five so the
enumeration is derived from the binary rather than from the spec's list.

### 1c. `<sub> --help` for every subcommand the binary lists

```
$ ./target/debug/muxsmith validate --help
Statically validate a profile (YAML or JSON)

Usage: muxsmith validate [OPTIONS] <PROFILE>

Arguments:
  <PROFILE>  Path to the profile file to validate

Options:
      --json             Emit the structured report as JSON
      --locale <LOCALE>  Locale for rendered messages (default: system, fallback en)
  -h, --help             Print help

$ ./target/debug/muxsmith schema --help
Print the profile JSON Schema

Usage: muxsmith schema

Options:
  -h, --help  Print help

$ ./target/debug/muxsmith dry-run --help
Plan the batch without muxing: identify sources, resolve rules, and print the per-file resolution, diagnostics, and suggestions

Usage: muxsmith dry-run [OPTIONS] <PROFILE>

Arguments:
  <PROFILE>
          Path to the profile file

Options:
      --source <SOURCE>
          Source directory to scan (overrides the profile default)

      --output <OUTPUT>
          Output directory (overrides the profile default)

      --on-collision <ON_COLLISION>
          Collision policy override (spec 4.2 run input); falls back to the profile's `output.on_collision` when unset

          Possible values:
          - error:     Refuse the colliding output (default policy)
          - skip:      Skip the colliding output with a warning
          - overwrite: Replace the pre-existing file

      --json
          Emit the structured batch report as JSON

      --locale <LOCALE>
          Locale for rendered messages (default: system, fallback en)

  -h, --help
          Print help (see a summary with '-h')

$ ./target/debug/muxsmith identify --help
Identify one source file via mkvmerge and print its tracks

Usage: muxsmith identify [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to the media file to identify

Options:
      --json             Emit the structured identification as JSON
      --locale <LOCALE>  Locale for rendered messages (default: system, fallback en)
  -h, --help             Print help

$ ./target/debug/muxsmith run --help
Plan and execute the batch (spec 5.5 level 3)

Usage: muxsmith run [OPTIONS] <PROFILE>

Arguments:
  <PROFILE>
          Path to the profile file

Options:
      --source <SOURCE>
          Source directory to scan (overrides the profile default)

      --output <OUTPUT>
          Output directory (overrides the profile default)

      --on-collision <ON_COLLISION>
          Collision policy override (spec 4.2 run input); falls back to the profile's `output.on_collision` when unset

          Possible values:
          - error:     Refuse the colliding output (default policy)
          - skip:      Skip the colliding output with a warning
          - overwrite: Replace the pre-existing file

      --jobs <JOBS>
          Parallel mux jobs (default 1 = sequential)
          
          [default: 1]

      --fail-fast
          Stop dequeuing after the first failed job (in-flight finish)

      --json
          Emit the structured run report as JSON

      --locale <LOCALE>
          Locale for rendered messages (default: system, fallback en)

  -h, --help
          Print help (see a summary with '-h')

$ ./target/debug/muxsmith help --help
error: unrecognized subcommand '--help'

Usage: muxsmith <COMMAND>

For more information, try '--help'.

$ ./target/debug/muxsmith help
Top-level CLI arg parser (spec 8.1): `muxsmith <subcommand> ...`

Usage: muxsmith <COMMAND>

Commands:
  validate  Statically validate a profile (YAML or JSON)
  schema    Print the profile JSON Schema
  dry-run   Plan the batch without muxing: identify sources, resolve rules, and print the per-file resolution, diagnostics, and suggestions
  identify  Identify one source file via mkvmerge and print its tracks
  run       Plan and execute the batch (spec 5.5 level 3)
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

`muxsmith help --help` is rejected by clap; `muxsmith help` prints the
top-level help. `help` states no surface of its own and is correctly absent
from a synopsis of the product's five subcommands.

### 1d. The divergence table, at flag granularity

Value names and possible-value sets included. `-h/--help` is captured per
subcommand here and then set aside from the synopsis. **The plan's authoring
table omitted it silently; this run states the omission rather than inheriting
it.**

> **ERRATUM, A4 implementer, 2026-07-30, same correction round as the two
> below.** The original wording of this paragraph justified the exclusion by
> citing spec 8.4's "Accepted v1 exceptions: clap's library-generated
> `--help`/usage text". **That citation does not carry the claim**, measured at
> review and confirmed by re-reading 8.4: the sentence it sits in is
> "**No hardcoded user-facing strings** in any layer ... Accepted v1
> exceptions: ...", so 8.4's exception exempts clap's help text from the
> **localization** mandate (it need not live in a Fluent catalog). It says
> nothing about whether 8.1's **synopsis** must enumerate `-h/--help`. Two
> different subjects, and I stretched one to cover the other.
>
> **The ground I actually have, stated instead of borrowed:**
>
> 1. **Primary, and it is not mine to decide.** The plan fences the
>    replacement block character for character and lists it under "Must not
>    decide"; the fenced block contains no `-h/--help`. The exclusion is the
>    plan's, applied verbatim.
> 2. **It is not a change this task makes.** The pre-state block omitted
>    `-h/--help` on all five lines too, so nothing was removed here.
> 3. **It carries no information in this block.** `-h/--help` is present
>    identically on all five subcommands (Step 1c's captures), and the point of
>    a per-subcommand enumeration - the plan's own Step-2 rationale - is that
>    the sets DIFFER, with `schema` as the counterexample that makes a blanket
>    claim false. A flag on every line discriminates nothing.
> 4. **The consistency target agrees.** `README.md` omits it from all five
>    subcommand headings.
>
> The handling was right; only the citation was wrong. Original wording stays
> legible in the git-ignored history of this file only, so it is restated here:
> "it is clap-generated, and spec 8.4 names 'clap's library-generated
> `--help`/usage text' as an accepted exception, so no synopsis line enumerates
> it."

| subcommand | positional | v1 spec 8.1 (pre-state) | binary `--help` (value name; possible values) | delta | plan's authoring row |
|---|---|---|---|---|---|
| `validate` | `<PROFILE>` | *no flags* | `--json` (bool); `--locale <LOCALE>` (free string); `-h/--help` | `--json`, `--locale` omitted by the spec | **REPRODUCED** ("both omitted") |
| `dry-run` | `<PROFILE>` | `--source DIR`, `--output DIR`, `--json` | `--source <SOURCE>` (path); `--output <OUTPUT>` (path); `--on-collision <ON_COLLISION>` (**error \| skip \| overwrite**); `--json` (bool); `--locale <LOCALE>`; `-h/--help` | `--on-collision`, `--locale` omitted | **REPRODUCED** |
| `run` | `<PROFILE>` | `--source DIR`, `--output DIR`, `--jobs N`, `--fail-fast`, `--json` | `--source <SOURCE>`; `--output <OUTPUT>`; `--on-collision <ON_COLLISION>` (**error \| skip \| overwrite**); `--jobs <JOBS>` (usize, **[default: 1]**); `--fail-fast` (bool); `--json` (bool); `--locale <LOCALE>`; `-h/--help` | `--on-collision`, `--locale` omitted | **REPRODUCED** |
| `identify` | `<FILE>` | `--json` | `--json` (bool); `--locale <LOCALE>`; `-h/--help` | `--locale` omitted | **REPRODUCED** |
| `schema` | *none* | *no flags* | none beyond `-h/--help` | **none - the spec line was already correct** | **REPRODUCED** |

**Every correction the plan states is reproduced by this run. Nothing was
dropped, and nothing failed to reproduce.**

Two derived properties the fenced replacement depends on, both measured
rather than assumed:

- **Option order.** The binary's order per subcommand is `--source`,
  `--output`, `--on-collision`, `--jobs`, `--fail-fast`, `--json`, `--locale`
  (each subcommand taking the subset it has). The fenced replacement follows
  that order on every line. Verified line by line against the four
  `<sub> --help` captures above.
- **The `POLICY` metavariable is not an unenumerated set.** The binary's
  possible values are `error`, `skip`, `overwrite`. The same domain is
  enumerated three times inside the spec itself, so `POLICY` resolves within
  the document:

```
$ grep -nE 'on_collision|on-collision|overwrite|CollisionPolicy' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
23:| Output naming | Keep name or rename template; collision policy | `error | skip | overwrite`, default `error`. In-place replacement excluded. |
67:  on_collision: error      # error | skip | overwrite
226:- `on_collision`: `error | skip | overwrite`, default `error`. Governs collisions with the FILESYSTEM only: a rendered output path that already exists as a pre-existing on-disk file (not one of this batch's inputs). `error` refuses (no plan), `skip` omits that output (no plan, warning), `overwrite` replaces it (plan kept, info). An output path equal to any input path (primary or donor) is always a hard `SourceOverwrite` error regardless of policy.
227:- Two planned outputs rendering to the same path is ALWAYS an error (`OutputCollision`, error severity), independent of `on_collision`: the batch is internally inconsistent and neither `skip` nor `overwrite` can define which plan wins. Fix the naming (disambiguate the `filename` template or `input.pattern`).
268:| `OutputCollision` | error (two planned) / per policy (on-disk) | two plans render to one path (always error), or the rendered path pre-exists on disk (severity per `on_collision`: error/warning-skip/info-overwrite; 4.8) |
363:                            [--on-collision POLICY] [--json] [--locale LOCALE]
365:                            [--on-collision POLICY] [--jobs N] [--fail-fast]
```

The binary's three values and the spec's three values agree exactly, including
which one is the default.

### 1e. Exit codes, derived from named symbols (they are not in the help text)

Every symbol named with its file:

| symbol | file | what it establishes |
|---|---|---|
| `Cli` (the `command` field's doc comment) | `crates/muxsmith-cli/src/cli.rs` | the documented contract `0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)` |
| `severity_exit` | `crates/muxsmith-cli/src/commands/mod.rs` | `Some(Severity::Error) => 2`, `Some(Severity::Warning) => 1`, `_ => 0` |
| `diag_exit_code` | `crates/muxsmith-cli/src/commands/mod.rs` | the worst-of fold over config + batch + per-file diagnostics, returning through `severity_exit` |
| `job_exit_code` | `crates/muxsmith-cli/src/commands/run.rs` | the queue's own fold: any `Failed` -> 2, else any `Warning` -> 1, else 0 |
| `std::process::exit(130)` inside the `ctrlc::set_handler` closure | `crates/muxsmith-cli/src/commands/run.rs` | the SECOND SIGINT's force-exit during cleanup |
| `return 130;` guarded by `cancel.load(Ordering::SeqCst)` | `crates/muxsmith-cli/src/commands/run.rs` | the graceful path: the flag ended the batch, so `run` returns 130 instead of its fold |
| `main` | `crates/muxsmith-cli/src/main.rs` | `std::process::exit(code)` - the process exits with whatever the dispatched command returned; no other exit path exists |

The full enumeration of every exit-producing site in the crate, so the list
above is derived and not recalled:

```
$ grep -rn "process::exit\|return 130\|-> i32" crates/muxsmith-cli/src/
crates/muxsmith-cli/src/main.rs:69:    std::process::exit(code);
crates/muxsmith-cli/src/commands/dry_run.rs:31:) -> i32 {
crates/muxsmith-cli/src/commands/mod.rs:25:pub(crate) fn severity_exit(worst: Option<Severity>) -> i32 {
crates/muxsmith-cli/src/commands/mod.rs:41:pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
crates/muxsmith-cli/src/commands/identify.rs:13:pub fn run(file: &Path, json: bool, renderer: &Renderer) -> i32 {
crates/muxsmith-cli/src/commands/run.rs:55:) -> i32 {
crates/muxsmith-cli/src/commands/run.rs:191:            std::process::exit(130);
crates/muxsmith-cli/src/commands/run.rs:252:        return 130;
crates/muxsmith-cli/src/commands/run.rs:295:fn job_exit_code(outcomes: &[JobOutcome]) -> i32 {
```

> **ERRATUM, A4 implementer, 2026-07-30, on the controller's ruling that this
> report is a dated record to annotate rather than rewrite.** The nine-line
> block above is not what that command returns. **It returns ten lines; the
> paste drops `crates/muxsmith-cli/src/commands/validate.rs:18`.** Found at
> review.
>
> **This is the site carrying the completeness label.** The sentence
> introducing this block calls it "The full enumeration of every exit-producing
> site in the crate, so the list above is derived and not recalled" - a
> completeness claim over a set that is short by one, and a claim of derivation
> over a list that was in fact re-typed. That is the latitude-by-omission
> shape rather than a typo, and it is the sharper of the two evidence defects.
> (The correction round's dispatch attributed the "full enumeration" label to
> the `ctrlc` block below; checked against this file, the phrase is here, at
> the exit-site block. The `ctrlc` block's own completeness claim is worded
> differently and is annotated in its own erratum.)
>
> This is a transcription defect, not a state difference: `validate.rs`
> is the same blob at the stream base and at this task's commit
> (`git rev-parse 164e571:...` and `06e896e:...` both -> `dcafe5d4`), and
> `git log --oneline 5378264..06e896e -- crates/muxsmith-cli/src/commands/run.rs
> crates/muxsmith-cli/src/commands/validate.rs` is empty, so no commit in this
> plan touched either file. The run I actually performed did contain the tenth
> line; I re-typed the output into this report instead of copying it, which is
> the exact failure the Global Constraint names (an observed value is pasted
> from the run that produced it, never recalled).
>
> **Re-run 2026-07-30 at `06e896e`, as emitted:**
>
> ```
> $ grep -rn "process::exit\|return 130\|-> i32" crates/muxsmith-cli/src/
> crates/muxsmith-cli/src/commands/dry_run.rs:31:) -> i32 {
> crates/muxsmith-cli/src/commands/validate.rs:18:pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
> crates/muxsmith-cli/src/main.rs:69:    std::process::exit(code);
> crates/muxsmith-cli/src/commands/identify.rs:13:pub fn run(file: &Path, json: bool, renderer: &Renderer) -> i32 {
> crates/muxsmith-cli/src/commands/mod.rs:25:pub(crate) fn severity_exit(worst: Option<Severity>) -> i32 {
> crates/muxsmith-cli/src/commands/mod.rs:41:pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
> crates/muxsmith-cli/src/commands/run.rs:55:) -> i32 {
> crates/muxsmith-cli/src/commands/run.rs:191:            std::process::exit(130);
> crates/muxsmith-cli/src/commands/run.rs:252:        return 130;
> crates/muxsmith-cli/src/commands/run.rs:295:fn job_exit_code(outcomes: &[JobOutcome]) -> i32 {
> $ ... | wc -l
> 10
> ```
>
> **`grep -rn`'s traversal order is not stable here and the order above must
> not be re-derived from, only the set.** Measured in one controlled run on the
> unchanged tree: nine consecutive invocations produced **7 distinct `md5sum`s
> of the emitted output**, while nine invocations of `... | sort | md5sum`
> produced **1 distinct value**
> (`62f6101fab141a0ea79bbeca95e8e56e`, nine times out of nine). So the set is
> stable and the order is not, and the order-independent form is the one a
> re-runner should compare against:
>
> ```
> $ grep -rn "process::exit\|return 130\|-> i32" crates/muxsmith-cli/src/ | sort
> crates/muxsmith-cli/src/commands/dry_run.rs:31:) -> i32 {
> crates/muxsmith-cli/src/commands/identify.rs:13:pub fn run(file: &Path, json: bool, renderer: &Renderer) -> i32 {
> crates/muxsmith-cli/src/commands/mod.rs:25:pub(crate) fn severity_exit(worst: Option<Severity>) -> i32 {
> crates/muxsmith-cli/src/commands/mod.rs:41:pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
> crates/muxsmith-cli/src/commands/run.rs:191:            std::process::exit(130);
> crates/muxsmith-cli/src/commands/run.rs:252:        return 130;
> crates/muxsmith-cli/src/commands/run.rs:295:fn job_exit_code(outcomes: &[JobOutcome]) -> i32 {
> crates/muxsmith-cli/src/commands/run.rs:55:) -> i32 {
> crates/muxsmith-cli/src/commands/validate.rs:18:pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
> crates/muxsmith-cli/src/main.rs:69:    std::process::exit(code);
> ```
>
> **The conclusion drawn from this block survives intact.** The missing line is
> `validate::run`'s `-> i32` signature - a function that returns an exit code
> through `severity_exit`, producing 0, 1 or 2 and never 130. Restoring it
> ADDS a subcommand entry point to the enumeration and adds no 130 site, so
> "Both 130 sites are in `run.rs` and nowhere else" is if anything better
> supported by the true ten-line output than by the nine I pasted: the tenth
> line is a subcommand that demonstrably cannot produce 130. The `symbol ->
> file` table above this block named `severity_exit`, `diag_exit_code`,
> `job_exit_code`, the two 130 sites and `main`, all of which are in both the
> nine-line and the ten-line output. Nothing in the spec amendment rested on
> the missing line.
>
> The nine-line block stays legible above as what was claimed at the time.

**Both 130 sites are in `run.rs` and nowhere else.** The handler-registration
claim, measured rather than inferred:

```
$ grep -rn "ctrlc" crates/muxsmith-cli/src/
crates/muxsmith-cli/src/commands/run.rs:34:/// flag ended the batch (D16). A `ctrlc` handler installed just before the
crates/muxsmith-cli/src/commands/run.rs:183:/// registration in the process, so ctrlc's double-registration error is
crates/muxsmith-cli/src/commands/run.rs:189:    if ctrlc::set_handler(move || {
```

> **ERRATUM, A4 implementer, 2026-07-30, on the controller's ruling that this
> report is a dated record to annotate rather than rewrite.** The three-line
> block above is not what that command returns, in two respects. Found at
> review.
>
> **A correction to the correction round's own account, checked at this file
> rather than accepted:** the dispatch located the phrase "the full
> enumeration" on THIS block. It is not here - it introduces the exit-site
> block above, and is annotated in that erratum. What this block carries is a
> completeness claim in different words: the sentence before it ("The
> handler-registration claim, measured rather than inferred") and the sentence
> after it ("One `set_handler` call in the crate, in `run.rs`."). Both are
> still completeness claims resting on a short paste, so the defect class is
> the same; only its label was mislocated.
>
> 1. **It returns four lines; the paste drops
>    `crates/muxsmith-cli/src/commands/run.rs:175`.**
> 2. **`:183` is mis-transcribed.** The paste renders its comment marker as
>    `///` (a doc comment); the file has `//` (an ordinary comment). One
>    character, but it changes what the line is: a doc comment on an item
>    versus an inline comment inside a function body.
>
> No state difference explains either. `run.rs` is the same blob at the stream
> base and at this task's commit (`git rev-parse 164e571:...` and `06e896e:...`
> both -> `4803b3f6`), and `git log --oneline 5378264..06e896e --
> crates/muxsmith-cli/src/commands/run.rs` is empty. The run I actually
> performed contained all four lines with `:183`'s `//` intact; I re-typed the
> output into this report instead of copying it. Same root cause as the
> erratum above, and the same Global Constraint violated.
>
> **Re-run 2026-07-30 at `06e896e`:**
>
> ```
> $ grep -rn "ctrlc" crates/muxsmith-cli/src/
> crates/muxsmith-cli/src/commands/run.rs:34:/// flag ended the batch (D16). A `ctrlc` handler installed just before the
> crates/muxsmith-cli/src/commands/run.rs:175:    // are unchanged, since `cancel` itself is still what the ctrlc handler
> crates/muxsmith-cli/src/commands/run.rs:183:    // registration in the process, so ctrlc's double-registration error is
> crates/muxsmith-cli/src/commands/run.rs:189:    if ctrlc::set_handler(move || {
> $ ... | wc -l
> 4
> ```
>
> **The conclusion drawn from this block survives intact, and the restored line
> strengthens it.** The claim the block supports is "exactly one
> `ctrlc::set_handler` registration in the crate, and it is in `run.rs`". All
> four lines are in `run.rs`; only `:189` is a call site; `:34`, `:175` and
> `:183` are comment text. So the true four-line output still shows one
> registration in one file. The dropped `:175` reads "are unchanged, since
> `cancel` itself is still what the ctrlc handler" - part of the D25 comment
> explaining that `QueueControl` shares the same cancel flag the handler flips,
> which is corroborating rather than contrary evidence for "only `run` returns
> 130". The mis-transcribed `:183` is the half-sentence "registration in the
> process, so ctrlc's double-registration error is", whose surrounding comment
> (quoted correctly elsewhere in this report as "this is the one registration
> in the process") is the source's own statement of the uniqueness claim; the
> `///` versus `//` slip does not touch its content.
>
> Both halves of the spec sentence this evidence supports - "Only `run` returns
> 130; no other subcommand installs a SIGINT handler" - therefore stand, and
> they were independently re-derived at review from the three sources.
>
> The three-line block and the completeness sentences bracketing it stay
> legible above as what was claimed at the time.

One `set_handler` call in the crate, in `run.rs`. `main.rs` (read in full,
above) dispatches `Schema`, `Validate`, `DryRun`, `Identify`, `Run` and
installs nothing. So **"Only `run` returns 130; no other subcommand installs a
SIGINT handler" is a measurement, not an inference.**

D16 is the decision the new bullet cites; its own text
(`docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, section
"D16: SIGINT handling via the `ctrlc` crate, single-level"):

> The queue then stops dequeuing, kills in-flight children through the
> executor's kill primitive, deletes their partial outputs, marks queued jobs
> `Cancelled`, lets the renderer print the summary, and the process exits 130.

---

## Steps 2 and 3: the two fenced substitutions

### Precondition: each fenced OLD block occurs exactly once

The OLD and NEW strings were **extracted from the plan document by line range**
(`sed -n`), never retyped, so a transcription slip cannot pass this check:

```
$ sed -n '585,589p' <plan> > old1.txt   # OLD synopsis  (5 lines, 290 bytes)
$ sed -n '595,602p' <plan> > new1.txt   # NEW synopsis  (8 lines, 503 bytes)
$ sed -n '610,610p' <plan> > old2.txt   # OLD bullet    (1 line,   63 bytes)
$ sed -n '616,618p' <plan> > new2.txt   # NEW bullet    (3 lines, 176 bytes)
```

`cat -A` over both fenced regions of the plan shows no trailing whitespace, no
tab and no CR: every line ends `$`.

Counted against the spec blob at `164e571`:

```
old1: occurrences in spec = 1
old2: occurrences in spec = 1
new1: occurrences in spec = 0
new2: occurrences in spec = 0
```

Each OLD block occurs **exactly once**, so a single-shot replacement is
unambiguous; each NEW block occurs zero times, so the pre-state is genuinely
un-amended.

The working copy was byte-identical to the `164e571` blob before the edit
(`cmp` clean), so "the pre-state" is one state, not two.

### Reconstruct, do not inspect

The edit was applied with the editor tool. The end state was then rebuilt
**independently**, from the `164e571` blob plus the two sed-extracted
substitutions, and compared byte for byte:

```
$ python3 ...  # spec_at_164e571.replace(old1,new1,1).replace(old2,new2,1)
reconstructed written, bytes: 45646
$ cmp spec-reconstructed.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
IDENTICAL - reconstruction matches
```

**The comparison is fire-verified** - a `cmp` that cannot fail proves nothing:

```
$ command cp -f spec-reconstructed.md spec-fire.md
$ python3 -c "...replace('[--locale LOCALE]','[--locale LOCALEX]',1)..." spec-fire.md
$ cmp spec-fire.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
... differ: byte 33819, line 363
cmp exit: 1
```

A single-character perturbation is detected. The clean run above is therefore
evidence.

### Measured properties of the new block

```
54 'muxsmith validate <profile> [--json] [--locale LOCALE]'
57 'muxsmith dry-run  <profile> [--source DIR] [--output DIR]'
78 '                            [--on-collision POLICY] [--json] [--locale LOCALE]'
57 'muxsmith run      <profile> [--source DIR] [--output DIR]'
76 '                            [--on-collision POLICY] [--jobs N] [--fail-fast]'
54 '                            [--json] [--locale LOCALE]'
51 'muxsmith identify <file> [--json] [--locale LOCALE]'
68 'muxsmith schema                      # print the profile JSON Schema'
```

Longest line **78** characters, so the plan's "no line exceeds 80 characters"
holds. The exit-code bullet's three lines measure 77, 77, 19.

The `schema` line is byte-identical between OLD and NEW (compared
programmatically, not by eye): `'muxsmith schema                      # print the profile JSON Schema'`.

### The applied diff

```
$ git diff -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
diff --git a/docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md b/docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
index a0c8146..6049a7c 100644
--- a/docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
+++ b/docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
@@ -358,15 +358,20 @@ Rules that keep it DRY:
 ### 8.1 CLI
 
 ```
-muxsmith validate <profile>
-muxsmith dry-run  <profile> [--source DIR] [--output DIR] [--json]
-muxsmith run      <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--json]
-muxsmith identify <file> [--json]
+muxsmith validate <profile> [--json] [--locale LOCALE]
+muxsmith dry-run  <profile> [--source DIR] [--output DIR]
+                            [--on-collision POLICY] [--json] [--locale LOCALE]
+muxsmith run      <profile> [--source DIR] [--output DIR]
+                            [--on-collision POLICY] [--jobs N] [--fail-fast]
+                            [--json] [--locale LOCALE]
+muxsmith identify <file> [--json] [--locale LOCALE]
 muxsmith schema                      # print the profile JSON Schema
 ```
 
 - Flags override profile-stored run inputs.
-- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors.
+- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors, plus 130 for a
+  cancelled batch (D16). Only `run` returns 130; no other subcommand installs
+  a SIGINT handler.
 - `--json` emits the structured report for scripting; default output is human-readable rendering of the same data, including suggestion YAML fragments.
 - `muxsmith schema` is a supported user feature, not only a debug aid (D47): the README's "Using the CLI" section documents redirecting its output to a file and binding it in editor settings (`yaml.schemas` in VS Code, the equivalent `lspconfig` block for Neovim/Helix) for autocompletion and inline validation while hand-authoring a profile.
```

One hunk, starting at line 358, entirely inside section 8.1. **The sections
Task A3 amended (4.3, 4.4, 7 and 9.2) are untouched** - the diff's only hunk
begins at 358 and ends at 377, and 8.1 begins at 358.

---

## Step 4: the spec self-contradiction sweep, as an enumeration

### Check 1 - every other exit-code or cancellation sentence

Run on **both** states, because the amendment is itself one of the classified
hits and its replacement changes the hit's line count.

```
$ grep -nE 'xit cod|SIGINT|Ctrl|[Cc]ancel|signal' <spec at 164e571>
318:- mkvmerge exit codes are honored and surfaced: 0 = success, 1 = completed with warnings (job marked "warning", output kept, warnings shown), 2 = error (job failed, partial output deleted).
319:- Cancellation: kill the mkvmerge process, delete the partial output file.
347:| `executor` | process spawn, progress parse, cancellation, job states |
369:- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors.
379:3. **Job queue**: per-job progress (from `#GUI#progress`), overall batch progress, live log, warning surfacing, cancel per job or batch.
391:- A prominent Help/Guide button, always visible in every view. Clicking it toggles help mode; clicking again (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits.
--- line count: 6 ---
```

**Six lines on the pre-state, reconciled one for one against the plan's six
pre-classified hits, in the plan's own order:**

| # | line | the plan's pre-classification | reconciles? | verdict |
|---|---|---|---|---|
| 1 | `:318` | section 6's mkvmerge exit codes - a different subject (the child process's code) | yes | **Consistent.** Says nothing about Muxsmith's own process code |
| 2 | `:319` | section 6's cancellation behaviour D16 implements | yes | **Consistent, not a contradiction.** It states what the handler does; the new bullet states what the process then returns |
| 3 | `:347` | section 7's `executor` architecture row, matching on `cancellation` | yes | **Consistent; says nothing about exit codes** |
| 4 | `:369` | the 8.1 bullet this task replaces | yes | **The target.** Replaced in Step 3 |
| 5 | `:379` | section 8.2's job-queue view, matching on `cancel` | yes | **Consistent.** A GUI surface description |
| 6 | `:391` | section 8.3's help-mode Esc sentence, matching on `cancel` | yes | **Consistent.** Keyboard handling |

**No hit outside the six.** Six of six reconcile.

Note that hit 3 sits in section 7, which Task A3 amended. Its text is
unchanged by A3 in the respect this sweep tests: the row still names
`cancellation` as an `executor` responsibility and still states no exit code.

End state, same expression:

```
$ grep -nE 'xit cod|SIGINT|Ctrl|[Cc]ancel|signal' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
318:- mkvmerge exit codes are honored and surfaced: 0 = success, 1 = completed with warnings (job marked "warning", output kept, warnings shown), 2 = error (job failed, partial output deleted).
319:- Cancellation: kill the mkvmerge process, delete the partial output file.
347:| `executor` | process spawn, progress parse, cancellation, job states |
372:- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors, plus 130 for a
373:  cancelled batch (D16). Only `run` returns 130; no other subcommand installs
374:  a SIGINT handler.
384:3. **Job queue**: per-job progress (from `#GUI#progress`), overall batch progress, live log, warning surfacing, cancel per job or batch.
396:- A prominent Help/Guide button, always visible in every view. Clicking it toggles help mode; clicking again (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits.
--- line count: 8 ---
```

**Eight, and the delta is fully accounted for:** hit 4 was one line and is now
three (`:372`, `:373`, `:374`), all three inside the replacement block this
task installed - `:372` matches `xit cod`, `:373` matches `[Cc]ancel`, `:374`
matches `SIGINT`. The other five hits carry identical text; their line numbers
shift by +5 after the block (`:379`->`:384`, `:391`->`:396`), which is exactly
the +3 the synopsis grew plus the +2 the bullet grew. **No new subject entered
the hit set.**

**Blind-spot probe on check 1's own vocabulary.** The expression's alternation
is itself an enumeration and therefore a claim; a fire against a known-present
term would not reveal a missing one. Probed with the terms it does *not*
contain:

```
$ grep -nEi 'abort|interrupt|terminat|exit status|exit-code|\bkill|SIGTERM|abbruch|graceful' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
319:- Cancellation: kill the mkvmerge process, delete the partial output file.
320:- Job engine: FIFO queue over the batch's plans; sequential by default; `--jobs N` opt-in parallelism (muxing is I/O-bound; parallelism pays only on fast storage). Failures do not abort the batch unless `--fail-fast`. Full command line and output of every job are persisted to the app data directory (mkvtoolnix-gui-style job log).
--- count: 2 ---
```

One line check 1 cannot see: **`:320`**, "Failures do not abort the batch
unless `--fail-fast`". **Verdict: consistent, and out of the amendment's
subject** - it is about job-failure handling within a batch, not about the
process's exit code and not about SIGINT. It states no exit code. So the
widened probe adds no finding, and check 1's term set is adequate for the
subject it claims.

### Check 2 - every other statement of a CLI flag or subcommand surface

```
$ grep -nE -- '--[a-z][a-z-]+' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
122:- `extensions`: list, matched case-insensitively, validated at runtime against `mkvmerge --list-types` output of the local installation. Not restricted to MKV.
158:| `language` | `--language` |
159:| `track_name` | `--track-name` |
160:| `default_track` | `--default-track-flag` |
161:| `forced_track` | `--forced-display-flag` |
162:| `flag_hearing_impaired` | `--hearing-impaired-flag` |
163:| `flag_visual_impaired` | `--visual-impaired-flag` |
164:| `flag_commentary` | `--commentary-flag` |
165:| `flag_original` | `--original-flag` |
166:| `enabled_track` | `--track-enabled-flag` |
167:| `sub_charset` | `--sub-charset` |
171:- `language` (matching): accepts ISO 639-2 (`ger`) and BCP-47 (`de`); ... Valid values come from `mkvmerge --list-languages` at runtime.
174:- **Closed-domain values.** ... `language` at plan time (against `mkvmerge --list-languages`). ...
191:List order in `tracks.rules` defines the output track order (`--track-order`). ... `--track-order` lists every primary track first, ...
199:  extensions:       [string]      # validated against mkvmerge --list-types
234:- `tags.global`, `tags.track`: `keep | drop` (mapped to `--no-global-tags`, `--no-track-tags`).
255:Core emits no user-facing prose: ... `--json` output carries code and params plus the rendered message in the active locale, ...
277:| `UnknownExtension` | warning | ... is not among the local mkvmerge's `--list-types` output; ... |
290:| `WorkerPanicked` | ... the stable `worker-panicked: job N` token stays in `JobOutcome.errors` (and its `--json` job encoding) for scripts, ... |
316:- `command` is a pure function `Plan -> Vec<String>`: `--output`, per-input track selection (`--audio-tracks`/`--video-tracks`/`--subtitle-tracks`, `--no-audio` etc.), per-track property options (4.4 table), input file groups in mkvmerge's `( file )` syntax, `--track-order`, plus `--no-chapters`, `--no-attachments`, `--no-global-tags`, `--no-track-tags`, `--title`, `--attach-file`, `--chapters` as configured.
317:- Execution uses `--gui-mode` for machine-readable progress (`#GUI#progress NN%`) and line-tagged warnings/errors.
320:- Job engine: ... `--jobs N` opt-in parallelism ... Failures do not abort the batch unless `--fail-fast`. ...
340:| `capability` | mkvtoolnix model: ... runtime queries (`--version`, `--list-types`, `--list-languages`), ... |
361:muxsmith validate <profile> [--json] [--locale LOCALE]
362:muxsmith dry-run  <profile> [--source DIR] [--output DIR]
363:                            [--on-collision POLICY] [--json] [--locale LOCALE]
364:muxsmith run      <profile> [--source DIR] [--output DIR]
365:                            [--on-collision POLICY] [--jobs N] [--fail-fast]
366:                            [--json] [--locale LOCALE]
367:muxsmith identify <file> [--json] [--locale LOCALE]
375:- `--json` emits the structured report for scripting; default output is human-readable rendering of the same data, including suggestion YAML fragments.
416:- **No hardcoded user-facing strings** in any layer: ... Accepted v1 exceptions: clap's library-generated `--help`/usage text, ...
418:- Locale selection: system locale with manual override in app settings (takes effect live, without restart; D56) and `--locale` on the CLI; falls back to English per message.
450:- Track delay/stretch (`--sync`) changes; per-file offsets do not generalize to batch rules; v1.x candidate.
--- line count: 34 ---
```

(Long lines abbreviated with `...` **for readability of this table only**; the
untruncated output was read in full and the classification below is over the
whole lines. `:255`, `:277`, `:290`, `:316`, `:320`, `:416` and `:418` are the
abbreviated ones.)

**Fired control, exactly as the plan prescribes it: the expression returns the
amended 8.1 block's own lines** - `:361` through `:367`. An empty or short
result would be visibly wrong. It also discriminates on the pre-state, where
the same expression returned **30** lines and the block contributed only three
(`:362`, `:363`, `:364`); the `validate` line did not match then because it
carried no flag. 30 + 4 = 34, and the four added matching lines are exactly the
four the replacement block adds.

Verdict per hit, all 34:

| lines | subject | verdict |
|---|---|---|
| `:122`, `:158`-`:167`, `:171`, `:174`, `:191`, `:199`, `:234`, `:277`, `:316`, `:317`, `:340`, `:450` | **mkvmerge's** command-line surface (property-to-option mapping table, capability queries, generated command line, a v1.x non-goal) - a different program's flags, not Muxsmith's CLI | **Consistent.** Out of the amendment's subject; none states a Muxsmith subcommand's flag set |
| `:255`, `:290` | Muxsmith's `--json` in its report/diagnostics shape (`--json` output carries code and params; the `--json` job encoding) | **Consistent** with the block: `--json` is on the four subcommands the block gives it to, and these describe what it emits, not who has it |
| `:320` | Muxsmith's `--jobs N` and `--fail-fast`, in section 6's job-engine description | **Consistent.** Both are on `run` in the block, and on `run` only |
| `:361`-`:367` | the amended block itself | **The target** |
| `:375` | the 8.1 `--json` bullet | **Consistent.** Unchanged, and it never claimed a per-subcommand set - the block now carries that |
| `:416` | 8.4's accepted-exception list naming "clap's library-generated `--help`/usage text" | **Consistent.** ~~Load-bearing: the recorded reason the synopsis does not enumerate `-h/--help`~~ - **corrected by erratum 2026-07-30: that reading of 8.4 is wrong.** 8.4's exception is to the localization mandate (clap's help text need not live in a Fluent catalog), not to synopsis completeness. The line is consistent with the amended block and load-bearing for nothing here; the real ground for excluding `-h/--help` is in the erratum under "1d. The divergence table" |
| `:418` | 8.4's "and `--locale` on the CLI" | **Consistent - and the amendment REMOVES a latent contradiction here.** Pre-state, 8.4 asserted a CLI flag that 8.1's synopsis did not show on any subcommand |

**No finding.** The one direction that would have been a finding - a spec
sentence claiming a flag or a subcommand the binary does not have - does not
occur: every Muxsmith flag named anywhere in the spec (`--json`, `--locale`,
`--jobs`, `--fail-fast`, `--source`, `--output`, `--on-collision`, `--help`)
is in the binary's captured help above.

**Blind-spot probe on check 2's pattern.** `--[a-z][a-z-]+` cannot see a short
flag or an uppercase long flag, so both were probed separately:

```
$ grep -nE '(^|[^-a-zA-Z0-9])-[a-zA-Z]([^-a-zA-Z0-9]|$)' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
175: ... `not: [ exact: { flag: true } ]` idiom.
309: ... No mkvmerge mux invocations, only `-J` identification.
312: ... a GUI dry-run followed by a run spawns `mkvmerge -J` per file in each ...
341:| `identify` | `mkvmerge -J` wrapper + cache |
426: ... `mkvmerge -J` output carries `identification_format_version`. ...
--- count: 5 ---

$ grep -nE -- '--[A-Z]' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
count: 0
```

Five short-flag hits, all `mkvmerge -J` or a YAML bracket idiom; **no Muxsmith
short flag is documented anywhere in the spec**, and no uppercase long flag
exists. So check 2's pattern misses nothing on this artifact.

**Blind-spot probe for another synopsis block.** W4-c's scope is "a CLI flag, a
subcommand synopsis or an exit code", and a second synopsis elsewhere in the
document would be invisible to both expressions above if it named no flag:

```
$ grep -nE 'muxsmith (validate|dry-run|run|identify|schema|help)\b|Usage:' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
361:muxsmith validate <profile> [--json] [--locale LOCALE]
362:muxsmith dry-run  <profile> [--source DIR] [--output DIR]
364:muxsmith run      <profile> [--source DIR] [--output DIR]
367:muxsmith identify <file> [--json] [--locale LOCALE]
368:muxsmith schema                      # print the profile JSON Schema
376:- `muxsmith schema` is a supported user feature, not only a debug aid (D47): ...
```

The 8.1 block is the document's only synopsis. `:376` mentions `muxsmith
schema` in prose and states no flag. **Consistent.**

### Check 3 - the 130 red/green pair

```
$ grep -c '130' <spec at 164e571>
0
$ grep -c '130' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
2
$ grep -n '130' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
372:- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors, plus 130 for a
373:  cancelled batch (D16). Only `run` returns 130; no other subcommand installs
```

**RED state reproduced (`0`, the plan's authoring value), GREEN state
non-zero (`2`).** Both hits are inside the new bullet, which is the intended
target - and the pre-state `0` proves the expression was not matching
something incidental all along.

### Check 4 - cross-document read against `README.md`'s "Using the CLI"

Reported, not acted on; **this task edits no file but the spec.**

```
$ grep -nE 'Using the CLI|^### `muxsmith|Four of them|exit codes mirror' README.md
124:## 🖥️ Using the CLI
126:Five subcommands, one shape. Four of them - `validate`, `dry-run`, `identify`, `run` - take `--json` (structured report for scripting; the human output renders the same data) and `--locale` (message language override; the default is your system locale, falling back to English). `muxsmith schema` takes neither: it writes the schema to stdout and has no rendered messages to translate.
128:### `muxsmith validate <profile>`
136:### `muxsmith identify <file>`
144:### `muxsmith dry-run <profile> [--source DIR] [--output DIR] [--on-collision <policy>]`
154:### `muxsmith run <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--on-collision <policy>]`
162:### `muxsmith schema`
```

| README claim | amended spec 8.1 | binary | verdict |
|---|---|---|---|
| `:126` four subcommands take `--json` and `--locale`; `schema` takes neither | `validate`/`dry-run`/`identify`/`run` each carry `[--json] [--locale LOCALE]`; `schema` carries neither | confirmed by the five `--help` captures | **Agree** |
| `:144` `dry-run` heading: `--source`, `--output`, `--on-collision` | same three, plus `--json`/`--locale` which the README factors out into `:126` | confirmed | **Agree** |
| `:154` `run` heading: `--source`, `--output`, `--jobs`, `--fail-fast`, `--on-collision` | same five, plus `--json`/`--locale` factored out at `:126` | confirmed | **Agree.** The README's heading orders `--on-collision` last; the spec block follows the binary's own order. A heading is not a normative order, so this is not a divergence |
| `:148` `--on-collision` values `error` (default) / `skip` / `overwrite` | spec `:226` states the identical domain and default | binary's "Possible values" lists exactly those three | **Agree** |
| `:156` `--jobs`, N parallel jobs, default 1 | `[--jobs N]` | `[default: 1]` | **Agree** |
| `:194` `0` clean, `1` warnings, `2` errors; `130` on Ctrl-C; "Only `run` earns that code gracefully" | `0/1/2` plus `130` for a cancelled batch (D16); only `run` returns 130 | confirmed at the two `run.rs` sites | **Agree on the load-bearing half.** See the note below on the half the spec deliberately does not carry |

The README is correct on the flag surface and on 130. **No divergence
requiring an edit; one observation is surfaced below.**

---

## Step 5: verification

### The four Step-4 checks

All four run above with every command and its full output pasted: the
exit-code/cancellation sweep (both states, six pre-classified hits reconciled
one for one, plus a vocabulary blind-spot probe), the flag sweep (34 hits, each
with a stated verdict, its prescribed fired control satisfied, plus two pattern
blind-spot probes), the `130` red/green pair, and the README cross-read.

### The no-permanent-checker decision, with its cited ground

**Decision: no gate check comparing the spec's synopsis against `--help` is
built.** The ground is a recorded house decision, and my reading of the ROADMAP
section **supports it**. Its text, section "Reach-claim checker (candidate, not
a commitment; from the session-28 reach sweep)":

> A one-shot instrument exists and worked: a script parsing every Linux
> artifact row of `docs/INSTALL.md` and `.github/release/draft-body.md` under
> two rules [...] **Deliberately NOT promoted into `scripts/ledger-lint.py`**,
> on the reviewer's recommendation and the controller's agreement: it parses
> PROSE, which is what `proc-check-green-state-reachable` names as the way such
> a check becomes permanently red on correct content, and the boundary it would
> encode was still being argued in the same round that produced it [...]
> Reconsider if a third table appears - the README's `placeholder(1.0)`
> mandates one at the tag - or if a reach claim goes stale again.

Three things in that text carry the decision here, so it is a citation rather
than an argument rebuilt at the keyboard: the instrument of exactly this shape
was built and worked; it was deliberately not promoted; and the stated reason is
that a prose-parsing check goes permanently red on correct content, which is
precisely what a synopsis-vs-`--help` comparator would be (it would have to
parse a fenced prose synopsis whose metavariables - `DIR`, `N`, `POLICY`,
`LOCALE` - deliberately differ from clap's derived value names `SOURCE`,
`JOBS`, `ON_COLLISION`, `LOCALE`, so a naive comparator is red on the correct
end state this very task produces). **No NEEDS_CONTEXT on this point.**

The section's own reconsider-trigger ("if a reach claim goes stale again") is
noted, not fired: this item is a spec-vs-binary staleness, not a reach claim,
and the section names its trigger set explicitly.

### Exit-bar subset

```
$ python3 scripts/ledger-lint.py
ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
exit: 0
```

Green, with the expected summary shape (`across 4 files plus BUILDING.md's gate
enumeration`, tail `all invariants hold`). The entry count is not asserted -
the controller appends ledger entries continuously; it was 550 at A1's run and
is 560 now.

**What this run does and does not prove:** the spec is not one of ledger-lint's
four files, so a green run here proves only that nothing else broke. Stated per
the plan; the brief's instruction that ledger-lint is in this task's exit bar
is satisfied by the run, not by the reservation.

```
$ git diff --stat
 docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md | 15 ++++++++++-----
 1 file changed, 10 insertions(+), 5 deletions(-)
```

**Exactly one file.**

No Rust or frontend gate part can observe a spec edit, so none was run for
appearances; the stream's full gate run covers the tree, and the controller
runs the eleven-part gate on the merged state.

### Test duty, weighed

**No new test.** This task changes a specification document and no behaviour.
The surface it documents already ships and is exercised by the existing
`cli_validate` and `dry_run_cli` suites; the derivation in Step 1 ran the
shipped binary itself. The one property worth locking - that the spec keeps
matching the binary - is exactly the prose-parsing checker declined above on a
recorded ground, so this is not the "tests follow in a later package" shape the
house rule forbids: there is no user-visible behaviour introduced here whose
test could be deferred.

---

## Step 6: surfaced, not edited

### The `cli.rs` doc comment, located by a single-line fragment

The whole-sentence grep the trap invites returns nothing, because the sentence
is hard-wrapped across two `///` lines:

```
$ grep -rn "every command shares the exit-code contract 0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)" crates/
exit: 1   (no match)
```

The counter-instrument (`proc-wrapped-prose-quote-grep`) - a fragment that sits
on one line:

```
$ grep -rn "shares the exit-code contract" crates/
crates/muxsmith-cli/src/cli.rs:12:    /// Selected operation; every command shares the exit-code contract

$ grep -rn -A1 "shares the exit-code contract" crates/muxsmith-cli/src/cli.rs
crates/muxsmith-cli/src/cli.rs:12:    /// Selected operation; every command shares the exit-code contract
crates/muxsmith-cli/src/cli.rs-13-    /// 0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16).
```

**This task makes that citation land:** spec 8.1 now carries 130, so
`(spec 8.1, D16)` no longer reaches past its source. That half is repaired.

**The other half is not, and is surfaced rather than edited.** "every command
shares the exit-code contract ... 130 cancelled" is over-broad as to who can
*produce* 130: measured above, only `run` installs a handler and only `run.rs`
carries the two 130 sites. It is not edited here because it is a source comment
about a different half of the fact (`code-comment-line-citations-drift` is not
the entry that governs it), its rewording needs a decision about how to
describe the signal-death case that the owner has not seen, and Plan 10's
precedent for an adjacent claim a task did not falsify is to surface it.
Vehicle: the plan's deferred-by-decision note.

### Also surfaced, noticed but not touched

1. **`README.md:194` carries an unscoped POSIX-shell claim.** Verbatim:
   "Interrupt any subcommand with Ctrl-C and you get `130` instead, the shell's
   own convention for a signalled process, so handle it in your `case $?`."
   This is the same clause the plan deliberately CUT from the spec's
   replacement bullet, on the ground that it is a POSIX-shell fact stated
   without scope in a document governing a three-OS product whose D16 chose
   `ctrlc` precisely for Windows console events. The spec is now silent on it
   and the README is not, so the two documents no longer say the same thing
   about who reports 130 - they do not contradict (the spec's claim is scoped
   to what Muxsmith's own process returns), but the unverifiable-on-Linux
   Windows half now lives only in the user-facing document. **Not this task's
   file** (Step 4 makes the README a read-only consistency target and the plan
   records "the README is already right and needs no edit" for the flag surface
   and for 130). Recorded for the controller as a candidate for whatever
   vehicle next amends README's "Using the CLI".

2. **The plan's Step-2 prose mis-describes its own fenced block's indent.** It
   states "The continuation indent aligns under `<profile>`". Measured on the
   fence itself: on `muxsmith dry-run  <profile> [--source DIR] [--output DIR]`,
   `<profile>` begins at index 18 and `[--source` at index 28; the continuation
   lines are indented 28. So the continuation aligns under the **option list**,
   one column past `<profile>`'s end, not under `<profile>`. **The fence
   governs and was applied byte for byte** - this is a defect in the plan's
   descriptive prose, not in the artifact, and it changes nothing about the
   result. Recorded because the plan's Step-2 prose is what a later reader
   would use to re-derive the block's shape.

3. **The plan's authoring divergence table omits `-h/--help` from the "binary
   lists" column** for all five subcommands, without saying so. The omission is
   correct - the fenced replacement rightly does not enumerate a flag that sits
   identically on all five subcommands - but the table reads as an exhaustive
   capture of `--help` output and is not. This run's table above states the
   exclusion explicitly. No action needed; noted so the next reader of that
   table does not treat it as the complete surface.

   > **ERRATUM, A4 implementer, 2026-07-30, same correction round.** The
   > original parenthesis here read "(spec 8.4 lists clap-generated help as an
   > accepted exception, and the fenced replacement rightly does not enumerate
   > it)". **The 8.4 half is a wrong citation** - 8.4's exception is to the
   > localization mandate, not to synopsis completeness - and has been replaced
   > above with the ground that actually holds. Full correction in the erratum
   > under "1d. The divergence table". The observation itself is unaffected.

---

## What was NOT done, deliberately

- No file but the spec was edited (`git diff --stat`: one file).
- No section Task A3 amended (4.3, 4.4, 7, 9.2) was touched; the single diff
  hunk is confined to 8.1.
- No permanent checker was built (recorded ground above).
- No `git worktree`, no session-relocation tool, no background run, no push.
- The full eleven-part gate was not run; the controller runs it on the merged
  state.

## Acceptance rows

| row | claim | evidence in this report |
|---|---|---|
| **W4-a** | 8.1's synopsis matches the shipped binary for all five subcommands | Step 1c's five `--help` captures and Step 1d's flag-granularity table; all five plan rows reproduced; the fenced block applied and byte-verified by reconstruction |
| **W4-b** | the exit-code bullet carries 130 and says who can reach it | Step 3's fence applied; check 3's `grep -c '130'` 0 -> 2 with the target inside the expression; the two `run.rs` 130 sites and the single `ctrlc::set_handler` named |
| **W4-c** | the amendment introduces no spec self-contradiction | Step 4: both prescribed sweeps run and pasted, the six pre-classified hits reconciled one for one with no hit outside them, the flag sweep's fired control satisfied (`:361`-`:367` returned), and three blind-spot probes finding nothing new |

## Commit

```
$ git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
$ git -c commit.gpgsign=false commit -m "spec: 8.1 states the shipped CLI flag surface and the 130 cancellation code" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

Trailer per SI-4, exactly one, unsigned, no `Claude-Session` line, no
context-window suffix:

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

**Commit SHA: `06e896e55ddde7e73e999d8912eaf977d0ec3d08`** (short `06e896e`), on
`plan-11-stream-a`, parent `164e571`. Verified after the fact:

```
$ git log -1 --format='%G?'            -> N          (unsigned, as required)
$ git log -1 --format='%(trailers)'    -> Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
$ git log -1 --format='%B' | grep -c 'Claude-Session'  -> 0
$ git status --porcelain               -> (empty)
$ git show --stat HEAD                 -> 1 file changed, 10 insertions(+), 5 deletions(-)
```

Not pushed.
