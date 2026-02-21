# NySVPN

A complete macOS VPN desktop application built with Rust and Tauri v2.

---

## Architecture

```
nysvpb/
├── shared/          # Shared types: VpnConfig, TunnelStatus, TunnelStats, …
├── core/            # VPN tunnel management (TUN device, encryption, packet loop)
├── daemon/          # Privileged background service (Unix socket IPC server)
├── client/          # IPC client library (used by CLI and Tauri backend)
├── cli/             # Command-line interface (nysvpb connect / disconnect / …)
├── server/          # VPN relay server
├── gui/             # Tauri v2 + React + TypeScript desktop app
│   ├── src/         # React frontend (screens: Main, Servers, Settings)
│   └── src-tauri/   # Rust Tauri backend (Tauri commands bridging GUI ↔ daemon)
└── macos/           # macOS-specific files (LaunchDaemon plist, Info.plist)
```

### Communication flow

```
GUI (React)  ──invoke──▶  Tauri backend (Rust)  ──Unix socket──▶  Daemon (root)
                                                                      │
CLI  ─────────────────────────────────────────────Unix socket──▶  Daemon (root)
                                                                      │
                                                              Core VPN logic
                                                           (TUN device, crypto)
```

The **daemon** runs as root (via launchd) and is the only process that needs
elevated privileges — it manages the TUN network interface.  The GUI and CLI
talk to it over a Unix domain socket at `/tmp/nysvpb-daemon.sock`.

---

## Build Instructions

### Prerequisites

| Tool       | Version   | Notes                    |
|------------|-----------|--------------------------|
| Rust       | ≥ 1.85    | Install via [rustup](https://rustup.rs) |
| Node.js    | ≥ 18      | Required for the GUI     |
| npm        | ≥ 9       | Bundled with Node.js     |
| macOS      | ≥ 13 Ventura | Target platform     |

### Quick install (macOS)

```bash
chmod +x install.sh
./install.sh
```

This script:
1. Checks for Rust and Node.js
2. Builds the Rust workspace
3. Installs frontend dependencies and builds the Tauri app
4. Copies the daemon binary and launchd plist (requires sudo)
5. Loads the daemon with `launchctl`
6. Installs the `.app` bundle to `/Applications/`

### Manual build

```bash
# Build all Rust crates
cargo build --workspace --release

# Build the GUI
cd gui
npm install
npm run tauri build
```

---

## Usage

### GUI

Open **NySVPN.app** from `/Applications/` or via Launchpad.

### CLI

```bash
# Connect to a server
nysvpb connect \
  --server 203.0.113.1:51820 \
  --pubkey <server-public-key-base64> \
  --privkey <your-private-key-base64> \
  --ip 10.0.0.2

# Check status
nysvpb status

# Show transfer stats
nysvpb stats

# Disconnect
nysvpb disconnect
```

The CLI requires the daemon to be running.  If you installed via `install.sh`
the daemon starts automatically.  To start it manually:

```bash
sudo nysvpb-daemon
```

---

## macOS Permissions Setup

### Network Extension entitlement

The app ships with `entitlements.plist` that requests
`com.apple.developer.networking.networkextension` → `packet-tunnel-provider`.

For distribution outside the Mac App Store you need:
1. An Apple Developer account
2. A provisioning profile with the Network Extension capability
3. Code-sign the `.app` with your Developer ID certificate

To sign locally during development:
```bash
codesign --force --sign "Developer ID Application: Your Name (TEAMID)" \
  --entitlements gui/src-tauri/entitlements.plist \
  /Applications/NySVPN.app
```

### TUN interface (root / daemon)

Creating TUN interfaces requires root.  The daemon runs as root via launchd:

```bash
# Load the daemon
sudo launchctl load /Library/LaunchDaemons/com.nysvpb.daemon.plist

# Check it is running
sudo launchctl list | grep nysvpb

# Unload (stop)
sudo launchctl unload /Library/LaunchDaemons/com.nysvpb.daemon.plist
```

---

## Development

```bash
# Run a hot-reloading dev server
cd gui
npm run tauri dev

# Lint Rust code
cargo clippy --workspace

# Run tests
cargo test --workspace
```

---

## License

MIT
