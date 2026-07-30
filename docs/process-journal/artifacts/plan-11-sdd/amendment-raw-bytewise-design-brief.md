# Plan 11 amendment - controller brief for the DESIGN half

You author the design for an amendment to an already **owner-approved** plan. This
is the design half of a FOUR-ROLE amendment: you write it against this brief, an
independent reviewer grades it, and only then does the plan's own author fold it
into the plan document with the plan's original reviewer judging that delta. The
amendment is four-role because it RE-CUTS a task - Plan 11's task A3 changes from a
documentation task into a behaviour change with its own tests - which is the case
the doctrine puts beyond a one-pair amendment.

Your artifact: `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`.

## Read first

- `docs/ROADMAP.md`, the "Docs accuracy" entry beginning "THE README'S FIRST
  EXAMPLE" is NOT yours; the one you need is the entry beginning `"byte-exact"
  overstates what `raw:` does for NUMERIC scalars`. **Read it completely.** It
  carries the owner ruling, its refinement, the measured mechanism, and the
  rejected larger variant with its steelman.
- `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md` -
  the approved plan, in particular task A3 as it stands.
- The v1 spec `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, sections
  4.4, 9.2 and the diagnostics table, plus section 4.3 on `exact`.
- `~/agents/peter/prompts/software-dev-process.md` - the doctrine. Section 1's ADR
  slot requirements (every rejected alternative carries its steelman), section 3's
  latitude ban in both forms, and the proposed-safeguard rule bind your artifact.
- Tier-2: `docs/conventions.yaml`, `docs/product-boundaries.yaml`,
  `docs/process-conventions.yaml`.

## The ruling you are designing to

**Owner ruling 2026-07-30: no type casting happens under `raw:`.** His settled
disposition is the SMALL variant - same-type value equality - together with
documenting it precisely. Both the code and the wording are defective, in
different ways, and the amendment carries both halves.

## Measured facts. Verify each; do not inherit them

1. **`scalar_eq`'s cross arms exist FOR the typed path.** Its own doc comment reads
   "Value equality between a profile `Scalar` and a track `PropValue`, with
   int/float cross-comparison (spec 4.3, `exact`)". The README documents that
   behaviour as intended for `exact` on a known property: "numbers numerically
   (`6` equals `6.0`)".
   **THEREFORE: do NOT remove the cross arms from `scalar_eq` itself.** That would
   break documented behaviour of the whole matcher. The defect is that the `raw:`
   arm calls the same function.
2. **Three `scalar_eq` call sites exist** in `crates/muxsmith-core/src/matcher.rs`.
   Establish which is the `raw:` arm, which is the typed arm, and what the third
   (a Boolean false-when-absent shortcut) belongs to. Cite them.
3. **Both sides are parsed before any comparison.** A profile value goes through
   YAML into a typed `Scalar`; a reported value goes through `serde_json`, and
   `PropValue::from` tries `as_i64()` then falls back to `as_f64()`. So `6.0` and
   `6.00` arrive as the same `Float` on either side and the textual difference is
   gone before the comparator exists.
   **Consequence you must carry into the wording: byte-exactness over the textual
   form is UNATTAINABLE by a comparator change, so no site may claim it.** The
   spec, the README and the matcher comment currently do, in four places.

## The rejected alternative, recorded so you do not re-litigate it

Comparing both sides as retained SOURCE TEXT would make `6.0` differ from `6.00`
and take no type decision at all - literally the owner's "no help whatsoever",
since deciding that an unknown property's value is a number is itself an
assumption. Steelman at strength: it is the only semantics under which the word
"raw" is honest, and it removes the last inference from the escape hatch. **He
rejected it** after the cost was named: it needs the `raw:` value to keep its YAML
source form instead of being typed and the reported side to keep mkvmerge's JSON
number token, which the default conversion discards - so it reaches into the
identify layer - and it would need its own rule about whether a JSON string's
quotes are part of the literal, relocating the type question rather than removing
it. Record it as rejected with this steelman; do not propose it.

