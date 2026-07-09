# Plan 2 independent review - 2026-07-09

Retrofit of the SDD independent-review stage that Plan 2 skipped (Plan 2 was
executed inline by the controller, so no per-task reviewer ran). Four fresh
reviewer subagents (sonnet), no stake in the plan, each given the spec + the D1-D6
decision memo as ground truth and a distinct slice of the diff. They ran against
a real mkvmerge (v100 on the machine at review time; pinned schema 20 still
matches) and reproduced several findings with standalone repro binaries. This
file preserves their reports verbatim-in-substance plus the controller triage.

The point this review demonstrates: the controller's "125 tests green" self-
verification missed every item below. Two reviewers independently found the same
headline bug (dry-run bypasses validate).

## Controller triage (what gets fixed)

Confirmed bugs -> fix pass (see docs/superpowers/plans/2026-07-09-plan-2-fixes.md):
- A. dry-run never runs validate()/lint -> config-time diagnostics unreachable; broken regex returns empty + exit 0; typo'd property -> misleading MissingTrack. (Reviewers 2 and 4, corroborated.)
- B. EmptyRenderedName tests `name` (post-.mkv-append) not `stem_only`; the `.`/`..` branch is dead; a template rendering to `.` yields `..mkv`.
- C. Suggestion engine with_rule_match uses BTreeMap::extend -> an AddSubstring candidate overwrites an existing track_name substring (widens, violates D6 "never relax").
- D. yaml_fragment interpolates values unquoted -> a track_name with `:`/`,`/`{` emits broken YAML into the CLI-printed suggestion.
- E. on_collision: skip maps to Warning but only Error drops a plan -> skip is a no-op.
- F. dry-run --json omits the rendered message (spec 5.2; validate --json includes it).
- G. UnknownPropertySkew message references {$property}, emitter only sets version -> renders literal {$property}.
- H. SourceOverwrite checks primaries but not external-donor source paths.
- I. Symlinked source files dropped by walk_files (neither is_file nor is_dir) with no diagnostic.
- J. Human renderer drops the file field -> batch-level diagnostics (IgnoredFile, DuplicateIdentifier) print with no file attribution.
- K. Identification failure misreported as MissingTrack/MissingExternal with an authored English string in `detail` (prose-free-core violation; detail dropped anyway).

Decisions (Şenol, 2026-07-09):
- #1 -> (b): absent boolean-typed matchable property compares equal to false for exact matching; mirror mkvtoolnix's own treatment.
- #2 -> (c): reject a present-but-empty `any`/`not` list at validate time.
- #3 -> (c): two-planned-output collision is always an error regardless of on_collision; the policy governs only pre-existing on-disk files; amend spec 4.8.

Nits (fix opportunistically, not blocking): OverlappingRules names only first two of >=3 claimants; rule-ref formatting differs (lint `0` vs planner `tracks[0]`); regex recompiled per match call; no proptest coverage (spec 10); D6 cap-3 truncation not logged; suggestion engine has no multi-file test; candidate gen ignores top-level codec/id; LanguageIndex row-collision fragility; separator-row empty-name filter; identify --json subset of human output; cache-key mtime fallback (platform edge).

Explicitly cleared by reviewers (sound): prose-free core I/O (no println in core); suggestion-engine determinism (all load-bearing collections are BTree*, sorts stable); the plan_core-vs-plan_batch diagnostic asymmetry (harmless - discovery diags are invariant under a tracks[] edit); PropValue int/float split; malformed-JSON defaults without panics; PINNED=20 matches real mkvmerge; CLI exit-code folding; case sensitivity per spec 4.3; codec_kind prefix matching.

---

## Reviewer 1 - matcher and capability

