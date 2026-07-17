# Post-close fix: dedicated `nav-editor` tab key

Owner ruling (Şenol, 2026-07-17, resolving the plan-6 surface pass, per
`docs/process-journal/artifacts/plan-6-sdd/owner-surface-pass.md` section 5):
the editor's nav tab gets a dedicated Fluent key instead of reusing
`batch-profile-heading` ("Profile"/"Profil"). Executed directly on
`master` (no worktree; plan-6 worktrees are already gone).

## What changed

- `locales/en/gui-common.ftl` / `locales/de/gui-common.ftl`: added
  `nav-editor = Editor` beside `nav-batch`/`nav-jobs` in the existing T9
  nav comment block (en catalog comment updated: "two views" -> "three
  views", plus a note on why the dedicated key was added and what it
  replaces). `gui-editor.ftl` untouched (its 45-key budget is a separate
  owner ruling; this key is nav-family, not editor-surface).
- `src/App.vue`: the editor tab's `$t()` call re-pointed from
  `batch-profile-heading` to `nav-editor`; the stale Task-13 "reuses
  batch-profile-heading" comment replaced with one explaining the
  post-close change and confirming `batch-profile-heading` itself is
  untouched (still captions `BatchView`'s own section heading).
- `e2e/smoke.spec.ts`: one assertion added, not re-pointed (see below), in
  the T13 nav test ("the nav opens the editor; ..."), pinning the tab's
  accessible name via `toHaveAccessibleName(en("nav-editor"))`.

## Deviation from the brief, disclosed

The brief instructed re-pointing "the one e2e assertion that pins the
tab's accessible name." I searched `e2e/smoke.spec.ts` and
`e2e/catalogs.spec.ts` exhaustively (grep for `batch-profile-heading`,
`nav-editor`, literal `"Profile"`/`"Profil"`, `toHaveAccessibleName`,
`toHaveText`) and confirmed no such assertion existed before this change
- the T13 nav test only did `page.getByTestId("nav-editor").click()` with
no name check, and `task-13-verdict.md`/`owner-surface-pass.md` (which
discuss this exact key-reuse debate at length) don't mention one either.
So instead of re-pointing, I added the assertion described above,
consistent with the brief's own fallback ("additive/re-point only; no
spec deleted or weakened").

## TDD: red then green

1. Added the catalog keys first (both locales) - no visible-behavior
   change yet, `App.vue` still bound to `batch-profile-heading`.
2. Added the new assertion (`toHaveAccessibleName(en("nav-editor"))`) and
   ran it scoped (`playwright test --grep "the nav opens the editor"`):
   **RED** - `Expected: "Editor"`, `Received: "Profile"`.
3. Re-pointed `App.vue`'s `$t()` call and comment.
4. Reran the same scoped test: **GREEN** (1 passed).

## Gate (foreground, all nine parts green)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | ok |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | ok |
| 3 | `cargo test --workspace` | ok (81 + 2 codegen tests passed) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | ok |
| 5 | `cargo deny check` | ok (advisories/bans/licenses/sources ok) |
| 6 | `pnpm lint` | ok |
| 7 | `pnpm build` | ok |
| 8 | `pnpm check:i18n` | ok (17 pre-existing unused-warning ids, unrelated to `nav-editor`; parity ok) |
| 9 | `pnpm test:e2e` | ok (31 passed, incl. the new/flipped assertion) |

## Files touched

- `locales/en/gui-common.ftl`
- `locales/de/gui-common.ftl`
- `src/App.vue`
- `e2e/smoke.spec.ts`

## Commit

`2591cd4a798d09b6d8337187f64aad7fd8e2731f` - "gui: dedicated nav-editor
tab key (owner surface-pass ruling 2026-07-17)", unsigned
(`commit.gpgsign=false`), `Co-Authored-By: Claude Sonnet
<noreply@anthropic.com>` trailer, explicit staging of the four files
above. Not pushed.
