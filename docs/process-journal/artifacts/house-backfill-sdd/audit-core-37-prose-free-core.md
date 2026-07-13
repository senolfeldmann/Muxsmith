# Audit: core-37-prose-free-core (PROMOTION candidate)

**Cluster:** `core-37-prose-free-core` - "core emits code+params only, text lives outside"
**Claimed count:** 6 | **promoted:** true | **status:** settled
**Audit date:** 2026-07-13 | **Method:** each occurrence's cited ref opened in the repo and checked against the claim "this (topic, approach) arose here as {kind}".

**Verdict: CONFIRMED** - 6 of 6 occurrences survive as genuine, distinct, on-topic attestation points. Promotion stands. Even under the most aggressive defensible re-count (collapsing find/fix, see caveat) the pattern never drops below 4 and spans 3+ plans, so promotion is robust under every counting rule.

---

## Statement under audit

> `muxsmith-core` emits a diagnostic code + structured params only; all human text lives in the Fluent catalog / an injected `DiagnosticRenderer` port; core never originates, hardcodes or localizes prose. Recurring enforcement: bug K (identification failure misreported as MissingTrack with an authored English string) fixed to emit `UnidentifiableSource` carrying only the third-party error text (pass-through allowed).

## Counting rule invoked by the cluster (cluster-core.md line 5)

> An occurrence = one distinct cited artifact/attestation point ... **Kept distinct: a fix commit that follows a review is its own touchpoint (find-vs-fix are two events).**

This rule is declared once and applied uniformly across the file (core-2, the suggestion-cap cluster, the bug-C/-D/-I clusters, etc. all split find and fix). It is a disclosed, systematic convention, not padding invented for core-37. The audit accepts it but flags its effect below.

---

## Per-occurrence findings

### 1. 2026-07-09 violated-corrected - "independent review bug K" - SURVIVES
- **Artifact:** `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`
- **Evidence:** line 28 item K: "Identification failure misreported as MissingTrack/MissingExternal with an authored English string in `detail` (prose-free-core violation; detail dropped anyway)." Corroborated by Reviewer 4 (line 82, CONFIRMED, planner.rs:244-258/309-327, "prose-free-core violation") and Reviewer 3 (line 60, "[K]").
- **Judgment:** Real artifact, exactly on-topic, explicitly named a prose-free-core violation. This is the find half of the bug-K enforcement.

### 2. 2026-07-09 violated-corrected - "F5 fix (commits 0e141d1/6f475b3)" - SURVIVES
- **Artifact:** git commits `0e141d1` ("fix(core): planner emits UnidentifiableSource and protects donor paths") and `6f475b3` ("fix(core): SourceOverwrite batch-wide, Display for IdentifyError"). Both present in history, both real.
- **Evidence:** 0e141d1 replaces MissingTrack/MissingExternal with `UnidentifiableSource` carrying the underlying IdentifyError in `detail`; 6f475b3 adds `Display for IdentifyError` so `detail` carries a terse phrase instead of raw Debug. `plan-2-fixes.md` F5 (lines 33-34) confirms F5 = fix for "bugs H, **K**, #5" with "third-party pass-through, allowed".
- **Judgment:** Real, on-topic, the fix half of bug K. Distinct artifact from #1 (a review event vs a commit-set), kept distinct per the line-5 rule.

