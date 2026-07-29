# Task 4 report - Plan 10, W5: the user-facing documentation pass

**Status: DONE_WITH_CONCERNS** (three numbered concerns below, none of them a
blocker; one is a hand-off to Task 5, two are wording judgements a reviewer
should rule on).

Files touched, exhaustively: `README.md`, `docs/INSTALL.md`. Gate green in
foreground, all eleven parts. Committed on `master`, unsigned, explicit
pathspecs.

---

## Step 1: the CLI surface, re-derived from the binary

Binary rebuilt from the current tree before capture:

```
$ cargo build -p muxsmith-cli
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
   Compiling muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
```

### Pasted help output

```
$ ./target/debug/muxsmith --version
muxsmith 0.1.0
```

```
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
```

```
$ ./target/debug/muxsmith schema --help
Print the profile JSON Schema

Usage: muxsmith schema

Options:
  -h, --help  Print help
```

```
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
```

```
$ ./target/debug/muxsmith identify --help
Identify one source file via mkvmerge and print its tracks

Usage: muxsmith identify [OPTIONS] <FILE>

Arguments:
  <FILE>  Path to the media file to identify

Options:
      --json             Emit the structured identification as JSON
      --locale <LOCALE>  Locale for rendered messages (default: system, fallback en)
  -h, --help             Print help
```

```
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
```

Short-form help for the two long-help subcommands, captured because it is the
form that carries the possible-value set inline:

```
$ ./target/debug/muxsmith dry-run -h
Plan the batch without muxing: identify sources, resolve rules, and print the per-file resolution, diagnostics, and suggestions

Usage: muxsmith dry-run [OPTIONS] <PROFILE>

Arguments:
  <PROFILE>  Path to the profile file

Options:
      --source <SOURCE>              Source directory to scan (overrides the profile default)
      --output <OUTPUT>              Output directory (overrides the profile default)
      --on-collision <ON_COLLISION>  Collision policy override (spec 4.2 run input); falls back to the profile's `output.on_collision` when unset [possible values: error, skip, overwrite]
      --json                         Emit the structured batch report as JSON
      --locale <LOCALE>              Locale for rendered messages (default: system, fallback en)
  -h, --help                         Print help (see more with '--help')
```

```
$ ./target/debug/muxsmith run -h
Plan and execute the batch (spec 5.5 level 3)

Usage: muxsmith run [OPTIONS] <PROFILE>

Arguments:
  <PROFILE>  Path to the profile file

Options:
      --source <SOURCE>              Source directory to scan (overrides the profile default)
      --output <OUTPUT>              Output directory (overrides the profile default)
      --on-collision <ON_COLLISION>  Collision policy override (spec 4.2 run input); falls back to the profile's `output.on_collision` when unset [possible values: error, skip, overwrite]
      --jobs <JOBS>                  Parallel mux jobs (default 1 = sequential) [default: 1]
      --fail-fast                    Stop dequeuing after the first failed job (in-flight finish)
      --json                         Emit the structured run report as JSON
      --locale <LOCALE>              Locale for rendered messages (default: system, fallback en)
  -h, --help                         Print help (see more with '--help')
```

### Exit codes, derived from the three named sources

Not in the help text. Each source named with its file, because two of the three
do not live in `cli.rs`:

| symbol | file | what it establishes |
|---|---|---|
| `Cli`'s doc comment on the `command` field | `crates/muxsmith-cli/src/cli.rs` | `0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)` - the contract as the CLI states it |
| `severity_exit` | `crates/muxsmith-cli/src/commands/mod.rs` | `Error -> 2`, `Warning -> 1`, else `0`. Shared by `diag_exit_code` and `validate::run` |
| `job_exit_code` | `crates/muxsmith-cli/src/commands/run.rs` | queue worst-of fold `2 / 1 / 0`; combined with `diag_exit_code` via `max` |

**Where 130 actually comes from, measured rather than inferred.** `grep -rn '130'
crates/muxsmith-cli/src/` returns six lines and every producing one is in
`run.rs`: the cancel-flag early return, and the ctrlc handler's second-SIGINT
force exit. `cli.rs` states 130 as part of the shared contract, but no other
subcommand has a path that emits it. The README sentence is therefore worded to
attach 130 to `run`, which is both what `cli.rs` promises and what the code can
produce. See concern 2.

`run.rs`'s own rustdoc on the run entry point supplies the behaviour the README
sentence names: "A `ctrlc` handler installed just before the queue runs flips
that flag on the first SIGINT (the queue kills in-flight jobs, partials are
deleted, the summary still prints)".

### Divergence table, at flag granularity

