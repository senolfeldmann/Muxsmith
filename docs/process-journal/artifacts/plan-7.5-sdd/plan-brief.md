# Plan 7.5 plan brief (controller-authored)

You author the EXECUTION PLAN for Plan 7.5 (track-rule add/remove in the
profile editor). Four-eyes: an independent plan reviewer grades your plan
- including a COVERAGE dimension walking the design section by section -
before execution. You write exactly ONE file:
`docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md`.
Do NOT commit (the controller commits); touch nothing else.

## Contract and template

- **The design is the contract**: docs/superpowers/specs/
  2026-07-22-plan75-track-rule-add-remove-design.md (D65-D72,
  owner-approved 2026-07-23 after a one-round four-eyes fix loop). The
  v1 spec is authoritative above it on conflict; flag, do not improvise.
- **The house plan template is the plan-7 plan**: docs/superpowers/plans/
  2026-07-21-plan-7-help-i18n.md. Reproduce its structural conventions
  exactly: the agentic-workers header note incl. the house deviation
  (progress NEVER enters the plan; tracker is
  `.superpowers/sdd/plan-7.5/progress.md`), Goal, Architecture,
  Global Constraints, "Execution method (binding)" naming
  superpowers:subagent-driven-development with fresh implementer +
  independent reviewer per task and a whole-branch review at the close,
  dependency graph with explicit edges, design-section coverage map
  (every D65-D72 and every design section -> the task(s) implementing
  it), then the tasks with exact steps.

## Plan-specific requirements

- **Task cut**: yours to decide, with the dependency graph making the
  cut checkable. Respect the house serialization rule (same-file-region
  tasks serialize in one worktree; genuinely disjoint work parallelizes
  in separate worktrees). This plan is small - do not force parallelism
  where the file graph is serial; equally do not serialize disjoint
  content work out of convenience.
- **Model-tier classification per task** (proc-03-model-assignment):
  mark each task implementer as mid tier (default: judgment
  implementation) or cheap tier (ONLY where the plan itself carries the
  code to transcribe); every task reviewer is mid tier. The controller
  sets the parameters at dispatch; the plan carries the classification.
- **Global constraints to carry** (from the plan-7 template, adapted):
  nine-part gate per BUILDING.md foreground before any push and after
  every merge; bilingual same-commit rule for any changed catalog/topic
  content, with final wording riding the owner's rendered-surface pass;
  no new dependencies of any kind; commits unsigned with the repo
  trailer, explicit staging, never `git add -A`; typography rules;
  foreground test runs only in implementer briefs; counts recomputed
  from enumerations; absence checks fire-verified once; every fork
  closed - NEEDS_CONTEXT with decision memo for anything discovered on
  code contact; Tier-2 files as ground truth alongside spec + design
  (name the entries that bind this plan, at minimum
  editor-generic-action-keys, gui-closed-domain-dropdowns,
  help-mode-suppression-pointer-scope, help-topic-h1-scheme,
  design-empirical-claims-reproducible, proc-normative-count-recomputed,
  core-83). Cite Tier-2 entries by id; re-verify any :line you attach.
- **The design's e2e plan (nine cases) maps to explicit task steps**;
  the D62/D55 ripple stays zero-new-ids/zero-new-topics per D71 - a
  task that would create a help-id is a plan defect.
- **Spec amendments (design amendments 1-2) land as their own task**,
  sequenced after the code they assert exists (plan-7 Task-21
  precedent), with the design's verbatim wording transcribed
  unabbreviated (the plan-7 T21 truncation defect is ledgered - do not
  repeat it: transcribe, then diff your transcription against the
  design text and state the check ran).
- **Plan-close items to pre-register** (so the close does not improvise):
  the salvage re-pointing trigger for the design's
  design-review-round-1.md citation (ROADMAP Triggers, registered
  2026-07-22/23), and the owner rendered-surface pass over any changed
  user-facing wording.

## Method duties

Latitude ban in both forms binds every task text. Counts recomputed.
File/symbol citations verified at the current tree. Do not restate
design content the tasks can cite by section - transcribe only what the
plan-7 template's "How this plan cites the design" logic would
transcribe (hardest-won enumerations an implementer must not
re-derive), and name which those are.

Constraints: read-only except your one output file; no git; never call
EnterWorktree/ExitWorktree or any session-relocation tool (and the plan
itself must carry the implementer-preamble ban: subagents never call
session-relocation tools, worktrees are plain directories, absolute
paths); anything you run, run foreground.

Final message: at most 3 lines + the plan path + any NEEDS_CONTEXT.
