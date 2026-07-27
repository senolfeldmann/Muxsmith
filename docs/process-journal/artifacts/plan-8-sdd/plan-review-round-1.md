# Plan 8 plan review, round 1

Independent reviewer (did not author the plan). Artifact:
`docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md` at commit
a0a8dea (verified: the plan's authoring commit, 6 tasks, working tree
clean for this file). Ground truth read in full: the owner-approved
design `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(all 2026 lines), `.superpowers/sdd/plan-8/plan-brief.md`, the plan-7
house template, `docs/ROADMAP.md` (Plan-8 anchor + "Ledger hygiene"),
`.github/workflows/ci.yml`, `scripts/ledger-lint.py`, the four house
YAML files (cited entries), and the live tree facts each claim rests on.
Every check below ran foreground; every transcription diff and count was
recomputed mechanically, not eyeballed.

## Verdict: NEEDS FIXES

One Major finding (M1, plan-close salvage enumeration wrong at the
tree), one Minor (m1), two nits. Everything else across all ten
dimensions verified clean - the fix surface is one plan-close bullet
plus an optional check tightening; no task body needs structural change.

---

## Findings

### M1 (Major) - plan close: the sdd-scratch salvage enumeration undercounts, 5 refs exist, the plan pre-registers "exactly these two"

**Location:** Plan close, bullet "sdd-scratch citations riding the
salvage".

**Evidence** (grep `design-review-round-1` over all four house YAML
files; all five refs verified present at the plan's own commit a0a8dea,
so this was wrong at authoring, not raced afterward):

- `docs/decision-ledger.yaml:4066` - `design-acceptance-observables-have-producers` (named by the plan)
- `docs/decision-ledger.yaml:4095` - `proc-quote-verbatim-or-paraphrase` (named by the plan)
- `docs/conventions.yaml:1025` - `code-comment-line-citations-drift`, ref "plan-8 design review round 1 m2 (design-review-round-1.md)" - **MISSED**
- `docs/process-conventions.yaml:551` - `design-empirical-claims-reproducible`, ref "plan-8 design review round 1 HARVEST (design-review-round-1.md)" - **MISSED**
- `docs/process-conventions.yaml:552` - `design-empirical-claims-reproducible`, ref "plan-8 delta review HARVEST (design-review-round-1.md round-2 section...)" - **MISSED**

The bullet's claim "What does point into the plan-8 scratch: two
decision-ledger occurrence refs" is false as a claim about the pointer
set, and executing the pre-registered action "re-point exactly these two
refs" leaves three refs dangling the moment the salvage moves the file -
the precise drift class the ruled citations-move-with-salvage pattern
exists to prevent. Note the bullet's fire-verified positive control
covered the *design-file* grep (which correctly found 0); the ledger-ref
sweep itself ran with no completeness control and was scoped to (at
most) the decision ledger instead of the four-file house set that
`scripts/ledger-lint.py` itself hardcodes. This is the plan-7-review
count defect class, and `proc-normative-count-recomputed` binds this
plan by its own global constraint.

**What to change:** re-enumerate the bullet as the five refs above
(file : entry : ref), or state the rule ("every house-YAML occurrence
ref naming a plan-8 scratch artifact, swept over all four files") and
record the recomputed count 5. Scanned for completeness: no other plan-8
scratch artifact is cited anywhere in the four YAML files -
`design-review-round-1.md` is the only one, so five is the full set.

### m1 (Minor) - Task 5 Step 5: the additive-only pattern has a blank-line hole and inverted exit semantics

**Location:** Task 5, Step 5
(`git diff master -- .github/workflows/ci.yml | grep -c '^-[^-]'`).

(a) A deleted existing **blank** line renders in the diff as a lone `-`,
which `^-[^-]` does not match - that removal passes the "Expected: 0"
check unseen. (b) `grep -c` exits 1 exactly when the count is 0, i.e.
the command's exit status signals failure precisely when the check
passes - harmless run interactively, a trap for any exit-status-sensitive
wrapper. The step's fire-verification covers content-line edits, so the
common firing direction is proven; the hole is narrow but free to close.

**What to change:** assert the deletions column of
`git diff master --numstat -- .github/workflows/ci.yml` is 0 (fires on
blank-line deletion too; clean exit semantics), keeping the same
fire-verify (edit one existing line, watch the column go nonzero,
restore).

### n1 (Nit) - "Tier-2 files" label covers two tier-1 ledger entries

Global constraint 4 lists `deps-first-party-pinned-over-convenience` and
`design-acceptance-observables-have-producers` under "Tier-2 files ...
entries that bind this plan"; both are `tier: 1` decision-ledger entries
at the tree (verified), not Tier-2 nature-file entries. Binding force is
unchanged (the ledger is ground truth alongside, and the controller
brief used the same label), but "Tier-2/ledger entries" would stop a
future reader hunting them in the wrong files.

### n2 (Nit) - Task 6 Step 6: the AppImage check needs a chmod the task does not mention

`./muxsmith-*.AppImage --appimage-extract` on a freshly downloaded asset
fails without `chmod +x` first (INSTALL.md documents the chmod for
users; the task step does not). Mechanical tool handling, not a design
fork - two added words save a fresh implementer a stall.

---

## Dimension walk

### 1. COVERAGE (load-bearing)

I walked D75-D90 and every design section myself before grading the
plan's map. Result: **every design element has a named implementing or
consuming task, and the plan's coverage map matches my walk with no gap
and no dilution.** The walk, compressed to the mapping I derived
independently: D75->T3 (+T4 link consumption, R8 at close); D76->T2
(absence preserved) /T4 (assert step + G4)/T6 (R5); D77->T4/T3/T6;
D78->T4/T6 (R2); D79->T4 (two triggers, input, shared jobs, naming)/T6
(runs A+B)/close (R10); D80->T2/T3/T6 (R6); D81->global constraint, no
implementing task (correct - it is a ban); D82->T2 (3.2/3.4/4.4)/T4
(sidecar staging)/T3 (CLI/PATH docs)/T6 (R6); D83->T4 (guard)/T6
(R1)/T5 (scope adjudication); D84->T4; D85->T4; D86->T2/close (R8
fallback protocol); D87->T1/T4/T6 (R9); D88->T4 (packing)/T3
(README.txt)/T6; D89->T4/T6 (R2); D90->T4/T6 (R4). Sections: 2->T4
(single-normative-copy decision, implementer transcribes from the
design and diffs - sound anti-drift shape); 3.1/3.2/3.4/4.4->T2;
3.3->T1; 4.1/4.2/4.3/4.5->T3; 5->close trigger 6; 6->T4's
negative-space grep (its positive control verified live: the pattern
hits ci.yml lines 40 and 82); 7->close + T5 ruling record; 8->T1
(G1-G3), T4 (G4-G5), T6 (R1-R10); 9->close mirror (recount: 9 triggers,
plan-close summaries match the design's items 1-9); 10->nothing (none
open); 11->global constraints + task-level restatements. **The ROADMAP
rider is present as REQUIRED scope, undiluted**: Task 5 carries both
halves (duplicate-key extension AND ci.yml wiring), the broken-fixture
fire-test, the local green proof, and the in-CI green-reachable
observation wired into the merge-order push watch.

### 2. Rider-vs-D83 adjudication - VERIFIED at the design text; no controller routing needed

The design's own words, re-read at the file:

- D83 decision: "`ci.yml` is **not touched**: its `v*` tag trigger keeps
  running the full test matrix + deny job on the tagged SHA" - the
  decision's entire content is the *release pipeline's* placement.
- Section 11, first bullet: "ci.yml is not modified, at all (D83)." -
  the heading scopes it: "What the implementer must not decide" binds
  the design's implementers.
- Section 7, last bullet: "Controller ruling, since recorded in the
  ROADMAP ledger-hygiene entries (2026-07-22 S22): Plan 8 absorbs the
  wiring as a rider task, bundled with the duplicate-key extension; the
  rider enters the plan brief at plan authoring and is explicitly 'not a
  design amendment (nothing in the plan-8 design depends on it)'".
- Section 10: "the wiring rides Plan 8's plan as a rider task, outside
  this design; no artifact in this document depends on it."
- ROADMAP "Ledger hygiene" (verified verbatim at the file): "Controller
  ruling: Plan 8 absorbs the CI wiring as a rider task, bundled with the
  duplicate-key extension (entry above). The rider enters the plan brief
  at plan authoring; not a design amendment (nothing in the plan-8
  design depends on it)."

The design therefore scopes its own ban exactly as the plan's
adjudication claims: the ban binds every design-scoped task (the plan
carries it so, global constraint 3), while the design itself twice
records the rider as outside its scope, and the plan brief pre-states
the scoping ("resolve against D83's ... decision, which was scoped to
the RELEASE pipeline"). The author's no-collision resolution is within
its mandate; the evidence carries the reading. The additive-only check
(Task 5 Step 5) is executable and fire-verifiable as written; m1
tightens its pattern.

### 3. Latitude scan (both forms, per task)

Tasks 1/2: zero latitude - code carried verbatim, reverts and insertion
anchors enumerated, and the anchors are real at the tree (BUILDING.md
"## Building and running" line 57 / "## Tooling quirks" line 97;
`.gitignore` "# JS/Tauri frontend" block ending at `src-tauri/gen/`).
Task 3: transcription with the owner-content ban restated; extraction
mechanism named. Task 4: one frozen artifact; section-11 freeze
restated. Task 5: the one explicit clause ("the code shape within these
bounds is the implementer's") adjudicated against
`proc-latitude-clause-boundary`'s test - no unenumerated set sits in a
normative position: mechanism class (SafeLoader subclass,
`construct_mapping`), scope (all mappings, all four files), violation
format with a worked example, aggregation/exit semantics, the S21
fixture shape, the docstring sweep with its verification grep, and
checks-1-5 invariance are all enumerated; what remains is genuinely code
shape on a mid-tier task. Sanctioned side of the boundary. Task 6:
emitters, the two-shape download fallback ("no third variant"), and the
R6 tool-absence STOP are enumerated. Section 11 is transmitted
undiluted (global constraint 2 binds it on every task; tasks restate the
locally relevant freezes). Omission scan: the nine-part gate is
enumerated (recount 9), the action set (4 new + 1 reused pin), the
banned shapes, the merge order, the asset/leg sets - no unenumerated set
found in a normative position anywhere in the plan body. The one
enumeration defect found sits in the plan close (M1).

### 4. Template conformance

Plan-7 skeleton reproduced exactly: agentic-workers header with the
no-progress house deviation verbatim; tracker
`.superpowers/sdd/plan-8/progress.md`; Goal / Architecture / Tech Stack
/ Global Constraints / Execution method (binding,
superpowers:subagent-driven-development, fresh implementer + independent
reviewer per task + whole-branch review) / How this plan cites the
design / Dependency graph and stream cut / waves / tasks with
Files-Interfaces-Steps / Plan close. The two additions over plan-7
(model-tier table, design-section coverage map) are both brief-mandated.
Global constraints carry the full brief list: SI-4 push rules, nine-part
gate no-subsets note, no-new-dependency with the bounded CI enumeration,
SHA-pin policy, tag/publish/placeholder ban, unsigned commits + trailer
+ explicit staging, typography with the D86 `Ş` carve-out, foreground
only, gh-log duties, counts-recomputed, fire-verification duty, and the
implementer session-relocation preamble verbatim. Plan-close
pre-registrations all present: rehearsal-draft cleanup (owner R10),
INSTALL.md owner rendered-surface pass, salvage items (defective - M1),
trigger mirror, rider bookkeeping, journal/HANDOFF.

### 5. Transcription fidelity

All ten transcribed blocks re-diffed mechanically (sed extraction, then
`diff` against the design text): guard script (3.3), full config end
state (3.1), overlay (3.2), BUILDING.md subsection (4.4), G1-G3, G4-G5,
R1-R10, the D85 leg table, the D89 8-asset set, the 4.5 README rider
comment. **Every diff empty - byte-identical.** The per-leg `--bundles`
summary line matches the section-2 matrix values.

### 6. Dependency graph vs reality

Wave-1 file sets re-derived from the task Files sections and checked
pairwise: A {tauri.conf.json, check-version-sync.sh,
tauri.bundle.conf.json, .gitignore, BUILDING.md}, B {docs/INSTALL.md,
draft-body.md, rehearsal-banner.md, linux-tarball-README.txt,
README.md}, C {release.yml}, D {ci.yml, ledger-lint.py} - disjoint,
matching the plan's enumeration. 1->2 same-file serialization correct;
3/4/5 rootless correct; all five ->6 edges load-bearing as stated (the
5->6 edge via the guard's gate-green consumption of ci.yml conclusions
is real and recorded as intended). The rehearsal's structural
sequencing (release.yml merged AND pushed to the default branch before
any dispatch) is explicit in three places (Architecture, merge order,
Task 6 preconditions) and correctly ordered: merges A->B->C->D each
followed by the full nine-part gate, then master sanity runs, then the
controller push, then the foreground ci watch including the new
ledger-lint job, then wave 2. Task 6 writes no repo file; `gh-log.md`
is git-ignored (`.gitignore:3`, verified).

### 7. Citations and counts

`README.md:99` verified at the tree (the release placeholder comment;
located by content as the task requires; placeholder count 4). Every
count recomputed from its enumeration: 6 tasks; 4 new pinned actions + 1
reused house pin; `uses:` in the section-2 YAML = 7, all
40-hex-SHA-pinned with version comments (0 violations); 4-leg matrix; 8
assets (7 + SHA256SUMS); INSTALL.md H2 headings = 3 (Windows/macOS/
Linux exactly); draft-body table rows = 7, `__VERSION__` tokens = 8;
nine-part gate = 9 commands; close triggers = 9; two dispatch runs = 12
jobs (2 x 6); guard poll 30 s x 90 = the stated 45-minute fail-safe;
watch ceiling 5400 s = 90 min; R1's pick/ls counts 1/1/3 and 1/1/4
consistent with the matrix and the tar.gz. Tree and environment claims
verified live: pre-change key order
`$schema,productName,version,identifier,build,app,bundle` (so the
post-deletion expectation string is exact), bundle block today
`active/targets:"all"/icon` only, Cargo workspace 0.1.0, package.json
0.1.0 + `pnpm@11.10.0`, `@tauri-apps/cli` 2.11.4, mise node 26.5.0,
rust-toolchain 1.96.1, exactly five icons, `tauri_build::build()`
(src-tauri/build.rs:5), the clap attribute at
crates/muxsmith-cli/src/cli.rs:10, gh 2.94.0, local PyYAML 6.0.3,
ci.yml positive-control hits at lines 40 (Swatinem/rust-cache) and 82
(mise-action), deny job last with ubuntu-26.04 + the house checkout pin
(so Task 5's block appends after it, indentation and runner family
matching), ledger-lint's docstring deferral sentence and its
summary/FAIL formats exactly as Task 5 Steps 1-2 expect, and
`steelman:` keys present in process-conventions.yaml for the fixture.
The single wrong enumeration found in the whole document is M1.

### 8. Model-tier classification (proc-03)

The plan's section conforms to the entry's owner-bound mapping: mid
default, cheap only where the plan carries the code verbatim, every
reviewer mid, explicit model parameter at every dispatch ("an omitted
parameter is not an assignment"), top model never a subagent. The cheap
claims hold on inspection: Task 1 and Task 2 are transcription-complete
in the plan itself (code verbatim, fully scripted checks, revert
semantics and insertion anchors spelled out - a transcription-tier
implementer needs zero judgment). Task 3's deliberate mid (the plan
cites rather than duplicates ~150 lines of owner-pass-bound prose, so
the carry condition is deliberately unmet) is exactly proc-03's rule
applied, not an evasion of it. Tasks 4/5/6 mid: correct (scratch-tree
fire-tests and extraction; new Python logic; live gh judgment).

### 9. Implementability walk

Each task is executable as written by a fresh implementer: worktree
paths given; the session-relocation ban + plain-directory + absolute-path
preamble is a global constraint carried into every dispatch; every gh
wait is foreground with an explicit ceiling (watch 5400 s, precondition
watch 2700 s, run-id resolution 10 x 15 s, tripped timeouts return to
the controller rather than silently extending); G1-G3 re-run in Task 1
and G4-G5 in Task 4 pre-merge with concrete extraction instructions
(script bodies sliced from the committed workflow, never retyped); the
rider's red-fixture fire-test plus an old-checks control plus a
reachable green; NEEDS_CONTEXT-with-decision-memo routing stated
globally and instantiated concretely (Task 6 Step 6's STOP for the
missing Fedora tools, owner-authorized install through the controller);
no background-run-plus-monitor anywhere (banned twice, and no step uses
it). Task 4's Interfaces note that cross-stream files are absent in its
worktree by construction pre-empts the likeliest false NEEDS_CONTEXT.
The one friction point is n2 (AppImage chmod).

### 10. Rehearsal task evaluability

Every R-observable is evaluated at its named emitter inside the task
steps: R1/R2/R3 -> Step 3 (run log, artifacts API + download recount,
step-status skip observable plus `gh release list` baseline
corroboration - the skip is a positive observation, not an absence
grep); R4/R5 -> Step 5 (release view, checksum round-trip with the
one-time corruption control, body composition with the fire-verified
`__VERSION__` absence grep against the known-8 template); R6 -> Step 6
(tool gate first, then every transcribed check, skipping declared a
defect); R7 -> Step 7 (with the transcribed master-ref positive
control); R9 -> Step 8 (version awk-parsed, not hardcoded); R8/R10 ->
Step 9, correctly routed as owner steps to the plan close with the
publisher-fallback protocol carried. The rehearsal-draft cleanup is
pre-registered at the close as owner action R10, and the task explicitly
does not delete the drafts (they are the owner's inspection
deliverable, D79). Nothing in the task publishes, un-drafts, or tags -
the global constraint plus D77/D79/D81 restatements hold in every step.

---

## HARVEST

- **The enumeration-not-recomputed class has moved into close sections.**
  Plan-7's instances sat in task-step counts; this plan's single
  instance (M1) sits in a plan-close bullet - written last, swept least.
  Candidate handle: any plan-close bullet that enumerates repo state
  ("exactly these N ...") owes the same grep-with-control discipline as
  a task step, and a house-YAML citation sweep runs over all four files
  by construction (the four-file list ledger-lint hardcodes is the
  authoritative surface).
- **A firing positive control does not prove surface completeness.** The
  M1 bullet's control proved the grep pattern fires (against the plan
  brief) - and the design-file zero was indeed correct - but the sweep
  that produced "two" was scoped to one file. Pattern validity and
  search-surface completeness are separate obligations; only the first
  is covered by break-and-watch.
- **`--numstat` deletions==0 is the tighter additive-only idiom** than
  `grep -c '^-[^-]'` on the unified diff (blank-line-safe, exit-status
  clean). If the additive-only check shape recurs once more, it is a
  conventions candidate.
- **Task 4's "absent by construction" Interfaces note** (cross-stream
  files not present in this worktree; absence expected, not a defect) is
  a transferable pattern for every parallel-stream plan: it pre-empts
  the likeliest false NEEDS_CONTEXT in a file-disjoint cut at the cost
  of one sentence.
- Transcription discipline held completely: ten of ten carried blocks
  byte-identical against the design, and the plan's stated
  diff-at-authoring claim reproduced on my independent re-run - the
  plan-7 T21 truncation class did not recur.

---

# Delta review (round 2)

Resumed original reviewer; artifact re-read at commit 9d0fc45 (29
insertions, 7 deletions on the plan file; all four findings applied,
none disputed). Every load-bearing fix claim re-run at the current tree,
foreground.

## Verdict: APPROVED

## Per-finding disposition

**M1 - FIXED, verified.** The salvage bullet is now rule-first and the
rule survives a moving surface, which is the fix's actual claim: the
binding sentence sweeps ALL FOUR house YAML files (the ledger-lint
surface) for every plan-8 scratch basename, re-points EVERY ref whose
text names plan-8 in the same change as the salvage, and demotes line
numbers to a dated snapshot with id + ref-text as the match key and a
recount mandated at salvage time. My independent re-run of the inline
sweep at the current tree reproduces the snapshot exactly: 19 hits, 8
naming plan-8 scratch artifacts, and all eight file:line pairs map to
precisely the entry ids the bullet lists (conventions:1025
code-comment-line-citations-drift; process-conventions:553/554
design-empirical-claims-reproducible; decision-ledger:4066/4095/4138/
4152/4166 design-acceptance-observables-have-producers,
proc-quote-verbatim-or-paraphrase, proc-sweep-surface-completeness,
ci-additive-only-check-numstat, plan-interfaces-absent-by-construction -
the last three are the freshly mined tier-1 entries at 4125/4139/4153,
verified present). The other-11 classification checks out: 8+5+4+2 = 19;
the two `design-brief` prose refs (process-conventions:378,
decision-ledger:3449) are confirmed plan-6-scoped; the ROADMAP carries
re-pointing triggers for the sibling salvages (grep confirmed;
per-plan consumed-status not re-audited - outside this close's binding
scope). The surface growing from 5 to 8 between my round-1 verdict and
the fix is the rule's own premise demonstrated, not a discrepancy: the
hit-count is snapshot, the rule is the deliverable, and the rule is
correct.

**m1 - FIXED, verified.** Task 5 Step 5 now asserts the deletions
column of `git diff master --numstat -- .github/workflows/ci.yml` is 0,
with "exactly one line" additionally guarding the no-diff case; the
blank-line-deletion hole is closed (numstat counts it), exit semantics
are clean, the fire-verify is retained and correctly predicts an
in-place edit as 1 deletion + 1 addition, and the cited
`ci-additive-only-check-numstat` exists as a tier-1 ledger entry.

**n1 - FIXED, verified.** The constraint heading now names both
surfaces ("Tier-2 nature files AND the Tier-1 decision ledger"); the two
ledger entries carry inline "(Tier-1 ledger; ...)" markers and the
trailing default marks the rest as nature-file entries - matching the
tree (both marked entries verified tier: 1 in round 1; the remaining
eight live in the nature files).

**n2 - FIXED, verified.** The chmod note landed as a "Mechanical note"
in Task 6 Step 6's prose, outside the verbatim checklist. The R1-R10
block was re-extracted from the fixed file and re-diffed against design
section 8: byte-identical. Same re-run for G1-G3 and G4-G5:
byte-identical. The author's transcription-intact claim holds.

Typography scan over the full fixed plan: zero AI-tell glyphs (pattern
control fired on a planted em-dash).

## New findings

**n3 (Nit, advisory - no further round needed):** the snapshot sweep
command enumerates five basenames while the plan-8 scratch dir holds six
citable .md files - `plan-review-brief` is missing from the grep
pattern. Verified zero house-YAML hits for that basename today (the
sibling basenames firing is the stated control), and the binding rule
sentence ("every plan-8 scratch basename", recount at salvage) already
covers it, so the gap lives only in the dated instantiation the bullet
itself declares non-authoritative. Recommended: add `plan-review-brief`
(and any basename the scratch dir has grown by then, e.g. `progress`)
to the pattern when the salvage executes or on any later touch of the
bullet - a salvager who pastes the command without re-reading the rule
would otherwise inherit exactly the class the rule was written against,
in miniature.

## HARVEST additions

- **A rule-first fix can still under-enumerate its own handle.** The
  corrected bullet gets the layering right (rule binding, snapshot
  dated, id+ref-text matching) and its executable command still missed
  one live basename (n3). When a fix converts an enumeration into a
  rule + snapshot, the snapshot's enumeration deserves the same
  completeness check as the original defect - here: derive the grep
  alternation FROM a directory listing at run time
  (`ls .superpowers/sdd/plan-8/` piped into the pattern) rather than
  hand-enumerating it, which removes the hand-maintained set entirely.
  That shape - generate the enumeration from the authoritative listing
  instead of restating it - is the same move ledger-lint makes with its
  hardcoded four-file surface, one level down.
