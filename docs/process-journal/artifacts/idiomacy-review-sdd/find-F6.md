# Idiomacy review - slice F6

Scope: `crates/xtask/**` (Cargo.toml, src/lib.rs, src/main.rs, src/codegen.rs, tests/codegen.rs, tests/fixtures/mini-schema.json), `scripts/check-i18n.mjs`, all 12 `.ftl` catalogs under `locales/en/` and `locales/de/` (structural Fluent idiom only; terminology untouched).

Toolchain ground truth applied: Rust 1.96.1 / edition 2024, Node 26.5.0 (pinned), Fluent syntax 1.0. Platform claims verified empirically against the installed Node 26.5.0 (`import.meta.dirname`, `Set.prototype.difference`, `readdirSync` `recursive`) and against projectfluent.org guide + fluent.ebnf and fluent-vue docs (comment levels, attributes, `$ta`/`v-t`, leading-blank stripping).

## Findings

### F6-1 (stdlib) scripts/check-i18n.mjs:160 - hand-rolled recursive directory walk

`walkSourceFiles` (lines 160-169, plus the two call lines 171-172) reimplements what Node's own fs API does natively since v18.17 and does on the pinned Node 26.5.0 (verified: returns the same 17 `src/**/*.{vue,ts}` files):

```js
const sourceFiles = readdirSync(SRC, { recursive: true })
  .filter((f) => /\.(vue|ts)$/.test(f))
  .map((f) => join(SRC, f));
```

The extension regex is end-anchored, so testing the full relative path is equivalent to testing `entry.name`. `fs.globSync` would also work on Node 26 but buys nothing over the one-liner here. lines_cut ~8.

### F6-2 (native) scripts/check-i18n.mjs:88 - `fileURLToPath(import.meta.url)` dance

`const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..")` is the pre-Node-21 idiom. Node 26 has stable `import.meta.dirname` (verified): `const ROOT = resolve(import.meta.dirname, "..")`. Drops the entire `node:url` import (line 86) and the `dirname` import. lines_cut 1.

### F6-3 (idiom) scripts/check-i18n.mjs:191-194 - manual regex exec loop with `lastIndex` reset

```js
CALL_RE.lastIndex = 0;
let m;
while ((m = CALL_RE.exec(line)) !== null) {
```

is the pre-ES2020 pattern; the manual `lastIndex` reset is exactly the shared-global-regex footgun `String.prototype.matchAll` exists to remove. Replacement: `for (const m of line.matchAll(CALL_RE)) { ... }` (keeps the `g` flag, regex untouched). lines_cut 2.

### F6-4 (native) scripts/check-i18n.mjs:280-281 - hand-rolled set difference

```js
const missingIds = [...refIds].filter((id) => !localeIds.has(id)).sort();
const extraIds = [...localeIds].filter((id) => !refIds.has(id)).sort();
```

Node 26 has the ES2025 Set methods (verified): `[...refIds.difference(localeIds)].sort()` / `[...localeIds.difference(refIds)].sort()`. Pure replacement, lines_cut 0.

### F6-5 (idiom) crates/xtask/Cargo.toml:11 - `[lib] path = "src/lib.rs"` restates the cargo default

```toml
[lib]
path = "src/lib.rs"
```

Cargo target auto-discovery finds `src/lib.rs` (and `src/main.rs`) without any manifest section; `path = "src/lib.rs"` is the literal default value. The whole `[lib]` block is dead config fighting cargo's own convention. lines_cut 3.

### F6-6 (idiom) locales - `#` comments where Fluent defines `###` / `##` levels (systemic)

Fluent's comment syntax is three-leveled with defined tool semantics (projectfluent.org guide, verified): `#` directly above a message (no blank line) *attaches to that message* as its translator note; `##` is a standalone group/section comment; `###` is a standalone resource (file) comment.

- Every `locales/de/*.ftl` file header is a `#` block, and in four of the six files it sits directly adjacent to the first message, so l10n tooling reads the whole file preamble as the translator note of that one message: `de/cli.ftl:1-5` -> attaches to `validate-ok`, `de/diagnostics.ftl:1-5` -> `severity-error`, `de/gui-common.ftl:1-7` -> `app-title`, `de/gui-settings.ftl:1-3` -> `settings-title`. Same in `en/gui-settings.ftl:1-2` and `en/gui-jobs.ftl:1-3` (that one standalone, but still file-scope content at `#` level). Correct level: `###`.
- Section comments covering a message block use `#` in `en/gui-common.ftl` (`# T9: app shell navigation ...` line 16, `# T9: shell-level IPC error codes` line 26, `# T9: first-run` line 49, `# D31: ...` line 3) where the correct level is `##` - exactly what `gui-jobs.ftl` already does (`## Batch header + run summary` etc.), so the tree is internally inconsistent about its own better pattern.
- Genuine single-message notes (e.g. `en/gui-common.ftl:34-41` on `identify-failed`) are correctly `#` and stay as they are.

