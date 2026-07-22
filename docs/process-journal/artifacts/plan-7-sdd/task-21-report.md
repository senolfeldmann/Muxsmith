# Task 21 report: v1-spec amendments (design section 6)

Verdict: **DONE** (see "Follow-up fix (post-review)" below - the review's
NEEDS FIXES verdict is closed).

Commit 1 (original amendments): `4ac8d8b26c74097c2d1eb3c9960c1493fa385f06`
on `master`, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
only.
Commit 2 (review fixes): `cc0e6d7e79bae20d34c52ba52a4bc4f3157e7d03` on
`master`, same file only.

## Follow-up fix (post-review)

Review verdict (`task-21-verdict.md`): NEEDS FIXES, both findings
brief-level (design->brief transcription loss and a formatting wart),
execution against the brief graded exemplary.

- **M1**: the brief's amendment-6(b) block truncated design section 6's
  directed clause. Re-read the origin (design section 6, lines 1762-1769
  of `2026-07-21-plan7-help-i18n-design.md`, not the brief and not the
  reviewer's paraphrase) and landed the full third condition at the L391
  pin bullet: `..., the active view is switched, in which case the hover
  state resets too and the sidebar shows the new view's topic, or help
  mode exits.` Verified this resolves the exact ambiguity the review
  flagged (L389/L390 neighbors now consistent: on view switch, hover
  resets and the sidebar shows the new view's default topic, matching
  L389's "initially shows the current view").
- **L1/L2**: amendment-4 block's lowercase sentence-start after a period
  ("the `@intlify/vue-i18n/no-raw-text`..." -> "The ...") and missing
  2-space bullet-continuation indent (confirmed the house convention
  against amendment 1's block at L406-408 and amendment 6(c)'s at
  L392-395) both fixed.
- Step-6 sweep greps re-run after both fixes: all five still pass exactly
  as before (`English content only` / `no-literal-string` / `only English
  catalogs` rc=1 each, `cli-multilang-rendering` count 1, `suppressed`
  hits L393).
- `git diff` for commit 2 confirmed exactly the two intended hunks (the
  L391 pin bullet and the L422-426 eslint block), nothing else touched:
  `1 file changed, 6 insertions(+), 6 deletions(-)`.
- Landed 6(b) clause, verbatim, as it now stands in the spec (L391):
  `- Clicking an element pins the selection: the element gets a prominent
  marking and the sidebar stays on its topic regardless of hover, until
  another element is clicked, the active view is switched, in which case
  the hover state resets too and the sidebar shows the new view's topic,
  or help mode exits.`

## Anchor re-verification (quoted text, not line number)

All six sites the brief cites were located by their quoted text on the
current tree before editing. None had drifted; every cited line number
matched exactly:

| brief cite | quoted anchor text | found at | drift |
|---|---|---|---|
| `:402` (amendment 1) | `- v1 ships English content only (non-goal 11); the mechanism ships complete.` | 402 | none |
| `:431` (amendment 2) | `- UI localization content: only English catalogs and help topics ship in v1. The mechanism (8.4) ships complete; adding a locale is content work, not a refactor.` | 431 | none |
| `:416` (amendment 4) | `eslint (no-literal-string rule) keeps hardcoded strings out of the frontend; core is prose-free by construction.` | 416 (tail of the i18n/help-completeness bullet) | none |
| `:401` (amendment 5) | `Locale selection: system locale with manual override in app settings and \`--locale\` on the CLI; falls back to English per message.` | 401 | none |
| `:388` (amendment 6, toggle bullet) | `A prominent Help/Guide button, always visible in every view. Clicking it toggles help mode; clicking again (or Esc) exits.` | 388 | none |
| `:390` (amendment 6, hover bullet) | `Hovering any help-annotated element highlights it...` | 390 | none |
| `:391` (amendment 6, pin bullet) | `Clicking an element pins the selection: ... until another element is clicked or help mode exits.` | 391 | none |

## Per-step outcome

- **Step 1 (amendment 1, 8.4 last bullet)**: done, fenced block landed
  verbatim.
- **Step 2 (amendment 2, non-goal 11)**: done, fenced block landed
  verbatim.
- **Step 3 (amendment 4, spec 10 eslint sentence)**: done, fenced block
  landed verbatim (inline mid-bullet replacement of the closing sentence).
- **Step 4 (amendment 5, locale-selection bullet)**: done, inline append
  `(takes effect live, without restart; D56)` placed exactly after
  "manual override in app settings" as instructed. CLI half left
  unchanged per the brief (recorded no-change).
- **Step 5 (amendment 6, 8.3 help-mode bullets)**: done, all three parts:
  - (a) additions: hover bullet (:390) gained the no-help-id / pin-without-activating
    sentences, appended verbatim.
  - (b) modifications: pin bullet (:391) release enumeration replaced with
    the brief's exact quoted text `until another element is clicked, the
    active view is switched, or help mode exits`; toggle bullet (:388) Esc
    clause replaced with the brief's exact quoted text `clicking again (or
    Esc, except while the settings dialog is open, whose native cancel
    consumes Esc) exits`.
  - (c) new bullet: the ruled activation-semantic bullet appended
    verbatim immediately after the (now-modified) pin bullet.
- **Step 6 (verify no superseded text survives)**: all five greps run,
  each fire-verified beforehand against `git show HEAD:` of the file (see
  below) to confirm the check pattern is sound, not vacuous. Results
  after amendment: all as expected.
- **Step 7 (commit)**: done, message and trailer exactly as specified,
  explicit `git add` of the spec file only.

## Step 6 verification, with fire-verification against HEAD

Fire-verification first (each pattern confirmed to hit at the expected
pre-amendment line, proving the check is not vacuous):

```
$ git show HEAD:$S | grep -n "English content only"
402:- v1 ships English content only (non-goal 11); the mechanism ships complete.
$ git show HEAD:$S | grep -n "no-literal-string"
416:- i18n and help completeness: ... eslint (no-literal-string rule) keeps hardcoded strings out of the frontend; core is prose-free by construction.
$ git show HEAD:$S | grep -n "only English catalogs"
431:- UI localization content: only English catalogs and help topics ship in v1. ...
$ git show HEAD:$S | grep -c "cli-multilang-rendering"
0
$ git show HEAD:$S | grep -c "suppressed"
0
```

Post-amendment run (the brief's actual step-6 commands):

```
$ S=docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
$ grep -n "English content only" $S    # rc=1, no output - expected
$ grep -n "no-literal-string" $S       # rc=1, no output - expected
$ grep -n "only English catalogs" $S   # rc=1, no output - expected
$ grep -cn "cli-multilang-rendering" $S
1
$ grep -n "suppressed" $S
393:  area is suppressed; the help toggle, the three view tabs, the settings
```

All five match the brief's expected outcomes.

## Self-contradiction sweep (beyond the brief's step 6)

Walked every amendment's neighbors and every spec section that restates
the amended mechanics, per the standing amendment duty:

- **Spec 8.3's mechanics sentence** ("help content is one markdown file
  per help-id per locale (`help/<locale>/<help-id>.md`)"): unchanged,
  agrees with D51. **Aligned/unaffected.**
- **Spec 10's "help-ids without a help topic file" clause**: unchanged,
  agrees with D62 (adds the reverse direction only, no conflict).
  **Aligned/unaffected.**
- **Spec 8.2's grid column list**: already names "order" in the
  track-rule grid tuple (`order, source, match summary, changes,
  optional`); D59 closes a gap without needing an amendment.
  **Verified-unaffected.**
- **Spec 8.4's catalog-source-of-truth bullet** (`One catalog source of
  truth under \`locales/\`...`): stays true under D55 (attributes are
  catalog content, not a new source). **Aligned/unaffected.**
- **Non-goal list, other items** (10 of 11 bullets besides the amended
  one): byte-identical diff confirms zero collateral change. **Verified
  by diff.**
- **Section 10 Testing, other bullets** (5 of 6 besides the amended
  one): byte-identical diff confirms zero collateral change. **Verified
  by diff.**
- **Section 8.3, other bullets/prose** besides the three amended
  bullets: byte-identical diff confirms zero collateral change.
  **Verified by diff.**
- **Line 403's "both English-only by design"** (JSON Schema `description`
  fields): distinct, deliberate, unrelated claim (schema docs stay
  English-only like the README, explicitly scoped to non-UI content in
  the existing sentence) - not a restatement of the UI-content locale
  claim amendment 1/2 touch. **Verified-unaffected.**
