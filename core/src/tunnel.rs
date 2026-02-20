use tokio::net::UdpSocket;
use anyhow::Result;

pub async fn connect(addr: &str) -> Result<UdpSocket> {

    let sock = UdpSocket::bind("0.0.0.0:0").await?;

    sock.connect(addr).await?;

    Ok(sock)

}
