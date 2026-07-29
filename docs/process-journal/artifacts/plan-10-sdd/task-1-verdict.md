# Task 1 verdict - Plan 10 (W3: the gate-count invariant)

**Verdict: APPROVED_WITH_MINORS.**

No committed file needs to change and no fix round is required. Every fenced
text is byte-exact, every behaviour Step 2 fixes is implemented and has now been
fired under an independent harness, the full eleven-part gate is green at
`ddb8f42`, and the blast radius on Tasks 2-5 is nil. The five findings below are
one report-evidence imprecision and four observations routed to the controller;
none of them is a defect in the shipped artifact.

The three files are byte-identical to `ddb8f42` after this review (blob-hash
proof in the appendix). The only working-tree modification, `docs/ROADMAP.md`,
was already present in the baseline I took before running anything, and is the
controller's own correction of the positional-ordinal routing entry.

---

## Findings

### 1. Minor - the report's docstring-alignment measurement overstates uniformity

`.superpowers/sdd/plan-10/task-1-report.md:493-496` states that check 7's
"separator column and continuation indent were read off checks 1-6 (`awk
'match($0,/^ */)'` -> 41 leading spaces on continuations, separator at 1-based
column 39)".

Evidence (my own per-line measurement over `scripts/ledger-lint.py:12-31`):

```
 12 ITEM lead= 2 sep_col= 39   1. count == len(occurrences)
 14 ITEM lead= 2 sep_col= 40   2. every occurrence carries a ref
 15 ITEM lead= 2 sep_col= 40   3. status: blocked => blocked_on set
 16 ITEM lead= 2 sep_col= 41   4. tier: 2 => promoted_at is not null
 17 ITEM lead= 2 sep_col= 41   5. id is unique across all four files
 18 ITEM lead= 2 sep_col= 39   6. no duplicate key in any mapping
 21 ITEM lead= 2 sep_col= 39   7. BUILDING.md gate total == blocks   <- the new one
```

The continuation indent is uniform at 41 and check 7 matches it exactly. The
separator column is **not** uniform: it is 39/40/40/41/41/39 across checks 1-6,
so "separator at column 39" is a property of two of the six, not of the set the
sentence attributes it to. The alignment actually chosen (39, matching the
immediately preceding check 6) is correct, so the artifact is right and only the
evidence line generalizes past its measurement.

**Required change: none in the tree.** Recorded because this project grades
evidence lines, and this is the `proc-normative-count-recomputed` shape applied
to a column rather than a count - a number stated about an enumeration that sits
a few lines away and does not survive recomputation from it.

### 2. Minor - `BUILDING.md:138` is the file's first non-fenced prose line over 80 characters

```
$ awk 'BEGIN{inf=0} /^```/{inf=!inf; next} !inf && length($0)>80 \
    {printf "%d (%d chars): %s\n", NR, length($0), $0}' BUILDING.md
138 (86 chars): and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)

$ git show 754cb73:BUILDING.md | awk 'BEGIN{inf=0} /^```/{inf=!inf; next} \
    !inf && length($0)>80 {print}'
(no output)
```

The pre-state carried no non-fenced prose line over 80; this is the first. The
implementer applied Step 1(e)'s fence literally and did not reflow, which is
correct (adjudication 1). Recorded as a routed cosmetic item, not a defect.

**Required change: none in this task.** Route it to the same vehicle the
positional ordinals already have ("whichever package next edits `BUILDING.md`'s
gate blocks after Plan 10's Task 1 lands"), where a paragraph reflow is in scope
rather than fenced off.

### 3. Minor - `states 1 commands` is not pluralized

`scripts/ledger-lint.py:223`. Observed under my F5 reproduction:

```
FAIL BUILDING.md: gate-block 'house' states 1 commands but enumerates 2
```

The plan fences the message shape by example at `states 4 commands but
enumerates 5`, and the implementation reproduces that example byte for byte
(verified by extraction, appendix). Keeping the shape was the right call
(adjudication 2).

**Required change: none.** If the controller ever wants it, the minimal form is
`command{'' if n == 1 else 's'}`, and it is a one-line edit that changes the
fenced example's rendering for `n == 1` only.

### 4. Minor - the plan's five fires do not reach the counting rule's exclusion branch

Step 2 fixes "a counted COMMAND LINE is a line whose stripped form is non-empty
and does not start with `#`" (`scripts/ledger-lint.py:153`). No gate block in
`BUILDING.md` contains a standalone comment line or a blank line, and none of
F1-F5 or S1-S2 introduces one, so that branch never executed - the green run
would look identical if the exclusion were inverted.

