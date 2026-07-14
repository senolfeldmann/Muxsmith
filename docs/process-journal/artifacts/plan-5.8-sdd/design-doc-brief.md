# Task brief: author the Plan 5.8 decisions document (four-eyes, implementer role)

You are the implementer in a four-eyes loop. You author ONE artifact; an
independent reviewer will grade it against this brief. Work only in
/home/senol/Git/Muxsmith.

## Deliverable

Create exactly one new file:

    docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md

containing two Architecture Decision Records, D38 and D39. Model the file's
structure, register, and slot layout on the existing
`docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md` (read it first):
H1 "Plan 5.8 decisions", one H2 per ADR, bold slots **Decision**,
**Rationale**, **Rejected alternatives**, **Interface/wire-format change**,
**Triggers created**, **Consistency note** (the SI-3 mkvtoolnix comparison).
D38 additionally carries a **Supersedes** slot and both ADRs carry a
**Spec amendments** slot (see below).

Do NOT modify any other file. The spec amendments and code changes the
document names are Plan 5.8 execution tasks, not part of this task. Do not
commit; the controller commits after review approval.

## Ground truth (read these before writing)

- `docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md` - structural model.
- `docs/product-boundaries.yaml` entry `core-83-zero-rule-keep-passthrough`
  (around line 388) - the owner ruling D38 implements; quote its substance
  faithfully.