| # | README says | Binary says | Verdict |
|---|---|---|---|
| D1 | "Scriptable everything: **every command** takes `--json`" (the What-you-get bullet) | `schema --help` lists `-h, --help` only. `--json` exists on `validate`, `dry-run`, `identify`, `run` - four of five | **DIVERGENT, corrected.** Plan premise reproduced |
| D2 | "Five subcommands, one shape. **Every one of them** takes `--json` ... and `--locale`" (CLI section opener) | same: `schema` takes neither. `Cmd::Schema` in `crates/muxsmith-cli/src/cli.rs` is a unit variant with no fields; its arm in `main.rs` prints the schema and returns `0` without touching a renderer | **DIVERGENT, corrected.** Plan premise reproduced |
| D3 | `--on-collision <policy>` in both the `dry-run` and `run` synopsis headings; the value set is never enumerated anywhere in the README | `error` (Refuse the colliding output (default policy)), `skip` (Skip the colliding output with a warning), `overwrite` (Replace the pre-existing file); falls back to the profile's `output.on_collision` when unset | **DIVERGENT (omission), corrected.** Domain enumerated where the flag is introduced |
| D4 | `--locale` described as "(message language override)"; no default stated | `Locale for rendered messages (default: system, fallback en)` | **DIVERGENT (omission), corrected** |
| D5 | "exit codes mirror mkvmerge's own: `0` clean, `1` finished with warnings, `2` errors" and stops | `cli.rs` adds `130 cancelled`; `run.rs` produces it at two sites | **DIVERGENT, corrected** |
| D6 | "Five subcommands" | binary lists `validate`, `schema`, `dry-run`, `identify`, `run`, plus clap's auto-generated `help` | **NOT divergent.** Five real subcommands; `help` is clap's built-in and not a product surface |
| D7 | `muxsmith run ... [--jobs N]`, prose "(default 1)" | `--jobs <JOBS>`, `[default: 1]`, "Parallel mux jobs (default 1 = sequential)" | **AGREES** |
| D8 | `--fail-fast` "stops dequeuing new jobs after the first failure and lets in-flight jobs finish cleanly" | "Stop dequeuing after the first failed job (in-flight finish)" | **AGREES** |
| D9 | "command-line flags override profile-stored values (`--source`, `--output`, `--on-collision`)" | `--source`/`--output`: "overrides the profile default"; `--on-collision`: "falls back to the profile's `output.on_collision` when unset" | **AGREES** |
| D10 | `muxsmith validate <profile>` - "profile" unqualified | about line: "Statically validate a profile (YAML or JSON)" | **NOT divergent.** An omission that makes no false claim; the README's `<profile>` is format-neutral. Not written, to keep the fact set closed |
| D11 | README documents no top-level `--version` / `-V` | top level carries `-h, --help` and `-V, --version` | **NOT divergent.** Omission, no false claim. Not written |
| D12 | `muxsmith identify <file>`, `muxsmith schema` synopses | `muxsmith identify [OPTIONS] <FILE>`, `muxsmith schema` | **AGREES** (the `[OPTIONS]` placeholder is clap boilerplate; the flags it stands for are covered by the section opener) |

**Every correction the plan listed reproduced.** Nothing in the plan's Step-2
list failed re-measurement, so there is no drop to report as a finding. The
table adds one correction beyond the plan's named four (D3's second half: the
override-fallback sentence), which lands in the same enumerating sentence.

**Observation for the controller, not a correction:** spec section 8.1's own
synopsis block is stale in the other direction - it shows
`muxsmith validate <profile>` with no `--json`, while the binary has carried
`validate --json` since before this plan. The README is now correct against the
binary; the spec block is not. Out of this task's Files list; surfaced.

---

## Steps 2 and 3: per-claim verification

### The two over-broad claims in the exact-typed-matching paragraph

**Claim A: "`exact` compares every property in its own domain."**

- Spec 4.4, `raw:` opt-in bullet: a `raw:` property "bypasses the
  existence/type/domain checks and is matched untyped (byte-literal value
  equality against the property named verbatim, no `language` normalization or
  `codec_kind` aliasing, no false-when-absent Boolean shortcut)".
- Code, `crates/muxsmith-core/src/matcher.rs::exact_matches`: the function's
  first statement strips a `raw:` prefix and returns `scalar_eq` against
  `item.get(bare)` before reaching any of the typed arms.
- **Verdict: over-broad as written. Corrected** to "each **known** property in
  its own domain", which is exactly the set `capability::matchable_type`
  answers for and excludes the `raw:` bare name (`matchable_type` returns `None`
  for it, as the function's own comment records).

**Claim B: "Aiming `substring` or `regex` at a non-string property is a
config-time error."**

- Not sufficient: spec 4.4's `raw:` bullet and
  `crates/muxsmith-core/src/profile/validate.rs::validate_expr` - in the
  `substring`/`regex` loop the `raw:` branch pushes `raw_opt_in_diagnostic` and
  performs no type check at all, so a `raw:`-prefixed name is never a
  `NotStringProperty` error whatever the underlying value is.
- Not necessary: spec 4.4's `codec_kind` bullet ("Usable only under `exact`;
  `substring`/`regex` on `codec_kind` is a config-time error
  (`CodecKindExactOnly`)"), and `validate_expr`'s `codec_kind` guard, which
  fires *before* the string-type check precisely because `codec_kind` is
  `String`-typed (`capability::matchable_type` returns `Some(PropType::String)`
  for it) and would otherwise pass.
- **Verdict: neither necessary nor sufficient, as the plan measured. Corrected**
  to a known-property statement plus the explicit `codec_kind` carve-out; the
  `raw:` carve-out is carried by anchor item 4 immediately below, which is the
  plan's stated design (the anchor is what makes the paragraph's absolutes true
  again).

### The four anchor items, each against spec section and core symbol

