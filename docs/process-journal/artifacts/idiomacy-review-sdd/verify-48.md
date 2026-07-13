# Verify-48: F5 yagni — dead null tolerance in i18n locale resolution

**Verdict: CONFIRMED**

Finding: `src/i18n/index.ts:79` — `buildBundles(locale: string | null | undefined)` and
`resolveLocale(): Promise<string | null>` (`src/main.ts:16`) carry null tolerance that is
unproducible; the type-guard filter (line 80-81) defends inputs the only caller already
normalized away. Replace with `Promise<string>` / `buildBundles(locale: string)`, delete the
`.filter((tag): tag is string => ...)` line, keep the Set dedup.

## Code verification (criterion a) — accurate

- `src/main.ts:16-22` — `resolveLocale(): Promise<string | null>`; try branch returns
  `(await getSettings()).locale ?? navigator.language`, catch returns `navigator.language`.
- `src/ipc.ts:31` — `AppSettings.locale: string | null`; `getSettings(): Promise<AppSettings>`
  (ipc.ts:234). So `.locale` is `string | null`, and `?? navigator.language` coalesces it.
  `navigator.language` is typed `string` in the DOM lib. Both branches therefore yield `string`;
  **null is genuinely unproducible** and the `| null` on the return type is dead. Confirmed.
- `src/i18n/index.ts:79-82` — signature `string | null | undefined`; the pipeline is
  `[locale, "en"].filter((tag): tag is string => typeof tag === "string" && tag.length > 0).map(primarySubtag)`.
  The filter's job is to strip null/undefined/"" — exactly the values the caller cannot pass. Confirmed.
- **Only caller**: `grep -rn buildBundles src/` → sole call site is `src/main.ts:26`
  `buildBundles(locale)` with `locale = await resolveLocale()` (a `string`). No other caller.
  It is exported, but Muxsmith is a Tauri app, not a published library, so there is no external
  consumer justifying the wide boundary. Confirmed.
- **Residual empty-string degradation**: traced without the filter. If `locale === ""`,
  `requested = ["", "en"].map(primarySubtag) = ["", "en"]`. `buildBundle("")` →
  `catalogsForLocale("")` matches no `/locales/<seg>/` directory → `sources.length === 0` →
  returns `null` → the "" bundle is skipped; "en" still builds. Degrades identically to the
  filtered path. Confirmed. (`primarySubtag("") = "".split("-")[0].toLowerCase() = ""`.)
- **Set dedup must stay**: with the filter gone and `locale === "en"`, `requested = ["en","en"]`;
  the `seen` Set collapses the duplicate so "en" builds once. The finding correctly flags keeping it.

## Idiomacy (criterion b) — replacement is current idiom

TypeScript 6.0.3 (held under the typescript-eslint ceiling). Narrowing a parameter/return to the
actual producible domain and removing an unreachable type-guard filter is standard TS practice, not
a version-specific concern. No toolchain nuance changes this. The `?? navigator.language` at the
real null boundary (main.ts) stays, so the one genuine `null` source is still handled. Sound.

## yagni tag construct check (criterion d) — satisfied

Concrete construct named (`.filter((tag): tag is string => ...)` line + the `| null` /
`| null | undefined` annotations) and concrete replacement named (`Promise<string>`,
`buildBundles(locale: string)`, delete the filter line, keep Set dedup). Matches the ROADMAP's own
`yagni` tag definition (line 168-171: "one caller, dead flags").

## Duplication (criterion c) — N/A

Not a duplication finding.

## Decision guard — no conflict, not tracked

- `grep -rniE "buildBundles|resolveLocale|i18n/index|null tolerance|type.guard"` over
  `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` → no hit.
- Cosmetic-cleanup sweep **group K** (ROADMAP 260-267) enumerates unrelated dead code
  (`at` param load.rs, TracksCfg placement model.rs, stale module doc, planner.rs eager resolve);
  this i18n construct is **not** among them.
- No plan-5 / plan-5.5 design decision records the null-tolerance of `buildBundles` as deliberate;
  the surrounding code comments cite spec 8.4 / spec 11 only for the glob-based catalog mechanism,
  not for the signature width. No DECISION_CONFLICT, no TRACKED.

## Conclusion

The cited code says exactly what the finding claims; the replacement is idiomatic and behavior-
preserving; the construct is untracked and unconflicted. **CONFIRMED.**
