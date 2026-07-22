### Task 18: D61's presence gate - IpcError codes against the GUI catalogs

**Stream G**, after Task 17. Read D61's presence-gate block.

**Files:**
- Modify: `scripts/check-i18n.mjs`

**Interfaces:** consumes `knownIds` and Task 17's structure; produces the extracted `ipcErrorCodes` set (fed into check 2's `usedIds`).

- [ ] **Step 1: Implement the scan + gate.**

```js
// --- D61: every IpcError::new("code") in src-tauri has a GUI message ----
// Line-based Rust scan, taking each file's content up to its first
// `#[cfg(test)]` line (test modules sit at file bottoms in this tree).
const SRC_TAURI = join(ROOT, "src-tauri", "src");
const IPC_ERROR_RE = /IpcError::new\(\s*"([A-Za-z][A-Za-z0-9_-]*)"/g;
const ipcErrorCodes = new Map(); // code -> "file:line"
for (const f of readdirSync(SRC_TAURI, { recursive: true }).filter((f) => f.endsWith(".rs"))) {
  const full = join(SRC_TAURI, f);
  const text = readFileSync(full, "utf8");
  const cut = text.indexOf("#[cfg(test)]");
  const scanned = cut === -1 ? text : text.slice(0, cut);
  scanned.split("\n").forEach((line, i) => {
    for (const m of line.matchAll(IPC_ERROR_RE)) {
      if (!ipcErrorCodes.has(m[1])) {
        ipcErrorCodes.set(m[1], `${relative(ROOT, full)}:${i + 1}`);
      }
    }
  });
}
const ipcErrors = [];
for (const [code, site] of [...ipcErrorCodes].sort()) {
  if (!knownIds.has(code)) {
    ipcErrors.push(`IpcError code "${code}" (${site}) has no message in the en GUI catalogs`);
  }
}
```

`ipcErrors` joins the hard-fail exit; `...ipcErrorCodes.keys()` joins the `usedIds` union - which closes the documented check-2 false-positive residual.

- [ ] **Step 2: Update the residual comment** (`:42-50` region): the "Known residual false positive" prose describes a gap this check removes - replace it with a sentence recording that shell IpcError codes are now extracted from `src-tauri/src` and both gated (presence, hard) and counted as used (check 2).

- [ ] **Step 3: Fire-verify**: temporarily add `IpcError::new("plan7-bogus-code")` to a non-test line of `src-tauri/src/error.rs`, run `pnpm check:i18n`, confirm exit 1 naming the Rust site; restore; confirm green. Also confirm the 19-code count in the ok-line output (extend the final `console.log` with `${ipcErrorCodes.size} IpcError code(s) gated`; expected 19).

- [ ] **Step 4: Full gate, then commit**

```bash
git add scripts/check-i18n.mjs
git -c commit.gpgsign=false commit -m "check-i18n: hard presence gate for src-tauri IpcError codes; codes feed check 2, residual comment retired (D61)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