## Decisions your design must settle, with no latitude left

The latitude ban is absolute and its commoner form is omission: an unenumerated set
in a normative position is latitude just as much as an explicit permission. Ask of
every normative sentence whether the implementer must invent something it is not
allowed to invent.

1. **The comparator's exact semantics** for the `raw:` arm, and where it lives (a
   new function, a parameter on the existing one, or something else) with the
   reason. Name every type pair and its result, exhaustively - a partial table is
   latitude.
2. **The wording target at EVERY site.** Derive the site set by grepping the tree
   for the claim, state your expression, and report the delta against the plan's
   existing six-repair/nine-retain split. Do not work from that split as an
   enumeration; the ROADMAP entry explicitly leaves the nine string-scoped sites
   (which concern `language` and `codec_kind`) to be **re-examined for
   consistency rather than assumed retained**, and that is your call to make with
   its reason.
3. **The test set, and one control in it is mandatory:** a test proving the typed
   `exact` path STILL coerces int against float. Without it, a future change that
   strips the cross arms from `scalar_eq` passes. State it as a safeguard; per the
   doctrine a proposed safeguard is not argued out during design and comes out only
   after being built and measured redundant.
4. **Whether `RawOnKnownProperty`'s scope changes.** Today it warns only for
   `language` and `codec_kind` - the two with special matching semantics - so
   pointing `raw:` at a known NUMERIC property gets `RawProperty` (info) only. The
   new semantics gives that case a newly different outcome, which is an argument
   for widening the warning and also an argument against (the diagnostic surface is
   owner-visible). Decide it, or escalate it as a product question with your
   recommendation - do not leave it open.
5. **Whether the `UnknownPropertySkew` plan-time warning's text needs to change**,
   given it exists to make the untyped match visible.
6. **What the spec's diagnostics-table row for `RawOnKnownProperty` says**, since
   it currently describes the degradation as "byte-literal untyped equality".

## Parity, and state it rather than inventing one

SI-3 binds every behavioural question: compare against mkvtoolnix-gui / mkvmerge,
reading the source at `~/Downloads/mkvtoolnix` and confirming behaviour by running
the binary. **The honest expected outcome here is that there is no parity model** -
mkvtoolnix has no profile concept and no equivalent of a matching escape hatch
(measured earlier this session: zero hits for a preset or profile concept across
its GUI sources). Verify that yourself and classify it as a genuine gap in the
reference tool rather than skipping the duty or inventing a precedent. If you find
anything comparable, it outranks this paragraph.

## Scope

- **This amendment touches Plan 11's task A3 and nothing else.** The dependency
  task, the `BUILDING.md` ordinals, the surviving line citations, the spec CLI
  synopsis and the README example step are approved and stay exactly as they are.
- Do not re-plan, do not re-cut other tasks, do not restate the plan.
- Do not edit any product file, the plan document, the trackers, or the
  house-knowledge YAML files. You write one design document.
- Do not commit, stage or push. Do not create a worktree. Do not call any
  session-relocation tool. To measure something, read, grep, build and run tests
  and binaries; scratch files go under
  `/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/raw-design/`.

## Form

ADR-shaped, with the doctrine's required slots: the decision; its rationale; every
rejected alternative WITH its steelman stated at strength, not as a strawman; any
triggers the design creates (surfaced for the controller to mirror into the
ROADMAP, since you do not write trackers); and the interface consequences, which
here means whether any serialized or user-visible surface moves.

Where a passage of your own concludes that something is unnecessary, RUN the
premise that makes it unnecessary rather than weighing it. That shape has produced
false claims repeatedly in this project, including three in one design round.

## Return value

Your final response is the return value, not a message to a human: the settled
semantics table, the site set with the expression that derived it and its delta
against the plan's split, your decision on each of the six items, the parity
finding, every brief premise you refuted with the measurement that refuted it, and
the file path you wrote. Confirm you committed nothing.
