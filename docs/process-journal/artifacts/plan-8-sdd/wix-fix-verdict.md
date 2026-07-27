# Verdict: WiX codepage fix (07c0255) - APPROVED

**Reviewer:** independent (fresh context). **Subject:** commit `07c0255`, diff package
`review-663b6ec..07c0255.diff`, report `wix-fix-report.md`.
**Scope:** read-only; nothing written outside this file and my `gh-log.md` entries. No commits, no git writes.

**APPROVED.** The shipped fix is correct, minimal, mechanism-verified at the pinned upstream
sources, and empirically proven on both Windows legs with the produced MSIs re-verified from
still-live artifacts. Two **record-level corrections are required** (F1, F2 below); neither touches
the two shipped files, so both route to the plan-8 close batch per the precedent set by the
joblog-fix review (wrong sweep figure -> APPROVED, `CORRECTION` line in `progress.md`).

---

## 1. Diff shape, JSON, XML - PASS

- `git diff 663b6ec..07c0255` is **exactly two paths**: `M src-tauri/tauri.conf.json` (1 line changed),
  `A src-tauri/wix/locale-en-US.wxl` (23 lines). No other path in the range.
- The diff package's body is **byte-identical** to `git diff -U10 663b6ec..07c0255` (regenerated at
  -U3/5/7/10; matched at -U10). Above the body the package carries a header (commit line + stat).
  The earlier apparent mismatch was context width plus that header, not content.
- `tauri.conf.json` parses under both `python3 -c json.load` and `jq`.
  `bundle.windows.wix.language` = `{"en-US": {"localePath": "wix/locale-en-US.wxl"}}`.
- The `.wxl` is well-formed XML (`xmllint --noout` clean, `ElementTree` parses). UTF-8, no BOM, LF
  line endings, trailing newline present. Root `WixLocalization` with `Culture="en-us"` **and
  `Codepage="1254"`**; exactly one child `<String Id="TauriCodepage">1254</String>`.
- Path coherence: the file sits at `src-tauri/wix/locale-en-US.wxl`, and `localePath` is
  `wix/locale-en-US.wxl` relative to `src-tauri/` - consistent with the cwd fact verified in §2.

## 2. Mechanism citations at the pinned sources - PASS

**The report's correction of the brief's premise is right, and the brief was wrong.**
`pnpm-lock.yaml` pins `@tauri-apps/cli` 2.11.4 (specifier and resolved version). `tauri-bundler`
appears in **no** lockfile in this repo (the app's lockfile is the root `Cargo.lock`; there is no
`src-tauri/Cargo.lock`) - correct and expected, because the bundler is compiled into the
npm-distributed prebuilt CLI binary, so a consuming lockfile structurally cannot name it. At tag
`tauri-cli-v2.11.4`: `crates/tauri-cli/Cargo.toml:50` is verbatim
`tauri-bundler = { version = "2.9.4", default-features = false, path = "../tauri-bundler" }`, the
tag's `Cargo.lock` carries `tauri-bundler` 2.9.4 (:8854-8855) alongside `tauri-cli` 2.11.4, and
`crates/tauri-bundler/Cargo.toml` declares 2.9.4. **Pinned bundler = 2.9.4.**

**Merge behavior** - `msi/mod.rs:821-856` is exactly the cited block, and the report's reading of it
is accurate: `locale_path: Some(p)` reads the file verbatim (:821-822); each default's id is
extracted as the `Id="..."` substring (:836-846) and appended **only** when
`!locale_contents.contains(&id)`; the survivors are spliced in via
`replace("</WixLocalization>", ...)` (:848-851).

Verified two independent ways, both agreeing:

1. **Simulation.** I reimplemented the extraction and merge in Python against the real repo file,
   the tag's `default-locale-strings.xml`, and its `languages.json` (en-US: `langId` 1033,
   `asciiCode` 1252 - the 1252 default the error complained about). Result: `Id="TauriCodepage"`
   suppressed (ours wins), the other five appended -> `TauriCodepage=1254`, `TauriLanguage=1033`
   plus the four UI strings, six `String` elements, valid XML.
2. **Empirically, from the run.** The D2 log's DEBUG step cats the merged `locale.wxl` the bundler
   actually handed to `light`. It matches my simulation, including ordering.