I fired it, with its control:

```
=== Y1  standalone '#' comment line + blank line inside the frontend block ===
ledger-lint: 531 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
EXIT=0

=== Y2  control: a standalone line that is NOT a comment ===
FAIL BUILDING.md: gate-block 'frontend' states 4 commands but enumerates 5
FAIL BUILDING.md: gate-total states 11 parts but the three gate blocks enumerate 12
EXIT=1
```

Y1 green with two comment lines and two blank lines inserted, Y2 red with one
real command inserted. The behaviour is correct as specified.

**Required change: none.** Carried forward as a verification-design note for
Tasks 2-5 (HARVEST).

### 5. Minor - the shipped check cannot see a FOURTH gate section

`scripts/ledger-lint.py:69` fixes the block set to three named markers and
`:74-76` fixes the sentence to four numbers. A future package that adds a fourth
gate section with its own `<!-- gate-block: docs; ... -->` marker gets no
coverage: the new block is not in `GATE_BLOCKS`, so its commands are not
counted, and `6 + 4 + 1` still equals the stated `11` while the file enumerates
more. The check stays green on exactly the drift it exists to catch.

The likely-in-practice path is red rather than green - an author who adds a
section usually also edits the sentence, and any fifth number breaks the
four-group regex (fired: my X6 scenario) - but the silently-green path exists.

**Not an implementation defect.** Step 2 fixes the counting rule and the "Must
not decide" list names it as not the implementer's to alter; the implementer
transcribed it correctly. Surfaced for the controller as a coverage boundary of
the shipped check.

---

## The five adjudications

### 1. The 86-character line: correct fidelity, or in-scope structural conformance?

**Verdict: leaving it is correct fidelity. Do not reflow, in this task.**

Step 1(e) does not merely fence the replacement string; it carries an explicit
within-paragraph qualifier, and that qualifier names the very tokens a reflow
would move: "Nothing else in that paragraph changes - in particular its
'Rust-gate parts 1-4' and 'part 6' ordinals stay exactly as they are."
`latitude-carveout-zero-content-structural-forks` settles the rest: the
house-pattern grant "fills SILENCE only - an explicit enumeration in brief,
design or spec always wins over it," and the grant's named in-scope case is
"repairing a REFERENCE which the task's OWN enumerated edit invalidated (a doc
link, a comment referent, an import)". A line-length overflow is not a
reference. There is no silence here to fill.

The counter-case is real and worth naming rather than dismissing: the overflow
is a consequence of the fenced edit itself, it satisfies all four
zero-outward-effect conditions, and finding 2 shows it breaks an envelope the
file had held perfectly until now. That is an argument for widening the grant,
which is a controller decision on the over-restriction watch the same entry
invites - not a licence the implementer had. It decided nothing at the keyboard
and surfaced it as concern 1, which is the behaviour the boundary is designed to
produce.

### 2. `states 1 commands`: acceptable as fenced, or a defect the fence did not intend?

**Verdict: acceptable as fenced. Not a defect.**

