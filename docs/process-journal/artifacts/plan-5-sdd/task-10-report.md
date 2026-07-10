# Task 10 report: Batch view (spec 8.2 view 2, minus apply-suggestion per D22)

Branch `plan5-t10` (worktree `.worktrees/plan5-t10`), base `65d7065`.

## What was implemented

- **`src/views/BatchView.vue`** (replaces the T9 placeholder): profile picker
  (`plugin-dialog`, `.yaml`/`.yml` filter, filter name localized via
  `useFluent()` since it's a JS-context string the template lint can't
  reach) + MRU recents list; picking (dialog or a recents entry) moves the
  path to the front of `recent_profiles` and runs `validate_profile`.
  Source/output directory pickers, prefilled from `dir_memory[profile]` on
  profile switch and persisted back both on `@change` (manual typing,
  blur-committed) and on picker selection. Dry-run button (`aria-busy`,
  disabled while any IPC call is in flight) renders the full
  `batch_document`. `role="status"` summary line with error/warning/info
  counts across config + batch + every file's diagnostics. Run button emits
  `start-run`, disabled with a Fluent tooltip when no validated report
  exists yet, mkvmerge is missing, or any diagnostic is error-severity.
- **`src/components/ResolutionTable.vue`**: one `batch_document.files[]`
  entry -- `<table>` with `<caption>` (source + identifier + output) and
  `<th scope="col">` headers, rule -> resolved track rows, plus that file's
  own diagnostics via `DiagnosticsPanel`. No plan -> a status paragraph
  instead of an empty table body.
- **`src/components/DiagnosticsPanel.vue`**: shared diagnostic-list renderer
  (severity dot, decorative `aria-hidden`, + localized severity word +
  `code`/`params` through `diagnostics.ftl`), reused for the top-level
  config+batch diagnostics and once per file inside `ResolutionTable`.
- **`src/components/SuggestionCard.vue`**: `config_path` header, YAML
  fragment in `<pre><code>`, Copy button (`@tauri-apps/plugin-clipboard-manager`),
  transient `role="status"` "Copied" confirmation. Show + copy only --
  `edit` (the structured/applicable form) is never read, the profile is
  never touched (D22).
- **`locales/en/gui-batch.ftl`**: every new string; `browse-button`/
  `browse-button-tooltip` reused from `gui-common.ftl` rather than
  duplicated. Verified by parsing all three loaded catalogs with
  `@fluent/bundle` directly and formatting every new message with sample
  args -- zero parse errors, all render correctly (build/lint never
  exercise the FTL parser, only the browser runtime does, so this was a
  manual check, not gate-covered).
- **`src/App.vue`**: added `pendingRun` ref, `onStartRun` handler
  (stores the payload, switches to Jobs), wired `@start-run` on `BatchView`
  and `:pending-run`/`@consumed` on `JobsView`.
- **`src/ipc.ts`**: gave `ReportFile.plan` and `ReportDocument.suggestions`
  real types (`FilePlan`/`PlanAssignment`/`Suggestion`, narrowed to the
  fields the Batch view actually renders, each documented against the
  unmirrored Rust fields -- same "opaque until a consumer exists" discipline
  the file already used). Added `RunRequest` (the wave-5 App/JobsView
  handoff shape, distinct from `startRun()`'s own optional-field
  parameters).
- **`src/views/JobsView.vue`**: **not touched.** `vue-tsc --noEmit` (via
  `pnpm build`) passed cleanly with the placeholder as-is -- Vue's
  fallthrough handling of the undeclared `pending-run` prop / `consumed`
  listener on an otherwise-untyped placeholder component raised no type
  error, so the brief's conditional "add the minimal declaration if
  needed" did not trigger.

## ipc.ts param-name verification (required by the brief)