So the report's claim - `TauriCodepage=1254, TauriLanguage=1033` plus four UI strings - is exact,
and its "four UI strings" count is right (6 defaults total, 2 of them the Tauri pair).

**LGHT0311 remediation reading** - `wixtoolset/wix3@wix3141rtm src/tools/wix/Data/messages.xml:2449`
is verbatim `<Message Id="InvalidStringForCodepage" Number="311">`, and its `<Instance>` text matches
the error quoted in the report **verbatim**, including the remediation list that names
`WixLocalization/@Codepage`. `Xsd/wixloc.xsd:31-35` is exactly the `Codepage` attribute block and
the quoted sentence is verbatim. `messages.xml:2692` is verbatim
`<Message Id="InvalidSummaryInfoCodePage" Number="349">` with *"You must specify an ANSI code page."*,
which is what rules out 65001. The 1254 choice is sound: ANSI, ASCII-compatible, `Ş` at 0xDE.

**Path resolution / schema** - `cli-config.rs:69-105` is precisely the `wix_settings` function and
line 88 is precisely `locale_path: config.locale_path.map(Into::into)`; `build.rs:158` and
`bundle.rs:139` are precisely the `set_current_dir(dirs.tauri)` sites, so a `localePath` relative to
`src-tauri/` is correct. The pinned tag's `config.schema.json` `WixLanguage` third variant is
`{"type":"object","additionalProperties":{"$ref":"#/definitions/WixLanguageConfig"}}`, and
`WixLanguageConfig` has exactly one property, `localePath`, with `additionalProperties: false`.
Every one of these citations is line-accurate.

**Diagnosis-stage mechanism** (secondary, checked since I had the files) - `utils/mod.rs` logs each
child stderr line via `log::debug!(action = "stderr"; ...)` (:87) and the failure path returns
`GenericError(format!("failed to run {program}"))` (:105-106), discarding them; `tauri-cli/src/lib.rs:308-310`
is the verbatim `0 => Level::Info, 1 => Level::Debug, _ => Level::Trace`; `log_level` occurs **0**
times in `msi/mod.rs`. All as reported.

## 3. Proof run 30269169772 - PASS

`gh run view` (read-only, logged): conclusion **success**, head SHA
`87673c4972f15f144bd82c9d43b8807cef502cad` (the report's `87673c4`), branch `debug/wix-stderr`,
event `workflow_dispatch`, workflow `release`, created 2026-07-27T13:12:05Z. **Exactly two jobs,
both `success`** - and both IDs match the report's table character-for-character:

| job id | leg | conclusion |
|---|---|---|
| 89987155655 | `bundle (windows-x86_64, windows-2025, msi, x64)` | success |
| 89987155742 | `bundle (windows-arm64, windows-11-arm, msi, arm64)` | success |

From the full log (2247 lines): **zero `error LGHT0311`, and zero `error LGHT` of any number.**
I made that negative fire before trusting it - a bare `grep -c LGHT0311` returns **2**, and both hits
are our own `.wxl` comment being echoed by the DEBUG step (`light.exe then rejects the link with
LGHT0311 ...`), not emissions. The report's "Zero LGHT0311" is correct as an error count; see H1.

The four `LGHT1076` ICE warnings appear exactly twice each (once per leg) and match the report's
identifiers verbatim: ICE03 `DownloadAndInvokeBootstrapper`, ICE40 REINSTALLMODE, ICE57
`CMP_UninstallShortcut`, ICE61 no maximum upgrade version. Non-fatal; flagging-not-acting is right.
`Manufacturer="Şenol Feldmann"` and `SummaryCodepage="!(loc.TauriCodepage)"` each occur twice in the
generated `main.wxs` at lines 20 and 29 - matching the upstream template (:19 `Language=`, :20
`Manufacturer=`, :29 `SummaryCodepage=`). The report's quoted size `6799360` appears verbatim (twice,
pre- and post-rename); arm64 is `6344704`.

## 4. MSI inspection - artifacts were NOT expired, so I re-verified rather than assessed coherence

The run page's artifact list (`gh api .../artifacts`, logged): `total_count` 2,
`muxsmith-windows-x86_64` (6569577 B) and `muxsmith-windows-arm64` (6129632 B), created
2026-07-27T13:19-13:20Z, `expires_at` 2026-08-03, **`expired: false`**. Retention had not eaten them,
so the honest move was to download and check, not to reason about internal coherence. Downloaded
MSI sizes match the log exactly (6799360 / 6344704).

