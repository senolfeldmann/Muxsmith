# Task 8 report: Scripts / config / CI (Stream F)

Branch `plan-5.6-f`, worktree `.worktrees/plan-5.6-f`. Setup: `mise install`
(all pinned tools already present) + `pnpm install --frozen-lockfile` (222
packages, lockfile unchanged).

## Per-item results

### 1. `check-i18n.mjs:88` (native) - DONE
`const ROOT = resolve(import.meta.dirname, "..");`, dropped the `node:url`
import line and removed `dirname` from the `node:path` import (its only use
site was this line). `import.meta.dirname` verified present in Node 26.5.0
(the repo's pinned version, confirmed with `mise exec -- node --version`).

### 2. `check-i18n.mjs:160` (stdlib) - DONE
`walkSourceFiles` (a hand-rolled recursive directory walker) replaced by
`readdirSync(SRC, { recursive: true }).filter((f) => /\.(vue|ts)$/.test(f)).map((f) => join(SRC, f))`.
Re-verified empirically (not just trusted the brief's claim): ran both
implementations against this repo's actual `src/` tree on the pinned Node
26.5.0 and diffed the resulting file lists - byte-identical, 17 files each,
same order.

### 3. `check-i18n.mjs:191` (idiom) - DONE
`for (const m of line.matchAll(CALL_RE))` replaces the manual
`CALL_RE.lastIndex = 0; let m; while ((m = CALL_RE.exec(line)) !== null)`
loop. `matchAll` internally clones the regex per call, so no `lastIndex`
management is needed; `CALL_RE` already carries the required `g` flag.

### 4. `check-i18n.mjs:280-281` (native) - DONE
`refIds.difference(localeIds)` / `localeIds.difference(refIds)` replace the
`[...set].filter(...)` pairs. Set methods (ES2025) are available on Node 26.

### 5. `deny.toml:6,49` (yagni) - VERIFY-FIRST: CONFIRMED, applied
Fetched the current cargo-deny book pages for both `checks/advisories/cfg.html`
and `checks/licenses/cfg.html`: both state verbatim "The version field is (at
the time of this writing) no longer used, the following fields have been
removed and will now emit errors." Both `version = 2` lines deleted.
`cargo deny check` output unchanged (advisories/bans/licenses/sources all ok;
same pre-existing `multiple-versions = "warn"` duplicate-crate warnings, none
mentioning `version` or deprecation).

### 6. `ci.yml:137` (yagni) - VERIFY-FIRST: CONFIRMED, applied
Fetched `EmbarkStudios/cargo-deny-action`'s actual `action.yml` at the pinned
SHA `bb137d7af7e4fb67e5f82a49c4fce4fad40782fe` via `gh api
repos/EmbarkStudios/cargo-deny-action/contents/action.yml?ref=<sha>`: the
`command` input's `default` is `"check"`. The `with:` block set exactly
`command: check` and nothing else, so it was pure restatement; deleted,
leaving the bare `uses:` line.

### 7. `ci.yml:74` (yagni) - VERIFY-FIRST: CONFIRMED, applied
Fetched `jdx/mise-action`'s actual `action.yml` at the pinned SHA
`e6a8b3978addb5a52f2b4cd9d91eafa7f0ab959d` the same way: the `cache` input's
`default` is `"true"`. The `with:` block set exactly `cache: true`; deleted.

### 8. `ci.yml:30` (idiom) - DONE
`run: rustup toolchain install` (no arguments) replaces `run: rustup show`.
Dropped the two-line comment above it ("Toolchain comes from
rust-toolchain.toml ...; rustup on the runner auto-installs it.") in full -
its claim was the stale premise being fixed (post-1.28, `rustup show` does
not install; the job only worked via the invisible toolchain-proxy
auto-install triggered by the first `cargo` invocation afterward), and the
new command's behavior (reads `rust-toolchain.toml` including components) is
rustup's own well-documented default, not something this call site needs to
re-explain.
KEPT untouched: the `v*` tag trigger at line 5 (deliberate Plan-6 scaffold) -
verified still present after all edits.

### 9. `Cargo.toml:2` (idiom) - DONE
`resolver = "3"` replaces `resolver = "2"`. This is a virtual workspace (no
root `[package]`), so there is no package edition for Cargo to derive the
resolver default from even though `edition = "2024"` is set in
`[workspace.package]` - the key must be stated explicitly. Verified
behavior-neutral: `Cargo.lock` is untouched (`git status` on it stays clean
across the whole task), and the full Rust gate (fmt/clippy/test/doc/deny)
passes identically.

### 10. `crates/xtask/Cargo.toml:11` (idiom+yagni) - DONE
Deleted the `[lib]` section (`path = "src/lib.rs"`). Verified this restates
Cargo's auto-discovery default by inspecting the actual crate layout:
`crates/xtask/src/lib.rs` and `src/main.rs` both exist at the default paths
Cargo's target auto-discovery expects for a package named `xtask`. Confirmed
behavior-neutral via `cargo test --workspace`: both the xtask lib unittest
binary and the xtask bin unittest binary still build and run (0 tests each,
as before) plus the `tests/codegen.rs` integration binary (2 tests, both
pass), unchanged from pre-edit.

### 11. `eslint.config.js:14` (idiom) - VERIFY-FIRST: CONFIRMED, applied
Checked the actual installed package rather than the docs alone (per
`proc-07-verify-against-source`): grepped
`node_modules/.pnpm/typescript-eslint@8.63.0.../typescript-eslint/dist/config-helper.d.ts`
and found `config()` carries `@deprecated ESLint core now provides this
functionality via defineConfig(), which we now recommend instead`, linking
`typescript-eslint.io/packages/typescript-eslint/#config-deprecated`
(typescript-eslint issue #10935). Confirmed ESLint 10.6.0 (the pinned
version) exports `defineConfig` from the `eslint/config` subpath
(`node_modules/.pnpm/eslint@10.6.0/.../eslint/package.json` exports map +
`lib/config-api.js`).
Fetched the typescript-eslint docs page for the two documented behavior
differences: (a) `files` scoping semantics differ when both a base config
and an `extends:` entry set `files` (override vs. intersect), (b) minor
type-declaration mismatches that "do not indicate a runtime problem." Neither
applies here - this config never uses the `extends:` convenience key at all,
composing its config array purely via array spreads (`...tseslint.configs.recommended`,
`...pluginVue.configs["flat/recommended"]`).
Change: added `import { defineConfig } from "eslint/config";`, changed
`export default tseslint.config(` to `export default defineConfig(`; the
argument list and every entry are untouched. `pnpm lint` (0 findings, exit 0)
and `pnpm build` (`vue-tsc --noEmit && vite build`, exit 0) both pass, so no
lurking type-declaration friction surfaced.

### KEEP - ci.yml:5 `v*` tag trigger
Untouched. Confirmed present, unchanged, at line 5 after every edit pass.

## Gate results (all nine parts, run foreground from the worktree root)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | pass, no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass, clean build, no warnings |
| 3 | `cargo test --workspace` | pass, every `test result: ok` line shows `0 failed` (36 test binaries/doc-test groups, several hundred tests total) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | pass, doc generated with no warnings |
| 5 | `cargo deny check` | pass: "advisories ok, bans ok, licenses ok, sources ok"; only pre-existing `multiple-versions = "warn"` duplicate-crate notices, unrelated to this task |
| 6 | `pnpm lint` | pass, exit 0, no findings |
| 7 | `pnpm build` | pass, `vue-tsc --noEmit && vite build` succeeded |
| 8 | `pnpm check:i18n` | pass; output diffed byte-for-byte against a `git stash`-recovered pre-edit run - **identical** |
| 9 | `pnpm test:e2e` | pass, 6/6 Playwright specs green (had to `pnpm exec playwright install chromium` first; `--with-deps` failed locally on missing sudo, irrelevant to CI which runs as root) |

`ci.yml` itself cannot be exercised locally; validated by parsing it with
Python's `yaml.safe_load` (structurally valid, both `test` and `deny` jobs
present) and by reasoning against the two pinned actions' real source (items
6/7 above) rather than from memory or the brief's claim, per
`proc-07-verify-against-source`. Real CI proof lands when this branch
reaches a push/PR.

