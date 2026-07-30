# Task A2 verdict - Plan 11 (W2)

**Verdict: APPROVED.**

**Reviewed:** `5d305a2` over `a0d5d3e` on `plan-11-stream-a`, worktree
`/home/senol/Git/muxsmith-plan11-a`. Independent instruments under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a2rev-independent/`.
Tree at exit: `git status --porcelain` empty, `git diff --stat 5d305a2` empty,
`git status --porcelain -uall` 0 lines. No product file edited, nothing
committed, nothing pushed, no session-relocation tool called, all runs
foreground.

The decisive evidence is one equality, not a walk of the diff. I reconstructed
the end state in my own process - `a0d5d3e`'s two blobs plus the plan's two
fenced substitutions, extracted from the plan document by line range rather than
retyped - and compared byte for byte with the committed blobs. Both match
exactly (`recon 10081 == head 10081`, `recon 1378 == head 1378`). That single
result answers contract compliance and scope together: every byte in the end
state is either a byte of `a0d5d3e` or a byte the plan fences. The instrument is
fired (a one-byte perturbation of the committed copy, `worker_count` ->
`worker_kount`, makes it report `False` with the offending line; restored, it
reports `True`).

---

## Findings

All five are informational. **None requires a fix round**, none changes a
product byte, and none is a condition on this approval.

### 1. INFO - the ledger-lint attribution in "Noticed, not touched" compresses what A1's review said

`task-a2-report.md`, "Noticed, not touched", fourth bullet: `ledger-lint` "was
deliberately NOT run and is NOT cited as coverage ... per the constraint carried
from Task A1's review". The constraint the report then quotes (four markers, the
fenced command lines, one `BUILDING.md` sentence, blind to every comment region)
is accurate and I reproduced it. What A1's verdict actually wrote in its HARVEST
item 2 is `A2, A3 and A4 each run python3 scripts/ledger-lint.py for free, and
their green run means less than it looks`, and the dispatch brief rendered that
as `Do not cite it as coverage for your own work` - a citation ban, not a run
ban. The report's "per the constraint" therefore reads as if A1 had told A2 not
to run it. It did not; the not-running is the implementer's own call, which I
adjudicate as correct below. Required change: none, but a future report
attributing a decision to an upstream verdict quotes the clause it rests on
rather than the conclusion it drew from it - the same class
(`code-comment-line-citations-drift`, second trigger: attributing a sentence to
the decision it paraphrases rather than to its host) this plan repairs.

### 2. INFO - `.github/workflows/ci.yml:93` is a 35-column line inside a 70-76 column comment paragraph

Measured: lines 88-95 are 76, 76, 75, 73, 71, **35**, 70, 29 columns. The
short line is `      # Plan 4 until this task. All`, the tail of the plan's own
fenced replacement (a) - the substitution grew the sentence by two lines and did
not reflow the paragraph. This is the plan's text character for character, so it
is not a defect in the execution, and the plan's "Must not decide" clause
forbade the implementer from re-wrapping it. Recorded for the HARVEST because
the re-deferred mise rider's new observable is "the next change that edits
`ci.yml`'s Plan-5.5 comment block for its own reasons", and that change should
reflow 90-95 in the same diff.

### 3. INFO - the commit subject says "outside docs/" where the plan's Interfaces say "every tracked file outside `docs/`"

`5d305a2` subject: `comments: locate code by symbol in the CI workflow and the
D48 fixture, closing the class outside docs/`. The plan's Interfaces clause is
"'tree-wide' here means every tracked file outside `docs/`". In this worktree
the two coincide - `git status --porcelain --untracked-files=all` returns 0
lines, so there is no untracked non-`docs/` file at all, and `.superpowers/` does
not exist in the worktree - so the subject is not false. It is a subject line,
not the disposition. The ROADMAP disposition at the plan close is where the word
"tracked" has to appear.

### 4. INFO - the report's `cargo test --workspace exit=0` is the one claim I did not fully reproduce

