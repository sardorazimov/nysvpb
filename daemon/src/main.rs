use nysvpn_core::crypto;

use nysvpn_core::tun;

use tokio::net::UdpSocket;

use rand_core::{ OsRng, RngCore };

use std::io::Read;

#[tokio::main]
async fn main() {
    println!("NySVPN daemon started");

    // TUN device
    let mut dev = tun::create_tun().expect("failed to create tun");

    // UDP socket with explicit type
    let sock: UdpSocket = UdpSocket::bind("127.0.0.1:0").await.expect("failed to bind");

    sock.connect("127.0.0.1:51820").await.expect("failed to connect");

    // explicit nonce type
    let mut nonce: [u8; 12] = [0u8; 12];

    loop {
        // explicit buffer type
        let mut buf: [u8; 1500] = [0u8; 1500];

        // explicit len type
        let len: usize = dev.read(&mut buf).expect("tun read failed");

        // fill nonce
        OsRng.fill_bytes(&mut nonce);

        // encrypt
        let encrypted: Vec<u8> = crypto::encrypt(&buf[..len], &nonce);

        // explicit packet type
        let mut packet: Vec<u8> = Vec::new();

        packet.extend_from_slice(&nonce);
        packet.extend_from_slice(&encrypted);

        // send
        let _sent: usize = sock.send(&packet).await.expect("send failed");

        println!("sent packet");
    }
}
