# Plan 7.5 design review, round 1

Artifact: `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`
(commit e662994). Reviewed against the controller brief, spec
§8.2/8.3/5.2/4.5, the Tier-2 house files, the current tree, and my own
foreground re-runs of every empirical claim (debug CLI, greps with
positive controls, mkvtoolnix source at ~/Downloads/mkvtoolnix).

## VERDICT: NEEDS FIXES

Minor-only. No owner ruling drifted, no fork survives into planning, every
empirical claim reproduced byte-exactly on re-run. The five findings are
one small edit batch; none reopens a decision.

---

## Findings

### Minor

**F1 - misattributed quote location (citation class:
code-comment-line-citations-drift).**
Artifact: design D71 constraint 3 and trigger 6. The quoted closure
sentence "adding or dropping a member is an owner change, not an
implementation nicety" is attributed to D54 ("D54's own closure
sentence"). It lives in the plan-7 design's §9 "What the implementer must
not decide" (2026-07-21-plan7-help-i18n-design.md:1980-1981: "The
annotated set is D54's tables, verbatim - 22 ids, 44 files; adding or
dropping a member is an owner change, not an implementation nicety.").
D54's own closure sentence reads "the id set, file set and host elements
above are closed here." Substance identical - both close the set - but a
quote is a claim about wording AND location. Change: re-point both sites
to the plan-7 design §9 bullet, or quote D54's actual sentence.

**F2 - ground-truth inaccuracy about the shipped delegation.**
Artifact: design section 1, "Help-mode delegation mechanics": "All
help-mode handlers are capture-phase listeners on `<main>`." The keydown
handler registers on `document`, capture phase
(`document.addEventListener("keydown", onHelpKeydown, true)` in App.vue's
`watch(helpMode)` block); mouseover/focusin/click/dragstart are on
`<main>`. D71's conclusions hold unchanged (still capture-phase, still
intercepts Enter/Space before button activation - I verified the
mechanics). Change: correct the sentence.

**F3 - D71 constraint 2 overstates the Tier-1 entry's reach.**
Artifact: design D71, "Why dedicated help-ids are structurally ruled
out", constraint 2. `gui-helpid-equals-labelkey`
(decision-ledger.yaml:3781) scopes the identity to "a REGISTRY control's
help-id IS its labelKey"; a sanctioned non-labelKey help-id class already
exists - `view-batch`/`view-jobs`/`view-editor` and
`batch-suggestion-card` are help-ids and none is a catalog id (verified:
grep over locales/en returns nothing). A dedicated template
`data-help-id` on the bespoke buttons would therefore not be "an
exception to the identity scheme" needing a Tier-1 owner change; what
actually rules the route out is constraint 3 (D54's owner-closed id/host
set plus the content criterion), which I verified stands. Change: reword
constraint 2; the rejected option's steelman should cite the existing
non-labelKey class as its strongest form. Decision unchanged.

