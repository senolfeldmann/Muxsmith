# Task 4 verdict - Plan 10, W5: the user-facing documentation pass

**Verdict: APPROVED_WITH_MINORS.**

Independent re-derivation, not a read of the report. The binary's help surface
was captured with my own commands and reproduces the report's paste exactly;
both counts, all three boundary checks with their fires, the historical
two-unit measurement at `62aaf61`, the warning string's byte identity against
the ROADMAP, the typography sweep, the corpus size, and all eleven gate parts
reproduce. All four anchor items are supported by spec section AND core symbol.
No claim the code does not support was written. No new `file:line` citation was
introduced.

Two Minors and two Nits below. None blocks the plan; one of the Minors is a
finding the implementer's own method could not have surfaced, because it turns
on an observable the measurement did not address.

---

## Findings

### 1. MINOR - the `130` sentence is scoped to `run` by a measurement that answers a different question than a scripting reader asks

**Where:** `README.md`, the "Two conventions that hold everywhere" paragraph
that closes the "Using the CLI" section - the sentence beginning "One code is
Muxsmith's own".

**What the implementer measured, and it is correct.** `grep -rn '130'
crates/muxsmith-cli/src/` returns six lines; the two *producing* statements
(`std::process::exit(130)` inside the `ctrlc` handler closure, and `return 130`
after the cancel-flag check) both sit in `crates/muxsmith-cli/src/commands/run.rs`.
`ctrlc::set_handler` is registered exactly once in the whole workspace, inside
`run`'s entry point. Reproduced here. So no other subcommand has a code path
that *emits* 130, and writing "every command can exit 130" would have asserted
an explicit exit path that does not exist. Scoping to `run` was the right
instinct against the false-absolute class this task exists to remove.

**What the sentence nonetheless gets wrong for its reader.** A README reader
scripting the CLI asks what `$?` carries, not which source file wrote it.
Because no handler is installed outside `run`, SIGINT keeps its default
disposition everywhere else, the process dies to the signal, and the shell
reports 128+2. Measured on the shipped binary:

```
=== A: dry-run, SIGINT at 0.8s ===
sigint_sent=1 raw_status=2 exited_with=0 killed_by_signal=2 shell_would_report=130
=== B (control, same probe, NO interrupt): SIGINT at 30s, process finishes first ===
sigint_sent=1 raw_status=0 exited_with=0 killed_by_signal=0 shell_would_report=0
=== C (control, second command): schema, SIGINT at 30s ===
sigint_sent=1 raw_status=0 exited_with=0 killed_by_signal=0 shell_would_report=0
```

The control varies the dimension the claim is about - the same command, same
profile, same tree, interrupted versus not - so the 130 in A is caused by the
interrupt and not by the command. (The probe resets `SIGINT` to `DEFAULT` in the
child on purpose: an `&` background job in a non-interactive shell inherits
`SIG_IGN` for SIGINT, which silently voids the experiment. A first attempt did
exactly that and returned exit 0 with the signal delivered.)

So a script wrapping `dry-run` and branching on `$? == 130` **will** see 130,
while the README's sentence attributes 130 to `run` and calls it "Muxsmith's
own". The reader's risk is an omission, not a wrong instruction, which is why
this is Minor rather than Major: nothing the sentence says about `run` is false.

**Exact required change** (one clause, no restructuring, keeps the implementer's
scoping): after "after killing in-flight jobs and deleting their partial
output", state that any command interrupted with Ctrl-C ends at `130` through
the shell's own signal convention, and that what is Muxsmith's own is the
graceful cleanup `run` performs first.

### 2. MINOR - `--on-collision`'s domain is unreachable from the `run` synopsis

**Where:** `README.md`, the `### muxsmith run <profile> ... [--on-collision <policy>]`
subsection.

The enumeration the correction owed lives only in the `dry-run` subsection's
prose. The `run` subsection repeats the bare `<policy>` placeholder and its body
explains `--jobs` and `--fail-fast` without touching `--on-collision` or
pointing anywhere. A reader who lands directly on the `run` section - deep link,
in-page search for "run", or arriving from the GUI section - meets the same bare
placeholder the correction was meant to retire.

This is **not** a plan violation; adjudication 1 below rules the implementer's
reading of "where the flag is introduced" correct. It is the residual reader gap
that reading leaves.

**Exact required change:** one back-reference clause in the `run` subsection's
prose (the `--on-collision` values are as described under `dry-run`), not a
second enumeration and not a rewrite of either synopsis heading.

### 3. NIT - the `130` sentence describes only the first-SIGINT path

`run`'s `ctrlc` handler force-exits `130` immediately on the **second** Ctrl-C,
from inside the closure, without the cleanup the README attributes to it
("after killing in-flight jobs and deleting their partial output"). That
description is true of the first Ctrl-C only. The documented path is the one
users take, so this is optional; naming it so a later pass does not have to
rediscover it.

### 4. NIT - "Two conventions that hold everywhere" over-reaches on its flag half

`--source`, `--output` and `--on-collision` exist on `dry-run` and `run` only;
`validate`, `identify` and `schema` carry none of them. The exit-code half of
the same sentence does hold for every command.

