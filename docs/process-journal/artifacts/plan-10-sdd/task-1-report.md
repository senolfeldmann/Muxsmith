# Task 1 report - Plan 10 (W3: the gate-count invariant)

**Status: DONE_WITH_CONCERNS.** Every step landed as fenced; the gate is green;
the commit is `ddb8f42`. Three concerns below are cosmetic/enumeration items a
reviewer can rule on yes/no, none of them a deviation from the plan's text.

Worked on `master` in `/home/senol/Git/Muxsmith`, no branch, no worktree, no
session-relocation tool, foreground runs only. Tree at start:
`git status --porcelain` empty, head `754cb73`.

---

## 1. Re-measurement of the three gate blocks, against the plan's `11 / 6 / 4 / 1`

Run BEFORE any edit, on the tree at `754cb73`, with the plan's own command shape.

```
$ cd /home/senol/Git/Muxsmith && grep -n '^```' BUILDING.md
28:```bash
30:```
34:```bash
37:```
62:```bash
66:```
76:```bash
83:```
104:```bash
109:```
113:```bash
115:```
141:```bash
148:```

$ sed -n '77,82p' BUILDING.md | grep -cvE '^\s*(#|$)'
6
$ sed -n '105,108p' BUILDING.md | grep -cvE '^\s*(#|$)'
4
$ sed -n '114p' BUILDING.md | grep -cvE '^\s*(#|$)'
1
```

Sum `6 + 4 + 1 = 11`. **Agrees with the plan's `11 / 6 / 4 / 1` exactly.** The
pre-routed NEEDS_CONTEXT fork therefore did not trigger and the fenced sentence
was transcribed unchanged.

Continuation check over the same three spans, plus the plan's fired control:

```
$ sed -n '77,82p' BUILDING.md | grep -c '\\$'
0
$ sed -n '105,108p' BUILDING.md | grep -c '\\$'
0
$ sed -n '114p' BUILDING.md | grep -c '\\$'
0

# FIRED CONTROL - the release-reproduction block, which does carry a continuation
$ sed -n '142,147p' BUILDING.md | grep -c '\\$'
1
$ sed -n '142,147p' BUILDING.md | grep -n '\\$'
4:cp "target/release/muxsmith$( [ "$(uname -o 2>/dev/null)" = Msys ] && echo .exe )" \
```

The pattern demonstrably matches a backslash continuation when one is present,
so the three zeros are a measurement rather than a malformed grep.

---

## 2. Per-file changes, region by region, against the Files list (EXHAUSTIVE)

### `BUILDING.md`

| Step | Region | Change |
|---|---|---|
| 1(a) | between the `pnpm build` paragraph and the Rust-gate heading | inserted the `<!-- gate-total; ... -->` marker plus the fenced canonical sentence, character for character as fenced |
| 1(b) | the Rust-gate heading | `### The Rust gate (six parts, run from the repo root, workspace-wide)` -> `### The Rust gate (run from the repo root, workspace-wide)`; the parenthetical's other content preserved verbatim |
| 1(c) | above each of the three `` ```bash `` gate fences | inserted the three `<!-- gate-block: rust\|frontend\|house; ... -->` marker lines, verbatim |
| 1(d) | House-knowledge check fence + paragraph | the fence's single command line replaced with the fenced string; the whole paragraph beneath the fence replaced with the fenced paragraph. PyYAML prerequisite and CI-job arrangement both retained, as the plan requires |
| 1(e) | CI paragraph parenthetical | `(house-knowledge invariants, Plan-8 rider)` -> `(house-knowledge and gate-count invariants, Plan-8 rider)`; nothing else in that paragraph touched, ordinals untouched |
| 1(f) | - | **no edit**, surfaced in section 6 below |

Nothing else in the file changed. Diff: 18 insertions, 5 deletions.

### `scripts/ledger-lint.py`

| Region | Change |
|---|---|
| module docstring, first line | `Structural integrity check for the four house-knowledge YAML files.` -> `... YAML files and BUILDING.md.` - it no longer claims the script only covers the four YAML files |
| module docstring, numbered check list | added check `7`, stating the canonical-sentence-vs-marked-blocks comparison and that the markers are the anchor because heading prose is not stable. Column alignment (separator at col 39, continuations at 41 spaces) measured off checks 1-6 and matched |
| imports | `import re` added. No other new import; `Path` was already there. No new dependency |
| module constants | `BUILDING`, `GATE_BLOCKS`, `GATE_TOTAL_MARKER`, `GATE_TOTAL_RE`, with a comment recording why the anchor is an HTML comment and not a heading |
| new `_next_non_empty(lines, start)` | helper: index of the first non-blank line at or after `start`, else `None` |
| new `_count_block_commands(name, lines, marker_idx, violations)` | counts one block; returns `None` where the count is not derivable (no opening fence, unterminated fence) so the caller can skip; the continuation guard reports and still returns a count, as the plan requires for F5 |
| new `check_building_gate_total(violations)` | the check proper, exactly the behaviour Step 2 fixes |
| `main()` | one call, `check_building_gate_total(violations)`, placed after the FILES loop and before the summary, so findings join the existing `violations` list and the existing exit-code logic |
| success line | widened to `ledger-lint: {total} entries across {len(FILES)} files plus BUILDING.md's gate enumeration, all invariants hold`, wrapped across two source lines |

