# Task A2 report - Plan 11, stream A (W2)

**Status:** DONE
**Commit:** `5d305a2a2902bfa91b8a392c4be2db089ad81f46` (`5d305a2`) on `plan-11-stream-a`
**Worktree:** `/home/senol/Git/muxsmith-plan11-a`, base `a0d5d3e` (Task A1)
**Date:** 2026-07-30 (`date -I` -> `2026-07-30`)

Every value below is pasted from the run that produced it. All runs foreground,
absolute paths, no session-relocation tool, no `git worktree`, no push.

---

## Step 1: re-measurement of both expressions on the PRE-state

### Expression A, filename citations

```
$ EXT=$(git ls-files | sed -n 's/.*\.\([A-Za-z0-9_]*\)$/\1/p' | sort -u | paste -sd'|')
$ echo "EXT=$EXT"
EXT=css|diff|ftl|gitattributes|gitignore|html|icns|ico|js|json|jsonc|lock|log|md|mjs|npmrc|png|py|rs|sh|snap|srt|toml|ts|txt|vue|wav|wxl|yaml|yml
$ git ls-files | grep -v '^docs/' | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+"
.github/workflows/ci.yml:90:      # correctness, so a broken intra-doc link (queue.rs:73, linking a
EXIT=0
```

**One line.** The alternation was derived from the tree by the first command in
the same shell, not typed from the plan; it reproduces the authoring run's
alternation character for character.

### Expression B, bare line spans

```
$ git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' \
  | while read -r f; do grep -nE '(^|[[:space:]`,(])[:][0-9]+' "$f" | sed "s|^|$f:|"; done