### 3. 2026-07-09 deferred - "FINAL review M3 + F5-review Minor #4 (residual English framing accepted)" - SURVIVES
- **Artifacts:** `plan-2-fixes-sdd/FINAL-review.md` (M3) and `plan-2-fixes-sdd/F5-review.md` (Minor #4).
- **Evidence:** FINAL-review M3 (lines 120-132): "core-authored, unlocalizable prose ... F5 improved it ... but did not fully eliminate core-authored English ... Accept; be aware `detail` will always be English." F5-review Minor #4 (lines 153-172): raw `{e:?}` Debug interpolated into a Fluent message, "Not blocking".
- **Judgment:** Genuine deferral (a residual carve-out deliberately accepted), materially different lifecycle action from the find/fix of #1/#2. Two review events merged into one occurrence here = conservative (under-count, not over-count).

### 4. 2026-07-09 reinforced - "Plan 3 Global Constraints" - SURVIVES
- **Artifact:** `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md` line 16 (under "## Global Constraints").
- **Evidence:** "**Core is prose-free:** `muxsmith-core` emits diagnostic `code` + `params` only; no user-facing strings."
- **Judgment:** Real, verbatim restatement of the pattern in a later plan doc. Legitimate "reinforced". Distinct plan/era from Plan 2.

### 5. 2026-07-10 decided - "task-2 verdict (DiagnosticRenderer port/adapter)" - SURVIVES
- **Artifact:** `docs/process-journal/artifacts/plan-5-sdd/verdicts/task-2-review-verdict.md` (final_message_ts 2026-07-10T11:48:59Z - date matches).
- **Evidence:** Design-deviation section (c), line 50: "core never originates, hardcodes, or has i18n/localization knowledge of that text - it only reserves a named slot (`"rendered"`) for content the caller's adapter supplies, the standard hexagonal port/adapter shape." Assessment: Approved; the `DiagnosticRenderer` trait is "the correct and essentially forced resolution".
- **Judgment:** Real, on-topic; establishes the `DiagnosticRenderer` port/adapter as the compliant mechanism for the prose-free boundary (a new approach-facet within the pattern). Minor nuance: the verdict recommends + flags for owner sign-off rather than unilaterally deciding, so "decided" is slightly loose - but consistent with how the file labels design-establishing verdicts (e.g. line 171), and the occurrence survives regardless of decided-vs-reinforced. Distinct plan (Plan 5).

### 6. 2026-07-12 reinforced - "CONVENTIONS.md Patterns" - SURVIVES
- **Artifact:** `docs/CONVENTIONS.md`, "## Patterns" section (committed b38a46f, 2026-07-12).
- **Evidence:** "**Diagnostics through the catalog.** User-facing diagnostics are constructed as `Diagnostic`/`DiagCode` and rendered via the Fluent catalog; never format a diagnostic string inline."
- **Judgment:** Real, on-topic restatement in the standing conventions doc. Legitimate "reinforced". Distinct artifact/era.

---

## Duplication / fabrication check

- **Fabricated:** none. All six refs resolve to real artifacts (two of them git commits verified in history) that mention the topic.
- **Misattributed:** none serious. #5's "decided" is mildly loose (a verdict recommending + flagging sign-off), but it neither inflates nor changes the surviving count.
- **Duplicate (same artifact cited twice):** none. #1 (review doc) and #2 (commit-set) are different artifacts; #3 merges two review events into one (conservative). No occurrence is the same attestation point double-booked.

## Caveat: find-vs-fix counting

The count of 6 relies on the cluster's declared "find-vs-fix are two events" rule (#1 = review finds bug K, #2 = commits fix bug K). This rule is disclosed and applied uniformly across the whole cluster file, so it is not a per-cluster manipulation. Still, if one imposed a stricter "collapse find+fix into one enforcement instance" rule:

- Collapse #1+#2 -> **5** distinct occurrences.
- Collapse the entire bug-K thread #1+#2+#3 -> **4** distinct occurrences (bug-K thread; Plan 3; task-2 verdict; CONVENTIONS.md).

Both remain >= 3, and the surviving occurrences span **Plan 2, Plan 3, Plan 5, and the standing CONVENTIONS.md** - i.e. the boundary genuinely recurred across the project timeline, which is the substantive test for a standing convention. The promotion is robust under every defensible counting rule; this is not one bug counted six ways.

## Result

- **verified_count = 6** (all occurrences survive under the cluster's declared counting rule).
- **Verdict: CONFIRMED** (>= 3 survive; promotion stands).
