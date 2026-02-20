use tokio::net::{UdpSocket, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

const KEY_BYTES: [u8; 32] = [1; 32];

#[tokio::main]
async fn main() {

    println!("NySVPN full forward server started");

    // VPN socket
    let vpn_socket: UdpSocket =
        UdpSocket::bind("0.0.0.0:51820")
        .await
        .expect("bind failed");

    // cipher
    let cipher =
        ChaCha20Poly1305::new(
            Key::from_slice(&KEY_BYTES)
        );

    let mut buf: [u8; 2000] = [0; 2000];

    loop {

        // receive VPN packet
        let (len, client_addr) =
            vpn_socket.recv_from(&mut buf)
            .await
            .expect("recv failed");

        if len < 12 {
            continue;
        }

        // split nonce and data
        let mut nonce_array: [u8; 12] = [0; 12];
        nonce_array.copy_from_slice(&buf[0..12]);

        let encrypted =
            &buf[12..len];

        // decrypt packet
        let decrypted =
            match cipher.decrypt(
                Nonce::from_slice(&nonce_array),
                encrypted
            ) {

                Ok(data) => data,

                Err(_) => {
                    println!("decrypt failed");
                    continue;
                }

            };

        println!(
            "VPN packet received: {} bytes",
            decrypted.len()
        );

        // connect to internet (example.com test)
        let mut internet =
            match TcpStream::connect("example.com:80")
            .await {

                Ok(s) => s,

                Err(e) => {
                    println!("internet connect failed: {}", e);
                    continue;
                }

            };

        // send HTTP request
        let request =
            b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";

        if internet.write_all(request).await.is_err() {
            continue;
        }

        // read response
        let mut response =
            vec![0u8; 4096];

        let size =
            match internet.read(&mut response).await {

                Ok(s) => s,

                Err(_) => continue,

            };

        println!(
            "Internet response: {} bytes",
            size
        );

        // encrypt response
        let encrypted_response =
            cipher.encrypt(
                Nonce::from_slice(&nonce_array),
                &response[..size]
            ).unwrap();

        // send back to client
        let mut packet =
            Vec::new();

        packet.extend_from_slice(&nonce_array);

        packet.extend_from_slice(&encrypted_response);

        vpn_socket.send_to(
            &packet,
            client_addr
        ).await.unwrap();

    }

}