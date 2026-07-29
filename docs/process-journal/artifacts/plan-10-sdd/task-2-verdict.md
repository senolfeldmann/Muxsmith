# Task 2 verdict - Plan 10 (W1: the D102 preserved-order producers)

**Verdict: APPROVED_WITH_MINORS.**

Reviewed at `35bc363`, read from the files rather than from the report. The
four-mutation measurement was re-run end to end with my own instrument against
my own reconstruction of the pre-producer tree, and it reproduces the
implementer's pattern exactly: M1 red, M2 red, M3 green, M4 green, with the two
named guarding tests being the only failures their mutations produce. The two
producers each fail under their own mutation and only under their own. The
production file is byte-identical to `39a9055`, the full eleven-part gate is
green, and the tree is byte-identical to `35bc363` with the integrity check
fired.

No finding requires a fix round. The three minors are one report-accuracy defect
(the code it justifies stands), one plan defect the implementer surfaced
correctly, and one item that needs a vehicle rather than an edit.

---

## Findings

### 1. MINOR - the report's house-pattern justification for the import merge is refuted; the merge itself stands

`.superpowers/sdd/plan-10/task-2-report.md:411-419` (divergence 2) justifies
folding `run_document` into the new brace group with the claim that "a second
`use` of the same module path is not this tree's pattern."

Measured over all 73 tracked `.rs` files, three carry exactly that shape:

```
crates/muxsmith-core/tests/prop_matcher.rs:19-20
    use muxsmith_core::profile::model::KeepDrop;
    use muxsmith_core::profile::model::{
crates/muxsmith-core/tests/mkvmerge_runtime.rs:5,9
    use muxsmith_core::capability::runtime::{MIN_SUPPORTED, Mkvmerge};
    use muxsmith_core::capability::runtime::RuntimeError;
crates/muxsmith-core/src/matcher.rs:9,11
    use crate::capability::codec_kind_prefixes;
    use crate::capability::{PropType, matchable_type};
```

The instrument was fired against a synthetic `use a::b::c; / use a::b::{d, e};`
pair before its output was believed.

The merged form is **dominant**, not **unbroken**, and the difference is exactly
the one the report leaned on. The committed line
(`crates/muxsmith-core/tests/report_json.rs:11`) is correct regardless: the plan
fences a brace group on that same module path and is silent on what to do with
the pre-existing single import, all four zero-outward-effect conditions hold, and
the merged form is what most of the tree does.

**Required change: none in code.** The claim should have read "the dominant
pattern" rather than asserting an unbroken one. Recorded because dimension 7
requires the premise behind a disposition to be run rather than weighed.

### 2. MINOR (plan defect, not implementer) - the plan's `mixed_severity()` fence and gate part 1 are in direct conflict

`docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md:319` fences a
one-line `Diagnostic::warning(...)` call that `cargo fmt --all --check` rejects.
Verified independently in scratch (not by re-running the implementer's
experiment): the reconstructed fenced file exits 1 under
`rustfmt --edition 2024 --check`, the committed file exits 0, and
`rustfmt --edition 2024` applied to the fenced file produces output
**byte-identical to the committed file** (`diff -u` exit 0).

My own measurements of the cause: the fenced line is **96 columns** with its
indentation, so `max_width` (default 100) is not the rejector; the argument list
is **66 characters** against rustfmt's `fn_call_width` default of **60**, which
is. The repo carries no `rustfmt.toml`, so defaults apply. The implementer's
diagnosis is correct on both counts.

**Required change: none in this task.** Adjudication 1 below rules the committed
form correct. The defect belongs to the plan and to future plan authoring: a plan
that fences Rust source character for character must fence rustfmt-stable text,
or state that the fence is content-level and the formatter's output governs.

### 3. MINOR - the module doc comment's staleness has no vehicle anywhere in Plan 10

`crates/muxsmith-core/tests/report_json.rs:1-7` enumerates the file's contents
and does not mention the two producers this task added. Adjudication 2 rules the
implementer's restraint correct and requires no edit here. The finding is that
nothing downstream picks it up: Task 5's Files list has **16 entries** and
`crates/muxsmith-core/tests/report_json.rs` is not among them (control: the same
grep finds `crates/muxsmith-core/tests/suggestions.rs` and
`crates/muxsmith-core/src/report/json.rs`, both of which are), and Task 5 is
comment-text-only in files it does list.

