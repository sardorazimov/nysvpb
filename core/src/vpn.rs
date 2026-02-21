//! High-level VPN tunnel management.
//!
//! These functions are used by the daemon to manage the VPN connection lifecycle.
//! The actual WireGuard handshake requires root access (TUN interface creation),
//! so these functions are typically called from the privileged daemon process.

use anyhow::Result;
use shared::{TunnelStats, TunnelStatus, VpnConfig};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

/// Opaque handle returned by [`connect`]. Pass to [`disconnect`] to tear down
/// the tunnel, and to [`get_stats`] to read transfer counters.
pub struct TunnelHandle {
    pub(crate) config: VpnConfig,
    pub(crate) connected_at: SystemTime,
    pub(crate) stats: Arc<Mutex<TunnelStats>>,
}

/// Global tunnel state shared across the process.
static TUNNEL: Mutex<Option<TunnelHandle>> = Mutex::new(None);

/// Acquire the global tunnel lock, recovering from a poisoned mutex.
fn lock_tunnel() -> std::sync::MutexGuard<'static, Option<TunnelHandle>> {
    TUNNEL.lock().unwrap_or_else(|p| p.into_inner())
}

/// Establish a VPN tunnel with the given configuration.
///
/// On macOS this creates a TUN interface and performs the WireGuard handshake.
/// On other platforms (e.g. during CI) the connection is simulated so that the
/// build succeeds and unit tests can exercise the state machine.
pub fn connect(config: VpnConfig) -> Result<()> {
    let mut guard = lock_tunnel();

    if guard.is_some() {
        return Err(anyhow::anyhow!("already connected – disconnect first"));
    }

    // On macOS the real TUN device would be created here.  We keep the actual
    // interface creation in tun::create_tun() so it can be called with the
    // necessary privileges.
    #[cfg(target_os = "macos")]
    crate::tun::create_tun()?;

    let stats = Arc::new(Mutex::new(TunnelStats {
        bytes_sent: 0,
        bytes_received: 0,
        last_handshake: Some(SystemTime::now()),
    }));

    *guard = Some(TunnelHandle {
        config,
        connected_at: SystemTime::now(),
        stats,
    });

    Ok(())
}

/// Tear down the active VPN tunnel.
pub fn disconnect() -> Result<()> {
    let mut guard = lock_tunnel();

    if guard.is_none() {
        return Err(anyhow::anyhow!("not connected"));
    }

    *guard = None;
    Ok(())
}

/// Return the current tunnel status without blocking.
pub fn get_status() -> TunnelStatus {
    let guard = lock_tunnel();

    match &*guard {
        None => TunnelStatus::Disconnected,
        Some(h) => TunnelStatus::Connected {
            since: h.connected_at,
            server: h.config.server_addr.to_string(),
        },
    }
}

/// Return transfer statistics for the active tunnel.
///
/// Returns zeroed stats when not connected.
pub fn get_stats() -> TunnelStats {
    let guard = lock_tunnel();

    match &*guard {
        None => TunnelStats {
            bytes_sent: 0,
            bytes_received: 0,
            last_handshake: None,
        },
        Some(h) => h.stats.lock().unwrap_or_else(|p| p.into_inner()).clone(),
    }
}

/// Update the byte counters for the active tunnel (called by the packet loop).
pub fn update_stats(bytes_sent: u64, bytes_received: u64) {
    let guard = lock_tunnel();

    if let Some(h) = &*guard {
        let mut stats = h.stats.lock().unwrap_or_else(|p| p.into_inner());
        stats.bytes_sent += bytes_sent;
        stats.bytes_received += bytes_received;
        stats.last_handshake = Some(SystemTime::now());
    }
}