## Files changed

- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/scripts/check-i18n.mjs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/deny.toml`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/.github/workflows/ci.yml`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/Cargo.toml`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/crates/xtask/Cargo.toml`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-f/eslint.config.js`

`Cargo.lock` and `pnpm-lock.yaml` are untouched (checked with `git status`
throughout).

## Commits (unsigned, `-c commit.gpgsign=false`)

1. `a0a0b4c` `chore(ci): drop no-op action with: blocks, fix rustup toolchain install`
2. `784a8f6` `chore(config): drop unused cargo-deny version field`
3. `04f011a` `chore(config): explicit workspace resolver, drop redundant xtask [lib]`
4. `df04428` `chore(config): migrate off deprecated tseslint.config to eslint/config's defineConfig`
5. `11fb35d` `chore(config): idiomatic Node stdlib/native fixes in check-i18n.mjs`

Grouped by file/rationale rather than one mega-commit, each independently
revertable; explicit `git add <path>` staging throughout, no `git add -A`.

## Self-review

- **Completeness**: all 11 items + the KEEP note addressed; nothing deferred
  or skipped.
- **Quality**: every VERIFY-FIRST item checked against the actual
  authoritative source (book page text, action.yml at the pinned SHA, or the
  installed package's own type declarations) rather than trusting the
  brief's framing or training-data memory - all four confirmed, none
  refuted, so no "refuted at implementation" cases to record.
- **Discipline**: nine-part gate run foreground before committing; commits
  staged explicitly per file group; no `--no-verify`, no signing bypass
  beyond the standing `-c commit.gpgsign=false` house rule; never pushed.
- **Pristine output**: `git status --short` is clean after the final
  commit; `git diff --stat` against the pre-task commit shows exactly the
  six in-scope files, net -23 lines (35 deletions, 12 insertions), no
  incidental churn.

## Surfaced patterns / deviations

- **Commit scope convention**: this task's brief mandates `chore(ci): ...` /
  `chore(config): ...`, which I followed literally for all five commits.
  The repo's dominant historical convention on these same files is bare
  `ci: ...`, `feat(xtask): ...`, `fix(gui): ...` (see `git log -- deny.toml
  Cargo.toml crates/xtask/Cargo.toml .github/workflows/`) - i.e., scope
  named after the *component*, not a generic `config`. Flagging this as an
  intentional, task-scoped divergence rather than a new house pattern to
  adopt elsewhere; worth a controller decision if Plan 5.6's other five
  streams use the same `chore(...)` convention consistently, in which case
  it may be worth codifying in `process-conventions.yaml` as the pre-1.0
  idiomacy-wave commit style specifically (distinct from the ordinary
  feature-commit convention already in use).
- **`rustup toolchain install` comment**: dropped the stale two-line comment
  in full rather than rewriting it to describe the new command's behavior,
  since that behavior (reads `rust-toolchain.toml` including components) is
  rustup's own well-documented default and didn't seem to need restating at
  the call site. Flagging in case the controller wants a one-line
  replacement comment instead of none.
- No new house-knowledge pattern is proposed; `ci-10-pin-everything` is
  unaffected (all action SHA pins and runner image pins are untouched -
  only the `with:` parameter blocks that duplicated documented defaults were
  removed).

## Review fix wave 1 (Important, confirmed by controller)

**Finding**: the new bare `rustup toolchain install` installs the pinned
channel from `rust-toolchain.toml` but does NOT install its declared
components (`rustfmt`, `clippy`) - rustup issue #4216, closed not-planned,
still true in current rustup. The subsequent gate steps (`cargo fmt --all
--check`, `cargo clippy -D warnings`) need both; GitHub-hosted runners do
not preinstall them for a pinned 1.96.1 toolchain, and proxy auto-install
never adds components to an already-installed toolchain. The local gate
could not catch this (the worktree toolchain already had the components
via mise before that line was exercised); the brief's premise for item 8
was itself incomplete on this point. This also amends the item-8 note
above about the dropped comment: the dropped claim ("including components")
was not merely unnecessary at the call site, it was wrong.

**Fix**: `rustup component add rustfmt clippy` added in the same `run:`
block directly after `rustup toolchain install`, with a one-line comment
citing rustup #4216. `component add` reads the active toolchain from
`rust-toolchain.toml` and is idempotent, so the version pin stays
single-sourced there.

**Commit**: `308effc` `chore(ci): install rust-toolchain.toml components
explicitly (rustup #4216)` (new commit, not an amend; unsigned; explicit
staging; not pushed).

**Re-verification**: YAML validity re-checked (`yaml.safe_load` parses;
`test`/`deny` jobs intact; the step's run block contains exactly the two
rustup lines). The Rust and pnpm gate parts are untouched by a ci.yml-only
edit and were not re-run. On-runner proof (3 green CI legs) is deliberately
deferred: pushes are blocked this session; the controller carries "first
push must show 3 green CI legs" as an open verification item.