**Required change: none in this task.** The controller routes it - either into
the plan close's ROADMAP dispositions or into the same vehicle finding 2 of the
Task-1 verdict already uses for `BUILDING.md`'s deferred cosmetics.

### 4. INFO - this machine carries a second, shadowed `mkvmerge`, which silently breaks the obvious no-mkvmerge instrument

`/home/linuxbrew/.linuxbrew/bin/mkvmerge` (symlink to the Cellar v100.0 build,
first on PATH, the one the plan's authoring section names) **and**
`/usr/bin/mkvmerge` (Fedora package, same size as `/bin/mkvmerge`). My first
attempt to measure adjudication 3's premise stripped PATH to `/usr/bin:/bin` and
the guard still went red; the reason was not that the gate is PATH-independent
but that mkvmerge was still on the stripped PATH. Redone against an empty
directory, the guard skips and passes (evidence in adjudication 3).

**Required change: none.** Recorded so the next agent that reaches for
`PATH=/usr/bin:/bin` to simulate a mkvmerge-less machine does not draw the wrong
conclusion from it, as I nearly did.

---

## Dimension results

| # | Dimension | Result |
|---|---|---|
| 1 | Re-run the measurement | **Reproduced exactly.** M1 red, M2 red, M3 green, M4 green, own instrument, own restores |
| 2 | The mkvmerge dependency | `mkvmerge v100.0`, exit 0. M1's failure is the reported guard and nothing else - it is the ONLY failing test workspace-wide |
| 3 | Producers against the fence | Scaffolding matches the fence; the sole deviation is finding 2's rustfmt wrap. P1/P2 bodies match the plan's specification field for field. Import list correct, `config_only_document` correctly absent |
| 4 | Do the producers assert what they claim | **Symmetric, verified.** M4 -> P1 alone red workspace-wide; M3 -> P2 alone red workspace-wide. No coupling, neither redundant |
| 5 | No production code changed | `crates/muxsmith-core/src/report/json.rs` byte-identical `39a9055..35bc363` (`git diff` empty). No pre-existing test changed behaviour: 503 -> 505, delta exactly 2 |
| 6 | House dimension | All four entries hold; see below |
| 7 | No-work-needed checks | Two run. One holds (adjudication 3's premise, now measured). One refuted as stated (finding 1) |
| 8 | Verification quality | All **11** gate parts green, run by me as `BUILDING.md` enumerates them. Every aggregate recomputed |
| 9 | Blast radius on Task 5 | **Corpus unchanged.** 20 lines / 13 files (A), 4 lines / 4 files (B), union 16 files. Task 2 added no member |

**House entries.**

- `tests-ship-with-the-feature-never-after` - **held.** Every half this package's
  measurement found unguarded got its producer in this same task. Nothing
  deferred, nothing resolved by documenting the gap.
- `comments-locate-by-symbol-never-by-line-number` - **held.** Both assertion
  messages cite "spec section 5.2, Diagnostics" by section. Both corpus
  expressions return zero hits against the committed file, and that absence was
  fired: injecting `report/json.rs:68` and a bare `` `:255` `` into a copy makes
  each expression match its own control.
- `latitude-carveout-zero-content-structural-forks` - **held**, see adjudication
  2. One over-restriction datum surfaced, in the do-not-loosen direction.
- `proc-verification-step-must-be-falsifiable`, PER ASSERTION - **held, and
  independently re-fired.** The two new assertions are not covered by one probe:
  each has its own mutation, each red state produced by me, each with the other
  assertion staying green in the same run. Every absence check in this verdict
  carries its own control.

---

## Adjudications

### 1. The fence versus `cargo fmt`

**Verdict: writing the formatter's output was correct fidelity to the plan taken
as a whole. It should not have returned NEEDS_CONTEXT, and the committed form
stands.**

**Does the committed form differ from the fence in anything but line breaks and
indentation?** **Yes - in exactly one further character.** rustfmt adds a
trailing comma after the last argument when it wraps the call, which the fenced
one-line form does not carry. Measured: stripping all whitespace still leaves the
two texts different; stripping all whitespace *and* normalizing a comma before a
closing paren makes them byte-identical, and that normalizer was fired (changing
one character of the string literal makes it report a difference). So the honest
answer is "line breaks, indentation, and one syntactic trailing comma". **No
token of content differs**: same call, same arguments, same values, same strings.

The ruling, with the counter-position stated first because it is real. A fence is
a fence: the brief says "transcribe, do not compose", the plan's Global
Constraints say a contradiction found on code contact is "refuted with evidence
or returned, never silently absorbed", and every fork in the plan is declared
closed with keyboard resolution forbidden. Under that reading the implementer
found a genuine plan defect, refuted nothing, and picked a side. And the fork was
not even forced: I measured a third option the report does not mention -
`#[rustfmt::skip]` on `mixed_severity()` keeps the fence verbatim **and** exits 0
under `rustfmt --check`. So "the gate makes the fence impossible" is false as an
absolute.

It still loses, on three grounds:

1. **The plan's own Step 5 makes `cargo fmt --all --check` a Task 2 exit bar**,
   named before the commit. The plan therefore requires the formatted form by its
   own text. Between two clauses of one document, the one that is a binding check
   on this task outranks a code block's line breaks.
2. **The fence's stated purpose is that no CONTENT is invented**, and rustfmt is
   a deterministic, content-preserving function of the fenced text. I proved the
   equality directly: `rustfmt(fence)` is byte-identical to what was committed.
   The implementer composed nothing; it applied a function.
3. **The third option invents content.** `#[rustfmt::skip]` is an attribute the
   plan does not fence, permanently added to the source, chosen at the keyboard.
   It preserves the fence's bytes by violating the principle the fence exists to
   serve. Between writing rustfmt's output and writing an unfenced attribute, the
   former is the smaller deviation by exactly the measure that matters.

A NEEDS_CONTEXT would have cost a controller round to authorize the only
content-preserving action available. The implementer surfaced it as concern 2
with pasted evidence and a correctly diagnosed cause, which is the disclosure the
boundary is designed to produce.

**Consistency with the Task-1 verdict, since it ruled the opposite way on a
superficially similar case.** There, the fenced text passed the gate and a reflow
would have been an extra cosmetic change beyond the fence; leaving it was correct.
Here the fenced text fails the gate, so leaving it verbatim was not an available
option. Both rulings apply one principle: do the minimum the plan's own binding
checks require, and invent nothing beyond it.

The plan defect is real and recorded as finding 2.

### 2. The module doc comment

**Verdict: correct restraint. This is not the grant's named in-scope case, and no
edit was owed.**

The text at `crates/muxsmith-core/tests/report_json.rs:1-7` reads "Direct core
coverage for `report::json` (Plan 5 Task 2): ... this file adds the one
direct-in-core assertion the brief calls for, plus the `run_document` unit tests
relocated from `muxsmith-cli/src/commands/run.rs`".

Two independent reasons it was right to leave it:

1. **Nothing it references was invalidated.** The grant names "repairing a
   REFERENCE which the task's OWN enumerated edit invalidated (a doc link, a
   comment referent, an import)". Every referent in this comment - `report::json`,
   the Plan-5 brief's assertion, the relocated `run_document` tests, the CLI
   integration tests - still exists and still says what the comment says it says.
   What changed is that the file grew a category the comment does not mention.
   That is an unmentioned addition, not a broken referent, and the grant's second
   condition ("nothing changes beyond the referent") cannot even be evaluated
   because there is no referent to repair. This is the same test the Task-1
   verdict applied to reach the opposite-looking conclusion on the 86-character
   line, and it lands the same way.
2. **The comment is provenance-tagged.** "(Plan 5 Task 2)" scopes the whole
   sentence to what that package added, and read that way the statement is not
   even incomplete - it is a true record. Widening it would require attributing
   the new tests to Plan 10 Task 2, i.e. composing prose the plan does not fence,
   which is the content invention the fence exists to prevent. The implementer's
   own framing - "incomplete, not false" - concedes more than it had to.

A third, weaker ground I name but do not rest on: Task 2's Files entry reads
"(the stub renderer, the shared fixture, and the producers Step 2's measurement
selects)", which is arguably the "named region" within-file qualifier the owner's
2026-07-28 ruling describes. I do not rely on it, because the parenthetical
reads at least as naturally as a description of what is being added as a fence
around what may be touched, and the ruling above does not need it.

