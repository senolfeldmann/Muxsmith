### Task 19: Fluent plural selectors (#17 step 1)

**Files:**
- Modify: `locales/en/cli.ftl:2` (`validate-summary`), `locales/en/gui-batch.ftl:31` (`batch-diagnostics-summary`), any `(s)`-pattern key added by Waves 1-2 (grep `(s)` across locales/en/)
- Test: renderer tests asserting singular/plural forms

- [ ] Failing test: render summary with 1/2 errors, assert "1 error" / "2 errors" (no parenthesis form). Convert to Fluent selectors (`{ $errors -> [one] ... *[other] ... }`). Full gate; commit `i18n: plural selectors replace the error(s) provisional (#17)`.