CONFIRMED (HIGH) matcher.rs:87-90 - exact fallback treats an absent property as never-equal, including for the four vanity boolean flags (flag_hearing_impaired/visual_impaired/commentary/original) which mkvmerge omits when unset. `exact: {flag_hearing_impaired: false}` matches zero tracks on normal files -> spurious MissingTrack. Reference profile 4.1 dodges it via `not:[exact:{flag:true}]`. Empirically reproduced against mkvmerge output. [-> decision #1(b)]

PLAUSIBLE (MEDIUM) matcher.rs:47-52 - empty `any: []` treated as no-constraint via the `!any.is_empty()` guard, contradicting the literal "at least one of zero holds = false". `not` side is unambiguous. [-> decision #2(c)]

PLAUSIBLE (LOW-MED) runtime.rs:143-158 - LanguageIndex::from_rows: if two rows ever shared a non-empty 639-1/-2 code, later overwrites earlier (order-dependent). Not live in the real v100 table (verified 8783 rows, no collisions) but unguarded. [nit]

PLAUSIBLE (LOW) runtime.rs:184 - separator filter `cols[0].chars().all(...)` is vacuously true on an empty string, so a blank-name row would be misclassified as separator and dropped. No such row today. [nit]

Nits: no proptest despite spec 10; regex recompiled per matches() call. Clean: case sensitivity, codec_kind prefix, int/float scalar_eq, parse_list_types, normalize/lang_eq for base codes, clippy silent.

## Reviewer 2 - planner resolution and output

1. CONFIRMED planner.rs:412 - SourceOverwrite checks only primaries, never a plan's own external-donor source paths -> a plan can render output onto a donor it reads. [H]
2. CONFIRMED dry_run.rs:55 - dry-run never calls validate/lint -> config-time diagnostics vanish; compounds with planner.rs masking a template parse failure as generic EmptyRenderedName. [A]
3. CONFIRMED planner.rs:513 - on_collision: skip -> Warning, but per spec 5.5 only error drops a file, so skip is a no-op. [E]
4. CONFIRMED planner.rs:480 - EmptyRenderedName "."/".." check tests wrong var (name post-append) -> dead branch; "." -> "..mkv". [B]
5. PLAUSIBLE planner.rs:309 - donor-identify-failure ignores rule.optional, inconsistent with the zero-hits branch. [fold into K]
6. CONFIRMED planner.rs:244 - primary identify-failure reuses MissingTrack + ad hoc detail instead of a dedicated code. [K]
7. PLAUSIBLE planner.rs:511 - planned-twice collapses Overwrite into Error, undocumented deviation from 4.8. [-> decision #3(c)]
8. CONFIRMED planner.rs:398 - OverlappingRules names only first two of >=3 claimants. [nit]
9. CONFIRMED planner.rs:413 - dead/redundant `out == &primary.path` clause. [nit]
10. CONFIRMED planner.rs:523 - Diagnostic::info then .severity mutated directly. [nit]
Corroborating: zero tests touch EmptyRenderedName/PathSeparatorInRenderedName/SourceOverwrite/OutputCollision/any collision policy; no planner test exercises an external-source rule. Cleared: OverlappingRules keying by (source_path, track_id) is correct and order-independent; PathBuf equality for donor/overwrite is sound.

## Reviewer 3 - suggestion engine (D6)

1. CONFIRMED planner.rs:754 - with_rule_match merges AddSubstring via BTreeMap::extend -> overwrites an existing track_name substring, widening not narrowing (violates D6 "never relax"); acceptance check does not catch it when the widened match happens not to collide. [C]
2. CONFIRMED planner.rs:837 - yaml_fragment interpolates string values with zero quoting/escaping -> colon/comma/brace in a track_name breaks the YAML the CLI prints verbatim (dry_run.rs:126). [D]
Spec gaps: cap-3 truncation (planner.rs:595) not logged despite D6 requiring it; candidate gen (planner.rs:636) never considers top-level codec/id (only type injected); diag_signature (planner.rs:784) uses BTreeSet where D6 says multiset (harmless today, latent). [nits]
Test gap: suggestions.rs plan() always one file, so the batch-wide claim has zero coverage; D6's multi-file property test does not exist.
Cleared (explicitly traced): the plan_core-vs-plan_batch diagnostic asymmetry is harmless (discovery diags depend only on input+fs, never on tracks[], and every candidate touches only tracks[ri].match_expr, so sim subset-of-baseline can neither false-reject nor hide a regression); determinism sound (no HashMap in the candidate/ranking path; the one HashMap is the point-looked-up identify cache).

## Reviewer 4 - identify, CLI, i18n/spec-compliance

1. CONFIRMED dry_run.rs:23-29 - never runs validate()/lint (dup of Reviewer 2 #2); broken regex -> empty + exit 0; `langauge:` typo -> MissingTrack instead of UnknownProperty. Not in HANDOFF deferred list -> oversight. [A]
2. CONFIRMED dry_run.rs:57-58 - --json serializes raw Batch, no rendered message (spec 5.2). validate.rs:29-33 injects v["rendered"]; dry_run does not. [F]
3. CONFIRMED planner.rs:260-266 + diagnostics.ftl:37 - UnknownPropertySkew message uses {$property}, emitter sets only version -> renders literal {$property}; catalog guard only checks existence not params. Deeper: validate hard-rejects unknown props config-time, so skew's "untyped forward matching" has no reachable path (known limitation). [G + note]
4. CONFIRMED discovery.rs:197-222 - symlinked source files dropped (symlink is neither is_file nor is_dir), no diagnostic. [I]
5. CONFIRMED i18n.rs:68-84 + cli.ftl:3 - diagnostic-line has no $file; batch-level diagnostics print with no file attribution -> misread as belonging to the file shown above. JSON path unaffected. [J]
6. CONFIRMED planner.rs:244-258/309-327 - identify failure -> MissingTrack/MissingExternal + authored English in detail (prose-free-core violation; detail dropped since ftl does not reference $detail). Same class at identify.rs:100-102. [K]
7. PLAUSIBLE identify.rs:223-235 - cache key degrades to size-only when modified() errs; a same-size in-place edit would serve stale. Narrow on primary targets. [nit]
8. CONFIRMED identify.rs:40-55 vs 57-83 - identify --json (id/type/codec) is a strict subset of the human view (adds language); pattern inverted. [nit]
Nits: OverlappingRules >=3 claimants; lint vs planner rule-ref formatting; no identify CLI test. Clean: prose-free core I/O; PropValue int/float; malformed-JSON defaults no panic; PINNED=20 matches v100; catalog param wiring for the rest of the Plan 2 codes; CLI exit-code folding; --source default ".".
