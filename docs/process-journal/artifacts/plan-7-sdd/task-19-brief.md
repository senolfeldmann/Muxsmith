### Task 19: D62 - the help-topic gate

**Stream G**, after Task 18. Read D62 in full.

**Files:**
- Modify: `scripts/check-i18n.mjs`

**Interfaces:** consumes the `help/` tree (Tasks 8-10), the annotation literals (Task 13), `VIEW_TOPICS` (Task 12).

- [ ] **Step 1: Implement the four hard-fail checks.**

```js
// --- D62: help-topic completeness, both directions, per locale ----------
const HELP_ROOT = join(ROOT, "help");
const HELP_ID_PROP_RE = /helpId:\s*(['"])([^'"]*)\1/g;      // (a) registry literals
const DATA_HELP_ID_RE = /data-help-id="([^"]+)"/g;           // (b) template literals
const VIEW_TOPIC_RE = /['"](view-[a-z-]+)['"]/g;             // (c) VIEW_TOPICS values

const referencedHelpIds = new Map(); // id -> "file:line" (first reference)
for (const [file, text] of fileTexts) {
  text.split("\n").forEach((line, i) => {
    for (const re of [HELP_ID_PROP_RE, DATA_HELP_ID_RE]) {
      for (const m of line.matchAll(re)) {
        const id = m[2] ?? m[1];
        if (!referencedHelpIds.has(id)) {
          referencedHelpIds.set(id, `${relative(ROOT, file)}:${i + 1}`);
        }
      }
    }
  });
}
// (c) is deliberately redundant with (b) for the three view ids: a view
// root losing its data-help-id, or the map growing an id without a topic,
// both still fail. Shape (a) cannot see them (no `helpId:` property name).
const stateText = readFileSync(join(SRC, "help", "state.ts"), "utf8");
for (const m of stateText.matchAll(VIEW_TOPIC_RE)) {
  if (!referencedHelpIds.has(m[1])) {
    referencedHelpIds.set(m[1], "src/help/state.ts (VIEW_TOPICS)");
  }
}

const helpErrors = [];
const helpLocales = readdirSync(HELP_ROOT, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)
  .sort();
const catalogLocales = readdirSync(LOCALES_ROOT, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)
  .sort();

// 1. referenced -> file, per locale
for (const [id, site] of [...referencedHelpIds].sort()) {
  for (const locale of helpLocales) {
    try {
      readFileSync(join(HELP_ROOT, locale, `${id}.md`));
    } catch {
      helpErrors.push(`help id "${id}" (referenced at ${site}) has no help/${locale}/${id}.md`);
    }
  }
}
// 2. file -> referenced (orphans)
for (const locale of helpLocales) {
  for (const f of readdirSync(join(HELP_ROOT, locale)).filter((f) => f.endsWith(".md"))) {
    const id = f.slice(0, -3);
    if (!referencedHelpIds.has(id)) {
      helpErrors.push(`help/${locale}/${f}: orphan topic (no helpId/data-help-id/VIEW_TOPICS reference)`);
    }
  }
}
// 3. locale-set lockstep with locales/
if (helpLocales.join() !== catalogLocales.join()) {
  helpErrors.push(`help/ locales [${helpLocales}] != locales/ [${catalogLocales}] (lockstep, D62)`);
}
// 4. external-URL ban (help is self-contained by design; cross-topic
// references are prose, never links)
for (const locale of helpLocales) {
  for (const f of readdirSync(join(HELP_ROOT, locale)).filter((f) => f.endsWith(".md"))) {
    const text = readFileSync(join(HELP_ROOT, locale, f), "utf8");
    if (/https?:\/\//.test(text)) {
      helpErrors.push(`help/${locale}/${f}: contains an external URL (banned, D62 check 4)`);
    }
  }
}
```

`helpErrors` joins the hard-fail exit; the ok-line gains `${referencedHelpIds.size} help id(s) x ${helpLocales.length} locale(s)` (expected 22 x 2). Widen the script's header comment to name the topic-tree scope (D62's rejected-alternative rationale: one gate, one place where "i18n-complete" is defined).

- [ ] **Step 2: Fire-verify all four**, one break each, restore each: (1) `data-help-id="plan7-bogus"` in a view -> red naming file:line; (2) `help/en/orphan.md` -> red; (3) `mkdir help/fr` with one file -> red lockstep (also red orphan - fine); (4) `https://example.com` in a topic -> red. Then green.

- [ ] **Step 3: Full gate, then commit**

```bash
git add scripts/check-i18n.mjs
git -c commit.gpgsign=false commit -m "check-i18n: help-topic gate - referenced<->file both directions, locale lockstep, external-URL ban (D62)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

