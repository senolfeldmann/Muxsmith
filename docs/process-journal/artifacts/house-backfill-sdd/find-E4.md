# House-knowledge backfill - Muxsmith era E4 (Plan 3.5, mkvtoolnix parity, 2026-07-09)

Reconstructed decision history from the persisted trail. Sources, richest first:
memo `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md` (D19-D21);
reviewer verdicts `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/*`; the
Plan-3.5 journal entry (`docs/process-journal.md`, 2026-07-09 session 4);
plan `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md`;
progress ledger `.../plan-3.5-sdd/progress.md`; `docs/IDEAS.md` #1-4 (born at
b04c4a2, the E4 design commit) for the rejected input-convenience guesses.

Commit range: b04c4a2 (memo/plan/ideas) -> 91b19eb..aa75025 (8 impl commits) -> c1d5614 (memo D20-B/canonicalize update). One record per occurrence; the same (topic,approach) at several points is recurrence, kept separate.

---

## A. Sequencing and audit framing (process)

### 1. Plan 3.5 slotted before Plan 4 :: finish the declarative pure layer mkvtoolnix-faithful before building the process layer on it
- kind: pattern | domain: process | occ_kind: decided
- occ_ref: memo (Status/Scope) + journal 2026-07-09
- evidence: memo "Plan 3.5 is a set of mkvtoolnix-parity fixes to the pure layer, slotted BEFORE Plan 4 ... same rationale as D7."

### 2. mkvtoolnix parity target criterion :: only muxing semantics/output are parity targets; input-time convenience guesses are not (interactive-GUI vs declarative-batch distinction)
- kind: pattern | domain: process | occ_kind: decided
- occ_ref: memo (Origin/Grounding)
- evidence: memo "compare against mkvtoolnix-gui / mkvmerge wherever meaningful, weighing the interactive-GUI vs declarative-batch distinction."

### 3. Parity-audit method :: formalized as SI-3 standing method (audit is now repeatable doctrine)
- kind: pattern | domain: process | occ_kind: reinforced
- occ_ref: journal 2026-07-09 ("Framing rule, now SI-3") + memo ("That audit is now a standing method, HANDOFF SI-3")
- evidence: journal "Framing rule, now SI-3: mkvtoolnix is INTERACTIVE ... Muxsmith DECLARATIVE BATCH ... only muxing semantics/output are parity targets."

## B. Rejected input-convenience guesses (core restraints, all dispositioned in the E4 audit)

### 4. Language derivation from filename :: infer a track's language from its filename
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: IDEAS.md #1 (b04c4a2) + journal 2026-07-09
- steelman: mkvtoolnix does it by default for audio/subs (IfAbsentOrUndetermined); sidecar naming like `Movie.eng.srt` is near-universal, so value is high for external-subtitle bulk workflows.
- evidence: IDEAS.md #1 "This is a magic guess ... would fire unseen across hundreds of files with no review step ... Şenol's ruling: 'not the concern of Muxsmith, hard no.'"

### 5. Flag derivation from filename/track name :: set commentary/hearing-impaired/forced flags from name
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: IDEAS.md #2 (b04c4a2) + journal 2026-07-09
- steelman: all three derivations are on by default in mkvtoolnix (m_setCommentaryFlagFromFileName etc.), emitted as --commentary-flag / --forced-display-flag.
- evidence: IDEAS.md #2 "Same category as idea 1: a filename-based magic guess ... wrong as an unseen default in declarative batch. Same hard no."

### 6. `unique` collision policy :: auto-suffix colliding outputs `Name (1).mkv` instead of erroring
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: IDEAS.md #3 (b04c4a2) + journal ("unique-name suffix")
- steelman: this is mkvtoolnix-gui's default (m_uniqueOutputFileNames=true); appends ` (1)`, ` (2)` until no collision with disk or queue.
- evidence: IDEAS.md #3 "Silent ` (1)` suffixing just litters the output tree and hides a real naming mistake. Fail loud instead."

