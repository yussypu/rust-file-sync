use aes_gcm::{Aes256Gcm, KeyInit}; 
use aes_gcm::aead::Aead; 
use rand_core::OsRng;
use rand::RngCore;

const KEY_BYTES: &[u8; 32] = b"an_example_very_secure_key_123!!"; // Exactly 32 bytes!

pub fn encrypt(data: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let key = KEY_BYTES;
    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("valid key length");

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    
    let ciphertext = cipher.encrypt(&nonce_bytes.into(), data)
        .expect("encryption failure!");

    (nonce_bytes.to_vec(), ciphertext)
}

pub fn decrypt(nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let key = KEY_BYTES;
    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("valid key length");

    cipher.decrypt(nonce.into(), ciphertext)
        .expect("decryption failure!")
}