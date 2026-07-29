# 🔨 Muxsmith

**Declare how your MKVs should look. Muxsmith forges the whole library into shape - one profile, hundreds of files, zero clicking.**

> ⚠️ **Work in progress.** You are early: Muxsmith is public on its way to 1.0, not there yet. The CLI and GUI run path works end to end and is CI-tested on Linux, macOS and Windows (live mkvmerge integration tests currently run on Linux), but expect rough edges and possibly breaking profile changes until the 1.0 tag. The [roadmap](docs/ROADMAP.md) is public and honest.

<!-- placeholder(1.0): GIF - profile -> dry-run -> run in ~15 seconds -->

## 😤 The problem

You have 400 episodes. The German audio is track 2 on some files and track 3 on others. Half the subtitle tracks are literally named "und" (hello, undetermined language tag), some carry forced flags that lie, and a few still haul around a title like `[GROUP] Show.S01E01.1080p.FINAL-v2`. mkvtoolnix-gui fixes each file beautifully - one file at a time, by hand, again next season.

Muxsmith flips the workflow: you write down **once** what a correct file looks like. Every file gets measured against that description and rebuilt to match. Same profile next season, same result.

## ⚖️ Is Muxsmith for you?

**Yes, if** you curate a library: the same fix applied to many files, repeatably, with a paper trail.

