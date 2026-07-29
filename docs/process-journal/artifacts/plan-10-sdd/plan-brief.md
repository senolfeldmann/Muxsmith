# Plan 10 plan brief

Controller-authored brief for the Plan 10 execution plan. Written 2026-07-29
(session 27). You are the plan's AUTHOR; a separate independent reviewer grades
this plan against this brief - running a coverage dimension that walks the five
work items below and names the task implementing each - before the owner sees
it, and a fix loop runs until it is approved.

**There is no design document for this package, deliberately and with the
owner's approval.** Three of the five work items are already specified to the
line by a prior review or by a ruling; the two that carried real design content
were settled controller-side with verified facts and are written out below.
This brief therefore stands in for the design as your coverage ground truth,
together with the ROADMAP entries it cites. Say so in the plan's own header so a
later reader does not go looking for a design that never existed.

## 1. Deliverable

`docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`

House shape, same as
`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` (read it
as the FORM reference, not for content): the agentic-worker header with the
house deviation on progress, Goal, Architecture, Tech Stack, Global Constraints,
Execution method, Model tiers, the sequencing/parallelism section, the tasks,
the acceptance map, and the close actions.

Write the file. Do NOT run any git command: the controller commits.

## 2. Where this package sits, because it changes what "done" means

This is the last planned product package before the 1.0 tag, and it is **not**
the last thing that happens before the tag. The owner ruled on 2026-07-29 that
no 1.0 release is cut before he has personally run a manual QA and bug-hunting
pass on his own hardware, and that its output - real bugs, behaviour he dislikes
even where it matches the spec, and v1.x items he decides belong in 1.0 - is
first-class scope input (Tier-2 `owner-manual-qa-gates-the-1-0-release`). A
build for that pass already exists as a rehearsal draft on commit `a5b63ba`, and
his pass runs in parallel with this package because none of the five work items
changes shipped product behaviour.

Consequence for the plan: **do not write any sentence claiming this package
completes 1.0 scope, and do not add a task that prepares, versions or tags the
release.** The version bump to 1.0.0 belongs to the tag pass, not here.

## 3. Ground truth, exhaustively enumerated

