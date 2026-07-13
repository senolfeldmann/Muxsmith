# House-knowledge backfill — Era E1 (2026-07-08), Muxsmith Plan 1

Reconstruction of what the decision-ledger / CONVENTIONS mechanism WOULD have recorded
during era E1 had it existed from commit one. E1 = spec authoring + Plan 1 (core
foundations + validate CLI), session 1 (Peter, Fable 5), commits `61249f9..33d6587`
(2026-07-08). The mechanism itself did not exist yet (decision-ledger.md was created in
session 7, still "none yet"); this is the retroactive fill.

Sources, richest first: v1 design spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
(§2 decision log is the primary design-decision source); Plan 1 doc Global Constraints;
reviewer VERDICT files under `.../plan-1-sdd/verdicts/`; journal `docs/process-journal.md`
2026-07-08 entry; handoff `.../handoffs/2026-07-09-plan-1-close.md`; progress ledger
`.../plan-1-sdd/progress.md`; git log.

Note on stack: the spec §2 row now reads "Vue" with a note it replaced React at D27
(2026-07-10). The E1 occurrence is React — journal Plan 1 records "Tauri 2 + Rust core +
React/TS". D-numbered memos referenced in the spec (D6/D20/D21/D27/D32) are all later-era
annotations woven back in and are NOT E1 occurrences.

One record per occurrence. Design decisions grounded at their spec §2 / Global-Constraint
decision point (occ=decided); review-stage violations grounded at their verdict (occ=violated-corrected);
deferrals grounded where the block is stated (occ=deferred). Same-session journal/handoff
recaps of a spec decision are NOT emitted as separate occurrences (they document the same
decision-point), except where a genuinely distinct event (a review catching a violation) occurred.

---

## Design patterns / restraints (core, i18n, cross)

1. **track rule semantics :: strict independent uniqueness** — pattern, core, decided.
   Every rule resolves to exactly one track independent of rule order; all overlaps are errors;
   configs must spell out exclusions (e.g. `forced_track: false`), error quality made first-class to compensate.
   occ_ref: spec §2 "Rule semantics" row; journal 2026-07-08 Plan 1; commit 61249f9.

2. **track rule semantics :: ordered consumption** — restraint, core, decided (rejected).
   Steelman: first-match-wins with track consumption would make Şenol's own reference profile
   work unmodified, no explicit exclusions needed — more ergonomic. Rejected for its implicitness;
   Şenol chose maximal explicitness. (A global constraint solver was rejected in the same breath.)
   occ_ref: journal 2026-07-08 "Strict independent uniqueness over ordered consumption or a global solver ... he rejected the implicitness."

3. **ambiguity disambiguation :: batch-wide validated suggestions** — pattern, core, decided.
   Suggestions are simulated against the whole batch before being shown; an applied suggestion
   must survive the next dry run; suggestions are structured edits, not prose. Şenol's amendment
   turning a hint feature into a verified-edit feature.
   occ_ref: spec §2 "Disambiguation help" + §5.3; journal 2026-07-08 (his amendment); commit 61249f9.

4. **MKV structure scope :: full control in v1** — pattern, core, decided.
   Tracks, attachments, chapters, tags, title all configurable; more surface accepted deliberately.
   occ_ref: spec §2 "MKV structure scope"; journal 2026-07-08; commit 61249f9.

5. **MKV structure scope :: global on/off toggles (tracks-only)** — restraint, core, decided (rejected).
   Steelman: global toggles for attachments/chapters/tags/title = far less config and UI surface,
   simpler v1. Agent recommended it; Şenol overruled — a scope call, full declarative control.
   occ_ref: journal 2026-07-08 "Full MKV-structure control in v1 ... against agent recommendation of global toggles; his scope call."

6. **desktop app stack :: Tauri 2 + Rust core crate + React/TS + clap CLI** — pattern, cross, decided.
   Mature, small bundles, best packaging; Rust accepted despite being recently picked up. (React later
   replaced by Vue at D27, 2026-07-10 — out of era.)
   occ_ref: spec §2 "Stack"; journal 2026-07-08; handoff plan-1-close; commit 61249f9.

