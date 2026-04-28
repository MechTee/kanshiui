# KanshiUI

KanshiUI is a small desktop GUI for managing display profiles with kanshi on wlroots-based compositors. It provides a visual canvas for arranging screens, a sidebar for per-screen settings, and an "Identify Screens" feature that shows one overlay window per connected output with the display name and connector.

## Screenshot

![KanshiUI screenshot](./screenshot.jpg)

## Features

- Visual canvas showing screen layout
- Sway support
- Hyprland support
- Mirroring
- Rotation
- Identify overlays
- Licensed under GPLv3 (see LICENSE)

## Quickstart

Requirements

- Rust + Cargo (for building from source)
- Sway or Hyprland compositor
- kanshi (optional for applying configuration)

## Build

```bash
cargo build --release
# resulting binary: target/release/KanshiUI
```

## Run

Start the GUI (dev):

```bash
cargo run --release
```
