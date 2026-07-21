# On collision (output)

This policy governs what happens when a rendered output path already exists on disk as a pre-existing file. The three values behave materially differently:

- **`error`** (the default): the affected file gets no plan and the collision is reported as an error. Nothing is overwritten and nothing is silently skipped.
- **`skip`**: the affected output is omitted with a warning; the rest of the batch proceeds normally.
- **`overwrite`**: the plan is kept and the existing file is replaced when the batch runs; reported as info.

What this policy does *not* govern:

- An output path equal to any input path - primary or donor - is always a hard error (`SourceOverwrite`), under every policy. Muxsmith never overwrites its own sources.
- Two planned outputs of the same batch rendering to the same path is always an error (`OutputCollision`) regardless of policy: neither `skip` nor `overwrite` could define which of the two plans wins. Fix the naming instead - disambiguate the filename template or the input pattern.

When to use which:

- Keep `error` while a profile is still settling: it surfaces naming mistakes instead of acting on them.
- Use `skip` for incremental re-runs over a growing directory: episodes already muxed in an earlier run stay untouched, only new files produce output.
- Use `overwrite` when you deliberately regenerate a batch after a profile change and the existing outputs are stale by definition.

A dry run shows the collision diagnostics without touching any file, so you can check what a policy would do before running.
