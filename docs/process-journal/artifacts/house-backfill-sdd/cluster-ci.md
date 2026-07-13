# House-knowledge clusters - domain `ci`

Reconstructed from 30 occurrence records spanning eras E0-E8. Records grouped by identical `(topic, approach)` even when worded differently across eras; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`.

Dates verified against the repo (`git show %cs` / mtime): commit `97ae031` = 2026-07-08 (E0/E1); `5561601`/`72c59d2`/`0e64c1e` = 2026-07-09 (E2/E3); Plan 4 progress + `2ee2d0c`/`45e941a`/`226fa06`/`fdf220b`/`da69eec`/`46c7874`/`63fdfc4`/`4902a2a`/`656449c` = 2026-07-10 (E4/E5/E6); `plan-5.5/task-2-verdict.md`/`task-12-verdict.md` = 2026-07-11 (E7); `b38a46f`/`c38a197` (CONVENTIONS.md consolidation) = 2026-07-12 (E8).

Four clusters reach the promotion threshold: ci-01 (CI matrix strategy, 3), ci-06 (per-commit gate, 3), ci-08 (gated tests actually run in CI, 6), ci-10 (pin everything, 3). No count was padded: same-decision citations within one document/session collapse to one occurrence, and the two records that literally repeat a decision across reconstruction eras (E0 `[3]` + E1 `[4]`) are deduped to a single occurrence. One record (`[11]`, the E4 per-commit gate) legitimately attests two distinct patterns and is counted in both ci-06 and ci-07; see the clustering notes.

---

## ci-01-matrix-strategy - CI matrix tracks repo visibility (Linux-only while private -> static 3-OS on go-public)
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** While the repo is private, branch pushes run Linux only and the full windows/macos/linux matrix runs only on PRs, tags and manual dispatch, selected via a dynamic `fromJSON` matrix (macOS Actions bill 10x, Windows 2x). The trim was always framed as reverting to a static 3-OS matrix on every push once the repo goes public and minutes become free; that revert was enacted at go-public.
- **Steelman:** Always running the full 3-OS matrix catches cross-platform breaks earlier, at higher Actions-minute cost.
- **Occurrences:** (E0 `[3]` and E1 `[4]` are the same decision re-attested one era later -> one occurrence)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §10 + journal 2026-07-08 "CI while private" + commit 97ae031 | "linux-only on branch pushes while private; full matrix on PR/tag/dispatch." |
| 2026-07-08 | deferred | journal 2026-07-08 Pending decisions + handoff plan-1-close | "go-public timing (flips CI matrix back to 3-OS on push; Actions then free). His call." |
| 2026-07-10 | decided | commit 226fa06 + journal Plan 5 | "static 3-OS matrix on every push (go-public)." |

---

## ci-02-sccache-rejected - sccache rejected (no compile-time pain at this size)
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** No sccache: there is no compile-time pain at this workspace size.
- **Steelman:** sccache would speed CI and local rebuilds as the workspace grows.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | BUILDING.md "Deliberately not used" | "sccache: no compile-time pain at this workspace size." |

---

## ci-03-cargo-outdated-rejected - cargo-outdated rejected (Renovate/Dependabot replaces it)
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** No cargo-outdated: Renovate/Dependabot will replace it once activated.
- **Steelman:** cargo-outdated gives an immediate local dependency-freshness view before the bots are wired.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | BUILDING.md "Deliberately not used" | "cargo-outdated: Renovate/Dependabot replaces it once activated." |

---

## ci-04-dependabot-cadence - Dependabot/Renovate activation and cadence deferred (CI-cost decision)
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** Dependabot/Renovate activation is deferred to Şenol's call; the eventual cadence is a CI-cost decision because every dependency PR triggers the full 3-OS matrix.
- **Blocked on:** go-public/1.0 timing (free only once the repo is public); activation remains Şenol's call.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | deferred | journal 2026-07-08 open threads + ROADMAP residue R3 | "every dep PR triggers the full 3-OS matrix, so the cadence choice is a CI-cost decision." |

---

## ci-05-cargo-deny-gate - cargo-deny supply-chain gate + deny.toml hygiene
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** cargo-deny runs as a CI gate; deny caught the intra-workspace path dependency as a wildcard, resolved by marking the workspace crates `publish = false` (they ship as app bundles, not to crates.io) - correct on its own terms. The license allow-list is kept minimal (trimmed from a speculative 8 to the 3 actually used: MIT, Apache-2.0, Unicode-3.0), extended only as licenses actually appear, and each RUSTSEC advisory ignore carries a reachability justification (e.g. quick-xml build-time-only).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | journal 2026-07-09 Plan 2 "What the process caught" + commit 5561601 | "cargo-deny caught the intra-workspace path dependency as a wildcard; resolved by marking the workspace crates publish = false ... trimmed the license allow-list from a speculative 8 to the 3 actually used." |
| 2026-07-10 | reinforced | task-4-review-verdict.md (strengths) | "the build-time-only claim is not just plausible, it's exactly right." |

---

## ci-06-per-commit-gate - Four-tool gate green before every commit (test + fmt --check + clippy -D + deny)
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Standing house rule: `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -D warnings` and `cargo deny check` must all pass before every commit, never skipped. Enforcement is real, not aspirational: the CI test job caught fmt-dirty commits when tasks 3 and 5 were pushed after running clippy but not `fmt --check` (intermediate run 0e64c1e failed, fixed 72c59d2); clippy separately caught two collapsible-if let-chains pre-push. A controller-discipline gap, not a plan gap.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | journal 2026-07-09 Plan 2 "What the process caught" + commit 72c59d2 | "CI (test job) caught fmt-dirty commits: tasks 3 and 5 were pushed after running clippy but not cargo fmt --check." |
| 2026-07-09 | reinforced | plan Global Constraints + progress.md (Plan 3) | "Per-commit gate, never skipped: cargo test --workspace AND cargo fmt --all --check AND cargo clippy ... -D warnings AND cargo deny check all pass before each commit." |
| 2026-07-10 | reinforced | plan Global Constraints + progress ledger (Plan 4) | "Per-commit gate, run all four, do NOT skip fmt." |

---

## ci-07-controller-regate - Controller re-runs the full gate after every task (SI-1)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** SDD controller discipline: the controller independently re-runs the full test/fmt/clippy/deny gate after every task rather than trusting the implementer report's test-count arithmetic (SI-1: never trust the report). Green each time across the 12 Plan-3 tasks; test count grew 164 -> 204.
- **Steelman:** null
- **Occurrences:** (`[11]` also feeds ci-06; its stated approach bundles both the per-commit gate and the controller re-run, so it legitimately attests both patterns)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | journal 2026-07-09 | Plan 3 complete (Mechanics/metrics) | "Controller re-ran the gate after every task (SI-1: never trust the report's arithmetic); all green each time." |
| 2026-07-10 | reinforced | plan Global Constraints + progress ledger (Plan 4) | "controller re-ran full gate ... after every task." |

---

## ci-08-gated-tests-run - Gated integration tests must actually run in CI, not silently self-skip
- **kind:** pattern | **status:** settled | **count:** 6 | **promoted:** yes (at 3)
- **Statement:** The mkvmerge-gated integration tier must run in CI, not self-skip. E2 accepted the self-skip (mkvtoolnix not installed) as a known coverage gap. E5 installed mkvtoolnix (Linux-only for minute economy; branch pushes are Linux-only and gated tests self-skip on the other legs) and verified post-push that the tier actually ran (CI run 29059480785 green, 0 "mkvmerge not found" markers). Go-public extended the install to all three legs (apt/choco/brew) with a skip-marker count-zero assertion on every leg - otherwise silent skipping is just traded for silent skipping. That assertion immediately caught its own target class on the maiden run: 18 silently-skipped Windows tests (choco writes machine PATH, the running job never re-reads it; fixed via `GITHUB_PATH` append), then surfaced a real Windows-only `set_modified`-on-read-only-handle bug on the first-ever live run.
- **Steelman:** null
- **Occurrences:** (E5's three artifacts - memo, review verdict, session-close journal - record the decision, its Linux-only scoping and its post-push verification respectively; distinct artifacts, kept distinct)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | journal 2026-07-09 Plan 2 Open threads | "CI does not install mkvtoolnix so the gated tests self-skip there." |
| 2026-07-10 | decided | memo D18 | "Install mkvtoolnix in CI: the gated integration tier ... actually runs there instead of self-skipping." |
| 2026-07-10 | reinforced | task-6-review-verdict.md | "Step guarded by if: runner.os == 'Linux'; branch pushes Linux-only, gated tests self-skip elsewhere, macOS/Windows installs a go-public follow-up." |
| 2026-07-10 | reinforced | journal 2026-07-10 session-5-close | "CI run 29059480785 green, mkvtoolnix install executed, 0 'mkvmerge not found' skip markers, gated tests ran and passed." |
| 2026-07-11 | decided | task-2-verdict.md / plan T2 | "otherwise silent skipping is traded for silent skipping" (install on all three legs + assert skip-marker count zero) |
| 2026-07-11 | reinforced | progress.md T2 fix waves / journal | "Windows leg failed ... with marker count 18 - choco writes machine PATH, running job never re-reads it; the assertion caught exactly its target case." |

---

## ci-09-skipmarker-contract - Skip-marker string is an unenforced cross-file contract (hardening deferred)
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The skip-marker string is an unenforced cross-file contract (19 call sites + 1 CI grep, no shared const), so a future reword silently reintroduces the false-negative that ci-08's assertion exists to catch. Hardening with a shared const was deferred.
- **Blocked on:** internal - idiomacy review.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | task-2-verdict.md m1 / whole-branch funnel T2-m1 | "Marker string is an unenforced cross-file contract ... a future reword silently reintroduces the false-negative." |

---

## ci-10-pin-everything - Pin everything exactly; a floating version is a defect
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Every toolchain and CI input is pinned exactly - rust 1.96.1 over floating stable, all GitHub Actions SHA-pinned, runner images pinned, ctrlc full-pinned, npm save-exact, and mkvmerge CI versions pinned per package manager (Şenol's standing preference is pin, not float). The resulting cross-leg mkvmerge divergence (apt 97 vs choco/brew 100) is sanctioned and recorded in a `ci.yml` comment. A floating version is a defect, not a convenience.
- **Steelman:** null
- **Occurrences:** (mkvmerge per-manager pinning `[25]` is filed here rather than in ci-08: the salient recurring principle is the pin doctrine, and the record's own evidence foregrounds "Şenol's standing preference is pin")

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | commits 2ee2d0c, 45e941a / journal Plan 5 | "pin-everything (rust 1.96.1 over floating stable, all CI actions SHA-pinned, runners pinned ...)." |
| 2026-07-11 | decided | plan T2 step 2 / progress.md T2-m3/m4 / whole-branch funnel | "Şenol's standing preference is pin, so default to choco install --version=X and brew formula pin." |
| 2026-07-12 | reinforced | CONVENTIONS.md Patterns (b38a46f) | "a floating version is a defect, not a convenience" (reinforced by idiomacy findings: rustup-show non-install, resolver 2 vs edition-2024 default 3, deny.toml dead version key). |

---

## ci-11-newest-over-lts - Newest-when-nothing-blocks over LTS for dev-only runtimes
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** For dev-only runtimes, newest-when-nothing-blocks beats LTS: the controller's node-24-LTS proposal was overturned by Şenol's policy and repinned twice (24 -> 26 -> 26.5.0; pnpm 11.10.0).
- **Steelman:** null (the overturned position - node-24-LTS for stability - was the controller's proposal, corrected in-session)
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | journal Plan 5 / commits 4902a2a, 656449c | "node 24-LTS proposal overturned by his newest-when-nothing-blocks policy; plan repinned twice." |

---

## ci-12-registry-sourcing - Resolve dependency versions against the registry, not stale training-data memory
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Dependency versions are resolved against the live registry, not typed from stale training-data memory: eslint was the lone stale pin (9.9.1, ~2 years old, swallowed by wide peer ranges while everything else was registry-current), corrected to 9.39.4 then bumped to 10.6.0.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | task-4-review-verdict.md (Important #1) / commits 46c7874, 63fdfc4 | "most plausibly typed from stale training-data memory rather than resolved against the registry." |

---

## ci-13-packaging-deferred - Release-tag packaging artifacts deferred until go-public
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** Release-tag packaging artifacts (msi/dmg/deb/rpm/AppImage) remain deferred until go-public, because tags trigger the full 3-OS matrix and paid minutes while the repo is private.
- **Blocked on:** external - go-public / paid-minutes cost.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | memo D29 | "Packaging artifacts remain deferred until go-public." |

---

## ci-14-cross-target-clippy - Sweep cross-target clippy locally before the first foreign-OS CI run
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** For `-D warnings` workspaces, run cross-target clippy locally before the first foreign-OS CI run and cfg-gate imports/helpers for cfg-gated tests: Windows legs went red twice on `-D warnings` from unix-only imports.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | journal Plan 5 close addendum (3-OS) / commits fdf220b, da69eec | "for -D warnings workspaces, cross-target clippy locally before the first foreign-OS CI run; cfg-gated tests need cfg-gated imports/helpers." |

---

## ci-15-rustdoc-gate - rustdoc as the ninth CI gate part
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `RUSTDOCFLAGS=-D warnings cargo doc --no-deps` runs as the ninth gate part inside the matrixed test job on all three legs (via an `env:` block); dead intra-doc links are delinked while the target is private and real-fixed once public (T12).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | task-12-verdict.md / plan T12 | "CI step inside the matrixed test job (all three legs), RUSTDOCFLAGS via env: block." |

---

## ci-16-mise-not-ci - mise is a dev tool, not a CI tool (no floating mise binary at runtime); removal deferred
- **kind:** restraint | **status:** blocked | **count:** 2 | **promoted:** no
- **Statement:** CI must not fetch a floating mise binary at run time: `jdx/mise-action` downloads and executes the latest mise release, contradicting the pin-everything doctrine. The restraint is recorded although current CI (mise-action without a version pin, `ci.yml:L73`) still violates it; dropping mise from CI is deferred to post-1.0.
- **Steelman:** jdx/mise-action is the convenient, currently-green way to get the pinned runtimes in CI.
- **Blocked on:** internal - kept until the release stabilizes; removal tracked in ROADMAP v1.x.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | CONVENTIONS.md Restraints (b38a46f) / idiomacy routed-out ci.yml:L73 | mise-action "downloads and executes the latest mise release at run time ... contradicting the repo's pin-everything doctrine." |
| 2026-07-12 | deferred | CONVENTIONS.md Restraints note / ROADMAP c38a197 | "Post-1.0 removal tracked, ROADMAP v1.x" (supersedes the routed pre-1.0 item). |

---

## Clustering notes (defensibility)

- **ci-01 count = 3, and the E0/E1 dedup:** records `[3]` (E0) and `[4]` (E1) cite the same three artifacts (spec §10, journal 2026-07-08, commit 97ae031) for the same private-trim decision; they are one reconstruction re-attesting the other, deduped to one occurrence. The three kept occurrences are genuinely distinct facets of the CI-matrix knob: the private trim (decided), the deferred revert timing (deferred), and the enacted public revert (decided). The go-public-timing deferral `[5]` resolved when the revert was enacted `[20]`, so the whole arc is one settled pattern, not a live block.
- **ci-06 vs ci-07 (`[11]` counted twice):** record `[11]` (E4 per-commit gate) states both "every commit runs all four checks" and "the controller re-runs the full gate after each task." Those are two distinct patterns with distinct actors and checkpoints (implementer/CI per-commit vs controller per-task), so `[11]` legitimately attests both and is an occurrence in each. This is one record supporting two clusters, not a padded single cluster. ci-06 stands on three independent records (`[7]`,`[10]`,`[11]`); ci-07 on two (`[9]`,`[11]`).
- **ci-08 count = 6, the E2->E5->E7 merge:** "gated tests must actually run in CI, not self-skip" is one consideration across three eras. E2 `[8]` deferred it (self-skip accepted). E5 `[12]`/`[13]`/`[14]` installed mkvtoolnix Linux-only and manually verified. E7 `[23]`/`[24]` extended the install to all three legs with an automated skip-marker count-zero assertion - the maturation of E5's manual "0 skip markers" verification and the exact go-public follow-up that `[13]` flagged. The assertion is the enforcement of the same "don't self-skip" rule, not an unrelated verification pattern, so it belongs in this thread. The E7 records `[23]`/`[24]` share identical `(topic, approach)` and are one cluster by construction.
- **ci-08 vs ci-10 (where mkvmerge version-pinning goes):** record `[25]` (mkvmerge versions pinned per package manager) sits at the intersection of "install mkvmerge in CI" (ci-08) and "pin everything" (ci-10). It is filed under ci-10 because the recurring principle it re-attests is the pin doctrine (its evidence leads with "Şenol's standing preference is pin"), and ci-10 as a general reinforced doctrine (E6, E7, E8) is genuinely promotion-worthy on its own terms. Filing it under ci-08 instead would understate a real cross-era doctrine.
- **ci-09 and ci-13 stay separate non-decisions:** ci-04 (Dependabot), ci-13 (packaging) and ci-01's revert all block on go-public timing, but they are distinct `(topic, approach)` deferrals; they are not merged into one "go-public" cluster, which would inflate a count by fusing unrelated considerations that merely share a blocker.
- **ci-16 kind vs status:** modeled as a `restraint` (a principled rejection of floating-mise-in-CI, with a steelman for why mise-action is convenient) whose enactment is `blocked` (removal deferred to post-1.0). The principle is settled; reality still violates it pending release, which is why status is blocked rather than settled.
