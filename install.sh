#!/bin/bash
# install.sh — NySVPN build and install script (macOS)
#
# Usage:
#   chmod +x install.sh
#   ./install.sh
#
# What this script does:
#   1. Checks / installs required tools (Rust, Node.js, Tauri CLI)
#   2. Builds all Rust workspace crates (cargo build --workspace --release)
#   3. Builds the Tauri GUI app (npm run tauri build)
#   4. Installs the daemon binary and launchd plist
#   5. Copies the .app bundle to /Applications/

set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_NAME="NySVPN"
DAEMON_BINARY="nysvpb-daemon"
CLI_BINARY="nysvpb"

echo "======================================================"
echo " NySVPN Installer"
echo "======================================================"

# ── 1. Check / install Rust ───────────────────────────────────────────────────
if ! command -v cargo &>/dev/null; then
  echo "→ Installing Rust toolchain via rustup…"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
  source "$HOME/.cargo/env"
else
  echo "✓ Rust $(rustc --version)"
fi

# ── 2. Check / install Node.js ────────────────────────────────────────────────
if ! command -v node &>/dev/null; then
  echo "→ Node.js not found. Please install Node.js 18+ from https://nodejs.org"
  exit 1
else
  echo "✓ Node.js $(node --version)"
fi

if ! command -v npm &>/dev/null; then
  echo "✗ npm not found. Please install npm."
  exit 1
fi

# ── 3. Build Rust workspace ───────────────────────────────────────────────────
echo ""
echo "→ Building Rust workspace…"
cd "$REPO_DIR"
cargo build --workspace --release
echo "✓ Rust workspace built"

# ── 4. Install Node dependencies and build Tauri app ─────────────────────────
echo ""
echo "→ Installing frontend dependencies…"
cd "$REPO_DIR/gui"
npm install

echo "→ Building Tauri application…"
npm run tauri build
echo "✓ Tauri application built"

# ── 5. Install daemon (requires sudo) ─────────────────────────────────────────
echo ""
echo "→ Installing daemon binary (requires sudo)…"

DAEMON_SRC="$REPO_DIR/target/release/$DAEMON_BINARY"
DAEMON_DST="/usr/local/bin/$DAEMON_BINARY"

if [ -f "$DAEMON_SRC" ]; then
  sudo install -m 755 "$DAEMON_SRC" "$DAEMON_DST"
  echo "✓ Daemon installed to $DAEMON_DST"
else
  echo "✗ Daemon binary not found at $DAEMON_SRC"
  exit 1
fi

# Install CLI binary
CLI_SRC="$REPO_DIR/target/release/$CLI_BINARY"
CLI_DST="/usr/local/bin/$CLI_BINARY"
if [ -f "$CLI_SRC" ]; then
  sudo install -m 755 "$CLI_SRC" "$CLI_DST"
  echo "✓ CLI installed to $CLI_DST"
fi

# ── 6. Install LaunchDaemon plist ─────────────────────────────────────────────
PLIST_SRC="$REPO_DIR/macos/com.nysvpb.daemon.plist"
PLIST_DST="/Library/LaunchDaemons/com.nysvpb.daemon.plist"

if [ -f "$PLIST_SRC" ]; then
  echo "→ Installing LaunchDaemon plist…"
  sudo install -m 644 "$PLIST_SRC" "$PLIST_DST"
  sudo launchctl load -w "$PLIST_DST" 2>/dev/null || true
  echo "✓ LaunchDaemon installed and loaded"
fi

# ── 7. Copy .app bundle to /Applications ─────────────────────────────────────
APP_SRC=$(find "$REPO_DIR/gui/src-tauri/target/release/bundle/macos" -name "*.app" 2>/dev/null | head -1)

if [ -n "$APP_SRC" ]; then
  echo "→ Installing ${APP_NAME}.app to /Applications…"
  sudo cp -Rf "$APP_SRC" "/Applications/"
  echo "✓ ${APP_NAME}.app installed to /Applications/"
else
  echo "⚠ .app bundle not found (expected in gui/src-tauri/target/release/bundle/macos/)"
fi

echo ""
echo "======================================================"
echo " Installation complete!"
echo ""
echo " Run:  nysvpb status"
echo " Or open NySVPN from /Applications/${APP_NAME}.app"
echo "======================================================"
