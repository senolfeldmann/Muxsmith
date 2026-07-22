# Task 21 verdict: v1-spec amendments (design section 6)

Reviewer: independent, fresh eyes. Working tree `/home/senol/Git/Muxsmith`,
master, HEAD `4ac8d8b`, parent `e4df011`. Single commit, spec file only.

## Combined verdict: NEEDS FIXES

Both fixes are controller-level (brief-authoring), not implementer execution
errors. The implementer's execution against its brief is exemplary; the
deliverable nonetheless carries one design-mandated clause the brief dropped.

- **Spec-compliance verdict: NEEDS FIXES.** All six enumerated amendments plus
  the row-29 alignment landed byte-perfect against the brief, and nothing else
  changed. But the brief itself truncated design amendment 6(b): it dropped the
  clause the design directed to land "unchanged" ("in which case the hover state
  resets too and the sidebar shows the new view's topic"), so the landed spec is
  not fully compliant with design section 6.
- **Quality verdict: high on execution, two brief-inherited blemishes to clean.**
  Verbatim fidelity exact, fire-verification genuine (independently reproduced),
  the one real extra contradiction (row 29) found and disclosed, commit clean and
  scoped. Blemishes: the 6(b) omission [Medium] and the amendment-4 block's
  lowercase sentence-start + unindented continuation [Low].

---

## Findings by severity

### Medium

**M1 - The brief dropped a design-mandated clause from amendment 6(b); the landed
spec re-opens the exact ambiguity the design closed.**

Design section 6, amendment 6(b) directs the pin-release enumeration to gain a
third condition quoted verbatim as: *"or the active view is switched, in which
case the hover state resets too and the sidebar shows the new view's topic"*, and
states the hover-reset half *"lands together with the rest of this amendment via
Task 21, unchanged."* The brief's step-5 replacement text is only *"until another
element is clicked, the active view is switched, or help mode exits"* - the
`in which case ...` clause is gone.

- Not the implementer's fault: the implementer was bound to verbatim fidelity and
  transcribed the brief's block byte-perfect (diff line 391). The divergence is
  design -> brief; there was no fidelity gate on that hop, only on brief -> spec.
- Not covered elsewhere in the landed spec. L389 says the sidebar initially shows
  the current view; L390 covers hovering an *unannotated* element (a different
  trigger, hoverId -> null); L391 now says the pin releases on view switch but
  never states hover is cleared or that the sidebar shows the *new* view's topic.
  A reader with only the spec can infer a stale `hoverId` survives the switch and
  the sidebar keeps the OLD view's hover topic - precisely the T13 self-consistency
  bug the design added this clause to fix. D52 (plan-7 design) carries the full,
  correct mechanic, but v1-design.md is the enduring product spec and should carry
  the durable behavior.
- Irony worth flagging: the design's own 6(b) preamble says "the spec is
  authoritative, so each D52 deviation is amended, **not silently narrowed**." The
  brief silently narrowed it.
- Fix: restore the full third condition into the L391 pin bullet
  (`... the active view is switched, in which case the hover state resets too and
  the sidebar shows the new view's topic, or help mode exits`), OR the owner
  explicitly ratifies the compression as adequate spec-level text. Controller
  call; the implementer cannot resolve it.

### Low

**L1 - Amendment-4 (eslint) block: lowercase sentence-start after a period.**
L422 reads `... without a help topic file. the \`@intlify/vue-i18n/no-raw-text\`
eslint rule (D27) ...` - "the" starts a sentence lowercase. Verbatim from the
brief's fenced block. The pre-amendment text also began the sentence lowercase
("eslint ..."), but "eslint" is a conventionally lowercased tool name; a bare
determiner "the" reads as a typo. Cosmetic; controller may capitalize "The".

**L2 - Amendment-4 block: continuation lines not indented to the doc's
bullet-continuation convention.** L423-426 sit at column 0, whereas every other
multi-line bullet in the doc uses a 2-space continuation indent (amendment 1 at
L406-408, amendment 6(c) at L392-395, non-goal at L441-444). It renders correctly
(CommonMark lazy continuation keeps them in the one bullet paragraph, no blank
line), so it is a house-style consistency slip, not a rendering defect. Verbatim
from the brief's block, which was written flush-left.

### Informational (non-blocking, no fix owed)

**I1 - Directory-tree example (L329-330) shows only `locales/en/*.ftl` /
`help/en/<help-id>.md`.** These are illustrative single-locale path examples, not
a ship-scope claim, so they do not contradict the en+de amendments. Consistent
with the report's read. At most a cosmetic staleness (could show `{en,de}`); not a
contradiction, no fix required.

---

## Dimension 1 - Transcription fidelity: PASS

Byte-compared each brief fenced block against the diff. All six land verbatim at
the anchor the brief names; the diff contains exactly the six amendments plus the
one surfaced row-29 alignment and nothing else.

| amendment | site | verbatim vs brief | diff lines |
|---|---|---|---|
| 1 (8.4 last bullet) | L406-408 | exact | +72..74 |
| 2 (non-goal 11) | L441-444 | exact | +109..112 |
| 4 (spec 10 eslint) | L422-426 | exact (incl. the L1/L2 warts) | +89..93 |
| 5 (locale-selection append) | L405 | exact, inserted after "app settings" | +71 |
| 6(a) hover append | L390 | exact | +54 |
| 6(b) pin release | L391 | exact vs brief (brief itself truncated - M1) | +55 |
| 6(b) toggle Esc | L388 | exact | +50 |
| 6(c) suppressed bullet | L392-395 | exact | +56..59 |

Insertion/deletion count independently recomputed from the diff: 8 deletions, 21
insertions - matches the `1 file changed, 21 insertions(+), 8 deletions(-)` stat
and the report. No collateral edits.

## Dimension 2 - Sweep completeness: PASS (implementer's find confirmed; no
further echo)

