# Task A1 implementer report - Plan 11, stream A (W5)

**Status:** DONE_WITH_CONCERNS
**Commit:** `a0d5d3e28d44ee464dd9175ea11e5446e9b0dc0c` on `plan-11-stream-a`
**Worktree:** `/home/senol/Git/muxsmith-plan11-a`
**Deliverable:** `BUILDING.md` loses its three positional gate ordinals and its
one over-80 non-fenced prose line, in one edit.

The two concerns are process defects found in the plan and in this brief, not
defects in the deliverable. Both are in section 6. Neither changed a single
character of what was applied: both fenced replacements went in verbatim.

---

## 1. Step 1 - re-measurement before editing

Both expressions were run from the plan's own characters (extracted
programmatically from the plan's fenced blocks, not retyped), in the worktree,
before any edit.

### Tooling note, recorded because the house has been bitten by it

`grep` in this shell is a function resolving to **ugrep 7.5.0**, not GNU grep:

```
$ type grep
grep is a shell function from /home/senol/.claude/shell-snapshots/snapshot-zsh-1785416419261-f6ak96.sh
$ grep --version | head -2
ugrep 7.5.0 x86_64-pc-linux-gnu +sse2; -P:pcre2jit; -z:zlib,bzip2,zstd,brotli,7z,tar/pax/cpio/zip
License: BSD-3-Clause; ugrep user manual: <https://ugrep.com>
```

Every negative result below was cross-checked against `/usr/bin/grep` and/or
`-P` where it carried a claim (section 3.1).

### 1.1 Expression O, pre-state

```
$ grep -nE 'part [0-9]|parts [0-9]' BUILDING.md
102:The cross-target clippy run (part 6) type-checks the workspace for Windows
134:CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
135:three OS legs (its Windows leg covers natively what part 6 cross-checks
exit=0
```

**Three lines: `:102`, `:134`, `:135`.** Identical to the authoring run.

### 1.2 The fence-aware length pass, pre-state

```
$ python3 - <<'EOF'
lines = open('BUILDING.md').read().split('\n')
fence = False
for i, l in enumerate(lines, 1):
    if l.startswith('```'):
        fence = not fence
        continue
    if fence:
        continue
    if len(l) > 80:
        print(f'{i}: len={len(l)}  {l!r}')
EOF
138: len=86  'and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)'
exit=0
```

**One line: `:138`, 86 characters.** Identical to the authoring run.

**Verdict on the decisive question: the re-measurement reproduces the authoring
run exactly, so the fenced replacements apply as written.** No hit outside the
fenced set, so no NEEDS_CONTEXT on that ground.

---

## 2. The fence verification, before applying anything

The two source texts and the two target texts were **extracted from the plan
file programmatically** rather than retyped, so what was applied is
character-identical to what the plan fences. The extractor walked the Task A1
section (from `## Task A1:` to `## Task A2:`) and collected its fenced blocks in
order; it found 7, which are, in order: the Step-1 grep, the Step-1 python
heredoc, (a) source, (a) target, (b) source, (b) target, the Step-5 commit
block. Blocks 3/4/5/6 are the two replacements.

```
$ python3 <extractor>
=== source-presence checks (exact substring, byte level) ===
A source: occurrences in BUILDING.md = 1
B source: occurrences in BUILDING.md = 1

=== replacement line widths (target texts) ===
A target: line lengths [65]  max=65
B target: line lengths [66, 75, 73, 75, 77, 72, 38]  max=77

=== ordinal token in either replacement? ===
A target: matches of part/parts+digit = []
B target: matches of part/parts+digit = []
```

Both sources occur **exactly once**, so neither replacement is ambiguous. The
replacement's longest line is **77**, which is the figure the plan states.

### 2.1 The Rust gate block's own commands, read out of `BUILDING.md`

The plan requires this to be read from the file, not taken from the plan. Read
by locating the `<!-- gate-block: rust; ... -->` marker and walking its fence:

```
  1. cargo fmt --all --check
  2. cargo clippy --workspace --all-targets -- -D warnings
  3. cargo test --workspace
  4. RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
  5. cargo deny check
  6. cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
  (block enumerates 6 commands)
```