**F4 - test-plan case 9 names a nonexistent structure and weakens the
house counterpart shape.**
Artifact: design section 5, case 9. "additive test in
`e2e/help-mode.spec.ts`, its editor describe" - no editor describe
exists; the file's describes are "help mode (D52)" (:95), "help mode
annotations (D54)" (:186), "help mode drag suppression (I1)" (:395). An
implementer must invent the placement (proc-latitude omission form,
mild). Second, the non-vacuity counterpart is delegated to cases 1/4,
which run in the mount harness, a different mounting than case 9's real
app; the direct sibling (help-mode.spec.ts:396, "a drag-reorder mutates
the rule grid outside help mode but is suppressed inside it") carries
the mutates-outside/suppressed-inside counterpart in the SAME test and
harness. Change: name the target describe and give case 9 the sibling's
in-test counterpart shape (or record why the cross-harness counterpart
suffices).

**F5 - amendment 2's selection criterion also reaches `NoTrackRules`,
unstated.**
Artifact: design section 4, amendment 2. Spec 5.2's table lacks
`NoTrackRules` too (it appears only in §4.5 prose: "Empty rules under
`drop` remain a `NoTrackRules` error"), and this design depends on it
exactly as it depends on `EmptyMatchExpression` (D69's zero-rule
surfacing; e2e case 7 mocks it). Overall the 5.2 table omits roughly 17
of `diag_codes!`'s members (parse-error, empty-extensions,
provable-overlap, the template family, ...; report/mod.rs), so "add the
missing row" reads as if the table were otherwise complete. The honest
distinguisher exists - `EmptyMatchExpression` appeared NOWHERE in the
spec while `NoTrackRules` is spec'd with severity and condition in 4.5 -
but the document does not state it. Change: one scoping sentence in
amendment 2 (table not exhaustive; this row lands because the design's
guidance mechanism cites a code with zero spec presence), or add the
`NoTrackRules` row as well.

### Nits (recorded, no fix owed)

- The quoted skeleton diagnostic JSON omits the emitted `rendered`
  member (the design quotes the rendered text separately in the next
  clause); every quoted value byte-matches my run.
- Amendment 1's "legal down to zero rules per 4.5" compresses the
  keep/drop asymmetry into the 4.5 pointer; 4.5 resolves it.
- D72 point 3's "cannot exist on a shared key" is strictly
  "possible but dishonest" (one site could bind the shared attribute
  alone); the latent-coupling and anti-inflation rationale carries the
  conclusion regardless.

---

## Re-run record (what I verified myself)

**Empirical claims (dimension 4) - all reproduce byte-exactly** against
`target/debug/muxsmith` (built 2026-07-21, predates the design):

- `rules: [ { match: {} } ]` -> exactly one diagnostic,
  `{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"severity":"warning"}`,
  rendered "This match expression is empty and would match every
  track.", exit 1.
- Skeleton beside `exact: { language: de }` -> same single warning at
  `tracks[1].match`, nothing else (no ProvableOverlap;
  `lint.rs::is_exact_only`'s guard byte-matches the design's quote).
- `rules: []` + default `drop` -> `no-track-rules` error at
  `tracks.rules`, rendered text byte-matches, exit 2.
- `rules: []` + `keep` -> `passthrough-profile` info, rendered text
  byte-matches, exit 0.

**Fire-verified negatives - re-fired with controls:** focus grep empty
over src/ + control file fires; no `.tooltip` on either action key in
either locale + positive control `editor-tracks-rules` at en:100-101
(exact lines); 46/46 gui-editor.ftl ids (en/de); `EmptyMatchExpression`
absent from the whole v1 spec, present at `locales/en/diagnostics.ftl:9`
and the plan-7 D57 emission table (:202).

**mkvtoolnix (SI-3), source at v100.0 (NEWS.md header verified):**
attachments.cpp has exactly 1 `setToolTip` at :490 ("Right-click for
attachment actions", byte-match); output.cpp 26; merge/ total 62 (D72's
"~62"); 0 of merge/*.h contain "rule", 12 contain "track";
`onRemoveAttachments` -> `removeSelectedAttachments(...selection())`
with no dialog (:338-341); `deletePressed` connect (:95);
`removeAttachmentsAction->setEnabled(hasSelection)` (:424-430); menu
order add-before-remove (:43-45); selection -> detail form +
`enableAttachmentControls(false)` (:366-395); move buttons (:99-100) and
Ctrl+Up/Down via `ctrlUpPressed`/`ctrlDownPressed`
(basic_tree_view.cpp keyPressEvent, attachments.cpp:92-93); add opens a
file dialog (`Util::getOpenFileNames`, :329). `m_muxThis` in
merge/track.h:33. Licensing boundary respected: quotes are evidentiary,
nothing adopted into product text.

**Rulings (dimension 1):** D65-D69 match the ROADMAP S22 KICKOFF block's
substance with no drift; each D-entry separates "Decision (ruling,
binding)" from "Resolved on top of the ruling". The
warning-not-error precision in D65 is surfaced honestly (section 0), does
not modify the ruling, and is pinned by trigger 4 + e2e case 6's
Save-enabled assertion. Every brief must-RESOLVE item is resolved
(placement/disabled state D70, skeleton D65, help-id D71, tooltips D72,
panel/focus D67, help-mode D71, diagnostics landing D65/D69, e2e +
ripple section 5, amendment section 4, wire surface section 3). Open
items: none, verified.

**Brief-correction audit (dimension 2):** section 0's verifications all
re-ran true (T10 finding-1 quote verbatim at task-10-verdict.md:53;
exactly ListWidget.vue + PropertyMapWidget.vue consume `editor-action-*`;
46/46; keys tooltip-less with positive control; boundary entries read as
summarized). "No correction needed" is itself correct.

**Latitude (dimension 3), both forms:** no explicit-permission clause
anywhere. Omission scan over every normative sentence: the change
inventory is a closed enumeration ("Anything not listed is not
touched"), D67/D70 shapes are verbatim code/template blocks, D71's topic
claims are enumerated with the deferred final wording routed through the
standing owner rendered-surface pass (sanctioned carve-out, correctly
cited). The single omission hit is F4's describe placement. Test fixture
VALUES (rule counts, model contents) are implementation-authored per the
house's inline-profile pattern with the design pinning the assertions
including anti-vacuity controls - judged in-altitude, see HARVEST.

**House conformance (dimension 6):** D62 gate mechanics verified at
scripts/check-i18n.mjs - the help-id set derives from `helpId:` registry
literals + `data-help-id` template literals + VIEW_TOPICS, so help-id-less
buttons are invisible to it; zero new ids + zero new topics -> gate green
by construction; URL/pipe/raw-HTML bans confirmed (checks 4-6) over the
two edited topic files. 46-budget claim counted true.
`help-mode-suppression-pointer-scope` conformance verified against the
shipped App.vue delegation (capture click preventDefault+stopPropagation
kills `@click`; Enter/Space keydown interception pre-empts native button
activation; `setHelpClass` is querySelector-first-match, confirming the
duplicate-host rejection). No h1 touched; scheme unaffected.

**No-work-needed passages (dimension 7) - premises run, not weighed:**
D72's obviousness premise checked against spec 8.3's own sentence
("every non-obvious control carries a tooltip", byte-verified) with all
runnable sub-premises re-run true (tooltip-less at both existing sites
via owner-ruled D54 exclusion row - byte-matched "generic list actions /
presentation column; no per-instance content"; mkvtoolnix counts above);
"zero new ids/topics" verified against the gate mechanics; "no IPC/wire
change" verified - add/remove are model swaps identical in kind to the
shipped `onDrop`/`setRuleValue`, the only wire crossings are the existing
`validateProfileModel`/`saveProfile`, and the skeleton's serde round-trip
(`- match: {}`) is attribute-verified (model.rs TrackRule +
match_expr.rs all-`skip_serializing_if` fields) and load-verified by my
fixture runs.

**ADR quality (dimension 8):** all eight entries carry
decision/rationale/rejected-with-steelman; steelmen are genuine (D66's
dialog steelman, D70's per-row steelman, D68 citing the boundary entry's
own latent-coupling steelman against itself). Triggers 1-6 named and
mirrorable. Numbering D65-D72, last prior ADR is D64 (swept `^## D`
myself), D75+ untouched.

**Spec amendment (dimension 9):** amendment 1's replace-target
byte-matches the current 8.2 view-1 sentence (spec :374); the splice is
clean; grep confirms 8.2's sentence is the spec's only rule-affordance
description; §11 carries no add/remove claim; 4.5 already carries D69's
semantics. Amendment 2's insertion anchor (`EmptyMatchList` row) exists;
the row's suppression clause matches validate.rs's comment and
`empty_list_here` guard. Residue: F5.

**e2e sanity (dimension 10):** the nine cases are implementable as named
- EditorView mount-harness precedent (smoke.spec.ts :941/:1007/:1332),
`installTauriMocks` + inline-profile pattern (editor-markers.spec.ts),
`data-diag-path` markers shipped in all widgets incl. SectionWidget's
legend (case 6's panel-marker assertion lands). Collision check: every
existing `editor-action-*` role-name selector sits in a standalone
widget mount (smoke.spec.ts :877/:883/:894/:899/:1069 - PropertyMap/
ListWidget mounts), never a full-EditorView mount, so the new same-name
buttons break no existing selector; Task 11/13b selectors are scoped as
the design claims. D62/D55 ripple enumeration is complete and correct.
Residue: F4.

---

## HARVEST

- **Dominant pattern: empirical discipline at its best so far.** Every
  load-bearing claim reproduced on the first re-run attempt - byte-exact
  rendered strings, exact counts (26/62/0/12 mkvtoolnix, 46/46 catalog),
  exit codes - and every absence claim ships with a named positive
  control that fires. The one-command-per-claim reproducibility made
  this review cheap; worth naming as the standard the next design is
  held to.
- **Repeated rejection shape:** "the shipped house pattern is the
  argument" (zero-focus-management precedent, ListWidget placement,
  D54 closure, `onDrop`'s selection-clearing rationale reused for
  removal). Consistently anchored; no strawman found.
- **Citation habit:** symbol anchors dominate; both bare line spans
  (en:100-101, attachments.cpp:490) verified true. The one citation
  defect (F1) is misattributed location, not fabrication - the
  drift-class the house entry predicts for quotes copied without
  re-opening the host.
- **Over-restriction flags: none.** No stop was forced by
  proc-latitude-clause-boundary or the carve-out; nothing this boundary
  blocked should have been covered. The fixture-values-at-implementation
  altitude (inline models authored in the spec file under
  design-pinned assertions with anti-vacuity controls) produced no
  latitude breach; no rule change proposed, but it is the pattern to
  watch if a future design pins assertions less tightly.
- **Controller watch item (outside this plan):** spec 5.2's table
  silently omits ~17 `diag_codes!` members. If the table is the catalog
  of record, that staleness wants a wholesale amendment batch, not one
  row per plan (F5 is its local symptom).

---

# Delta review (round 2)

Delta under review: commit 0d8f6d7 (design doc + ROADMAP watch-item
tightening), against my five round-1 findings. Same standards; every
load-bearing author claim re-run, not believed.

## VERDICT: APPROVED

## Per-finding disposition

**F1 - RESOLVED.** Both sites re-pointed correctly. D54's actual closure
sentence is quoted byte-exact: "the id set, file set and host elements
above are closed here" (verified wrap-tolerant against the D54 section
tail; the sentence spans a line break, which defeats a naive line grep -
my first check false-negatived and the multiline re-run fired). The
restatement "adding or dropping a member is an owner change, not an
implementation nicety" is now attributed to the plan-7 design's §9
annotated-set bullet - §9 heading confirmed at :1961, the bullet's
wording byte-matches (:1980-1981). Trigger 6 carries the same corrected
attribution.

**F2 - RESOLVED.** The corrected sentence matches the shipped
`watch(helpMode)` block exactly: mouseover/focusin/click/dragstart
register on `<main>` (App.vue :101-104), keydown on `document` (:105),
all capture-phase, both registrations inside the watch block as the fix
states.

**F3 - RESOLVED.** Constraint 2 now correctly scopes
`gui-helpid-equals-labelkey` to registry controls and grounds the
non-labelKey route on the sanctioned class. Its verification claim
re-run: `view-batch`/`view-jobs`/`view-editor`/`batch-suggestion-card`
return nothing as anchored ids over `locales/en/*.ftl` (exit 1) while
`editor-tracks-rules` fires as the positive control. The rejected
option's steelman now leads with its strongest form (the non-labelKey
class precedent), and the conclusion correctly lands on the owner-closed
D54 set as the real blocker. Heading updated consistently ("both routes
walked, each closed").

**F4 - RESOLVED, and strengthened beyond the asked fix.** Case 9 is
homed at `test.describe("help mode (D52)")` - the describe name
byte-exists (e2e/help-mode.spec.ts:95, `test.describe("help mode
(D52)", () => {`), and the fix's enumeration of the file's three
describes matches the tree. The counterpart is now in-test and
in-harness per the I1 sibling (title byte-matches :396). The upgrade -
both channels asserted mutating outside help mode, and Add rather than
Remove carrying the assertions because a disabled Remove would make the
suppression check vacuous - is sound reasoning of exactly the house's
anti-vacuity class, and mechanically correct (Remove disables at
`selectedIndex === null`).

**F5 - RESOLVED.** The measurement re-run confirms the fix's figures
exactly: `diag_codes!` carries 47 members (grep count over
report/mod.rs), the spec 5.2 table 30 rows, difference 17 - "17 of 47"
is measured, not estimated. All named absentees confirmed absent from
the table (ParseError, EmptyExtensions, ProvableOverlap, the four-member
template family, NoTrackRules). The §4.5 quote "Empty rules under
`drop` remain a `NoTrackRules` error" byte-matches spec :180. The
distinguisher (zero spec presence vs spec'd-in-prose) is now stated. The
ROADMAP watch item's tightened figure is the same verified measurement.

## New findings

None. The diff touches only the five findings' sites plus the ROADMAP
watch item; no passed dimension is disturbed - no new latitude (case 9
is now more closed than before), no ruling drift, no new empirical
claim left unverified.

One observation, no fix owed: amendment 2's scoping paragraph cites
`.superpowers/sdd/plan-7.5/design-review-round-1.md` (this file), which
is gitignored and currently uncommitted. This follows the ruled house
pattern (ROADMAP: the round-8 adjudication - `.superpowers/sdd`
citations in a committed design are correct at commit time and must
move WITH the plan-close SDD salvage in the same change; plan-7's three
were re-pointed exactly so). Controller duty at plan-7.5 close: include
this citation in the salvage re-pointing.

## HARVEST additions

- **Fix-round quality:** each fix carried its own verification claim
  (the grep with positive control, the measured 17-of-47, the byte-
  quoted describes), and every one reproduced on re-run. A fix round
  that ships its evidence inline is materially cheaper to delta-review;
  worth keeping as the standard.
- **Reviewer-side lesson (mine):** the D54 closure sentence
  false-negatived under a line-based grep because prose wraps; a quote
  check against wrapped Markdown needs a whitespace-tolerant pattern
  before "not found" is trusted - the make-the-check-fire discipline
  applies to the reviewer's own tooling too.

---

# Delta review (round 3): case-9 witness amendment

Delta under review: commit 89782cd (design doc only, 124/5) - the
post-T2 case-9 event-level witness amendment. Judged against the four
dispatch questions; every load-bearing claim re-run at the tree.

## VERDICT: NEEDS FIXES

One minor finding (a false date, two sites); everything substantive is
verified correct. Single-token fix, no re-design.

## (1) Witness design - load-bearing claims verified

- **`attemptDrag` precedent:** module-level in e2e/help-mode.spec.ts
  (:68-90, beside the doc comment), synthetic `DragEvent` dispatch with
  `{ bubbles: true, cancelable: true }`, `defaultPrevented` read - the
  exact shape the helper copies. The amendment's "No rAF flush (unlike
  `attemptDrag`)" note is accurate and correctly reasoned: attemptDrag
  double-rAFs (:88) because it asserts rendered DOM afterward; the probe
  reads event state synchronously. `import type { Page }` exists (:24),
  so the helper signature compiles as written.
- **Sole keydown listener:** grep over `src/` re-run in BOTH trees
  (main and .worktrees/plan75-a): exactly App.vue:105/:111 - the
  `onHelpKeydown` add/remove pair, capture-phase on `document`, inside
  the `watch(helpMode)` block. The broad pattern ("keydown") also
  covers `@keydown` template handlers and fires on the App.vue
  registrations, which are its own positive control.
- **Escape branch:** re-read (App.vue :81-89) - settings-dialog check
  then return, `helpMode = false` then return; no `preventDefault`. So
  a `defaultPrevented` Enter keydown is attributable to exactly the
  Enter/Space branch, as claimed.
- **`cancelable: true` load-bearing:** correct per DOM semantics -
  `preventDefault()` on a non-cancelable event is a no-op and
  `defaultPrevented` stays false, making the inside-phase witness
  structurally red on correct code. Recorded in the design, as it
  should be.
- **Probe side effects:** the untrusted dispatch runs no activation
  behavior (keydown->click synthesis is gated on trusted events), and
  the design TESTS that premise with the outside-phase count guard
  rather than assuming it - the right hygiene. The inside-phase pin
  side effect is recorded and harmless: verified against the landed
  test (plan75-a :186-252), whose keyboard half ends on the count
  assertion with no pin/sidebar read after the probe's insertion point.
- **T2 evidence borrowed correctly:** task-2-verdict.md confirms every
  cited item - rounds A-D (round B: preventDefault/stopPropagation
  removed, full suite `62 passed`; round B2: entire branch removed,
  help-mode `9 passed`, suite `62 passed`; round D: pin removed from
  onHelpClick, PASSED -> M1), Q1's over-determination analysis, M2's
  refuted separate guard. The ledger entry
  `redundant-layers-need-mechanism-witness` exists
  (decision-ledger.yaml:4266, tier 1, occurrence dated 2026-07-27).
  The M1-scoped rewording of the click half ("topic identity only, NOT
  pin evidence") matches round D exactly.

## (2) Acceptance criterion - falsifiable and provable as stated

Yes. Both neutralization shapes are concrete and deterministic; under
either, nothing in `src/` calls `preventDefault` on a keydown (sole
listener, Escape branch clean), so the inside-phase `toBe(true)` fails
by construction, while the landed assertions' greenness under exactly
these two mutations is what T2 rounds B/B2 already measured at the same
tree (62 passed both times). The outside-`false` / inside-`true` pair
mirrors the I1 sibling's shipped discriminating pair byte-for-byte
(help-mode.spec.ts: `outside.dragstartPrevented).toBe(false)` /
`inside...toBe(true)`). The specified fire-test (neutralize both ways ->
witness fails alone; restore -> full file passes) is executable by the
fix-round implementer without interpretation.

## (3) Minimality - confirmed

Three hunks, all declared: the D71 keydown bullet's redundancy clause,
the case-9 block (M1 rewording + witness extension), the amendment log.
No rider content. The dependent-sentence sweep's
"verified-unaffected" list checks out: section 8 still truthfully says
nine cases in two files (the witness is inside case 9), the
zero-production-code enumeration is untouched (test-side only), the
gate ripple is unchanged, trigger 4 unrelated. D71's closing outcome
sentence remains true (T2 re-verified the outcome; only attribution
changed).

## (4) Latitude scan - clean

No explicit-permission clause in the new text. No omission latitude:
helper code closed verbatim, both insertion points fixed relative to
landed assertions, side effects enumerated, acceptance criterion names
its two mutation shapes and both expected outcomes. Nothing is left for
the implementer to invent.

## Findings

**F6 (minor) - the amendment's self-date is false, two sites.**
"Amendment 1 (mid-run, 2026-07-22, post-T2)" (amendment log header) and
"(amendment 1, 2026-07-22, post-T2; controller ruling ...)" (case-9
witness block). The amendment happened 2026-07-27: T2's commit 92ba1e7
landed 2026-07-27 11:55, the T2 verdict was written 12:11, commit
89782cd is authored 2026-07-27 12:20, and the ledger occurrence the
amendment cites is itself dated 2026-07-27. "2026-07-22, post-T2" is
internally impossible (the plan is dated 2026-07-23; T2 cannot precede
its plan). The date was evidently copied from the document's title
date. A date in a durable artifact is a number claim: change both sites
to 2026-07-27.

## HARVEST additions

- **The witness design is the strongest test artifact in this series:**
  it discriminates a mechanism behind a masking redundant layer, tests
  its own untrusted-dispatch premise instead of assuming it, records
  its side effects, and ships a falsifiable acceptance criterion whose
  failure mode T2 already demonstrated empirically. Worth citing as the
  reference shape the new ledger entry's "event-level witness per
  layer" clause points to.
- **Controller reminder:** T2's recommended Medium comment correction
  (the landed click-half comment still claims "the pinned topic is this
  half's evidence that the listener actually handled the click",
  refuted by M1/round D) should ride the same fix-round dispatch as the
  witness, so the comment and the design's corrected scoping land
  together.
- **The date-copying slip (F6)** is the same class as round-1 F1
  (provenance copied instead of established) one level up: title dates
  propagate into amendment stamps the way quoted sentences propagate
  into citations. A mid-run amendment stamp is authored at amendment
  time, not inherited from the document header.

**F6 disposition (scoped re-review of commit e525813): RESOLVED -
round 3 now APPROVED.** The delta is exactly the two self-dating sites
(:924 witness-extension header, :1110 amendment-log header), both now
2026-07-27; my own grep re-run confirms six remaining `2026-07-22` hits
(:3, :6, :20, :45, :113, :826), each a legitimate creation-era
reference (doc status, D-number collision check, S22 owner rulings,
design-time empirical runs, the F5 measurement made in the 07-22 fix
round) - no third self-dating site exists.
