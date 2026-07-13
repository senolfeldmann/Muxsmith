# Seed 13 — spec §5.2: WorkerPanicked severity cell says "info"

**Verdict: CONFIRMED**

- **File:** `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
- **Line:** 286
- **Tag:** doc
- **Origin:** whole-branch final-verification nit (salvaged: `docs/process-journal/artifacts/plan-5.5-sdd/whole-branch-verdict.md`, line 6): "the WorkerPanicked severity cell says 'info' while the token only ever accompanies a job reported Failed; no code assigns any severity (no Diagnostic is ever constructed) ... 'n/a' would be tighter at next touch."

## State at HEAD

The §5.2 catalog table row still reads:

```
| `WorkerPanicked` | info | a queue worker thread panicked while running a job ... |
```

Verified against source at HEAD that the underlying claim still holds:

- `crates/muxsmith-core/src/executor/queue.rs:408` — `WorkerPanicked` is emitted only as a string token `worker-panicked: job N` in `JobOutcome.errors`; no `Diagnostic` is constructed, so no `Severity` value ever exists in code.
- `crates/muxsmith-core/src/report/mod.rs:173` — only the key mapping `WorkerPanicked => "worker-panicked"`; no severity assignment anywhere.

The cell's "info" is therefore a spec-side invention with no code counterpart: nothing can drift against it, but it misstates the contract (the job is reported `Failed`; "info" suggests a benign rendered diagnostic).

## Fix

Replace the severity cell content at line 286:

- from: `info`
- to: `n/a (job-error token, not a rendered diagnostic)`

Condition-column text is already accurate and needs no change. Pure one-cell doc-precision edit; no code, no dependencies affected.

- **lines_cut:** 0 (in-place cell rewrite)
- **deps_cut:** 0
