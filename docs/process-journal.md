# Muxsmith process journal

Raw material for a future process-focused writeup: decisions with their why,
what the review process caught, mechanics, friction. Append-only; entries are
lab-notebook register, not publication prose. See the entry prompt for rules.

## 2026-07-08 | Spec + Plan 1 complete, repo live | session 1

**Scope.** Entire first session: requirements interview, spec, Plan 1
authoring, subagent-driven execution of all 13 tasks, rustdoc backfill,
GitHub setup, CI trim. Commits 61249f9..97ae031 (33 commits). First-run
salvage pass done: all 49 files of `.superpowers/sdd/` survived a reboot
(repo dir, not /tmp) and are archived under
`docs/process-journal/artifacts/plan-1-sdd/`; nothing lost.

**Decisions and why (not obvious from artifacts).**
- Strict independent uniqueness over ordered consumption or a global solver:
  Şenol chose maximal explicitness knowing configs need explicit exclusions
  (forced_track: false); compensated by making error quality and the
  suggestion engine first-class. Ordered consumption would have made his own
  example work unmodified; he rejected the implicitness.
- Suggestion engine contract (his amendment): suggestions are simulated
  against the whole batch before being shown; an applied suggestion must
  survive the next dry run. Turned a hint feature into a verified-edit feature.
- Full MKV-structure control in v1 (attachments/chapters/tags/title) against
  agent recommendation of global toggles; his scope call.
- Stack: Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk vs his Go
  learning goal) and Avalonia (delivery certainty, smaller OSS pull). Rust
  accepted although not on his learning list. MIT over Apache-2.0.
- mkvmerge identification schema: build-time fact extraction via xtask with
  committed generated.rs; schema never vendored (licensing sidestep), no
  build.rs network dependency. Runtime skew handled as untyped-match warning.
- i18n: Fluent chosen because it is the one system with first-class Rust AND
  JS implementations, so one catalog serves CLI and future GUI. Core emits
  code+params only; this rule later forced a real fix (see below).
- Docs: deny(missing_docs) for public API; private items documented by
  judgment only (blanket private-doc lint rejected as comment-noise).
- CI while private: dynamic fromJSON matrix, Linux on branch pushes, 3-OS on
  PR/tags/dispatch (macOS bills 10x); revert on going public.

**What the process caught.** Real defects, all origin=plan-authored code:
- serde deny_unknown_fields silently ineffective on untagged inline struct
  variants (task review T4) -> TemplateBlock/ExternalBlock newtype fix.
- `gen` reserved in edition 2024 (implementer T5); implementer's
  edition-2021 downgrade workaround rejected by controller -> module renamed.
- key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL +
  exhaustive consistency tests.
- --json output unsorted while text sorted (task review T12).
- Template-error params carried English prose out of core, violating the
  plan's own exit criterion (final review; spec-wins rule applied).
- match_to_source: false raised a spurious LocatorConflict (final review).
- UnknownProperty name collision between spec table and code (final review)
  -> spec amended, code kept.
Evidence-integrity finds: one synthesized test transcript (T3 review);
haiku implementers mis-totaled workspace test counts in 5 of 13 reports
(code correct each time) -> controller began independently re-running every
suite; that verification never contradicted a green claim, only the counts.

**Mechanics and metrics (approx where labeled).**
- ~31 subagent dispatches: 13 implementers, 13 task reviews, ~7 fix/re-review
  waves, 1 docs pass, 1 final whole-branch review + confirmation.
- Model split: haiku for transcription implementers (plan contained complete
  code), sonnet for judgment implementers (T9, T12, fix wave, docs) and all
  task reviewers, fable for the final review. Subagent token use 37k-210k
  each; largest was the docs backfill (~209k).
- Tests 0 -> 81; fmt + clippy -D warnings clean throughout; first CI run
  green on all 3 OSes in 2m44s; trimmed run verified as exactly one job.
- Rustdoc: 151 public items backfilled in one pass + per-variant DiagCode
  docs via macro meta-forwarding; 5 restatement-style docs caught by review.

**Friction and failure.**
- Monitor script used zsh read-only variable `status`; watch died silently.
- gh initially missing and repo private -> first CI verdict unobservable
  until Şenol installed gh; anonymous API returned Not Found.
- GPG signing blocks agent commits; standing workaround
  `-c commit.gpgsign=false`.
- Session limit interrupted one turn mid-bookkeeping; ledger + git log made
  resume trivial (the ledger earned its existence here).
- Mid-session rule change (no unrequested commits) later explicitly reversed
  for this repo; gh-log.md audit convention added the same day.

**Moments.**
- T2 reviewer suspected supply-chain anomaly in Cargo.lock (`zmij`), checked
  crates.io, found it is dtolnay's renamed `ryu`; suspicion correctly
  withdrawn (artifacts: review in plan-1-sdd).
- T5 implementer independently discovered the edition-2024 keyword collision
  the plan missed; wrong fix, right diagnosis.
- Final reviewer graded the plan's own code against the plan's constraints
  and failed it on three counts; "the plan does not grade its own work"
  proved to be the load-bearing process rule.

**Deltas.** yaml-serde crate name resolved to yaml_serde 0.10.4 (plan
anticipated the fallback); T13 catalog guard upgraded from the plan's
hand-copied key list to DiagCode::ALL iteration; audio_sampling_frequency
type assumption in a plan comment was moot by test construction.

**Open threads.** Plan 2 handoff notes (6 items, see
artifacts/plan-1-sdd/progress.md); deferred minors: Fluent plurals, schema
keyword domains for the GUI, substring-test hardening, xtask dedup
tie-break, unused `at` param in load.rs, invalid-template selector default
variant. Pending decisions: go-public timing + CI matrix revert; Dependabot
cadence if enabled. Plan 4 installs parked: Tauri system deps, pnpm via
corepack. Plan 2 awaits explicit go; Şenol announced additional context
before that go.

**Addendum, same day.** The salvage pass itself nearly failed twice: the
archive commit initially contained only the journal because the copied sdd
directory carried the tooling's own `.gitignore` (a bare `*`), silently
excluding all 49 artifacts; caught by reading the commit stat line, fixed in
411087f. Lesson recorded: after any archive/copy commit, verify the file
count in the commit, not the working tree.
