# Task 9 report: Frontend app shell, Fluent, first-run, settings dialog

Executed directly on `master` (single-task wave, no worktree). Base `7ce88db`,
commit `65d7065`.

## What was implemented

**`src/i18n/index.ts`** (new): the fluent-vue catalog loader. Uses
`import.meta.glob(["../../locales/*/gui-*.ftl", "../../locales/*/diagnostics.ftl"], { query: "?raw", import: "default", eager: true })`
rather than literal per-file imports, globbed by *locale directory* rather
than hardcoded to `en`. Two consequences: T10's `gui-batch.ftl` and T11's
`gui-jobs.ftl` are picked up automatically with no loader edit, and a
future non-English locale is pure `.ftl` content under a new
`locales/<tag>/` directory (spec 11's "mechanism ships complete, content is
separate" applied to the frontend too). `buildBundles(locale)` builds the
fallback chain fluent-vue negotiates against (`[requestedLocale, "en"]`,
deduped) -- spec 8.4's "falls back to English per message" is real
per-message fallback via fluent-vue's own bundle-chain semantics, not just
a startup default. `cli.ftl` is deliberately excluded (CLI-only
vocabulary the frontend never renders).

**`src/main.ts`** (rewritten): now `async`, resolves the locale
(`settings.locale ?? navigator.language`) via `getSettings()` *before*
`createApp().mount()`, so first paint already uses the right catalog chain
instead of flashing English and re-rendering. `get_settings` failing is
not a startup blocker (falls back to `navigator.language`, which
`buildBundles` itself falls back from to `"en"`).

**`src/ipc.ts`** (new): typed `invoke()` wrapper per Rust
`#[tauri::command]` in `lib.rs`/`run.rs` (all 11: `validateProfile`,
`dryRun`, `identify`, `detectMkvmerge`, `getSettings`, `setSettings`,
`startRun`, `cancelRun`, `cancelJob`, `listRuns`, `getJobLog`), argument
names camelCased per Tauri's default IPC convention (verified against
`@tauri-apps/api` docs, not assumed -- the one Rust param with an
underscore, `run_id`, is `runId` on the JS side). Types mirror the Rust
structs field-for-field (`IpcError`, `AppSettings`, `DirMemory`,
`MkvmergeInfo`, `StartedRun`, `RunMeta`, `JoblogStatus`, `JobEvent`'s six
tagged variants, `JobOutcome`, `JobLogRecord`). `ReportDocument`/
`ReportFile` leave `files[].plan`/`suggestions` as `unknown` rather than
half-mirroring the planner's `FilePlan`/`Suggestion` shapes -- no T9 code
reads them, and T10 (the Batch view, the actual consumer) is where that
shape earns a real type. T9 itself only calls
`detectMkvmerge`/`getSettings`/`setSettings`; the rest exist so T10/T11
don't redefine this layer.

**`src/App.vue`** (rewritten): startup gate is two plain refs
(`checking`, `blockedError: IpcError | null`) rather than a discriminated
union, deliberately -- vue-tsc's template narrowing is reliable for a
plain nullable-ref guard (`v-else-if="blockedError"` narrowing it for the
`:error="blockedError"` prop binding immediately after) but is a needless
risk for narrowing a union's field inside the same expression, and
plain-ref narrowing is exactly what `pnpm build` confirmed works. On
mount, `App.vue` calls `detectMkvmerge()` exactly once; success shows the
shell (`<header>` app-title, `<nav>` with `aria-current="page"` on the
active Batch/Jobs tab plus a Settings button, `<main>` mounting
`BatchView`/`JobsView`, `SettingsDialog` mounted alongside). Failure
mounts `FirstRun`, which owns every retry itself and only reports back via
`@resolved` once its own re-detect actually succeeds (no redundant
re-probe from the parent).

**`src/views/FirstRun.vue`** (new, D28): per-OS guidance keyed off
`platform()` from `@tauri-apps/plugin-os` (windows/macos/linux/fallback),
wording verified against `capability::runtime::platform_candidates`
(`crates/muxsmith-core/src/capability/runtime.rs:263`) so the paths named
in the guidance text are the same ones the detection ladder actually
probes. A labeled path input (prefilled from `getSettings()` if an
override is already set) plus a dialog-plugin file picker ("Browse...", a
shared `browse-button` key SettingsDialog also uses); "Use this path"
(`set_settings` then re-detect) and "Retry detection" (re-detect only, for
"I just installed it in a standard location") are separate actions. Every
`IpcError` this screen can hit (`mkvmerge-not-found`, `mkvmerge-too-old`,
`mkvmerge-spawn-failed`, `mkvmerge-query-failed`) renders via
`$t(currentError.code, currentError.params)` uniformly -- the same
code+params-select-a-template mechanism core diagnostics use, not a
per-code hand-written branch.

