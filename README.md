# radiome

Minimal Rust radio player for [Radio Record](https://www.radiorecord.ru/).

This is a first working scaffold:

- loads stations from the public Radio Record API
- shows a simple terminal UI
- plays a chosen station through `mpv`

## Requirements

- `curl`
- `mpv`

## Run

```bash
cargo run
```

## Commands

- `help`
- `list`
- `search <text>`
- `clear`
- `play [n|text]`
- `stop`
- `now`
- `refresh`
- `up`
- `down`
- `quit`

The player prefers `stream_320`, then `stream_hls`, then `stream_128`, then `stream_64`.