crates/muxsmith-core/tests/fixtures/all-non-default.yaml:2:# :1517-1535) set to a value that is NOT its default. A predicate that
DONE
```

**One line.**

### Verdict on the corpus

The re-measured set is **identical to the authoring run's**: exactly one hit each,
in exactly the two files the Files list names, both inside comments, both fenced
in the plan. No NEEDS_CONTEXT case arose - no hit in an unnamed file, no hit
outside a comment, no unfenced hit.

---

## Step 2: the two rewrites, with their targets opened first

### (a) `.github/workflows/ci.yml` - symbol derived from the citing commit's parent

**Which commit wrote the comment, measured rather than borrowed from the plan:**

```
$ git blame -L 88,93 -- .github/workflows/ci.yml
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 88)       # Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 89)       # #![deny(missing_docs)] already gates presence; it says nothing about
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 90)       # correctness, so a broken intra-doc link (queue.rs:73, linking a
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 91)       # private item) rotted silently since Plan 4 until this task. All
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 92)       # legs, matching the cross-target lint rule (cfg-gated items can
004e1e8b (Şenol Feldmann 2026-07-11 22:17:26 +0200 93)       # differ per platform).
```

The citing commit is `004e1e8`, as the plan states.

**`queue.rs` resolves to exactly one tracked file:**

```
$ git ls-files | grep -E '(^|/)queue\.rs$'
crates/muxsmith-core/src/executor/queue.rs
```

**TODAY, line 73 is the struct header, so naming what the line now holds would
name the wrong thing:**

```
$ awk 'NR>=70 && NR<=76 {printf "%d: %s\n", NR, $0}' crates/muxsmith-core/src/executor/queue.rs
70:
71: /// Queue policy (spec 6, D14).
72: #[derive(Debug, Clone, Copy)]
73: pub struct QueueOpts {
74:     /// Requested worker count; clamped to >= 1, then further capped at the
75:     /// batch's spec count (see the private `worker_count` helper) so a `--jobs` far larger
76:     /// than the batch never spawns idle OS threads. Default 1 (sequential).
```

**AT THE CITING COMMIT'S PARENT, line 73 IS the broken-link line:**

```
$ git show 004e1e8^:crates/muxsmith-core/src/executor/queue.rs | awk 'NR>=68 && NR<=80 {printf "%d: %s\n", NR, $0}'
68:
69: /// Queue policy (spec 6, D14).
70: #[derive(Debug, Clone, Copy)]
71: pub struct QueueOpts {
72:     /// Requested worker count; clamped to >= 1, then further capped at the
73:     /// batch's spec count (see [`worker_count`]) so a `--jobs` far larger
74:     /// than the batch never spawns idle OS threads. Default 1 (sequential).
75:     pub jobs: usize,
76:     /// Soft fail-fast (D14): on the first Failed, dequeue nothing further;
77:     /// in-flight jobs finish; queued jobs become Cancelled.
78:     pub fail_fast: bool,
79: }
80:
```

So line 73 sat inside the doc comment of the `jobs` field (line 75 declares
`pub jobs: usize,`) of `QueueOpts`, and that line carried `[`worker_count`]`.
This reproduces the authoring section exactly: parent line 71 is
`pub struct QueueOpts {`, line 72 the first doc-comment line, line 73 the link.

**`worker_count` is private at both states** (no `pub`), which is what makes
"private helper" true rather than assumed:

```
$ grep -n 'worker_count' crates/muxsmith-core/src/executor/queue.rs
75:    /// batch's spec count (see the private `worker_count` helper) so a `--jobs` far larger
188:    let workers = worker_count(opts.jobs, specs.len());
384:fn worker_count(jobs: usize, spec_count: usize) -> usize {
...
$ git show 004e1e8^:crates/muxsmith-core/src/executor/queue.rs | grep -n 'worker_count'
73:    /// batch's spec count (see [`worker_count`]) so a `--jobs` far larger
171:    let workers = worker_count(opts.jobs, specs.len());
299:fn worker_count(jobs: usize, spec_count: usize) -> usize {
```

**W2-c for this site:** the rewritten comment names `QueueOpts::jobs`'s doc
comment and the private `worker_count` helper, which is what the cited line
held at the moment the comment was written. The fenced replacement was applied
verbatim; nothing was composed.

### (b) `crates/muxsmith-core/tests/fixtures/all-non-default.yaml`

The surviving identifier `D48` is one the comment's own first token already
supplies. The precedent Plan 10 committed for the identical citation:

```
$ grep -n 'design D48\|defaulted fields' crates/muxsmith-core/tests/profile_save.rs
64:/// D48 guard 1: every one of the 17 defaulted fields set to a NON-default
95:/// defaulted fields (design D48), that the published JSON
```

`(design D48)` is byte-for-byte the form applied here.

**W2-c for this site:** `D48` is the design decision the fixture already names;
the count `17` was neither re-measured nor changed, per the plan's explicit
"Must not decide" clause.

---

## Step 3: scope boundary

No file under `docs/` was touched. The boundary runs by the artifact DOING the
citing, which is why the fixture's citation INTO a design document is swept
rather than exempted. `git diff --stat` below is the mechanical evidence.

---

## Step 4: verification, six checks

### 1. Absence check A - END state

```
$ EXT=$(git ls-files | sed -n 's/.*\.\([A-Za-z0-9_]*\)$/\1/p' | sort -u | paste -sd'|')
$ echo "EXT=$EXT"
EXT=css|diff|ftl|gitattributes|gitignore|html|icns|ico|js|json|jsonc|lock|log|md|mjs|npmrc|png|py|rs|sh|snap|srt|toml|ts|txt|vue|wav|wxl|yaml|yml
$ git ls-files | grep -v '^docs/' | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+"
grep exit=123
```

**GREEN: zero lines** (xargs exit 123 = every `grep` invocation returned
no-match; no output on stdout). The `EXT` alternation is byte-identical to the
pre-state run's, so the two runs are the same instrument.

**FIRE (RED):** Step 1's own pre-state run of this exact command returned
exactly **1** line. Same command, same tree, one line before and zero after -
the strongest available fire for an absence.

**SOUNDNESS CONTROL** (the pattern hits a filename-plus-line citation when one
is present):

```
$ grep -cE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+" docs/*.yaml
docs/conventions.yaml:11
docs/product-boundaries.yaml:1
docs/process-conventions.yaml:10
docs/decision-ledger.yaml:20
```

All four house files match, and the per-file values agree exactly with the
authoring section's (conventions 11, decision-ledger 20, process-conventions 10,
product-boundaries 1 - the shell's glob order differs from the prose's, the
numbers do not).

**W2-a satisfied.**

### 2. Absence check B - END state

```
$ git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' \
  | while read -r f; do grep -nE '(^|[[:space:]`,(])[:][0-9]+' "$f" | sed "s|^|$f:|"; done
B done (no lines above means empty)
```

**GREEN: zero lines.**

**FIRE (RED):** Step 1's own pre-state run of this exact expression returned
exactly **1** line.

**SOUNDNESS CONTROL:**

```
$ grep -cnE '(^|[[:space:]`,(])[:][0-9]+' docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
15
```

The expression sees a bare span when one is present.

**W2-b satisfied.**

### 3. The blind spot both expressions share - prose locators

Both patterns require a colon, so neither can see `line NNN` / `lines NNN`.

```
$ git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' | xargs grep -nEi '\blines? [0-9]+'
e2e/smoke.spec.ts:635:      line: "mkvmerge output line 1",
e2e/smoke.spec.ts:647:    await expect(jobs.getByTestId("live-log")).toContainText("mkvmerge output line 1");
exit=0
```

**Two hits, both classified line by line - this list IS the full output above,
not a summary of it:**

| # | hit | classification |
|---|---|---|
| 1 | `e2e/smoke.spec.ts:635` `      line: "mkvmerge output line 1",` | **Test DATA.** A string literal inside a fake-mkvmerge output record the harness feeds the live log. The words "line 1" are the simulated tool's own output text, not a locator pointing at any file. Not a citation. |
| 2 | `e2e/smoke.spec.ts:647` `    await expect(jobs.getByTestId("live-log")).toContainText("mkvmerge output line 1");` | **Test DATA.** The assertion that the same literal from hit 1 reaches the live-log widget. Not a citation. |

No hit is a citation, so no NEEDS_CONTEXT arises from this sweep. The sweep's
own soundness needs no separate control: it returned a non-empty result, which
is itself proof that it looks at the tree.

### 4. The workflow is still valid YAML

```
$ python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"
yaml exit=0
```

**Stronger check added on top of the prescribed one** (an addition, not a
substitution): the parsed workflow is compared object-for-object against
`HEAD`'s, so a comment that swallowed a step would be caught rather than merely
parsing:

```
$ git show HEAD:.github/workflows/ci.yml > <scratch>/ci-head.yml
$ python3 -c "
import yaml
a = yaml.safe_load(open('<scratch>/ci-head.yml'))
b = yaml.safe_load(open('.github/workflows/ci.yml'))
print('semantically identical to HEAD:', a == b)
"
semantically identical to HEAD: True
```

(Run before the commit, so `HEAD` was `a0d5d3e`, the pre-edit state.)

**W2-d satisfied.**

### 5. Comment-only, in both files

```
$ git diff -U0 -- .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml
diff --git a/.github/workflows/ci.yml b/.github/workflows/ci.yml
index 278bc54..faa84a5 100644
--- a/.github/workflows/ci.yml
+++ b/.github/workflows/ci.yml
@@ -90,2 +90,4 @@ jobs:
-      # correctness, so a broken intra-doc link (queue.rs:73, linking a
-      # private item) rotted silently since Plan 4 until this task. All
+      # correctness, so a broken intra-doc link (in `QueueOpts::jobs`'s doc
+      # comment in crates/muxsmith-core/src/executor/queue.rs, which then
+      # linked the private `worker_count` helper) rotted silently since
+      # Plan 4 until this task. All
diff --git a/crates/muxsmith-core/tests/fixtures/all-non-default.yaml b/crates/muxsmith-core/tests/fixtures/all-non-default.yaml
index 7872f6c..42a9a18 100644
--- a/crates/muxsmith-core/tests/fixtures/all-non-default.yaml
+++ b/crates/muxsmith-core/tests/fixtures/all-non-default.yaml
@@ -1,2 +1,2 @@
-# D48 guard 1 fixture: every one of the 17 defaulted fields (design
-# :1517-1535) set to a value that is NOT its default. A predicate that
+# D48 guard 1 fixture: every one of the 17 defaulted fields (design D48)
+# set to a value that is NOT its default. A predicate that
```

The hunk header `@@ -90,2 +90,4 @@` on `ci.yml` shows the edit is confined to
the two authorized lines; the following comment lines (`# legs, matching the
cross-target lint rule ...`) are outside the hunk and unchanged.

**The comment-only property asserted mechanically rather than by eye, with its
fire:**

```
$ git diff -U0 -- .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml \
  | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' | sed -E 's/^[+-]//' | grep -vnE '^[[:space:]]*#'
classifier exit=1
```

Zero changed lines fail the comment test.

```
$ printf 'diff --git a/x b/x\n--- a/x\n+++ b/x\n@@ -1 +1 @@\n-      # a comment\n+      run: cargo doc\n' \
  | grep -E '^[+-]' | grep -vE '^(\+\+\+|---)' | sed -E 's/^[+-]//' | grep -vnE '^[[:space:]]*#'
2:      run: cargo doc
control exit=0
```

The classifier demonstrably reports a non-comment changed line when one is
present, so its empty result above is a finding rather than a malformed filter.

```
$ git diff --stat
 .github/workflows/ci.yml                                 | 6 ++++--
 crates/muxsmith-core/tests/fixtures/all-non-default.yaml | 4 ++--
 2 files changed, 6 insertions(+), 4 deletions(-)
```

**Exactly two files.**

**Exit-bar subset - `cargo test --workspace` green:**

```
$ cargo test --workspace >/dev/null 2>&1; echo "cargo test exit=$?"
cargo test exit=0
```

The D48 guard that consumes the edited fixture ran and passed:

```
$ cargo test --workspace 2>&1 | grep -E 'all_non_default_fields_survive_the_round_trip'
test all_non_default_fields_survive_the_round_trip ... ok
```

The only two lines in the whole run matching `FAILED|panicked` are test NAMES,
both passing, not failures:

```
$ cargo test --workspace 2>&1 | grep -n 'FAILED\|panicked'
7:test commands::run::tests::finished_panicked_renders_two_lines_without_na ... ok
350:test panicked_outcome_persists_its_payload_on_the_job_record ... ok
```

**W2-e satisfied.**

### 6. Test duty, weighed

**No new test**, and the plan's own reasoning is adopted rather than re-derived:
this task changes comment text in a workflow and in a test fixture and produces
no user-visible consequence. The fixture's DATA, which does have observable
consequences, is asserted unchanged by the comment-only `git diff -U0` above,
not by the test run - `all_non_default_fields_survive_the_round_trip` proves
round-trip self-consistency, so a data change that still round-trips would pass
it. The one newly observable property, that the workflow still parses, got the
explicit check above (plus the HEAD-comparison strengthening) rather than a
test, because the repository has no harness that loads workflow YAML and adding
one would be new test infrastructure - the one exemption
`tests-ship-with-the-feature-never-after` leaves standing.

### Typography check on the edited regions

```
$ sed -n '88,95p' .github/workflows/ci.yml | LC_ALL=C grep -nP '[^\x00-\x7F]'
ci exit=1
$ sed -n '1,4p' crates/muxsmith-core/tests/fixtures/all-non-default.yaml | LC_ALL=C grep -nP '[^\x00-\x7F]'
fixture exit=1
$ LC_ALL=C grep -cP '[^\x00-\x7F]' docs/ROADMAP.md      # fired control
33
```

ASCII-only in both edited regions; the probe demonstrably fires on a file that
does contain non-ASCII.

---

## The BINDING CROSS-TASK CONSTRAINT: the mise rider was NOT consumed

`docs/ROADMAP.md`'s "Remove mise from CI (post-1.0)" entry carries a rider gated
on "the next `ci.yml`-touching change whichever it is - the edit is the trigger",
whose fenced replacement text targets the very comment block this task edits.
Its text contains `BUILDING.md, Rust gate part 6`.

**It was not applied.** Task A1 (`a0d5d3e`) removed `BUILDING.md`'s positional
gate ordinals earlier today, so applying the rider as written would have written
that ordinal back into `ci.yml` - re-creating in another file the construction
this plan just deleted, invisibly to every check this task runs.

**That premise was verified rather than borrowed from the dispatch**, since it is
the whole reason the rider is skipped:

```
$ grep -nE 'part [0-9]|parts [0-9]' BUILDING.md          # end state
grep exit=1 (no ordinals remain)
$ git show a0d5d3e^:BUILDING.md | grep -nE 'part [0-9]|parts [0-9]'   # fired control
102:The cross-target clippy run (part 6) type-checks the workspace for Windows
134:CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
135:three OS legs (its Windows leg covers natively what part 6 cross-checks
control exit=0
```

Three positional ordinals existed before A1 and none exists now, so the rider's
`BUILDING.md, Rust gate part 6` would have re-introduced the exact construction
into a second file.

**Evidence that the rider's text is absent from `ci.yml`:**

```
$ grep -n 'gate part\|BUILDING' .github/workflows/ci.yml
88:      # Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.
99:      # quietly. Kept identical to BUILDING.md's gate block - a single-site
174:    # ids, per-entry duplicate keys, plus BUILDING.md's gate-count
```

All three are **pre-existing and untouched** (they are outside both diff hunks).
Line 88 is the dated Plan-5.5 provenance statement the plan's authoring section
records as a measured NON-defect. The rider's string `Rust gate part 6` appears
nowhere in `ci.yml`. Fired control for that grep family, showing it can find the
string when present:

```
$ grep -c 'Rust gate part 6' /home/senol/Git/Muxsmith/docs/ROADMAP.md
2
```

**The comment block as committed, lines 88 to 95:**

```
88:       # Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.
89:       # #![deny(missing_docs)] already gates presence; it says nothing about
90:       # correctness, so a broken intra-doc link (in `QueueOpts::jobs`'s doc
91:       # comment in crates/muxsmith-core/src/executor/queue.rs, which then
92:       # linked the private `worker_count` helper) rotted silently since
93:       # Plan 4 until this task. All
94:       # legs, matching the cross-target lint rule (cfg-gated items can
95:       # differ per platform).
```

Lines 94 and 95 are the rider's target and are byte-identical to their
pre-state.

The controller's record of the re-deferral is present on `master`
(`/home/senol/Git/Muxsmith/docs/ROADMAP.md`, in the "Remove mise from CI"
entry): "FIRED 2026-07-30 at Plan 11 Task A2 ... deliberately RE-DEFERRED rather
than consumed", with the new observable being the next change that edits
`ci.yml`'s Plan-5.5 comment block for its own reasons.

---

## Step 5: SURFACED, not edited (W2-f)

The tracker entry is in `docs/ROADMAP.md`, section **"Docs accuracy"**, the
bullet beginning **"ONE member of the comment line-citation class SURVIVES Plan
10's sweep, and it is outside the corpus's file selector"** (Task-5 review
finding 1, 2026-07-29). Two things in it are now stale, both controller close
actions:

1. **Its "OPEN OWNER QUESTION" paragraph.** It reads, quoted from the entry:
   "**OPEN OWNER QUESTION riding the same line:** the ruling
   `comments-locate-by-symbol-never-by-line-number` was scoped by the owner to
   SOURCE comments and explicitly not widened, and its comment-form enumeration
   names `//`, `///`, `//!` and `/* */` - not `#` or `<!-- -->`. Whether a CI
   workflow comment is in scope is his call, not the controller's."
   The owner answered it in session 28. The answer is recorded in the Tier-2
   statement itself (`docs/conventions.yaml`,
   `comments-locate-by-symbol-never-by-line-number`): "WIDENED BY OWNER RULING
   2026-07-29 (session 28): the rule reaches CI and CONFIGURATION comments too,
   not only the source forms // /// //! and /* */ - a # comment in a workflow or
   config file is in scope on the same reasoning, since a line number rots there
   exactly as it does in source."
   The paragraph now poses as open a question that is closed, and it cites the
   pre-widening enumeration as current.

2. **Its ONE-surviving-member claim.** The entry's headline says "ONE member ...
   SURVIVES" and its body describes a single `ci.yml` hit ("exactly one hit,
   fired control 1, negative control 0"). The derivation returns **two**:
   expression A finds the `ci.yml` hit, and expression B - which expression A
   cannot see, because a bare span carries no filename - finds
   `crates/muxsmith-core/tests/fixtures/all-non-default.yaml:2`. Both are
   reproduced in Step 1 of this report. The reviewer's original run used
   expression A only, which is exactly how the second member escaped.

**A third observation on the same entry, offered because it is the same defect
class one level up:** the plan and the task brief both locate this entry as
"`docs/ROADMAP.md`'s 'Docs accuracy' **first** entry". It is no longer first.
The README-example-profile entry ("THE README'S FIRST EXAMPLE PROFILE DOES NOT
LOAD") was deliberately filed at the top of that section on 2026-07-30, so the
line-citation entry is now the section's **second** bullet. A positional
locator drifted while the plan was being written. Naming the entry by its
opening words, as this report does, survives the next insertion; naming it by
its ordinal does not. Controller's call whether that is worth a house-knowledge
occurrence.

---

## Step 6: commit

```
$ git add .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml
$ git -c commit.gpgsign=false commit -m "comments: locate code by symbol in the CI workflow and the D48 fixture, closing the class outside docs/" -m "Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>" -- .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml
[plan-11-stream-a 5d305a2] comments: locate code by symbol in the CI workflow and the D48 fixture, closing the class outside docs/
 2 files changed, 6 insertions(+), 4 deletions(-)
```

Verification of the commit:

```
$ git log -1 --format='%H%n%s%n---%n%b'
5d305a2a2902bfa91b8a392c4be2db089ad81f46
comments: locate code by symbol in the CI workflow and the D48 fixture, closing the class outside docs/
---
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git status --porcelain
(empty)
$ git rev-parse --abbrev-ref HEAD
plan-11-stream-a
$ git log -1 --format='%G?'
N
```

Exactly one trailer, no `Claude-Session` line, no context-window suffix,
unsigned (`%G?` -> `N`), staging explicit, branch correct, tree clean, not
pushed. The standing grant (SI-4) was verified in the repo's own house knowledge
rather than taken from the dispatch alone:
`docs/decision-ledger.yaml`, id `dispatch-restates-the-standing-commit-grant`.

---

## Acceptance map coverage

| row | state | producer in this report |
|---|---|---|
| W2-a | GREEN | Step 4 check 1: pre-state 1 line, end state 0, soundness control matching in all four house YAML files |
| W2-b | GREEN | Step 4 check 2: pre-state 1 line, end state 0, soundness control 15 matches in the Plan-10 document |
| W2-c | GREEN | Step 2: both targets opened, `QueueOpts::jobs` and the private `worker_count` quoted from `004e1e8^`; `D48` quoted from the fixture's own first token and from the `profile_save.rs` precedent |
| W2-d | GREEN | Step 4 check 4: `yaml.safe_load` exit 0, plus object-equality against `HEAD` |
| W2-e | GREEN | Step 4 check 5: `git diff -U0` pasted in full, mechanical comment-only classifier with its fire, `git diff --stat` naming exactly two files, `cargo test --workspace` exit 0 |
| W2-f | GREEN | Step 5 above, both stale claims quoted from the entry |

---

## Noticed, not touched

- **`crates/muxsmith-core/src/executor/queue.rs`'s longest doc-comment line in
  `QueueOpts::jobs` is 91 characters** (`awk 'NR==75 {print length($0)}'` -> 91;
  fired control, line 71 -> 31), a
  result of the earlier repair of this same class (the `[`worker_count`]` link
  became "the private `worker_count` helper" in prose). No Rust line-length gate
  exists (`cargo fmt` does not rewrap comments and no `rustfmt.toml` sets
  `wrap_comments`), so this is cosmetic and outside this task's Files list.
  Recorded only so nobody mistakes it for a defect this task introduced.
- **`ci.yml:88`'s "the ninth gate part"** is a dated Plan-5.5 provenance
  statement, which the plan's own authoring section already records as a
  measured NON-defect ("a record is not falsified to today's count"). Left
  untouched deliberately.
- **This worktree's `docs/ROADMAP.md` predates the controller's re-deferral
  note.** The branch base `a0d5d3e` carries the "Remove mise from CI" entry
  without the "FIRED 2026-07-30 ... RE-DEFERRED" paragraph, which exists on
  `master` in the main worktree. The branch will pick it up at merge. Flagged
  only so a reviewer grading against this worktree's ROADMAP is not surprised by
  its absence.
- **`python3 scripts/ledger-lint.py` was deliberately NOT run and is NOT cited
  as coverage** for this task, per the constraint carried from Task A1's review:
  that gate part reads four markers, the fenced command lines and one sentence
  in `BUILDING.md`, and is blind to every comment region this task touched.
- The full gate was not run; per the brief that is the controller's dispatch
  before the stream merges, not this task's.