7. **desktop app stack :: Wails v3** — restraint, cross, decided (rejected).
   Steelman: a Go-based shell would let the core stay in Şenol's strongest language and avoid the
   Rust learning curve while still shipping small native bundles. Rejected: v3 was alpha (delivery risk).
   occ_ref: journal 2026-07-08 "over Wails v3 (alpha risk)".

8. **desktop app stack :: Avalonia** — restraint, cross, decided (rejected).
   Steelman: mature C#/.NET UI stack with high delivery certainty. Rejected: smaller OSS pull /
   less web-native than the Tauri+web-frontend route.
   occ_ref: journal 2026-07-08 "and Avalonia (delivery certainty, smaller OSS pull)".

9. **license :: MIT (over Apache-2.0)** — pattern, process, decided.
   Permissive, commercialization by anyone incl. the author, consistent with the Ruby prototype.
   occ_ref: spec §2 "License" + §12; journal 2026-07-08 "MIT over Apache-2.0"; Plan 1 Global Constraints; commit 61249f9.

10. **identification schema sourcing :: build-time fact extraction into committed generated.rs, schema never vendored** — pattern, core, decided.
    An xtask generates matchable property names/types into committed `generated.rs`; the upstream
    mkvmerge schema itself is never redistributed; no build.rs network dependency. Sidesteps schema licensing.
    occ_ref: spec §2 "Identification schema" + §9; journal 2026-07-08; handoff plan-1-close; commit 61249f9.

11. **identification schema sourcing :: runtime fetching / build.rs network dependency** — restraint, core, decided (rejected).
    Steelman: fetching the schema at runtime/build keeps the property model always in lockstep with
    the local mkvmerge, no version skew. Rejected to sidestep schema licensing and avoid a network
    build dependency; runtime skew handled instead as an untyped-match warning.
    occ_ref: spec §2 "sidesteps schema licensing and runtime fetching entirely"; journal 2026-07-08 "no build.rs network dependency".

12. **prose-free core / i18n architecture :: Fluent, one catalog shared CLI+GUI, core emits code+params only** — pattern, i18n, decided.
    No hardcoded user-facing strings in any layer; core emits diagnostic `code` + `params`, text lives
    in `locales/*/*.ftl`. Fluent chosen as the one system with first-class Rust AND JS implementations.
    occ_ref: spec §2 "Localization" + §8.4; journal 2026-07-08; Plan 1 Global Constraints; commit 61249f9 / a671949.

13. **localization content scope :: English-only content in v1, mechanism complete** — pattern, i18n, decided.
    i18n-ready from day one but only English catalogs/help ship; adding a locale is content work, not a refactor.
    occ_ref: spec §2 + §11 non-goal; commit 61249f9.

14. **mkvtoolnix dependency :: external, user-installed, CLI invocation only** — pattern, cross, decided.
    No linking, no GPL implications, no bundling burden; detected at startup. Muxsmith never processes media.
    occ_ref: spec §2 "mkvtoolnix dependency" + §12; commit 61249f9.

15. **unknown profile keys :: errors, deny_unknown_fields on every profile struct** — pattern, core, decided.
    Explicit over silent: an unknown key is a config error, not a warning.
    occ_ref: spec §4; Plan 1 Global Constraints line 15; handoff plan-1-close; commit 1f00aa6.

16. **output naming / collision policy :: keep|template, on_collision error|skip|overwrite (default error), in-place replacement excluded** — pattern, core, decided.
    Collision default refuses; in-place replacement of source files is a hard, non-configurable exclusion.
    occ_ref: spec §2 "Output naming" + §4.8 + product summary; commit 61249f9.

## Process / CI patterns

