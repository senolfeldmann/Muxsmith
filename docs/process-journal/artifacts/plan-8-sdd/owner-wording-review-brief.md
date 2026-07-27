# Review brief: plan-8 owner rendered-surface wording pass

You are the independent reviewer for the owner's ruled wording pass on Plan
8's shipped installation documentation (Muxsmith, `/home/senol/Git/Muxsmith`,
a public Rust + Tauri 2/Vue 3 MKV muxing tool). You did not write it.

## What was supposed to happen

The plan-8 whole-branch reviewer surfaced nine wording findings on the
user-facing install documentation and routed all nine to the owner, because
shipped prose is his call under the design's section 11. The owner ruled
eight of them and deferred one (item 7, whether two continuation lines in the
release-body template render inline) to his own inspection of the rendered
draft release.

The implementer's contract is `.superpowers/sdd/plan-8/owner-wording-brief.md`,
which carries the exact replacement text per edit. Its report is
`.superpowers/sdd/plan-8/owner-wording-report.md`. Read the brief first, then
verify the tree yourself rather than grading the report.

Files in scope: `docs/INSTALL.md`, `.github/release/draft-body.md` (ONE line),
`packaging/linux-tarball-README.txt`. Establish the commit set with
`git log` rather than taking a SHA from the report.

There are TWO commits, not one: `e477e37` (the eight ruled edits) and
`4716b0c` (a macOS clause added afterwards on a further owner ruling). Grade
both. The second exists because the implementer refuted a controller premise:
the brief asserted that `sha256sum` is absent from macOS; the implementer
established from Apple's own `md5(1)` man page that the `*sum` aliases with
GNU-mode `-c` are present on macOS 13/14+ and absent on 11/12, withheld the
clause rather than ship a claim it could not support, and recommended
`shasum -a 256` as the form covering the whole documented support range. The
owner then ruled that in. It also refuted the brief's Windows artifact name
(`windows-x64`, which exists nowhere; the scheme is `windows-x86_64`) and
used the correct one.

Both refutations are accepted. Your job includes checking that what replaced
the refuted premises is itself right.

## Two edits rest on external facts - check them, do not accept them

1. **The checksum commands.** The brief asserts that `sha256sum` is a GNU
   coreutils command absent from a stock macOS (which ships `shasum`), and
   that PowerShell's `Get-FileHash` is the Windows equivalent. The Windows
   half was the owner's ruling; the macOS half is a controller extension
   flagged to him as a reversible assumption. Verify BOTH from authoritative
   sources, not memory. If either is wrong, that is a finding of the first
   order - it would mean a wrong instruction shipped in place of a different
   wrong instruction.
2. **The glibc floor.** The brief asserts 2.35, derived from the runner image
   the Linux release leg builds on (named in `.github/workflows/release.yml`).
   Verify the runner label from the workflow and the mapping from an
   authoritative source.
3. **The one-file, three-commands claim.** The intro now names three
   different checksum commands against a single `SHA256SUMS` file, so it
   asserts that all three read that file's format. The implementer verified
   this at the parser level rather than from documentation: it fetched the
   Digest::SHA release that macOS `shasum` runs, read its check-mode regex,
   established that GNU text-mode output's second space IS the mode symbol
   that regex requires, and ran four malformed control lines to prove the
   acceptance was not vacuous. Re-examine that reasoning. If it is wrong, a
   user discovers it at the worst possible moment - while checking whether a
   download was tampered with.

## Dimensions

1. **Fidelity**: did each of the eight ruled edits land as specified, and did
   nothing outside the three files change? In particular confirm that
   `.github/release/draft-body.md` lines 2-4 are untouched - that item is
   deliberately the owner's and must not have been "fixed" in passing.
2. **Correctness in context**: each replacement has to be true of the shipped
   artifacts, not merely well-phrased. The AppImage sentence is the one to
   check hardest: the claim is that the AppImage carries both binaries inside
   a single self-contained file, so its CLI is not reachable as a `muxsmith`
   command without extracting the image. The rehearsal's evidence is in
   `.superpowers/sdd/plan-8/task-6-report.md` (the AppImage extraction shows
   `usr/bin/muxsmith` and `usr/bin/muxsmith-gui`).
3. **Completeness against a named surface**: are there further sites carrying
   the same defects - another `sha256sum` instruction, another
   `/usr/local/bin` assumption, another "one-time step" singular? Name the
   surface you swept and state it explicitly. A caution earned twice in this
   session: a second run of the same pattern is not a second measurement, and
   a pattern with a structural blind spot reproduces its blind spot. The
   concrete instance today was `see the [^)]*topic`, which cannot match a
   title containing parentheses.
4. **House conformance** against the four `docs/*.yaml` house-knowledge files,
   by entry id; and against the repo's typography rules (ASCII hyphens,
   straight quotes, no Unicode ellipsis) - with the deliberate exception that
   proper names keep their real orthography.
5. **Commit hygiene**: one commit, unsigned (`%G?` = `N`), trailer present,
   explicit staging, nothing pushed, no path outside the three files.

## Output

Write `.superpowers/sdd/plan-8/owner-wording-verdict.md`:
`## VERDICT: APPROVED` or `## VERDICT: NEEDS FIXES`, findings by severity with
evidence, an explicit statement of what you verified externally and how, and a
`## HARVEST` section. If you believe one of the owner's eight rulings produced
text that is wrong on the facts, say so plainly - the ruling was his, but a
factual error in the result is a finding, not a preference.

That file plus scratchpad files are your only writes. No git write operations,
no product edits, no session-relocation tools, no GitHub release touched (the
draft `rehearsal-30273529210` is the owner's pending inspection input). All
commands foreground, absolute paths, `command grep` or `git grep` when
searching the git-ignored `.superpowers/` tree. Quote only what you have
opened; measure every number you report.

Note that other agents committed to this repository during and around this
task; `git status` may show unrelated modifications and HEAD may have moved.
Pin your range to the wording commit itself and say which SHA you graded.
