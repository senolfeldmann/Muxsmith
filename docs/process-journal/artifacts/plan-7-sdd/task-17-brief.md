### Task 17: D55's check-i18n extensions - attributes, `$ta`, tooltip completeness, parity

**Stream G.** Read D55's "check-i18n extensions" block (rules 1-5) and the script's own header charter (line-based, not a Fluent parser).

**Files:**
- Modify: `scripts/check-i18n.mjs`

**Interfaces:**
- Consumes: the migrated catalogs (Tasks 3/5), the `$ta` call sites (Tasks 4/5/12).
- Produces: `parseCatalog(path)` (Task 18 reuses the scan structure; Task 19 extends the same file).

The five rules (all hard-fail except where stated; the parser stays line-based per its charter):

1. `parseCatalogIds` becomes `parseCatalog`, returning per file: message ids, per-id attribute-name sets, and per-id/per-attribute pattern bodies (the id or attribute line's value plus its indented continuation lines; an indented `.name =` line starts an attribute of the current id). Terms stay unregistered.
2. Literal `$ta("id")`/`ta("id")` calls scanned with the `CALL_RE` mechanics; the id must exist - hard fail. Attribute member access after `$ta(...)` is not statically resolved (same skip posture as dynamic `$t` keys).
3. Editor tooltip completeness: every id collected by `LABEL_KEY_RE` must carry a `tooltip` attribute in the en catalog - hard fail naming the id.
4. Check-3 extension: for every id shared across locales, the attribute-NAME set must equal en's - hard fail on missing and extra alike.
5. Placeable-set and selector-structure parity per pattern (message values and attribute values, against en): (a) the set of `$name` placeable references (variants included) must be equal; (b) the number of select expressions equal; (c) each select's selector variable equal; (d) variant-key sets equal, EXCEPT keys that are CLDR plural categories (`zero`, `one`, `two`, `few`, `many`, `other` - exactly this list) or numeric literals (`[0]`, `[1]`, ...), where the rule is instead: at least one variant, exactly one `*`-default. Hard fail. Rules 4-5 run wherever check 3 runs (ALL `.ftl` including `cli.ftl`).

- [ ] **Step 1: Restructure the parser.**

```js
/** One catalog file, line-parsed (charter above): message ids, each id's
 *  attribute-name -> pattern body, and the id's own value body. */
function parseCatalog(path) {
  const messages = new Map(); // id -> { value: string, attrs: Map<name, body> }
  const text = readFileSync(path, "utf8");
  let current = null;
  let target = null; // { kind: "value" } | { kind: "attr", name }
  for (const line of text.split("\n")) {
    const idMatch = MESSAGE_ID_RE.exec(line);
    if (idMatch) {
      current = { value: line.slice(line.indexOf("=") + 1), attrs: new Map() };
      messages.set(idMatch[1], current);
      target = { kind: "value" };
      continue;
    }
    if (current === null) continue;
    const attrMatch = /^\s+\.([a-z][a-z0-9-]*)\s*=/.exec(line);
    if (attrMatch) {
      current.attrs.set(attrMatch[1], line.slice(line.indexOf("=") + 1));
      target = { kind: "attr", name: attrMatch[1] };
      continue;
    }
    if (/^\s+\S/.test(line)) {
      // continuation line of the current value or attribute
      if (target.kind === "value") current.value += "\n" + line;
      else current.attrs.set(target.name, current.attrs.get(target.name) + "\n" + line);
    }
  }
  return messages;
}

function parseCatalogIds(path) {
  return [...parseCatalog(path).keys()];
}
```

(keeping `parseCatalogIds` as a one-line wrapper preserves every existing call site; the header comment's ATTRIBUTES bullet is updated in Step 5.)

- [ ] **Step 2: Rule 2 - the `$ta` scan.** Beside `CALL_RE`:

```js
const TA_CALL_RE = /(?<![\w$])\$?ta\(\s*(['"])([^'"]*)\1/g;
```

and in the per-line loop, the same treatment as `CALL_RE` (add to `literalCallIds`, push to `missing` when unknown).

- [ ] **Step 3: Rules 3-5.** Placed in/after the check-3 region, where `referenceCatalogFiles` and the locale loop already exist (rule 3 uses the labelKey ids from check 1 - collect them into their own `labelKeyIds` set in the existing `LABEL_KEY_RE` branch):

```js
// --- D55 rule 3: every registry label carries a .tooltip in en ---------
const enCatalogs = new Map(
  referenceCatalogFiles.map((f) => [f, parseCatalog(join(LOCALES_EN, f))]),
);
const tooltipErrors = [];
for (const id of [...labelKeyIds].sort()) {
  const hasTooltip = [...enCatalogs.values()].some(
    (msgs) => msgs.get(id)?.attrs.has("tooltip"),
  );
  if (!hasTooltip) {
    tooltipErrors.push(`labelKey "${id}" has no .tooltip attribute in the en catalog`);
  }
}

// --- D55 rules 4+5: attribute-name and pattern-structure parity --------
const PLURAL_KEYS = new Set(["zero", "one", "two", "few", "many", "other"]);
const PLACEABLE_RE = /\$([A-Za-z][A-Za-z0-9_-]*)/g;
const SELECTOR_RE = /\{\s*\$([A-Za-z][A-Za-z0-9_-]*)\s*->/g;
const VARIANT_RE = /^\s*(\*)?\[([^\]]+)\]/;

function patternStructure(body) {
  const placeables = new Set([...body.matchAll(PLACEABLE_RE)].map((m) => m[1]));
  // Variants belong to the most recent selector above them (line order);
  // this tree's catalogs have no nested selects (line-based charter).
  const selects = [];
  for (const line of body.split("\n")) {
    for (const m of line.matchAll(SELECTOR_RE)) {
      selects.push({ selector: m[1], keys: [], defaults: 0 });
    }
    const v = VARIANT_RE.exec(line);
    if (v && selects.length > 0) {
      const current = selects[selects.length - 1];
      current.keys.push(v[2].trim());
      if (v[1] === "*") current.defaults += 1;
    }
  }
  return { placeables, selects };
}

function comparePatterns(where, enBody, locBody, errors) {
  const a = patternStructure(enBody);
  const b = patternStructure(locBody);
  if ([...a.placeables].sort().join() !== [...b.placeables].sort().join()) {
    errors.push(`${where}: placeable set differs from en ({${[...a.placeables]}} vs {${[...b.placeables]}})`);
  }
  if (a.selects.length !== b.selects.length) {
    errors.push(`${where}: select-expression count differs from en (${a.selects.length} vs ${b.selects.length})`);
    return;
  }
  a.selects.forEach((sa, i) => {
    const sb = b.selects[i];
    if (sa.selector !== sb.selector) {
      errors.push(`${where}: select ${i} selector differs ($${sa.selector} vs $${sb.selector})`);
    }
    const plural = (k) => PLURAL_KEYS.has(k) || /^\d+$/.test(k);
    if (sa.keys.every(plural) && sb.keys.every(plural)) {
      // CLDR carve-out: category sets legitimately differ per locale;
      // require at least one variant and exactly one *-default.
      if (sb.keys.length === 0 || sb.defaults !== 1) {
        errors.push(`${where}: select ${i} needs >=1 variant and exactly one *-default`);
      }
    } else if (sa.keys.slice().sort().join() !== sb.keys.slice().sort().join()) {
      errors.push(`${where}: select ${i} variant keys differ from en ([${sa.keys}] vs [${sb.keys}])`);
    }
  });
}
```

Wire both into the check-3 locale loop: for each shared file and each shared id, compare attribute-name sets (rule 4) and run `comparePatterns` on the value and on each shared attribute (rule 5), pushing into `parityErrors` (they ride check 3's existing hard-fail exit).

- [ ] **Step 4: Fire-verify every rule, one deliberate break each** (record each red run in the task report, then restore):

```bash
# rule 2: add $ta("no-such-id") to any src file        -> exit 1 naming it
# rule 3: comment out one .tooltip in en gui-editor    -> exit 1 naming the labelKey
# rule 4: delete one .tooltip in de gui-editor only    -> exit 1 (attr-set parity)
# rule 5a: rename { $detail } to { $detailx } in one de message -> exit 1
# rule 5d: remove the *[other] default in a de plural  -> exit 1
pnpm check:i18n   # after each break; then restore and confirm green
```

- [ ] **Step 5: Update the header comment**: the ATTRIBUTES bullet (":118-123") now describes the implemented extension (attributes registered per id, addressed by rules 2-5), and check 3's description gains rules 4-5. Delete the stale "no catalog has attributes today" sentences.

- [ ] **Step 6: Full gate** (`pnpm check:i18n` first, then the nine parts). Expected: green against the real tree - if rule 5 finds a genuine pre-existing en/de structural drift, that is a real catalog defect: fix the de catalog in this task (bilingual discipline) and note it in the task report.

- [ ] **Step 7: Commit**

```bash
git add scripts/check-i18n.mjs
git -c commit.gpgsign=false commit -m "check-i18n: attribute-aware catalog model, \$ta scan, tooltip completeness, attribute + placeable/selector parity (D55 rules 1-5, i18n-12)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

