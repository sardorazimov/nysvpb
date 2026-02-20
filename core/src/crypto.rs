use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

const KEY_BYTES: [u8; 32] = [1; 32];

pub fn encrypt(data: &[u8], nonce: &[u8;12]) -> Vec<u8> {

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    cipher.encrypt(
        Nonce::from_slice(nonce),
        data
    ).unwrap()

}

pub fn decrypt(data: &[u8], nonce: &[u8;12]) -> Vec<u8> {

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&KEY_BYTES));

    cipher.decrypt(
        Nonce::from_slice(nonce),
        data
    ).unwrap()

}
