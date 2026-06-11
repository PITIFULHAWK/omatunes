# lavanda

A native Wayland music player written in Rust, built for [Omarchy](https://omarchy.org/) / Hyprland rices. Themed with [Catppuccin Mocha](https://github.com/catppuccin/catppuccin) and accented in lavender (`#cba6f7`).

![lavanda](https://raw.githubusercontent.com/sheep-farm/lavanda/master/assets/screenshot.png)

---

## Features

- **Audio formats** — MP3, FLAC, OGG, Opus, WAV, AAC, M4A, AIFF and more via [Symphonia](https://github.com/pdeljanov/Symphonia)
- **Folder-based library** — navigates your `~/Music` subdirectory structure as-is; no forced re-organisation
- **Incremental scanner** — only re-indexes files that changed (mtime cache); detects renames and deletions
- **Real seek** — click anywhere on the progress bar to jump
- **Dynamic volume** — slider takes effect immediately, mid-playback
- **Shuffle & repeat** — per-session, no playlist required
- **Album art** — embedded cover displayed in the player panel
- **MPRIS2** — full D-Bus integration; works with `playerctl`, Waybar `mpris` module, AGS, EWW, etc.
- **Nerd Font icons** — Font Awesome tier-1 codepoints (universal across any Nerd Font)
- **Catppuccin Mocha theme** — `BASE #11111b`, `TEXT #cdd6f4`, `ACCENT #cba6f7`

---

## Requirements

| Requirement | Notes |
|---|---|
| Rust ≥ 1.75 | `rustup` recommended |
| A Nerd Font | `JetBrainsMono Nerd Font Mono` by default; any Nerd Font works |
| PipeWire or PulseAudio | Audio output via cpal |
| D-Bus session bus | For MPRIS2 (`DBUS_SESSION_BUS_ADDRESS` must be set) |
| Wayland compositor | Tested on Hyprland; works on any wlroots compositor |

---

## Installation

### From source

```bash
git clone https://github.com/sheep-farm/lavanda
cd lavanda
cargo build --release
./target/release/lavanda
```

### With cargo install

```bash
cargo install --git https://github.com/sheep-farm/lavanda
```

---

## Music library

lavanda scans `~/Music` on startup. Subdirectories are shown as folders in the sidebar — the structure you already have is respected.

**Tag fallback hierarchy:**
- If a file has no artist tag → parent folder name is used as artist
- If a file has no album tag → immediate parent folder name is used as album
- If a file has no title tag → filename stem is used

The library database is stored at `~/.local/share/lavanda/lavanda.db`. Delete it to force a full rescan.

---

## Waybar integration

Add to your Waybar `config.jsonc`:

```jsonc
"modules-center": ["mpris"],

"mpris": {
    "format": "<span color='#c4a0f0'>{player_icon}</span>  {title} — {artist}",
    "format-paused": "<span color='#6c7086'>{player_icon}</span>  <i>{title} — {artist}</i>",
    "format-stopped": "",
    "player-icons": { "lavanda": "󰝚", "default": "󰝚" },
    "status-icons": { "paused": "󰏤", "playing": "󰐊", "stopped": "󰓛" },
    "max-length": 45,
    "on-click": "playerctl play-pause",
    "on-click-right": "playerctl next",
    "on-scroll-up": "playerctl next",
    "on-scroll-down": "playerctl previous",
    "tooltip-format": "{title}\n{artist} — {album}"
}
```

Then reload Waybar:

```bash
pkill -SIGUSR2 waybar
```

---

## playerctl

```bash
playerctl -p lavanda play-pause
playerctl -p lavanda next
playerctl -p lavanda previous
playerctl -p lavanda metadata
```

---

## Font

lavanda uses `JetBrainsMono Nerd Font Mono` by default — the same font used by Omarchy's Waybar. Any Nerd Font will work for the icons; change the family name in `src/ui/icons.rs` if you use a different one.

---

## Architecture

```
src/
├── main.rs
├── app.rs              # iced Application — state, messages, subscriptions
├── audio/
│   ├── player.rs       # symphonia decode + cpal output thread
│   ├── mpris.rs        # MPRIS2 D-Bus server (mpris-server 0.8)
│   └── spectrum.rs     # FFT analyser (unused in UI — use cava externally)
├── library/
│   ├── scanner.rs      # walkdir + lofty + mtime cache + orphan cleanup
│   ├── db.rs           # SQLite queries (rusqlite, bundled)
│   └── models.rs       # Track, Album, Artist, Playlist
└── ui/
    ├── theme.rs        # Catppuccin Mocha palette + container styles
    ├── icons.rs        # Nerd Font constants + UI font constants
    ├── views/          # library, player, playlist views
    └── components/     # progress bar, playback controls
```

---

## Status

**0.1.0-beta** — functional for daily use; rough edges remain.

Known limitations:
- Playlists UI exists but drag-and-drop population is not yet implemented
- Seek accuracy depends on the container format (Symphonia limitation)
- No gapless playback

---

## License

MIT
