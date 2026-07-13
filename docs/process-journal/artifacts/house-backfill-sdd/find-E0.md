# Era E0 (2026-07-08) — decision-history reconstruction

Session 1 (Peter, Fable 5): requirements interview, v1 spec, Plan 1 authoring +
subagent-driven execution of all 13 tasks, rustdoc backfill, GitHub setup, CI
trim. Commit range `61249f9..97ae031` (33 commits, all 2026-07-08).

**Sources mined:** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
(full read — the E0 design output), the "Spec + Plan 1" journal entry
(`docs/process-journal.md` lines 7-113, the dated E0 point), `BUILDING.md`
(Plan-1-dated sections), and `git log 61249f9..97ae031`.

**Deliberately excluded as out-of-era (not E0):** all `IDEAS.md` items (1-4 from
the Plan 3.5 parity audit 2026-07-09; 5-7 from the 2026-07-11 sweep); spec
decision references D6/D19/D20/D21/D27/D32/D33/D35 and the whole ROADMAP body
(Plan 2 onward); `BUILDING.md` "Cross-target lint rule" (explicitly a Plan 5
lesson). Attributing any of these to E0 would be fabrication. Where a spec row
was later amended in place (e.g. React→Vue at D27), the E0 record captures the
*original* 2026-07-08 choice only.

One record per occurrence. `occ_kind`: decided / reinforced / violated-corrected
/ deferred.

---

## Core architecture — decided patterns

**E0-1 · Rule resolution semantics — strict independent uniqueness** (core, pattern, decided)
Every rule must resolve to exactly one track regardless of rule order; all overlaps are errors; configs spell out exclusions explicitly.
occ_ref: spec §2 decision-log row 1 + journal 2026-07-08 bullet 1.
evidence: "Şenol chose maximal explicitness knowing configs need explicit exclusions (forced_track: false); compensated by making error quality and the suggestion engine first-class."
steelman (rejected alt): "Ordered consumption would have made his own example work unmodified; he rejected the implicitness." A global solver was the other rejected option.

**E0-2 · Suggestion engine = verified edits** (core, pattern, decided)
Suggestions are simulated against the whole batch before being shown; an applied suggestion must survive the next dry run.
occ_ref: spec §2 row 2 + §5.3 + journal 2026-07-08 bullet 2.
evidence: "Turned a hint feature into a verified-edit feature" (his amendment).

**E0-3 · Full MKV-structure control in v1** (core, pattern, decided)
Tracks, attachments, chapters, tags, title all configurable — against agent recommendation of global toggles.
occ_ref: spec §2 row 3 + journal 2026-07-08 bullet 3.
evidence: "Full MKV-structure control in v1 against agent recommendation of global toggles; his scope call."
steelman: "Global on/off toggles per structure would be far less surface to build and reason about."

**E0-4 · Output naming + collision policy** (executor, pattern, decided)
`filename: keep|template`; `on_collision: error|skip|overwrite` default `error`; in-place replacement excluded.
occ_ref: spec §2 row 4 + §4.8.
evidence: "error | skip | overwrite, default error. In-place replacement excluded."

**E0-5 · Stack: Tauri 2 + Rust core + React/TS + clap CLI** (cross, pattern, decided)
Chosen over Wails v3 and Avalonia; Rust accepted despite being recently picked up. (React later swapped to Vue at D27/2026-07-10 — not E0.)
occ_ref: spec §2 row 5 (original) + journal 2026-07-08 bullet 4.
evidence: "Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk) and Avalonia (delivery certainty, smaller OSS pull). Rust accepted although only recently picked up."
steelman: "Avalonia gave delivery certainty in C# (a language Şenol already knows) and a smaller OSS pull; Wails v3 a simpler Go stack — both dodge the Rust learning curve."

**E0-6 · License MIT over Apache-2.0** (process, pattern, decided)
occ_ref: spec §2 row 6 + journal 2026-07-08 bullet 4.
evidence: "Permissive, commercialization by anyone including the author; consistent with the Ruby prototype." / "MIT over Apache-2.0."
steelman: "Apache-2.0 adds an explicit patent grant MIT lacks."

