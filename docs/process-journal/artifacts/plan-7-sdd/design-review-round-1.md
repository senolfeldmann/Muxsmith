# Plan 7 design review, round 1

Artifact: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
Reviewer: independent (no stake in authorship). Ground truth: v1 spec >
controller design brief > Tier-2 house files > the real tree and installed
packages. Every load-bearing claim below was re-measured against the tree,
the installed packages, the npm registry, and `~/Downloads/mkvtoolnix` on
2026-07-21; every passing-by-absence check was validated against a
known-present control first.

## Verdict: NEEDS FIXES

## Findings (severity-ranked)

### 1. BLOCKER - the D57 config_path grammar enumeration is falsified in both directions

Section 1 ("`config_path` grammar, as core actually emits it" / "Full
emission set") and D57's normative closure ("The implementer derives
nothing: D57 fixes the anchor paths by construction from this grammar";
section 9: "exact string equality on the emission grammar as written in
ground truth") rest on an enumeration that is wrong on five counts:

- **Phantom member `tracks`**: the doc lists "`tracks` (:70 unmatched-keep
  info)". The unmatched-keep info (PassthroughProfile) anchors at
  `"tracks.rules"`, not `tracks` (`validate.rs:70-73`). No bare `"tracks"`
  config_path exists anywhere in validate.rs/lint.rs/report (grep with
  positive control `"tracks.rules"` firing at :66 and :72). D57's anchor
  list ("`tracks.rules` and `tracks` anchor at the grid heading and tracks
  section") builds a dead anchor from it.
- **Phantom suffix `.path`**: "validate_locator appends `.path`, ..." - no
  `.path` emission exists in validate.rs (grep `\.path`: zero hits;
  control: `.extensions` fires at :457). validate_locator emits
  `.extensions` (:457), `.match_to_source` (:468), `.match_pattern` (:479)
  and the **bare locator path** (LocatorConflict, `path.to_string()`,
  :461-464) - the last is missing from the enumeration.
- **Missing member `input.extensions`**: emitted at :55-57; the doc
  misattributes :55 to "input.pattern (:38,:55)". input.pattern is emitted
  once, at :38.
- **Missing members bare `.any` / `.not`**: EmptyMatchList errors anchor
  at `{path}.any` (:352-355) and `{path}.not` (:363-366), not only at the
  indexed forms.
- **"template suffixes below" is empty**: validate_template anchors every
  diagnostic at the bare template path (`path.to_string()`, :499-530);
  there are no template suffixes. The entries "`output.filename.template`
  (template suffixes below)" and "`title.template...`" (an ellipsis, in a
  document claiming "no ellipses") imply suffixes that do not exist.

Impact: the widget-tree construction rules in D57 do, by construction,
cover all real emissions I could find (input.extensions via
SectionWidget, bare .any/.not at ListWidget roots, bare locator path at
the block section) - the mechanics survive. But the enumeration is the
normative reference the plan and its tests will transcribe; as written
the implementer meets three unlisted real paths and two phantoms at the
keyboard, which is precisely the NEEDS_CONTEXT fork the document's own
"Every other fork is closed" claim (:24) says cannot occur, and precisely
the D49/D44 defect class recorded in `proc-latitude-clause-boundary`'s
occurrences. Fix: re-derive the emission set from validate.rs/lint.rs and
re-cite; re-state D57's anchor list against the corrected set.

### 2. MAJOR - D61's TS-side enumerations are incomplete (render sites and refs)

- "The four render sites (`FirstRun.vue:94`, `RunHistory.vue:155`,
  `JobsView.vue:246`, `:252`)": there are **five** direct
  `$t(*.code, *.params)` IpcError sites - **`RunHistory.vue:241`**
  (`jobLogError`) is missing, and it is the site that renders exactly the
  promoted numeric `index` param from `run.rs:935` (job-log-not-found).
- "the two `errorParams` refs typed accordingly (`SettingsDialog.vue:16`,
  `BatchView`'s `ipcErrorParams`)": **`EditorView.vue:121-122`** declares
  its own `ipcErrorCode`/`ipcErrorParams` pair
  (`ref<Record<string, string>>`), rendered at `EditorView.vue:436` - a
  third ref pair the typed-wire change must touch, unmentioned anywhere
  in D61. (Full site inventory measured: FirstRun:94, RunHistory:155,
  :241, JobsView:246, :252, EditorView:436, BatchView:438,
  SettingsDialog:93.)

