# Plan-8 fix-wave report

Implementer: fix-wave dispatch, fresh context. Brief: `fix-wave-brief.md`;
contracts: `whole-branch-verdict.md` (Fix wave + Adjudication), the plan-8
design, the four house-knowledge YAMLs as ground truth. Every command
foreground, `master` in the main worktree, absolute paths, no branch, no
worktree, no session-relocation tool. **No `gh` command was run** (none was
needed), so no `gh-log.md` entry is owed; no release was created, edited,
published or deleted, and no workflow was dispatched.

## VERDICT: all three items applied, verification green

Three commits:

| # | SHA | Scope |
|---|---|---|
| 1 | `ecab53acc22c9b57e6035d8414983c96df489c77` | `scripts/ledger-lint.py` (item A) |
| 2 | `bcb67f305ec2f35535c3ee5f70a95dd10a081eba` | `BUILDING.md` (item B) |
| 3 | `6d81738afb9aafa0ea1bc485f195974460c22ea8` | design document + the plan's two supersession lines (item D + the ruled fork) |

All staged explicitly by path, unsigned (`git -c commit.gpgsign=false`),
trailer `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>` (which is also
the house form in the four commits preceding this wave). Nothing pushed.

Working tree at the end carries only controller-side modifications:
`docs/ROADMAP.md`, `docs/conventions.yaml` and `docs/decision-ledger.yaml` were
already modified when this dispatch started (the parallel close writer the
whole-branch review recorded), and `docs/process-conventions.yaml` joined them
during the run, between my second-to-last and last `git status`. None was
touched, staged, or read for edit by me; nor were `docs/process-journal.md`,
`docs/product-boundaries.yaml` (used only as a fire-test target, restored
`cmp`-identical), or `progress.md`. The linter is green against that in-flight
state as of 18:46 on 2026-07-27 (467 entries, exit 0).

## Premises checked before editing

- **HEAD moved past the reviewed pin, but not into the pathset.** HEAD at
  dispatch was `eb4608be`, four commits past `7302e1b`
  (`8e2c044` plan-7.5 salvage, `d5a6470` plan-7.5 close, `ddb707a`/`eb4608b`
  help wording). `git log 7302e1b..HEAD --name-only` touches `help/**`, the
  plan-7.5 design and plan, and `docs/process-journal/artifacts/plan-7.5-sdd/**`
  only. **No file of this wave changed after the pin**, so every content-located
  edit site in the verdict still resolved.
- **Every `:line` in the verdict resolved to its quoted text.** All eighteen
  edit sites were located by content; none was a miss.
- **ci.yml's trigger block, read myself** (item A edit 2 required it):
  `push.branches: [master]`, `push.tags: ['v*']`, `pull_request`,
  `workflow_dispatch`. The replacement sentence matches.
- **A2's tree-facing claims verified at the artifact**, not relayed:
  `07c0255` changes exactly `src-tauri/tauri.conf.json` (2 lines) and adds
  `src-tauri/wix/locale-en-US.wxl` (23 lines); the shipped
  `bundle.windows.wix.language` is the single-key map with
  `localePath: wix/locale-en-US.wxl`; the locale file carries
  `Codepage="1254"` and `<String Id="TauriCodepage">1254</String>`; run
  `30268008932` is the wix-fix report's D1 diagnosis run, whose §"ASCII A/B"
  row records "still red".
- **Adjudication 4's emitter**: `release.yml:54` is
  `echo "ci gate green for $GITHUB_SHA"`. The design text was the wrong side,
  as ruled.
- **Adjudication 12's premise**: section 1 spans `:58-278` and contains no
  Apple or Gatekeeper reference; the only `apple.com` citation in the document
  is line 51, section 0's note 2. Confirmed by a whole-file
  `apple\.com|gatekeeper` sweep (3 hits: :51, :1593, :1622 - the last two are
  the INSTALL.md transcription's own Gatekeeper prose, not citations).
- **Adjudication 5's "two sites, no more"**: a fixed-string sweep for
  `./usr/bin/muxsmith` over the design and the plan returns exactly design
  :1907 and plan :654. No INSTALL.md or README site uses the form.
- **The ruled fork's "two is the measured claim"**: a `git grep '\["en-US"\]'`
  over all tracked files returns exactly three sites - plan :248, design :958,
  design :1511. Prose variants (`en-US language list`) add design :2007 and
  plan :262. **No third frozen plan-copy site exists.** The `.superpowers/`
  hits (searched with `command grep -rn`, since the shell's `grep` function
  respects `.gitignore`) are review artifacts and task briefs, not plan copies.
- **`rust-toolchain.toml` carries channel + components and no `targets` key**,
  as the second ruled fork assumes. No key was added.

## Edits applied, one line each

**A. `scripts/ledger-lint.py`** (commit 1)

1. `DuplicateKeyLoader(text)` moved inside the parse `try`, with
   `loader = None` before it and `finally: if loader is not None:
   loader.dispose()` - exactly the task-5-verdict m1 snippet.
2. Docstring: "runs this script on every push and pull request" -> "on every
   master push, `v*` tag and pull request" (task-5 verdict m3).

**B. `BUILDING.md`** (commit 2)

3. `:65-68` replaced with adjudication 1(a)'s text verbatim ("out of scope for
   local development" removed, "not part of the CI gate" kept, "yet" dropped).
4. `:92-95` replaced with adjudication 1(b)'s text verbatim (parts 1-5 native
   on three legs, corrected trigger set, `ledger-lint` as the second
   independent job).
