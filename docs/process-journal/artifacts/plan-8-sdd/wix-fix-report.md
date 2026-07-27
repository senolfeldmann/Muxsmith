# Windows MSI / WiX light.exe failure - diagnosis and fix

**Status:** FIXED. Cascade branch taken: **(2) the technical route that keeps the Ş**.
The ruled ASCII fallback (branch 3) was never reached - and was empirically shown to
be insufficient anyway (see Stage 1, channel (a)).

**Master commit:** `07c0255bdecc4add16640c162ddb606fabe3e603` (unpushed; controller pushes).
**Runs:** D1 `30268008932` (diagnosis, red by design), D2 `30269169772` (fix proof, green,
both Windows legs).
**Debug branch/worktree:** `debug/wix-stderr` created, used, deleted (local + remote);
`.worktrees/debug-wix` removed. The two runs and their logs survive the branch deletion.

---

## 0. Correction to a premise in the brief

The brief named "the PINNED tauri-bundler 2.11.4". `2.11.4` is the pinned
`@tauri-apps/cli` version (`package.json` line 30). At the `tauri-cli-v2.11.4` tag the
CLI depends on **tauri-bundler 2.9.4**:

```
crates/tauri-cli/Cargo.toml:50:
tauri-bundler = { version = "2.9.4", default-features = false, path = "../tauri-bundler" }
```

and the tag's `Cargo.lock` pins `name = "tauri-bundler" / version = "2.9.4"`. All source
citations below are from that tag.

---

## 1. Stage 1 - diagnosis

Debug-branch scaffold (commit `a7b2f1e`, never merged): matrix trimmed to
`windows-x86_64`; the `guard` and `assemble` jobs removed; the `rehearse-draft-release`
input removed.

Removing `guard` was not cosmetic. `ci.yml` triggers on `push.branches: [master]` only,
so no `ci.yml` run can ever exist for a `debug/wix-stderr` SHA, and the guard's gate loop
would have spun its full 45-minute budget and then failed the run without ever reaching
the bundle job.

### Channel (b) - the real light.exe error

Two independent emitters in the same run, both agreeing.

**Mechanism (verified in source before the run, not inferred from the result):**
`tauri-bundler/src/utils/mod.rs` `CommandExt::output_ok` pipes the child's stdout and
stderr and logs every line with `log::debug!(action = "stderr"; ...)`, then throws the
lines away in the error path (`Err(GenericError(format!("failed to run {program}")))`).
`tauri-cli/src/lib.rs` maps `-v` occurrences to a level - `0 => Level::Info, 1 =>
Level::Debug, _ => Level::Trace` - and installs it as the global filter. So a single
`-v` is enough to surface what the bundler discards. Verbosity reaches bundling only as
`settings.set_log_level(...)`, which `msi/mod.rs` never reads (0 occurrences), so `-v`
changes logging and nothing else.

**Result** (run 30268008932, both from the `-v` build and from a direct `light.exe`
re-run with the same argv):

```
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(23) : error LGHT0311 : A string was provided with characters that are not available in the specified database code page '1252'. Either change these characters to ones that exist in the database's code page, or update the database's code page by modifying one of the following attributes: Product/@Codepage, Module/@Codepage, Patch/@Codepage, PatchCreation/@Codepage, or WixLocalization/@Codepage.
C:\agent\_work\36\s\wix\src\ext\UIExtension\wixlib\LicenseAgreementDlg.wxs(27) : error LGHT0311 : A string was provided with characters that are not available in the specified database code page '1252'. ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(15) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(91) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(105) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(127) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(147) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(65) : error LGHT0311 : ...
D:\a\Muxsmith\Muxsmith\target\release\wix\x64\main.wxs(68) : error LGHT0311 : ...
=== light.exe exit code: 55 ===
```

Nine errors. Supporting artefacts from the same step:

```
=== generated locale.wxl (verbatim) ===
<WixLocalization Culture="en-us" xmlns="http://schemas.microsoft.com/wix/2006/localization"><String Id="TauriLanguage">1033</String>
<String Id="TauriCodepage">1252</String>
...
=== main.wxs: Product/Package attributes carrying metadata ===
20:            Manufacturer="Şenol Feldmann"
29:                 SummaryCodepage="!(loc.TauriCodepage)"/>
=== main.wxs Manufacturer line, byte level ===
0000020   f   a   c   t   u   r   e   r   =   " 305 236   e   n   o   l
```

`305 236` octal = `0xC5 0x9E` = UTF-8 for U+015E.

### Channel (a) - the ASCII A/B, and the finding that changes the cascade

