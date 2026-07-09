# F5 review: planner resolution - SourceOverwrite + identify-failure code

Independent review of `crates/muxsmith-core/src/planner.rs::resolve_file` against
`F5-review-package.txt` (diff), `F5-report.md`, spec 4.8/5.1/5.2
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), and the task
definition in `docs/superpowers/plans/2026-07-09-plan-2-fixes.md` (F5).
Verified by reading the current `planner.rs`/`report.rs`/`identify.rs`,
running the full test suite/clippy, and driving a standalone harness
(path-dependency on `muxsmith-core`, no repo files touched) to observe actual
runtime diagnostics for constructed scenarios beyond what the shipped tests
cover.

## Verdict

- **SPEC: fail.** Fix (b) (identify-failure -> `UnidentifiableSource`) is
  correct. Fix (a) (donor-inclusive `SourceOverwrite`) is only half done: it
  protects a primary's *own* resolved donors, not the batch-wide donor set
  the plan doc and spec 4.8 actually call for. See Critical #1.
- **QUALITY: changes-needed.** Test 3 is well-constructed for the case it
  covers (genuinely isolates the donor-path branch, not an incidental
  on-disk collision); it just doesn't cover the batch-wide case because the
  implementation doesn't handle it. See Critical #1, Important #2, Minor
  #3/#4.

## Critical

### 1. `donor_paths` is scoped per-primary, not batch-wide; a different primary in the same batch can silently overwrite a donor another primary reads from

**Location:** `planner.rs:274` (`donor_paths` declared inside `resolve_file`,
reset to empty on every call) and `planner.rs:418`
(`donor_paths.contains(out)`).

The plan doc's own F5 task text is explicit that this must be batch-wide:

> Collect ALL input paths for the **batch** = primaries + every resolved
> donor. (`docs/superpowers/plans/2026-07-09-plan-2-fixes.md:33`)

and spec 4.8:

> An output path equal to any input path (primary or donor) is always a
> hard `SourceOverwrite` error regardless of policy.

`primary_paths` is built exactly this way: once in `plan_core` (line 145)
from every primary in the whole batch, then passed unchanged into every
`resolve_file` call. `donor_paths`, added by this fix, is **not** analogous:
it's a local `Vec` declared at the top of `resolve_file` (line 274) and
populated only from donors the *current* primary's own rules resolve. It is
never merged across files, and no second pass (comparable to
`detect_output_collisions`) re-checks every plan's output against every
other file's donors.

**Failure scenario** (confirmed empirically with a harness that path-depends
on `muxsmith-core` and drives `plan_batch` directly, mirroring test 3's
construction):

- Batch has two primaries sharing one profile: `Show.S01E01.mkv` (A) and
  `Show.S01E02.mkv` (B).
- A rule with `external: { match_to_source: true }` resolves A's donor to a
  real on-disk file, `donors/Donor.S01E01.mkv` (A's own identifier,
  `S01E01`, is embedded in that donor's name).
- B's identifier is `S01E02`; B's own external-locator lookup for that same
  rule does not match `Donor.S01E01.mkv`, so B never resolves that donor
  itself -> B's local `donor_paths` stays empty.
- `output.filename` is a template with no `{match}` field
  (`'Donor.S01E01.mkv'`), so *every* primary in the batch, A and B alike,
  renders to the identical absolute path `donors/Donor.S01E01.mkv`.
- Result with `on_collision: overwrite` (a real, supported policy):
  - A: `SourceOverwrite` (error), plan dropped - correct, this is the
    same-primary case the fix does handle.
  - B: **plan survives** (`plan.is_none() == false`), output path is
    `donors/Donor.S01E01.mkv`, and the only diagnostic is `OutputCollision`
    at `Severity::Info` (per `detect_output_collisions`'s on-disk-exists +
    `Overwrite` branch, which is documented to *keep* the plan). No
    `SourceOverwrite` is ever raised for B.
  - Consequence: running this profile would invoke mkvmerge to overwrite
    `donors/Donor.S01E01.mkv` while producing B's output - the exact file A
    needs to read its external audio track from. Depending on write vs.
    read ordering this is either a silently corrupted A-run or a
    read-after-write race; either way it is precisely the class of damage
    spec 4.8's "always a hard error regardless of policy" clause exists to
    prevent, and it is not prevented here.
  - With `on_collision: error` (the default) this happens to be masked: the
    on-disk-exists branch maps to `Severity::Error` too, so B's plan is
    still dropped, just via `OutputCollision` instead of the mandated
    `SourceOverwrite`. This is almost certainly why none of the three new
    tests caught it - none of them exercises `on_collision: overwrite`,
    and none of them uses a batch of more than one primary sharing a donor
    pool.

**Fix shape:** treat donor paths the same way `primary_paths` is already
treated: resolve every external-source rule's donor for every primary once
(most naturally as a pre-pass over `primaries` in `plan_core`, or by
collecting `donor_paths` per file inside `resolve_file` as today but then
merging them across all files - e.g. via a batch-wide
`BTreeSet<PathBuf>` built before the per-file `SourceOverwrite` check runs -
so a given file's `donor_paths.contains(out)` check queries the union across
the whole batch, not just its own rules).

## Important

### 2. Delivered donor-identify-failure behavior contradicts the plan doc's own F5 instruction, unreconciled

**Location:** `planner.rs:316-332` (donor `Err(e)` branch, unconditional on
`rule.optional`); `docs/superpowers/plans/2026-07-09-plan-2-fixes.md:34`.

The plan doc that scopes this exact task states:

> Donor-identify-failure must respect `rule.optional` consistently with the
> zero-hits branch.

i.e. when `rule.optional == true`, a donor identify failure should behave
like the zero-hits branch does under `optional` (no error, proceed with
`track_id: None`) - not hard-block the plan. The shipped code and
`F5-report.md` do the opposite on purpose: the donor `Err(e)` branch is
unconditional, and the report frames this as satisfying a "hard error
regardless of optional" requirement, with a dedicated test
(`unidentifiable_donor_yields_unidentifiable_source_not_missing_external`)
proving `optional: true` still yields no plan.

The master spec catalog (`2026-07-08-muxsmith-v1-design.md` 5.2) backs the
shipped behavior: `MissingTrack`/`MissingExternal` explicitly say
"non-optional" in their condition text, `UnidentifiableSource`'s row does
not mention `optional` at all, and 4.8 talks about hard errors "regardless
of policy" in the same spirit. So the delivered behavior is very likely the
actually-intended one - but the plan doc that this task was executed under
(which explicitly claims co-equal "ground truth" status with the spec, plan
doc line 5) was never corrected to match, and neither the implementation nor
`F5-report.md` flags the direct contradiction. Left as-is, a future pass
that takes the plan doc's F5 line at face value could "fix" this back to
optional-gated and silently reintroduce a spec violation.

## Minor

### 3. `F5-report.md`'s GREEN-state narrative for test 3 is factually wrong

**Location:** `F5-report.md:116-121`; `planner.rs:164-171` (`plan_core`'s
`finalize_plans` -> `detect_output_collisions` -> `finalize_plans` sequence).

The report claims: "`OutputCollision` still fires alongside `SourceOverwrite`
in test 3 ... both are independently true facts about the same rendered
path." Running test 3's exact scenario (verified via the same standalone
harness) shows only `SourceOverwrite` in the resulting diagnostics -
`finalize_plans` runs *before* `detect_output_collisions` and already drops
the plan on the `SourceOverwrite` error found inside `resolve_file`;
`detect_output_collisions`'s first loop (building per-path counts) and
second loop (`let Some(plan) = &f.plan else { continue }`) both skip a file
whose plan is already `None`, so `OutputCollision` is never added for that
file. This doesn't affect test 3's correctness (it asserts `SourceOverwrite`
presence and `plan.is_none()`, not `OutputCollision`), but the report
mis-describes the control flow it claims to have traced, which is worth
distrusting elsewhere in the same report.