Walked the amended spec myself for every restatement of the amended mechanics.

- **Localization / content-language:** the only genuine additional contradiction
  was section-2 decision-log **row 29** ("English-only content ships in v1"),
  which the implementer caught and aligned. No other localization echo is stale.
  L403's "both English-only by design" is correctly scoped to non-UI documentation
  (schema `description` fields, this spec, README) and is deliberate, not a
  restatement of the UI-content ship claim. L255 ("rendered message in the active
  locale") and L404 (catalog source of truth, fluent-rs CLI embedded at build
  time) are locale-generic mechanism statements, true under D63. Row 30
  (Discoverability) mentions help mode only at the "hover-to-explain" altitude,
  unaffected.
- **Help-mode / pin / hover / Esc / activation:** all `pin`/`pinned` hits outside
  8.3 (L174, L176, L284, L285, L412-413) are the *pinned identification schema*
  sense, unrelated. Only the amended 8.3 bullets carry help-mode mechanics.
- **D52/D54/D63/D64 / CLI language:** no spec sentence asserts the CLI is
  English-only. Nothing contradicts the en+de amendments.

The design's own sweep enumerated 8.3's mechanics sentence, spec 10's help-ids
clause, 8.2's grid list, 8.4's catalog-source bullet, and the non-goal list - and
missed the section-2 decision log. The brief's six sites also missed it. That both
missed row 29 is evidence the echo set was enumerated over prose sections but not
over the decision-log/summary table (HARVEST H2). The implementer's independent
find is the correct closure; my walk finds no seventh site.

## Dimension 3 - Fire-verification: PASS (independently reproduced)

The report claims each step-6 grep was fire-verified against pre-amendment HEAD
(then `e4df011`). Re-ran all five on the current tree and re-ran the three
"must-disappear" patterns plus the two "must-appear" patterns against `HEAD~1`
(= `e4df011`, pre-amendment) as a positive control:

- Post-amendment (current tree): `English content only`, `no-literal-string`,
  `only English catalogs` all rc=1 (no output); `cli-multilang-rendering` count 1;
  `suppressed` hits L393. All as the brief specifies.
