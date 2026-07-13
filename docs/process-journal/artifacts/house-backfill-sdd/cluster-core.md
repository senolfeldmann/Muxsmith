# House-knowledge clusters - domain `core`

Reconstructed from 179 occurrence records spanning eras E0-E8. Records were grouped by identical `(topic, approach)`; occurrences merged, identical `date+ref` deduped, distinct cited artifacts kept as genuine recurrence. `promoted = count >= 3`. (Cluster IDs run to 121; 49 and 101 are intentionally vacant - folded into core-37 and core-35 respectively - so the roster is 119 clusters.)

**Occurrence-counting rule (same as sibling domains).** An occurrence = one distinct cited *artifact/attestation point*: a design spec, a dated decision-journal entry, a design memo (`Dnn`), an implementation commit-set, a handoff, a plan doc, an independent review event, a per-task review verdict, a whole-branch/FINAL review, `BUILDING.md`/`CONVENTIONS.md`/`IDEAS.md`/`ROADMAP`. Collapsed to one occurrence: multiple sections/bullets of the same document; a co-cited commit-set implementing one work item; a single review event cited via both its verdict file and a journal/progress mention. Kept distinct: a fix commit that follows a review is its own touchpoint (find-vs-fix are two events).

**E0 and E1 are reconstruction eras, not separate decision events.** The E1 records re-attest the same 2026-07-08 Plan-1 decisions with additional downstream artifacts (commit, handoff). They are merged; the count reflects the distinct artifacts, not the number of era-records.

**Dates** are reconstructed from the era and the ref, not git-verified for this pass: E0/E1 = 2026-07-08 (Plan 1); E2/E3/E4 = 2026-07-09 (Plans 2 / 3 / 3.5); E5/E6 = 2026-07-10 (cleanup / GUI-Plan 5, inferred); E7 = 2026-07-11 (Plan 5.5; ratification addenda 2026-07-12); E8 = 2026-07-12 (convention/idiomacy extraction).

38 of 119 clusters reach promotion. The largest recurrences are the prose-free-core boundary (6), the batch-wide SourceOverwrite protection set (5), the unknown-keys-are-errors rule (5), and the "no mkvtoolnix magic guesses" restraint (4), each touched across three or more plans.

---

## Promoted clusters (recurred >= 3)

### core-01-rule-uniqueness - strict independent uniqueness
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Every rule resolves against all tracks independently, no consumption, no order effects; overlaps and multiple candidates are errors, configs spell out exclusions explicitly (`forced_track: false`), error quality and the suggestion engine compensate.
- **Steelman:** null (the rejected alternatives are core-02).
- **Occurrences:** 2026-07-08 decided (spec §2 rule-semantics row); 2026-07-08 decided (journal Plan 1); 2026-07-08 decided (commit 61249f9).