- `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md` around
  line 157 ("Open mechanics (plan-time)", "Zero rules under `keep`") - the
  marked assumption D38 supersedes.
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` sections 4.5,
  5.2 (diagnostics table incl. the EmptyPlan and InvalidPropertyValue rows),
  5.4, 8.4.
- Code sites (verify every line number you cite by reading the file):
  - `crates/muxsmith-core/src/profile/validate.rs:61-63` - the
    unconditional NoTrackRules error on empty `tracks.rules`.
  - `crates/muxsmith-core/src/profile/model.rs` - `TracksCfg.unmatched:
    KeepDrop`.
  - `crates/muxsmith-core/src/planner.rs:440-448` (walk_exact_languages)
    and `:811-824` (resolve_changes) - the two emitters passing the English
    prose string "a valid ISO 639/BCP-47 language code" as the `allowed`
    param, with the comment at :819-822.
  - `crates/muxsmith-core/src/profile/validate.rs:274-289` - the
    closed-domain emitter (type/codec_kind) passing a real value list via
    `domain_hint`.
  - `locales/en/diagnostics.ftl:7` and `locales/de/diagnostics.ftl:14`
    (no-track-rules), `locales/en/diagnostics.ftl:43` and
    `locales/de/diagnostics.ftl:50` (invalid-property-value).
  - `crates/muxsmith-cli/tests/catalog_completeness.rs` - `fixture_args`
    (InvalidPropertyValue entry around line 63) and the site-level leak
    test `invalid_changes_language_diagnostic_renders_without_placeholder_leak`
    (around line 397) with the doc comment at lines 41-47.
  - `src/diagnosticFluentParams.ts` - the strictness doc comment (lines
    19-27) claiming scientific-notation rejection.
- `docs/ROADMAP.md` Triggers section: the entry "Any change to diagnostic
  wire params or NUMERIC_DIAGNOSTIC_PARAMS -> fix the
  diagnosticFluentParams.ts strictness comment (or implement /^\d+$/)
  (routed-items item 7)".

## Requirements

### R1 - D38: zero-rule-keep passthrough

Record this decision (all points are owner-approved 2026-07-14; transcribe,
do not re-decide):

1. A profile with empty `tracks.rules` and `tracks.unmatched: keep` is a
   LEGAL pure-passthrough remux (change only title / attachments /
   chapters, or normalize the container). Empty rules with
   `unmatched: drop` (the default) stays a `NoTrackRules` error.
2. validate.rs lifts the error only for the keep case. The keep case
   instead emits a NEW info-severity diagnostic `DiagCode::PassthroughProfile`
   (config_path `tracks.rules`, no params) at validate time, confirming the
   profile is a deliberate passthrough - this keeps an accidental
   delete-all-rules edit visible.
3. The `no-track-rules` catalog message (en + de) gains a pointer hint:
   at least one rule is required, OR set `tracks.unmatched: keep` for a
   pure passthrough remux. Propose exact en + de strings in the document
   (marked as proposals for owner wording review; match the existing de
   catalog register, e.g. "Spurregeln").
4. Bilingual catalog entries for the new `passthrough-profile` message
   (propose exact en + de strings).
5. README gains a short passthrough recipe subsection in the profile/usage
   area (name the placement; content: the unmatched:keep idiom, the three
   use cases). The GUIDE version lands at 1.0, not now.
6. Tests the plan must carry: validate_semantics cases (keep+zero -> info,
   not error; drop+zero -> error), catalog_completeness fixture for the new
   code, and an end-to-end dry-run + run of a zero-rule-keep profile
   proving the executor path (the D20 "passthrough counts as matched"
   machinery; spec 5.2's EmptyPlan row already exempts the case).
7. **Supersedes** slot: the D20 "Open mechanics" marked assumption
   ("Zero rules under keep ... Assumption: still an error", plan-3.5
   decisions file; H12) is superseded by this decision; the execution plan
   adds a "superseded by D38" annotation at that spot in the plan-3.5 file.
8. **Spec amendments** slot: name the v1-design.md edits the plan must
   make - 4.5 (zero-rules legality under keep), 5.2 (new PassthroughProfile
   row), 5.4 (the static-lint prose "at least one rule" becomes
   policy-dependent, check the actual wording). Note the required
   self-contradiction sweep per the ADR convention.
9. Rationale cites: owner ruling 2026-07-13 (escalation resolution, session
   10; recorded in product-boundaries.yaml core-83), executor path existing
   since D20, core-83's "MUST be documented and hinted".
10. Rejected alternatives (with why): keep erroring under keep (contradicts
    the ruling; was exactly the superseded assumption); warning severity
    (nags the sanctioned use case); silently legal with no diagnostic
    (accidental rule deletion becomes invisible; core-83 mandates the
    hint); GUIDE-only documentation (GUIDE lands at 1.0; the error message
    is the discovery moment).
11. Consistency note (SI-3): classify as MATCH with mkvmerge default
    semantics - `mkvmerge -o out.mkv in.mkv` copies all tracks; a
    passthrough remux is mkvtoolnix-gui's trivial base case; declarative
    Muxsmith needs an explicit profile idiom for it, which is
    `unmatched: keep` + zero rules.

### R2 - D39: allowed-param wire cleanup + catalog selector

1. Catalog-side: `invalid-property-value` (en + de) becomes a Fluent
   select on the existing `$property` param: a `[language]` arm rendering
   registry-membership wording ("must be a valid ISO 639 or BCP-47
   language code", localized properly in de) WITHOUT `$allowed`; the
   default `*[other]` arm keeps the current allowed-list wording. Propose
   exact en + de message bodies.
2. Core-side: the two planner.rs emitters stop sending the English-prose
   `allowed` param (property=language emissions carry `property` + `value`
   only). The validate.rs closed-domain emitter (type/codec_kind, real
   value lists via domain_hint) is unchanged - its lists are locale-neutral
   tokens.
3. Wire-format slot: this is a param REMOVAL on the diagnostics JSON
   surface for property=language emissions of invalid-property-value;
   type/codec_kind emissions unchanged; pre-1.0, no compatibility promise.
4. Test topology the plan must carry: `fixture_args` for
   InvalidPropertyValue switches to the `*[other]` arm (e.g. property
   "type" with a real allowed list) so the list-arm placeholder is
   fixture-guarded, while the existing site-level leak test (which renders
   the REAL resolve_changes emitter output) pins the language arm - both
   arms covered, respecting the documented single-fixture-per-code
   limitation. The stale emitter-site comment at planner.rs:819-822
   ("requires allowed ... or {$allowed} leaks") is updated.
5. Trigger consumption: the ROADMAP trigger "any change to diagnostic wire
   params -> fix the diagnosticFluentParams.ts strictness comment (or
   implement /^\d+$/) (routed-items item 7)" FIRES with this change and is
   consumed by Plan 5.8: the same diff fixes the strictness doc comment or
   implements the /^\d+$/ guard (the current comment overclaims - Number()
   accepts scientific notation that normalizes to an integer, e.g.
   Number("1e3") === 1000 passes Number.isInteger). Record it as
   fired-and-consumed in the Triggers slot.
6. Rationale cites: the pre-1.0 whole-branch finding I2 (mixed-language
   render in de mode), root cause = core emitting user-facing English
   prose in a param, violating the spec 5.2 sentence "Core emits no
   user-facing prose" and absent from the 8.4 exception list; selection on
   the existing property param needs no new wire element.
7. Rejected alternatives (with why): a new `allowed-kind` wire param (adds
   a wire element where an existing param already discriminates); a new
   dedicated DiagCode for language values (heavier wire change, new
   catalog keys and fixtures, no benefit over the selector); keep sending
   the prose param unused (dead English prose on the JSON wire, spec
   violation stays); amending the 8.4 exception list to legalize the prose
   (treats the symptom, contradicts the prose-free-core architecture).
8. Spec amendments slot: 5.2 InvalidPropertyValue row (params/wording if
   the row names `allowed`; check the actual row text) - and state
   explicitly that 8.4 needs NO change because the prose leaves core
   instead of being excepted.
9. Consistency note (SI-3): mkvtoolnix ships fully localized catalogs; a
   mixed-language diagnostic render is a genuine gap against that bar;
   closing it is a MATCH.

### R3 - scope boundaries section

A short closing section "Deliberately out of scope" naming: batch-level
settable-language check (v1.x, D18 remainder); GUIDE passthrough text (at
1.0); the README magic-matching content anchor (D32 addendum) untouched.

### R4 - style and correctness constraints

- English, matching the register and typography of the plan-5.7 file: no
  em/en dashes (use "-"), straight quotes, no Unicode ellipsis.
- Every code claim carries file:line, and you have verified each cited
  line number by reading the file in its current state.
- Quote code and existing catalog strings exactly; never from memory.
- Where the brief leaves wording latitude (catalog strings, README
  placement), mark the passage as a proposal for owner review.
- This file is committed to a PUBLIC repo: no references to anything
  outside the repository (no home paths, no agent-framework or private
  project material). In-repo references (ADR numbers, session-10
  escalation, owner ruling, routed-items verdict) follow the existing
  house pattern.

## Report contract

Write your report to
`.superpowers/sdd/plan-5.8/design-doc-implementer-report.md`: per numbered
requirement (R1.1-R1.11, R2.1-R2.9, R3, R4) one line on how it is met, plus
anything you flag. End with exactly one status:

- DONE - complete and self-verified against every requirement.
- DONE_WITH_CONCERNS - complete, but name each residual concern.
- NEEDS_CONTEXT - a requirement is unclear or two conflict; name it, stop.
- BLOCKED - cannot proceed; name the blocker.

Your final message: the status line plus the report file path.