**The first four are `cargo fmt`, `cargo clippy`, `cargo test`, `cargo doc`** -
exactly what the (b) replacement names, in exactly that order. My reading agrees
with the plan's, so no NEEDS_CONTEXT on that ground either.

---

## 3. Step 2 - the edits

```
edit (a) applied; source now present 0 times, target 1 times
edit (b) applied; source now present 0 times, target 1 times
BUILDING.md: 8151 -> 8227 chars
```

Applied as exact substring replacements of the extracted plan text, each
asserted to have exactly one occurrence first.

---

## 4. Step 3 - the five checks

### 4.1 Absence check O, positional ordinals

**Fire (its own pre-state run):** section 1.1 above, three lines. Re-derived
independently from git after the edit, so the fire is reproducible from the
committed history rather than only from this transcript:

```
$ git show HEAD:BUILDING.md | grep -nE 'part [0-9]|parts [0-9]'
102:The cross-target clippy run (part 6) type-checks the workspace for Windows
134:CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
135:three OS legs (its Windows leg covers natively what part 6 cross-checks
exit=0
```

**End state:**

```
$ grep -nE 'part [0-9]|parts [0-9]' BUILDING.md
exit=1
```

Zero hits.

**Soundness control - THE PLAN'S PRESCRIBED CONTROL DOES NOT FIRE.** This is
concern 1; the full diagnosis is in section 6.1. Step 3 prescribes
`docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` as the
known-present control. It returns nothing, on three independent engines:

```
$ grep -nE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
exit=1
$ grep -nP 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
exit=1
$ /usr/bin/grep -nE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
exit=1
```

The file is present and 40122 bytes, so this is not a missing-file artefact. The
cause is measurable: **that file carries only SPELLED ordinals, never the digit
form.**

```
$ grep -noiE '[a-z]* parts? [a-z0-9()-]*' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
18:adds part nine
315:gate part (
319:gains part nine)
323:gate part (
363:as part of
$ grep -noiE '(first|...|twelfth)[- ](gate|part)|(gate|part) (one|...|twelve)\b|as the [a-z]+ gate part' <same file>
7:ninth gate
18:part nine
271:third-part
315:ninth gate
319:part nine
323:ninth gate
```

