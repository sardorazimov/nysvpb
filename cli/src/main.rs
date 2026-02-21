//! NySVPN command-line interface.
//!
//! Usage:
//!   nysvpb connect --server <addr> --pubkey <key> --privkey <key> --ip <ip>
//!   nysvpb disconnect
//!   nysvpb status
//!   nysvpb stats

use anyhow::Result;
use clap::{Parser, Subcommand};
use client::DaemonClient;
use shared::{TunnelStatus, VpnConfig};
use std::net::{IpAddr, SocketAddr};

#[derive(Parser)]
#[command(name = "nysvpb", about = "NySVPN command-line interface", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to a VPN server.
    Connect {
        /// Server address (IP:port), e.g. 203.0.113.1:51820
        #[arg(long)]
        server: SocketAddr,

        /// Server WireGuard public key (base64)
        #[arg(long)]
        pubkey: String,

        /// Client WireGuard private key (base64)
        #[arg(long)]
        privkey: String,

        /// Assigned VPN client IP address
        #[arg(long)]
        ip: IpAddr,

        /// DNS servers (comma-separated), default: 1.1.1.1
        #[arg(long, default_value = "1.1.1.1")]
        dns: String,
    },

    /// Disconnect from the VPN.
    Disconnect,

    /// Show current connection status.
    Status,

    /// Show transfer statistics.
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut client = DaemonClient::connect().await.map_err(|e| {
        anyhow::anyhow!(
            "Cannot connect to daemon at {}: {e}\n\
             Make sure the daemon is running: sudo nysvpb-daemon",
            shared::SOCKET_PATH
        )
    })?;

    match cli.command {
        Commands::Connect {
            server,
            pubkey,
            privkey,
            ip,
            dns,
        } => {
            let dns_servers: Vec<IpAddr> = dns
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            let config = VpnConfig {
                server_addr: server,
                server_public_key: pubkey,
                client_private_key: privkey,
                client_ip: ip,
                dns_servers,
                allowed_ips: vec!["0.0.0.0/0".to_string()],
            };

            client.vpn_connect(config).await?;
            println!("Connected to {server}");
        }

        Commands::Disconnect => {
            client.vpn_disconnect().await?;
            println!("Disconnected");
        }

        Commands::Status => {
            let status = client.vpn_status().await?;
            match status {
                TunnelStatus::Disconnected => println!("Status: Disconnected"),
                TunnelStatus::Connecting => println!("Status: Connecting…"),
                TunnelStatus::Connected { server, since } => {
                    let elapsed = since
                        .elapsed()
                        .map(|d| format_duration(d))
                        .unwrap_or_else(|_| "unknown".to_string());
                    println!("Status: Connected to {server} (uptime {elapsed})");
                }
                TunnelStatus::Error(e) => println!("Status: Error – {e}"),
            }
        }

        Commands::Stats => {
            let stats = client.vpn_stats().await?;
            println!(
                "↑ Sent:     {} bytes\n↓ Received: {} bytes",
                stats.bytes_sent, stats.bytes_received
            );
            if let Some(ts) = stats.last_handshake {
                let ago = ts
                    .elapsed()
                    .map(|d| format!("{}s ago", d.as_secs()))
                    .unwrap_or_else(|_| "unknown".to_string());
                println!("Last handshake: {ago}");
            }
        }
    }

    Ok(())
}

/// Format a [`std::time::Duration`] as `hh:mm:ss`.
fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

