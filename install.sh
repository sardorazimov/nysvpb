#!/bin/bash

set -e

echo "Creating NysVPN workspace..."

mkdir -p nysvpn
cd nysvpn

# Workspace Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "core",
    "daemon",
    "client",
    "server",
    "shared",
    "cli"
]
resolver = "2"
EOF

# Create crates
cargo new core --lib
cargo new shared --lib
cargo new daemon
cargo new client
cargo new server
cargo new cli

################################
# CORE
################################

cat > core/src/lib.rs << 'EOF'
pub mod crypto;
pub mod tunnel;
EOF

cat > core/src/crypto.rs << 'EOF'
use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

const KEY_BYTES: [u8; 32] = [1; 32];

pub fn encrypt(data: &[u8], nonce: &[u8;12]) -> Vec<u8> {

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    cipher.encrypt(
        Nonce::from_slice(nonce),
        data
    ).unwrap()

}

pub fn decrypt(data: &[u8], nonce: &[u8;12]) -> Vec<u8> {

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    cipher.decrypt(
        Nonce::from_slice(nonce),
        data
    ).unwrap()

}
EOF

cat > core/src/tunnel.rs << 'EOF'
use tokio::net::UdpSocket;
use anyhow::Result;

pub async fn connect(addr: &str) -> Result<UdpSocket> {

    let sock = UdpSocket::bind("0.0.0.0:0").await?;

    sock.connect(addr).await?;

    Ok(sock)

}
EOF

cat >> core/Cargo.toml << 'EOF'

tokio = { version = "1", features = ["full"] }
anyhow = "1"
chacha20poly1305 = "0.10"
EOF

################################
# DAEMON
################################

cat > daemon/src/main.rs << 'EOF'
use nysvpn_core::crypto;
use nysvpn_core::tunnel;
use rand_core::{OsRng, RngCore};

#[tokio::main]
async fn main() {

    println!("NysVPN daemon started");

    let sock = tunnel::connect("127.0.0.1:51820").await.unwrap();

    let mut nonce = [0u8; 12];

    loop {

        OsRng.fill_bytes(&mut nonce);

        let data = b"vpn packet";

        let encrypted = crypto::encrypt(data, &nonce);

        let mut packet = Vec::new();

        packet.extend_from_slice(&nonce);
        packet.extend_from_slice(&encrypted);

        sock.send(&packet).await.unwrap();

        println!("Packet sent");

        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    }

}
EOF

cat >> daemon/Cargo.toml << 'EOF'

nysvpn-core = { path = "../core" }
tokio = { version = "1", features = ["full"] }
rand_core = "0.6"
EOF

################################
# SERVER
################################

cat > server/src/main.rs << 'EOF'
use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

use tokio::net::UdpSocket;

const KEY_BYTES: [u8; 32] = [1; 32];

#[tokio::main]
async fn main() {

    let sock = UdpSocket::bind("0.0.0.0:51820").await.unwrap();

    println!("Server started");

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    let mut buf = [0u8; 2000];

    loop {

        let (len, addr) = sock.recv_from(&mut buf).await.unwrap();

        let nonce = Nonce::from_slice(&buf[..12]);

        let encrypted = &buf[12..len];

        let decrypted = cipher.decrypt(nonce, encrypted).unwrap();

        println!("Received from {}: {:?}", addr, decrypted);

    }

}
EOF

cat >> server/Cargo.toml << 'EOF'

tokio = { version = "1", features = ["full"] }
chacha20poly1305 = "0.10"
EOF

################################
# CLIENT
################################

echo 'fn main(){println!("NysVPN client");}' > client/src/main.rs

################################
# CLI
################################

echo 'fn main(){println!("NysVPN CLI");}' > cli/src/main.rs

################################
# SHARED
################################

echo 'pub fn init(){}' > shared/src/lib.rs

################################
# GUI
################################

echo "Creating Tauri GUI..."

npm create tauri-app@latest gui <<EOF
NysVPN
Vanilla
JavaScript
EOF

echo "DONE"
echo "Run:"
echo "cd nysvpn"
echo "cargo build"
echo "cargo run -p server"
echo "cargo run -p daemon"