The Rust-side promotion enumeration (error.rs:169, :170, :174, run.rs:935
and no others) I verified as complete and correct. The TS sweep
enumeration is not, and section 9 pins the plan to D61's lists verbatim.

### 3. MAJOR - two D52 deviations from binding spec 8.3 mechanics carry no spec amendment

Spec 8.3 (binding per the brief) says the pin holds "until another element
is clicked or help mode exits", and "clicking again (or Esc) exits". D52
changes both: (a) **switching views clears the pin** - a third clear
condition the spec's enumeration excludes; (b) **Esc is suppressed while
the settings dialog is open**. Both are reasonable designs, but amendment
6 (section 6) lists only the three *additive* interaction decisions
(unannotated hover, pin-not-activate, Esc exits) and amends neither
modification. The spec-amendment list is therefore incomplete against the
document's own decisions - the exact "spec-amendment list complete and
consistent" dimension of this review.

### 4. MAJOR - counts contradicting their own enumerations

- D55 headline: "The complete migration set - **30 ids fold away**, 1 is
  renamed in place". The D55 tables and section 2's (correct) recount give
  **27 removed + 1 renamed** (21 tooltip-family + 6 hint fold away; 22nd
  tooltip id renamed). 30 is not derivable from any reading of the tables.
- Section 0 postscript: "the **18** `:title` sites all live in the other
  views". Measured: **20** title-attribute sites (= D55's own "19 template
  sites + `runTooltip`"); 18 is the `-tooltip` id count, not the site
  count.