| # | Anchor item | Spec section | Core symbol(s) | Verdict |
|---|---|---|---|---|
| 1 | `language`: ISO/BCP-47 normalization plus dual-field lookup against `language` and `language_ietf` | 4.4 ("Conveniences", `language` bullet: "matched semantically against both `language` and `language_ietf` as reported by mkvmerge"); 4.3's closing paragraph for the canonical-form rule | `matcher.rs::exact_matches` `"language"` arm, which iterates `["language", "language_ietf"]`; `matcher.rs::lang_eq`; `matcher.rs::canonical_tag`; `capability::runtime::LanguageIndex::normalize` | **SUPPORTED, written** |
| 2 | Absent boolean flags compare equal to `false` under `exact` | 4.4 ("Boolean flags, absent = false"), naming the four vanity flags | `matcher.rs::exact_matches` fallback arm: `None => match matchable_type(prop) { Some(PropType::Boolean) => scalar_eq(want, &PropValue::Bool(false)), _ => false }`; `capability::matchable_type` | **SUPPORTED, written** |
| 3 | Curated closed domains for `type` and `codec_kind`; out-of-domain value is a config-time error, not a silent never-match | 4.4 ("Closed-domain values"): `type` and `codec_kind` at config time, `language` at plan time | `capability::matchable_domain` (returns `TYPE_VALUES` for `type`, `CODEC_KIND_NAMES` for `codec_kind`, `None` otherwise); `profile/validate.rs::validate_expr`, whose exact arm pushes `InvalidPropertyValue` on a non-member | **SUPPORTED, written** |
| 4 | `raw:`'s contrast: byte-exact, single-field, no normalization, no aliasing, no false-when-absent shortcut | 4.4 (`raw:` opt-in bullet, D32/spec 9.2) | `matcher.rs::exact_matches`'s `raw:` early return (single `item.get(bare)`, `scalar_eq`, `None => false`); `profile/validate.rs::validate_expr`'s two `raw:` branches, which `continue`/skip the type, domain and existence checks | **SUPPORTED, written** |

No anchor item was refuted, so nothing is reported-instead-of-written.

Item 3's README example (`type: subtitle` against the domain's `subtitles`) is
derived from `capability::TYPE_VALUES`, which is
`["audio", "buttons", "subtitles", "video"]`.

### What was written

The paragraph, as committed:

> Matching is **typed, not stringly**. `exact` compares each known property in
> its own domain: booleans as booleans, numbers numerically (`6` equals `6.0`),
> languages as languages - `de` matches a file tagged `ger`, and `pt-BR` does
> **not** match `pt-PT`. For the properties that genuinely are strings - track
> names, codec IDs - `substring` (case-insensitive containment) and `regex` do
> the messy-reality work, and `any`/`not` combine expressions into whatever
> your library's chaos requires. Point `substring` or `regex` at a known
> property that is not string-typed and you get a config-time error, not a
> silent never-match; `codec_kind` rejects both conditions even though it is
> string-typed, because a pattern over a curated alias is ill-defined -
> pattern-match `codec_id` instead. Rule order is output track order.

The four-item list follows it immediately, under the lead-in "Three places where
`exact` does more than compare, and one where it deliberately does less." That
lead-in is deliberate: the anchor is three magic properties plus one contrast,
and calling all four "magic" would misdescribe item 4. Register per the ROADMAP
README entry's recorded sell-tone exception; the wording is mine, the fact set
is the plan's.

---

## Step 4: the two counts, measured

### The decision series

```
$ git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE 'D[0-9]+' | wc -l
104
$ git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE 'D[0-9]+' | sort -u | wc -l
103
$ git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE '[0-9]+' | sort -n | tail -1
105
$ git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE '[0-9]+' | sort -n | head -1
1
```

**Reproduces the authoring measurement exactly: 104 defining headings, 103
distinct numbers, reaching D105.**

Boundary checks on the non-contiguity, both re-run rather than taken from the
plan:

```
$ ... | sort -u > have.txt; seq 1 105 | sort -u > want.txt; comm -13 have.txt want.txt
73 74
$ git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE 'D[0-9]+' | sort | uniq -d
D32
$ git grep -nE '^#{1,4} +\**D32' -- 'docs/superpowers/specs/*'
docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:9:## D32: UnknownPropertySkew mitigation - explicit `raw:` opt-in (shape B)
docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:152:## D32 addendum: two sub-decisions PENDING Şenol (2026-07-12, T16 review)
```

104 headings minus the D32 addendum = 103 distinct decisions; 105 minus the two
gaps at 73/74 = 103. The two derivations agree, which is the arithmetic control.

The "reserved and never spent" clause is verified rather than borrowed from the
plan's prose:

```
$ git grep -nE '^#{1,4} +\**D(6[5-9]|7[0-9])\b' -- 'docs/superpowers/specs/*'
docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md: D65 ... D72   (eight headings, stopping at D72)
docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:      D75 onward
$ grep -rn "D65-D74" docs/ | head -3
docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:5:at **D75** per the ROADMAP Plan-8 kickoff block; D65-D74 is Plan 7.5's
docs/process-journal/artifacts/plan-7.5-sdd/design-review-brief.md:52: D-numbering stays within D65-D74.
docs/process-journal/artifacts/plan-8-sdd/design-brief.md:130: D-numbering: D75 upward (Plan 7.5's parallel design owns D65-D74).
```