Deliberately rated a Nit and not pressed: "hold everywhere" is a
section-closing idiom for "these are global rules", not a quantifier over
commands, which is what made the two corrected claims ("**every command** takes
`--json`", "**Every one of them** takes `--json` ... and `--locale`") flatly
false rather than loose. Pressing this would be over-reach into a fact set the
plan closed. Recorded because the next README pass should meet it as a known
judgement call rather than as a fresh discovery.

### 5. OBSERVATION, no action - the plan-8 design spec carries a stale copy of the file-top comment

`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` contains
the drafted `INSTALL.md`, including the two-member "SmartScreen and Gatekeeper"
form of the file-top comment this task extended. It is a historical design
record that had already diverged from the shipped file before this task (its
opening paragraph differs too: "All 1.0-era builds are **unsigned**" versus the
shipped "No 1.0-era build carries a developer identity"). Task 4's Files list is
exhaustive and the spec is ground truth on conflict, so leaving it is correct -
editing it would falsify the record of what Plan 8 designed.

---

## Dimension-by-dimension results

### 1. Every corrected claim, re-derived from the binary

My own capture (`./target/debug/muxsmith --version`, `--help`, and both `-h` and
`--help` for all five subcommands) reproduces the report's pasted output
**exactly**, including the long-help possible-value block and the short-help
`[possible values: error, skip, overwrite]` inline form. My divergence table,
built independently:

| flag / claim | binary | README at `e657263` | verdict |
|---|---|---|---|
| `--json` blanket | absent on `schema`; present on `validate`, `dry-run`, `identify`, `run` | both sites now name the four by name | corrected, plan premise reproduced |
| `--locale` blanket | same four; `schema --help` lists `-h, --help` only | same | corrected, plan premise reproduced |
| `--locale` default | `Locale for rendered messages (default: system, fallback en)` | "the default is your system locale, falling back to English" | corrected, accurate |
| `--on-collision` domain | `error` (default policy), `skip`, `overwrite`; falls back to profile `output.on_collision` when unset | enumerated in the `dry-run` subsection, with the fallback | corrected; see Finding 2 for placement |
| exit code `130` | `cli.rs`'s `Cli` doc comment states the shared contract; `run.rs` is the only file with a producing statement | added, scoped to `run` | corrected; see Finding 1 |
| `--jobs` | `[default: 1]`, "Parallel mux jobs (default 1 = sequential)" | "`N` parallel mux jobs (default 1)" | agrees |
| `--fail-fast` | "Stop dequeuing after the first failed job (in-flight finish)" | "stops dequeuing new jobs after the first failure and lets in-flight jobs finish cleanly" | agrees |
| `--source` / `--output` | "overrides the profile default" | "command-line flags override profile-stored values" | agrees |
| `schema`'s behaviour | `Cmd::Schema` is a unit variant; `main`'s arm `println!`s the pretty schema and returns `0`, touching no `Renderer` | "needs no flag - it prints the profile's JSON Schema and nothing else"; "writes the schema to stdout and has no rendered messages to translate" | agrees, verified at the source |

The report's aggregate "12 rows examined, 5 divergent" is internally consistent
(D1-D5 divergent, D6/D10/D11 not-divergent omissions, D7/D8/D9/D12 agree; 5+7=12)
and matches my own walk.

### 2. Did the pass MISS a divergence?

Walked every subcommand and every flag the binary lists against the README.
**No substantive miss.** Every flag the binary carries is documented except
`-h/--help` and `-V/--version`, both pure omissions that make no false claim;
positional argument names, subcommand set (five real, plus clap's built-in
`help`), value metavariables (`<policy>`, `DIR`, `N` versus the binary's
`<ON_COLLISION>`, `<SOURCE>`, `<JOBS>`) and all three stated defaults check out.
The only structural gap is Finding 2, which is placement rather than a wrong
claim, plus the Nit at Finding 4.

### 3. The exact-typed-matching paragraph and the four anchor items

Each verified against BOTH the spec section and the core symbol, independently.

| item | spec | code | verdict |
|---|---|---|---|
| `language` normalizes, reads both fields | 4.4 `language` bullet; 4.3 canonical form | `exact_matches`'s `"language"` arm iterates `["language", "language_ietf"]`; `lang_eq` tries `LanguageIndex::normalize` first, then `canonical_tag` (BCP-47 canonicalize: case, script suppression, deprecated-subtag replacement) | SUPPORTED |
| absent booleans compare `false` under `exact` | 4.4 "Boolean flags, absent = false", which names **exactly the four** the README names | `exact_matches`'s fallback arm: `None => matchable_type(prop) == Some(Boolean) ? scalar_eq(want, Bool(false)) : false` | SUPPORTED |
| curated closed domains for `type` and `codec_kind` | 4.4 "Closed-domain values" | `matchable_domain` returns `TYPE_VALUES` / `CODEC_KIND_NAMES`; `validate_expr`'s exact arm pushes `InvalidPropertyValue` on a non-member | SUPPORTED |
| `raw:`'s contrast | 4.4 `raw:` opt-in bullet; 9.2 | `exact_matches`'s `raw:` early return (single `item.get(bare)`, `scalar_eq`, `None => false`); `validate_expr`'s two `raw:` branches skip type, domain and existence checks | SUPPORTED |

**The enumerated set inside the boolean item was checked as a set, not by a
single fire.** All four names the README lists - `flag_commentary`,
`flag_original`, `flag_hearing_impaired`, `flag_visual_impaired` - are present in
`capability::generated::MATCHABLE_PROPERTIES` as `PropType::Boolean`, and spec
4.4 names the same four. (The false-when-absent *mechanism* covers every
Boolean-typed matchable property, of which there are nine; the README's
parenthetical is scoped to which flags mkvmerge omits, which is how the spec
words it too. Not a defect.)

The corrected paragraph itself: "each **known** property" matches the set
`matchable_type` answers for and excludes the bare `raw:` name; the
`NotStringProperty` claim and the `codec_kind` carve-out both match
`validate_expr`, whose `codec_kind` guard fires deliberately *before* the
string-type check; "pattern-match `codec_id` instead" is sound - `codec_id` is
`PropType::String`, so `substring`/`regex` on it pass the check.

Item 3's README example (`type: subtitle` against the domain's `subtitles`) is
correct: `TYPE_VALUES` is `["audio", "buttons", "subtitles", "video"]`.