**E0-7 · mkvtoolnix as external CLI dependency only** (cross, pattern, decided)
Invoked as user-installed executables; no linking, no bundling; detected at startup. Sidesteps GPL.
occ_ref: spec §2 row 7 + §12.
evidence: "No linking, no GPL implications, no bundling burden. Detected at startup."
steelman: "Bundling the binary would remove the install-mkvtoolnix first-run friction."

**E0-8 · Identification schema: build-time extraction, never redistributed** (core, pattern, decided)
Property names/types generated into the capability model at build time via xtask with committed `generated.rs`; the schema file itself never vendored; no `build.rs` network dependency.
occ_ref: spec §2 rows 8-9 + §9 + journal 2026-07-08 bullet 5 + commits 830dc47, 4750abb.
evidence: "build-time fact extraction via xtask with committed generated.rs; schema never vendored (licensing sidestep), no build.rs network dependency."
steelman: "Fetching the schema at build/runtime would always track the exact local mkvmerge instead of a pinned snapshot."

**E0-9 · Runtime version skew as untyped-match warning** (core, pattern, decided)
A property the pinned model lacks is matched untyped and warned, not hard-failed. (The E0 sketch; later formalized as the D32 `raw:` opt-in in Plan 5.5 — not E0.)
occ_ref: journal 2026-07-08 bullet 5 tail + spec §9.
evidence: "Runtime skew handled as untyped-match warning."
steelman: "Hard-failing on any unknown property is simpler and never silently mismatches."

**E0-10 · i18n via Fluent, one catalog for CLI + GUI** (i18n, pattern, decided)
Fluent chosen because it is the one system with first-class Rust AND JS implementations; one catalog serves the CLI (fluent-rs) and the future GUI (@fluent/bundle).
occ_ref: spec §2 row 10 + §8.4 + journal 2026-07-08 bullet 6.
evidence: "Fluent chosen because it is the one system with first-class Rust AND JS implementations, so one catalog serves CLI and future GUI."
steelman: "gettext/ICU are more established but lack a matched Rust+JS pair, forcing two catalogs."

**E0-11 · Core emits code + params only, no user-facing prose** (i18n, pattern, decided)
muxsmith-core produces diagnostic codes + structured params; all user-facing strings live in Fluent catalogs / per-locale markdown.
occ_ref: spec §5.2 + §8.4 + journal 2026-07-08 bullet 6 tail.
evidence: "Core emits code+params only; this rule later forced a real fix."

**E0-12 · DRY: one core crate; frontend does zero semantic validation** (cross, pattern, decided)
Validation, planning and execution share one code path; CLI and GUI are renderers; every profile edit round-trips through a core validate command.
occ_ref: spec §2 (DRY row) + §7.
evidence: "Validation, planning and execution share one code path; GUI and CLI are renderers."

**E0-13 · Discoverability: self-explanatory UI + integrated help mode** (gui, pattern, decided)
Tooltips + inline explanations everywhere plus a hover-to-explain help mode with sidebar; extra UI surface accepted deliberately.
occ_ref: spec §2 row 11 + §8.3 + commit a671949.
evidence: "More UI surface, accepted deliberately: modern tools underinvest here."
steelman: "Plain tooltips alone would be far less to build than a full help-mode sidebar system."

**E0-14 · Muxsmith never processes media** (core, pattern, decided)
It generates and executes mkvmerge commands only; output always MKV, input anything the local mkvmerge supports.
occ_ref: spec §1.
evidence: "Muxsmith never processes media itself: it generates and executes mkvmerge commands, exactly like mkvtoolnix-gui's job queue does."

**E0-15 · Declarative batch, not interactive per-file** (core, pattern, decided)
The profile is reusable declarative selection rules applied to every source file in a tree; no interactive per-file review step.
occ_ref: spec §1.
evidence: "declarative selection rules, not as picks from one concrete file. Muxsmith applies the profile to every source file in a directory tree."

**E0-16 · One profile data model; YAML/JSON equivalent; schema generated** (core, pattern, decided)
Single serde model; YAML and JSON fully equivalent; JSON Schema generated from the same model via schemars, never hand-maintained.
occ_ref: spec §4.
evidence: "one data model (serde), both formats fully equivalent. A JSON Schema ... generated from the same model (schemars) ... never hand-maintained."