The plan gives the message as a shape with an example: "in the existing `FAIL
{rel}: ...` shape, naming both numbers, e.g. `BUILDING.md: gate-block 'frontend'
states 4 commands but enumerates 5`". The implementation reproduces that example
character for character - verified by extracting the plan's string and comparing
it programmatically against the string my F2 reproduction emitted (EQUAL: True).

Adding pluralization would have been a construct the plan did not write, on a
string the plan did write, in a task whose "Must not decide" list is long and
whose Global Constraints forbid resolving anything at the keyboard. The grammar
wart is the cost of that discipline and is the cheaper error: an implementer who
improves fenced strings is a worse failure mode than one who transcribes an
awkward one. It also only renders at `n == 1`, which on a green tree never
happens.

### 3. The two supplementary fires: in scope as evidence, or unrequested extension?

**Verdict: in scope, and required - though the ground is not the one the concern
names.**

The asymmetry the brief flags is correct: `proc-proposed-safeguard-stays`
protects a proposed safeguard and does not mandate new ones, so it is not the
authority here. The authority is `proc-verification-step-must-be-falsifiable`,
which is explicit: "PER ASSERTION, not per script: a probe that exercises three
checks with one mutation leaves the unexercised ones unverified - the probe set
must hit every check whose absence is being trusted." The duplicate-marker
branch (S1) and the missing-opening-fence branch (S2) are absence-shaped checks
whose green state the task's green run trusts, and neither is reached by F1-F5.
Firing them is that entry's handle, not an extension of scope.

On the size concern: evidence is not an artifact. S1 and S2 produced no file, no
test, no persisted state; both restores are proven and my own blob-hash check
confirms the committed tree carries nothing from them. "A task the owner
approved at its current size" constrains deliverables, and the deliverable set
is unchanged.

Finding 4 is the honest completion of the same argument: the probe set should
have been one pair larger still. I fired the missing pair and it passes.

### 4. Step 1(f)'s surfacing duty against a set of THREE

**Verdict: yes on both halves. The task correctly edited none of the three, and
the third site's presence inside an edited paragraph is not a reason it should
have been handled differently within this task.**

My own enumeration, independent of the report:

```
$ grep -nE '\bpart[s]? [0-9]' BUILDING.md
102:The cross-target clippy run (part 6) type-checks the workspace for Windows
134:CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
135:three OS legs (its Windows leg covers natively what part 6 cross-checks
```

Three sites, and `git diff 754cb73..ddb8f42 -- BUILDING.md` carries all three as
context lines, never as changed lines. Step 1(f)'s two pre-edit citations are
also sound: `git show 754cb73:BUILDING.md | sed -n '95p;121p'` returns exactly
the two lines the plan quotes.

On the second half - the sharper one: `:135` sits inside the paragraph Step 1(e)
edits, so it is not merely un-enumerated by Step 1(f), it is affirmatively
fenced by Step 1(e), whose qualifier names "'part 6' ordinals" as staying
exactly as they are. Editing it would have violated a fence to satisfy an
enumeration. The gap was in Step 1(f)'s count, not in the handling, and the
correct treatment of an enumeration gap in a task forbidden to edit `ROADMAP.md`
is precisely what happened: surface it (report concern 3 plus the section 9
ledger candidate) and let the controller write. That routing worked - the
ROADMAP entry now reads "Three sites, not the two this entry named until
2026-07-29 (session 28)".

### 5. The heading rewrite's reach

**Verdict: the assertion holds at the current tree. No live consumer follows the
wording.**

```
$ git grep -n "The Rust gate\|the-rust-gate\|Rust gate" -- . \
    ':(exclude)docs/ROADMAP.md' ':(exclude)docs/process-journal.md' | wc -l
58
```

58 at `ddb8f42`, distributed as: `BUILDING.md` (the heading itself, 1), 45 under
`docs/process-journal/artifacts/`, 6 in four retired plan documents, 5 in the
Plan-10 document, and 1 new hit in `scripts/ledger-lint.py:66` - a comment this
task itself added, recording that the heading was reworded. Nothing outside that
set:

```
$ grep -vE '^(BUILDING\.md|docs/process-journal/artifacts/|docs/superpowers/plans/|\.superpowers/)' hits
scripts/ledger-lint.py:66:# gets reworded (the Rust gate's own heading was, by the change that added this
```

**On the plan's number 57, which does not reproduce as a bare count.** The same
expression at `de4ea38` returns 52, not 57. The reconciliation is exact and not a
defect: the Plan-10 document did not exist in that commit (`git cat-file -e
de4ea38:docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md` -> "exists
on disk, but not in 'de4ea38'") and carries 5 hits, so 52 committed + 5
working-tree = the plan's 57. The current 58 is that 57 plus the one comment
this task added. Every delta is accounted for.

The load-bearing half is not the count but the consumer question, and it holds
independently of it: the only hit outside history and plan documents is a prose
comment, no matching logic anywhere reads the heading (`scripts/ledger-lint.py`
anchors on the HTML-comment markers, and "heading" appears in it only inside
docstring and comment prose), and no anchor link targets it -
`git grep -nE '\]\([^)]*#the-rust-gate'` returns nothing, with the plan's own
fired control (`git grep -nE '\]\([^)]*#[a-zA-Z]' -- '*.md'`) returning 4 files,
so the expression demonstrably matches an anchor link when one exists.

---

## Evidence appendix

All instruments built fresh under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t1rev-independent/`.
No instrument the implementer wrote was re-run. **The tracked tree was never
mutated:** every fire ran against a sandbox copy of `BUILDING.md` and the script,
with `docs/` symlinked in so `REPO` resolves inside the sandbox.

| path | what it is |
|---|---|
| `fence_compare.py` | extracts each fenced text from the plan by line span and asserts the exact byte sequence in the target file |
| `fire.py` | mutation harness, twelve scenarios, sandbox-only, restore-and-reverify after each |
| `sandbox/` | copy of `BUILDING.md` + `scripts/ledger-lint.py`, `docs/` symlinked; baseline sha256 equal to the tree |
| `gate.log` | the eleven gate parts, each with its own exit code |
| `baseline-sha256.txt` | hashes taken before I ran anything |
| `rustgate-hits.txt` | the adjudication-5 corpus |
| `typo-fixture.txt` | fired control for the typography pattern |

### Contract compliance (dimension 1)

`fence_compare.py`, run from the repo root, all thirteen assertions pass:

```
sha256(step1a fence) = 0206ec8a66da1f681938d30908b9e14fbd6e649bca670909dfab1fe4c2587e30
[OK ] Step 1(a) gate-total marker + canonical sentence -> BUILDING.md (occurrences=1)
[OK ] Step 1(c) marker <!-- gate-block: rust; checked by ... -> BUILDING.md (occurrences=1)
[OK ] Step 1(c) marker <!-- gate-block: frontend; checked... -> BUILDING.md (occurrences=1)
[OK ] Step 1(c) marker <!-- gate-block: house; checked by... -> BUILDING.md (occurrences=1)
[OK ] Step 1(d) house command line -> BUILDING.md (occurrences=1)
sha256(step1d paragraph) = 55d589ba54e98760ed6f4c74774f4a33c8014b4e9cfdbe7483094deb9942607c
[OK ] Step 1(d) replacement paragraph -> BUILDING.md (occurrences=1)
[OK ] Step 1(b) new heading present -> BUILDING.md (occurrences=1)
[OK ] Step 1(b) old heading absent from BUILDING.md
[OK ] Step 1(e) new parenthetical -> BUILDING.md (occurrences=1)
[OK ] Step 1(e) old parenthetical absent from BUILDING.md
[OK ] Step 3 new step name -> .github/workflows/ci.yml (occurrences=1)
[OK ] Step 3 old step name absent from ci.yml

ALL PASS
```

The regex was compared the same way rather than by eye - plan string extracted
from Step 2, implementation string extracted from the source, `EQUAL: True`. The
summary line renders as the plan fences it:
`ledger-lint: 531 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold`,
with `531` and `4` recomputed independently of the script (531 entries carrying
an id, 531 unique). The docstring's first line no longer claims the four YAML
files only, and check 7 carries both required claims (the comparison and the
markers-because-heading-prose-is-not-stable ground).

### Re-measured enumeration (dimension 3)

My own marker-driven awk counter, structurally unlike both the script and the
report's `sed` spans:

```
rust	6
frontend	4
house	1
```

Sum 11. **Agrees with the sentence's `11 / 6 / 4 / 1` and with the plan's
authoring measurement.** Backslash scan over the whole file returns one line,
`158`, inside the release-reproduction block - which doubles as my own fired
control for the continuation pattern (it matches a real continuation elsewhere
in the file and nothing inside the three gate blocks).

### Fires (dimensions 2 and 4)

All twelve run under `fire.py`; each restores and reverifies green before the
next. F1-F5 reproduce the implementer's outputs exactly, including F5's `line
123`.

| id | scenario | violations | verdict |
|---|---|---|---|
| F1 | stated total 11 -> 12 | 1 (total mismatch) | matches W3-b |
| F2 | frontend block gains a command | 2 (frontend + total) | matches W3-c |
| F3 | gate-total marker deleted | **exactly 1** | matches W3-d |
| F4 | frontend block marker deleted | **exactly 1** | matches W3-e |
| F5 | house command backslash-continued | 3, **including** the continuation message | matches W3-f |
| X1 | frontend marker deleted **and** rust block gains a command | 2 - frontend missing, rust mismatch, **no total line** | block skip proven by construction |
| X2 | total marker deleted **and** frontend block gains a command | **exactly 1** - missing total marker only | total skip proven by construction |
| X3 | rust marker duplicated | 1, `found 2` | exactly-one rule holds |
| X4 | house opening ```bash fence removed | 1, quoting what was found | fence rule holds |
| X5 | house fence never closed **and** frontend gains a command | "never closed" reported, house and total comparisons skipped, frontend mismatch still named | unterminated-fence skip proven |
| X6 | sentence present but unparseable **and** rust gains a command | **exactly 1**, quoting the found line | every comparison skipped |
| X7 | gate-total marker duplicated | 1, `found 2` | exactly-one rule holds |
| Y1 | comment + blank lines inserted into a block | 0, **green** | exclusion branch fired |
| Y2 | control for Y1 - a real command inserted | 2 | exclusion is not blanket ignoring |
| Y3 | `BUILDING.md` deleted | 1, `file not found` | missing-file branch fired |

X1, X2, X5 and X6 are the point: they verify the SKIP logic **by construction**
rather than through a fire that walks into it, which is where a wrong
implementation would still look green. In X1 the total comparison is absent from
the output while a real block mismatch is reported alongside the missing marker,
which is only possible if the skip is deliberate.

### Verification quality (dimension 8)

The full gate, exactly as `BUILDING.md` now enumerates it, foreground, no
subsets, each part with its own exit code:

```
exit  command
0  cargo fmt --all --check
0  cargo clippy --workspace --all-targets -- -D warnings
0  cargo test --workspace
0  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
0  cargo deny check
0  cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
0  pnpm lint
0  pnpm build
0  pnpm check:i18n
0  pnpm test:e2e
0  python3 scripts/ledger-lint.py
```

Result lines: `advisories ok, bans ok, licenses ok, sources ok`;
`check-i18n: ok (41 source files scanned, 212 catalog ids, ...)`;
`68 passed (2.9s)`;
`ledger-lint: 531 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold`;
every `cargo test` binary `test result: ok ... 0 failed`. Working tree after the
gate: only the pre-existing `docs/ROADMAP.md` modification.

Report aggregates recomputed: `183 insertions, 9 deletions` across three files,
`BUILDING.md 18/5`, `scripts/ledger-lint.py 161/2`, `ci.yml 4/2` - all as
reported. The report's zsh note reproduces: `PIPESTATUS[0]=[]`,
`pipestatus[1]=[0]`, `SHELL=/usr/bin/zsh`. Its ci.yml width premise holds and is
weaker than reality - the file's longest pre-existing comment line is 120
characters at `:117`, so the new 76-character line at `:174` is well inside the
envelope.

Typography over the three touched files: no hits, with a fired control I built
during this review matching all seven denied classes (em-dash, en-dash, both
curly quote pairs, ellipsis, NBSP, Unicode minus).

### House dimension (6)

| id | assessment |
|---|---|
| `ledger-lint-runs-before-every-push` | held. The check landed **inside** the existing gate part, the gate still enumerates eleven commands, and the entry's own "the block now enumerates eleven commands" is untouched and still true - verified by my recount, not carried from the report |
| `latitude-carveout-zero-content-structural-forks` | **not over-reach.** Both named-region repairs (`ci.yml`'s job comment and step name, `BUILDING.md`'s CI parenthetical) are explicitly enumerated in Task 1's Files list, and an explicit enumeration wins over the grant rather than needing it. The plan's own "Why the two extra regions are in scope" paragraph measures all four self-descriptions. No stop was forced and no finding had to be filed |
| `proc-normative-count-recomputed` | held on the artifact, missed once in the report. Trigger 2 applies (a member joined the docstring's check enumeration): I swept for any count-word describing ledger-lint's check set outside the docstring and found none, with a fired control proving the pipeline finds a count-word that does exist. Finding 1 is the one place a stated number does not survive recomputation |
| `proc-verification-step-must-be-falsifiable` | held, and it is the entry that authorizes S1/S2 (adjudication 3). Finding 4 is the residual gap, now closed by my Y1/Y2 pair |
| `proc-check-green-state-reachable` | held. The green state is reached on the intended end state, not the pre-state, and Y1 additionally shows green is reachable in the presence of comments and blank lines the counting rule promises to tolerate |
| `proc-latitude-clause-boundary` | held on every fenced item. The one soft edge is plan-side, not implementer-side: Step 2 fences one violation message by example and leaves five others to composition. See HARVEST |

### Latitude, both forms (dimension 5)

Nothing was decided at the keyboard that belonged in a NEEDS_CONTEXT memo. The
pre-routed recount fork did not fire, and my independent recount confirms why -
`6 / 4 / 1` is correct at the current tree, so the fence was rightly transcribed
unchanged. The helper decomposition (`_next_non_empty`, `_count_block_commands`)
is composition, which the plan's own model-tier row assigns to this task ("the
counting rule is fixed, the integration into an existing script is composed").

The inverse form is also clean: nothing the plan had settled was returned or
omitted. All five fires ran, all fenced text was applied, and Step 1(f)'s
non-edit was honoured and surfaced rather than quietly taken.

### The no-work-needed check (dimension 7)

Three "unnecessary / cannot happen" conclusions were run rather than weighed.

- Report section 3: "The one shape still unfired by construction is the
  never-closed fence, which cannot be produced without also destroying a later
  block's structure." **The premise holds** - my X5 could only produce it by
  removing every bare closing fence after the house marker, which does destroy
  the release-reproduction block. The path nevertheless behaves correctly when
  the state is constructed, which is now measured rather than argued.
- Report concern 1: "reflowing the paragraph would have edited lines the plan
  fenced off." **Premise holds**, and it is stronger than the report puts it -
  Step 1(e)'s qualifier names the specific ordinals a reflow would move.
- Report section 7.2: "76 is inside the file's existing width envelope."
  **Premise holds**; measured max is 120.

### Blast radius (dimension 9)

The new check cannot fail on a tree Tasks 2-5 legitimately produce. It reads
`BUILDING.md` and nothing else, and none of Tasks 2-5 touches that file
(`crates/muxsmith-core/tests/report_json.rs`, `renovate.jsonc`, `README.md`,
`docs/INSTALL.md`, source comments).

The pre-existing YAML checks are unchanged in behaviour. Every deletion in the
script's diff, in full:

```
$ git diff 754cb73..ddb8f42 -- scripts/ledger-lint.py | grep -E '^-[^-]'
-"""Structural integrity check for the four house-knowledge YAML files.
-    print(f"ledger-lint: {total} entries across {len(FILES)} files, all invariants hold")
```

Two lines, both self-description. Checks 1-6 and the `DuplicateKeyLoader` are
byte-untouched; the only structural change to `main()` is one added call at
`:302`.

**One cross-task interaction I checked because nobody had:** Task 1 added 161
lines to a `.py` file, and Task 5's corpus is computed at execution time over
`*.py` among others. Neither of Task 5's two expressions matches anything in
`scripts/ledger-lint.py`, and the corpus is still 20 lines across 13 files for
expression A, exactly the plan's authoring measurement. Both absence claims
carry a fired control (expression A returns 20/13 over the corpus; expression B
returns the known hits in `profile_save.rs:95` and `registries.ts:12`).

### Tree proof

```
$ git status --porcelain
 M docs/ROADMAP.md

$ git diff ddb8f42 --stat
 docs/ROADMAP.md | 12 ++++++++++--
 1 file changed, 10 insertions(+), 2 deletions(-)

BUILDING.md
  worktree 911fdba996fcafecbd736f92c1820ec0078d8144
  ddb8f42  911fdba996fcafecbd736f92c1820ec0078d8144
scripts/ledger-lint.py
  worktree a20bf89e5d0c098150cb23defd06d05f31f3edaa
  ddb8f42  a20bf89e5d0c098150cb23defd06d05f31f3edaa
.github/workflows/ci.yml
  worktree 278bc545d5d813973b35cf5cc34b22f3e3dabbc6
  ddb8f42  278bc545d5d813973b35cf5cc34b22f3e3dabbc6

$ git status --porcelain --untracked-files=all | grep '^??'
(none)
```

The three task files are blob-identical to `ddb8f42`. Their sha256 values also
equal the baseline I recorded before running anything. `docs/ROADMAP.md` carries
the same sha256 as that baseline, so this review did not touch it either. No
untracked file was created inside the repository; every instrument lives in the
scratchpad.

---

## HARVEST

**Observed dominant patterns.**

- **Fenced-text discipline paid off completely.** Thirteen independent
  byte-for-byte assertions, zero deviations, including the one place where
  fidelity produced an ugly result (`states 1 commands`) and the one place where
  it produced a cosmetic regression (the 86-character line). Both were surfaced
  as numbered concerns instead of being improved. That is the behaviour the
  fencing regime exists to buy, and Tasks 2-5 should be held to the same bar -
  Task 3's `renovate.jsonc` is the next fully fenced file.
- **The implementer surfaced rather than decided, three times, and every surface
  was correct.** Concern 3 in particular found a real enumeration gap the plan
  author and the ROADMAP both carried, and routing it produced a controller
  correction the same session. The mechanism worked end to end.
- **Absence-shaped checks remain the highest-yield review target.** The task
  specified five fires, the implementer added two, and I still found an
  unexercised branch (finding 4). The pattern is stable across this project:
  whoever writes the check under-enumerates its branches, because the branches
  that never fire on a green tree are invisible from inside the change.

**Repeated rejections.**

- No design decision was re-opened. The recount fork did not fire, and I confirm
  the fence was right to transcribe.
- The "reflow it anyway" and "pluralize it anyway" temptations were both
  correctly declined; both are recorded above as controller decisions, not as
  latitude.

**What the remaining four tasks must carry.**

1. **`${PIPESTATUS[0]}` is empty in this shell.** Confirmed under my own hand:
   `PIPESTATUS[0]=[]`, `pipestatus[1]=[0]`, `SHELL=/usr/bin/zsh`. Tasks 2-5 all
   run gate parts and Task 2's four-mutation measurement depends on exit codes.
   Use `cmd >log 2>&1; echo $?`.
2. **Per-branch fire discipline, not per-check.** `proc-verification-step-must-be-falsifiable`
   says "PER ASSERTION, not per script". Task 2's four mutations and Task 5's two
   absence expressions are the same shape. Task 5 in particular needs a fired
   control for BOTH expressions, and expression B needs the per-file form - the
   naive one-pipeline version matches its own `file:line:` prefix, which is how
   an earlier measurement came back empty.
3. **A control that proves green is reachable, not only that red is.** My Y1/Y2
   pair is the template: mutate toward the thing the rule must TOLERATE and
   require green, then mutate toward the thing it must CATCH and require red. A
   single red fire cannot distinguish a working check from one that fails on
   everything.
4. **The gate now includes a check over `BUILDING.md`.** No remaining task edits
   that file, so this should be invisible. If any task's gate run reports a
   `FAIL BUILDING.md: ...` line, something wrote to `BUILDING.md` outside its
   Files list - treat it as a defect signal and NEEDS_CONTEXT, not a local fix.

**Surfaced for the controller (house-knowledge candidates; I do not write).**

- **Coverage boundary of the shipped check (finding 5).** The block set is fixed
  at three named markers and the sentence at four numbers, so a future FOURTH
  gate section carrying its own marker is invisible to the check and can drift
  silently. Worth a line in the ROADMAP's gate-count entry so the next author of
  a gate section knows the check will not catch them.
- **The 86-character line (finding 2)** wants the same vehicle as the positional
  ordinals - "whichever package next edits `BUILDING.md`'s gate blocks after
  Plan 10's Task 1 lands". It is a two-line reflow with zero rendered effect.
- **`latitude-carveout-zero-content-structural-forks`, over-restriction watch,
  calibration datum in the DO-NOT-loosen direction.** The boundary stopped
  exactly one thing in this task: a purely typographic consequence of the
  fenced edit itself. I agree with the implementer that this should stay
  stopped - the case is a hair's breadth from "reflow a paragraph the plan
  fenced", and the routed-item vehicle already exists to absorb it. Recording it
  as evidence that the tight setting is behaving, not as a proposed loosening.
- **Plan-authoring observation, `proc-latitude-clause-boundary`.** Step 2 fences
  one violation message by example and leaves five others (missing marker, no
  opening fence, unterminated fence, continuation, unparseable sentence) to be
  composed. Under a strict reading, "a step that requires inventing a string" is
  a fork. It is a benign one here - the messages are developer-facing lint
  output, the plan specifies each one's semantic content, and returning
  NEEDS_CONTEXT per string would be absurd - but a future plan that fences
  diagnostic output should either fence all of it or say explicitly that the
  wording is composed. Candidate for a brief-authoring note rather than a new
  entry.
