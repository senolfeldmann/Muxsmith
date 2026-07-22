# Task 11 verdict (D50 marked pin + D51 topic loader)

**VERDICT: APPROVED**

Reviewer: independent SDD task reviewer (Opus). Commit `c445aa7` on `plan7-f`,
parent `c8cf309` (fully-merged wave-1 master). 7-file diff, +80/-1. The
implementation is the plan's Step 2/3/4 prescribed code transcribed verbatim;
every claimed behavior was re-run here, not taken on report.

---

## Findings

### 1. Dependency discipline - PASS

- `package.json` `dependencies` gains exactly one line: `"marked": "18.0.7"`,
  bare-pinned in the house style (siblings `vue` 3.5.39, `fluent-vue` 3.8.2,
  `@tauri-apps/*` all bare). No `devDependencies` change.
- Registry re-verified myself: `pnpm view marked@18.0.7 --json` ->
  `version 18.0.7`, `dependencies {}`, `engines node >= 20`, `license MIT`.
  Matches D50's ground-truth table (0 deps, MIT).
- Zero transitive deps confirmed from two independent sources: the lockfile
  snapshot line 2157 is `marked@18.0.7: {}` (empty dep map), and the installed
  `node_modules/marked/package.json` reports `deps: {}`. The lockfile diff adds
  only marked's three own entries (importer specifier :32, package resolution
  :984, snapshot :2157); `magic-string`/`minimatch` in the diff are unchanged
  context lines.
- Nothing else changed dependency-wise (npm or cargo). Global Constraint
  "no other new dependency" honored.

### 2. Loader compliance - PASS

- `src/help/topics.ts` glob:
  `import.meta.glob("../../help/*/*.md", { query: "?raw", import: "default", eager: true })`.
  Options object is byte-identical to the i18n loader `src/i18n/index.ts:17-20`
  (`{ query: "?raw", import: "default", eager: true }`); only the glob target
  differs (`help/*/*.md` vs `locales/*/gui-*.ftl`), which is inherent. Faithful
  house mirror per D51 - HOUSE dimension passes.