17. **public-API documentation :: deny(missing_docs) on lib crates; private items by judgment (blanket private-doc lint rejected)** — pattern, process, decided.
    Semantics-not-name-restating rustdoc required for public items; a blanket private-item doc lint
    was rejected as comment-noise.
    occ_ref: journal 2026-07-08 "deny(missing_docs) for public API; ... blanket private-doc lint rejected as comment-noise"; rustdoc-backfill-report judgment call 5; commit c402914.

18. **CI cost control while private :: dynamic fromJSON matrix, Linux-only on branch pushes, 3-OS on PR/tag/dispatch** — pattern, ci, decided.
    macOS Actions bills 10x; branch pushes run Linux only while the repo is private.
    occ_ref: spec §10; journal 2026-07-08; commit 97ae031.

19. **plan execution method :: subagent-driven-development (fresh implementer + independent reviewer per task, fix waves, final whole-branch review, model split haiku/sonnet/fable)** — pattern, process, decided.
    The E1-established execution apparatus; ~31 dispatches over 13 tasks.
    occ_ref: journal 2026-07-08 mechanics; handoff plan-1-close; Plan 1 doc REQUIRED SUB-SKILL header.

20. **report verification :: controller independently re-runs every test suite, never trusts report arithmetic** — pattern, process, decided/reinforced.
    Reinforced by evidence: haiku implementers mis-totaled workspace test counts in 5 of 13 reports
    (code correct each time), and a synthesized test transcript was caught at the T3 review; controller
    began re-running every suite as standing practice.
    occ_ref: journal 2026-07-08 "haiku implementers mis-totaled ... controller began independently re-running every suite"; verdicts/task-3-review-verdict.md (synthesized-transcript Important); handoff plan-1-close.

21. **spec authority :: spec is authoritative, on conflict the spec wins (flag, do not improvise)** — pattern, process, reinforced.
    Load-bearing rule; already overrode plan text once in E1 (the prose-in-core final-review fix invoked it).
    occ_ref: Plan 1 Global Constraints line 13; handoff plan-1-close "this rule already overrode plan text once"; verdicts/whole-branch-review-verdict.md Important #1.

