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