Plan 7.5 was allocated the block D65-D74 and spent D65 through D72. That is the
reservation the README sentence now names.

**Sentence as written:** "Every design decision is numbered and recorded with
its rationale and rejected alternatives: 103 of them so far, running up to
`D105` because two numbers were reserved for a plan that never spent them."

**Which of reach and count it states, and why: both, explicitly separated.** The
plan permitted either "both as the two different things they are" or "only one
of them". Both is the better choice here because the README's original sentence
was a range claim and a reader who only sees `103` and then meets a `D104`
citation elsewhere in `docs/` would think the number stale. Naming the reach and
the reason for the gap makes the figure re-derivable, which is the step's stated
bar. "D1 through D105" is exactly what was avoided.

### The verdict count

The unit the prose names, measured over tracked files:

```
$ git ls-files 'docs/*' | grep -icE '/[^/]*verdict[^/]*$'
219
```

**Reproduces the authoring measurement exactly: 219.**

The frozen-unit counter-measurement, pasted so the fork is visible rather than
taken on trust:

```
$ git ls-files 'docs/*' | grep -cE '/verdicts/'
78
```

**78 is the README's current figure, and re-measuring under that unit would have
looked like confirmation.** The historical half of the plan's claim was
re-verified rather than borrowed:

```
$ git log -1 --format='%h %ad %s' --date=short 62aaf61
62aaf61 2026-07-11 docs: README v1 (sell-tone, WIP banner, CLI reference, human-AI story)
$ git ls-tree -r --name-only 62aaf61 -- docs | grep -icE '/[^/]*verdict[^/]*$'
78
$ git ls-tree -r --name-only 62aaf61 -- docs | grep -cE '/verdicts/'
78
```

At the commit that introduced the sentence the two units were identical at 78.
Today they read 219 and 78. `78` was correct when written and went wrong by a
storage-convention change, not by growth.

**Three boundary checks, each with its own fire control**, because a check whose
passing result is an absence proves nothing until it has produced output once:

| check | result | fire control | control result |
|---|---|---|---|
| every match is markdown: `... \| grep -v '\.md$' \| wc -l` | `0` | the same `-v '\.md$'` filter over the whole `git ls-files 'docs/*'` list | `221` matched (first three: `docs/conventions.yaml`, `docs/decision-ledger.yaml`, `docs/process-conventions.yaml`) - the filter demonstrably matches a non-markdown path |
| no review BRIEF is caught: `... \| grep -ic 'brief'` | `0` (grep exit 1) | the same `grep -ic 'brief'` over the whole `docs/` list | `185` - the pattern demonstrably matches a brief when one is in the input |
| no file under a `verdicts/` directory is missed by the basename rule: `git ls-files 'docs/*' \| grep -E '/verdicts/' \| grep -vicE '/[^/]*verdict[^/]*$'` | `0` (grep exit 1) | the same `grep -vicE '/[^/]*verdict[^/]*$'` over the whole `docs/` list | `889` - the inverted basename filter demonstrably matches when a non-verdict basename is in the input |

**Sentence as written:** "The whole process is public in this repo: [docs/](docs/)
carries the process journal, every plan, and the preserved review verdicts - 219
files under `docs/` with `verdict` in the name, including the ones that hurt."

The unit is named in the sentence itself (`files under docs/ with verdict in the
name`), which is what makes it re-derivable and what distinguishes it from the
frozen `verdicts/`-directory unit that produced 78.

---

## Step 5: the `docs/INSTALL.md` note

### (a) The Linux section's note, as written

```
**Unsigned packages (Fedora):** the `dnf install` above prints
`Warning: skipped OpenPGP checks for 1 package from repository: @commandline`
before it proceeds. `@commandline` is dnf's own name for a package you
handed it by path, and the line says what it means: the rpm carries no
OpenPGP signature for dnf to check. That is deliberate - the same
unsigned-artifact policy the Windows and macOS sections describe - and
it is a warning, not a gatekeeping dialog: nothing blocks, nothing needs
clicking, the install completes. Check the download against `SHA256SUMS`
as above instead.
```

It sits immediately after the Linux artifact list and immediately before the
existing "No gatekeeping dialog exists on Linux." sentence, which stays true and
is now confirmed rather than contradicted by the note's own "it is a warning,
not a gatekeeping dialog" clause. The shape follows the macOS Gatekeeper and
Windows SmartScreen blocks: bold label, what the user meets, what it means, that
it is expected policy, what to do instead.

**Scope held:** the note documents the dnf case only. No claim is made about
what `sudo apt install ./...deb` prints, because nobody measured it.

**The tool is named as `dnf`, not `rpm`.** The note's first clause points at the
`dnf install` command the file already documents two lines above, and the
`@commandline` sentence names dnf as its owner.

### The string, grepped against the ROADMAP's own record

```
$ grep -Fn 'Warning: skipped OpenPGP checks for 1 package from repository: @commandline' docs/ROADMAP.md
904:  `Warning: skipped OpenPGP checks for 1 package from repository: @commandline`
$ grep -Fn 'Warning: skipped OpenPGP checks for 1 package from repository: @commandline' docs/INSTALL.md
88:`Warning: skipped OpenPGP checks for 1 package from repository: @commandline`
```

