# T1 report: ci.yml workflow-level permissions (Stream A, verdict item 9)

Implementer report. Worktree `/home/senol/Git/Muxsmith/.worktrees/plan57-a` (branch `plan57-a`).

## What changed

`.github/workflows/ci.yml` only. Inserted at workflow top level, after the
`on:` block and before the pinning-policy comment:

```yaml
# Least privilege (verdict item 9): repo default verified read on 2026-07-14; this explicit block guards against out-of-band default changes and satisfies OSSF Scorecard.
permissions:
  contents: read
```

Effect: `GITHUB_TOKEN` gets `contents: read` and every other scope is
implicitly `none`, workflow-wide (both jobs). No job-level `permissions:`
blocks exist, so nothing overrides it. Nothing else touched.

## Least-privilege audit (every step, both jobs)

Re-check of the reviewer question in the plan ("do rust-cache /
mise-action / all steps need more than contents:read?" - adjudicated no
in the verdict). Confirmed no, step by step:

**Job `test`** (matrix ubuntu-26.04 / windows-2025 / macos-15):

| Step | Token use | Needs beyond contents:read? |
|---|---|---|
| `actions/checkout@9c091bb` (v7.0.0) | `GITHUB_TOKEN` to clone this repo | No - `contents: read` is exactly the required scope |
| Install pinned Rust toolchain (`rustup ...`) | none (rustup CDN) | No |
| `Swatinem/rust-cache@c193711` (v2.9.1) | GitHub Actions cache service via the runner's `ACTIONS_RUNTIME_TOKEN`, not `GITHUB_TOKEN`; the `permissions:` block does not govern the cache service | No - cache save/restore unaffected |
| Install mkvtoolnix (apt / choco / brew, 3 steps) | none (package-manager mirrors) | No |
| Install Tauri Linux build dependencies (apt) | none | No |
| `jdx/mise-action@e6a8b39` (v4.2.0) | default `github.token` only for GitHub API reads (release download / rate limit); public-read, no write scope | No |
| `pnpm install --frozen-lockfile` | none (npm registry) | No |
| `cargo fmt` / `cargo clippy` / `cargo test` / `cargo doc` | none (local; crates.io for deps) | No |
| Assert no gated tests silently skip (`cargo test` + grep) | none; `::error::` is a workflow log command, not the Checks API | No |
| `pnpm lint` / `pnpm build` / i18n gate | none | No |
| Install Playwright Chromium / Playwright smoke | none (Playwright CDN) | No |

**Job `deny`:**

| Step | Token use | Needs beyond contents:read? |
|---|---|---|
| `actions/checkout@9c091bb` (v7.0.0) | clone | No |
| `EmbarkStudios/cargo-deny-action@bb137d7` (v2.0.20) | none; cargo-deny fetches the RustSec advisory DB anonymously | No |

No step pushes commits, writes checks/PR comments, uploads artifacts via
the Releases API, or publishes packages. `contents: read` suffices
workflow-wide.

## Verification (foreground)

1. YAML well-formedness (PyYAML safe_load): PASS
   ```
   $ python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml')); print('yaml OK')"
   yaml OK
   ```
2. Block sits at workflow level, not job level (structural + grep check): PASS
   ```
   $ python3 (parse + assert)
   top-level keys: ['name', True, 'permissions', 'jobs']
   workflow-level permissions: {'contents': 'read'}
   job-level permissions: none (jobs: test, deny)
   $ grep -n "^permissions:" .github/workflows/ci.yml
   10:permissions:
   $ grep -n "  permissions:" .github/workflows/ci.yml
   (no indented job-level permissions - expected)
   ```
   (The `True` top-level key is YAML 1.1 parsing bare `on:` as a boolean -
   pre-existing GitHub-workflows normal, unrelated to this change.)
   Diff scope confirmed: `git diff --stat` = 1 file, 4 insertions, ci.yml only.
3. Workflow lint: no local workflow linter available (checked actionlint,
   zizmor, action-validator - none installed), so per the plan's
   "local gate parts that cover workflow lint (if any)" this part is
   empty. The on-runner proof is the plan-close CI run (T5), per plan.

## Commit

- `7c75f00a4f587ae07fd5f1b6b1b78e0a20579d60` on `plan57-a`:
  "ci: workflow-level least-privilege permissions (contents: read)",
  unsigned (`-c commit.gpgsign=false`), Co-Authored-By trailer, staged
  file-explicit (no `git add -A`).

## Deviations surfaced (none resolved silently)

1. **ci-06-per-commit-gate vs plan Task 1 verify scope.** ci-06
   (process-conventions.yaml) says the five-part Rust gate + deny must
   pass before every commit, never skipped. Plan 5.7 Task 1 explicitly
   scopes T1 verification to workflow lint (none available) + YAML
   well-formedness, with the on-runner proof at plan close (T5). The
   diff touches only `.github/workflows/ci.yml` (no Rust/frontend
   surface), and the controller brief repeats the narrowed scope. I
   followed the plan and did NOT run the nine-part gate; flagging the
   tension explicitly so the controller/reviewer can rule (plan-5.6 T1
   precedent: gate-scope deviations get disclosed, not resolved
   silently).
2. **Comment length vs house comment style.** ci.yml wraps its policy
   comments at ~75 columns; the brief specified a one-line comment, so
   the rationale is a single long comment line (~170 chars). Cosmetic;
   re-wrap if the reviewer prefers house wrapping.

No other deviations: no pins added or changed (ci-10 untouched), matrix
untouched (ci-01), gated-test steps untouched (ci-08).

## Fix wave 1 (T1-verdict.md F1, minor)

Rationale comment re-wrapped across three `#` lines (65/68/39 chars, all
<=78, matching the file's workflow-level comment wrapping); content
unchanged. Deviation 2 above is thereby resolved. Re-verified: PyYAML
parse OK, `permissions:` still top-level at column 0. Amended in place so
the stream stays one commit; new hash
`6c0a7207c12f818f8012a337c925907685f3ba16` (supersedes `7c75f00a...`,
same message, unsigned, trailer kept; diff now 6 insertions, ci.yml only).
