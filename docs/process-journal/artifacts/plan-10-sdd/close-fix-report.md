# Plan-10 close fix wave - implementer report

**Status:** DONE (3/3 findings landed), with two numbered concerns, neither
blocking. **Commit:** `1805949` `docs+comments: the whole-branch review's three
close repairs` - unsigned, three files by explicit pathspec, one trailer. Not
pushed.

Worked on `master` in `/home/senol/Git/Muxsmith`, no branch, no worktree, all
runs foreground. Read the findings from
`docs/process-journal/artifacts/plan-10-sdd/whole-branch-verdict.md`, not from
the brief alone.

---

## Finding 1 - the exit-code pronoun (`README.md`)

Applied the fixed repair verbatim: `Interrupt any of them with Ctrl-C` ->
`Interrupt any subcommand with Ctrl-C`. Rest of the sentence untouched, register
unchanged.

Shipped text re-read out of the commit, not out of my edit:

```
$ git show HEAD:README.md | grep -o 'Interrupt any subcommand with Ctrl-C'
Interrupt any subcommand with Ctrl-C
```

## Finding 2 - the two ragged wraps

Both sites named by the verdict were present as the *only* short non-final lines
in their comment blocks, and both are gone.

Method: greedy re-fill of each comment paragraph at its own block's fill width,
words treated as atomic, no wording change. **The fill width was derived from the
blocks, not assumed** - a greedy fill at 75 reproduces the pre-existing lines
byte-identically up to the ragged one (9 identical leading lines in
`smoke.spec.ts`, 6 in `EditorView.vue`), which is what establishes 75 as the
surrounding style rather than my choice.

Acceptance, measured against `HEAD` by a script independent of the one that
produced the edit (extracts both block versions, whitespace-normalizes, compares
word lists):

```
e2e/smoke.spec.ts:
  HEAD words=121 WORKTREE words=121 identical=True
  short (<40 col) non-final lines  HEAD=[1437]  WORKTREE=[]
  max col WORKTREE=75
src/views/EditorView.vue:
  HEAD words=137 WORKTREE words=137 identical=True
  short (<40 col) non-final lines  HEAD=[87]  WORKTREE=[]
  max col WORKTREE=75
```

Word sequence byte-identical, the two fragments the verdict named are the two
that disappeared, nothing exceeds the fill width. Both files lost one line
(15->14, 16->15), which is the reflow absorbing the fragment.

Pre-existing intra-token break at `EditorView.vue` lines 82/83
(`editor-rule-` / `select">`) left alone: it is outside the two sites the verdict
names and predates this wave.

## Finding 3 - the verdict figure (`README.md`)

**219 -> 225.** Unit wording untouched, no range, the sentence's own unit kept.

All four commands re-run at the final tree state (`HEAD` = `b37eac1` at
measurement time, see concern 1), pasted from the runs that produced them.

**The figure, by the package's established command:**

```
$ git ls-files 'docs/*' | grep -icE '/[^/]*verdict[^/]*$'
225
$ git ls-files 'docs/*' | grep -iE '/[^/]*verdict[^/]*$' | wc -l
225
```

Counted twice deliberately: `grep -c` counts lines, not matches (the house's own
ledgered instrument slip), so `wc -l` on the list is the confirming instrument.

**Property 1 - every match is markdown:**

```
$ git ls-files 'docs/*' | grep -iE '/[^/]*verdict[^/]*$' | grep -v '\.md$'
P1 exit=1 (1 = empty); control (same list, .zzz instead of .md): 225
```

Empty. The control is the fire test the empty result needs: swapping `.md` for a
suffix nothing carries returns all 225, so the expression does select the list
and the emptiness is a property of the list, not of a malformed pattern.

**Property 2 - no review BRIEF is caught:**

```
$ git ls-files 'docs/*' | grep -iE '/[^/]*verdict[^/]*$' | grep -i 'brief'
P2 exit=1 (1 = empty); control (brief across all of docs/): 202
```

Empty. Control: the same `brief` filter over all of `docs/` returns 202, so the
filter works and the verdict list genuinely contains none of them.

