Muxsmith __VERSION__ - unsigned builds; read the install note for your OS
before first launch: [Windows](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#windows)
| [macOS](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#macos)
| [Linux](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#linux)

**Runtime requirement:** Muxsmith drives `mkvmerge` from
[MKVToolNix](https://mkvtoolnix.download/). The deb/rpm packages declare
it as a recommended dependency; on Windows/macOS install it yourself
(details in the install notes).

| Artifact | For |
|---|---|
| `muxsmith-__VERSION__-windows-x86_64.msi` | Windows 10/11, Intel/AMD |
| `muxsmith-__VERSION__-windows-arm64.msi` | Windows 11 on ARM |
| `muxsmith-__VERSION__-macos-arm64.dmg` | macOS 11+, Apple Silicon |
| `muxsmith-__VERSION__-linux-x86_64.deb` | Debian/Ubuntu |
| `muxsmith-__VERSION__-linux-x86_64.rpm` | Fedora & co. |
| `muxsmith-__VERSION__-linux-x86_64.AppImage` | any Linux distro |
| `muxsmith-__VERSION__-linux-x86_64.tar.gz` | portable, CLI + GUI |

Verify downloads: put `SHA256SUMS` beside the files and run
`sha256sum -c SHA256SUMS`.

---