I ran `cargo test -p muxsmith-core --test profile_save`: 7 passed, 0 failed,
including `all_non_default_fields_survive_the_round_trip`, the D48 guard that
consumes the edited fixture. I did not re-run the whole workspace. I did not
need to, and the reason is stronger than a re-run: `yaml.safe_load` of the
fixture pre and post returns equal objects and the non-comment line sequence is
byte-identical (36 lines, identical), and no test in the workspace reads
`.github/workflows/ci.yml` (`git grep "workflows/ci\.yml"` outside `docs/`
returns three prose mentions - `BUILDING.md:134`, `deny.toml:2`, and
`crates/muxsmith-core/src/lib.rs:22`, the last of which records the *reverse*
direction, that `ci.yml` greps test output). A workspace test result at
`5d305a2` that differed from `a0d5d3e` would therefore be impossible.

### 5. INFO - the two prescribed expressions have three further blind spots beyond the prose form, all measured clean

Dimension 4 names the shared colon requirement. I swept three more forms the
plan's expressions and its prose probe all miss, over the same surface, each
fired against a control I built:

| form | probe | tree | control |
|---|---|---|---|
| GitHub permalink | `#L[0-9]+` | 0 | fires (1) |
| file + prose locator with `at`/`L`/`@` | `[A-Za-z0-9_-]+\.(rs\|ts\|vue\|mjs\|js\|py\|md\|toml\|yml\|yaml\|sh)[[:space:],]+(at[[:space:]]+)?(line\|L\|@)[[:space:]#]*[0-9]+` | 0 | fires (3) |
| `L<n>-L<m>` span | `\bL[0-9]+-L?[0-9]+\b` | 0 | fires (1) |

Plus the file types expression B excludes by extension (`png ico icns wav snap
lock`, 20 tracked files outside `docs/`): re-run with `grep -a` and no
exclusion, **0** hits; same pipeline over the Plan-10 document returns 15, so it
reads what it is pointed at. The class is closed on a wider surface than the
plan asked for.

---

## Dimension walk

**1. Contract compliance, character for character.** PASS by the reconstruction
equality above. The `old` blocks occur exactly once each in the base
(`occurrences of fenced OLD block = 1` for both), so the substitution is
deterministic and there is no ambiguity about where it landed. `git diff
--name-status a0d5d3e 5d305a2` names exactly the two files; a full `git diff`
excluding both returns empty; the range holds exactly one commit.

**2. The symbol is the right one.** PASS, both reads done at the artifact.

- At `004e1e8^` (`git show 004e1e8^:crates/muxsmith-core/src/executor/queue.rs |
  cat -n`): line 71 `pub struct QueueOpts {`, line 72 the first line of the
  `jobs` field's doc comment, line **73** `    /// batch's spec count (see
  [`worker_count`]) so a `--jobs` far larger`. The cited line **is** the link
  line, inside `QueueOpts::jobs`'s doc.
- `004e1e8` is the citing commit (`git log -S 'broken intra-doc link
  (queue.rs:73' -- .github/workflows/ci.yml` returns it and `5d305a2`, nothing
  else).
- Today, line 73 is `pub struct QueueOpts {`, so naming the symbol the line now
  holds would name the struct rather than the field - the silent regression the
  task exists to prevent. The replacement names the field.
- `worker_count` is private at both states: `fn worker_count(jobs: usize,
  spec_count: usize) -> usize {` with no `pub`, at `004e1e8^:299` and at
  `5d305a2:384`. The doc today reads `(see the private `worker_count` helper)` -
  a code span, not a link - so "which then linked" is the correct tense and the
  comment stays a true historical record while being anchored to a durable name.
- `queue.rs` resolves to exactly one tracked file.
- For site (b): `D48` is resolvable and greppable (`docs/superpowers/specs/
  2026-07-08-muxsmith-v1-design.md:377`, `2026-07-15-plan-6-design.md:96`,
  `:106`, `:113`, `:117`), where `:1517-1535` named neither a file nor anything
  greppable. Strict improvement, and byte-identical to the `(design D48)` form
  Plan 10 committed at `profile_save.rs:95`.

