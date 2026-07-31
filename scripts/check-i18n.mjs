#!/usr/bin/env node
// Task 12/20 i18n completeness + cross-locale parity gate (spec 8.4, #17
// step 2). No dependencies beyond Node itself. Five independent hard-
// failure checks (exit 1), plus one warning-only pass (check 2). Checks
// 1-3 below run over the catalog/source scan; the other three hard-failure
// checks are D55 rule 3's editor-tooltip completeness, D61's IpcError
// presence gate over src-tauri/src, and D62's help-topic-tree gate over
// help/ (referenced<->file both directions, locale lockstep, and topic
// content hygiene) -- the one place where "i18n-complete" is defined now
// spans the help/ topic tree as well as the catalogs; each documented at
// its own code block:
//
//  1. HARD FAILURE (exit 1): every LITERAL `t('id')`/`$t('id')` call found
//     in src/**/*.{vue,ts} must resolve to a real message id in the known
//     catalog (locales/en/gui-*.ftl + diagnostics.ftl -- the same set
//     src/i18n/index.ts itself globs; cli.ftl is CLI-only vocabulary the
//     frontend never renders, excluded exactly like the real loader. This
//     exclusion is scoped to checks 1 and 2 only -- see check 3 below for
//     why cli.ftl is NOT excluded from cross-locale parity.)
//     A call whose first argument is not a plain quoted string (a
//     computed key -- `$t(stateKey)`, `$t(err.code, err.params)`, a
//     template literal `` $t(`severity-${d.severity}`) ``) is dynamic and
//     cannot be statically resolved, so it is skipped here, never
//     flagged.
//     D45: the same hard-failure scan also covers every literal
//     `labelKey: "..."` in src/**/*.{vue,ts} (a registry's `FieldSpec`,
//     e.g. src/editor/registries.ts) -- the editor components read the
//     key off the spec and pass it to $t() at render time, so this is
//     check 1's own coverage extended to that literal shape, not a
//     second mechanism.
//     D55 rule 2: the same scan covers literal fluent-vue attribute
//     accessors `$ta('id')` / `ta('id')` (TA_CALL_RE) -- the id must
//     resolve, same hard fail. Attribute *member* access after the call
//     (`$ta('id').tooltip`, `$ta('id')[name]`) is dynamic and skipped
//     exactly like a computed $t key; its coverage is checks 3-5.
//
//  2. WARNING ONLY (always exit 0 on this half): gui-* catalog ids never
//     referenced anywhere in src/. Many ids are reached only dynamically
//     rather than as a literal $t() argument -- a diagnostic code via
//     `$t(d.code, d.params)`, a shell IpcError code via
//     `$t(err.code, err.params)`, a state-chip key returned from a
//     mapping function (`jobRowState.ts`'s `jobStateKey`) -- so "used"
//     here also counts any catalog id that appears anywhere in src/ as a
//     quoted string literal (covers the mapping-function case, e.g.
//     `case "ok": return "jobs-state-ok";`), plus two explicit
//     allowlists for ids that never appear as a literal ANYWHERE in the
//     frontend because they are produced entirely on the Rust side:
//       - every diagnostics.ftl id (muxsmith_core::report::DiagCode::key(),
//         reached only via a runtime Diagnostic.code, never spelled out
//         in src/ at all)
//       - gui-common.ftl's four close-abort-* keys (D31: consumed by
//         src-tauri's own `include_str!` lookup in run.rs, never by the
//         frontend)
//     Shell IpcError codes (gui-common.ftl's mkvmerge-spawn-failed etc.,
//     gui-jobs.ftl's run-already-active etc.) are reached via the generic
//     `$t(err.code, err.params)` pattern; most never appear as literals
//     (FirstRun's two detect codes, mkvmerge-not-found/mkvmerge-too-old, are
//     the exception -- spelled out as switch cases in FirstRun.vue). D61's
//     presence gate now extracts every
//     `IpcError::new("code")` from src-tauri/src and both hard-gates it
//     (a code with no en message fails) and adds it to this check's
//     usedIds union, so those codes are counted as used here instead of
//     surfacing as a residual false-positive "unused" warning.
//
//  3. HARD FAILURE (exit 1): cross-locale key parity (Task 20, #17 step
//     2). `locales/en/` is the reference locale -- src/i18n/index.ts
//     falls back to it for any message missing from another locale's
//     bundle, and it is the only locale checks 1/2 validate against -- so
//     every OTHER `locales/<tag>/` directory must carry exactly the same
//     set of `.ftl` catalog *files* as `locales/en/`, and within each
//     shared file, exactly the same message ids. D55 extends this parity
//     to Fluent attributes and pattern structure (rules 4-5 below): for
//     every shared id, the attribute-NAME set must equal en's (rule 4),
//     and for every message value and every attribute value the placeable
//     set and the flat-derived select structure must match en's (rule 5,
//     an en-reference parity check -- see comparePatterns; absolute Fluent
//     validity is delegated to e2e assertAllCatalogsParseCleanly).
//
//     UNLIKE checks 1/2, this check covers ALL `.ftl` files under
//     `locales/en/`, INCLUDING `cli.ftl`. Decision (Task 20, does not
//     revisit checks 1/2's own exclusion): Task 10's Rust-side
//     `catalog_completeness.rs` guards only the EN catalog's internal
//     wiring -- every cli.ftl key resolves to a DiagCode or is
//     allowlist-fixtured, and renders without a leaked `{$param}` -- it
//     says nothing about a SECOND locale's cli.ftl tracking en's key set,
//     because it only ever parses `locales/en/cli.ftl`. Task 21 creates
//     `locales/de/cli.ftl` as one of its six translated catalogs, so once
//     it lands, cli.ftl is a real, shipped, translatable catalog like any
//     gui-*.ftl -- excluding it from parity would leave the one catalog
//     most likely to visibly regress (CLI-facing text) with no structural
//     guard against a de/cli.ftl silently drifting out of sync (missing
//     keys falling back to raw ids, stale keys nobody notices). Keeping
//     it excluded here would protect against nothing that isn't already
//     covered elsewhere for the EN side, while leaving a real gap on the
//     DE side. So cli.ftl participates in check 3 while staying excluded
//     from checks 1/2, whose exclusion reason ("the frontend never calls
//     $t() for it") is unrelated and still holds.
//
//     The comparison loop below iterates every `locales/<tag>/` other than
//     en. With `locales/de/` now present (this run reports "1 other
//     locale(s) checked"), de is validated against en's key, attribute and
//     pattern sets. Before any second locale landed the loop had nothing to
//     iterate and passed trivially by construction, not by a special case;
//     it activated the moment `locales/de/` (Task 21) landed.

