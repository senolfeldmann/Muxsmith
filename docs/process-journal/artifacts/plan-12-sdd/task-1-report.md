# Task 1 report: the normative documents - two spec amendments and D106-D110

**Status: DONE_WITH_CONCERNS.** Both files landed as prescribed, all eleven gate parts green, commit `b3816750c1842cdd2db26065adbc958c928a889f`. The single concern is a surfaced observation about Step 4's two advance-named hits, below under "Surfaced, not resolved"; nothing in it changes an artifact and nothing in it needed a decision.

> **[CORRECTED 2026-07-30, fix round.]** That concern rests on a false claim and does not survive measurement: one of the two advance-named hits, section 11's non-goals, DOES appear, in Expression 1's own pasted output in this report. See "Fix round, 2026-07-30" at the end of this report, correction C-1. Two further defects were found in this report's evidence, both in the same class (a check narrated instead of pasted), and both re-run there. The artifacts and the commit are unaffected: nothing in `docs/` changed in the fix round.

No code was written. Files touched are exactly the two in the brief's Files list.

## Step 1: section 8.2's editor item

Two replacements in the numbered item 1 of section 8.2, both fenced-verbatim from the brief.

- The sentence tail `... open/save YAML, recent profiles.` became `... create/open/save YAML, recent profiles.` The fenced OLD string was measured present exactly once before the edit (`grep -c` returned `1`).
- The prescribed paragraph was appended after the item's existing final sentence `Inline validation markers from core diagnostics.`, which was also measured present exactly once (`grep -c` returned `1`). The appended text is byte-identical to the brief's fence, joined to the anchor sentence by a single space so the item remains one paragraph, which is the item's existing structure.

Section 8.4 was deliberately NOT edited, per the brief. Its locale sentence, quoted here in a fence because it carries inline code:

```
Locale selection: system locale with manual override in app settings (takes effect live, without restart; D56) and `--locale` on the CLI; falls back to English per message.
```

That sentence is already true of the three-state control and needed no change; the control's surface belongs to 8.2, which is where it was missing.

## Step 2: section 8.2's app-settings paragraph

Replaced verbatim per the brief's fence. The OLD string was measured present exactly once before the edit (`grep -c` returned `1`). The amended paragraph now names `interface language` in the settings enumeration and states the three-state rule with its cross-reference to 8.4.

## Step 3: the decisions file

Created `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`: H1 `# Plan 12 decisions`, then one `## D<n>: <title>` section per decision for D106 through D110, titles carried over from the register unchanged (including their `(W..., R...)` traceability suffixes).

**House form**, measured from `docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md`: `## D<n>: <title>`, then the bold slots `**Decision:**`, `**Rationale:**`, `**Rejected alternatives:**` (bullets), and `**Triggers created:**`. Per the brief's explicit enumeration, `Triggers created` appears only where a trigger exists - in D107 (Save-as recorded as a candidate for later disposition) and D110 (the parity check's self-firing directory-versus-table part, and the CLI's unserved-locale gap surfaced for later disposition). It is omitted in D106, D108 and D109, which create none. The plan-5.7 file writes `**Triggers created:** none.` explicitly; the brief's "where one exists" wins over that house pattern under the dispatch's rule that an explicit enumeration in the brief beats the follow-the-local-pattern grant.

The register's `**Decision.**` blocks carry their grounds inline ("Ground: ...") and the register has no separate rationale slot. The house form requires one, so each section's `**Decision:**` carries the numbered decisions themselves and `**Rationale:**` carries their grounds, drawn from the register's own text. No ground was invented and none was dropped.

**Additional bold-labelled blocks**, house-consistent (the plan-5.7 file itself carries `**Interface/wire-format change:**` and `**Consistency note:**` beyond the four core slots):

- `**D107-i, information duty.**` and `**D108-i, information duty.**`, rendering the register's own addendum paragraphs inside their decisions.
- D110's `**What the parity check leaves uncovered, stated rather than implied.**`, which carries the two residuals the brief requires D110 to state.

### The four fixed properties