Three `ninth gate` hits - which is exactly the plan's **authoring-section**
control, whose claim is about the SPELLED sweep ("the same family of terms over
`docs/superpowers/plans` hits `2026-07-11-plan-5.5-pre-1.0-hardening.md` three
times"). Step 3 borrowed that file as the control for the DIGIT expression,
where it cannot fire by construction.

**Controls actually used, all from the plan's own authoring section, none
invented at the keyboard.** The authoring section states the discriminating
control for this exact expression: "the same expression over `README.md` returns
`0`, and over `BUILDING.md` returns 3, so it discriminates."

```
$ grep -cE 'part [0-9]|parts [0-9]' README.md
0
exit=1
$ git show HEAD:BUILDING.md | grep -nE 'part [0-9]|parts [0-9]'    # pre-state
(3 lines, pasted above)
$ grep -ncE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md
15
```

The expression is therefore proven to **discriminate**: 0 on a file without the
form, 3 on the pre-state of the file under test, 15 on a live file carrying the
form. The empty end-state result is a measurement, not a broken pattern.

For the controller, the retired plans that DO carry the digit form and could
have served as Step 3's control:

```
$ grep -rlE 'part [0-9]|parts [0-9]' docs/superpowers/plans/
docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md
docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md
```

### 4.2 Absence check L, line length

**Fire (its own pre-state run):** section 1.2, one line at 86.

**End state:** the script prints nothing, exit 0.

**Soundness control, threshold lowered to 60:** prints **73 lines**, so the
script measures and reports rather than silently printing nothing. (Full 73-line
output was inspected; it spans the whole file, `:3` through `:189`.)

**The fence exception tested in BOTH directions**, because a single test cannot
tell an exception that works from one that swallows everything:

- *Must-ignore half.* Eight lines inside fences exceed 80 and are correctly
  invisible to the check:

```
  29: len=81   sudo dnf install -y webkit2gtk4.1-devel librsvg2-devel libappindicator-gtk3-devel
  36: len=136  sudo apt install -y libwebkit2gtk-4.1-dev build-essential ... librsvg2-dev
  87: len=83   RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
  89: len=85   cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
 116: len=121  pnpm test:e2e         # Playwright smoke + axe a11y + i18n completeness (...)
 123: len=103  python3 scripts/ledger-lint.py   # house-knowledge YAML invariants + ...
 159: len=84   cp "target/release/muxsmith$( ... )" \
 160: len=94      "src-tauri/binaries/muxsmith-$triple$( ... )"
  -> 8 such lines
```

- *Must-catch half.* The pre-state's `:138` at 86 characters, outside any fence,
  was caught (section 1.2).

This also reproduces the plan's authoring arithmetic: **9 over-80 lines total in
the pre-state, 8 of them fenced, 1 not.** End state: 8 fenced, 0 non-fenced.

### 4.3 `scripts/ledger-lint.py`, green and made to fire

**Baseline hash of the edited file, taken BEFORE any mutation** (the brief's
shell hazard; see concern 2 in section 6.2 for why `git checkout -- BUILDING.md`
was not the restore mechanism):

```
3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f  BUILDING.md
```

**Green on the end state:**

```
$ python3 scripts/ledger-lint.py
ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
exit=0
```

Shape as the plan requires: `across 4 files plus BUILDING.md's gate enumeration`
and the `all invariants hold` tail. The entry count is 560 today; the plan
deliberately does not fence it.

**Fire.** `11 parts` occurs exactly once in the file (verified before mutating;
`12 parts` occurs zero times), so the mutation is unambiguous:

```
$ python3 -c "<replace '11 parts' -> '12 parts'>"
mutated: 11 parts -> 12 parts
$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-total states 12 parts but the three gate blocks enumerate 11

ledger-lint: 1 violation(s) across 560 entries
exit=1
```

The check demonstrably still reads this file and still compares the canonical
sentence against the marked blocks' enumeration.

**Restore, proven by hash rather than asserted:**

```
$ python3 -c "<replace '12 parts' -> '11 parts'>"
restored: 12 parts -> 11 parts
$ sha256sum -c building-baseline.sha256
BUILDING.md: OK
$ python3 scripts/ledger-lint.py
ledger-lint: 560 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
exit=0
```

### 4.4 Diff scope, in both states

**DIRTY state** (canonical sentence still mutated for the fire above) - the RED
half the plan says comes free. The canonical sentence at `:75` DOES appear as
changed:

```
$ git diff -U0 -- BUILDING.md
diff --git a/BUILDING.md b/BUILDING.md
index 911fdba..8842483 100644
--- a/BUILDING.md
+++ b/BUILDING.md
@@ -75 +75 @@ release bundle locally" below covers the local invocation.
-The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three
+The pre-push gate is 12 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three
@@ -102 +102 @@ which the flag caught the moment it was added) - stays invisible.
-The cross-target clippy run (part 6) type-checks the workspace for Windows
+The cross-target clippy run type-checks the workspace for Windows
@@ -134,6 +134,7 @@ PyYAML; CI runs it from a throwaway venv as its own job.
-CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
-three OS legs (its Windows leg covers natively what part 6 cross-checks
-from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and
-`pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
-and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)
-run as independent jobs.
+CI (`.github/workflows/ci.yml`) runs the Rust block's `cargo fmt`,
+`cargo clippy`, `cargo test` and `cargo doc` commands natively on all three
+OS legs (its Windows leg covers natively what the cross-target clippy run
+cross-checks from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`,
+and `pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
+and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants,
+Plan-8 rider) run as independent jobs.

$ git diff --stat
 BUILDING.md | 17 +++++++++--------
 1 file changed, 9 insertions(+), 8 deletions(-)