### 7. Batch append of multi-part sources :: auto-concatenate CD1/CD2 sequences into one output
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: IDEAS.md #4 (b04c4a2) + journal ("sequence append")
- steelman: mkvtoolnix m_mergeReconstructSequencesWhenAdding=true auto-detects numeric sequences and appends them.
- evidence: IDEAS.md #4 "Appending is a fundamentally different operation from Muxsmith's model ... spec section 11 lists it explicitly out of v1 scope"; auto-detect is the rejected magic-guess pattern.

## C. D19 - BCP-47 language (core)

### 8. Language acceptance predicate :: valid = bare ISO code OR well-formed BCP-47 tag (parse() only), not ISO-index membership
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D19 (Decision - Acceptance = well-formedness)
- evidence: memo "A language value is valid if it is either a bare ISO code ... OR a well-formed BCP-47 language tag ... Grammatically-well-formed-but-nonexistent combinations (e.g. xx-YY) are accepted here and left for mkvmerge to reject at mux time."

### 9. Accept-liberal / match-precise :: accept side uses parse() well-formedness, match side uses canonicalize() (deliberate asymmetry)
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D19 ("Accept-liberal, match-precise (deliberate asymmetry)")
- evidence: memo "The accept side (validation) uses parse() well-formedness only ... The match side uses canonicalize(). Coherent: exact:{language:xx-YY} then never matches any real track, which is harmless."

### 10. Accept-liberal / match-precise :: re-affirmed sound by whole-branch review
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: whole-branch-review-verdict.md (Strengths + Assessment)
- evidence: whole-branch "The BCP-47 accept/match asymmetry is sound and self-consistent ... bare ISO codes never leave the fast path ... region/script tags route to the canonical path." Walked pt-BR against a por + language_ietf:pt-BR track.

### 11. Registry validate() on the accept side :: reject well-formed-but-nonexistent tags at plan time
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: memo D19 ("registry validate() is NOT called") + plan self-review ("Not in scope, deferred by decision")
- steelman: calling the crate's registry validate() would catch typos like xx-YY at plan time rather than deferring rejection to mkvmerge at mux time.
- evidence: memo "over-accepting a pathological tag is far less harmful than today's over-rejecting of real ones, and matching mkvmerge's exact registry cheaply is not possible per-tag."

### 12. BCP-47 parsing dependency :: use the `language-tags` crate rather than a hand-rolled RFC 5646 parser
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D19 ("Resolved, Şenol 2026-07-09")
- evidence: memo "Use the language-tags crate (pure Rust, RFC 5646, bundles the IANA registry) ... Correctness on a standard grammar beats a hand-rolled parser; one vetted dependency (through cargo deny) is justified."

### 13. `language-tags` 0.3.2 :: dependency vetted clean (MIT/Apache-2.0, no transitive deps, deny.toml untouched)
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: task-4-review-verdict.md + progress ledger Task 4
- evidence: task-4 verdict "deny.toml untouched, MIT/Apache-2.0 already allowed"; progress "language-tags 0.3.2 added (MIT/Apache-2.0, no transitive deps, deny.toml untouched)."

### 14. `exact` operator semantics :: exact is typed value-equality, not raw string equality (numbers numerically, languages canonicalized); regex for byte-literal
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D19 (Principle, Şenol 2026-07-09) -> spec 4.3 (Task 5 step 6)
- evidence: memo "Muxsmith's exact operator is typed value-equality, not raw string equality: each property is compared in its own domain ... de==ger, pt-Latn-BR==pt-BR while meaningful distinctions survive (pt-BR!=pt-PT). This is one of the tool's core semantics; surface it in the public docs at 1.0."

### 15. `exact` value-equality principle :: flagged as load-bearing for the 1.0 README pass, keep the flag alive
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: whole-branch-review-verdict.md (Recommendations)
- evidence: whole-branch "the exact is typed value-equality ... principle now stated in spec 4.3 is genuinely load-bearing and easy to lose ... Keep that flag alive."

