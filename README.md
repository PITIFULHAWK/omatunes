# lavanda

A native Wayland music player written in Rust, built for [Omarchy](https://omarchy.org/) / Hyprland rices. Follows the active Omarchy theme automatically — colors update live when you switch themes.

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
- **Live Omarchy theming** — reads `~/.config/omarchy/current/theme.name` and updates the palette within 3 seconds of a theme switch; no restart required

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

## Configuration

lavanda generates `~/.config/lavanda/config.toml` on first run with all options commented and set to their defaults. Edit it with any text editor; changes take effect on next launch.

```toml
# ~/.config/lavanda/config.toml

# Path to your music library. Subdirectories are shown as folders in the sidebar.
music_dir = "~/Music"

# Initial volume (0.0 = mute, 1.0 = 100%)
volume = 0.8

# Start the session with shuffle enabled
shuffle = false

# Start the session with repeat enabled
repeat = false

# Interface language. Options: "auto", "en", "pt_BR", "es"
# "auto" detects from $LANG
language = "auto"

# Seek step in seconds for the ← → arrow keys
seek_step = 5

# Volume delta per + / - keypress
volume_step = 0.05
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

## Omarchy theming

lavanda reads the active Omarchy theme from `~/.config/omarchy/current/theme.name` and maps its `colors.toml` to the UI palette:

| `colors.toml` key | lavanda role |
|---|---|
| `background` | window background |
| `foreground` | primary text |
| `accent` | accent color (highlights, active elements) |
| `color8` | muted/overlay color; also used to derive surface shades |
| `color1`–`color4` | red / green / yellow / blue |
| `color15` | subtext |

Works with all built-in Omarchy themes (Catppuccin, Nord, Gruvbox, Tokyo Night, Rose Pinè, etc.) and custom user themes in `~/.config/omarchy/themes/`.

### Waybar integration

For the Waybar `mpris` module to also follow the theme, add an Omarchy `theme-set` hook at `~/.config/omarchy/hooks/theme-set` that regenerates `~/.config/waybar/colors.css` and sends `SIGUSR2` to Waybar. An example hook is shown below — adapt it to the CSS variable names your `style.css` uses:

```bash
#!/bin/bash
THEME_NAME="$1"
COLORS_FILE="$HOME/.config/omarchy/themes/$THEME_NAME/colors.toml"
[ -f "$COLORS_FILE" ] || COLORS_FILE="$HOME/.local/share/omarchy/themes/$THEME_NAME/colors.toml"
[ -f "$COLORS_FILE" ] || exit 0

get_color() { grep -E "^$1\s*=" "$COLORS_FILE" | grep -oE '[0-9a-fA-F]{6}' | head -1; }

BG=$(get_color background); FG=$(get_color foreground); ACCENT=$(get_color accent)
# ... generate your colors.css ...
pkill -SIGUSR2 waybar 2>/dev/null
```

Style the module via CSS classes — avoid hardcoded Pango colors in `format`:

```jsonc
"mpris": {
    "format": "{player_icon}  {title} — {artist}",
    "format-paused": "{player_icon}  {title} — {artist}",
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

```css
/* style.css */
#mpris         { color: @ACCENT; }
#mpris.paused  { color: @GRAY0; font-style: italic; }
```

---

## Keybindings

These work when the lavanda window is focused.

| Key | Action |
|---|---|
| `Space` | play / pause |
| `→` / `←` | seek +5s / −5s |
| `n` / `p` | next / previous track |
| `s` | toggle shuffle |
| `r` | toggle repeat |
| `+` or `=` | volume +5% |
| `-` | volume −5% |

For system-wide controls (lavanda running in background), wire `playerctl` to your compositor. Example for Hyprland:

```ini
# hyprland.conf
bind = SUPER, F5, exec, playerctl play-pause
bind = SUPER, F6, exec, playerctl previous
bind = SUPER, F7, exec, playerctl next
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
    ├── theme.rs        # Omarchy theme reader + live palette + container styles
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
