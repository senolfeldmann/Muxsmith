/**
 * Diagnostic params that must reach Fluent as a number, not a string (T19,
 * #17 step 1): `Diagnostic.params` mirrors the Rust wire type
 * (`BTreeMap<String, String>`, spec 5.2/8.4 -- core stays prose- and
 * type-free), so a diagnostic code whose `diagnostics.ftl` message uses a
 * CLDR plural selector (`[one]`/`*[other]`) on one of its own params needs
 * that value promoted back to a number here, at the render boundary --
 * `@fluent/bundle`'s resolver only resolves a plural selector against a
 * JS `number`, a `string` argument always falls through to `*[other]`.
 * Mirrors `muxsmith_cli::i18n::numeric_diagnostic_params` on the Rust
 * side, same two codes and param names; keep both lists in lockstep if
 * either changes.
 */
const NUMERIC_DIAGNOSTIC_PARAMS: Record<string, readonly string[]> = {
  "suggestions-capped": ["dropped"],
  "suggestion-partition": ["dropped", "count"],
};

/**
 * Fluent params for one diagnostic `(code, params)`, promoting every name
 * [`NUMERIC_DIAGNOSTIC_PARAMS`] lists for `code` from its wire string form
 * to a real number. A listed value that does not parse as a non-negative
 * integer is left as a string rather than dropped, so it degrades to the
 * selector's `*[other]`/`*[group]` branch instead of leaking `{$name}`.
 * Mirrors the Rust side's `parse::<usize>()` strictness: rejects negative
 * numbers, floats, empty strings, and scientific notation.
 */
export function diagnosticFluentParams(
  code: string,
  params: Record<string, string>,
): Record<string, string | number> {
  const numericKeys = NUMERIC_DIAGNOSTIC_PARAMS[code];
  if (!numericKeys) {
    return params;
  }
  const result: Record<string, string | number> = { ...params };
  for (const key of numericKeys) {
    const raw = params[key];
    if (raw === undefined) {
      continue;
    }
    // Reject empty / whitespace-only strings explicitly since Number("") === 0.
    if (raw.trim() === "") {
      continue;
    }
    const n = Number(raw);
    // Only accept non-negative integers (usize parity with Rust side).
    if (Number.isInteger(n) && n >= 0) {
      result[key] = n;
    }
  }
  return result;
}