**E0-17 · Unknown profile keys are errors** (core, pattern, decided)
Explicit over silent: an unknown key is an error, not a warning.
occ_ref: spec §4.
evidence: "Unknown keys are errors, not warnings (explicit over silent)."

**E0-18 · Property model split: matchable generated vs settable curated** (core, pattern, decided)
Two disjoint sets: matchable properties generated from the identification schema at build time; settable properties a curated table mapped to mkvmerge options.
occ_ref: spec §4.4.
evidence: "Two disjoint property sets ... Matchable properties are generated at build time ... Settable properties are a curated table mapped to mkvmerge options."

**E0-19 · One code path, three operation levels (validate / dry-run / run)** (core, pattern, decided)
Static validate, dry-run (validate + full planning, `-J` only), run (re-plan + execute) share one path.
occ_ref: spec §5.5.
evidence: "One code path, three entry points." (The "dry-run is a strict superset of validate" clarification is a Plan-2-fix addition, 2026-07-09 — not claimed for E0.)

---

## Restraints — deliberately rejected in v1

**E0-20 · Source files never modified / in-place replacement excluded** (executor, restraint, decided)
occ_ref: spec §1 + §2 row 4 + §11.
evidence: "Source files are never modified or overwritten. Hard rule, not configurable."
steelman: "In-place metadata editing (mkvpropedit-style) saves disk and avoids a separate output dir for small tweaks."

**E0-21 · No transcoding, ever** (core, restraint, decided)
occ_ref: spec §11.
evidence: "Any transcoding, ever; Muxsmith muxes."
steelman: "A convert-and-remux step would serve users whose source codecs the target player can't handle."

**E0-22 · No season/episode arithmetic** (core, restraint, decided)
Identifiers are opaque match keys; transforms are per-file, not cross-file.
occ_ref: spec §11.
evidence: "Season/episode arithmetic (identifiers are opaque match keys; transforms are per-file, not cross-file)."
steelman: "Cross-batch renumbering/offset is a real bulk-rename need."

**E0-23 · No wildcard multi-track rules** (core, restraint, decided)
"Keep all remaining audio" breaks strict uniqueness; possible later as explicit `multi: true`.
occ_ref: spec §11.
evidence: "Wildcard multi-track rules ... breaks strict uniqueness; possible later as explicit multi: true rules."
steelman: "mkvtoolnix's 'keep all remaining tracks of type X' is a common convenience; uniqueness forces enumerating every track."

**E0-24 · No per-file manual overrides in the GUI** (gui, restraint, decided)
That is mkvtoolnix-gui's job; an "open in mkvtoolnix-gui" escape hatch is a v1.x candidate.
occ_ref: spec §11.
evidence: "Per-file manual overrides in the GUI (that is mkvtoolnix-gui's job...)."
steelman: "Users will hit a file the rules can't express and must then leave the tool entirely."

**E0-25 · No bundling of mkvtoolnix binaries** (cross, restraint, decided)
A Windows convenience downloader is a v1.x candidate.
occ_ref: spec §11 + §12.
evidence: "Bundling mkvtoolnix binaries; a Windows convenience downloader is a v1.x candidate."
steelman: "Bundling removes the biggest first-run friction — the user having to install mkvtoolnix."

**E0-26 · `just` runner rejected** (process, restraint, decided)
occ_ref: BUILDING.md "Deliberately not used" (Plan-1 stock-take, 2026-07-08).
evidence: "just runner: xtask covers every dev task; a second entry point drifts."
steelman: "just is the ergonomic, discoverable task runner most Rust projects reach for; xtask is more boilerplate."

**E0-27 · `sccache` rejected** (ci, restraint, decided)
occ_ref: BUILDING.md "Deliberately not used".
evidence: "sccache: no compile-time pain at this workspace size."
steelman: "sccache would speed CI/local rebuilds as the workspace grows."

**E0-28 · `cargo-outdated` rejected** (ci, restraint, decided)
occ_ref: BUILDING.md "Deliberately not used".
evidence: "cargo-outdated: Renovate/Dependabot replaces it once activated."
steelman: "cargo-outdated gives an immediate local dependency-freshness view before the bots are wired."

