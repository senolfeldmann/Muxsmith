# Wording fix round: delta judgement

Companion to `.superpowers/sdd/plan-8/owner-wording-verdict.md`. Same judge,
same standards, settled non-findings not reopened. This grades only what
changed since that verdict.

## VERDICT: APPROVED

All five items discharged. The MEDIUM is closed by a fix I re-verified on my
own harness rather than accepting; the three LOWs are right and the AppImage
paragraph is true of every format it now names; A3 records the supersession
completely and section 4 is byte-unchanged; the site recount is correct and
mine was wrong. Three advisories below, none blocking, plus one process
finding about how this review's evidence was almost compromised.

## What I graded

`0b8e70e..645faf0`, i.e. `e61264fb1d230fb00d919fae4cf622fdac0c33ba` and
`645faf0e298dd1a9d30aa8dc41948b1b8de52e72`. HEAD moved again during the
review (`e38e30b`, "house: two more classes from the wording round's
reviews"); `git diff --name-status 645faf0 HEAD` lists only
`docs/decision-ledger.yaml` and `docs/process-conventions.yaml`, so the three
graded files stand as committed. Six commits ahead of `origin/master`,
nothing pushed. Both commits unsigned (`%G?` = `N`), both carrying
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, paths confined to
`docs/INSTALL.md` + `.github/release/draft-body.md` (`e61264f`) and the design
document (`645faf0`).

## 1. Does `--ignore-missing` discharge the MEDIUM without a worse defect?

**Yes - confirmed, on an independent harness, after the first attempt at
confirming it turned out not to be independent (see the process finding).**

Reviewer's own `Digest/SHA.pm` at an isolated path, deliberately instrumented:
its missing-file branch dies with the marker
`REVIEWER-SHIM-REACHED-MISSING-FILE`, so a missing file that reaches
`sumfile()` is *visible* rather than silently handled. Driving the
freshly-re-fetched, unmodified `perl5@v5.30.3` `shasum`:

```
1. all present, --ignore-missing        3x OK                       exit 0
2. only the dmg, NO flag                REVIEWER-SHIM-REACHED-MISSING-FILE x2
                                        + 2x FAILED open or read    exit 1
3. only the dmg, --ignore-missing       1x OK, marker absent        exit 0
4. only the dmg TAMPERED, flag          FAILED + "no file was verified"  exit 1
5. all present, one TAMPERED, flag      2x OK + 1x FAILED           exit 1
6. --ignore-mising (misspelt)           "Unknown option"            exit 1
```

GNU coreutils 9.10 reproduces states 1-6 identically (`unrecognized option
'--ignore-mising'` for 6).

Three things follow, and the second is the one that actually answers the
question:

- The green state is reachable (1, 3), so the exit-1 results are a firing
  check rather than a command that always fails.
- **The flag's scope is observed, not argued.** My marker fires in run 2 and
  is absent in run 3 on the same directory, so `--ignore-missing` skips the
  absent entries *before* the digest path and leaves present files untouched.
  Runs 4 and 5 then show a tampered file still failing under the flag. That
  is a stronger form of the report's evidence, which cited `shasum` line 281
  (`next if $ignore_missing && ! -e $fname;`) as the explanation - correct,
  and now also demonstrated.
- The flag is genuinely parsed by both implementations (6), so the passes in
  3-5 are not an ignored argument.

No worse defect introduced. The one behavioural change worth stating, and it
is acceptable: under the flag an artifact the user *meant* to download but
whose filename is misspelt is skipped silently instead of failing. GNU and
`shasum` both catch the degenerate case where nothing at all verified
("SHA256SUMS: no file was verified", exit 1), and both print one `OK` line per
file actually checked, so a reader can see what was covered. That is the right
trade against the defect it removes.

The wording is right too: `--ignore-missing` sits before `-c` in both
commands, which is where both implementations accept it (runs above use the
documented spelling verbatim).

## 2. The three LOWs, and the AppImage paragraph

All three land, verified wrap-aware against the tree (13 checks, every old
string at 0 and every new string at 1, with a one-character mutation of a
present string counting 0 as the sensitivity control; the five untouched
edits from the first round all still counted at 1).

- **Case-insensitivity** - added in both files: "compared against that file's
  line in `SHA256SUMS` (the comparison is case-insensitive)". Correct and
  minimal; it is the human doing the comparison, and this is the clause that
  stops a careful reader from mistrusting a good download.
- **Generic `<file>`** - `docs/INSTALL.md:10` now matches
  `.github/release/draft-body.md:24`. Better than my own suggestion of naming
  both msi files: `<file>` plus "that file's line in `SHA256SUMS`" is
  self-resolving for any of the seven artifacts, and the intro sits above the
  per-OS sections, where naming one artifact was the thing that misled.
- **The PATH sentence** - "so only the GUI is directly runnable - deb/rpm put
  `muxsmith` on PATH; the tar.gz carries both binaries for you to place."

**Is the paragraph true of every format it names?** Checked per format, all
against evidence already in the record:

| Format | Claim | Basis |
|---|---|---|
| AppImage | both binaries inside one file, only the GUI directly runnable | `task-6-report.md` R6 extraction (`usr/bin/muxsmith` + `usr/bin/muxsmith-gui`); tauri-bundler at `tauri-cli-v2.11.4` sets the AppDir desktop file's `Exec` to `main_binary_name` (`linux/freedesktop/mod.rs:103,176`), which `src-tauri/Cargo.toml` names `muxsmith-gui` |
| deb | puts `muxsmith` on PATH | R6 re-run PASS, `./usr/bin/muxsmith` + `./usr/bin/muxsmith-gui` payload |
| rpm | puts `muxsmith` on PATH | `rpm -qpl` -> `/usr/bin/muxsmith`, `/usr/bin/muxsmith-gui` |
| tar.gz | carries both binaries for you to place | R6 listing: `muxsmith`, `muxsmith-gui`, `LICENSE`, `README.txt`; the archive's own README says to put the directory on PATH or symlink into `~/.local/bin` |

True in all four, and the correction removes exactly the false half of the old
sentence without over-correcting: it no longer implies the tar.gz does the
placing, and it no longer implies the AppImage is unusable. `deb/rpm` as one
compound is the file's own idiom (`docs/INSTALL.md:83`, `:84`, `:93`). The msi
and dmg staying out of the tail is right - the paragraph exists for the
AppImage exception, and both other platforms carry their own CLI paragraph.

Style note only, no action: "carries both binaries" now appears twice inside
the one sentence. It reads fine; flagged because I would rather say it than
have it discovered as something I did not notice.

## 3. A3 and section 4

**Section 4 is genuinely byte-unchanged.** Extracted every fenced block in the
section-4 region from `0b8e70e` and from HEAD's working file and compared the
bodies: 7 blocks each, sizes 83/24/6/27/7/3/4, **all bodies byte-identical**,
with a one-character mutation flipping the comparison to CHANGED. The three
hunks of `645faf0` are the status line, four inserted lines between the `## 4.`
heading and `### 4.1`, and A3 at the end of the file - none inside a fence.

**A3 does not mislead a reader of section 4.** The pointer sits at the section
head, before the first subsection, and says the tree is authoritative at eight
sites with the blocks below being the wording the transcription was graded
against. It names the commits (`e477e37`, `4716b0c`, `e61264f`), states the
ruling and its precedent, enumerates the sites by content rather than by line
number, and states the granularity it counts at. Every one of its factual
claims reproduces on my own extraction:

| A3 claim | My measurement |
|---|---|
| 4.1 fence 83 vs tree 94, six regions | 83 / 94, 6 at zero context |
| 4.2 draft-body fence 24 vs tree 28, one region | 24 / 28, 1 |
| 4.2 rehearsal banner not superseded | fence 6 vs tree 6, 0 regions; `git log adb0f6e..HEAD -- .github/release/rehearsal-banner.md` empty, with the same invocation over `draft-body.md` returning three commits as the firing control |
| 4.3 tarball fence 27 vs tree 27, one region | 27 / 27, 1 |
| 4.4 and 4.5 not superseded | both riders present verbatim under whitespace-normalised search (count 1 each), mutated negative control 0 each |

Total at zero context: 6 + 1 + 1 = **eight**.

## 4. Ruling on the count: theirs is right, mine was wrong

**Eight is correct and seven was not.** More than that: my "five hunks" for
4.1 does not reproduce at *any* diff context, against either tree.

```
design 4.1 fence (83 lines) vs docs/INSTALL.md
  pre-fix tree  (93 lines):  n=0 -> 6   n=1 -> 4   n=2 -> 4   n=3 -> 4
  post-fix tree (94 lines):  n=0 -> 6   n=1 -> 4   n=2 -> 4   n=3 -> 4
```

The unified diff my original verdict printed carried four `@@` headers. I
reported five, because I counted the *ruled edits* grouped in that region
(1+5, 6, 4, 3+8, 2) and wrote them down as hunks. That is a number I did not
measure, in a verdict whose own HARVEST is about measurement discipline. The
extraction sizes I reported (83 vs 93) reproduce exactly, which is why the
substantive claim - section 4 has diverged, at these places - survived
unaffected. Recorded here as my defect, not the implementer's.

**Does it matter for what A3 claims?** Only in one clause, and only in the
direction of being too kind to me. A3 says my figure "reproduces at neither
zero nor three lines of diff context (six and four respectively)", which is
true but invites the inference that it reproduces at some other context. It
does not reproduce anywhere; it was not a diff-hunk count. See advisory A1.

The eight itself is the right thing to record, because "contiguous divergent
region at zero context" is a granularity that does not depend on a tool
parameter, and A3 states it. Enumerated, the six regions of 4.1 are: the
intro paragraph (the `steps` fix and the whole checksum sentence together),
the AppImage exception, the User-variables pane, the symlink command, the
no-sudo alternative, the `GUI only:` label.

## 5. The two disclosures

**(a) The status-line amendment - inside the grant.** The design's line 3
enumerated the amendment set as "(A1, 2026-07-23)" and had been stale since A2
landed. Adding A3 makes that line a reference over the very set the commit
changes, which is the standing sweep rule's own case: an enumeration or count
over a set is a dependency, not a duplicate, and changing the set means
visiting the references that named the old one. The repo carries the same rule
as `proc-normative-count-recomputed`. The brief's enumeration did not override
it, the edit is mechanical, it was disclosed as a separately revertible hunk,
and half-fixing it (adding A3 while leaving A2 unlisted) would have been worse
than either extreme. Correct call, correctly disclosed.

**(b) The nested fence in 4.4 - correctly routed.** Verified rather than
accepted: design `:1752` opens ` ```markdown `, `:1760` opens ` ```bash `, and
under CommonMark the first line with at least three backticks closes the open
block - so `:1760` closes the outer block, `:1767` opens a new one, and the
upgradeCode note at `:1769-1770` falls outside with `:1771` left as a stray
fence. My own fence extractor, written independently, sees 4.4 as **two**
blocks of 7 and 3 lines, which is that behaviour observed rather than
reasoned. 4.1 avoids it with a four-backtick wrapper.

Routing was not merely permitted here, it was mandatory: 4.4 is inside section
4, which the ruling froze byte-for-byte. Touching it would have violated the
ruling the same commit exists to record. And A3 is unaffected - 4.4's content
is present verbatim in `BUILDING.md` either way, which I confirmed with a
mutated negative control.

## The shim-fidelity disclosure

**The corrected harness is faithful enough, and the load-bearing claims never
depended on the part that was broken.** The implementer's first stand-in
returned `undef` where the real `Digest::SHA` dies, so `shasum`'s
`eval {}`-guarded `sumfile()` did not produce the documented
`FAILED open or read`; it corrected the shim to die and re-ran rather than
reporting around it, which is the right handling.

Two reasons this does not reach the conclusions:

- The broken path is the **no-flag** path (control H), and that control's
  result is independently corroborated by the GNU run of the same state, which
  uses no shim at all.
- Under `--ignore-missing` the missing entries never reach `sumfile()`, so the
  shim's missing-file behaviour is not on the path at all. I proved this
  rather than inferring it: my shim dies loudly on a missing file, and the
  marker appears without the flag and is absent with it.

What the shim must be faithful about for the safety claim is that
`hexdigest` returns the true digest - it delegates to coreutils, so it does -
and that it does not swallow errors, which after the correction it does not.
The residual difference is the `$errmsg` string text, which no claim rests on.

## Advisories (none blocking)

**A1. Tighten A3's one clause about the verdict's figure.** Recommended
wording: "its 4.1 figure of five is not a diff-hunk count at any context (six
at zero, four at one through three) - it counted ruled edits, not regions".
Same sentence also worth closing the other half of the mapping: A3's six
numbered descriptions map onto the six regions with a compensating pair -
descriptions 1 and 2 share one region, description 5 spans two. A3 discloses
the second and not the first, so a later recount can land on seven or five
depending on which end it starts from. One clause fixes both. A3 is amendment
prose, not frozen section-4 text, so this is editable without touching the
ruling.