Behaviour implemented, point by point against Step 2:

- missing `BUILDING.md` -> violation, return.
- each of the three markers matched by **exact string on the stripped line**;
  `len(hits) != 1` is a violation naming the marker and the count found.
- next non-empty line after a block marker must be exactly `` ```bash ``; else a
  violation quoting what was found.
- counted command line = stripped non-empty, not starting with `#`; counting
  stops at the next line that is exactly `` ``` ``; an unterminated fence is a
  violation.
- any line inside a block ending with a backslash -> its own violation naming
  the line and stating that the check does not model shell continuations. It
  does **not** suppress the comparisons (F5 depends on that).
- exactly one `gate-total` marker; next non-empty line must match
  `^The pre-push gate is (\d+) parts: (\d+) Rust, (\d+) frontend, (\d+) house-knowledge\.`;
  either failure is **one violation that stands alone** and every comparison is
  skipped.
- per-block comparisons in the plan's fenced shape; the total compared against
  the sum. **A block whose count is not derivable skips its own comparison AND
  the total**, so a single deleted marker emits exactly one violation.

### `.github/workflows/ci.yml`

| Region | Change |
|---|---|
| `ledger-lint` job's leading comment block | the check enumeration extended: `... duplicate keys, plus BUILDING.md's gate-count invariant (its canonical gate-total sentence against the commands its three marked gate blocks enumerate). Rider on Plan 8 ...` |
| that job's step `name:` | `ledger-lint (house YAML invariants)` -> `ledger-lint (house YAML + gate-count invariants)` |

No `runs-on`, no pin, no `run:` line, no other job or step touched. Diff on this
file: 6 lines (`+3 -1` in the comment, `+1 -1` on the step name).

---

## 3. The five fires

Baseline taken before the first mutation and every restore proven against it:

```
$ command cp -f BUILDING.md "$SCRATCH/BUILDING.md.baseline"
$ sha256sum BUILDING.md "$SCRATCH/BUILDING.md.baseline"
2e056a4920ee69431fead338c618789e07991c7517d6436b361789989cb01798  BUILDING.md
2e056a4920ee69431fead338c618789e07991c7517d6436b361789989cb01798  .../BUILDING.md.baseline
```

Every mutation was applied by a Python heredoc with an `assert` on the match
count (so a silently-missed mutation cannot masquerade as a fire), and every
restore used `command cp -f` - the bare `cp` alias hazard the brief names was
avoided throughout. After every restore: `sha256sum` back to
`2e056a49...cb01798` and `git diff --stat -- BUILDING.md` back to
`1 file changed, 18 insertions(+), 5 deletions(-)`, i.e. this task's intended
edit and nothing else. That proof is identical after all seven mutations and is
pasted once per fire below in abbreviated form.

### F1 - the stated total

