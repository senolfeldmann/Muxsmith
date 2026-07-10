# Roadmap

Living, forward-looking work tracker: commitments with their milestone, and
candidates for later. Items here are DISCUSSION ANCHORS, not execution
licenses - each is talked through with Şenol when its turn comes, unless it
already carries a settled decision reference. History lives in the process
journal; decisions live in the specs/memos; unbuilt product ideas with full
analysis live in IDEAS.md.

## Plan 6 (next initiative)

Scope fixed by D22 (plan-5 memo): profile editor (comment-preserving YAML
round-trip is the hard design question and the reason apply-suggestion
waited), help-mode sidebar (spec 8.3 full mechanics), one-click
apply-suggestion, packaging/release pipeline. Starts with brainstorming.

## Pre-1.0 release gates

Must be resolved before the first tagged release; none blocks Plan 6 work.

- **CSP**: `tauri.conf.json` still has `"csp": null` (scaffold default,
  flagged in the Plan-5 whole-branch review). The IPC surface is real;
  defense-in-depth wants a real policy. Şenol has not thought it through
  yet - DISCUSS the policy shape with him when tackled, do not just set one.
- **README**: definitely 1.0. Absorbs or links BUILDING.md's "building from
  source" (plan-5 T4 note). Şenol has concrete expectations about voice and
  content - PROPOSE, then ASK; never write it freischnauze.
- **Guide + blog posts (process + product)**: 1.0 deliverables, written in
  fresh sessions from the process journal as primary source. Şenol has
  specific ideas about how he wants these - PROPOSALS welcome, but the
  format/scope/voice interview with him comes FIRST. Not to be produced
  unprompted.
- **Log pruning decision**: D26 defers pruning out of v1; before 1.0 decide
  keep-forever vs `prune` facility vs setting. (Şenol: "ist für später,
  schon notiert.")
- **mkvtoolnix version pin in CI**: currently floats with the distro
  (backlog note in ci.yml, Şenol 2026-07-10).
- **Packaging pipeline**: msi/dmg/deb/rpm/AppImage on release tags (spec 10;
  deliberately deferred out of Plan 5's CI work). Lands via Plan 6.

## Near-1.0

- **Dependabot/Renovate activation**: Şenol's call, timing "when 1.0 is
  essentially done". Free since the repo went public; SHA-pinned actions
  and exact dep pins are ready for it.

## v1.x candidates

Deferred with reasons; source: Plan-5 whole-branch triage (ledger, archived
at docs/process-journal/artifacts/plan-5-sdd/progress.md) and design memos.

- NDJSON `--json-events` stream; `--fail-fast=now` (deferred pre-Plan-5).
- Mutex poison recovery on AppState.active; joblog atomic writes (settings
  half was hardened in the Plan-5 fix wave).
- i18n check false-positive noise for shell IpcError codes (11 warn-only).
- RunMeta/summary.json cannot express joblog_status=incomplete (history
  view nuance).
- Per-run jobs control in the GUI (currently settings default only).
- Dialog-suppression setting (mkvtoolnix `m_warnBeforeAbortingJobs` parity,
  D31 note).
- Shared test-support crate if the mirrored fake-mkvmerge helpers grow
  beyond three copies.
- On-disk identification cache (spec 5.5 note).
- Richer joblog error type (distinguish per-job-write-lost vs summary-failed).
- Input-convenience parity features: see IDEAS.md 1-4 (kept there with
  their full analysis; NOT targets without a new decision).
