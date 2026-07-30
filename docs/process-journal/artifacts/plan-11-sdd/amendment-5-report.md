# Amendment 5 report - Plan 11

**Status: DONE.** One file edited,
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
in the main worktree on `master`, committed with an explicit pathspec.

**Base:** `master` at `dd64e7a442435bec53d664dca1d2ea5778330e18`, clean tree at
start, nothing staged by anyone else at commit time. `deny.toml` and the plan
document were both verified byte-identical to `5378264` before any edit
(`git diff --stat 5378264 HEAD -- <path>` empty for both), so every measurement
below was taken against the same pre-state the A1 and B1 verdicts measured.

**Scale:** one-pair, no task added, removed or re-cut. 40 acceptance rows before
and after; 8 deferred-by-decision rows before and after; 6 corrections rows
before and after; task set A1-A4 plus B1 unchanged; B1's work unchanged.

---

## 1. Every site changed, with its reason

Line numbers are the pre-amendment ones (at `dd64e7a`), given for locating the
edits in the diff, not as durable citations.

| # | Site (pre-amendment) | Defect | What changed |
|---|---|---|---|
| 1 | Authoring, `cargo deny` block, mechanism bullet (`:91`) | 4 + 3 | The `workspace`-scope mechanism restated at the filter (`direct_dependents`, `ws_set.contains(...) ^ transitive`) instead of "glib is an external crate"; the class breakdown corrected to 18 ignore-entry / 16 unmaintained / 2 vulnerability, with the vulnerability case explained (no `informational` class, so no scope key) |
| 2 | Authoring, `cargo deny` block, new bullet after `:88` | 8 | The local-vs-CI cargo-deny version skew recorded, with the byte-identity that makes it harmless here and the changelog history that makes it worth recording |
| 3 | Authoring, `glib` block (`:97`) | 6 | The twelfth-consumer caveat replaced by the four measurements that refute it; "eleven is the complete direct-consumer set" stated; the unit kept |
| 4 | Authoring, item-5 block (`:151`) | 2 | "The one live consumer" replaced by a two-member enumeration with its derivation, its discriminator, and a classification of every non-consumer hit |
| 5 | Corrections table, row 4 (`:168`) | 4 | "glib is external, so the default scope excludes it" corrected to the direct-dependents ground |
| 6 | Acceptance row W1-g (`:220`) | 4 | Same shorthand corrected inside the row's producer clause |
| 7 | Acceptance row W1-l (`:225`) | 6 | Proc-macro parenthetical replaced; unit kept with its reason |
| 8 | Task A1 Step 3, absence check O (`:343`) | 1 | Soundness control re-pointed at the plan-10 document (2 lines), with plan-5.5's real role named and the unit fixed to lines |
| 9 | Task B1 Step 4 premise (`:706`) | 4 | Same shorthand corrected |
| 10 | Task B1 Step 4, fence A (`:711-718`) | 3 + 4 | Fence replaced whole with the B1 reviewer's wording, plus a paragraph recording what it replaces, why the count is dropped rather than corrected, and the artifact check |
| 11 | Task B1 Step 5 (`:741`) | item 7 | The unperformable `git diff --exit-code -- deny.toml` "clean" replaced by a `sha256sum` baseline taken after Step 4 and re-taken after each variant run |
| 12 | Task B1 Step 8 (`:758`) | 6 | Twelfth-consumer sentence replaced by the completeness statement |
| 13 | Task B1 Step 9, test duty (`:763`) | 5 | Refuted premise replaced by the three measured guard states |
| 14 | Plan close, ROADMAP dispositions (`:787`) | 5 | The residue line now carries the measurement and records a parked owner decision rather than an infrastructure cost |
| 15 | Deferred-by-decision, guard row (`:804`) | 5 | Why and Vehicle rewritten; every restatement in the row fixed. **This cell said "both restatements" and the measured answer is three, two in Why and one in Vehicle - corrected at fix round 1, below; the repair was complete either way** |
| 16 | Self-review, brief refutations (`:925`) | 4 | Same shorthand corrected |
| 17 | Self-review, gate-count audit kind 3 (`:915`) | audit duty | Kind 3's example list extended to name the hits this amendment added |
| 18 | New `## Amendment 5` section, before the self-review | deliverable | Routing, scale, what moved, how each set was established, every figure re-derived, what was deliberately left alone |