### 16. Language match equality :: compare BCP-47 canonical form via canonicalize() (case + script suppression + deprecated-subtag replacement), raw fallback otherwise
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D19 ("Equality for matching = canonical form") -> Task 5
- evidence: memo "so pt-Latn-BR==pt-BR (redundant default script dropped) and iw==he (deprecated code replaced), while meaningful distinctions are preserved: pt-BR!=pt-PT, zh-Hans!=zh-Hant."

### 17. Canonicalize-matching now (option B) vs cut it as a no-op :: done now
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: journal 2026-07-09 (Decisions - D19 matcher)
- steelman: the pre-existing raw fallback already matched equal tags case-insensitively, so canonicalize() looked like a no-op that could be skipped.
- evidence: journal "D19 matcher (canonicalize) was nearly cut as a no-op ... Trace showed the cited fragility (pt-Latn-BR vs pt-BR) needs canonicalize(), which language-tags bundles (cheap). Şenol chose to do it now (option B)."

### 18. Invalid-language test fixtures :: must use a value that fails BCP-47 well-formedness (`notalanguage`, len>8), not short `zz`/`zzz`
- kind: pattern | domain: testing | occ_kind: decided
- occ_ref: task-4-review-verdict.md (Named-risk verification) + journal ("What the process caught", Task 4)
- evidence: journal "implementer found zz/zzz invalid-language fixtures are WELL-FORMED BCP-47, so the widened predicate would silently accept them -> those tests stop testing. Changed to notalanguage. Reviewer re-probed the crate to confirm. Origin: plan."

## D. D20 - tracks block and keep policy (core)

### 19. Unmatched-track policy :: `tracks.unmatched: keep | drop`, default `drop` (the declarative default)
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D20 (Decision)
- evidence: memo "Default stays drop - that IS Muxsmith's declarative essence and its primary (curative) use case ... tracks get the same policy [as attachments] with the opposite default."

### 20. `keep` value necessity :: retained despite Şenol questioning the need, because additive bulk ops are inexpressible under drop
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: journal 2026-07-09 (Decisions) + memo D20 (Why)
- evidence: journal "D20 keep kept despite Şenol questioning the need. One real case: additive bulk ops (add a sub to a library, keep the rest) are inexpressible under drop (one-track-per-rule forbids a catch-all)."