**`src/components/SettingsDialog.vue`** (new, D27): native `<dialog>`
opened imperatively (`defineExpose({ open })`, App.vue holds the template
ref) rather than `<dialog open>`/declarative binding, and the inner
`<form>` deliberately has no `method="dialog"` -- saving needs an async
IPC round trip to complete (and the dialog to stay open on failure), which
`method="dialog"`'s synchronous-close-on-submit cannot express. Three
labeled fields (`label for`/`id`): mkvmerge path override (+ Browse...),
default parallel jobs (`type="number" min="1"`, clamped defensively on
save mirroring the core's own `>=1` clamp), and locale (`<select>` with a
single "English" option today -- structurally complete for v1.x locale
additions, per the same mechanism/content split as the i18n loader).

**Catalogs**: `locales/en/gui-common.ftl` extended (nav/settings-button
strings, every IpcError code T9 surfaces, all `firstrun-*` screen copy);
new `locales/en/gui-settings.ftl` for the dialog's own strings. Both load
into the SAME FluentBundle at runtime (the loader merges all `gui-*.ftl`
files), so `gui-settings.ftl` freely references `browse-button` from
`gui-common.ftl` -- the file split is ownership/organization, not a
namespace boundary.

**`src/views/{BatchView,JobsView}.vue`** (new, placeholders): one `<h2>`
Fluent heading each (`nav-batch`/`nav-jobs`, reusing the nav's own keys
rather than minting throwaway placeholder-only keys), `data-testid`
matching the D29 structural-node convention. T10/T11 replace these files'
content; the view-switch mechanism (`App.vue`'s `ref<'batch'|'jobs'>`)
does not change when they do.

## Deviations from the brief, flagged explicitly

1. **`@tauri-apps/plugin-os`, not `@tauri-apps/api`, for `platform()`.**
   The brief names `@tauri-apps/api` for platform detection; verified via
   context7's current Tauri docs (query: "os plugin JavaScript") and this
   repo's own `node_modules`/`Cargo.toml` that Tauri 2 moved OS info into a
   separate plugin -- `@tauri-apps/api` v2.11.1 ships no `os` module at all
   (checked its `.d.ts` file list directly). Added `@tauri-apps/plugin-os`
   (JS, exact `2.3.2`) and `tauri-plugin-os` (Rust, exact `2.3.2`, matching
   the latest available on both registries), registered
   `.plugin(tauri_plugin_os::init())` in `lib.rs`, added `"os:default"` to
   `capabilities/default.json`. This is the one place the task's "ZERO new
   deps" expectation was not met; the alternative (`navigator.userAgent`
   sniffing in the webview) is the kind of brittle special-case this
   codebase's own conventions steer away from, and it is not what Tauri's
   own docs recommend for this exact purpose.
2. **Fixed a real gap in the D29 accessibility lint contract.** The task
   brief and D29 both assert "aria-labels included" are lint-covered by
   `@intlify/vue-i18n/no-raw-text`. Reading the installed rule's source
   (`node_modules/.pnpm/@intlify+eslint-plugin-vue-i18n@.../no-raw-text.js`)
   showed `attributes` defaults to an empty set when no rule option is
   passed -- as configured before this task, the rule only ever checked
   text nodes, never `title`/`aria-label`/`placeholder`/`alt`. Added the
   `attributes: { "/.*/": [...] }` option to `eslint.config.js` and
   verified empirically (temporarily added a raw `title="literal test
   string"` to a placeholder view, confirmed `pnpm lint` fails with `raw
   text 'literal test string' is used`, reverted). This is scoped
   narrowly: it only catches STATIC (non-`:`-bound) attributes; a correct
   `:title="$t(...)"` binding is a directive and was always outside this
   rule's reach regardless of the option, so no existing code needed
   fixing -- this closes a "forgot to bind it" gap, nothing more.

Both are small, targeted, and directly serve this task's own stated
requirements rather than expanding scope; happy to discuss either if the
controller wants a different call.