- Positive control (HEAD~1): the three "must-disappear" patterns **fire** at
  L402, L416, L431 (proving the greps are sound, not vacuous);
  `cli-multilang-rendering` and `suppressed` both count 0. Line numbers match the
  report's fire-verification listing exactly.

The check is verified by making the negative produce output once - done. The
report's fire-verification is genuine, not asserted.

## Dimension 4 - Q1 adjudication

**Q1(a) process - was proceeding (no NEEDS_CONTEXT) correct? YES; no routing
owed.** The alignment was both permitted and required:

- *Required* by the amendment-sweep duty ("a spec amendment sweeps the spec for
  self-contradictions before commit"). Row 29's Choice cell said "English-only
  content ships in v1" while the just-landed 8.4 body says en+de on both surfaces.
  Leaving it stale would have committed a self-contradicting spec - the exact
  defect the sweep exists to prevent. The sweep's whole point is to reach sites
  beyond the enumerated six.
- *Not stopped* by the dispatch's rule (only NON-verbatim-safe alignments route
  NEEDS_CONTEXT). This alignment is verbatim-safe: it invents no decision content,
  only propagates the fact amendments 1/2 already ruled (which themselves encode
  the governing-human E1 Ruling 2, `cli-multilang-rendering`, D63). No residual
  decision, ambiguity, ripple, or hidden consumer - only a stale echo to
  synchronize, and exactly one correct target wording (mirror the ruled amendment).