**E0-29 · Blanket private-doc lint rejected** (core, restraint, decided)
`deny(missing_docs)` gates the public API; private items are documented by judgment; a blanket private-doc lint was rejected as comment-noise. (Paired with E0-33.)
occ_ref: journal 2026-07-08 "Docs" bullet + commit c402914.
evidence: "private items documented by judgment only (blanket private-doc lint rejected as comment-noise)."
steelman: "A blanket private-doc requirement guarantees every item is documented, no judgment calls."

---

## Non-decisions — deferred / blocked

**E0-30 · Watch/daemon mode deferred** (cross, non-decision, deferred)
occ_ref: spec §11.
evidence: "Watch/daemon mode." (v1 non-goal, no rationale, no v1.x tag)
blocked_on: post-v1 scope; no v1 use case established.

**E0-31 · Track delay/stretch (`--sync`) deferred** (core, non-decision, deferred)
occ_ref: spec §11.
evidence: "Track delay/stretch (--sync) changes; per-file offsets do not generalize to batch rules; v1.x candidate."
blocked_on: no batch-generalizable model for per-file offsets.

**E0-32 · mkvpropedit metadata-only fast path deferred** (executor, non-decision, deferred)
occ_ref: spec §11.
evidence: "mkvpropedit fast path for metadata-only changes." (v1 non-goal)
blocked_on: v1.x optimization; full remux path ships first.

**E0-33 · On-disk identification cache deferred** (core, non-decision, deferred)
In-memory cache only (keyed path+mtime+size); on-disk cache a future candidate.
occ_ref: spec §5.5 + §11.
evidence: "On-disk cache is a future candidate."
blocked_on: in-memory cache sufficient for v1; no measured need.

**E0-34 · UI localization content deferred (mechanism ships)** (i18n, non-decision, deferred)
The i18n mechanism ships complete; only English catalogs and help topics ship; adding a locale is content work, not a refactor.
occ_ref: spec §2 row 10 + §11.
evidence: "i18n-ready from day one; English-only content ships in v1."
blocked_on: translation content; no target locale committed for v1.

**E0-35 · Coverage tooling (cargo-llvm-cov) deferred** (testing, non-decision, deferred)
Left out of the Plan-1 tooling stock-take ("signal we don't need yet"); revisit at v1.x planning.
occ_ref: BUILDING.md "Deliberately not used".
evidence: "Coverage tooling (cargo-llvm-cov): revisit at v1.x planning."
blocked_on: no measured coverage need in v1.

**E0-36 · Dependabot cadence deferred, framed as a CI-cost decision** (ci, non-decision, deferred)
Activation is Şenol's call; cadence is a CI-cost decision because every dep PR triggers the full 3-OS matrix (hence the monthly lean).
occ_ref: journal 2026-07-08 open threads + ROADMAP residue R3 ("recovered from Plan 1").
evidence: "Dependabot cadence if enabled." / "every dep PR triggers the full 3-OS matrix, so the cadence choice is a CI-cost decision."
blocked_on: go-public/1.0 timing; free only once the repo is public.

---

## Testing strategy — decided (spec §10)

**E0-37 · Property-based tests (proptest) as the correctness core** (testing, pattern, decided)
occ_ref: spec §10 + journal 2026-07-08 testing bullet.
evidence: "matcher + planning semantics: unit tests plus property-based tests (proptest); this is the correctness core."

**E0-38 · Golden tests for command generation** (testing, pattern, decided)
occ_ref: spec §10.
evidence: "command: golden tests, fixture identification JSON -> expected argv."

**E0-39 · Real-mkvmerge integration tests in CI via generated fixtures** (testing, pattern, decided)
occ_ref: spec §10.
evidence: "Integration: real mkvmerge in CI generates tiny fixture MKVs ... end-to-end dry-run and run against them." (Gated tests self-skip until mkvtoolnix is installed in CI — the install landed later.)

**E0-40 · Thin GUI tests because logic lives in core** (testing, pattern, decided)
occ_ref: spec §10.
evidence: "GUI: thin Playwright smoke; logic lives in core, so UI tests stay shallow."

