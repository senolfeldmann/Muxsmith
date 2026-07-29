# Installing Muxsmith

Release downloads live on the
[GitHub releases page](https://github.com/senolfeldmann/Muxsmith/releases).
No 1.0-era build carries a developer identity - the Windows installers
are unsigned, the macOS app is ad-hoc signed (no Apple certificate, no
notarization) - so your OS will warn before the first launch. The
sections below show the one-time steps per OS. Verify
downloads against the release's `SHA256SUMS`, with the files beside it:
`sha256sum --ignore-missing -c SHA256SUMS` on Linux,
`shasum -a 256 --ignore-missing -c SHA256SUMS` on macOS, and in
PowerShell on Windows `Get-FileHash <file> -Algorithm SHA256` compared
against that file's line in `SHA256SUMS` (the comparison is
case-insensitive).

Every install ships two programs: **Muxsmith** (the GUI) and
**`muxsmith`** (the command-line tool). The AppImage is the practical
exception: it carries both binaries inside a single self-contained
file, so only the GUI is directly runnable - deb/rpm put `muxsmith` on
PATH; the tar.gz carries both binaries for you to place.

<!-- When code signing lands (registered ROADMAP trigger), the
     SmartScreen, Gatekeeper and Linux unsigned-package sections below
     shrink to the signed-app reality; keep the CLI/PATH halves. -->

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

**Gatekeeper:** the app is ad-hoc signed - no Apple developer identity,
not notarized - so macOS treats it as coming from an unidentified
developer.

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

- `muxsmith-<version>-linux-x86_64.deb` - Debian 13+ / Ubuntu 24.04+: `sudo apt install ./muxsmith-<version>-linux-x86_64.deb`
- `muxsmith-<version>-linux-x86_64.rpm` - rpm distributions with glibc 2.39+ (Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1): `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`
- `muxsmith-<version>-linux-x86_64.AppImage` - any distro with glibc 2.39+: `chmod +x` the file, then run it
- `muxsmith-<version>-linux-x86_64.tar.gz` - any distro with glibc 2.39+, portable archive with
  both binaries: see its `README.txt`

**Unsigned packages (Fedora):** the `dnf install` above prints
`Warning: skipped OpenPGP checks for 1 package from repository: @commandline`
before it proceeds. `@commandline` is dnf's own name for a package you
handed it by path, and the line says what it means: the rpm carries no
OpenPGP signature for dnf to check. That is deliberate - the same
unsigned-artifact policy the Windows and macOS sections describe - and
it is a warning, not a gatekeeping dialog: nothing blocks, nothing needs
clicking, the install completes. Check the download against `SHA256SUMS`
as above instead.

**glibc 2.39 or newer** - not a package you install but what your
distribution ships: every Linux artifact here, the AppImage included,
is built on Ubuntu 24.04 (glibc 2.39), so systems below that floor -
Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot run them.

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