## Self-review

- **Zero hardcoded strings**: grepped all new/changed files for unbound
  `aria-label="`/`title="` (none found) and for stray `console.log`/
  `debugger`/`TODO` (none found); `pnpm lint` is clean with the tightened
  attribute check, and its bite was verified empirically (see above), not
  assumed.
- **A11y (D29)**: semantic HTML throughout (`<nav>`, `<main>`, `<dialog>`,
  `<button>`, `label for`/`id` on every form field); `aria-current="page"`
  on the active nav tab; `aria-live="polite"` regions on the startup
  "checking" message and FirstRun's heading/detail block (status changes
  across retries get announced); `role="alert"` on SettingsDialog's error
  line; `aria-busy` + `disabled` on every button that triggers an async
  IPC call; `data-testid` on every structural node (`nav-batch`,
  `nav-jobs`, `open-settings`, `first-run`, `settings-dialog`,
  `view-batch`, `view-jobs`) per the D29/D30 convention (`data-testid="job-row"`
  precedent). Skipped tooltips on the Batch/Jobs nav buttons deliberately
  (self-explanatory top-level navigation, not "non-obvious" per D29's own
  bar) while adding one on Settings (its scope -- mkvmerge path, jobs,
  locale -- is not obvious from the single word alone); noting the
  judgment call rather than asserting blanket tooltip coverage.
  ESLint 10.6.0 / vue-eslint-parser output round-trips real HTML minus
  Vue directives, so tag/attribute soup structurally matches D29's rules.
- **Event-ordering contract**: T9 does not touch `start_run`/job events at
  all (T8's `start_run` doc note about registering listeners before
  invoking is T11's concern), so nothing here interacts with that.
- **Type safety**: `pnpm build` (`vue-tsc --noEmit && vite build`) is
  clean with zero suppressions/`any`; the one place I deliberately chose
  `unknown` (`ReportDocument.files[].plan`/`.suggestions`) is documented as
  an explicit scope boundary, not a shortcut.

## Gate (all six, run together, foreground)

```
$ cargo test --workspace                              -> every suite "0 failed"
$ cargo fmt --all --check                              -> exit 0
$ cargo clippy --workspace --all-targets -- -D warnings -> clean, 0 warnings
$ cargo deny check                                      -> advisories ok, bans ok, licenses ok, sources ok
$ pnpm lint                                             -> exit 0, 0 warnings
$ pnpm build                                            -> vue-tsc clean, vite build succeeds (96 KB / 36 KB gzip)
```

Rust suite is unchanged in substance (269 pre-existing tests + the 72
`muxsmith-gui` unit tests, all pass; no Rust IPC command logic was
touched, only plugin registration/capabilities). The `os_info`/
`tauri-plugin-os` addition surfaced no new `cargo deny` advisory/license/
bans issues; the pre-existing multi-version-dependency warnings in its
output (`base64`, `winnow`, etc.) are unrelated to this change and were
already present in the dependency tree via `tauri`/`tauri-build` itself.

Did not launch `pnpm dev`/the Tauri window (no display assumptions, per
instructions); build + lint + vue-tsc are the automated checks until T12's
Playwright smoke lands.

## Files changed

- New: `src/i18n/index.ts`, `src/ipc.ts`, `src/views/FirstRun.vue`,
  `src/views/BatchView.vue`, `src/views/JobsView.vue`,
  `src/components/SettingsDialog.vue`, `locales/en/gui-settings.ftl`
- Modified: `src/App.vue`, `src/main.ts`, `locales/en/gui-common.ftl`,
  `eslint.config.js`, `package.json`, `pnpm-lock.yaml`,
  `src-tauri/Cargo.toml`, `Cargo.lock`, `src-tauri/src/lib.rs`,
  `src-tauri/capabilities/default.json`

## Issues or concerns

None blocking. The two flagged deviations (plugin-os dependency, eslint
attribute-check fix) are the only departures from the brief's literal
text; both are documented above with rationale and were verified, not
assumed. `src-tauri/gen/schemas/*.json` did not change on disk (git shows
no diff) despite `cargo build` running after the new plugin/capability
were added -- the build succeeded and `capabilities/default.json`'s new
`"os:default"` entry validated without error, so this is not blocking,
but noting it in case a future task expects those schema files to always
reflect the live plugin set.
