# Whole-branch verdict delta: Plan 8 fix wave, range 7302e1b..6d81738

Same judge as `whole-branch-verdict.md`, same standards; settled non-findings
are not re-litigated. **Range pinned to `7302e1b..6d81738`**: HEAD moved one
commit past the fix wave during this review (`bd7dba9`, plan-7.5 plan -
disjoint from every file judged here), and the working tree carries the
controller's uncommitted house-YAML/ROADMAP edits plus other implementers'
in-flight work, all outside this delta's files. Writes this round: this file
only; no `gh` command was run, so no `gh-log.md` entry is owed. All fire-test
mutations restored alias-proof (`command cp -f`) and `cmp`-verified against
pre-mutation backups.

## VERDICT: READY

The three commits discharge fix-wave items A, B and D completely and
verbatim; the ledger-lint fix is correct as code and reproduced on both sides
of the delta; nothing my original pass had verified is broken - the two most
load-bearing identities were re-measured and hold. The two recorded items F1
and F2 need no further wave (rulings below: one named-vehicle line for the
controller's close batch, one no-change). Plan 8's close is no longer blocked
by anything in my NEEDS FIXES verdict; what remains open is item E
(controller close-batch lines, never this wave's scope) and the owner steps
R8/R10.

## 1. Item-by-item discharge - all 18 edit sites verified, verbatim where mandated

Verified against the diffs of `ecab53a` / `bcb67f3` / `6d81738` directly, not
against the report.

**A. `scripts/ledger-lint.py` (ecab53a) - 2/2.**
- Loader construction moved inside the parse `try`, `loader = None` before
  it, `finally: if loader is not None: loader.dispose()` - exactly the
  task-5-verdict m1 snippet.
- Docstring trigger claim now "on every master push, `v*` tag and pull
  request" (m3), matching ci.yml's actual trigger block.

**B. `BUILDING.md` (bcb67f3) - 4/4, all texts verbatim mine.**
- :65-68 replacement = adjudication 1(a) text, word-for-word; "not part of
  the CI gate" kept true, "yet" dropped.
- :92-95 replacement = adjudication 1(b) text, word-for-word.
- Tenth gate part, all four pieces = adjudication 2(i)+(ii): prerequisite
  line under `### Rust toolchain`, heading five->six, the
  `--target x86_64-pc-windows-msvc` clippy line as the fence's last line,
  the rationale paragraph.
- `## Cross-target lint rule` deleted (controller ruled no veto).

**D. Design + plan (6d81738) - 12/12.**
- **A2** appended after A1 (A1 byte-untouched), text = adjudication 6
  verbatim.
- Language sites: :958 (map + code-page clause, my table row), :1511 (fence
  line), :2007 (frozen-list item), :1966 (trigger-7 premise - the site no
  recorded list had carried). **:1012 correctly received no edit.**
- Three fallback-clause markings: :941-943, R8's :1921 region, section 11's
  fallback bullet - each "(superseded, A2)", marked not deleted, so R8 keeps
  its rendering check.
- R1 wording ("names the gated SHA (the `ci gate green for <sha>` echo)") +
  the R1 addendum line (adjudication 10, producer still the `pick:` stderr
  log, so `design-acceptance-observables-have-producers` is satisfied).
- R6 dpkg payload form (adjudication 5 text).
- D75 cross-ref -> "section 0, note 2" (adjudication 12).
- **Ruled fork applied as ruled:** plan :248 fence byte-unchanged with the
  brief's supersession line added after it, verbatim; plan :262 frozen-list
  item retains its wording with " (superseded by design amendment A2)"
  appended. Nothing else in the plan changed (numstat 3/1, coheres).

**Absence proof, re-measured myself** (each pattern run against the
`7302e1b` blob first, firing, then against `6d81738`): design
`\["en-US"\]` 2->0, `names the found ci run` 1->0, `citation in section 1`
1->0, `the mechanism is a config` 1->0, `en-US language list` 1->0,
fixed-string `./usr/bin/muxsmith` 1->0; BUILDING.md `out of scope for local
development` 1->0, `nine parts total` 1->0, `Rust gate (five parts` 1->0,
`Cross-target lint rule` 1->0; ledger-lint `on every push` 1->0. Every zero
is a fired-then-absent measurement.

## 2. The ledger-lint fix as code - CONFIRMED, both sides reproduced

The implementer's independent reproduction is confirmed, not just consistent.
My own run, identical `\x08`-at-offset-0 mutation of
`docs/product-boundaries.yaml`, backup-verified restore:

- **Pre-fix script** (the `7302e1b` blob, patched ONLY in its `REPO` path so
  it resolves the repo from the scratchpad): uncaught
  `yaml.reader.ReaderError` traceback, no FAIL line, no summary, exit 1.
- **Fixed script** (working tree): `FAIL docs/product-boundaries.yaml: does
  not parse (unacceptable character #x0008: ...)` plus
  `ledger-lint: 1 violation(s) across 440 entries`, exit 1 - the linter's
  own contract, no traceback.

Code-path review of the restructured block: constructor raise -> `loader`
stays `None`, `except yaml.YAMLError` catches (ReaderError is a subclass,
re-verified), `finally` skips dispose on `None`; `get_single_data` raise
after construction -> dispose runs on the live loader; success path ->
`loader.duplicate_keys` is read after the `try` on a non-None loader. Check 6
re-fired post-fix (planted duplicate `steelman` -> `FAIL ... (lines 22 and
23)`, exit 1); green reachable after every restore. Entry recount at this
run: **472** across 4 files (467 at my original pass; the delta is the
controller's in-flight uncommitted house-YAML additions, and the
violation-run's 440 = 472 minus product-boundaries' 32 ids, arithmetic
coherent). The fix is correct as code, not merely as a diff.

## 3. Rulings on F1 and F2

**F1 (ci.yml:92 cites the deleted section title) - record with a named
vehicle; not a fix-now dispatch, not nothing.** Verified at the site: the
dated Plan-5.5 comment block's sentence reassembles to "All legs, matching
the cross-target lint rule (cfg-gated items can differ per platform)", and
`git grep` shows it is the **only** live tracked citation of the deleted
title (all other hits are frozen journal artifacts, which stay). Both
directions weighed: it is not "no change needed" territory - unlike :88's
"ninth gate part" (a dated naming fact, which I ruled history), :92 is
**present-tense rationale for a live config choice**, and its referent title
is now ungreppable in BUILDING.md; a config comment should point at
something findable. But it is also not worth its own dispatch now: nothing
is false (the rule survives as gate part 6), no behavior is touched, and a
ci.yml edit - even comment-only - costs a commit plus a watched CI run under
the house's D83 sensitivity. Ruling: **close-batch line, gated on the next
ci.yml-touching change** (the registered v1.x "remove mise from CI" item
owns the next ci.yml restructuring and is the natural carrier; any earlier
ci.yml edit inherits the duty - the edit is the trigger). Exact replacement
so nobody invents it: `# legs, matching the cross-target clippy gate part
(BUILDING.md, Rust gate part 6; cfg-gated items can differ per platform).`
Self-accounting: my adjudication-2 sweep checked that block for the
gate-part *count* and not for the *title citation* - the miss is mine, and
it is an instance of my own fix-wave HARVEST rule (a deleted thing's
enumerations include "who cites its title", one grep per set).

**F2 (A2 pins bare :line references) - stands as written, no change
needed.** Verified the lines still resolve (:941-943 is the fallback clause
with its marking, :958 is the language value). The distinction that decides
it: HARVEST item 2 binds **forward-looking correction lists** - deferred
targets someone must later locate, where a drifted :line sends the fixer to
the wrong text. A2's site list is **backward-looking provenance in an
executed amendment** - nobody re-executes A2, and every :line there is
paired with a content descriptor ("fallback clause", "language value",
"section 3.1 fence", "trigger 7 premise") that carries the load if the
numbers drift. Reworded-now would also break the verbatim-application record
this delta just verified. Named bias: I authored the A2 text I am ruling on;
the against-case was examined first, and the ruling rests on the structural
forward/backward distinction, not on taste. If the house later adopts the
task-3 verdict's proposed widening of `code-comment-line-citations-drift`
(any positional pointer into a durable artifact), A2's parenthetical line
numbers can be dropped in that pass; nothing is owed now.

## 4. Anything broken - nothing found

The two load-bearing identities from my original pass, re-measured at
`6d81738`:

- **Design section-2 fence vs `.github/workflows/release.yml`: still
  byte-identical** (extracted and diffed - empty). The fix wave's design
  edits did not graze the workflow contract.
- **Design 3.1 fence: still parses as JSON**, and its language line is now
  byte-identical to the shipped `src-tauri/tauri.conf.json:52` - the one
  divergence my original pass carried as "recorded, owner-authorized" is now
  closed, an upgrade, not a break.

Also checked: range purity on my pathset (`7302e1b..6d81738` touches only
`BUILDING.md` and `scripts/ledger-lint.py` of the product pathset, plus the
two ground-truth documents as intended; everything else in range is the
disjoint plan-7.5 close/salvage and help wording); no other tracked living
file consumed the deleted "five-part Rust gate" phrasing (grep empty, pattern
fired on the pre-fix blob); typography clean on all four changed files
(pattern set fired in the original pass); A1 untouched; plan :248 fence
byte-unchanged; the draft release was not queried or touched this round. The
cross-target clippy command itself was not re-run: no Rust source changed in
the delta, my original green + fire-proof stands, and the implementer's
re-run (green, then exit 101 with the gate removed, restored) corroborates
independently.

## Outstanding after this delta - none of it wave scope

Item E close-batch lines (controller): the `inline-wrapper-keeps-try-scope`
occurrence correction, the joblog fixture-invariant occurrence, the R8
license-dialog addendum, progress residual (b) marked done-by-verification,
the nine-item owner wording list, ROADMAP mirroring (9 triggers, rider DONE
notes, tenth-gate ruling record) - **plus the new F1 vehicle line above**.
Owner steps R8 and R10 remain pending by design, with the preserved draft
`rehearsal-30273529210` as their input.