**3. Both corpus expressions re-measured, 1 -> 0.** PASS. Run in my own process
with the alternation derived from each tree, pre-state taken from a `git archive
a0d5d3e` extraction (1416 files, identical file set to the end state, no file
added or removed):

| | pre `a0d5d3e` | end `5d305a2` |
|---|---|---|
| Expression A | 1 (`.github/workflows/ci.yml:90`) | **0** |
| Expression B | 1 (`crates/muxsmith-core/tests/fixtures/all-non-default.yaml:2`) | **0** |

The derived alternation is identical in both states and contains `yaml` and
`yml` (checked, because the instrument's own enumeration is a claim -
`a-search-whose-terms-come-from-memory-produces-a-false-absence`). Three
independent fires for the zeros, none of them the implementer's: (i) my own
synthetic three-file control dir, where A returns 1 and B returns 1 and neither
fires on a `12:30`-style non-citation; (ii) the pre-state run itself, same
pipeline, same shell; (iii) the same pipeline over the **end-state** tree with
the `docs/` filter removed, returning 5499 (A) and 2757 (B), which proves the
instrument reads the real tree at the state where it reports zero.

**4. The shared blind spot.** PASS. The prescribed prose sweep returns 2, both
`e2e/smoke.spec.ts` (`:635`, `:647`), both the string literal `"mkvmerge output
line 1"` in the fake-mkvmerge harness record and its assertion - test data, not
citations. My wider independent sweep (`zeilen?|lines?|ln\.?|l` plus a digit and
an optional range) returns 8: the same 2, plus `run.rs:556/564/572/574/575`
(`lines0`/`lines1` locals) and `capability/mod.rs:127` (`A_MPEG/L3`). No
citation among them. Both sweeps fire on a control carrying `see line 42 of
queue.rs` and `Zeile 17`.