## 2. How each multi-site set was established

By searching for the fact and classifying every hit, never by extending a list.
The brief names defects 3, 4 and 6 as the multi-site ones; **the search found a
fourth**, defect 5.

- **The `external`-crate shorthand (defect 4).**
  `/usr/bin/grep -nE 'glib. (is )?(an )?external|external crate'` against the
  pre-amendment document: **6 lines** - the shipped fence plus five prose
  restatements (authoring block, corrections row 4, W1-g, B1 Step 4,
  self-review). All six corrected. A post-edit re-run returns only corrected
  text and the records of the correction.
- **The misattributed 18 (defect 3).** Every occurrence of the bare token `18`
  in the document, extracted with 70 characters of context on each side and
  classified by hand into ignore-entry uses (correct: the `grep -c` figure, the
  `advisory-ignored` notes, the 18 -> 19 blast-radius line, the recomputed-counts
  line, the self-review reconciliation) and unmaintained-CLASS uses (wrong).
  **Exactly two sites attributed it to the class**: the fence and the authoring
  sentence that restates the fence in its own words. Both fixed.
- **The proc-macro caveat (defect 6).**
  `/usr/bin/grep -nE 'proc-macro|glib-macros|twelfth|welve|normal edge'` plus
  `grep -n 'inclusion graph'`. The expression is deliberately over-wide and
  returns **24 lines** pre-amendment, mostly A3's twelve repair sites and the
  spelled-ordinal alternation's own `twelfth`. Classified: **three live sites**
  (authoring block, W1-l, B1 Step 8), all fixed, plus **one** dated record in
  Amendment 1's figure-correction bullet, left standing (verified to be the only
  amendment-log site by grepping the Amendment 1-4 range separately).