Checked every command's JS-side invoke keys against the actual Rust
parameter names in `src-tauri/src/lib.rs`/`run.rs` (no
`#[tauri::command(rename_all = ...)]` override exists anywhere in either
file, so Tauri's default camelCase-JS -> snake_case-Rust conversion applies
uniformly). Every existing wrapper is already correct, including the one
multi-word case: `getJobLog(runId, index)` invokes with `{ runId, index }`,
which Tauri converts to the Rust `get_job_log(run_id, index)` signature.
**No fixes were needed.** (`validate_profile(path)`, `dry_run(profile,
source, output)`, `identify(file)`, `detect_mkvmerge()`, `get_settings()`,
`set_settings(settings)`, `start_run(profile, source, output, jobs)`,
`cancel_run()`, `cancel_job(index)`, `list_runs()` all single-word params or
already-correct camelCase -- verified individually against the
`#[tauri::command]` signatures.)

## Notable decisions

1. **`jobs` in the `start-run` payload**: this view has no jobs-count
   control (outside the brief's scope). `start_run`'s own Rust default for
   `jobs: None` is hardcoded `1` (sequential) -- it does **not** read
   `AppSettings.default_jobs` (confirmed by reading `run.rs::start_run`
   directly). Passing `null` would silently downgrade every run to
   sequential regardless of the user's configured default. Since this view
   already loads `AppSettings` for recents/dir-memory, it passes
   `settings.default_jobs` instead of `null`.
2. **Diagnostics scope**: the brief's step 2 says "config + per-file
   diagnostics"; `batch_diagnostics` (cross-file/runtime facts like
   `DuplicateIdentifier`, which can be error-severity) is not named
   explicitly. Rendered it anyway, grouped with `config_diagnostics` at the
   top: omitting a diagnostic category that can gate Run via the
   errors-exist check while giving the user no way to see *why* would be a
   functional gap, not a minimal-scope reading.
3. **Run enablement**: does not require an explicit prior dry-run click --
   `validate_profile` already populates a report on profile pick, and
   `start_run` re-plans/dry-runs internally regardless (spec 5.5), same as
   the CLI's `run` command. Only the two brief-named conditions
   (error-severity diagnostic present, mkvmerge missing) plus the
   functional precondition of having *a* report gate the button.
4. **CLI parity for two format choices**: unmatched track cells render the
   ASCII `"-"` placeholder (not a Fluent key) and the suggestion card header
   omits `resolves`/`DiagCode` -- both mirror
   `muxsmith-cli/src/commands/mod.rs::print_batch_human`'s identical choices
   exactly, per spec 7's "CLI and GUI render the same report structures."
5. **`track_kind`** (video/audio/subtitles/buttons) is rendered verbatim,
   treated as passthrough mkvmerge vocabulary (spec 8.4's `detail`-param
   exception), not app-authored copy needing a Fluent key.

## ESLint `no-raw-text` gotcha (carry-forward for later view work)

