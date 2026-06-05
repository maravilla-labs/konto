use crate::error::{CryptoError, Result};
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};

/// AES-256-GCM seal: returns `nonce(12) || ciphertext || tag(16)`.
///
/// A fresh random nonce is generated per call, so the same key may safely
/// encrypt many payloads. Used both to wrap the DEK (password mode) and to
/// encrypt uploaded files.
pub fn seal(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| CryptoError::Cipher(e.to_string()))?;
    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// AES-256-GCM open of a `seal()` blob. Authentication failure (wrong key or
/// tampered data) returns `CryptoError::Cipher`.
pub fn open(key: &[u8; 32], blob: &[u8]) -> Result<Vec<u8>> {
    if blob.len() < 12 + 16 {
        return Err(CryptoError::Cipher("ciphertext too short".into()));
    }
    let (nonce_bytes, ciphertext) = blob.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|e| CryptoError::Cipher(e.to_string()))
}