Same run, an `if: failure()` step that rewrote `publisher` and `copyright` to ASCII and
rebuilt. **It did not go green:**

```
ascii-ized: "copyright": "Copyright (c) 2026 Senol Feldmann",
=== rebuild with ASCII metadata ===
...
C:\agent\_work\36\s\wix\src\ext\UIExtension\wixlib\LicenseAgreementDlg.wxs(27) : error LGHT0311 : A string was provided with characters that are not available in the specified database code page '1252'. ...
=== ASCII rebuild exit code: 1 ===
total 0
```

`total 0` is the `ls -l target/release/bundle/msi/`: no MSI produced.

So the hypothesis was confirmed in **mechanism** (1252 vs U+015E) and refuted in
**scope**. There are three Ş sinks, not the two the brief assumed:

| sink | reaches the MSI? | evidence |
|---|---|---|
| `bundle.publisher` -> `main.wxs` `Manufacturer` | yes, 8 string rows | the 8 main.wxs LGHT0311 lines |
| `LICENSE` line 3 -> generated `LICENSE.rtf` -> `WixUILicenseRtf` -> `LicenseAgreementDlg` `LicenseText` control | yes, 1 string row | the LicenseAgreementDlg LGHT0311, surviving the ASCII A/B |
| `bundle.copyright` | **no** | `copyright` appears nowhere in `msi/mod.rs` or `main.wxs`; it only reaches the exe's PE version resource, which is UTF-16 |

`main.wxs` line 58-59 wires the license: `{{#if license}} <WixVariable Id="WixUILicenseRtf" Value="{{license}}" />`, and `msi/mod.rs:542-560` generates that RTF from a non-`.rtf`
`licenseFile` by inlining the file's text.

**Consequence for the ruled cascade:** branch (3) as written - ASCII in the MSI metadata
fields only - is demonstrably *not a fix*. Exactly that change was made and the build
still failed. Had the technical route lost, the honest return would have been
NEEDS_CONTEXT (ASCII-izing the LICENSE is a different decision: it is not metadata and it
changes every artifact, not the Windows ones).

---

## 2. Stage 2 - the fix

Two files, both on master in `07c0255`.

**`src-tauri/tauri.conf.json`**

```diff
-        "language": ["en-US"]
+        "language": { "en-US": { "localePath": "wix/locale-en-US.wxl" } }
```

**`src-tauri/wix/locale-en-US.wxl`** (new): `<WixLocalization Culture="en-us"
Codepage="1254" ...>` with `<String Id="TauriCodepage">1254</String>`.

### Mechanism, with citations

1. **The error itself names the hook.** LGHT0311's remediation text lists
   `WixLocalization/@Codepage` among the attributes to change. WiX v3's message catalogue
   at the exact toolset the bundler downloads (`wix3141rtm`, matching the
   `Linker version 3.14.1.8722` banner in the log):
   `wixtoolset/wix3@wix3141rtm src/tools/wix/Data/messages.xml:2449`,
   `<Message Id="InvalidStringForCodepage" Number="311">`.

2. **`WixLocalization/@Codepage` sets the output database's code page.**
   `wixtoolset/wix3@wix3141rtm src/tools/wix/Xsd/wixloc.xsd:31-35`:
   `<xs:attribute name="Codepage" type="xs:string">` - *"The code page integer value or
   web name for the resulting database."*

3. **tauri-bundler exposes that file, and merges rather than replaces.**
   `tauri-bundler/src/bundle/windows/msi/mod.rs:821-856`: when `language_config.locale_path`
   is set the file is read verbatim; the bundler then appends each of its own default
   strings **only if the file does not already contain that `Id="..."` substring**, and
   splices them in before `</WixLocalization>`. Overriding `TauriCodepage` therefore also
   moves `Package/@SummaryCodepage="!(loc.TauriCodepage)"` (`main.wxs:29`) while
   `TauriLanguage` stays on the bundler's 1033. Simulated locally against the real file
   before dispatching the run; the merged result parses as XML and yields exactly
   `TauriCodepage=1254, TauriLanguage=1033` plus the four UI strings.

4. **Path resolution.** `tauri-cli/src/helpers/config.rs:69-105` (`wix_settings`) passes
   `localePath` through unmodified (`String -> PathBuf`, line 88), and both `build.rs:158`
   and `bundle.rs:139` `set_current_dir(dirs.tauri)` before bundling, so the path is
   relative to `src-tauri/`.

   No cross-platform exposure: `bundle.windows.wix` is only read inside
   `build_wix_app_installer`, so the dmg/deb/rpm/appimage/tar.gz legs never look at the
   new file. They keep the Ş unchanged, as they always did.