### 4. The two counts, re-measured

**Decision series**, my own run:

```
headings total: 104
distinct:       103
max:            105
min:            1
duplicates:     D32
missing in 1..105: [73, 74]
```

Fire control for the heading pattern: the `D32` expression returns both the
decision heading and its addendum heading, so the pattern demonstrably matches.
**Enumerated-set control on the instrument itself**, which a fire against a
known-present member cannot supply: the pattern hard-codes heading depths
`#{1,4}`, so I ran `#{5,6}` over the same corpus (returns 0) and a no-hash
`^\**D[0-9]+:` form (returns 0). Nothing escapes the depth set.

**Unit control, which the plan did not require and which strengthens the
figure:** the same expression over the *whole* `docs/` tree, not just
`docs/superpowers/specs/`, returns the same **103 distinct numbers**. The count
is therefore not an artifact of the prescribed pathspec - every D-number cited
anywhere under `docs/` is defined in the specs directory.

**Which unit each figure names, and re-derivability.** The sentence states both,
separated: the count ("103 of them so far") and the reach ("running up to
`D105`"), with the two-number gap explained ("because two numbers were reserved
for a plan that never spent them"). Verified independently: D73 and D74 are
exactly the two absentees, Plan 7.5 was allocated D65-D74 and its design spends
D65 through D72. A later reader can re-derive all three claims. The one thing
the sentence does not warn about is the `D32` addendum heading, so a reader who
counts *headings* rather than *numbers* gets 104; since the sentence's unit is
decisions and decisions are identified by number, deduping is the natural read.
Acceptable.

**Verdict count**, my own run:

```
basename unit (what the prose names): 219
frozen verdicts/-directory unit:       78
at 62aaf61, basename unit:             78
at 62aaf61, verdicts/ unit:            78
```

**The trap did not catch this pass.** The frozen unit reproduces the README's
old `78` and reads as confirmation; the report pasted both, and both reproduce
here. `78` was correct when written and went wrong by a storage-convention fork,
not by growth.

Three boundary checks, each with its own fired control:

| check | result | fired control | control result |
|---|---|---|---|
| every match is markdown | 0 | same `-v '\.md$'` over the whole `docs/` list | 221 |
| no review BRIEF is caught | 0 | same `grep -ic 'brief'` over the whole list | 185 |
| no file under `verdicts/` missed by the basename rule | 0 | same inverted basename filter over the whole list | 889 |

**Two extra checks on the unit's wording, beyond the plan's three.** The
sentence says "with `verdict` in the name", which admits two readings -
basename or full path. **Both return 219**, and the set difference is empty. And
the appositive ("the preserved review verdicts - 219 files ...") holds: every
basename shape in the set is a review verdict (`task-N-verdict.md` 87,
`task-N-review-verdict.md` 59, round/delta/whole-branch/amendment/fix variants
for the rest). No template, no tracker, no harvest file rides in. The figure is
re-derivable under any plausible reading of its own wording, which is what makes
it robust rather than merely explicit.

### 5. `docs/INSTALL.md`, both named regions

**The string, grepped not eyeballed.** Fixed-string grep locates it on one
unbroken line in each file (`docs/INSTALL.md:88`, `docs/ROADMAP.md:904`,
measured at `e657263`), and `od -c` dumps of the two extracted occurrences are
byte-identical. **Four independent near-miss fire controls** (`packages` for
`package`, `check` for `checks`, `one` for `1`, `GPG` for `OpenPGP`) all return
0, and a known-present substring returns 1, so the grep discriminates rather
than matching anything.

**Attribution:** the note points at "the `dnf install` above" and names
`@commandline` as dnf's own, never the `rpm` binary. Correct per the ROADMAP
entry, which itself carries the correction ("**The tool is `dnf`, not the `rpm`
binary**").

**Scope:** the note is labelled "(Fedora)" and speaks only to the `dnf` case.
No claim about the deb path. Held.

**Register:** it follows the SmartScreen and Gatekeeper blocks' shape - bold
label, what the user meets, what it means, that it is deliberate policy, what to
do instead. The one divergence is that the two existing labels name the OS
mechanism while this one names the cause, which is unavoidable since Fedora's
warning has no branded mechanism to name.

**The neighbouring sentence stays true.** "No gatekeeping dialog exists on
Linux." now follows a note that ends by asserting the same thing ("it is a
warning, not a gatekeeping dialog: nothing blocks, nothing needs clicking, the
install completes"). Confirmed rather than contradicted, as the plan ordered.
Placement is right: the note sits between the artifact list and that sentence,
which is where a reader who just met the warning arrives.

**The file-top comment**, located by `code signing lands` on its own line as
prescribed, joined and compared:

```
before (f2e9d75): ... the SmartScreen and Gatekeeper sections below shrink ...
after  (e657263): ... the SmartScreen, Gatekeeper and Linux unsigned-package sections below shrink ...
```

The "before" form is byte-identical to the string Amendment 2 records. The
enumeration is now three members, so the 1.x signing work will not strand the
new note. Repo-wide sweep for other enumerations of what shrinks when signing
lands returns only this comment, the plan's two references to it, and the
plan-8 design spec's historical copy (Finding 5).

Also checked and clean: the ROADMAP's own `docs/INSTALL.md:82` citation still
points at the dnf artifact line, because the insertion landed below it.

### 6. What must NOT have changed

- Four `placeholder(1.0)` comments present; fired control (`placeholder(2.0)` -> 0, `placeholder(` -> 4).
- Work-in-progress banner present.
- **The passthrough recipe is byte-identical to its pre-task state**, extracted by content rather than by line number so a shifted span cannot fake the comparison: 177 bytes at `f2e9d75`, 177 bytes at `e657263`, equal. Fired control: the same comparison against a one-word mutation returns unequal.
- **`crates/muxsmith-cli/tests/run_live.rs`'s inlined copy still matches it**, decoded from the Rust string literal and compared byte-for-byte: equal, 177 bytes both sides, with the mutated-copy control returning False.
- The guard test ran green in my own gate run: `test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok`.

### 7. No new `file:line` citation, and the corpus re-derived

Both of Task 5's Step-1 expressions, run against the tree at `e657263`:

- **Expression A: 20 lines across 13 files.**
- **Expression B: 4 lines across 4 files.**
- **Union: 24 matched lines across 16 files** (`suggestions.rs` shared).

Identical to the plan's authoring measurement. Both expressions fired by their
own non-empty result, so neither empty half can be a malformed pattern.

Task 4 introduced no member: the union expression applied to Task 4's own diff
over both edited files returns nothing, **fired** against a synthetic
`+ see README.md:91 and design \`:889-936\``, which matches.

### 8. House dimension

- `owner-manual-qa-gates-the-1-0-release` (`docs/process-conventions.yaml`): honoured. The Fedora finding is DOCUMENTED, not fixed; signing stays a ruled 1.x item per the ROADMAP's "Artifact signing: firm 1.x" entry, which I read and which confirms "Pre-1.0 the warnings are documented rather than removed".
- `proc-wrapped-prose-quote-grep`: honoured in both directions - the file-top comment was located by its unwrapped-line anchor, and the warning string was written on **one unbroken line** so the reader who greps it hits.
- `latitude-carveout-zero-content-structural-forks`: the two `INSTALL.md` repairs are the plan's named regions, nothing else in the file moved.
- **Register:** the sell-tone basis is the ROADMAP README entry, verified verbatim ("sell-tone per Şenol's register override - a case-scoped exception to the writeup-stimme rule"). `latitude-carveout-presentation-tokens` is correctly not invoked.
- **Typography:** the eleven-glyph sweep over both edited files returns nothing, **with each glyph class fired alone on its own line** (eleven separate controls, each returning 1) rather than as one bundled control, which would have proved only that at least one member fires.

### 9. The no-work-needed check

Every premise behind a "no correction needed" conclusion was re-run, not read.
All reproduce: the five subcommands versus clap's built-in `help`; `validate`'s
YAML-or-JSON acceptance as an omission that makes no false claim; the
undocumented top-level `-V/--version`; the four "AGREES" rows; `schema`'s
zero-flag, stdout-only, always-`0` behaviour (verified at `main`'s `Cmd::Schema`
arm, a unit variant that never constructs a `Renderer`); and the report's claim
that no plan-listed correction failed re-measurement - all four reproduce.

The one premise that does **not** survive scrutiny is the 130 scoping premise,
and it fails not because the measurement is wrong but because it measures
emitting sites where the reader-facing question is observed exit status. That is
Finding 1.

### 10. Verification quality

**The full gate as `BUILDING.md` enumerates it - eleven parts, foreground, all
green,** run by me from the file's own three marked blocks:

| block | parts | result |
|---|---|---|
| Rust | `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `RUSTDOCFLAGS="-D warnings" cargo doc ...`, `cargo deny check`, cross-target clippy for `x86_64-pc-windows-msvc` | 6/6 exit 0 |
| frontend | `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e` | 4/4 exit 0 |
| house-knowledge | `python3 scripts/ledger-lint.py` | exit 0 |

No `FAIL BUILDING.md:` line, so Task 1's check saw a `BUILDING.md` this task did
not touch.

**Every stated aggregate recomputed and reproduced:** 505 `... ok` lines from
`cargo test --workspace`; 68 e2e tests passed; `ledger-lint: 538 entries across
4 files plus BUILDING.md's gate enumeration`; 104 headings / 103 numbers / D105;
219 versus the frozen 78; 12 divergence rows of which 5 divergent.

---

## The three adjudications

### Adjudication 1: where `--on-collision`'s domain landed

**Verdict: the implementer's reading is the right one, and the committed
placement satisfies the plan. The reader gap it leaves is real but separate, and
is Finding 2.**

Phrased in the other direction first: if the plan had meant both synopsis
headings, it would not have written "enumerated **where the flag is
introduced**" - a first-occurrence locution, singular - and it would not have
contrasted that with "instead of being left as a bare `<policy>` placeholder in
two synopses". The contrast is between "enumerated once, somewhere a reader
meets it" and "nowhere in the README except two bare placeholders". The reading
that sentence *excludes* is precisely the one that would substitute
`error|skip|overwrite` into both headings. The defect the plan names - the value
set existing nowhere in the README - is closed.

But the brief's second half deserves a straight answer, and it is yes with a
qualifier: **the reader who lands on `### muxsmith run` does still lack what the
correction owed them.** The enumeration is in a different subsection, the `run`
body never mentions `--on-collision`, and nothing points. The repair is a
back-reference clause, not a second enumeration - which is why this is Finding 2
at Minor and not a rejection of the implementer's judgement call.

### Adjudication 2: exit code 130's attribution

**Verdict: the committed form is true of the shipped binary; `cli.rs`'s
command-neutral wording is ALSO true of it as observed, by a different
mechanism; the committed form is the better README sentence but is incomplete,
and yes - it could mislead someone scripting `dry-run`.**

Both directions, on measurement rather than reading:

- **For the committed form.** `run` is the only subcommand that installs a
  `ctrlc` handler and the only one with a statement that emits 130. A
  command-neutral "`130` cancelled" would imply every command has that path.
  It does not, and inventing one would be a fresh false absolute in the very
  paragraph this task was fixing absolutes in. The implementer was right to
  refuse it.
- **Against the committed form.** `cli.rs`'s doc comment ("every command shares
  the exit-code contract 0 clean / 1 warnings / 2 errors / 130 cancelled") turns
  out to be observationally accurate, not merely aspirational. Measured above:
  `dry-run` interrupted by SIGINT is killed by signal 2 and the shell reports
  **130**, with a control on the same command showing 0 when the signal arrives
  after completion. Every subcommand reaches 130 under interruption; only `run`
  reaches it deliberately.

**Which serves a README reader better:** the committed scoping, extended by one
clause. What a reader needs is (a) that 130 can come back from anything they
interrupt, so their `case $?` must handle it, and (b) that `run` alone earns it
gracefully - cleanup, partials deleted, summary printed - rather than dying
where it stands. The committed sentence carries (b) and drops (a). `cli.rs`'s
form carries (a) and drops (b). Neither alone is the right README sentence.

**Could it mislead someone scripting `dry-run`? Yes.** "One code is Muxsmith's
own: a `run` you interrupt with Ctrl-C exits `130`" invites the inference that
130 is unreachable from the command they are wrapping, and that inference is
false at the shell.

### Adjudication 3: the anchor list's lead-in

**Verdict: the committed framing is more accurate than the acceptance row's
phrasing, and it does not under-deliver the row.**

W5-c's machine-checkable content is that the four items are explicitly listed.
That is delivered in full: four items, each verified against a named spec
section and a named core symbol before it was written, each supported. Nothing
is missing.

What differs is the row's **noun**, and the ground truth settles that against
the row rather than against the implementer. The anchor originates in the
ROADMAP's Content anchors block, which the plan names as its source, and which
records the owner's own remark as three magic properties followed by "**contrast
with** `raw:`'s no-magic byte-exact single-field rule". The plan's Step 3
reproduces that structure verbatim ("(4) `raw:`'s **contrast**"). And the code
agrees: `exact_matches` returns from its `raw:` branch *before* reaching any
convenience arm, so `raw:` is definitionally the absence of magic.

Had W5-c's phrasing been treated as binding on the prose, the task would have
been ordered to write something its own ground truth refutes. The lead-in
"Three places where `exact` does more than compare, and one where it deliberately
does less" is the faithful rendering; W5-c's "four magic properties" is
shorthand for the deliverable, not a specification of its wording.

---

## Tree state - byte-identity, proven

**A concurrent commit landed on `master` while this review ran.** HEAD moved
from `e657263` to `2f1dca0` ("roadmap: the v1 spec's 8.1 synopsis underclaims
validate's flags, surfaced by task 4"), touching `docs/ROADMAP.md` only. That is
the controller's own close action, not a change to anything under review, but it
means `git status` alone can no longer carry the proof. Blob-level instead:

```
README.md          working=1c9c7c8513af6544af4340dfe3618a29b9a80268  e657263=1c9c7c8513af6544af4340dfe3618a29b9a80268
docs/INSTALL.md    working=c61011227dfd28824165173a26ec8a88ba8f25ad  e657263=c61011227dfd28824165173a26ec8a88ba8f25ad
```

and against the sha256 baseline I took before running anything:

```
61ca74672f3162c775717bdb44a082437e1546770eaa2c6829051c5e0476707b  ./README.md
d905c34c27524c47bae3c2b41374dbcd7651247295f0a8698dcd1683a77180b5  ./docs/INSTALL.md
```

`git diff --name-status e657263` lists exactly one path, `docs/ROADMAP.md`, the
concurrent commit's. `git status --porcelain` is empty. **I mutated nothing**:
every instrument wrote to the scratchpad, and the only repo-adjacent side
effects are gitignored build outputs the gate itself regenerates (`target/`,
`dist/`).

---

## Evidence appendix

**Instrument directory** (all independent; no instrument the implementer wrote
was re-run, and no shared default path was used):
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t4rev-independent/`

| file | what it holds |
|---|---|
| `help-capture.txt` | my own full help surface: `--version`, `--help`, and `-h` + `--help` for all five subcommands |
| `sigint-probe.pl` | the SIGINT probe (forks, resets `SIGINT` to `DEFAULT`, execs, signals after a delay, decodes the wait status) |
| `p.yaml` | the throwaway passthrough profile the probe drives |
| `dnums.txt`, `h2.txt` | the D-number extraction and its deduped numeric form |
| `exprA.txt`, `exprB.txt`, `union_files.txt` | Task 5's two corpus expressions and their union |
| `glyphs.txt` | the eleven-glyph typography fire-control fixture, one class per line |
| `recipe_before.txt`, `recipe_now.txt` | passthrough-recipe extractions for the byte-identity check |
| `baseline-tree.sha256`, `final-tree.sha256` | 26659-file hash sweeps before and after |
| `tests.out`, `gate.out`, `g.out` | gate run outputs |

**Commands that carry a load-bearing claim**, each run in the foreground in zsh
with exit codes taken from `$?` directly (`${PIPESTATUS[0]}` is bash-only):

```bash
# CLI surface
./target/debug/muxsmith --help; ./target/debug/muxsmith <sub> -h; ./target/debug/muxsmith <sub> --help

# 130: emitting sites, then observable status
grep -rn '130' crates/muxsmith-cli/src/
git grep -n 'ctrlc' -- 'crates/*' 'src-tauri/*'
perl sigint-probe.pl 0.8 ./target/debug/muxsmith dry-run p.yaml --source /usr --output /tmp/out   # A
perl sigint-probe.pl 30  ./target/debug/muxsmith dry-run p.yaml --source /usr --output /tmp/out   # B, control
perl sigint-probe.pl 30  ./target/debug/muxsmith schema                                           # C, control

# counts
git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/superpowers/specs/*' | grep -oE 'D[0-9]+'
git grep -hoE '^#{5,6} +\**D[0-9]+' -- 'docs/superpowers/specs/*'        # instrument-set control
git grep -hoE '^#{1,4} +\**D[0-9]+' -- 'docs/'                           # unit control, also 103
git ls-files 'docs/*' | grep -icE '/[^/]*verdict[^/]*$'                  # 219
git ls-files 'docs/*' | grep -cE '/verdicts/'                            # 78, frozen unit
git ls-files 'docs/*' | grep -ic 'verdict'                               # 219, the other reading
git ls-tree -r --name-only 62aaf61 -- docs | grep -icE '/[^/]*verdict[^/]*$'   # 78
git ls-tree -r --name-only 62aaf61 -- docs | grep -cE '/verdicts/'             # 78

# INSTALL.md
grep -Fn '<the warning string>' docs/INSTALL.md docs/ROADMAP.md
grep -Foh '<the warning string>' <each file> | od -c        # byte identity
grep -Fc '<four near-miss variants>' docs/INSTALL.md        # fire controls, all 0

# untouched regions
git show f2e9d75:README.md   # recipe extracted by regex, compared to now and to the run_live.rs literal

# corpus + typography
git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py' | xargs grep -nE '<expression A>'
git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py' | while read -r f; do grep -nE '<expression B>' "$f" | sed "s|^|$f:|"; done
grep -nP '[\x{2013}\x{2014}\x{2012}\x{2015}\x{2212}\x{201C}\x{201D}\x{2018}\x{2019}\x{2026}\x{00A0}]' README.md docs/INSTALL.md
```

---

## HARVEST

### What Task 5 must carry

**1. The README-quoting comment is now doubly stale, and its quotation must be
DROPPED, not re-anchored.** In `crates/muxsmith-cli/tests/run_live.rs`, the
comment above the `dry-run --json` assertion inside
`readme_passthrough_recipe_with_title_template_survives_dry_run_and_run` reads
`per README.md:91 "every command takes --json"`. Measured at `e657263`: README
line 91 is now the "A real dry-run." bullet, the bullet it meant moved to line
98, **and its text changed**, so the quoted phrase exists nowhere in the file -
`grep -c 'every command takes' README.md` returns 0, fired against
`grep -c 'each take \`--json\`'` which returns 1. Task 5 must name the README
anchor - the **"What you get"** section's **"Scriptable everything"** bullet -
and must not preserve the old wording, which is now a false quotation.

**2. The two `README.md:71-78` citations, with the unit corrected.** Both sit in
the same test (the rustdoc above it and the inline comment before `fs::write`).
`71-78` was the **fence** span (the ```yaml opener at 71, the closer at 78). At
`e657263` the fence is **78-85** and the recipe *content* is 79-84. The task-4
report states the move as "71-78 to 79-84", which compares a fence span against
a content span - immaterial to Task 5, which deletes the span outright, but
material to anything downstream that quotes the report's numbers. The anchor to
name is the **"Pure passthrough: a profile with zero rules"** heading. The
recipe's bytes are unchanged (177 bytes, identical at `f2e9d75` and `e657263`,
and identical to the literal `run_live.rs` inlines), so the rewrite may state
that the literal still matches.

**3. The corpus is unchanged by Task 4: 24 lines across 16 files** (A: 20/13,
B: 4/4, `suggestions.rs` shared), re-derived here with both prescribed
expressions, each fired by its own non-empty result. Task 4 added no member.

### Patterns worth the ledger

**4. A reviewer's byte-identity duty must be stated against blobs, not against
`git status`.** A controller commit (`2f1dca0`) landed on `master` mid-review,
moving HEAD off the `e657263` this review brief names as "the tree". Nothing
under review changed, but the prescribed proof ("leave the tree byte-identical
to `e657263` - prove it") became unprovable by a clean `git status`, which is
what a reviewer reaches for. The durable form: prove identity by
`git hash-object <file>` against `git rev-parse <commit>:<file>` for the files
under review, plus `git diff --name-status <commit>` to name every path that
did move and who moved it. Handle: you are about to cite a clean working tree as
evidence in a project where another writer may commit concurrently.

**5. A measurement's SCOPE decides which claim it can carry - and "which code
emits X" is not "which invocations produce X".** Finding 1 is a clean instance:
the implementer measured emitting sites correctly, reproduced here, and drew a
reader-facing conclusion the measurement does not reach, because the observable
the reader cares about (`$?`) has a second producer the source grep cannot see
(default signal disposition). This is the same family as the recorded
"a borrowed measurement answers its own question, not yours" occurrence, one
level sharper: the measurement was the implementer's own and correctly executed.
Handle: your evidence line names a *source construct* and your conclusion names
a *runtime observable*.

**6. A stated unit is well-formed when its plausible re-readings converge.** The
README's verdict sentence says "files under `docs/` with `verdict` in the name",
and both readings of "in the name" - basename and full path - return 219, with
an empty set difference. The decision sentence has the same property: the count
is 103 whether measured over the specs directory the plan prescribed or over all
of `docs/`. That is a stronger property than "the unit is stated", and it is the
one worth writing into the candidate entry the report proposed: **a count in
durable prose names a unit whose plausible re-readings agree, or the next reader
re-confirms it under whichever one they guessed.** Both of the report's two
ledger candidates (the frozen-unit fork; naming the unit inline) are confirmed
live by independent measurement here and should go to the close.

**7. An occurrence of 6, produced by this verdict itself.** The tree-identity
section first quoted the two sha256 sums in truncated form and one tail was
invented (`...80f5` for a hash ending `...80b5`) - the measurement was correct
when taken and went wrong when restated in shortened form. Caught by diffing the
quoted lines back against a live `sha256sum`, then replaced with the full
digests so there is no truncation to get wrong. Recorded rather than quietly
fixed, because it is the third artifact in this package to show that
abbreviating a measurement while restating it is where precision gets invented -
and because a verdict that harvests the pattern while committing it is the
sharpest available evidence that awareness does not prevent it.

### Repeated rejections - none

Nothing in this task's committed output re-opens a settled decision, and the one
place where an acceptance row's wording conflicted with the ground truth
(adjudication 3) was surfaced as a numbered concern rather than absorbed
silently. The report's four named judgement calls are each defensible and each
was routed to the reviewer, which is the behaviour the plan's no-latitude rule
is meant to produce.

---
---

# DELTA VERDICT - Task 4, fix round 1 (`d72d73e..845cf89`)

**Scope:** the fix diff only - one commit, `845cf89`, `README.md`, +2/-2. Settled
non-findings are not re-litigated. Findings 4 and 5 were not routed and are not
revisited.

**Delta verdict: all three routed findings ADDRESSED. One new NIT introduced by
the fix diff itself, which does not warrant another round.**

**Tree state, per file against blobs rather than against a clean status** (my own
HARVEST 4 discipline, and the coordinator named the two intervening commits so
this could be done rather than discovered):

```
README.md                  IDENTICAL 9bbe877c8b778c5475a9b7a1371ea3bd36658609
docs/INSTALL.md            IDENTICAL c61011227dfd28824165173a26ec8a88ba8f25ad
docs/ROADMAP.md            IDENTICAL df142fde3ea1a751460fa133f81f3f79e92d8fda
docs/decision-ledger.yaml  IDENTICAL 81a3cd965279805b207d5af77b05aedc58a26964
```

`git status --porcelain` empty; `git diff --name-status 845cf89` empty;
`README.md`'s sha256 identical to the baseline I took before touching anything
this round. `2f1dca0` and `d72d73e` touched only `docs/ROADMAP.md` and
`docs/decision-ledger.yaml` respectively and are correctly outside this review.

---

## Finding 1 (MINOR, exit code 130) - **ADDRESSED**

The committed sentence pair now carries both halves the adjudication demanded:
130 reaches any interrupted command and belongs in a `case $?`, and `run` alone
earns it gracefully.

**Every load-bearing claim in the fix report reproduced with my own instruments,
not read.**

**(a) `timeout -s INT` is the wrong instrument - reproduced, and I added the
control the report did not run:**

```
timeout -s INT 0.3 ./target/debug/muxsmith dry-run ...   -> $? = 124
timeout -s INT 30  ./target/debug/muxsmith schema        -> $? = 0
```

The control matters: without it, `124` could be read as the child's own status.
It is not - `timeout` substitutes 124 only when it fires, so the tool cannot
answer this question, exactly as the report says.

**(b) The reader's real observable, a shell that waits on the child directly.**
My probe uses `set -m` to turn job control on so the child does not inherit
`SIG_IGN` - a different mechanism from the report's `fork` + `$SIG{INT}='DEFAULT'`,
deliberately, so this is an independent reproduction rather than a re-run:

```
E:         dry-run over /usr, SIGINT at 0.3s (runtime ~0.82s)   -> $? = 130
E CONTROL: SAME command, SAME source, SIGINT at 20s             -> $? = 0
```

**(c) A SECOND subcommand measured directly, which the fix report did not do.**
`validate` on a generated 3000-rule profile runs 0.70 s, long enough to interrupt:

```
V:         validate, SIGINT at 0.3s   -> $? = 130
V CONTROL: SAME command, SIGINT at 60s -> $? = 0
```

So the sentence's "any of them" is now verified on two of the five subcommands at
the shell, not extrapolated from one.

**(d) An instrument of a different kind for the mechanism: the kernel's own view.**
`/proc/<pid>/status` of a live `dry-run`:

```
SigCgt=0000000000000440   ->  caught signals [7, 11] = SIGBUS, SIGSEGV
SIGINT(2) CAUGHT: False
```

Fired control on the decode itself: the mask is non-empty and resolves to two
real signals, so a `False` for SIGINT is a measurement rather than an all-zero
read. This is direct evidence that no handler exists on a non-`run` path -
independent of any source grep.

**(e) The mechanism as the COUNT it is.** `ctrlc::set_handler` has exactly **one**
call site in the workspace, in `crates/muxsmith-cli/src/commands/run.rs`. Fire
control: the same pattern against that file returns 1, so it matches when a match
exists. **Enumerated-set control the report did not run:** the instrument's
alternation is itself a claim, so I swept the neighbouring registration
mechanisms too - `signal_hook`, `sigaction`, `SigAction`, `libc::signal` - across
`crates/`, `src-tauri/` and `xtask/`: zero files. `ctrlc` is the only route, so
the single call site really is the whole population.

**Does the rewritten sentence stay true of every command the README documents?**
Yes. `validate`, `dry-run`, `identify` and `run` all run with SIGINT's default
disposition until `run` installs its handler, so an interrupt lands at 128+2.
`schema` is the one case that is true but practically unobservable - it completes
in about a millisecond, so it cannot be interrupted in practice; the sentence is
a conditional ("interrupt X and you get 130") and a conditional whose antecedent
is hard to satisfy is not falsified by that. **Nothing the new sentence says
about `schema` or `identify` is contradicted by the binary**, and it makes no
positive claim about either beyond the shared conditional.

## Finding 3 (NIT, only the first-SIGINT path) - **ADDRESSED**

The new clause - "the first Ctrl-C kills the in-flight jobs, deletes their
partial output and still prints the summary, and a second one force-exits on the
spot, part-way through that cleanup" - is correct at **three** code sites I read
rather than took:

- `handler_cancel.swap(true, Ordering::SeqCst)` returns the PREVIOUS value, so the
  `std::process::exit(130)` branch is taken only on the second SIGINT. The report's
  claim is right.
- The graceful path's ordering is real, not assumed: `render_summary`'s `println!`
  sits **before** the cancel-flag check that returns 130, so the summary genuinely
  still prints. (This is the half the report asserted from the rustdoc; I checked
  the control flow.)
- Partial deletion is D17 in `crates/muxsmith-core/src/executor/job.rs`, whose
  `JobState` docs record "Killed while the caller's cancellation flag was set;
  partial output ... deleted".

The entry point's own rustdoc states the same contract, so the README is now a
faithful rendering of it rather than a partial one.

## Finding 2 (MINOR, `--on-collision` back-reference) - **ADDRESSED**

Committed clause: "`--fail-fast` stops dequeuing new jobs after the first failure
and lets in-flight jobs finish cleanly, and `--on-collision` carries the same
three policies described under `dry-run` above."

This is a back-reference, not a second enumeration and not a heading rewrite -
exactly the shape prescribed.

**Does it survive a reader who lands in the `run` subsection cold?** Yes. Cold,
they learn three things: the flag has a value domain, it has exactly **three**
members, and the description is under `dry-run` above. A count plus a location is
a usable pointer - the reader knows when they have found all of it, which a bare
"see above" would not give them.

**The new count is itself a claim, so I measured it:** `muxsmith run -h` prints
`[possible values: error, skip, overwrite]` and the long help enumerates exactly
**3** possible-value lines. "Three" is correct.

"above" is accurate: the `dry-run` subsection precedes `run` in document order.

---

## New breakage introduced by the fix diff

### NIT (new) - "Interrupt any of them" has no antecedent in its own paragraph

The replaced sentence had an explicit subject ("a `run` you interrupt with
Ctrl-C"). The replacement uses a pronoun, and the referent it needs - the
subcommands - is not in the paragraph. Measured over the text preceding the
pronoun:

```
subcommand-nouns before the pronoun: ['command']   <- inside "command-line", an adjective
plural NPs available to bind 'them': ['conventions', 'flags', 'values', 'codes', 'errors', 'scripts']
```

None of the six bindable plurals is a command; the nearest one that can even be
interrupted is "your scripts", which is a coherent but wrong reading. The
intended referent is supplied only by the section opener ("Five subcommands, one
shape"), several paragraphs up - fine for a linear reader, less so for the
scripter who searches for "exit codes" and reads this paragraph alone, which is
the exact reader Finding 1 was about.

(Recorded honestly: my first instrument for this was `grep -c` over a one-line
paragraph, which counts lines and returned a meaningless `1`. Re-run per
occurrence, it showed the only "command" token is inside "command-line".)

**Repair, if taken: two words** - "Interrupt any subcommand with Ctrl-C" or
"Interrupt any of them" -> "Interrupt any command". **This does not warrant
another round.** The finding it belongs to is ADDRESSED in substance, the claim
is true, and no reader is misinformed - only briefly under-pointed. Fold it into
the next README touch, or take it now at the controller's discretion.

Nothing else in the diff introduces breakage. The `--fail-fast` sentence it
extends still reads cleanly, no heading moved, and the diff touches nothing
outside the two prescribed lines.

---

## Sweeps and untouched regions, on the fix tree

| check | result | fired control |
|---|---|---|
| eleven-glyph typography over the fix diff's added lines | clean, exit 1 | the em-dash class against a synthetic line returns 1 |
| new `file:line` citation in the fix diff | none, exit 1 | the same expression against `+ ... README.md:91 ... \`:889-936\`` returns 1 |
| four `placeholder(1.0)` comments | 4 | - |
| work-in-progress banner | present | - |
| passthrough recipe unchanged by the fix | True, 177 bytes at `e657263` and `845cf89` | mutated copy compares False |

**The full gate as `BUILDING.md` enumerates it - eleven parts, foreground, all
exit 0**, run by me on the fix tree. `cargo test --workspace`: 505 `... ok`
lines, including
`test readme_passthrough_recipe_with_title_template_survives_dry_run_and_run ... ok`.
`ledger-lint: 541 entries across 4 files plus BUILDING.md's gate enumeration, all
invariants hold`; no `FAIL BUILDING.md:` line.

**The report's 538 -> 541 attribution checked rather than accepted.** In the range
`f2e9d75..845cf89` the only commit touching any house YAML is `d72d73e`, the
controller's harvest; the entry-count delta across it is **+3**, reproduced by two
different counting patterns (the absolute totals differ between them, so only the
delta is quoted - the two patterns match different supersets). Neither task commit
touches house YAML, so the plan's single-writer constraint held.

---

## Deferred, not extending the loop

- The new NIT above, if the controller prefers to bank it rather than spend a
  round.
- Nothing else. No out-of-scope observation surfaced in this delta that is worth
  a ledger entry beyond what round 1 already harvested.

## Delta harvest

**A fix that re-measures the reviewer's finding can still under-measure it, and
the gap is the population, not the mechanism.** The fix report reproduced Finding
1 correctly on `dry-run` and proved the mechanism as a one-site count - but the
sentence it wrote quantifies over *every* subcommand, and one measured member does
not carry a universal. Adding `validate` (a second member, directly at the shell)
and the kernel's `SigCgt` (the disposition itself, not its source cause) is what
turns the claim from well-argued into measured. Handle: **your prose says "any",
your evidence says "this one".**

**The instrument's own shape is a claim, twice over in this round.** `grep -c`
over a one-line paragraph counts lines and answers a question nobody asked -
mine, caught by re-running per occurrence. And a registration sweep that names
only `ctrlc` asserts that `ctrlc` is the only route; the honest form enumerates
the neighbours (`signal_hook`, `sigaction`, `libc::signal`) and shows them empty.
Both are the same defect as the round-1 occurrence already in the ledger, now with
two more shapes: a counting unit that does not match the artifact's shape, and an
alternation that omits the alternatives it exists to exclude.