- **The live-consumer set (defect 2).** `git grep -n 'gate part 6'` over the
  whole **tree**, because the set the claim quantifies over lives there.
  **12 lines across 6 files on the pre-amendment tree**, every one classified:
  **2 live consumers** (`docs/process-conventions.yaml`'s Tier-2 statement;
  `docs/ROADMAP.md`'s mise rider, whose fenced text is forward-looking),
  **2 dated ROADMAP records** (the `bcb67f3` provenance line and the
  re-deferral's own record), **1 ledger specimen** (Tier-1
  `a-positional-ordinal-into-a-list-is-a-reference-that-drifts`, controller-ruled
  and reviewer-concurred not stale), **3 lines in 2 archived process-journal
  artifacts**, and **4 lines in the plan document itself**, which is the subject
  rather than a consumer. The same command returns 16 after the amendment, the
  delta being the plan's own enumeration growing from 4 lines to 8 - which is why
  the figure is stated with its date and its tree. A wider run
  (`git grep -nE 'parts? [0-9]' -- ':!docs/superpowers/plans' ':!docs/process-journal*'`
  filtered for `building`) returns the same two consumers plus `BUILDING.md`
  itself and the ledger specimen, so the narrower expression is not hiding a
  member.
- **The guard premise (defect 5), where searching beat the brief's own list.**
  `/usr/bin/grep -nE 'permanent guard|permanently guard|new gate infrastructure|silent|unused-ignored'`
  returns the refuted premise on **two lines** and in **four restatements**:
  B1 Step 9's test-duty bullet carries one, and the deferral row carries three
  across its columns (two in Why, one in Vehicle), which a line-based grep prints
  as a single line. The plan-close ROADMAP disposition is a consumer of the row's
  routing and asserts neither half. Recorded in the amendment because the unit is
  the restatement, not the line.
  **Corrected at fix round 1: this bullet first said three lines and two
  restatements-in-the-row** - both terms wrong, the total of four right by
  compensating errors, in the passage arguing that the line is the wrong unit.
  The repair was complete in either accounting; the accounting was not. The
  measurement per cell is in the fix-round section below.

## 3. Every figure re-derived, with its run

All run in `/home/senol/Git/Muxsmith` at `dd64e7a`, foreground, absolute paths.
Variant configs were written to the session scratchpad, never into the repo;
`git diff --exit-code -- deny.toml` exited 0 after every cargo-deny run.

**Defect 1 - the control target.**

```
$ /usr/bin/grep -cE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md
2
$ /usr/bin/grep -oE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md | wc -l
5
$ /usr/bin/grep -cE 'part [0-9]|parts [0-9]' docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
0
$ /usr/bin/grep -cEio '(first|...|twelfth)[ -](gate|part)|(gate|part) (one|...|twelve)\b' <plan-5.5>
6
```

The A1 reviewer's replacement text says the plan-10 file "returns 2 matches".
**Set to 2 LINES in the plan**, because absence check O's own red and green
states are line counts (`RED: exactly 3 lines`), and the same run yields 5 match
occurrences. That is the one place I departed from the verdict's wording, and
only in the unit.

**Defects 3 and 4 - the class tally and the mechanism.**

```
$ cargo deny --version
cargo-deny 0.19.9
$ cargo deny -L info check advisories | grep -oE '^note\[[a-z-]+\]' | sort | uniq -c
     18 note[advisory-ignored]
     16 note[unmaintained]
      2 note[vulnerability]
$ cargo deny check advisories --show-stats | tail -1
 advisories ok: 0 errors, 0 warnings, 36 notes
```

Id sets extracted per class from the same run:
`advisory-ignored` = 18 ids; `unmaintained` = 16; `vulnerability` =
`{RUSTSEC-2026-0194, RUSTSEC-2026-0195}`; and
`unmaintained | vulnerability == advisory-ignored` is **True** as a set
comparison, not as arithmetic.

Mechanism read at the tool's source
(`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/`):
`src/advisories.rs` maps only `Informational::Unmaintained` and
`Informational::Unsound` to a scope and lets everything else fall through to the
emit branch; the `Scope::Workspace | Scope::Transitive` arm computes
`ctx.krates.direct_dependents(nid)` and tests
`ws_set.contains(&dd.krate.id) ^ transitive`. `src/advisories/cfg.rs`'s `Default`
sets `unmaintained: Scope::All`, `unsound: Scope::Workspace`. Workspace members
from `cargo metadata --no-deps`: `muxsmith-core`, `muxsmith-cli`, `xtask`,
`muxsmith-gui` - none of them among glib's eleven direct dependents.

Counterfactual, scope on and no glib ignore, scratch config:

```
error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
   ID: RUSTSEC-2024-0429
advisories FAILED            EXIT=1
```

**Defect 6 - the glib edges.**

```
$ cargo tree -i glib@0.18.5 -e normal --depth 1        -> glib + 11 parents (atk, cairo-rs, gdk,
    gdk-pixbuf, gdkx11, gio, gtk, javascriptcore-rs, pango, soup3, webkit2gtk)
$ for k in normal build dev all; do cargo tree -i glib@0.18.5 -e $k --depth 1 | grep -c glib-macros; done
0 0 0 0
$ cargo tree -p glib@0.18.5 -e normal --depth 1 | grep -i macro
+-- glib-macros v0.18.5 (proc-macro)
$ cargo tree -i glib@0.18.5 -e build     -> warning: nothing to print.
$ cargo tree -i glib@0.18.5 -e dev       -> warning: nothing to print.
$ cargo tree -i quick-xml@0.39.4 -e normal --depth 1
quick-xml v0.39.4
+-- wayland-scanner v0.31.10 (proc-macro)     # fire control: a proc-macro PARENT survives -e normal
$ cargo tree -i quick-xml@0.39.4 -e build -> warning: nothing to print.   # control for the empty results
$ cargo tree --help | grep -A3 -- '-e, --edges'
[possible values: all, normal, build, dev, features, public, no-normal, no-build, no-dev, no-proc-macro]
```

cargo-deny's own inclusion graph from the counterfactual run: first level is the
same **11** crates; `grep -c glib-macros` over the entire diagnostic returns
**0**. So the plan's claim that glib-macros "appears in cargo-deny's inclusion
graph" is false too - a fourth wrongness beyond the three the brief enumerates,
and it is recorded as such.

**Defect 5 - the three guard states**, scratch configs built from `master`'s
`deny.toml` plus the glib ignore entry:

```
key dropped, ignore kept, defaults      -> warning[advisory-not-detected], "no crate matched
                                           advisory criteria", advisories ok,     EXIT 0
same + unused-ignored-advisory = "deny" -> error[advisory-not-detected], advisories FAILED, EXIT 1
scope on + ignore + that key            -> advisories ok,                          EXIT 0
```

The middle run is also the fired control for the key itself: the exit code moves
from 0 to 1 purely by adding it, so the key is read rather than ignored.

**Item 8 - the version skew.** `.github/workflows/ci.yml` pins
`EmbarkStudios/cargo-deny-action@bb137d7af7e4fb67e5f82a49c4fce4fad40782fe`;
`WebFetch` of that SHA's `Dockerfile` returns `ENV deny_version="0.19.8"`.
Downloaded `cargo-deny-0.19.8.crate` from static.crates.io and compared:
`src/advisories/cfg.rs` byte-identical to 0.19.9's,
`d8d4356d22f066e71e07878b93850f9ad4c81d6cbb68ca2f10aaeb8f8d1871bf` on both. The
in-crate `CHANGELOG.md` supplies the history the record rests on:
`[advisories.unsound]` added at **0.19.0** (2026-01-08, PR#826, defaulting to
`workspace`); the identically named lint-level fields removed at **0.16.0**;
`unmaintained` reintroduced as a scope-valued key at **0.18.2**. The B1 verdict's
"were `LintLevel` before becoming `Scope`" framing was not carried over unchecked
- the changelog says it more precisely and that is what the plan now states.

**The replacement fence against the artifact it lands in.** 11 lines, max width
**76**, zero non-ASCII codepoints, zero glyphs from the house denylist.
`deny.toml`'s own maximum line width measured **77** (its licence-comment line),
not 78 as the B1 verdict's parenthetical says - immaterial to the fence, recorded
because an unchecked figure is the recurring defect in this document's history.
Both insertion anchors still occur exactly once in `deny.toml`
(`^yanked = "deny"$`, `^# All entries below are transitive`), as does fence B's
anchor.

## 4. The plan's own audits, re-run

- **Gate-count audit** (the document's own expression, executed verbatim from the
  document rather than retyped): **26 hit lines before, 30 after**. All four new
  hits are kind 3 - three from the live-consumer bullet growing from one line to
  four, one from Amendment 5 recording the expression it derived that set with.
  Kind 3's example list was extended to name both, since the previous list said
  "quoted in the authoring section" and the new one sits in the amendment log.
  Controls outside the document still discriminate: **4** against `BUILDING.md`,
  **0** against `renovate.jsonc`.
- **Placeholder audit:** `TBD`, `TODO`, `appropriate`, `similar to`,
  `and so on`, `etc.`, `as needed` each still return exactly **1** line, the
  self-review sentence that lists them.
- **Ellipsis kinds:** `...` returned 12 lines before and 12 after. Two of my
  first drafts added a thirteenth and fourteenth - a truncated sha256 and an
  elided quotation whose full text no longer exists in the document - and both
  were rewritten rather than left to break the self-review's three-kinds claim.
- **Typography:** zero non-ASCII codepoints in the whole file, zero trailing
  whitespace, fence-marker count unchanged at 58, all edited table rows still
  have their exact column count.
- **Counts:** 40 acceptance rows, 8 deferred rows, 6 corrections rows, unchanged.
  No count in the self-review's recomputed-counts paragraph is affected, checked
  member by member (the `glib` eleven, the ignore 18 -> 19, and the acceptance
  totals all describe sets this amendment did not touch).

## 5. Found and deliberately NOT changed

- **Amendments 1 to 4 keep their dated records.** Exactly one of them restates
  the void proc-macro caveat (Amendment 1's figure-correction bullet, measured -
  not two, as my own first draft of the amendment said before I grepped the
  amendment range). It is a true record of what round 1 wrote and a false
  statement about the tree; the plan's MEASURED-block principle keeps it, and
  Amendment 5 names it so a reader meets the disposition beside it.
- **Acceptance row W5-e and Task A1 Step 4's surfacing list keep their
  single-consumer shape.** A1 is committed and reviewed, its reviewer ruled W5-e
  satisfied by what was delivered, and its implementer surfaced the second
  consumer anyway (report 5.4) under `proc-sweep-surface-completeness`. Widening
  the row would have named a producer step that never named the member.
- **The mise rider is not added to the plan-close surfacing list.** That list is
  explicitly what was surfaced at plan-AUTHORING; this was found at execution and
  the controller already disposed of it (FIRED and deliberately RE-DEFERRED,
  visible in `docs/ROADMAP.md` on `master`). Adding it would misdate it and move
  a count over a set that did not change.
- **"cargo-deny's default scope for the unsound class excludes transitive
  dependencies" stays**, at both sites that say it (authoring bullet, B1 Step 6).
  Checked rather than assumed: `Scope::Transitive` is literally the complement of
  `Scope::Workspace` in the same filter expression, so the sentence uses the
  tool's own vocabulary and is true of this tree. Only the form keyed on the
  crate being EXTERNAL was false, and only that form was replaced.
- **The authoring section's own `git diff --exit-code -- deny.toml` clean claim
  stays.** It describes the authoring-time runs, when the repo file was unedited
  - the same condition under which I re-ran it today, exit 0. Only B1 Step 5's
  copy, which runs after Step 4 has edited the file, was unperformable.
- **Fence B (the ignore entry's comment) is untouched.** Its claims were checked
  against the RustSec record and `cargo tree` and none of them is affected by the
  corrections above.
- **Nothing compressed.** The standing ruling is that this plan's meta-text is
  compressed at the plan close, not mid-execution.

## 6. Concerns for the controller

1. **A B1 fix round is owed and its input is now in the plan.** The replacement
   fence A is what it applies to `deny.toml`; the shipped `c422999` still carries
   the false comment. The B1 reviewer's recommendation is that this lands before
   stream B merges.
2. **The parked one-key decision needs to reach the owner's batch.** The deferral
   row states it, but the row is not the batch; whoever assembles the parked
   questions should pick it up with its measurement (the three guard states are
   in this report and in the row).
3. **The plan close's "7 controller-surfaced items" list stayed at 7 by design.**
   If the controller prefers the mise rider recorded there as an eighth despite
   its execution-time provenance, that is a count change with a consumer in the
   self-review's recomputed-counts paragraph, and it is a controller call rather
   than mine.
4. **`docs/ROADMAP.md` asserts "the 18 commented RUSTSEC ignores" at two sites**
   that B1's nineteenth entry makes stale (tracker item 8). Not touched here - the
   ROADMAP is controller-owned - and the amendment does not restate the figure.
   **The instrument matters and the concern now names it** (delta review,
   dimension 8): the figure and the noun phrase straddle a hard wrap at both
   sites, so `grep -c '18 commented' docs/ROADMAP.md` returns **0** and a
   newline-flattened pass returns **2**. Both assertions straddle the wrap:
   `docs/ROADMAP.md:643-644` ends `:643` with "Then walk the 18" and opens `:644`
   with "commented RUSTSEC ignores in `deny.toml`", and `:1439-1440` does the same
   with "prune the 18" / "commented RUSTSEC ignores in deny.toml". The delta review's line-based check
   was aimed at the form that cannot appear here; the flattened pass is the same
   handle B1's implementer built for this exact fact, and it reproduces. The
   concern stands as substance; its earlier stated form did not name which
   instrument sees it.
5. **WITHDRAWN at fix round 1.** This concern said the B1 verdict's "the file's
   existing maximum is 78" was off by one against a measured 77. **The verdict is
   right and this report was wrong**: `deny.toml`'s longest line is 77 on
   `master` and **78 at `c422999`**, the state the verdict reviewed and the state
   the fence lands in, and the three 78-character lines are ones B1's own second
   fence added. Two measurements of two states, not a disagreement. The escalation
   is retracted, the plan now names both states beside the figure, and the rule it
   cost is recorded there: a figure that disagrees with a source figure is a state
   or a unit question until reconciled.
6. **Two delta-review minors live outside this file and are the controller's to
   route**, recorded here so they do not vanish at the file boundary. (a) The B1
   verdict required `task-b1-report.md`'s "silent by construction" sentence to be
   corrected and its Step-5 contradiction to be recorded as a third numbered
   finding; neither has happened. (b) A1 adjudication 5 requires the plan close to
   reconcile the "A neighbouring class" ROADMAP entry's **Vehicle** line
   ("whichever package next edits `BUILDING.md`'s gate blocks after Plan 10's Task
   1 lands") against what A1 actually landed, which was prose and no gate block.
   For (a) the disposition is a genuine question rather than a formality: a task
   report is a dated record of what an implementer claimed, so correcting it,
   annotating it as superseded, and leaving it under the MEASURED-block principle
   are all defensible, and the verdict asked for the first.

---

# Fix round 1 (2026-07-30)

**Routing:** `.superpowers/sdd/plan-11/amendment-5-verdict.md`, NEEDS_FIXES, two
Important and one Minor addressed to this author (its findings 4 and 5 address
files this author may not edit and are the controller's to route; both are now
named in concern 6 above). **Status: all three fixed.** Same file, same rules:
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
only, neither worktree entered, `c422999`'s `deny.toml` read as a blob with
`git show`.

## Finding 1 - the fenced instruction's operation, re-derived against the state it runs in

**Reproduced before editing, on my own copies built from the `c422999` blob:**

```
c422999 deny.toml, 0-based:  line 5 = yanked = "deny"            [1 occurrence]
                             lines 6-13 = B1's shipped fence     [8 lines, incl. unsound = "all" at 13]
                             line 14 = # All entries below are transitive   [1 occurrence]

literal INSERT after line 5   -> 2 lines matching ^unsound =
  $ cargo deny check advisories -c literal-insert.toml
  [ERROR] failed to parse config from '<scratch>': duplicate key: `unsound`      EXIT 1
region REPLACE of 6-13        -> 1 line matching ^unsound =
  $ cargo deny check advisories -c replace-region.toml
  advisories ok                                                                  EXIT 0
```

Both directions discriminate, so neither result is a check that cannot fail. The
repository's `deny.toml` was untouched throughout (`git diff --exit-code --
deny.toml` -> 0 after the runs), and the fence applied to both copies was sliced
out of the plan's own markup rather than retyped.

**Changed:** Step 4(a)'s operative sentence is now a REPLACE of the eight lines
between the two anchors, naming what sits there and why; the fence is
byte-identical (sha256 `eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a`,
the same value the delta review measured, re-computed after every edit in this
round). The insert wording B1 was given is kept beside it as the record of what
that task was told, explicitly not as the operative sentence. A postcondition a
fix round can check was added: `grep -c '^unsound = ' deny.toml` returns 1.

**And the class, not the instance.** Three further sentences described the edit as
an insertion and are false in the same way. They were found by searching, not by
recall: `grep -niE 'insertion|inserts?\b|inserted'` over the whole document, then
classifying every hit. The three are Step 4's lead-in ("Two verbatim
insertions"), its post-application count line ("After both insertions the
`ignore` list holds 19 ids") and the "Must not decide" list ("the two fenced
`deny.toml` insertions"). All three corrected. The remaining hits in Task B1 are
the lockfile diff's "9 insertions", my own new sentence about fence (b) staying
an append, and the quoted record of B1's original wording - all correct usages.
This is the delta review's own harvest item taken literally: when a round repairs
an unperformable instruction, the repair's scope is the class across the
document, established by search.

## Finding 2 - the width figure, and the escalation withdrawn

**Measured, both states of the same file:**

```
master   deny.toml  max 77  at line 48   "# MIT project (spec 12); allow exactly the permissive licenses our dependency"
c422999  deny.toml  max 78  at lines 54, 57, 58
```

`diff -u` of the two blobs puts all three 78-character lines on the `+` side:
they are lines B1's own fence B (the ignore entry's comment) added. So the B1
verdict's "the file's existing maximum is 78" is a correct measurement of the
state it reviewed, which is also the state the replacement fence lands in; my 77
was a correct measurement of the other state; and the first round of this
amendment named neither state while calling a correct figure wrong.

**Changed:** both plan sites now read "76 against `deny.toml`'s own longest of 77
before B1's fence B and 78 after it, so it clears the file in either state", the
accusation is removed, and the rule is recorded where the figure sits - **a
figure that disagrees with a source figure is a STATE or a UNIT question until
reconciled**. Report concern 5 is withdrawn in place rather than deleted.

Worth stating plainly, because it is the third time this document has paid for
it: this is the exact class Amendment 5 exists to repair, produced by the
amendment repairing it. The first round did re-measure - the reflex worked - and
then skipped the reconciliation step and went straight to "the source is wrong".
Re-measuring is half the discipline; the other half is that two numbers from two
states are not a contradiction.

## Finding 3 - the guard-premise accounting, counted per cell

**Re-derived on the pre-amendment blob (`git show 83af0d5:<plan>`), per line and
then per table cell:**

```
:763  B1 Step 9      silent 0  infrastructure 2  -> 1 restatement
      (the other `infrastructure` is "using the existing infrastructure rather
       than a new scenario", about part (a)'s test duty, not the guard premise)
:787  plan close     silent 0  infrastructure 0  -> 0 restatements
:804  deferral row   silent 2  infrastructure 1  -> 3 restatements, split by column:
        Why     "the failure would be silent in exactly the way the original defect was"
        Why     "A lint asserting a `deny.toml` key would be new gate infrastructure,
                 which the tests-belong-to-the-package rule explicitly still allows deferring"
        Vehicle "the failure mode here is silent by construction - drop the key and nothing happens"
```

So: **two lines, four restatements, three of them in the deferral row (two in
Why, one in Vehicle)**. The first round said three lines and two-in-the-row - both
terms wrong, the total right by compensating errors, in the paragraph offered as
the demonstration that the unit is the restatement rather than the line. The
plan-close line is a consumer of the row's ROUTING (it says the residue exists
and how the ROADMAP line will be phrased) and asserts neither half of the refuted
ground; it was updated for the routing, which is why it looked like a site.

**Changed:** the accounting sentence now states both figures with what they count,
names the false-positive on `:763` explicitly, and says how the row was split.
**The repair itself did not move** - all four restatements were already gone as
assertions before this round, which is why the finding is Minor.

## What did not change in this round, verified rather than assumed

- **The replacement fence**: sha256 `eed7ff9276e85a80b42134b3d7f532023653ba15ee92c1a991c30a251655b47a`
  before and after, 11 lines, max width 76, zero non-ASCII.
- **The six defect repairs and their site sets.** Nothing in the substance moved;
  the delta review had already reproduced them independently.
- **Every other figure**: audit 26 -> 30 hit lines (30 after this round too, so
  no new hit entered), `...` at 12 lines, the seven placeholder terms at exactly
  1 each, 40 acceptance rows, 8 deferred rows, 58 fence markers, zero non-ASCII
  codepoints, zero trailing whitespace.
- **Amendments 1 to 4, Tasks A2, A3 and A4**: untouched, as before.

## Concerns from this round

1. **The B1 fix round now has a performable instruction, and it should be checked
   against the file it holds rather than against `master`.** The postcondition to
   assert is one `^unsound = ` line; the failure mode if the old wording is
   followed is a parse error, not a wrong comment, so it fails loudly - but only
   after the edit.
2. **Concerns 4 and 6 above are new or reworked** and are the ones that need a
   controller decision rather than a note: the ROADMAP's two wrapped "18" sites,
   and the two verdict requirements that address files outside this author's
   scope.
