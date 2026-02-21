use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::time::SystemTime;

/// Configuration for a VPN tunnel connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    /// Remote VPN server address (IP:port).
    pub server_addr: SocketAddr,
    /// Server's WireGuard public key (base64).
    pub server_public_key: String,
    /// Client's WireGuard private key (base64).
    pub client_private_key: String,
    /// Assigned VPN tunnel IP for this client.
    pub client_ip: IpAddr,
    /// DNS servers to use inside the tunnel.
    pub dns_servers: Vec<IpAddr>,
    /// CIDR ranges routed through the tunnel, e.g. ["0.0.0.0/0"].
    pub allowed_ips: Vec<String>,
}

/// Commands sent from clients (CLI / GUI) to the daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VpnCommand {
    Connect(VpnConfig),
    Disconnect,
    GetStatus,
    GetStats,
}

/// Response envelope sent by the daemon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VpnResponse {
    Ok,
    Status(TunnelStatus),
    Stats(TunnelStats),
    Error(String),
}

/// Current state of the VPN tunnel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TunnelStatus {
    Disconnected,
    Connecting,
    Connected {
        /// Wall-clock time when the connection was established.
        since: SystemTime,
        /// Server address of the active connection.
        server: String,
    },
    Error(String),
}

/// Network transfer statistics for the active tunnel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_handshake: Option<SystemTime>,
}

/// Metadata for a VPN server shown in the server-list UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub id: String,
    pub country: String,
    pub country_flag: String,
    pub city: String,
    pub address: String,
    pub public_key: String,
    /// Round-trip latency in milliseconds (None if not yet measured).
    pub ping_ms: Option<u32>,
}

/// Path to the Unix domain socket used for daemon IPC.
pub const SOCKET_PATH: &str = "/tmp/nysvpb-daemon.sock";