**5. Comment-only, both files, fixture DATA unmoved, and the report attributes
it correctly.** PASS. `git diff -U0` hunks are `@@ -90,2 +90,4 @@` and `@@ -1,2
+1,2 @@`. My own classifier (strip the diff marker, strip leading whitespace,
require `#`) reports **0** non-comment changed lines and fires (reports 1) on a
synthetic diff whose added line is `run: cargo doc`. Independently: parsed
fixture objects equal pre/post, and the 36 non-comment lines byte-identical.
The report attributes the data-unmoved property to the `-U0` diff and explicitly
argues the round-trip test cannot carry it ("a data change that still
round-trips would pass it") - which is the correct attribution and the one the
plan's Step 4 demands.

**6. The workflow still parses, and nothing reads it.** PASS. `python3 -c
"import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0; the
same command over a deliberately broken copy exits 1, so the check is not
vacuous. Stronger: `yaml.safe_load` of pre and post are equal objects, so no
step, `run:`, `name:`, `runs-on` or pin moved semantically either. The premise
that no gate part reads this file holds - `scripts/ledger-lint.py`'s `FILES` is
the four house YAMLs, plus `BUILDING.md` for check 7, and nothing else; no test
harness loads workflow YAML.

**7. The count `17` is untouched.** PASS. All three sites carry it unchanged
across the diff: `crates/muxsmith-core/src/profile/model.rs:28`,
`crates/muxsmith-core/tests/fixtures/all-non-default.yaml:1`,
`crates/muxsmith-core/tests/profile_save.rs:64`. The pre-state carries the same
three. Only the parenthetical moved.

**8. Latitude, both forms.** PASS, including the inverse. Explicit permission:
none granted, none taken - every string written is fenced in the plan, verified
by the byte equality. Omission: the one place the task could have invented
something is the wording of replacement (a), and it did not. The **inverse**
form - a fork the task resolved at the keyboard that it should have returned -
is also absent: the one live fork (the mise rider whose text targets lines
inside this task's own authorized region) was routed by the controller before
dispatch and the implementer verified the routing premise itself rather than
borrowing it. Step 1's three NEEDS_CONTEXT cases did not arise because the
re-measured set matched.

**9. House dimension, by id.**

| id | judgement |
|---|---|
| `comments-locate-by-symbol-never-by-line-number` | Conformant, on the WIDENED clause (`# comment in a workflow or config file is in scope`). The handle is applied exactly: number replaced by the symbol the line sat in. Its "Historical statements get NO exception" sentence is what resolves the apparent conflict with `code-comment-line-citations-drift`'s evidentiary class below - and the replacement satisfies both anyway, since "which **then** linked" keeps the record historical while anchoring it to a durable name. |
| `code-comment-line-citations-drift` | Conformant. The two-class rule would have permitted an evidentiary record to keep its span; the governing entry above overrides that for source and now config comments, and the plan says so in Step 2. No new `file:line` enters the tree. |
| `a-search-whose-terms-come-from-memory-produces-a-false-absence` | Conformant, both halves. The file selector is `git ls-files` (the tree itself) and the pattern alternation is derived by `sed` from the tree's own extension tally in the same shell, in both the implementer's run and mine. |
| `proc-sweep-surface-completeness` | Conformant, and exceeded. The surface is named explicitly (tracked files outside `docs/`) and the classification section pastes the command and its full output rather than summarising - which is A1's HARVEST item 4 discharged. My finding 5 widens the surface further and finds nothing. |
| `proc-verification-step-must-be-falsifiable` | Conformant. Every absence in the report carries a fire; I built my own for each rather than re-running theirs. |
| `proc-check-green-state-reachable` | Conformant. The green state was reachable by construction - the single matched line in each expression sat inside the comment the fenced replacement rewrites, and neither replacement contains a filename-plus-digits or a bare span. |
| `tests-ship-with-the-feature-never-after` | Conformant. No user-visible consequence is created; the one newly observable property (the workflow parses) gets an explicit check, and the exemption invoked is the standing one for new test INFRASTRUCTURE, which a workflow-YAML harness would be. |
| `ledger-lint-runs-before-every-push` | Not triggered. Its readable trigger is "you are about to push"; A2 does not push. |
| `agent-commit-trailer-set` | Conformant. Exactly one trailer, `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, canonical model name with no context-window suffix, no `Claude-Session` line, `%G?` -> `N` (unsigned). |
| `concurrent-writers-need-pathspec-scoped-commits` | Conformant. `git commit ... -- <two paths>`, explicit staging, no `git add -A`. |
| `proc-no-work-needed-check` | Discharged - see below. |
| `design-empirical-claims-reproducible` | Conformant. Every figure in the report is pasted, and every one I re-ran reproduced (see the evidence appendix). |
| `a-document-never-cites-a-line-number-inside-itself` | Not applicable - the report cites other files, never itself. |

**The no-work-needed check (`proc-no-work-needed-check`), premises run rather
than weighed.** Every "therefore unnecessary" in the report:

| premise | run | result |
|---|---|---|
| "no gate part parses `.github/workflows/ci.yml`", so the YAML check earns its own row | `scripts/ledger-lint.py` `FILES` list read; `git grep "workflows/ci\.yml"` outside `docs/` | Holds. Three prose mentions, zero readers. `lib.rs:22` is the reverse direction. |
| "the repository has no harness that loads workflow YAML", so no test | `git grep -l "yaml.safe_load\|workflows/"` over `crates/ e2e/ scripts/ src/ src-tauri/` | Holds. Only `crates/muxsmith-core/src/lib.rs`, and that is prose about `ci.yml` grepping test output. |
| "`ledger-lint` is blind to every comment region this task touched" | its `FILES` list vs `git diff --name-only a0d5d3e 5d305a2` | Holds. Zero intersection. Measured runtime, since "free" is itself a claim: **0.39 s**, output `ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold`. |
| "the rider's `Rust gate part 6` appears nowhere in `ci.yml`" | `grep -c` on the file | Holds, 0. Fired control on `master`'s ROADMAP: 2. |
| "no Rust line-length gate exists", so `queue.rs:75` at 91 columns is cosmetic | file-existence loop (fires: `Cargo.toml` PRESENT); `git grep "max_width\|wrap_comments"` | Holds. No `rustfmt.toml`, `.rustfmt.toml`, `.editorconfig`, prettier config; no width setting anywhere. `awk` confirms line 75 = 91, line 71 = 31. |
| "the count `17` is a different fact with its own consumers" | `git grep "17 defaulted fields"` over `crates/` | Holds. Three sites, all unchanged. |

**Typography.** Both edited regions are ASCII-only (`LC_ALL=C grep -P
'[^\x00-\x7F]'` empty; fired control: `docs/ROADMAP.md` returns 33) and carry no
em-dash, en-dash, smart quote, ellipsis or Unicode minus (probe empty; fires on
a synthetic em-dash). The report itself: 0 such glyphs.

---

## The five adjudications

### 1. The rider the task was forbidden to consume - **CORRECTLY OBSERVED, and no conflict with the plan.**

The rider lives in `master`'s `docs/ROADMAP.md` under "Remove mise from CI
(post-1.0)" and prescribes, verbatim, `# legs, matching the cross-target clippy
gate part (BUILDING.md, Rust gate part 6; cfg-gated items can differ per
platform).` Its target is the two lines that today read `      # legs, matching
the cross-target lint rule (cfg-gated items can` / `      # differ per
platform).`

**The target lines are byte-identical to their pre-state.** `a0d5d3e`'s ci.yml
lines 92-93 and `5d305a2`'s lines 94-95 have the same md5; the byte-for-byte
reconstruction proves it globally, since the only bytes that differ anywhere in
the file are the fenced substitution.

**The premise was verified, not borrowed**, and I re-verified it: `grep -nE
'part [0-9]|parts [0-9]' BUILDING.md` returns nothing on the end state and three
lines (`:102`, `:134`, `:135`) at `a0d5d3e^`. So applying the rider would have
written `Rust gate part 6` back into a second file hours after A1 removed it
from the first - invisibly, because nothing reads `ci.yml` for ordinals.

**No conflict with anything the plan requires.** The plan never mentions the
rider in any task context (`grep -ni rider` over the plan returns six lines,
none of them about A2 or `ci.yml`). Task A2's Files list authorizes "the `cargo
doc` step's leading comment block, comment text only", which does contain the
rider's target lines - but Step 2 fences exactly two lines and the "Must not
decide" clause forbids composing wording not written in the plan. Applying the
rider would have violated the plan; leaving it violates nothing. The controller's
record on `master` ("FIRED 2026-07-30 ... deliberately RE-DEFERRED", with the new
observable named) closes the loop and the worktree's older ROADMAP legitimately
lacks it.

### 2. The positional locator that drifted - **RIGHT ENTRY SURFACED, and the surfacing is complete enough to act on.**

I reproduced the drift independently. In **both** the worktree and `master`, the
"Docs accuracy" section's first bullet is `**THE README'S FIRST EXAMPLE PROFILE
DOES NOT LOAD**` and the line-citation entry (`**ONE member of the comment
line-citation class SURVIVES Plan 10's sweep ...**`, worktree `:1900`) is the
**second**. The plan's Step 5 and Read-first list, and the dispatch brief, all
say "first entry".

The implementer surfaced the right entry, and named it by its opening words
rather than its ordinal - which is the durable anchor and the same principle the
task itself enforces one level down. The surfacing carries: the file, the
section, the entry's opening words, both stale claims **quoted from the entry**
(I verified both quotations against `docs/ROADMAP.md:1913-1917` and `:1908` -
they match), the Tier-2 clause that answers the open question (verified against
`docs/conventions.yaml`), and the derivation that returns two members where the
entry claims one. It also flags the ordinal drift itself and routes the
house-knowledge question to the controller instead of deciding it. Nothing has
to be re-derived to act. W2-f is GREEN.

### 3. The scope boundary - **APPLIED AS THE CONVENTION STATES IT, and the closure claim carries its surface.**

The convention's boundary runs "by the artifact DOING the citing": process
artifacts **under `docs/`** keep citing a line at a named commit. The task
applied exactly that - it swept the fixture's citation **into** a design
document, because the citing artifact is a test fixture, not a process artifact,
and it touched no file under `docs/` (proven at the tree, adjudication 4). The
inverse reading, exempting the fixture because the cited artifact is a design
document, is the one the plan explicitly forecloses and the implementer did not
take.

**The surface is carried honestly, in all three places it appears.** The report
makes no closure claim at all - `grep -niE "tree-wide|class (is )?closed|
closure"` over it returns nothing. The commit subject says "closing the class
outside `docs/`". The plan's Interfaces clause spells out that "tree-wide" means
"every tracked file outside `docs/`". Measured residue, so the close's wording is
written against a number rather than an impression: under `docs/` the same
expression finds citations in 566 process-journal artifact files, 16 plan
documents and **12 design/spec documents** (`docs/superpowers/specs/`, including
`2026-07-30-plan11-raw-bytewise-design.md` at 70 and `2026-07-21-plan7-help-i18n-design.md`
at 124), the four house YAMLs and the ROADMAP. The journal, plans and YAMLs are
squarely the boundary's named process class; the **design documents are the
interesting residue** - they are governed by `code-comment-line-citations-drift`'s
two-class rule, not closed by anything. A close statement of the form "the class
is closed" without "outside `docs/`" would be false, which is precisely the
over-claim the ROADMAP entry was written to prevent.

### 4. What was NOT edited - **PROVEN AT THE TREE.**

Not from the report. `git diff --name-status a0d5d3e 5d305a2` returns exactly
two `M` lines. A full `git diff` over the range with both files excluded by
pathspec returns empty. The range holds one commit. The byte-for-byte
reconstruction closes it from the other side: the end state is the base plus the
two fenced substitutions and nothing else, so within the two touched files
nothing but comment text moved either. Specifically:

- **No file under `docs/`**: not in `name-status`.
- **No line of code or data**: 0 non-comment changed lines by my own fired
  classifier; the fixture's 36 non-comment lines byte-identical and its parsed
  object equal.
- **No `runs-on`, no pin, no `run:`, no `name:`**: `yaml.safe_load` of pre and
  post are **equal objects**, which is a stronger statement than checking those
  four keys individually - it covers every key in the workflow at once.

### 5. The free `ledger-lint` run the task did not make - **RIGHT, and the omission costs nothing.**

Ruling: correct. Three grounds, in order of weight.

1. **The obligation does not exist here.** `ledger-lint-runs-before-every-push`
   has a readable trigger - "you are about to push" - and A2 does not push; the
   plan's exit-bar subset for A2 is `cargo test --workspace` plus `git diff
   --stat`, and the plan names `ledger-lint` as an exit bar for A1 and A4, not
   A2. The dispatch brief told the implementer not to **cite** it, not to run it,
   so nothing was disobeyed either way.
2. **A green run here is provably zero-information about this deliverable.**
   Its input set (`docs/conventions.yaml`, `docs/process-conventions.yaml`,
   `docs/product-boundaries.yaml`, `docs/decision-ledger.yaml`, plus
   `BUILDING.md`'s markers) has **zero intersection** with this diff. Its green
   is guaranteed before it runs, which is exactly the shape
   `proc-verification-step-must-be-falsifiable` calls decoration that reads as
   coverage. A1's reviewer already fired this empirically by mutating the prose
   A1 edited and watching `ledger-lint` stay green.
3. **The generic argument for running it anyway - "it might catch an unintended
   edit elsewhere" - is strictly dominated here.** `git diff --name-status` over
   the range already proves exactly two files changed, which is a complete
   statement over the whole tree where `ledger-lint` would have covered five
   files. Running a weaker check in addition to a stronger one is not an
   integrity duty.

For the record, since "cheap" and "free" are claims and the plan's own habit is
to measure them rather than assert them: I ran it once, read-only, and it costs
**0.39 s wall** and prints `ledger-lint: 560 entries across 4 files plus
BUILDING.md's gate enumeration, all invariants hold`, exit 0. So the cost
argument was never the reason, and it is right that the report did not make it
one. The counter-position - that a task editing a tracked file owes the cheapest
integrity check regardless - would be right if the check's surface reached the
edit. It does not, and a task that cited it would be inflating its evidence, not
adding to it.

---

## Evidence appendix

All instruments under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a2rev-independent/`.
Nothing was written into any repository. None of these is a re-run of an
instrument the implementer wrote.

| path | what it is |
|---|---|
| `reconstruct.py` | Builds the end state from `base-*` plus `a-new.txt`/`b-new.txt` and compares byte for byte with `head-*`. Requires the OLD block to occur exactly once. |
| `a-old.txt`, `a-new.txt`, `b-old.txt`, `b-new.txt` | The four fenced blocks, extracted from the plan by line range (`sed -n '403,404p'`, `'410,413p'`, `'421,422p'`, `'428,429p'`) - copied out of the document, never retyped. |
| `base-ci.yml`, `base-fixture.yaml`, `head-ci.yml`, `head-fixture.yaml` | `git show a0d5d3e:` / `git show 5d305a2:` of both files. |
| `recon-ci.yml`, `recon-fixture.yaml` | The reconstructed end states. |
| `pre/` + `pre-files.txt` | `git archive a0d5d3e | tar -x` (1416 files) and `git ls-tree -r --name-only a0d5d3e`, for running the corpus expressions on the pre-state without touching the worktree. |
| `head-files-index.txt`, `head-files-tree.txt` | `git ls-files` and `git ls-tree -r --name-only 5d305a2`; identical to each other and to the pre-state file set. |
| `corpus-run.sh` | Runs expressions A and B against a given root + file list, deriving `EXT` from that list. Used unchanged on pre-state, end state and control. |
| `ctrl/` + `ctrl-files.txt` | My synthetic control tree: a `.yml` with a filename citation, a `.yaml` with a bare span, a `.rs` with near-miss non-citations (`12:30`), a `.rs` with prose locators, a `.rs` with permalink / `at line` / `L73-L80` forms. |
| `broken-ci.yml` | `sed 's/^jobs:/jobs: [broken/'` of the workflow, the negative control for the YAML parse check (exit 1). |
| `gitfire/` | Throwaway git repo used to fire `git status --porcelain` (clean `[]`, dirty `[ M f.txt]`) without dirtying the worktree. |

Load-bearing commands, all foreground, all absolute-path or run with an explicit
`cd` into the worktree:

```
python3 <scratch>/reconstruct.py                      # True/True; False on a one-byte perturbation
git diff --name-status a0d5d3e 5d305a2                # exactly 2 M lines
git diff a0d5d3e 5d305a2 -- . ':(exclude).github/workflows/ci.yml' ':(exclude)crates/muxsmith-core/tests/fixtures/all-non-default.yaml'   # empty
<scratch>/corpus-run.sh <scratch>/pre <scratch>/pre-files.txt          # A=1 B=1
<scratch>/corpus-run.sh /home/senol/Git/muxsmith-plan11-a <scratch>/head-files-tree.txt   # A=0 B=0
<scratch>/corpus-run.sh <scratch>/ctrl <scratch>/ctrl-files.txt        # A=1 B=1 (control fires)
git ls-files | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+" | wc -l                   # 5499, docs/ included: instrument reads the real end-state tree
git show 004e1e8^:crates/muxsmith-core/src/executor/queue.rs | cat -n | sed -n '65,85p'   # line 73 = the [`worker_count`] link line
git log -S 'broken intra-doc link (queue.rs:73' -- .github/workflows/ci.yml               # 004e1e8 (+ 5d305a2)
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"                # exit 0; broken copy exits 1
python3 -c "...yaml.safe_load(base) == yaml.safe_load(head)"                              # True, for BOTH files
cargo test -p muxsmith-core --test profile_save                                           # 7 passed, incl. the D48 guard
/usr/bin/time -f "wall=%es" python3 scripts/ledger-lint.py                                # 0.39s, 560 entries, exit 0
git status --porcelain ; git diff --stat 5d305a2 ; git status --porcelain -uall | wc -l   # empty, empty, 0
```

---

## HARVEST

**Method worth keeping.** The reconstruct-and-compare equality is the cheapest
complete review this repo has found for a fenced-substitution task, and it should
be the default for A3 and A4. It cost one small script and it answers contract
compliance, diff scope and "was anything else touched" in a single boolean,
including the class of change a diff walk cannot see because it renders as
context. **Its precondition is the thing to check first:** the fenced OLD block
must occur exactly once in the base. Where it does not - and A3's twelve sites
across six files is where that will bite - the equality has to be built per site
with an anchor, or it silently reconstructs the wrong occurrence.

**For Task A3, specifically.**

1. **A3 is the first stream-A task where the equality will not be a one-liner.**
   Twelve repair sites, seven retained sites, two comparator functions and three
   transcribed tests, all fenced in D111 rather than in the plan. Extract the
   fences from D111 by line range, never retype them, and check the
   occurs-exactly-once precondition per site before trusting any substitution.
2. **The retained set needs the inverse equality.** Seven sites must be proven
   *unchanged*, and `matcher.rs` and the v1 spec are in both sets, so a
   file-level byte identity is impossible for them. The per-site form is the only
   one available there.
3. **A3's alternation-free vocabulary sweep is where finding 5's lesson lands.**
   An alternation is an enumeration and an enumeration is a claim; derive it from
   the artifacts and fire it against a known-present member of *each* branch, not
   just one.
4. **The `raw:` behaviour change makes A3 the first task in this stream with a
   user-visible consequence**, so `tests-ship-with-the-feature-never-after` binds
   for real rather than being weighed away. T-1 in particular is the safeguard
   that survives `proc-proposed-safeguard-stays`.

**For Task A4.** Its exit-bar subset explicitly includes `ledger-lint` where A2's
did not, and the plan states why (the spec is not one of its four files, so it
proves only that nothing else broke). That asymmetry between A2 and A4 is
deliberate and adjudication 5 above should not be read as licence to drop it
from A4.

**For the plan close.**

5. **The ROADMAP disposition must carry "tracked" and "outside `docs/`".** The
   measured residue is in adjudication 3: 12 live design/spec documents under
   `docs/superpowers/specs/` still carry `file:line` citations, and they are not
   the boundary's named process class - they are durable artifacts under
   `code-comment-line-citations-drift`'s two-class rule. That is a real open
   surface, not a technicality, and a disposition that says "the class is closed"
   flat is the same over-claim the entry was written to stop.
6. **The line-citation ROADMAP entry needs two repairs, not one, plus a third
   observation.** Its "OPEN OWNER QUESTION" paragraph is answered (session-28
   widening, recorded in the Tier-2 statement); its "ONE member" headline is
   contradicted by a two-member derivation; and the plan's own Step 5 and
   Read-first list still locate it as the section's **first** entry when it is
   the **second** in both `master` and the branch. The third is the controller's
   to fix in the plan document or to let the merge carry.
7. **The re-deferred mise rider now has a second reason to fire.**
   `ci.yml:93` is a 35-column line in a 70-76 column paragraph (finding 2). When
   the rider's new observable trips - the next change editing that comment block
   for its own reasons - the reflow rides the same diff, and the rider's own
   replacement text must be re-derived from `BUILDING.md`'s then-current wording
   rather than reused, because the fenced version names an ordinal that no longer
   exists.
8. **The house-knowledge occurrence the implementer routed rather than decided.**
   The ordinal-drift observation (a plan locating a tracker entry by position,
   falsified while the plan was being written) is a clean instance of the class
   `a-document-never-cites-a-line-number-inside-itself` exists for, one level up:
   the number is an ordinal rather than a line, and the container name -
   the entry's opening words - was available and durable. If the controller wants
   it recorded, `a-document-never-cites-a-line-number-inside-itself` is the entry
   whose statement generalises, and this would be its third occurrence.
9. **Report-attribution hygiene (finding 1).** When a task report justifies a
   decision by an upstream verdict, it quotes the clause it rests on, not the
   conclusion it drew. A1's HARVEST said A2 "each run ... for free, and their
   green run means less than it looks"; the report rendered that as a constraint
   not to run. Same class as the second trigger of
   `code-comment-line-citations-drift`, one artifact type over.
