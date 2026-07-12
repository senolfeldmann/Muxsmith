### Task 20: check:i18n locale parity (#17 step 2)

**Files:**
- Modify: `scripts/check-i18n.mjs` (enforce key parity across all `locales/*` dirs; keep the documented cli.ftl exclusion decision OR extend to cli.ftl now that T10 guards it - implementer reads the script header note and decides WITH the reviewer, recording the choice)
- Test: the script IS the test (gate part); add a fixture-based self-test if the script grows logic.

- [ ] Commit `i18n: check:i18n enforces cross-locale key parity (#17)`.

