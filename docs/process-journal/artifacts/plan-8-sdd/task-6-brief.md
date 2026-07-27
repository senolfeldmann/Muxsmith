### Task 6: the REHEARSAL - two workflow_dispatch runs, R1-R10

Read design section 8 in full, D79, D83, and the merge-order section above. Model tier: mid.

**Sequencing constraint (structural):** `workflow_dispatch` requires `release.yml` on the default branch, so this task runs AFTER the workflow lands on master and is pushed (controller pushes; standing authorization SI-4). Preconditions, verified before the first dispatch, in order:

1. All five wave-1 tasks merged, nine-part gate green on master, master sanity runs green (merge-order section), master pushed.
2. The push-triggered ci run on the head SHA completed green including the `ledger-lint` job: `gh run list --workflow ci.yml --commit "$(git rev-parse HEAD)" --json status,conclusion` shows completed/success (else the release guard will poll against a red or absent gate). If the run is still in flight, wait foreground: `timeout 2700 gh run watch <run-id> --exit-status --interval 30`.
3. Baseline for R3: record `gh release list` output (expected today: empty - the repo has no releases).

**gh usage rules bind throughout**: every gh interaction on this repo gets a `gh-log.md` entry - command, effect, manual-UI equivalent (format per the file's header); nothing that costs money; public-repo Actions on standard runners are free (two dispatch runs = 12 jobs total, recomputed: 6 per run - guard, 4 legs, assemble; design section 6).

**Wall-clock note:** four native legs incl. `windows-11-arm` (4-CPU), cold builds by design (no cache, D85). Every wait is a foreground `gh run watch` or a bounded poll with an explicit timeout - **never background-run-plus-monitor**. Ceilings: run-id resolution 10 x 15 s; each `gh run watch` wrapped in `timeout 5400` (90 min); a tripped timeout stops the task and returns to the controller with the run URL and partial evaluation - it is never silently extended.

**The acceptance checklist (design section 8, transcribed verbatim - this is the plan's acceptance test; evaluate each observable at its named emitter):**

```
- **R1** (run A): guard passes both steps; the gate-green step's log
  names the found ci run. All four legs green; the rename step's log
  carries one `pick: <path>` line per selected artifact (1 per Windows
  leg, 1 on macOS, 3 on Linux - the function logs every selection to
  stderr) and its closing `ls -l` lists exactly the renamed files
  (1 / 1 / 4 incl. the tar.gz).
- **R2** (run A): four workflow artifacts named `muxsmith-<leg>` exist
  with retention 7; downloaded together they contain exactly the 7 files
  of D89 with the scheme-conformant names (count check: 7).
- **R3** (run A): **no release was created** (the assemble release step
  shows as skipped in the run's job view - the skip is the observable,
  not an absence-grep; `gh release list` unchanged as corroboration).
- **R4** (run B): draft `rehearsal-<run_id>` exists (created by the
  assemble job's release step), `isDraft: true`, with exactly 8 assets
  (7 + SHA256SUMS). Download all; `sha256sum -c SHA256SUMS` passes.
  Falsifiability control once: corrupt one downloaded byte, watch `-c`
  fail, redownload.
- **R5** (run B): body = rehearsal banner + template with the version
  substituted + generated notes. And in each of the four leg logs, the
  step "Assert no updater artifacts were produced" printed its summary
  line (`updater-artifact check: 0 hits across N bundle output files`,
  N > 0 - the N is the step's built-in positive control that it saw the
  bundle tree; the step's red and control-red states are the G4
  fire-test).
- **R6** (run B or A, artifact-content checks on a Linux machine):
  `dpkg-deb -I muxsmith-*.deb` shows `Recommends: mkvtoolnix` and
  `Depends:` containing `libwebkit2gtk-4.1-0` and `libgtk-3-0`;
  `dpkg-deb -c` lists `./usr/bin/muxsmith` and `./usr/bin/muxsmith-gui`;
  `rpm -qp --recommends muxsmith-*.rpm` prints `mkvtoolnix`;
  `./muxsmith-*.AppImage --appimage-extract` yields
  `squashfs-root/usr/bin/muxsmith`; the tar.gz lists the D88 four-file
  layout under its version-named directory; `msiextract` (msitools) on
  each msi lists `muxsmith.exe` and `muxsmith-gui.exe`.
- **R7** (run B): **no tag ref was created**:
  `git ls-remote origin | grep -c rehearsal` is 0 while the same
  `ls-remote` output demonstrably lists `refs/heads/master` (the
  positive control proving the listing works; the repo currently has no
  tags, so master is the known-present ref).
- **R8** (owner, on real hardware, once per OS): msi installs on a
  Windows machine after the SmartScreen "Run anyway" flow exactly as
  `docs/INSTALL.md` describes; Programs-and-Features shows publisher
  "Şenol Feldmann" unmangled (D86's fallback fires only if this fails);
  dmg opens on an Apple-Silicon Mac via the Settings > Open Anyway flow
  as documented; `muxsmith` runs from the documented per-OS CLI
  locations. These three walk-throughs double as the screenshot source
  for any future doc polish.
- **R9** (run B): version self-report - `./muxsmith --version` from the
  tar.gz prints the Cargo workspace version (D87's artifact-level
  criterion; the guard already pinned tag == that version).
- **R10** (owner): delete the rehearsal draft(s) after inspection.
```

- [ ] **Step 1: Dispatch run A** (artifact path; input defaults to false):

```bash
gh workflow run release.yml --ref master
```

gh-log the dispatch. Resolve the run id by bounded poll (10 tries x 15 s foreground sleep): `gh run list --workflow release.yml --limit 1 --json databaseId,status,event,createdAt` until a `workflow_dispatch` run created after the dispatch timestamp appears.

- [ ] **Step 2: Watch run A foreground**: `timeout 5400 gh run watch <idA> --exit-status --interval 30` (flags verified against gh 2.94.0 `--help` at plan-authoring). Expected: all 6 jobs green.

- [ ] **Step 3: Evaluate R1-R3.** Emitter mapping: R1 - `gh run view <idA> --log` (guard log's gate-green line; per-leg `pick:` stderr lines: recount 1/1/3 against the leg matrix; the rename step's closing `ls -l` listings: 1/1/4 files). R2 - `gh api "repos/senolfeldmann/Muxsmith/actions/runs/<idA>/artifacts" --jq '.artifacts[] | {name, expires_at}'` (four names `muxsmith-<leg id>`; `expires_at` ~7 days out), then `gh run download <idA> --dir <scratch>/runA` and count + name-check the 7 files against the transcribed D89 set (the version token is the current Cargo workspace version - recompute from `Cargo.toml`, today 0.1.0). R3 - `gh run view <idA> --json jobs` shows the release-creation step `skipped` in the assemble job (the skip IS the observable); `gh release list` equals the Step-0 baseline. gh-log every command.

- [ ] **Step 4: Dispatch run B** (rehearsal path):

```bash
gh workflow run release.yml --ref master -f rehearse-draft-release=true
```

Resolve the id as in Step 1; watch as in Step 2 (`timeout 5400`, foreground). Expected: all 6 jobs green, the assemble release step RUNS this time.

- [ ] **Step 5: Evaluate R4-R5.** R4 - `gh release view "rehearsal-<idB>" --json isDraft,name,assets` (`isDraft: true`; asset count exactly 8); download all assets to `<scratch>/runB` - primary `gh release download "rehearsal-<idB>" --dir <scratch>/runB`; if the draft's tagless state defeats the tag-name lookup, fall back to `gh api repos/senolfeldmann/Muxsmith/releases --jq '...'` to get the release id and fetch each asset via `gh api` with `Accept: application/octet-stream` (both shapes gh-log'd; no third variant). Then `sha256sum -c SHA256SUMS` in that directory passes; run the falsifiability control once (corrupt one byte, `-c` fails naming the file, redownload). R5 - `gh release view "rehearsal-<idB>" --json body`: banner first, then the template with the version substituted (no `__VERSION__` survivor - `grep -c __VERSION__` on the body: 0; fire-verify the grep against the committed template file, which has 8), then the generated-notes section; and `gh run view <idB> --log | grep "updater-artifact check:"` yields exactly 4 lines (one per leg), each `N > 0`.

- [ ] **Step 6: Evaluate R6** on run B's downloaded assets. Tool gate first: `command -v dpkg-deb rpm msiextract sha256sum tar`. Measured on the execution machine (Fedora) at plan-authoring: `rpm`, `sha256sum`, `tar` present; **`dpkg-deb` and `msiextract` ABSENT**. If still absent, STOP and request the owner-authorized install through the controller (`sudo dnf install dpkg msitools` - a system change on Şenol's machine is his call, standing rule), then run every R6 check as transcribed. Mechanical note: `chmod +x` the downloaded AppImage before `--appimage-extract` - a fresh download is not executable (INSTALL.md documents the same step for users). Skipping any R6 check is a defect, not an option.

