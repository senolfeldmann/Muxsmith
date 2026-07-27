# Installing Muxsmith

Release downloads live on the
[GitHub releases page](https://github.com/senolfeldmann/Muxsmith/releases).
All 1.0-era builds are **unsigned**: your OS will warn before the first
launch. The sections below show the one-time steps per OS. Verify
downloads against the release's `SHA256SUMS`, with the files beside it:
`sha256sum -c SHA256SUMS` on Linux, and in PowerShell on Windows
`Get-FileHash muxsmith-<version>-windows-x86_64.msi -Algorithm SHA256`
compared against that file's line in `SHA256SUMS`.

Every install ships two programs: **Muxsmith** (the GUI) and
**`muxsmith`** (the command-line tool). The AppImage is the practical
exception: it carries both binaries inside a single self-contained
file, so only the GUI is directly runnable - use the deb, rpm or
tar.gz if you want the CLI on your PATH.

<!-- When code signing lands (registered ROADMAP trigger), the
     SmartScreen and Gatekeeper sections below shrink to the signed-app
     reality; keep the CLI/PATH halves. -->

## Windows

Artifact: `muxsmith-<version>-windows-x86_64.msi` (Intel/AMD) or
`muxsmith-<version>-windows-arm64.msi` (Windows on ARM, e.g. Snapdragon
laptops). The installer is per-machine and installs to
`C:\Program Files\Muxsmith`.

**SmartScreen:** because the installer is unsigned, Windows shows
"Windows protected your PC" with no run button. Click **More info**,
then **Run anyway**. The publisher shows as unknown; that is expected
for an unsigned build.

**CLI:** `muxsmith.exe` is installed next to the app in
`C:\Program Files\Muxsmith`. The installer does **not** modify PATH.
To call `muxsmith` from any terminal, add that folder to your user PATH:
Settings > System > About > Advanced system settings > Environment
Variables > select `Path` under **User variables** > Edit > New >
`C:\Program Files\Muxsmith` > OK, then open a new terminal.

## macOS

Artifact: `muxsmith-<version>-macos-arm64.dmg` (Apple Silicon,
macOS 11+). There is currently no Intel build.

Open the dmg and drag **Muxsmith** to Applications.

**Gatekeeper:** the app is unsigned and not notarized.

- **macOS 15 (Sequoia) and newer:** double-click Muxsmith once; macOS
  blocks it. Open **System Settings > Privacy & Security**, scroll to
  the security section, and click **Open Anyway** next to the Muxsmith
  entry, then confirm. This is needed once.
- **macOS 11-14:** Control-click (or right-click) Muxsmith.app and
  choose **Open**, then **Open** again in the dialog. Once is enough.
- **Terminal alternative** (any version):
  `xattr -d com.apple.quarantine /Applications/Muxsmith.app`

**CLI:** the command-line tool is inside the app bundle at
`/Applications/Muxsmith.app/Contents/MacOS/muxsmith`. Nothing touches
PATH. To call it as `muxsmith`, link it once:

```sh
sudo mkdir -p /usr/local/bin && sudo ln -s /Applications/Muxsmith.app/Contents/MacOS/muxsmith /usr/local/bin/muxsmith
```

Without `sudo`: add the directory to your PATH instead by appending
`export PATH="/Applications/Muxsmith.app/Contents/MacOS:$PATH"` to your
shell profile.

## Linux

Artifacts (x86_64):

- `muxsmith-<version>-linux-x86_64.deb` - Debian/Ubuntu: `sudo apt install ./muxsmith-<version>-linux-x86_64.deb`
- `muxsmith-<version>-linux-x86_64.rpm` - Fedora & co.: `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`
- `muxsmith-<version>-linux-x86_64.AppImage` - any distro: `chmod +x` the file, then run it
- `muxsmith-<version>-linux-x86_64.tar.gz` - portable archive with both binaries; see its
  `README.txt`

No gatekeeping dialog exists on Linux. deb/rpm install both `muxsmith`
and `muxsmith-gui` to `/usr/bin` (already on PATH). The deb/rpm packages
declare **mkvtoolnix** as a recommended dependency; the AppImage and
tar.gz do not manage dependencies, so install the runtime requirements
yourself:

- **mkvtoolnix** (provides `mkvmerge`; required for every mux/dry run):
  `sudo apt install mkvtoolnix` / `sudo dnf install mkvtoolnix`
- **GUI only:** webkitgtk 4.1 and gtk3
  (`libwebkit2gtk-4.1-0` on Debian/Ubuntu, `webkit2gtk4.1` on Fedora) -
  the deb/rpm declare these as hard dependencies; the AppImage bundles
  them; for the tar.gz install them via your package manager.
