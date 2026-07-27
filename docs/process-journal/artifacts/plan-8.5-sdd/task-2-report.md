# Task 2 report: the license experiment, then the picked branch (A5)

**Commit:** `50e08cdd5763b8e5baa5716e1afd2726f564461c` (`50e08cd`), on `master` in
the main worktree, unsigned (`git log -1 --format='%G?'` -> `N`), 3 files changed,
37 insertions, 2 deletions. **Not pushed** (`origin/master..HEAD` carries it plus
the six earlier unpushed commits).

Built on Task 1 (`9460daf` + `5060ef5`); tree was clean at start, clean after.

## The experiment (Steps 1-3): re-run, matches the authoring-time result

Executed on this machine with the repo-pinned CLI (`./node_modules/.bin/tauri
--version` -> `tauri-cli 2.11.4`). Precondition held without a rebuild:
`target/debug/muxsmith-gui` present (281228536 bytes, mtime Jul 27 23:10).

**Step 1, baseline (the known-present control):**

```
./node_modules/.bin/tauri bundle -d --bundles deb
dpkg-deb -I target/debug/bundle/deb/Muxsmith_*_amd64.deb | grep -cE '^ Recommends'
```

-> **`1`**. Full `dpkg-deb -I` output carried ` Recommends: mkvtoolnix` and
` Depends: libwebkit2gtk-4.1-0, libgtk-3-0`; control archive reported
`429 bytes, 12 lines control`.

**Step 2, the null-clear probe.** `src-tauri/tauri.linux.conf.json` written with
exactly the plan's four-key block (`bundle.licenseFile: null`,
`bundle.linux.deb.recommends: null`). The identical bundle command succeeded (no
validation error, bundle produced), and the identical grep printed **`0`**.
Positive control in the same output: `grep -cE '^ Depends'` -> `1`, line still
` Depends: libwebkit2gtk-4.1-0, libgtk-3-0`. Control archive shrank to
`406 bytes, 11 lines control` - exactly one line removed.

The absence result is fire-verified by construction: the same grep invocation
returned `1` on the baseline minutes earlier, in the same shell, against the same
path glob.

**Step 3, cleanup (alias-proof).** `command rm -f src-tauri/tauri.linux.conf.json
&& command rm -rf target/debug/bundle`; `git status --porcelain` and
`git diff --stat` both empty against the pre-mutation state (HEAD `5060ef5`, tree
clean). That empty `git status` is itself fire-verified: the same invocation run
minutes later, with the new `src-tauri/tauri.macos.conf.json` on disk, printed
`?? src-tauri/tauri.macos.conf.json`.

**Step 4, branch rule:** Step 2 printed `0` -> **CLEAR BRANCH** (Steps 5-8).
No NEEDS_CONTEXT trigger fired. The fallback branch was not executed and
`packaging/macos-license.rtf` was not created.

## Edits

1. **`src-tauri/tauri.macos.conf.json` (new, Step 5)** - exactly
   `{"bundle": {"licenseFile": null}}`, four lines plus trailing newline, no
   `$schema` (matches `src-tauri/tauri.bundle.conf.json`'s house shape, which was
   read to confirm). JSON validity checked with `python3 -c json.load`: parses to
   `{'bundle': {'licenseFile': None}}`, key present, value `None`.
2. **`BUILDING.md` (Step 6)** - the clear-branch paragraph inserted verbatim in
   "Reproducing a release bundle locally", between the sidecar-overlay paragraph
   ending `... so dev/test builds need no staging step.` and `To reproduce what CI
   ships:`, with a blank line on each side (the two were previously one
   unseparated block).
3. **Design doc amendment log (Step 7)** - A5 appended verbatim after A4, no
   composition. `LC_ALL=C grep -n '[^ -~]'` over the appended block returns exactly
   one hit, the `Ş` in "the third `Ş` sink"; no smart quotes, no long dashes, no
   Unicode ellipsis smuggled in. A5's two tree references were spot-checked to
   exist: `### 4.4 \`BUILDING.md\` rider (D82/D86, addition, complete)` and
   `## 11. What the implementer must not decide`.
