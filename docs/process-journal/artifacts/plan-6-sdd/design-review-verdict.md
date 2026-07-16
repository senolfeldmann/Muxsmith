# Plan 6 design review verdict

Reviewer: independent (did not author the document, did not see the author's reasoning).
Target: `docs/superpowers/specs/2026-07-15-plan-6-design.md` (D41-D48).

- **Round 1** (2026-07-15, doc at 1180 lines): NEEDS FIXES — 2 Major, 6 Minor.
- **Round 2** (2026-07-15, doc at 1747 lines): NEEDS FIXES — 1 Major, 3 Minor.
- **Round 3** (2026-07-15, doc at 1869 lines): NEEDS FIXES — 1 Major, 1 Minor.
- **Round 4** (2026-07-15, doc at 1951 lines): **APPROVED. No findings.**

Rounds are recorded newest-first. Earlier rounds are kept in full: what was found
is worth more than a clean final page.

---

# ROUND 4 — final

## Verdict: APPROVED

No Major, no Minor, no notes-that-are-really-findings. The document is right. I am
not going to manufacture a fifth round of polish out of a correct artifact —
reaching for a reason to keep working is the same failure this review has now
caught three times in the other direction.

## The one that was mine to answer: the equivalence test I ran in round 2

The question: deriving all 17 makes three of them stop matching the pristine
control. Does my round-2 equivalence property dying mean something broke?

**Measured it myself, on the real model, deriving all 17:**

```
Profile.output       default = {}          (pristine: {"filename":"keep","on_collision":"error"})
Profile.attachments  default = {}          (pristine: {"rules":[],"unmatched":"keep"})
Profile.tags         default = {}          (pristine: {"global":"keep","track":"keep"})
Profile.chapters     default = "keep"      unchanged
Profile.title        default = "keep"      unchanged

$defs/OutputCfg.filename          default = "keep"     survives
$defs/OutputCfg.on_collision      default = "error"    survives
$defs/AttachmentsCfg.unmatched    default = "keep"     survives
$defs/TagsCfg.global              default = "keep"     survives

from {}                        -> OutputCfg { directory: None, filename: Keyword("keep"), on_collision: Error }
from {filename,on_collision}   -> OutputCfg { directory: None, filename: Keyword("keep"), on_collision: Error }
EQUAL? true    both == OutputCfg::default()? true
```

Three of seventeen, exactly as reported; the two enum-valued Profile fields are
unaffected. **The author's information-loss argument holds, and both of its legs
are verified**: the two annotations deserialize to an identical model, and the
children's annotations survive in `$defs` intact.

**The proxy outlived its usefulness, and I should say exactly why, because it was
my test.** Byte-identity against the pristine control was the *correct* test in
round 2 and is the *wrong* test now, and the reason is not that the standard
slipped — it is that the control moved. The pristine schema is the schema of a
product that **materializes** defaults. D48 changes the product so a fully-default
`output` is never written to a file at all. So the pristine annotation
`{"filename":"keep","on_collision":"error"}` now describes a shape D48 *guarantees
will never appear in a saved profile*. When the annotation was a hand-written
literal, byte-identity verified the literal was right; once the annotation derives
from the same function serde uses, demanding byte-identity with a pre-D48 control
is demanding the schema describe pre-D48 behaviour.

**There is a stronger argument for the author's call than the one the document
makes, and it is worth having:** for those three fields the derived `{}` is not a
degraded annotation, it is the **accurate** one. The schema and the serializer now
say the same thing — "a fully-default `output` is nothing" — where the pristine
form would have had them disagree. That is a coherence gain, not a tolerated loss.

**On the named consumer, `yaml-language-server` under D47.** No harm, checked
against how the annotation is actually used:
- **Validation** is unaffected — both `{}` and the populated object validate, and
  `default` is not a constraint.
- **Hover** on `output:` shows `Default: {}`, which is correct (verified: it
  deserializes to `OutputCfg::default()`). And under D48 `output:` is *absent* from
  a saved profile, so the only user hovering it is one who typed it themselves;
  the children they then complete carry their own annotations from `$defs`.
- **Any tool that pre-fills from a parent `default`** now inserts `{}` — which is
  precisely what D48 rules a profile should contain. The old behaviour would have
  had the schema fighting the serializer.
- **Our own GUI is not a consumer**: D45 rejects schema-driven forms.

The document's own drill-into-`$defs` claim ("A reader asking what an absent
`output` means drills into `$defs/OutputCfg` and finds `filename: keep`,
`on_collision: error` exactly as before") is verified true, right-hand column above.

Hand-writing three literals to preserve the pristine bytes would have reintroduced,
for cosmetics, the exact second copy the section exists to delete. The author made
the right call and recorded the divergence rather than hiding it — including
correcting the interface claim to "**unchanged for 14 of the 17 fields**", which is
now precisely true, and carrying the `{}` outcome into §8's pre-decided list so an
implementer cannot "fix" it back to a literal.

## Everything else in the delta

| Item | Verdict | Verified how |
|---|---|---|
| **R3-1 (Major)** — "cannot derive" | **CLOSED** | All 17 derive: `serde(default = "F")` / `skip_serializing_if = "is_F"` / `extend("default" = to_value(F()))`. One function, three mentions, no copies. The false sentence is gone. |
| Guard 2 dropped | **CORRECT** | It asserted "schema `default` == `to_value` of the serde default" while the annotation now *is* that expression — a tautology. Guard 1 (all-non-default round-trip) correctly stays: it guards the predicates, not the annotation. |
| `only drift surface` / `guarded, not trusted` | **CORRECTLY RETIRED** | Both phrases now absent (grep). They described a second copy that no longer exists; leaving them would have contradicted the derivation. |
| **R3-2 (Minor)** — gate overclaim | **CLOSED, and the closure checks out** | Corrected with the measured table. The claimed backstop is real: `pnpm build` is at `ci.yml:120` with **no `if:` guard**, inside the matrix job — so it runs on *every* leg, not just Linux, and a missing `keywords.ts` is an unresolved import in the registries that fails it. Declining `git status --porcelain` is right: it would turn every leg into a working-tree cleanliness assertion, which is a different check with its own false positives. |
| Stale steelman cut | **CORRECT, and cut the right way** | The emit-defaults steelman had credited emission with dodging a drift surface the `extend` annotation introduced; the annotation introduces none now. Verified gone (grep), and the removal is *recorded with its reason* rather than silently excised — so the next reader does not re-add it. |
| Surviving steelman still true? | **YES, checked line by line** | Self-documenting file; `core-83`'s owner reasoning; `tracks.unmatched` absent-means-destructive; "costs zero new code: no predicates, no `skip_serializing_if`, no `schemars(extend)`, and no guard test" — all still true after round 4, and the "no guard test" clause is *more* accurate now than when it was written, since omission's cost dropped to guard 1 alone. The summary ("emission is safer, simpler, and better documentation; it is worse only at the one thing this plan happens to care about") stands unaltered and still passes the believer test. |

Nothing regressed: latitude scan clean, the 43/42/1 enumeration and 13+7=20 closure
intact, every citation corrected in rounds 1-3 still resolves.

## Harvest addendum — two entries the author authored, both worth taking

Both are the author's own phrasings and both are better than mine:

1. **On guards vs derivation** — *"a guard is what you build when derivation is
   unavailable; if you still want the guard afterwards, you probably didn't
   derive."* This retires my round-2 framing entirely (I had it backwards) and
   gives the round-1 pattern its missing test. **Ledger-ready as the operational
   half of *derive the second copy from the first*.**
2. **On steelman maintenance** — *"a steelman resting on a claim that has since
   become false is not a strong argument, it is a stale one."* This is new and it
   generalises past this document. The steelman doctrine says record the losing
   argument at its strongest; the unstated corollary is that a recorded steelman is
   **load-bearing text with a shelf life**, and a decision that invalidates one of
   its premises obliges a revisit. Cutting rather than patching is the right
   default, because a patched steelman quietly becomes the reviser's argument, not
   the believer's. **Ledger-ready.**

Round 3's entry stands, with its evidence now complete: the "no work needed here"
tell fired three times, survived the author's own self-diagnosis, and was caught
each time only by an external check. That is the strongest argument for four-eyes
this project has produced, and it was produced by the mechanism itself.

## Closing assessment

Four rounds, 2+6, 1+3, 1+1, 0. Twelve findings, none contested, and the one place
the author overruled me it was right to — which is how this is supposed to work:
the reviewer is not the ceiling, and a document that only ever absorbs the
reviewer's view is a worse document than one that argues back and wins.

The last round is the one worth noting. The fix I forced (derive all 17) had a
consequence I did not foresee, and the author neither hid it nor let it silently
override a property I had measured: it measured the divergence, reasoned about
whether the property was the goal or a proxy for the goal, concluded correctly
that it was a proxy, and recorded the whole thing including the part that made its
own document look less tidy. That is what the record is for.

**APPROVED.** Ship it.

---
---

# ROUND 3 — re-review of the delta

## Verdict: NEEDS FIXES — 1 Major, 1 Minor

## The headline: the author's overshoot is a better fix, and I was wrong

Answering the question I was asked, directly. **It is a better fix, not a costing
that flatters itself.** Both halves of the author's case check out:

**1. "Derivation beats a guard" is correct, and it is my own harvested pattern
applied better than I applied it.** I offered "add the guard test, or record the
gap", preferring the guard. The author is right that a guard *detects* drift while
derivation makes it *unrepresentable*, and right that the pattern I harvested in
round 1 — *derive the second copy from the first so the two can never drift*,
`capability/mod.rs:125-129`, which D46's rider already cites as its model — points
at derivation. I named the pattern and then recommended the weaker instrument
against it. The author caught that; I did not.

**2. The costing claim is TRUE. I tested it rather than admiring it.** Built a
scratch git repo and ran the real gate:

| scenario | `git diff --exit-code src/bindings/` |
|---|---|
| tracked `keywords.ts` modified (**the drift scenario**: keyword added -> emitter rewrites) | **exit 1 — caught** |
| tracked file deleted (emitter stops writing it) | **exit 1 — caught** |
| new file, untracked | exit 0 — invisible (see R3-2) |

So the directory-level gate **does** cover `keywords.ts` for free once it is
committed, and a stale array **does** become a red CI leg. The marginal cost really
is the emitter alone. The emitter also genuinely rides the same invocation: ts-rs's
own export is a test side effect of `cargo test -p muxsmith-core --features ts`, so
a sibling `#[test]` behind the `ts` feature writing `keywords.ts` runs in the same
command the CI step already runs. "D44 already builds this" is intra-plan rather
than pre-existing — `src/bindings/` does not exist in the tree today — but D44 and
D46 land in the same plan, so that is legitimate, not a borrowed asset.

**NEW-1 (round-2 Major): CLOSED, by a stronger fix than the one offered.**

## But the exemption it draws for D48 is false, and that is the new Major

### R3-1 (Major) — "`schemars(extend)` needs a literal in an attribute position" is false; D48 *can* derive

**File:** `:932-934`.

> "D48 guards rather than derives only because it *cannot* derive - `schemars(extend)` needs a literal in an attribute position. Here derivation is available, so the weaker instrument is not the right one."

**Refuted empirically.** I patched the real model to replace D48's hand-written
literal with a derived expression naming the same function the `default` attribute
names:

```rust
// was:  #[schemars(extend("default" = "drop"))]
#[serde(default = "drop_policy", skip_serializing_if = "is_drop_policy")]
#[schemars(extend("default" = serde_json::to_value(drop_policy()).unwrap()))]
```

It compiles, and the schema is byte-identical to both the literal and the pristine
control:

```
TracksCfg.unmatched = {"$ref":"#/$defs/KeepDrop","default":"drop","description":"..."}
default is  = Some(String("drop"))
MATCHES pristine "drop"? true
```

`schemars(extend)` accepts an arbitrary expression. It does **not** need a literal.

**What follows, and it is worth real money:**

- **D48 can derive all 17.** Each `extend` can call the very function its `default`
  attribute names — `serde_json::to_value(drop_policy())`, `serde_json::to_value(default_true())`,
  `serde_json::to_value(OutputCfg::default())`, and so on. That is D48's own stated
  principle ("**Every predicate calls the very function the `default` attribute
  names**") extended to the third consumer, which is exactly what D46's rider does
  for its three consumers.
- **The last drift surface disappears.** D48 currently says the `extend` literal
  "is the **one** place the default is written a second time, and it is the only
  drift surface this decision opens. It is guarded, not trusted" (`:1191`).
  Derivation deletes the second copy. Nothing is left to trust or to guard.
- **Guard 2 becomes redundant.** The 17-row table test asserts "the schema's
  `default` equals `serde_json::to_value` of that field's serde default." Under
  derivation the annotation *is* that expression, so guard 2 asserts a tautology.
  A 17-row test can be deleted. (Guard 1, the all-non-default round-trip fixture,
  is untouched and stays — it guards the predicates, not the annotation.)

**This is the same tell, a third time, inside the paragraph that diagnoses the
tell.** The author's own self-diagnosis (per the note passed to me): the standard
got applied where it had just been thinking hard, and not where it was reaching
for a reason to stop, and *the tell in both cases was a rationale that concluded
the work was unnecessary*. "D48 cannot derive" is precisely that rationale: it
concludes no work is needed on D48, and it is false. The author established the
right principle, applied it correctly to the keywords, and then wrote an exemption
to keep it away from the seventeen annotations next door.

**In fairness, and this matters:** I did not catch this in rounds 1 or 2 either. I
reproduced D48's mechanism myself with hand-written literals and never asked
whether an expression would work. The author's round-3 argument is what made me
test it. A false claim that provokes the right experiment is still a false claim,
but the review is better for it having been made.

**Required change:** delete the "cannot derive" sentence (it is false). Then either
(a) derive all 17 `extend` annotations and retire guard 2 — which I would take, it
is the same principle the author just argued for and it removes the document's last
self-declared drift surface — or (b) keep the literals and justify them on a reason
that survives, at which point the D46/D48 asymmetry needs a different argument than
capability.

### R3-2 (Minor) — the gate covers *tracked* files in the directory, not the directory

**File:** `:940-942` ("already gates it with `git diff --exit-code src/bindings/`
... covers `keywords.ts` **for free** the moment it lands in the same directory").

Verified: a new **untracked** file in the gated directory returns **exit 0** —
invisible. `git diff` does not see untracked files. So "the moment it lands in the
directory" is not quite the condition; the condition is "the moment it is
committed."

Narrow, and I want to be clear it does not damage the argument: the drift scenario
that matters (keyword added -> tracked `keywords.ts` changes) is caught, and a
never-committed generated file is caught by a different gate — the TS build fails
on the missing import, since `keywords.ts` is imported by the registries. So the
hole is covered, just not by this check. Worth one word of precision in a document
held to this standard, and worth knowing it applies to D44's `profile.ts` equally.
If the controller wants it closed properly rather than incidentally,
`git status --porcelain --untracked-files=all src/bindings/` is the check that sees
both.

## Disposition of the four round-2 findings: all closed

| # | Round-2 finding | Disposition | Verified how |
|---|---|---|---|
| NEW-1 | Keyword drift accepted on a refuted premise | **CLOSED, better than asked** | Array is now generated. Costing claim tested against real git (table above) and holds. The dead `profile_version` rationalisation is **kept with its refuting citation** rather than deleted — the right call: an argument that collapses under its own source is exactly what the next reader rebuilds from scratch, and the record now stops that. |
| NEW-2 | `FieldWidget` claimed 11, union defined 10 | **CLOSED** | `:774` now reads "**(c) `FieldWidget`: 10 variants, closed. Derived.**"; union recounts to 10. Root cause named (`fixed` was a widget before `FieldSpec` split; the label never followed the refactor) — the list was right, the label lagged. |
| NEW-3 | "create" premise unsupported; "derived" over-claimed | **CLOSED, honestly** | Now four labelled parts with "**(a) and (c) are derived; (b) is a decision on a safe default**", and explicitly: "an earlier draft called every part derived and one of them is not." (a) now derives from the forcing function "with no appeal to the spec's view of the editor's scope" — which is exactly the independent footing I said it stood on. No appeal to `create` survives. The enumeration is untouched, as it should be. |
| NEW-4 | 42 called "the `gui-editor.ftl` key count" | **CLOSED, and the routing is verified, not assumed** | `:766` now says "**registry label-key count** - not `gui-editor.ftl`'s total". Per-catalog table (`:1615-1619`) routes 42+1=43 -> `gui-editor.ftl` (new), apply's 2 -> `gui-batch.ftl`, codes -> `gui-common.ftl`. I checked all three against the tree: `gui-batch.ftl` and `gui-common.ftl` both exist, and `gui-common.ftl:26` is literally headed "## T9: shell-level IPC error codes (src-tauri/src/error.rs::IpcError)" — so the error-code row is filed correctly. Apply's strings belong in `gui-batch.ftl` because spec 8.2 puts apply-suggestion in the batch view, which the document itself established at D41. No row mis-filed. |

## Nothing regressed

- Latitude re-scan over 1869 lines: only the document's own meta-statements. No
  ellipsis survives in a normative type or registry position (the four remaining
  `...` are a path elision, the ADR's own retrospective, a quoted `tsc` message,
  and the modeline example).
- The 43/42/1 enumeration, the 13+7=20 closure, the widget list, D48's table and
  predicates, and every citation I corrected in rounds 1-2 are unchanged and still
  correct.

## Harvest addendum — and yes, it generalises

The author's self-diagnosis should go in the ledger, and **R3-1 is the evidence that
it generalises**: the tell fired a third time *inside the paragraph diagnosing the
tell*. That is the finding. Awareness did not prevent it. The entry should therefore
carry the mechanical form, not the introspective one:

> **A rationale whose conclusion is "no work needed here" is the highest-risk
> sentence in a design document.** It is where the standard applied everywhere else
> gets suspended, and it is not caught by intending to catch it. Three instances in
> Plan 6, each an exemption from a rule the same document was enforcing elsewhere:
> the `profile_version` bump argument (round 2), "the schema cannot express our
> keyword domains" as a *deliberately unrecorded* argument (which the document
> caught in itself, correctly, in its own §0 correction #7), and "schemars(extend)
> needs a literal" (round 3). **Check: when a paragraph concludes that a guard, a
> derivation, or an enumeration is unnecessary, verify the claim that makes it
> unnecessary before accepting the conclusion.** That is a mechanical trigger a
> reviewer can run; "notice when you are reaching for a reason to stop" is not.

Worth noting the document already contains one instance where the author caught this
in itself unprompted (§0 correction #7). So the pattern is not "the author rationalises";
it is "this failure mode survives self-inspection and needs an external check" — which
is a much better argument for four-eyes than any I could construct.

Round-1's entry stands and is strengthened: *derive the second copy from the first*
now has three positive instances (`CODEC_KIND_NAMES`, D46's constant set, D48's
predicates), one **newly available** instance the document should take (D48's
`extend`, per R3-1), and one correctly-taken instance (the keyword arrays). The
round-2 "guard test is the fallback" framing was **my error** and should not reach
the ledger: the fallback is a guard only where derivation is genuinely unavailable,
and Plan 6 has not yet produced a case where it is.

## Assessment of the delta

Three rounds, eight + four findings, none contested, and the one place the author
overruled me it was **right to**. That is the mechanism working in the direction it
is supposed to work: the reviewer is not the ceiling.

The remaining Major is one false sentence and the exemption it buys. Delete it,
derive the seventeen, drop guard 2, and the document has no self-declared drift
surface anywhere in it — which is a better resting place than it has occupied in any
round so far. That is a small edit with an unusually good return.

---
---

# ROUND 2 — re-review of the delta

## Verdict: NEEDS FIXES — 1 Major, 3 Minor

Scope: the delta only. The sixteen passing steelmen, the measurements reproduced
in round 1, and the ADRs not flagged are settled and were not re-litigated.

## Disposition of the eight round-1 findings: all closed

| # | Round-1 finding | Disposition | Verified how |
|---|---|---|---|
| M1 | D45 latitude: three unenumerated sets (`...`) | **CLOSED** | All three closed. 13 registries + 7 `never`-arm enums = 20 = the whole model. `FieldWidget` closed as a union. See New-3 for a caveat on the *derivation*, not the enumeration. |
| M2 | Six ROADMAP citations misresolve; §7 items 7/8 already landed | **CLOSED** | Re-resolved all six against HEAD myself: `:19-27` -> corrected D22 text; `:29-64` -> Plan 6 header; `:38-64` -> "Named design inputs (6):"; `:52-55` -> the trigger correction; `:58-64` -> input #6 (`runActive`); `:68-70` -> Plan 7. All correct. Items 7/8 now read "RESOLVED in `fdcdcba`, verified at HEAD - do not re-issue" (`:1683`, `:1689`); D46's trigger paragraph now concurs with a fix already made (`:1072`). |
| m1 | D48 overstated the silent surface (4 vs 2) | **CLOSED** | 4-row table (`:1330-1333`) matches my own measurement exactly, including `error[E0277]: the trait bound FilenameCfg: Default is not satisfied`. Adds the right conclusion: "two of the four cannot reach data, because the type system stops them." |
| m2 | `check-i18n.mjs:172-179` wrong | **CLOSED** | Now `:191-198` with the `includes` test at `:193`, and it volunteered the half I flagged: `\|\| text.includes(\`'${id}'\`)`. |
| m3 | `check-i18n.mjs:110-127` wrong | **CLOSED** | Now `:102-120` (`:949`). |
| m4 | `jobRowState.ts:44-54` off by one | **CLOSED** | Now `:44-55` at both sites (`:46`, `:888`). |
| m5 | D47's population argument does not reach the VS Code path | **CLOSED** | Deleted, not repaired. `grep "close to empty\|population it serves"` returns nothing. The rejection now rests on the unrequested-write argument, which was always the sufficient one. |
| m6 | `gui-22`/`exec-44` needs a verdict | **CLOSED** | Now carries both ledger refs (`:243-252`, `:15-23`) and is classified as one of the four observable contested criteria in `proc-latitude-clause-boundary` (`:1698-1705`). |

The author pushed back on none and checked each. Every correction I could
mechanically verify, I did, and all landed.

## What I verified with my own hands in the delta

| New claim | Result |
|---|---|
| 43 fields across 13 structs; 42 editable, 1 fixed | **CONFIRMED, recounted mechanically from `model.rs` + `match_expr.rs`.** Every per-struct count in the table is right (Profile 9, Meta 2, Input 3, OutputCfg 3, TemplateBlock 1, ExternalBlock 1, TrackRule 4, Locator 6, AttachmentsCfg 2, TracksCfg 2, AttachmentRule 3, TagsCfg 2, MatchExpr 5 = **43**). 13 structs in the model, 13 in the table, no residue. `Profile.profile_version` is the single fixed field. |
| 13 structs + 7 enums = 20 = the whole model | **CONFIRMED.** The 7 enums (`FilenameCfg`, `SourceCfg`, `ChaptersCfg`, `TitleCfg`, `CollisionPolicy`, `KeepDrop`, `Scalar`) are exactly the model's enums. No type is in both lists or in neither. |
| `optionalFlag` is two-state, not tri-state, because `validate.rs:466-472` rejects `Some(false)` | **CONFIRMED, citation exact to the line.** `:466-472` is precisely `if locator.match_to_source == Some(false)` -> `InvalidKeyword` with `found="false"`, `allowed="true"`. The Rust type says three states; validation permits two. Deriving the widget from the evidence rather than the type is the right call and a genuinely good catch. |
| `export_to` self-catch: the original shape would have emitted 20 files | **CONFIRMED, and the fix is right.** `ts-rs-12.0.1/src/lib.rs:204-208` reads: "Defaults to `<name>.ts`. The path given to the `export_to` attribute is relative to the `TS_RS_EXPORT_DIR` environment variable, or, if `TS_RS_EXPORT_DIR` is not set, to `./bindings`." So without `export_to`, 20 types -> 20 files; `export_to = "profile.ts"` on each -> one file. The `:206-207` sub-citation for the `TS_RS_EXPORT_DIR`/`./bindings` default is also exact. |
| D48's `E0277` split | **CONFIRMED** (re-measured in round 1; the table now matches it). |
| Catalog count 42 derivable from D45's table | **CONFIRMED** as arithmetic (42 `EditableField` -> 42 `labelKey`). See New-4 for a wording tension. |
| `FieldWidget`: "11 variants, closed" | **REFUTED — the union defines 10.** See New-2. |
| Keyword-array drift is safe because a keyword change needs a `profile_version` bump | **REFUTED by the document's own citation.** See New-1. |

This is the self-catch worth naming: closing the `export_to` ellipsis is what
exposed the 20-file defect. The enumeration discipline paid for itself inside the
same fix round — an ellipsis was hiding a real bug, not just an unmade decision.
That is the strongest evidence that M1 was worth raising.

---

## New Major

### NEW-1 (Major) — the keyword arrays accept an unguarded drift surface on a premise its own citation refutes

**File:** `:827-843`.

The four TS arrays (`FILENAME_KEYWORDS`, `SOURCE_KEYWORDS`, `CHAPTERS_KEYWORDS`,
`TITLE_KEYWORDS`) restate D46's Rust constants by hand. The document accepts the
drift risk with this reason:

> "The drift risk is accepted **because a keyword-domain change is a profile-format change**: spec 4 increments `profile_version` 'only on breaking format changes', so it is a coordinated edit across the spec, the model, `validate.rs`, the schema and the GUI - never a one-line slip."

**The quoted words defeat the argument.** Verified verbatim — spec `:46`: "`profile_version: 1` required; incremented on **breaking** format changes", and `model.rs:20-21`: "Incremented **only on breaking** format changes (spec 4)." The inference silently swaps *format change* for *breaking format change*. They are not the same set, and the difference is exactly the likely case:

- **Adding** a keyword (say `chapters: merge`) is **not breaking** — every existing profile still parses; a new value simply becomes legal. So **no `profile_version` bump is required**, and none of the coordinated-edit machinery fires.
- D46's rider makes that a genuine **one-line edit**: adding to `ChaptersCfg::KEYWORDS` flows automatically to the guard, the `allowed` param and the schema. That is D46 working as designed.
- It does **not** flow to `CHAPTERS_KEYWORDS` in TS. The GUI silently stops offering a keyword the format now accepts.

So the "never a one-line slip" claim is false for additions and true only for
removals, which are the rarer case. The asymmetry with D48 does not hold as
stated. Answering the coordinator's question plainly: **it is a rationalisation**,
though I think an unintentional one — the author's other two arguments (ts-rs
exports types not values; the untagged enums project to `Block | string` so no
`satisfies` guard can reach the domain) are both **correct and verified**, and
those are what actually forced the hand. The defect is the third argument, which
converts a *forced* gap into a *safe* one.

**The rejection considered two expensive options and missed the cheap one the document already uses.** Rejected: generating `keywords.ts` (a second artifact plus an emitter) and adding marker enums. Both fairly costed. But neither is needed — the requirement is not to *generate* the array, it is to *fail the build when the two disagree*. That is a guard test, and D48 four sections down mandates exactly this shape for exactly this reason:

- D48 guard 2 is a table test asserting the schema's `default` equals each field's serde default, guarding the single drift surface `extend` opens, under the words **"It is guarded, not trusted"** (`:1191`).
- The house pattern is already in the tree: `capability/mod.rs`'s `settable_maps_to_mkvmerge_options` asserts a `const EXPECTED` table against the real thing, and `CODEC_KIND_NAMES` is derived from `CODEC_KINDS` with the doc comment "so the two can never drift" (`:125-129`) — which D46's rider itself cites as its model.

Six literals guarded by one table test is cheaper than either rejected option and
is the document's own doctrine. Accepting an unguarded second copy, four sections
from a decision that guards one, is the inconsistency.

**Blast radius, stated honestly so it can be weighed:** this is **degraded UX, not
data loss.** A profile carrying a GUI-unknown keyword still round-trips — the
model holds `Keyword(String)`, so D41/D48 preserve it on save. The failure is that
the GUI cannot offer a newly legal keyword, and may mis-render an existing one.
That is materially less severe than D48's hazard. It is Major because the *reason*
is refuted rather than merely thin: a rejection recorded on an argument that
collapses under its own citation is precisely what gets re-litigated the moment
someone reconstructs it, which is the failure the steelman doctrine exists to
prevent.

**Required change:** drop the `profile_version` argument (it is wrong), keep the
two that are right, and either add the guard test or record the gap honestly as
*accepted and unguarded* with the reason being tooling limits rather than safety.
The first is cheap and I would take it.

## New Minor

### NEW-2 (Minor) — `FieldWidget` is claimed as 11 variants; the union defines 10

**File:** `:712` vs `:721-731`.

`:712` reads "**(b) `FieldWidget`: 11 variants, closed.**" The union enumerates
**ten**: `text`, `bool`, `optionalFlag`, `select`, `keywordOrBlock`,
`directoryPath`, `stringList`, `propertyMap`, `list`, `section`. Counted
mechanically.

The enumeration is what binds and it appears **adequate** — I walked all 42
editable fields against it and each has a fitting widget, so an implementer builds
10 and is correct. This is a wrong label on a right list.

It is flagged at Minor rather than trivial because of what it is: the document
raised, correctly, that the ROADMAP asserted "(6 / 5 / 1 / 8)" as "recountable
rather than asserted" while its own bullets did not recount — and then shipped a
count that does not recount. Fix the label to 10, or add the missing variant if
one was cut in editing.

### NEW-3 (Minor) — the "create" premise is not supported by the text cited for it

**File:** `:684-690`.

> "spec 8.2 requires the editor to open **and create** profiles ("open/save YAML"), which it cannot do if any field is unreachable."

Spec 8.2 (`:373`) says, in full on this point: "open/save YAML, recent profiles."
It does not say *create*. `grep -i "create a profile|new profile|profile creation|creating a profile|from scratch|author a profile"` across the whole spec returns **zero hits**. Saving an opened profile is not creating one, and "recent profiles" reads naturally as opening existing ones. The parenthetical citation does not carry the word attached to it.

This matters because the section's own claim is that each set is "**derived**, not
chosen."

**But the conclusion does not collapse, and I want to be exact about that rather
than adopt the framing I was handed.** The derivation has two halves, and only one
leans on `create`:

- **(a) all 13 structs get registries, all 7 enums get `never` arms.** This stands
  **independently**, on the forcing function's own logic: the registry's value is
  total coverage, and `FixedField` exists precisely to record deliberate
  non-exposure. Registering a subset means adding a field to an unregistered
  struct is silently unnoticed — the exact failure D45 exists to prevent. Sound
  without `create`.
- **(b) 42 of 43 fields are `EditableField` rather than `FixedField`.** This is
  where `create` bites. Without it, exposing everything is a **choice** — a safe
  one (it cannot strand a field, and each `FixedField` would need its own
  justification), but a choice.

So the enumeration is **not arbitrary in a new costume**; it is right, on a
narrower footing than claimed. `gui-02` is also thinner than its billing: it
settles that there is no *per-file override*, which supports "the profile is the
only lever", not "the editor must expose every field" — and D47 blesses
hand-authoring, so an unexposed field is reachable by hand, not stranded.

**Required change:** stop calling (b) derived. State it as what it is: nothing in
the spec restricts the editor's field scope, `gui-02` makes the profile the GUI's
only lever, so exposing all 42 is the only option that cannot strand a field, and
`FixedField` carries the one exception. That is honest and just as binding. The
reductio already in the text ("an editor that could not set `input.pattern` ...
could not author one at all") is good and should stay — it refutes "spec 8.2's
list is exhaustive", which is all it needs to do.

### NEW-4 (Minor, low) — 42 is called "the `gui-editor.ftl` key count"; section 2's own table says otherwise

**File:** `:705` vs `:1504-1508`.

`:705`: "42 are `EditableField` ... That is the `gui-editor.ftl` key count
(section 2)." But section 2's table lists three contributors: D41 (1 + codes),
D43 (2 + codes), D45 (**42**). D41's save-surface note is an editor-surface string
and plausibly lands in `gui-editor.ftl`, which would make the file's count 43 +
codes, not 42. (D43's apply keys belong to the batch view — the document says so
itself at D41 — so those are presumably `gui-batch.ftl`, but it does not say.)

Section 2's own wording is careful and correct about what 42 *is* (`:1510-1512`,
"one `labelKey` per `EditableField` across the 13 registries"). Only `:705`
overreaches by promoting it to the whole file's count. Fix `:705` to say "the
registry label-key count", and name the catalog D41's note lands in.

---

## Anything the fixes broke

Three things, all listed above and all in the fix round's new material: **NEW-1**
(the refuted keyword premise — the only substantive one), **NEW-2** (the 11-vs-10
count), **NEW-3** (the over-claimed derivation). Nothing that was passing in round
1 regressed:

- I re-resolved all six ROADMAP citations; none broke in the re-pointing.
- The D48 table, the 17-field enumeration, the predicates, both guards and the
  `schemars(extend)` mechanism are untouched and still match my measurements.
- No latitude clause was reintroduced: the keyword scan over the 1747-line document
  returns only the document's own meta-statements about the ban, and the three
  `...` ellipses that constituted round-1's M1 are gone.
- `cargo test` still passes; the schema still emits 17 `default` annotations.
- No new deviation from the Tier-2 files.

## Harvest addendum

One entry strengthens materially. The pattern I named in round 1 — **"derive the
second copy from the first, so the two can never drift"** — now has a **negative**
instance to record alongside its three positive ones (`CODEC_KIND_NAMES` from
`CODEC_KINDS`; D46's constant set; D48's predicate-calls-its-own-default-fn). The
TS keyword arrays are the case where the tooling *cannot* derive and the correct
fallback is a **guard test, not an accepted gap** (NEW-1). A ledger entry that
carries both the pattern and its fallback is worth more than one that carries only
the happy path, because the fallback is where it will actually be tested.

Second: **`FixedField` is a small pattern worth keeping.** "A field that exists and
is deliberately not exposed gets an explicit entry with a reason, never an `Omit<>`"
generalises past this registry — `Omit` silently disables a forcing function
forever, and the document's reasoning for preferring the union over one instance
("One instance justifies the variant because the alternative is wrong, not merely
larger") is the right test.

---

## Assessment of the delta

The fix round is good work. Eight findings addressed, none contested, none
papered over, and one — the `export_to` 20-file defect — found by the author only
because closing an ellipsis forced them to look. That is the enumeration
discipline earning its keep inside a single round, and it is the best argument
available that M1 was worth the cost.

The one Major is narrow and it is the same species as the round-1 citation
findings, in a subtler form: round 1 had citations that pointed at the wrong
lines; NEW-1 is a citation that points at the *right* line and does not say what
it is claimed to say. That survives a grep and only falls to reading the source.
It is worth fixing on the reason alone, since the decision it supports is close to
forced anyway — the honest version ("the tooling cannot derive this; here is the
guard") is stronger than the one on offer and costs one table test.

Fix NEW-1's reasoning and add the guard, correct the two counts, restate (b) as a
choice, and this is approvable.

---
---

# ROUND 1 — original review (superseded above; kept as the record)

**Round-1 verdict: NEEDS FIXES — 2 Major, 6 Minor.** All eight closed in round 2;
see the disposition table above. Retained in full below.

## Verdict: NEEDS FIXES

**2 Major, 6 Minor.** No ADR's *decision* is wrong. Every load-bearing measured
claim I re-ran reproduced, several to the digit. The fixes are to one genuine
latitude hole and to a cross-reference layer that no longer resolves.

This is a strong document. Grading it strictly is the point: it becomes ground
truth, so its citation layer has to survive being followed.

---

## What I verified by running it, not by reading about it

Everything below was re-measured independently. Scratch work outside the repo;
the repo was not modified. Patched-crate probes used a copy at
`/tmp/.../scratchpad/core-d48`.

| Claim | Result |
|---|---|
| `reference.yaml` 80 hand-authored lines, 12 comments | **CONFIRMED** exactly |
| Canonical save **emitting** defaults -> 141 lines, 0 comments | **CONFIRMED** exactly |
| Canonical save **omitting** defaults (D48) -> **112** lines | **CONFIRMED** exactly (applied all 17 predicates to a crate copy) |
| Round-trip yields an equal model (`p == p2`) | **CONFIRMED** `true`, both emitting and omitting |
| 17-field `skip_serializing_if` table, line numbers | **CONFIRMED** all 17 line refs correct against `profile/model.rs` |
| 13-generic / 4-specific split | **CONFIRMED** as a predicate assignment (but see Minor 1 for the risk framing) |
| `KeepDrop::default()` is `Keep`; `tracks.unmatched` defaults to `Drop` | **CONFIRMED** (`model.rs:186-187` vs `:306`, `:314-316`) |
| The hazard: generic predicate omits `unmatched: keep`, reloads as `drop` | **REPRODUCED VERBATIM**: `model in: Keep` -> `saved: "rules: []"` -> `reloaded: Drop` -> `PRESERVED? false` |
| That profile is `core-83`'s ruled-legal pure passthrough; one save makes it `NoTrackRules` | **CONFIRMED** (`core-83` statement verbatim; `validate.rs:60-70` raises `NoTrackRules` on zero-rules+`Drop`, info on zero-rules+`Keep`) |
| `skip_serializing_if` strips `default` from the published schema | **CONFIRMED, and it is total**: pristine schema carries exactly **17** `default` annotations; with D48 skips and no `extend`, **0**. The doc's "all 17" is exact. |
| `#[schemars(extend("default" = ...))]` restores byte-identically | **CONFIRMED**: all 17 restored with matching values; full schema **semantically identical** to the pristine control |
| Whole blocks vanish: `output`, `attachments`, `chapters` | **CONFIRMED** in the measured 112-line output |
| `tags:` survives carrying only `global: drop`; `title: clear` survives | **CONFIRMED** exactly |
| `cargo test` | **PASSES** |
| `cargo run -q -p muxsmith-cli -- schema` self-inconsistency (4 untagged -> bare string; 2 real enums -> `oneOf`/`const`) | **CONFIRMED** |

### Versions (registry-verified, crates.io API with User-Agent, 2026-07-15)

| crate | doc says | registry |
|---|---|---|
| `ts-rs` | 12.0.1 | max_stable **12.0.1** ✅ |
| `yamlpath` | 1.27.0 | **1.27.0** ✅ |
| `yamlpatch` | 1.26.1 | **1.26.1** ✅ |
| `schemars` | 1.2.1 | **1.2.1** ✅ |
| `yaml_serde` | 0.10.4 | **0.10.4** ✅ |
| `serde_json` | 1.0.150 | **1.0.150** ✅ |
| `proptest` | =1.11.0 | **1.11.0** ✅ |
| `tauri-specta` | Tauri-2 line `2.0.0-rc.25`, `max_stable_version` 1.0.2 | newest **2.0.0-rc.25**, max_stable **1.0.2** ✅ |
| `tauri-specta` rc.1 2023-10-04, rc.25 2026-05-08 | | **2023-10-04** / **2026-05-08** ✅ ("two years and seven months" is right) |

`tsc` 6.0.3 and `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit` both confirmed by running them.

### The brief's own premises, as refuted by the document

The document refutes the brief in eight places. I checked the load-bearing ones
and **the author is right in every case I tested**:

- **Correction #1 (`apply_edit_to_first_rule` is a fixture generator, nothing to hoist): CONFIRMED.** `tests/suggestions.rs:95` is `fn apply_edit_to_first_rule(edit: &StructuredEdit) -> String`. It takes no `Profile`, mutates nothing, and bakes `type: subtitles, codec_kind: srt, language: en` in as string literals. Its own comment says it *mirrors* apply. The brief's "reuse before writing: hoist it" directive was wrong and would have imported a test fixture into production. Correctly redirected to `rule_index_of` (`planner.rs:2032`, exact) and `with_rule_match` (`planner.rs:1853`, public).
- **Correction #2 (`yamlpath` is tree-sitter-based, not `yaml_serde`): CONFIRMED.** `yamlpath` 1.27.0 normal deps are `tree-sitter ^0.26.9`, `tree-sitter-yaml ^0.7.2`, `tree-sitter-iter`, `self_cell`, `line-index`, `serde`, `thiserror` — **no `yaml_serde`**. `yaml_serde ^0.10` appears only in `yamlpatch` 1.26.1. The brief inherited the corpus §1 error; adopting the pair would add a second YAML parser. The rejected alternative is *more* expensive than the brief credited.
- **Correction #5 (one i64, not "every numeric field"): CONFIRMED.** `Scalar::Int(i64)` is the only 64-bit integer; `profile_version: u32` maps to `number` regardless.
- **Correction #6 (check 2 does not go blind): CONFIRMED.** Only check 1's hard gate misses the registry.
- **Type count corrected 18 -> 20: CONFIRMED.** 13 structs + 7 enums across `model.rs` + `match_expr.rs`. Counted.
- **D42's `on_blocking` premise, corrected: CONFIRMED AND RIGHT.** No `fs::`/`File::`/`read_to_string` anywhere in `validate.rs` or `lint.rs`; the doc comment "touches no filesystem beyond the profile itself" is real; `get_settings` (`:383`) and `set_settings` (`:391`) are genuinely non-async while `validate_profile` (`:303`) and `detect_mkvmerge` (`:370`) are async. The corrected rationale ("could stall the webview", not "touches the disk") is the accurate one.

`check:i18n` Linux-only gating (`ci.yml:128-130`, `if: runner.os == 'Linux'`), `.gitattributes` `* text=auto eol=lf`, MIT in `deny.toml`'s allow list, xtask's "never invoked at build time" comment, and "no drift check exists today" all **confirmed**.

---

## Major findings

### MAJOR 1 — D45 leaves real design latitude: the registry's domain is never enumerated

**File:** `docs/superpowers/specs/2026-07-15-plan-6-design.md:614-641` (and `:1231-1246`, `:1449-1451`).

D45 is the forcing function for the whole editor, and it is the one ADR that
does not close its own forks. Three unenumerated sets:

1. **Which structs get a registry.** The decision says "One per edited struct,
   colocated with the component that renders it" and then gives exactly **one**
   example (`outputFields` for `OutputCfg`). The set of edited structs is never
   named. The model has 20 types. Spec 8.2 names the editor surface ("track-rule
   grid ... panels for attachments/chapters/tags/title"), but that is not a
   struct list: it does not settle whether `Meta`, `Input`, `Locator`,
   `AttachmentRule`, `TemplateBlock`, `ExternalBlock` or `Profile` itself get a
   registry. An implementer must decide. That is a fork.
2. **`FieldWidget`'s variants.** Line 627 reads
   `widget: FieldWidget;   // discriminated union: text | bool | select | template | locator | ...`
   — a literal `...`. The variant set of the discriminated union is open. D45
   *also* mandates that sum types close with a `never` arm; a `never` arm over an
   undefined domain is not a decision, it is a placeholder.
3. **Each widget's facets.** `widget: { kind: "keywordOrBlock", ... }` and
   `widget: { kind: "select", ... }` — literal `...` again. What a
   `keywordOrBlock` widget carries is left open.

**Why this is Major, not cosmetic.** It is a latitude clause in a shape the
keyword scan cannot catch: an ellipsis rather than "the implementer may choose".
That is precisely the disguise the ban exists to cover, and
`proc-latitude-clause-boundary` binds design documents as of the 2026-07-15 owner
ruling — a boundary this document itself asserts twice (`:11-19`, `:1426-1435`).
The document's claim "**Every fork in this document is closed**" is not true as
written.

It also **ripples into two other decisions**:

- **The Fluent catalog cannot be authored.** Section 2 (`:1244`) says D45 creates
  "one `labelKey` per registry entry across the edited structs", and the brief
  requires new strings to land bilingual (en+de) **in the same wave**. The key set
  is undefined until the struct set is. `locales/{en,de}/gui-editor.ftl` has no
  determinable contents.
- **D45's own CI gate has no defined target.** `LABEL_KEY_RE` hard-gates label
  keys, which is a real gain — over an undefined key set.

**The document holds itself to the right standard elsewhere and should here.**
D48 enumerates all 17 fields with an explicit rationale (`:1109-1111`):
*"Enumerated rather than described, because 'the defaulted fields' is exactly the
kind of phrase an implementer has to guess at."* "Each edited struct" and
"`text | bool | select | template | locator | ...`" are exactly that kind of
phrase. Apply D48's standard to D45.

**Required change:** enumerate (a) the structs that get a registry, by name;
(b) `FieldWidget`'s complete variant list; (c) each variant's fields. If any of
the three genuinely cannot be settled without an owner call (e.g. whether `Meta`
is editable in v1), that is a NEEDS_CONTEXT escalation with a decision memo, not
an ellipsis.

### MAJOR 2 — every ROADMAP citation in the document is stale, and two of its controller action items are already done

**File:** design doc `:7`, `:48`, `:836-844`, `:1347-1353`, `:1403-1416`.

All **six** distinct ROADMAP citations resolve to the wrong text at HEAD. The
ROADMAP was corrected in commit `fdcdcba` ("roadmap: Plan-6 anchor corrected -
dead premise, unrecountable split, resolved inputs", 2026-07-15 20:16:53); the
design doc's last write is 20:26. The likely cause is a concurrent controller
edit rather than author carelessness — the document's §7 is explicitly addressed
to the controller — but the residue must be fixed, because this document is what
later reviews check against.

| doc cites | doc expects there | actually there now |
|---|---|---|
| `ROADMAP.md:24-42` (`:7`, scope) | the Plan-6 re-cut | Plan 6 section is now `:29-65` |
| `ROADMAP.md:40-42` (`:48`, `:1347`) | the `runActive` re-check input | Plan-6 input **#1** (schema-driven vs UI-model). The `runActive` input is now at **`:59`** |
| `ROADMAP.md:37-38` (`:836-844`, D46) | the trigger "a GUI generating an editor from the schema" | "Named design inputs (6):". The trigger text now sits at **`:53`** |
| `ROADMAP.md:47-48` (`:1355-1358`) | Plan 7 help-mode sequencing | Plan-6 input #3. Plan 7 starts at **`:66`** |
| `ROADMAP.md:18-20`, `:26-28` (`:1403-1410`, item 7) | D22's dead premise asserted as live | the **corrected** D22 text |
| `ROADMAP.md:15-16` (`:1411-1416`, item 8) | "(6 / 5 / 1 / 8)", Plan 6 listing 2 | still says "(6 / 5 / 1 / 8)" — and Plan 6 **now lists 6** |

**Two §7 items are already landed and must not be re-issued:**

- **Item 7** (`:1403-1410`) quotes two ROADMAP passages verbatim. **Neither string exists in the file any more** (`grep "rationale intact"` and `grep "hard design question"` both return nothing). ROADMAP `:19-24` now reads "D22's editor+apply pairing is KEPT, but **not on D22's stated reason**", i.e. exactly the correction the item requests. **Verdict: RESOLVED — strike it.**
- **Item 8** (`:1411-1416`) claims "Plan 6 lists 2, not 6. The arithmetic gives 16, not 20." Plan 6 now lists **6** numbered inputs; Plans 7/8/9 list 5/1/8. The split recounts to 20 and "nothing was dropped" is now verifiable from the document. **Verdict: RESOLVED — strike it.**
- **D46's trigger paragraph** (`:836-844`) argues the recorded trigger "would never have fired" and should be corrected. ROADMAP `:52-55` **already records exactly that**: *"but NOT via the recorded trigger ('a GUI generating an editor from the schema'), which input 1's resolution means would never have fired. The real reason is input 2."* The ADR's reasoning stays valid and worth keeping as the ADR-side record; only its "is recorded as wrong" framing and its `:37-38` citation need re-pointing.

**Why Major rather than Minor:** the staleness is systematic (6/6), and the
failure mode is not a dangling link. `ROADMAP.md:40-42` now resolves to real,
plausible text *about the registry* — a reader following the document's own
citation for "the `runActive` input" lands on the ts-rs decision and has no
signal anything is wrong. A citation that silently misresolves is worse than one
that breaks.

**Required change:** re-point all six citations against HEAD; strike items 7 and
8 as landed (or re-file them as "verified applied in `fdcdcba`"); re-point D46's
trigger paragraph to `:52-55` and reframe it as concurring with a correction
already made.

---

## Minor findings

### MINOR 1 — D48 overstates the silent-data-loss surface: 2 of the 4 divergent fields fail loudly

**File:** `:1106-1108`.

> "The other three divergent fields fail the same way in miniature: `Input.recursive` defaults to `true`, so a generic predicate would omit `recursive: false` and invert it."

**Refuted for two of the three.** `FilenameCfg` and `SourceCfg` have **no `Default` impl at all** — only associated constructors `keep()` and `primary()` (`model.rs:155-161`, `:232-238`; neither derive line includes `Default`). A generic `fn is_default<T: Default + PartialEq>` on `OutputCfg.filename` or `TrackRule.source` therefore **does not compile**. Measured:

```
error[E0277]: the trait bound `FilenameCfg: Default` is not satisfied
error[E0277]: the trait bound `SourceCfg: Default` is not satisfied
```

So the honest split is **two silent hazards, not four**: `TracksCfg.unmatched`
(destroys the `core-83` passthrough — reproduced, and the doc's demonstration of
it is exact) and `Input.recursive` (inverts). The other two are caught by the
compiler and cannot reach data.

This does not change the decision, the table, or any predicate — all correct. It
is flagged because the document's authority rests on its measurements, this
particular sentence is reasoned rather than measured while sitting among measured
ones, and "the four divergent fields are the whole risk" (`:1087`) mis-sizes the
thing the reader is being warned about. Fix: state that two diverge silently and
two are compile-caught for want of a `Default` impl. The compile-time catch is
*good news* and strengthens D48; it costs nothing to say so.

### MINOR 2 — `check-i18n.mjs:172-179` is wrong (twice)

**File:** `:46` and `:731`. The `text.includes(\`"${id}"\`)` mechanism is real and
the argument built on it is sound, but it lives at **`:193`** (loop `:191-198`),
not `:172-179`. Verified against HEAD and against four prior revisions — it
matches **no** revision, so this is not a stale-checkout artifact.

Also: the doc quotes only half the condition. Actual:
`if (text.includes(\`"${id}"\`) || text.includes(\`'${id}'\`))` — single quotes count
too. Narrower in the doc than in reality, which only makes D45's case safer.

### MINOR 3 — `check-i18n.mjs:110-127` is wrong

**File:** `:721`. The "PARSING CONSTRAINT (deliberate, line-based -- not a Fluent
parser)" comment exists verbatim at **`:102-120`**. `:110-127` overlaps only its
tail and spills into `MESSAGE_ID_RE` (`:121`) and `parseCatalogIds` (`:124-127`).

### MINOR 4 — `jobRowState.ts:44-54` is off by one

**File:** `:660`, `:733`. `jobStateKey` spans **`:44-55`** (signature 44, switch
45-54, closing brace 55). Cosmetic, but the doc leans on this file twice as the
house-pattern precedent.

### MINOR 5 — D47's startup-write rejection does not answer the case D47 itself recommends

**File:** `:918-921`.

> "the population it serves is close to empty: a user who never opens a terminal is also not configuring `yaml-language-server` in Neovim, and a user who *is* hand-authoring profiles against an LSP has a shell."

The steelman is strong and fairly stated ("telling a GUI user to open a terminal
is a broken handoff"). The **rejection**, though, answers the Neovim case and
skips the one D47 leads with: D47's own delivery decision (`:880-884`)
recommends **VS Code's `yaml.schemas`**, which a user configures through the
settings UI without ever opening a shell. So "is also not configuring
yaml-language-server" is false for the primary recommended path. The rejection
holds on its **second** argument, which is untouched and sufficient — an
unrequested write into a directory the user did not ask us to touch, on every
startup, for a feature most GUI users never enable. Fix: drop the population
claim or scope it to the non-VS-Code path, and let the unrequested-write argument
carry the rejection.

Applying the believer test as instructed: a JSON-Forms/VS-Code believer would
**not** recognise their case in that sentence. This is the one rejection of the
seventeen where the losing side is answered with an argument that does not reach
it.

### MINOR 6 — required verdict on the document's remaining self-flagged item

**§7 item 9 (`:1417-1422`), the `gui-22` / `exec-44` collision. Verdict: VALID, CONFIRMED, hand to the controller; correctly not touched here.**

Verified: `product-boundaries.yaml:243-252` — `gui-22` states "no automatic
'remove completed jobs after N days' setting in v1; v1 keeps all run logs,
pruning is v1.x", `status: settled`, no supersession marker. `exec-44-runlog-14day-autoprune`
(`:15-23`) records D35 reversing exactly that to an automatic 14-day fixed prune.
A real recorded-statement collision. Out of Plan 6's scope; surfacing rather than
resolving it was the right call. It needs a controller action item, which it does
not currently have.

---

## Dimension-by-dimension

### 1. Requirement compliance — PASS with one gap

Every binding decision in brief §4 is recorded with rationale and rejected
alternatives: §4.1 -> D41; §4.2 -> D44 (ts-rs, committed+CI-checked, `TS_RS_LARGE_INT`)
and D45 (hand-built, registry, `never` arm, check-i18n in the same wave); §4.3 ->
D46 (`schema_with`, constant set, the "do not type the arm" trade recorded) and
D47 (user artifact, README, delivery). The controller's two "strike it if you
disagree" additions are both engaged rather than rubber-stamped: the save-surface
note is **kept and its wording corrected** (the controller's proposed wording
named only comments; the measured behaviour is that formatting and dropped
defaults dominate — a genuine improvement on the brief), and the committed+CI
assumption is **confirmed with a house precedent the controller did not cite**
(`core-06`).

All six brief §5 gaps are closed, each with an independent verification verdict —
including one that **refutes** the brief (the `apply_edit_to_first_rule` hoist).

Deferrals carry observable triggers. The brief warned that it "itself records a
trigger that could never have fired"; the document catches exactly that
(`:836-844`, D46's ROADMAP trigger) and replaces it with an observable one. The
seven triggers in §7 are all observable events (a crate publishes a stable
release; a field gains `#[serde(default)]`; a second artifact needs TS types),
not predictions. **Gap: Major 1** — D45's registry domain is neither closed nor
deferred with a trigger.

### 2. `house` — no deviations found; two surfaced by the author

No unflagged deviation from `conventions.yaml`, `product-boundaries.yaml` or
`process-conventions.yaml` found. Two deliberate deviations are surfaced as the
Tier-2 rule requires, both with reasons that survive checking:

- **Codegen via `cargo test --features ts` rather than xtask** (`:533-540`). The
  reason given is correctness, not taste: xtask would need
  `muxsmith-core = { features = ["ts"] }`, and Cargo unifies features across
  workspace members within one invocation, so `cargo build --workspace` would
  enable `ts` for every consumer of core and put `ts-rs` into the shipped tree.
  That is a real Cargo behaviour and the reasoning is sound. Correctly recorded
  as a deviation rather than slipped in.
- **The CI drift check is a new pattern** (`:549-564`), with the sharp observation
  that `generated.rs` structurally *cannot* have one (its input is never
  redistributed per spec 9.1/`core-06`) while the TS bindings can, because their
  input is in the repo. Verified: no `git diff --exit-code` exists in `ci.yml`
  today.

`ci-10-pin-everything` is correctly scoped (`:511-513`): it binds toolchain and CI
inputs, not Cargo dep syntax, and `=`-pins in this tree really are reserved for
dev-dependencies (`proptest = "=1.11.0"`). Caret `ts-rs = "12.0.1"` matches the
manifest's own pattern. Not a deviation.

**Harvest for the ledger** (dominant patterns and repeated rejections observed):

1. **"Derive the second copy from the first, so the two can never drift"** — now
   at count 3+ and the strongest promotion candidate here. Instances:
   `capability::CODEC_KIND_NAMES` derived from `CODEC_KINDS` with that exact
   rationale in its doc comment (`capability/mod.rs:125-129`); D46's one constant
   set feeding guard + `allowed` + schema; D48's rule that **every predicate calls
   the very function the `default` attribute names**. Same principle, three
   independent sites, one of them already in the tree. Worth a Tier-2 entry.
2. **"Committed generated artifact + CI drift check"** — `core-06` is the
   committed half; D44 adds the drift half. The document names this itself
   (trigger 6) as reaching count 2 toward promotion. Concur.
3. **Repeated rejection: generated/schema-driven UI.** D45 rejects it; the
   supporting survey (14 Rust tools, VS Code settings UI, Zed) all points the same
   way. Candidate restraint: *UI is hand-built; the schema describes the file, not
   the form.* Note the document is honest that Zed's rejected approach was
   macro-based and therefore not a direct refutation — do not let that honesty get
   flattened on promotion.
4. **Repeated rejection: option surface not earned.** `exec-45-runlog-config-deferred`
   reused as precedent in D48's `--canonical`/`--minimal` rejection. Count 2 —
   candidate for a general Tier-2 restraint.
5. **Repeated rejection: network/hosted dependency.** `core-07-runtime-fetching-rejected`
   reinforced by D47's SchemaStore rejection.
6. **Repeated rejection: RC / pre-1.0 dependency** on the pin-everything doctrine
   (`tauri-specta`). Consistent with `ci-10`.
7. **New, worth recording:** *"Touches the disk" is not the `on_blocking`
   criterion; "could stall the webview" is* (D42, `:334-339`). It is the rule the
   shell already follows in practice (`get_settings`/`set_settings` are non-async
   despite real file I/O) but nowhere states. Verified against the tree.

### 3. Latitude — FAIL (Major 1)

Keyword scan over the full document for "may choose", "either approach",
"if a simpler alternative", "at the implementer's discretion", "TBD", "as
appropriate", "whichever", "optionally", "alternatively", "or similar" returns
**only the document's own meta-statements** about the ban (`:12-13`, `:1430-1431`).
On the letter of the ban, it is clean, and §8's pre-decided list (`:1437-1465`) is
genuinely good — it closes the forks an implementer would actually improvise on
(writer location, format selection, `DiagCode` deserialization, xtask-vs-test,
`enum`-vs-`oneOf`, tree-walk-vs-derives, guard placement).

It fails on the **disguise**: three unenumerated sets in D45, marked with literal
`...` (Major 1). An ellipsis in a normative type definition is a latitude clause
that no keyword scan catches.

### 4. Steelman honesty — PASS on 16 of 17

I re-applied the believer test to all seventeen rejected alternatives
independently, without regard to which three the author says it strengthened.
These are better than the norm. Four are strong enough to note specifically:

- **D41 / `yamlpath`+`yamlpatch`** (`:167-181`): names it as "the research
  corpus's own primary recommendation", reports that it *worked* (12 comments in,
  12 out, all seven ops valid), and concedes outright: "On the narrow question
  'can apply-suggestion be lossless', the answer is yes, and it was demonstrated."
  Then rejects it on a **conditional the corpus itself wrote** (§6: reconsider if
  the surface becomes "restructure the rule tree" — which spec 8.2's drag-reorder
  grid is). That is the research's own gate resolving against the option, not an
  override of it. A believer would recognise this and concede the gate.
- **D48 / emit defaults** (`:1013-1037`): the strongest steelman in the document.
  It grants that emission is "safer, simpler, and better documentation", costs
  "zero new code, zero predicates, zero attributes and zero guard tests", "cannot
  be got wrong", and avoids the `extend` drift surface entirely — then loses on one
  specific interaction with D47. It closes: "anyone reopening this argues against
  the reason it lost, not against a caricature." That is the record working as
  designed.
- **D46 / type the arm** (`:770-777`): concedes the alternative is "strictly
  simpler than this decision and more consistent with the two real enums sitting
  beside it", and that the reviewer who flagged it "was reasonable". Then gives
  the asymmetry that actually decides it (`KeepDrop` can afford typing because it
  is not *inside* an untagged enum; the `Keyword` arms cannot). Verified.
- **D44 / `tauri-specta`** (`:566-582`): concedes the loser does the *larger* half
  of the job ("it would type the **commands**, not just the model — which is the
  larger share of the boundary this plan is adding"). Conceding that the rejected
  option is better at the bigger thing is the test passing.

Also credited: D45 concedes a generator would be "strictly stronger than a
registry"; D48's tree-walker rejection concedes the walker "would have avoided the
`extend` annotation entirely"; D42's temp-file rejection concedes "zero new IPC
surface" and a real fidelity argument; the ecosystem evidence is explicitly
downgraded to "suggestive, not decisive" with Zed's non-applicability volunteered
(`:710-716`).

**The one failure is Minor 5** (D47's startup write): the steelman is strong but
the rejection's first argument does not reach the case D47 itself recommends.

### 5. Spec self-consistency — PASS

Four amendments proposed (`:1250-1291`). Swept for the named failure class (4.9
contradicting 4.5) and for new contradictions:

- **8.2** (canonical save, no comments/key order/formatting, defaults not written
  back): consistent. Spec 8.2 currently says only "open/save YAML" — no conflict.
- **8.1** (`muxsmith schema` a supported feature): consistent.
- **8.4** (schema `description` fields as an accepted exception): **necessary and
  correct.** Verified that spec 8.4's accepted-exceptions list (clap's help text,
  third-party `detail` text, `IdentifyError`'s `Display`) does **not** cover
  schema descriptions, so without this amendment the schema is a standing 8.4
  violation the moment D47 promotes it. Good catch. The D45/D47 tension (reject
  English doc-comment labels, accept English doc-comment descriptions) is
  pre-empted with the right distinction — application UI vs file-format
  documentation — and is sound.
- **4.8 / 4.9 no change**: verified correct. Spec 4.9 states the domains
  normatively in prose (`chapters: keep | drop`, `title: keep | clear | template`,
  `tags.*: keep | drop`) and D46 makes the schema agree with text that is already
  right.
- Spec 4.1's reference example (flow-style, omits `source`/`optional`): the
  document's sweep is correct that D48's "omit" ruling keeps it consistent, and it
  correctly separates the surviving formatting difference as non-contradictory
  (4.1 illustrates the format, it does not specify editor output).

The 4.9-vs-4.5 asymmetry is stated in one place and cross-referenced, not
duplicated — and D48 depends on that asymmetry rather than contradicting it. No
new contradiction introduced. `core-83` remains satisfied under D48: `unmatched:
keep` is non-default, so it is always written, and `validate.rs` still emits its
info diagnostic.

---

## What must change

| # | Sev | Site | Change |
|---|---|---|---|
| 1 | **Major** | `:614-641`, `:1244`, `:1449-1451` | Enumerate the structs that get a registry, `FieldWidget`'s full variant list, and each variant's facets. Delete all three `...`. Escalate as NEEDS_CONTEXT anything that needs an owner call. Then reconcile §2's catalog table with the enumerated key set. |
| 2 | **Major** | `:7`, `:48`, `:836-844`, `:1347-1353`, `:1403-1416` | Re-point all six ROADMAP citations against HEAD. Strike §7 items 7 and 8 as landed in `fdcdcba`. Re-point D46's trigger paragraph to `ROADMAP.md:52-55` and reframe as concurring with a correction already made. |
| 3 | Minor | `:1087`, `:1106-1108` | State the split honestly: 2 fields diverge silently (`TracksCfg.unmatched`, `Input.recursive`), 2 are compile-caught (`FilenameCfg`/`SourceCfg` have no `Default` impl -> `E0277`). |
| 4 | Minor | `:46`, `:731` | `check-i18n.mjs:172-179` -> `:191-198` (the `includes` call is `:193`). Optionally note the `'${id}'` half of the condition. |
| 5 | Minor | `:721` | `check-i18n.mjs:110-127` -> `:102-120`. |
| 6 | Minor | `:660`, `:733` | `jobRowState.ts:44-54` -> `:44-55`. |
| 7 | Minor | `:918-921` | Drop or scope the "population is close to empty" claim; it does not reach the VS Code path D47 recommends. Let the unrequested-write argument carry the rejection. |
| 8 | Minor | `:1417-1422` | §7 item 9 (`gui-22`/`exec-44`) confirmed valid — needs a controller action item, which it currently lacks. |

**Not required, offered:** the subagent verification found `LABEL_KEY_RE` would
also match inside comments (`// labelKey: "x"` becomes a hard CI failure) and
lacks `CALL_RE`'s `(?<![\w$])` guard. Both are defensible as-is — the regex was
tested against the real `src/` tree with 0 false positives across 17 files, and
the repo has no prettier to wrap the registry lines — so this is a note, not a
finding.

---

## Assessment

The engineering is excellent and the verification culture is real: eight brief
premises checked and refuted with evidence, the `yaml_serde`/tree-sitter error
traced back through the corpus to its §1 source, the comment-count decoration
retracted ("the hazard needs no exaggeration"), the self-invalidating third
argument against schema-driven forms *deliberately not recorded* because D46
would kill it in the same plan. That last one is the sharpest judgement in the
document. Every number I re-ran was right, including three I expected to drift.

The two Major findings are narrow. Major 2 is bookkeeping against a file that
moved under the author. Major 1 is the real one: the single ADR whose job is to
force completeness is the one that does not enumerate its own domain — and it
sits directly beside D48, which enumerates 17 fields and says in as many words
why enumeration beats description. Apply D48's own sentence to D45 and the
document is approvable.