import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const ROOT = resolve(import.meta.dirname, "..");
const LOCALES_ROOT = join(ROOT, "locales");
const LOCALES_EN = join(LOCALES_ROOT, "en");
const SRC = join(ROOT, "src");

// D31/D109/D110: src-tauri/src/run.rs's `ftl_message()` include_str!s
// gui-common.ftl directly for the shell's native close-confirmation
// dialogs; the frontend never calls $t() for these ten -- shell-consumed,
// not just the original D31 four.
const RUST_ONLY_IDS = new Set([
  "close-abort-title",
  "close-abort-message",
  "close-abort-confirm",
  "close-abort-dismiss",
  "close-discard-title",
  "close-discard-message",
  "close-discard-confirm",
  "close-abort-discard-title",
  "close-abort-discard-message",
  "close-abort-discard-confirm",
]);

// PARSING CONSTRAINT (deliberate, line-based -- not a Fluent parser,
// mirroring src-tauri/src/run.rs::ftl_message's identically documented
// line-lookup constraint): a message id is recognized only as a
// column-0 `id =` line. What that means for Fluent syntax this catalog
// tree uses today or may grow:
//   - MULTILINE VALUES (indented continuation lines, incl. selectors like
//     diagnostics.ftl's `invalid-template` / gui-jobs.ftl's
//     `jobs-row-warning-count`): fine -- the id sits on the first line and
//     continuation lines are indented, so they can never register a bogus
//     id, and Fluent syntax inside the value is never inspected.
//   - ATTRIBUTES (indented `.name = ...` lines, e.g. `.tooltip`/`.hint`):
//     NOT registered as ids (they are not column-0), but D55 registers
//     them PER ID: `parseCatalog` records each id's attribute-name set and
//     each attribute's pattern body alongside the id's own value. They are
//     addressed by rules 2-5 -- literal `$ta('id')` resolution (rule 2),
//     editor tooltip completeness (rule 3), and cross-locale attribute-name
//     and pattern-structure parity (rules 4-5). Attribute *member* access
//     (`$ta('id').tooltip`) stays dynamic and is not statically resolved.
//   - TERMS (`-brand-name = ...`): NOT registered (leading `-` fails the
//     regex). Correct as-is: terms are catalog-internal and can never be
//     a `$t()` argument.
const MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/;

