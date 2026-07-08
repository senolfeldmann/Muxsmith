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

## 2026-07-09 | Plan 2 written and implemented | session (Peter, Opus 4.8)

**Scope.** Plan 2 design finalization, plan authoring, and full implementation.
Commits `3b71a71..e1bfba7`. Design memo + spec fold-in, the 12-task plan doc,
then all 12 tasks executed. 125 workspace tests green; CI green at e1bfba7
(`test` + new `deny`).

**Decisions and why.**
- Session opened under a Fable quota crunch; Şenol asked what was worth the last
  Fable tokens. Decided: spend model strength on the DECISIONS (D1-D6) not the
  transcription. Wrote the design memo first, then folded D1-D5 into the
  authoritative spec so "spec wins on conflict" could not silently override them
  (4.3+4.4 previously implied codec_kind under substring; a bad enum value
  degraded to MissingTrack). Then quota moved to Opus and the rest proceeded.
- D2 dropped an xtask codegen path: the v20 identification schema types `type`
  as a plain string (no enum) and only `aac_is_sbr` carries a schema enum, so
  generating value domains for one irrelevant field was an abstraction the scale
  had not earned. `type`/`codec_kind` domains are curated in `capability`
  instead. Grounded in the source Şenol dropped at ~/Downloads/mkvtoolnix.
- Execution deviated from Plan 1's subagent-driven-development: the controller
  (Opus) executed the tasks inline with live `cargo` verification, rather than
  dispatching a fresh subagent per task. Rationale: the plan's code was fully
  specified and every task was locally compile/test-verifiable, so the SDD
  apparatus (implementer + reviewer subagents) was overhead for mechanical
  transcription. Tradeoff: no independent per-task reviewer; a whole-branch
  adversarial review is still owed (open thread).
- mkvmerge `-J` shape, `--list-languages`/`--list-types` formats, and the track
  `type` domain were confirmed empirically by running the installed v99 binary
  rather than trusting memory or the schema alone. Both the identify parser and
  the runtime parsers passed against real output first try.

**What the process caught.**
- CI (test job) caught fmt-dirty commits: tasks 3 and 5 were pushed after
  running clippy but not `cargo fmt --check`; the intermediate run at 0e64c1e
  failed only on `cargo fmt --all --check`. Fixed by rustfmt commit 72c59d2.
  Origin: controller discipline gap, not the plan.
- clippy caught two collapsible-if blocks in the matcher (let-chains), pre-push;
  fixed by amend. Origin: implementer (plan code was pre-let-chain style).
- cargo-deny caught the intra-workspace path dependency as a wildcard; resolved
  by marking the workspace crates `publish = false` (Muxsmith ships as app
  bundles, not crates.io packages), which is also correct on its own terms.
  Also trimmed the license allow-list from a speculative 8 to the 3 actually
  used (MIT, Apache-2.0, Unicode-3.0).

**Process mechanics.** 12 tasks, 0 subagent dispatches (controller-inline
execution). Model: Opus 4.8 (1M) throughout after the Fable handoff. One clippy
amend, one rustfmt catch-up commit, one style commit. ~14 commits in the range.
Every push logged in gh-log.md. Live end-to-end dry-run demonstrated the
suggestion engine emitting three forced_track discriminators for an ambiguous
English-SRT rule.

**Deltas.** Task 4 shrank from an xtask-codegen task to a curated-domains task
after checking the real schema (above). Plan's `candidates_for_rule` iterator
chain had a temporary-borrow bug flagged in the plan itself; implemented via the
owned-Vec workaround the plan noted. Plan's Task 10/11 split (extract plan_core)
was collapsed: planner written with plan_core/plan_batch in final shape from the
start, suggestion engine added as the placeholder->real step.

**Open threads.** Whole-branch adversarial review still owed (executed inline,
no independent reviewer). Deferred: OverlappingRules auto-suggestions and the
no-single-fix partition report (D6 remainder); Plan 3 (attachments/chapters/
tags/title, command generation, executor); `--list-types` extension validation
(no diag code yet); CI does not install mkvtoolnix so the gated tests self-skip
there. See HANDOFF.md.
