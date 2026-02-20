use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

use tokio::net::UdpSocket;

const KEY_BYTES: [u8; 32] = [1; 32];

#[tokio::main]
async fn main() {

    let sock = UdpSocket::bind("0.0.0.0:51820").await.unwrap();

    println!("Server started");

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    let mut buf = [0u8; 2000];

    loop {

        let (len, addr) = sock.recv_from(&mut buf).await.unwrap();

        let nonce = Nonce::from_slice(&buf[..12]);

        let encrypted = &buf[12..len];

        let decrypted = cipher.decrypt(nonce, encrypted).unwrap();

        println!("Received from {}: {:?}", addr, decrypted);

    }

}
