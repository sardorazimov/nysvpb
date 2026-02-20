use chacha20poly1305::{
    ChaCha20Poly1305,
    Key,
    Nonce,
    aead::{Aead, KeyInit}
};

const KEY_BYTES: [u8; 32] = [1; 32];

pub fn encrypt(data: &[u8], nonce_bytes: &[u8; 12]) -> Vec<u8> {

    let key = Key::from_slice(&KEY_BYTES);

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.encrypt(nonce, data).unwrap()
}

pub fn decrypt(data: &[u8], nonce_bytes: &[u8; 12]) -> Vec<u8> {

    let key = Key::from_slice(&KEY_BYTES);

    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, data).unwrap()
}