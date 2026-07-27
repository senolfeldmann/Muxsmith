# Task 5 report - RIDER: ledger-lint duplicate-key extension + CI wiring

**Stream D**, worktree `/home/senol/Git/Muxsmith/.worktrees/plan8-d`, branch `plan8-d`.
**Status: DONE.** Commit `92c62f1` (single commit, unsigned, repo trailer).

Read before any edit: `implementer-preamble.md`, `task-5-brief.md`, the two
ROADMAP "Ledger hygiene" rider rulings (`docs/ROADMAP.md:554-561` duplicate-key
gap incl. TRIGGER FIRED AND CONSUMED 2026-07-22 S22; `docs/ROADMAP.md:592-617`
CI-wiring deferral incl. the S22 controller ruling), `scripts/ledger-lint.py` in
full, and D83 with its rationale and its rejected alternative
(`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:715-758`).
No fork encountered; nothing decided at the keyboard.

## Files changed (exactly the two the brief names)

- `scripts/ledger-lint.py` - check 6 (duplicate keys) + docstring sweep.
- `.github/workflows/ci.yml` - one appended `ledger-lint` job, additive only.

`git diff master --stat`: 2 files, 65 insertions, 3 deletions (the 3 deletions
are the 2 replaced docstring lines + the replaced `yaml.safe_load` line, all in
the .py; ci.yml is 17/0 - see Step 5).

## Step 1: check 6 in `scripts/ledger-lint.py`

Implemented per the closed contract:

