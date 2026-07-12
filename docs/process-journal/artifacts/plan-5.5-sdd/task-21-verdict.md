# Task 21 reviewer verdict (model: opus, 2026-07-12)

Diff: b094ff9..1262c14, fix c229400 on plan55-t21

## Round 1: Needs fixes (test-only)
German content + terminology anchoring APPROVED: zero objective German
errors (orthography/grammar/placeholders/plural variants machine- and
hand-checked); zero sentence overlap with mkvtoolnix de.po (12 distinctive
sentences grepped; anchor terms genuinely present); loader S15 logic
correct; e2e asserts real German with BiDi-stripping; nested-selector
indentation reproduced clean under the real parser. TWO Important
test-coverage gaps: S15 normalization exercised by nothing (e2e pinned
"de"); nested-selector real-parse guard existed only as a throwaway
script (buildBundle console.warns; check-i18n line-regex - T20's recorded
gap).

## Fix wave (c229400), re-review APPROVED
1. de-AT e2e flip: statically verified causal (without primarySubtag the
   chain is [de-AT,en], buildBundle(de-AT) null, first assertion fails on
   en "Batch"); other assertions intact.
2. All-locales message-presence cross-check - the fixer CORRECTED the
   reviewer's own suggested mechanism (addResource errors ONLY on id
   collisions; Junk silently dropped - verified from library source and
   reproduced): column-0 scan ids asserted present in the real parser's
   output, per locale, in the app's two runtime groupings (cli.ftl
   standalone - it legitimately shares 3 ids with gui-common). Injected
   unbalanced-brace regression caught. Reviewer concedes cleanly.
3. German content byte-identical (blob hashes compared); terminology
   surface for Şenol unchanged.

## Residual (recorded)
- Presence, not formatting: placeable-name drift in de would pass the
  guard (parity holds today, machine-verified); future check-i18n
  extension candidate.
- Şenol gates before merge: terminology table (report §1), register mix;
  dropdown + stale settings-locale-hint recommended as immediate
  pre-merge follow-up; native close dialog deferred (shell i18n).

## Assessment
Spec compliance ✅ (technical). Task quality: Approved - merge blocked
only on the owner's terminology gate.