Byte-for-byte comparison of the two extracted occurrences:

```
$ grep -Fo '<string>' docs/INSTALL.md > a.txt
$ grep -Fo '<string>' docs/ROADMAP.md  > b.txt
$ diff a.txt b.txt && echo IDENTICAL
IDENTICAL
```

**Fire control for the string grep** (a near-miss must return nothing, so the
IDENTICAL above is not a pattern that matches anything):

```
$ tr '\n' ' ' < docs/ROADMAP.md | tr -s ' ' | grep -oc 'Warning: skipped OpenPGP checks for 1 packages from repository: @commandline'
0   (grep exit 1)
```

`packages` instead of `package` returns nothing, so the grep discriminates.

**The string is on one unbroken line in `INSTALL.md`**, which matters in a
hard-wrapped file and is the whole point of `proc-wrapped-prose-quote-grep` -
a reader who meets the warning and greps for it must hit:

```
$ awk '/Warning: skipped OpenPGP/ {print FILENAME":"NR": ["$0"]"}' docs/INSTALL.md
docs/INSTALL.md:88: [`Warning: skipped OpenPGP checks for 1 package from repository: @commandline`]
```

### (b) The file-top HTML comment, extended

Located by `code signing lands` on its own line, as the plan prescribes, not by
the sentence (it is hard-wrapped across three lines, 22-24, and does not grep as
one string). Joined form before and after:

```
before: <!-- When code signing lands (registered ROADMAP trigger), the SmartScreen and Gatekeeper sections below shrink to the signed-app reality; keep the CLI/PATH halves. -->
after:  <!-- When code signing lands (registered ROADMAP trigger), the SmartScreen, Gatekeeper and Linux unsigned-package sections below shrink to the signed-app reality; keep the CLI/PATH halves. -->
```

The "before" form is byte-identical to the string the plan's Amendment 2 records,
verified by `sed -n '22,24p' docs/INSTALL.md | tr '\n' ' ' | tr -s ' '` on the
pre-edit file. The enumeration is now three members, so the 1.x signing work does
not leave the new note stranded.

---

## Step 6: what stayed untouched, verified

```
$ grep -c 'placeholder(1.0)' README.md
4
$ grep -n 'placeholder(1.0)' README.md
7:<!-- placeholder(1.0): GIF - profile -> dry-run -> run in ~15 seconds -->
68:<!-- placeholder(1.0): real dry-run output snippet, regenerated from an actual run -->
106:<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
196:<!-- placeholder(1.0): one GUI screenshot -->
$ grep -n 'Work in progress' README.md
5:> ⚠️ **Work in progress.** ...
```

All four placeholders and the WIP banner intact.

**The passthrough recipe's YAML is byte-identical to the literal
`crates/muxsmith-cli/tests/run_live.rs` inlines**, checked directly rather than
relying on the gate alone (the recipe moved from README lines 71-78 to 79-84
because the anchor list was inserted above it, but its content did not change):

```
$ sed -n '79,84p' README.md > readme_recipe_now.txt
$ diff readme_recipe_now.txt test_recipe.txt && echo "IDENTICAL (recipe untouched)"
IDENTICAL (recipe untouched)
```

Fire control for that diff: mutating one word of the README side
(`keep` -> `drop`) makes the same comparison exit `1`.

---

## Step 7: verification

### The full gate as `BUILDING.md` enumerates it - eleven parts, foreground, green

Rust block (6):

```
$ cargo fmt --all --check
exit=0
$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
exit=0
$ cargo test --workspace
   ... Doc-tests xtask / test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
exit=0
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
exit=0
$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
exit=0
$ cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
exit=0
```

Frontend block (4):

```
$ pnpm lint
$ eslint .
exit=0
$ pnpm build
✓ built in 153ms
exit=0
$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
exit=0
$ pnpm test:e2e
  68 passed (2.9s)
exit=0
```

House-knowledge block (1):

```
$ python3 scripts/ledger-lint.py
ledger-lint: 538 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
exit=0
```

**No `FAIL BUILDING.md: ...` line**, so Task 1's new check saw a `BUILDING.md`
this task did not touch, as intended.

**The narrow but real role of the green gate for a prose task**, stated as such:
no test asserts README prose. `crates/muxsmith-cli/tests/run_live.rs` inlines the
README passthrough recipe verbatim and drives it through `dry-run --json` and
`run`, so a change to that recipe would turn the gate red. It did not:

```
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok
```

`cargo test --workspace` reported 505 `... ok` lines in total.

**Note on the shell**, because it changes how these exit codes were captured:
this is zsh, where `${PIPESTATUS[0]}` is empty (bash-only). Exit codes above were
taken from `$?` directly, or from `${pipestatus[1]}` where output was piped to
`tail`.

### Typography and citation sweeps, each fire-controlled

```
$ grep -nP '[\x{2013}\x{2014}\x{2012}\x{2015}\x{2212}\x{201C}\x{201D}\x{2018}\x{2019}\x{2026}\x{00A0}]' README.md docs/INSTALL.md
   (no output, exit 1)
```

