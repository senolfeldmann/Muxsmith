# Verify-50 — idiomacy finding F6 (slice F6)

**Verdict: CONFIRMED**

## Finding
`scripts/check-i18n.mjs:88` uses the pre-Node-21 idiom
`resolve(dirname(fileURLToPath(import.meta.url)), "..")`; replace with
`const ROOT = resolve(import.meta.dirname, "..")`, dropping the `node:url`
import and the `dirname` name from the `node:path` import. lines_cut: 1,
deps_cut: 0, tag: native.

## Verification (HEAD 2f17880)

### (a) Cited code matches
Line 85-88 read exactly:
```js
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
```
The finding describes line 88 verbatim. Confirmed.

### (b) Replacement is current idiom for the pinned toolchain (node 26.5.0)
- `node --version` in the repo: **v26.5.0** (matches pin).
- Empirical check in this exact runtime: `node -e "console.log(typeof import.meta.dirname)"` → `string`. The API is present and populated, not undefined.
- `import.meta.dirname` is defined as `path.dirname(import.meta.filename)`, i.e. exactly `dirname(fileURLToPath(import.meta.url))`. `resolve(import.meta.dirname, "..")` is behaviorally identical (module dir → parent = project root).
- The file is `.mjs` (ES module), so `import.meta` is valid here; the API is a stable Node feature (added 20.11 / 21.2), stable in Node 26. Not a CJS pitfall.
- The script's stated constraint "No dependencies beyond Node itself" is preserved — `import.meta.dirname` is a core language/runtime feature, no new dependency.

### Safety of the two removals (adversarial usage scan)
`grep` over the whole file for the affected names:
- `dirname` — used **only** on line 88. Removing it from the `node:path` import is safe; `join`, `relative`, `resolve` remain used elsewhere (lines 89-91, 152, 162, 198, 248, 260, 279).
- `fileURLToPath` — used **only** on line 88. The entire `node:url` import line (86) can be deleted = **1 line cut**, matching `lines_cut: 1`. `deps_cut: 0` correct (`node:url` is stdlib, not a package dependency).

### (d) tag=native, not yagni — no construct/replacement-naming gate applies. Both concrete construct and concrete replacement are named regardless.

## Decision guard — no conflict, not tracked
Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `import.meta` / `dirname` / `fileURLToPath` / cosmetic-cleanup / modernize:
- Only ROADMAP hits on `check-i18n` are unrelated: line 189 (severity-cell text), line 303 (T20-m2 fixture self-test — about parity/drift logic, not this import).
- **Cosmetic cleanup group K** (ROADMAP:260-267) enumerates its scope explicitly (dead `at` param, invalid-template mislabel, TracksCfg placement, stale module doc, Plan-1 archive remnants, eager resolve on planner.rs) — this `check-i18n.mjs` import modernization is **not** among them.
- No design memo (D1-D35), IDEAS entry, or deferred/deliberate-restraint entry references this construct.

No recorded decision constrains it; it is not already tracked. Verdict stands as CONFIRMED.