### core-03-suggestion-verified-edit - batch-wide simulated suggestions
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Suggestions are structured edits simulated against the cached identification of the whole batch before being shown; only refinements that resolve every instance and add no new diagnostic are emitted; an applied suggestion survives the next dry run (re-run through the real planning pass, no parallel matcher).
- **Steelman:** null.
- **Occurrences:** 2026-07-08 decided (spec §2/§5.3, Şenol's amendment); 2026-07-08 decided (journal Plan 1); 2026-07-08 decided (commit 61249f9); 2026-07-09 decided (memo D6 steps 3-4).

### core-04-mkv-structure-full-control - all MKV structure configurable in v1
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** v1 configures tracks, attachments, chapters, tags and title, accepting the larger surface deliberately over agent-recommended global toggles.
- **Steelman:** null (rejected alternative is core-05).
- **Occurrences:** 2026-07-08 decided (spec §2 MKV-structure row); 2026-07-08 decided (journal Plan 1); 2026-07-08 decided (commit 61249f9).

### core-06-schema-build-time-extraction - xtask codegen, schema never vendored
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Matchable property names/types are generated into the capability model at build time by an xtask into a committed `generated.rs`; only derived facts ship, the identification schema itself is never redistributed, and there is no `build.rs` network dependency.
- **Steelman:** null (rejected alternative is core-07).
- **Occurrences:** 2026-07-08 decided (spec §2 rows 8-9 + §9); 2026-07-08 decided (journal Plan 1); 2026-07-08 decided (commits 830dc47/4750abb/61249f9, impl set); 2026-07-09 decided (handoff plan-1-close).

### core-12-unknown-keys-are-errors - deny_unknown_fields, hard error
- **kind:** pattern | **status:** settled | **count:** 5 | **promoted:** yes (at 3)
- **Statement:** An unknown profile key is a config-time hard error, not a warning (explicit over silent); `#[serde(deny_unknown_fields)]` on every profile struct. Enforcement occurrence: the attribute was silently ineffective on inline untagged struct variants until the TemplateBlock/ExternalBlock newtype fix restored rejection.
- **Steelman:** null.
- **Occurrences:** 2026-07-08 decided (spec §4 + Plan 1 Global Constraints); 2026-07-08 decided (handoff plan-1-close); 2026-07-08 decided (commit 1f00aa6); 2026-07-08 violated-corrected (task-4 review event); 2026-07-08 violated-corrected (fix commit b5eaa3d).

### core-21-rustdoc-states-meaning - quality bar beyond presence
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `deny(missing_docs)` enforces presence only; the quality bar is that rustdoc states meaning, contract and edge cases, not a name echo. Enforcement occurrence: five restatement-style docs from the 151-item backfill were caught by review and sharpened.
- **Steelman:** null.
- **Occurrences:** 2026-07-08 decided (BUILDING.md Documentation standard, agreed Plan 1); 2026-07-08 violated-corrected (journal Rustdoc); 2026-07-08 violated-corrected (commit 9a7f49f).

### core-22-edition-2024-codegen-rename - root-cause over downgrade
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `gen` is reserved in Rust edition 2024; the implementer's edition-2021 downgrade workaround was rejected by the controller and the module renamed to `codegen` instead.
- **Steelman:** Downgrading the crate to edition 2021 would have compiled immediately without renaming.
- **Occurrences:** 2026-07-08 violated-corrected (journal T5); 2026-07-08 violated-corrected (task-5 review verdict); 2026-07-08 violated-corrected (commit e78847d).

### core-24-diagcode-key-integrity - tie key() and serde encoding
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The hand-authored `.key()` literals and serde's kebab rename were two independent encodings linked only by hand; T2 flagged the divergence risk, corrected with `DiagCode::ALL` plus exhaustive consistency and uniqueness tests.
- **Steelman:** null.
- **Occurrences:** 2026-07-08 violated-corrected (task-2 review verdict); 2026-07-08 violated-corrected (journal); 2026-07-08 violated-corrected (commit a7c0d89).

### core-26-unknownproperty-split - config-error vs skew-warning names
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The code used `unknown-property` for a config-time typo error while spec 5.2 defined `UnknownProperty` as the plan-time skew warning; resolved by amending the spec (split into `UnknownProperty` error + `UnknownPropertySkew` warning) and keeping the code - a deliberate spec-wins exception because the code was correct.
- **Steelman:** null.
- **Occurrences:** 2026-07-08 violated-corrected (whole-branch review Important #2); 2026-07-08 violated-corrected (journal); 2026-07-08 violated-corrected (fix commits cd3f239/f7afa8d).

### core-29-no-value-domain-codegen - curate value domains by hand
- **kind:** restraint | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** No xtask codegen path for value domains; `type`/`codec_kind` domains are curated by hand in capability, because the v20 schema types `type` as a plain string and only `aac_is_sbr` carries a schema enum - codegen would serve one irrelevant field.
- **Steelman:** Generate value domains from the pinned schema at build time for consistency with the identification-schema extraction - rejected because only one irrelevant field would benefit; the abstraction the scale had not earned.
- **Occurrences:** 2026-07-09 decided (journal Plan 2, Decisions); 2026-07-09 decided (memo D2 note); 2026-07-09 decided (commit 7e02f86).

### core-31-rendered-filename-invariant - guard the invariant post-interpolation
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** The planner re-checks the rendered filename independently of the config-time template-text check (no separators, non-empty stem, not `.`/`..`); new per-file errors `PathSeparatorInRenderedName` + `EmptyRenderedName`. Enforcement occurrence: the empty-stem case (`template: ".mkv"` -> hidden `.mkv`) slipped through the fix pass because the check ran on the pre-append value; the whole-branch review caught what every per-task review missed.
- **Steelman:** Today all template fields are basename-derived so separators cannot occur and the check is dead - rejected: guard the invariant, not the induction proof over field sources.
- **Occurrences:** 2026-07-09 decided (memo D4); 2026-07-09 decided (commit af76a3a); 2026-07-09 violated-corrected (FINAL review I1); 2026-07-09 violated-corrected (commit 59d24c8).

### core-34-suggestion-cap-non-silent - cap 3, log the truncation
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Deterministic preference order among accepted candidates; emit at most 3 per conflict group; if more were accepted, log the cap (`SuggestionsCapped` info) - bounded output, no silent truncation. Enforcement occurrence: the inline code truncated silently; F7 added the diag code threaded through report.rs + diagnostics.ftl + spec 5.2.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (memo D6 step 5); 2026-07-09 violated-corrected (independent review nit); 2026-07-09 violated-corrected (F7 review); 2026-07-09 violated-corrected (commit 68ec6aa).

### core-35-overlap-auto-suggestions - deferred in Plan 2, decided as D33 policy 3
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** OverlappingRules auto-narrowing suggestions: generate narrowing candidates for ALL overlap claimants symmetrically (including claimant 0); the existing `resolves_without_regression` acceptance criterion selects the feasible ones - feasibility, not a precedence guess, decides which rule narrows. Deferred out of Plan 2, decided as D33 in Plan 5.5, shipped at T18.
- **Steelman:** null (the rejected selection policies are core-102).
- **Occurrences:** 2026-07-09 deferred (journal Plan 2, Open threads); 2026-07-11 decided (memo D33, Şenol); 2026-07-11 reinforced (task-18 verdict).

### core-37-prose-free-core - core emits code+params only, text lives outside
- **kind:** pattern | **status:** settled | **count:** 6 | **promoted:** yes (at 3)
- **Statement:** `muxsmith-core` emits a diagnostic code + structured params only; all human text lives in the Fluent catalog / an injected `DiagnosticRenderer` port; core never originates, hardcodes or localizes prose. Recurring enforcement: bug K (identification failure misreported as MissingTrack with an authored English string) fixed to emit `UnidentifiableSource` carrying only the third-party error text (pass-through allowed).
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (independent review bug K); 2026-07-09 violated-corrected (F5 fix, commits 0e141d1/6f475b3); 2026-07-09 deferred (FINAL review M3 + F5-review Minor #4, residual English framing accepted); 2026-07-09 reinforced (Plan 3 Global Constraints); 2026-07-10 decided (task-2 verdict, DiagnosticRenderer port/adapter); 2026-07-12 reinforced (CONVENTIONS.md Patterns).

### core-38-absent-bool-equals-false - mirror mkvmerge/Matroska for exact match
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** An absent boolean-typed matchable property compares equal to `false` for exact matching (mirrors mkvmerge/Matroska), so `exact:{flag:false}` matches a track lacking the flag; `:true` still does not. Scope is any boolean matchable, not just the four vanity flags.
- **Steelman:** The reference profile 4.1 dodges it via `not:[exact:{flag:true}]`, so absent-is-never-equal was defensible until the `flag:false` ergonomic case.
- **Occurrences:** 2026-07-09 decided (independent review decision #1(b)); 2026-07-09 decided (F4 review, SPEC pass); 2026-07-09 decided (commit 213e1e9).

### core-40-output-collision-planned-twice - always a hard error
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Two planned outputs colliding is always a hard `OutputCollision` error (drop both plans) regardless of `on_collision`; the policy (error/warn+drop/info+keep) governs only pre-existing on-disk files. Corrected an inline deviation that collapsed Overwrite into Error and made skip a no-op; spec 4.8 amended.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (independent review decision #3(c)); 2026-07-09 decided (F6 review); 2026-07-09 decided (commit b5acada).

### core-41-sourceoverwrite-batch-wide - protect all input/donor paths, batch-wide
- **kind:** pattern | **status:** settled | **count:** 5 | **promoted:** yes (at 3)
- **Statement:** Collect all input paths batch-wide (primaries + every resolved track/attachment/chapters donor) before rendering and fire `SourceOverwrite` if any rendered output equals any of them; capture is independent of render success, so donors of render-failed files are protected too. The protection set was repeatedly found incomplete (per-primary scoping, render-failed donors, attachment donors, chapters donors) and progressively closed by construction.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (F5 review, per-primary -> batch-wide, commit 6f475b3); 2026-07-09 deferred (FINAL review M2, render-failed-donor gap accepted for v1); 2026-07-11 decided (task-7, render-failed donors protected); 2026-07-11 reinforced (task-7.5, attachment donors); 2026-07-11 reinforced (task-7.6, chapters donors, class closed by exhaustive match).

### core-42-unidentifiable-hard-regardless-optional - broken file is not "zero matches"
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** A present-but-unidentifiable source is a hard `UnidentifiableSource` error regardless of `rule.optional` (optional means "zero matches acceptable", not "a broken file is acceptable"); spec 5.2 makes it unconditional. The fix-plan draft that said donor-identify-failure should respect optional was superseded and the plan text reconciled to shipped behavior.
- **Steelman:** Make donor-identify-failure respect optional, consistent with the zero-hits branch - rejected because spec 5.2 makes `UnidentifiableSource` unconditional.
- **Occurrences:** 2026-07-09 decided (fix plan F5 text, superseding draft); 2026-07-09 decided (F5 review Important #2); 2026-07-09 decided (commit 953c5cd).

### core-44-suggestion-no-clobber - never widen an existing match
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `with_rule_match` must not overwrite an existing exact/substring key (`or_insert` semantics); a clobbering candidate becomes a no-op then rejected by the acceptance sim; not-list append stays additive. Bug C: the inline engine merged via `BTreeMap::extend`, so an `AddSubstring` candidate widened an existing `track_name` substring (violating D6 "never relax") and the acceptance check missed it when the widened match did not happen to collide.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (independent review bug C); 2026-07-09 violated-corrected (F7 review (a)); 2026-07-09 violated-corrected (commit 68ec6aa).

### core-45-yaml-fragment-serializer - render suggestions via the serializer
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Render the suggestion `yaml_fragment` by serializing the real `MatchExpr` delta via `yaml_serde`, not by hand-formatting, so any value (colon/comma/brace, bool/int) round-trips. Bug D: the inline code interpolated values unquoted, breaking the YAML the CLI prints verbatim; the serializer also fixed an unstated bool/int-stringification bug ("Norway problem" among 40 adversarial inputs).
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (independent review bug D); 2026-07-09 violated-corrected (F7 review (b)); 2026-07-09 violated-corrected (commit 68ec6aa).

### core-46-symlink-source-discovery - resolve regular-file symlinks, guard cycles
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** A symlink whose target is a regular file must be discovered (classify via `fs::metadata`); do not recurse into directory symlinks (cycle guard); skip broken symlinks silently. Bug I: `walk_files` under `symlink_metadata` dropped symlinked source files (neither `is_file` nor `is_dir`) with no diagnostic.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (independent review bug I); 2026-07-09 violated-corrected (F8 review, SPEC pass); 2026-07-09 violated-corrected (commits cb3ae84/608f2b5).

### core-47-with-severity-builder - encapsulate severity mutation
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Add a rustdoc'd `Diagnostic::with_severity` builder and use it instead of mutating the public `severity` field directly (`Diagnostic::info` then `.severity = ...`).
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (fix plan F6); 2026-07-09 violated-corrected (independent review Reviewer 2 #10); 2026-07-09 violated-corrected (F6 review).

### core-48-extension-validation - --list-types, once per batch, degrade-with-warning
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Profile and locator extensions are validated against mkvmerge `--list-types` once per batch (not per file), degrading to a warning when mkvmerge is absent, mirroring the language-validation walk; new `UnknownExtension` warning. Deferred without a diag code in Plan 2, built for profile extensions at T5 and for locator extensions (recursive walk reusing `validate_extension_values`) at T5.9.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 deferred (journal Plan 2, Open threads, no diag code yet); 2026-07-11 decided (task-5, profile extensions); 2026-07-11 decided (task-5.9, locator extensions).

### core-54-reuse-plan2-machinery - wire new resolution sites to existing mechanisms
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Every new Plan-3 resolution site reuses a Plan-2 mechanism (locator machinery, template engine, `LanguageIndex`, existing `DiagCode` variants) rather than introducing a new one; add no new DiagCodes; the attachment-scope call is the only genuinely new semantic. Defaults (unmatched keep, chapters keep, tags keep/keep) match spec 4.9 so an omitted section never silently drops data.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (memo D10 Rationale); 2026-07-09 decided (Plan 3 Global Constraints); 2026-07-09 reinforced (whole-branch verdict Strengths).

### core-58-attach-add-cardinality - add attaches ALL matched, zero-match warns
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** An `add` locator attaches all matched files (collection-populator, like select/drop), dedup by path; zero matches emit a warning (not an error) that does not suppress the plan. The real invariant is slot-vs-collection: track/chapters donors fill a unique slot (error on 0/>=2), select/drop/add populate the attachment collection, so add-attaches-all is the consistent rule, not a font special case.
- **Steelman:** Treat `add` like a track/chapters donor (exactly-one, error on 0 or >=2) so a font an ASS subtitle needs fails loudly - rejected via a two-round debate; exactly-one would instead make `add` the lone unique attachment rule kind.
- **Occurrences:** 2026-07-09 decided (memo D12); 2026-07-09 decided (commit 62d4956); 2026-07-09 reinforced (journal Plan 3, two-round debate ratified); 2026-07-09 reinforced (task-8 verdict, zero findings).

### core-59-matchable-trait - one match algebra for tracks and attachments
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The matcher is genericized over a `Matchable` trait implemented for both `Track` and `Attachment` so one match algebra evaluates both. The plan brief's regression-guard claim ("resolves via type inference") was factually wrong because `Iterator::filter` hands `&&Track`; solved with a blanket `impl<M: Matchable> Matchable for &M`, verified sound and right-sized.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (Plan 3 Architecture + Task 2/3); 2026-07-09 decided (journal Plan 3, &&Track unification); 2026-07-09 decided (task-2 verdict, blanket &M impl).

### core-63-input-group-membership - exclude zero-track donor sources
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** `input_groups` = primary always (group 0) + donor sources with at least one `track_id=Some` assignment; exclude track-less donors (they would produce empty `--no-*` groups). The first implementation unconditionally included every distinct `assignment.source`, a misreading of the canonical reference's primary-only carve-out. Separately, the exclusion-comment's stated reason ("mkvmerge may reject empty groups") was empirically false against v100 (which accepts them) and corrected while keeping the guard.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 violated-corrected (task-9 verdict, membership fix + test); 2026-07-09 violated-corrected (commit d55f19d); 2026-07-09 violated-corrected (whole-branch verdict Minor 2, false rationale); 2026-07-09 violated-corrected (commit 7d46547).

### core-65-empty-plan-warning - a zero-track plan must say so
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** A plan resolving to zero track assignments emits `EmptyPlan` (warning, per-file, batch-visible); keep-mode plans are naturally exempt (always carry the primary's tracks). A deliberate divergence from mkvtoolnix-gui: an interactive GUI shows the selection state by construction, a declarative batch tool must state it. Deferred twice (Plan 3, Plan 4/D18) before being built at T6, where the check was relocated post-finalize (gated on `plan.is_some()`), closing an unenumerated `CollisionPolicy::Skip` false-warning case.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 deferred (whole-branch verdict Minor 3, Plan 3); 2026-07-10 deferred (memo D18, Plan-4 cleanup); 2026-07-11 decided (task-6, EmptyPlan built); 2026-07-11 violated-corrected (task-6 fix a60e9a0, relocation).

### core-67-no-mkvtoolnix-magic-guesses - the profile is the spec
- **kind:** restraint | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Filename/track-name-derived language and flags, auto-title, unique-name `(1)` suffixing and numeric-sequence auto-append are deliberately not emulated (IDEAS.md #1-4, all hard no). Muxsmith is declarative-batch: a guess would fire unseen across hundreds of files with no review step; fail loud or stay out of the model instead.
- **Steelman:** mkvtoolnix-gui enables all of these by default (`m_setCommentaryFlagFromFileName`, filename language for audio/subs, `m_uniqueOutputFileNames`, `m_mergeReconstructSequencesWhenAdding`) - high value for a user who reviews each file interactively.
- **Occurrences:** 2026-07-09 decided (IDEAS.md #1-4); 2026-07-09 decided (commit b04c4a2); 2026-07-09 decided (journal Plan 3.5, four hard-nos); 2026-07-12 reinforced (CONVENTIONS.md Restraints).

### core-72-exact-typed-value-equality - not raw string equality
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `exact` compares each property in its own domain (numbers numerically, languages canonicalized so `de==ger`, `pt-Latn-BR==pt-BR`), preserving meaningful distinctions; `regex` is the byte-literal escape hatch. One of the tool's core semantics; surfaced in spec 4.3 at 1.0.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (memo D19 Principle); 2026-07-09 decided (spec 4.3, Task 5 step 6); 2026-07-09 reinforced (whole-branch verdict, load-bearing-and-easy-to-lose flag).

### core-80-keep-mode-track-order - primary first, donors trail (reversed once)
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Under `keep`, `--track-order` lists all primary tracks first in source order (`0:id`), donors trailing (`g:id` in rule order). The initial assumption (only matched tracks listed, mkvmerge appending kept-unmatched after) placed a donor-only-keep profile's added track ahead of the primary's video/audio; the whole-branch review flagged this as a usability trap on the one workflow keep exists to enable, and it was reversed to option B (kept-unmatched primary tracks count as matched: "keep = match what's already there"). Both variants were live-verified against mkvmerge v100.
- **Steelman:** null (the reversed initial assumption was mkvmerge's literal native behavior and the simplest rendering).
- **Occurrences:** 2026-07-09 decided (memo D20 / Plan 3 Task 3, assumption A); 2026-07-09 violated-corrected (whole-branch verdict Important #1, donor-first trap); 2026-07-09 decided (journal + memo D20-B, reversal, commit c1d5614); 2026-07-09 reinforced (task-7 verdict, option B built + verified, commits 51567d7/aa75025).

### core-81-unsupported-source-gate - one clean error before rule resolution
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** A source mkvmerge identifies (exit 0) but cannot mux emits one clear `UnsupportedSource` error (distinct code from `UnidentifiableSource`, different remediation) before rule resolution and skips the file, instead of per-rule `MissingTrack` noise. The gate fires on `!container_recognized || !container_supported` only, not `is_identifiable()`; a recognized+supported zero-track container stays a per-rule `MissingTrack`.
- **Steelman:** null.
- **Occurrences:** 2026-07-09 decided (memo D21); 2026-07-09 reinforced (task-6 verdict, three corner tests); 2026-07-09 reinforced (whole-branch verdict Strengths).

### core-85-report-json-dry - hoist report documents into core
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The batch/config/run JSON report documents (spec 7) are hoisted from the CLI into `core::report::json`, lifted 1:1, so CLI and GUI render byte-identical report structures from a single core module; neither surface owns document logic.
- **Steelman:** null.
- **Occurrences:** 2026-07-10 decided (Plan T2); 2026-07-10 decided (spec §7); 2026-07-10 decided (task-2 verdict, char-identical claim).

### core-87-version-floor - MIN_SUPPORTED (86,0), empirically derived
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The mkvmerge version floor `MIN_SUPPORTED = (86,0)` was derived empirically from the identification-schema v19->v20 diff traced to NEWS.md v86.0, with the evidence in the const doc; independently re-derived by the reviewer.
- **Steelman:** null.
- **Occurrences:** 2026-07-10 decided (Plan T3 step 1); 2026-07-10 decided (task-3 verdict, inductive step confirmed); 2026-07-10 decided (commit c7ef52a).

### core-89-homebrew-apple-silicon-path - added on the right authority
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `/opt/homebrew/bin/mkvmerge` (Apple-Silicon Homebrew) is a detection candidate: a Finder-launched Tauri app does not inherit the shell PATH, so without it detection fails for the most common macOS install route. It was first excluded under the SI-3 evidence rule (no formula in the mkvtoolnix source tree), then added when the exclusion was found to rest on the wrong authority.
- **Steelman:** null (the exclusion was a defensible read of "cite paths against mkvtoolnix's own packaging" until the Finder-PATH fact overrode it).
- **Occurrences:** 2026-07-10 deferred (task-3 verdict Minor, product-scoping question to T7); 2026-07-10 violated-corrected (whole-branch verdict triage-14, FIX-NOW); 2026-07-10 violated-corrected (commit 5e76a15).

### core-99-schema-drift-advisory - once-per-batch SchemaDrift, rebuilt after drop
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** A newer-schema advisory is its own `SchemaDrift` diagnostic, once per batch (info severity), message carrying the `raw:` discovery hint. T16 removed the mis-scoped per-file skew warner (its design-round "dead code" premise was false) and raised whether any advisory should survive; Şenol ruled it should, rebuilt once-per-batch at T16.5 (single emission site outside the loop, `found_version(max)` + `pinned` params, Option-guarded).
- **Steelman:** null.
- **Occurrences:** 2026-07-11 deferred (task-16 verdict, per-file warner dropped, open question); 2026-07-12 decided (memo D32 addendum RESOLVED / plan T16.5); 2026-07-12 reinforced (task-16.5 verdict, shipped).

### core-102-overlap-selection-rejected - no precedence-based rule selection
- **kind:** restraint | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Neither "later-by-list-order" (narrow `rules[1]`) nor "broader-by-match-domain-size" was adopted as the policy for which overlap rule gets narrowed; Muxsmith's resolution model has no rule precedence, both are intent guesses, and the earlier rule is frequently the accidental over-broad one. Both survive only as a rank tiebreak (core-103).
- **Steelman:** Policy 1 is cheapest and fully deterministic and matches a "first rule wins, later is the intruder" model; policy 2 is the strongest intent heuristic (the rule matching more tracks is the accidental over-claimer). Rejected as primary policies in favor of symmetric generation + acceptance filter.
- **Occurrences:** 2026-07-11 decided (memo D33 / d33-analysis, policy 1); 2026-07-11 decided (memo D33 / d33-analysis, policy 2).

### core-119-invalidpropertyvalue-allowed - single-site divergence caught
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** `planner.rs` emitted `InvalidPropertyValue` without the template-required `$allowed` on the `changes.language` path, rendering a literal `{$allowed}` to users; found by the T10 fixture-fidelity spot-check (per-DiagCode fixtures are structurally blind to single-site divergence), routed to T9(ix), fixed with a site-specific `no-{$` regression test.
- **Steelman:** null.
- **Occurrences:** 2026-07-11 violated-corrected (task-10 verdict, fixture-fidelity spot-check); 2026-07-11 violated-corrected (task-9 verdict (ix)); 2026-07-11 violated-corrected (plan T9(ix)).

### core-121-planner-seam-and-hoist - shared plan_pipeline deferred to Plan 6
- **kind:** non-decision | **status:** blocked | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** The four-copy planning pipeline (~100 lines across cli `dry_run.rs`/`run.rs` and src-tauri `lib.rs`/`run.rs`) and the never-decided injectable-planner-seam interface (S4/S5/S6) are one question: a shared core `plan_pipeline()` IS the seam. Left undecided whether to do the hoist as an idiomacy-fix wave now or fold it into the Plan 6 profile-editor design.
- **Blocked on:** internal - Şenol's call (idiomacy-fix wave vs Plan 6); reactivate when Plan 6 brainstorming starts.
- **Occurrences:** 2026-07-12 deferred (CONVENTIONS.md Non-decisions, seam interface); 2026-07-12 deferred (ROADMAP L37); 2026-07-12 deferred (idiomacy-review-findings, four-copy hoist).

---

## Patterns (settled, count < 3)

- **core-08-runtime-skew-untyped** [pattern·settled·2] E0 sketch: a runtime property the pinned model lacks is matched untyped and warned about rather than hard-failing; later reworked as D32 `raw:` opt-in (core-91). Occ: 2026-07-08 decided (journal Plan 1); 2026-07-08 decided (spec §9).
- **core-09-media-boundary** [pattern·settled·1] Muxsmith never processes media; it only generates and executes mkvmerge commands, like mkvtoolnix-gui's job queue; output always MKV, input anything the local mkvmerge supports. Occ: 2026-07-08 decided (spec §1).
- **core-10-declarative-batch** [pattern·settled·1] The profile expresses the output as declarative selection rules resolved against each file's actual tracks, applied across a whole tree with no per-file review step. Occ: 2026-07-08 decided (spec §1).
- **core-11-profile-format** [pattern·settled·1] One serde model backs YAML and JSON equivalently; the published JSON Schema is generated from the same model via schemars, never hand-written. Occ: 2026-07-08 decided (spec §4).
- **core-13-property-model-split** [pattern·settled·1] Two disjoint property sets in capability: matchable (generated from the identification schema at build time) vs settable (curated table mapped to mkvmerge options). Occ: 2026-07-08 decided (spec §4.4).
- **core-14-operation-unification** [pattern·settled·1] validate (static only), dry-run (validate + full planning, `-J` only) and run (re-plan then execute) share one planning code path; one code path, three entry points. Occ: 2026-07-08 decided (spec §5.5).
- **core-23-output-naming** [pattern·settled·2] Output filename is keep-name or a rename template; `on_collision` = error|skip|overwrite (default error); in-place replacement of sources is a hard, non-configurable exclusion. Occ: 2026-07-08 decided (spec §2/§4.8/§1); 2026-07-08 decided (commit 61249f9).
- **core-25-match-to-source-false** [pattern·settled·2] `match_to_source: false` must not raise `LocatorConflict`; validate.rs used `is_some()` but the spec types it `?: true`, so conflict only on `Some(true)` and reject `Some(false)` explicitly. Occ: 2026-07-08 violated-corrected (whole-branch verdict Important #3); 2026-07-08 violated-corrected (commit 3c24845).
- **core-28-closed-domain-value-validation** [pattern·settled·1] New `InvalidPropertyValue` diagnostic: validate exact-condition values against closed domains - `type` (pinned schema, config-time), `codec_kind` (curated table, config-time), `language` (mkvmerge `--list-languages`, plan-time once per config path); a typo'd enum value must be a config-time error, not surface as a misleading `MissingTrack` on every file. *Accepted tradeoff:* a new enum value from a newer mkvmerge is rejected at validate with no v1 escape hatch - fine because track-type enums are extremely stable. Occ: 2026-07-09 decided (memo D2).
- **core-30-locator-path-pathbuf** [pattern·settled·2] `Locator.path` changes String -> PathBuf to match `output.directory`; the honest type, natural planner joins, serde/schemars treat it as string so the profile format and JSON Schema are unchanged. Occ: 2026-07-09 decided (memo D3); 2026-07-09 decided (commit 339c99d).
- **core-32-template-brace-semantics** [pattern·settled·1] Lock current template behavior with golden tests, no code change: `{{`/`}}` escape, lone `}` literal, lone unclosed `{` is `InvalidTemplate` (forgiving where unambiguous, strict where ambiguous). Occ: 2026-07-09 decided (memo D5).
- **core-36-empirical-verification** [pattern·settled·1] Confirm the `-J` shape, `--list-languages`/`--list-types` formats and the track-type domain by running the installed mkvmerge (v99), not from memory or the schema; both parsers passed real output first try (continues Plan-1 discipline). Occ: 2026-07-09 reinforced (journal Plan 2). *Note: the same empirical-grounding discipline recurs inside core-51 (identify), core-57 (argv), core-87 (version floor); counted there, not double-counted here.*
- **core-39-empty-matchlist-error** [pattern·settled·2] A present-but-empty `any:[]` or `not:[]` is a config-time error (new `EmptyMatchList`), not silently no-constraint ("at least one of zero holds = false"). Occ: 2026-07-09 decided (independent review decision #2(c)); 2026-07-09 decided (commit a67879e).
- **core-43-keep-name-mkv-preserved** [pattern·settled·2] The keep-name arm appends `.mkv` unconditionally per-arm (not the unified conditional append), so `Show.S01E01.mkv.mkv` keeps its full basename; unifying the arms silently truncated inner-`.mkv` keep names with no diagnostic (spec 4.8 violation). Occ: 2026-07-09 violated-corrected (F6 review Finding 1); 2026-07-09 violated-corrected (commit 550ba59).
- **core-50-command-purity-ir** [pattern·settled·2] `command` is a pure `Plan -> Vec<String>` with no Profile access; the Plan/Assignment carry every resolved decision format-neutrally, so `command` is the only module below capability that knows mkvmerge's CLI surface - a future backend or flag change touches `command` alone. Occ: 2026-07-09 decided (memo D8); 2026-07-09 reinforced (whole-branch verdict Strengths).
- **core-51-identify-attachments-chapters** [pattern·settled·2] `Identification` gains attachments (id/file_name/size/content_type?/description?/uid?) and a summed chapters entry-count, parsed from the `-J` already fetched (no extra spawn); fields pinned to schema v20 and confirmed against the binary, `parse_attachment` mirroring `parse_track`. Occ: 2026-07-09 decided (memo D9); 2026-07-09 reinforced (task-1 verdict, Approved).
- **core-52-chapters-count-only** [pattern·settled·1] Chapters are parsed as a total entry count; only presence (count>0) is consumed for keep/drop in v1, the count parsed for future use - intentional per D9, not a wiring gap. Occ: 2026-07-09 reinforced (whole-branch verdict Minor 8).
- **core-53-attachments-primary-only** [pattern·settled·1] Attachment rules and unmatched apply to the primary's attachments only; donors always get `--no-attachments`; external files enter as attachments only via an explicit `add` locator. A donor is a track source, not an attachment source (least-surprise); the one point spec 4.9 left open, flagged for Şenol at the review gate. Occ: 2026-07-09 decided (memo D10).
- **core-55-settable-language-per-file** [pattern·settled·2] `changes.language` values are validated per-file at the matched-branch application point (reuse `LanguageIndex` + `InvalidPropertyValue`), not folded into the batch-level walk; an invalid settable language on an optional rule that matched nothing goes uncaught but is inert. Occ: 2026-07-09 decided (Plan Task 5); 2026-07-09 decided (memo D10).
- **core-57-argv-construction** [pattern·settled·2] Global options + per-input-file groups (one per distinct input path) with track selection/property/attachment flags + `--track-order` encoding `fileidx:trackid`, mirroring mkvtoolnix-gui; grouping + `--track-order` is the only way to express cross-file ordering; argv accepted as-is by real mkvmerge v100, no command.rs correction. Occ: 2026-07-09 decided (memo D11); 2026-07-09 reinforced (task-12 verdict, live round trip).
- **core-60-title-ctx-source-stem** [pattern·settled·1] `resolve_title` adds `source_stem` to the render `Ctx` so its field set matches validate.rs's `title.template` allowed fields, closing a validated-but-silently-empty-field bug the brief's literal wording would have produced. Occ: 2026-07-09 decided (task-6 verdict).
- **core-62-external-chapters-reuse** [pattern·settled·1] `resolve_chapters` mirrors the track-external branch, reuses `resolve_locator` + Missing/AmbiguousExternal, diverges only where required (no optional escape on 0 hits, no `identify()` on the 1-hit path), and does not mkvmerge-identify the chapters file. Occ: 2026-07-09 reinforced (task-7 verdict).
- **core-68-bcp47-accept-well-formedness** [pattern·settled·1] A language value is valid if it is a bare ISO code OR a well-formed BCP-47 tag (`parse()` only), not ISO-index membership, so real regional tags (`xx-YY`) are accepted and left for mkvmerge to reject at mux time. Occ: 2026-07-09 decided (memo D19).
- **core-69-bcp47-accept-liberal-match-precise** [pattern·settled·2] Deliberate asymmetry: acceptance uses `parse()` well-formedness (liberal), matching uses `canonicalize()` (precise); a well-formed-but-nonexistent tag is accepted and simply never matches. Occ: 2026-07-09 decided (memo D19); 2026-07-09 reinforced (whole-branch verdict, pt-BR trace).
- **core-71-language-tags-crate** [pattern·settled·2] Use the vetted `language-tags` crate (pure Rust, RFC 5646, bundled IANA registry) for BCP-47 parse/canonicalize rather than a hand-rolled grammar; 0.3.2 vetted clean (MIT/Apache-2.0, no transitive deps, deny.toml untouched). Occ: 2026-07-09 decided (memo D19, Şenol); 2026-07-09 reinforced (task-4 verdict).
- **core-73-language-canonicalize-match** [pattern·settled·2] Match equality canonicalizes both non-ISO operands (case + script suppression + deprecated replacement) so `pt-Latn-BR==pt-BR`, `iw==he` while `pt-BR!=pt-PT`, `zh-Hans!=zh-Hant`; built now (option B) rather than cut as an apparent no-op, because the cited pt-Latn-BR fragility genuinely needs it. Occ: 2026-07-09 decided (memo D19); 2026-07-09 decided (journal Plan 3.5).
- **core-74-tracks-unmatched-policy** [pattern·settled·2] `tracks` gains an `unmatched` policy keep|drop defaulting to drop - drop is Muxsmith's declarative/curative essence; keep retained after Şenol questioned it, justified by the one inexpressible case (additive bulk ops under drop are blocked by one-track-per-rule). Occ: 2026-07-09 decided (memo D20); 2026-07-09 reinforced (journal Plan 3.5).
- **core-75-tracks-nested-block** [pattern·settled·2] `tracks` is restructured into a `{ unmatched, rules }` block (Şenol drove placement) so the policy lives with its rules, matching attachments and output/tags policy-in-section; Peter's top-level `unmatched_tracks` ergonomic recommendation conceded on whole-profile consistency. Occ: 2026-07-09 decided (memo D20); 2026-07-09 decided (journal Plan 3.5).
- **core-76-trackscfg-no-default** [pattern·settled·2] Only `unmatched` defaults (via `drop_policy()`); `rules` has no serde default, so `tracks:` stays a mandatory Profile field - exactly preserving the pre-existing requirement that a list be present. Occ: 2026-07-09 decided (task-1 verdict); 2026-07-09 decided (Plan Task 1 Step 3).
- **core-77-keep-scope-primary-only** [pattern·settled·1] The `keep` policy passes through only the primary's own unmatched tracks; donors and attachments unaffected (keeping unmatched donor tracks would pull in the donor's video/subs); mirrors D10's primary-only attachment scope. Occ: 2026-07-09 decided (memo D20).
- **core-78-keep-donor-isolation** [pattern·settled·1] Donor isolation under keep is structural, not conventional: `input_groups` always seeds group 0 with the primary, so the `source==plan.source` suppression guard can never fire for a donor. Occ: 2026-07-09 reinforced (task-2 verdict, Named Risk Check).
- **core-79-keep-no-reposition** [pattern·settled·1] Under keep, explicit primary rules change properties only, not position; to reorder the primary the user switches to drop mode (held open for veto, not vetoed). Occ: 2026-07-09 decided (memo D20).
- **core-86-mkvmerge-detection-ladder** [pattern·settled·2] `detect_mkvmerge` probes override -> PATH -> platform-standard locations; on failure, per-OS first-run guidance + manual picker + version floor. A configured override is authoritative (early return), never masked by an automatic fallthrough to PATH. Occ: 2026-07-10 decided (memo D28); 2026-07-10 decided (task-3 verdict, override-authoritative).
- **core-88-platform-candidate-paths** [pattern·settled·2] Each Windows/macOS/Linux candidate path is verified against mkvtoolnix's own packaging and cited (correcting the brief's macOS bundle-name glob to `MKVToolNix.app`), unused ones dropped. Occ: 2026-07-10 decided (Plan T3 step 2); 2026-07-10 decided (task-3 verdict).
- **core-90-go-public-gates** [pattern·settled·2] Pulled-forward go-public gate: `ConcurrencyTracker` (test instrumentation) is `doc(hidden)` from rustdoc but kept `pub` for cross-crate tests; static 3-OS matrix pulled forward too. Occ: 2026-07-10 decided (commit 7a2bc15); 2026-07-10 decided (journal Plan 5).
- **core-91-raw-opt-in** [pattern·settled·2] Unknown property names stay hard-rejected at config time UNLESS marked with a `raw:` prefix; a `raw:` property matches untyped, emits `RawProperty` info at validate and `UnknownPropertySkew` at plan time (shape B). Shipped exactly per the binding B-1..B-11 acceptance table. Occ: 2026-07-11 decided (memo D32, Şenol); 2026-07-11 reinforced (task-16 verdict).
- **core-95-rawproperty-visible** [pattern·settled·1] `RawProperty` (info, config-time) is kept rather than the minimal variant that drops it - the escape valve must be visible at validate; a silent bypass undercuts the point of B over C. Occ: 2026-07-11 decided (memo D32, marked assumption).
- **core-97-raw-on-known-property** [pattern·settled·1] `raw:` applied to a property with special semantics (language, codec_kind) emits `RawOnKnownProperty` (warning): it bypasses ISO-639/BCP-47 normalization / alias expansion to byte-literal equality. Occ: 2026-07-11 decided (memo D32, B-4).
- **core-98-raw-language-single-field** [pattern·settled·2] `raw:X` reads exactly the property literally named `X`, byte-exact (`raw:language` reads only `language`, so `de` vs `ger` = no match); no language exception. Read as dual-field in the memo, ratified single-field by Şenol, no code change. Occ: 2026-07-11 deferred (task-16 verdict, pending ratification); 2026-07-12 decided (memo D32 addendum RESOLVED, Şenol).
- **core-103-overlap-rank-tiebreak** [pattern·settled·1] Ties between surviving narrowings on different rules break broader-rule-first, then lower rule index (the rejected selection policies survive here as a deterministic ordering). Occ: 2026-07-11 decided (memo D33, marked assumption).
- **core-104-overlap-seed-not-only** [pattern·settled·2] Seed overlap narrowing candidates as NOT-polarity only from the shared track's property vector, dropping positive AddExact/AddSubstring on the overlap path - the NOT form is the honest "this rule should not take that track", and monotone narrowing guarantees convergence; a deliberate departure from the ambiguous path. Occ: 2026-07-11 decided (memo D33); 2026-07-11 decided (d33-analysis).
- **core-107-overlap-reuse-no-new-mechanism** [pattern·settled·1] Overlap suggestions reuse `candidates_for_rule` / `resolves_without_regression` / `partition_for_rule` with no new edit variant, selection mechanism, or DiagCode. Occ: 2026-07-11 decided (memo D33).
- **core-108-narrow-optional-to-empty** [pattern·settled·1] Narrowing an optional rule to an empty match is a first-class suggestion (the only feasible fix, user sees the exact edit), not down-ranked or annotated as "disables the rule here". Occ: 2026-07-11 decided (memo D33, marked assumptions).
- **core-110-three-claimants-partition** [pattern·settled·2] Three-or-more claimants cannot be resolved by a single narrowing and degrade to the partition/no-fix report, which must name ALL claimants (ties to ROADMAP polish item i). Occ: 2026-07-11 decided (memo D33 technically-forced list); 2026-07-11 decided (d33-analysis).
- **core-111-overlap-alpha-tiebreak** [pattern·settled·2] Within equal rank on the same rule, overlap suggestions order alphabetically on the property name - a deliberate deterministic tiebreak recorded at the T18 review. Occ: 2026-07-11 decided (memo D33 TC-A); 2026-07-11 decided (task-18 verdict adjudication 1).
- **core-113-d6-mechanical-completion** [pattern·settled·2] D6 completed: external-source suggestions (source-agnostic, skip removed), codec/id narrowing dimensions, multiset `diag_signature` (`BTreeMap<sig,count>`), and a §5.3 `SuggestionPartition` report capped at 5 groups. Occ: 2026-07-11 decided (task-13 verdict); 2026-07-11 decided (Plan T13).
- **core-114-partition-suggestion-keyed** [pattern·settled·2] Partition grouping is keyed on the per-file suggestion (the more literal reading of spec §5.3, which mandates the outcome not a key), superseding D6's conflict-signature-multiset key which could over-report by splitting files that share a resolution. Occ: 2026-07-11 decided (task-13 verdict ratification 1); 2026-07-11 decided (memo residues). *Steelman (rejected D6 property-vector key):* D6's own Algorithm-doc §5.3:286 specified the conflict-signature multiset as the key.
- **core-116-known-extensions-delivery** [pattern·settled·1] `known_extensions` is delivered as an `Identify` trait default method + `IdentifyCache` memo, not a `plan_core` param - zero production edits (a param would break ~20 call sites and collide with parallel streams). Occ: 2026-07-11 decided (task-5 verdict).
- **core-118-name-all-claimants** [pattern·settled·2] `OverlappingRules` names all claimants via a rendered `$rules` list (deterministic, `BTreeMap` + ascending rule index) rather than only `rules[0]`/`rules[1]` - feeds D33's all-claimants requirement. Occ: 2026-07-11 decided (task-9 verdict (i)); 2026-07-11 decided (Plan T9).
- **core-120-donor-named-unsupported** [pattern·settled·2] Donor-side `UnsupportedSource` names the offending donor via a new `donor` param and a single-key `$kind` selector (mirroring `DonorIsPrimary`'s `$donor`), landing before T21 translation; primary rendering byte-identical. Occ: 2026-07-11 decided (task-9.5 verdict); 2026-07-11 decided (Plan T9.5).

## Restraints (rejected; steelman kept)

- **core-02-ordered-consumption-rejected** [restraint·settled·1] Ordered consumption (first-match-wins with track consumption) and a global constraint solver were both rejected for strict independent uniqueness. *Steelman:* ordered consumption would make Şenol's own reference profile work unmodified (no explicit `forced_track:false`), more ergonomic; a solver could auto-resolve overlaps. Rejected for maximal explicitness over implicitness. Occ: 2026-07-08 decided (journal Plan 1).
- **core-05-global-toggles-rejected** [restraint·settled·1] Exposing attachments/chapters/tags/title as simple global on/off toggles instead of full declarative control was rejected. *Steelman:* far less config/UI surface and a simpler v1 (the agent recommended it). Overruled as Şenol's scope call. Occ: 2026-07-08 decided (journal Plan 1).
- **core-07-runtime-fetching-rejected** [restraint·settled·2] Fetching the identification schema at runtime or over the network at build was rejected. *Steelman:* keeps the property model in lockstep with the local mkvmerge, eliminating skew. Rejected to sidestep schema licensing and avoid a network build dependency; skew is surfaced as an opt-in untyped-match warning instead. Occ: 2026-07-08 decided (spec §2/§9); 2026-07-08 decided (journal Plan 1).
- **core-15-transcoding-out** [restraint·settled·1] Transcoding is permanently out of scope; the tool only muxes. *Steelman:* a convert-and-remux step would serve users whose source codecs the target player cannot handle - but that is a fundamentally different tool. Occ: 2026-07-08 decided (spec §11).
- **core-16-season-episode-arithmetic-out** [restraint·settled·1] Cross-file season/episode renumbering/offsetting is excluded; identifiers are opaque match keys and transforms are per-file. *Steelman:* cross-batch renumbering is a real bulk-rename need mkvtoolnix users hit. Occ: 2026-07-08 decided (spec §11).
- **core-17-wildcard-multitrack-out** [restraint·settled·1] "Keep all remaining audio" wildcard rules are rejected for v1 (they break strict independent uniqueness); a later explicit `multi: true` form is named but not committed. *Steelman:* "keep all remaining tracks of type X" is a common mkvtoolnix convenience; strict uniqueness forces enumerating every track. Occ: 2026-07-08 decided (spec §11).
- **core-18-blanket-private-doc-lint-rejected** [restraint·settled·2] Public items are doc-gated by `deny(missing_docs)`; private items are documented by judgment only, a blanket private-doc lint rejected as comment-noise. *Steelman:* a blanket requirement guarantees every item is documented with no judgment calls. Occ: 2026-07-08 decided (journal Docs bullet); 2026-07-08 decided (commit c402914).
- **core-27-codec-kind-exact-only** [restraint·settled·1] `codec_kind` may appear only under exact conditions; a pattern over the curated alias token is ill-defined (against the token? the prefix set? the codec_id?), so it is a validate-time error (new `CodecKindExactOnly`), not a silent never-match. *Steelman:* allow substring/regex over `codec_kind` for loose family matching - rejected because real pattern matching is already available via `codec_id`. Occ: 2026-07-09 decided (memo D1).
- **core-33-suggestion-narrow-only** [restraint·settled·1] The suggestion edit grammar is a deliberately closed `StructuredEdit` enum (v1): suggestions only ever narrow the conflicted rule's match, never reorder rules, never touch other rules, never relax; narrow-only guarantees convergence (iterated apply terminates) and keeps YAML-fragment rendering and simulation trivial. *Steelman:* a richer grammar (reorder rules, relax over-narrow rules) would auto-resolve more conflict classes - deferred out of v1 for determinism and convergence. Occ: 2026-07-09 decided (memo D6).
- **core-61-primary-ctx-helper-declined** [restraint·settled·1] Declined to extract a shared `primary_ctx()` helper for a ~4-line, two-instance duplication between `render_output` and `resolve_title` (does not clear the codebase's "three similar lines beat a premature abstraction" bar). *Steelman:* extracting it would prevent the two Ctx-building sites drifting apart. Occ: 2026-07-09 decided (task-6 verdict Strengths).
- **core-70-registry-validate-not-called** [restraint·settled·2] `registry.validate()` is deliberately not called on the language accept side; over-accepting a pathological well-formed tag is less harmful than over-rejecting real ones, and mkvmerge rejects it at mux time. *Steelman:* calling it would catch typos like `xx-YY` at plan time rather than deferring to mkvmerge. Occ: 2026-07-09 decided (memo D19); 2026-07-09 decided (Plan self-review).
- **core-82-donor-side-gate-not-built** [restraint·settled·2] The `UnsupportedSource` gate is deliberately primary-only; no donor-side gate was built (D21 scope). *Steelman:* a donor mkvmerge cannot mux also fails downstream; a symmetric donor gate would surface it earlier with the same clean diagnostic. Occ: 2026-07-09 decided (Plan self-review); 2026-07-09 decided (task-6 verdict).
- **core-92-raw-shape-a-rejected** [restraint·settled·2] Shape A (enrich `UnknownProperty` with a nearest-name did-you-mean suggestion, drop the §9.2 automatic-untyped promise) rejected because the §9.2 forward-compat promise is kept deliberately. *Steelman:* maximal typo protection; the cleaner downgrade if forward-compat-without-a-release is YAGNI for this audience. Occ: 2026-07-11 decided (memo D32); 2026-07-11 decided (d32-analysis).
- **core-93-raw-shape-c-rejected** [restraint·settled·1] Shape C (unknown name becomes a warning at every level + untyped match, no opt-in) rejected outright - guts typo protection unconditionally, including on non-skewed mkvmerge, and opens the optional-rule and `not:`-inversion silent holes. *Steelman:* full and automatic forward-compat, the cheapest literally-§9.2-compliant path. Occ: 2026-07-11 decided (memo D32).
- **core-94-raw-version-gated-rejected** [restraint·settled·1] The obvious version-gated hybrid (error at validate, warning at dry-run) rejected pre-emptively - it inverts spec 5.5's "dry-run is a strict superset of validate", so the disambiguator must be stable across levels. *Steelman:* matches the actual information available per level (no mkvmerge at validate, `format_version` readable at dry-run). Occ: 2026-07-11 decided (d32-analysis).
- **core-96-rawproperty-minimal-rejected** [restraint·settled·1] The minimal variant that omits `RawProperty` and relies only on plan-time `UnknownPropertySkew` rejected - it trades validate-time honesty for fewer codes. *Steelman:* two fewer DiagCodes and two fewer C1 fixtures to maintain. Occ: 2026-07-11 decided (d32-analysis).
- **core-100-schemadrift-primaries-only** [restraint·settled·1] `SchemaDrift` is emitted from primaries only; extending it to donors rejected as over-engineering for a structurally unreachable case (schema version is a build constant of the one local mkvmerge binary). *Steelman:* a donor on a newer schema could carry undocumented skew. Occ: 2026-07-12 decided (task-16.5 verdict adjudication 1).
- **core-105-overlap-seed-positive-rejected** [restraint·settled·1] Mirroring the ambiguous path's both-polarity generation on the overlap path rejected - positive edits cannot redirect and only add near-tie noise to the ranked list. *Steelman:* uniformity with the ambiguous path; harmless since acceptance filters it. Occ: 2026-07-11 decided (d33-analysis open taste call 3).
- **core-106-overlap-specific-dimension-rejected** [restraint·settled·1] A separate overlap-specific dimension (diff the two rules' matched track sets) rejected - on the single contested file the two rules match the identical track so the difference set is empty; reuse `candidates_for_rule` instead. *Steelman:* the properties on which the two matched sets differ would be the natural discriminator in a multi-file batch. Occ: 2026-07-11 decided (d33-analysis).
- **core-109-two-required-no-fix** [restraint·settled·1] Two required rules colliding on one track in one file yield no suggestion, only the explicit no-fix/partition report - an unavoidable property of the narrow-only v1 grammar. *Steelman:* a user expecting a one-click fix for every overlap will find overlap less helpful than ambiguity. Occ: 2026-07-11 decided (memo D33 Tradeoff, accepted).
- **core-112-contested-property-dimension-rejected** [restraint·settled·1] A contested-property selection dimension (to honor the memo's `default_track`-vs-`forced_track` TC-A illustration) rejected - it would be a new selection mechanism D33 forbids. *Steelman:* honor the memo's illustration where an equal-rank track's `forced_track` should rank first. Occ: 2026-07-11 decided (task-18 verdict adjudication 1).

## Non-decisions (deferred; blocked_on kept)

- **core-19-sync-delay-stretch** [non-decision·blocked·1] Per-file `--sync` delay/stretch changes deferred; per-file offsets do not generalize to batch rules (v1.x candidate). Blocked on: no batch-generalizable model for per-file offsets. Occ: 2026-07-08 deferred (spec §11).
- **core-20-ondisk-cache** [non-decision·blocked·2] The identification cache is in-memory per session (keyed path+mtime+size); an on-disk cache is a future candidate, not built in v1. Blocked on: in-memory cache sufficient for v1, no measured need. Occ: 2026-07-08 deferred (spec §5.5/§11); 2026-07-08 deferred (handoff plan-1-close).
- **core-56-batch-level-language-fold** [non-decision·blocked·1] Optionally folding settable-language validation into `walk_exact_languages` (to catch the all-optional-unmatched invalid-language case) left for v1 - low value since the uncaught case is inert. Blocked on: optional refinement, not required for v1 correctness. Occ: 2026-07-09 deferred (whole-branch verdict Recommendations + progress roll-up).
- **core-64-donor-group-ordering** [non-decision·blocked·1] With a donor whose assignments mix None/Some, its group position is set by its first Some (can differ from a literal first-appearance reading); no golden arbitrates it, left as a conscious choice into Task 12. Blocked on: no golden pins the ordering. Occ: 2026-07-09 deferred (task-9 verdict round-2 Minor + progress roll-up).
- **core-66-eager-resolution-discarded-plan** [non-decision·blocked·1] On a `render_output` failure, `resolve_chapters`/`resolve_attachments` still run (wasted I/O + stray diagnostics on plans that will be discarded); consistent with the deliberate pre-finalize design (`detect_source_overwrites` needs pre-drop plans), left as-is. Blocked on: tied to `detect_source_overwrites` pre-finalize design, low priority. Occ: 2026-07-09 deferred (whole-branch verdict Minor 6 + progress roll-up).
- **core-83-zero-rule-keep-passthrough** [non-decision·blocked·1] Whether a zero-rule `keep` profile should be a legal pure-passthrough remux is open; the assumption stands that it remains a `NoTrackRules` error. Blocked on: Şenol veto. Occ: 2026-07-09 deferred (memo D20, Open mechanics).
- **core-84-regex-recompile** [non-decision·blocked·1] The regex recompiled per `matches` call is deferred to a later cleanup pass. Blocked on: later cleanup pass. Occ: 2026-07-10 deferred (memo D18).
- **core-115-partition-unresolvable-file** [non-decision·blocked·2] A fixless affected file is silently dropped from the partition (`best=None` skip, unreachable under v1 id-uniqueness); hardening with an invariant comment or an explicit "unresolvable" group deferred. Blocked on: idiomacy review (internal). Occ: 2026-07-11 deferred (task-13 verdict m1); 2026-07-11 deferred (whole-branch funnel T13-m1).
- **core-117-known-extensions-make-required** [non-decision·blocked·1] `known_extensions` default-None on `Identify` inverts the idiom (sole production impl overrides; a future impl silently gets vacuous validation); making the method required deferred. Blocked on: idiomacy review (internal). Occ: 2026-07-11 deferred (task-5 verdict m2 + progress T5-m2).

---

## Clustering notes (defensibility)

- **E0/E1 collapse, not double-count.** The Plan-1 foundational patterns (core-01, -03, -04, -06, -12) reach count 3-4 from *distinct artifacts* (spec, journal, one or more implementation commits, handoff), exactly as sibling `cross-01` did - not from counting the E0 record and the E1 record as two occurrences of the same spec+journal citation. Where E0 and E1 cite the same spec section and the same journal entry, that is one spec occurrence and one journal occurrence.
- **Find-vs-fix are two events.** The many Plan-2/Plan-3 correctness clusters (core-31, -34, -38, -40, -44, -45, -46, -47, -63) promote because a bug was independently *found* (independent review / whole-branch / task verdict), its *fix reviewed* (Fn-review), and *committed* - three distinct attestation points for the same `(topic, approach)`. A review's verdict file plus a journal/progress mention of that same review collapse to one; the fix commit does not.
- **Deferral-then-resolution stays one cluster.** core-35 (overlap suggestions), core-41 (SourceOverwrite), core-48 (extension validation) and core-65 (EmptyPlan) each carry an early `deferred` occurrence and later `decided`/`reinforced` occurrences of the *same* topic+approach; the cluster kind is the resolved state (pattern), `blocked_on` is null. Only clusters that never resolved are non-decisions with a live `blocked_on`.
- **Restraint families kept separate from their principle.** The four "no magic guesses" rejections (filename-language, filename-flags, unique-suffix, sequence-append) are merged into one restraint (core-67) because CONVENTIONS.md itself bundles them as one principle over IDEAS.md #1-4; but the D33 overlap-design rejections (core-102, -105, -106, -109, -112) are kept as separate restraints because each rejects a distinct alternative mechanism with its own steelman.
- **No count was inflated by lumping approaches.** The suggestion engine spans several clusters (verified-edit core-03, narrow-only core-33, cap core-34, no-clobber core-44, yaml core-45, D6 completion core-113, partition key core-114) rather than one fat "suggestion engine" count, because each is a distinct `(topic, approach)`. core-37 (prose-free core, count 7) is the one large cluster and is genuine: the same core-emits-code-not-prose boundary is re-attested across Plans 2, 3, 5 and the E8 convention.
- **Dates are era/ref-reconstructed, not git-verified this pass** (unlike the `cross` file). The relative ordering is reliable from the ref (spec -> journal -> commit -> review -> later-plan); absolute dates within a plan are the plan's session date.