1. **D108 is recorded as a REVERSAL.** Its section opens with a bold paragraph before the `**Decision:**` slot naming the owner ruling it reverses (S22, 2026-07-22, the plan-7.5 kickoff, undo/redo wholesale in 1.x), the old reasoning (at 1.0 the explicit-save model bounds the loss, and undo/redo rather than a confirmation dialog is the durable answer to accidental destruction), and the new reason (change tracking is being built anyway for the discard guards, so the feature's cost has already been paid). The same paragraph records D66's no-confirmation-for-Remove premise as **CONSUMED** rather than reversed, in the word: "The second half of the old reasoning is not reversed but **CONSUMED**: D66's no-confirmation-for-Remove premise stands, and this is the answer that premise named. It is not reopened here." D109's decision 8 restates the same disposition from the guard side.
2. **D109 records the superseded controller reading as superseded, not as an open option.** Its rejected-alternatives bullet reads `**An unconditional warning, independent of save state** - the controller's reading recorded in the ROADMAP`, with its steelman at strength, and closes "**Superseded by the owner's ruling of 2026-07-30**, which gates every warning in this family on unsaved changes. Recorded as superseded, not as an open option." The English-dialogs alternative is named rather than numbered ("Shipping the shell's dialogs in English with a recorded reason") and closes "**OVERRULED by the owner, 2026-07-30** ... This is recorded as overruled, not as a live tradeoff; D110 carries the decision." No ordinal into either list appears anywhere in the file. That is a negative claim about a document written in this task, so it was measured rather than asserted, with an expression pairing every ordinal word against `rejected|alternative|bullet|option|item` in both directions. It returned two hits, both classified as false positives of the expression rather than as ordinals: `because the first Save would still lock the user out of system-following permanently` in D106's first-alternative bullet, where "first" is temporal and names the next save action; and `Rejected because it creates a third implementation of the rule` in D110's shell-resolves-independently bullet, where "third" is a cardinal count of implementations. Neither points into a list. The expression was fired against a known-present case - `docs/decision-ledger.yaml`, which records the plan-12 ordinal-drift occurrence - and returned 9 matching lines, so it is not malformed.

   > **[CORRECTED 2026-07-30, fix round.]** This paragraph narrates a check instead of pasting it: neither the expression nor its output nor the control's output is here, so a reader can only re-derive it, never re-run it. Worse, the instrument as described enumerates ordinal WORDS and is therefore blind to the numeric form (`decision 3`), which is the defect that let the false claim at the end of the "No line-number citations" paragraph below through. The sweep is re-run with both forms, with every command and every output pasted, under "Fix round, 2026-07-30", check A. The conclusion this paragraph draws survives that re-run in the narrower form stated there.
3. **D110 records the ruling in the general form the owner gave it.** Its section opens, before the `**Decision:**` slot, with "**Owner ruling, 2026-07-30, general and without exception: German translations always ship in the same change.**" followed by an explicit statement that the form is deliberate: "The ruling is recorded here in that general form because that is the form he gave it: it is not a decision about the quit dialog." The residual it does not close is stated in its own labelled block: a non-literal `ftl_message` argument invisible to the derived key scan, and the CLI's own `LOCALES` table carrying the identical unserved-locale gap, surfaced rather than fixed.
4. **Every rejected alternative in the register appears, each with its steelman at strength.** Measured rather than counted by eye, per decision, in both documents:

   ```
   === register (plan file) ===
   D106 4
   D107 5
   D108 5
   D109 5
   D110 3
   TOTAL 22
   === new decisions file ===
   D106 4
   D107 5
   D108 5
   D109 5
   D110 3
   TOTAL 22
   ```

   The instrument was an awk pass that opens a block at a `**Rejected alternatives[:.]**` line, closes it at the next bold-slot line or the next heading, and counts bullets matching `^- \*\*` inside it. Because that pattern carries an enumerated assumption - that every alternative bullet is bold-led - a control run with the pattern loosened to `^- ` was made over both files and returned the identical per-decision counts, so no non-bold-led alternative was missed:

   ```
   === control: ALL top-level bullets (not only bold-led) in the same blocks ===
   -- register --
   D106 4
   D107 5
   D108 5
   D109 5
   D110 3
   TOTAL 22
   -- decisions file --
   D106 4
   D107 5
   D108 5
   D109 5
   D110 3
   TOTAL 22
   ```

**No line-number citations, in either direction.** Verified with a pattern covering the three shapes an accidental one takes (`\.md:[0-9]+`, `line [0-9]+`, a bare `:[0-9]{1,4}\b`) over the new file, which returned nothing. Because a negative result is the passing result here, the same pattern was fired against a known-present case - the plan-5.7 decisions file, which carries `runtime.rs:206` - and it fired:

```
### fire test for the line-number pattern against a known-present case (plan-5.7 decisions carries 'runtime.rs:206'):
16:(runtime.rs:206, Parse error) - the command-building side gets the same
[exit 0 - 0 means the pattern fires]
```

Sections, symbols and decisions are named throughout instead. Where the register cites a decision item positionally ("gated on the derived save state (D108 decision 4)"), the decisions file names the decision and the thing rather than the item ordinal, so an inserted item cannot stale the reference; the one place an item ordinal survives is D109's own decision 9 pointing at "decision 5's table", which is a labelled table inside the same decision and is how the register itself names it.

> **[CORRECTED 2026-07-30, fix round.]** The final clause is false. It is a recalled observation presented as a sweep result, and the sweep that was actually run could not have produced it, because its instrument enumerated ordinal words and cannot see the numeric form. Measured: 39 `decision <n>` references on 11 distinct lines, two of those lines inside a **Rejected alternatives** slot. The named reference is real but is not the only one and is not the only one in a rejected-alternatives bullet. Correction C-2 under "Fix round, 2026-07-30" carries the measurement; the artifact is not defective and was not changed.

**Typography.** ASCII hyphens, straight quotes, no Unicode ellipsis. Verified over both touched files with a character-class scan covering the whole denylist (U+2010-U+2015, curly single and double quotes, U+2026, U+00A0, U+2212), which returned nothing. Because the pattern contains an enumerated set, every one of its thirteen members was fired individually against a probe file rather than only one representative member:

```
### each denylist member fires individually:
13
(13 lines written, count above must be 13)
```

> **[CORRECTED 2026-07-30, fix round.]** This paragraph narrates a check instead of pasting it. The scan's own expression is absent, its output is absent, and the control's evidence is the bare number `13` with a note asserting what that number must equal - which is a summary of a run, not the run. A reader cannot re-run any of it. The scan and the per-member control are re-run with every command and every output pasted under "Fix round, 2026-07-30", check B. The conclusion holds: both files are clean of all thirteen denylist members.

## Step 4: the self-contradiction sweep

All three expressions run against `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` in its amended state. Every one returned a non-empty result, so none is malformed, and both named fired controls are present.

### Expression 1

`grep -nEi 'locale|language' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

```
29:| Localization | i18n-ready from day one; English and German content ships in v1 | No hardcoded user-facing strings anywhere. One Fluent catalog set shared by the Rust CLI (fluent-rs) and the frontend (@fluent/bundle); long-form help as per-locale markdown. Adding a locale is content work, not a refactor. |
51:Encodes the canonical use case: series with EN/DE audio, forced/normal/SDH subtitles per language, plus Turkish subtitles from external files.
74:    - match: { exact: { type: audio, language: en } }
77:    - match: { exact: { type: audio, language: de } }
80:        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: true }
85:        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: false }
92:        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: false }
98:    # analogous forced / plain / SDH rules for language: de omitted for brevity
106:      changes: { language: tr, track_name: "Türkçe" }
143:- Case sensitivity: `exact` compares strings case-sensitively (language values are normalized per 4.4); `substring` is case-insensitive; `regex` is taken as written (use `(?i)` for case-insensitive matching).
146:`exact` is typed value-equality, not raw string equality: each property is compared in its own domain. Numbers compare numerically (`6` == `6.0`); languages compare as languages, with ISO 639 spellings and BCP-47 tags reduced to canonical form (`de` == `ger`, `pt-Latn-BR` == `pt-BR`) while meaningful distinctions are preserved (`pt-BR` != `pt-PT`, `zh-Hans` != `zh-Hant`). Use `regex` for byte-literal matching. A `raw:`-prefixed key is outside all of this: it has no domain, and it compares without type conversion (4.4).
152:**Matchable properties** are generated at build time from the mkvmerge identification output schema: `type`, `codec_id`, `language`, `language_ietf`, `track_name`, `default_track`, `forced_track`, `enabled_track`, `flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original`, `audio_channels`, `audio_sampling_frequency`, `pixel_dimensions`, and the rest of the schema's track properties. Names in profiles are exactly the identification schema names.
158:| `language` | `--language` |
171:- `language` (matching): accepts ISO 639-2 (`ger`) and BCP-47 (`de`); matched semantically against both `language` and `language_ietf` as reported by mkvmerge. Valid values come from `mkvmerge --list-languages` at runtime.
174:- **Closed-domain values.** For properties whose value set is closed, an `exact` value outside the domain is `InvalidPropertyValue` (not a silent never-match): `type` and `codec_kind` at config time (against the pinned schema enum and the alias table respectively), `language` at plan time (against `mkvmerge --list-languages`). Open-ended values (`sub_charset`, free-text `track_name`) are exempt.
176:- **`raw:` opt-in (forward compatibility, D32).** A match property not in the pinned model is rejected at config time (`UnknownProperty`), which protects against typos: a mistyped name that would silently never-match is the worst failure mode for a declarative batch tool. To match a property the local mkvmerge reports but this build's schema does not yet carry (a newer identification schema), prefix the name with `raw:` inside `exact`/`substring`/`regex`, e.g. `exact: { raw:dolby_complexity_index: 3 }`. A `raw:` property bypasses the existence/type/domain checks and is matched untyped (value equality against the property named verbatim with no type conversion: a string matches only a reported string - byte-for-byte, no `language` normalization, no `codec_kind` aliasing - an integer only an integer, a float only a float, a boolean only a boolean, so `exact: { raw:x: 6 }` does not match a reported `6.0` the way `exact` on a known property does; and no false-when-absent Boolean shortcut). Config time flags the bypass (`RawProperty`, info; `RawOnKnownProperty`, warning, on `language`/`codec_kind`), and plan time raises `UnknownPropertySkew` per consumed `raw:` property (9.2). A `raw:` key with an empty bare name (a bare `raw:`) is a config-time error (`EmptyRawProperty`): it names no property and the rule could never match, which is always a typo. The prefix is a matching opt-in only; it is not accepted in `changes`, where an unknown key stays `UnknownSettableProperty`. YAML parses the prefix as part of the key (a colon not followed by a space stays inside the plain scalar), so no quoting is needed.
206:- The locator selects candidate files; the rule's `match` expression then selects exactly one track inside the located file. Donor files are full containers: an external MKV with a rule matching `{ type: audio, language: de }` is the supported way to pull matching German audio from a second release of the same series.
255:Core emits no user-facing prose: `code` plus structured `params` select and fill a message and hint template from the shared catalog at presentation time (8.4). `--json` output carries code and params plus the rendered message in the active locale, so scripts key on codes, humans read text. In every report document, the `config_diagnostics` array is ordered errors-first (error, warning, info), stable within a severity (ties keep collection order); per-file `diagnostics` and `batch_diagnostics` keep collection order.
261:| `MissingTrack` | error | non-optional rule matches 0 tracks; hint lists near-misses (tracks of same type/language and which condition each failed) |
280:| `RawOnKnownProperty` | warning | `raw:` applied to a model property with special matching semantics (`language`, `codec_kind`), degrading it to plain value equality against the property named verbatim, with no normalization and no type conversion (config-time; 4.4, 9.2) |
283:| `InvalidPropertyValue` | error | `exact` value outside a closed domain (`type`/`codec_kind` config-time, `language` plan-time; 4.4) |
302:Best-effort, file-independent checks at validate time: regex/template compilation, type errors, unknown properties (unless `raw:`-opted, 4.4/9.2; an empty bare `raw:` name is `EmptyRawProperty`), closed-domain value checks (`type`, `codec_kind`; `language`'s domain needs runtime and is checked at plan time), `codec_kind` exact-only, and provable rule overlaps (rule A's condition set logically subsumes rule B's, so any track matching B must overlap A). Static analysis never replaces the dry run; it catches what is decidable without looking at files.
331:├── locales/                # Fluent catalogs (locales/en/*.ftl), shared by CLI and frontend
332:└── help/                   # long-form help topics, markdown per locale (help/en/<help-id>.md)
340:| `capability` | mkvtoolnix model: build-time generated matchable properties, curated settable table, runtime queries (`--version`, `--list-types`, `--list-languages`), codec_kind aliases, version-skew handling |
361:muxsmith validate <profile> [--json] [--locale LOCALE]
363:                            [--on-collision POLICY] [--json] [--locale LOCALE]
366:                            [--json] [--locale LOCALE]
367:muxsmith identify <file> [--json] [--locale LOCALE]
386:App settings (not profile data): mkvmerge path override, default parallelism, interface language. Stored in the platform config directory. The language control is three-state - follow the system language, English, German - where following the system IS the absence of a stored override, is preselected until the user chooses otherwise, and stays reachable afterwards, so saving without touching the control stores no override (8.4).
410:Mechanics: every help-annotated element carries a stable `help-id`; help content is one markdown file per help-id per locale (`help/<locale>/<help-id>.md`), rendered in the sidebar. Views have their own help-ids for the default sidebar content.
416:- **No hardcoded user-facing strings** in any layer: not in the frontend, not in the CLI, not in core. Core emits diagnostic codes and params only (5.2); labels, tooltips, messages and hints live in Fluent catalogs; long-form help lives in per-locale markdown. Accepted v1 exceptions: clap's library-generated `--help`/usage text, third-party error text passed through as a `detail` param (regex, serde, I/O), the worker-panic payload surfaced as the `worker-panicked` message's `$detail` param (developer-diagnostic content, untranslatable by nature; 5.2), the fixed English framing in `IdentifyError`'s `Display` (e.g. "mkvmerge failed: ...") surfaced via a `detail` param, which wraps that same third-party mkvmerge/serde/I-O error text, and the JSON Schema's `description` fields (Rust doc comments, D47). The schema documents a file format, the same category as this spec and the README, both English-only by design; it is not application UI and not a diagnostic, so Fluent's localization mandate does not reach it.
417:- One catalog source of truth under `locales/`, consumed by fluent-rs (CLI rendering, embedded at build time) and @fluent/bundle in the frontend. Diagnostic message templates exist exactly once, shared by both surfaces.
418:- Locale selection: system locale with manual override in app settings (takes effect live, without restart; D56) and `--locale` on the CLI; falls back to English per message.
421:  D63); further locales are content work (non-goal 11).
426:2. **Runtime**: the local mkvmerge is queried for version, supported file types and languages. `mkvmerge -J` output carries `identification_format_version`. A property name unknown to the pinned model is a config-time `UnknownProperty` error (typo protection) unless the profile opts in with a `raw:` prefix (D32). A `raw:`-prefixed property bypasses the capability existence/type/domain checks and is matched untyped: value equality against the property named verbatim with no type conversion, so the profile value matches only a reported value of the same kind (a string only a string, compared byte-for-byte; an integer only an integer; a float only a float; a boolean only a boolean) and `raw:x: 6` does not match a reported `6.0`; with no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut. The opt-in is announced at config time by `RawProperty` (info), or by `RawOnKnownProperty` (warning) when the bare name is a model property with special matching semantics (`language`, `codec_kind`) that `raw:` degrades to byte-literal equality. An empty bare name is not an opt-in at all and is rejected at config time (`EmptyRawProperty`, error). At plan time each `raw:` property consumed while resolving a rule raises an `UnknownPropertySkew` warning carrying `property`, `found_version` (the file's `identification_format_version`) and `pinned` (this build's schema), so the untyped match is visible whether the runtime schema is genuinely newer than pinned or the same. Additionally, planning emits a `SchemaDrift` info notice once per batch if any source file's `identification_format_version` exceeds the pinned version, alerting the user that their mkvmerge may support properties not yet in Muxsmith's model and enabling opt-in via the `raw:` prefix. Forward compatibility without lying about type safety: the untyped path is opt-in and declared, never inferred from a version bump. Upgrading the pinned schema (item 1) remains the typed path for a property that should become first-class.
454:- Locales beyond English and German. The mechanism (8.4) ships complete on
455:  both surfaces; adding a locale is content work (catalogs + help topics
```

**Fired control:** section 8.4's own locale bullets are returned (the four bullets on no-hardcoded-strings, the one-catalog-source-of-truth rule, locale selection, and the English-and-German shipping statement), and section 8.2's amended app-settings paragraph is returned. Both are present, so the expression is aimed at what it claims.

**Classification of every hit:**

- The decision-log Localization row: **consistent**. It commits to i18n-readiness and to English and German content in v1. A three-state control adds no locale and removes none.
- The reference example's rule bodies and its narrative sentence about EN/DE audio and per-language subtitles, the `changes: { language: tr, ... }` line, the match-algebra case-sensitivity and typed-equality paragraphs, the matchable-property list, the settable-property table row, the `language` matching bullet, the closed-domain-values bullet, the `raw:` opt-in paragraph, the external-locator paragraph, the `MissingTrack` / `RawOnKnownProperty` / `InvalidPropertyValue` diagnostic rows, the static-lint paragraph, the architecture table's capability row, and the runtime capability paragraph: **all consistent, and all a different sense of the word**. Every one of these is `language` as an mkvmerge *track property*, not the interface language. Nothing in the amendment touches track-language matching.
- The architecture tree's `locales/` and `help/` lines: **consistent**. The amendment adds no catalog and moves none.
- The four CLI usage lines carrying `--locale LOCALE`: **consistent**. The CLI's per-invocation override is a separate surface from the GUI's stored setting; 8.4 already states both exist side by side, and the amendment says nothing about the CLI.
- Section 8.2's amended app-settings paragraph: **consistent by construction**, and one of the two fired controls. It is the Step 2 edit.
- Section 8.3's help mechanics sentence (`help/<locale>/<help-id>.md`): **consistent**. Per-locale help topics are unaffected by a control that selects among them.
- Section 8.4's no-hardcoded-strings bullet: **consistent, and strengthened by D110**. Its accepted-exception list does not include shell dialog text, which is exactly what D110's locale-aware `ftl_message` brings into conformance.
- Section 8.4's one-catalog-source-of-truth bullet: **consistent**. D110 has the shell read `gui-common.ftl` out of that same tree rather than introducing a second catalog.
- Section 8.4's locale-selection bullet (`system locale with manual override in app settings (takes effect live, without restart; D56)`): **consistent, and this is the sentence that makes 8.4 need no edit**. "System locale with manual override" is precisely the three-state semantics: following the system is the absence of an override, the override is the stored value, and the live-switch requirement is preserved by D106's decision that the switch resolves through the shared seam.
- Section 8.4's English-and-German shipping statement: **consistent**. It enumerates the surfaces that ship German content as the GUI catalogs and help topics plus the CLI's embedded catalogs. The Tauri shell's dialog strings are GUI user-visible strings under that statement rather than a fourth surface it omits, and D110 brings them into it. Recorded as an observation rather than a finding: the sentence under-specifies where the shell sits, it does not contradict the amendment.
- Section 11's non-goal `Locales beyond English and German`: **consistent**. The control's third state is not a locale; it selects between the two that ship.

No contradiction found under this expression.

### Expression 2

`grep -nEi 'undo|redo|unsaved|discard|confirm' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

```
382:1. **Profile editor**: track-rule grid (order, source, match summary, changes, optional; drag to reorder; Add appends an empty rule - incomplete until filled, announced by a validation warning - selects it and opens its detail editor; Remove deletes the selected rule without confirmation, legal down to zero rules (per 4.5: `unmatched: keep` = passthrough, `drop` = NoTrackRules)), detail editor per rule, panels for attachments/chapters/tags/title, create/open/save YAML, recent profiles. Saving writes canonical YAML rendered fresh from the in-memory model, not a patch of the file on disk: comments, key order and formatting are not preserved (D41), and a field left at its serde default is omitted rather than written back explicitly (D48). Inline validation markers from core diagnostics. New creates a blank profile in the editor and touches no file: the seed carries the format version, one candidate extension and one empty track rule, so it is incomplete-until-filled and announced by a validation warning exactly as Add's empty rule is, never by an error that would disable Save. A profile created this way has no path yet; Save opens a save dialog and the picked path becomes the profile's path from then on. Undo and redo cover every model mutation - field edits, rule add and remove including the unconfirmed delete, drag-reorder, and every list or map widget mutation - at one step per editing burst, where a burst ends at a focus change or a grid operation; saving marks a position in that history rather than clearing it, and the history is what the editor derives "has unsaved changes" from. The editor holds at most one profile. Replacing it - by creating another, or by opening one - warns first and only while unsaved changes exist, naming what would be overwritten; switching views never touches it; and closing the app with unsaved changes warns as well, in one prompt that also covers a running batch when both hold.
```

**Classification:** one hit, section 8.2's amended editor item, which is the only place in the spec where any of these five words occurs.

- The pre-existing clause `Remove deletes the selected rule without confirmation` and the appended guard sentences are **consistent** with each other, and deliberately so: D109's decision 8 states that the guard family's triggers are whole-model replacement and app exit and does not reach a single rule mutation, so Remove's no-confirmation rule is untouched. D66's premise that undo is the durable answer to accidental destruction is satisfied by the appended undo/redo sentence rather than reopened.
- The appended `has unsaved changes` derivation and the `warns first and only while unsaved changes exist` clause are **consistent** with each other: one names where the state comes from, the other gates every guard on it.

That this expression returns exactly one line is itself the finding-shaped fact worth stating: before the amendment the spec said nothing at all about undo, redo, unsaved state or discard guards, which is why the amendment was needed rather than an edit to an existing statement. Nothing prior exists to contradict.

No contradiction found under this expression.

### Expression 3

`grep -nEi 'create|new profile|open/save' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

```
382:1. **Profile editor**: track-rule grid (order, source, match summary, changes, optional; drag to reorder; Add appends an empty rule - incomplete until filled, announced by a validation warning - selects it and opens its detail editor; Remove deletes the selected rule without confirmation, legal down to zero rules (per 4.5: `unmatched: keep` = passthrough, `drop` = NoTrackRules)), detail editor per rule, panels for attachments/chapters/tags/title, create/open/save YAML, recent profiles. Saving writes canonical YAML rendered fresh from the in-memory model, not a patch of the file on disk: comments, key order and formatting are not preserved (D41), and a field left at its serde default is omitted rather than written back explicitly (D48). Inline validation markers from core diagnostics. New creates a blank profile in the editor and touches no file: the seed carries the format version, one candidate extension and one empty track rule, so it is incomplete-until-filled and announced by a validation warning exactly as Add's empty rule is, never by an error that would disable Save. A profile created this way has no path yet; Save opens a save dialog and the picked path becomes the profile's path from then on. Undo and redo cover every model mutation - field edits, rule add and remove including the unconfirmed delete, drag-reorder, and every list or map widget mutation - at one step per editing burst, where a burst ends at a focus change or a grid operation; saving marks a position in that history rather than clearing it, and the history is what the editor derives "has unsaved changes" from. The editor holds at most one profile. Replacing it - by creating another, or by opening one - warns first and only while unsaved changes exist, naming what would be overwritten; switching views never touches it; and closing the app with unsaved changes warns as well, in one prompt that also covers a running batch when both hold.
```

**Fired control:** the amended editor item is returned. Present.

**Classification:** one hit, section 8.2's amended editor item.

- `create/open/save YAML` in the capability list and `New creates a blank profile in the editor and touches no file` are **consistent**: the first names the capability, the second defines it.
- The appended `A profile created this way has no path yet; Save opens a save dialog` is **consistent** with the item's pre-existing saving paragraph (canonical YAML rendered fresh from the in-memory model, D41, defaults omitted per D48), which describes how a save writes and says nothing about where the path comes from.

No contradiction found under this expression.

### The two advance-named hits

The brief names two hits as known in advance and consistent, so they are not reported as findings: section 8.3's help-mode Escape sentence, and section 11's non-goals. **Neither appeared in the output of any of the three expressions** - section 8.3's Escape sentence contains none of the five words in expression 2, and section 11's non-goals contain none of expression 3's terms. Both remain non-findings on their own terms, checked by reading rather than by grep: the help-mode Escape sentence concerns help mode's own Escape channel and the settings dialog's native cancel, which is a different keyboard channel from the editor's undo/redo binding (D108 keeps them separate explicitly, noting that help mode's capture-phase handler takes only Escape and Enter/Space on a help target); and section 11's non-goals name neither profile creation nor undo/redo, so nothing there is contradicted by the amendment. This discrepancy between the brief's advance naming and the expressions' actual output is surfaced below rather than treated as a defect, because the step's completion criteria - every expression non-empty, both named fired controls present - are met either way and nothing depends on it.

> **[CORRECTED 2026-07-30, fix round.]** The bolded sentence "**Neither appeared in the output of any of the three expressions**" is false, and this report's own pasted evidence contradicts it. Section 11's non-goals DO appear: the `locale|language` expression returns two lines from section 11, file lines 454 and 455 (`- Locales beyond English and German. ...` and its continuation), both of them visible in Expression 1's pasted output above, and this report's own classification list for Expression 1 carries the entry "Section 11's non-goal `Locales beyond English and German`: **consistent**." The two narrow claims in the same sentence are true and were re-measured: section 8.3's help-mode Escape sentence returns 0 hits under all three expressions, and section 11 returns 0 under expressions 2 and 3. Only the unqualified generalisation was wrong. Correction C-1 under "Fix round, 2026-07-30" carries the per-passage measurement and its fired control.

## Step 5: verification

The full gate as `BUILDING.md` enumerates it: 11 parts, 6 Rust, 4 frontend, 1 house-knowledge. All run foreground on `master` in the main working tree. All green.

| # | part | result |
|---|---|---|
| 1 | `cargo fmt --all --check` | clean, exit 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| 3 | `cargo test --workspace` | `TOTAL passed=507 failed=0 across 39 suites` |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | `DOC EXIT=0` |
| 5 | `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` / `DENY EXIT=0` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | `EXIT=0` |
| 7 | `pnpm lint` | `EXIT=0` |
| 8 | `pnpm build` | `EXIT=0`, `built in 152ms` |
| 9 | `pnpm check:i18n` | `EXIT=0`, `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | `EXIT=0`, `68 passed (3.0s)` |
| 11 | `python3 scripts/ledger-lint.py` | `EXIT=0`, `ledger-lint: 566 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

The part-3 total was produced by re-running `cargo test --workspace` through `grep -E '^test result:'` and summing the passed and failed columns, so the figure is the sum over all suites rather than the tail of the run.

Scope of the diff, before staging:

```
=== git diff --stat ===
 docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md | 4 ++--
 1 file changed, 2 insertions(+), 2 deletions(-)
=== git status --short ===
 M docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
?? docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

`git diff --stat` shows one file because the second is new and therefore untracked; `git status --short` shows it as the only untracked entry. After the explicit staging of exactly the two paths, the staged diffstat covers both and nothing else:

```
=== git diff --cached --stat ===
 .../specs/2026-07-08-muxsmith-v1-design.md         |   4 +-
 .../specs/2026-07-30-plan-12-decisions.md          | 152 +++++++++++++++++++++
 2 files changed, 154 insertions(+), 2 deletions(-)
```

Together these cover exactly the two files in the brief's Files list. No file outside it was touched, other than this report, which the commit's pathspec excludes.

## Step 6: commit

Staged explicitly by path (never `git add -A`), committed unsigned and pathspec-scoped, one trailer:

```
[master b381675] spec+adr: the editor creates, undoes and guards its unsaved profile; app settings carry a three-state language
 2 files changed, 154 insertions(+), 2 deletions(-)
 create mode 100644 docs/superpowers/specs/2026-07-30-plan-12-decisions.md
=== SHA ===
b3816750c1842cdd2db26065adbc958c928a889f
=== trailers ===
spec+adr: the editor creates, undoes and guards its unsaved profile; app settings carry a three-state language

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

=== files in commit ===
b381675 spec+adr: the editor creates, undoes and guards its unsaved profile; app settings carry a three-state language
 .../specs/2026-07-08-muxsmith-v1-design.md         |   4 +-
 .../specs/2026-07-30-plan-12-decisions.md          | 152 +++++++++++++++++++++
 2 files changed, 154 insertions(+), 2 deletions(-)
```

Not pushed. The single push is a controller action at the plan close.

## D-number verification

The controller's resolution 1 was independently re-measured rather than taken on trust, because the number assignment is what the rest of the plan reads. Two sweeps over the tracked tree, excluding `node_modules` and `target`:

- **Assignment-shaped occurrences of D106-D110** (a heading, a leading bold `**D1nn`, or a leading bullet-bold) exist in exactly one file, `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` - the Decision register this task renders, plus its two `D107-i` / `D108-i` addendum paragraphs. Nothing outside plan 12 assigns any of the five.
- **Files mentioning D106-D110 at all**: the plan-11 raw-bytewise design (which states in its own D-number collision check that `D106`-`D110` are reserved by Plan 12), `docs/ROADMAP.md`, this plan, `docs/process-journal.md`, `docs/decision-ledger.yaml`, a session-29 close handoff, and a plan-11 amendment review. Every one of those is a reservation reference or a forward citation into this plan's register, never an assignment.
- **Highest D-number in `docs/`**: D111, Plan 11's, which skipped past the reserved block.

D106-D110 were therefore assigned exactly as the brief prescribes. No renumbering, and no NEEDS_CONTEXT over D111.

## Surfaced, not resolved

1. **The brief's two advance-named sweep hits do not appear in any of the three expressions' output.** Detail under Step 4. Both are non-findings on their own terms and the step's completion criteria are met, so nothing was changed and nothing was decided; it is surfaced because a reviewer walking the brief against this report will otherwise look for two hits that are not there. If the intent was that these expressions should reach those two passages, the expressions - not the passages - would be what needs revising, and that is a controller call rather than an implementer one.

   > **[CORRECTED 2026-07-30, fix round.]** The heading of this item is false as written: only ONE of the two advance-named passages is absent from all three expressions' output. Section 11's non-goals appear, twice, in Expression 1's output. The item stands only for section 8.3's help-mode Escape sentence, which returns 0 hits under all three expressions and is therefore genuinely unreachable by this sweep. Restated correctly, and measured per passage, in correction C-1 under "Fix round, 2026-07-30".
2. **Section 8.4's English-and-German shipping statement does not name the Tauri shell among the surfaces that ship German content.** It enumerates the GUI catalogs and help topics plus the CLI's embedded catalogs. D110 brings the shell's dialog strings into German, which is consistent with the statement read as covering GUI user-visible strings, but the sentence does not say so in as many words. Not a contradiction and not edited - the brief's Files list is exhaustive and 8.4 is deliberately not touched - surfaced so a later reader deciding whether 8.4 wants a clause has the observation on record.
3. **A house-form deviation, made deliberately and named here rather than absorbed.** The plan-5.7 file writes `**Triggers created:** none.` explicitly even where there is no trigger; the new file omits the slot in the three decisions that create none, because the brief's enumeration of the bold slots says "where one exists". The dispatch's grant to follow a touched file's structural patterns fills silence only, and this was not silence.
4. **No house-knowledge YAML, `docs/ROADMAP.md` or `docs/process-journal.md` was edited**, per the plan's Global Constraints. Nothing ledger-worthy beyond the three items above was found in this task.

---

## Fix round, 2026-07-30

An independent review of this report found three defects **in its evidence**, not
in the artifacts it produced. The spec amendment, the decisions file and commit
`b3816750c1842cdd2db26065adbc958c928a889f` all passed review and are unchanged:
this round touched no file except this report, made no commit, and amended none.

The three findings, and where each is answered:

| finding | what was wrong | answered by |
|---|---|---|
| I-1, important | two absence-shaped verifications (the ordinal sweep under fixed property 2, and the `**Typography.**` paragraph) were narrated instead of pasted, so neither could be re-run, only re-derived | checks A and B below |
| I-2, important | Step 4's bolded "**Neither appeared in the output of any of the three expressions**" contradicts Step 4's own pasted output | correction C-1 |
| M-1, minor | "the one place an item ordinal survives" is a recalled observation presented as a sweep result, and is false | correction C-2 |

Every original sentence is left standing where it was written; each defective
passage carries an inline `[CORRECTED 2026-07-30, fix round.]` marker pointing
here, so both the claim and the measurement stay visible.

**Rule this round exists to restore** (`design-empirical-claims-reproducible`,
restated in plan 12's Global Constraints as "Evidence lines carry pasted
output"): every command below is written out, every output below is the live
output of the command directly above it, and every absence claim carries a run
of the same expression against a case where it is known to fire. All runs are
foreground, in the main working tree, on `master`, with the tree clean.

### Check A: the ordinal sweep, re-run for real

The claim under repair is fixed property 2's "No ordinal into either list
appears anywhere in the file", asserted over
`docs/superpowers/specs/2026-07-30-plan-12-decisions.md`. The original
instrument enumerated ordinal WORDS only. That enumeration is itself a claim,
and it is the one that failed: an expression over words cannot see `decision 3`.
The sweep is therefore run in three halves - ordinal words, the numeric form,
and the suffixed numeric form - and the numeric half is what M-1 exposed.

#### A1: ordinal words in list position

Command:

```bash
grep -nEi '(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|last|final|preceding|previous|above|below)[^.]{0,40}(rejected|alternative|bullet|option|item)|(rejected|alternative|bullet|option|item)[^.]{0,40}(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|last|final|preceding|previous|above|below)' docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

Output:

```
20:- **Initialise the control from the effective locale and leave the option set at two.** Steelman, at strength: one line, no catalog key, no shared seam, no test infrastructure, and it removes the visible disagreement the owner actually reported, which is the whole of what he saw. Rejected because it is defective on any system whose locale is neither German nor English, where the control would hold a value absent from its own option list, and because the first Save would still lock the user out of system-following permanently - the part of the finding that is a spec defect rather than a display mismatch. Recorded as having lost in the ROADMAP already; carried here so this record is self-contained.
145:- **Let the shell resolve independently** (stored setting, then `sys_locale::get_locale()`, then en - a literal transcription of D63's own cascade). Steelman, at strength: it is the closest possible conformance to the house pattern the ruling itself points at; it needs no command, no state field and no frontend wiring, so the shell keeps working even if the frontend never speaks; and `sys-locale` is already in the workspace's tree, so the dependency is free in practice. Rejected because it creates a third implementation of the rule whose duplication is the finding under repair in W1, and because its two fallbacks can disagree exactly where the user has no stored override - the case the QA finding came from.
[exit 0 - 0 means the expression returned hits]
```

Both hits are false positives of the expression, not ordinals into a list, and
this is the same pair the original narration described: "the first Save" is
temporal and names the next save action, "a third implementation" is a cardinal
count of implementations. Neither points into the rejected-alternatives list.

#### A2: the numeric form, which the original instrument could not see

The noun set is deliberately wider than the original's, because a noun the
expression does not enumerate is invisible to it:

```bash
grep -nEoi '(decision|alternative|bullet|option|item|point|step|paragraph|slot)s? [0-9]+' docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

Output (39 matches; the full list, since the count is the load-bearing value):

```
16:decision 2
16:Decision 3
16:Decision 5
16:Decision 6
16:Decision 7
16:Decision 8
23:decision 3
39:Decision 2
39:Decision 4
39:Decision 5
39:Decision 6
39:Decision 7
39:Decision 8
67:decision 10
70:decision 1
70:Decision 2
70:Decision 3
70:Decision 4
70:decision 3
70:Decision 5
70:Decision 6
70:Decision 7
70:Decision 8
70:Decision 9
70:Decision 10
70:decision 9
103:decision 5
103:Decision 5
107:decision 5
114:Decision 6
114:Decision 9
120:decision 9
120:Decision 9
132:decision 2
139:decision 1
139:Decision 2
139:Decision 3
139:Decision 4
139:Decision 5
```

Normalised, so the shape rather than the number carries:

```bash
grep -nEoi '(decision|alternative|bullet|option|item|point|step|paragraph|slot)s? [0-9]+' docs/superpowers/specs/2026-07-30-plan-12-decisions.md | sed -E 's/^[0-9]+://; s/ [0-9]+$/ <n>/' | tr 'A-Z' 'a-z' | sort | uniq -c
```

```
     39 decision <n>
```

Distinct lines carrying at least one:

```bash
grep -nEoi '(decision|alternative|bullet|option|item|point|step|paragraph|slot)s? [0-9]+' docs/superpowers/specs/2026-07-30-plan-12-decisions.md | cut -d: -f1 | sort -un | tr '\n' ' '
```

```
16 23 39 67 70 103 107 114 120 132 139 
```

So: **39 references, all of them of the form `decision <n>`, on 11 distinct
lines.** Widening the noun set past the original's five to nine did not move the
count, which is the measured answer to "is the enumeration complete enough" - no
`alternative <n>`, `bullet <n>`, `option <n>`, `item <n>`, `point <n>`,
`step <n>`, `paragraph <n>` or `slot <n>` occurs anywhere in the file.

#### A3: which slot each reference sits in

A raw count does not say whether a reference points into a rejected-alternatives
list, which is what fixed property 2 is actually about. This pass tracks the
enclosing decision and bold slot and attributes every match:

```bash
awk '
  /^## D[0-9]+/ { dec=substr($2,1,length($2)-1); slot="(preamble)" }
  /^\*\*[A-Za-z]/ { slot=$0; sub(/^\*\*/,"",slot); sub(/[:.]\*\*.*/,"",slot) }
  {
    line=$0
    while (match(line, /[Dd]ecision [0-9]+|[Aa]lternative [0-9]+|[Bb]ullet [0-9]+|[Oo]ption [0-9]+|[Ii]tem [0-9]+/)) {
      printf "%d\t%s\t%s\t%s\n", NR, dec, slot, substr(line, RSTART, RLENGTH)
      line = substr(line, RSTART+RLENGTH)
    }
  }
' docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

Output:

```
16	D106	Rationale	decision 2
16	D106	Rationale	Decision 3
16	D106	Rationale	Decision 5
16	D106	Rationale	Decision 6
16	D106	Rationale	Decision 7
16	D106	Rationale	Decision 8
23	D106	Rejected alternatives	decision 3
39	D107	Rationale	Decision 2
39	D107	Rationale	Decision 4
39	D107	Rationale	Decision 5
39	D107	Rationale	Decision 6
39	D107	Rationale	Decision 7
39	D107	Rationale	Decision 8
67	D108	Decision	decision 10
70	D108	Rationale	decision 1
70	D108	Rationale	Decision 2
70	D108	Rationale	Decision 3
70	D108	Rationale	Decision 4
70	D108	Rationale	decision 3
70	D108	Rationale	Decision 5
70	D108	Rationale	Decision 6
70	D108	Rationale	Decision 7
70	D108	Rationale	Decision 8
70	D108	Rationale	Decision 9
70	D108	Rationale	Decision 10
70	D108	Rationale	decision 9
103	D109	Decision	decision 5
103	D109	Decision	Decision 5
107	D109	Decision	decision 5
114	D109	Rationale	Decision 6
114	D109	Rationale	Decision 9
120	D109	Rejected alternatives	decision 9
120	D109	Rejected alternatives	Decision 9
132	D110	Decision	decision 2
139	D110	Rationale	decision 1
139	D110	Rationale	Decision 2
139	D110	Rationale	Decision 3
139	D110	Rationale	Decision 4
139	D110	Rationale	Decision 5
```

Summarised by slot:

```bash
awk '
  /^## D[0-9]+/ { dec=substr($2,1,length($2)-1); slot="(preamble)" }
  /^\*\*[A-Za-z]/ { slot=$0; sub(/^\*\*/,"",slot); sub(/[:.]\*\*.*/,"",slot) }
  {
    line=$0
    while (match(line, /[Dd]ecision [0-9]+|[Aa]lternative [0-9]+|[Bb]ullet [0-9]+|[Oo]ption [0-9]+|[Ii]tem [0-9]+/)) {
      printf "%d\t%s\t%s\t%s\n", NR, dec, slot, substr(line, RSTART, RLENGTH)
      line = substr(line, RSTART+RLENGTH)
    }
  }
' docs/superpowers/specs/2026-07-30-plan-12-decisions.md | awk -F'\t' '{print $2"\t"$3}' | sort | uniq -c
```

```
      6 D106	Rationale
      1 D106	Rejected alternatives
      6 D107	Rationale
      1 D108	Decision
     12 D108	Rationale
      3 D109	Decision
      2 D109	Rationale
      2 D109	Rejected alternatives
      1 D110	Decision
      5 D110	Rationale
```

Note on the instrument itself: the first version of this awk pass used `next`
after matching a bold-slot line, which made it skip the slot-marker lines - and
those lines carry prose, so it silently missed every reference in a
`**Rationale:**` line. That version reported 8 matches where the grep reported
39. The version above sets the slot and then falls through to scan the same
line, and its total agrees with A2's independent count. A disagreement between
two instruments over the same file is how that blind spot surfaced.

#### A4: the suffixed numeric form

```bash
grep -nEo '[0-9]+(st|nd|rd|th)\b' docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

Output:

```
[exit 1 - 1 means no hit]
```

An absence, so it carries its control below with the others.

#### A-control: all three halves fired against a known-present case

An empty or thin result and a broken pattern look identical, and each half of
this instrument contains an enumerated set whose members are individually
invisible if missing. The probe therefore carries one known-present case per
enumerated noun, not one representative case:

```bash
cat > /tmp/ordinal-probe.md <<'PROBE'
## D999: probe for the ordinal instrument
**Rejected alternatives:**
- Rejected on decision 3's reasoning.
- Superseded by alternative 2.
- See bullet 4 of this slot.
- Covered by option 1.
- Restated at item 7.
- Answered under point 5.
- Carried out at step 6.
- Restated in paragraph 8.
- Named in slot 9.
- The 3rd alternative already answers this.
- The first rejected alternative already answers this.
PROBE
```

Control a, the numeric half, must return one line per enumerated noun:

```bash
grep -nEoi '(decision|alternative|bullet|option|item|point|step|paragraph|slot)s? [0-9]+' /tmp/ordinal-probe.md
```

```
3:decision 3
4:alternative 2
5:bullet 4
6:option 1
7:item 7
8:point 5
9:step 6
10:paragraph 8
11:slot 9
```

All nine nouns fire, so none of them is a member the expression cannot see.

Control b, the suffixed half:

```bash
grep -nEo '[0-9]+(st|nd|rd|th)\b' /tmp/ordinal-probe.md
```

```
12:3rd
```

Control c, the ordinal-word half:

```bash
grep -nEi '(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|last|final|preceding|previous|above|below)[^.]{0,40}(rejected|alternative|bullet|option|item)|(rejected|alternative|bullet|option|item)[^.]{0,40}(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|last|final|preceding|previous|above|below)' /tmp/ordinal-probe.md
```

```
13:- The first rejected alternative already answers this.
```

All three halves fire. The A2/A3/A4 results are therefore measurements, not
silence from a malformed pattern.

### Check B: the typography scan, re-run for real

The claim under repair is the `**Typography.**` paragraph's, asserted over both
touched files. Its original evidence was the bare number `13` and a note saying
what that number had to be, which is a summary of a run rather than the run.

#### B1: the scan

Command:

```bash
grep -nP '[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]' \
  docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md \
  docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

Output:

```
[exit 1 - 1 means no denylist character found; the command printed nothing above this line]
```

#### B-control: every one of the thirteen members fired individually

The scan's character class is an enumerated set, so firing it against one
representative member would leave a missing member invisible. The probe carries
one line per member, and each member is then fired by its own single-character
expression, so the output names which member fired on which line rather than
counting fires:

```bash
printf 'U+2010 HYPHEN [‐]\nU+2011 NON-BREAKING HYPHEN [‑]\nU+2012 FIGURE DASH [‒]\nU+2013 EN DASH [–]\nU+2014 EM DASH [—]\nU+2015 HORIZONTAL BAR [―]\nU+2018 LEFT SINGLE QUOTE [‘]\nU+2019 RIGHT SINGLE QUOTE [’]\nU+201C LEFT DOUBLE QUOTE [“]\nU+201D RIGHT DOUBLE QUOTE [”]\nU+2026 ELLIPSIS […]\nU+00A0 NO-BREAK SPACE [ ]\nU+2212 MINUS SIGN [−]\n' > /tmp/typo-probe.txt
```

Control 1, the same class expression against the probe, which must return all
thirteen lines - this is what proves the range `\x{2010}-\x{2015}` genuinely
covers all six dashes rather than one of them:

```bash
grep -nP '[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]' /tmp/typo-probe.txt
```

```
1:U+2010 HYPHEN [‐]
2:U+2011 NON-BREAKING HYPHEN [‑]
3:U+2012 FIGURE DASH [‒]
4:U+2013 EN DASH [–]
5:U+2014 EM DASH [—]
6:U+2015 HORIZONTAL BAR [―]
7:U+2018 LEFT SINGLE QUOTE [‘]
8:U+2019 RIGHT SINGLE QUOTE [’]
9:U+201C LEFT DOUBLE QUOTE [“]
10:U+201D RIGHT DOUBLE QUOTE [”]
11:U+2026 ELLIPSIS […]
12:U+00A0 NO-BREAK SPACE [ ]
13:U+2212 MINUS SIGN [−]
```

Control 2, each member on its own:

```bash
for cp in 2010 2011 2012 2013 2014 2015 2018 2019 201C 201D 2026 00A0 2212; do
  hit=$(grep -nP "[\\x{$cp}]" /tmp/typo-probe.txt)
  printf 'U+%s -> %s\n' "$cp" "${hit:-NO HIT (instrument blind to this member)}"
done
```

```
U+2010 -> 1:U+2010 HYPHEN [‐]
U+2011 -> 2:U+2011 NON-BREAKING HYPHEN [‑]
U+2012 -> 3:U+2012 FIGURE DASH [‒]
U+2013 -> 4:U+2013 EN DASH [–]
U+2014 -> 5:U+2014 EM DASH [—]
U+2015 -> 6:U+2015 HORIZONTAL BAR [―]
U+2018 -> 7:U+2018 LEFT SINGLE QUOTE [‘]
U+2019 -> 8:U+2019 RIGHT SINGLE QUOTE [’]
U+201C -> 9:U+201C LEFT DOUBLE QUOTE [“]
U+201D -> 10:U+201D RIGHT DOUBLE QUOTE [”]
U+2026 -> 11:U+2026 ELLIPSIS […]
U+00A0 -> 12:U+00A0 NO-BREAK SPACE [ ]
U+2212 -> 13:U+2212 MINUS SIGN [−]
```

Thirteen members, thirteen distinct fires, no member the instrument is blind to.
B1's empty result is therefore a measurement.

**Consequence worth declaring, because it is a trap for the next scan.** The
control cannot be written without the characters it fires on, so the fences
above deliberately contain every one of the thirteen denylist members. This
report therefore does NOT pass a typography scan, by construction, and a later
reader running one over `.superpowers/` will get hits here that are evidence
rather than defects. B1 is unaffected: it names its two files explicitly and
neither is this one. The alternative - writing the probe in escape notation -
would have turned the control back into a narration of itself, which is the
defect this round exists to remove.

### Correction C-1 (finding I-2): what the three sweep expressions actually reach

**The false claim.** Step 4's "The two advance-named hits" paragraph states, in
bold: "**Neither appeared in the output of any of the three expressions**".

**The corrected fact.** One of the two DID appear. The `locale|language`
expression returns two lines from section 11, and both are in this report's own
pasted Expression 1 output, and this report's own classification list for
Expression 1 already carries the entry "Section 11's non-goal `Locales beyond
English and German`: **consistent**." The claim contradicted evidence printed a
few hundred lines above it in the same document.

Measured per passage. Section boundaries first, since the measurement is
range-restricted and the ranges are themselves a claim:

```bash
grep -nE '^#{1,3} ' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md | grep -E '^[0-9]+:#+ (8|9|10|11|12)'
```

```
356:## 8. Surfaces
358:### 8.1 CLI
378:### 8.2 GUI
390:### 8.3 Self-explanation and help mode
412:### 8.4 Internationalization architecture
423:## 9. Capability model and version skew
428:## 10. Testing
442:## 11. Non-goals for v1
459:## 12. Licensing
```

So section 8.2 is lines 378-389, section 8.3 is 390-411, section 8.4 is 412-422,
and section 11 is 442-458. The help-mode Escape sentence is line 396, and it
spells the key `Esc` rather than `Escape`, which is why a naive search for the
word "Escape" does not find it:

```bash
sed -n '396p' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

```
- A prominent Help/Guide button, always visible in every view. Clicking it toggles help mode; clicking again (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits.
```

Each of the three sweep expressions against each named passage:

```bash
F=docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
for range in "8.3-Escape-sentence:396:396" "8.3-whole-section:390:411" "11-non-goals:442:458"; do
  name=${range%%:*}; rest=${range#*:}; lo=${rest%%:*}; hi=${rest##*:}
  for expr in 'locale|language' 'undo|redo|unsaved|discard|confirm' 'create|new profile|open/save'; do
    n=$(awk -v lo=$lo -v hi=$hi 'NR>=lo && NR<=hi' $F | grep -Ei "$expr" | wc -l)
    printf '%-24s lines %s-%-4s  %-38s -> %s hit(s)\n' "$name" "$lo" "$hi" "$expr" "$n"
  done
done
```

```
8.3-Escape-sentence      lines 396-396   locale|language                        -> 0 hit(s)
8.3-Escape-sentence      lines 396-396   undo|redo|unsaved|discard|confirm      -> 0 hit(s)
8.3-Escape-sentence      lines 396-396   create|new profile|open/save           -> 0 hit(s)
8.3-whole-section        lines 390-411   locale|language                        -> 1 hit(s)
8.3-whole-section        lines 390-411   undo|redo|unsaved|discard|confirm      -> 0 hit(s)
8.3-whole-section        lines 390-411   create|new profile|open/save           -> 0 hit(s)
11-non-goals             lines 442-458   locale|language                        -> 2 hit(s)
11-non-goals             lines 442-458   undo|redo|unsaved|discard|confirm      -> 0 hit(s)
11-non-goals             lines 442-458   create|new profile|open/save           -> 0 hit(s)
```

The two section-11 hits, verbatim (numbering is relative to the section, so 13
and 14 are file lines 454 and 455, exactly the two lines at the tail of
Expression 1's pasted output above):

```bash
awk 'NR>=442 && NR<=458' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md | grep -nEi 'locale|language'
```

```
13:- Locales beyond English and German. The mechanism (8.4) ships complete on
14:  both surfaces; adding a locale is content work (catalogs + help topics
```

**C-1 control.** Seven of the nine cells above are zeros, so the range-restricted
instrument has to be shown firing. Section 8.2 (lines 378-389) is the
known-present case: its amended editor item contains terms of all three
expressions, so all three must return a hit there.

```bash
for expr in 'locale|language' 'undo|redo|unsaved|discard|confirm' 'create|new profile|open/save'; do
  n=$(awk 'NR>=378 && NR<=389' $F | grep -Ei "$expr" | wc -l)
  printf '8.2 lines 378-389   %-38s -> %s hit(s)\n' "$expr" "$n"
done
```

```
8.2 lines 378-389   locale|language                        -> 1 hit(s)
8.2 lines 378-389   undo|redo|unsaved|discard|confirm      -> 1 hit(s)
8.2 lines 378-389   create|new profile|open/save           -> 1 hit(s)
```

All three fire on the known-present section, so the zeros over section 8.3 and
over section 11 under expressions 2 and 3 are real absences.

**C-1, stated as the report should have stated it.** Of the brief's two
advance-named hits, section 11's non-goals ARE returned - twice, by the
`locale|language` expression, at file lines 454 and 455 - and are classified as
consistent in this report's Expression 1 classification list. Section 8.3's
help-mode Escape sentence is returned by none of the three expressions: it holds
0 hits under all three, and the whole of section 8.3 holds 0 under expressions 2
and 3. Both remain non-findings on their own terms; what changes is that only
one of them is unreachable by the sweep, not both. The surfaced item 1 under
"Surfaced, not resolved" therefore stands for section 8.3 alone, and the
question it hands to the controller - whether expression 2 should have been able
to reach that sentence - is unchanged in substance and halved in scope.

### Correction C-2 (finding M-1): the "one place an item ordinal survives" clause

**The false claim.** The "No line-number citations" paragraph closes: "the one
place an item ordinal survives is D109's own decision 9 pointing at 'decision
5's table'".

**The corrected fact,** from check A above. The decisions file carries **39
references of the form `decision <n>`, on 11 distinct lines** (A2), and **two of
those lines sit inside a Rejected-alternatives slot** (A3): line 23 in
D106, whose alternative closes "Rejected on decision 3's reasoning", and line
120 in D109, whose alternative reads "not superseded by decision 9". The named
reference is real - it is line 107, inside D109's Decision slot - but it is
neither the only such reference nor the only one in a rejected-alternatives
bullet.

**Where this differs from the review's own measurement, stated rather than
smoothed over.** The verdict recording M-1 names "one inside a
rejected-alternatives bullet" and cites D106's. The attribution pass finds two
lines, carrying three references, in that slot: D106's line 23 and D109's line
120. The count of references (39) and of distinct lines (11) reproduce exactly;
only the rejected-alternatives subtotal is larger than the verdict's. It is
recorded here as measured rather than reshaped to agree.

**The artifact is not defective, and was not changed.** The brief's fixed
property reads "No line-number citations, in either direction ... Name sections,
symbols and decisions", and the house form numbers its decisions precisely so
the Rationale slot can point at them. Naming a decision is the prescribed form,
not a violation. The narrower property that fixed property 2 is actually about -
no reference identifying a REJECTED ALTERNATIVE by its position in the
rejected-alternatives list - holds, and is now measured rather than recalled: A2
finds zero occurrences of `alternative <n>`, `bullet <n>`, `option <n>` or
`item <n>` anywhere in the file, A4 finds zero suffixed ordinals, A1's two
ordinal-word hits are both false positives, and all three halves are shown
firing in A-control.

**What actually failed** was the instrument, exactly as M-1 says: an expression
enumerating ordinal WORDS cannot see the numeric form, so firing it against a
file that contains ordinal words proved the pattern valid and never proved the
enumeration complete. The clause it produced was a recollection wearing a
measurement's clothes.

### Also re-run, since C-2 rewrites the end of its paragraph

The same paragraph's principal claim - no line-number citations in the new file -
was pasted-with-control in the original report and is not among the findings. It
was re-run anyway, because a correction to the end of a paragraph is worth
nothing if the start of it has since drifted:

```bash
grep -nE '\.md:[0-9]+|line [0-9]+|:[0-9]{1,4}\b' docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

```
[exit 1 - 1 means no hit]
```

Control, the same pattern against the plan-5.7 decisions file, which carries
`runtime.rs:206`:

```bash
grep -nE '\.md:[0-9]+|line [0-9]+|:[0-9]{1,4}\b' docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md
```

```
16:(runtime.rs:206, Parse error) - the command-building side gets the same
[exit 0 - 0 means the pattern fires]
```

The claim holds.

### Scope of this round

This report cannot appear in `git status`: `.superpowers/` is ignored, which is
worth stating because an empty status is otherwise indistinguishable from "no
work was done".

```bash
git check-ignore -v .superpowers/sdd/plan-12/task-1-report.md
```

```
.gitignore:2:.superpowers/	.superpowers/sdd/plan-12/task-1-report.md
```

So the claim that has to be shown is the other one: `docs/` is untouched and HEAD
is where task 1 left it plus the ledger commit that followed.

```bash
git status --short docs/ ; git diff --stat HEAD -- docs/ ; git log --oneline -1
```

```
f083bc2 ledger: the task-1 harvest - an exemplar is not a corpus, and a sweep cannot see the document it was written against
```

Both `docs/` commands print nothing, which is an absence and therefore needs its
control. The same pathspec against `HEAD~1`, a commit that did change `docs/`:

```bash
git diff --stat HEAD~1 -- docs/
```

```
 docs/decision-ledger.yaml     | 28 ++++++++++++++++++++++++++++
 docs/process-conventions.yaml |  6 ++++--
 2 files changed, 32 insertions(+), 2 deletions(-)
```

The pathspec reaches `docs/` and reports changes when there are any, so the two
empty results above are measurements. (The control compares against a commit
rather than mutating a file, because this round is not permitted to touch
`docs/` even transiently.)

The only modified file is this report. No file under `docs/` was touched, no
commit was made, commit `b381675` was not amended, and nothing was pushed. The
three findings were defects in this report's evidence; the artifacts they
describe were correct and remain byte-identical.