**E0-41 · CLI-rendering snapshot tests (insta)** (testing, pattern, decided)
occ_ref: spec §10.
evidence: "CLI rendering: snapshot tests (insta)."

**E0-42 · CI completeness gates + no-literal-string lint** (testing, pattern, decided)
CI fails on missing catalog keys, diagnostic codes without message templates, help-ids without a topic; eslint no-literal-string keeps hardcoded strings out of the frontend; core prose-free by construction.
occ_ref: spec §10 + commit c7a70f7 ("diagnostic catalog completeness guard").
evidence: "CI fails on catalog keys referenced but missing ... on diagnostic codes without message templates, and on help-ids without a help topic file."

---

## CI / packaging — decided

**E0-43 · CI matrix while private (dynamic fromJSON)** (ci, pattern, decided)
Linux on branch pushes; full 3-OS on PR/tags/dispatch (macOS bills 10x, Windows 2x); revert to full matrix on going public.
occ_ref: spec §10 + journal 2026-07-08 "CI while private" bullet + commit 97ae031.
evidence: "linux-only on branch pushes while private; full matrix on PR/tag/dispatch."
steelman: "Always running the full 3-OS matrix catches cross-platform breaks earlier, at higher Actions-minute cost."

---

## Process doctrine — decided

**E0-44 · Subagent-driven-development execution** (process, pattern, decided)
Plan authored, then executed via SDD: fresh implementer per task, independent task reviewer, fix waves, final whole-branch review.
occ_ref: journal 2026-07-08 "Mechanics" (~31 dispatches: 13 implementers, 13 task reviews, ~7 fix waves, 1 final whole-branch review).
evidence: "~31 subagent dispatches: 13 implementers, 13 task reviews, ~7 fix/re-review waves, 1 docs pass, 1 final whole-branch review."

**E0-45 · The plan does not grade its own work** (process, pattern, decided)
An independent final whole-branch review grades the plan's code against the plan's own constraints.
occ_ref: journal 2026-07-08 "Moments".
evidence: "Final reviewer graded the plan's own code against the plan's constraints and failed it on three counts; 'the plan does not grade its own work' proved to be the load-bearing process rule."

**E0-46 · Controller re-runs every suite; report arithmetic not trusted** (process, pattern, decided)
occ_ref: journal 2026-07-08 "evidence-integrity finds".
evidence: "haiku implementers mis-totaled workspace test counts in 5 of 13 reports (code correct each time) -> controller began independently re-running every suite" (also: one synthesized test transcript caught at T3 review).

**E0-47 · Model split by task type** (process, pattern, decided)
haiku for transcription implementers (plan carries complete code), sonnet for judgment implementers and all task reviewers, fable for the final review.
occ_ref: journal 2026-07-08 "Mechanics" model-split bullet.
evidence: "haiku for transcription implementers ... sonnet for judgment implementers ... and all task reviewers, fable for the final review."

**E0-48 · Spec-wins-on-conflict (code-wins only on a spec error)** (process, pattern, decided)
On a plan/code-vs-spec conflict the spec wins and code is fixed; where the spec itself is wrong (naming collision) the spec is amended and code kept.
occ_ref: journal 2026-07-08 "What the process caught" + commits 3c24845, cd3f239/f7afa8d.
evidence: "spec-wins rule applied" (prose fix) vs "UnknownProperty name collision between spec table and code -> spec amended, code kept."

**E0-49 · gh-log push-audit convention** (process, pattern, decided)
Every push logged to a gitignored operational `gh-log.md` (not repo content).
occ_ref: journal 2026-07-08 "Friction" + commit fad067c.
evidence: "gh-log.md audit convention added the same day."