1. **The v1 spec**, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` -
   authoritative on conflict, above everything below.
2. **This brief**, for the five work items and the decisions already settled.
3. **`docs/ROADMAP.md`**, specifically: the D102 paragraph in the Plan 9 anchor
   (its "RULED 2026-07-29 ... BUILD IT" close); the "Gate-count derivation has
   no check" section including its "MEASURED 2026-07-29" and "NARROWED FORM"
   blocks; the "Docs accuracy" section's stale-citation entry including its
   "RULED 2026-07-29" block; the README entry in the Pre-1.0 release gates
   section; the Renovate entry in the same section.
4. **The four house-knowledge files** as ground truth alongside the spec:
   `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`. Cite entries by
   id. The ones that bind this package hardest:
   `comments-locate-by-symbol-never-by-line-number`,
   `tests-ship-with-the-feature-never-after`, `ci-04-dependabot-cadence`,
   `owner-manual-qa-gates-the-1-0-release`, `ledger-lint-runs-before-every-push`.
5. **`BUILDING.md`** for the gate, verbatim, and as a work item's own subject.

Every premise in this brief is verifiable against the tree. **Refuting one is a
valid completion** - see section 8.

## 4. The five work items

### W1. The D102 producer

**Origin.** D102's scope boundary - `config_diagnostics` sorts, while per-file
`diagnostics` and `batch_diagnostics` stay in collection order - is asserted in
three normative places (D102 itself, a spec line Plan 9's Task 5 added, and both
builder rustdocs) and was guarded in none. The Plan-9 Task-5 reviewer measured
it rather than arguing it: widening `batch_document` to sort all three arrays
left `cargo test --workspace` at exit 0 with zero failures. The owner ruled on
2026-07-29 that the producer gets built.

**Shape the review named:** a `batch_document` case with a mixed-severity
`batch_diagnostics` vector asserting it is NOT reordered.

**Two things the task must do rather than assume:**

- **Measure per array, not once.** The contract has three arrays and two
  halves - preserved-order and sorted. Establish by mutation which halves are
  currently unguarded: widen or remove the relevant sort, run the suite, record
  what goes red and what stays green, restore. A half that is already guarded
  gets no second producer (reuse before writing); a half that is not gets one.
  The doctrine's rule that an observable's HALVES each need a named producing
  test is the reason this is measured rather than eyeballed.
- **Do not duplicate the existing guard.** `crates/muxsmith-cli/tests/dry_run_cli.rs`
  already carries a test for the `batch_document` half of the same D102 change -
  locate it by its doc comment, which names itself as exactly that - and it
  guards the SORTED side. The new producer is for the preserved-order side.

The measurement itself belongs in the task's report so the reviewer can
re-measure independently, with its own instrument.

### W2. The comment line-citation sweep

**Origin.** Owner ruling 2026-07-29, recorded as Tier-2
`comments-locate-by-symbol-never-by-line-number`: a comment never locates code
by line number - name the symbol (function, method, type, module, match arm,
config key). Naming the file stays fine and wanted.

**Corpus, measured by the controller on 2026-07-29 at commit `6b12d0c` and to be
RE-MEASURED by the task before it edits anything:** 17 sites across Rust,
TypeScript and Vue. The re-measurement is not ceremony - line numbers in the
citations shift as the sweep itself edits files, and a stale corpus list is the
exact defect the ruling exists to remove. Derive the set with a search over
tracked source files for a filename followed by a colon and digits inside a
comment, and state your search in the report so the reviewer can reproduce it
with a different instrument.

**Two rules that make this a sweep rather than a judgment call:**

- **Historical citations lose their numbers too.** Several sites record where
  something was BEFORE a fix ("pre-fix: panics at ..."). Under the ruling those
  are rewritten to name the function and what happened, which is both true and
  durable. Nothing has to be classified as live-versus-historical.
- **Ambiguous file references get disambiguated while you are there.** At least
  one site says "run.rs" where two files carry that name. Naming the symbol
  solves it; if the file name stays, it stays as a path unambiguous in the repo.

Scope boundary, from the convention entry itself: this touches comments in
SOURCE files. It does not touch dated evidence citations in process artifacts
(review verdicts, journal entries, ledger occurrence refs, tracker
measurements), which legitimately cite a line at a named commit.

### W3. The gate-count invariant

**Origin and its correction.** The ROADMAP carried a candidate: parse
BUILDING.md's check blocks and compare the count against every "N parts per
BUILDING.md" claim in the tracked docs. The controller measured the corpus
before briefing it and the cross-file form does not survive: of the 12 tracked
files outside the process journal that state a gate part count, ten are retired
plan documents whose "nine-part gate" was true when written, the ROADMAP's own
hits are historical statements about closed plans, and the one
process-conventions hit is an occurrence ref. Every current occurrence is a
record of what the gate was at the time, so a cross-file lint would fire on all
of them and demand that history be falsified.

**What the measurement exposed instead, and what the task builds:** BUILDING.md
is called the gate's single authoritative enumeration and never states the
TOTAL - it says six parts in one section and four in another and leaves every
reader to assemble eleven from three sections. A derived number with no
canonical statement is exactly what diverged at the plan-9 close pass.

So: **BUILDING.md states the total once, canonically, and a check verifies that
the stated total equals the number of commands actually enumerated in the gate
blocks.** One file, no cross-file matching, no history problem, no false
positives.

**Settled controller-side so it does not reach you as a fork** (refutable with
evidence per section 8):

- **The check lives in `scripts/ledger-lint.py`**, not in a new script. A new
  script would need a new gate part, which would change the gate's own count -
  a recursion worth avoiding for a check that costs microseconds. ledger-lint is
  already gate part eleven and already the house's docs-invariant checker in
  practice. Its header comment and any self-description widen accordingly, so
  the name's narrowness does not mislead the next reader. A rename is NOT in
  scope: it would ripple into BUILDING.md, `ci.yml` and every doc citing the
  script.
- **The check must be FIRE-VERIFIED, and the verification is part of the
  deliverable, not of the report's prose.** A check whose passing result is an
  absence proves nothing until it has been made to produce output once. Change
  the stated total, watch it go red, restore, and put both observed outputs in
  the task report. A green run alone is not evidence that the check works.
- The canonical sentence in BUILDING.md must be **machine-findable by a stable
  anchor**, not by counting prose. Choose the anchor and state why it is stable.

### W4. The Renovate configuration

**Origin.** Owner ruling 2026-07-29: Renovate activation is a firm pre-1.0
commitment. Cadence and shape are ALL settled - see Tier-2
`ci-04-dependabot-cadence` for the ruled statement. Nothing here is open.

**Deliverable: `renovate.jsonc` at the repo root.** JSONC because this repo
comments every pin decision and the vendor's own docs prefer `.jsonc` over JSON
with comments; the file name is inside Renovate's documented search order.

**The settled configuration.** Every item below was established by a recon
against current official documentation and, where the docs were thin, against
Renovate's source on `main`, read 2026-07-29. **Verify each against the current
docs yourself before writing it** - a brief premise refuted at the source is a
valid completion, and this is fast-moving vendor tooling:

- `$schema: "https://docs.renovatebot.com/renovate-schema.json"`.
- `extends: ["config:recommended", "helpers:pinGitHubActionDigests"]`.
  `config:best-practices` is deliberately NOT used: it pulls a weekly lock-file
  refresh that contradicts the ruled monthly cadence, plus Docker pinning this
  repo has no use for. `security:minimumReleaseAgeNpm` is deliberately NOT used
  either: its interaction with a narrow schedule window was the one thing the
  recon could not verify, and a monthly cadence already means most picked-up
  versions are weeks old, so it buys little against a real unknown.
- `timezone: "Europe/Berlin"`.
- `schedule: ["* * 1-3 * *"]` - three full days at the start of each month.
  **NOT the `schedule:monthly` preset.** The controller verified at the preset
  source that `monthly` is `['* 0-3 1 * *']`, a four-hour window on the 1st,
  and the hosted service runs a repo's job as rarely as daily depending on
  status - so the preset can silently skip whole months. Same cadence, no
  starvation. Put that reasoning in a comment in the file: the next person to
  read it will otherwise "simplify" it back to the preset.
- `prHourlyLimit: 0`. The schema's default is 2, which would dribble several
  group PRs across successive jobs.
- Package rules:
  - Cargo group, with **`rangeStrategy: "bump"`**. The manager's default only
    updates `Cargo.lock` and leaves the manifest string stale, which would make
    this repo's visible pins false within a month.
  - npm group.
  - GitHub Actions group, matched on the `action` dep type.
  - **Runner images disabled** (`github-runner` dep type). `release.yml` pins an
    older Ubuntu on purpose - D85, oldest supported base for the AppImage glibc
    floor - and Renovate cannot know that.
  - **`packageManager` dep type disabled.** Consequence of the owner's ruling
    that the `mise` manager stays off: pnpm is pinned in both `mise.toml` and
    `package.json` as a deliberate mirror, CI takes its pnpm from mise and a
    developer's corepack from `package.json`, so managing one side while the
    other stands still breaks the mirror instead of maintaining it. Both move by
    hand until mise leaves.
  - **`mise` manager disabled** (owner ruling).
  - `rust-toolchain` in its OWN group (owner ruling that it stays managed; its
    characteristic failure is new lints under `-D warnings`, which must not
    block an unrelated bump).
- **Majors are NOT folded into the ecosystem groups** (owner ruling). That is
  Renovate's default behaviour, so the correct implementation is to NOT set
  `separateMajorMinor: false` anywhere. Do not "helpfully" add it.
- `vulnerabilityAlerts` is deliberately NOT configured: the shipped defaults are
  already the ruled behaviour (immediate, ungrouped, bypassing schedule and
  limits). Verify that claim before relying on it; if the defaults have moved,
  configure explicitly and say so.

**Validation is required evidence, not a claim.** Validate the file with the
official validator at a pinned version, paste the command and its output into
the task report. Add NO permanent gate part, NO CI job and NO runtime dependency
for it: this file changes rarely and a permanent check is not earned.

**What the task must NOT claim.** Writing this file does not activate Renovate.
Activation needs two actions only the owner can take - installing the app
against this repository, and enabling the repository's dependency-graph and
alert feed so the immediate-security half exists at all. Those are OWNER
actions and the plan records them as OPEN, never as done. The plan should also
state the ordering that matters: the config must be on `master` BEFORE the app
is installed, which is what suppresses the vendor's onboarding PR.

### W5. The README accuracy pass

**Origin.** ROADMAP README entry, split by owner ruling 2026-07-29.

**In scope:**

- Re-check the CLI usage reference against the actually shipped CLI surface -
  every subcommand, flag and default, derived from the binary's own help output
  rather than from reading the source. A prior reviewer warned this section is
  "easy to lose"; treat divergence as expected, not as surprising.
- Re-check the exact-typed-matching paragraph against actual behaviour.
- Write in the content anchor recorded under the same ROADMAP entry: properties
  with language-like matching MAGIC must be EXPLICITLY LISTED - language with
  its ISO/BCP-47 normalization and dual-field lookup, absent boolean flags
  comparing false for exact, the curated type and codec-kind domains - contrasted
  with the no-magic byte-exact single-field rule of the raw form. Verify each
  claim against the spec and the code before writing it; this anchor is a
  paraphrase of an owner remark, not a specification.

**Explicitly OUT, and the plan says so where a reader would look for them:** the
four `placeholder(1.0)` comments and the work-in-progress banner. The owner
ruled 2026-07-29 that the placeholders may remain as placeholders, since the
README already states publicly that the project is in progress; two of them need
artifacts that do not exist yet and two are his taste call. The banner's own text
ties it to the tag. A task that fills a placeholder or drops the banner is out of
scope, not helpful.

## 5. What the plan must contain

Beyond the house form:

- **Sequencing argued, not assumed.** The doctrine's handle is a comparison, not
  a count: a worktree stream costs a setup, a merge, a full gate run on the
  merged state and the controller choreography around both, so it earns its place
  only when the task's own work exceeds that. Weigh these five against it
  explicitly, and note the file overlap that exists - the comment sweep touches
  files in the same crates and frontend directories the other tasks touch. If
  the answer is serial in one tree, say so in writing with the reasoning, the way
  plans 8.5 and 9 did.
- **Model tiers per `proc-03-model-assignment`**, as a table, with the top tier
  reserved for the plan-close whole-branch review and its delta re-reviews only.
  Every dispatch names its model explicitly; an omitted parameter inherits the
  session default, and inheritance is not an assignment.
- **Global constraints**, covering at least: ground-truth precedence; the gate
  per BUILDING.md, foreground, no subsets, before any push, with **no count
  written into the plan** - name the file, because W3 is about to change what the
  file says and a plan that hardcodes a number would fork the contract it
  executes against; the latitude ban; no task edits house-knowledge YAML, the
  controller is the single writer; the commit-trailer convention with the model
  string DERIVED from the dispatch's model parameter rather than written as a
  literal; and the standing commit grant restated in any dispatch that expects a
  task to commit, because a subagent inherits a global never-commit default and
  cannot see a grant it was never shown.
- **An acceptance map** from each of the five work items to the task
  implementing it and to the named test or observation that produces it. Where a
  work item has two observable halves, name a producer per half - one producer
  named for the whole observable satisfies the map while covering one side, which
  is how a real gap survived a plan review two packages ago.
- **Close actions** as an explicit list the controller executes: verdict-harvest
  mining into the ledger, the blocked-pool sweep, ROADMAP dispositions for every
  item this package closes, the SDD salvage with its file count verified in the
  commit, and the journal entries. Note explicitly that the Renovate owner
  actions stay OPEN at the close and that the package's completion is not 1.0
  completeness.

## 6. What the plan must NOT do

- No version bump, no tag preparation, no release-body edits.
- No filling of README placeholders, no banner removal.
- No renaming of `scripts/ledger-lint.py`.
- No new permanent gate part, CI job or runtime dependency for the Renovate
  config's validation.
- No task that edits any of the four house-knowledge YAML files.
- No design-latitude clause anywhere - not "if you find a simpler equally-safe
  alternative", not "either approach works", and not the commoner form: a
  mandated set that is never enumerated, a list ending in an ellipsis, a "one per
  X" with no X list. The test is not whether a permission appears but whether the
  implementer must invent something it is not allowed to invent.

## 7. House rules that bind the plan document itself

- Progress never enters the plan: no ticked checkboxes, no status markers. The
  SDD scratch carries the tracker.
- Every acceptance item is machine-checkable or explicitly marked as an owner
  observation.
- A safeguard this plan proposes stays until it is built and MEASURED redundant;
  it is not argued away during authoring or review.
- Any test this package's own behaviour makes observable ships in the same task,
  not in a later item.

## 8. Refuting this brief is a valid completion

If a premise here is false against the tree - a corpus count that no longer
holds, an existing guard that already covers a half W1 would add, a Renovate
option that the current docs describe differently, a BUILDING.md structure that
makes the chosen anchor unstable - say so with the evidence and do not build
around it silently. Brief errors are the one class the downstream catch
demonstrably covers, and an implementer verifying a premise against the tree and
refuting it is the mechanism working, not a failure to comply.

Where you hit a genuine fork this brief does not settle, return it as
NEEDS_CONTEXT with the options, their costs against the named invariants, and a
recommendation. Do not decide it and report afterwards.
