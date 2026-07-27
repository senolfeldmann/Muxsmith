# Whole-branch verdict: Plan 8 (packaging / release pipeline), range aec4cef..7302e1b path-scoped

Reviewer: independent whole-branch reviewer (fresh context, wrote none of the
code). Brief: `whole-branch-brief.md`. Every command foreground, main worktree,
absolute paths; writes: this file plus `gh-log.md` appends (3 entries for the
read-only gh/api queries) and scratchpad files. All temporary fire-test
mutations were backup-verified restorations (`command cp -f`, `cmp` against a
pre-mutation backup, `git status` clean on every pathset file afterwards).

## VERDICT: NEEDS FIXES

Nothing here questions the shipped pipeline: the rehearsal is genuinely green,
the workflow is byte-identical to its contract, the pins resolve, the guard
and linter run green locally and in CI on the pinned head. What blocks the
close is the already-deferred documentation debt the run itself routed to this
review (BUILDING.md two stale sites + the owner-approved tenth gate part, the
design's post-WiX-fix stale sites, the D86 supersession bookkeeping) plus one
small shipped-code regression (ledger-lint's escaped `ReaderError`) and one
dropped review minor. All of it is one fix-wave dispatch plus controller
close-batch lines; none of it invalidates the rehearsal evidence.

---

## 0. Scoping verification

Done first, as ordered.

- **Diff regeneration: byte-identical.** `git diff aec4cef..7302e1b` over the
  brief's 16-entry pathset regenerated and `cmp`-compared against
  `whole-branch-8.diff`: identical, 31522 bytes both.
- **HEAD state.** At review time `git rev-parse HEAD` =
  `7302e1b4bd705c2431a34a28249ca39e49539f4c` = the pinned SHA; no commit after
  the pin touches anything. **No plan-8 pathset file changed after `7302e1b`**
  (nothing to change it; working tree clean on the pathset throughout).
- **Disjointness verified from `git log aec4cef..7302e1b --name-only`**
  (174-line listing, every file classified):
  - Plan-8 pathset: all 16 brief-named paths appear only in plan-8 commits
    (7e36f96, c884bd6, c890b0f, fe46424, 92c62f1, the merges, 07c0255 with
    `src-tauri/wix/locale-en-US.wxl`, and the two controller fixes c06b8dd /
    f4f932e).
  - Plan-7.5 pathset: `src/views/EditorView.vue`, `e2e/editor-rule-add-remove.spec.ts`,
    `e2e/help-mode.spec.ts`, `help/**`, the 7.5 design, and the v1-spec edits.
    The two v1-spec commits (70282fd, 406e91b) were opened and checked: both
    edit section 8.2 / 5.2 territory only (406e91b's single spec hunk is the
    8.2 view-1 paragraph), inside the declared 7.5 scope.
  - Everything else in range is process artifact: the four house-knowledge
    YAMLs, `docs/process-journal*`, handoff snapshots, and the two plan
    designs' own amendments (plan-8 A1 `d21a19f` - a named ground-truth
    amendment; plan-7.5's date fix).
  - **No product file in range falls outside both pathsets.**
- **Mid-review observation (recorded, no action):** during this review the
  working tree acquired uncommitted modifications to `docs/ROADMAP.md`,
  `docs/conventions.yaml`, `docs/decision-ledger.yaml` - the parallel plan-7.5
  close writer the brief predicted (it predicted commits; these arrived as
  working-tree edits first). None is in my pathset; my two fire-test mutations
  targeted `docs/product-boundaries.yaml`, which stayed untouched by the
  parallel writer (clean against HEAD after my cmp-verified restores).
- CI on the pinned head: run 30276189309, completed/**success**, including the
  `ledger-lint` job (read-only query, gh-logged). The two previously-OPEN push
  watches (f97776e, 7bed02d) are also success - the controller's open item
  from the S22 close can be marked observed.

---

## Dimension findings

Numbered W1..W6; severity per the brief (Blocker / Important / Minor).
Findings that duplicate an adjudication item are stated there and only
referenced here.

**W1 (Important) - shipped-code regression in `scripts/ledger-lint.py`: the
loader constructor sits outside the parse `try`.** Reproduced myself
(dimension 6 protocol, backup-verified restore): a `\x08` planted at offset 0
of `docs/product-boundaries.yaml` produces an uncaught
`yaml.reader.ReaderError` traceback from line 97 (`DuplicateKeyLoader(text)`),
no `FAIL` line, no summary - exit 1, so CI stays red and the gate does not
fail open, but the linter's own contract ("does not parse" violation line) is
broken for this input class. **The record mischaracterizes it**: progress.md:20
and the ledger occurrence (`inline-wrapper-keeps-try-scope`,
decision-ledger.yaml:4223) say "pre-existing on master". Verified false: the
pre-plan-8 script (`git show aec4cef:scripts/ledger-lint.py`) wraps
`yaml.safe_load` inside the `try` (line 57), and `yaml.reader.ReaderError` is
a `yaml.YAMLError` subclass (checked live: `issubclass(...) == True`), so old
master caught it cleanly - the task-5 verdict's own m1 reproduction says
exactly this. The escape was **introduced by 92c62f1**. Disposition:
adjudication 3 (fix wave) + a close-batch correction of the ledger occurrence
wording.

**W2 (Minor, NEW - dropped review finding).** Task-5 verdict m3 ruled the new
docstring sentence overstates the CI trigger set
(`scripts/ledger-lint.py:30-31`: "on every push and pull request", while
ci.yml gates only master pushes, `v*` tags, PRs, dispatch) and said plainly
"this is on the task. Fix: ...". The task closed APPROVED, progress.md routed
m1 and m2 to whole-branch triage - **and m3 to nowhere**. The current file
still carries the wrong sentence. Disposition: folded into the fix wave's
ledger-lint item (same file, one edit). Process residue in HARVEST.

**W3 (Minor) - two sites of the recorded stale-language list are wrong, one
real site is missing.** Content-located every recorded candidate (adjudication
7): design :1012 is **not** stale ("`wix.language` fixes the msi UI language
contract at en-US" stays true under the map - the key is still en-US), and
plan :718's trigger-7 line carries no mechanism premise at all - but design
**:1966** ("the mechanism is a config list") does, and is on no recorded list.
Details and replacements in Adjudication 7.

**W4 (Minor) - `design-acceptance-observables-have-producers`, R1 wording.**
The one surviving observable/emitter imprecision (design :1880-1881 "names the
found ci run" vs the emitter's SHA echo) - already ledgered
(decision-ledger.yaml:4052). Ruled in Adjudication 4.

Dimensions with **no finding**, evidence stated:

- **Cross-task integration (dim 1): PASS.** The version chain holds on merged
  state: `Cargo.toml` `[workspace.package] version` 0.1.0 -> `tauri.conf.json`
  carries no `version` key -> guard script asserts exactly that (my green run:
  `version-sync: OK (0.1.0)`, exit 0; tag arm fired on `v9.9.9`, exit 1) ->
  release.yml's two awk parsers are byte-identical to the script's ->
  D89 names in INSTALL.md/draft-body/tar.gz README all use
  `muxsmith-<version>-<os>-<arch>.<ext>` -> the rehearsal produced exactly
  `muxsmith-0.1.0-*` (R2's seven names, verified in the report and sampled
  live: draft still holds 8 assets). Every run-time consumer contract
  resolves in the tree: `scripts/check-version-sync.sh` (release.yml:38/40,
  executable bit set), `src-tauri/tauri.bundle.conf.json` (:118),
  `.github/release/rehearsal-banner.md` (:215), `draft-body.md` (:217),
  `packaging/linux-tarball-README.txt` (:173), `wix/locale-en-US.wxl`
  (tauri.conf localePath, proven by two green Windows legs), `.gitignore:16`
  `src-tauri/binaries/`, INSTALL.md's three `##` headings = the three anchor
  targets.
- **Design/spec/tree three-way (dim 2): PASS with recorded divergences.**
  D75-D90 walked against the tree; every decision is implemented as written
  (release.yml is byte-identical to the design's section-2 fence - 222 lines,
  diff empty, A1's quoted step name included; tauri.conf matches 3.1 except
  `wix.language`, whose divergence is the owner-authorized WiX fix with its
  bookkeeping ruled in Adjudications 6/7). v1 spec §10's packaging sentence
  ("msi, dmg, deb, rpm, AppImage on release tags") is satisfied; the added
  tar.gz is an owner-ruled superset, no contradiction; `cross-06-no-bundling-v1`
  holds (mkvtoolnix is Recommends, never bundled).
- **Acceptance set (dim 3): PASS on sampling.** Re-verified at the named
  emitters, read-only: draft `rehearsal-30273529210` exists, `isDraft: true`,
  8 assets, untouched; runs 30272619000/30273529210 both success. The report's
  transcripts are consistent with its verdicts throughout, and its own
  falsifiability controls (byte-flip on SHA256SUMS, `__VERSION__` grep
  fire-verified at 8, synthetic rehearsal-ref control) are the strong form.
  **Not verifiable outside real hardware**: R8 entirely (SmartScreen flow,
  Programs-and-Features rendering, Gatekeeper flow, dmg drag-install), the
  macOS CLI path claim in INSTALL.md, and the license-dialog rendering
  (Adjudication 9). I claim no coverage there.
- **Pinning/supply chain (dim 4): PASS.** All five action pins resolved live
  (gh api, logged): checkout v7.0.0 = `9c091bb2...`, setup-node v7.0.0 =
  `82076278...`, pnpm/action-setup v6.0.9 = `0ebf4713...`, upload-artifact
  v7.0.1 = `043fb46d...`, download-artifact v8.0.1 = `3e5f45b2...` - each
  matching its SHA + version comment. Pin set is exactly the enumerated one;
  banned-shape grep on release.yml empty with a firing positive control on
  ci.yml (2 hits: mise-action, Swatinem). The three recorded floats
  (windows-11-arm undated, runner gh CLI = trigger 9, ci.yml python
  interpreter) all carry their recorded comments.
- **ci.yml additivity (dim 5): PASS.** `--numstat` on the range:
  `17 0 .github/workflows/ci.yml`; zero removed lines
  (`grep -c '^-[^-]'` = 0); single appended hunk at EOF
  (`@@ -160,3 +160,20 @@`), job block identical to the plan's Step-3 block.
- **Local runnability (dim 6): PASS.** version-sync green + tag-arm red (both
  observed); `python3 scripts/ledger-lint.py` green (467 entries, exit 0),
  fire-verified: planted duplicate `steelman` -> `FAIL ... (lines 22 and 23)`,
  exit 1; restored, green again; YAML parse of release.yml + ci.yml clean.
  Plus the W1 reproduction above.
- **House dimension (dim 7): no unrecorded deviation found.** The one
  convention-relevant defect class in range (stale enumerations after a set
  gains a member - BUILDING.md:92-95, the design's language sites) is exactly
  what the deferred items already carry; no new entry-id violation surfaced.
- **Surviving latitude (dim 8): none found beyond the ledgered R1 wording
  (W4).** The two forks the run hit on contact (A1's unparseable scalar, the
  WiX code page) both returned as NEEDS_CONTEXT/owner rulings rather than
  keyboard decisions - the ban held in practice.
- **No-work-needed check (dim 9): premises run, all hold.** D80's "no
  post-processing needed" (schema recommends fields) - proven by R6's actual
  `Recommends: mkvtoolnix` in the built deb/rpm. D76's absence - guarded by a
  step with a built-in positive control, G4-fire-verified, N>0 on all four
  legs twice. Section-0 note 5's "no NEEDS_CONTEXT needed" (arm64 msi) -
  proven by a green windows-11-arm leg. D82's "nothing spawns it, so no
  shell-plugin capability" - verified: no shell/sidecar reference in
  `src-tauri/capabilities/default.json`, `tauri.conf.json`, or
  `src-tauri/src/`. A1's "no R-observable quotes the YAML line" - checked: R5
  quotes only the step NAME string, which A1 left byte-identical.
- **Documentation truthfulness (dim 10): PASS except the pre-recorded items.**
  Every factual claim in INSTALL.md/tar.gz README checkable from tree or
  rehearsal evidence holds (per-machine `PFiles/Muxsmith` via msiextract, both
  binaries in every Linux payload, Recommends/Depends as documented, tar.gz
  4-file layout, `muxsmith 0.1.0` self-report). The stale sites are
  BUILDING.md's two (Adjudication 1) - both pre-recorded, nothing new found.

---

## Adjudication

**1. BUILDING.md, two stale sites - fix-before-close** (both content-verified).

(a) Current text, BUILDING.md:65-68, quoted:

> `pnpm build` only builds the frontend bundle (`dist/`); it does not invoke
> `cargo tauri build`. Building the desktop bundle itself
> (`pnpm exec tauri build`) is out of scope for local development and not
> part of the CI gate yet.

Exactly as the corrected record says: only "out of scope for local
development" is now false (the file's own §"Reproducing a release bundle
locally" documents that invocation); "not part of the CI gate" stays true.
Replacement (drop the "yet", which implied bundling would join the gate -
release.yml is deliberately not the gate, D83):

```markdown
`pnpm build` only builds the frontend bundle (`dist/`); it does not invoke
`cargo tauri build`. Building the desktop bundle itself
(`pnpm exec tauri build`) is not part of the CI gate; release bundles are
built by `release.yml` on `v*` tags and manual dispatch, and "Reproducing a
release bundle locally" below covers the local invocation.
```

(b) Current text, BUILDING.md:92-95, quoted:

> CI (`.github/workflows/ci.yml`) runs the same five-part Rust gate plus
> `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and `pnpm test:e2e` on every
> push/PR (nine parts total); `cargo deny check` runs as an
> independent job.

Stale twice over: `ledger-lint` is now a second independent job, and "every
push" overstates the trigger set (master pushes, `v*` tags, PRs, dispatch).
Replacement below is written to compose with item 2's ten-part change:

```markdown
CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-5 natively on all
three OS legs (its Windows leg covers natively what part 6 cross-checks
from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and
`pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
and `scripts/ledger-lint.py` (house-knowledge invariants, Plan-8 rider)
run as independent jobs.
```

**2. Tenth gate part: cross-target Windows clippy - fix-before-close**
(owner-approved, progress.md:52 ruling 7). Determined empirically, not
relayed:

- **Command:** `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`
- **Runnable on a Linux dev machine: YES, verified on this one.** Green in
  1.34s on a warm cache (cold is a one-time dependency type-check, minutes
  not hours); clippy is check-only, so no Windows linker or MSVC toolchain is
  needed. Two harmless build-script warnings appear
  ("GNU compiler is not supported for this target", from the resource
  embedder; non-fatal, cosmetic).
- **Fire-verified that it catches the motivating class:** temporarily removed
  the `#[cfg(unix)]` gate on the `ParamValue` import in `src-tauri/src/lib.rs`
  -> `error: unused import: crate::error::ParamValue` at :593, compile fails;
  restored (cmp-verified), green again. The command demonstrably reproduces
  the five-run-unobserved CI red locally.
- **Prerequisite:** `rustup target add x86_64-pc-windows-msvc`, one-time per
  machine (`rust-toolchain.toml` pins channel + components but **no**
  `targets`, so nothing auto-installs it). It belongs in BUILDING.md's
  Prerequisites section, not inline in the gate block.
- **Open fork, returned rather than resolved:** `rust-toolchain.toml` accepts
  a `targets = ["x86_64-pc-windows-msvc"]` key that would make rustup
  auto-install the target for every contributor - mechanically cleaner, but
  it changes toolchain state on every machine that builds the repo, which is
  beyond the ruled BUILDING.md scope. **Owner/controller call; the text below
  assumes the documented-prerequisite form.**

Exact BUILDING.md changes (three edits, one file):

(i) Under `### Rust toolchain` (after the existing paragraph), add:

```markdown
The pre-push gate's cross-target clippy part needs the Windows target's
standard library once per machine: `rustup target add x86_64-pc-windows-msvc`.
```

(ii) Heading `### The Rust gate (five parts, run from the repo root,
workspace-wide)` becomes `### The Rust gate (six parts, run from the repo
root, workspace-wide)`, and the fence gains as its new last line:

```bash
cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
```

with this paragraph after the fence's existing prose:

```markdown
The cross-target clippy run (part 6) type-checks the workspace for Windows
without linking, so it runs on any OS. It catches what a host-only clippy
cannot see - cfg-gated imports and Windows-only lints - the class that went
CI-red twice in Plan 5 and sat unobserved for five runs in Plan 8 (an
in-tree comment is not a gate; owner ruling S22). CI needs no equivalent
step: its Windows leg runs clippy natively.
```

(iii) Delete the `## Cross-target lint rule` section (BUILDING.md:140-145) -
its content is now part 6 of the gate and its "before the first push that
hits a new OS leg" trigger is superseded by "every gate run". Its rationale
(the Plan-5 incident) is carried into (ii)'s paragraph, so nothing is lost.
This deletion is my completion of the ruled change, marked so the controller
can veto it; keeping both texts would say the same rule twice with weaker
force.

Consuming-line sweep for the nine->ten count, run: BUILDING.md:94 is
item 1(b)'s replacement (which no longer states a total); ci.yml:88's "ninth
gate part" comment is a dated historical record of Plan 5.5, accurate as
history; ROADMAP:128/:439/:582 are episodic status lines; the plan-8 plan is a
frozen execution artifact. No other living normative site states the count.

**3. `ledger-lint.py` `ReaderError` - fix-before-close** (W1). Reproduced;
the plan touched the file, the fix is ten lines and already written verbatim
in task-5-verdict m1 (construction moves inside the `try`, `loader = None` +
`finally: if loader is not None: loader.dispose()`). Apply exactly that.
Same edit fixes W2's docstring sentence: `scripts/ledger-lint.py:30-31`
"on every push and pull request" -> "on every master push, `v*` tag and pull
request". Re-run the three fire-tests after (duplicate key, control char ->
now a clean `does not parse` FAIL, green). Additionally, close-batch: correct
the `inline-wrapper-keeps-try-scope` occurrence's "pre-existing on master,
reproduced there" to "introduced by 92c62f1 (master at aec4cef caught it via
safe_load's internal try)" - a house-YAML line, controller's routing.

**4. R1 observable wording - close-batch doc one-liner; the design text is
the wrong side.** Both sites located by content: design :1880-1881 ("the
gate-green step's log names the found ci run") vs the emitter release.yml:54
(`echo "ci gate green for $GITHUB_SHA"`). The emitter is right: the SHA is
the actual gate key (the guard's contract is "a green ci run exists for this
exact SHA"); a run id would add nothing the SHA does not already pin, and
changing release.yml costs a GitHub round-trip re-verification for zero
behavioral gain. Replacement for design :1880-1881:

```markdown
- **R1** (run A): guard passes both steps; the gate-green step's log names
  the gated SHA (the `ci gate green for <sha>` echo). All four legs green; ...
```

The plan's transcribed copy (:627-628) is a frozen execution artifact retired
at the close; it is not edited (its transcription claims would break).
Already ledgered as an occurrence on
`design-acceptance-observables-have-producers` (decision-ledger.yaml:4052);
no further ledger action owed.

**5. R6 deb-payload path form - close-batch doc one-liner.** All affected
sites, located by content (`./usr/bin/muxsmith` grep over docs/): design
:1907 and the plan's frozen transcription :654 - **two sites, no more** (no
INSTALL.md or README site uses the `./` form). Replacement for design :1907:

```markdown
  `dpkg-deb -c` payload paths include `usr/bin/muxsmith` and
  `usr/bin/muxsmith-gui` (dpkg >= 1.22 emits no leading `./`);
```

Plan copy: not edited, same reason as item 4.

**6. D86 publisher fallback, superseded on the merits - verified, and the
claim holds.** I verified the load-bearing claim from the wix-fix evidence
itself, not from the summary: the diagnosis run's channel (a) transcript
(wix-fix-report §1) shows the ASCII A/B step rewrote **both** `publisher` and
`copyright` to ASCII and the rebuild still failed -
`LicenseAgreementDlg.wxs(27): error LGHT0311` with exit 1 and an empty msi
output dir (`total 0`) - because the third sink is the LICENSE text WiX
inlines into the license dialog. Three sinks (publisher 8 rows, LICENSE 1
row, copyright none), so D86's publisher-only ASCII fallback could never have
produced a building msi; the code page is the instrument (07c0255, both legs
green, PID_CODEPAGE 1254, Ş intact - independently re-verified by the wix-fix
reviewer from re-downloaded MSIs). Proposed ADR bookkeeping (rides the fix
wave's design edit as amendment-log entry **A2**, which also carries item 7):

```markdown
**A2 (2026-07-27, owner ruling 6 + post-rehearsal record).** The WiX fix
`07c0255` changes two D86 literals: `windows.wix.language` becomes the map
`{ "en-US": { "localePath": "wix/locale-en-US.wxl" } }`, and the new
committed file `src-tauri/wix/locale-en-US.wxl` (Codepage 1254) joins the
D86 config surface. D86's pre-decided publisher fallback (ASCII
"Senol Feldmann" in `publisher` only, triggered by R8) is **superseded on
the merits**: the diagnosis run (30268008932) demonstrated three Ş sinks -
publisher (8 msi string rows), the LICENSE text inlined into
LicenseAgreementDlg (1 row), copyright (never reaches WiX) - and its ASCII
A/B control rebuilt with ASCII publisher AND copyright still failed on the
LICENSE sink, so the fallback could not have fixed the build; the database
code page is the instrument. R8 remains the owner's rendering check; if it
ever fails, the answer is code-page work, not transliteration. Sites
updated by this amendment: D86 decision text (:941-943 fallback clause,
:958 language value), section 3.1 fence, section 11 (frozen-literal list,
fallback bullet), section 8 R8 parenthetical, section 9 trigger 7 premise.
```

The design's dependent fallback sites (:941-943, :1921, :2022-2024) get a
"(superseded, A2)" touch in the same edit - enumerated so no implementer
sweeps blind. The plan's fallback sites (:25, :94, :668, :707, :716) are the
frozen copy; the close's owner-steps line should quote A2, not the stale
protocol, when the controller puts R8 to the owner.

**7. Stale `"language": ["en-US"]` sites - fix-before-close (owner-authorized,
ruling 6), with two corrections to the recorded list.** Located every site by
content; what I actually found:

| recorded site | verdict | replacement |
|---|---|---|
| design :958 | **stale** | `` `windows.wix.language`: `{ "en-US": { "localePath": "wix/locale-en-US.wxl" } }` (A2) - the installer UI language set; the locale file pins the MSI database code page to 1254 so `Ş` survives WiX's 1252 en-US default; `` (rest of the sentence unchanged) |
| design :1012 | **NOT stale** - "fixes the msi UI language contract at en-US" stays true under the single-key map | no change needed |
| design :1511 (3.1 fence) | **stale** | `        "language": { "en-US": { "localePath": "wix/locale-en-US.wxl" } }` |
| design :2007 (section-11 frozen list) | **stale** | `...minimumSystemVersion \`11.0\`, the en-US language map with its locale file \`wix/locale-en-US.wxl\` (A2));` |
| plan :248 (fence) | stale, frozen copy | owner-authorized per ruling 6 / verdict F2 routing: apply the same one-line fence substitution as design :1511, or leave frozen at the controller's discretion - fork returned, see below |
| plan :262 (frozen list) | stale, frozen copy | same handling as :248: "the en-US language list" -> "the en-US language map (A2)" |
| plan :718 (trigger 7) | **NOT stale** - the line is a bare pointer ("reopen D86's `wix.language`"), no mechanism premise | no change needed |
| design :1966 (**missing from the recorded list**) | **stale** - trigger 7's "(the mechanism is a config list; ...)" | `(the mechanism is a per-language map carrying a locale file - see A2; the cost is more msi artifacts or a transform decision, which is why it waits for a request)` |

Net: five real sites (design :958, :1511, :2007, :1966 + the A2 entry
itself), two recorded sites need no change, one real site was unrecorded.
**Fork returned to the controller:** items 4/5 rule "frozen plan copies stay";
verdict F2 and progress:56 explicitly route plan :248/:262 into the close
batch as owner-authorized edits. Both positions are defensible (living-doc
sync vs frozen-transcription integrity); I did not resolve it - the
controller picks one rule and applies it to all plan-copy sites uniformly.

**8. Bundler version citation sweep - no change needed.** Both numbers
established myself: `@tauri-apps/cli` **2.11.4** from `package.json:30` and
`pnpm-lock.yaml:403`; `tauri-bundler` **2.9.4** from the upstream tag
(`raw.githubusercontent.com/.../tauri-cli-v2.11.4/crates/tauri-cli/Cargo.toml`
line 50, fetched live), and confirmed `tauri-bundler` appears in **no** local
lockfile (structurally impossible: it ships compiled into the npm CLI binary;
the same grep form fires on `@tauri-apps/cli` in `pnpm-lock.yaml` as the
positive control). **Search surface, named:** all tracked `docs/` (including
the v1 spec and ROADMAP; `docs/process-journal` excluded as dated episodic
record), `BUILDING.md`, `README.md`, `docs/INSTALL.md`, the four house YAMLs,
and every `.superpowers/sdd/plan-8/*.md` via `command grep` (gate-logs
excluded: their `tauri-runtime-wry v2.11.4` hits are a genuinely distinct
crate at that version). Result: 11 narrative hits total, each classified -
every one is either a correct tag/CLI attribution (design 1.2 heading, D84,
plan tech stack, review rounds) or the correction record itself
(wix-fix-report §0, wix-fix-verdict §2, progress:53). The original
miscitation ("the PINNED tauri-bundler 2.11.4") lived only in the wix-fix
dispatch brief, which is not a persisted file. Zero live miscitations;
progress:53's residual (b) can be marked done-by-verification at the close.

**9. `ansicpg1252` cosmetic residual - close-batch one-liner, into R8's
owner checklist.** The brief is right that a frozen report is history, not a
backlog. But this needs no new ROADMAP trigger either: the observable event
already has a pending owner step attached to it - R8's Windows walk-through
puts the owner in front of the license dialog (LicenseAgreementDlg renders
during the msi install he is instructed to perform). One-liner for the
close's R8 item:

```markdown
R8 addendum: during the Windows msi install, check the license dialog's
text renders "Şenol Feldmann" correctly (upstream tauri-bundler hardcodes
`\ansicpg1252` in the generated RTF header while the body is CP-what-WiX
stored; unverified without a real install - wix-fix report, residual). If
mangled: cosmetic, upstream; file/track as a tauri-bundler issue or v1.x
note - the installed product is unaffected.
```

**10. Task-1 i3 (no acceptance observable reads a bundler-produced version) -
close-batch one-liner.** What it actually is, verified: a formal gap that the
executed rehearsal already closed incidentally. R9 reads clap (a Cargo-native
path that cannot catch a Tauri-fallback failure); R6 as **designed** reads
Recommends/Depends/payloads, no version field - so the designed set indeed
never reads a bundler-produced version. But the **executed** R6/R1 evidence
does carry it three ways: `dpkg-deb -I` printed `Version: 0.1.0`, `msiinfo`
printed `ProductVersion 0.1.0`, and all six `pick:` lines log bundler-native
names carrying `0.1.0`. The mechanism is additionally source-verified at the
pinned crates (task-1 verdict: tauri-codegen falls back to
`CARGO_PKG_VERSION`). Ruling: no new acceptance item, no trigger; the
cheapest producer-backed closure is the task-1 verdict's own suggestion as a
design R1 one-liner (rides the same design edit):

```markdown
R1 addendum: each `pick:` path contains the guard's version (the
bundler-native filenames carry the bundler-produced version, so this line
is the artifact-level check that bundle metadata inherited the workspace
version, D87).
```

**11. Joblog comment note - record, not fix; vehicle: the ledger.** The four
sites (joblog.rs:70, run.rs:1340/:1499/:1510) pass stale stamps as `create`'s
`run_id` argument, safe via the implicit prune-before-leaf invariant; the
joblog-fix verdict verified all four and its H4 names the promotion path.
Three of the four live in `src-tauri/src/run.rs`, **outside the plan-8
pathset** - a comment edit there is not this plan's fix wave. Ruling:
recorded for next touch, concretely as a new occurrence (or facet) on
`test-fixture-dates-outside-retention-windows`:

```yaml
- {date: "2026-07-27", kind: reinforced, ref: "plan-8 joblog-fix verdict §2+H4 (joblog-datebomb-fix-verdict.md): four sites (joblog.rs:70, run.rs:1340/:1499/:1510) pass stale stamps as create's run_id ARGUMENT, safe only via the implicit prune-before-leaf invariant; on next touch of either file, state the invariant in a comment at the first such site - a second create against the same root re-creates the class silently"}
```

(Controller applies at close; ledger lines are controller writes.)

**12. D75 cross-reference - fix-before-close design one-liner, verified.**
Design :303-304 reads "(Apple citation in section 1)"; the Apple citation
(developer.apple.com/news/?id=saqachfa) lives in **section 0**, note 2's
Reality cell (line 51). Section 1 carries no Apple/Gatekeeper reference
(grep over :58-278: zero hits; the same pattern finds line 51). Replacement:

```markdown
  then System Settings > Privacy & Security > "Open Anyway" (Apple
  citation in section 0, note 2); macOS 11-14: Control-click -> Open; ...
```

**13. The nine owner wording items - all nine confirmed still open, still
correctly owner-routed.** Each content-checked against the current tree
(none silently landed; the fix wave must not touch them - they are owner
calls under design section 11). Compact restatement for the controller to put
to the owner:

1. `sha256sum -c` is given as the all-OS verification (INSTALL.md:6-8,
   draft-body:21-22); Windows has no such command - consider one PowerShell
   line (`Get-FileHash`/`certutil`) in the Windows section.
2. INSTALL.md:80-83: bullet labeled "GUI only, deb/rpm/tar.gz" but its body
   explains the AppImage - widen the label or move the clause.
3. INSTALL.md:59: `sudo ln -s ... /usr/local/bin/...` can fail on a clean
   Apple-Silicon Mac (`/usr/local/bin` may not exist) - prefix
   `sudo mkdir -p /usr/local/bin &&`; confirmable during R8.
4. INSTALL.md:32-33: "select `Path`" does not say USER vs System pane
   (D82 says user PATH).
5. INSTALL.md:6: "the one-time step per OS" (singular) - Windows has two
   steps, macOS up to three; plural reads truer.
6. INSTALL.md:10: "Every install ships two programs" - literally true of the
   AppImage, but the supported Linux CLI channels are deb/rpm/tar.gz (D82);
   may mislead AppImage users.
7. draft-body.md:2-4: continuation lines beginning with `|` are lazy GFM
   continuations, should render inline - confirm on the rendered draft
   (R5/R8 inspection already shows it to the owner).
8. Optional: carry D82's no-sudo macOS alternative ("add `Contents/MacOS`
   to PATH") into INSTALL.md - pairs with item 3.
9. tarball README:22: "glibc from Ubuntu 22.04 (2022) or newer" - name the
   actual floor (glibc 2.35) beside the shorthand.

---

## Fix wave

Consolidated, in application order, dispatchable as one task. Items A-C are
product/doc file edits; D is the design document (owner-authorized where
noted); E enumerates what is deliberately NOT in the wave.

**A. `scripts/ledger-lint.py`** (adjudications 3 + W2):
1. Move `DuplicateKeyLoader(text)` construction inside the parse `try`
   exactly per task-5-verdict m1's snippet (`loader = None` before the try;
   `finally: if loader is not None: loader.dispose()`).
2. Docstring :30-31: "on every push and pull request" -> "on every master
   push, `v*` tag and pull request".
3. Re-run fire tests: planted control char now yields
   `FAIL ...: does not parse (...)` + summary + exit 1; planted duplicate key
   still fires; green reachable (467 entries at this writing; recount).

**B. `BUILDING.md`** (adjudications 1 + 2, one file, four edits):
1. :65-68 replacement (item 1a text).
2. :92-95 replacement (item 1b text).
3. Tenth gate part: prerequisites line, heading five->six parts, the clippy
   `--target x86_64-pc-windows-msvc` line + rationale paragraph (item 2
   text).
4. Delete `## Cross-target lint rule` (:140-145), superseded by 3
   (controller may veto; then reconcile the two texts instead).

**C. Verification for A+B:** nine-part gate is NOT owed for a docs+linter
fix; run `python3 scripts/ledger-lint.py` (green), the three fire tests
above, and the new gate part once
(`cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`,
expect green; it ran green on this machine today).

**D. Design document
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`**
(owner-authorized where ruling 6 applies; one commit):
1. Amendment-log entry **A2** (adjudication 6 text).
2. :958 language-map replacement; :1511 fence line; :2007 frozen-list item;
   :1966 trigger-7 premise (adjudication 7 texts).
3. Fallback-clause touches at :941-943, :1921, :2022-2024 -> "(superseded,
   A2)" (adjudication 6).
4. R1 wording :1880-1881 (adjudication 4 text) + R1 addendum line
   (adjudication 10 text).
5. R6 dpkg path form :1907 (adjudication 5 text).
6. D75 cross-ref :304 "section 1" -> "section 0, note 2" (adjudication 12).
   All :line references above re-verified by content today; locate by content
   at application time.

**E. Close-batch (controller lines, not the fix implementer):** ledger
occurrence correction on `inline-wrapper-keeps-try-scope` (W1); the
`test-fixture-dates-outside-retention-windows` occurrence (adjudication 11);
R8 checklist addendum for the license dialog (adjudication 9); progress
residual (b) marked done-by-verification (adjudication 8); the nine-item
owner list (adjudication 13); ROADMAP mirroring of the design's 9 triggers +
rider DONE notes + the tenth-gate-part ruling record (plan close section,
pre-existing duties). **Fork returned (adjudication 7):** whether the frozen
plan's :248/:262 get the same substitution (F2/progress routing) or stay
frozen (items 4/5 logic) - one rule, applied uniformly, controller's call.

---

## HARVEST

1. **An APPROVED-with-minors verdict needs a disposition line per minor, or
   minors evaporate.** Task-5's m1/m2 were explicitly routed to whole-branch
   triage in progress.md; m3 ("this is on the task") reached no routing line
   and shipped unfixed (W2). The house handle is mechanical: at task close,
   every non-clean finding in the verdict gets exactly one progress.md
   disposition (fixed-now / deferred-to-<vehicle> / rejected-because). The
   trigger is readable: you are writing the "Task N: complete" line while the
   verdict lists findings.
2. **A recorded site list for a future doc correction drifts like a line
   number.** Of seven recorded stale-language sites, one was not stale
   (:1012), one pointed at the wrong artifact (plan :718 vs design :1966),
   and one real site was missing. The brief's own instruction (locate by
   content, report deltas) is the correct standing form and caught all three;
   worth folding into the close-batch convention: a deferred-correction entry
   names the *claim text* to find, never bare `:line` lists alone.
3. **A borrowed qualifier survives more hops than a borrowed number.** "
   pre-existing on master" travelled progress -> ledger occurrence -> this
   brief unchallenged, while the same chain's figures were all corrected en
   route. The verdict that coined the finding contained the disproof in the
   same section (master's clean FAIL vs the commit's traceback). Same class
   as `proc-quote-verbatim-or-paraphrase`/`feedback`-level quote-and-number
   rules, one abstraction up: provenance/causality qualifiers ("pre-existing",
   "unchanged", "already covered") are claims to verify at the artifact, not
   relay.
4. **The rehearsal's cross-run positive control is a house-worthy pattern.**
   R3's "skipped" in run A proven live by the same step's "success" in run B
   - two runs differing in exactly the gating input - is the strongest
   absence-verification shape in this whole plan and cost nothing extra.
   Candidate occurrence on `proc-verification-step-must-be-falsifiable`: when
   a paired run exists, the fire-proof of a conditional's negative arm is the
   other run's positive arm, no synthetic break needed.
5. **Brief/convention friction, measured:** the brief's tooling-trap warnings
   were justified in direction but one did not reproduce as stated - plain
   `grep` here IS a shell function, yet with an explicit path into
   `.superpowers/` it returned the hit (1 = `command grep`'s result); the
   false-empty presumably needs the rooted `-r` sweep form. I used
   `command grep` throughout regardless. No brief or convention boundary
   forced a stop I judged wrong; the one place I stopped short of executing a
   recorded routing (plan :248/:262) is returned as an explicit fork rather
   than resolved, per the brief's own rule.
6. **Cross-target clippy as a gate part generalizes.** The check-only
   `--target` run needs no foreign toolchain, catches the entire cfg-gated
   lint class, and took 1.3s warm. Any workspace whose CI matrix is wider
   than its dev machines can close the "red only on the OS I don't run"
   window this cheaply; candidate for `docs/conventions.yaml` once the
   BUILDING.md text lands (the ruling is already owner-approved).