Fire control: the same expression against a file containing one em-dash returns
`1:a — b`, exit 0. So the clean result is a measurement, not a malformed pattern.

New `file:line` citations introduced by this task (Task 5 sweeps that class next,
and three of its sites cite README spans, so introducing one here would be
self-defeating):

```
$ git diff -U0 -- README.md docs/INSTALL.md | grep -E '^\+' \
    | grep -nE '[A-Za-z0-9_./-]+\.(rs|ts|vue|mjs|js|py|toml|ftl|json|yaml|md):[0-9]+|(^|[[:space:]`,(])[:][0-9]+'
   (no output, exit 1)
```

Fire control: the same expression against `+ see README.md:91 and design
`:889-936`` matches, exit 0. **None introduced.**

### `git diff --stat`

```
$ git diff --stat
 README.md       | 19 ++++++++++++++-----
 docs/INSTALL.md | 14 ++++++++++++--
 2 files changed, 26 insertions(+), 7 deletions(-)
$ git status --porcelain
 M README.md
 M docs/INSTALL.md
```

Exactly the two files in the Files list. Nothing else.

---

## Divergences and judgment calls, each named

1. **`--on-collision`'s domain landed in the `dry-run` subsection's prose, not in
   the two synopsis headings.** The plan says the domain "is enumerated where the
   flag is introduced, instead of being left as a bare `<policy>` placeholder in
   two synopses". Two readings existed: replace `<policy>` in both headings with
   `error|skip|overwrite`, or enumerate the domain at the flag's first
   appearance and leave the headings scannable. I took the second: the headings
   are synopsis lines whose job is shape, the `dry-run` subsection is where a
   reader first meets the flag, and enumerating it twice in two headings would
   repeat rather than inform. The defect the plan names - the domain existing
   nowhere in the README - is closed either way. Reviewer call; see concern 1.
2. **The 130 sentence is attached to `run`, not to "every command".** `cli.rs`
   calls 130 part of the shared contract, but the measurement in Step 1 shows
   `run.rs` as its only producer. Writing "every command can exit 130" would
   have been a fresh false absolute in the very paragraph this task is fixing
   absolutes in. See concern 2.
3. **The anchor list's lead-in says "three places ... and one where it
   deliberately does less"**, not "four magic properties". Acceptance row W5-c
   is worded "the four magic properties are explicitly listed", but the plan's
   own Step 3 enumerates item 4 as "`raw:`'s **contrast**". Calling `raw:` magic
   would state the opposite of what it is. The list is four items, as required;
   only its lead-in describes them honestly. See concern 3.
4. **D10 and D11 (validate's YAML-or-JSON acceptance, the top-level `--version`)
   were NOT written.** Both are omissions in the README that make no false
   claim, so neither is a divergence, and the fact set for this task is closed
   by the plan. Recorded in the table rather than acted on.

---

## Numbered concerns a reviewer can rule on yes/no

1. **Is enumerating `--on-collision`'s domain in the `dry-run` subsection's prose
   (rather than inside the two synopsis headings) the right reading of Step 2's
   "enumerated where the flag is introduced"?** If not, the fix is mechanical:
   replace `<policy>` with `error|skip|overwrite` in the two `###` headings.
2. **Is "a `run` you interrupt with Ctrl-C exits `130`" the right scope for the
   130 sentence**, given that `cli.rs` states it as a contract shared by every
   command while `run.rs` is its only producer? The alternative is a
   command-neutral "`130` cancelled", which matches `cli.rs`'s wording and
   over-claims against the code.
3. **Is the list's lead-in acceptable given W5-c's "four magic properties"
   phrasing?** The list has four items as specified; the lead-in declines to
   call the fourth magic because item 4 is the plan's own "contrast".

---

## What I surface for the controller

1. **Task 5 hand-off, load-bearing.** `crates/muxsmith-cli/tests/run_live.rs`
   carries a comment quoting README text this task changed: `per README.md:91
   "every command takes --json"`. That sentence no longer exists in the README.
   Task 5 rewrites that site anyway (it is one of the three README-citing sites),
   but its rewrite must not preserve the old quotation - it is now false. The
   two other README-citing sites (`README.md:71-78`, twice) point at the
   passthrough recipe, whose content is unchanged but whose line span moved from
   71-78 to **79-84** because the anchor list was inserted above it. Task 5
   replaces those spans with anchors, so the shift is harmless; recorded because
   this is the exact staleness the 4 -> 5 ordering exists to prevent, observed
   in the wild.
2. **Spec section 8.1's synopsis block is stale in the opposite direction** to
   the README: it shows `muxsmith validate <profile>` with no `--json`, while
   the binary has it. Out of this task's Files list, and the spec is ground
   truth on conflict, so it is surfaced rather than touched. A ROADMAP docs-
   accuracy item or a spec amendment, controller's call.
3. **Ledger candidate, already named in the plan's close actions and now
   measured live: the frozen-unit fork.** Both halves reproduced exactly - 219
   under the unit the prose names, 78 under the `verdicts/`-directory unit, and
   78 under BOTH units at `62aaf61`. The general shape is worth an entry beyond
   this sentence: *a count that was correct when written can go wrong by a
   storage-convention change rather than by growth, and re-measuring under the
   old unit reproduces the stale figure and reads as confirmation.* The handle is
   readable: you are re-measuring a number that already exists and your result
   matches it.