**Calibration for the over-restriction watch: DO NOT loosen on this datum.** The
boundary stopped nothing that should have moved. Both this task and Task 1 now
show the tight setting producing the wanted behaviour - decide nothing, surface
it, let the controller route it. What the pair does show is that the class keeps
recurring, which is a routing-cost observation rather than a boundary problem; it
belongs in the harvest, and finding 3 gives it its missing vehicle.

### 3. The `have_mkvmerge()`-gated sorted-half guard

**Verdict: correct handling. Nothing belongs in this task. The vehicle stays
open, and its wording needs repair at the close.**

**The ROADMAP already tracks it.** `docs/ROADMAP.md:539-549`, immediately after
the D102 "RULED ... BUILD IT" close, records the fact in its own paragraph,
states why ("a fact with no vehicle evaporates"), names both candidate fixes ("a
second producer on the no-mkvmerge path or a gate-independent construction"), and
sets a vehicle. So the coverage fact is not at risk of being lost, which is the
only thing this task could have protected.

**The premise, measured rather than borrowed.** With M1 applied and PATH pointed
at an empty directory, the guard prints `mkvmerge not found; skipping` and the
run exits **0**:

```
running 1 test
mkvmerge not found; skipping
test dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out
```

Against the same binary with mkvmerge present, the same M1 gives exit 101. So the
implementer's concern 3 is exactly right: on a mkvmerge-less machine W1-a would
have measured unguarded and selected P3 for the wrong reason. (Getting this
measurement right required finding 4 - my first attempt used a PATH that still
contained mkvmerge.)

**Why building something anyway would have been wrong.** The same ROADMAP
paragraph says in so many words: "Not Plan 10's problem and deliberately not
folded into it ... both are their own decision." Task 2's must-not-decide list
forbids duplicating the two CLI guards and enumerates the four candidate
producers exhaustively; a fifth producer on the no-mkvmerge path is an
unenumerated producer, which is precisely the latitude the plan closes. The
implementer stopped in the right place and surfaced it.

**The wording defect worth fixing at the close.** The vehicle reads "whichever
package next touches the diagnostics ordering contract, or the owner QA pass if
it surfaces the symptom first" - and Plan 10 Task 2 **is** a package touching that
contract, in the same paragraph that exempts Plan 10 by name. The two sentences
contradict each other on their face. The nearer and more specific one (the
explicit Plan-10 exemption) governs, which is how I rule, but a vehicle whose
condition fires on the package that is exempted from it will force this
re-derivation on every future reader. At the close it should name a package
*after* Plan 10, or state the exemption inside the vehicle sentence itself.

---

## Evidence appendix

All instruments built fresh under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t2rev-independent/`.
Nothing the implementer wrote was re-run. All runs foreground, on `master` in
`/home/senol/Git/Muxsmith`, no worktree, no session-relocation tool.

**Instruments**

| Path | What it is |
|---|---|
| `rvw_mutate.py` | My own mutation driver. Function-scoped replacement (the sort block occurs **twice** in the file, once per builder, so a naive whole-file replace would mutate both); aborts unless the anchor matches exactly once inside the targeted function; refuses a no-op |
| `json.rs.BASELINE-35bc363`, `report_json.rs.BASELINE-35bc363`, `baseline.sha256` | Baselines taken before any mutation |
| `A0-preproducer-baseline.log`, `A-M1..M4.log` | Phase A: the measurement re-run against my reconstruction of the pre-producer tree (`git show 39a9055:... > <path>`, working tree only, index untouched) |
| `B-M3.log`, `B-M4.log` | Phase B: the symmetry fire against the committed tree |
| `gate-1.log .. gate-11.log` | The eleven-part gate |
| `fence-scaffolding.rs`, `committed-scaffolding.rs`, `fenced_form.rs`, `fenced_then_formatted.rs`, `fenced_skip.rs` | Adjudication 1 |
| `corpusA.txt`, `corpusB.txt`, `fired_corpus.rs`, `task5-files.txt` | Dimension 9 |
| `nobin/`, `nomkv.log` | Adjudication 3 |

**The measurement, my run, pre-producer tree**

| # | Command | Exit | ok binaries | passed | Failing test(s) |
|---|---|---|---|---|---|
| baseline | `cargo test --workspace` | 0 | 39 | **503** | none |
| M1 | `cargo test --workspace` | **101** | - | - | `dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran` (the only one) |
| M2 | `cargo test --workspace` | **101** | - | - | `dry_run_and_validate_json_agree_on_config_diagnostics_ordering` (the only one) |
| M3 | `cargo test --workspace` | **0** | 39 | 503 | none |
| M4 | `cargo test --workspace` | **0** | 39 | 503 | none |

M1 and M2 stop the workspace run at the first failing binary, which is why no
39/503 aggregate exists for them; the failure marker count is 4 on those logs and
0 on M3/M4, so the counting instrument is demonstrably able to see a failure.
Mutated-state fire pasted per mutation (`git status --porcelain` and
`git diff --stat` both non-empty during each), restored hash printed after each
and equal to `5c16e5c6d203c593b417ef58e4c055795b35d407ab09f92d240149edaec25b8a`
every time. My M1 mutated hash is `04f636297a44b982...`, matching the
implementer's - independent confirmation the same edit was applied.

**Symmetry, my run, committed tree** (`cargo test --workspace`, both exit 101)

| Mutation | Red, workspace-wide | Green |
|---|---|---|
| M4 | `batch_document_preserves_batch_diagnostics_collection_order` | `batch_document_preserves_per_file_diagnostics_collection_order` + all 4 others in the binary |
| M3 | `batch_document_preserves_per_file_diagnostics_collection_order` | `batch_document_preserves_batch_diagnostics_collection_order` + all 4 others |

In each case that new producer is the **only** failing test in the entire
workspace, which is a stronger statement than the report's per-binary run: it
also proves no pre-existing test changes behaviour under either mutation.

**Gate, run by me as `BUILDING.md` enumerates it** - 11 parts, 6 Rust, 4
frontend, 1 house-knowledge, in the file's own order, foreground, no subsets.
Every part exit 0. Part 3: 39 binaries, **505** passed, 0 failure markers. Part
10: `68 passed`. Part 11:
`ledger-lint: 533 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold`.
**505 - 503 = 2**, recomputed from my own two runs rather than carried from the
report.

**Tree integrity, with the check fired**

```
git rev-parse HEAD          -> 35bc3630061e9d318a8ae0de440a07856c319a20
git status --porcelain      -> (empty)
git diff --quiet 35bc363    -> exit 0   (worktree identical)
git diff --quiet --cached   -> exit 0   (index identical)
FIRE: apply M4              -> git diff --quiet 35bc363 exit 1
      git checkout --       -> git diff --quiet 35bc363 exit 0
git stash list              -> (empty)
```

Both files I mutated hash back to their pre-review baselines. Commit metadata
checked: `%G?` = `N` (unsigned, as policy), exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, one file, 97
insertions / 1 deletion.

---

## HARVEST

**For the controller. I surface; I do not write the house-knowledge files.**

1. **`proc-verification-step-must-be-falsifiable`, a new failure shape worth an
   occurrence: the control that does not control.** Finding 4 is the sharpest
   datum this review produced. I fired a check correctly - stripped mkvmerge from
   PATH, watched the guard stay red - and the fire was meaningless because the
   thing I removed was still there under a second path. The existing statement
   covers greps whose pattern cannot match; this is the sibling where the
   *environment manipulation* silently fails to manipulate. Handle in the same
   readable form as the rest: after removing a dependency to prove something
   depends on it, verify the removal itself (`command -v`) before believing the
   result. Cost here: one wrong measurement, caught only because the result was
   surprising enough to check.

2. **Plan-authoring rule candidate: a character-for-character fence of Rust
   source must be formatter-stable.** Finding 2 plus adjudication 1. The trigger
   is readable at authoring time: you are about to paste a Rust code block into a
   plan as a verbatim fence, in a repo whose gate runs `cargo fmt --check`. The
   handle is one command - run the block through `rustfmt` before fencing it - and
   it costs seconds against a fix round or a NEEDS_CONTEXT. The generalization,
   which is what makes it worth an entry rather than a note: **a fence and a gate
   check are both normative, and where they collide the plan has silently opened
   the fork it declares closed.** Either the fence is formatter-stable, or the
   plan says explicitly that the fence is content-level and the formatter's
   output governs. This is the second Plan-10 task whose adjudication 1 turns on
   a fenced text meeting a mechanical check it was not written against.

3. **The neighbouring-artifact class has now cost a routing in each of Plan 10's
   two tasks, and it is a plan-authoring gap rather than an implementer or
   boundary problem.** Task 1's 86-character line and Task 2's module doc are one
   shape: an artifact the task's own fenced edit leaves cosmetically or
   descriptively behind, which the grant does not reach, which the implementer
   correctly surfaces, and which then needs a controller routing decision. The
   ledger's plan-9 T2 entry is the *near* case that shows why implementers keep
   having to judge it - there the same shape WAS grant-covered (a broken doc link
   and a stale "see the drain loop below") and was correctly repaired in-task.
   The discriminator is whether a referent broke, which is a fine line to walk
   per task. Two routings in two tasks is a cost worth removing at the source. The
   candidate handle is on the plan author, not the implementer: **when a task's
   Files list names a file, check whether that file carries a self-description -
   a module doc, a header comment, a contents index - and either fence its update
   or state that it stays.** That converts a recurring escalation into a
   one-sentence authoring step. Recorded as calibration data in the
   DO-NOT-loosen direction for the over-restriction watch: the boundary itself
   behaved correctly all three times.

4. **The ROADMAP's mkvmerge vehicle contradicts its own exemption** (adjudication
   3). Fix the wording at the plan close, in the same disposition pass that
   resolves the D102 paragraph. As written, the condition fires on the package the
   sentence above it exempts by name.

5. **What the measurement established, for the D102 disposition.** The
   ROADMAP-recorded finding covered `batch_diagnostics` only; the four-mutation
   method found a **second** independently unguarded half, per-file
   `files[].diagnostics` (W1-d). Both sorted halves were already guarded, each by
   exactly one CLI test, and the two new producers are disjoint - each reddens
   under its own mutation only. That is a direct empirical vindication of the
   plan's decision to split W1 into four acceptance rows rather than one, and it
   is the second consecutive package in which "one producer named for a whole
   observable" would have hidden a real gap. It belongs on
   `tests-ship-with-the-feature-never-after`'s halves-walking companion handle as
   a reinforcing occurrence.

6. **What Tasks 3-5 must carry.**
   - **Task 5 is unaffected by this task.** Corpus re-measured on the current
     tree: **20 lines / 13 files** under expression A, **4 lines / 4 files** under
     expression B, union **16 files**, matching Task 5's 16-entry Files list.
     Task 2 introduced no member; both expressions return zero hits against the
     committed test file, fired against injected controls. Task 5's Step-1
     re-measurement will land on the same numbers unless Tasks 3 and 4 move them.
   - **`crates/muxsmith-core/src/report/json.rs` is untouched at `35bc363`**, so
     Task 5 inherits it exactly as the plan's transient-write-set section
     predicted. That is the one production file two tasks touch, and the handoff
     is clean.
   - **Task 1's `BUILDING.md` gate-count check ran green in this review's own
     independent gate run** and emitted the widened summary line
     (533 entries across 4 files). Second independent execution.
   - **The `cargo test --workspace` fail-fast behaviour** is worth knowing for
     Tasks 3-5: a red binary stops the run, so "39 binaries / N passed" aggregates
     only exist on green runs. Any task reporting a workspace aggregate from a red
     run is reporting a partial.
