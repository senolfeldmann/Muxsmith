# Task 8 review verdict: Scripts / config / CI (Stream F)

Base `0b3149a` -> Head `11fb35d`.

## Spec Compliance

| # | Item | Verdict |
|---|---|---|
| 1 | `check-i18n.mjs:88` native `import.meta.dirname` | ✅ |
| 2 | `check-i18n.mjs:160` stdlib recursive `readdirSync` | ✅ (see Important #2 below for a latent edge case) |
| 3 | `check-i18n.mjs:191` idiom `matchAll` loop | ✅ |
| 4 | `check-i18n.mjs:280-281` native `Set.difference` | ✅ |
| 5 | `deny.toml:6,49` VERIFY-FIRST cargo-deny `version` field | ✅ verified independently (cargo-deny book: "no longer used... will now emit errors" - matches report's quote) |
| 6 | `ci.yml:137` VERIFY-FIRST cargo-deny-action `command` default | ✅ verified independently against `action.yml` at the pinned SHA `bb137d7a...` - `default: "check"` confirmed |
| 7 | `ci.yml:74` VERIFY-FIRST mise-action `cache` default | ✅ verified independently against `action.yml` at the pinned SHA `e6a8b397...` - `default: "true"` confirmed |
| 8 | `ci.yml:30` idiom `rustup toolchain install` | ⚠️ Applied exactly as the brief specified, but the brief's premise ("reads rust-toolchain.toml incl. components") is **false** - see Important #1 |
| 9 | `Cargo.toml:2` `resolver = "3"` | ✅ |
| 10 | `crates/xtask/Cargo.toml:11` delete `[lib]` | ✅ |
| 11 | `eslint.config.js:14` VERIFY-FIRST `tseslint.config()` -> `defineConfig()` | ✅ verified independently (typescript-eslint #10935 + docs confirm the `files`-under-`extends` and type-declaration differences; read the full file - no `extends:` key anywhere, composition is pure array-spread, so neither difference applies) |
| KEEP | `ci.yml:5` `v*` tag trigger | ✅ not in diff = untouched, survived |
| 12 | Full gate + **push must show all three CI legs green** | ❌ **Missing.** Report's own "Discipline" section states "never pushed"; "Real CI proof lands when this branch reaches a push/PR." The brief ties this requirement explicitly to "this task edits the workflow itself" - i.e. precisely because local reasoning cannot substitute for an actual CI run on this one task. Skipped. |

## Strengths

- All four VERIFY-FIRST items carry a named, checkable source (cargo-deny book pages, both actions' `action.yml` at the pinned SHA, the installed `typescript-eslint` package's own `.d.ts`) - I re-fetched all four independently and every one matches the report's quoted verdict exactly.
- `eslint.config.js` migration is genuinely safe: read the full post-change file, confirmed no `extends:` key exists anywhere (composition is pure array-spread), so neither documented `defineConfig`/`tseslint.config` behavior difference applies.
- `check-i18n.mjs:160` rewrite was verified empirically against the actual repo tree (byte-identical 17-file list, same order), not just trusted from the brief.
- `ci-10-pin-everything` respected: only no-op `with:` blocks were removed; no SHA pin, version pin, or runner image pin was touched.
- Commit hygiene: five scoped, independently-revertable commits; explicit per-path staging; diff-stat (12 insertions/35 deletions across 6 files) matches the report's self-review exactly.
- The commit-scope deviation (`chore(config)` vs. the repo's dominant per-component scope convention) was self-flagged as a controller-level question rather than silently imposed as new precedent.

## Issues

### Critical (Must Fix)

None.

### Important (Should Fix)

**#1. `ci.yml:30` - `rustup toolchain install` likely does not install the components CI needs (plan-mandated).**
`rust-toolchain.toml:5` declares `components = ["rustfmt", "clippy"]`. Verified against `rust-lang/rustup` issue #4216 ("`rust-toolchain.toml` is not used for `rustup toolchain install`", closed **not planned** by the rustup maintainers, still true through the latest 1.29.0 per the changelog): bare `rustup toolchain install` resolves the *channel* from the override file but does **not** install the `components`/`targets` also listed there - the reporter's exact case (a declared component silently missing after the command) matches this task's shape one-for-one. The very next two steps in the same job are `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` - both require `rustfmt`/`clippy` for the pinned `1.96.1` toolchain, which is not a version GitHub-hosted runners preinstall. This is very likely to break all three CI legs.

This is exactly the gap the brief's own "push must show all three CI legs green" requirement exists to catch (Important #2), and exactly why the local 9-part gate gave false confidence: the dev worktree's toolchain + components were already present via `mise install` *before* this line ever ran, so the local run cannot distinguish "correctly provisions a bare runner" from "no-ops because everything was already there."

The brief stated the "incl. components" premise as settled fact (this item was not one of the four VERIFY-FIRST-tagged ones), but `proc-07-verify-against-source` ("load-bearing tooling behavior is confirmed against source, never a brief's claim") arguably should have caught this regardless of tagging - it is exactly the class of claim that turned out wrong. Worth surfacing to the controller as a process refinement (verify load-bearing tool-behavior claims in a brief even when untagged), separate from fixing the line itself.

**#2. Required push/CI verification was skipped.**
Brief item: "push must show all three CI legs green (this task edits the workflow itself)." Report explicitly states "never pushed" and defers real proof to "when this branch reaches a push/PR." Given #1 above, this is not a formality - it is the one verification step that would have surfaced the likely breakage before the task was called done.

### Minor (Nice to Have)

**#3. `check-i18n.mjs:159-161` - recursive `readdirSync` includes directories, not just files (plan-mandated, currently inert).**
`readdirSync(SRC, { recursive: true })` without `withFileTypes` returns directory entries interleaved with file entries. The `.filter((f) => /\.(vue|ts)$/.test(f))` only excludes them today because no directory under `src/` happens to end in `.vue`/`.ts`. The old code excluded directories structurally (`entry.isDirectory()`); the new code excludes them by naming coincidence only. A future directory literally named e.g. `types.ts` would enter `sourceFiles` and crash the script with `EISDIR` at the `readFileSync` call. Plan-mandated exact snippet, empirically verified byte-identical against the current tree by the implementer - flagging as latent fragility, not a live bug.

## House dimension

- `ci-10-pin-everything`: not violated - checked every pinned SHA/version in the diff (`actions/checkout`, `Swatinem/rust-cache`, `EmbarkStudios/cargo-deny-action`, `jdx/mise-action`, `rust-toolchain.toml` channel); only the redundant `with:` parameter blocks were removed, no pin loosened or floated.
- `proc-07-verify-against-source`: satisfied for all four formally-tagged VERIFY-FIRST items (evidence re-checked independently, matches). **Not** satisfied for the untagged `rustup toolchain install` claim (Important #1) - candidate occurrence for the controller to weigh: does `proc-07` scope to "tagged VERIFY-FIRST items" or "any load-bearing external-tool-behavior claim in a brief"? This task is a concrete data point for the latter reading.
- Commit-scope convention: implementer's self-flagged deviation (`chore(config): ...` per this brief's literal instruction vs. the repo's dominant per-component scope, e.g. `ci: ...`, `fix(gui): ...`) is correctly raised as a controller decision, not silently adopted or silently ignored. No action needed from this review; noted for the controller's roll-up.
- No new pattern proposed by the implementer beyond what's already codified; none identified by this review beyond the proc-07-scope question above.

## Assessment

**Task quality:** Needs fixes

**Reasoning:** Ten of eleven line-items plus the KEEP are correct and independently re-verified against source; the four VERIFY-FIRST items are genuinely solid. But the `rustup toolchain install` change - applied exactly as the brief prescribed - is very likely to break `cargo fmt`/`cargo clippy` on all three CI legs because it does not install the `rustfmt`/`clippy` components `rust-toolchain.toml` declares (confirmed via the rustup project's own closed-not-planned issue), and the one step that would have caught this before merge (an actual push showing three green legs) was explicitly skipped.

---

## Re-review: fix wave 1 (11fb35d..308effc)

**Spec Compliance delta**

- Important #1 (rustup components): **Resolved.** `308effc` adds `rustup component add rustfmt clippy` in the same `run:` block directly after `rustup toolchain install` (`.github/workflows/ci.yml`, "Install pinned Rust toolchain" step). The two components named are exactly the two `rust-toolchain.toml:5` declares and exactly what the job's `cargo fmt --all --check` / `cargo clippy ... -D warnings` steps need; `component add` targets the active toolchain, which in the checked-out workspace is the pinned 1.96.1 the preceding line just installed, so the version pin stays single-sourced in `rust-toolchain.toml`. The comment states the true reason and cites rustup #4216 - matching my verified source, no overclaim. The commit changes nothing else: one file, 5 insertions/1 deletion, all within this step and its comment; no pin touched (`ci-10-pin-everything` still intact); the `v*` trigger untouched.
- Important #2 (push/CI proof): **Resolved as documented-open, honestly.** The appended "Review fix wave 1" report section states pushes are blocked this session and that the controller carries "first push must show 3 green CI legs" as an open verification item - accurate, and it additionally self-corrects the original item-8 note (the dropped comment's "including components" claim was wrong, not merely unnecessary at the call site). Per controller adjudication this item is not re-reviewable now; the deferred-proof note is truthful and traceable.

**New findings in the fix commit**

- Minor: the component list (`rustfmt clippy`) is now stated twice - `rust-toolchain.toml:5` and the new ci.yml line. If a component is ever added to `rust-toolchain.toml`, the ci.yml line must be updated in lockstep or the same silent-gap class recurs. A nudge in the existing comment ("keep in sync with rust-toolchain.toml") would close it; parsing the TOML in the workflow would be over-engineering. Not blocking.
- Nitpick, not a defect: on the `windows-2025` leg the multi-line `run:` executes under pwsh, which only checks `$LASTEXITCODE` after the last command - a failed `rustup toolchain install` could in principle be masked by a succeeding second line. In practice `component add` against a missing toolchain also fails, so the step still goes red; noted only so the pattern isn't copied into blocks where the last command can succeed independently.

**Task quality:** Approved

**Reasoning:** The fix is exactly scoped to the confirmed finding, covers both required components with a truthful source-cited comment, and touches nothing else; the remaining CI-legs proof is now an honestly documented, controller-carried open item rather than a silent gap.
