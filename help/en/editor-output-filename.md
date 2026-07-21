# Filename (output)

This control decides what each output file is called. Two modes:

- **`keep`**: the output keeps the source file's basename; the extension is enforced to `.mkv`.
- **Template**: a literal-mode template renders the name from the file's identifier fields, for example `Show - S{season}E{episode}.mkv`. A missing `.mkv` extension is appended automatically. The fields and filters are the template engine's; see the Template topic.

Two rules are checked on the *rendered* name (not just the template text), identically on every platform:

- A path separator (`/` or `\`) in the rendered name is an error (`PathSeparatorInRenderedName`); the filename cannot create subdirectories in v1.
- An empty stem, `.` or `..` is an error (`EmptyRenderedName`) - typically a template whose fields all rendered empty.

Collision consequences:

- Two planned outputs rendering to the same path is always an error (`OutputCollision`), independent of the collision policy: the batch is internally inconsistent and no policy could define which plan wins. Disambiguate the template or the input pattern.
- A rendered path that already exists on disk as a pre-existing file is governed by the On collision policy; see that topic for the three behaviors.
- A rendered path equal to any input path is always a hard error (`SourceOverwrite`).

When to use which: `keep` when the source names are already right and only the contents change - the common case for in-place library cleanup into a separate output directory. A template when you normalize a whole batch to one naming scheme; that is what the input pattern's capture groups exist for.
