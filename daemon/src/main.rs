//! NySVPN privileged background daemon.
//!
//! Listens on a Unix domain socket at [`shared::SOCKET_PATH`] and handles
//! [`VpnCommand`] messages from the client (CLI / GUI Tauri backend).
//!
//! On macOS this process is installed as a LaunchDaemon so it runs as root,
//! which is required to create TUN network interfaces.

use anyhow::Result;
use shared::{VpnCommand, VpnResponse, SOCKET_PATH};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> Result<()> {
    println!("NySVPN daemon starting – socket: {SOCKET_PATH}");

    // Remove stale socket file if present.
    if Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }

    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("NySVPN daemon ready");

    loop {
        let (stream, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                eprintln!("client handler error: {e}");
            }
        });
    }
}

/// Handle a single client connection: read newline-delimited JSON commands,
/// dispatch them, and write back a JSON response.
async fn handle_client(stream: tokio::net::UnixStream) -> Result<()> {
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<VpnCommand>(&line) {
            Ok(cmd) => dispatch(cmd),
            Err(e) => VpnResponse::Error(format!("parse error: {e}")),
        };

        let mut json = serde_json::to_string(&response)?;
        json.push('\n');
        write_half.write_all(json.as_bytes()).await?;
    }

    Ok(())
}

/// Execute a [`VpnCommand`] and return the appropriate [`VpnResponse`].
fn dispatch(cmd: VpnCommand) -> VpnResponse {
    match cmd {
        VpnCommand::Connect(config) => match nysvpn_core::vpn::connect(config) {
            Ok(()) => VpnResponse::Ok,
            Err(e) => VpnResponse::Error(e.to_string()),
        },
        VpnCommand::Disconnect => match nysvpn_core::vpn::disconnect() {
            Ok(()) => VpnResponse::Ok,
            Err(e) => VpnResponse::Error(e.to_string()),
        },
        VpnCommand::GetStatus => VpnResponse::Status(nysvpn_core::vpn::get_status()),
        VpnCommand::GetStats => VpnResponse::Stats(nysvpn_core::vpn::get_stats()),
    }
}

