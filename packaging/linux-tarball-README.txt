Muxsmith - portable Linux archive (x86_64)
==========================================

Contents:
  muxsmith       command-line tool
  muxsmith-gui   desktop app (Tauri/GTK)
  LICENSE        MIT license
  README.txt     this file

This archive "just runs": no installation step. Run the tools from this
directory (./muxsmith --help, ./muxsmith-gui) or put the directory on
your PATH / symlink the binaries into ~/.local/bin.

Requirements
------------
- mkvtoolnix (provides mkvmerge) - required for every dry run and mux:
    Debian/Ubuntu:  sudo apt install mkvtoolnix
    Fedora:         sudo dnf install mkvtoolnix
- For muxsmith-gui: webkitgtk 4.1 and gtk3
    Debian/Ubuntu:  libwebkit2gtk-4.1-0 (and libgtk-3-0)
    Fedora:         webkit2gtk4.1
- glibc 2.39 or newer (the version in Ubuntu 24.04, 2024): these
  binaries are built on that base, so Ubuntu 22.04 LTS (2.35) and
  Debian 12 (2.36) are below the floor. The AppImage from the same
  release is built on the same base and does not lift it; what it
  does bundle is webkitgtk and gtk3, so use it when only those are
  missing.

Docs and source: https://github.com/senolfeldmann/Muxsmith
Install notes:   https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md
