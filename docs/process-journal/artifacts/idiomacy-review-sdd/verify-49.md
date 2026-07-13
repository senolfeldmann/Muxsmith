# Verify-49: check-i18n.mjs walkSourceFiles vs native recursive readdir

**Finding (F6, tag=stdlib):** `walkSourceFiles()` at `scripts/check-i18n.mjs:160`
hand-rolls a recursive directory walk that Node's native
`readdirSync(SRC, { recursive: true })` already provides; replace and delete the
helper.

**Verdict: CONFIRMED**

## (a) Does the cited code say what the finding claims? Yes

`scripts/check-i18n.mjs:160-169` is exactly a hand-rolled recursive walk:

```js
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
```

Consumed at lines 171-172 (`const sourceFiles = []; walkSourceFiles(SRC, sourceFiles);`).
`readdirSync` is already imported (line 84). The description matches the code.

## (b) Is the replacement current idiom for the pinned toolchain? Yes

Node pinned to **26.5.0** via `mise.toml` (`node = "26.5.0"`, confirmed
`node --version` -> v26.5.0). The `recursive` option on `fs.readdirSync` /
`fs.readdir` has been stable since Node 20.1.0 and is the idiomatic way to list
a tree. On Node 26 it is unquestionably available and standard. Without
`withFileTypes`, it returns path strings relative to the root, so
`.map((f) => join(SRC, f))` restores the absolute paths the original produced.

## (c) Load-bearing difference between the two sites? No (verified empirically)

Ran both on the real tree at HEAD (2f17880):

```
walk count: 17   recursive count: 17
only in walk: []   only in recursive: []
identical: true
```

Identical file set, confirming the finding's "verified on the pinned Node 26.5.0
to return the identical file set." The finding's end-anchored-regex reasoning is
also correct: `/\.(vue|ts)$/` anchors at end-of-string, and the last path
segment of the relative path *is* `entry.name`, so testing the full relative
path is equivalent to testing the basename. Order differs (DFS vs readdir
order) but order is not load-bearing here (results feed Sets and an unsorted
`missing` array used only for console output ordering).

**Minor caveat for the implementer (not a refutation):** the native form filters
directory entries by regex rather than by `isDirectory()`. A directory literally
named `*.vue`/`*.ts` would be matched and later `readFileSync`'d (EISDIR),
whereas the original recurses into it and never pushes it. No such directory
exists in `src/` (hence the identical 17==17), and it is a non-scenario for a
Vue/TS project, so the finding's claim holds for the actual codebase. An
implementer wanting belt-and-suspenders could keep `withFileTypes: true` and
filter `!isDirectory()`, but that is a robustness refinement, not a correction.

## (d) tag=stdlib, not yagni

Criterion (d) does not apply.

## Decision guard: not tracked, no conflict

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md`:

- ROADMAP mentions `check-i18n` only at line 189 (M1 severity-cell reference)
  and line 303 (`check-i18n.mjs fixture self-test (T20-m2)` — a self-test
  backlog item, unrelated to the walk implementation).
- Cosmetic-cleanup **sweep group K** (ROADMAP:260-267) enumerates its cleanups
  (dead `at` param, invalid-template mislabel, TracksCfg placement, stale module
  doc, Plan-1 archive remnants, eager chapters/attachments resolve) — it does
  **not** include `walkSourceFiles` or this script.
- Spec hits for `recursive: true` (v1-design.md:62,123,198) are Muxsmith's own
  rule-schema `recursive` option, not this script.

No recorded decision defends the hand-rolled walk and no tracker entry already
owns this cleanup. Verdict stands as CONFIRMED.