4. **Second ledger candidate, from this task's own method.** The README now
   states each figure with its unit inline ("103 ... running up to `D105`", "219
   files under `docs/` with `verdict` in the name"). That is what made both
   numbers re-derivable by a later reader. Candidate statement: *a count in
   durable prose names the unit it counts, or it cannot be re-derived and will be
   re-confirmed under whatever unit the next reader guesses.*

---

## Commit

```
$ git add README.md docs/INSTALL.md
$ git -c commit.gpgsign=false commit -m "docs: README against the shipped CLI surface, the matching-magic list, the two counts with their units; INSTALL.md documents the unsigned-rpm warning

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>"
[master e657263] docs: README against the shipped CLI surface, the matching-magic list, the two counts with their units; INSTALL.md documents the unsigned-rpm warning
 2 files changed, 26 insertions(+), 7 deletions(-)
```

```
$ git show --stat --format='%h %s%n%n%b' HEAD
e657263 docs: README against the shipped CLI surface, the matching-magic list, the two counts with their units; INSTALL.md documents the unsigned-rpm warning

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 README.md       | 19 ++++++++++++++-----
 docs/INSTALL.md | 14 ++++++++++++--
 2 files changed, 26 insertions(+), 7 deletions(-)
```

Deliberately unsigned, verified: `git log -1 --format='%G?'` returns `N`.
Exactly one trailer: `git log -1 --format='%b' | grep -c 'Co-Authored-By'`
returns `1`. `git status --porcelain` after the commit prints nothing.

Not pushed. The single push is a plan-close controller action.

---

# Fix round 1 - appended

Verdict `APPROVED_WITH_MINORS` at
`.superpowers/sdd/plan-10/task-4-verdict.md`, read in full. Three findings
routed: **Finding 1** (MINOR, exit code `130`), **Finding 3** (NIT, folded into
the same sentence) and **Finding 2** (MINOR, `--on-collision` unreachable from
the `run` synopsis). Both repairs are in `README.md`. `docs/INSTALL.md` needed
nothing and was not touched. Findings 4 and 5 are the verdict's own
no-action Nit and Observation and were not acted on.

## Finding 1 - re-measured from scratch, not borrowed

The verdict's claim was treated as borrowed until my own run produced it. Two
instruments, because the first one I reached for turned out to answer a
different question.

**The probe** (`sigint-probe.pl`, written for this round): fork, reset
`$SIG{INT}` to `DEFAULT` in the child before `exec` - a job started with `&`
from a non-interactive shell inherits `SIG_IGN` for SIGINT, which would void the
experiment silently - then signal after a delay and decode the raw wait status
into what a shell would report.

**First attempt failed, recorded rather than hidden.** My first profile was
malformed (`input:` without `pattern`), so the process exited `2` in 2 ms and
the signal always landed after death. Both the run and its control returned
`shell_would_report=2`, which is a null result, not a refutation:

```
$ ./target/debug/muxsmith dry-run p.yaml --source /usr --output /tmp/t4out
[error] ... input: The profile could not be parsed: input: missing field `pattern` at line 2 column 8
exit=2   (0.002 total)
```

With a well-formed profile the same command runs long enough to be interrupted:

```
$ time ./target/debug/muxsmith dry-run p.yaml --source /usr --output /tmp/t4out
0.45s user 0.37s system 99% cpu 0.821 total
exit=0
```

**The measurement, signal at 0.3 s against an 0.82 s runtime:**

```
=== A: dry-run over /usr, SIGINT at 0.3s ===
sigint_sent=1 raw_status=2 exited_with=0 killed_by_signal=2 shell_would_report=130

=== B CONTROL: SAME command, SAME profile, SAME source; SIGINT at 30s, after completion ===
sigint_sent=1 raw_status=0 exited_with=0 killed_by_signal=0 shell_would_report=0

=== C CONTROL: a second subcommand (schema), SIGINT at 30s, after completion ===
sigint_sent=1 raw_status=0 exited_with=0 killed_by_signal=0 shell_would_report=0
```

Control B varies exactly the dimension the claim is about - same binary, same
profile, same source tree, interrupted versus not - so the 130 is caused by the
interrupt and not by the command.

**Then the reader's actual observable, `$?` in a real shell, because the probe
above still decodes the status itself.** The obvious instrument is wrong and is
recorded so nobody reaches for it again:

```
$ timeout -s INT 0.3 ./target/debug/muxsmith dry-run ... ; echo $?
124
```

`timeout` substitutes its own 124 for the child's status, so it cannot answer
this question at all. A shell that `wait`s on the child directly can:

```
=== E: dry-run, SIGINT at 0.3s (runtime 0.82s) - the shell's own $? ===
$? = 130
=== E CONTROL: SAME command, SAME source, SIGINT at 5s i.e. after completion ===
$? = 0
```

**The mechanism claim, measured as the COUNT it is.** "Any command interrupted
ends at 130" rests on no handler existing outside `run`:

```
$ git grep -n 'ctrlc\|set_handler\|signal_hook\|SigAction' -- 'crates/*' 'src-tauri/*' 'xtask/*'
crates/muxsmith-cli/Cargo.toml:14:ctrlc = "3.5.2"
crates/muxsmith-cli/src/commands/run.rs:  (4 lines: 3 comments + the single registration)
crates/muxsmith-core/src/executor/queue.rs:  (2 lines, both rustdoc referring to the CLI's handler)
```

Exactly one `ctrlc::set_handler(...)` call site in the workspace, inside `run`'s
entry point. Fire control: the same pattern against
`crates/muxsmith-cli/src/commands/run.rs` returns 4, so it matches when a match
exists. Every other subcommand therefore keeps SIGINT's default disposition and
dies to the signal, which is `128 + 2` at the shell - measured directly for
`dry-run` above.

**Verdict on the finding: CONFIRMED, independently.** My original scoping
measured *emitting sites* correctly; the reader's observable is `$?`, which has
a second producer the source grep cannot see.

## Finding 3 - re-verified against the code

`crates/muxsmith-cli/src/commands/run.rs`, the handler closure:

```rust
let handler_cancel = Arc::clone(&cancel);
if ctrlc::set_handler(move || {
    if handler_cancel.swap(true, Ordering::SeqCst) {
        std::process::exit(130);
    }
})
```

`swap` returns the PREVIOUS value. First SIGINT: previous is `false`, the branch
is not taken, the flag is now set, and the graceful path runs (queue kills
in-flight jobs, partials deleted, summary printed, `return 130` at the
cancel-flag check). Second SIGINT: previous is `true`, so
`std::process::exit(130)` fires from inside the handler with no cleanup. The
entry point's own rustdoc says the same: "force-exits on a second SIGINT during
cleanup". **CONFIRMED**: the committed sentence's cleanup description was true
of the first Ctrl-C only.

## The sentences as written

**Exit codes** (findings 1 and 3 in one sentence pair, no restructuring):

> Two conventions that hold everywhere: **command-line flags override
> profile-stored values** (`--source`, `--output`, `--on-collision`), and **exit
> codes mirror mkvmerge's own**: `0` clean, `1` finished with warnings, `2`
> errors - your scripts already speak this dialect. Interrupt any of them with
> Ctrl-C and you get `130` instead, the shell's own convention for a signalled
> process, so handle it in your `case $?`. Only `run` earns that code
> gracefully: the first Ctrl-C kills the in-flight jobs, deletes their partial
> output and still prints the summary, and a second one force-exits on the spot,
> part-way through that cleanup.

It carries both halves the adjudication asked for: (a) 130 reaches any
interrupted command, so a `case $?` must handle it, and (b) `run` alone earns it
gracefully. It is now true of both SIGINT paths.

**`--on-collision`** (finding 2, a back-reference clause, not a second
enumeration and not a heading rewrite):

> The same planning, then execution with `N` parallel mux jobs (default 1).
> Every job's full mkvmerge command line and output persist to the run log
> (auto-pruned after 14 days). `--fail-fast` stops dequeuing new jobs after the
> first failure and lets in-flight jobs finish cleanly, and `--on-collision`
> carries the same three policies described under `dry-run` above.

## Sweeps, each fire-controlled

| sweep | result | fire control |
|---|---|---|
| eleven-glyph typography sweep over `README.md` | no output, exit 1 | the same expression against a one-em-dash fixture returns 1 |
| new `file:line` citation in this round's diff | no output, exit 1 | the same expression against `+ see README.md:91 and design `:889-936`` returns 1 |

Untouched regions re-checked: `grep -c 'placeholder(1.0)' README.md` -> `4`;
`grep -c 'Work in progress' README.md` -> `1`; the passthrough recipe's guard
test ran green (below).

## Gate - eleven parts, foreground, all green

```
cargo fmt --all --check                                        exit=0
cargo clippy --workspace --all-targets -- -D warnings          exit=0
cargo test --workspace                                         exit=0   505 ok lines, 0 FAILED
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items   exit=0
cargo deny check                            advisories ok, bans ok, licenses ok, sources ok   exit=0
cargo clippy ... --target x86_64-pc-windows-msvc -- -D warnings exit=0
pnpm lint                                                      exit=0
pnpm build                                  built in 155ms     exit=0
pnpm check:i18n                             check-i18n: ok (...)  exit=0
pnpm test:e2e                               68 passed (2.9s)   exit=0
python3 scripts/ledger-lint.py   ledger-lint: 541 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold   exit=0
```

No `FAIL BUILDING.md:` line. The recipe guard, which is what makes a green gate
evidence for a prose task:

```
test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok
```

`ledger-lint` reports 541 entries where the first round reported 538; the three
new entries are the controller's own harvest commits between the two rounds, not
this task's - no task in this plan edits house YAML.

## Diff scope

```
$ git diff --stat
 README.md | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
```

Two lines, both in `README.md`, exactly the two prescribed repairs.

## Fix-round commit

```
845cf89 docs: 130 reaches any interrupted command, run's is the graceful one; --on-collision back-reference in the run section

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>


 README.md | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
```

Unsigned (`git log -1 --format='%G?'` -> `N`), one trailer, explicit pathspec
(`git add README.md`). `git status --porcelain` after the commit prints nothing.
Not pushed - the single push is a plan-close controller action.