Mutation: `The pre-push gate is 11 parts:` -> `12 parts:`.

```
$ grep -n "^The pre-push gate is" BUILDING.md
75:The pre-push gate is 12 parts: 6 Rust, 4 frontend, 1 house-knowledge. The three

$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-total states 12 parts but the three gate blocks enumerate 11

ledger-lint: 1 violation(s) across 531 entries
EXIT=1
```

Restore: `2e056a49...cb01798`, `18 insertions(+), 5 deletions(-)`. **Meets W3-b.**

### F2 - the enumeration

Mutation: one command line (`pnpm lint`, duplicated) appended to the frontend
gate block.

```
$ sed -n '/gate-block: frontend/,/^```$/p' BUILDING.md
<!-- gate-block: frontend; checked by scripts/ledger-lint.py -->
```bash
pnpm lint            # eslint (Vue rules, TypeScript rules, D27 no-raw-text)
pnpm build            # vue-tsc type-check + production frontend build
pnpm check:i18n       # frontend Fluent catalog completeness gate (spec 8.4)
pnpm test:e2e         # Playwright smoke + axe a11y + i18n completeness (type-checks e2e/, builds the harness, then runs)
pnpm lint
```

$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-block 'frontend' states 4 commands but enumerates 5
FAIL BUILDING.md: gate-total states 11 parts but the three gate blocks enumerate 12

ledger-lint: 2 violation(s) across 531 entries
EXIT=1
```

Names the frontend block AND the total, as the plan requires. Restore proven.
**Meets W3-c.**

### F3 - the total anchor

Mutation: the `<!-- gate-total; checked by scripts/ledger-lint.py -->` line
deleted.

```
$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: expected exactly one gate-total marker line '<!-- gate-total; checked by scripts/ledger-lint.py -->', found 0

ledger-lint: 1 violation(s) across 531 entries
EXIT=1
```

**Violations are EXACTLY ONE**, naming the missing total anchor; the comparisons
are skipped because no stated numbers exist. Restore proven. **Meets W3-d.**

### F4 - a block anchor

Mutation: the `<!-- gate-block: frontend; ... -->` line deleted.

```
$ grep -n 'gate-block:' BUILDING.md
82:<!-- gate-block: rust; checked by scripts/ledger-lint.py -->
120:<!-- gate-block: house; checked by scripts/ledger-lint.py -->

$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-block 'frontend': expected exactly one marker line '<!-- gate-block: frontend; checked by scripts/ledger-lint.py -->', found 0

ledger-lint: 1 violation(s) across 531 entries
EXIT=1
```

**Violations are EXACTLY ONE**, naming the missing `frontend` block marker; the
frontend comparison and the total are both skipped, exactly as Step 2 fixes and
F4 requires. Restore proven. **Meets W3-e.**

### F5 - the continuation guard

Mutation: the house-knowledge block's single command rewritten as two
backslash-continued lines.

```
$ sed -n '/gate-block: house/,/^```$/p' BUILDING.md
<!-- gate-block: house; checked by scripts/ledger-lint.py -->
```bash
python3 \
  scripts/ledger-lint.py   # house-knowledge YAML invariants + BUILDING.md's gate-count invariant
```

$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-block 'house': line 123 ends with a backslash; this check does not model shell continuations, so the block must enumerate one command per line
FAIL BUILDING.md: gate-block 'house' states 1 commands but enumerates 2
FAIL BUILDING.md: gate-total states 11 parts but the three gate blocks enumerate 12

ledger-lint: 3 violation(s) across 531 entries
EXIT=1
```

The violations **include** the continuation message, and the accompanying
house-block (1 vs 2) and total (11 vs 12) mismatches appear exactly as the plan
predicts them. This is the plan's stated expected shape met, not approximated.
Restore proven. **Meets W3-f.**

### Supplementary fires S1 and S2 (beyond the plan's five)

Two code paths I wrote are not covered by the plan's five fires. Task 1 exists
partly so a broken check surfaces here rather than at the plan close, so both
were fired the same way, restore-proven, with nothing persisted. Reported as
supplementary evidence, not as a scope change.

```
# S1: a DUPLICATED rust gate-block marker
$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-block 'rust': expected exactly one marker line '<!-- gate-block: rust; checked by scripts/ledger-lint.py -->', found 2

