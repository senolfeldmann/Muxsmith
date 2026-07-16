#!/usr/bin/env node
// Task 12/20 i18n completeness + cross-locale parity gate (spec 8.4, #17
// step 2). No dependencies beyond Node itself. Three independent checks
// over the same catalog/source scan:
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
//     Known residual false positive, accepted because this half is a
//     warning, never a failure: a handful of shell IpcError codes (e.g.
//     gui-common.ftl's mkvmerge-spawn-failed/mkvmerge-query-failed/
//     settings-io-failed/settings-parse-failed/internal-task-failed,
//     gui-jobs.ftl's run-already-active/no-active-run/invalid-run-id/
//     job-log-unavailable/job-log-not-found) are reached only via a
//     generic `$t(err.code, err.params)` pattern and never spelled out
//     literally anywhere in src/, so they can surface as "unused" even
//     though they are genuinely rendered whenever that IPC error occurs.
//
//  3. HARD FAILURE (exit 1): cross-locale key parity (Task 20, #17 step
//     2). `locales/en/` is the reference locale -- src/i18n/index.ts
//     falls back to it for any message missing from another locale's
//     bundle, and it is the only locale checks 1/2 validate against -- so
//     every OTHER `locales/<tag>/` directory must carry exactly the same
//     set of `.ftl` catalog *files* as `locales/en/`, and within each
//     shared file, exactly the same message ids. No Fluent attributes
//     (`.label = ...` style) exist in any catalog today (grepped across
//     locales/en/*.ftl), so attribute-level parity is not needed yet;
//     extend this check (and MESSAGE_ID_RE's own note below) if one is
//     ever added.
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
//     With only `locales/en/` present (current tree), the comparison loop
//     below has no other locale directory to iterate and passes trivially
//     by construction, not by a special case; it activates the moment a
//     second `locales/<tag>/` directory (e.g. `locales/de/`, Task 21)
//     exists.

import { readFileSync, readdirSync } from "node:fs";
import { join, relative, resolve } from "node:path";

const ROOT = resolve(import.meta.dirname, "..");
const LOCALES_ROOT = join(ROOT, "locales");
const LOCALES_EN = join(LOCALES_ROOT, "en");
const SRC = join(ROOT, "src");

// D31: src-tauri/src/run.rs's `ftl_message()` include_str!s gui-common.ftl
// directly for the native close-confirmation dialog; the frontend never
// calls $t() for these four.
const RUST_ONLY_IDS = new Set([
  "close-abort-title",
  "close-abort-message",
  "close-abort-confirm",
  "close-abort-dismiss",
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
//   - ATTRIBUTES (`.label = ...` lines): NOT registered as ids. If a
//     catalog ever adds attributes the frontend addresses (fluent-vue's
//     `$t("msg.attr")` form), this scanner will flag those references as
//     missing -- extend parseCatalogIds then, don't work around it. The
//     same extension would need to reach check 3's parity comparison
//     (currently id-set-only, since no catalog has attributes today).
//   - TERMS (`-brand-name = ...`): NOT registered (leading `-` fails the
//     regex). Correct as-is: terms are catalog-internal and can never be
//     a `$t()` argument.
const MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/;

/** Message ids found in one catalog file, given its full path. */
function parseCatalogIds(path) {
  const ids = [];
  const text = readFileSync(path, "utf8");
  for (const line of text.split("\n")) {
    const m = MESSAGE_ID_RE.exec(line);
    if (m) {
      ids.push(m[1]);
    }
  }
  return ids;
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
    for (const m of line.matchAll(LABEL_KEY_RE)) {
      const id = m[2];
      literalCallIds.add(id);
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

const usedIds = new Set([
  ...literalCallIds,
  ...literalAnywhereIds,
  ...diagnosticsIds,
  ...RUST_ONLY_IDS,
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
    const localeIds = new Set(parseCatalogIds(join(dir, file)));
    const missingIds = [...refIds.difference(localeIds)].sort();
    const extraIds = [...localeIds.difference(refIds)].sort();
    for (const id of missingIds) {
      parityErrors.push(`locales/${locale}/${file}: missing id "${id}" (present in locales/en/${file})`);
    }
    for (const id of extraIds) {
      parityErrors.push(`locales/${locale}/${file}: extra id "${id}" (not present in locales/en/${file})`);
    }
  }
}

if (parityErrors.length > 0) {
  console.error("check-i18n: cross-locale key parity violations:");
  for (const line of parityErrors) {
    console.error(`  ${line}`);
  }
}

if (missing.length === 0 && parityErrors.length === 0) {
  console.log(
    `check-i18n: ok (${sourceFiles.length} source files scanned, ${knownIds.size} catalog ids, ` +
      `${unused.length} unused warning(s), ${otherLocales.length} other locale(s) checked for parity ` +
      `against ${referenceCatalogFiles.length} en/ catalog(s)).`,
  );
  process.exit(0);
}
process.exit(1);
