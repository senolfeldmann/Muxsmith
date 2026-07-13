### Task 8: Scripts / config / CI (Stream F)

**Files:**
- Modify: `scripts/check-i18n.mjs`, `deny.toml`, `.github/workflows/ci.yml`, `Cargo.toml` (workspace root), `crates/xtask/Cargo.toml`, `eslint.config.js`

**Interfaces:** none. Four items carry a VERIFY-FIRST step; a refuted claim = keep + record, valid completion.

- [ ] `check-i18n.mjs:160` **stdlib** - walkSourceFiles becomes `readdirSync(SRC, { recursive: true }).filter((f) => /\.(vue|ts)$/.test(f)).map((f) => join(SRC, f))` (verified identical file set on pinned Node 26.5.0; regex end-anchored so relative-path test equals entry.name test).
- [ ] `check-i18n.mjs:191` **idiom** - `for (const m of line.matchAll(CALL_RE))`; the manual lastIndex reset and `let m` go.
- [ ] `check-i18n.mjs:88` **native** - `const ROOT = resolve(import.meta.dirname, "..");` drop the node:url import line.
- [ ] `check-i18n.mjs:280-281` **native** - `refIds.difference(localeIds)` / `localeIds.difference(refIds)` (ES2025 Set methods, verified on Node 26).
- [ ] `deny.toml:6, :49` **yagni** - VERIFY current cargo-deny docs state the version field "is no longer used"; if confirmed delete both `version = 2` lines, else keep + record refuted.
- [ ] `ci.yml:137` **yagni** - VERIFY cargo-deny-action's action.yml defaults command=check; if confirmed delete the `with:` block, leaving the bare `uses:`.
- [ ] `ci.yml:74` **yagni** - VERIFY jdx/mise-action's cache default=true; if confirmed delete the `with:` block.
- [ ] `ci.yml:30` **idiom** - `run: rustup toolchain install` (no arguments; reads rust-toolchain.toml incl. components) replaces `rustup show` (post-1.28 it no longer installs; the job only works via invisible proxy auto-install inside the first cargo call). Drop the stale "rustup on the runner auto-installs it" comment.
- [ ] `Cargo.toml:2` **idiom** - `resolver = "3"` (edition-2024 default; explicit key required for a virtual workspace; behavior-neutral with the committed Cargo.lock).
- [ ] `crates/xtask/Cargo.toml:11` **idiom+yagni (double-tagged, one item)** - delete the `[lib]` section (restates cargo target auto-discovery; unlike muxsmith-cli's `[[bin]]` rename or src-tauri's crate-type, it carries no information).
- [ ] `eslint.config.js:14` **idiom** - VERIFY the tseslint.config() deprecation (typescript-eslint #10935); if confirmed: `import { defineConfig } from "eslint/config"; export default defineConfig(...same entries...)` (pinned typescript-eslint 8.63.0 / ESLint 10.6.0; none of the composed configs hit the two documented behavior differences).
- [ ] KEPT (do not touch, ledger note only): ci.yml:5 `v*` tag trigger - deliberate scaffold, Plan-6 packaging consumes it.
- [ ] Full gate; push must show all three CI legs green (this task edits the workflow itself); commits `chore(ci): ...` / `chore(config): ...`.