ledger-lint: 1 violation(s) across 531 entries
EXIT=1

# S2: the house block's opening ```bash fence removed
$ python3 scripts/ledger-lint.py
FAIL BUILDING.md: gate-block 'house': marker is not followed by an opening ```bash fence (found "python3 scripts/ledger-lint.py   # house-knowledge YAML invariants + BUILDING.md's gate-count invariant")

ledger-lint: 1 violation(s) across 531 entries
EXIT=1
```

Both emit exactly one violation and skip the comparisons, consistent with F3/F4.
The one shape still unfired by construction is the never-closed fence, which
cannot be produced without also destroying a later block's structure.

---

## 4. The green end state

Reached on the intended end state, after all restores.

```
$ python3 scripts/ledger-lint.py
ledger-lint: 531 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
EXIT=0

$ git status --porcelain
 M .github/workflows/ci.yml
 M BUILDING.md
 M scripts/ledger-lint.py

$ git diff --stat
 .github/workflows/ci.yml |   6 +-
 BUILDING.md              |  23 +++++--
 scripts/ledger-lint.py   | 163 ++++++++++++++++++++++++++++++++++++++++++++++-
 3 files changed, 183 insertions(+), 9 deletions(-)
```

Exactly the three files in the Files list, nothing else.

### The full gate as `BUILDING.md` enumerates it - all 11 parts, foreground, no subsets

Run against the end state, i.e. against the file's OWN new enumeration,
including this task's own widened check.

| # | Part | Result |
|---|---|---|
| Rust 1 | `cargo fmt --all --check` | EXIT=0 |
| Rust 2 | `cargo clippy --workspace --all-targets -- -D warnings` | EXIT=0 |
| Rust 3 | `cargo test --workspace` | EXIT=0 |
| Rust 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | EXIT=0 |
| Rust 5 | `cargo deny check` | EXIT=0 |
| Rust 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | EXIT=0 |
| FE 1 | `pnpm lint` | EXIT=0 |
| FE 2 | `pnpm build` | EXIT=0 |
| FE 3 | `pnpm check:i18n` | EXIT=0 |
| FE 4 | `pnpm test:e2e` | EXIT=0 |
| House 1 | `python3 scripts/ledger-lint.py` | EXIT=0 |

Pasted result lines from those runs:

```
# Rust 3 - every binary, 0 failed everywhere; sample of the aggregated result lines
      1 test result: ok. 122 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
      1 test result: ok. 80 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
      1 test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
$ cargo test --workspace >/dev/null 2>&1; echo "EXIT=$?"
EXIT=0

# Rust 4
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
EXIT=0

# Rust 5
advisories ok, bans ok, licenses ok, sources ok
EXIT=0

# FE 3
check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
EXIT=0

# FE 4
  68 passed (2.9s)
EXIT=0

# House 1
ledger-lint: 531 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
EXIT=0
```

Note on instrumentation: an earlier attempt at parts 4-6 used
`${PIPESTATUS[0]}`, which is bash-only and printed empty in this zsh shell. The
three parts were re-run with a plain redirect and `$?` before any exit code was
claimed; the table above carries the re-run values, not the empty ones.

### Typography

```
$ grep -nP '[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{2212}\x{00A0}]' \
    BUILDING.md scripts/ledger-lint.py .github/workflows/ci.yml
none found
```

**Fired control for that pattern** (a fixture carrying an em-dash, curly quotes,
a Unicode ellipsis and an NBSP):

```
1:em-dash — here
2:curly “q” here
3:ellipsis … here
4:nbsp   here
control exit=0
```

The pattern demonstrably matches all four denied glyph classes, so the empty
result on the three touched files is evidence rather than a malformed grep.

---

## 5. Commit