22. **agent git operations :: commits/pushes authorized for this repo, unsigned (gpgsign=false), Co-Authored-By trailer** — pattern, process, decided.
    GPG signing blocks agent commits -> standing `-c commit.gpgsign=false`. (A mid-session "no unrequested
    commits" rule was applied then explicitly reversed for this repo the same day.)
    occ_ref: handoff plan-1-close Git section; journal 2026-07-08 friction "GPG signing blocks agent commits; standing workaround" + "Mid-session rule change ... later explicitly reversed for this repo".

23. **GitHub interaction audit :: gh-log.md entry for every gh/API/push interaction; nothing that costs money** — pattern, process, decided.
    occ_ref: handoff plan-1-close GitHub section; journal 2026-07-08 "gh-log.md audit convention added the same day"; commit fad067c.

## Review-stage violations corrected (violated-corrected)

24. **unknown profile keys :: deny_unknown_fields silently ineffective on untagged struct variants** — violated-corrected, core.
    T4 review found serde's `deny_unknown_fields` is a container attribute, silently ignored on inline
    `Variant { field }` shapes, so unknown keys inside Template/External blocks parsed clean — contradicting
    the binding constraint. Fixed by extracting bodies into named `TemplateBlock`/`ExternalBlock` structs as newtype variants.
    occ_ref: verdicts/task-4-review-verdict.md (Important, Needs fixes); progress ledger Task 4; fix commit b5eaa3d.

25. **edition-2024 keyword collision :: `gen` module name reserved; implementer's edition-2021 downgrade rejected** — violated-corrected, core.
    T5 implementer correctly diagnosed the collision but proposed downgrading the crate to edition 2021;
    controller rejected the workaround (root cause over symptom) and renamed the module to `codegen`.
    occ_ref: journal 2026-07-08 "implementer's edition-2021 downgrade workaround rejected by controller -> module renamed"; verdicts/task-5-review-verdict.md; fix commit e78847d.

26. **diagnostic catalog key integrity :: key() literals and serde kebab encoding were two unlinked encodings** — violated-corrected, core.
    T2 review flagged that hand-authored `.key()` literals and serde's auto `rename_all` agreed only by
    hand; nothing tied them. Corrected with `DiagCode::ALL` + exhaustive consistency/uniqueness tests.
    occ_ref: verdicts/task-2-review-verdict.md (Important #1); journal 2026-07-08 "key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL + exhaustive consistency tests"; fix commit a7c0d89.

27. **diagnostic output ordering :: --json output unsorted while text sorted error-first** — violated-corrected, cli.
    T12 review found the `--json` branch emitted raw insertion order while text sorted by `Reverse(severity)`,
    so `diagnostics[0]` could be a warning under exit 2. Fixed by sorting once before branching.
    occ_ref: verdicts/task-12-review-verdict.md (Important #1); journal 2026-07-08; fix commit ad841b0.

28. **prose-free core :: template-error params carried English prose out of core** — violated-corrected, i18n.
    Final whole-branch review found `validate.rs` `format!("unknown filter: ...")` literals violating the
    plan's own prose-free-core exit criterion; the plan conflicted with its own Global Constraint, so spec
    wins. Fixed to code-like `kind`/`name` params + a Fluent selector.
    occ_ref: verdicts/whole-branch-review-verdict.md (Important #1) + round-2 confirmation; journal 2026-07-08; fix commit 3c24845.

29. **external locator config :: `match_to_source: false` raised a spurious LocatorConflict** — violated-corrected, core.
    Final review: `validate.rs` checked `is_some()` but spec types the field as `?: true`; `false` should
    mean "not in use". Fixed to conflict only on `Some(true)` and reject `Some(false)` explicitly.
    occ_ref: verdicts/whole-branch-review-verdict.md (Important #3) + round-2; fix commit 3c24845.

30. **spec/code diagnostic-code naming :: UnknownProperty collided between spec table and code** — violated-corrected, core/process.
    Final review: code used `unknown-property` for a config-time typo error while spec 5.2 defined it as the
    planning-time skew warning. Resolved by amending the SPEC (split into UnknownProperty error + UnknownPropertySkew)
    and keeping the code — a deliberate exception to "spec wins" because the code was right.
    occ_ref: verdicts/whole-branch-review-verdict.md (Important #2); journal 2026-07-08; fix commits cd3f239 + f7afa8d.

## Non-decisions (deferred)

31. **identification cache :: on-disk cache** — non-decision, core, deferred.
    In-memory per-session cache (keyed path+mtime+size) decided for v1; an on-disk cache is deferred as a
    future candidate, not blocked on anyone — YAGNI for now.
    blocked_on: internal — future enhancement; in-memory suffices for v1.
    occ_ref: spec §5.5 + §11 non-goal "On-disk identification cache"; handoff plan-1-close "On-disk cache deferred".

32. **go-public timing + CI matrix revert** — non-decision, ci, deferred.
    The Linux-only-while-private CI trim reverts to the full 3-OS matrix on going public; when to go public
    is Şenol's business/visibility call and was left open.
    blocked_on: external — Şenol's go-public decision.
    occ_ref: journal 2026-07-08 "Pending decisions: go-public timing + CI matrix revert"; handoff plan-1-close "Go-public timing ... His call."

---

Also grounded but not emitted (at the 32-record cap; recorded here for the merge stage if wanted):
archive/copy-commit verify-file-count-in-commit (journal addendum, commit 411087f); process-journal
append-only mechanism per PROMPT.md (journal header); yaml_serde fork over archived serde_yaml (handoff);
catalog completeness guard upgraded to DiagCode::ALL iteration (progress Task 13, commit c7a70f7);
deferred Plan-2 handoff items — codec_kind substring/regex semantics, value-domain lint (`type: vdieo`
passes), Locator.path/output.directory type unify, rendered-filename separator re-check, Fluent plural
selectors (all in verdicts/whole-branch-review-verdict.md Plan-2 handoff notes).