Both are the recorded count-recompute defect class
(`process-conventions.yaml:444` entry, failure shape 1). The enumerations
themselves I verified complete against the real catalogs (18 `-tooltip`
ids + 4 `batch-run-tooltip-*` variants + 6 `-hint` ids, exactly; the
non-migration list matches the remaining id inventory; all seven
section-2 catalog rows recompute correctly from today's measured counts).

### 5. MAJOR - explicit implementer-latitude clause in D52 (highlight CSS)

"Exact colors/widths are CSS the implementer writes within these two named
classes; they are presentation tokens, not behavior forks."
`proc-latitude-clause-boundary` bans implementer-choice clauses in any
artifact an implementer reads, names no presentation exemption, and routes
user-visible forks to the governing human. The recorded grant
(`latitude-carveout-zero-content-structural-forks`) cannot cover it: it
lives in the implementer brief (not in a design document), fills silence
only (this is an explicit clause), and its condition 4 excludes anything
user-visible ("no layout, nothing a user or screen reader perceives") -
a highlight outline is user-perceivable by purpose. The document even
argues the boundary away in-line ("presentation tokens, not behavior
forks"), which is self-classification of the kind the T13 occurrence
records as a violation. Fix: enumerate the two class definitions in the
design, or route the two tokens through the owner's rendered-surface pass
alongside the wording that already goes there. (See HARVEST: this is also
calibration evidence that the house may *want* a recorded
presentation-token rule - but today none exists.)

### 6. Minor - D61's string-param complement list omits `code`

"Every other `.with` site stays a string (`detail`, `found`, `minimum`,
`path`, `property`, `run_id`, `file`)" - `error.rs:87-91` also sets a
`code` param on `mkvmerge-query-failed`
(`code.map(|c| c.to_string()).unwrap_or_else(|| "signal".into())`).
Keeping it a string is correct (the "signal" fallback forces it), but the
list is presented as the complete complement and is not.

### 7. Minor - Tier-2 citation id wrong, and the budget claim overstated (D59, Grounding)

The cited `gui-generic-action-keys` does not exist; the entry is
**`editor-generic-action-keys`** (`product-boundaries.yaml:404`). Its
statement fixes the gui-editor budget at 45 as "a hard boundary against
prose growth (tooltips remain Plan 7)" - it reserves the *tooltip pass*,
not a 46th message id. D59's "reserved exactly this plan's growth" is not
what the entry says: adding `editor-track-rule-order` (45 -> 46) is a
boundary revision that should be surfaced as such (substantive cover
exists - the ordinal column is a ROADMAP anchor input - but the last
budget revision, 43 -> 45, took an explicit owner ruling).

### 8. Minor - D62 check 1(c)'s extraction mechanism rests on a false premise

"(c) the view-topic literals in `src/help/state.ts`'s
`Record<View, string>` (shape (a) already catches object-literal
values...)" - shape (a) is `helpId:\s*(['"])...`; a
`Record<View, string>` literal has keys `batch:`/`jobs:`/`editor:`, which
(a) cannot match. As written, (c) has no specified scan. Harmless in
practice (the three view roots carry `data-help-id`, so shape (b) already
puts the ids in the referenced set), but the clause as specified is
unimplementable without invention. Also: D52 enumerates `state.ts` as
three refs; the Record D62 references never appears in D52's module
contents.

### 9. Minor - correction #4 miscounts DiagnosticsPanel's `$t` calls

"its one `$t` call sits at `:48`" - there are three (`:46` line template,
`:47` severity, `:48` code) in one nested expression. The load-bearing
conclusion (not an IpcError site, promotion already handled by
`diagnosticFluentParams`, Diagnostic wire untouched) is correct - I
verified it - but the stated measurement is wrong in a section whose whole
charter is measured corrections.

### 10. Minor - D54's classification method leaves app-shell controls unclassified

The method claims completeness via "the complete set of `:title`-carrying
controls plus view-level surfaces". The settings-open button
(`App.vue:110`) **is** `:title`-carrying and appears in D55's migration
table but in neither D54 classification table; the three nav tabs and the
new help toggle (interactive, live during help mode per D52's allowlist)
are likewise unclassified; SuggestionCard's copy/apply buttons
(`:80`/`:94`, both `:title`-carrying) are subsumed by the card row but not
listed. Intent (shell chrome out) is obvious; the stated method makes the
omission a gap by the document's own test.

## Verified clean (the load-bearing claims that held)

- **No-work-needed claims, all executed**: `$ta` native in installed
  fluent-vue 3.8.2 (`dist/index.mjs:295` default `"$ta"`, `:500-502`
  install, `index.d.mts:16` type; `bundles` setter shallowRef-backed
  `:474-483`; `v-t` watchEffect `:434-438`; symbols `:62-64` reads the
  ref). e2e real-parses every catalog incl. cli.ftl
  (`e2e/catalogs.spec.ts:12` -> `i18n-en.ts:118-136`), so the ROADMAP
  :689-690 sentence is indeed stale. No `v-html` in src (control:
  sibling-directive grep fires). DiagnosticsPanel needs no promotion work.
  19 IpcError codes independently reproduced (12 error.rs / 5 run.rs /
  2 lib.rs distinct codes, test modules excluded; 14 gui-common + 5
  gui-jobs catalog entries all present).
- **Registry/catalog ground truth**: 42 unique labelKeys + 1 FixedField
  (`registries.ts:73-75`), 10 widget components + dispatcher; en=de id
  counts per file exactly as tabled; zero Fluent attributes (control
  validated); D54's 18+24=42 classification complete and correct against
  the real 42 label ids; D55's migration + non-migration sets complete
  against the real id inventory; close-abort line-parser constraint real
  (`run.rs:544,554-560`).
- **Registry/external**: marked 18.0.7 published 2026-07-21, 0 deps, MIT,
  12454 B min+gzip (bundlephobia); micromark 4.0.2, 17 deps, 14568 B;
  snarkdown 2.0.0 dormant since 2020/2022; markdown-it 14.3.0, 6 deps,
  @types at 14.1.2 (lag exact); package.json exact-pin style confirmed;
  pnpm-lock markdown grep zero hits with the doc's own 152-hit control
  reproduced exactly. CSP as cited (`tauri.conf.json`). CLDR carve-out
  set {zero, one, two, few, many, other} + numeric literals is the
  complete CLDR plural-category set - correct.
- **mkvtoolnix (SI-3), spot-checked at source**: `Util::setToolTip`
  helper exact (`util/widget.cpp:95-113`, uiDisableToolTips honor +
  `<span>` wrap); 62 `settooltip` occurrences in 4 files under `merge/`;
  the no-in-app-help negative reproduced with the same positive control
  (whatsthis/Key_F1/HelpContents/contextHelp: zero files; helpEvent only
  `fancy_tab_widget.cpp`); `s_valUiDisableToolTips`
  (`settings_names.h:152`); bundled offline per-locale manual with
  fallback (`main_window.cpp:662-696`).
- **Structure**: D50-D62 all carry decision/rationale/steelmanned
  rejections/interface changes; all 11 anchor inputs + rider + all 14
  design questions land in named ADRs; owner decisions honored (option B
  enumerated with per-control justifications, bilingual throughout,
  both gates in scope); E1-E3 each carry recommendation + the other
  ruling's delta; D49-last-ADR verified (`plan-6-apply-seam.md:335`);
  no safeguard proposed in the document is argued back out (D62's URL
  ban is explicitly kept under the rule).

## HARVEST

- **Dominant pattern, positive**: verified-negative discipline (every
  "zero hits" claim in the doc ships its own positive control) - held up
  under re-measurement everywhere I checked, including the exact 152-hit
  pnpm-lock control. Candidate for a recorded testing convention if it
  is not one already.
- **Dominant pattern, positive**: brief-refutation as first-class output
  (section 0 kills a phantom `$ta` work item, a phantom parse-gap, a
  wrong ledger address, a wrong promotion site) - proc-57 working as
  designed.
- **Repeated rejection shape**: "no second representation of the same
  truth" - D53 (no tooltipKey), D57 (no parser), D58 (no separate
  domains.ts, no TS hardcode), D62 (no manifest). Reads like an unnamed
  house principle; convention candidate.
- **New convention candidates the design establishes** (no Tier-2 entry
  covers them today): (a) single-`v-html`-site license bound to
  first-party-input-only + CSP (D50, trigger 6 already points at it);
  (b) the closed Fluent attribute-name set {tooltip, hint,
  tooltip-<state>} (D55); (c) help-id == labelKey coupling with CI as
  rename tracker (D51/D62).
- **Funnel gap surfaced by finding 2**: eight scattered
  `$t(*.code, *.params)` IpcError render sites exist. A single
  error-display funnel (component or composable) would have made D61's
  sweep enumerable by construction - the house one-funnel pattern.
  Candidate for the Plan-9-neighborhood registry/capability work, not a
  Plan-7 defect.
- **Presentation-token fork (from finding 5)**: the house has no recorded
  rule for who fixes exact CSS values; Plan 6 designs were silent and
  implementers styled everything, this design made the latitude explicit
  and got flagged. The owner should rule once: either CSS tokens are
  design-enumerated, or a recorded narrow carve-out (non-layout,
  class-scoped presentation values) legitimizes what is already de facto
  practice.
- **Over-restriction watch** (per the grant's standing brief): no stop in
  this design was forced by the structural-conformance boundary that its
  spirit should have covered - nothing to flag in that direction. The one
  deliberate over-broad guard worth watching after build: D62 check 4
  bans `http(s)://` byte sequences anywhere in a topic file, which also
  hits URLs quoted as plain text/code examples in help prose; per the
  safeguard rule it correctly stays until built and measured - recorded
  here as calibration data only.

## Whole-document justification

This is a strong design document by the house's own standard - the
enumerations I could falsify-test mostly survived contact with the tree
(the 42-control classification, the 28-id migration set, the catalog
arithmetic, the package table, the SI-3 audit are all exactly right), the
brief-corrections are genuine and verified, and the escalation discipline
(E1-E3 with both-ruling deltas) is exemplary. But the document's central
warrant is "every set enumerated, verified, nothing left to invent", and
that warrant fails at three load-bearing points: the D57 grammar
enumeration - the single set an implementer can least afford to receive
wrong, and the one the document most emphatically claims is
verified-by-construction - is wrong in both directions (finding 1); the
D61 TS sweep misses a real render site and a real ref pair (finding 2);
and two binding-spec deviations escaped the amendment list (finding 3).
Alongside an explicit latitude clause of the exact form the process
convention bans (finding 5) and two self-contradicting counts (finding
4), the document needs a fix round before a plan can be written from it
without keyboard-side invention. Nothing found touches the architecture:
every decision (marked, eager glob, data-help-id delegation, exact-match
anchoring, typed ParamValue wire, check-i18n consolidation) stands on
verified ground and none of the findings reopens one.