**Property 3 - both readings of the unit converge (the gating property):**

```
$ git ls-files 'docs/*' | grep -icE '/[^/]*verdict[^/]*$'    # verdict in the FILE NAME
225
$ git ls-files 'docs/*' | grep -ic 'verdict'                  # verdict ANYWHERE in the path
225
$ diff <(basename-reading | sort) <(fullpath-reading | sort)
diff exit=0   (identical sets, empty difference both directions)
```

**Converged.** Equal counts *and* an empty set difference, so the agreement is
set identity, not a coincidence of two equal numbers. No NEEDS_CONTEXT on this
finding.

**Why the value moved, verified rather than assumed** - the six new members are
exactly the salvage's verdict files, matching the verdict's prediction:

```
$ comm -13 <(git ls-tree -r --name-only 80e5c19 -- docs | grep -iE '/[^/]*verdict[^/]*$' | sort) <(current | sort)
docs/process-journal/artifacts/plan-10-sdd/task-1-verdict.md
docs/process-journal/artifacts/plan-10-sdd/task-2-verdict.md
docs/process-journal/artifacts/plan-10-sdd/task-3-verdict.md
docs/process-journal/artifacts/plan-10-sdd/task-4-verdict.md
docs/process-journal/artifacts/plan-10-sdd/task-5-verdict.md
docs/process-journal/artifacts/plan-10-sdd/whole-branch-verdict.md
```

Pre-salvage count at `80e5c19` measured 219, which independently reproduces the
figure the README shipped. 219 + 6 = 225.

Shipped text re-read out of the commit:

```
$ git show HEAD:README.md | grep -o '225 files under `docs/` with `verdict` in the name'
225 files under `docs/` with `verdict` in the name
```

---

## The gate

**The full eleven-part gate as `BUILDING.md` enumerates it (6 Rust, 4 frontend,
1 house), foreground, green, before the commit.** Run twice end to end; the
second run is the authoritative one because the tree changed under me between
them (concern 1). Both runs: every part exit 0, captured individually.

```
===== PART  1: cargo fmt --all --check                                        ===== EXIT=0
===== PART  2: cargo clippy --workspace --all-targets -- -D warnings          ===== EXIT=0
===== PART  3: cargo test --workspace                                         ===== EXIT=0
===== PART  4: RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
                --document-private-items                                      ===== EXIT=0
===== PART  5: cargo deny check                                               ===== EXIT=0
===== PART  6: cargo clippy --workspace --all-targets
                --target x86_64-pc-windows-msvc -- -D warnings                ===== EXIT=0
===== PART  7: pnpm lint                                                      ===== EXIT=0
===== PART  8: pnpm build                                                     ===== EXIT=0
===== PART  9: pnpm check:i18n                                                ===== EXIT=0
===== PART 10: pnpm test:e2e            (68 passed (3.0s))                    ===== EXIT=0
===== PART 11: python3 scripts/ledger-lint.py                                 ===== EXIT=0
             ledger-lint: 544 entries across 4 files plus BUILDING.md's gate
             enumeration, all invariants hold
```

**No `FAIL BUILDING.md:` line** - and the instrument behind that negative is
verified rather than trusted. My first grep for the literal string
`FAIL BUILDING.md` in the checker returned *nothing*, which would have made the
negative worthless; reading the source shows why and confirms the shape is real:
violations print through `print(f"FAIL {v}")` in `ledger-lint.py`'s `main`, and
every violation string appended by `check_building_gate_total` starts
`f"{BUILDING}: ..."` with `BUILDING = "BUILDING.md"`. So `FAIL BUILDING.md:` is exactly what a write to
that file outside this wave's Files list would emit; part 11 printed the success
line instead.

## `git diff --stat`

Exactly the three files in the Files list, nothing else:

```
 README.md                |  4 ++--
 e2e/smoke.spec.ts        | 11 +++++------
 src/views/EditorView.vue | 19 +++++++++----------
 3 files changed, 16 insertions(+), 18 deletions(-)
```

Working tree clean after the commit.

## Standing rules

