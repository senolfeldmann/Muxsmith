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

Matching is **typed, not stringly**. `exact` compares every property in its own domain: booleans as booleans, numbers numerically (`6` equals `6.0`), languages as languages - `de` matches a file tagged `ger`, and `pt-BR` does **not** match `pt-PT`. For the properties that genuinely are strings - track names, codec IDs - `substring` (case-insensitive containment) and `regex` do the messy-reality work, and `any`/`not` combine expressions into whatever your library's chaos requires. Aiming `substring` or `regex` at a non-string property is a config-time error, not a silent never-match. Rule order is output track order.

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
- **Scriptable everything**: every command takes `--json` and emits a structured report. `muxsmith schema` prints the profile's JSON Schema.
- **Two surfaces, one core**: the CLI and the desktop GUI (Tauri + Vue) share the same Rust engine - same profiles, same semantics, same logs.
- **Localized**: English today, German landing for 1.0; diagnostics are structured, prose is generated per locale.

## 📦 Install

Muxsmith needs **mkvmerge >= 86.0** on the PATH or in a standard install location (it ships with [MKVToolNix](https://mkvtoolnix.download/)).

<!-- placeholder(1.0): release artifacts per OS (msi/dmg/deb/rpm/AppImage) once the packaging pipeline lands -->

Until packaged releases land, build from source:

```console
$ cargo build --release -p muxsmith-cli   # the muxsmith CLI binary
$ pnpm install && pnpm exec tauri build   # the desktop GUI
```

Toolchain versions are pinned in the repo (`rust-toolchain.toml`, `mise.toml`); [BUILDING.md](BUILDING.md) is the full developer document, including the quality gate the project runs on.

## 🖥️ Using the CLI

Five subcommands, one shape. Every one of them takes `--json` (structured report for scripting; the human output renders the same data) and `--locale` (message language override).

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

```console
$ muxsmith dry-run series.yaml --source /media/series --output /media/clean
```

### `muxsmith run <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--on-collision <policy>]`

The same planning, then execution with `N` parallel mux jobs (default 1). Every job's full mkvmerge command line and output persist to the run log (auto-pruned after 14 days). `--fail-fast` stops dequeuing new jobs after the first failure and lets in-flight jobs finish cleanly.

```console
$ muxsmith run series.yaml --source /media/series --output /media/clean --jobs 8
```

### `muxsmith schema`

Prints the profile's JSON Schema, pretty-printed, to stdout.

Two conventions that hold everywhere: **command-line flags override profile-stored values** (`--source`, `--output`, `--on-collision`), and **exit codes mirror mkvmerge's own**: `0` clean, `1` finished with warnings, `2` errors - your scripts already speak this dialect.

## 🖱️ The GUI

The desktop app wraps the same engine: a batch view (pick a profile, pick directories, dry-run, run), a live jobs view with per-job cancel, and a run history with log export. Same profiles, same semantics, same logs as the CLI.

<!-- placeholder(1.0): one GUI screenshot -->

## 🤝 How this got built (a human-AI story)

Muxsmith existed twice. The first time as Ruby CLI drafts in a drawer - the classic personal-tool fate. The second time as a deliberate experiment: what happens when you build a real product with a fleet of AI agents under tight human direction?

The setup: implementer agents write code, independent reviewer agents tear it apart (fresh context, their ground truth is the spec, not the implementation), and a controller session orchestrates plans, briefs, and merges - asking the human at every decision that matters. Every design decision is numbered and recorded with its rationale and rejected alternatives (D1 through D35 so far). The whole process is public in this repo: [docs/](docs/) carries the process journal, every plan, and the preserved review verdicts - all 78 of them, including the ones that hurt.

It has been a journey, and the journal does not hide the failures: bugs that slipped through when discipline slipped, process holes found by auditing our own transcripts, rules written in the ashes of the mistake that motivated them. That is the point. The code is the product; the process record is the experiment's data.

## 🗺️ Status

Pre-1.0. The run path is complete on both surfaces; the [roadmap](docs/ROADMAP.md) lists what stands between here and the tag - hardening, a German locale, packaging, and a deliberately paranoid review pass. Deferred ideas live in [docs/IDEAS.md](docs/IDEAS.md) with their reasoning, so "no" stays explainable.

## 📄 License

[MIT](LICENSE). mkvmerge is invoked as an external process and is licensed separately (GPL) - Muxsmith contains no MKVToolNix code.