Mechanical fix (`#` -> `###`/`##` on the affected lines); no message ids change, `MESSAGE_ID_RE` and the run.rs line parser are unaffected (both only match `id =` at column 0). lines_cut 0.

### F6-7 (idiom) locales - label/tooltip/hint sibling messages instead of Fluent attributes (systemic)

The catalogs encode every widget's facets as suffixed sibling messages: `browse-button` + `browse-button-tooltip` (`en/gui-common.ftl:23-24`), `settings-open-label`/`-tooltip`, `batch-profile-pick`/`-tooltip`, `batch-source-label`/`-hint`, `settings-*-label`/`-hint`/`-tooltip`, `firstrun-picker-label`/`-hint`, `jobs-*-label`/`-tooltip`, etc. The Fluent guide's recommended structure for "multiple translatable messages per one widget" (label + placeholder + aria-label + title/tooltip) is one message with attributes:

```ftl
browse-button = Browse...
    .tooltip = Choose the file with a file picker.
```

fluent-vue supports this natively (`$ta("browse-button")` returns the attribute object; the `v-t` directive binds attributes to the element directly), so the frontend consumption path exists. This keeps a widget's strings as one translation unit, which is the point of the attribute syntax.

Honest cost accounting: the migration touches the Vue components (call-site change from `$t('x-tooltip')` to `$ta('x').tooltip` or `v-t`), `check-i18n.mjs`'s `parseCatalogIds` + check-3 parity (the script already documents exactly this extension path at lines 113-118: "extend parseCatalogIds then, don't work around it"), and both locales in lockstep. **Exempt:** the four `close-abort-*` messages in gui-common.ftl must stay flat single-line messages (D31: consumed by src-tauri run.rs's line parser, pinned by unit test). State-dependent tooltips like `batch-run-tooltip-no-profile`/`-errors`/`-mkvmerge-missing`/`-run-active` select among *different* tooltips by app state and are fine either way (they could become `.tooltip-no-profile` attributes, but sibling messages are defensible there). lines_cut 0 (line-neutral in the catalogs).

## Routed (out of scope for findings)

- **locales/en/cli.ftl:20-21 (and de/cli.ftl:25-26)** - `dry-run-assignment =   rule ...` / `dry-run-output =   output: ...` visually intend a 2-space indent under `dry-run-file`, but the Fluent grammar consumes `blank_inline` after `=` (fluent.ebnf `Message` rule, verified), so any spec-conforming parser strips those spaces and the rendered CLI lines are *not* indented. If the alignment is intended, the Fluent way is a string-literal placeable: `dry-run-assignment = {"  "}rule { $rule } -> track { $track }`. Fixing it changes rendered output (and any snapshot pinning it), so this is a behavior/correctness question, not an idiomacy edit.

## Considered, deliberately not flagged

- `Result<String, String>` errors and hand-rolled `env::args` matching in xtask: the ecosystem norm for a dependency-light xtask binary (matklad pattern); clap/anyhow would be unearned deps.
- xtask lib+bin split with a single module: standard shape for integration-testing binary logic; not a yagni layer.
- `out.push_str(&format!(...))` in codegen.rs: clippy's `format_push_string` is an off-by-default restriction lint; `write!` would be a perf nicety, and performance is out of scope.
- contains-based assertions instead of insta snapshots in xtask tests: targeted asserts are fine; pulling insta into xtask is not owed.
- check-i18n.mjs being line-based instead of using `@fluent/syntax`: "No dependencies beyond Node itself" is a documented decision mirrored in run.rs; the parsing constraint is extensively documented in-file.
- Fluent terms (`-brand-name`) for the repeated Muxsmith/mkvmerge/MKVToolNix literals: textbook Fluent, but the payoff (spelling consistency of never-translated ASCII names across 2 locales) is near zero against real costs (check-i18n term-parity extension, run.rs line-lookup edge for gui-common.ftl). Not worth it at this scale.
- `$startedAt` (gui-jobs.ftl:49) camelCase vs the snake_case of every other placeable variable: consistency nit, driven by the frontend's param object; renaming ripples through TS for no structural gain. Prose note only.
- Known non-findings honored: version pins, TS 6.0.3, D26, matcher.rs regex recompilation, fake-mkvmerge copies, RECENT_PROFILES_CAP (none surfaced in this slice anyway).

## Slice verdict

Seven findings (four mechanical script modernizations on the pinned Node 26, one dead cargo config block, two systemic-but-cheap Fluent structure items), one routed behavior observation. No stdlib/native issues in the xtask Rust; the .ftl selector usage (plural categories, nested selectors, the column-0 `{ $fix }` dedent trick) is correct Fluent throughout.
