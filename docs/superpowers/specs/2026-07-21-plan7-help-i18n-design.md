# Plan 7 design: help mode + i18n cluster

Status: DRAFT 2026-07-21. Numbering starts at **D50**; the last existing ADR
is D49 (`2026-07-16-plan-6-apply-seam.md:335`), verified by sweeping `^## D`
across `docs/superpowers/specs/`.

Scope per the ROADMAP Plan-7 anchor (`docs/ROADMAP.md:68-122`): the 11 named
inputs (help-mode sidebar, editor tooltips, help-id CI guard, live locale
switch, Fluent attribute reorg, resolvedTrackLabel punctuation,
curated-domain dropdowns, field-anchored markers, ordinal column, IpcError
presence gate + number promotion, check-i18n placeable/selector parity) plus
the spec-staleness rider. Owner decisions of 2026-07-21 bind: curated help
annotation (option B), both gate items in scope, the rider in scope, all new
content bilingual en+de.

**Three forks were escalated to the governing human and are RULED
(2026-07-21)**: E1 CLI rendering (first ruled English-only, **REVERSED
same day to multilingual** - `cli-multilang-rendering`, folded in as
D63/D64), E2 view-topic set (the three spec-8.2 views), E3 help-mode
activation blocking (global suppression with allowlist). Section 7
records each ruling - both E1 events - with its analysis; the rulings
are folded into the ADRs and amendments below. **Every fork in this
document is closed.** No
design-latitude clause appears anywhere in it, in either form (explicit
permission or omission); the one implementer-owned surface (highlight
presentation tokens, D52) is covered by the owner-ratified
`latitude-carveout-presentation-tokens`, not by a latitude clause.

Grounding: v1 design spec §8.2/§8.3/§8.4/§10/§11 (authoritative; §8.3's
mechanics are binding); Tier-2 `docs/conventions.yaml`,
`docs/product-boundaries.yaml` (`gui-closed-domain-dropdowns` :419,
`editor-generic-action-keys` :404, `cli-multilang-rendering` :448
superseding `cli-english-only` :433),
`docs/process-conventions.yaml` (incl.
`latitude-carveout-presentation-tokens` :456,
`proc-proposed-safeguard-stays` :425); ledger
`gui-26`, `i18n-10`, `i18n-12`, `i18n-16`; D41/D45/D48/D49; mkvtoolnix
source and binary at v100.0 (`~/Downloads/mkvtoolnix`; `mkvmerge --version`
-> `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit`, run 2026-07-21).
Every npm registry figure below was verified against the registry or the
installed package on 2026-07-21, never from memory
(`proc-07-verify-against-source`).

---

## 0. Corrections to the brief

Each checked against the tree or the installed packages before anything was
built on it (`proc-57-briefs-not-ground-truth`).

