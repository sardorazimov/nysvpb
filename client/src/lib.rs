//! NySVPN IPC client library.
//!
//! Connects to the daemon Unix socket and sends [`VpnCommand`] messages,
//! returning the deserialized [`VpnResponse`].  Used by both the CLI and
//! the Tauri GUI backend.

use anyhow::Result;
use shared::{TunnelStats, TunnelStatus, VpnCommand, VpnConfig, VpnResponse, SOCKET_PATH};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// A connected client session to the NySVPN daemon.
pub struct DaemonClient {
    stream: UnixStream,
}

impl DaemonClient {
    /// Connect to the daemon at [`SOCKET_PATH`].
    pub async fn connect() -> Result<Self> {
        let stream = UnixStream::connect(SOCKET_PATH).await?;
        Ok(Self { stream })
    }

    /// Send a command and wait for the daemon's response.
    pub async fn send(&mut self, cmd: VpnCommand) -> Result<VpnResponse> {
        let mut json = serde_json::to_string(&cmd)?;
        json.push('\n');

        let (read_half, mut write_half) = self.stream.split();
        write_half.write_all(json.as_bytes()).await?;

        let mut reader = BufReader::new(read_half);
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        let response = serde_json::from_str::<VpnResponse>(line.trim())?;
        Ok(response)
    }

    /// Ask the daemon to connect with the given configuration.
    pub async fn vpn_connect(&mut self, config: VpnConfig) -> Result<()> {
        match self.send(VpnCommand::Connect(config)).await? {
            VpnResponse::Ok => Ok(()),
            VpnResponse::Error(e) => Err(anyhow::anyhow!(e)),
            other => Err(anyhow::anyhow!("unexpected response: {other:?}")),
        }
    }

    /// Ask the daemon to disconnect.
    pub async fn vpn_disconnect(&mut self) -> Result<()> {
        match self.send(VpnCommand::Disconnect).await? {
            VpnResponse::Ok => Ok(()),
            VpnResponse::Error(e) => Err(anyhow::anyhow!(e)),
            other => Err(anyhow::anyhow!("unexpected response: {other:?}")),
        }
    }

    /// Query the current tunnel status.
    pub async fn vpn_status(&mut self) -> Result<TunnelStatus> {
        match self.send(VpnCommand::GetStatus).await? {
            VpnResponse::Status(s) => Ok(s),
            VpnResponse::Error(e) => Err(anyhow::anyhow!(e)),
            other => Err(anyhow::anyhow!("unexpected response: {other:?}")),
        }
    }

    /// Query transfer statistics.
    pub async fn vpn_stats(&mut self) -> Result<TunnelStats> {
        match self.send(VpnCommand::GetStats).await? {
            VpnResponse::Stats(s) => Ok(s),
            VpnResponse::Error(e) => Err(anyhow::anyhow!(e)),
            other => Err(anyhow::anyhow!("unexpected response: {other:?}")),
        }
    }
}