### 21. `tracks` shape :: restructure the bare rule list into a nested `{ unmatched, rules }` block, NOT a top-level `unmatched_tracks` key
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D20 (Decision) + journal 2026-07-09
- steelman: a top-level `unmatched_tracks` key is more ergonomic for the bare-list case (Peter's recommendation).
- evidence: journal "Şenol drove placement: nested block ... because the profile already keeps policies in their section (output.on_collision, tags.global). I'd recommended top-level for the bare-list ergonomic; conceded on whole-profile consistency."

### 22. `TracksCfg.rules` :: intentionally no serde default, so `tracks:` stays a mandatory Profile field (only `unmatched` defaults, via drop_policy())
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: task-1-review-verdict.md + plan Task 1 Step 3
- evidence: task-1 verdict "rules having no #[serde(default)], which is deliberate: tracks: itself stays a mandatory Profile field with no default, exactly preserving the old requirement that the list must be present."

### 23. keep policy scope :: governs the PRIMARY's own tracks only; donors and attachments unaffected (a donor contributes only its rule-selected track)
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D20 ("Donors and attachments are unaffected")
- evidence: memo "keep unmatched donor tracks is never wanted (it would pull in the donor's video/subs). Mirrors the primary-only scope of attachment rules (D10/E5)."

### 24. keep donor-isolation :: proven STRUCTURAL, not conventional (input_groups always makes group 0 the primary; guard source==plan.source can never match a donor)
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: task-2-review-verdict.md (Named Risk Check)
- evidence: task-2 verdict "no group in groups other than index 0 can ever equal plan.source ... a donor sharing the primary's path would already have been absorbed into group 0 ... structurally guaranteed."

### 25. keep + explicit primary rules :: change properties, not position; to reorder the primary use `drop` mode
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D20 ("the one point held open for veto, not vetoed")
- evidence: memo "explicit primary rules under keep change properties, not position; to reorder the primary, use drop mode."

### 26. keep-mode `--track-order` :: assumption #3 - only matched tracks listed, mkvmerge appends kept-unmatched after (so a donor-only keep profile puts the added track FIRST)
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: plan Task 3 + memo D20 (the earlier assumption, later reversed)
- steelman: this is mkvmerge's actual native behavior (`--track-order 1:0` with primary+donor yields [DONOR,PRIM0,PRIM1]), the faithful/simplest rendering, live-verified against mkvmerge v100.
- evidence: memo "The earlier assumption (matched-ordered-first, unmatched-appended-after) produced donor-FIRST ordering for a donor-only keep profile ... which the Plan 3.5 whole-branch review flagged."

### 27. keep-mode `--track-order` :: assumption #3 confirmed against real mkvmerge v100 (BRAVO,ALPHA,CHARLIE)
- kind: restraint | domain: testing | occ_kind: reinforced
- occ_ref: task-3-review-verdict.md + progress ledger Task 3
- evidence: progress "SI-3 CONFIRMED against mkvmerge v100: keep_unmatched ordering only id1 of a 3-track source yields BRAVO,ALPHA,CHARLIE (ordered first, unlisted after in source order). D20 assumption holds; NO planner change needed."

### 28. keep-mode `--track-order` :: reversed to option B - all primary tracks first in source order (0:id), donors trail (g:id in rule order)
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: whole-branch-review-verdict.md (Important #1, the flag) -> journal + memo D20-B (Şenol's reversal, commit c1d5614)
- evidence: whole-branch flagged donor-first as "a genuine usability trap on the one workflow keep exists to enable"; journal "Şenol called B (donors trail): keep = match what's already there makes kept-unmatched primary tracks count as matched, so --track-order lists them (invariant holds), primary first."

### 29. keep-mode `--track-order` :: option B built and live-verified (primary+donor -> 0:0,0:1,1:0 -> [PA,PB,DONOR])
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: task-7-review-verdict.md + commits 51567d7, aa75025 + progress Task 7
- evidence: progress "Under keep --track-order = all primary tracks (0:id source order) then donors (g:id rule order) ... verified real mkvmerge v100: primary[PA,PB]+donor[DONOR] -> [PA,PB,DONOR]."

### 30. keep+donor track-order coverage :: a gated live test alone is insufficient; add a deterministic (non-gated) unit guard that runs without mkvmerge
- kind: pattern | domain: testing | occ_kind: violated-corrected
- occ_ref: task-7-review-verdict.md (Important) -> fix commit aa75025
- evidence: task-7 verdict "the exact scenario this task exists to fix ... has zero guaranteed-run regression coverage in an environment without mkvmerge"; fix aa75025 "deterministic Plan-literal unit test ... implementer verified it's a real guard (scratch-reverted keep branch two ways -> both FAIL)."

## E. D21 - UnsupportedSource diagnostic (core)

### 31. Unsupported source handling :: pre-resolution gate emits one clean UnsupportedSource error and skips the file, instead of per-rule MissingTrack noise
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D21 (Decision)
- evidence: memo "A source that is not usable media emits one clear error-severity diagnostic and its plan is skipped ... the user sees rule X: missing track when the real cause is this file is not a source mkvmerge can mux."

### 32. UnsupportedSource code :: a distinct DiagCode from UnidentifiableSource, because the remediation differs (identified-but-unsupported vs mkvmerge-exit-nonzero)
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D21 ("New code UnsupportedSource, distinct")
- evidence: memo "the latter stays for mkvmerge exit-nonzero ... the former is mkvmerge identified it but it is not a muxable source. Two codes because the remediation differs."

### 33. UnsupportedSource gate predicate :: `!container_recognized || !container_supported` ONLY, not is_identifiable(); a recognized+supported zero-track container stays MissingTrack
- kind: pattern | domain: core | occ_kind: decided
- occ_ref: memo D21 (Open mechanic / assumption #5)
- evidence: memo "fire UnsupportedSource when !container_recognized || !container_supported; a recognized+supported container with zero tracks stays a per-rule MissingTrack issue (a valid but empty container, a different problem)."

### 34. UnsupportedSource gate predicate :: re-affirmed correct by review (three tests lock both OR-branches and the zero-track boundary)
- kind: pattern | domain: core | occ_kind: reinforced
- occ_ref: task-6-review-verdict.md + whole-branch-review-verdict.md (Strengths)
- evidence: whole-branch "Crucially it uses the raw !container_recognized || !container_supported predicate, not is_identifiable() ... the three regression tests lock all three corners including the zero-track-stays-missing-track boundary."

### 35. Donor-side UnsupportedSource gate :: not built; the gate is primary-only per D21
- kind: restraint | domain: core | occ_kind: decided
- occ_ref: plan self-review ("Not in scope, deferred by decision") + task-6-review-verdict.md
- steelman: a donor mkvmerge cannot mux would also fail later; a symmetric donor gate would catch it earlier.
- evidence: plan self-review "donor-side UnsupportedSource gate (primary-only per D21)"; task-6 verdict "Report explicitly scopes out the donor-path gate as deferred-by-decision ... cites where that scope call comes from (D21 + plan self-review)."

### 36. Zero-rule profile under `keep` :: assumption kept as a NoTrackRules error, held open for Şenol veto (a pure-passthrough remux is a semantically valid no-op)
- kind: non-decision | domain: core | occ_kind: deferred
- occ_ref: memo D20 (Open mechanics, plan-time)
- blocked_on: Şenol veto - whether a zero-rule `keep` passthrough profile should be legal rather than a NoTrackRules error
- evidence: memo "Assumption: still an error (a zero-rule profile is almost certainly a mistake, even though keep+zero-rules is a semantically valid no-op remux); contradict if a pure passthrough profile should be legal."

## F. Testing and SI-3 verification discipline (testing/process)

### 37. mkvmerge behavior facts :: confirm by RUNNING the binary (v100), never from memory (SI-3)
- kind: pattern | domain: testing | occ_kind: decided
- occ_ref: plan Global Constraints
- evidence: plan "mkvmerge is external (v100, identification schema v20). Confirm mkvmerge behavior by RUNNING the binary, never from memory (gated tests self-skip when mkvmerge is absent)."

### 38. SI-3 verification :: whole-branch reviewer drove real mkvmerge v100 for the cross-file donor+keep ordering claim rather than trusting the single-file test
- kind: pattern | domain: testing | occ_kind: reinforced
- occ_ref: whole-branch-review-verdict.md (header note + Important #1)
- evidence: whole-branch "I verified the one load-bearing runtime claim (donor+keep ordering) against the real mkvmerge v100 rather than trusting the single-file test."

### 39. SI-3 verification :: Task 3 and Task 7 implementers hand-ran mkvmerge before encoding the order assertion, unprompted
- kind: pattern | domain: testing | occ_kind: reinforced
- occ_ref: journal 2026-07-09 (Moments) + task-3/task-7 verdicts
- evidence: journal "Tasks 3 and 7 implementers both hand-ran mkvmerge before encoding the order assertion, unprompted (SI-3 holding)."

### 40. Gated mkvmerge tests :: self-skip (eprintln + early return) when the binary is absent, for CI parity with sibling gated tests
- kind: pattern | domain: testing | occ_kind: reinforced
- occ_ref: task-3-review-verdict.md + plan Task 3
- evidence: task-3 verdict "Self-skip idiom identical to sibling (CI parity): let Some(m) = mkvmerge() else { eprintln!(...); return; } - same helper, same message pattern, no assertion silently skipped."

### 41. Dependency behavior :: verify against the crate's actual source/probes, not the brief's assumption (Task 5 corrected the brief's wrong `xx-YY`->None doc-comment)
- kind: pattern | domain: process | occ_kind: reinforced
- occ_ref: task-5-review-verdict.md + journal ("What the process caught", Task 5)
- evidence: journal "implementer corrected a brief doc-comment (xx-YY canonicalizes to itself, not None; crate errors only on MultipleExtendedLanguageSubtags). Reviewer cross-checked crate source (Suppress-Script table). Origin: my brief."

## G. Standing process/CI/i18n conventions reapplied in E4

### 42. Plan execution :: SDD (subagent-driven-development) per SI-1 - fresh implementer + independent reviewer per task, controller re-runs the suites itself
- kind: pattern | domain: process | occ_kind: reinforced
- occ_ref: plan Global Constraints + progress ledger (Execution)
- evidence: plan "Execute via SDD per HANDOFF SI-1 (fresh implementer + independent reviewer per task); the controller re-runs suites itself."

### 43. SDD execution parallelism :: strictly-serial execution was the miss; independent streams (D19/D20/D21, disjoint planner regions) should run parallel in worktrees (SI-1 rewritten)
- kind: pattern | domain: process | occ_kind: violated-corrected
- occ_ref: journal 2026-07-09 (Friction/failure)
- evidence: journal "STRICTLY SERIAL execution ... Şenol: I am waiting for something that could have been faster. SI-1 rewritten (Superpowers-throughout + parallelize-independent) ... The clearest miss of the session."

### 44. Per-commit gate :: run all four - `cargo test --workspace` + `fmt --all --check` + `clippy --all-targets -D warnings` + `deny check` - and the controller re-runs them after every task
- kind: pattern | domain: ci | occ_kind: reinforced
- occ_ref: plan Global Constraints + progress ledger (green each gate)
- evidence: plan "Per-commit gate, run all four, do NOT skip fmt"; progress "controller re-ran full gate (test/fmt/clippy -D warnings/deny) after every task."

### 45. Diagnostics i18n :: core emits no user-facing prose (DiagCode + params only); every human string lives in Fluent locales, a new DiagCode without a `.ftl` message fails catalog_completeness
- kind: pattern | domain: i18n | occ_kind: reinforced
- occ_ref: plan Global Constraints + Task 6 (added unsupported-source.ftl)
- evidence: plan "Core emits no user-facing prose ... A new DiagCode without a matching .ftl message fails ... catalog_completeness.rs"; Task 6 added the `unsupported-source` message alongside the new code.

### 46. Decided restructure :: apply thoroughly - migrate every consumer, fixture and inline-test YAML, leave zero orphaned sites (thorough_separation)
- kind: pattern | domain: process | occ_kind: reinforced
- occ_ref: task-1-review-verdict.md + whole-branch-review-verdict.md
- evidence: whole-branch "The tracks restructure is complete. A full-repo sweep for profile.tracks bare-list accesses returns zero un-migrated sites ... thorough_separation applied correctly." Task-1 reviewer independently re-scanned the repo, not just the diff.

### 47. Plan wire-format :: new serialized `Plan` fields (keep_unmatched; later primary_track_ids) are conscious interface extensions with downstream consumers (Plan-4 executor, Plan-5 GUI)
- kind: pattern | domain: cross | occ_kind: decided
- occ_ref: whole-branch-review-verdict.md (Minor #5) [later memo-level at 88484ed addendum, out of E4]
- evidence: whole-branch "keep_unmatched: bool now serializes into any Plan JSON. Additive and documented, harmless for Plan 4's consumer, noted only so it is a conscious part of the plan wire format going forward."

---

Notes on scope boundaries observed:
- IDEAS.md #5-7 (zero-track options, un-dispositioned parity extras, run-log prune) landed 2026-07-11 (commits 5659dad/2ca5ddd), NOT E4 - excluded.
- The memo's 2026-07-11 wire-format addendum (commit 88484ed) is a later-era reinforcement of record 47; the E4 occurrence is the whole-branch verdict.
- "auto-title" appears in the journal's shelved list but its IDEAS entry (#6, "Output naming from title, never tiered") is 2026-07-11 - no groundable E4 disposition, excluded to avoid mis-grounding.
- Task 4's `zz`/`zzz`->`notalanguage` fixture fix (record 18) and Task 5's doc-comment correction (record 41) are the two implementer/plan-origin catches the E4 review chain surfaced.