| # | Claim | Reality |
|---|---|---|
| 1 | "Ledger context: `docs/decision-ledger.yaml` (i18n-05, i18n-10, i18n-12 among others)" | **`i18n-05` is not in the decision ledger.** The ledger's i18n ids jump from `i18n-04-param-agreement` to `i18n-06-plural-numeric-args` (`decision-ledger.yaml:1803,1815`). `i18n-05-plural-selectors` lives in **`docs/conventions.yaml:452`** (Tier-2 pattern). Its content is real and load-bearing for D61; only the address was wrong. |
| 2 | "recon says ~19 `-tooltip` keys exist" | Exact: **18** message ids ending `-tooltip` (grep `^[a-z][a-z0-9-]*-tooltip *=` over `locales/en/`), plus **4** state-variant siblings `batch-run-tooltip-{no-profile,errors,mkvmerge-missing,run-active}` (`gui-batch.ftl:63-70`), so the tooltip family is 22 ids. D55's migration table enumerates all 22. "Zero Fluent attributes anywhere" confirmed (grep `^\s*\.[a-z]` over `locales/`: no hits; the same pattern fires on a scratch catalog with an attribute, so the empty result is a verified negative). |
| 3 | Design question 7: "the `$ta` helper design" | **There is nothing to design or build: fluent-vue 3.8.2 ships `$ta` natively.** The installed package registers it as a Vue 3 global property (`node_modules/fluent-vue/dist/index.mjs:295` defaults `globalFormatAttrsName` to `"$ta"`, `:500-502` installs it; `dist/index.d.mts:16` types it `(key, value?) => Record<string, string>`). D55 adopts it; the design work is the usage convention and the check-i18n coverage, not a helper. |
| 4 | Promotion-site recon: "FirstRun.vue:94, RunHistory.vue:155, JobsView.vue:246/252, DiagnosticsPanel.vue:34" | The first three entries are verified IpcError render sites (`$t(err.code, err.params)` at exactly those lines), but the recon list is also **incomplete**: `RunHistory.vue:241` (`jobLogError`) is a fifth direct site, and `EditorView.vue:121-122` a third params ref pair (both found in review round 1; D61 carries the full eight-site inventory). **`DiagnosticsPanel.vue` is not an IpcError site and needs no promotion work**: its `$t` calls sit in one nested expression at `:46-48` (line template, severity, code), render a core `Diagnostic`, and already promote numbers via `diagnosticFluentParams` (`src/diagnosticFluentParams.ts:31`). The `Diagnostic` wire stays `Record<string,string>` by design (spec 5.2, core prose- and type-free); D61 touches only the shell's `IpcError`. |
| 5 | ROADMAP v1.x entry folded as input 11's context: "today no gate Fluent-parses the de CLI/diagnostics catalogs" | **Stale.** `e2e/catalogs.spec.ts:12` runs `assertAllCatalogsParseCleanly` (`e2e/i18n-en.ts:118-136`), which real-Fluent-parses **every `.ftl` of every locale directory, `cli.ftl` included** - the i18n-10 closure (`decision-ledger.yaml:1865`). The parse-all half of the folded entry already exists; **Plan 7's input 11 is only the placeable-set and selector-structure parity extension** (D62's sibling in D55/D61's script, section D55). The ROADMAP entry's stale sentence is listed for correction in section 8 (trigger 9). |
| 6 | "all 19 currently HAVE catalog messages (14 gui-common, 5 gui-jobs)" | Confirmed: exactly 19 production `IpcError::new` codes (12 constructed by `src-tauri/src/error.rs`'s `From` impls, 5 in `run.rs:126-935`, 2 in `lib.rs:80-146`), 14 with `gui-common.ftl` entries, 5 with `gui-jobs.ftl` entries. One nuance the brief omits: three of the 14 (`identify-failed`, `mkvmerge-not-found`, `mkvmerge-query-failed`) additionally exist in `cli.ftl` with CLI-specific wording; the frontend loader excludes `cli.ftl`, so the gui-common copies are the ones the GUI renders. No behavior consequence; recorded so D61's gate is not "fixed" to deduplicate them. |
| 7 | "check-i18n ... parser at `scripts/check-i18n.mjs:118-127` is line-based" | Confirmed, and the pre-documented extension path is real: `:118-123` states attributes are "NOT registered as ids" and directs "extend parseCatalogIds then, don't work around it", with the same note pointing at check 3's parity comparison. One drift: the brief's Plan-6-era line refs for check 2 (`:191-198`) now sit at `:212-219` (the `LABEL_KEY_RE` addition moved them). All other cited lines verified as claimed. |

Two verified premises worth restating because ADRs lean on them: the 43rd
registry field really is the sole `FixedField` (`profile_version`,
`src/editor/registries.ts:73-75`); and the editor really ships with zero
tooltips (no `title=`/`:title` anywhere in `src/views/EditorView.vue` or
`src/editor/widgets/`; the **20** `:title` sites - 19 static bindings plus
`BatchView.vue:507`'s computed `runTooltip` - all live outside the
editor; the full list is in D55's table).

---

## 1. Verified ground truth

Established by reading the tree and running the tools, not by reading about
them.

**Catalog state.** en and de each carry 7 catalogs with identical id counts
(cli 26, diagnostics 50, gui-batch 39, gui-common 41, gui-editor 45,
gui-jobs 46, gui-settings 13; counted 2026-07-21 with the same
`^id =` regex `check-i18n.mjs:127` uses). Zero Fluent attributes anywhere
(correction #2).

**The CLI renders English only, structurally.** `crates/muxsmith-cli/src/i18n.rs:7-8`
embeds `locales/en/{diagnostics,cli}.ftl` via `include_str!` and `:32-37`
loads exactly those two constants; `--locale de` today negotiates a `de`
langid over English-only resources. The rustdoc claims "v1 ships English
content only" (`:12`, `:19`) - stale as a repo statement (de shipped in
Plan 5.5), accurate as a description of this renderer today. Escalated
as E1; ruled English-only, then **RE-RULED the same day: the CLI renders
multilingually** (`cli-multilang-rendering`,
`product-boundaries.yaml:448`, superseding `cli-english-only` :433).
D63 designs the embed + fallback chain, D64 the locale-pinning audit
its `sys_locale` fallback forces on the test suite; amendments 1-3 and 5
align spec and rustdoc. A related test-surface fact, measured for D64:
**no CLI test pins a locale today** (grep `locale` over
`crates/muxsmith-cli/tests/`: only `catalog_completeness.rs`'s
catalog-file references; every `cargo_bin("muxsmith")` invocation runs
unpinned), harmless while only en is embedded, host-locale-dependent the
moment de lands.

**Frontend i18n mechanics.** Bootstrap-once: `src/main.ts:24-30` builds
bundles pre-mount from `getSettings().locale ?? navigator.language`
(`AppSettings.locale`, `src/ipc.ts:33`). The loader
(`src/i18n/index.ts`) globs `locales/*/{gui-*,diagnostics}.ftl` eagerly
with `?raw` and negotiates a `[locale, "en"]` fallback chain.
**fluent-vue 3.8.2's runtime switch is a first-class API**: the instance's
`bundles` property is a `shallowRef`-backed setter
(`node_modules/fluent-vue/dist/index.mjs:474,478-483`), every `$t` reads
the ref inside the render effect
(`dist/symbols-BJarZrcz.mjs:62-64`), the `v-t` directive wraps a
`watchEffect` (`dist/index.mjs:434-438`), and the official docs page
(fluent-vue.demivan.me/howto/change-locale, fetched 2026-07-21) prescribes
exactly `fluent.bundles = [...]`. No message cache exists to invalidate; a
swap must **replace** the array, never mutate it. `@fluent/bundle` 0.19.1's
only module global is a per-locale Intl memoizer that helps here
(`esm/memoizer.js:1-10`).

**mkvtoolnix (SI-3 source facts, full audit in section 3).** Tooltips via
the `Util::setToolTip` helper (`src/mkvtoolnix-gui/util/widget.cpp:95-113`;
HTML-wraps for word-wrap, honors a global `uiDisableToolTips` setting,
localized via gettext `QY()`), ~62 `setToolTip` calls under `merge/`. **No
in-app long-form help of any kind**: case-insensitive searches for
`whatsthis`/`QWhatsThis`/`helpEvent`/`contextHelp`/`Key_F1`/
`HelpContents` over `src/mkvtoolnix-gui/` return nothing (the same search
form finds `settooltip` in 4 files under `merge/`, so the negative is
verified, not a malformed pattern; the one `QHelpEvent` hit,
`util/fancy_tab_widget.cpp:154-166`, renders an ordinary tooltip). Long-form
help is the Help menu opening external URLs
(`main_window/main_window.cpp:230-244,328-337,654-660`) plus a **bundled
offline per-locale mkvmerge manual with online fallback**
(`main_window.cpp:662-696`).

**Markdown renderer landscape (registry-verified 2026-07-21).** No markdown
package exists in `package.json` or anywhere in `pnpm-lock.yaml`
(grep for marked/markdown-it/micromark/remark/unified/snarkdown/commonmark:
zero hits; the same grep form finds `vue|tauri` 152 times, so the negative
is verified). Vue 3.5.39's dependency closure carries no markdown
capability (`node_modules/vue/package.json`). Candidates, from
`registry.npmjs.org` + bundlephobia:

| package | latest | published | runtime deps | min+gzip | TS types | license |
|---|---|---|---|---|---|---|
| marked | 18.0.7 | 2026-07-21 | **0** | 12.5 kB | native | MIT |
| markdown-it | 14.3.0 | 2026-07-02 | 6 | 45.7 kB | @types only (lags: 14.1.2) | MIT |
| micromark | 4.0.2 | 2025-02-27 | 17 | 14.6 kB | native | MIT |
| commonmark | 0.31.2 | 2024-09-19 | 3 | 48.3 kB | @types lags API | BSD-2 |
| snarkdown | 2.0.0 | 2020-08-31 | 0 | 1.1 kB | native | MIT |

marked's own README (fetched) warns it does not sanitize output; micromark
documents safe-by-default encoding. Relevant CSP: D34's strict policy is in
force (`src-tauri/tauri.conf.json:21-29`: `default-src 'none'`,
`script-src 'self'`, no `unsafe-inline`), so inline `<script>` and inline
event handlers cannot execute in the webview regardless of what lands in
the DOM. `src/` contains no `v-html` today (grep: zero hits).

**Editor state.** 13 registries / 42 `EditableField` + 1 `FixedField`
(`src/editor/registries.ts:72-215`), 10 widget kinds + dispatcher with
`never` arm (`src/editor/widgets/`), the bespoke rule grid with 4 columns
(`src/views/EditorView.vue:463-481`) and semantic drag-reorder
(`:358-372`). `PropertyMapWidget.vue` already renders **typed** value cells
from `SETTABLE_TYPES`/`MATCHABLE_TYPES` (`src/bindings/settables.ts`,
emitted by `crates/muxsmith-core/tests/ts_export.rs::emit_settables_ts`)
with a text-cell fallback for unknown keys. **One pre-existing flaw
surfaced, not created, by this plan**: the widget resolves value types
against the *track* matchable table for every matchable map, including
`attachmentRule.select`/`drop` maps whose real domain is the attachment
property set (`validate.rs` passes `attachment_prop_type` there,
`:250-255`). Cosmetic only - core diagnoses the property anyway - recorded
as a trigger (section 8), and D58 gates its dropdowns so it does not extend
the flaw.

**Curated domains.** `TYPE_VALUES` (4 values,
`crates/muxsmith-core/src/capability/mod.rs:64`), `CODEC_KINDS` (17
aliases, `:114-133`), `CODEC_KIND_NAMES` derived (`:137-138`),
`matchable_domain` routing (`:72-78`). Not exported to the frontend today
(`settables.ts` types `type`/`codec_kind` as plain `"string"`).

**`config_path` grammar, re-derived at source (fix round 1, review
finding 1)** - the complete emission set of `config_diagnostics`
(`validate()` + `lint::provable_overlaps`, `validate.rs:189-198`), i.e.
everything `validate_profile_model` can return; plan-time diagnostics are
per-file and do not reach the editor's validate round trip. Each member
cites its constructor site; the members the first draft got wrong were
additionally **fire-verified** by running
`muxsmith validate --json` against a probe profile built to emit them,
which returned exactly `input.extensions`, `tracks[0].match.any`,
`tracks[0].source.external`, and twice `output.filename.template`
(2026-07-21). The grammar is **asymmetric and is mirrored verbatim, never
normalized**: track rules are `tracks[{i}]` (never `tracks.rules[{i}]`),
attachment rules are `attachments.rules[{i}]`.

| path | emitted at (validate.rs unless noted) |
|---|---|
| `profile_version` | :28 |
| `input.pattern` | :38 (once; nothing else anchors here) |
| `input.extensions` | :55-57 |
| `tracks.rules` | :66 (NoTrackRules) and :70-73 (PassthroughProfile). **No bare `tracks` path exists anywhere** |
| `tracks[{i}]` | lint.rs:34 (ProvableOverlap, anchored at the pair's second rule) |
| `tracks[{i}].match` | :87-91 (EmptyMatchExpression); also the expr root, sub-grammar below |
| `tracks[{i}].changes.{prop}` | :100-101 -> :374-395 |
| `tracks[{i}].source` | :107 |
| `tracks[{i}].source.external` | :112-118, locator root, sub-grammar below |
| `attachments.rules[{i}]` | :244 (AttachmentRuleShape) |
| `attachments.rules[{i}].select` / `.drop` | :250-252, expr roots |
| `attachments.rules[{i}].add` | :255, locator root |
| `output.filename` | :131 |
| `output.filename.template` | :138-143, template root |
| `chapters` | :151 |
| `chapters.external` | :155-160, locator root |
| `title` | :168 |
| `title.template` | :174-180, template root |

Sub-grammars, complete:

- **expr** (`{p}` = an expr root above, extended recursively by
  `.any[{i}]`/`.not[{i}]`): `{p}.exact.{prop}` (:265-303),
  `{p}.substring.{prop}` and `{p}.regex.{prop}` (:306-349), **bare
  `{p}.any` and `{p}.not`** (EmptyMatchList, :350-355 and :361-366),
  `{p}.any[{i}]` and `{p}.not[{i}]` (recursion, :357-359 and :368-370).
- **locator** (`{p}` = a locator root above): **bare `{p}`**
  (LocatorConflict, :460-465 - **no `.path` suffix exists; no emission
  targets the locator's `path` field**), `{p}.extensions` (:454-459),
  `{p}.match_to_source` (:466-472), `{p}.match_pattern` (:473-483, then
  the template rule).
- **template** (`{p}` = a template root above, or a
  `{locator}.match_pattern`): **every template diagnostic anchors at the
  bare root itself** (`path.to_string()` throughout `validate_template`,
  :487-530) - template paths have no suffixes.

Out of band: the editor's open flow can put one load-failure `Diagnostic`
into the same `diagnostics` ref (D42's `load_profile` envelope) whose
`config_path` is load-derived and unbounded; it takes D57's panel-only
fallback by construction and is not part of this grammar. The implementer
derives nothing: D57 fixes the anchor paths by construction from this
table.

**IpcError.** `pub params: HashMap<String, String>`
(`src-tauri/src/error.rs:44`). The only numeric-valued params in the tree:
`index`/`rules` at `error.rs:169-170,174` and `index` at `run.rs:935`. The
19-code inventory and its catalog coverage: correction #6.

**check-i18n.** Three checks (hard literal-id, warn-only unused with the
documented IpcError false-positive residual at `:42-50`, hard cross-locale
id parity `:247-297`); line-based id parser `:108-127` with the attribute
extension path pre-documented. The e2e all-locales real-parse guard exists
(correction #5). `run.rs::ftl_message` (`:539-560`) line-parses
`locales/en/gui-common.ftl` for the four `close-abort-*` keys - **those
four must stay single-line, attribute-free messages** (the catalog carries
the same pinned comment).

**eslint.** The real rule is `@intlify/vue-i18n/no-raw-text`
(`eslint.config.js:61-68`), scanning template text nodes plus the four
configured static attributes `title`, `aria-label`, `placeholder`, `alt`.
Spec 10's "eslint (no-literal-string rule)" is wrong on both name and
scope; rider amendment in section 6.

**App shell.** All three views stay mounted via `v-show`
(`src/App.vue:117-137`); FirstRun replaces the whole shell behind the
detection gate (`:63-67`); SettingsDialog is a native modal `<dialog>`
(`src/components/SettingsDialog.vue`). Apply-suggestion loads, applies and
**saves to disk** in one click (`src/views/BatchView.vue:217-248`).

---

## D50: The help sidebar renders markdown with `marked` 18.0.7, pinned exact, first-party input only

**Decision.** The sidebar renders help topics with **marked**, pinned
`"marked": "18.0.7"` in `package.json` (exact pin, matching every existing
entry's style there). Output goes into the sidebar via `v-html` - the
**only** `v-html` site the application may have. Input is exclusively the
first-party topic files under `help/` (D51); no user-influenced string
(profile content, `meta.description`, mkvmerge output, file paths, IPC
params) may ever reach `marked` or `v-html`. That is the complete license:
first-party, repo-reviewed markdown, one render site.

marked is called as `marked.parse(topicSource)` with its defaults; no
sanitizer is added. Two facts make that sound rather than sloppy: the input
trust model is "same as a Vue template" (a topic file is authored and
reviewed in the same PRs as source code), and D34's CSP
(`script-src 'self'`, no `unsafe-inline`) makes even a hypothetical
injected `<script>`/`onclick` inert as defense-in-depth. What the CSP does
**not** neutralize - markup-level injection like a phishing link - is
neutralized by the trust model plus D62's external-URL ban on topic files.
No sanitizer dependency is therefore earned; adding DOMPurify for content
we author ourselves would be cargo cult.

**Rationale.** Earned-dependency analysis on registry-verified numbers
(ground truth table): 0 runtime dependencies, 12.5 kB gzip, native types,
MIT (`deny.toml`-compatible licensing posture; npm side has no deny gate),
publish cadence measured in days. The house rule (conventions: established
well-maintained library over hand-rolling what it solves) points here; the
counter-rule (a niche/poorly-maintained dependency is debt) does not bite
on a 0-dep library with marked's maintenance record.

**Rejected: hand-rolled markdown subset.** Steelman: the content is ours,
so the feature set is closable by fiat (headings, paragraphs, lists,
emphasis, code); ~100 lines, zero dependencies, no third-party parser in a
CSP-hardened webview, and no risk of a renderer feature (raw HTML,
autolinks) we never wanted. Rejected: a hand-rolled parser is homegrown
bulk the house rule exists to prevent - edge cases (nested lists, inline
code containing `*`, escaping) are exactly where ad-hoc parsers silently
mangle content, the "closable feature set" promise decays as topics grow,
and 12.5 kB of 0-dep, natively-typed, daily-maintained library is cheaper
in total ownership than 100 lines we own forever. The subset idea also has
no forcing function: nothing would stop a topic author using unsupported
syntax that renders as garbage.

**Rejected: micromark.** Steelman: safe-by-default output encoding (its
README's own security section), the strongest correctness pedigree
(CommonMark reference-adjacent), native types. Rejected: 17 transitive
dependencies against marked's 0 for a threat model (untrusted input) this
feature does not have, and a slower release cadence.

**Rejected: markdown-it.** Steelman: the most-used plugin ecosystem, HTML
off by default. Rejected: 6 runtime deps, 45.7 kB gzip, and types only via
DefinitelyTyped currently lagging the published lib (14.1.2 vs 14.3.0) -
a standing version-skew annoyance in a TS-strict tree.

**Rejected: snarkdown / commonmark.js.** snarkdown: dormant six years (last
publish 2020, repo last pushed 2022) - fails maintenance outright.
commonmark.js: 48.3 kB, npm publish 22 months old, types lag the API.

**Interface changes:** one new npm dependency; no wire, no Rust.

---

## D51: Help-ids, topic layout `help/<locale>/<help-id>.md`, eager build-time loading

**Decision.**

- **Topic layout is spec 8.3's, verbatim and binding**: one markdown file
  per help-id per locale at `help/<locale>/<help-id>.md`, `help/` at the
  repo root beside `locales/`, with `help/en/` and `help/de/` both shipped
  (owner: bilingual).
- **help-id scheme.** A help-id is a stable kebab-case identifier of a
  *topic*, 1:1 with its file pair. For an annotated registry control the
  help-id is **the same string as its `labelKey`** (e.g.
  `editor-input-pattern`) - one identifier per control, already unique and
  stable, no second naming scheme to invent. For the non-registry topics
  the ids are bespoke and enumerated in D54's table (`view-batch`,
  `view-jobs`, `view-editor`, `batch-suggestion-card`). The equality with
  `labelKey` is a naming convention, not a derivation mechanism: the
  `helpId` value is always written out literally (D53) so the guard's
  literal scan (D62) sees it. Renaming a labelKey therefore renames its
  help-id and both topic files in the same change; D62 fails the build
  until they agree.
- **Load mechanism: eager build-time glob, mirroring the Fluent loader.**
  A new module `src/help/topics.ts` does
  `import.meta.glob("../../help/*/*.md", { query: "?raw", import: "default", eager: true })`
  - byte-for-byte the pattern `src/i18n/index.ts:18-21` established for
  `.ftl`. `topicHtml(helpId, locale)` resolves
  `help/<primarySubtag(locale)>/<id>.md`, falls back per topic to
  `help/en/<id>.md` (the same per-item fallback posture the bundle chain
  has), and renders through D50. If even the en topic is missing (only
  reachable in a build that dodged CI) it returns the raw help-id as
  visible text - the `Renderer::msg` raw-id fallback posture
  (`muxsmith-cli/src/i18n.rs:41-46`), never a silent blank.
- **Drift/parity guarantees**: D62 gives the topic tree the same CI
  guarantees the catalogs enjoy (existence per locale, no orphans, locale
  sets in lockstep). Nothing at runtime enforces parity; CI does.

**Rationale.** Eager raw import makes topics part of the built artifact:
offline by construction (the mkvtoolnix parity point it deliberately
improves on), atomic with the code that references the ids, zero runtime
I/O or fetch machinery under a CSP whose `connect-src` allows only the IPC
origin. Cost: bundle growth linear in topic count - bounded and accepted:
44 files (D54: 22 ids x 2 locales) at realistic 1-3 kB each is on the
order of 100 kB raw text in a desktop webview bundle.

**Rejected: lazy glob (`eager: false`, per-topic dynamic import).**
Steelman: topics load only when help mode opens; the main chunk stays
minimal; Vite still fingerprints and serves them from `'self'` so CSP is
untouched. Rejected: it buys startup bytes that do not matter in a Tauri
desktop app and pays with an async render path (loading states in the
sidebar, race on fast hover swaps) and a departure from the established
eager `.ftl` house pattern for the identical problem shape.

**Rejected: runtime fetch from a resource directory (Tauri asset scope).**
Steelman: topics updatable without rebuilding; smaller binary. Rejected:
adds a filesystem/asset capability surface and a packaging step for
content that versions with the code anyway; and out-of-band-updatable help
that can drift from the running binary's UI is an anti-feature. The house
already rejected runtime fetching for build-consumed data once
(`core-07-runtime-fetching-rejected`).

**Rejected: help-id decoupled from labelKey (own `help-*` namespace).**
Steelman: renaming a Fluent key would not ripple into help file names.
Rejected: two stable identifiers for the same control is a standing
mapping to maintain and get wrong; the ripple is a feature (one rename
surfaces every artifact of the control), and D62 makes the ripple a red CI
leg rather than a hazard.

**Interface changes:** new `help/` tree, new `src/help/topics.ts`. No wire.

---

## D52: Help-mode interaction: `data-help-id` attributes, delegated events, pin/hover/Esc, suppressed activation

**Decision.** Help mode is app-shell state owned by `App.vue` plus a
`src/help/state.ts` module (a `ref<boolean>` `helpMode`, a
`ref<string|null>` `pinnedId`, a `ref<string|null>` `hoverId`, and the
default-topic map
`export const VIEW_TOPICS = { batch: "view-batch", jobs: "view-jobs", editor: "view-editor" } as const`
- the module's complete contents), and a
`src/components/HelpSidebar.vue` rendered as a right-hand
`<aside aria-label="$t('help-sidebar-label')">` in a two-column layout
around `<main>`; views are not unmounted or reflowed beyond the width
change, and the sidebar scrolls independently.

**Annotation attachment: a plain `data-help-id` attribute, no directive,
no per-component wiring.** Registry controls get it generically:
`FieldWidgetDispatcher.vue` renders `:data-help-id="spec.helpId"` on the
dispatched component (Vue attribute fallthrough puts it on each widget's
root element; `undefined` renders nothing). The bespoke rule grid puts it
on its heading region (`tracks.rules`' id) per D54; the non-registry
annotated elements (D54's table) carry hand-written `data-help-id="..."`
literals in their templates. Both literal shapes are exactly what D62's
scan reads.

**Event mechanics (delegated, active only while `helpMode` is true):**

- One capture-phase `mouseover` + one `focusin` listener on `<main>`:
  `event.target.closest("[data-help-id]")` sets `hoverId` (null when the
  closest is null). Hover and keyboard focus are equivalent triggers
  (a11y).
- One capture-phase `click` listener on `<main>`: on an annotated element,
  `preventDefault()` + `stopPropagation()` and set `pinnedId` (replacing
  any prior pin); on an unannotated target, no topic change - the sidebar
  keeps its current topic (owner decision B, verbatim).
- One capture-phase `keydown` listener on `document`: `Escape` exits help
  mode **unless the settings dialog is open** (`dialogEl.open` - the
  native modal's own cancel semantics win; the dialog sits in the top
  layer above help mode anyway). `Enter`/`Space` on a focused annotated
  element pins it, activation suppressed, same as click.
- **All other activation inside `<main>` is suppressed while help mode is
  active** (the same capture click listener calls `preventDefault` +
  `stopPropagation` for unannotated targets too). The exhaustive allowlist
  of controls that stay live: the help toggle button, the three nav tab
  buttons (`nav-batch`, `nav-jobs`, `nav-editor`), the settings button,
  and the sidebar's own interior (scroll; it contains no controls).
  **RULED (owner, 2026-07-21, E3)**: this shape is final; spec amendment
  6(c) folds it into spec 8.3.

**Topic resolution**, in priority order: `pinnedId` else `hoverId` else
the active view's topic (`view-batch`/`view-jobs`/`view-editor` keyed off
`App.vue`'s `activeView`). Switching views clears `pinnedId` (a pin
highlights a visible element; a hidden `v-show` view's pin would be a
highlight nobody can see) and the sidebar falls to the new view's topic.
Exiting help mode (toggle or Esc) clears both refs and removes the
listeners.

**Toggle button**: in the `App.vue` nav, after the settings button - the
one bar every spec-8.2 view shares, satisfying "prominent, always visible
in every view" (§8.3). Fluent: `help-toggle-label` (+ `.tooltip`),
`aria-pressed` bound to `helpMode`. FirstRun and the settings dialog carry
no toggle and are not help-annotated hosts (**RULED**, owner 2026-07-21,
E2).

**Highlight**: CSS only, no layout shift - `outline` (not `border`) via
two classes the delegation code applies: `help-hover` (faint, spec's
"faint border") on the current `hoverId` element, `help-pinned`
(prominent) on the pinned element. **The semantic mappings are
enumerated and closed** (the carve-out below does not cover them): the
two-class structure itself; `help-hover` = the faint hover/focus
highlight, `help-pinned` = the visually prominent pinned marking, and
prominence must be distinguishable (the pin reads as stronger than the
hover, per spec 8.3's faint-vs-prominent pair); `outline`, not `border`,
so no layout shift. Exact colors and widths within those constraints are
implementer-owned presentation tokens under the owner-ratified
`latitude-carveout-presentation-tokens`
(`process-conventions.yaml:456`, ruled 2026-07-21 out of review round
1's finding 5) - a recorded carve-out, not a latitude clause.

**Rejected: a `v-help` custom directive.** Steelman: idiomatic Vue for
attaching behavior to arbitrary elements; could register/unregister
elements centrally and carry per-element state. Rejected: it reimplements
what one attribute plus three delegated listeners already do, puts per-node
listeners on dozens of elements, and adds an import to every annotated
template. The attribute is data; the mode is behavior; keeping them
separate is the smaller machine.

**Rejected: annotation via component prop threading (help-id as a prop on
every widget/component).** Steelman: type-checked, visible in each
component's API. Rejected: 10 widget components plus every annotated
view element would each grow a prop and a binding for what is one DOM
attribute; the dispatcher fallthrough achieves the registry half in one
line.

**Rejected: annotated-only interception (unannotated controls stay live
in help mode).** Steelman: help mode never gets in the user's way; one
listener branch less. Rejected, and the rejection was **ruled** by the
governing human (2026-07-21, E3): the asymmetry "curated annotated
controls are inert, everything else fires" is unlearnable precisely
because the annotated set is curated, and the worst-case misfire (Run,
apply-and-save) is destructive. Section 7 records the ruling.

**Interface changes:** none on the wire. New Fluent ids
`help-toggle-label` (with `.tooltip`), `help-sidebar-label`
(`gui-common.ftl`, both locales; section 2 recounts).

---

## D53: `FieldSpec` gains an optional `helpId`; tooltips are `.tooltip` attributes of the label message, not a second key field

**Decision.** In `src/editor/fieldSpec.ts`:

```ts
export interface EditableField {
  labelKey: string;
  helpId?: string;   // D54's annotated set; value always === labelKey when present
  widget: FieldWidget;
}
```

`FixedField` is unchanged (nothing renders it; it can carry no tooltip and
no help-id). No new `FieldSpec` variant: annotation is an *optional facet*
of an editable field, not a new kind of field - a variant split
(`AnnotatedField | PlainField`) would force every registry entry to choose
a shape and would duplicate the `EditableField` contract in two arms for
zero type safety gained (the discriminant would encode exactly "has
helpId").

**Tooltips add no field at all.** Every one of the 42 editable controls
gets its tooltip as the **`.tooltip` attribute of its existing `labelKey`
message** in `gui-editor.ftl` (D55's convention). The widgets render it
generically: each of the 10 widget components binds
`:title="$ta(spec.labelKey).tooltip"` on its labelled control (the same
element its `<label>`/`legend` names). Because all 42 tooltips are
mandatory (owner: tooltips are universal for the editor), presence is not
encoded per-field in TS at all - it is enforced globally by D55's
check-i18n rule "every id matched by `LABEL_KEY_RE` carries a `.tooltip`
attribute in en". A `tooltipKey` field would be 42 more literals that can
typo and a second drift surface; the attribute convention has one name and
one gate.

The 13 registries change only by adding `helpId: "<labelKey>"` lines to
exactly the 18 editor entries D54 includes (enumerated there; the other 24
entries are untouched).

**Rejected: `tooltipKey: string` on `EditableField`.** Steelman: explicit
over magic - the key is visible at the registry, greppable, and could
diverge from the label key if a shared tooltip were ever wanted. Rejected:
universality makes the field pure boilerplate (42 mechanical
`tooltipKey: labelKey + "-tooltip"` lines), divergence is not a feature
anyone asked for, and the suffixed-sibling shape is exactly what input 5
exists to remove.

**Rejected: `helpId: boolean` (derive the id from labelKey).** Steelman:
cannot typo. Rejected: the id disappears from every literal scan (D62's
guard and check-i18n see strings, not derivations), which trades a typo
class CI catches for an invisibility class it cannot. Correction: the
convention keeps the equality, the literal keeps the scannability.

**Interface changes:** `FieldSpec` shape (TS only, no wire).

---

## D54: The annotated set: 3 view topics + 19 control topics, every candidate classified

Owner decision B applied. The classification below covers **every**
candidate: all 42 editable registry fields, the bespoke grid, every
`:title`-carrying control (all 20 sites, D55's table), the app-shell
chrome (nav tabs, settings button, the new help toggle), and the
view-level surfaces (fix round 1, review finding 10: the shell chrome was
previously unclassified while the stated method implied it). Included entries get
`helpId` (registry) or `data-help-id` (template) with the id shown;
the test criterion is owner decision B's own: genuine "when to use /
interactions with other settings" content beyond tooltip depth. Excluded
entries are listed with the reason, so the cut is reviewable
(an unenumerated exclusion would be latitude by omission).

**View topics (3)** - `data-help-id` on each view's root `<section>`;
these are also the sidebar's default topics (D52):

| help-id | host | justification |
|---|---|---|
| `view-batch` | `BatchView.vue` root | the profile->validate->dry-run->run workflow, what the resolution table and diagnostics mean |
| `view-jobs` | `JobsView.vue` root | run lifecycle, cancel semantics, history/log export |
| `view-editor` | `EditorView.vue` root | model-editing concept, save semantics (D41 canonical rewrite), validate-on-edit |

**Editor controls: 18 included** (registry entry gains
`helpId: "<labelKey>"`):

| # | field | help-id | one-line justification |
|---|---|---|---|
| 1 | `Input.pattern` | `editor-input-pattern` | regex + identifier capture drive file grouping; interacts with locator `match_to_source` matching |
| 2 | `Input.extensions` | `editor-input-extensions` | gates what enters the batch; interacts with per-locator `extensions` and `recursive` |
| 3 | `OutputCfg.filename` | `editor-output-filename` | keyword `keep` vs template block; template fields/filters; collision consequences |
| 4 | `OutputCfg.on_collision` | `editor-output-on-collision` | three policies with materially different failure/overwrite behavior |
| 5 | `TemplateBlock.template` | `editor-template-block-template` | the template engine (spec 4.7/4.8): fields, filters, literal mode |
| 6 | `TrackRule.source` | `editor-track-rule-source` | `primary` vs external donor files - the whole external-locator concept and when to use it |
| 7 | `TrackRule.match` | `editor-track-rule-match-expr` | the match algebra (exact/substring/regex/any/not) as one concept |
| 8 | `TrackRule.optional` | `editor-track-rule-optional` | required-vs-optional rule semantics; interacts with uniqueness diagnostics and the no-fix partition |
| 9 | `TrackRule.changes` | `editor-track-rule-changes` | the settable property table, typed values, relation to suggestions/apply |
| 10 | `MatchExpr.exact` | `editor-match-expr-exact` | typed equality, curated `type`/`codec_kind` domains, `raw:` bypass, false-when-absent booleans |
| 11 | `Locator.match_to_source` | `editor-locator-match-to-source` | subtle donor-to-source pairing; only-true flag; interacts with `match_pattern` |
| 12 | `Locator.match_pattern` | `editor-locator-match-pattern` | template-in-regex-mode - the third text syntax, the one users conflate with plain regex |
| 13 | `TracksCfg.unmatched` | `editor-tracks-unmatched` | keep/drop policy for unmatched tracks; ordering interaction (`core-80`) |
| 14 | `TracksCfg.rules` | `editor-tracks-rules` | the grid: rule order = output track order, drag semantics, per-rule detail editing (also the grid's own topic - the bespoke grid heading carries this `data-help-id`) |
| 15 | `AttachmentsCfg.unmatched` | `editor-attachments-unmatched` | keep/drop for unmatched attachments - same control shape as #13, different domain semantics |
| 16 | `AttachmentsCfg.rules` | `editor-attachments-rules` | first-match-wins attachment rules; select/drop/add interplay lives here |
| 17 | `Profile.chapters` | `editor-profile-chapters` | keep/drop keyword vs external donor chapters |
| 18 | `Profile.title` | `editor-profile-title` | keep/clear keyword vs title template |

**Editor controls: 24 excluded**, each with the reason:

| field(s) | reason |
|---|---|
| `Profile.meta`, `.input`, `.output`, `.tracks`, `.attachments`, `.tags` (6 section wrappers) | structural containers; `view-editor` covers layout, their child controls carry the content |
| `Meta.name`, `Meta.description` | plain prose fields; tooltip depth suffices |
| `Input.recursive` | one checkbox, self-explanatory with its tooltip |
| `OutputCfg.directory` | tooltip covers it (empty = profile's own directory) |
| `ExternalBlock.external` | wrapper section; the donor concept lives in #6, the fields in #11/#12 |
| `Locator.path`, `.recursive`, `.extensions`, `.case_sensitive` | self-explanatory with tooltips; no cross-setting interaction beyond what #6 explains |
| `AttachmentRule.select`, `.drop`, `.add` | the triple's interplay is exactly #16's topic; three more topics would restate it |
| `TagsCfg.global`, `.track` | two keep/drop selects whose tooltips state the whole semantic |
| `MatchExpr.substring`, `.regex`, `.any`, `.not` | covered by #7's algebra topic; only `exact` (#10) has content beyond it (types, domains, `raw:`) |

Recount: 18 + 24 = 42 = the registry's editable set. ✓

**Non-editor controls: 1 included, all others excluded:**

| element | help-id | verdict |
|---|---|---|
| `SuggestionCard.vue` root | `batch-suggestion-card` | **INCLUDED**: one-click apply loads, narrows and **saves the profile to disk** (`BatchView.vue:217-248`), narrow-only guarantee (`core-33`), no auto-refresh afterward - consequences well beyond its tooltip |
| profile pick, recents, source/output dir, dry-run, run (BatchView) | - | tooltips + hints + the 4 run-gate state tooltips already carry the depth; workflow lives in `view-batch` |
| cancel batch, per-job cancel, log filter, history refresh/view/copy/save (JobsView family) | - | single-action controls; tooltip depth; lifecycle lives in `view-jobs` |
| settings dialog controls, FirstRun controls | - | not help-annotated hosts (RULED, owner 2026-07-21, E2); their `-hint` texts already carry the guidance |
| settings-open button (`App.vue:110`, `:title`-carrying) | - | opens a modal the sidebar cannot serve (same E2 ruling); its tooltip states the whole action |
| nav tabs `nav-batch`/`nav-jobs`/`nav-editor` (`App.vue:73-106`) | - | navigation chrome, live during help mode (D52 allowlist); the view topics they lead to ARE the content |
| help toggle button (new, D52) | - | operates the help mechanism itself; annotating it would pin its own topic (self-referential); its `.tooltip` carries usage |
| SuggestionCard copy/apply buttons (`:80`/`:94`, `:title`-carrying) | - | subsumed by the card's own `batch-suggestion-card` region annotation above; two child topics would restate it |
| editor Open/Save buttons | - | Open is a file pick; Save's semantics are carried by the standing D41 save-surface note already on that surface |
| `editor-action-add`/`-remove` buttons, grid ordinal column | - | generic list actions / presentation column; no per-instance content |

**Totals: 22 help-ids** (3 view + 18 editor + 1 batch), **44 topic files**
(en+de). In help mode, hovering anything not in this table sets no hover
topic (D52: `hoverId` becomes null); the sidebar then shows the pinned
topic if one is pinned, else the active view's topic - a visible
fallback when an annotated element's topic was showing unpinned. D52 is
the interaction authority; this sentence follows its mechanics exactly
(aligned at plan-authoring - the earlier "does nothing and keeps its
topic" paraphrase contradicted them). Topic *content* wording is
authored at implementation and goes
through the owner's plan-close rendered-surface pass (the standing route
for user-visible strings, `latitude-carveout` occurrence 2026-07-16); the
id set, file set and host elements above are closed here.

**Rejected: universal annotation (every control gets a topic).** Steelman:
no inclusion judgment to maintain; the guard is trivially "every control".
Rejected by the owner's explicit option-B ruling ("explicitly NOT
inflationary"); 24 of the editor's controls have nothing to say beyond
their tooltip, and empty near-duplicate topics train users to ignore the
sidebar.

**Rejected: per-severity/per-widget-kind derivation of the set.** Any rule
("all keywordOrBlock fields") misses the actual criterion, which is
content, and produces exactly the wrong members on both sides
(`chapters` in, `tags.global` out, under every mechanical rule tried).
A judged, enumerated set with a completeness gate (D62) is the honest
form.

**Interface changes:** none on the wire; registry literals + template
attributes per the tables.

---

## D55: Fluent attribute reorg: 22 tooltip ids + 6 hint ids fold into attributes; `$ta` adopted; check-i18n learns attributes and placeable/selector parity

**Decision.** Widget facets become Fluent **attributes of the message they
facet**; suffixed sibling ids are removed. The complete migration set -
**27 ids fold away, 1 is renamed in place** (the 22-id tooltip family
loses 21 by removal and 1 by rename; the 6 hint ids are all removed), none
is left half-migrated:

**Tooltip folds (18 + 4 variants):**

| base message (file) | folded sibling(s) -> attribute(s) | template site(s) |
|---|---|---|
| `settings-open-label` (gui-common) | `settings-open-tooltip` -> `.tooltip` | `App.vue:110` |
| `browse-button` (gui-common) | `browse-button-tooltip` -> `.tooltip`; **`batch-browse-dir-tooltip` (gui-batch) -> `.tooltip-directory`** | `SettingsDialog.vue:109`, `FirstRun.vue:113`; `BatchView.vue:403,426` |
| `firstrun-use-path` (gui-common) | `firstrun-use-path-tooltip` -> `.tooltip` | `FirstRun.vue:124` |
| `firstrun-retry` (gui-common) | `firstrun-retry-tooltip` -> `.tooltip` | `FirstRun.vue:132` |
| `settings-save` (gui-settings) | `settings-save-tooltip` -> `.tooltip` | `SettingsDialog.vue:154` |
| `settings-cancel` (gui-settings) | `settings-cancel-tooltip` -> `.tooltip` | `SettingsDialog.vue:161` |
| `batch-profile-pick` (gui-batch) | `batch-profile-pick-tooltip` -> `.tooltip` | `BatchView.vue:348` |
| `batch-dry-run` (gui-batch) | `batch-dry-run-tooltip` -> `.tooltip` | `BatchView.vue:446` |
| `batch-suggestion-copy` (gui-batch) | `batch-suggestion-copy-tooltip` -> `.tooltip` | `SuggestionCard.vue:80` |
| `batch-suggestion-apply` (gui-batch) | `batch-suggestion-apply-tooltip` -> `.tooltip` | `SuggestionCard.vue:94` |
| `batch-run` (gui-batch) | `batch-run-tooltip` -> `.tooltip`; `-no-profile` -> `.tooltip-no-profile`; `-errors` -> `.tooltip-errors`; `-mkvmerge-missing` -> `.tooltip-mkvmerge-missing`; `-run-active` -> `.tooltip-run-active` | `BatchView.vue:507` + `runDisabledReason`/`runTooltip` (`:290-307`) switch to attribute names + `$ta("batch-run")[attr]` |
| `jobs-cancel-batch-label` (gui-jobs) | `jobs-cancel-batch-tooltip` -> `.tooltip` | `JobsView.vue:262` |
| `jobs-row-cancel-label` (gui-jobs) | `jobs-row-cancel-tooltip` -> `.tooltip` | `JobRow.vue:66` |
| `jobs-history-refresh` (gui-jobs) | `jobs-history-refresh-tooltip` -> `.tooltip` | `RunHistory.vue:145` |
| `jobs-history-copy-log` (gui-jobs) | `jobs-history-copy-tooltip` -> `.tooltip` | `RunHistory.vue:251` |
| `jobs-history-save-log` (gui-jobs) | `jobs-history-save-tooltip` -> `.tooltip` | `RunHistory.vue:261` |

**The one baseless tooltip**: `batch-recents-select-tooltip` has no base
message (the control's visible text is the recent path itself, data not
prose). It becomes a **value-less message with an attribute**
(valid Fluent):

```ftl
batch-recents-select =
    .tooltip = Open this profile again.
```

Template site `BatchView.vue:370`. Net id count unchanged (one id out, one
in); this keeps the invariant "every tooltip in the app is a `.tooltip`
attribute" total, with zero exceptions.

**Hint folds (6 pairs = 6 ids):** the `label`+`hint` pairs are
the other suffixed-sibling widget facet in the tree; same fold:

| base message | folded sibling -> `.hint` | template site |
|---|---|---|
| `firstrun-picker-label` (gui-common) | `firstrun-picker-hint` | `FirstRun.vue:106` region |
| `settings-mkvmerge-path-label` (gui-settings) | `settings-mkvmerge-path-hint` | `SettingsDialog.vue:102` |
| `settings-default-jobs-label` (gui-settings) | `settings-default-jobs-hint` | `SettingsDialog.vue:124` |
| `settings-locale-label` (gui-settings) | `settings-locale-hint` (text also rewritten by D56) | SettingsDialog locale row |
| `batch-source-label` (gui-batch) | `batch-source-hint` | `BatchView.vue:392` |
| `batch-output-label` (gui-batch) | `batch-output-hint` | `BatchView.vue:415` |

The `aria-describedby` wiring at those sites is unchanged; only the `$t`
of the hint id becomes `$ta(labelId).hint`.

**Enumerated non-migrations** (facet-shaped ids deliberately left as
messages, so the sweep is visibly complete): `firstrun-guidance-*` (4,
OS-selected alternative values, not facets of one widget),
`severity-*` (3) and `jobs-state-*` (6) (dynamically-selected value
vocabularies), `settings-locale-option-en`/`-de` (option list),
`jobs-row-output-pending`, `jobs-row-progress-label`,
`jobs-log-region-label`, `jobs-log-filter-label`,
`jobs-history-run-label`, `jobs-history-log-region-label`, `nav-label`
(standalone accessible names with no base message), and the four
`close-abort-*` keys (**hard constraint**: `run.rs::ftl_message`'s line
parser consumes them, `run.rs:539-560`; they stay single-line, valueful,
attribute-free).

**New attributes beyond migration**: the editor's 42 `.tooltip`
attributes (D53) on the 42 `gui-editor.ftl` label messages, and
`help-toggle-label.tooltip` (D52). All attributes land en+de in the same
change (i18n-16).

**Naming convention, closed**: attribute names are `tooltip`, `hint`, and
`tooltip-<state>` for state-variant tooltips. No other attribute name may
be introduced by the plan.

**`$ta` adoption**: fluent-vue's native `$ta` (correction #3) in
templates; `useFluent().formatAttrs`/`$ta` in script (the `runTooltip`
computed). No wrapper is written.

**check-i18n extensions** (all in `scripts/check-i18n.mjs`, which stays
the single i18n gate; the parser stays line-based per its own charter):

1. **Attribute-aware catalog model.** `parseCatalogIds` becomes
   `parseCatalog`, returning per file: message ids, per-id attribute-name
   sets, and per-id/per-attribute **pattern bodies** (the id line's value
   plus its indented continuation lines - the same line discipline the
   header documents). Terms stay unregistered.
2. **Check 1 extension**: literal `$ta("id")` / `ta("id")` calls scanned
   with the `CALL_RE` mechanics (`(?<![\w$])\$?ta\(\s*(['"])...`); the id
   must exist - hard fail. Attribute *member* access after `$ta(...)` is
   not statically resolved (same posture as dynamic `$t` keys: skipped,
   never flagged); coverage comes from rules 3-5.
3. **Editor tooltip completeness** (the input-2 guard): every id matched
   by `LABEL_KEY_RE` must carry a `tooltip` attribute in the en catalog -
   hard fail naming the id.
4. **Check 3 parity extension**: for every id shared across locales, the
   **attribute-name set must be identical** to en's - hard fail (missing
   and extra attributes both).
5. **Placeable-set and selector-structure parity per pattern (ledger
   i18n-12, the D39 auto-guard)**: for every message value and every
   attribute value, compare against en: (a) the set of placeable variable
   references (`$name` occurrences anywhere in the pattern, variants
   included) must be equal; (b) the number of select expressions must be
   equal; (c) each select expression's selector variable must match; (d)
   variant-key sets must be equal **except** keys that are CLDR plural
   categories (`zero`, `one`, `two`, `few`, `many`, `other`) or numeric
   literals (`[0]`, `[1]`, ...), for which the rule is instead: at least
   one variant, exactly one `*`-default. Hard fail. The carve-out exists
   because plural-category sets legitimately differ per locale (a future
   `ru` needs `few`/`many`); without it the gate would forbid a correct
   catalog. Non-plural selectors (the D39 `$property` kind) get full key
   equality - exactly the drift class that motivated the entry.

Rules 4-5 run wherever check 3 runs (all `.ftl` including `cli.ftl`).

**Rejected: keep suffixed siblings, skip the reorg.** Steelman: the
migration touches 28 ids, 2 locales, the 20 `:title` sites plus the 6
hint render sites, and the e2e fixture layer for zero user-visible
change, and the sibling convention works today. Rejected: the anchor input is an owner-scoped roadmap item;
attributes are Fluent's own mechanism for exactly this ("widget facets"),
they co-locate a control's prose (the `browse-button` case currently
splits one control's label and tooltip across two *files*), and the
check-i18n attribute machinery is prerequisite work for the editor's 42
tooltips either way - doing those as 42 more suffixed keys would double
the eventual migration.

**Rejected: one `.tooltip` attribute with a `$state` selector instead of
four `batch-run` variant attributes.** Steelman: one attribute, Fluent
selecting on a state param, fewer names. Rejected: the state is decided by
TS control flow (`runDisabledReason`, `BatchView.vue:290-305`) that
returns per-state ids today; moving the branch into Fluent splits one
decision across two languages and makes the unchosen branches invisible to
the parity gate's per-attribute check.

**Rejected: renaming `-label` bases while folding (e.g.
`settings-open-label` -> `settings-open`).** Steelman: post-fold, the
`-label` suffix is vestigial naming. Rejected: renames ripple into e2e
fixtures and both locales for cosmetics; id stability is worth more than
suffix aesthetics, and D51 couples help-ids to labelKeys, raising rename
cost further. No id is renamed by this plan except
`batch-recents-select-tooltip` -> `batch-recents-select` (forced: the
tooltip becomes an attribute and needs a message to live on).

**Interface changes:** catalog id/attribute sets (section 2 recounts every
file); `scripts/check-i18n.mjs`; the 20 `:title` sites (incl.
`runTooltip`) plus the 6 hint render sites; e2e
`i18n-en.ts` fixture helpers gain attribute rendering for the migrated
assertions.

---

## D56: Live locale switch: `fluent.bundles` swap via an owned i18n module; settings write unchanged; views keep state

**Decision.** A new module `src/i18n/fluent.ts` owns the fluent-vue
instance and the reactive locale:

```ts
export const currentLocale = shallowRef("en");
export const fluent = createFluentVue({ bundles: buildBundles("en") });
export function applyLocale(locale: string): void {
  currentLocale.value = locale;
  fluent.bundles = buildBundles(locale);        // fresh array, never mutation
  document.documentElement.lang = primarySubtag(locale);
}
```

`src/main.ts` imports `fluent` and calls `applyLocale(resolvedLocale)`
before mount (bootstrap behavior unchanged: first paint in the right
locale). `SettingsDialog.save()` (`SettingsDialog.vue:53-73`), after a
successful `setSettings`, calls `applyLocale(next.locale)` when
`next.locale !== baseline.locale`. That is the entire mechanism - verified
against the installed fluent-vue 3.8.2 (setter is a `shallowRef`,
`dist/index.mjs:474-483`; every `$t`/`$ta`/`v-t` tracks it; official
recipe identical; no cache to invalidate, no cleanup, ground truth).
`primarySubtag` is exported from `src/i18n/index.ts` (it exists there
today) rather than duplicated.

**What re-renders and what does not, enumerated:**

- All mounted views re-render text in place. Views are `v-show`-mounted
  (`App.vue:117-137`), so **no state is lost**: editor model/diagnostics,
  batch report, jobs run state all survive; only strings change.
- The help sidebar re-renders its topic through `currentLocale` (D51's
  `topicHtml` takes it as a reactive input).
- Diagnostics/IpcError text re-renders (all through `$t`).
- **Not** re-rendered, all pre-existing and recorded: the native
  close-abort dialog (en-only `include_str!`, `run.rs:543`; the de catalog
  header documents the residual), the CLI (locale is resolved
  **per invocation** - D63's `--locale`/system/en order at process start;
  a GUI live switch neither reaches nor needs to reach a separate
  process), the static `index.html` window title, native OS chrome (file
  pickers).
- `settings-locale-label.hint` (post-D55 fold) drops its "takes effect
  after restarting Muxsmith" sentence - now false. Replacement wording is
  evergreen (no locale enumeration, `i18n-15-settings-hint-evergreen`),
  final text via the owner's rendered-surface pass.

**Rejected: full reload (`location.reload()`) on locale change.**
Steelman: trivially correct, exercises the bootstrap path, no reactivity
reasoning at all. Rejected: it destroys live state (a running batch's
JobsView listeners, an unsaved editor model - real data loss), and it
reloads to work around a reactivity mechanism the library ships and
documents for exactly this purpose.

**Rejected: app remount (unmount + `createApp` again) without process
reload.** Same state destruction as reload minus the honesty; strictly
dominated.

**Rejected: restart-notice status quo (gui-26 as shipped).** Steelman:
zero code, zero risk. Rejected: gui-26 is a fired anchor input of this
plan by owner call; the deferral existed because Plan 5.5 could not spare
the mechanism work, which is now this ADR.

**Interface changes:** `src/i18n/fluent.ts` (new), `src/main.ts`,
`SettingsDialog.vue`, one hint text (en+de). No wire change
(`AppSettings.locale` already exists and is already written).

---

## D57: Field-anchored markers: exact-string path anchoring built where widgets render; panel stays complete; no parser anywhere

**Decision.** The mapping from `Diagnostic.config_path` to a control is
**exact string equality against paths the widget tree constructs while
rendering**, mirroring core's emission grammar verbatim (ground truth). No
parser exists on either side of the wire for this: core's paths are opaque
keys; the frontend builds the same keys by construction and looks them up.

Mechanics:

- `EditorView` computes
  `diagnosticsByPath: Map<string, Diagnostic[]>` from its existing
  `diagnostics` ref (grouped by `config_path`) and `provide()`s it
  (Vue injection key `editorDiagnosticsByPath`).
- Every widget receives a new optional `path?: string` prop, threaded by
  the composition layer: `FieldWidgetDispatcher` passes it through;
  `SectionWidget` appends `.{fieldKey}` per child using the **serialized**
  field name; `ListWidget` appends `[{i}]` per item; `KeywordOrBlockWidget`
  passes its own path to its block section unchanged (core's grammar:
  `output.filename.template`, no extra segment for the block);
  `PropertyMapWidget` appends `.{rowKey}` per row. The roots: the
  top-level section composition in `EditorView` starts each
  `profileFields` entry at its serialized name (the eight editable ones:
  `meta`, `input`, `output`, `tracks`, `attachments`, `chapters`, `tags`,
  `title`); the bespoke rule grid supplies `tracks[{i}]` per row (which
  also anchors lint's `ProvableOverlap`) and `tracks[{i}]` as the detail
  panel's root; `tracks.rules` anchors at the grid heading - **and
  nothing anchors a bare `tracks`, because no such path exists** (section
  1). **The grammar's asymmetries are mirrored, not normalized**: rule
  paths are `tracks[{i}]` (never `tracks.rules[{i}]`), attachment rule
  paths are `attachments.rules[{i}]`, `matchExpr` paths go through
  `.match`/`.select`/`.drop` (the serialized names) - all exactly as the
  section-1 table formats them. Three anchor placements the section-1
  re-derivation makes explicit, so the plan transcribes rather than
  derives them: **bare `{p}.any` / `{p}.not`** (EmptyMatchList) anchor at
  the `ListWidget` root rendered for the `any`/`not` field (its own path,
  before any `[{i}]` is appended); the **bare locator root**
  (LocatorConflict) anchors at the locator `SectionWidget` rendered for
  `externalBlock.external` / `attachmentRule.add` (path examples:
  `tracks[0].source.external`, `chapters.external`,
  `attachments.rules[2].add`); and **template diagnostics anchor at the
  bare template path** (`output.filename.template`, `title.template`,
  `{locator}.match_pattern`), i.e. at the `TextWidget` for the `template`
  / `match_pattern` field - there are no deeper template paths.
- A widget (and the grid row) that finds its path in the map renders the
  marker; everything else renders nothing. **Paths with no rendered
  control** (`profile_version`, and any future core path the tree does not
  know) are silently panel-only - the panel is unchanged and always
  complete, so the marker layer is strictly additive and its miss mode is
  the status quo.

**Marker UX** (the recorded Plan-6 shape of spec 8.2's markers is the
panel; this is the field-anchor sibling, per the anchor input):

- One marker element per anchored control: a `<span class="diag-marker
  diag-marker--{severity}">` rendered inside the widget's label/legend
  line, plus a `diag-anchored--{severity}` outline class on the widget's
  input element.
- Severity is the **worst** of the path's diagnostics
  (error > warning > info; the three severities are the complete set,
  `report/mod.rs:242-255`); the marker's accessible name is
  `$t("severity-{severity}")` (existing keys, `diagnostics.ftl:1-3`); its
  `title` is the rendered messages of all diagnostics at the path
  (`$t(d.code, diagnosticFluentParams(d.code, d.params))`, joined by
  newline - the exact render the panel uses, `DiagnosticsPanel.vue:48`).
- Error-severity markers additionally set `aria-invalid="true"` on the
  anchored form control where one exists (text/number/select/checkbox
  inputs); section/list/grid anchors carry only the marker span.
- No new Fluent keys: severity names and diagnostic messages all exist.

**Parser locus - rejected: reusing the Rust-side `ApplyError` parsing.**
Steelman: "reuse before writing"; core already parses a config path
(`rule_index_of`, `planner.rs:2032`) and D43's shell mapping sits at
`error.rs:161-178`. Rejected on D43's own recorded boundary:
`rule_index_of` parses **`Suggestion.config_path`**, a deliberately narrow
token (`tracks[<N>].match` only), and D43 warns against conflating it with
`Diagnostic.config_path`, the general field this feature consumes. There
is nothing to reuse; a new core parser + IPC round trip to interpret
strings the frontend can key on directly would be pure cost.

**Rejected: a TS-side grammar parser (parse paths into segments, resolve
against the registries).** Steelman: robust to novel paths - a parser
could anchor `tracks[3].match.exact.foo` at its nearest rendered ancestor
by walking segments. Rejected: interpretation invites exactly the drift
the exact-match design cannot have (a parser's idea of the grammar vs
core's format strings), the prefix-walking benefit is small (the panel
already catches everything), and the failure mode of exact matching is
visible and safe (panel-only) while the failure mode of a wrong parse is a
marker on the wrong control.

**Rejected: nearest-ancestor prefix fallback on top of exact matching.**
Steelman: a deep unanchored path still gets a nearby marker. Rejected for
v1 of this feature: it reintroduces path interpretation (segment
boundaries) through the back door, and the enumeration above already
anchors every path core emits today at its exact depth - the fallback
would fire only for future grammar, which is precisely when a silent
approximate anchor is least trustworthy.

**Interface changes:** `path` prop on the 10 widgets + dispatcher;
provide/inject key; CSS classes. No wire change.

---

## D58: `type`/`codec_kind` dropdowns in exact-match value cells; domains exported beside the type maps; `raw:` and out-of-domain fall back to text

**Decision.**

- **Export**: `emit_settables_ts` (`crates/muxsmith-core/tests/ts_export.rs`)
  additionally emits, into the same `src/bindings/settables.ts`:
  `export const TYPE_VALUES = ["audio", "buttons", "subtitles", "video"] as const;`
  and `export const CODEC_KIND_NAMES = [...] as const;` (17 entries),
  sourced from `capability::TYPE_VALUES` / `capability::CODEC_KIND_NAMES`.
  One emitter keeps one file whole; the D44 CI drift gate
  (`ci.yml:135-139`, directory-scoped) covers the change with no CI edit.
- **Rendering boundary, per the decree** (`gui-closed-domain-dropdowns`,
  `product-boundaries.yaml:419`): only the **exact-match value cells** -
  in widget terms `propertyMap` with `properties: "matchable"` **and**
  `values: "scalar"`, which is exactly `matchExpr.exact`
  (`registries.ts:198-201`); `substring`/`regex` cells
  (`values: "string"`) and `changes` cells (`properties: "settable"`;
  `type`/`codec_kind` are not settable, `capability/mod.rs:104-110`) are
  outside it by construction.
- **Cell resolution**, extending `cellKindFor`
  (`PropertyMapWidget.vue:88-115`) with a `select` kind resolved **before**
  the scalar-type switch. A row's value cell renders a `<select>` iff all
  four hold: (1) the widget is matchable+scalar; (2) the row's *track*
  context holds - the D57 `path` prop starts with `tracks[` (the
  attachment `select`/`drop` maps share `matchExprFields` but have a
  different property universe, ground-truth flaw note; gating on path
  keeps the dropdown out of a context where `type` is not even a valid
  property); (3) the key is exactly `type` or exactly `codec_kind`
  (byte equality - a `raw:type` key fails this and keeps its free-text
  cell, preserving the `raw:` bypass by construction, spec 9.2/D32);
  (4) the current value is `""` (a fresh row) or a domain member. The
  select's options are the domain array plus, for the `""` case, one empty
  placeholder option; selecting writes `Scalar::Str` exactly as the text
  cell does.
- **Out-of-domain existing value** (e.g. a loaded profile with
  `type: vido`): condition (4) fails and the cell stays a **text input**
  with the value intact - the dropdown must never eat data it cannot
  represent. Core's `InvalidPropertyValue` (validate.rs:295) plus D57's
  marker make the defect visible at the cell; once the user enters a
  domain member, reactivity re-resolves the cell to a select.

**Rejected: a select that includes the invalid current value as an extra
option.** Steelman: the control stays a dropdown in all states, visually
consistent. Rejected: it presents a known-invalid token inside the
closed-domain affordance whose entire message is "these are the legal
values" - a dropdown offering `vido` is wrong by the decree's own logic
('alles andere wäre komisch' cuts both ways).

**Rejected: exporting domains as a separate `domains.ts` binding.**
Steelman: `settables.ts` is documented as the type maps; a value-domain
export is a third artifact kind like `keywords.ts`. Rejected: the consumer
(`PropertyMapWidget`) already imports from `settables.ts`, the emitter
already owns that file, and a fourth binding file means a fourth emitter
and import for two arrays; the file's doc header is updated to say "type
maps + curated matchable value domains" instead.

**Rejected: hardcoding the four/17 values in TS.** Violates D46's
single-source rule for these exact constants; the drift gate exists so
this never has to be argued again.

**Interface changes:** `settables.ts` content (committed generated
artifact), `PropertyMapWidget.vue` (`select` cell kind + path gate). No
wire change.

---

## D59: Ordinal column: presentation-only 1-based index, one new header key

**Decision.** The rule grid (`EditorView.vue:463-481`) gains a leading
column: header `$t("editor-track-rule-order")` (new key, en "Order" / de
per the rendered-surface pass), cell content `{{ index + 1 }}` - the
1-based render of the row's array position. **Confirmed no data change**:
order remains encoded solely as `tracks.rules` array position (the model
has no order field to update - `bindings/profile.ts` `TrackRule` carries
`source`/`match`/`optional`/`changes` only), drag-reorder mechanics
(`:358-372`) are untouched, and the ordinal re-renders reactively because
`v-for` re-runs on the rebuild. The number is plain data (locale-neutral
digit), not prose - no per-row Fluent key.

**Rejected: making the ordinal cell the drag handle.** Steelman: a
dedicated handle is the common grid idiom and would free the rest of the
row for text selection. Rejected: rows are already whole-row draggable
(`draggable="true"` on `<tr>`, shipped in Plan 6); narrowing the drag
surface is a behavior change outside this cosmetic item's anchor.

**Interface changes:** one `gui-editor.ftl` key (en+de), one template
column. `gui-editor.ftl` goes 45 -> 46 ids (section 2). This is a
**budget revision** of `editor-generic-action-keys`
(`product-boundaries.yaml:404`), surfaced as such (fix round 1, review
finding 7): that entry fixes the budget at 45 as "a hard boundary against
prose growth (tooltips remain Plan 7)" - it reserves the *tooltip pass*,
which D53/D55 deliver as attributes adding **zero** ids, so the 46th id
is growth the entry does not cover. Substantive cover is the ordinal
column's own owner-scoped ROADMAP anchor input; the precedent is the
owner-ruled 43 -> 45 revision recorded in the same entry. The controller
updates the entry's statement (45 -> 46, tooltip-attribute note) when
consuming this design - named as trigger 10.

---

## D60: `resolvedTrackLabel` punctuation moves into Fluent; the "-" placeholder decision is respected and stays in code

**Decision.** New `gui-batch.ftl` key
`batch-resolved-track = { $id } ({ $kind })` (de: identical pattern - the
point is that the composition is catalog-controlled, not that the two
locales differ today). `ResolutionTable.vue:24-26` becomes:

```ts
a.track_id === null ? "-" : $t("batch-resolved-track", { id: a.track_id, kind: a.track_kind })
```

Two recorded prior decisions bound this ADR, and it respects both rather
than drifting past them (`ResolutionTable.vue:12-21`): the bare `"-"` for
an unmatched track **stays a code-side literal**, deliberately un-Fluent,
mirroring the CLI's identical unlocalized `"-"`
(`muxsmith-cli/src/commands/mod.rs:103-106`, rendered through
`dry-run-assignment`) - that is a cross-surface consistency decision this
plan does not supersede; and `track_kind` stays an untranslated mkvmerge
vocabulary passthrough (the comment's spec-8.4 third-party analogy). The
component comment is updated to note the punctuation now lives in the
catalog while those two decisions stand.

**Rejected: fold the "-" into the Fluent message as a selector
(`{ $id -> [none] - ... }`).** Steelman: one render path, zero code-side
string literals. Rejected: it silently supersedes the recorded CLI-parity
decision for cosmetic uniformity, and encodes a sentinel ("none") into the
wire contract where the wire today has an honest `null`.

**Rejected: leave as-is.** Steelman: `(` `)` are locale-neutral in every
plausible v1 locale. Rejected: the anchor input is a fired roadmap item;
parenthesized appositions are genuinely locale-variable (CJK full-width
parens), and the fix is one key.

**Interface changes:** one key (en+de), one component line. `gui-batch`
count in section 2.

---

## D61: IpcError codes gated by check-i18n against the GUI catalogs; params typed `string|number` end to end

**Decision.**

**Presence gate** - a new hard-fail check in `scripts/check-i18n.mjs`
(the same run, no new CI step): scan `src-tauri/src/**/*.rs`, taking each
file's content **up to its first `#[cfg(test)]` line** (the house's own
line-based discipline; test modules sit at file bottoms in this tree),
extract every `IpcError::new("...")` literal, and require each code to be
a message id in the en GUI-visible catalog set (`knownIds`, i.e.
`gui-*` + `diagnostics`). A code with no message is a hard failure naming
the Rust site. This codifies the currently-holding invariant (all 19 codes
have messages, correction #6) so it can never silently un-hold. The same
extracted set is added to check 2's `usedIds`, which **closes the
documented false-positive residual** at `check-i18n.mjs:42-50` (those
comments are updated in the same change - the exemption prose describes a
gap this check removes).

**Rejected: a Rust-side exhaustiveness test (the
`catalog_completeness.rs` pattern).** Steelman: it is the established
house gate for exactly this shape, and it runs where the codes live.
Rejected because the precondition that makes `catalog_completeness.rs`
work is absent here: `DiagCode` is an enum, so a `match` forces
exhaustiveness by construction; IpcError codes are **stringly**
(`error.rs:33-44`) - a Rust test would have to grep its own source text
for literals, which Node already does with machinery check-i18n owns
(`CALL_RE`/`LABEL_KEY_RE` precedents). Same scan either way; the JS home
also lets the result feed check 2. A Rust enum for the codes was
considered and rejected as out of this plan's scope (it would touch all
19 construction sites and the wire shape for a purity gain the gate
already delivers).

**Rejected: both gates.** Two scanners drift; the second adds no failure
class the first misses.

**Number promotion.** The wire and both ends become typed:

- Rust: `IpcError.params` becomes `HashMap<String, ParamValue>` with

  ```rust
  #[derive(Debug, Clone, PartialEq, Serialize)]
  #[serde(untagged)]
  pub enum ParamValue { Num(u64), Str(String) }
  ```

  and `From<&str>/From<String>/From<usize>` impls; `.with` takes
  `impl Into<ParamValue>`. **The complete promotion-site set** (every
  numeric-semantic param in the tree, ground truth): `error.rs:169`
  (`index`), `:170` (`rules`), `:174` (`index`) - the three lose their
  `.to_string()` - and `run.rs:935` (`index`). Every other `.with` site
  stays a string (`detail`, `found`, `minimum`, `path`, `property`,
  `run_id`, `file`, and `code` at `error.rs:87-91`, where the
  `"signal"` fallback for a signal-killed process forces string).
- TS: `src/ipc.ts` `IpcError.params: Record<string, string | number>`;
  the **three** IpcError-params ref pairs typed accordingly
  (`SettingsDialog.vue:16`, `BatchView.vue:48`, `EditorView.vue:121-122`
  - the third was missed by the recon and found in review round 1). The
  complete render-site inventory (re-measured, fix round 1): **five**
  direct `$t(x.code, x.params)` sites - `FirstRun.vue:94`,
  `RunHistory.vue:155` and `:241` (`jobLogError`, the site that renders
  `job-log-not-found`'s promoted `index`), `JobsView.vue:246`, `:252` -
  plus the **three** ref-fed sites `EditorView.vue:436`,
  `BatchView.vue:438`, `SettingsDialog.vue:93`. All eight pass params
  through unchanged - fluent-vue's params type accepts
  `string | number`, so **no per-site promotion table exists for
  IpcError**, unlike diagnostics.
- The `Diagnostic` wire is **untouched** (spec 5.2: core stays type-free
  on the wire; `diagnosticFluentParams` and
  `cli::i18n::msg_with_counts` remain the diagnostic-side promotion
  pattern, correction #4).
- Catalog: with `$rules` now numeric, `apply-rule-index-out-of-range`
  (`gui-common.ftl`) gains the CLDR plural selector i18n-05 mandates:
  `(rule count: { $rules -> [one] 1 rule *[other] { $rules } rules })`,
  en+de. That is the **only** message that gains a selector: `$index` in
  all three apply/job-log messages is an identifier, not a count, and
  plural-selecting an identifier would be wrong. D55's parity rule 5
  guards the new selector cross-locale from the same commit.

**Rejected: render-boundary promotion for IpcError (a
`NUMERIC_IPC_PARAMS` lockstep table like `diagnosticFluentParams`).**
Steelman: symmetric with the existing house pattern, zero Rust change,
zero wire change. Rejected: the diagnostic pattern exists because spec 5.2
*forbids* typing that wire (core must stay prose- and type-free toward
its remote consumers); `IpcError` is a shell-local type with no such
constraint, so the honest fix is the typed wire - the lockstep list is
the workaround shape, and it rots (its own doc comment pleads "keep both
lists in lockstep").

**Rejected: `serde_json::Value` params.** Steelman: no new enum.
Rejected: admits bool/null/arrays the renderer cannot interpolate; the
two-variant enum is the exact contract.

**Interface changes:** `IpcError` wire shape (string params serialize
identically; numeric params change from `"3"` to `3` - the e2e IPC mocks
that fabricate IpcErrors are swept for numeric params in the same
change), check-i18n check + comment update, one catalog message (en+de).

---

## D62: Help-id completeness: check-i18n gains the topic-tree gate, both directions, per locale, plus the external-URL ban

**Decision.** The guard lives in `scripts/check-i18n.mjs` (with D55/D61's
extensions; one i18n gate, one CI step - spec 10 names help-id
completeness in the same sentence as the catalog checks). Hard-fail
checks, all four:

1. **Referenced -> file, per locale**: collect the referenced help-id set
   as the union of (a) `helpId:\s*(['"])([^'"]*)\1` literals in
   `src/**/*.{vue,ts}` and (b) `data-help-id="..."` literals in the same
   files and (c) the `VIEW_TOPICS` values in `src/help/state.ts`,
   extracted by a dedicated scan of that one file for quoted
   `view-`-prefixed literals (`/['"](view-[a-z-]+)['"]/g`) - corrected in
   fix round 1 (review finding 8): shape (a) cannot see them, since its
   anchor is the `helpId:` property name, which the map's `batch:`/
   `jobs:`/`editor:` keys do not carry. (c) is deliberately redundant
   with (b) for the three view ids: a view root losing its
   `data-help-id`, or the map growing an id without a topic, both still
   fail. Every referenced id must have
   `help/<locale>/<id>.md` for **every** locale directory under `help/` -
   a missing file fails naming id, locale, and the referencing source
   line.
2. **File -> referenced (orphans)**: every `help/*/<id>.md` must map back
   to a referenced id; an orphan topic is a hard failure. (A stale topic
   for a renamed control is exactly the drift this direction catches.)
3. **Locale-set lockstep**: the set of locale directories under `help/`
   must equal the set under `locales/` - the owner's bilingual duty made
   structural, and the guarantee that a future locale lands catalogs and
   help together or fails visibly.
4. **External-URL ban**: a topic file containing `http://` or `https://`
   is a hard failure. Help is self-contained by design (D50's trust model,
   the offline posture, and CSP - the webview must not navigate out of
   the app); cross-topic references are prose ("see the Match topic"),
   not links, because sidebar navigation is hover/pin-driven (D52). This
   is a proposed safeguard and per the standing rule is not argued back
   out during design.

The empty-state note, recorded so the gate's first run is understood
(corrected at plan-authoring): with `help/` absent (today's tree), **two**
checks hard-fail, not one - check 1 on the first annotated control the
moment D53/D54 land, and check 3 unconditionally (an absent `help/` tree
has no locale-directory set to equal `locales/`' `{en, de}`). The gate
therefore cannot land green before `help/en/` and `help/de/` exist: the
gate, the annotations and the topic tree land together or CI is red -
which is the intended forcing order, now stated without implying the
gate could precede the tree.

**Rejected: a separate `check-help.mjs` + own CI step.** Steelman:
check-i18n's name and charter say catalogs; a second script is
single-purpose and independently runnable. Rejected: help topics are
per-locale localized content - the same family (spec 8.4 puts long-form
help in the same architecture sentence as the catalogs), the checks share
the scan machinery (literal regexes over `src/`, locale-dir enumeration),
and one gate means one place where "i18n-complete" is defined. The
script's header comment is updated to name its widened scope.

**Rejected: a build-time manifest (generate the annotated set into a JSON
the gate reads).** Steelman: no regex scanning; the set is data. Rejected:
the manifest is a second representation of the registries + templates that
must be regenerated and committed (another drift gate to build), replacing
two regexes that follow the house's established literal-scan pattern
(D45's `LABEL_KEY_RE` precedent).

**Interface changes:** check-i18n checks; no wire.

---

## D63: The CLI renders multilingually: both locales embedded, a two-bundle fallback chain, resolution --locale > system > en

**Decision** (owner re-ruling 2026-07-21, `cli-multilang-rendering`,
`product-boundaries.yaml:448`, superseding `cli-english-only` :433; this
ADR is the E1 re-fold). `crates/muxsmith-cli/src/i18n.rs` embeds **both**
locales at build time and renders through a **fallback chain of
per-locale `FluentBundle`s**, mirroring the frontend's `buildBundles`
mechanism (`src/i18n/index.ts`) on the Rust side:

- **Embed table** - the one place a CLI locale exists, four `include_str!`
  constants in a static table:
  `[("en", EN_CLI, EN_DIAGNOSTICS), ("de", DE_CLI, DE_DIAGNOSTICS)]`,
  the de pair being `locales/de/{cli,diagnostics}.ftl`. Adding a CLI
  locale is one row plus content; the code-row-per-locale asymmetry with
  the frontend's zero-code glob is accepted because `include_str!` is
  compile-time and has no glob form (trigger 11 records the duty).
- **Chain construction.** `Renderer::new` resolves the requested tag
  exactly as today (explicit `--locale` > `sys_locale::get_locale()` >
  `"en"`, `i18n.rs:21-27` - the boundary entry's binding order), collapses
  it to its primary language subtag (`LanguageIdentifier::language`, the
  Rust mirror of the frontend's `primarySubtag`: "de-AT" resolves the
  "de" row), and builds `bundles: Vec<FluentBundle>` as
  `[requested, "en"]` deduplicated - one bundle when the request is en or
  unknown, two when it is de. Each bundle gets its own locale's two
  resources and its own langid (so CLDR plural rules are per-locale
  correct - the reason this is a chain and not one merged bundle),
  and keeps `set_use_isolating(false)` (`i18n.rs:30-31`: grep-able
  output).
- **Per-message fallback**: `render` walks the chain and uses the first
  bundle that has the message id with a value; a message missing
  everywhere falls back to the raw id exactly as today
  (`i18n.rs:79-83`) - so the order is de -> en -> raw id, the boundary
  entry verbatim. `msg`, `msg_with_counts`, `diagnostic`,
  `diagnostic_no_file` and the numeric-param promotion are untouched
  above the lookup.
- **Gate coverage, stated so nothing new is invented**:
  `catalog_completeness.rs` stays en-scoped by design (en is the
  reference locale); de structural correctness is already gated by
  check-i18n id parity, D55 rule 4/5 attribute + placeable/selector
  parity, and the e2e all-locales real-parse guard - a de message with a
  wrong placeable name is a hard CI failure there, so no de twin of
  `catalog_completeness.rs` is added.
- **New renderer unit tests, enumerated** (in `i18n.rs`'s existing test
  module, beside the `zz-ZZ-invalid` case at `:212`): (1) a de request
  renders a de message; (2) a message present only in en renders the en
  value under a de chain (per-message fallback); (3) a region-qualified
  "de-DE" resolves the de row; (4) an unknown tag renders the en chain.
  No new snapshot files: the 11 insta snapshots stay en-pinned (D64) and
  de rendering is covered here plus by the parity gates.

**Rejected: single bundle, en resources overridden by de
(`add_resource_overriding`).** Steelman: one bundle, no chain walk, and
per-message fallback falls out for free (a message missing in de keeps
its en value). Rejected: a `FluentBundle` carries one locale set for
CLDR plural-rule selection, so en fallback messages would pluralize
under de rules - harmless for the de/en pair (identical
one/other categories) and silently wrong for any locale with a richer
category set (`ru`), which is exactly the class D55 rule 5's carve-out
already anticipates. The chain is the mechanism the frontend already
uses; matching it keeps one mental model (spec 8.4: "falls back to
English per message").

**Rejected: runtime catalog loading from disk.** Steelman: no embed
table, locales droppable beside the binary. Rejected: the CLI's catalogs
have been build-time-embedded since Plan 2 (`i18n.rs:7-8`), a
self-contained binary is the product shape, and
`core-07-runtime-fetching-rejected` records the house posture for
build-consumed data.

**Interface changes:** none on the wire; CLI rendering behavior (de
output for de users - the user-visible point of the ruling);
`Renderer::new` keeps its signature; amendment 3 rewrites the rustdoc.

---

## D64: Locale-pinning audit: every CLI-output-asserting test invokes through one en-pinned funnel

**Decision** (the re-ruling's companion constraint, binding via
`cli-multilang-rendering`'s statement; a proposed safeguard under
`proc-proposed-safeguard-stays`, `process-conventions.yaml:425` - not to
be argued back out before it is built and measured). Once de is embedded,
`Renderer::new`'s `sys_locale` fallback makes every unpinned CLI
invocation host-locale-dependent (green on en CI, red on a German dev
machine - the exact failure the entry names). Therefore:

- **Mechanism: a shared harness funnel.** `tests/support/mod.rs` gains
  `pub fn muxsmith(args: &[&str]) -> assert_cmd::Command` building
  `Command::cargo_bin("muxsmith")` with `args` plus a trailing
  `"--locale", "en"` (clap accepts the flag after positionals; `--locale`
  is a per-subcommand arg, `cli.rs:31,55,66,93`, so it must follow the
  subcommand - which appending guarantees). Every existing per-file
  `muxsmith()` helper (e.g. `cli_validate.rs:3-5`) and direct
  `cargo_bin` call is replaced by it.
- **Where it applies, enumerated - the complete CLI-invoking test
  surface** (measured: `cargo_bin` grep, 2026-07-21):
  `cli_validate.rs` (1 constructor, 3 snapshots), `dry_run_cli.rs` (13
  invocation sites, 3 snapshots), `run_cli.rs` (1 constructor, 4
  snapshots), `run_live.rs` (1 constructor, 1 snapshot),
  `cli_schema.rs` (2 sites: `schema_json()` routes through the funnel -
  schema output is locale-independent JSON Schema, pinned regardless -
  and `no_args_shows_usage_and_fails` uses the bare helper below). That
  covers all **11 insta snapshots** (`tests/snapshots/`, counted) and
  every non-snapshot stdout/stderr assertion in those files - including
  `--json` assertions, whose envelope carries the locale-rendered
  `rendered` field and is therefore locale-sensitive too.
- **One enumerated exception, closed** (controller ruling at
  plan-authoring, internal technical fork, recorded here):
  `no_args_shows_usage_and_fails` (`cli_schema.rs:26-27`) verifies that a
  **bare** `muxsmith` invocation - a real user scenario - prints usage
  and fails. Routing it through the funnel would run
  `muxsmith --locale en` instead, which fails as an unexpected top-level
  argument (`--locale` is per-subcommand): same assertion result,
  different verified behavior - a silent test-meaning change, which is
  never acceptable. It therefore uses `support::muxsmith_bare() ->
  assert_cmd::Command` (a no-args `cargo_bin` constructor, also in
  `tests/support/mod.rs`, so the grep invariant below holds verbatim).
  Locale pinning is moot for it by construction: clap rejects the
  invocation before any `Renderer` exists, so no locale-dependent text
  is rendered. The exception is **closed**: `muxsmith_bare` has exactly
  this one caller, and a second caller reopens D64 rather than riding
  the helper.
- **Post-sweep invariant, greppable**: `cargo_bin("muxsmith")` appears in
  exactly one file, `tests/support/mod.rs`. A future test that bypasses
  the funnel is a review defect findable by that grep; a future test
  using the funnel is pinned by construction.
- **The e2e (Playwright) suite invokes no CLI binary** - verified
  (grep over `e2e/*.ts` for muxsmith/cargo_bin invocations: zero hits;
  the suite drives `dist/` with mocked Tauri IPC). The constraint binds
  it prospectively through the boundary entry should that ever change.
- **What is deliberately NOT pinned**: the four renderer unit tests in
  `i18n.rs` construct `Renderer::new(Some(...))` in-process with
  explicit tags (`:206-275` today) - they are the locale-behavior tests
  themselves and never consult `sys_locale`.

**Rejected: pinning via environment (`LC_ALL`/`LANG` on the child
`Command`).** Steelman: one env line in the funnel, no argument-order
concern, and it also pins any future locale-sensitive libc behavior.
Rejected on portability: `sys_locale` reads OS APIs, not environment
variables, on Windows and macOS - the pin would hold on exactly the CI
legs that never break and miss the developer machines the audit exists
for. `--locale en` uses the CLI's own contractual surface on every
platform.

**Rejected: making `--locale` a clap global argument so the funnel can
prepend it.** Steelman: one flag definition instead of four
per-subcommand copies, and `muxsmith --locale de validate` reads
naturally. Rejected for this plan: it widens the accepted CLI syntax (a
user-visible interface change) for zero pinning benefit - appending
after the subcommand works today with the existing four flags. Recorded
as an idea, not a defect.

**Interface changes:** test-support surface only; no product code.

---

## 2. Fluent catalog changes (en + de, both, in the same change - i18n-16)

Exact id-count deltas per file. "attrs" are attribute definitions added
under retained ids; attribute values are new bilingual content.

| catalog | today | folds out | renames | adds | result | attrs added |
|---|---|---|---|---|---|---|
| `gui-common.ftl` | 41 | 5 (settings-open-tooltip, browse-button-tooltip, firstrun-use-path-tooltip, firstrun-retry-tooltip, firstrun-picker-hint) | - | 2 (help-toggle-label, help-sidebar-label) | **38** | 7 (D55 table + `browse-button.tooltip-directory` + `help-toggle-label.tooltip`) |
| `gui-settings.ftl` | 13 | 5 (settings-save-tooltip, settings-cancel-tooltip, settings-mkvmerge-path-hint, settings-default-jobs-hint, settings-locale-hint) | - | - | **8** | 5 |
| `gui-batch.ftl` | 39 | 12 (batch-profile-pick-tooltip, batch-dry-run-tooltip, batch-suggestion-copy-tooltip, batch-suggestion-apply-tooltip, batch-run-tooltip + 4 variants, batch-browse-dir-tooltip, batch-source-hint, batch-output-hint) | 1 (batch-recents-select-tooltip -> batch-recents-select) | 1 (batch-resolved-track) | **28** | 12 |
| `gui-jobs.ftl` | 46 | 5 (jobs-cancel-batch-tooltip, jobs-row-cancel-tooltip, jobs-history-refresh-tooltip, jobs-history-copy-tooltip, jobs-history-save-tooltip) | - | - | **41** | 5 |
| `gui-editor.ftl` | 45 | - | - | 1 (editor-track-rule-order) | **46** | 42 (`.tooltip` on every registry label; the 4 attribute-less ids: editor-save-note, editor-action-add, editor-action-remove, editor-track-rule-order) |
| `diagnostics.ftl` | 50 | - | - | - | **50** | - |
| `cli.ftl` | 26 | - | - | - | **26** | - |

Recount of the fold total: 5+5+12+5 = 27 removed ids, plus the 1 rename,
= 28 = the 22 tooltip-family ids + the 6 hint ids (the tooltip family
loses 21 by removal and 1 by rename; `batch-browse-dir-tooltip` is
removed from gui-batch and resurfaces as `browse-button.tooltip-directory`
in gui-common, counted once, on the removal side). ✓

Value changes to retained content: `settings-locale-label.hint` rewritten
(D56), `apply-rule-index-out-of-range` gains the `$rules` plural selector
(D61). The four `close-abort-*` messages are untouched (D55 hard
constraint). Help topics: 44 new markdown files (D54). All final
user-facing wording (42 tooltip attrs, 22x2 topics, the rewritten hint,
the new keys) routes through the owner's plan-close rendered-surface pass;
the id/attribute/file **structure** above is closed here.

---

## 3. mkvtoolnix parity audit (SI-3)

Method per `testing-si3-run-binary` and `proc-06-mkvtoolnix-parity`:
source read at `~/Downloads/mkvtoolnix`, binary run 2026-07-21
(`mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit` - matching the
source tree's version). No muxing-semantics surface exists in this plan,
so the binary contributed only the version check; every citation below is
source. Licensing: behavior and facts only; **no literal text taken, no
wording modeled** (tooltip/topic prose is authored fresh under the
rendered-surface pass).

| Plan-7 surface | mkvtoolnix-gui reality (cited) | classification |
|---|---|---|
| Universal tooltips on non-obvious controls | ~62 `setToolTip` calls under `merge/` alone via the `Util::setToolTip` helper (`util/widget.cpp:95-113`), gettext-localized (`QY()`, `common/qt.h:25`) | **MATCH in intent** (spec 8.3 baseline = their practice); mechanism differs (Fluent attributes vs gettext) - internal, no parity surface |
| Native `title` tooltips (platform-rendered) | helper HTML-wraps text in `<span>` to force Qt rich-text word-wrap (`widget.cpp:108-112`) | **Justified divergence**: webview-native `title` tooltips wrap per platform; the Qt workaround has no analogue here |
| Global tooltip-disable setting | `uiDisableToolTips` (`util/settings_names.h:152`; checkbox `forms/main_window/preferences_dialog.ui:200`; honored in `widget.cpp:103-106`) | **Genuine gap, deliberately not adopted in v1**: Muxsmith's tooltip volume is a fraction of theirs and nothing in the anchor inputs asks for it; recorded as a trigger (section 8) rather than silently missing |
| In-app help mode + sidebar (spec 8.3) | **absent entirely** - no WhatsThis/F1/context-help/sidebar anywhere (verified negative with positive control, ground truth) | **Deliberate divergence, spec-mandated**: spec 8.3's "usable without reading external documentation" is a product premise mkvtoolnix does not share; the brief's presumption is confirmed, with the nuance below |
| Long-form documentation reachability | Help menu opens external URLs (`main_window.cpp:230-244,328-337,654-660`; URLs `common/common_urls.h:3-12`); the mkvmerge manual is a **bundled offline per-locale HTML copy with online fallback** (`main_window.cpp:662-696`) | **MATCH in offline-first posture**: D51 embeds topics in the binary; mkvtoolnix bundles its manual. The "links out to online docs" framing in the brief is only half their story - recorded so the divergence claim stays honest |
| Per-control long-form explanations | none - tooltips are the ceiling (some are multi-sentence, e.g. `merge/output.cpp:201-203` telling the user to read the documentation) | **Divergence in kind**: where their deep content overflows a tooltip they point at the manual; Muxsmith's overflow goes to the help topic (D54's inclusion criterion is exactly this overflow) |
| First-run / onboarding guidance | none (searched: no wizard/welcome/tour; ground truth) | Muxsmith's FirstRun predates this plan (D28); no parity surface here |
| CLI output localization (D63, E1 re-fold) | the CLI tools are gettext-localized from the same catalogs as the GUI: one `"mkvtoolnix"` textdomain (`common/translation.cpp:435-437`), 28 `po/*.po` locales incl. `de.po` serve mkvmerge and the GUI alike | **MATCH after D63**: one catalog source per locale rendered on both surfaces (spec 8.4's own architecture); cited in the re-ruling's steelman and verified at source here |

---

## 4. Recon/gap table from the brief, closed

| Brief claim | Verified? | Closure |
|---|---|---|
| No markdown renderer dep; none transitive | **Yes** (package.json + full pnpm-lock grep with positive control) | D50 adds marked 18.0.7 |
| No `help/` dir anywhere | **Yes** (`ls`: absent at root, `src/`, `locales/en/`) | D51 creates it |
| No config_path->field mapping in `src/` | **Yes** - `config_path` appears in `src/` only as an opaque forwarded token (`SuggestionCard.vue`, `BatchView.vue`, `ipc.ts:69,133`) | D57 builds the anchor map; nothing existed to reuse |
| "Rust-side ApplyError parsing in `src-tauri/src/error.rs:164-166`" | **Yes with a precision**: `:161-178` is the ApplyError->IpcError *mapping*; the parsing itself is core's `rule_index_of` - and D43 records it as the narrow Suggestion-path token, unusable for Diagnostic paths | D57 rejected-alternative #1 |
| Curated domains not exported; `settables.ts` types them plain `"string"` | **Yes** (`settables.ts` MATCHABLE_TYPES: `"type": "string"`, `"codec_kind": "string"`) | D58 exports the value domains |
| Rule grid 4 columns, order = row position, drag rebuild | **Yes** (`EditorView.vue:463-481`, `:358-372`) | D59 adds the ordinal column |
| 42 editable + 1 fixed; 13 registries; 10 widget kinds; zero editor tooltips | **Yes** (all verified; correction #0-postscript) | D53/D54 |
| bootstrap-once `main.ts:24-30`; locale via settings | **Yes** | D56 |
| check-i18n three checks at the cited behaviors | **Yes** (line drift noted, correction #7) | D55/D61/D62 extend it |
| 19 IpcError codes, 14/5 catalog split, presence gate codifies a holding invariant | **Yes** (correction #6) | D61 |

---

## 5. Deliberately out of scope

- **CLI live locale switching** - the CLI resolves its locale once per
  invocation (D63); there is no long-running CLI process to re-render.
  CLI German *rendering* itself is IN scope since the E1 re-ruling
  (D63/D64, `cli-multilang-rendering`) and is no longer listed here.
- **Shell native-dialog i18n** (`close-abort-*` en-only, `run.rs:543`) -
  recorded residual since Task 21 (the de catalog header documents it);
  untouched, and D55 explicitly protects its line-parser constraint.
- **The i18n-17 `allowed`-param residual** - settled pre-1.0 item, not
  reopened.
- **Fixing PropertyMapWidget's attachment-context type-table lookup**
  (ground-truth flaw) - surfaced with a trigger (section 8); D58 gates
  around it, fixing it belongs with the capability/registry work, not the
  i18n cluster.
- **Post-apply auto-refresh of the batch report** - existing deferred
  ROADMAP candidate (`BatchView.vue:200-206` comment); the
  `batch-suggestion-card` help topic describes the shipped behavior.
- **A tooltip-disable setting** (mkvtoolnix parity gap) - trigger only.
- **Help search, keyboard shortcut (F1) for help mode, topic
  cross-linking** - none of the anchor inputs; D62's URL ban makes
  linklessness structural for v1.
- **Markdown in tooltips** - tooltips are plain `title` text; only the
  sidebar renders markdown (D50's single-site license).
- **A Rust enum for IpcError codes** - D61 rejected-alternative; the gate
  delivers the guarantee without the refactor.

---

## 6. Spec amendments proposed

Per `proc-04-spec-wins`, with the self-contradiction sweep at the end.
Amendments 1-3 and 5 fold in the E1 **re-ruling**
(`cli-multilang-rendering`) and amendment 6 the finding-3 modifications
plus the E3 ruling; no variant wording remains.

1. **Spec 8.4, last bullet** ("v1 ships English content only (non-goal
   11); the mechanism ships complete"): replace with "v1 ships English
   and German content on both surfaces - GUI catalogs and help topics,
   and the CLI's embedded catalogs (`cli-multilang-rendering`, D63);
   further locales are content work (non-goal 11)."
2. **Non-goal 11** (spec §11, "UI localization content: only English
   catalogs and help topics ship in v1..."): replace with "Locales
   beyond English and German. The mechanism (8.4) ships complete on both
   surfaces; adding a locale is content work (catalogs + help topics
   land together, enforced by CI) plus one row in the CLI embed table
   (D63), not a refactor."
3. **Renderer rustdoc** (`crates/muxsmith-cli/src/i18n.rs:11-13` and
   `:19-20`): the stale "v1 ships English content only" claim is
   replaced by the D63 reality: the renderer embeds the en and de
   catalogs and renders through a per-message fallback chain
   (requested locale, then en, then the raw id), locale resolution
   `--locale` > system locale > en (`cli-multilang-rendering`).
4. **Spec 10, i18n sentence**: "eslint (no-literal-string rule) keeps
   hardcoded strings out of the frontend" becomes "the
   `@intlify/vue-i18n/no-raw-text` eslint rule (D27) keeps hardcoded
   strings out of Vue templates - template text nodes plus the configured
   static attributes (`title`, `aria-label`, `placeholder`, `alt`);
   `:`-bound expressions are covered by the check-i18n literal scan
   instead" (matches `eslint.config.js:53-68` reality).
5. **Spec 8.4, locale selection bullet** ("Locale selection: system
   locale with manual override in app settings and `--locale` on the
   CLI; falls back to English per message"): one change - the GUI
   override "takes effect live, without restart" (gui-26 closure, D56).
   The CLI half of the sentence **becomes true as written** under D63
   (the promise was dangling only while the renderer embedded en alone)
   and needs no amendment - recorded here so the sweep shows the claim
   was checked, not skipped.
6. **Spec 8.3, help-mode bullets** - two parts (fix round 1, review
   finding 3): **(a) additions** the spec left unstated and this design
   closed: hovering an element without a help-id sets no hover topic -
   the sidebar shows the pinned topic if one is pinned, else the current
   view's topic; clicking an annotated element pins without activating
   it.
   **(b) modifications of stated mechanics** - the spec is authoritative,
   so each D52 deviation is amended, not silently narrowed: the pin
   release enumeration ("until another element is clicked or help mode
   exits") gains a third condition, "or the active view is switched"; and
   the Esc rule ("clicking again (or Esc) exits") gains the qualifier
   "except while the settings dialog is open, whose native cancel
   consumes Esc". **(c) the ruled activation semantic** (owner
   2026-07-21, E3): "while help mode is active, control activation
   inside the main content area is suppressed; the help toggle, the
   three view tabs, the settings button and the sidebar stay live;
   clicking an annotated element pins its topic instead of activating
   it."

**Self-contradiction sweep, run against the amendments:** spec 8.3's
"help content is one markdown file per help-id per locale
(`help/<locale>/<help-id>.md`)" agrees with D51 unchanged. Spec 10's
"help-ids without a help topic file" sentence agrees with D62 (which adds
the reverse direction; the spec sentence states the minimum, not a
ceiling - no conflict). Spec 8.2's grid column list already names "order"
(D59 closes a gap, no amendment). Spec 8.4's catalog-source-of-truth
bullet stays true under D55 (attributes are catalog content). Non-goal
list's other items untouched. **Re-run after the E1 re-fold**:
amendments 1 and 2 both carry the multilang claim
(`cli-multilang-rendering`) and must land together with D63's code -
amending the spec before the embed lands would make 8.4 assert
rendering the CLI cannot do yet, so the amendments ride the same plan
that builds D63, per the existing one-change bilingual discipline;
amendment 3 (rustdoc) rides the D63 code change itself; amendment 5 now
touches only the GUI half of its bullet, and its CLI half is a recorded
no-change; amendment 6's parts (b)/(c) modify the same 8.3 bullet list
that part (a) extends, one edit; no amendment contradicts another.

---

## 7. Escalations: ruled by the governing human, 2026-07-21

Three product-scope / user-visible forks were routed per
`proc-latitude-clause-boundary` with a recommendation and the other
ruling's delta each; **all three are ruled** (plus the review's finding-5
fork, ruled in the same batch). The analysis is kept as the record of
what was decided between; the losing variants' consequences are moot.

**E1 - CLI German rendering. Two same-day rulings; the second stands.**
The fork: the de catalogs for cli/diagnostics exist and are
parity-gated, but the CLI embeds en only
(`muxsmith-cli/src/i18n.rs:7-8,32-37`), while spec 8.4 promised
`--locale` selection. The design recommended (a) fold de embedding into
Plan 7 (mechanism locale-generic, content ships, i18n-cluster plan);
alternative (b) defer to v1.x.

- **Ruling 1 (2026-07-21): English-only as standing product shape** -
  stronger than (b): multilingual rendering declared a GUI feature by
  decree (`cli-english-only`, `product-boundaries.yaml:433`). Briefly
  folded into amendments 1-3/5 and the out-of-scope list.
- **Ruling 2 (same day, after the controller's steelman review with a
  measured effort delta at `i18n.rs`): REVERSED - the CLI renders
  multilingually** (`cli-multilang-rendering`,
  `product-boundaries.yaml:448`, which marks `cli-english-only`
  superseded). This is the design's original (a) recommendation, now
  binding, with two additions from the re-ruling analysis: the locale
  resolution order and per-message fallback are fixed in the boundary
  entry's own statement, and the **locale-pinning audit** is a companion
  constraint (every CLI-output-asserting test pins its locale).

Folded in as **D63** (embed table, two-bundle fallback chain,
resolution order) and **D64** (the pinning audit as a proposed
safeguard); amendments 1-3 rewritten to the multilang form, amendment
5's CLI half withdrawn as no-longer-needed (the spec sentence becomes
true), section 1/5/D56/section 9 swept. Both analyses are kept above and
in D63/D64's rejected-alternatives; the English-only steelman survives
in `cli-multilang-rendering`'s own steelman field.

**E2 - the view-topic set. RULED: the three spec-8.2 views only**
(editor, batch, jobs); FirstRun and the settings dialog are not
help-annotated hosts. The fork: owner decision B says "every VIEW"; spec
8.2 defines three views; FirstRun and settings are outside that
enumeration and technically hostile to the sidebar (FirstRun replaces
the shell hosting the toggle, `App.vue:63-67`; the settings `<dialog>`
is a native modal whose top layer inerts the page behind it). The ruling
matches the recommendation; D54's tables and counts stand as designed
(22 ids, 44 files). The other variant's delta (+2 ids, +4 files, two
further design rounds) is moot.

**E3 - help-mode activation blocking. RULED: global suppression with
the enumerated allowlist** (help toggle, three nav tabs, settings
button, sidebar), as designed in D52: help mode is a safe inspection
overlay in which no click can start a run, apply a suggestion, or save a
file, while view switching stays possible for browsing per-view help.
The rejected alternative (annotated-only interception) is recorded in
D52 with the ruling; spec amendment 6(c) carries the ruled semantic into
spec 8.3. Rationale as recommended: the asymmetry "annotated controls
are inert, unannotated ones fire" is unlearnable precisely because the
annotated set is curated, and the worst-case misfire is destructive.

**Finding-5 fork (presentation tokens). RULED: carve-out ratified** as
`latitude-carveout-presentation-tokens`
(`process-conventions.yaml:456`): visual presentation tokens within the
existing design language are implementer-owned; semantic-carrying
mappings stay enumeration-bound. D52's highlight clause now cites the
convention, and its semantic mappings (hover-vs-pin distinction,
severity classes in D57, outline-no-layout-shift) are enumerated.
Related owner bar, recorded for context: v1's visual standard is a
reasonably good usable layout; deliberate visual refinement ("schick
machen") is a 1.x ROADMAP item.

---

## 8. Triggers created (for the controller to mirror into the ROADMAP)

1. **CONSUMED 2026-07-21** (twice): the E1/E2/E3 and finding-5 rulings
   landed in the fix round and were folded in; the same-day E1
   **re-ruling** (`cli-multilang-rendering`) landed after and re-folded
   as D63/D64 (sections 6 and 7, D52, D54, header). Kept numbered so
   later cross-references to triggers 2-11 stay stable.
2. **A third locale directory is added** -> D62's lockstep gate and
   D55's parity rules fire by construction; the one manual check: if the
   locale's CLDR plural categories exceed {one, other}, verify D55 rule
   5's category carve-out passes its catalogs before blaming them.
3. **A labelKey is renamed** -> D51's coupling renames the help-id and
   both topic files; D62 is the tracker (red until they agree). Recorded
   so the mechanism is understood as the tracker; no entry needed.
4. **fluent-vue or @fluent/bundle is bumped** -> re-verify the `bundles`
   setter contract and `$ta` global (D56/D55 rest on the installed
   3.8.2's verified behavior); the Renovate rider covers the bump
   arriving.
5. **marked major release via Renovate** -> re-check the "0 dependencies,
   no sanitizer needed for first-party input" premises of D50 before
   merging.
6. **A second `v-html` site is proposed anywhere** -> D50's single-site
   license is the boundary; the proposal reopens D50, not a per-case
   judgment call.
7. **The attachment-context propertyMap type lookup** (ground-truth
   flaw: track tables used for attachment select/drop maps) -> candidate
   for the capability/registry work (Plan 9 neighborhood); D58's path
   gate must be revisited when it is fixed properly.
8. **A user asks for quieter UI / tooltip suppression** -> the
   mkvtoolnix `uiDisableToolTips` parity gap (section 3) becomes a v1.x
   candidate with precedent cited.
9. **The stale ROADMAP sentence** in the v1.x "Gate part that
   Fluent-parses ALL catalogs" entry ("today no gate Fluent-parses the de
   CLI/diagnostics catalogs", refuted by correction #5) -> controller
   corrects the entry in place when consuming this design (the entry's
   remaining live content - placeable/selector parity - is D55 rule 5).
10. **`editor-generic-action-keys` budget revision** (D59): the
   controller updates the entry's statement
   (`product-boundaries.yaml:404`) from the 45-id budget to 46, noting
   the tooltip pass landed as attributes (zero ids) and the 46th id is
   the ordinal column's header key, when consuming this design -
   mirroring the recorded 43 -> 45 owner-ruling precedent.
11. **A locale beyond en/de ships** -> beside trigger 2's GUI duties, the
   CLI needs its embed-table row (D63's one-row-per-locale asymmetry
   with the frontend's zero-code glob), and D64's pinned suite stays
   green by construction (`--locale en` is locale-set-independent).
   Created by the E1 re-fold.

---

## 9. What the implementer must not decide

Every fork below is closed above; a fork discovered on code contact
returns as NEEDS_CONTEXT with a decision memo, it is not resolved at the
keyboard (`proc-latitude-clause-boundary`).

- The renderer is marked 18.0.7, exact-pinned, defaults, no sanitizer;
  `v-html` exists in exactly one component (D50).
- `help/` lives at the repo root; topics load via eager `?raw` glob; the
  missing-topic fallback is the raw help-id as text, never blank (D51).
- Registry help-ids equal their labelKey, written out literally, never
  derived (D51/D53).
- Annotation is `data-help-id`; no directive, no props threading;
  dispatcher fallthrough for registry controls (D52).
- The pin is cleared on view switch; Esc yields to an open settings
  dialog; hover and focusin are equivalent (D52).
- `EditableField` gains optional `helpId` only; no `tooltipKey`, no new
  `FieldSpec` variant (D53).
- The annotated set is D54's tables, verbatim - 22 ids, 44 files; adding
  or dropping a member is an owner change, not an implementation nicety.
- Tooltip/hint attribute names are `tooltip`, `hint`,
  `tooltip-<state>` - no others (D55).
- The `close-abort-*` block and `batch-recents-select`'s value-less shape
  are exactly as specified (D55).
- No catalog id is renamed except `batch-recents-select-tooltip` ->
  `batch-recents-select` (D55).
- check-i18n stays line-based; the five D55 rules, the D61 gate (with
  the `#[cfg(test)]` cutoff and the check-2 residual-comment update), and
  the four D62 checks are the exhaustive extension set.
- The plural-category carve-out list is exactly
  {zero, one, two, few, many, other} + numeric literals (D55 rule 5).
- The locale switch is `applyLocale` in `src/i18n/fluent.ts`: fresh
  array assignment, `currentLocale` ref, `document.documentElement.lang`;
  no reload, no remount (D56).
- Marker anchoring is exact string equality against section 1's
  re-derived, probe-verified emission table - never normalized, never
  parsed, and no member added or dropped at the keyboard (D57).
- The CLI embed set is exactly D63's two-row table; the chain is
  per-locale bundles (never one merged bundle); resolution order and
  per-message fallback are the boundary entry's, verbatim (D63).
- Every CLI test invocation goes through D64's en-pinned support funnel,
  except the one enumerated `muxsmith_bare()` caller
  (`no_args_shows_usage_and_fails`; a second caller reopens D64);
  `cargo_bin("muxsmith")` ends up in exactly one file
  (`tests/support/mod.rs`), and the pinning is `--locale en`, never
  environment variables (D64).
- The four D63 renderer unit tests are the enumerated set; no de
  snapshot files are added (D63/D64).
- Unanchored paths are panel-only; the panel is never filtered (D57).
- Marker severity is worst-of; the three severities are the whole set
  (D57).
- Dropdown conditions are D58's four, byte-exact key equality, text
  fallback for non-empty out-of-domain values, `raw:` untouched.
- `TYPE_VALUES`/`CODEC_KIND_NAMES` are emitted into `settables.ts` by the
  existing emitter, never hand-written in TS (D58).
- The ordinal column is `index + 1`, no model field, no drag-handle
  change (D59).
- The `"-"` placeholder stays a code literal; only the parenthesized
  composition moves to `batch-resolved-track` (D60).
- `ParamValue` is the untagged two-variant enum; the promotion sites are
  the four listed lines and no others; `Diagnostic` params stay strings
  (D61).
- `apply-rule-index-out-of-range` is the only message that gains a
  selector (D61).
- All wording (tooltips, topics, the rewritten hint, new keys) goes
  through the owner's plan-close rendered-surface pass; the implementer
  authors draft content but final strings are the owner's gate.
