use crate::error::{CryptoError, Result};
use rand::TryRngCore;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// The 32-byte Data Encryption Key (DEK).
///
/// This single key encrypts both the SQLCipher database and uploaded files.
/// It is held in memory only, zeroized on drop, and never written to disk in
/// plaintext: on disk it lives either in the OS keychain (keychain mode) or
/// wrapped by an Argon2id-derived key (password mode).
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Dek([u8; 32]);

impl Dek {
    /// Generate a fresh random DEK from the OS CSPRNG.
    pub fn generate() -> Result<Self> {
        let mut bytes = [0u8; 32];
        rand::rngs::OsRng
            .try_fill_bytes(&mut bytes)
            .map_err(|e| CryptoError::InvalidKeyMaterial(format!("rng failure: {e}")))?;
        Ok(Self(bytes))
    }

    /// Construct from raw 32 bytes.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Parse a DEK from a user/env-supplied string: 64 hex chars or base64 of 32 bytes.
    pub fn from_str_material(s: &str) -> Result<Self> {
        let s = s.trim();
        let bytes = if s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            hex::decode(s).map_err(|e| CryptoError::InvalidKeyMaterial(e.to_string()))?
        } else {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| CryptoError::InvalidKeyMaterial(e.to_string()))?
        };
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|_| CryptoError::InvalidKeyMaterial("key must be 32 bytes".into()))?;
        Ok(Self(arr))
    }

    /// Raw key bytes. Keep the borrow short-lived.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Lowercase hex encoding (used for the SQLCipher raw-key PRAGMA).
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}