```
$ git add BUILDING.md scripts/ledger-lint.py .github/workflows/ci.yml
$ git -c commit.gpgsign=false commit \
    -m "gate: BUILDING.md states the gate total once, ledger-lint checks it against the enumerated commands" \
    -m "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>"
[master ddb8f42] gate: BUILDING.md states the gate total once, ledger-lint checks it against the enumerated commands
 3 files changed, 183 insertions(+), 9 deletions(-)

$ git show --stat HEAD
commit ddb8f4278688d4760fc479520478098570e3bda2
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Wed Jul 29 14:14:20 2026 +0200

    gate: BUILDING.md states the gate total once, ledger-lint checks it against the enumerated commands

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 .github/workflows/ci.yml |   6 +-
 BUILDING.md              |  23 +++++--
 scripts/ledger-lint.py   | 163 ++++++++++++++++++++++++++++++++++++++++++++++-
 3 files changed, 183 insertions(+), 9 deletions(-)

$ git status --porcelain
(empty)
```

Unsigned, explicit pathspecs (never `git add -A`), exactly one trailer, no
`Claude-Session` line, no context-window suffix. **Not pushed** - the single push
is a controller close action.

---

## 6. Step 1(f) - the surfaced ordinals, with line context

Neither line was edited. Line numbers are post-edit, at `ddb8f42`.

```
$ grep -n "part 6" BUILDING.md
102:The cross-target clippy run (part 6) type-checks the workspace for Windows
135:three OS legs (its Windows leg covers natively what part 6 cross-checks

$ grep -n "parts 1-4" BUILDING.md
134:CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
```

Full context of each:

- **`:102`**, opening the paragraph under the Rust gate block:
  "The cross-target clippy run (part 6) type-checks the workspace for Windows
  without linking, so it runs on any OS."
- **`:134`**, opening the CI paragraph (the same paragraph Step 1(e) edits):
  "CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
  three OS legs ..."
- **`:135`**, the continuation of that same sentence: "(its Windows leg covers
  natively what part 6 cross-checks from Linux)".

The task's reasoning, carried per Step 1(f): both are Rust-block-LOCAL positions
rather than totals, so the new check - which parses totals - does not reach them;
covering them would need a second parser with its own anchor scheme over a
numbering the canonical sentence does not define, which is scope this package's
brief did not authorize. And both effects the plan names are real: Step 1(b)'s
argument about leaving unchecked numbers in the file does reach them in spirit,
and now that the file says "The pre-push gate is 11 parts", a bare "part 6"
acquires a second possible referent that only section context resolves.

**A third site the routing enumeration does not name** - see concern 3.

---

## 7. Divergences and judgment calls, each named

1. **No fenced text was altered.** The canonical sentence, the three block
   markers, both Step 1(d) replacements, the Step 1(e) replacement, the Step 6
   commit command and message: transcribed character for character. The
   pre-routed recount fork did not fire because the recount agreed.
2. **`ci.yml` comment block not re-wrapped.** My insertion pushed one comment
   line to 76 characters. I measured the file's own envelope before deciding:

   ```
   $ awk 'length($0) > 70 && $0 ~ /^[[:space:]]*#/' .github/workflows/ci.yml | wc -l
   42
   $ awk 'length($0) >= 76 && $0 ~ /^[[:space:]]*#/ { printf "%d (%d)\n", NR, length($0) }' \
       .github/workflows/ci.yml
   25 (76)
   33 (76)
   73 (77)
   74 (76)
   88 (76)
   89 (76)
   95 (76)
   117 (120)
   126 (77)
   137 (76)
   162 (77)
   174 (76)
   ```

   `:174` is my own line; the other eleven are pre-existing. 76 is inside the
   file's existing width envelope, so re-wrapping would have changed
   neighbouring lines for no gain. Judgment call, disclosed.
3. **`BUILDING.md`'s Step 1(e) line NOT re-wrapped** - see concern 1; the plan
   fences that replacement and says nothing else in the paragraph changes, so I
   applied it literally rather than reflowing.
4. **Docstring alignment matched by measurement, not by eye.** Check 7's
   separator column and continuation indent were read off checks 1-6
   (`awk 'match($0,/^ */)'` -> 41 leading spaces on continuations, separator at
   1-based column 39) before writing.
