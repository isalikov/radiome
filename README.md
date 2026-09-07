# radiome

Keyboard-only Rust radio player for [Radio Record](https://www.radiorecord.ru/).
Categories on the left, stations on the right, recent tracks below.
Cyan marks playback; magenta marks the selected category and favorites.

## Run

Requires Rust/Cargo, curl with compression support, and mpv with FFmpeg's astats filter.
Supports macOS and Linux. On macOS: `brew install rust curl mpv`.

```sh
make run
```

Plain `make` prints help. Minimum terminal size: 44 × 12. History is hidden in
short windows to leave room for stations. Truecolor terminals show the full palette;
`NO_COLOR` disables colors.

## Install

Once release archives are published, users can install with one command:

```sh
curl -fsSL https://raw.githubusercontent.com/isalikov/radiome/master/scripts/install.sh | sh
```

The script downloads the latest GitHub Release for the current macOS or Linux
architecture. If no release archive exists yet, it falls back to `cargo install`
from this repository, so the same command also works for local development
machines that already have Rust installed.

Installed binaries go to `~/.local/bin/radiome` by default. If that directory is
not in `PATH`, add it once in your shell profile.

## Keyboard

| Key | Action |
| --- | --- |
| Tab / Shift+Tab | Next / previous category, wrapping around |
| ↑ / ↓, j / k | Select a station without changing playback |
| Enter | Play selected station |
| F7 / F9 | Play previous / next station in the current category |
| F8 / Space | Play / pause |
| - / = | Player volume, 5% steps, 0–100% |
| f | Toggle favorite |
| i | Show / hide recent tracks |
| PgUp / PgDn, Home / End | Scroll stations |
| r | Refresh stations and history |
| s | Stop |
| ? / Esc | Open / close help |
| q / Ctrl+C | Quit |

Previous/next follows the playing station when it is in the current category;
otherwise it starts at the selected station. Unavailable streams are skipped.
Function transport keys also work while help is open.
Left/right arrows have no action. History is read-only. There is no search.

Favorites have a small magenta `+` outside the Favorites category. Inside Favorites,
the category itself identifies them, so the marker is hidden. Station and track
names retain their original spelling from the API; application labels are English.

Terminal F7/F8/F9 and forwarded media-key events are supported. On macOS, once
playback has started, mpv's system media controls route previous/next to Rust and
play/pause to the audio engine. This uses mpv's
[RemoteCommandCenter integration](https://github.com/mpv-player/mpv/blob/v0.41.0/osdep/mac/remote_command_center.swift).
If macOS handles your top-row keys as system controls, Fn+F7/F8/F9 sends ordinary
function keys to the terminal. No global keyboard interception is installed.

## Audio and settings

The thin line follows the current decoded audio level at 20 Hz. Its scale adapts
to the last four seconds of the station's loudness, leaving headroom for beats.
Attack and release are smoothed; there is no scrolling or synthetic animation.
Silence, pause and buffering settle the line. Physical stroke thickness is
determined by the terminal font, not an exact pixel size.

The UI and application logic use Rust, Ratatui and Crossterm. Audio uses an
external mpv process with private IPC; curl fetches API data on worker threads.
Volume is mpv's software gain and does not change system volume.

Favorites and volume are saved atomically in `~/.config/radiome/settings.json`.
Directory precedence: `RADIOME_CONFIG_DIR`, `$XDG_CONFIG_HOME/radiome`,
`~/.config/radiome`. Corrupt settings are reported rather than overwritten.

History and current track metadata refresh every 15 seconds. The API may lag
behind audio, especially after pausing. `RADIOME_BASE_URL` overrides the API
base URL for development. Requests time out after 15 seconds.
Stream priority: `stream_320`, `stream_hls`, `stream_128`, `stream_64`.

## Verify

```sh
make build
make test
make check
make test-audio
```

The audio test uses real mpv with a sine wave and silent output. It checks
audio levels, software volume, media play/pause bindings, and previous/next
delivery over IPC. Physical hardware key routing still depends on macOS and
which media session is active.
