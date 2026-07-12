### Task 1: `.gitattributes` + renormalization (#15)

**Files:**
- Create: `.gitattributes`
- Modify: none (renormalization commit touches whatever git rewrites)

**Interfaces:** none consumed; produces LF-stable text files for T22's byte-exact snapshots.

- [ ] Step 1: Create `.gitattributes`:

```gitattributes
* text=auto eol=lf
# binary test assets must never be normalized; extend as assets appear
*.png -text
*.ico -text
*.icns -text
```

Then `grep -rIL . --exclude-dir={target,node_modules,.git,dist} | head` and `git ls-files | xargs file | grep -v text` to find any further binary assets; add each as `-text`.
- [ ] Step 2: Commit `.gitattributes` alone: `chore: normalize line endings via .gitattributes (#15)`.
- [ ] Step 3: `git add --renormalize .`; inspect `git status` - expected: few or zero files (repo was authored on Linux). Commit separately if non-empty: `chore: renormalize tracked files to LF`.
- [ ] Step 4: Full gate. Commit nothing else in this task.

