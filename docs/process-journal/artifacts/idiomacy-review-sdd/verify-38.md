# Verify-38: unused `_state: State<AppState>` on `list_runs` / `get_job_log` (F4-3, yagni)

**Verdict: CONFIRMED**

## (a) Code says what the finding claims — yes

`src-tauri/src/run.rs` at HEAD:

- Line 548: `pub fn list_runs(_state: State<AppState>) -> Result<Vec<RunMeta>, IpcError>` — body calls only `list_runs_in(resolve_runs_root().as_deref())`; nothing reads state.
- Lines 556-560: `pub fn get_job_log(_state: State<AppState>, run_id: String, index: usize)` — body calls only `get_job_log_in(...)`; nothing reads state.
- Lines 544-546 and 554 carry exactly the two doc paragraphs the finding cites: "`state` is currently unused ... part of the given interface for parity with the other run-lifecycle commands and any future runs-root override."

## (b) Replacement is current Tauri 2 idiom — yes

Checked against current official docs (context7, `v2.tauri.app/develop/calling-rust` and `/develop/state-management`, Tauri 2):

- `State` is opt-in per command: a command declares the parameter only when it reads managed state. Commands with zero state params (and zero params at all) are the docs' baseline case, not a special one.
- `State`/`AppHandle`/`Window` params are injected by Tauri and are not part of the JS `invoke` argument object. The frontend confirms this: `src/ipc.ts:278` invokes `list_runs` with **no** args, `:282` invokes `get_job_log` with `{ runId, index }` only. Dropping `_state` is invisible to every caller.
- The repo itself already breaks the claimed "parity": `validate_profile` (`src-tauri/src/lib.rs:297`) is a command on the same D23 IPC surface with no state parameter. The parity rationale describes a convention the codebase does not actually hold.
- No other callers: the only references are the `generate_handler!` registration (`lib.rs:449-450`) and the frontend invokes above; no shell test calls the two functions directly. The change is exactly the scoped 6 lines (two signatures, two doc paragraphs), and re-adding `State` later is one parameter.

## (c) Duplication difference — n/a

No duplication claim in this finding.

## (d) yagni form — passes

Concrete construct (the two `_state: State<AppState>` params, run.rs:548/557, plus doc paragraphs 544-546/554) and concrete replacement (drop the param from both signatures, delete the two paragraphs) are both named.

## Decision guard — no conflict, not tracked

- **D23** (plan-5 memo, "enumerated IPC surface"): fixes the eleven command *names*, not their parameter lists. The signature sketch does not appear in the memo; grep for `list_runs`/`get_job_log`/`State<AppState>`/`_state` across `docs/superpowers/specs/*.md` hits only D23's name enumeration. Dropping the param keeps the enumerated surface intact. The doc comment's "part of the given interface" over-reads D23; the recorded decision does not bind signatures.
- **D26 / ROADMAP S9** ("User-facing runs-root override", run.rs:306): tracks the *override feature* as a deliberate v1.x decision. It does not record keeping an unused state param in the meantime; if the override lands as an AppState-read setting, re-adding the param then is the one-line change the finding already prices in. S9 strengthens the yagni call rather than conflicting with it.
- **ROADMAP group K** (cosmetic cleanup) lists the directly analogous "dead `at` param (load.rs:56,64)" but **not** this one — so the project's own convention treats dead params as cleanup targets, and this instance is not already tracked anywhere (grep of ROADMAP.md, IDEAS.md, specs).
- **Known-non-findings list** (per the F-slice briefs): MUXSMITH_RUNS_ROOT debug seam, fake-mkvmerge copies, RECENT_PROFILES_CAP, regex recompilation, version pins, TS 6.0.3 ceiling — the `_state` param is not on it.

## Conclusion

The finding survives all four refutation gates and both decision-guard checks. Note for the merge stage: applying it should also drop the now-dangling cross-reference in `get_job_log`'s doc ("see [`list_runs`]'s doc", line 554) — the finding's replacement already covers this via "delete the two doc paragraphs".
