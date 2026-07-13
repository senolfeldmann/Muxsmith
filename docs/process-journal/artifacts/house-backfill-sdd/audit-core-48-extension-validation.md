# Audit: core-48-extension-validation (PROMOTION candidate)

- **Cluster id:** core-48-extension-validation
- **Kind:** pattern / domain: core
- **Claimed count:** 3 | **promoted:** true | **promoted_at:** 3
- **Verdict:** CONFIRMED (3/3 occurrences survive; promotion stands)
- **Audited:** 2026-07-13, against `/home/senol/Git/Muxsmith`

## Statement under audit

> Profile and locator extensions are validated against mkvmerge `--list-types`
> once per batch (not per file), degrading to a warning when mkvmerge is absent,
> mirroring the language-validation walk; new UnknownExtension warning. Deferred
> without a diag code in Plan 2, built for profile extensions at T5 and locator
> extensions (recursive walk reusing validate_extension_values) at T5.9.

## Occurrence-by-occurrence verification

### Occ 1 — deferred, "journal Plan 2 (Open threads, no diag code yet)", 2026-07-09

**Artifact:** `docs/process-journal.md`, Plan 2 entry header line 115
(`## 2026-07-09 | Plan 2 written and implemented | session 2`), **Open threads**
block lines 173-178. Verbatim:

> Deferred: OverlappingRules auto-suggestions and the no-single-fix partition
> report (D6 remainder); Plan 3 (attachments/chapters/tags/title, command
> generation, executor); `--list-types` extension validation (no diag code yet);
> CI does not install mkvtoolnix so the gated tests self-skip there.

**Supports the ref exactly.** Right section (Plan 2 Open threads), right date
(2026-07-09), right topic (`--list-types` extension validation), right kind
(deferred), and the "(no diag code yet)" parenthetical in the ref is a verbatim
quote from the journal. **SURVIVES.**

### Occ 2 — decided, "task-5 (profile extensions)", 2026-07-11

**Artifacts:** `.superpowers/sdd/plan-5.5/task-5-{brief,report,verdict}.md`
(the plan-5.5 task-5 set, dated Jul 11; not the stale Plan-4 top-level `sdd/task-5-*`
dated Jul 10). Verdict header: `Task 5 reviewer verdict (model: opus, 2026-07-11)`.

- Report: `profile.input.extensions` now checked **once per batch** against local
  mkvmerge `--list-types`; unmatched entry raises warning-severity
  `UnknownExtension` naming the extension; degrades when runtime unavailable;
  `validate_extension_values` signature deliberately "mirrors
  `validate_language_values`'s shape".
- Brief Step 2 verbatim: "once per batch (not per file), degrade-with-warning when
  mkvmerge is absent - same pattern as language validation".
- Report/brief introduce the new `DiagCode::UnknownExtension` (warning) + Fluent
  message.
- Verdict: "✅ on all brief points ... once-per-batch semantics ... DiagCode
  UnknownExtension warning ... Task quality: Approved."

Every load-bearing element of the statement (once-per-batch, mirrors language
walk, new UnknownExtension warning, profile extensions) is present as a *decided
and built* event. **SURVIVES.**

### Occ 3 — decided, "task-5.9 (locator extensions)", 2026-07-11

**Artifacts:** `.superpowers/sdd/plan-5.5/task-5.9-{brief,report,verdict}.md`.
Verdict header: `Task 5.9 reviewer verdict (model: sonnet, 2026-07-11)`.

- Brief: extend the T5 batch walk to locator extensions (spec §4.6) via a
  "recursive walk over track rules / chapters / attachments `add` entries,
  mirror `walk_exact_languages`' traversal", "reusing T5's
  `validate_extension_values` core".
- Report: `validate_extension_values` now also walks the three locator positions
  (`tracks.rules[i].source` External, `chapters` External, `attachments.rules[i].add`);
  the per-item loop was extracted into a shared `validate_extension_list` helper so
  the comparison lives in one place. Same `UnknownExtension` warning, same
  once-per-batch/no-dedup semantics as T5. 4 new tests, full gate green.
- Verdict: position-completeness verified against the only Locator field sites;
  "Once-per-batch preserved ... consistent with T5 and the language walk";
  "Spec compliance ✅. Task quality: Approved."

Distinct scope from Occ 2 (locator vs profile extensions), separately routed by
the T5 review, separately built. Supports "locator extensions (recursive walk
reusing validate_extension_values) at T5.9" as a *decided* event. **SURVIVES.**

## Distinctness / duplication check

Three distinct artifacts, three distinct events, no duplication:
- Occ 1 = a deferral (journal), 2026-07-09.
- Occ 2 = a build decision for the profile-extensions half, 2026-07-11.
- Occ 3 = a build decision for the locator-extensions half (routed *by* the T5
  review as materially distinct recursive-walk work), 2026-07-11.

The two "decided" occurrences are not the same decision re-counted: T5's own
review (`plan-5.5/task-5-verdict.md`, "Locator.extensions verdict ... ROUTED: new
plan Task 5.9") explicitly splits the locator work off as a separate task with its
own walk and tests. Genuine recurrence of one pattern across profile and locator
surfaces, not an inflated count.

## Notes (do not affect the verdict)

Two wording imprecisions in the promotion statement, worth flagging for when the
convention text is finalized, neither of which invalidates an occurrence:

1. **"degrading to a warning when mkvmerge is absent"** overstates the runtime
   behavior. The T5 report and verdict are explicit that the *in-plan* degrade on
   a missing/failed `list_types` is a **silent no-op**, not a warning; a truly
   absent mkvmerge is caught upstream (dry_run exit 2), and the `None` path only
   fires when `list_types` fails after `list_languages` succeeded. The
   "degrade-with-warning" phrasing traces to the T5 brief's own loose wording, so
   this is a drafting carry-over, not a fabrication.
2. **"recursive walk reusing validate_extension_values"** is slightly loose:
   T5.9 extracted the per-item logic into a new `validate_extension_list` helper;
   `validate_extension_values` is the batch entry point that drives the recursive
   locator walk and calls that helper per hit. Substance holds.

## Conclusion

verified_count = **3** (all occurrences survive; none fabricated, misattributed,
or duplicated). >=3 survive -> **CONFIRMED**. The promotion to standing
house-knowledge stands; its recurrence count is real.