5. Tenth gate part, all four pieces per adjudication 2 (i)+(ii): the
   `rustup target add x86_64-pc-windows-msvc` prerequisite line under
   `### Rust toolchain`; heading "five parts" -> "six parts"; the clippy
   `--target x86_64-pc-windows-msvc` line as the fence's new last line; the
   rationale paragraph after the fence's existing prose.
6. `## Cross-target lint rule` section deleted (controller ruling: no veto).

**D. Design document + plan document** (commit 3)

7. Amendment-log entry **A2** appended after A1, adjudication 6's text
   verbatim.
8. `:958` D86 language value -> the map + locale-file clause (adjudication 7
   table row).
9. `:1511` section-3.1 fence line -> the one-line map form (table row).
10. `:2007` section-11 frozen-literal list -> "the en-US language map with its
    locale file `wix/locale-en-US.wxl` (A2)" (table row).
11. `:1966` trigger-7 premise -> "a per-language map carrying a locale file -
    see A2" (table row; the site no recorded list carried).
12. Fallback clauses at `:941-943`, `:1921`, `:2022-2024` marked
    "(superseded, A2)" - marked, not deleted, so R8 keeps its rendering check.
13. R1 observable `:1880-1881` -> "names the gated SHA (the `ci gate green for
    <sha>` echo)" (adjudication 4).
14. R1 addendum line appended to the R1 bullet (adjudication 10).
15. R6 dpkg payload path form `:1907` (adjudication 5).
16. D75 cross-reference "section 1" -> "section 0, note 2" (adjudication 12).
17. Plan `:248`: the supersession line added after the fence, brief's wording
    verbatim; the fence itself untouched.
18. Plan `:262`: " (superseded by design amendment A2)" appended to the
    frozen-list item "the en-US language list"; the frozen wording itself
    untouched.

`:1012` received no edit, per the verdict's own correction.

## Verification

**1. `python3 scripts/ledger-lint.py`: exit 0, `467 entries across 4 files, all
invariants hold`.** The 467 is my own recount, run four times across this
session (baseline, after each restore, and finally after all three commits) -
identical every time.

**2. The three fire tests.** Mutation target `docs/product-boundaries.yaml`,
which the parallel controller writer is not touching (it is not among the three
modified files). Pre-mutation backup taken with `command cp -f`, every
restoration `cmp`-verified against it, `git status` clean on the file after
each.

- **Control character.** `\x08` planted at offset 0. The fixed script prints
  the linter's own violation line (PyYAML's message wraps, so it occupies two
  output lines):

  ```
  FAIL docs/product-boundaries.yaml: does not parse (unacceptable character #x0008: special characters are not allowed
    in "<unicode string>", position 0)

  ledger-lint: 1 violation(s) across 435 entries
  ```

  exit 1 - the linter's own contract, no traceback. (435 rather than 467
  because the unparseable file contributes no ids; the difference is
  `product-boundaries.yaml`'s 32 entries.)
  **Fire-proved against the pre-fix script on the identical mutated file**: a
  copy of `HEAD:scripts/ledger-lint.py` placed inside `scripts/` (so its
  `Path(__file__).resolve().parent.parent` repo root resolves) raised the
  uncaught `yaml.reader.ReaderError` traceback through
  `DuplicateKeyLoader(text)` -> `Reader.__init__` -> `check_printable`, exit
  1. The control script was removed with `command rm -f` and its absence
  confirmed. **This also independently reproduces W1's causal claim**: the
  escape is real and the fix is what closes it.
  *Note on method:* my first attempt at this control ran the copy from the
  scratchpad and reported four `file not found` violations - a malformed
  control, caught because its output did not match the claimed failure mode.
  Re-run correctly as described.
- **Duplicate key.** A second `  steelman: null` planted inside the first
  entry -> `FAIL docs/product-boundaries.yaml: duplicate key 'steelman' (lines
  22 and 23)`, exit 1. Check 6 still fires.
- **Green reachable.** After each restore: exit 0, 467 entries.

**3. `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc --
-D warnings`: exit 0**, with the two known cosmetic build-script warnings
("GNU compiler is not supported for this target"). **The target was already
installed** on this machine (`rustup target list --installed` shows
`x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu`), so **no toolchain
addition was made and none is owed as a report item**.

Because a green clippy run is an absence-shaped result, it was fire-proved
rather than trusted: removing the `#[cfg(unix)]` gate above
`use crate::error::ParamValue;` in `src-tauri/src/lib.rs` turns the same
command red with ``error: unused import: `crate::error::ParamValue` `` and
``error: could not compile `muxsmith-gui` (lib test) due to 1 previous error``,
cargo exit **101**.
Restored from a `command cp -f` backup, `cmp`-identical, green again, and the
file is absent from `git status`.

