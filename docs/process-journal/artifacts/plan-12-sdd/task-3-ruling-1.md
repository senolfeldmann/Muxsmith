# Controller ruling 1 on Task 3 (Plan 12)

**Order of authority.** This file sits directly below the plan and above the
task brief for the one deviation it authorises, and touches nothing else. Where
it is silent, the brief and the plan govern unchanged.

## The fork

Step 3's fenced `doSave` body does not type-check, so the plan's fenced text and
the plan's own gate constraint cannot both be satisfied.

**Reproduced by the controller** on the tree the implementer left in its
deliberate unresolved state (`pnpm build`, exit **2**, exit code captured
directly rather than through a pipeline):

```
src/views/EditorView.vue(354,23): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
src/views/EditorView.vue(359,53): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
```

**Cause**, as the implementer diagnosed it and as the errors confirm:
`const needsPath = path === null` gives no narrowing, because TypeScript's
aliased-condition analysis requires the aliased variable to be a `const` or an
un-reassigned `let`, and `path` is reassigned in the dialog branch. `path`
therefore remains `string | null` at `saveProfile(path, profile)` and at
`rememberRecentProfile(path)`.

## Routing: the controller's, not the owner's

Named so this is not read as a decision taken by default. Nothing here is
product-visible: the observable behaviour is identical under every option below,
no string, no wire format and no user-facing surface moves. The colliding
statements are both inside the plan, not spec against ADR, and the plan's own
precedence already ranks them - a fenced block that cannot build cannot satisfy
"the gate as `BUILDING.md` enumerates it, green, before any push". Escalating a
TypeScript narrowing detail to the owner would cost him a decision he has no
information advantage on.

## Decision: option A, one token

**Inside the `try`, the FIRST guard becomes `if (path === null)` instead of
`if (needsPath)`.** Everything else in the fenced block stays byte-identical,
including `const needsPath = path === null;` itself and the SECOND
`if (needsPath)` that gates the recents write.

**Why the behaviour is provably identical**, which is what makes this a
formatting-level deviation rather than a design change: between
`const needsPath = path === null;` and the first guard, the fenced block writes
only `saving.value` and `ipcErrorCode.value`. `path` is not written, so the two
conditions are equal at that point by construction, not by measurement.

**Why `needsPath` survives rather than being deleted:** D107 decision 5 gates
the recents write on the path having been NEWLY established, and after the
dialog branch runs, `path === null` is false - so the second site must keep
asking the original question. The constant is what carries that question across
the branch, which is exactly why the plan introduced it.

**One comment is owed at the site**, because the two conditions now read as
redundant side by side and a later simplifier would unify them back and
re-break the build. State the narrowing reason in one line, by symbol, not by
line number.

**Rejected: two `as string` casts.** Steelman, at its strongest: the block then
stays byte-identical to the plan, which is the property the fenced form exists
to protect, and the casts are provably safe because the branch above establishes
the value. Rejected because a cast suppresses the checker where the guard
satisfies it, and it would leave two assertions in the file that a future change
to the branch structure could silently falsify - the checker would no longer be
watching.

**Rejected: a second `const` after the branch.** Steelman: it narrows without
touching either existing condition. Rejected as a new name for a value that
already has one, in a block whose whole point is a discipline about which value
is used where.

## No plan amendment is owed

The fenced block's purpose is the capture-before-the-await discipline and the
branch structure, and both are preserved intact. A four-eyes amendment to change
one token would spend an authoring round and a review round making a document
match code that its own reviewer will already be grading against this ruling.
The reviewer reads both, and the deviation is one line, named here.

## Also surfaced by the implementer, dispositions

- **The brief's Step 1 says "the four candidate seeds"; the authoring section it
  points at enumerates FIVE and D107 reasons about five.** The implementer ran
  all five - a superset, which cannot under-satisfy - and all five reproduce the
  authoring output on both instruments including exit codes. Correct handling.
  Recorded as a count-against-its-own-enumeration instance.
- **The model-tier table sizes this task at "seven new tests"; Step 7 enumerates
  SIX.** The enumeration governs; six is right. Same class as the item above.
- **Step 6's placement clause ("append after its existing generic-action
  section") is under-determined now that a rule-grid-ordinal section sits after
  that one.** Exactly one placement satisfies both clauses here, so reading it
  was correct. **Tasks 4 and 5 append to the same catalogs under the same
  wording with yet another section in between, so their dispatches carry an
  explicit placement instead of inheriting this ambiguity.** Carried as a
  cross-task constraint.
