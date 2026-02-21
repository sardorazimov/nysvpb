//! NySVPN Tauri application library.
//!
//! Exposes Tauri commands that the React frontend calls to manage the VPN tunnel.
//! Commands communicate with the privileged daemon over the Unix socket IPC.

use serde::{Deserialize, Serialize};
use shared::{ServerInfo, TunnelStats, TunnelStatus, VpnConfig};
use tauri::Manager;

// ── Tauri commands ────────────────────────────────────────────────────────────

/// Connect to a VPN server.
#[tauri::command]
async fn vpn_connect(config: VpnConfig) -> Result<(), String> {
    let mut client = client::DaemonClient::connect()
        .await
        .map_err(|e| format!("cannot connect to daemon: {e}"))?;

    client
        .vpn_connect(config)
        .await
        .map_err(|e| e.to_string())
}

/// Disconnect from the active VPN tunnel.
#[tauri::command]
async fn vpn_disconnect() -> Result<(), String> {
    let mut client = client::DaemonClient::connect()
        .await
        .map_err(|e| format!("cannot connect to daemon: {e}"))?;

    client.vpn_disconnect().await.map_err(|e| e.to_string())
}

/// Query the current tunnel status.
#[tauri::command]
async fn vpn_status() -> Result<TunnelStatus, String> {
    let mut client = client::DaemonClient::connect()
        .await
        .map_err(|e| format!("cannot connect to daemon: {e}"))?;

    client.vpn_status().await.map_err(|e| e.to_string())
}

/// Query transfer statistics for the active tunnel.
#[tauri::command]
async fn vpn_stats() -> Result<TunnelStats, String> {
    let mut client = client::DaemonClient::connect()
        .await
        .map_err(|e| format!("cannot connect to daemon: {e}"))?;

    client.vpn_stats().await.map_err(|e| e.to_string())
}

/// Return the built-in list of available VPN servers.
#[tauri::command]
async fn list_servers() -> Result<Vec<ServerInfo>, String> {
    Ok(builtin_servers())
}

// ── App setup ─────────────────────────────────────────────────────────────────

/// Application settings persisted in the Tauri store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub kill_switch: bool,
    pub auto_connect: bool,
    pub dns_leak_protection: bool,
    pub launch_at_login: bool,
    pub protocol: String,
    pub selected_server_id: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            kill_switch: false,
            auto_connect: false,
            dns_leak_protection: true,
            launch_at_login: false,
            protocol: "wireguard".to_string(),
            selected_server_id: None,
        }
    }
}

/// Return the bundled list of VPN servers.
///
/// **Deployment note:** Replace each `public_key` field with the actual
/// WireGuard public key for your server. These placeholder values are used
/// during development only and will cause connections to fail in production.
fn builtin_servers() -> Vec<ServerInfo> {
    vec![
        ServerInfo {
            id: "us-ny-1".to_string(),
            country: "United States".to_string(),
            country_flag: "🇺🇸".to_string(),
            city: "New York".to_string(),
            address: "us-ny-1.nysvpb.example:51820".to_string(),
            public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=".to_string(),
            ping_ms: None,
        },
        ServerInfo {
            id: "de-fra-1".to_string(),
            country: "Germany".to_string(),
            country_flag: "🇩🇪".to_string(),
            city: "Frankfurt".to_string(),
            address: "de-fra-1.nysvpb.example:51820".to_string(),
            public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=".to_string(),
            ping_ms: None,
        },
        ServerInfo {
            id: "jp-tyo-1".to_string(),
            country: "Japan".to_string(),
            country_flag: "🇯🇵".to_string(),
            city: "Tokyo".to_string(),
            address: "jp-tyo-1.nysvpb.example:51820".to_string(),
            public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=".to_string(),
            ping_ms: None,
        },
        ServerInfo {
            id: "gb-lon-1".to_string(),
            country: "United Kingdom".to_string(),
            country_flag: "🇬🇧".to_string(),
            city: "London".to_string(),
            address: "gb-lon-1.nysvpb.example:51820".to_string(),
            public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=".to_string(),
            ping_ms: None,
        },
        ServerInfo {
            id: "sg-sin-1".to_string(),
            country: "Singapore".to_string(),
            country_flag: "🇸🇬".to_string(),
            city: "Singapore".to_string(),
            address: "sg-sin-1.nysvpb.example:51820".to_string(),
            public_key: "REPLACE_WITH_REAL_PUBLIC_KEY=".to_string(),
            ping_ms: None,
        },
    ]
}

/// Application entry point – called from `main.rs`.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            vpn_connect,
            vpn_disconnect,
            vpn_status,
            vpn_stats,
            list_servers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running NySVPN application");
}