### 4. `detail` embeds raw Rust `Debug` output directly into user-facing Fluent text

**Location:** `planner.rs:250`, `planner.rs:323` (`format!("{e:?}")`);
`locales/en/diagnostics.ftl:43` (`unidentifiable-source = ... { $detail }.`).

`IdentifyError` only derives `Debug`, so `detail` becomes e.g.
`Runtime(Spawn(Os { code: 2, kind: NotFound, message: "No such file or
directory" }))` - Rust enum/struct syntax, not a clean sentence - and that
string is interpolated straight into a Fluent message the CLI prints to end
users. Functionally satisfies "carries the underlying error" and avoids
authoring new English prose (matching the plan doc's "third-party
pass-through, allowed" carve-out for bug K), but it's a UX smell (internal
Rust representation surfacing in end-user output) and a stability smell
(the rendered message's shape is now coupled to `IdentifyError`'s derive
output, which isn't a stable contract). Not blocking; worth a `Display` impl
on `IdentifyError` in a later pass if the CLI's `--json`/human output for
this code gets scrutinized.

## What's actually correct (checked, not just assumed)

- The redundant `out == &primary.path` clause is safely removed:
  `primary_paths` is built once per batch from every primary
  (`plan_core:145`) and is always a superset containing `primary.path`
  itself for every `resolve_file` call, so the clause was provably dead code
  even before this diff.
- Within one primary, `donor_paths` correctly accumulates across every
  external-source rule that resolves exactly one donor (the push sits inside
  the per-rule loop, unconditional on which rule index it is), and correctly
  records the donor even when that donor's own identification later fails
  (`donor_paths.push` happens before the `id.identify(&donor)` call) - so an
  unidentifiable donor still counts as "read from" for this primary's own
  `SourceOverwrite` check.
- `UnidentifiableSource` fires for both the primary and the donor
  identify-failure paths, `detail` is non-empty (real `IdentifyError` debug
  text), `plan` is `None` in both cases, and neither `MissingTrack` nor
  `MissingExternal` fires instead - confirmed by the two dedicated tests,
  which are not vacuous (they assert absence of the old codes, not just
  presence of the new one).
- Test 3's directory layout (donor in a sibling `donors/` dir, outside the
  scanned `src/` tree) is a deliberate and correct choice to avoid the donor
  being independently re-discovered as a second primary, which would have
  made `primary_paths.contains(out)` true too and defeated the test's
  purpose of isolating the new `donor_paths` branch - confirmed this is
  exactly what happens if the donor's name overlaps the primary's identifier
  substring and lives in a directory that also gets scanned (see Critical
  #1's construction, where this exact coincidence was deliberately
  engineered to build the batch-wide counter-example).
- No regression in the zero-hits (`MissingExternal`, gated on `!rule.optional`)
  or ambiguous (`AmbiguousExternal`) branches: neither is touched by the
  diff, and neither contributes to `donor_paths`, correctly, since neither
  resolves a single donor.
- `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D
  warnings`, and `cargo fmt --all --check` all reproduce clean on the
  current tree, matching the report's claims for those three commands.