Independently reproduced, both MSIs:

- `msiinfo export <msi> Property` -> `Manufacturer` = `0xC5 0x9E` + `enol Feldmann`, i.e. UTF-8 `Ş`.
  That msitools emits well-formed UTF-8 here means it **decoded a stored `0xDE` through the database
  codepage** - independent corroboration that the string table really is CP1254.
- Raw summary-information stream parse: **`PID_CODEPAGE` (PID 1, VT_I2) = 1254**; `PID_AUTHOR`
  (PID 4, VT_LPSTR) raw = `b'\xdeenol Feldmann'`, cp1254 = `'Şenol Feldmann'`. Matches the report's
  quoted repr exactly, for both x64 and arm64.
- CP1254 `Şenol Feldmann` = **4** occurrences per MSI (as reported); ASCII `Senol Feldmann` = **0**.

**F1 (Medium, report-level - correction required).** The report's third byte count, "raw UTF-8 `Ş` 0",
is **wrong as written**. Whole-file `b'\xc5\x9e'` occurs **89** times (x64) and **102** times (arm64).
The substance still holds, and I established that on a stronger basis than the report did: every one
of those hits lies inside the embedded CAB (MSCF at offset 20480; lowest hit 50885 / 152852), **zero
in the metadata region**, and `b'\xc5\x9eenol'` - an actual mojibake `Şenol` - is **0** in both files.
The observed counts sit right on the chance expectation for a random 2-byte pair in high-entropy
compressed data (~104 / ~97). So: no mis-encoded `Ş` anywhere that matters, but the number as
published is not what the file contains. Restate it scoped, e.g. *"no UTF-8-encoded `Ş` outside the
compressed CAB payload; 0 occurrences of `\xc5\x9eenol`"*.

Minor: the report says `msiinfo suminfo` prints the Author as `?enol Feldmann`. It actually emits the
raw `0xDE` byte, which a UTF-8 terminal *renders* as `?`/replacement. The report's interpretation
(suminfo ignores `PID_CODEPAGE`; not an MSI defect) is correct - only the `?` is a rendering
presented as output.

## 5. No drive-by changes; D89 naming and D86 structure - PASS, with F2

The range touches only the two files, so nothing else changed by construction. On substance:

- **D86 literals all intact on master**: `upgradeCode` `9262b417-b687-5ea3-ace1-18b9d51b215f`,
  `publisher` `Şenol Feldmann`, `copyright` `Copyright (c) 2026 Şenol Feldmann`, `category` `Video`,
  deb `section` `video`, deb/rpm `recommends` `mkvtoolnix`, `minimumSystemVersion` `11.0`, `targets`
  unchanged. Absences preserved: `createUpdaterArtifacts`, `fileAssociations`, `macOS.dmg`,
  `linux.appimage` still absent. **Worth stating explicitly: the debug branch's ASCII A/B step
  rewrote `publisher` and `copyright`, and none of that leaked onto master** - both still carry `Ş`.
- **D89 naming intact**: the run produced `muxsmith-0.1.0-windows-x86_64.msi` and
  `muxsmith-0.1.0-windows-arm64.msi`, matching plan D89 items 1-2 character-for-character; the
  artifacts I downloaded carry exactly those names. Because the language **key** stayed `en-US`, the
  bundler's pre-rename filename stayed `Muxsmith_0.1.0_<arch>_en-US.msi` and the rename step's
  `pick()` matched unchanged. `release.yml` untouched.

**F2 (Medium, record-level - entry required).** The fix changed a literal that design section 11
freezes, and the design and plan still describe the old shape while not knowing the new file:

- design `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`: **:958**
  (`windows.wix.language`: `["en-US"]`, explicit), **:1012**, **:1511** (the verbatim config fence),
  **:2007** (section 11's frozen-literal list: "en-US language list")
- plan `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`: **:248** (transcribed fence),
  **:262** (same frozen-literal enumeration)

The shipped value is now a **map**, and `src-tauri/wix/locale-en-US.wxl` is a new member of the D86
config surface that neither document mentions. The change itself is authorized - owner ruling (6)
licensed exactly one bounded codepage attempt - so this is not an unauthorized deviation; what is
missing is the dependency sweep `conventions.md` requires when a standing enumeration's referent
changes. The report's residual list names two follow-ups (the `ansicpg1252` cosmetic, the 2.9.4
citation sweep) but not this one, and `progress.md`'s closing line likewise omits it. Add it to the
same plan-close one-liner batch. Related: plan **:718 trigger 7** ("German installer UI request ->
reopen D86's `wix.language`") is now stale in its premise - a reopen starts from a map carrying a
`localePath`, not from a list.

## 6. Version-sync guard - PASS

`./scripts/check-version-sync.sh` -> `version-sync: OK (0.1.0)`, exit 0.

## Minor citation nits (no action required beyond the close batch, if convenient)

- `msi/mod.rs:547` is cited for the hard-coded `\ansi\ansicpg1252` header; the literal is on **548**
  (547 is `let license_rtf = format!(`). The `542-560` range for the license block is correct, and
  the residual itself is real and correctly characterized as upstream and cosmetic.
- "`bundle.windows.wix` is only read inside `build_wix_app_installer`" - `.wix` is also read at
  `msi/mod.rs:376`, inside `run_candle`. The substance stands completely: all five read sites
  (376, 446, 569, 591, 663) are in `msi/mod.rs`, and `locale_path` specifically is read only at :821,
  so no dmg/deb/rpm/appimage/tar.gz leg ever looks at the new file.
- "`copyright` appears nowhere in `msi/mod.rs`" - it appears twice, both in the file's own copyright
  header comment; there is no `settings.copyright()` read, and `main.wxs` has 0 occurrences. The
  three-sink conclusion is sound.
- The `wixloc.xsd` quotation stops mid-documentation (the source continues "You can also specify -1
  which will not reset the database code page...") without an ellipsis. Verbatim as far as it goes.

## HARVEST

1. **A fix that documents its own error code makes that code self-matching.** `grep LGHT0311` over
   the green log now hits the `.wxl` comment explaining the fix. Any future "still zero" re-check must
   grep the **emission form** (`error LGHT0311`), not the bare code. General rule: when a remediation
   comment names the error string, scope every later negative check to the emitted shape.
2. **tauri-bundler's locale merge is substring-based over the whole file, comments included.** A
   default is suppressed when the file contains `Id="<Name>"` anywhere, and the splice is
   `replace("</WixLocalization>", ...)` on **all** occurrences. The current comment is safe (it spells
   `TauriCodepage`/`TauriLanguage` bare and never the literal closing tag), but a future editor
   writing `Id="TauriLanguage"` in prose would silently drop a default string. Worth a line in the
   file's own comment if it is ever touched.
3. **A prebuilt-binary toolchain pin is unverifiable from the consuming repo.** No lockfile here can
   name `tauri-bundler`, because it ships inside the npm CLI binary; the version lives only in the
   upstream tag. Version claims about it must cite the tag (this report does). Registered plan trigger
   3 (re-verify the pinned bundler facts on a CLI bump) is the right home for this.
4. **msitools splits codepage handling.** `msiinfo export <table>` honors the database codepage and
   emits UTF-8; `msiinfo suminfo` does not and emits raw bytes. Judging a name broken from `suminfo`
   alone is a false alarm; the sound pair is `export` plus a raw `PID_CODEPAGE` parse.
5. **Byte-count evidence over a whole installer is dominated by its compressed payload.** A 2-byte
   probe hits roughly `size/65536` times by chance (~100 in 6.8 MB), which is exactly what F1 was.
   Scope such counts to the metadata region (before the `MSCF` offset) or use a longer discriminating
   needle (`\xc5\x9eenol`, not `\xc5\x9e`).

## Required before this is considered closed

Neither item touches `tauri.conf.json` or the `.wxl`; the commit is good as it stands.

1. Correct F1's byte-count claim in `wix-fix-report.md` (scope it, or restate as the CAB-only finding).
2. Add F2 to the plan-8 close one-liner batch: sync design :958/:1012/:1511/:2007 and plan :248/:262
   to the map shape, register `src-tauri/wix/locale-en-US.wxl` as part of the D86 config surface, and
   refresh plan :718 trigger 7's premise.