5. **Two supplementary fires added** (S1, S2). Additive evidence on code paths
   the plan's five fires do not reach; nothing persisted, restores proven. Not a
   deviation from "meet the five, do not approximate them".

Nothing was decided at the keyboard that the plan left open; no NEEDS_CONTEXT
fork arose.

---

## 8. Numbered concerns a reviewer can rule on yes/no

1. **`BUILDING.md`'s Step 1(e) line is now 86 characters, the file's only prose
   line over 80.** Measured, not eyeballed - `awk 'length($0) > 76' BUILDING.md`
   returns 20 lines, but the discriminating measurement is the one that excludes
   code fences:

   ```
   $ awk '/^```/ { inf = !inf; next } (!inf && length($0) > 80) \
       { printf "%d (%d): %s\n", NR, length($0), $0 }' BUILDING.md
   138 (86): and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)
   ```

   One hit, and it is mine. The plan fences the
   replacement string and states "Nothing else in that paragraph changes", so
   reflowing the paragraph would have edited lines the plan fenced off. I
   applied the replacement literally. **Rule: leave as is, or is a reflow of
   that paragraph wanted?** If wanted, it is a two-line change with zero
   rendered effect.
2. **`gate-block 'house' states 1 commands`** - the violation message does not
   pluralize. The plan fences the message shape by example
   (`... states 4 commands but enumerates 5`), and dynamic pluralization would
   have been an invention on a fenced string, so I kept the shape. **Rule:
   acceptable, or worth a `command(s)` form?**
3. **`BUILDING.md` carries THREE positional-ordinal sites, not two.** Step 1(f)
   and the ROADMAP routing entry that carries the vehicle both enumerate two -
   "the cross-target clippy run (part 6)" and "CI runs Rust-gate parts 1-4" -
   but the CI paragraph carries a second, independent "part 6" at `:135`
   ("what part 6 cross-checks from Linux"), inside the very paragraph Step 1(e)
   edits. It is fenced out by Step 1(e)'s "nothing else in that paragraph
   changes", so I did not touch it, and the ROADMAP's own routing text does not
   name it. **Rule: is the third site in the routed set?** It matters because
   the vehicle is "whichever package next edits `BUILDING.md`'s gate blocks",
   and an unenumerated third member is exactly the shape of miss that
   `proc-normative-count-recomputed` exists for.

---

## 9. Surfaced for the controller

- **Ledger candidate, `proc-normative-count-recomputed` (or the ROADMAP's
  gate-count entry directly):** concern 3 above. The ROADMAP's "neighbouring
  class" paragraph that routes the positional ordinals enumerates two sites;
  the file has three. The third is inside the paragraph the same section's own
  Step 1(e) edits. Worth correcting in the ROADMAP entry before the vehicle
  package picks it up, since the entry is what that package will read.
- **Ledger candidate, `latitude-carveout-zero-content-structural-forks`,
  over-restriction watch, do-NOT-over-restrict direction:** this task's Files
  list named all four self-descriptions as regions (the plan's own F3/F4 plan
  review already recorded that generalization). The result is that no stop was
  forced and no finding had to be filed - the boundary behaved as intended here,
  which is calibration data of the wanted kind. The one residual friction is
  concern 1: an explicit within-paragraph fence ("nothing else in that paragraph
  changes") also fences off a purely typographic consequence of the fenced edit
  itself. That is a narrower case than a referent repair and probably should
  stay stopped; recording it as an observation, not a proposed loosening.
- **Not anticipated by the plan:** nothing material. The plan's Step 2 behaviour
  spec was complete enough to implement without a single invention, including
  the skip semantics that F3, F4 and F5 depend on. The one thing an implementer
  had to derive rather than transcribe was the *shape* of the two helper
  functions, which is composition, not a fork.
- **Operational note for later tasks:** `${PIPESTATUS[0]}` is bash-only and
  silently yields an empty string in this project's zsh shell. Tasks 2-5 all run
  gate parts and may reach for it; `cmd >log 2>&1; echo $?` works in both
  shells.