```

**CLEAN state** (after the restore) - the canonical sentence is gone from the
diff and only Step 2's two regions remain:

```
$ git diff -U0 -- BUILDING.md
diff --git a/BUILDING.md b/BUILDING.md
index 911fdba..efb4b2f 100644
--- a/BUILDING.md
+++ b/BUILDING.md
@@ -102 +102 @@ which the flag caught the moment it was added) - stays invisible.
-The cross-target clippy run (part 6) type-checks the workspace for Windows
+The cross-target clippy run type-checks the workspace for Windows
@@ -134,6 +134,7 @@ PyYAML; CI runs it from a throwaway venv as its own job.
-CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
-three OS legs (its Windows leg covers natively what part 6 cross-checks
-from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and
-`pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
-and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)
-run as independent jobs.
+CI (`.github/workflows/ci.yml`) runs the Rust block's `cargo fmt`,
+`cargo clippy`, `cargo test` and `cargo doc` commands natively on all three
+OS legs (its Windows leg covers natively what the cross-target clippy run
+cross-checks from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`,
+and `pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
+and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants,
+Plan-8 rider) run as independent jobs.

$ git diff --stat
 BUILDING.md | 15 ++++++++-------
 1 file changed, 8 insertions(+), 7 deletions(-)

$ git status --porcelain
 M BUILDING.md
```

Exactly one file.

#### 4.4.1 The scope claim machine-verified, and the verifier itself fired

The plan's W5-d claim is that no changed line is a marker line, a fence line, a
line inside a fence, or the canonical gate-total sentence. Reading the diff by
eye satisfies nobody, so a verifier was written that parses the `-U0` hunk
headers, reconstructs each changed line's number on its own side, and tests all
four properties against the old file (`git show HEAD:BUILDING.md`) and the new.
All four marker strings are named in full, not abbreviated.

```
$ python3 diffscope.py                       # clean state
changed lines examined: 15  (7 removed, 8 added)
the four named marker lines, all present and unchanged in the new file: [1, 1, 1, 1]
OK: no marker line, no fence line, no line inside a fence, no canonical gate-total sentence among the changed lines
exit=0
```

A green verifier proves nothing until it has been made to fail, and
`proc-verification-step-must-be-falsifiable` makes that duty **per assertion**,
not per script. All four were fired separately, each mutation restored and the
restore proven by hash before the next:

```
--- FIRE assertion 4: canonical gate-total sentence ---   (11 parts -> 12 parts)
VIOLATIONS:
  old:75 is the CANONICAL gate-total sentence: 'The pre-push gate is 11 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three'
  new:75 is the CANONICAL gate-total sentence: 'The pre-push gate is 12 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three'
  exit=1   restored, hash matches baseline: True

--- FIRE assertion 1: marker line ---                     (house marker, one space added)
the four named marker lines, all present and unchanged in the new file: [1, 1, 1, 0]
VIOLATIONS:
  old:121 is a MARKER line: '<!-- gate-block: house; checked by scripts/ledger-lint.py -->'
  exit=1   restored, hash matches baseline: True

--- FIRE assertion 2: fence line ---                      (```bash -> ```sh)
VIOLATIONS:
  old:83 is a FENCE line: '```bash'
  old:83 is INSIDE a fence: '```bash'
  new:83 is a FENCE line: '```sh'
  new:83 is INSIDE a fence: '```sh'
  exit=1   restored, hash matches baseline: True

--- FIRE assertion 3: line inside a fence ---             (cargo test --workspace --quiet)
VIOLATIONS:
  old:86 is INSIDE a fence: 'cargo test --workspace'
  new:86 is INSIDE a fence: 'cargo test --workspace --quiet'
  exit=1   restored, hash matches baseline: True

final hash == pre-fire baseline: True
  3ce5b604d353e9f235860a6696e42b3be2ef3e36615a640bd087c829fa026c1f
```

The marker fire incidentally proves the marker tally is live too: it dropped
from `[1,1,1,1]` to `[1,1,1,0]` when the house marker was disturbed.

### 4.5 Test duty, weighed (`tests-ship-with-the-feature-never-after`)

**No test ships, and the reason is that there is no observable behaviour, not
that the scope is tight.** This task changes prose in a build document. Nothing
compiles it, nothing renders it into the product, no user-visible consequence
follows from it. The house rule's own boundary applies: it forbids deferring a
scenario the existing infrastructure can already express, and there is no
scenario here to express.

The observable behaviour in the neighbourhood - the gate-count invariant - is
already checked by `scripts/ledger-lint.py`, and this task **exercised and fired
that check** (section 4.3) rather than extending it. The 80-column norm has no
checker anywhere in the repo (the plan measured this: no `.editorconfig`, no
markdownlint config, no prettier config, no gate part reading `BUILDING.md` for
width), and building one is not in this task's Files list.

### 4.6 Final state, re-verified after the commit

```
$ grep -nE 'part [0-9]|parts [0-9]' BUILDING.md          -> exit=1, no output
$ <fence-aware length pass>                              -> over-80 non-fenced lines: 0
$ python3 scripts/ledger-lint.py                         -> exit=0, all invariants hold
$ python3 diffscope.py                                   -> OK, exit=0
$ git diff --stat master...HEAD
 BUILDING.md | 15 ++++++++-------
 1 file changed, 8 insertions(+), 7 deletions(-)
$ git status --porcelain                                 -> (clean)
```

Typography of the added lines, with the probe's own control fired:

```
  em-dash U+2014: 0     en-dash U+2013: 0      figure dash U+2012: 0
  horizontal bar U+2015: 0                     minus U+2212: 0
  curly quotes: 0       ellipsis U+2026: 0     nbsp U+00A0: 0
  non-ASCII codepoints in the added lines: []
  (control fired: a literal em-dash in this probe string counts as 1)
```

---

## 5. Step 4 - surfaced, not edited

The plan names three. A **fourth** was found by a sweep and is item 4.

### 5.1 `docs/process-conventions.yaml`, Tier-2 `gate-includes-cross-target-lint-for-the-unrun-os`

Quoted from the file, `:661`, the clause this task falsifies (it carries no
backticks in the source):

> `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`, **documented as gate part 6 in BUILDING.md** with `rustup target add x86_64-pc-windows-msvc` as a one-time documented prerequisite

`BUILDING.md` no longer documents it as "gate part 6"; it names the run without
a position. The plan may not edit house-knowledge YAML - the controller is its
single writer.

### 5.2 `docs/ROADMAP.md`, the "A neighbouring class" paragraph

In the section "Gate-count derivation has no check". It enumerates the three
sites this task removes and cites the long line:

> re-measured at commit `ddb8f42` with `grep -nE 'part [0-9]|parts [0-9]'
> BUILDING.md` [...] `:102` "The cross-target clippy run (part 6)", `:134` "runs
> Rust-gate parts 1-4", and `:135` "what part 6 cross-checks"

and

> Task 1's fenced Step-1(e) replacement leaves `BUILDING.md:138` at 86
> characters, the file's only non-fenced prose line over 80

Both are now done. Disposition is a controller close action.

### 5.3 `.github/workflows/ci.yml:88` - a measured NON-defect, deliberately NOT edited

```
      # Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.
```

A dated provenance statement about what Plan 5.5 Task 12 added, corroborated by
that retired plan's own task heading (its `:7`, `:315`, `:323` carry `ninth
gate`, measured in section 4.1). The ROADMAP's MEASURED block establishes that
such a record is not renumbered to today's count. Named here so a later sweep
does not "repair" it.

### 5.4 NEW - `docs/ROADMAP.md`, a FOURTH live consumer the plan's Step-4 list does not name

Found by sweeping the tree for the fact rather than trusting the plan's list,
per `proc-sweep-surface-completeness` and the brief's own "the scope unit for a
repeated fact is the set of assertions".

```
$ git grep -nE 'parts? [0-9]' -- ':!docs/superpowers/plans' ':!docs/process-journal*' | grep -i building
docs/ROADMAP.md:2725:  (BUILDING.md, Rust gate part 6; cfg-gated items can differ per platform).`
```

Context (in the post-1.0 item "Remove mise from CI"):

> **Rider, gated on the next ci.yml-touching change whichever it is** [...]
> Exact replacement, so nobody re-derives it: `# legs, matching the
> cross-target clippy gate part (BUILDING.md, Rust gate part 6; cfg-gated
> items can differ per platform).`

**Why this one is materially different from 5.3 and is not covered by the
MEASURED block's history principle:** it is not a dated record of what was once
true. It is a **forward-looking prescription** - a fenced exact replacement text
that a future ci.yml-touching change is instructed to apply verbatim, precisely
so nobody re-derives it. As of this commit that prescribed text cites a position
`BUILDING.md` no longer states. If it is applied as written, it writes a stale
citation into `ci.yml` at that moment.

Not edited: the plan forbids any task editing `docs/ROADMAP.md`, and this is a
controller close action. Flagged because the plan's authoring section asserts
"**The one live consumer** of the 'part 6' wording is a Tier-2 statement", and
that claim measures as understated by one.

### 5.5 A lower-confidence neighbour, offered as a judgment call, not a claim

```
docs/decision-ledger.yaml:5430: "Referring to a list member by its POSITION - the
third rejected alternative, gate part 6, the second bullet - is a reference that
goes stale [...] This is a SIBLING of the owner's line-number ruling and its
BUILDING.md instance, not a widening of either"
```

`gate part 6` here is an **illustrative example of the anti-pattern**, not a
claim about `BUILDING.md`'s content, so it is arguably untouched by this edit.
The phrase "its BUILDING.md instance" is the only part that now points at
something removed. Recorded for the controller's judgment; no action taken and
none obviously needed.

### 5.6 Sweep completeness, and what the sweep cannot see

The digit-form sweep over live files (excluding plans and the process journal)
returned, besides the three above: two `docs/IDEAS.md` hits about *file* parts in
an appending feature, one `decision-ledger.yaml` occurrence `ref` reading `part
3`, and one `decision-ledger.yaml` statement using `gate part 1` as an example.
None is a claim about `BUILDING.md`'s ordinals.

The spelled-form sweep is **noisy in a way a re-runner should know**: the
alternation `(first|second|third|...)[- ](gate|part)` matches the token
`third-party`, which occurs widely in this codebase. The only genuine
gate-ordinal hit in a live file is `ci.yml:88` (item 5.3). ROADMAP `:1791` and
`:1844` say "eleventh part" about `ledger-lint` joining the block - historical
records of a landed change, and unaffected by this edit, which touches no block.

---

## 6. Concerns

Both are defects in the instructions, not in the deliverable. Neither altered a
character of what was applied.

### 6.1 CONCERN - the plan's Step-3 soundness control for absence check O cannot fire

Step 3 states: "**Soundness control** [...] the same expression over
`docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` returns
matches, so the pattern demonstrably hits a positional gate ordinal when one is
present."

**It returns nothing**, on ugrep `-E`, ugrep `-P` and `/usr/bin/grep -E`, because
that file carries only spelled ordinals (`ninth gate part`, `part nine`) and
never the digit form (section 4.1, with the file's actual contents pasted).

The likely origin is visible in the plan itself: its **authoring section** names
that same file as the fired control for the **spelled-ordinal sweep** ("hits
[it] three times" - which reproduces exactly, three `ninth gate` hits). Step 3
appears to have carried that file across to the digit expression, where it is
unsatisfiable by construction.

**Why this did not become a NEEDS_CONTEXT return.** Nothing about the deliverable
is undecided: both replacements are fenced, both applied verbatim, and the
plan's own authoring section states a *working* control for this *same*
expression ("the same expression over `README.md` returns `0`, and over
`BUILDING.md` returns 3, so it discriminates"), which reproduces. Choosing
between two control statements the plan itself makes, one false and one true, is
not a design decision resolved at the keyboard; it is reading the plan. The check
is therefore fire-verified to the standard
`proc-verification-step-must-be-falsifiable` demands - 3 hits pre-state, 0 on a
file without the form, 15 on a live file with it, 0 on the end state - rather
than trusted as a bare empty result. The controller may of course rule
otherwise; the correction is one file name, and section 4.1 lists the two retired
plans that would serve.

This is itself an instance of what that Tier-2 entry warns about one level up: a
control that cannot fire is the vacuous-negative failure applied to the control.

### 6.2 CONCERN - the brief's prescribed restore mechanism would have destroyed the deliverable

The brief's preamble says: "Your Step-3 fire mutates `BUILDING.md` and must
restore it. Take the baseline BEFORE mutating, **restore with `git checkout --
BUILDING.md`**, and PROVE the restoration in the diff check the step prescribes."

At the point Step 3 runs, the Step-2 edits are **uncommitted working-tree
changes** (Step 5 is the commit). `git checkout -- BUILDING.md` restores the file
from the index/HEAD and would have **silently discarded both replacements**, then
left every subsequent check passing against an unmodified file: check O would
have returned three hits (visibly wrong), but a less careful sequencing could
have produced a clean `git diff` read as success while the deliverable no longer
existed.

It also contradicts the plan's own Step 3, which requires the post-restore diff
to still SHOW Step 2's two regions and `git diff --stat` to name exactly one
file - impossible after a checkout.

**What was done instead:** a sha256 baseline of the *edited* file taken before
any mutation, mutation and restore performed as exact inverse string
replacements, and the restore proven by `sha256sum -c` returning `OK` after every
single fire (five of them: the ledger-lint fire plus the four verifier
assertions), with the final hash re-asserted equal to the pre-fire baseline. That
satisfies the brief's actual requirement - take a baseline before mutating, prove
the restoration - by a mechanism that does not delete the work.

The brief's shell-hazard paragraph is otherwise sound and was respected: no `cp`
was used anywhere in this task, every file operation ran through Python or git,
and every run was foreground.

### 6.3 Minor - the plan's "one live consumer" claim is understated by one

Section 5.4. The authoring section asserts "The one live consumer of the 'part 6'
wording is a Tier-2 statement". `docs/ROADMAP.md:2725` is a second, and it is a
forward-looking prescription rather than a historical record. Surfaced only; not
edited.

---

## 7. Commit

```
$ git add BUILDING.md
$ git -c commit.gpgsign=false commit \
    -m "docs: BUILDING.md names the gate commands instead of numbering them, and its CI paragraph fits 80 columns" \
    -m "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>" \
    -- BUILDING.md
[plan-11-stream-a a0d5d3e] docs: BUILDING.md names the gate commands instead of numbering them, and its CI paragraph fits 80 columns
 1 file changed, 8 insertions(+), 7 deletions(-)
```

Verified shape:

```
SHA: a0d5d3e28d44ee464dd9175ea11e5446e9b0dc0c
AUTHOR: Şenol Feldmann <senol.feldmann@gmail.com>
SIGNED: N                      (unsigned, per SI-4)
SUBJECT: docs: BUILDING.md names the gate commands instead of numbering them, and its CI paragraph fits 80 columns
BODY:    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
Co-Authored-By trailer count: 1
Claude-Session lines: 0
files in commit: BUILDING.md only
working tree after commit: clean
```

Staged explicitly, never `git add -A`. **Not pushed** - the controller pushes
once at the plan close.

---

## 8. Preamble compliance

- Worked only in `/home/senol/Git/muxsmith-plan11-a` on `plan-11-stream-a`.
  Never on `master`, never in the main worktree. The only main-repo path touched
  is this report file, which the brief names.
- No session-relocation tool called (no EnterWorktree/ExitWorktree). `git
  worktree` never run.
- Foreground runs only; no background-plus-monitor.
- Absolute paths for anything outside the worktree.
- The full gate was **not** run - the stream runs it once before merge, which is
  the controller's dispatch.
- ASCII hyphens, straight quotes, no ellipsis character, no em-dash, in the
  edited file (measured, section 4.6) and in this report.

## 9. Acceptance map coverage

| row | claim | discharged by |
|---|---|---|
| W5-a | no positional gate ordinal remains | 4.1: RED 3 lines pre-state, GREEN 0, control discriminates 0/3/15 |
| W5-b | no non-fenced line over 80 | 4.2: RED 1 line at 86, GREEN 0, threshold-60 control prints 73, fence exception tested both directions |
| W5-c | ledger-lint's invariant still holds, and the check was made to fire | 4.3: exit 0 with its summary line; fired to exit 1 naming the mismatch; restored with hash proof |
| W5-d | markers, fences, fenced lines and the canonical sentence byte-identical | 4.4 + 4.4.1: diff pasted dirty and clean; machine-verified; all four assertions fired separately with hash-proven restores |
| W5-e | the Tier-2 statement is surfaced, not orphaned | 5.1, with the clause quoted; plus a fourth consumer in 5.4 the plan does not name |