`@intlify/eslint-plugin-vue-i18n`'s `no-raw-text` flags: (a) any bare
punctuation/text as a template-literal *sibling* of a mustache (e.g.
`{{ $t(...) }}:` -- the trailing `:` is its own `VText` node and gets
flagged even though it's a single character), and (b) a **top-level**
ternary inside `{{ }}` whose `consequent`/`alternate` branch is itself a
string/template literal with no substitutions (e.g.
`{{ cond ? "-" : \`${x}\` }}` flags the `"-"` branch). Neither is caught by
wrapping the *whole* ternary or concatenation in a function/computed call
first (`{{ formatX(...) }}`) -- the rule only inspects the container's
top-level expression node, so a `CallExpression` at that position is exempt
regardless of what literals live inside its arguments or the called
function's body. Fixed here by (1) composing the diagnostic "severity:
message" line as one Fluent message with two interpolated, independently-
localized substrings instead of concatenating three separate `$t()` calls
in the template, and (2) moving the resolved-track ternary into a
`<script setup>` helper function. Worth keeping in mind for Plan 6's editor
grid, which will have plenty of conditional cell rendering.

## Gate results

All six green, foreground, no new dependencies:
- `cargo fmt --all --check` -- clean
- `cargo clippy --workspace --all-targets -- -D warnings` -- clean
- `cargo test --workspace` -- 72+ suites, 0 failures (includes
  `catalog_completeness.rs`, unaffected -- no new `DiagCode`s)
- `cargo deny check` -- advisories/bans/licenses/sources all ok
- `mise exec -- pnpm lint` -- clean
- `mise exec -- pnpm build` (`vue-tsc --noEmit && vite build`) -- clean

## D22 self-check

Grepped the whole diff for profile-mutation surface: no `save`/`write`
calls against any `.yaml`/`.yml` path anywhere, `SuggestionCard` reads only
`config_path`/`yaml_fragment`, `edit` is imported into the type but never
dereferenced. Confirmed show + copy only.

## Concerns / carry-forwards

- No live GUI smoke was run (no display/mkvmerge in this environment); T12
  owns the Playwright smoke test per the brief. Verification here rests on
  full type-checking, lint, the manual FTL parse/render check above, and
  careful manual tracing of every Rust command/report-document shape
  against source.
- `jobs: settings.default_jobs` (see decision 1 above) is a judgment call
  worth the controller's explicit sign-off if T11/Plan 6 later add a
  per-run jobs override control -- that control should take priority over
  this fallback.

---

# Fix round 1 (commit 638eda2, on top of 1b76fd6)

## 1. IMPORTANT: client-side recents cap (BatchView.vue)

Chose the truncate-in-the-mutation option over re-reading `getSettings()`
after `setSettings()`: one fewer IPC round trip per profile pick, and
`settings.value` is set to exactly the object that was handed to `save()`
post-truncation, so client and disk agree deterministically (a re-read
would also work but pays a round trip to learn what the client can compute
itself). Added a local `RECENT_PROFILES_CAP = 10` with a doc comment naming
the Rust constant it mirrors -- referenced as
`src-tauri/src/settings.rs::RECENT_PROFILES_CAP` (symbol, not line number:
the coordinator's message said settings.rs:27, the constant actually sits
at line 29, which is exactly why a line-number reference would rot). The
mutation now `.slice(0, RECENT_PROFILES_CAP)`s the newest-first list,
matching `save()`'s truncate-from-the-tail semantics.

Note: `dir_memory` has no equivalent Rust-side cap (unbounded by design in
settings.rs), so no parallel client cap exists or is needed there.

## 2. Integration fix: both views stay mounted (App.vue)

`v-if`/`v-else` -> `v-show` on both `BatchView` and `JobsView`. Verified
against the stated constraints:

- **First-run gate unaffected**: the outer `v-if="checking"` /
  `v-else-if="blockedError"` / `v-else` chain is untouched; the shell
  (nav + main + both views) still mounts only after detection succeeds.
- **aria-current**: keys on `activeView` in the nav, which is unchanged by
  this fix; still tracks the active tab.
- **Hidden view and AT/focus**: `v-show` renders `display: none`, which
  removes the subtree from the accessibility tree and from tab order --
  no focus trap, nothing exposed to AT. (Confirmed this is the mechanism
  the coordinator's message itself names as sufficient.)
- **Behavior consequence for the wave-5 handoff**: with JobsView now
  eagerly mounted, `onStartRun`'s prop update reaches an already-mounted
  JobsView (a `watch` on `pending-run` in T11's implementation) instead of
  arriving as an initial prop at mount time. Both arrival paths are within
  the declared contract (prop + `consumed` emit); flagging so T11's
  implementer/reviewer confirms their watcher handles the
  set-while-mounted path (it must anyway for any second run started from
  Batch after the first).
- BatchView's eager mount just runs its `getSettings()` prefetch at
  startup instead of first tab visit -- negligible, as the coordinator
  noted.

## Gate results (fix round)

All six green, foreground: `cargo fmt --all --check` clean; `cargo clippy
--workspace --all-targets -- -D warnings` clean; `cargo test --workspace`
364 passed / 0 failed; `cargo deny check` all ok; `mise exec -- pnpm lint`
clean; `mise exec -- pnpm build` (vue-tsc + vite) clean.

## Residual concerns

- T11 must handle `pending-run` arriving via prop *update* on an
  already-mounted JobsView (see above) -- with v-show this is now the ONLY
  path, not just the second-run path.
- The `jobs: settings.default_jobs` decision from the base commit stands
  unreviewed-in-detail (flagged in the original report).
