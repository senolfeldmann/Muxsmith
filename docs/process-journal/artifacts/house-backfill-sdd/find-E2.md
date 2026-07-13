# Muxsmith era E2 (2026-07-09) — reconstructed decision history

Scope: everything on 2026-07-09 — the D1-D6 design memo, Plan 2 authoring, Plan 2
INLINE execution, the retrofit independent review (~11 bugs), Şenol's three
triage decisions (#1/#2/#3), and the SDD corrective fix pass (F1-F9 + I1).

Sources cited per record. Commits: memo `3b71a71`, spec fold-in `d4390d7`, Plan 2
impl `05899c3..e1bfba7`, review archive `847b476`, fix pass `d9422b3..59d24c8`.

One record per occurrence. `RC` = record id (local, for reading only).

---

## A. Design memo D1-D6 (2026-07-09, `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`)

### RC1 — D1 codec_kind exact-only [restraint / core / decided]
- topic: codec_kind pattern-matching scope
- approach: Restrict codec_kind to `exact` conditions only; reject it under substring/regex with a new config-time `CodecKindExactOnly` diagnostic.
- statement: codec_kind is a curated alias over codec_id prefix sets; a pattern over the alias token is ill-defined (against the token? the prefix set? the codec_id?), so it is a validate-time error, not a silent never-match.
- steelman: allow substring/regex over codec_kind so codec families can be matched loosely — rejected because real pattern matching is already available via `codec_id`, and alias-token semantics are ambiguous.
- occ_ref: memo D1. evidence: "codec_kind may appear only under exact ... a pattern over the alias token is ill-defined ... new diagnostic CodecKindExactOnly."

### RC2 — D2 closed-domain value validation [pattern / core / decided]
- topic: closed-domain value validation (type / codec_kind / language)
- approach: New `InvalidPropertyValue` diagnostic; validate `exact`-condition values against closed domains — `type` (pinned schema, config-time), `codec_kind` (curated alias table, config-time), `language` (mkvmerge --list-languages, plan-time once per config path).
- statement: strict-explicitness doctrine — a typo'd enum value must be a config-time error, not surface as a misleading MissingTrack on every file.
- steelman: version skew — a genuinely new enum value from a newer mkvmerge is rejected at validate with no v1 escape hatch; accepted because track-type enums are extremely stable.
- occ_ref: memo D2. evidence: memo D2 property/domain/checked-at table + rationale.

### RC3 — D2 sub-decision: no xtask codegen for value domains [restraint / core / decided]
- topic: value-domain source (xtask build-time codegen vs curated table)
- approach: Do NOT add an xtask codegen path for value domains; curate `type`/`codec_kind` domains by hand in `capability` (commit `7e02f86`).
- statement: the v20 schema types `type` as a plain string (no enum) and only `aac_is_sbr` carries a schema enum, so codegen would serve one irrelevant field — an abstraction the scale had not earned.
- steelman: generate value domains from the pinned schema at build time for consistency with the identification-schema extraction already done that way — rejected because only one irrelevant field would benefit.
- occ_ref: journal 2026-07-09 Plan 2 entry + memo D2 note. evidence: journal "D2 dropped an xtask codegen path ... an abstraction the scale had not earned."

### RC4 — D3 unify path types on PathBuf [pattern / core / decided]
- topic: path types in the model (Locator.path)
- approach: Change `Locator.path` from `String` to `PathBuf` to match `output.directory` (commit `339c99d`).
- statement: both are filesystem paths; PathBuf is the honest type and makes planner joins natural; serde/schemars treat PathBuf as string so profile format and JSON Schema are unchanged.
- occ_ref: memo D3. evidence: "Locator.path changes from String to PathBuf ... PathBuf is the honest type."

### RC5 — D4 rendered-filename invariant in the planner [pattern / core / decided]
- topic: rendered output-filename invariant (post-interpolation)
- approach: Planner re-checks the RENDERED filename independently of the config-time template-text check; two new per-file error diagnostics `PathSeparatorInRenderedName` + `EmptyRenderedName` (commit `af76a3a`).
- statement: guard the invariant (no separators, non-empty stem, not `.`/`..`), not the induction proof over field sources; cheap today, load-bearing the day a non-basename field source is added. The empty case (hidden `.mkv`) is reachable now via `template: ""`.
- steelman: today all template fields are basename-derived so separators cannot occur and the check is dead — rejected: guard the invariant, not the proof.
- occ_ref: memo D4. evidence: memo D4 rationale.

### RC6 — D5 lock template brace/escape semantics [pattern / core / decided]
- topic: template brace/escape semantics
- approach: Lock current behavior with tests, no code change — `{{`/`}}` escape, lone `}` literal, lone unclosed `{` is `InvalidTemplate` (forgiving where unambiguous, strict where ambiguous).
- statement: pin behavior with golden tests; a lone `}` cannot start a field so make it literal, an unclosed `{` is a user error.
- occ_ref: memo D5. evidence: memo D5.

### RC7 — D6 suggestion engine simulates against the REAL planner [pattern / core / decided]
- topic: suggestion engine architecture (verified-edit)
- approach: Pure post-pass; each candidate edit is applied to a cloned profile and re-run through the REAL planning pass (`plan_core`, no parallel matcher) over the cached identification of the whole batch.
- statement: "an applied suggestion survives the next dry run" becomes an executable acceptance invariant — accept only if every instance of the conflict is gone batch-wide AND the diagnostic set gains nothing new anywhere.
- occ_ref: memo D6 algorithm steps 3-4. evidence: "re-run the REAL planning pass (same code path as dry-run, no parallel implementation) ... This IS the 'an applied suggestion survives the next dry run' invariant."

### RC8 — D6 closed narrow-only edit grammar [restraint / core / decided]
- topic: suggestion edit-grammar scope
- approach: Closed `StructuredEdit` enum (v1): suggestions only ever NARROW the conflicted rule's match; never reorder rules, never touch other rules, never relax.
- statement: narrow-only guarantees convergence (iterated apply terminates, no oscillation) and keeps YAML-fragment rendering and simulation trivial; simulation covers the whole candidate space by construction.
- steelman: a richer grammar (reorder rules, relax over-narrow rules) would auto-resolve more conflict classes — deferred out of v1 for determinism and convergence.
- occ_ref: memo D6. evidence: "The edit grammar is deliberately closed ... never relax anything (v1 scope)."

### RC9 — D6 cap at 3, non-silent truncation [pattern / core / decided]
- topic: suggestion cap + non-silent truncation
- approach: Deterministic preference order among accepted candidates; emit at most 3 per conflict group; if more were accepted, log the cap in the report.
- statement: bounded output but no silent truncation — record how many were dropped.
- occ_ref: memo D6 step 5. evidence: "Emit at most 3 per conflict group, log the cap in the report if more were accepted (no silent truncation)." (Later violated by inline code, corrected at F7 — RC28.)

### RC10 — D6 remainder deferred (OverlappingRules suggestions + no-single-fix partition) [non-decision / core / deferred]
- topic: OverlappingRules auto-suggestions + no-single-fix partition report
- approach: Ship AmbiguousRule suggestions in Plan 2; defer OverlappingRules candidate generation and the no-single-fix partition report (D6 remainder) to a later plan.
- blocked_on: internal — pending dev work in a later plan.
- occ_ref: journal 2026-07-09 Plan 2 entry, Open threads. evidence: "Deferred: OverlappingRules auto-suggestions and the no-single-fix partition report (D6 remainder)."

---

## B. Process decisions (Plan 2 era)

### RC11 — Spend scarce model strength on decisions, not transcription [pattern / process / decided]
- topic: allocating scarce model capacity (Fable quota crunch)
- approach: Under the Fable quota crunch, spend the strongest model on the DECISIONS (write the D1-D6 memo) and the spec fold-in first; let cheaper/other capacity do mechanical transcription.
- statement: model strength goes where judgment is, not where transcription is; the memo + spec fold-in were written under Fable, then quota moved to Opus for the rest.
- occ_ref: journal 2026-07-09 Plan 2 entry, Decisions. evidence: "spend model strength on the DECISIONS (D1-D6) not the transcription."

### RC12 — Fold normative decisions into the authoritative spec, not just the memo [pattern / process / decided]
- topic: where normative decisions live (spec vs memo)
- approach: Fold the normative D1-D5 rules into the v1 spec (4.4, 4.8, 5.2, 5.4, 9); keep the memo as decision record/rationale only (commit `d4390d7`).
- statement: because "spec wins on conflict," a decision left only in the memo could be silently overridden by stale spec text (4.3/4.4 previously implied codec_kind under substring); fold-in closes that gap.
- occ_ref: memo header + commit `d4390d7`. evidence: memo "The normative rules (D1-D5) are folded into the v1 design spec ... this memo remains the decision record."

### RC13 — Execute Plan 2 INLINE, skipping per-task SDD review [restraint / process / decided]
- topic: Plan 2 execution method (inline vs subagent-driven-development)
- approach: Controller executes the 12 tasks inline with live cargo verification, skipping the SDD apparatus (fresh implementer + independent reviewer per task) used in Plan 1.
- statement: deviation from Plan 1; tradeoff acknowledged at the time — no independent per-task reviewer, whole-branch adversarial review still owed.
- steelman: the plan's code was fully specified and every task was locally compile/test-verifiable, so the SDD implementer+reviewer apparatus looked like overhead for mechanical transcription.
- occ_ref: journal 2026-07-09 Plan 2 entry, Decisions. evidence: "Execution deviated from Plan 1's subagent-driven-development: the controller executed the tasks inline ... Tradeoff: no independent per-task reviewer."

### RC14 — Independent review before merge (SDD) reinstated by the fix pass [pattern / process / violated-corrected]
- topic: independent review before merge (SDD implementer/reviewer separation)
- approach: Fresh implementer subagent per task + independent reviewer + whole-branch review; the plan never grades its own work.
- statement: inline Plan 2 self-verified "125 tests green" but shipped ~11 bugs; the retrofit independent review + SDD fix pass caught them all before merge — the concrete before/after for multi-stage review.
- occ_ref: journal 2026-07-09 fix-pass entry + `plan-2-review/independent-review-2026-07-09.md`. evidence: fix-pass "the independent reviewer/controller separation turned '125 tests green, shipped ~11 bugs' into caught-before-merge."

### RC15 — Whole-branch review catches what per-task reviews miss [pattern / process / reinforced]
- topic: whole-branch adversarial review as a distinct stage
- approach: Run an independent whole-branch review after all per-task reviews pass.
- statement: the FINAL whole-branch review (opus) caught the `.mkv`-literal empty-stem output that EVERY per-task review missed (F6 checked pre-append, not the final stem) — the whole-branch stage earning its place.
- occ_ref: journal fix-pass entry + `plan-2-fixes-sdd/FINAL-review.md` I1 + commit `59d24c8`. evidence: "FINAL whole-branch review (opus) caught what EVERY per-task review missed."

### RC16 — Confirm tool behavior against the real binary [pattern / core / reinforced]
- topic: verify tool behavior empirically (mkvmerge -J / --list-* formats)
- approach: Confirm the `-J` shape, `--list-languages`/`--list-types` formats, and the track `type` domain by running the installed mkvmerge (v99), not from memory or the schema alone.
- statement: both the identify parser and the runtime parsers passed against real output first try; continues the Plan-1 empirical-grounding discipline.
- occ_ref: journal 2026-07-09 Plan 2 entry, Decisions. evidence: "confirmed empirically by running the installed v99 binary rather than trusting memory or the schema alone."

### RC17 — Prose-free core (code+params, no authored English) [pattern / core / violated-corrected]
- topic: prose-free core — diagnostics carry code + structured params only
- approach: Core emits only a diagnostic code + params; all human text lives in the Fluent catalog. No authored English strings in core.
- statement: bug K — identification failure was misreported as MissingTrack/MissingExternal with an authored English string in `detail`; F5 fixed it to emit `UnidentifiableSource` carrying only the third-party IdentifyError text (pass-through allowed) (commits `0e141d1`/`6f475b3`).
- occ_ref: independent-review bug K + F5. evidence: review "identify failure -> MissingTrack/MissingExternal + authored English in detail (prose-free-core violation)."

---

## C. Şenol's triage decisions (2026-07-09), implemented in the fix pass

### RC18 — Decision #1: absent boolean = false in exact matching [pattern / core / decided]
- topic: absent boolean-typed matchable property in exact matching
- approach: An absent boolean-typed matchable property compares equal to `false` for `exact` (mirror mkvmerge/Matroska), so `exact:{flag:false}` matches a track lacking the flag; `:true` still does not (F4, commit `213e1e9`).
- statement: the inline matcher treated absent as never-equal, so `exact:{flag_hearing_impaired:false}` matched zero tracks -> spurious MissingTrack; Şenol ruled mirror-mkvmerge. Scope is any boolean matchable, not just the four vanity flags.
- steelman: the reference profile 4.1 already dodges it via `not:[exact:{flag:true}]`, so absent=never-equal was defensible until the `flag:false` ergonomic case.
- occ_ref: independent-review decision #1(b) + F4-review.md (SPEC pass). evidence: review "exact fallback treats an absent property as never-equal ... -> decision #1(b)."

### RC19 — Decision #2: reject empty any/not at validate time [pattern / core / decided]
- topic: present-but-empty any/not match list
- approach: A present-but-empty `any: []` or `not: []` is a config-time error (new `EmptyMatchList`), not silently treated as no-constraint (F3, commit `a67879e`).
- statement: the inline matcher treated empty `any:[]` as no-constraint via the `!any.is_empty()` guard, contradicting "at least one of zero holds = false"; Şenol ruled it a validate-time error (config-bug-is-config-time-error doctrine).
- occ_ref: independent-review decision #2(c). evidence: review "empty any: [] treated as no-constraint via the !any.is_empty() guard ... -> decision #2(c)."

### RC20 — Decision #3: planned-twice collision always error; policy governs on-disk only [pattern / core / decided]
- topic: output-collision semantics (planned-twice vs on-disk)
- approach: Two planned outputs colliding is ALWAYS a hard `OutputCollision` error (drop both plans) regardless of `on_collision`; the policy governs only pre-existing on-disk files (error / warn+drop / info+keep). Spec 4.8 amended (F6, commit `b5acada`).
- statement: the inline code collapsed Overwrite into Error for planned-twice (undocumented deviation) and `skip` was a no-op; Şenol ruled + amended the spec.
- occ_ref: independent-review decision #3(c) + F6-review.md. evidence: review "two-planned-output collision is always an error regardless of on_collision; the policy governs only pre-existing on-disk files; amend spec 4.8."

### RC21 — dry-run is a strict superset of validate [pattern / cli / violated-corrected]
- topic: dry-run runs config-time validate + lint
- approach: dry-run must run `profile::validate` + `lint::provable_overlaps` and fold those diagnostics in before/alongside planning; `--json` attaches the rendered message per diagnostic. Spec 5.5 made explicit (F1, commit `b507f6e`).
- statement: bug A (found independently by two reviewers) — inline dry-run never ran validate/lint, so a broken regex returned empty + exit 0 and a typo'd property degraded to a misleading MissingTrack.
- occ_ref: independent-review bug A + journal fix-pass. evidence: review "dry-run never runs validate()/lint -> config-time diagnostics unreachable; broken regex returns empty + exit 0."

---

## D. Fix-pass per-task corrections (F1-F9 + I1)

### RC22 — SourceOverwrite must be batch-wide, not per-primary [pattern / core / violated-corrected]
- topic: SourceOverwrite scope (batch-wide donor set)
- approach: Collect ALL input paths batch-wide (primaries + every resolved donor) and fire `SourceOverwrite` if any rendered output equals any of them.
- statement: the F5 implementer scoped `donor_paths` per-primary (local Vec reset per `resolve_file`), so under `on_collision:overwrite` one primary's output could silently overwrite another primary's donor; the dispatch said batch-wide; the independent reviewer found it Critical; fixed via a batch-wide post-pass `detect_source_overwrites` (commit `6f475b3`).
- occ_ref: `plan-2-fixes-sdd/F5-review.md` Critical #1 + journal fix-pass. evidence: F5-review "donor_paths is scoped per-primary, not batch-wide ... a different primary can silently overwrite a donor another primary reads from."

### RC23 — UnidentifiableSource is unconditional (spec over plan text) [pattern / core / decided]
- topic: identification-failure severity vs rule.optional
- approach: A present-but-unidentifiable source is a HARD `UnidentifiableSource` error regardless of `rule.optional` (optional means "zero matches acceptable," not "a broken file is acceptable").
- statement: the fix plan's F5 draft line said donor-identify-failure should respect `optional`; the F5 dispatch superseded it (spec 5.2 UnidentifiableSource is unconditional); the plan text was reconciled to the shipped behavior after the reviewer flagged the unreconciled contradiction (commit `953c5cd`).
- steelman: make donor-identify-failure respect `optional`, consistent with the zero-hits branch — rejected because spec 5.2 makes UnidentifiableSource unconditional.
- occ_ref: fix plan F5 text + F5-review.md Important #2 + commit `953c5cd`. evidence: F5-review "Delivered donor-identify-failure behavior contradicts the plan doc's own F5 instruction, unreconciled."

### RC24 — Keep-name output must preserve inner `.mkv` (per-arm append) [pattern / core / violated-corrected]
- topic: keep-name output filename reconstruction
- approach: Preserve the Keyword ("keep") arm's unconditional `.mkv` append per-arm, not the unified conditional append, so a source named `Show.S01E01.mkv.mkv` keeps its full basename.
- statement: unifying the two `render_output` arms made the append conditional (`if !ends_with .mkv`), silently truncating keep names with an inner `.mkv` (`Show.S01E01.mkv.mkv` -> `Show.S01E01.mkv`) with no diagnostic — a spec 4.8 violation introduced by the diff; reviewer caught Important, fixed `550ba59`.
- occ_ref: `plan-2-fixes-sdd/F6-review.md` Finding 1. evidence: F6-review "render_output's shared .mkv-append step silently truncates 'keep' filenames that contain an inner .mkv."

### RC25 — Empty-stem `.mkv` output must be rejected (check stem after stripping .mkv) [pattern / core / violated-corrected]
- topic: empty-stem rendered output (hidden `.mkv`)
- approach: The `EmptyRenderedName` check must test the stem AFTER stripping a trailing case-insensitive `.mkv`; a template rendering to `.mkv` is an empty-stem hidden file and must be rejected.
- statement: F6 checked the raw pre-append rendered value, so `template: '.mkv'` passed all checks and produced a hidden `.mkv` output at exit 0 — the exact degenerate case D4 exists to prevent; the whole-branch review (I1) caught what every per-task review missed, fixed `59d24c8`.
- occ_ref: `plan-2-fixes-sdd/FINAL-review.md` I1. evidence: FINAL-review "template rendering to .mkv produces a hidden empty-stem output instead of EmptyRenderedName (regression) ... the fix pass lost that."

### RC26 — Suggestion edits never clobber an existing key (narrow never relax) [pattern / core / violated-corrected]
- topic: suggestion no-clobber (D6 "never relax")
- approach: `with_rule_match` must NOT overwrite an existing exact/substring key (`or_insert` semantics); a clobbering candidate becomes a no-op then rejected by the acceptance sim; `not`-list append stays additive.
- statement: bug C — the inline engine merged via `BTreeMap::extend`, so an AddSubstring candidate overwrote an existing `track_name` substring (widened, violating D6 "never relax"), and the acceptance check missed it when the widened match didn't happen to collide; fixed to `or_insert` (commit `68ec6aa`).
- occ_ref: independent-review bug C + F7-review.md (a). evidence: review "with_rule_match uses BTreeMap::extend -> an AddSubstring candidate overwrites an existing track_name substring (widens, violates D6 'never relax')."

### RC27 — Render YAML fragments via the serializer, not string interpolation [pattern / core / violated-corrected]
- topic: YAML fragment rendering for suggestions
- approach: Render `yaml_fragment` by serializing the real `MatchExpr` delta via `yaml_serde`, not by hand-formatting, so any value (colon/comma/brace, or a bool/int) round-trips.
- statement: bug D — the inline code interpolated values unquoted, so a `track_name` with `:`/`,`/`{` emitted broken YAML into the CLI-printed suggestion; using the serializer also fixed an unstated bug (bool/int rendered as strings via StructuredEdit's String value). Reviewer round-tripped 40 adversarial inputs incl. the "Norway problem" (commit `68ec6aa`).
- occ_ref: independent-review bug D + F7-review.md (b). evidence: review "yaml_fragment interpolates string values with zero quoting/escaping -> colon/comma/brace ... breaks the YAML the CLI prints verbatim."

### RC28 — SuggestionsCapped: log the cap, don't truncate silently [pattern / core / violated-corrected]
- topic: non-silent suggestion cap logging (D6 requirement)
- approach: Add `DiagCode::SuggestionsCapped` (Info) logging how many accepted suggestions were dropped past the cap of 3 (commit `68ec6aa`).
- statement: D6 required logging the cap; the inline code truncated silently; F7 added a third diag code (threaded through report.rs + diagnostics.ftl + spec 5.2) — a small scope growth the task invited.
- occ_ref: independent-review nit ("cap-3 truncation not logged") + F7-review.md (c) + journal fix-pass. evidence: F7-review "SuggestionsCapped diagnostic ... Emitted only when total_accepted > 3 ... matches D6: 'log the cap ... no silent truncation'."

### RC29 — Discover symlinked source files [pattern / core / violated-corrected]
- topic: symlink handling in source discovery
- approach: A symlink whose target is a regular file must be discovered (resolve via `fs::metadata` to classify); do NOT recurse into directory symlinks (cycle guard); skip broken symlinks silently (F8, commits `cb3ae84`/`608f2b5`).
- statement: bug I — `walk_files` under `symlink_metadata` dropped symlinked source files (neither `is_file` nor `is_dir`) with no diagnostic.
- occ_ref: independent-review bug I + F8-review.md (SPEC pass). evidence: review "Symlinked source files dropped by walk_files (neither is_file nor is_dir) with no diagnostic."

### RC30 — Human renderer attributes the file on batch-level diagnostics [pattern / cli / violated-corrected]
- topic: human-renderer file attribution
- approach: `Renderer::diagnostic` must include `d.file` when present so batch-level diagnostics (IgnoredFile, DuplicateIdentifier) are attributed, not misread as belonging to the file shown above (F9, commit `2e0dc00`).
- statement: bug J — the inline human renderer dropped the file field; JSON path was unaffected.
- occ_ref: independent-review bug J + F9-report.md. evidence: review "Human renderer drops the file field -> batch-level diagnostics ... print with no file attribution."

### RC31 — Fluent message references only params its emitter supplies [pattern / i18n / violated-corrected]
- topic: Fluent message/emitter param agreement
- approach: The `unknown-property-skew` message must reference only `$version` (what the emitter sets), not `$property`, so it never renders a literal `{$property}` (F9, commit `2e0dc00`).
- statement: bug G — the catalog guard only checked message existence, not param wiring, so the literal `{$property}` leaked. (Deeper: validate hard-rejects unknown props config-time, so skew's untyped-forward-matching path is unreachable in v1 — a known limitation.)
- occ_ref: independent-review bug G + F9. evidence: review "UnknownPropertySkew message references {$property}, emitter only sets version -> renders literal {$property}."

### RC32 — Diagnostic::with_severity builder over public-field mutation [pattern / core / violated-corrected]
- topic: diagnostic severity mutation (builder vs public field)
- approach: Add a `Diagnostic::with_severity` builder (rustdoc'd for `deny(missing_docs)`) and use it instead of mutating the public `severity` field directly (F6).
- statement: nit — the inline code did `Diagnostic::info` then mutated `.severity` directly; F6 encapsulated it behind a builder.
- occ_ref: fix plan F6 + independent-review Reviewer 2 #10 + F6-review.md. evidence: fix plan "Prefer a Diagnostic::with_severity builder over mutating the public field."

### RC42 — mkvmerge-not-found path must still surface config diagnostics [pattern / cli / violated-corrected]
- topic: config diagnostics on the mkvmerge-not-found path
- approach: Even when mkvmerge is absent, dry-run must still run config-time validate+lint and surface those diagnostics (human + JSON); the not-found branch is testable via a PATH override (commit `09d7244`).
- statement: the F1 implementer waved the mkvmerge-not-found path off as a judgment call (dropping config diags); the independent reviewer FAILED it on spec 5.5; the fixer then found the branch WAS testable via PATH override.
- occ_ref: journal fix-pass "What the process caught" F1 + progress.md ledger F1. evidence: fix-pass "reviewer FAILED spec on the mkvmerge-not-found path silently dropping config diagnostics - the implementer had explicitly waved it off."

---

## E. CI / supply-chain (Plan 2)

### RC33 — cargo-deny supply-chain gate; workspace crates publish=false; trim license allow-list [pattern / ci / decided]
- topic: supply-chain gate (cargo-deny)
- approach: Add cargo-deny as a CI gate (Linux-only while private); mark workspace crates `publish = false` (Muxsmith ships as app bundles, not crates.io); trim the license allow-list to the 3 actually used (MIT, Apache-2.0, Unicode-3.0) (commit `5561601`).
- statement: deny caught the intra-workspace path dependency as a wildcard; `publish=false` resolves it and is correct on its own terms; the speculative 8-license allow-list was cut to 3.
- occ_ref: journal 2026-07-09 Plan 2 entry, "What the process caught." evidence: "cargo-deny caught the intra-workspace path dependency as a wildcard; resolved by marking the workspace crates publish = false ... trimmed the license allow-list from a speculative 8 to the 3 actually used."

### RC34 — All gates (fmt/clippy/deny/test) green per commit [pattern / ci / violated-corrected]
- topic: fmt/clippy/deny/test green per commit
- approach: Every commit must pass fmt --check, clippy -D warnings, deny, and test; CI enforces it.
- statement: tasks 3 and 5 were pushed after clippy but not `cargo fmt --check`; the intermediate CI run at `0e64c1e` failed on `fmt --all --check` (fixed `72c59d2`); clippy separately caught two collapsible-if let-chains pre-push. Controller-discipline gap, not the plan.
- occ_ref: journal 2026-07-09 Plan 2 entry, "What the process caught" + commit `72c59d2`. evidence: "CI (test job) caught fmt-dirty commits: tasks 3 and 5 were pushed after running clippy but not cargo fmt --check."

---

## F. Deferrals / accepted tradeoffs recorded in E2

### RC36 — --list-types extension validation deferred (no diag code yet) [non-decision / core / deferred]
- topic: input/locator extension validation against mkvmerge --list-types
- approach: Defer runtime extension validation against `--list-types`; no diagnostic code yet.
- blocked_on: internal — pending dev work in a later plan.
- occ_ref: journal 2026-07-09 Plan 2 entry, Open threads. evidence: "`--list-types` extension validation (no diag code yet)."

### RC37 — CI does not install mkvtoolnix; gated tests self-skip [non-decision / ci / deferred]
- topic: mkvmerge-gated integration tests in CI
- approach: CI does not install mkvtoolnix, so mkvmerge-gated tests self-skip there; verification relies on local real-binary runs.
- blocked_on: internal — Plan 4 later adds the install step.
- occ_ref: journal 2026-07-09 Plan 2 entry, Open threads. evidence: "CI does not install mkvtoolnix so the gated tests self-skip there."

### RC38 — mkvmerge-query-failed path still drops config diags (same class as F1) [non-decision / cli / deferred]
- topic: config diagnostics on the mkvmerge-query-failed path
- approach: Defer fixing the query-failed path (`list_languages` fails), which drops config diagnostics — the same class as the F1 mkvmerge-not-found fix but left out of F1 scope.
- blocked_on: internal — pending dev work / logged in the ledger.
- occ_ref: progress.md ledger F1 residual + journal fix-pass, Open threads. evidence: progress.md "the mkvmerge-query-failed path (list_languages fails) has the same defect - config diags dropped - left out of F1 scope."

### RC39 — Six Minor final-review items deferred to follow-up [non-decision / cross / deferred]
- topic: the six Minor whole-branch-review findings (M1-M6)
- approach: Ship the fix pass and defer the 6 Minor final-review findings (double-report, render-fail donor gap, IdentifyError English, TempDir leaks, double file print, mkvmerge_found JSON asymmetry) to a follow-up.
- blocked_on: internal — pending dev work / owner disposition.
- occ_ref: `plan-2-fixes-sdd/FINAL-review.md` MINOR section + journal fix-pass, Open threads. evidence: fix-pass "6 Minor items from the final review recorded in the archived FINAL-review.md for a follow-up."

### RC40 — Render-failed file's donor escapes batch-wide SourceOverwrite (narrow gap accepted) [non-decision / core / deferred]
- topic: SourceOverwrite gap for render-failed files' donors
- approach: Accept for v1 that a donor referenced only by a render-failed file (`plan == None`, assignments discarded) escapes the batch-wide SourceOverwrite set.
- blocked_on: internal — pending dev work; rated Minor "extremely narrow."
- occ_ref: `plan-2-fixes-sdd/FINAL-review.md` M2. evidence: FINAL-review "a donor referenced only by a render-failed file escapes the batch-wide SourceOverwrite set ... a real run would clobber X."

### RC41 — IdentifyError English framing in `detail` accepted as third-party pass-through [non-decision / core / deferred]
- topic: third-party error framing in UnidentifiableSource.detail
- approach: Accept core-authored English framing in `IdentifyError` ("mkvmerge failed:", "cannot read file:") flowing into the `detail` param as sanctioned third-party pass-through; defer a clean `Display` impl.
- blocked_on: internal — a `Display` impl on IdentifyError deferred to a later pass.
- occ_ref: `plan-2-fixes-sdd/FINAL-review.md` M3 + F5-review.md Minor #4. evidence: FINAL-review "F5 improved it ... but did not fully eliminate core-authored English ... Accept; be aware detail will always be English."