4. **Design doc status line (routed instruction, same file)** - the stale
   amendment enumeration removed, not updated. Old:
   `... ; amendment log at\nthe end (A1, 2026-07-23; A2 and A3, 2026-07-27). Numbering starts`.
   New: `... ; every amendment\nis recorded in the \`## Amendment log\` section at the end. Numbering starts`.
   The DRAFT date, the fix-round note and the whole D-numbering paragraph are
   byte-unchanged (visible in the two-line diff hunk). Post-state
   `command grep -c 'A2 and A3, 2026-07-27' <design>` -> `0`, fire-verified by the
   identical invocation piped from `git show HEAD:<design>` -> `1`. Pointer target
   verified present: `command grep -c '^## Amendment log$'` -> `1`.

**Step 8:** `./node_modules/.bin/tauri inspect wix-upgrade-code` exits 0 and prints
the pinned GUID `9262b417-b687-5ea3-ace1-18b9d51b215f` on both `Info` lines. Per
the plan, this makes no claim about the macOS merge - on Linux the platform file
is not loaded; its live proof is Task 4 RR4 and owner step O2.

## Consumer check for the removed enumeration (the routed instruction's gate)

The controller's ruling stated `git grep` finds no consumer of the enumeration
anywhere in `docs/`. Verified before cutting, and the phrasing needs one
correction: **`git grep` does return hits outside the design doc** - just no
consumers.

- `git grep -n 'amendment log at'` -> 1 hit, the design's own line 3.
- `git grep -n 'A1, 2026-07-23'` -> the design line plus
  `docs/process-journal/artifacts/plan-8-sdd/owner-wording-verdict-delta.md:182`
  and `.../wording-fix-round-report.md:186,:188`.
- `git grep -n 'A2 and A3'` -> the design line plus
  `.../wording-fix-round-report.md:188` and
  `docs/process-journal/artifacts/handoffs/2026-07-27-session-23-close.md:135`.
- A broader sweep for `status line` / `header.*amendment` over `docs/`, `.github/`
  and the root `*.md` returned hits only inside `docs/process-journal/artifacts/**`.

Every one of those is a **dated historical record narrating the earlier edit**
("it now reads ..."), in the class the plan's Task-1 sweep classification declares
out of surface ("dated historical records; supersession lives in the live
surfaces"). None reads the enumeration as a current fact, so none breaks when it
goes; they stay accurate accounts of 2026-07-23 / 2026-07-27. No live surface
(ROADMAP, house YAMLs, plans, BUILDING/INSTALL/README, workflows) references the
header list at all. The three greps above are demonstrably live invocations: each
returned hits.

## Premises checked against the tree

- **Authoring-time experiment result: CONFIRMED, not refuted.** `1` -> `0`,
  positive control intact, CLI accepted the null-carrying config. The clear branch
  executes exactly as the plan predicted.
- **Controller's "no hits" phrasing on the enumeration: imprecise, ruling
  unaffected.** Corrected above - hits exist in frozen journal artifacts, none is a
  consumer.
- **Note, not a defect:** `tauri bundle -d` patches `target/debug/muxsmith-gui` in
  place (`Info Patching ... with bundle type information: deb`). `target/` is
  gitignored and the plan's Step-3 cleanup scope covers only
  `target/debug/bundle`, so the patched debug binary remains, as it did at
  plan-authoring. No tree state, no gate surface.

## Not claimed

Nothing here verifies ruling 2. This task produced the config; the macOS-side
effect is Task 4's RR4 machine half, and the acceptance is owner step O2 (no
pre-mount license dialog on the re-run dmg, on Apple-Silicon hardware). The
Windows msi is untouched by construction - the global `licenseFile` and the whole
Windows config are byte-identical in this commit's diff.
