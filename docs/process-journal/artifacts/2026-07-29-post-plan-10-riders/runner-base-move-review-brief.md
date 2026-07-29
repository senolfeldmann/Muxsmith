# Review brief - the Linux release base move to ubuntu-24.04

**Role:** independent reviewer of a single owner-ruled change. You did not write
it. Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `e260845`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/runnerrev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never a shared
  default path.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty. A bare `cp` is aliased
  interactive.
- **Prove tree identity per FILE against blobs**, not by a clean `git status`.
  No other writer is active in this tree while you work; if that changes you
  will see it as a moved HEAD, and it is not your finding.

## What was decided, and by whom

GitHub announced that the Ubuntu-22 runner images begin deprecation 2026-09-17
and are fully unsupported 2027-04-17 (actions/runner-images issue 14254). That
fired a registered trigger in `docs/ROADMAP.md` whose prescription is to move
the Linux release leg to `ubuntu-24.04` and record the raised floor. **The owner
ruled it: release on 24.04, tests stay on 26.04.** He accepted the consequence,
which is a product-reach decision rather than a version bump: the glibc floor
rises 2.35 -> 2.39, dropping Ubuntu 22.04 LTS (2.35) and Debian 12 (2.36).
Debian 13 carries 2.41 and is unaffected; building on 26.04 was rejected because
its 2.43 floor would drop Debian 13 too.

The rationale for the split existing at all is the vendor's: Tauri's AppImage
guide says to build "using the oldest base system you intend to support",
because glibc is backward but not forward compatible.

## The change

Commit `e260845`, three files: `.github/workflows/release.yml`,
`packaging/linux-tarball-README.txt`, `docs/INSTALL.md`. The implementer's brief
is `.superpowers/sdd/runner-base-move-brief.md` and its report is
`.superpowers/sdd/runner-base-move-report.md` - **evidence, not ground truth**.

## Dimensions

1. **Every pin moved, and only the intended ones.** Three sites in
   `release.yml` (`guard.runs-on`, the `bundle` matrix `os:` for the
   `linux-x86_64` leg, `assemble.runs-on`). Derive the set yourself rather than
   trusting the report's list - and check the OTHER legs (`windows-2025`,
   `macos-15`, `windows-11-arm`) are untouched. `.github/workflows/ci.yml` must
   still be on `ubuntu-26.04`.
2. **The workflow still parses and its structure is unchanged.** Parse it and
   compare the job/step structure against the previous commit rather than
   reading the diff for reassurance.
3. **The policy comment is TRUE after the edit.** It records two deliberate D85
   deviations. Check both halves: the `windows-11-arm` half must be untouched,
   and the divergence half must now describe 24.04-versus-26.04 with a rationale
   that still holds.
4. **The two user-facing requirement texts.** `packaging/linux-tarball-README.txt`
   and `docs/INSTALL.md` must name the SAME floor, and every claim in the
   rewritten passages must be true: the implementer reports it rewrote more of
   the tarball README bullet than a version swap because two neighbouring claims
   went false. Check what those claims were and whether the replacement is
   accurate - in particular any statement about what the AppImage does or does
   not rescue, and any statement naming distributions as below the floor.
   **A distribution named as unsupported in shipped text is a claim about the
   world**: verify each named version's glibc rather than accepting it.
5. **No invented webkit version floor.** The package name is the requirement.
6. **Nothing else moved.** No action SHA, no other job, no step logic, no
   dependency.
7. **The no-work-needed check.** Wherever the report concludes something needed
   no change or could not break, run the premise.
8. **The limit of the evidence, stated honestly by the implementer and to be
   checked by you:** no gate part reads any of the three files, so a green gate
   proves only that nothing else broke. Verify that claim with your own search,
   and say what remains unproven until a draft release run happens. **Name the
   most likely breakage point on the new base** if you can identify one from the
   workflow's own steps - the apt build-dependency install is the implementer's
   candidate.

## Adjudication questions (one explicit verdict each, phrased in both directions)

1. **The tarball README rewrite's breadth.** The implementer went beyond
   swapping the version because the bullet's remaining claims had become false.
   Correct repair of statements its own edit falsified, or scope creep in a file
   whose brief named one line?
2. **`docs/INSTALL.md`'s new bullet placement.** It sits under an intro scoped
   to the AppImage and tar.gz, while the floor applies to deb and rpm as well.
   Is the placement misleading to a reader, and if so what is the minimal fix?
3. **Naming two distributions as unsupported in shipped user-facing text.**
   Ubuntu 22.04 LTS and Debian 12 are now named as below the floor. Is that the
   right service to a reader, or does it date the document in a way a plain
   version floor would not?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/runner-base-move-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, evidence run, exact required change;
the three adjudications; an evidence appendix; and a HARVEST. Note that the
implementer already surfaced one follow-up outside its file list - a comment in
`renovate.jsonc` still saying release.yml pins ubuntu-22.04 - which the
controller is handling; confirm it exists and say whether anything ELSE in the
tree still asserts the old base.

Your final message carries the same in short form.
