# Install Nothing – Terminal Simulator

A playful installer simulator you can run in the terminal or in the browser.
The web app renders a full‑screen terminal with themes, controls, and presets; no real installation happens.

## Features
- CLI and Web: same simulation engine; the web server streams real CLI output over WebSocket.
- Themes: macOS, Windows CMD/PowerShell, Ubuntu/Debian/Red Hat/Mint, Draco/One Dark, and more.
- Controls overlay: Play/Pause, Restart, Stop, shareable URLs (double‑click/tap to reveal, auto‑hide).
- Duration selector (HH:MM:SS) or Endless mode; live countdown in the top bar.
- Font controls: family dropdown + size selector, full‑height terminal with xterm.js FitAddon.

## Quick Start (CLI)
```bash
cargo run --release -- --all                 # run all stages (default)
cargo run --release -- bios kernel deno      # run specific stages
cargo run --release -- --cycles 1 -- bios    # run finite number of cycles
cargo run --release -- --help                # list all options
```

## Quick Start (Web)
```bash
cargo run --release --bin installer-web
# Open http://127.0.0.1:3000
```
- Pick a theme, simulator, duration (or Endless), font family/size; click Start.
- Double‑click/tap to reveal the control bar (auto‑hides after 5s).

Shareable URLs: the Share button copies a link with theme/stack/duration (and font) in query params.

## Docker
```bash
docker build -t installnothing:web .
docker run --rm -p 3000:3000 installnothing:web
# Open http://localhost:3000
```

## Deploy (Coolify)
- Type: Dockerfile, Internal Port: 3000, Domain: your domain (enable SSL).
- Env (optional): `RUST_LOG=info`.

## Repo Layout
- `src/main.rs`, `src/cli.rs`, `src/installer.rs`: core CLI app.
- `src/stages/*`: individual simulator stages.
- `src/web_server.rs`: Axum WebSocket server for the web UI.
- `web/`: single‑page web UI (xterm.js).

License: MIT‑style; see LICENSE.
