# Idiomacy verify-47: e2e/mocks.ts JSON round-trip through exposeFunction

**Finding (F5, tag=native):** Manual `JSON.stringify` (page side, line 101) /
`JSON.parse` (node side, line 148) round-trip through `page.exposeFunction` is
redundant with Playwright's own argument serialization. Replace with
`window.__muxsmithRecordInvoke__?.(cmd, args ?? null)` and push the received
value directly; `global.d.ts` signature becomes `(cmd: string, args: unknown) => void`.

**Verdict: CONFIRMED**

## (a) Does the code say what the finding claims? Yes, verbatim.

- `e2e/mocks.ts:101` — `window.__muxsmithRecordInvoke__?.(cmd, JSON.stringify(args ?? null));`
- `e2e/mocks.ts:148-149` — `page.exposeFunction("__muxsmithRecordInvoke__", (cmd: string, argsJson: string) => { recorded.push({ cmd, args: JSON.parse(argsJson) as unknown }); })`
- `e2e/global.d.ts:32` — `__muxsmithRecordInvoke__?: (cmd: string, argsJson: string) => void;`

Three sites, exactly as the finding describes. The page side stringifies, the
Node side re-parses, and the ambient type carries the intermediate `argsJson: string`.

## (b) Is the replacement current idiom? Yes.

Checked against current Playwright docs (WebSearch, Playwright API `class-page`
+ exposeBinding history), not memory. `page.exposeFunction` serializes its
arguments across the page->Node boundary itself: JSON-compatible values plus
`undefined`/`NaN`/`Infinity`/`-0`/`BigInt`/`Date`; **plain objects and arrays
serialize deeply** (a deep copy at transfer). Circular references throw on both
paths, so nothing is lost by dropping the manual stringify. Hand-rolling a JSON
round-trip on top of a mechanism that already deep-copies is the textbook
"native platform feature over reimplementation" case: passing the value
directly and letting Playwright serialize it is the idiomatic form.

The pinned toolchain (Vue 3 / TS 6.0.3 / Tauri 2 / node 26.5.0 / pnpm 11.10.0)
does not bear on this — it is a Playwright API behavior, stable across recent
versions. Playwright is not among the version-pinned deps, so there is no
version ceiling to violate.

## (c) Load-bearing difference between the two "sites"? No, for this codebase.

The only divergence between a JSON round-trip and Playwright's serialization is
non-JSON values: JSON.stringify drops `undefined` properties, maps
`NaN`/`Infinity`->`null`, throws on `BigInt`, and coerces `Date`->ISO string,
whereas Playwright preserves them. This divergence cannot fire here: the mock
records `invoke` command arguments, which are **JSON-shaped by construction**
(they cross a real JSON IPC boundary in production) — the finding states this
explicitly and it is correct. The `?? null` top-level coalescing is preserved
identically in the replacement (`args ?? null`), so the undefined-args case
(commands invoked with no payload) still records `null`, unchanged.

Snapshot semantics: `exposeFunction` deep-copies at transfer, and the mock
handler is invoked synchronously by the app's `invoke()` with a fresh args
object that is not subsequently mutated in this codebase, so the recorded value
is a faithful snapshot either way. No behavioral change for any current or
plausible test assertion.

The proposed signature change is coherent and complete across all three sites
(page call, Node callback, `global.d.ts` ambient type).

## (d) yagni gate: N/A.

Tag is `native`, not `yagni`. A concrete construct (`JSON.stringify`/`JSON.parse`
round-trip) and a concrete replacement (direct value pass + signature change)
are both named.

## Decision guard: not tracked, no conflict.

Grepped `docs/ROADMAP.md`, `docs/IDEAS.md`, and `docs/superpowers/specs/*.md`
(D1-D35) for `mocks.ts` / `recordInvoke` / `argsJson` / `exposeFunction` /
`JSON.stringify`. The construct appears only in:
- the source and its typings (`e2e/mocks.ts`, `e2e/global.d.ts`),
- historical process-journal artifacts (Plan-5 / Plan-5.5 review diffs and task
  reports) documenting the original implementation.

It is **not** listed as a deferred/tracked cleanup in ROADMAP cosmetic-cleanup
group K, the test-hygiene collection (B-minors), IDEAS.md, or any D-memo, and no
decision authorizes or forbids the JSON round-trip. So neither DECISION_CONFLICT
nor TRACKED.

## Conclusion

Real, low-risk idiomacy cleanup. The manual JSON serialization duplicates
Playwright's own deep-copy argument transfer; for JSON-shaped Tauri invoke args
the two are equivalent, and the direct form is the native idiom. CONFIRMED.
