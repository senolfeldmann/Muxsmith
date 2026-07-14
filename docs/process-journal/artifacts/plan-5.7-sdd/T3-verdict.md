# T3 verdict (Stream C): dry-run indent + ctrlc registration warning

- Reviewer: independent (did not implement); object: commit `17ae87c` on `plan57-c` (base cd5e917), worktree `/home/senol/Git/Muxsmith/.worktrees/plan57-c`.
- Ground truth read in order: plan 5.7 Task 3; adjudication verdict items 5 + 8; docs/conventions.yaml + docs/process-conventions.yaml; T3-report.md.
- All verification FOREGROUND, run by the reviewer, not taken from the report.

## VERDICT: APPROVED

No blocking, major, or minor findings. Two informational notes and harvest observations below.

## Checks performed (evidence)

### 1. Indent (item 8, owner ruling YES)

- All four lines use the single-placeable form on disk: en/cli.ftl:20-21 and de/cli.ftl:27-28 read `= {"  "}rule ...` / `= {"  "}output: ...` / `= {"  "}Regel ...` / `= {"  "}Ausgabe: ...`. Verified by grep on the committed worktree, matches the diff.
- **End-to-end probe, stronger than the implementer's unit probe:** built the debug binary and ran the real CLI (`muxsmith dry-run p.yaml --source ... --output ...`) against a real mkvmerge-built SRT-track fixture (mkvmerge v-installed, fixture in reviewer scratchpad, nothing written into the repo). Output through `sed -n l`:
  - `  rule 0 -> track 0$` and `  output: /...$` - exactly two literal spaces, **no FSI/PDI bytes** (isolation off confirmed at i18n.rs:31 `set_use_isolating(false)`), `dry-run-file` header flush-left. The production rendering is the intended two-space indent.
- No snapshot/test pins the old rendering: grepped `dry-run-assignment|dry-run-output` across all `*.rs/*.snap/*.ts/*.tsx` - hits only at the render site (commands/mod.rs:108,113) and the completeness fixtures. All 11 `.snap` files enumerated; grep for `rule|output|Regel|Ausgabe` over them: zero matches. Report's "no test updates needed" claim confirmed independently.

### 2. ctrlc warning (item 5)

- **Err path mirrors the joblog degradation shape exactly.** Line-by-line: joblog site (`create_logger`, run.rs:342) is `eprintln!("{}", renderer.msg("run-joblog-unavailable", &[]));` guarded by `if logger.is_none()`, never aborts; new site (run.rs:233) is `eprintln!("{}", renderer.msg("run-signal-handler-unavailable", &[]));` guarded by `if ctrlc::set_handler(...).is_err()`, never aborts. Same call shape, same zero-arg list, same stderr channel, same continue-semantics; the doc comment cross-references `create_logger`'s degradation contract.
- **Ok path / D16 contract unchanged.** Diffed run.rs 216-233 against base `cd5e917`: the handler closure is byte-identical (`handler_cancel.swap(true, SeqCst)`; second Ctrl-C -> `process::exit(130)`). Only the discard (`let _ =`) became the `if ... .is_err()` guard; registration itself, the shared `cancel` Arc wiring into `QueueControl`, and the double-Ctrl-C force-exit are untouched.
- Comment claim "the one registration in the process" verified: repo-wide grep shows run.rs:226 is the only `ctrlc::set_handler` call site; core only references ctrlc in docs.
- **Key in both catalogs:** en/cli.ftl:41, de/cli.ftl:48, both appended after the joblog block (catalog order preserved).
- **German register:** real umlauts (`Löschen unvollständiger Ausgaben`, `für diesen Lauf`); `Strg-C` is the correct German key name; `Exit-Code` matches the existing `run-job-failed` terminology (de:42 `Exit-Code { $code }`); the closing shape `... ist für diesen Lauf nicht verfügbar; es wird fortgefahren.` parallels the joblog sibling's `... für diesen Lauf ...; es wird ... fortgefahren.` Register consistent with the de catalog.
- **catalog_completeness registration mirrors the sibling exactly:** appended to `ALLOWLISTED_CLI_KEYS` (:198, after the joblog keys, matching catalog order) and added to the same zero-arg fixture arm as `run-joblog-unavailable` (:214). The wiring test is bidirectional (orphan check cli.ftl->allowlist plus stale-entry check allowlist->cli.ftl), so the key is provably present and wired.

