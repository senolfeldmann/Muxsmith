# Verify-44: e2e/global.d.ts hand-mirrored Tauri signatures

**Verdict: CONFIRMED**

Finding (tag=idiom, slice F5): `window.__muxsmithE2E__` member types in `e2e/global.d.ts`
lines 16-22 hand-mirror the pinned `@tauri-apps/api` function signatures instead of
`typeof`-importing them; already drifted; type-only imports are erased so the addInitScript
serialization constraint does not apply. Replacement: `import type` the four functions, then
`mockIPC: typeof mockIPC` etc.

## (a) Does the cited code say what the finding claims? YES

`e2e/global.d.ts` HEAD (2f17880), lines 15-23:

```ts
__muxsmithE2E__: {
  mockIPC: (
    cb: (cmd: string, args?: unknown) => unknown,
    options?: { shouldMockEvents?: boolean },
  ) => void;
  mockWindows: (current: string, ...rest: string[]) => void;
  clearMocks: () => void;
  emit: (event: string, payload?: unknown) => Promise<void>;
};
```

These are literal hand-written mirrors of the four real functions.

## Drift against the pinned version is real (@tauri-apps/api 2.11.1, pinned exactly in package.json)

Real signatures from node_modules:

- `mocks.d.ts:91`: `export declare function mockIPC(cb: (cmd: string, payload?: InvokeArgs) => unknown, options?: MockIPCOptions): void;`
  - Mirror drifted twice: callback param `args?: unknown` vs real `payload?: InvokeArgs`;
    inline `{ shouldMockEvents?: boolean }` vs the named `MockIPCOptions` interface.
- `event.d.ts:129`: `declare function emit<T>(event: string, payload?: T): Promise<void>;`
  - Mirror drifted: non-generic `(event: string, payload?: unknown)` vs real generic `<T>`.
- `mocks.d.ts:131`: `mockWindows(current: string, ..._additionalWindows: string[])` -- mirror
  type-identical (only rest-param name differs, irrelevant to assignability).
- `mocks.d.ts:177`: `clearMocks(): void` -- mirror identical.

So the "already drifted" claim holds for `emit` and `mockIPC`.

## (b) Is the replacement current idiom for the pinned toolchain? YES

`import type ... ; ... : typeof <fn>` is the standard TypeScript pattern for binding a wrapper
type in lockstep with an imported source of truth. It works under this project's config:

- Root `tsconfig.json`: `moduleResolution: "Bundler"` resolves the subpath exports
  `@tauri-apps/api/mocks` and `.../event` (the same specifiers `tauri-mock-entry.ts` already
  imports at value level); `isolatedModules: true` is compatible with `import type` (fully
  erased). TS held at 6.0.3 -- `import type` + `typeof` predates that and is unaffected.
- Member key `mockIPC:` is a property name, not a binding, so `typeof mockIPC` referring to the
  imported binding is not a collision; standard usage.

Exports confirmed present: `mocks.d.ts` exports `mockIPC`, `mockWindows`, `clearMocks`;
`event.d.ts` exports `emit` (line 147). The `typeof` target is exact because
`tauri-mock-entry.ts` assigns those very functions:
`window.__muxsmithE2E__ = { mockIPC, mockWindows, clearMocks, emit };`

### The serialization-constraint objection does not defeat the replacement

The file's own doc (lines 3-5) and `mocks.ts` explain why `mocks.ts`'s `installMockIPC` cannot
`import` these -- it is serialized into the page via `page.addInitScript`. That constraint is a
RUNTIME concern for `mocks.ts`. `global.d.ts` is a pure ambient declaration file (`declare
global`), emits no runtime JS, and is not part of any addInitScript payload. A type-only import
there is erased at compile time and creates no runtime dependency. Adding `import type` keeps the
file a module (it already is one via `export {}`), so the `declare global` augmentation continues
to work. The replacement is technically sound and strictly more faithful than the current
approximation.

## (c) Load-bearing divergence between the two sites? NO

The hand-written mirror is not intentionally different from the real signatures -- the intended
type IS the real function's type (`tauri-mock-entry.ts` assigns the real functions). The
divergence is accidental drift, not a deliberate narrowing. No load-bearing reason to keep them
separate.

## Decision guard: not tracked, no conflict

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `global.d.ts`,
`__muxsmithE2E__`, `hand-mirror`, `typeof import`, `tauri-mock-entry`, `mockIPC` -- no hits.
Reviewed ROADMAP cosmetic-cleanup group K (260-267) and test-hygiene collection (307-321):
neither covers this ambient-type mirror. No recorded decision to hand-write these types; not on
any deferred/deliberate-restraint list.

## Conclusion

Cited code accurate, drift real, replacement is current idiom that resolves under the pinned
toolchain, the serialization constraint genuinely does not apply to the `.d.ts`, and the item is
untracked. CONFIRMED.
