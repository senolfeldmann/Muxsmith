#!/usr/bin/env node
// Task 12 i18n completeness gate (spec 8.4). No dependencies beyond Node
// itself. Two independent checks over the same catalog/source scan:
//
//  1. HARD FAILURE (exit 1): every LITERAL `t('id')`/`$t('id')` call found
//     in src/**/*.{vue,ts} must resolve to a real message id in the known
//     catalog (locales/en/gui-*.ftl + diagnostics.ftl -- the same set
//     src/i18n/index.ts itself globs; cli.ftl is CLI-only vocabulary the
//     frontend never renders, excluded exactly like the real loader).
//     A call whose first argument is not a plain quoted string (a
//     computed key -- `$t(stateKey)`, `$t(err.code, err.params)`, a
//     template literal `` $t(`severity-${d.severity}`) ``) is dynamic and
//     cannot be statically resolved, so it is skipped here, never
//     flagged.
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

import { readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const LOCALES_EN = join(ROOT, "locales", "en");
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
//     missing -- extend parseCatalogIds then, don't work around it.
//   - TERMS (`-brand-name = ...`): NOT registered (leading `-` fails the
//     regex). Correct as-is: terms are catalog-internal and can never be
//     a `$t()` argument.
const MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/;

function parseCatalogIds(file) {
  const ids = [];
  const text = readFileSync(join(LOCALES_EN, file), "utf8");
  for (const line of text.split("\n")) {
    const m = MESSAGE_ID_RE.exec(line);
    if (m) {
      ids.push(m[1]);
    }
  }
  return ids;
}

const catalogFiles = readdirSync(LOCALES_EN)
  .filter((f) => f.endsWith(".ftl") && (f.startsWith("gui-") || f === "diagnostics.ftl"))
  .sort();

/** id -> source catalog file. */
const knownIds = new Map();
const diagnosticsIds = new Set();
for (const file of catalogFiles) {
  for (const id of parseCatalogIds(file)) {
    knownIds.set(id, file);
    if (file === "diagnostics.ftl") {
      diagnosticsIds.add(id);
    }
  }
}

function walkSourceFiles(dir, out) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, entry.name);
    if (entry.isDirectory()) {
      walkSourceFiles(full, out);
    } else if (/\.(vue|ts)$/.test(entry.name)) {
      out.push(full);
    }
  }
}

const sourceFiles = [];
walkSourceFiles(SRC, sourceFiles);

// Requires a non-identifier character (or start of line) immediately
// before the optional `$` -- without this, `t(` also matches the tail of
// `emit(`, `writeText(`, `useFluent(`, `attempt(`, `.mount(`, etc., which
// this codebase has plenty of (verified empirically against src/ before
// settling on this pattern).
const CALL_RE = /(?<![\w$])\$?t\(\s*(['"])([^'"]*)\1/g;

const missing = []; // { id, file, line }
const literalCallIds = new Set();
const literalAnywhereIds = new Set();
const fileTexts = new Map();

for (const file of sourceFiles) {
  const text = readFileSync(file, "utf8");
  fileTexts.set(file, text);

  const lines = text.split("\n");
  lines.forEach((line, i) => {
    CALL_RE.lastIndex = 0;
    let m;
    while ((m = CALL_RE.exec(line)) !== null) {
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

if (missing.length === 0) {
  console.log(
    `check-i18n: ok (${sourceFiles.length} source files scanned, ${knownIds.size} catalog ids, ` +
      `${unused.length} unused warning(s)).`,
  );
  process.exit(0);
}
process.exit(1);