- `topicHtml` chain walked against the code: `sourceFor(helpId, primarySubtag(locale))`
  `?? sourceFor(helpId, "en")`; if `null`, `return helpId`; else
  `marked.parse(source, { async: false })`. That is exactly
  requested-primary-subtag collapse -> per-topic en fallback -> raw help-id,
  never blank (D51's order verbatim). `primarySubtag` = `split("-")[0].toLowerCase()`
  (verified export at `src/i18n/index.ts:42`), so `de-AT`->`de`, `fr`->`fr`,
  `en-US`->`en`.
- `marked.parse(..., { async: false })`: no sanitizer, no custom renderer, no
  options beyond `async:false` (D50's "defaults, no sanitizer"). `async:false`
  is marked's own default made explicit so the overload narrows the return to
  `string` for the `: string` signature - a type necessity, not a semantic
  deviation, and it matches the plan's prescribed code and the spec's expected-
  value computation (like-for-like).
- v-html absence FIRE-VERIFIED: `grep -rn "v-html" src/ e2e/` -> rc=1, no hits;
  the identical grep for `"marked"` returns hits (rc=0), proving the mechanism
  fires. Task 12's single-site license (`HelpSidebar.vue`) correctly does not
  exist yet. The one `sanitiz` hit is in the gitignored built bundle
  `e2e/.generated/mount-harness.js` (fluent-vue internals), not in source,
  package.json, or the diff.

### 3. Behavioral (run here) - PASS

- `pnpm lint` clean (exit 0). `pnpm check:i18n` -> "check-i18n: ok" (exit 0;
  17 unused-key warnings are pre-existing, non-fatal). `pnpm build` clean
  (`vue-tsc --noEmit` passes -> the TS layer that was RED is now green;
  190 kB bundle).
- `pnpm build` run first as the webServer prerequisite (playwright.config
  `webServer` = `vite preview` over `dist/`), then full `pnpm test:e2e`:
  **34 passed**, matching the expected count. The new `help-topics.spec.ts`
  (test 4) passed.
- The new spec genuinely proves the fallback chain against real files: it reads
  `help/en/view-batch.md` and `help/de/view-batch.md` (both present; all 44
  topic files exist beneath this worktree, `find help -name '*.md'` = 44), then
  asserts en, de, de-AT (primary-subtag collapse -> de), fr (unknown -> en
  topic), and no-such-topic (raw id, never blank). Node-side `marked.parse`
  vs browser-side `window.__muxsmithTopicHtml__`, same marked 18.0.7 -
  a real cross-boundary render + fallback assertion, not a tautology.

### 4. loadHarness refactor - PASS (behavior-identical)

Compared against parent `c8cf309:e2e/mount.ts`. Parent `mountComponent` ran the
sequence `setContent -> addScriptTag -> evaluate(__muxsmithMount__)`. The change
extracts the first two calls into `loadHarness(page)` and re-points
`mountComponent` to `loadHarness(page)` then `evaluate`. Identical operation
sequence, no second bootstrap copy. `mountWidget` still funnels through
`mountComponent`, unchanged. The 33 pre-existing specs (smoke 30 + catalogs +
editor-tooltips + locale-switch), all of which mount through this path, stayed
green in the 34-pass run - the behavior-identity control.

### 5. Quality - PASS

- 7-file diff exact, matching the plan's file list and Step 6 staging list
  precisely: `package.json`, `pnpm-lock.yaml`, `src/help/topics.ts` (new),
  `e2e/mount-entry.ts`, `e2e/mount.ts`, `e2e/global.d.ts`,
  `e2e/help-topics.spec.ts` (new). No stray files; working tree clean.
- Commit trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`
  present, matching the plan's prescribed commit command. Clean tree with
  exactly the 7 files evidences explicit staging (no `git add -A` residue).
- RED-state claim accepted as reproducible logic (not re-run, per brief): before
  `src/help/topics.ts` and the `mount.ts` `loadHarness` export existed,
  `e2e/mount-entry.ts`'s `import { topicHtml } from "../src/help/topics"` and the
  spec's `import { loadHarness } from "./mount"` fail `tsc --noEmit -p
  e2e/tsconfig.json` (the first step of `test:e2e`) - a TS compile error, as
  claimed. GREEN confirmed here.

### 6. Own absence check fire-verified against a control - DONE

See finding 2 (v-html grep rc=1 vs the identical "marked" grep rc=0 with hits).
Also cross-checked the zero-transitive claim against a positive source (the
registry `dependencies {}` and installed package `deps {}`), not only the
lockfile snapshot.

---

## HARVEST

- **Ideal SDD outcome**: the plan fully specified the code (Steps 2/3/4), and
  the implementer transcribed it verbatim rather than paraphrasing. Grading
  collapses to "does the tree match the prescribed code + do the claimed gates
  actually pass" - fast and unambiguous. Worth noting as the target shape when a
  plan carries prescribed code.

- **Stale line citation in a shipped comment** (minor, not a Task 11 defect):
  `topics.ts`'s fallback comment cites the raw-id posture as
  `muxsmith-cli/src/i18n.rs:41-46`, but 41-46 is inside `Renderer::new` (the
  constructor). The actual raw-id fallback (`id.to_string()`) lives in `render`
  around line 123. The referenced behavior genuinely exists; only the line
  number is wrong. The implementer copied this from the plan/design verbatim,
  and the plan itself warns that in-repo line refs drift - so this is a
  design/plan-doc artifact, not an implementer miss. Framework harvest: line-
  number citations that land in shipped code comments (not just cross-doc refs)
  are a drift surface the "cite by quoted text, not line number" rule should
  also cover for code comments.

- **Micro-redundancy, sub-defect** (no action): for a genuinely-missing topic
  requested with an en-family locale, `topicSources` is linearly scanned twice
  (`sourceFor(id, "en")` then the `?? sourceFor(id, "en")` fallback both target
  "en"). At 44 topics this is negligible and `sourceFor`'s linear scan mirrors
  i18n's own `catalogsForLocale` filter, so it is scale-appropriate. A
  `Map<`${dir}/${id}`, source>` precompute only becomes idiomatic if the topic
  count reaches the hundreds - premature now.

- **Over-restriction watch: clean.** This task introduces no new restriction of
  its own. The constraints it honors (single v-html site, exact 18.0.7 pin, no
  sanitizer, no other dependency) are all owner-approved D50/D51 decisions with
  their steelmen recorded in the design; the implementer enforced them without
  widening or narrowing. The raw-id branch returns unescaped `helpId` into what
  will be v-html, but a kebab-case id carries no HTML metacharacters and the
  branch is only reachable in a build that dodged D62 - the specified posture,
  not an over-restriction.

- **Probe hygiene**: all probes foreground; the only writes to the tree were the
  gitignored `dist/` and `e2e/.generated/` build outputs (`git check-ignore`
  confirms both ignored), working tree `git status --porcelain` empty after the
  full gate run. Verdict file is the sole intentional write.
