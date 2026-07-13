# Idiomacy verification — finding 52 (slice F6)

**Verdict: CONFIRMED**

Finding: `scripts/check-i18n.mjs:280` — hand-rolled set difference
`[...refIds].filter((id) => !localeIds.has(id))` (and the mirror on line 281)
where the pinned toolchain ships ES2025 `Set.prototype.difference`.
Proposed replacement:
`const missingIds = [...refIds.difference(localeIds)].sort();`
`const extraIds = [...localeIds.difference(refIds)].sort();`

## (a) Cited code matches — yes

At HEAD `2f17880`, lines 280-281 read verbatim:

```js
const missingIds = [...refIds].filter((id) => !localeIds.has(id)).sort();
const extraIds = [...localeIds].filter((id) => !refIds.has(id)).sort();
```

Both operands are genuine `Set` instances, so `.difference()` is callable in
both directions:
- `refIds = referenceIdsByFile.get(file)`; the Map is built at line 245-250 with
  values `new Set(parseCatalogIds(...))`.
- `localeIds = new Set(parseCatalogIds(join(dir, file)))` at line 279.

## (b) Replacement is current idiom for the pinned toolchain — yes

- Node pinned to `26.5.0` (`mise.toml` line 2). Local `node --version` = `v26.5.0`.
- Empirical check on the installed runtime (authoritative, not training memory):
  `'difference' in Set.prototype` → `true`; `new Set([1,2,3]).difference(new Set([2]))`
  → `[1, 3]`. No flag required. ES2025 Set methods have shipped in V8 since Node 22.
- Behavior is identical. `Set.prototype.difference(other)` returns a new Set of
  elements of the receiver not in `other`, preserving the receiver's insertion
  order — the same elements the `filter` produced. The trailing `.sort()` makes
  ordering moot regardless. Variable names (`missingIds`, `extraIds`) match the
  existing code.
- This is the human-normal solution on a Node-26 toolchain: ES2025 Set algebra
  exists precisely for this; spread+filter is the pre-ES2025 workaround. Aligns
  with the "native platform feature over hand-rolled equivalent" convention.

## (c) Not a claimed duplication — n/a

Tag is `native`, not a reuse/duplication finding. The two sites (280, 281) are
mirror directions of the same operation and are both covered by the replacement.

## (d) Not yagni — n/a

Tag is `native`; a concrete construct (spread+filter set difference) and a
concrete replacement (`Set.prototype.difference`) are both named.

## Decision guard — no conflict, not tracked

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for
`difference`, `set.prototype`, `set method`, `key parity`, `check-i18n`. The only
`check-i18n` hits are unrelated:
- `ROADMAP.md:189` — "de catalog headers overclaim what check-i18n enforces (M1)"
  (catalog header wording, not the Set construct).
- `ROADMAP.md:303` — "check-i18n.mjs fixture self-test (T20-m2)" (a planned
  fixture-based self-test, not this construct).

No design memo (D1-D35), IDEAS entry, cosmetic-cleanup group K, or
deliberate-restraint entry references this hand-rolled difference. Not a recorded
decision and not already tracked.

## Conclusion

Code is as cited; replacement is correct, behavior-preserving, and the current
idiom on the pinned Node 26.5.0 runtime (empirically verified). No decision
conflict, not tracked. **CONFIRMED.**