- **Directory-tree example** (`locales/en/*.ftl`, `help/en/<help-id>.md`
  at :329-330): illustrative single-locale example paths, not a
  ship-scope claim. **Verified-unaffected.**
- **"pin"/"pinned" hits elsewhere** (:174, :176, :284, :285, :412-413):
  all refer to the *pinned identification schema*, an unrelated sense of
  the word. **Verified-unaffected.**
- **Section 2 Decision log, "Localization" row** (line 29, pre-amendment):
  `| Localization | i18n-ready from day one; English-only content ships
  in v1 | ... |` - **genuine contradiction found**, not enumerated by the
  brief's six sites nor covered by the design's own self-contradiction
  sweep (design section 6's sweep paragraph lists 8.3's mechanics
  sentence, spec 10's help-ids clause, 8.2's grid list, 8.4's
  catalog-source bullet, and the non-goal list - it does not mention
  section 2). Directly contradicted the newly-landed fact ("v1 ships
  English and German content on both surfaces"). See Concerns.

## Concerns

1. **Section 2 Decision log row 29 aligned outside the brief's
   enumeration.** The "Localization" row's Choice cell said "English-only
   content ships in v1", contradicting the just-landed amendment 1/2 fact.
   I judged this a verbatim-safe alignment (not a keyboard-resolved fork):
   the underlying fact was already fully and unambiguously ruled by
   amendments 1/2 (D63, `cli-multilang-rendering`), there was no
   remaining decision content, ambiguity, ripple cost, or hidden consumer
   to weigh - only a stale echo to synchronize. I changed the cell from
   "English-only content ships in v1" to "English and German content
   ships in v1", mirroring the ruled wording, and left the Rationale cell
   (already accurate, mentions no locale-count claim) untouched. Diff:

   ```
   -| Localization | i18n-ready from day one; English-only content ships in v1 | ...
   +| Localization | i18n-ready from day one; English and German content ships in v1 | ...
   ```

   Flagging this per the "any contradiction you cannot align
   verbatim-safely is NEEDS_CONTEXT" rule's converse: I judged this one
   *could* be aligned verbatim-safely (no invented decision content, only
   propagation of an already-decided fact to a second echo site) and did
   so, but it is outside the brief's six enumerated steps and outside the
   design's own sweep, so the controller/reviewer should confirm this
   judgment call.

## Commands run (verbatim, foreground only)

```
grep -n "^## \|^### \|^#### " docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
sed -n '382,420p' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md   # (and similar targeted reads)
awk '/^```markdown$/{f=1;next} /^```$/{f=0} f' .superpowers/sdd/plan-7/task-21-brief.md | cat -A
grep -n "English content only" $S ; grep -n "no-literal-string" $S ; grep -n "only English catalogs" $S
grep -cn "cli-multilang-rendering" $S ; grep -n "suppressed" $S
git show HEAD:$S | grep -n "English content only" / "no-literal-string" / "only English catalogs" ; grep -c "cli-multilang-rendering" / "suppressed"
diff <(git show HEAD:$S | awk '/^## 11\. Non-goals for v1/{f=1;next} /^## 12\./{f=0} f') <(awk '/^## 11\. Non-goals for v1/{f=1;next} /^## 12\./{f=0} f' $S)
diff (section 10 Testing, section 8.3, section 8.4) via the same extract_sec awk pattern
grep -n -i "english-only\|english only" $S
grep -n -i "locale\|german\|i18n\|help mode\|help-id\|fluent" $S
grep -n -i "\bpin\b\|\bpinned\b\|\bhover\b\|activation\|Esc\b" $S
git status --short
git branch --show-current
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "spec: fold plan-7 amendments 1/2/4/5/6 - bilingual v1 on both surfaces, live locale switch, real eslint rule, ruled help-mode semantics (design section 6)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
git log -1 --format="%H %s"
git status --short   # clean after commit
```

## Surfaced items

- New pattern established: none (pure prose amendment, no code/config
  touched).
- Deliberate deviation from the brief: none within the six enumerated
  steps; the section-2 alignment (Concern 1) is an addition beyond the
  brief's enumeration, done under the sweep's own "align verbatim-safely"
  latitude rather than as a silent decision.
- Premise refutations: none - all six cited anchors matched the tree
  exactly, no line drift.
- `git diff --stat` for the commit: `1 file changed, 21 insertions(+), 8
  deletions(-)` (20+7 from the six brief amendments plus the extra
  1-line/1-line swap for the Concern 1 alignment).
