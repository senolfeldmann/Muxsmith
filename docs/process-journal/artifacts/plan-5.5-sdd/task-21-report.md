# Task 21 report: German locale (#17 step 3)

Branch `plan55-t21` (worktree `.worktrees/t21`), based on post-T19/T20 master.
Six German Fluent catalogs, loader primary-subtag normalization (S15), one
de e2e case. **This draft goes to Şenol for the terminology gate BEFORE
merge** (walkthrough #17). The table below is the review surface.

---

## 1. Terminology table (review this)

### 1a. Anchored in mkvtoolnix-de (`~/Downloads/mkvtoolnix/po/de.po`)

Single established domain **terms** only (SI-3 licensing boundary: facts,
no sentence copying). Line numbers are in that `de.po`.

| Muxsmith (en) | German | mkvtoolnix-de anchor |
|---|---|---|
| track | **Spur** | `Track` → `Spur` (1773) |
| default track | **Standardspur** | `Default track` → `Standardspur` (11963) |
| forced (display) | **erzwungen** / "Anzeige erzwingen" | `Forced display` → »Anzeige erzwingen« (11969) |
| file | **Datei** | pervasive (`Datei »{0}«`) |
| container | **Container** | `container` → `Container` (5705) |
| attachment | **Dateianhang** | `Attachment` → `Dateianhang` (9262) |
| chapter(s) | **Kapitel** | `Chapters` → `Kapitel` (2222) |
| tag(s) | **Tags** | `Tags` → `Tags` (2393) |
| language | **Sprache** | `Language` → `Sprache` (1902) |
| codec | **Codec** | `Codec` → `Codec` (11108) |
| profile | **Profil** | `H.264 profile` → `H.264-Profil` (2561) |
| mux / multiplexing | **multiplexen / Multiplexen / Multiplex-Quelle** | `Multiplexing took` → `Das Multiplexen dauerte` (7973); `Start multiplexing` → `Multiplexen starten` (9634) |
| error (severity) | **Fehler** | `Error` → `Fehler` (15277) |
| warning (severity) | **Warnung** | `Warning:` → `Warnung:` (3098) |
| directory | **Verzeichnis** | `Directory` → `Verzeichnis` (11127) |
| output file | **Ausgabedatei** | `Ausgabedatei` (4379) |
| job queue | **Jobwarteschlange** / Job | `Job queue` → `Jobwarteschlange` (9548) |
| copy to clipboard | **In die Zwischenablage kopieren** | `C&opy to clipboard` → `In &Zwischenablage kopieren` (11030) |
| Save / Cancel / Add / Start | **Speichern / Abbrechen / Hinzufügen / Starten** | 16421 / 8824 / 15747 / 8349 |

GUI imperative register per mkvtoolnix-de: **infinitive** ("Multiplexen
starten", "In Zwischenablage kopieren", "Hinzufügen"). CLI hint imperatives
use the Sie-less **du-Imperativ** ("Installiere...", "gib... an").

### 1b. Own choices (Muxsmith-specific; no mkvtoolnix anchor) — decide these

| Muxsmith (en) | German | Rationale / alternative |
|---|---|---|
| rule | **Regel** | brief-mandated; standard |
| suggestion | **Vorschlag** | brief-mandated; standard |
| **batch** | **Stapel** | proper German (Stapel[verarbeitung]). **Alt: keep "Batch"** (anglicism, also common in DE IT). Affects `nav-batch`, `batch-view-heading`, run tooltips. ⚑ |
| **Run** (button) | ~~Ausführen~~ → **Starten** | execute/run; mkvtoolnix's "Multiplexen starten" is longer. **OWNER-CORRECTED (2026-07-12 review): "Starten".** ✓ |
| dry run | **Probelauf** | trial run without side effects; natural (over literal "Trockenlauf"). ⚑ |
| **diagnostic(s)** | ~~Diagnose(n)~~ → **Meldung(en)** | panel heading was "Diagnosen". **OWNER-CORRECTED (2026-07-12 review): "Meldung(en)"** (softer, common for a messages panel); applies everywhere "diagnostic" maps to German, not only the panel heading. ✓ |
| identifier | **Kennung** | standard DE for identifier (`dry-run-file`, `duplicate-identifier`, `batch-file-caption`). ⚑ |
| (external) locator | ~~(externer) Locator~~ → **(externer) Verweis** | Muxsmith concept. **OWNER-CORRECTED (2026-07-12 review): "externer Verweis"**; case endings adjusted (des Verweises, mit dem Verweis). ✓ |
| resolution group | **Behebungsgruppe** | "Behebung" (fixing), deliberately **not** "Auflösung" (would clash with video "resolution"). ⚑ |
| info notice (GUI count) | **Hinweis(e)** | "notice" → "Hinweis"; kept distinct from CLI "info" → "Info" to mirror en's own "info" vs "info notice" split. ⚑ |
| info (CLI severity) | **Info** | short inline `[severity]` marker; mkvtoolnix's "Informationen" (9426) is too long here. |
| donor | **Spender(-Datei)** | external donor file providing tracks/attachments. |
| capability model | **Fähigkeitsmodell** | |
| template | **Vorlage** | filename template. |
| state (job) | **Status** | mkvtoolnix uses "Status". |
| queued (state chip) | **In Warteschlange** | **Alt: "Wartet" / "Eingereiht".** ⚑ |
| run (noun) | **Lauf** | one execution of the queue. |
| history | **Verlauf** | |
| log | **Protokoll** | standard DE software term. |
| refinement | **Verfeinerung** | |

⚑ = the choices most worth an explicit yes/no.

### 1c. Owner corrections (terminology review, 2026-07-12)

Şenol's pass approved the catalogs with three corrections, applied in
`locales/de/gui-batch.ftl` and `locales/de/diagnostics.ftl` only (en
reference untouched); commit `i18n(de): terminology corrections from the
owner review (Starten, Meldungen, Verweis)` on branch `plan55-t21`.

1. **Run button: Ausführen → Starten.** Every occurrence naming the Run
   action, including nominalized/verb forms in its tooltips. 5 occurrences
   (`gui-batch.ftl`: `batch-run`, `batch-run-tooltip-no-profile`,
   `batch-run-tooltip-errors`, `batch-run-tooltip-mkvmerge-missing`,
   `batch-profile-pick-tooltip`). Generic-verb uses of "ausführen" that
   are not the Run action (a worker thread "beim Ausführen dieses Jobs"
   in `diagnostics.ftl`'s `worker-panicked`; the parallel-jobs description
   in `gui-settings.ftl`'s `settings-default-jobs-hint`) were left as-is;
   they describe execution generically, not the button.
2. **Diagnostics panel: Diagnose(n) → Meldung(en).** 3 occurrences
   (`gui-batch.ftl`: `batch-diagnostics-heading`, `batch-file-no-plan`,
   `batch-run-tooltip-errors`). Same gender (die Diagnose/die Meldung), no
   article changes needed.
3. **Locator → Verweis.** 2 occurrences (`diagnostics.ftl`:
   `missing-external`, `ambiguous-external`), both "externen Verweis"
   (masc. accusative, same as the replaced "externen Locator" — no case
   change needed). `locator-conflict`'s message id and value were left
   untouched: the id doesn't translate (parity-enforced) and its value
   never contained the word "Locator" in either language.

Verification: `pnpm check:i18n` ok (same 12 pre-existing unused-key
warnings, exit 0); `pnpm lint` ok; `pnpm test:e2e` 5/5 pass (the de e2e
case pins "Stapel"/"Probelauf"/`batch-profile-current`, none of which this
correction touched). No Rust source embeds any `locales/de/*` file (only
`locales/en/*` via `include_str!`), so `cargo test` was not re-run.

---

## 2. Per-catalog notes

- **cli.ftl** (36 keys): status/diagnostic lines, declarative. `identifier`
  → Kennung; `output` → Ausgabe. Terse job-state markers: `Start / ok /
  Warnung / fehlgeschlagen / abgebrochen`; `exit {code}` → "Exit-Code".
  `run-summary` keeps en's unpluralized label style ("{n} Warnung").
- **diagnostics.ftl** (57 keys): config-field names and keywords stay
  literal (`profile_version`, `codec_kind`, `codec_id`, `exact`,
  `select/drop/add`, `any/not`, `raw:`). `capability model` →
  Fähigkeitsmodell; `matched untyped` → untypisiert abgeglichen;
  `muxing source` → Multiplex-Quelle. Straight ASCII `"` for quoting (mirrors
  en; not mkvtoolnix's »«) — keeps both catalogs structurally parallel and
  ASCII-clean.
- **gui-batch.ftl** (39 keys): Batch → Stapel, Dry run → Probelauf,
  Diagnostics → Diagnosen, Resolved track → Aufgelöste Spur. `severity` (as
  a concept, run-tooltip) → Schweregrad.
- **gui-common.ftl** (33 keys): first-run guidance keeps `executable` →
  Programmdatei vs `binary` → Binärdatei (mirrors en's wording split).
  IpcError-code keys mirror en 1:1.
- **gui-jobs.ftl** (42 keys): run → Lauf, history → Verlauf, log →
  Protokoll, State → Status. `## section` comments kept (terse) for
  scannability; they never register as ids (start with `#`).
- **gui-settings.ftl** (13 keys): see concern (3) on `settings-locale-hint`.

De catalogs carry only a short header comment + kept `##` dividers; the
verbose per-component implementation notes stay in the en source-of-truth
catalogs (not duplicated into translations).

---

## 3. Plural-rule handling (CLDR German)

German has the same two integer categories as English — `one` (n = 1),
`*[other]` — so every en `[one]`/`*[other]` selector maps 1:1; counts here
are integers, so `[one]` = exactly 1. What changes is the **word forms**,
handled inside each branch:

- `Fehler` is invariant (1 Fehler / n Fehler) — selector still required
  structurally (parity + the "1" vs number).
- `Warnung` → plural `Warnungen`; `Vorschlag` → `Vorschläge` (wurde →
  wurden, weiterer → weitere); `Behebungsgruppe` → `Behebungsgruppen`;
  `Hinweis` → `Hinweise`; `Datei` → `Dateien`.

Selectors touched (all mirror en structure and indentation exactly):
`cli.ftl` validate-summary, dry-run-summary, run-job-warning;
`diagnostics.ftl` invalid-template, unsupported-source, suggestions-capped,
**suggestion-partition** (nested `overflow`/`group` × `one`/`other`);
`gui-batch.ftl` batch-diagnostics-summary; `gui-jobs.ftl`
jobs-row-warning-count.

FluentBundle is constructed with the normalized primary subtag (`"de"`),
which drives `Intl.PluralRules("de")`; the frontend already promotes count
args to numbers (T19 `diagnosticFluentParams.ts`), so the de selectors need
nothing extra.

---

## 4. Loader change (S15 primary-subtag normalization)

`src/i18n/index.ts`. The latent gap the Plan-5 task-9 review recorded:
`catalogsForLocale` matched the locale against the `locales/<tag>/`
directory by exact string equality, so a system reporting `de-DE`/`de-AT`
(what `navigator.language` and a saved setting commonly give) would skip
the new `locales/de/` directory and fall through to English.

Fix: a `primarySubtag(locale)` helper (`locale.split("-")[0].toLowerCase()`),
applied in `buildBundles` when building the requested chain, so `de-DE` →
`de`, `en-US` → `en` (which then dedups against the unconditional `en`
fallback into a single bundle). German regional variants share one CLDR
plural-rule set, so collapsing the region is lossless here. The stale
"v1 ships English only" docstring was updated. No other loader change: the
`import.meta.glob` catalog discovery already picks up `locales/de/`
automatically (the "adding a locale is content, not a refactor" design
holds).

---

## 5. Runtime-parse + e2e evidence (T20 recorded risk)

The T20 review flagged that a botched multiline continuation indent can
pass the line-regex parity gate but be rejected by the real Fluent parser.
Two independent proofs:

1. **Full catalog parse/format check** (throwaway node script, real
   `@fluent/bundle`): **all 202 messages across 6 de catalogs parse with
   zero `FluentResource` errors, format with no leaked placeables, and
   en↔de key parity holds per file.** The nested `suggestion-partition`
   selector resolved to the correct deep branch — the strongest indentation
   proof.
2. **GUI e2e** (`e2e/smoke.spec.ts`, `german locale` describe): locale
   pinned via mocked `get_settings.locale = "de"` (the channel `main.ts`
   reads), asserts three **de-only** strings render — heading "Stapel"
   (en "Batch"), button "Probelauf" (en "Dry run"), and the
   placeable-bearing `Ausgewähltes Profil: {path}` via `visibleText`
   (BiDi-mark stripping, mirroring the T19 pattern). Literal assertions by
   design: the point is proving de loaded rather than en-fallback, so
   deriving through the catalog under test would prove nothing.
   **5/5 e2e pass.**

---

## 6. Gate results (nine parts, all foreground, all green)

| # | Part | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | ok |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | ok |
| 3 | `cargo test --workspace` | ok |
| 4 | `cargo deny check` | advisories/bans/licenses/sources ok |
| 5 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | ok |
| 6 | `pnpm lint` | ok |
| 7 | `pnpm build` | ok (de catalogs bundled) |
| 8 | `pnpm check:i18n` | ok — **1 locale checked for parity against 6 en catalogs** (the parity gate T20 armed is now actively passing on de) |
| 9 | `pnpm test:e2e` | 5/5 pass |

`check:i18n` decision (T20 header note): cli.ftl **participates in parity**
(de/cli.ftl is a real shipped catalog now) — nothing to change, the script
already includes it in check 3.

`pnpm install --frozen-lockfile` run once (fresh worktree). Rust gate parts
are formalities here (no Rust touched: the CLI/shell `include_str!` only the
en catalogs), run for completeness.

---

## 7. Self-review

- Structure mirrors en byte-for-byte for every placeable, selector key, and
  continuation indent → parity + runtime parse both hold (proven, §5).
- Real German orthography throughout (ä/ö/ü/ß, capitalized nouns); no
  AI-tell glyphs (straight quotes, ASCII `->`, no em-dash/ellipsis).
- `{ $var }` names never translated; keywords/field-names never translated.
- Terminology consistent across catalogs (Spur, Datei, Profil, Regel,
  Vorschlag, Lauf, Job, Protokoll, Kennung).

## 8. Concerns / open dimensions (raised, not silently decided)

1. **No German option in the settings dropdown.** `SettingsDialog.vue`
   still offers only `<option value="en">`; there is no
   `settings-locale-option-de` key. So a user can reach German only via a
   system locale that normalizes to `de` (or a hand-edited settings file),
   not from the UI. Adding it means a new key in **both** catalogs + a
   `<option>` + wording — that touches the en reference and a Vue component,
   **outside this brief's stated Modify scope** ("locale loader/scanner").
   **Assumption: left out of this task — say the word and I add it.**
2. **Native close-confirmation dialog stays English.** `src-tauri/src/run.rs`
   `include_str!`s `locales/en/gui-common.ftl` and line-parses it, so the de
   `close-abort-*` strings ship but are never shown. A real (small) shell-i18n
   gap; out of scope here. The de keys are translated + single-line for a
   later fix.
3. **`settings-locale-hint` is now factually stale** — it mirrors en
   verbatim ("In dieser Version ist nur Englisch enthalten…"), which the de
   build contradicts. Translated faithfully to keep de a true mirror of the
   (unchanged) en reference; **en itself should be updated** as part of
   (1)/the review, then de follows. Flagged rather than unilaterally reworded.
4. **⚑ terminology choices** in §1b (Stapel, Ausführen, Diagnosen,
   Probelauf, Behebungsgruppe, Locator, Kennung, In Warteschlange,
   Info/Hinweis split) — the substantive review decisions.

---

## 9. Review follow-up: two committed regression guards (branch `plan55-t21`, worktree `.worktrees/t21`)

The terminology gate passed; two test-gap findings from the review were fixed test-only (code and German untouched), commit `c229400`.

**Finding 1 (S15 e2e coverage).** The de e2e pinned `locale: "de"`, a
primary subtag, so it passed with or without `primarySubtag` normalization.
Fixed by flipping `DE_SETTINGS` to `DE_AT_SETTINGS` with `locale: "de-AT"`
in `e2e/smoke.spec.ts`, plus renaming the test and updating the describe
comment to state what it now proves. Verified by temporarily disabling
`primarySubtag` in `src/i18n/index.ts` and rebuilding: the test then fails
(falls back to the en "Batch" heading); restored, all 5 e2e cases pass.

**Finding 2 (nested-selector real-parse guard).** Took the "extend
`e2e/i18n-en.ts`'s `buildEnBundle` pattern to all locale dirs" option, but
with a correction found while implementing it: `@fluent/bundle`'s runtime
`FluentResource` parser (verified from
`node_modules/@fluent/bundle/esm/resource.js`) only ever reports an
`addResource` error for a duplicate message/term id; a malformed or
Junk-dropped entry (e.g. an unbalanced brace inside a nested selector) is
silently dropped with zero `addResource` errors, confirmed empirically. So
"throws on addResource errors" alone, extended to more locale dirs, would
not have closed the described gap. `assertAllCatalogsParseCleanly` in
`e2e/i18n-en.ts` additionally cross-checks every id `check-i18n.mjs`'s
column-0 scan finds in a catalog's source text against the real parser's
message output (`bundle.getMessage(id)?.value`), throwing on any that were
silently dropped. Walks every `locales/<tag>/` directory in the app's own
two runtime groupings (gui-*+diagnostics combined, `cli.ftl` standalone,
the latter to avoid a false collision on ids `cli.ftl` and `gui-common.ftl`
legitimately share). Verified by constructing an unbalanced-brace
regression in `de/diagnostics.ftl`: `addResource` reported zero errors,
but the new guard caught the dropped `suggestion-partition` message and
hard-failed the whole `test:e2e` run.

Gate results: `pnpm check:i18n` ok (same 12 pre-existing warnings, exit 0);
`pnpm test:e2e` 5/5 pass; `pnpm lint` exit 0. No Rust touched, so the Rust
gate was not re-run.