**A2. Two report-internal citations do not reproduce.** The report cites the
`deb/rpm` compound at ":82-83 and :91"; against the pre-fix file `:82-83` is
right and `:91` is `:92` (current file: `:83`, `:84`, `:93`). And its table
gives 4.4's fence as 14 lines; I measure the intended block at 18 (design
`:1753-1770`) and a strict extractor at 7+3. Neither figure reaches A3 or any
shipped file, and 4.4's "not superseded" verdict is confirmed independently,
so this is a citation-hygiene note, not a correction.

**A3. The shipped surface now carries four `-c SHA256SUMS` instruction sites,
all four with the flag.** Established by reading the hits, not by an absence:

```
git grep -nE '(sha256sum|shasum)[^`]*-c SHA256SUMS' -- <the named surface>
```

over the shipped surface named in the first verdict returns
`docs/INSTALL.md:8,9` and
`.github/release/draft-body.md:22,23`, each visibly carrying
`--ignore-missing`. `.github/workflows/release.yml` keeps plain `sha256sum` -
it is the emitter, not an instruction. Nothing to do; recorded so the next
sweep has a baseline.

## Process finding: a shared scratchpad turned my re-run into their re-run

Not a defect in either commit, and it changes no conclusion - but it nearly
cost this review its independence, so it is on the record.

The fix-round implementer wrote its own `Digest/SHA.pm` and its own copy of the
upstream `shasum` script **to the exact scratchpad paths I had used** in the
first review (same session, so the same scratchpad directory). Timestamps:
my verdict was written 19:27; `p5303-shasum.pl` was rewritten 19:30 and
`shim/Digest/SHA.pm` 19:31.

My first re-run of the tamper matrix in this delta session therefore executed
**the implementer's instrument, not mine** - a re-run of the thing under
review, presented to myself as independent reproduction. I caught it only
because an error string changed between my two sessions
(`No such file or directory` -> `open failed`), which is luck, not method.

Recovery: re-fetched the upstream `shasum` to a clean path and diffed it
against the in-place copy (identical - the script under test was never
tampered with), wrote my own shim to an isolated directory, and re-ran
everything. All results above come from that run.

The registerable rule, with a readable trigger: **when a reviewer reproduces
an implementer's empirical claim, the reviewer's instruments live at a path
the implementer could not have written.** The trigger is the moment you reach
for a harness you built in an earlier turn - check whether anyone else could
have reached the same path first. This is the same family as the house's
false-empty rule: there, an absence is not evidence until the check has been
seen to fire; here, an agreement is not confirmation until the instrument is
known to be yours. A borrowed instrument reproduces the borrowed instrument's
blind spots, exactly as a re-run of a blind pattern reproduces its blind spot.

Concrete second instance from this same session, cheap to state because it was
loud rather than silent: my first attempt at the shasum matrix put the
multi-word command in a zsh scalar and every invocation exited 127. Same
mechanism as the unquoted-pathspec case the house now cites, but it announced
itself instead of returning clean empties - which is the whole argument for
preferring checks that fail loudly.