- [ ] **Step 7: Evaluate R7**: `git ls-remote origin > <scratch>/ls-remote.out`; `grep -c rehearsal <scratch>/ls-remote.out` -> 0 AND `grep -c 'refs/heads/master' <scratch>/ls-remote.out` -> >= 1 (the transcribed positive control; read-only git needs no gh-log entry).

- [ ] **Step 8: Evaluate R9**: extract run B's tar.gz, run `./muxsmith --version` from the extracted version-named directory; output equals the `Cargo.toml` workspace version (awk-parse it; do not hardcode).

- [ ] **Step 9: Route R8 and R10 to the plan close.** Both are owner steps (real hardware; draft deletion). The task report lists them as pending-owner with the D86 publisher-fallback protocol note (fires only on R8's observable failure; changes exactly the `publisher` field to `"Senol Feldmann"`). The task does NOT delete the rehearsal drafts - the draft is the rehearsal's deliverable for the owner's inspection (D79).

- [ ] **Step 10: Report.** R1-R7 + R9 verdicts with the observed values, the two run URLs, every gh-log entry written, scratch cleanup done (downloaded assets removed; the drafts remain for the owner).

---

## Plan close (controller actions, not tasks)

- **Whole-branch review** by the resumed independent reviewer against the design, before any close action (house standing).
- **Owner steps R8 + R10** (pre-registered): the three real-hardware walk-throughs (Windows msi incl. Programs-and-Features publisher rendering - the ONLY trigger for D86's pre-decided fallback, `publisher` -> `"Senol Feldmann"`, no other field; Apple-Silicon dmg via the Settings flow; per-OS CLI locations), then deletion of the rehearsal draft(s). The owner publishes nothing in this plan (D81).
- **Owner rendered-surface pass** over the user-facing collateral wording: `docs/INSTALL.md` (pre-registered by the brief), the draft-body and rehearsal-banner templates, `packaging/linux-tarball-README.txt`, the BUILDING.md subsection, the README rider comment. Structure and facts are the design's; wording is the owner's.
- **Triggers to mirror into the ROADMAP** (design section 9, recomputed: 9): 1 (ubuntu-22.04 retirement -> move the Linux leg to 24.04 + raise the documented floors in the same change), 2 (dated windows-arm64 label appears -> pin it, closing D85's recorded deviation), 3 (tauri/@tauri-apps/cli bump -> re-verify the four pinned bundler facts before the next release), 4 (signing revisit fires -> shrink INSTALL.md per its embedded comment; evaluate artifact attestations first - EXTENDS the existing registered signing trigger), 5 (Intel-dmg request -> `macos-x86_64` leg on `macos-15-intel`, extend D89's set + D77's table - EXTENDS the existing registered Intel trigger), 6 (windows-portable request -> the section-5 7z parity gap becomes a v1.x candidate), 7 (German installer UI request -> reopen D86's `wix.language`), 8 (a tar.gz bundler lands / cargo-dist earns its keep -> revisit D88), 9 (runner-image gh breaks a release-ops invocation -> pin gh by direct versioned download).
- **ROADMAP "Ledger hygiene" bookkeeping**: record the rider as executed (CI wiring + duplicate-key extension DONE via Task 5) on both entries.
- **sdd-scratch citations riding the salvage** (pre-registered; corrected in plan fix round 1 per `proc-sweep-surface-completeness`, Tier-1 ledger: a firing positive control proves the sweep PATTERN is valid, never that its SEARCH SURFACE is complete - the round-1 bullet swept one file and pre-registered "exactly these two" where the four-file surface already held five, and the review's own ledgering has since added three more). **The rule, binding at salvage time:** when the plan-8 sdd salvage runs, re-run the citation sweep over ALL FOUR house YAML files (the surface `scripts/ledger-lint.py` hardcodes) for every plan-8 scratch basename, and re-point EVERY ref whose text names plan-8 to the salvaged artifact path, in the same change as the salvage (the ruled round-8 house pattern, ROADMAP Triggers precedent). The design itself contains no `.superpowers/sdd` citation (grep for `sdd`/`superpowers`/`review-round` over the design: zero hits; positive control against the plan brief fired). The sweep and its hit list, run 2026-07-23 in the fix round (line numbers are as-of-date - this round already shifted the reviewer's by two - so match hits at salvage by entry id + ref text, never by line):

  ```bash
  grep -n 'design-brief\|design-review-brief\|design-review-round-1\|plan-brief\|plan-review-round-1' \
    docs/conventions.yaml docs/process-conventions.yaml docs/product-boundaries.yaml docs/decision-ledger.yaml
  ```

  19 hits at the fix date; the 8 naming plan-8 scratch artifacts (recount at salvage - every ledgered review round grows this surface, and the fix round itself watched the ledger lines shift by +1 while a parallel plan-7.5 writer appended an occurrence, which is why the id + ref-text match is the rule and the lines below are only the dated snapshot):

  - `docs/conventions.yaml:1025` - `code-comment-line-citations-drift` - "plan-8 design review round 1 m2 (design-review-round-1.md)"
  - `docs/process-conventions.yaml:553` - `design-empirical-claims-reproducible` - "plan-8 design review round 1 HARVEST (design-review-round-1.md)"
  - `docs/process-conventions.yaml:554` - `design-empirical-claims-reproducible` - "plan-8 delta review HARVEST (design-review-round-1.md round-2 section, post-promotion)"
  - `docs/decision-ledger.yaml:4066` - `design-acceptance-observables-have-producers` - "plan-8 design review round 1 M1 (design-review-round-1.md)"
  - `docs/decision-ledger.yaml:4095` - `proc-quote-verbatim-or-paraphrase` - "plan-8 design review round 1 m3 (design-review-round-1.md)"
  - `docs/decision-ledger.yaml:4138` - `proc-sweep-surface-completeness` - "plan-8 plan review round 1 M1 (plan-review-round-1.md)"
  - `docs/decision-ledger.yaml:4152` - `ci-additive-only-check-numstat` - "plan-8 plan review round 1 m1 (plan-review-round-1.md)"
  - `docs/decision-ledger.yaml:4166` - `plan-interfaces-absent-by-construction` - "plan-8 plan review round 1 HARVEST (plan-review-round-1.md)"

  The other 11 hits are outside this close (recount at the fix date): 5 x `design-review-round-1` naming plan-7/plan-7.5 scratch files (conventions:1028, process-conventions:426/551/552, decision-ledger:4109), 4 x `plan-review-round-1` naming plan-7.5's (process-conventions:335/502, decision-ledger:4110/4124) - those carry their own registered ROADMAP re-pointing triggers (plan-7's already CONSUMED, plan-7.5's registered) - plus 2 prose refs to the plan-6 design brief (`design-brief` without a filename, process-conventions:378 and decision-ledger:3449), prior plans' scope, not plan-8's. The basename `design-review-brief` has zero house-YAML hits; the same grep form firing on the sibling basenames is the pattern's positive control.
- **Journal + HANDOFF snapshot** per SI-2 at the plan close (standing duty; listed for completeness, not new).
