# Idiomacy verify-51 — check-i18n.mjs matchAll refactor

**Verdict: CONFIRMED**

**Finding:** `scripts/check-i18n.mjs:191` — manual `while ((m = CALL_RE.exec(line)))` loop with explicit `CALL_RE.lastIndex = 0` reset is the pre-ES2020 pattern; replace with `for (const m of line.matchAll(CALL_RE)) { ... }`, dropping the reset and the `let m` declaration (2 lines cut).

## (a) Cited code matches the claim — YES

HEAD `2f17880a956e05f833a3afdec2c650c176e391e5`, lines 179, 191-201:

```js
const CALL_RE = /(?<![\w$])\$?t\(\s*(['"])([^'"]*)\1/g;   // module-level, g flag
...
lines.forEach((line, i) => {
    CALL_RE.lastIndex = 0;                    // 192  <- the manual reset
    let m;                                     // 193
    while ((m = CALL_RE.exec(line)) !== null) {// 194
      const id = m[2];
      ...
    }
  });
```

The construct is exactly as described: shared module-level global regex, per-line manual `lastIndex = 0` reset, `let m` + `while (exec)` loop. Only `m[2]` is consumed from each match.

## (b) Replacement is current idiom for the pinned toolchain — YES

Toolchain: node 26.5.0 (confirmed via `node --version`). `String.prototype.matchAll` is stable ES2020, fully supported. It is the language feature that exists precisely to remove the `exec`-in-a-loop-with-manual-lastIndex footgun.

Empirically verified in the pinned node (not from memory), reproducing `CALL_RE` and a line containing `t(...)`, `$t(...)` and a decoy `emit(...)`:

- Old loop and `for...of line.matchAll(CALL_RE)` produce identical results: `["foo","bar","baz"]` (decoy `emit(` correctly excluded, `$t` correctly captured) — **equal: true**.
- `matchAll` clones the regexp internally, so `CALL_RE.lastIndex` is **0 before and after** iteration — the `lastIndex = 0` reset is genuinely unnecessary. This is the load-bearing point: because `CALL_RE` is a *shared* module-level regex reused across every file and line, the old code needs the reset to avoid cross-line state leakage; `matchAll` removes that shared-mutable-state hazard entirely rather than just shortening the code.
- `matchAll` requires the `g` flag (throws `TypeError` otherwise); `CALL_RE` has it, so the swap is safe. Regex and flag unchanged, `m[2]` access unchanged.

`lines_cut: 2` is accurate: `CALL_RE.lastIndex = 0;` and `let m;` are deleted, the `while` becomes the `for...of` header.

## (c) Not a duplication claim — n/a

## (d) Not a yagni tag — tag is `idiom`; concrete construct and concrete replacement both named.

## Decision guard — no conflict, not tracked

Grepped `docs/superpowers/specs/*.md` (D-memos plan-2 through pre-1.0), `docs/IDEAS.md`, `docs/ROADMAP.md` for `CALL_RE`, `matchAll`, `lastIndex`, `.exec(`, ES2020:

- No spec/IDEAS mention of this loop or a matchAll refactor.
- ROADMAP `check-i18n.mjs` entries are unrelated: **T20-m2** is a *fixture self-test* for the parity/drift logic; **M1** and the **B-minors** test-hygiene collection concern parser/parity gaps (`.attr` scanner, id-set-only parity), not the source-scan `exec` loop.
- Cosmetic-cleanup **sweep group K** enumerates specific unrelated items (dead `at` param, `*[empty-field]` mislabel, TracksCfg placement, stale module doc, Plan-1 remnants, eager chapters resolve) — the CALL_RE loop is not among them.

No recorded decision to keep the manual pattern; not already tracked. Verdict stands as CONFIRMED.