- **No new `file:line` citation.** All **16** added lines of commit `1805949`
  (README 2, `smoke.spec.ts` 5, `EditorView.vue` 9) scanned for
  `<name>.<ext>:<digits>` across the tree's source extensions: no hits. Control:
  the same pattern's filename half matches 2 of those lines (the added text does
  carry `ListWidget.vue` and `mount.ts`), so the expression reaches real
  filenames here and only the `:<digits>` half comes back empty. The two reflows
  move breaks only, so no citation can be introduced by construction.

  *Correction, recorded because it is the exact failure class this package
  ledgers:* an earlier draft of this line said "49 added lines". That count was
  taken with `git diff` while the concurrent writer of concern 1 had in-flight
  lines in the same working tree, so it measured a tree that was not my commit.
  Re-measured against the commit itself: 16, and the breakdown sums to it. The
  scan's verdict is unchanged - the 49-line set was a superset of my 16 - but
  the number was wrong and is corrected rather than quietly dropped.
- **Typography.** The added lines scanned for em/en dash, the typographic
  dashes and Unicode minus, smart quotes, `...` ellipsis and NBSP: no hits, with
  a control confirming the pattern fires on a known-present sample.
- **No house-knowledge YAML, `docs/ROADMAP.md` or `docs/process-journal.md`
  edited by me** - see concern 1 for a change to `ROADMAP.md` that was not mine.
- **Comment-only in the two source files, no code, no markup, no test logic** -
  visible in the diff; the gate's green is therefore meaningful evidence.
- **No design latitude taken.** Finding 1 verbatim, finding 2 words unchanged,
  finding 3's unit unchanged.

---

## Concerns

1. **The brief's "you are the only writer in this tree" premise did not hold;
   three controller commits landed under me mid-run.** At my session start
   `git status --porcelain` was empty and `HEAD` was `af1fa7b`. During my first
   gate run `docs/ROADMAP.md` appeared modified (50 lines of plan-10 disposition
   prose, `mtime 17:56:29`, after gate part 10 finished at `17:56:05`). It is
   genuine controller work, not corruption - it cites `35bc363`, `1a23283` and
   `ddb8f42`, all real task commits in this repo - and by the time I re-checked,
   `HEAD` had advanced `af1fa7b -> d5382e7 -> 90fae0e -> b37eac1`, one of which is
   literally titled *"house: the controller's own concurrent-commit near-miss at
   this close"*, so the controller saw it too.

   **Impact assessed, not waved off.** (a) `git diff --stat af1fa7b..HEAD` shows
   those commits touched only `docs/ROADMAP.md` and `docs/decision-ledger.yaml` -
   none of my three files. (b) No gate part reads `docs/ROADMAP.md`:
   `ledger-lint.py`'s inputs are the four house YAMLs plus `BUILDING.md`, and no
   other script or config references the path. (c) I nevertheless re-ran all
   eleven parts on the final tree, and (d) re-ran all four finding-3
   measurements at the new `HEAD` - the figure holds at 225 under both readings
   with an empty set difference, because neither commit added or removed a
   verdict-named file. (e) Staging was by explicit pathspec, so the controller's
   work is not in my commit.

   Flagged rather than fixed because the resolution is the controller's: the
   handling here happened to be safe, but a fix wave and a controller writing
   the same tree concurrently is a race that explicit pathspecs only partly
   contain - a concurrent write to one of my three files would have been silently
   folded into my commit.

2. **Finding 3 is a treadmill, and this commit re-arms it rather than
   disarming it.** The verdict says so at finding 3, and the mechanism is
   visible in this very report: the figure was measured true three times inside
   the package and falsified by the package's own close. The count is now 225,
   and plan 11's salvage will falsify it again the moment it lands. I did not
   change the phrasing, correctly - the brief fences the unit and the verdict
   records the durable fix (growth-marked phrasing) as the owner's register
   call. Recording it so the decision is not lost between plan closes: **the
   owner still has to rule on growth-marked phrasing, or someone re-measures
   this sentence at every future close forever.**

No NEEDS_CONTEXT items. The unit's two readings converged, so the one condition
that would have forced one did not fire.