### 3. Message content accuracy

- en: "The Ctrl-C cleanup handler could not be registered; graceful abort (cancelling in-flight jobs, deleting partial outputs, exit code 130) is unavailable for this run; continuing."
- de: same content ("kontrollierter Abbruch (Beenden laufender Jobs, Löschen unvollständiger Ausgaben, Exit-Code 130) ... nicht verfügbar ... es wird fortgefahren").
- Accurate, no overclaim: exactly the D16 cleanup semantics that are lost (in-flight cancel via the queue flag, partial-output deletion, deliberate exit-130), and the run continuing. It does not claim Ctrl-C stops working entirely (terminal SIGINT still kills the process group - correctly left out of a user-facing one-liner, and carried by the code comment). Matches the adjudication's prescribed content ("D16 cleanup semantics unavailable ...; the run continues") precisely.

### 4. Scope

- `git show --stat 17ae87c`: exactly the four plan-named files (run.rs, catalog_completeness.rs, locales/de/cli.ftl, locales/en/cli.ftl; 22+/8-). Single commit on the branch over base. Worktree clean after all review runs (`git status --porcelain` empty).

### 5. Gates (all re-run foreground by the reviewer)

| Gate | Result |
|---|---|
| `cargo test -p muxsmith-cli` | 67 passed across 9 targets, 0 failed |
| `cargo test -p muxsmith-cli --test catalog_completeness` | 4 passed, 0 failed |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| `pnpm check:i18n` | ok, 179 catalog ids, de parity green; the 12 unused warnings are pre-existing `gui-*.ftl` keys, none in cli.ftl |
| End-to-end binary dry-run (real mkvmerge fixture) | two-space indent renders in production output |

### 6. House dimension

- core-37's surface-side counterpart honored: the new user-facing string goes through `renderer.msg` / the Fluent catalog, no bare `eprintln!` English (the exact house violation the adjudication warned the naive fix would commit).
- Bilingual-same-commit honored (both cli.ftl files in 17ae87c).
- ci-06 per-commit gate: implementer's claimed runs are consistent with what I reproduced; all parts green on the committed HEAD.
- Idiom: `.is_err()` over `if let Err(_)` is clippy-canonical; `{"  "}` single placeable is valid Fluent StringLiteral usage and empirically renders identically to `{" "}{" "}` (adjudication offered both; plan brief and this review sanction the single-placeable form).

## Informational notes (no fix required)

1. **en/de surface asymmetry:** en says "Ctrl-C cleanup handler", de "Strg-C-Handler" without a "cleanup" qualifier. Content parity is intact via the parenthetical; noted only so a future de polish pass doesn't read it as drift.
2. **Brief line-number drift:** the plan cited de/cli.ftl:25-26; actual keys sit at :27-28 (8-line header comment). The implementer keyed on content, not line numbers - correct behavior, already disclosed in T3-report deviations.

## Harvest observations (for the plan-close ledger sweep, doctrine section 7)

1. **Degradation-warning shape now has two instances** (`run-joblog-unavailable`, `run-signal-handler-unavailable`): "a CLI best-effort facility that fails renders a zero-arg Fluent warning to stderr and the run continues; never abort, never bare English." Second instance reinforces what the adjudication already called "the established shape"; at a third instance this deserves its own conventions.yaml pattern entry (i18n or cli domain) rather than living only in adjudication prose.
2. **Fluent leading-whitespace idiom is non-obvious and will recur:** the parser's `skip_blank_inline` silently strips post-`=` spaces, so any intended leading/trailing whitespace in a catalog line must be a `{"  "}` StringLiteral placeable. Candidate i18n-domain convention entry; without it the next indented catalog line will re-ship the same dead-spaces defect.
3. **Process, minor:** briefs that cite catalog positions by line number drift with header comments; citing key names (as the plan also did) is the stable half - prefer keys alone in future briefs.

## Report-claim audit

Every load-bearing claim in T3-report.md re-verified independently: probe result (reproduced end-to-end, stronger form), snapshot blast radius (reproduced by grep), gate results (re-run), staging scope (git show), catalog placement and register (read on disk). No claim found inflated or wrong.
