# Task A4 verdict - Plan 11, stream A (W4)

**Verdict: APPROVED_WITH_MINORS**

**The product file is flawless.** The end-state spec is byte-for-byte the
`164e571` blob with exactly the two fenced substitutions and nothing else
(reconstruction below), and the surface it now states agrees with the shipped
binary in every cell of a table I re-derived from `--help` myself, without
reading the plan's table first. The one commit touches one file. No section
Task A3 amended moved.

**The defects are in the report's evidence, not in the artifact.** Two pasted
`grep` outputs in Step 1e are not the outputs of the commands they are
attributed to - each omits one line the command returns, and one mis-transcribes
a comment prefix. The Global Constraints name this class verbatim
(`design-empirical-claims-reproducible`: "never attributed to a command that was
not the one run"). The underlying claims are true - I verified them independently
at the three sources - so no product change and no fix round is owed; the
correction is to the report, and it must ride the merge because this is the last
task of the plan.

---

## Findings

### 1. IMPORTANT - two pasted grep outputs in the report are not what the commands return

`.superpowers/sdd/plan-11/task-a4-report.md:289-298` and `:305-308`.

Both sit in Step 1e, the section whose whole purpose is that the exit-code claim
is "a measurement, not an inference", and the second is introduced by the words
"The full enumeration of every exit-producing site in the crate, **so the list
above is derived and not recalled**". The enumeration is not full.

**Paste A** (`report:289`), attributed to
`grep -rn "process::exit\|return 130\|-> i32" crates/muxsmith-cli/src/`, shows
**9** lines. The command returns **10**. Missing:

```
crates/muxsmith-cli/src/commands/validate.rs:18:pub fn run(profile_path: &Path, json: bool, renderer: &Renderer) -> i32 {
```

The omitted line is `validate`, one of the four non-`run` subcommands the
surrounding claim is *about*. (The paste also reorders `mod.rs` ahead of
`identify.rs` relative to the real traversal order; that alone would be weak
evidence, the missing line is not.)

**Paste B** (`report:305`), attributed to `grep -rn "ctrlc" crates/muxsmith-cli/src/`,
shows **3** lines. The command returns **4**. Missing:

```
crates/muxsmith-cli/src/commands/run.rs:175:    // are unchanged, since `cancel` itself is still what the ctrlc handler
```

and `report:307` transcribes the `:183` hit with a `///` prefix where the file
carries four spaces and `//`:

```
crates/muxsmith-cli/src/commands/run.rs:183:    // registration in the process, so ctrlc's double-registration error is
```

**No state explains it.** `crates/muxsmith-cli/src/commands/run.rs` is blob
`4803b3f64b33e9578cb06cecabfc5506bb9982aa` at `5378264`, `164e571`, `06e896e`
and `master` alike, so there is no commit at which either command returns the
pasted text.

**The conclusions survive.** I re-derived both independently:
`ctrlc::set_handler` occurs exactly once in the tree, in `run.rs:189`; the two
`130` sites are `run.rs:191` (inside the handler closure) and `run.rs:252`
(guarded by `cancel.load`); `severity_exit` in `commands/mod.rs:25` returns only
`2/1/0`; `main.rs:69` is the sole process exit and dispatches `Schema` (literal
`0`), `Validate`, `DryRun`, `Identify`, `Run`. `validate.rs`, `dry_run.rs` and
`identify.rs` return only `0`, `1`, `2`. So **"Only `run` returns 130; no other
subcommand installs a SIGINT handler" is correct as shipped in the spec.**

**Required change (report only; no product edit, no new commit, no fix round):**
replace both pasted blocks with the actual output of the two commands as given
above, and drop the word "full" from the Paste-B lead-in or make it true. The
merge carries the corrected report.

**Why IMPORTANT and not MINOR:** the constraint is named explicitly in Global
Constraints, these are the sole evidence pastes behind a sentence this task put
into the authoritative document, and the defect appears **twice in one section**,
which makes it a pattern rather than a slip. Why not NEEDS_FIXES: the artifact is
byte-correct, the claim is independently verified, and a round would change no
product byte.

### 2. MINOR - spec 8.4 is cited for a proposition it does not carry

`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:416`; consumed at
`report:227-231` and `report:610` and in the review brief's adjudication 3.

8.4's exception reads: "**No hardcoded user-facing strings** in any layer ...
Accepted v1 exceptions: clap's library-generated `--help`/usage text ...". Its
subject is the **localization mandate** - that text need not be translated. It
does not say 8.1's synopsis may omit `-h/--help`.

The omission is still right; its ground is ordinary synopsis convention (no man
page enumerates `-h`), not 8.4. The conclusion is unaffected.

**Required change:** none in the spec. When the point is restated (close notes,
whole-branch review), attribute the omission to synopsis convention and cite 8.4
only for what it says - that clap's generated text is outside the localization
mandate.

### 3. MINOR - the no-permanent-checker deferral points at a section that does not hold its trigger

`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
Deferred-by-decision row "No permanent checker compares the spec's 8.1 synopsis
against `--help`", vehicle "The ROADMAP's 'Reach-claim checker' candidate
section, which already holds this question for its whole class."

I ran the premise. `docs/ROADMAP.md:2407-2421` **supports the decision** in its
own terms: the instrument was "**Deliberately NOT promoted** into
`scripts/ledger-lint.py`, on the reviewer's recommendation and the controller's
agreement: it parses PROSE, which is what `proc-check-green-state-reachable`
names as the way such a check becomes permanently red on correct content". That
is the ground the task claims, and it holds.

What does not hold is the vehicle's completeness: that section's stated
reconsider triggers are "if a third table appears - the README's
`placeholder(1.0)` mandates one at the tag - or if a reach claim goes stale
again", both scoped to `docs/INSTALL.md` and the release-body table. The
spec-8.1-versus-`--help` question is not named there, so the row routes to a
section that does not know it exists.

**Required change:** at the plan close, add one line to the ROADMAP's
Reach-claim checker section naming the spec-8.1 synopsis as a second member of
the class, so the deferral has a home rather than a mention. Controller's file
(`docs/ROADMAP.md` is a close action, not a task edit).

### 4. MINOR - `README.md:194`'s Windows-false clause has no deferred-by-decision row, and the plan judged the same clause by two standards

`README.md:194`; the plan's item-4 authoring block ("The README is already right
and needs no edit") against Task A4 Step 3 ("its Windows half cannot be measured
here"). Surfaced correctly by the implementer at `report:801-813`.

Detail is in adjudication 1 below. In short: the plan cut the shell-convention
clause from the spec because its Windows half is unverifiable, and in the same
document asserted the README "is already right ... on 130" while the README
carries that same clause unscoped. The deferred-by-decision table has rows for
the `cli.rs` sentence, the seven retained `byte-literal` assertions, `glib`,
`raw:codec_kind` and the 17-defaulted-fields count - and none for this.

**Required change:** a deferred-by-decision row at the plan close, with the
`cli.rs` row's vehicle shape (whichever package next amends README's "Using the
CLI"). Not a change to this task, whose Files list correctly forbids the edit.

### 5. MINOR - the plan's Step-2 prose mis-describes its own fence

Plan Task A4 Step 2: "**The continuation indent aligns under `<profile>`**".
Measured on the fence itself, reproducing the implementer's figure exactly: on
`muxsmith dry-run  <profile> [--source DIR] [--output DIR]`, `<profile>` begins
at index **18**, `[--source` at index **28**, and the continuation lines are
indented **28**. The indent aligns under the **option list**, not under
`<profile>`.

The fence governs, was applied byte for byte, and the artifact is right. Detail
and disposition in adjudication 2.

---

## Adjudications

### 1. The README keeps what the spec cut

**Verdict: not a contradiction - two true statements about different subjects -
but the README's clause carries an independent defect that needs a vehicle, and
the right moment is the plan close, not now.**

*Not a contradiction.* The two sentences have different subjects, and each is
true of its own:

- Spec 8.1: "Only `run` returns 130; no other subcommand installs a SIGINT
  handler." Subject: what **Muxsmith's own process returns**. Verified at
  `main.rs:69`, `commands/mod.rs:25`, `commands/run.rs:191`/`:252`.
- README:194: "Interrupt any subcommand with Ctrl-C and you get `130` instead,
  the shell's own convention for a signalled process, so handle it in your
  `case $?`." Subject: **what the user observes in `$?`**, which for a
  non-`run` subcommand is the shell's 128+signal synthesis, not a muxsmith
  return. The README carries the distinction itself in the next clause: "Only
  `run` **earns** that code gracefully."

So no spec-versus-README repair is owed, and the implementer's read is right.

*But the README clause is independently defective, and this task concentrated
it.* The claim is false on Windows. `ctrlc` registers a console-event handler in
`run` only; for `validate`, `dry-run`, `identify` and `schema` on Windows there
is no handler, Ctrl-C terminates with `STATUS_CONTROL_C_EXIT` (0xC000013A), not
130 - and `case $?` is POSIX-shell syntax that neither `cmd` nor PowerShell
speaks. That is exactly the ground on which Task A4 Step 3 cut the clause from
the spec. The plan applied that standard to the authoritative document and the
opposite one to the user-facing document in the same breath ("The README is
already right and needs no edit"). Net effect of this task: **the unverifiable
platform claim now lives only in the document users actually read.**

*Vehicle timing: the close, not now.* Not now, because the task's Files list is
exhaustive and correctly so - rewording a shipped README sentence about
three-OS signal behaviour needs the same owner decision the plan deferred for
`cli.rs`, and deciding it inside a spec-only task is the sanctioned-fork shape
the latitude constraint bans. At the close, because that is where the plan's
deferred-by-decision table lives and it currently has no row for this
(finding 4). The implementer surfaced the fact; what they did not name is the
plan's own asymmetry, which is the part the controller needs in order to write
the row.

### 2. The plan's prose mis-describes its own fence

**Verdict: the artifact is right; the plan's descriptive sentence is a genuine
defect; a close-note is the proportionate vehicle, not an amendment.**

The measurement reproduces exactly (finding 5): `<profile>` at index 18,
`[--source` at index 28, continuation indent 28. "Aligns under `<profile>`" is
wrong; "aligns under the option list" is right.

*The artifact is right* because the fence is the normative object and it was
applied byte for byte - the reconstruction below proves the end state is the
pre-state plus exactly these two substitutions.

*Why a close-note rather than an amendment.* An amendment exists to change what
a task **does**. This sentence changed nothing: the fence was already
authoritative, the implementer was forbidden to adjust it, and the block that
shipped is the block the plan fenced. Amending an executed task's descriptive
prose after the fact rewrites the record of what was dispatched, which cuts
against the plan's own constraint that a retired plan document is history whose
wording "was true when written" - here it was not true when written, and the
honest repair is a note saying so, not a silent edit making the plan look like
it always said the right thing.

*Why it is not merely cosmetic.* The Step-2 prose is the fallback a later reader
uses if the block is ever re-derived rather than copied. Following the wrong
sentence yields indent 18, which produces a different block - legal, since line
363 would then be 68 characters and the plan's "no line exceeds 80" conclusion
survives either way, but not the block that shipped. A wrong-but-legal
regeneration is worth a note and not worth a round.

### 3. The divergence table omits `-h/--help`

**Verdict: yes, the omission weakens the plan's table as evidence - it is a
latitude-by-omission shape - and the implementer handled it correctly, with one
citation I would not inherit.**

*It weakens the table.* The column is headed "the binary's `--help` lists",
which is a **capture** claim, and it silently drops `-h/--help` from all five
rows and the `help` subcommand from the row set entirely. The binary's top-level
`--help` lists **six** commands. A reader re-deriving the surface from that
table and a reader re-deriving it from `--help` get different answers, which is
the unenumerated-set-in-normative-position shape the plan's own latitude clause
bans - the plan's own Latitude paragraph even claims "the five subcommands of
Task A4" as a closed set, and the binary lists six.

*The implementer handled it correctly, twice over.* Their table lists
`-h/--help` per row and states the exclusion explicitly - "**The plan's
authoring table omitted it silently; this run states the omission rather than
inheriting it**" - which is precisely the right repair for a
latitude-by-omission finding: state it, do not inherit it. Separately they
captured `muxsmith help --help` (clap rejects it) and `muxsmith help` (prints
top-level help) and classified it: `help` states no surface of its own and is
correctly absent from a synopsis of the product's five subcommands. I reproduce
both outputs exactly. Nothing was left for me to find here.

*The one thing I do not inherit:* their ground for the exclusion is spec 8.4,
which does not carry that proposition (finding 2). The exclusion is right on
synopsis convention; 8.4 governs localization.

### 4. Half a repair, by design

**Verdict: leaving the second half to a deferred vehicle is right.** A task that
makes a citation land does not thereby own every sentence the citation sits in.

The two halves are different facts with different owners. "Does spec 8.1 carry
130?" is a property of the spec and this task's file; it is repaired, and I
verified the citation now lands (`grep -c '130'`: 0 on the pre-state, 2 on the
end state, both inside the new bullet). "Does **every command** produce 130?" is
a property of `cli.rs` **and** of an unruled platform question - what a non-`run`
subcommand does under SIGINT on three OSes - which is the same signal-death
wording the plan deliberately refused to decide for the spec. Deciding it inside
a one-file spec task, at the keyboard, is the fork the plan bans.

*Steelman for the other side:* the task's own measurement is what exposes the
sentence as over-broad, so the task manufactured the finding and should carry
it. I reject it because it proves too much - under that rule every measurement
obliges its own repair and a task's boundary becomes whatever it happened to
measure. And the premise is false here: `cli.rs` was already over-broad before
A4 ran. A4 did not falsify the sentence; it removed the *other* defect in the
same sentence. Plan 10's precedent for an adjacent claim a task did not falsify
is to surface it, which is what happened.

*The deferral is properly vehicled* - "whichever package next edits
`crates/muxsmith-cli/src/cli.rs`, carrying the measurement from A4 Step 6" is an
event somebody already watches, not a promise. *One weakness for the
whole-branch review:* if no package edits `cli.rs` before 1.0, that vehicle
never fires and the over-broad comment ships. Worth deciding there whether it
wants a ROADMAP line with its own trigger instead.

### 5. Completeness against the acceptance map

**Verdict: every half is produced. One item in the task's observable surface
lacks a producing check; it is by design, correctly grounded, and I ran the
premise.**

| row | halves | producer | my re-derivation |
|---|---|---|---|
| **W4-a** | (i) the fence applied, (ii) the table re-derived from `<sub> --help` for every subcommand the binary lists | five `--help` captures + flag-granularity table + applied fence | **Both present.** I captured all six commands' help myself before reading their table; my table agrees with the shipped block in every cell - per-subcommand membership, the binary's own option order, the metavariables, and `schema` correctly flagged as already right |
| **W4-b** | (i) the fence applied, (ii) `grep -c '130'` non-zero with the target inside the expression, not in the prose beside it | fence + red/green pair | **Both present**, and correctly kept as a separate row: I re-ran both states (0 -> 2) and confirmed both hits are inside the new bullet, not in neighbouring prose |
| **W4-c** | (i) the exit-code sweep as an enumeration, (ii) the flag sweep with its fired control returning the amended block | both sweeps pasted, six hits classified, control `:361`-`:367` | **Both present.** See the sweep re-runs below |

*The item with no producing check:* nothing verifies that the spec's synopsis
**stays** matching the binary. That is named, weighed and declined on a cited
house record, and I ran the premise as instructed - `docs/ROADMAP.md:2407-2421`
supports the decision in its own words. Supported, with the vehicle gap at
finding 3.

*Test duty:* correctly discharged rather than waived. This task changes a
specification document and introduces no user-observable behaviour, so
`tests-ship-with-the-feature-never-after` is satisfied on its own terms - there
is no consequence this package created for which a test is owed.

---

## Dimension walk

**1. The surface is what the binary says.** Binary proven current two ways: the
newest tracked `.rs` is `matcher.rs` at 16:20:05.882226631, the binary at
16:23:01.999860891, and the staleness `find` fires when handed an older
reference; then `cargo build -p muxsmith-cli --bin muxsmith` finished in 0.05s
with no compilation units, so it matches by cargo's fingerprint and not only by
mtime. My table, built from my own captures before reading theirs:

| subcommand | positional | binary's flags, in the binary's own order (value name; possible values) | shipped 8.1 line | verdict |
|---|---|---|---|---|
| `validate` | `<PROFILE>` | `--json`; `--locale <LOCALE>`; `-h/--help` | `[--json] [--locale LOCALE]` | **agrees** |
| `dry-run` | `<PROFILE>` | `--source <SOURCE>`; `--output <OUTPUT>`; `--on-collision <ON_COLLISION>` (error \| skip \| overwrite); `--json`; `--locale <LOCALE>`; `-h/--help` | `[--source DIR] [--output DIR] [--on-collision POLICY] [--json] [--locale LOCALE]` | **agrees** |
| `run` | `<PROFILE>` | `--source`; `--output`; `--on-collision` (error \| skip \| overwrite); `--jobs <JOBS>` ([default: 1]); `--fail-fast`; `--json`; `--locale`; `-h/--help` | `[--source DIR] [--output DIR] [--on-collision POLICY] [--jobs N] [--fail-fast] [--json] [--locale LOCALE]` | **agrees** |
| `identify` | `<FILE>` | `--json`; `--locale <LOCALE>`; `-h/--help` | `[--json] [--locale LOCALE]` | **agrees** |
| `schema` | none | `-h/--help` only | no flags | **agrees** |
| `help` | `[COMMAND]...` | clap builtin; `help --help` is rejected | absent | **agrees** by the same convention as `-h/--help` (adjudication 3) |

**No cell of my derivation disagrees with the shipped block.** Metavariables:
the spec uses descriptive `DIR`/`N`/`POLICY` rather than clap's derived
`SOURCE`/`JOBS`/`ON_COLLISION`; `DIR` and `N` are pre-existing spec convention,
untouched by this change, and `POLICY` is **not** an unenumerated set - the same
domain and default are stated at spec `:23`, `:67` and `:226`, matching the
binary's three "Possible values" exactly. Top-level `-V/--version` is absent
from the synopsis by the same convention as `-h/--help`.

**2. Both fenced replacements, character for character.** OLD and NEW extracted
from the plan by line range (`sed -n '585,589p'`, `'595,602p'`, `'610p'`,
`'616,618p'`), never retyped, then applied to the `164e571` blob by my own
script:

- precondition per site: fence1 OLD occurs **1x** in the pre-state, fence2 OLD
  occurs **1x**;
- counter fired both ways: the NEW `validate` line occurs **0x** in the
  pre-state (absent control), `muxsmith ` occurs **6x** (present control);
- NEW blocks: 0x pre / 1x post each; OLD blocks: 0x remaining in post;
- **reconstruction byte-identical to `06e896e`**, and the comparison instrument
  itself fired (one mutated byte -> differs).

So nothing outside the two fences moved. The A3 concern is discharged
concretely: the section-9.2 line carrying both a repaired and a retained clause
is md5 `bfaedd7ac9240fe627c9ef1361479cf8` at pre `:421` and at post `:426` -
identical, shifted by the +5 the block and bullet grew. `git show --stat`: one
file, 10 insertions, 5 deletions, **one hunk** at `@@ -358,15 +358,20 @@`. The
`schema` line is byte-identical to the original including its column alignment.
Every line of the new block and bullet is <= 78 characters.

**3. The exit-code claim at its sources.** Verified independently; see
finding 1's paragraph "The conclusions survive". Both halves of the sentence
hold, and no other subcommand can reach 130 by any path: `severity_exit` yields
only `2/1/0`, `diag_exit_code` returns through it, `job_exit_code` yields only
`2/1/0`, `Schema` returns a literal `0` in `main.rs`, and `main.rs:69` is the
only other process exit in the crate.

**4. The sweep as an enumeration.** Sweep 1 on the pre-state returns **exactly
6** lines, reconciling one for one with the plan's six pre-classified hits; on
the end state it returns **8**, and the delta is fully accounted for - hit 4 was
one line and is now three (`:372` matches `xit cod`, `:373` `[Cc]ancel`, `:374`
`SIGINT`), the other five carry identical text shifted +5. **No hit outside the
six.** Sweep 2 returns **34** on the end state (**30** pre), and its prescribed
control is satisfied: the expression returns the amended block's own lines
`:361`-`:367`. I classified all 34 independently and reach the implementer's
verdicts: `:122`, `:158`-`:167`, `:171`, `:174`, `:191`, `:199`, `:234`, `:277`,
`:316`, `:317`, `:340`, `:450` are **mkvmerge's** flags (a different program);
`:255`, `:290` describe what `--json` emits; `:320` carries `--jobs N` and
`--fail-fast`, both on `run` in the block and on `run` only; `:375` is the
`--json` bullet; `:416` is 8.4's exception list; `:418` is 8.4's `--locale`
sentence. `grep -c '130'`: **0** pre (the red state), **2** post.

*One point worth carrying forward as a positive:* `:418` asserted a CLI
`--locale` that the pre-state synopsis showed on no subcommand. **The amendment
removes a latent self-contradiction that already existed in the spec**, which is
a stronger result than "introduces none".

*My own blind-spot probe*, terms deliberately outside sweep 1's alternation
(`abort|interrupt|terminat|kill|SIGTERM|SIGKILL|return code|status code|exit
status|12[0-9]|13[0-9]|graceful|Abbruch|\^C`), returns one line sweep 1 cannot
see: **`:320`**, "Failures do not abort the batch unless `--fail-fast`" -
job-failure handling, no exit code, **consistent**, and caught by sweep 2
anyway. The implementer's own independently-derived probe found the same line
and gave the same verdict. No hit outside the classified set from any
instrument.

**5. What the task deliberately did not do.** The cut is judged **right**, and
the surviving bullet **is** complete enough to be the authority `cli.rs` cites:
the citation asks whether spec 8.1 carries 130, and it now does, with the
producing subcommand named. What was cut was never about muxsmith's exit code at
all - it was about what a shell synthesises for a process it signalled, a
different actor. A spec that governs three OSes cannot carry an unscoped POSIX
claim, and the Windows half is unmeasurable on this machine. Cutting rather than
rescoping was also the only option available to a must-not-decide fence: a
rescoped clause would have required inventing the Windows half. Consequence and
its vehicle at adjudication 1 and finding 4.

**6. The no-permanent-checker decision.** Premise run; supported. Vehicle gap at
finding 3.

**7. Latitude, both forms.** *Explicit permission:* none taken - both fences
applied verbatim from a line-range extraction, no string invented, the
must-not-decide list respected in all six clauses. *By omission:* one real
instance, in the plan rather than the task - the authoring table's silent
`-h/--help` and `help` exclusion (adjudication 3) - correctly caught and stated
rather than inherited. `POLICY` was checked against the spec's own three
statements of the domain and is not an open set. *The inverse (over-constraint):*
two instances, both handled correctly - the Step-2 indent prose describes the
fence wrongly and the implementer applied the fence and reported the prose
(adjudication 2), and the cut clause was a must-not-decide string that was left
uncut-down and its consequence surfaced.

**House dimension, by id.** `proc-04-spec-wins`: honored, no plan-versus-spec
conflict arose. `design-empirical-claims-reproducible`: **violated twice**
(finding 1). `a-search-whose-terms-come-from-memory-produces-a-false-absence`:
honored - three blind-spot probes on the sweeps' own alternations, plus mine.
`proc-verification-step-must-be-falsifiable` / `proc-check-green-state-reachable`:
honored - the `find` fire with cleanup, the 130 red/green pair, the flag sweep's
control. `proc-wrapped-prose-quote-grep`: honored and demonstrated; I reproduce
both the failing whole-sentence grep (exit 1) and the working fragment
(`cli.rs:12`). `code-comment-line-citations-drift`: citation half repaired.
`proc-no-work-needed-check`: premise run (finding 3).
`tests-ship-with-the-feature-never-after`: correctly discharged.
`concurrent-writers-need-pathspec-scoped-commits`: commit is pathspec-scoped,
one file. `agent-commit-trailer-set` / SI-4: exactly one trailer,
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, unsigned (`%G?` = `N`),
no `Claude-Session` line, no context-window suffix - identical in shape to
`a0d5d3e`, `5d305a2` and `164e571`.
`a-document-never-cites-a-line-number-inside-itself`: the amended spec cites no
line number inside itself.

---

## Tree state

The worktree is byte-identical to `06e896e`, proven three ways:

```
$ git rev-parse HEAD
06e896e55ddde7e73e999d8912eaf977d0ec3d08
$ git diff --stat HEAD          -> empty, exit 0
$ git diff --cached --stat      -> empty, exit 0
$ git status --porcelain -uall  -> empty
$ git stash create              -> empty (nothing to stash)
$ git rev-parse HEAD^{tree}     -> 81210d1b088b3d99c63b5eff04353c204c8cad33
$ git rev-parse 06e896e^{tree}  -> 81210d1b088b3d99c63b5eff04353c204c8cad33
```

My only writes were to the instrument directory. `cargo build` was run once and
was a no-op (0.05s, no compilation units); it writes only to `target/`, which
`.gitignore:1` excludes, and the binary's sha256
`0c66b5580938ddb15722d73c8a512cb0d1f22af943eef414482796ba80c92389` is unchanged.
`/home/senol/Git/muxsmith-plan11-b` was never touched. No session-relocation
tool was called; every run was foreground with absolute paths.

`python3 scripts/ledger-lint.py` -> exit 0,
`ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration,
all invariants hold` (the shape the plan makes invariant; the count is
deliberately unfenced).

---

## Evidence appendix

**Instrument directory** (mine, never the implementer's, never a shared default):
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a4rev-independent/`

| file | what it is |
|---|---|
| `reconstruct.py` | the reconstruction + precondition + fired-counter script |
| `fence1-old.txt`, `fence1-new.txt`, `fence2-old.txt`, `fence2-new.txt` | the four fence bodies, extracted by line range from the plan, never retyped |
| `rebuilt.md`, `actual-post.md` | the reconstructed and actual end states |
| `help-capture.txt` | my own `--version`, top-level `--help` and all six `<sub> --help` captures |
| `sweep-independent.txt` | my blind-spot sweep, terms outside sweep 1's alternation |
| `bin-before.txt` | binary sha256 before the no-op rebuild |

**Commands run** (all in `/home/senol/Git/muxsmith-plan11-a` unless noted):

- currency: `stat`/`find -printf` on the newest `.rs` versus the binary; the
  staleness `find` fired against an older reference (`-newer Cargo.toml`);
  `cargo build -p muxsmith-cli --bin muxsmith`; `sha256sum target/debug/muxsmith`
- surface: `./target/debug/muxsmith --version`, `--help`, and
  `{validate,schema,dry-run,identify,run,help} --help`
- fences: `sed -n '585,589p;595,602p;610p;616,618p'` on the plan;
  `python3 reconstruct.py`
- exit codes: `grep -rn "130" crates/muxsmith-cli/src/`;
  `git grep -n "ctrlc"`; `git grep -n "process::exit"`; targeted reads of
  `commands/run.rs:165-260`, `commands/mod.rs`'s `severity_exit`/`diag_exit_code`,
  `main.rs`, `cli.rs:1-60`; a per-file exit-literal scan across
  `validate.rs`/`dry_run.rs`/`identify.rs` with `run.rs` as the fired control
- sweeps: the plan's two expressions on both blob states plus
  `grep -c '130'`; my own out-of-alternation probe with a `mkvmerge`
  present-control; per-hit classification of all 34 flag hits;
  `grep -oE -- '--[a-z][a-z-]+'` on the four ambiguous lines
- report audit: each pasted command in Step 1e re-run verbatim;
  `git ls-tree` on `run.rs` at `5378264`/`164e571`/`06e896e`/`master`
- premise: `docs/ROADMAP.md:2407-2421`
- typography/format: per-line `length($0)` over `:361`-`:374`; index
  measurement of `<profile>` and `[--source`; md5 of the `schema` line and of
  the A3 dual-clause line across both blobs
- tree: `git diff --stat HEAD`, `git diff --cached --stat`,
  `git status --porcelain -uall`, `git stash create`, `git rev-parse ^{tree}`,
  `git log -1 --format='%an <%ae> | gpg:%G?'`, `git show --name-only`

---

## HARVEST

**The merge must carry:**

1. **The corrected report.** Finding 1's two evidence blocks are replaced with
   the commands' real output before the report becomes the permanent record of
   this task. Report-only; no product byte moves, no new commit on
   `plan-11-stream-a`, no fix round.
2. **No product action.** `06e896e` merges as it stands. The spec is byte-correct
   and the acceptance map is fully produced.

**The whole-branch review must carry:**

3. **A pattern check for finding 1's class across stream A's other three task
   reports.** Two mis-pasted grep outputs in one section of one report is a
   pattern, not a slip, and A1, A2 and A3 all lean on pasted enumerations -
   A3 especially, whose twelve-repair / seven-retained split *is* a pasted grep
   result. Spot-re-run the load-bearing pastes rather than reading them.
4. **The `cli.rs` deferral's trigger.** Its vehicle is "whichever package next
   edits `cli.rs`". Decide whether that event is reliable before 1.0 or whether
   the over-broad "every command shares the exit-code contract" sentence needs a
   ROADMAP line with a trigger of its own (adjudication 4).
5. **The class-closure wording.** This task closes the spec-8.1-drift item for
   the **CLI surface and the exit-code contract in section 8.1**. It does not
   close "the spec matches the binary" generally, and nothing now guards against
   the drift recurring (finding 3). The close's completion statement carries that
   qualifier or it over-claims - the same shape the plan already flags for A2.

**Close actions for the controller** (all in files no task may edit):

6. A **deferred-by-decision row** for `README.md:194`'s unscoped shell claim,
   with the plan's own asymmetry recorded (finding 4, adjudication 1).
7. A **line in `docs/ROADMAP.md`'s Reach-claim checker section** naming the
   spec-8.1-versus-`--help` question as a second member of that class, so the
   no-checker deferral routes somewhere that knows it exists (finding 3).
8. A **close-note** that the plan's Task A4 Step-2 sentence "the continuation
   indent aligns under `<profile>`" is wrong - the fence, which governs and
   shipped, indents under the option list at column 28 (finding 5,
   adjudication 2).
9. A **close-note** disposing of `docs/ROADMAP.md`'s "The v1 spec's section 8.1
   synopsis omits `validate`'s flags" entry, whose stated vehicle ("whichever
   package next amends the v1 spec") this task discharged in widened form.
10. **Ledger-worthy, if the controller agrees it generalizes:** the amendment
    *removed* a pre-existing self-contradiction (spec 8.4 asserted a `--locale`
    CLI flag that 8.1's synopsis showed on no subcommand). The spec-amendment
    sweep found it as a by-product. That is an argument for the sweep's value
    beyond "introduces no new contradiction", and worth a line where the sweep
    doctrine lives.
