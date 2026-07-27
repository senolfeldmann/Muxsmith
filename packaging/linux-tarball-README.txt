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
- glibc from Ubuntu 22.04 (2022) or newer; any current distribution
  qualifies. If this archive does not run on your system, the AppImage
  from the same release bundles its dependencies.

Docs and source: https://github.com/senolfeldmann/Muxsmith
Install notes:   https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md