/** One catalog file, line-parsed (charter above): message ids, each id's
 *  attribute-name -> pattern body, and the id's own value body. */
function parseCatalog(path) {
  const messages = new Map(); // id -> { value: string, attrs: Map<name, body> }
  const text = readFileSync(path, "utf8");
  let current = null;
  let target = null; // { kind: "value" } | { kind: "attr", name }
  for (const line of text.split("\n")) {
    const idMatch = MESSAGE_ID_RE.exec(line);
    if (idMatch) {
      current = { value: line.slice(line.indexOf("=") + 1), attrs: new Map() };
      messages.set(idMatch[1], current);
      target = { kind: "value" };
      continue;
    }
    if (current === null) continue;
    const attrMatch = /^\s+\.([a-z][a-z0-9-]*)\s*=/.exec(line);
    if (attrMatch) {
      current.attrs.set(attrMatch[1], line.slice(line.indexOf("=") + 1));
      target = { kind: "attr", name: attrMatch[1] };
      continue;
    }
    if (/^\s+\S/.test(line)) {
      // continuation line of the current value or attribute
      if (target.kind === "value") current.value += "\n" + line;
      else current.attrs.set(target.name, current.attrs.get(target.name) + "\n" + line);
    }
  }
  return messages;
}

/** Message ids found in one catalog file, given its full path. */
function parseCatalogIds(path) {
  return [...parseCatalog(path).keys()];
}

/** All `.ftl` file names directly inside a locale directory, sorted. */
function listCatalogFiles(dir) {
  return readdirSync(dir)
    .filter((f) => f.endsWith(".ftl"))
    .sort();
}

const catalogFiles = listCatalogFiles(LOCALES_EN).filter(
  (f) => f.startsWith("gui-") || f === "diagnostics.ftl",
);

/** id -> source catalog file (checks 1/2 scope: gui-* + diagnostics.ftl only). */
const knownIds = new Map();
const diagnosticsIds = new Set();
for (const file of catalogFiles) {
  for (const id of parseCatalogIds(join(LOCALES_EN, file))) {
    knownIds.set(id, file);
    if (file === "diagnostics.ftl") {
      diagnosticsIds.add(id);
    }
  }
}

const sourceFiles = readdirSync(SRC, { recursive: true })
  .filter((f) => /\.(vue|ts)$/.test(f))
  .map((f) => join(SRC, f));

