<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-4  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01E32K8fbFckHJ6ViXjUo5fR
  agent_id:           a958b7214433df250
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a958b7214433df250.jsonl
  dispatch_desc:      Review Task 4 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T12:11:05.423Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

All six brief steps are present and match; no missing or extra scope beyond one justified addition (`pnpm-workspace.yaml`). Details below.

### Strengths

- **`deny.toml` advisory work is genuinely rigorous, not rubber-stamped.** I independently verified all 18 RUSTSEC IDs against rustsec.org: every ID matches the claimed crate, and the "unmaintained/informational" vs. "actual vulnerability" split is correct (`RUSTSEC-2025-0075/0080/0081/0098/0100` = unic-* unmaintained; `RUSTSEC-2024-0370/0411..0420` = gtk-rs GTK3 unmaintained; `RUSTSEC-2026-0194/0195` = quick-xml DoS, confirmed real advisories with the exact titles claimed). Cross-referencing the (excluded-from-diff-but-stat-visible) `Cargo.lock` diff at `/home/senol/Git/Muxsmith/.worktrees/plan5-t4/.superpowers/sdd/review-c822a17..fe3d2d5.diff:3026-3041,4824-4833,2861-2872` confirms the reachability claim precisely: `quick-xml 0.39.4` (vulnerable) has exactly one consumer, `wayland-scanner` (a build-time codegen crate); `quick-xml 0.41.0` (patched) has exactly one consumer, `plist`. The build-time-only claim is not just plausible, it's exactly right.
- **License allow-list additions all verified correct** against crates.io metadata: `cssparser`→MPL-2.0, `target-lexicon`→Apache-2.0 WITH LLVM-exception, `clipboard-win`→BSL-1.0, `foldhash`→Zlib, `alloc-no-stdlib`→BSD-3-Clause. All exact matches.
- **TypeScript 6.0.3 downgrade is well-founded.** Confirmed via npm registry: `typescript@latest` is genuinely `7.0.2`, and `typescript-eslint@8.63.0` (itself genuinely the current `latest` dist-tag, so no newer release could have avoided the conflict) declares `peerDependencies.typescript: ">=4.8.4 <6.1.0"`. `6.0.3` is exact-pinned (no `^`), documented in the report, and satisfies both that ceiling and `vue-tsc@3.3.7`'s `>=5.0.0` floor.
- **CI additions verified byte-exact against authoritative sources**: the `jdx/mise-action` SHA `e6a8b3978addb5a52f2b4cd9d91eafa7f0ab959d` is confirmed via GitHub's API to be precisely the `v4.2.0` tag commit; the apt package list matches the current `v2.tauri.app/start/prerequisites/` page verbatim. CI style (SHA + version comment, Linux-gated apt step, ordering: apt → mise-action → `pnpm install --frozen-lockfile` → existing cargo steps → `pnpm lint`/`pnpm build`) is preserved exactly as specified.
- **`pnpm-workspace.yaml`/`allowBuilds` is legitimate, not scope creep.** `allowBuilds` is pnpm v11's actual replacement for the legacy `onlyBuiltDependencies` mechanism (confirmed via pnpm docs/changelog), and `fluent-vue@3.8.2` genuinely depends on `vue-demi`, which genuinely ships a `postinstall` script. Without the approval, `--frozen-lockfile` CI installs would block on it.
- **Caught and fixed a real bug via empirical testing, not inspection**: the `eslint.config.js` block-order issue (typescript-eslint's global parser override clobbering `vue-eslint-parser` for `.vue` files) was found by actually running the Step 2 probe, exactly the kind of verification-over-assertion practice the brief demands for D27.
- **`BUILDING.md`'s Fedora line is genuinely copied verbatim** from the T0 plan doc (`git show c822a17:docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md:48`) — no drift between the three places (T0 plan, CI, BUILDING.md) that list Linux prerequisites.
- Icon pruning to exactly the 5 files `tauri.conf.json` references (dropping mobile/Store variants) is correct scope discipline given the stated desktop-only target.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)

1. **`eslint` is pinned to a ~2-year-stale version with no annotation.** `package.json` (`.../review-c822a17..fe3d2d5.diff:6130`) pins `"eslint": "9.9.1"`, published **2024-08-23**. The current `9.x` line is at `9.39.4` (published 2026-03-06, four months before this diff's date). I verified this via `npm view eslint time`. Every other dependency in this same file (`vue`, `vite`, `@vitejs/plugin-vue`, `typescript-eslint`, `eslint-plugin-vue`, `vue-tsc`, all `@tauri-apps/*`, `fluent-vue`, `@fluent/bundle`, `@intlify/eslint-plugin-vue-i18n`) matches the exact current npm `latest` tag, so this one value stands out as an outlier, most plausibly typed from stale training-data memory rather than resolved against the registry (the report's own framing — "version choices are decisions, not defaults" — is applied rigorously to TypeScript but not here, and not disclosed). It didn't surface as a conflict because `eslint-plugin-vue`/`@intlify/eslint-plugin-vue-i18n`'s peer ranges (`^8.57.0 || ^9.0.0 || ^10.0.0`, `^8.0.0 || ^9.0.0-0 || ^10.0.0`) are wide enough to swallow it silently. Not a functional break, but a config-hygiene defect in a task whose explicit charter is exact, deliberate pins. Bump to the current `9.x` (or document a concrete reason to stay old, if one exists).

#### Minor (Nice to Have)

1. `pnpm-workspace.yaml` (`.../review-c822a17..fe3d2d5.diff:6143-6145`) has no inline comment explaining why `vue-demi`'s install script is trusted, breaking the otherwise strong self-documenting convention this diff establishes elsewhere (`deny.toml`, `.npmrc`, `eslint.config.js` all carry rationale comments). The justification only lives in the external report.
2. `deny.toml`'s "gtk-rs GTK3 bindings ... archived upstream" header comment (`.../review-c822a17..fe3d2d5.diff:5975-5978`) groups `RUSTSEC-2024-0370` (`proc-macro-error`) under it, but that advisory's actual reason is `proc-macro-error`'s own independent unmaintained status via the `glib-macros` build dependency, not GTK3 archival per se. The inline `# proc-macro-error (glib-macros build dep)` comment is accurate; only the shared group header slightly overgeneralizes. Cosmetic.
3. `tauri.conf.json` sets `"security": {"csp": null}` (`.../review-c822a17..fe3d2d5.diff:6300`), disabling CSP. Standard Tauri scaffold default and not a Task 4 requirement, but worth a forward pointer before this ships with real IPC surface.
4. Literal `"Muxsmith"` remains in `tauri.conf.json`'s `productName`/window `title` and `index.html`'s `<title>`, as the implementer flagged. Confirmed accurate: these are native OS/webview chrome outside the Vue-template lint's reach, not a violation of D27's actual mechanism. Flagging per instructions; branding-exemption call is the controller's, not mine.

### Assessment
**Task quality:** Needs fixes
**Reasoning:** One Important, easily-fixed config-hygiene gap (`eslint` pinned two years stale, contrary to the pin-everything policy's "decisions, not defaults" standard applied rigorously everywhere else in this diff); everything else, including the extensively-flagged `cargo deny` work, checked out as accurate and well-justified under independent verification.