**E0-50 · Unsigned agent commits (GPG workaround)** (process, pattern, decided)
GPG signing blocks agent commits; standing workaround `-c commit.gpgsign=false`. (The "signature = authorship claim, stay unsigned as policy" framing is Şenol's 2026-07-09/10 decision — not E0.)
occ_ref: journal 2026-07-08 "Friction".
evidence: "GPG signing blocks agent commits; standing workaround -c commit.gpgsign=false."

**E0-51 · Repo-scoped commit authorization (no-unrequested-commits reversed here)** (process, pattern, decided)
The global no-unrequested-commits rule was explicitly reversed for this repo mid-session.
occ_ref: journal 2026-07-08 "Friction".
evidence: "Mid-session rule change (no unrequested commits) later explicitly reversed for this repo."

**E0-52 · Verify archive commits by commit stat, not working tree** (process, pattern, violated-corrected)
A copied `.superpowers/sdd` dir carried a bare-`*` `.gitignore` that silently excluded all 49 artifacts; caught by reading the commit stat line.
occ_ref: journal 2026-07-08 "Addendum" + commit 411087f.
evidence: "the copied sdd directory carried the tooling's own .gitignore (a bare *), silently excluding all 49 artifacts; caught by reading the commit stat line, fixed in 411087f."

---

## Doc standard — decided

**E0-53 · Rustdoc states meaning, not name echo** (core, pattern, decided)
`deny(missing_docs)` enforces presence only; the meaning/contract/edge-case bar is the quality gate.
occ_ref: BUILDING.md "Documentation standard" (agreed Plan 1) + journal 2026-07-08.
evidence: "Rustdoc states MEANING, not a name echo ... #![deny(missing_docs)] enforces presence only; this line carries the quality bar (agreed Plan 1, previously chat-only)."

---

## Enforcement occurrences (violated-corrected) — E0 defects the process caught

**E0-54 · Unknown-key rejection enforced inside untagged config blocks** (core, pattern, violated-corrected)
`serde deny_unknown_fields` was silently ineffective on untagged inline struct variants → TemplateBlock/ExternalBlock newtype fix. (Enforcement occurrence of E0-17.)
occ_ref: journal 2026-07-08 "What the process caught" (T4) + commit b5eaa3d.
evidence: "serde deny_unknown_fields silently ineffective on untagged inline struct variants (task review T4) -> TemplateBlock/ExternalBlock newtype fix."

**E0-55 · Edition-2024 keyword collision: rename over edition-downgrade** (core, pattern, violated-corrected)
`gen` reserved in edition 2024; implementer's edition-2021 downgrade workaround rejected by controller → module renamed to `codegen` (root-cause over symptom).
occ_ref: journal 2026-07-08 "What the process caught" (T5) + commit e78847d.
evidence: "implementer's edition-2021 downgrade workaround rejected by controller -> module renamed."
steelman (rejected workaround): "Downgrading the crate to edition 2021 would have compiled immediately without renaming anything."

**E0-56 · DiagCode key/serde consistency enforced by exhaustive test** (testing, pattern, violated-corrected)
`key()`/serde kebab encodings were unlinked → `DiagCode::ALL` + exhaustive consistency tests (later the T13 catalog guard was upgraded to iterate DiagCode::ALL).
occ_ref: journal 2026-07-08 "What the process caught" (T2) + commit a7c0d89.
evidence: "key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL + exhaustive consistency tests."

**E0-57 · --json output sorted identically to text** (cli, pattern, violated-corrected)
`--json` output was unsorted while text was sorted → sort diagnostics before json output (surface parity).
occ_ref: journal 2026-07-08 "What the process caught" (T12) + commit ad841b0.
evidence: "--json output unsorted while text sorted (task review T12)."

**E0-58 · Template-error prose leaked out of core** (i18n, pattern, violated-corrected)
Template-error params carried English prose out of core, violating "core emits no user-facing prose" → prose-free template errors (enforcement occurrence of E0-11; spec-wins rule applied).
occ_ref: journal 2026-07-08 "What the process caught" (final review) + commit 3c24845.
evidence: "Template-error params carried English prose out of core, violating the plan's own exit criterion (final review; spec-wins rule applied)."

**E0-59 · Five restatement rustdocs caught by review** (core, pattern, violated-corrected)
Enforcement occurrence of the E0-53 doc-quality bar during the 151-item backfill.
occ_ref: journal 2026-07-08 "Rustdoc" + commit 9a7f49f.
evidence: "5 restatement-style docs caught by review" → "sharpen the five restatement rustdocs from review."