5. **Why 1254 and not UTF-8.** `messages.xml:2692`,
   `<Message Id="InvalidSummaryInfoCodePage" Number="349">`: *"The code page '{0}' is
   invalid for summary information. You must specify an ANSI code page."* 65001 is out.
   Locally checked with `iconv`: CP1252 and CP1257 cannot encode `Şenol Feldmann`; CP1250
   and CP1254 can (`Ş` = 0xDE in 1254). 1254 (Turkish) is the apt one for the name; 1250
   would have worked equally. 1254 is ASCII-compatible and every other string in the
   installer is ASCII, so nothing but the one character is affected.

6. **Schema-legal.** The pinned CLI's `config.schema.json` `WixLanguage` accepts
   `{"type": "object", "additionalProperties": {"$ref": "#/definitions/WixLanguageConfig"}}`,
   and `WixLanguageConfig` has exactly one property, `localePath`.

### Proof

Run **30269169772** on `87673c4`, both Windows legs, `conclusion: success`:

| job | leg | conclusion |
|---|---|---|
| 89987155655 | windows-x86_64 / windows-2025 | success |
| 89987155742 | windows-arm64 / windows-11-arm | success |

```
      Running [tauri_bundler::bundle::windows::msi] light to produce D:\a\Muxsmith\Muxsmith\target\release\bundle\msi\Muxsmith_0.1.0_x64_en-US.msi
...
20:            Manufacturer="Şenol Feldmann"
29:                 SummaryCodepage="!(loc.TauriCodepage)"/>
-rw-r--r-- 1 runneradmin 197121 6799360 Jul 27 13:20 Muxsmith_0.1.0_x64_en-US.msi
pick: target/release/bundle/msi/Muxsmith_0.1.0_x64_en-US.msi
-rw-r--r-- 1 runneradmin 197121 6799360 Jul 27 13:20 muxsmith-0.1.0-windows-x86_64.msi
```

Zero LGHT0311. Both legs emit four `warning LGHT1076` ICE notes (ICE03 string overflow on
`DownloadAndInvokeBootstrapper`, ICE40 REINSTALLMODE, ICE57 `CMP_UninstallShortcut`,
ICE61 no maximum upgrade version). All four originate in tauri-bundler's own `main.wxs`
template, none is fatal, and they are new to the log only because no light run had ever
succeeded before. Flagging, not acting.

### Artifact-level verification (green build is not the same as correct metadata)

Both MSIs downloaded and inspected locally:

```
$ msiinfo export muxsmith-0.1.0-windows-x86_64.msi Property | grep Manufacturer
Manufacturer	Şenol Feldmann
```

Raw summary-information stream parse (x64):

```
PID 1: VT_I2 = 1254                     <- PID_CODEPAGE
PID 4: VT_LPSTR raw=b'\xdeenol Feldmann' cp1254='Şenol Feldmann'   <- PID_AUTHOR
```

Byte counts in each MSI: CP1254-encoded `Şenol Feldmann` 4 occurrences, ASCII
`Senol Feldmann` 0, raw UTF-8 `Ş` 0. Same result for arm64. (`msiinfo suminfo` prints the
Author as `?enol Feldmann`; that is msitools ignoring PID_CODEPAGE, not an MSI defect -
PID 1 = 1254 is right there in the stream.)

### One residual, unverified and cosmetic

`msi/mod.rs:547` generates `LICENSE.rtf` with a hard-coded `\ansi\ansicpg1252` header
while the license body now reaches the installer as `Ş`. Whether the RichEdit control in
the license dialog renders that one character correctly was **not** verified - it would
need the MSI installed on a Windows box. It is one character in tauri-bundler's generated
RTF, not in our configuration, and it does not affect the build, the installed files, or
the Add/Remove Programs metadata.

---

## 3. Run and command ledger

Every `gh` interaction is in `/home/senol/Git/Muxsmith/gh-log.md` (main tree) with
timestamp, exact command, effect and manual-UI equivalent, including the two branch
pushes and the branch deletion. Nothing cost money (public-repo standard runners), no tag
was created, no release was created, and the debug branch's workflow had no release path
at all.

| run | ref | purpose | conclusion |
|---|---|---|---|
| 30263340264 | master | the original rehearsal (pre-existing) | failure, both Windows legs in light.exe |
| 30268008932 | debug/wix-stderr `a7b2f1e` | diagnosis, both evidence channels | failure by design; LGHT0311 x9 captured, ASCII A/B still red |
| 30269169772 | debug/wix-stderr `87673c4` | fix proof | success, both Windows legs |
