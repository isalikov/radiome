# Changelog

All notable changes to this project are documented here.

## [Unreleased]

- Wait up to 20 seconds (was 5) for mpv to open its IPC socket, so a slow first
  launch of a freshly installed mpv on macOS no longer fails with
  `mpv IPC: No such file or directory`. The error now says how to retry.
- Installer checks for mpv and curl after installing and prints the install
  command when missing; warms up mpv once so the first station plays promptly.
- README: troubleshooting section for the two mpv startup errors; mise-based
  Rust setup instructions.
- `scripts/uninstall.sh` removes the installed binary and, with `--purge` or
  `RADIOME_PURGE=1`, the settings directory. Documented in the README.

## [v1.0.0] - 2026-09-07

Initial release of the keyboard-only Rust radio player.

- Browse Radio Record stations by category.
- Play, pause, stop, and switch between stations with the keyboard.
- Support F7/F8/F9 transport controls and forwarded media-key events.
- Manage favorites with persistent local settings.
- Adjust player volume independently from system volume.
- Show recent tracks and current playback metadata.
- Display a compact, audio-reactive level line.
- Provide macOS and Linux installation through the release installer.
- Include `make` targets for running, building, testing, checking, and audio integration tests.