// Requires a non-identifier character (or start of line) immediately
// before the optional `$` -- without this, `t(` also matches the tail of
// `emit(`, `writeText(`, `useFluent(`, `attempt(`, `.mount(`, etc., which
// this codebase has plenty of (verified empirically against src/ before
// settling on this pattern).
const CALL_RE = /(?<![\w$])\$?t\(\s*(['"])([^'"]*)\1/g;

// D55 rule 2: fluent-vue's attribute accessor `$ta("id")` / `ta("id")` --
// same CALL_RE mechanics and same hard-fail-on-unknown-id treatment as
// $t(). The id must exist; attribute *member* access after the call
// (`$ta("id").tooltip`, `$ta("id")[name]`) is not statically resolved,
// exactly like a dynamic $t key (skipped, never flagged) -- attribute
// coverage comes from checks 3-5 instead.
const TA_CALL_RE = /(?<![\w$])\$?ta\(\s*(['"])([^'"]*)\1/g;

// D45: a registry's `FieldSpec.labelKey` (src/editor/registries.ts) is a
// message id exactly like a literal $t() call, just never passed through
// $t() itself -- the editor components (Tasks 10-13) read it off the spec
// and hand it to $t() at render time, so this scan is check 1's own
// coverage extended to that one additional literal shape, same
// line-based approach as CALL_RE (not a Fluent parser, not a TS parser).
const LABEL_KEY_RE = /labelKey:\s*(['"])([^'"]*)\1/g;

const missing = []; // { id, file, line }
const literalCallIds = new Set();
const literalAnywhereIds = new Set();
const labelKeyIds = new Set(); // D55 rule 3: ids reached via LABEL_KEY_RE
const fileTexts = new Map();

for (const file of sourceFiles) {
  const text = readFileSync(file, "utf8");
  fileTexts.set(file, text);

  const lines = text.split("\n");
  lines.forEach((line, i) => {
    for (const m of line.matchAll(CALL_RE)) {
      const id = m[2];
      literalCallIds.add(id);
      if (!knownIds.has(id)) {
        missing.push({ id, file: relative(ROOT, file), line: i + 1 });
      }
    }
    for (const m of line.matchAll(TA_CALL_RE)) {
      const id = m[2];
      literalCallIds.add(id);
      if (!knownIds.has(id)) {
        missing.push({ id, file: relative(ROOT, file), line: i + 1 });
      }
    }
    for (const m of line.matchAll(LABEL_KEY_RE)) {
      const id = m[2];
      literalCallIds.add(id);
      labelKeyIds.add(id);
      if (!knownIds.has(id)) {
        missing.push({ id, file: relative(ROOT, file), line: i + 1 });
      }
    }
  });
}

for (const id of knownIds.keys()) {
  for (const text of fileTexts.values()) {
    if (text.includes(`"${id}"`) || text.includes(`'${id}'`)) {
      literalAnywhereIds.add(id);
      break;
    }
  }
}

if (missing.length > 0) {
  console.error("check-i18n: literal t()/$t() ids missing from the en catalog:");
  for (const m of missing) {
    console.error(`  ${m.file}:${m.line}  "${m.id}"`);
  }
}

// --- D61: every IpcError::new("code") in src-tauri has a GUI message ----
// Line-based Rust scan, taking each file's content up to its first
// `#[cfg(test)]` line (test modules sit at file bottoms in this tree).
const SRC_TAURI = join(ROOT, "src-tauri", "src");
const IPC_ERROR_RE = /IpcError::new\(\s*"([A-Za-z][A-Za-z0-9_-]*)"/g;
const ipcErrorCodes = new Map(); // code -> "file:line"
for (const f of readdirSync(SRC_TAURI, { recursive: true }).filter((f) => f.endsWith(".rs"))) {
  const full = join(SRC_TAURI, f);
  const text = readFileSync(full, "utf8");
  const cut = text.indexOf("#[cfg(test)]");
  const scanned = cut === -1 ? text : text.slice(0, cut);
  scanned.split("\n").forEach((line, i) => {
    for (const m of line.matchAll(IPC_ERROR_RE)) {
      if (!ipcErrorCodes.has(m[1])) {
        ipcErrorCodes.set(m[1], `${relative(ROOT, full)}:${i + 1}`);
      }
    }
  });
}
const ipcErrors = [];
for (const [code, site] of [...ipcErrorCodes].sort()) {
  if (!knownIds.has(code)) {
    ipcErrors.push(`IpcError code "${code}" (${site}) has no message in the en GUI catalogs`);
  }
}

if (ipcErrors.length > 0) {
  console.error("check-i18n: src-tauri IpcError codes with no en GUI catalog message (D61):");
  for (const line of ipcErrors) {
    console.error(`  ${line}`);
  }
}

const usedIds = new Set([
  ...literalCallIds,
  ...literalAnywhereIds,
  ...diagnosticsIds,
  ...RUST_ONLY_IDS,
  ...ipcErrorCodes.keys(),
]);

const unused = [...knownIds.entries()]
  .filter(([id, file]) => file.startsWith("gui-") && !usedIds.has(id))
  .map(([id, file]) => `${id}  (${file})`)
  .sort();

if (unused.length > 0) {
  console.warn("check-i18n: gui-* catalog keys with no detected reference in src/ (warning only):");
  for (const line of unused) {
    console.warn(`  ${line}`);
  }
}

// --- Check 3: cross-locale key parity (Task 20, #17 step 2) --------------
// See the header comment for the full cli.ftl inclusion decision. Scope
// here is deliberately ALL `.ftl` files in locales/en/, not `catalogFiles`
// (which is checks 1/2's gui-*/diagnostics-only subset).

const referenceCatalogFiles = listCatalogFiles(LOCALES_EN);
const referenceIdsByFile = new Map(
  referenceCatalogFiles.map((file) => [
    file,
    new Set(parseCatalogIds(join(LOCALES_EN, file))),
  ]),
);

const otherLocales = readdirSync(LOCALES_ROOT, { withFileTypes: true })
  .filter((e) => e.isDirectory() && e.name !== "en")
  .map((e) => e.name)
  .sort();

const parityErrors = [];

// --- D55 rule 3: every registry label carries a .tooltip in en ---------
const enCatalogs = new Map(
  referenceCatalogFiles.map((f) => [f, parseCatalog(join(LOCALES_EN, f))]),
);
const tooltipErrors = [];
for (const id of [...labelKeyIds].sort()) {
  const hasTooltip = [...enCatalogs.values()].some(
    (msgs) => msgs.get(id)?.attrs.has("tooltip"),
  );
  if (!hasTooltip) {
    tooltipErrors.push(`labelKey "${id}" has no .tooltip attribute in the en catalog`);
  }
}

// --- D55 rules 4+5: attribute-name and pattern-structure parity --------
const PLURAL_KEYS = new Set(["zero", "one", "two", "few", "many", "other"]);
const PLACEABLE_RE = /\$([A-Za-z][A-Za-z0-9_-]*)/g;
const SELECTOR_RE = /\{\s*\$([A-Za-z][A-Za-z0-9_-]*)\s*->/g;
const VARIANT_RE = /^\s*(\*)?\[([^\]]+)\]/;

function patternStructure(body) {
  const placeables = new Set([...body.matchAll(PLACEABLE_RE)].map((m) => m[1]));
  // Flat, line-order derivation (line-based charter, NOT a Fluent parser):
  // variants attach to the most recent selector seen. This does NOT model
  // Fluent faithfully -- nested selects (diagnostics.ftl `suggestion-
  // partition`) mis-attribute the outer variants, and sibling selects whose
  // reopener `}, { $x ->` sits at column 0 (cli.ftl `validate-summary`,
  // gui-batch.ftl `batch-diagnostics-summary`) get that selector dropped by
  // parseCatalog's continuation guard. That is fine here because comparePatterns
  // uses this derivation only for en-vs-de PARITY: it is deterministic and
  // applied identically to both locales, so a real drift still surfaces as a
  // difference. Absolute Fluent validity is the e2e parse guard's job (D55
  // rule 5, amended round 7).
  const selects = [];
  for (const line of body.split("\n")) {
    for (const m of line.matchAll(SELECTOR_RE)) {
      selects.push({ selector: m[1], keys: [], defaults: 0 });
    }
    const v = VARIANT_RE.exec(line);
    if (v && selects.length > 0) {
      const current = selects[selects.length - 1];
      current.keys.push(v[2].trim());
      if (v[1] === "*") current.defaults += 1;
    }
  }
  return { placeables, selects };
}

function comparePatterns(where, enBody, locBody, errors) {
  const a = patternStructure(enBody);
  const b = patternStructure(locBody);
  if ([...a.placeables].sort().join() !== [...b.placeables].sort().join()) {
    errors.push(`${where}: placeable set differs from en ({${[...a.placeables]}} vs {${[...b.placeables]}})`);
  }
  if (a.selects.length !== b.selects.length) {
    errors.push(`${where}: select-expression count differs from en (${a.selects.length} vs ${b.selects.length})`);
    return;
  }
  a.selects.forEach((sa, i) => {
    const sb = b.selects[i];
    if (sa.selector !== sb.selector) {
      errors.push(`${where}: select ${i} selector differs ($${sa.selector} vs $${sb.selector})`);
    }
    const plural = (k) => PLURAL_KEYS.has(k) || /^\d+$/.test(k);
    if (sa.keys.every(plural) && sb.keys.every(plural)) {
      // CLDR carve-out, en-reference PARITY (D55 rule 5(d), amended round
      // 7): plural-category sets legitimately differ per locale, so this
      // does not assert an absolute shape -- it compares de against en's
      // flat derivation: variant presence (empty vs non-empty) and
      // *-default count must match en's. Absolute per-select validity
      // (every select well-formed, exactly one *-default) is delegated to
      // e2e assertAllCatalogsParseCleanly, which real-Fluent-parses every
      // locale; a select missing its default is a parse error caught
      // there, not here. The flat model collapses sibling/nested selects
      // identically for both locales, so this parity stays sound within
      // the line-based charter.
      if (
        (sa.keys.length === 0) !== (sb.keys.length === 0) ||
        sa.defaults !== sb.defaults
      ) {
        errors.push(`${where}: select ${i} plural variant presence / *-default count differs from en (${sa.keys.length}/${sa.defaults} vs ${sb.keys.length}/${sb.defaults})`);
      }
    } else if (sa.keys.slice().sort().join() !== sb.keys.slice().sort().join()) {
      errors.push(`${where}: select ${i} variant keys differ from en ([${sa.keys}] vs [${sb.keys}])`);
    }
  });
}

for (const locale of otherLocales) {
  const dir = join(LOCALES_ROOT, locale);
  const localeFiles = new Set(listCatalogFiles(dir));

  for (const file of referenceCatalogFiles) {
    if (!localeFiles.has(file)) {
      parityErrors.push(`locales/${locale}/${file}: missing (present in locales/en/)`);
    }
  }
  for (const file of localeFiles) {
    if (!referenceIdsByFile.has(file)) {
      parityErrors.push(`locales/${locale}/${file}: unexpected catalog file (no locales/en/${file})`);
    }
  }

  for (const file of referenceCatalogFiles) {
    if (!localeFiles.has(file)) {
      continue; // already reported as a missing catalog file above
    }
    const refIds = referenceIdsByFile.get(file);
    const localeMsgs = parseCatalog(join(dir, file));
    const localeIds = new Set(localeMsgs.keys());
    const missingIds = [...refIds.difference(localeIds)].sort();
    const extraIds = [...localeIds.difference(refIds)].sort();
    for (const id of missingIds) {
      parityErrors.push(`locales/${locale}/${file}: missing id "${id}" (present in locales/en/${file})`);
    }
    for (const id of extraIds) {
      parityErrors.push(`locales/${locale}/${file}: extra id "${id}" (not present in locales/en/${file})`);
    }

    // D55 rules 4+5: for every id shared between en and this locale,
    // attribute-name-set equality (rule 4) and placeable/selector-structure
    // parity on the value and each shared attribute (rule 5).
    const enMsgs = enCatalogs.get(file);
    for (const id of [...refIds].filter((x) => localeIds.has(x)).sort()) {
      const enMsg = enMsgs.get(id);
      const locMsg = localeMsgs.get(id);
      const enAttrs = [...enMsg.attrs.keys()].sort();
      const locAttrs = [...locMsg.attrs.keys()].sort();
      if (enAttrs.join() !== locAttrs.join()) {
        parityErrors.push(
          `locales/${locale}/${file}: id "${id}" attribute set differs from en ({${enAttrs}} vs {${locAttrs}})`,
        );
      }
      comparePatterns(
        `locales/${locale}/${file}: "${id}" value`,
        enMsg.value,
        locMsg.value,
        parityErrors,
      );
      for (const attr of enAttrs) {
        if (locMsg.attrs.has(attr)) {
          comparePatterns(
            `locales/${locale}/${file}: "${id}".${attr}`,
            enMsg.attrs.get(attr),
            locMsg.attrs.get(attr),
            parityErrors,
          );
        }
      }
    }
  }
}

if (tooltipErrors.length > 0) {
  console.error("check-i18n: editor labelKeys without a .tooltip attribute in the en catalog:");
  for (const line of tooltipErrors) {
    console.error(`  ${line}`);
  }
}

if (parityErrors.length > 0) {
  console.error("check-i18n: cross-locale key parity violations:");
  for (const line of parityErrors) {
    console.error(`  ${line}`);
  }
}

// --- D62: help-topic completeness + content hygiene, both directions, ----
// per locale. The i18n gate is the one place "i18n-complete" is defined
// (D62's rejected `check-help.mjs` alternative); with D62 that definition
// now spans the help/ topic tree too. Six hard-fail conditions feed
// helpErrors:
//   1. referenced -> file, per locale     4. external-URL ban
//   2. file -> referenced (orphans)       5. table/pipe ban (ZERO-PIPE)
//   3. help/ vs locales/ locale lockstep  6. raw-HTML ban (code-span exempt)
// Checks 5-6 are the run's cross-task content-hygiene constraints, siblings
// of the external-URL ban; the brief (task-19) sketches only 1-4 (design
// section D62 calls those four "exhaustive"), so 5-6 are surfaced in the
// task-19 report as a controller-directed extension.
const HELP_ROOT = join(ROOT, "help");
// Referenced-id extraction. The captured value is constrained to the
// help-id grammar [a-z][a-z0-9-]* (every real id: view-*, editor-*,
// batch-suggestion-card) so the scan cannot pick up a dynamic Vue bind
// (`:data-help-id="spec.helpId"`, FieldWidgetDispatcher.vue) or a
// querySelector template (`[data-help-id="${id}"]`, App.vue) as a bogus
// referenced id -- both otherwise capture a non-id and fail check 1
// forever. VIEW_TOPIC_RE already carries the same grammar posture. (The
// brief's `[^'"]*` / `[^"]+` captures predate those two src sites; the
// grammar constraint is the task-19 minimal adaptation, surfaced.)
const HELP_ID_PROP_RE = /helpId:\s*(['"])([a-z][a-z0-9-]*)\1/g; // (a) registry literals
const DATA_HELP_ID_RE = /data-help-id="([a-z][a-z0-9-]*)"/g; //    (b) template literals
const VIEW_TOPIC_RE = /['"](view-[a-z-]+)['"]/g; //               (c) VIEW_TOPICS values

const referencedHelpIds = new Map(); // id -> "file:line" (first reference)
for (const [file, text] of fileTexts) {
  text.split("\n").forEach((line, i) => {
    for (const re of [HELP_ID_PROP_RE, DATA_HELP_ID_RE]) {
      for (const m of line.matchAll(re)) {
        const id = m[2] ?? m[1];
        if (!referencedHelpIds.has(id)) {
          referencedHelpIds.set(id, `${relative(ROOT, file)}:${i + 1}`);
        }
      }
    }
  });
}
// (c) is deliberately redundant with (b) for the three view ids: a view
// root losing its data-help-id, or the map growing an id without a topic,
// both still fail. Shape (a) cannot see them (no `helpId:` property name).
const stateText = readFileSync(join(SRC, "help", "state.ts"), "utf8");
for (const m of stateText.matchAll(VIEW_TOPIC_RE)) {
  if (!referencedHelpIds.has(m[1])) {
    referencedHelpIds.set(m[1], "src/help/state.ts (VIEW_TOPICS)");
  }
}

const helpErrors = [];
const helpLocales = readdirSync(HELP_ROOT, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)
  .sort();
const catalogLocales = readdirSync(LOCALES_ROOT, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)
  .sort();

// 1. referenced -> file, per locale
for (const [id, site] of [...referencedHelpIds].sort()) {
  for (const locale of helpLocales) {
    try {
      readFileSync(join(HELP_ROOT, locale, `${id}.md`));
    } catch {
      helpErrors.push(`help id "${id}" (referenced at ${site}) has no help/${locale}/${id}.md`);
    }
  }
}
// 2. file -> referenced (orphans)
for (const locale of helpLocales) {
  for (const f of readdirSync(join(HELP_ROOT, locale)).filter((f) => f.endsWith(".md"))) {
    const id = f.slice(0, -3);
    if (!referencedHelpIds.has(id)) {
      helpErrors.push(`help/${locale}/${f}: orphan topic (no helpId/data-help-id/VIEW_TOPICS reference)`);
    }
  }
}
// 3. locale-set lockstep with locales/
if (helpLocales.join() !== catalogLocales.join()) {
  helpErrors.push(`help/ locales [${helpLocales}] != locales/ [${catalogLocales}] (lockstep, D62)`);
}
// 4-6. per-topic content hygiene, one read per file:
//   4. external-URL ban -- help is self-contained by design (D50 trust
//      model, offline posture, CSP: the webview must not navigate out);
//      cross-topic references are prose ("see the Match topic"), not links.
//   5. table/pipe ban (ZERO-PIPE) -- topics are prose, never tabular. A
//      bare `|` is flagged, not a `|...|` pair, so a headerless / outer-
//      pipe-less GFM table (`a | b`, one pipe) cannot slip through.
//   6. raw-HTML ban -- markdown prose only, no injected elements. Inline
//      code spans (`...`) are stripped before the tag scan, because the
//      pattern topics carry angle brackets legitimately inside code
//      (`(?<season>\d{2})`); without the exemption both locales' copies of
//      editor-input-pattern.md would go red on valid content.
const RAW_HTML_RE = /<\/?[a-zA-Z]/; // an opening or closing HTML-tag start
for (const locale of helpLocales) {
  for (const f of readdirSync(join(HELP_ROOT, locale)).filter((f) => f.endsWith(".md"))) {
    const text = readFileSync(join(HELP_ROOT, locale, f), "utf8");
    if (/https?:\/\//.test(text)) {
      helpErrors.push(`help/${locale}/${f}: contains an external URL (banned, D62 check 4)`);
    }
    text.split("\n").forEach((line, i) => {
      if (line.includes("|")) {
        helpErrors.push(`help/${locale}/${f}:${i + 1}: contains a table/pipe character (banned, D62 check 5)`);
      }
      if (RAW_HTML_RE.test(line.replace(/`[^`]*`/g, ""))) {
        helpErrors.push(`help/${locale}/${f}:${i + 1}: contains raw HTML (banned, D62 check 6; inline code spans exempt)`);
      }
    });
  }
}

if (helpErrors.length > 0) {
  console.error("check-i18n: help-topic gate violations (D62):");
  for (const line of helpErrors) {
    console.error(`  ${line}`);
  }
}

if (
  missing.length === 0 &&
  parityErrors.length === 0 &&
  tooltipErrors.length === 0 &&
  ipcErrors.length === 0 &&
  helpErrors.length === 0
) {
  console.log(
    `check-i18n: ok (${sourceFiles.length} source files scanned, ${knownIds.size} catalog ids, ` +
      `${ipcErrorCodes.size} IpcError code(s) gated, ` +
      `${referencedHelpIds.size} help id(s) x ${helpLocales.length} help locale(s), ` +
      `${unused.length} unused warning(s), ${otherLocales.length} other locale(s) checked for parity ` +
      `against ${referenceCatalogFiles.length} en/ catalog(s)).`,
  );
  process.exit(0);
}
process.exit(1);