**4. Absence-based edits: every pattern was fired before the edit.** Search
surface named per line. Baselines were measured on the pre-edit tree; the
post-edit zero is therefore a real absence, not a malformed pattern.

| pattern | surface | before | after |
|---|---|---|---|
| `on every push` | `scripts/ledger-lint.py` | 1 | 0 |
| `out of scope for local development` | `BUILDING.md` | 1 | 0 |
| `^## Cross-target lint rule` | `BUILDING.md` | 1 | 0 |
| `nine parts total` | `BUILDING.md` | 1 | 0 |
| `Rust gate (five parts` | `BUILDING.md` | 1 | 0 |
| `\["en-US"\]` | design | 2 | 0 |
| `en-US language list` | design | 1 | 0 |
| `./usr/bin/muxsmith` (fixed-string) | design | 1 line | 0 |
| `names the found ci run` | design | 1 | 0 |
| `the mechanism is a config` | design | 1 | 0 |
| `citation in section 1` | design | 1 | 0 |

One pattern was rejected before use: `the mechanism is a config list` returns
**0 on the pre-edit tree** because the phrase wraps across `:1966-1967`. Had
that been run only after the edit it would have read as a clean absence. The
shortened `the mechanism is a config` was used instead, with a measured
baseline of 1.

**5. Post-edit structural check.** The design's section-3.1 fence still parses
as JSON after the language-key substitution (extracted between its fence
markers and `json.loads`-ed: parses). The plan's `:248` fence is byte-unchanged
in the diff.

## Findings

**F1 (report, not fixed - outside the wave's scope): one live consuming
reference to the deleted BUILDING.md section survives.**
`.github/workflows/ci.yml:92` carries the phrase `matching the cross-target
lint rule` inside the dated Plan-5.5 comment block at `:88-93`; reassembled
across the comment's wrap (`:91-93`) the sentence is "All legs, matching the
cross-target lint rule (cfg-gated items can differ per platform)." It cites the
`## Cross-target lint rule` section by its title, and that title no longer
exists. Adjudication 2's consuming-line
sweep examined this block for the gate-part *count* (`:88`'s "ninth gate part",
correctly ruled accurate as dated history) but not for the *section-title
citation* at `:92`. The rule itself survives as gate part 6, so nothing is
false; the pointer is dangling. `ci.yml` is not among this wave's edits, so it
is recorded for the controller rather than touched. The remaining hits for that
phrase are frozen artifacts (`docs/process-journal/artifacts/**` diffs and
`house-backfill-sdd/find-E0.md`, plus their `.superpowers/` originals).

**F2 (record): A2's verbatim text pins two line numbers, and they still
resolve - for now.** The mandated A2 wording ends with "Sites updated by this
amendment: D86 decision text (:941-943 fallback clause, :958 language value)
...". Checked post-edit: the fallback clause is at `:941-943` and the language
value at `:958`, so both are accurate today. They are the drift-prone form the
verdict's own HARVEST item 2 argues against ("a deferred-correction entry names
the *claim text* to find, never bare `:line` lists alone"). Left verbatim per
the brief's "do not improve it"; flagged so the controller can decide whether
the house's own new rule should apply to its own amendment text.

**No premise of the brief or the verdict was refuted.** Every claim I could
check against the tree held: the regression's cause and commit, the trigger
set, the emitter, the two-site dpkg claim, the missing `:1966` site, `:1012`
being correctly excluded, the two frozen plan-copy sites and no third, and the
absent `targets` key. The one methodological miss was mine (the malformed
control run in fire test 1), caught and re-run.

## Not done, deliberately

Item C is verification (above); item E is controller work. Untouched, as
briefed: `docs/ROADMAP.md`, `docs/process-journal.md`, the four
`docs/*.yaml` house-knowledge files, `progress.md`. No `targets` key in
`rust-toolchain.toml`. No plan-document rewrite beyond the two ruled
supersession additions. The draft release `rehearsal-30273529210` was not
queried, touched, or otherwise contacted - no `gh` command ran at all.