- Steelman for "owed a routing" (the decision log is a sensitive ledger, and the
  site is outside the brief's enumeration): rejected. The dispatch's gate is
  verbatim-safety, not enumeration-membership; the sweep duty is precisely a
  mandate to act beyond the enumeration; and the implementer disclosed the call in
  its report for controller confirmation rather than burying it. A NEEDS_CONTEXT
  here would have been over-routing on a pure fact-propagation. Proceeding +
  disclosing is the right calibration.

**Q1(b) content - is the new wording exactly true under the amendments' letter?
YES; no overclaim. The adjudication question's premise is stale and must be
corrected.**

The Q1(b) prompt parenthesizes "E1: CLI stays English-only, GUI/help ships en+de".
That is **false under the binding ruling.** It describes E1 **Ruling 1**
(`cli-english-only`), which E1 **Ruling 2** (same day, `cli-multilang-rendering`,
`product-boundaries.yaml:448`, marking `cli-english-only` superseded) **REVERSED**.
D63 makes it concrete: `crates/muxsmith-cli/src/i18n.rs` embeds **both** en and de
and renders through a two-bundle fallback chain (`--locale` > system > en). The
landed amendment-1 text itself commits to it: "on both surfaces - GUI catalogs and
help topics, **and the CLI's embedded catalogs** (`cli-multilang-rendering`, D63)".

So under the amendments' actual letter, v1 ships en+de content on **all** surfaces
including the CLI. Row 29's unqualified "English and German content ships in v1" is
therefore accurate on its face and does **not** overclaim for the CLI - the
row-context ("Localization", spanning the Rust CLI and the frontend) reinforces
rather than rescues it. Had Ruling 1 stood, the unqualified sentence *would* have
overclaimed for the CLI and the row-context would not have saved a general
content-ship claim - but Ruling 1 did not stand. Verified against the artifact
(design E1 section 7, D63), not accepted from the prompt's parenthetical.

## Dimension 5 - House dimension

Docs-only prose amendment; no code/config, so most Tier-2 code conventions do not
apply. House-style deviations are the two amendment-4 block warts (L1, L2 above),
both verbatim-inherited from the brief. No new pattern introduced by the
implementer. Commit message and trailer match the brief exactly; `git add` scoped
to the one file.

---

## HARVEST

- **H1 (ledger candidate) - design->brief transcription needs its own fidelity
  gate.** The brief->spec hop was verbatim-checked (step 1-6, fire-verified). The
  design->brief hop was not, and it silently truncated design amendment 6(b)
  (M1). When a brief carries verbatim replacement blocks lifted from a design's
  amendment section, those blocks should be diffed against the design's own
  amendment-quoted text before dispatch - the implementer, bound to brief-fidelity,
  structurally cannot catch a brief that diverges from the design.
- **H2 (ledger candidate) - a spec self-contradiction sweep must include the
  decision-log / summary tables, not only prose sections.** Both the design's own
  sweep and the brief's grep set enumerated body sections and missed section-2 row
  29, which restates the same fact. Decision-log rows and summary tables are
  first-class echo sites. The implementer's catch is the evidence for the rule.
- **H3 (over-restriction watch) - keep the verbatim-safe-alignment latitude; do
  not invert it into "any out-of-enumeration site must NEEDS_CONTEXT".** That
  inversion would have forced over-routing on row 29's pure fact-propagation and,
  if the implementer had instead under-swept, left a self-contradicting spec. The
  current gate (verbatim-safety test + mandatory in-report disclosure) is correctly
  calibrated; the fix for the missed enumeration is broadening what the sweep
  *covers* (H2), not forbidding inline alignment.
- **H4 (dispatch-hygiene note) - adjudication prompts must cite the BINDING
  ruling.** The Q1(b) parenthetical asserted "CLI stays English-only", the reversed
  E1 Ruling 1. A reviewer taking it at face value would have mis-adjudicated Q1(b)
  as an overclaim. Cite E1 Ruling 2 (`cli-multilang-rendering`) when the CLI
  language scope is load-bearing.
- **H5 (house-style nit) - docs-amendment briefs' fenced blocks that land
  mid-list should carry the doc's 2-space bullet-continuation indent and must not
  begin a sentence with a lowercase determiner after a period** (L1, L2).

---

## Delta verdict (2026-07-22) - fixes in cc0e6d7: APPROVED

Judged commit `cc0e6d7` (master, on top of `4ac8d8b`), which applies the two prior
findings. Result: both resolved cleanly, minimally, and with no collateral.

- **Scope:** `cc0e6d7` touches the spec file only - `1 file changed, 6
  insertions(+), 6 deletions(-)`, exactly two hunks (L391; L422-426). The
  `decision-ledger.yaml` change visible in the cumulative `4ac8d8b..cc0e6d7` range
  belongs to the separate intervening commit `c0f7250` (the H1/H2 harvest landing),
  not to this delta. Nothing else in the spec moved.
- **M1 resolved (6(b) hover-reset clause restored).** The design-directed clause
  "in which case the hover state resets too and the sidebar shows the new view's
  topic" is now present **verbatim** in the L391 pin bullet (byte-compared against
  design section 6, `2026-07-21-plan7-help-i18n-design.md:1765-1767`). It sits
  inside the already-accepted brief enumeration order (clicked / view-switched
  [+clause] / help-mode-exits) rather than the design's literal tail position; the
  ordering was ratified in the prior review and "gains a third condition" governs
  content, not slot, so this is immaterial. The landed sentence is grammatical and
  matches D52's "switching views clears both refs; the chain lands on the new
  view's topic". Sweep around the site re-checked: consistent with L389 (initial
  current-view topic) and L390 (hover resolution); no new contradiction.
- **L1/L2 resolved (amendment-4 block).** `file. The \`@intlify` - "The"
  capitalized. Continuation lines 423-426 now carry exactly the doc's 2-space
  bullet-continuation indent (`cat -A` confirmed), matching the amendment-1,
  6(c) and non-goal blocks. Pure whitespace/case; no semantic change, no sweep
  impact.
- **Fire-verification non-finding stands.** Step-6 greps on the current tree:
  `English content only`=0, `no-literal-string`=0, `only English catalogs`=0,
  `cli-multilang-rendering`=1, `suppressed`=1 - all as specified.
- **Prior non-findings stand.** I1 (directory-tree `en`-only illustrative paths)
  untouched, still non-blocking; the other five amendments and the row-29 alignment
  untouched.

**Combined verdict now: APPROVED.** Both review findings are correctly and
minimally addressed; the spec is fully compliant with design section 6.