**No, if** you want to inspect and tweak one file interactively - that is [MKVToolNix](https://mkvtoolnix.download/) territory, it is excellent at it, and Muxsmith happily coexists with it. In fact mkvmerge does all the actual muxing underneath; Muxsmith never reimplements the format, it orchestrates the tool that already does it right.

**What Muxsmith deliberately does not do: guess.** No language-from-filename, no auto-title, no "this is probably what you meant". Interactive tools can afford pre-filled guesses because you review every field before hitting go. An unattended batch tool cannot - a wrong guess applied 400 times is 400 broken files. The profile is the spec; if the profile does not say it, it does not happen.

## 🧠 How it works

Three steps: write a profile, dry-run it, run it.

```yaml
profile_version: 1

input:
  extensions: [mkv]
  recursive: true

output:
  filename: keep
  on_collision: error        # never clobber anything by accident

tracks:
  unmatched: drop            # nothing you didn't ask for survives
  rules:
    - match: { exact: { type: video } }

    - match: { exact: { type: audio, language: de } }
      changes: { default_track: true }

    - match: { exact: { type: audio, language: en } }

    - match: { exact: { type: subtitles, language: de, forced_track: true } }
      optional: true         # fine if a file doesn't have one
      changes: { track_name: German forced, default_track: true }
```

Matching is **typed, not stringly**. `exact` compares each known property in its own domain: booleans as booleans, numbers numerically (`6` equals `6.0`), languages as languages - `de` matches a file tagged `ger`, and `pt-BR` does **not** match `pt-PT`. For the properties that genuinely are strings - track names, codec IDs - `substring` (case-insensitive containment) and `regex` do the messy-reality work, and `any`/`not` combine expressions into whatever your library's chaos requires. Point `substring` or `regex` at a known property that is not string-typed and you get a config-time error, not a silent never-match; `codec_kind` rejects both conditions even though it is string-typed, because a pattern over a curated alias is ill-defined - pattern-match `codec_id` instead. Rule order is output track order.

Three places where `exact` does more than compare, and one where it deliberately does less. Worth knowing by name, because everything else is a plain value comparison:

- **`language` normalizes, and reads both fields.** ISO 639 spellings and BCP-47 tags are reduced to canonical form before comparison, and the value is matched against both `language` and `language_ietf` as mkvmerge reports them. One `language: de` therefore covers a file that tags `ger`, one that tags `de`, and one that only fills the IETF field.
- **Boolean flags are false when absent.** mkvmerge emits the vanity flags (`flag_commentary`, `flag_original`, `flag_hearing_impaired`, `flag_visual_impaired`) only when they are set, and Matroska defines them false otherwise - so `exact: { flag_commentary: false }` matches a track that never mentioned the flag. No `not:` gymnastics required.
- **`type` and `codec_kind` have curated closed domains.** A value outside them - `type: subtitle` where the domain says `subtitles`, a codec alias that does not exist - is a config-time error, not a rule that quietly never matches. The typo surfaces at validate time instead of after 400 files came out wrong.
- **`raw:` is the deliberate opt-out.** Prefix a property name with `raw:` to match a field your mkvmerge reports but this build's schema does not know yet, and every convenience above switches off: byte-exact value equality against that one field, named verbatim. No language normalization, no codec aliasing, no false-when-absent, and no type check on `substring`/`regex` either - you asked for untyped, you get untyped.

Then:

```console
$ muxsmith dry-run profile.yaml --source /media/series --output /media/clean
```

<!-- placeholder(1.0): real dry-run output snippet, regenerated from an actual run -->

The dry-run is the heart of the tool. It identifies every file, resolves every rule, and tells you exactly what would happen - including **suggestions**: when a rule matches two tracks or none, Muxsmith proposes the narrowing that fixes it, as a YAML fragment you can paste straight into the profile. A suggestion is only offered if it survives the *next* dry-run too - no whack-a-mole.

Happy with the plan? Same command, `run` instead of `dry-run`, `--jobs 8` if you are in a hurry.

### Pure passthrough: a profile with zero rules

A profile whose `tracks` block is `{ unmatched: keep, rules: [] }` is a legal pure-passthrough remux: every track of each source file is copied unchanged. Use it to change only the title, attachments, or chapters, or to normalize containers in bulk, without writing a single track rule:

```yaml
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  unmatched: keep
  rules: []
title: { template: 'S{season}E{episode}' }
```

Validation announces the passthrough with an info notice (`passthrough-profile`), so an accidentally emptied rule list never fails silently. With `unmatched: drop` (the default), an empty rule list stays an error - a profile that drops everything and selects nothing cannot produce output.

## ✨ What you get

- **A real dry-run.** Nothing touches disk until you say so; the plan is printed, diagnosed, and suggestible.
- **Typed matching** on the properties mkvmerge actually reports (language, codec, flags, track names, ...).
- **External sources**: pull subtitles from sidecar files next to each episode, matched per file.
- **Attachments handled with care**: fonts are kept by default, because silently dropping them breaks ASS subtitles - the classic batch-tool footgun.
- **Chapters, tags, and title policies**: keep, drop, or clear - stated in the profile, not remembered in your head.
- **Collision protection twice over**: an `on_collision` policy for outputs, plus a guard that refuses to overwrite any file that is also a mux *source*.
- **A parallel job engine** with per-job logs: every run persists the full command line and output of every job (auto-pruned after 14 days). Exit codes mean something; `--fail-fast` exists.
- **Scriptable everything**: `validate`, `dry-run`, `identify` and `run` each take `--json` and emit a structured report. `muxsmith schema` needs no flag - it prints the profile's JSON Schema and nothing else.
- **Two surfaces, one core**: the CLI and the desktop GUI (Tauri + Vue) share the same Rust engine - same profiles, same semantics, same logs.
- **Localized**: English today, German landing for 1.0; diagnostics are structured, prose is generated per locale.

## 📦 Install

Muxsmith needs **mkvmerge >= 86.0** on the PATH or in a standard install location (it ships with [MKVToolNix](https://mkvtoolnix.download/)).

<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS install-hurdle
     steps; drop the WIP banner in the same pass -->

Until packaged releases land, build from source:

```console
$ cargo build --release -p muxsmith-cli   # the muxsmith CLI binary
$ pnpm install && pnpm exec tauri build   # the desktop GUI
```

Toolchain versions are pinned in the repo (`rust-toolchain.toml`, `mise.toml`); [BUILDING.md](BUILDING.md) is the full developer document, including the quality gate the project runs on.

## 🖥️ Using the CLI

Five subcommands, one shape. Four of them - `validate`, `dry-run`, `identify`, `run` - take `--json` (structured report for scripting; the human output renders the same data) and `--locale` (message language override; the default is your system locale, falling back to English). `muxsmith schema` takes neither: it writes the schema to stdout and has no rendered messages to translate.

### `muxsmith validate <profile>`

Static profile check without touching any media: schema shape, rule sanity, language tags, pattern syntax. Prints diagnostics with severities.

```console
$ muxsmith validate series.yaml
```

### `muxsmith identify <file>`

One file's tracks exactly as mkvmerge reports them - which is exactly what the matchers see. This is where you look up the property names and values to write `match:` expressions against.

```console
$ muxsmith identify S01E01.mkv
```

### `muxsmith dry-run <profile> [--source DIR] [--output DIR] [--on-collision <policy>]`

Plans the whole batch: scan, identify, resolve every rule against every file. Prints the per-file resolution, all diagnostics, and paste-ready suggestions. Touches nothing on disk, ever.

`--on-collision` decides what happens when a planned output already exists: `error` refuses it (the default policy), `skip` skips it with a warning, `overwrite` replaces the pre-existing file. Left unset, it falls back to the profile's `output.on_collision`.

```console
$ muxsmith dry-run series.yaml --source /media/series --output /media/clean
```

### `muxsmith run <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--on-collision <policy>]`

The same planning, then execution with `N` parallel mux jobs (default 1). Every job's full mkvmerge command line and output persist to the run log (auto-pruned after 14 days). `--fail-fast` stops dequeuing new jobs after the first failure and lets in-flight jobs finish cleanly, and `--on-collision` carries the same three policies described under `dry-run` above.

```console
$ muxsmith run series.yaml --source /media/series --output /media/clean --jobs 8
```

### `muxsmith schema`

Prints the profile's JSON Schema, pretty-printed, to stdout. Redirect it to a file and point your editor at it, and hand-authoring a profile gets the same autocompletion and inline validation as any other structured config format: every key, every enum value, every field's shape, right there while you type.

```console
$ muxsmith schema > muxsmith-profile.schema.json
```

**VS Code**, via the [YAML extension](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml), in `settings.json`:

```jsonc
{
  "yaml.schemas": {
    "./muxsmith-profile.schema.json": "*.muxsmith.yaml"
  }
}
```

**Neovim / Helix**, via `yaml-language-server`, in your `lspconfig` settings:

```lua
settings = {
  yaml = {
    schemas = {
      ["./muxsmith-profile.schema.json"] = "*.muxsmith.yaml",
    },
  },
}
```

Bind it in editor settings, not with the in-file `# yaml-language-server: $schema=...` modeline. The modeline is one line and no editor config, which is exactly why it is a trap: it lives inside a YAML comment, and the GUI's save does not preserve comments, key order, or formatting - it writes the profile fresh from its own model. Wire up a modeline, then save the same profile once from the GUI, and the binding is gone. No error, no warning: the file still works, your editor just quietly stops helping. Bind the schema in your editor's own settings instead, and it survives every save because it never lived in the file the GUI rewrites.

Two conventions that hold everywhere: **command-line flags override profile-stored values** (`--source`, `--output`, `--on-collision`), and **exit codes mirror mkvmerge's own**: `0` clean, `1` finished with warnings, `2` errors - your scripts already speak this dialect. Interrupt any of them with Ctrl-C and you get `130` instead, the shell's own convention for a signalled process, so handle it in your `case $?`. Only `run` earns that code gracefully: the first Ctrl-C kills the in-flight jobs, deletes their partial output and still prints the summary, and a second one force-exits on the spot, part-way through that cleanup.

## 🖱️ The GUI

The desktop app wraps the same engine: a batch view (pick a profile, pick directories, dry-run, run), a live jobs view with per-job cancel, and a run history with log export. Same profiles, same semantics, same logs as the CLI.

<!-- placeholder(1.0): one GUI screenshot -->

## 🤝 How this got built (a human-AI story)

Muxsmith existed twice. The first time as Ruby CLI drafts in a drawer - the classic personal-tool fate. The second time as a deliberate experiment: what happens when you build a real product with a fleet of AI agents under tight human direction?

The setup: implementer agents write code, independent reviewer agents tear it apart (fresh context, their ground truth is the spec, not the implementation), and a controller session orchestrates plans, briefs, and merges - asking the human at every decision that matters. Every design decision is numbered and recorded with its rationale and rejected alternatives: 103 of them so far, running up to `D105` because two numbers were reserved for a plan that never spent them. The whole process is public in this repo: [docs/](docs/) carries the process journal, every plan, and the preserved review verdicts - 219 files under `docs/` with `verdict` in the name, including the ones that hurt.

It has been a journey, and the journal does not hide the failures: bugs that slipped through when discipline slipped, process holes found by auditing our own transcripts, rules written in the ashes of the mistake that motivated them. That is the point. The code is the product; the process record is the experiment's data.

## 🗺️ Status

Pre-1.0. The run path is complete on both surfaces; the [roadmap](docs/ROADMAP.md) lists what stands between here and the tag - hardening, a German locale, packaging, and a deliberately paranoid review pass. Deferred ideas live in [docs/IDEAS.md](docs/IDEAS.md) with their reasoning, so "no" stays explainable.

## 📄 License

[MIT](LICENSE). mkvmerge is invoked as an external process and is licensed separately (GPL) - Muxsmith contains no MKVToolNix code.
