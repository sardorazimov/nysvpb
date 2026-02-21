//! WireGuard-style packet tunnel: reads from the TUN interface, encrypts,
//! and forwards over UDP to the VPN server, and vice-versa.

use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

/// Open a UDP socket bound to an ephemeral local port and connected to `addr`.
pub async fn connect_udp(addr: SocketAddr) -> Result<UdpSocket> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.connect(addr).await?;
    Ok(sock)
}