- **Mechanism**: `class DuplicateKeyLoader(yaml.SafeLoader)` overriding
  `construct_mapping` (PyYAML's documented extension point). The pre-pass walks
  the raw mapping node's `node.value`, constructs each key node, and records
  `(key, first line, duplicate line)` from `key_node.start_mark.line + 1`; then
  delegates to `super().construct_mapping(...)`. Duplicates are **collected, not
  raised**, so one run reports every duplicate in the file. Unhashable keys (a
  mapping/sequence used as a key) are skipped via `TypeError`. No regex, no line
  heuristic - the docstring's real-parser principle holds.
- **Load path**: `yaml.safe_load(...)` replaced by an explicit
  `DuplicateKeyLoader(text)` + `get_single_data()` in `try/except
  yaml.YAMLError/finally loader.dispose()`, so the loader instance survives for
  its `duplicate_keys` list (`dispose()` only resets parser state). File-read is
  now its own `try` for the pre-existing `FileNotFoundError` violation, keeping
  the two error violations byte-identical to before.
- **Scope**: every mapping at any depth in all four files (superset of the
  ROADMAP's per-entry requirement; simpler than entry-scoping the loader).
- **Violation format**: `FAIL docs/process-conventions.yaml: duplicate key
  'steelman' (lines 48 and 49)`, appended to the same `violations` list, so
  summary line and exit semantics (0 clean / 1 on any violation) are untouched.
- **Docstring maintenance in the same edit**: numbered list gained item 6 (dash
  aligned with items 3/5, continuation indent matching item 1); the stale
  deferral sentence replaced by "CI wiring exists: the ci.yml `ledger-lint` job
  runs this script on every push and pull request (Plan 8 rider, ROADMAP
  'Ledger hygiene' ruling 2026-07-22)."
  Sweep verified: `grep -n "CI wiring" scripts/ledger-lint.py` -> exactly one
  hit, line 30, the new sentence.

## Step 2: fire-test (foreground, verbatim outputs)

Baseline before any fixture:

```
ledger-lint: 452 entries across 4 files, all invariants hold
exit=0
```

**RED (new check)** - duplicated the `  steelman: null` line (line 48) inside
entry `proc-02-whole-branch-review` of `docs/process-conventions.yaml`, i.e. the
observed S21 shape:

```
FAIL docs/process-conventions.yaml: duplicate key 'steelman' (lines 48 and 49)

ledger-lint: 1 violation(s) across 452 entries
exit=1
```

`git checkout -- docs/process-conventions.yaml` restored.

**CONTROL (old checks still live after the loader swap)** - bumped
`docs/conventions.yaml` line 32 `count: 3` -> `count: 4`:

```
FAIL docs/conventions.yaml: core-03-suggestion-verified-edit: count is 4 but has 3 occurrences

ledger-lint: 1 violation(s) across 452 entries
exit=1
```

`git checkout -- docs/conventions.yaml` restored.

**GREEN (reachable)** - identical to baseline:

```
ledger-lint: 452 entries across 4 files, all invariants hold
exit=0
```

**Extra fire-test beyond the brief** (two claims of mine that the brief's three
runs do not cover: the "any mapping at any depth" scope I wrote into the class
docstring, and "checks 1-5 unchanged" for checks 2-5). One combined fixture:
duplicate `date:` inside a flow mapping on `docs/process-conventions.yaml:40`,
plus in `docs/conventions.yaml` a duplicated id, `status: blocked` on an entry
with `blocked_on: null`, `tier: 2` with `promoted_at: null`, and an occurrence
with its `ref` field removed:

```
FAIL docs/conventions.yaml: core-03-suggestion-verified-edit: occurrence #0 has no ref
FAIL docs/conventions.yaml: core-03-suggestion-verified-edit: status blocked without a blocked_on
FAIL docs/conventions.yaml: core-03-suggestion-verified-edit: tier 2 but promoted_at is null
FAIL docs/conventions.yaml: id 'core-03-suggestion-verified-edit' also defined in docs/conventions.yaml
FAIL docs/process-conventions.yaml: duplicate key 'date' (lines 40 and 40)

ledger-lint: 5 violation(s) across 451 entries
exit=1
```

All five old/new checks fire; the nested flow mapping is caught (both marks on
line 40 because the mapping is a single line - honest output, not a defect).
Both files restored via `git checkout --`; `git status --short` showed only the
two intended modifications throughout.

Fixture mutations were applied with self-asserting `python3` heredocs (each
asserts the exact target line before touching it) so a shifted file aborts
instead of silently mutating the wrong line.

## Step 3: `ledger-lint` job appended to `.github/workflows/ci.yml`

The brief's block, verbatim, appended after the `deny` job with one blank line
separating them (house style between jobs). No existing line touched. The
workflow still parses and carries the intended job set:

```
['test', 'deny', 'ledger-lint']   # yaml.safe_load over ci.yml
ubuntu-26.04                      # jobs['ledger-lint']['runs-on']
```

Pin/mechanism rationale is as recorded in the brief (PyYAML==6.0.3 registry
latest 2026-07-23 and the local install's version - locally measured
`yaml.__version__` = 6.0.3; stdlib venv as the first-party mechanism per
`deps-first-party-pinned-over-convenience`, decision-ledger.yaml:4068; runner
label matches the file's existing jobs; `ci-10-pin-everything`,
process-conventions.yaml:232).

## Step 4: the job's exact step commands, locally

```
Collecting PyYAML==6.0.3
  Using cached pyyaml-6.0.3-cp314-cp314-manylinux...whl.metadata (2.4 kB)
Using cached pyyaml-6.0.3-...whl (794 kB)
Installing collected packages: PyYAML
Successfully installed PyYAML-6.0.3
[notice] A new release of pip is available: 26.0.1 -> 26.1.2
ledger-lint: 452 entries across 4 files, all invariants hold
exit=0
```

Pinned install succeeds in a throwaway venv, script green, exit 0. The in-CI red
state is deliberately not exercised (brief's recorded reasoning); the in-CI GREEN
is the controller's post-merge-push observation.

## Step 5: additive-only property (D83-compat observable)

```
$ git diff master --numstat -- .github/workflows/ci.yml
17	0	.github/workflows/ci.yml
```

Exactly one line, deletions column 0 -> pure addition. Re-checked after the
commit against the committed state: same `17 0`.

**Fire-verified** (`proc-verification-step-must-be-falsifiable`,
process-conventions.yaml:408; idiom per `ci-additive-only-check-numstat`,
decision-ledger.yaml:4140): edited one existing ci.yml comment line in place
(line 158, appended " (FIRE TEST)"), re-ran the same command:

```
18	1	.github/workflows/ci.yml
```

Deletions went nonzero as the ledger entry predicts for an in-place edit. Then
restored and re-verified `17 0` plus `grep -c "FIRE TEST" .github/workflows/ci.yml`
-> `0`.

Incident worth recording: the restore step used `cp` inside the same compound,
`cp` is aliased interactively on this machine, the prompt blocked and the tool
call hit its 2-minute timeout with ci.yml left in the mutated state. Restored
deterministically from the scratchpad backup via `python3` (asserting the
"FIRE TEST" string is absent from the backup before writing), backup removed.
No residue: post-restore numstat and grep above, and the commit contains no
fire-test text.

## Step 6: commit

```
92c62f1 ci: ledger-lint job (Plan 8 rider, S22 ruling) + per-entry duplicate-key check via SafeLoader subclass; fixture fire-verified
```

Staged explicitly (`git add scripts/ledger-lint.py .github/workflows/ci.yml`,
never `-A`), `-c commit.gpgsign=false` (`%G?` = `N`), trailer
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present. Working tree
clean afterwards. All git invocations were pure git via `git -C <abs worktree
path>` - no `cd` segment chained in, no session-relocation tool used at any
point.

## Self-review

- **Contract conformance**: mechanism (SafeLoader subclass via
  `construct_mapping`), scope (any mapping, any depth), violation format
  (file + key + both 1-based lines), aggregation/exit semantics, the S21 shape
  caught, docstring swept, checks 1-5 unchanged - each verified by an executed
  run above, not by inspection alone.
- **Known boundary, recorded**: the pre-pass reads `node.value` before
  `SafeConstructor.construct_mapping` applies `flatten_mapping`, so a YAML merge
  key (`<<`) would be seen as a literal key. The four files contain no anchors,
  aliases or merge keys (grepped), so this is inert today; if merges ever enter
  the house files, a duplicated `<<` would be reported (arguably correct) and a
  merge-shadowed explicit key would not (correct - override is intentional). Not
  documented in the script: speculative commentary for a construct the files do
  not use.
- **Typography**: ASCII hyphens, straight quotes, no Unicode ellipsis in both
  changed files (the ci.yml block is the brief's text verbatim).
- **No scope creep**: no new dependency, no `v*` tag, no release touched, no
  README placeholder touched, no product code path touched. The nine-part gate
  is unaffected by construction (no Rust/TS/locale file changed); running it is
  the controller's pre-push/post-merge action per the preamble.
- **Sweep for consuming lines** (`proc-normative-count-recomputed`): the only
  place outside the script that enumerates its checks is `docs/ROADMAP.md:626`
  ("All 386 entries across the four files now pass the four checks above"), a
  dated 2026-07-16 statement inside its own entry, scoped to that entry's four
  checks - a historical record, not a live count over the new set, so it is not
  stale by this change. The two rider bullets already record TRIGGER FIRED AND
  CONSUMED. Archived `docs/process-journal/artifacts/handoffs/*` mention the
  duplicate-key gap as open; those are dated journal artifacts and are not
  retro-edited. No ROADMAP or journal edit made - the brief's Files list is two
  files, and the ROADMAP disposition is a controller/plan-close action.

## Concerns

None blocking. Two notes for the controller:

1. The ROADMAP's two "Ledger hygiene" rider bullets still read as open work
   (they record the triggers as fired and consumed, but not the rider as
   landed). Closing them is a plan-close controller action; out of this task's
   Files scope.
2. The job's Python interpreter floats with the runner image by design (only
   PyYAML is pinned, as the brief's comment states). PyYAML 6.0.3 ships
   manylinux wheels broadly, so this is low risk; a future image bump to a
   Python version without a matching wheel would fall back to a source build.
   Recorded, not acted on.
