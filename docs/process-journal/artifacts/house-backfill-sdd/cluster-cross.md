# House-knowledge clusters - domain `cross`

Reconstructed from 14 occurrence records spanning eras E0-E8. Records were grouped by identical `(topic, approach)`; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`.

Dates verified against the repo (`git log`/mtime): commit `61249f9` = 2026-07-08; handoff plan-1-close = 2026-07-09; FINAL-review = 2026-07-09; plan-4/plan-5 whole-branch verdicts = 2026-07-11; `ponytail-mining.md` = 2026-07-12.

Only one cluster reaches the promotion threshold (the Tauri stack, count 4). Everything else is a single- or double-touchpoint decision. No count was padded: the two cross-era clusters (stack, mkvtoolnix-external-CLI) collapse same-document/same-section citations into one occurrence and only keep distinct artifacts (spec vs journal vs handoff vs commit).

---

## cross-01-stack - Tauri 2 + Rust core + web frontend + clap CLI
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Build on Tauri 2 with a Rust core crate, a web frontend and a clap CLI; Rust accepted despite being only recently picked up. React was the initial frontend, later swapped to Vue (D27 / 2026-07-10, outside the cross era) - a within-frontend refinement, not a stack reversal, so the stack itself stays settled.
- **Steelman:** null (the argument *against* this stack lives in the two rejected-alternative restraints below).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §2 (Stack row) | "Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk) and Avalonia (delivery certainty, smaller OSS pull)." |
| 2026-07-08 | decided | journal 2026-07-08 (Plan 1, bullet 4) | "Rust accepted although only recently picked up." |
| 2026-07-08 | decided | commit 61249f9 | plan-1 implementation of the chosen stack |
| 2026-07-09 | decided | handoff 2026-07-09-plan-1-close.md | stack carried through plan-1 close |

---

## cross-02-wails-rejected - Wails v3 rejected as desktop shell
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Wails v3 was weighed and rejected as the desktop shell.
- **Steelman:** A Go-based Wails shell would keep the core in Şenol's strongest language and dodge the Rust learning curve while still shipping small native bundles. Rejected because v3 was alpha (delivery risk).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 Plan 1 entry | "Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk)." |

---

## cross-03-avalonia-rejected - Avalonia (C#/.NET) rejected as UI stack
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Avalonia (C#/.NET) was weighed and rejected as the UI stack.
- **Steelman:** Avalonia is a mature C#/.NET UI stack in a language Şenol already knows, with high delivery certainty and a smaller OSS dependency pull. Rejected for weaker web-native fit than the Tauri + web-frontend route.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 Plan 1 entry | "... and Avalonia (delivery certainty, smaller OSS pull)." |

---

## cross-04-mkvtoolnix-external-cli - mkvtoolnix as external CLI only
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** mkvtoolnix is invoked as external, user-installed executables only, detected at startup: no linking, no code reuse, no bundling. Muxsmith never processes media itself, which sidesteps GPL implications and the bundling burden.
- **Steelman:** null (the bundling counter-argument is captured as cross-06).
- **Occurrences:** (spec §2 dependency row and §12 licensing rationale are the same decision in one document -> one occurrence)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §2 (mkvtoolnix row) + §12 | "External, user-installed, CLI invocation only. No linking, no GPL implications, no bundling burden. Detected at startup." |
| 2026-07-08 | decided | commit 61249f9 | plan-1 implementation |

---

## cross-05-dry-single-core - One core crate owns all logic; CLI/GUI are renderers
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Validation, planning and execution share one code path in muxsmith-core; the frontend performs zero semantic validation and round-trips every edit through a core `validate` command; CLI and GUI only render shared structures.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §2 (DRY row) + §7 | "Validation, planning and execution share one code path; GUI and CLI are renderers." |

---

## cross-06-no-bundling-v1 - No mkvtoolnix binaries bundled in v1
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Muxsmith does not bundle mkvtoolnix binaries in v1; a Windows convenience downloader is named as a v1.x candidate only.
- **Steelman:** Bundling the binary would remove the biggest first-run friction - the user having to install mkvtoolnix.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §11 + §12 | "Bundling mkvtoolnix binaries; a Windows convenience downloader is a v1.x candidate." |

---

## cross-07-watch-daemon-mode - Watch/daemon mode as bare v1 non-goal
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** Watch/daemon mode is a v1 non-goal with no rationale and no reconsider trigger: deferred rather than principled-rejected.
- **Blocked on:** post-v1 scope; no v1 use case established.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | deferred | spec §11 | "Watch/daemon mode." (bare v1 non-goal, no rationale attached) |

---

## cross-08-minor-findings-deferred - Six Minor whole-branch findings deferred past fix-pass merge
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The whole-branch review's 6 Minor items (double-report, render-fail donor gap, IdentifyError English, TempDir leaks, double file print, mkvmerge_found JSON asymmetry) were recorded for a follow-up rather than blocking the fix-pass merge.
- **Blocked on:** internal - pending dev work / owner disposition.
- **Occurrences:** (review doc + journal document one deferral event -> one occurrence)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | plan-2-fixes-sdd/FINAL-review.md MINOR section + journal fix-pass Open threads | "6 Minor items from the final review recorded in the archived FINAL-review.md for a follow-up." |

---

## cross-09-plan-wire-format-extension - New serialized Plan fields as conscious wire-format extensions
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** New serialized `Plan` fields (`keep_unmatched`, `primary_track_ids`) are recorded as deliberate wire-format extensions consumed downstream (Plan-4 executor, Plan-5 GUI), not incidental additions.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | plan-4 whole-branch-review-verdict.md (Minor #5) | "keep_unmatched: bool now serializes into any Plan JSON. Additive and documented ... noted only so it is a conscious part of the plan wire format going forward." |

---

## cross-10-jobevent-wire-contract - JobEvent wire shape pinned at three layers
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The JobEvent wire contract is pinned at three layers that fail loudly on drift, not assumed: Rust serde golden test, `src/ipc.ts` mirroring the Rust structs field-for-field, and e2e fixtures typed `satisfies JobEvent`.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | reinforced | plan-5 whole-branch-review-verdict.md (strengths) | "the JobEvent serde golden test, src/ipc.ts mirroring Rust structs field-for-field, and the e2e fixtures typed satisfies JobEvent give three layers that fail loudly on drift." |

---

## cross-11-native-platform-before-dependency - Idiomacy directive gains native-platform-before-dependency clause
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The shared "dependencies are earned" bullet gains a clause preferring a native platform primitive over any dependency that reimplements it (browser/CSS/DB/OS feature over a library that duplicates it).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md candidate 1 | "A covers stdlib<->library but never names the native-platform layer ... Distinct axis. Adopt." |

---

## cross-12-comprehension-gate-minimalism - Comprehension-gate on minimalism
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Minimalism shortens the solution, never the comprehension: trace every file the change touches before choosing the simplest mechanism; a small change in the wrong place is a second bug. Generalizes the diagnosis-scoped verify-topology reflex to the building side.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md candidate 3 | "Best single idea in the whole system; generalizes the diagnosis-scoped verify_topology to the building side." |

---

## Clustering notes (defensibility)

- **cross-01 vs cross-02/03:** the E0 stack record named "over Wails v3 and Avalonia" in its own approach text, but the two rejected alternatives are distinct `(topic, approach)` restraints with their own steelmen, so they are their own clusters rather than folded into the adopted-stack count. This keeps the stack count honest (4 genuine touchpoints of the *adopted* decision) instead of absorbing the alternatives to inflate it.
- **cross-01 count = 4:** spec, journal, handoff and commit are four distinct cited artifacts (two documents on 2026-07-08, plus the 2026-07-09 handoff and the implementation commit). Same-section citations within one document were collapsed (spec §2 "row 5" and §2 "Stack" -> one spec occurrence; journal "bullet 4" and "Plan 1 entry" -> one journal occurrence).
- **cross-04 count = 2:** spec (§2 dependency row + §12 licensing rationale, one document) and commit 61249f9. The two spec sections document one decision in one artifact -> one occurrence, not two.
- **E0 and E1 are reconstruction eras, not separate decision events:** the E1 records re-attest E0 decisions with additional downstream artifacts (handoff, commit). Merging them and counting distinct artifacts - rather than counting one occurrence per era-record - is what the dedup/keep-distinct rule prescribes.
