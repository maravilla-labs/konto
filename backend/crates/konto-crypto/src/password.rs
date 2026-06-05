use crate::aead;
use crate::dek::Dek;
use crate::error::{CryptoError, Result};
use argon2::{Algorithm, Argon2, Params, Version};
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

/// Argon2id parameters, persisted alongside the wrapped DEK so the same
/// derivation can be reproduced on unlock. Defaults follow OWASP guidance for
/// interactive use (64 MiB, 3 iterations, parallelism 4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdfParams {
    /// Memory cost in KiB.
    pub mem_kib: u32,
    /// Iteration (time) cost.
    pub iters: u32,
    /// Degree of parallelism.
    pub parallelism: u32,
    /// Base64-encoded random salt.
    pub salt: String,
}

impl KdfParams {
    /// Fresh parameters with a new random 16-byte salt.
    pub fn new_random() -> Result<Self> {
        let mut salt = [0u8; 16];
        rand::rngs::OsRng
            .try_fill_bytes(&mut salt)
            .map_err(|e| CryptoError::Kdf(format!("rng failure: {e}")))?;
        use base64::Engine;
        Ok(Self {
            mem_kib: 65_536,
            iters: 3,
            parallelism: 4,
            salt: base64::engine::general_purpose::STANDARD.encode(salt),
        })
    }

    fn salt_bytes(&self) -> Result<Vec<u8>> {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD
            .decode(&self.salt)
            .map_err(|e| CryptoError::Kdf(format!("bad salt: {e}")))
    }

    /// Derive the 32-byte Key-Encryption-Key (KEK) from the master password.
    fn derive_kek(&self, password: &str) -> Result<Zeroizing<[u8; 32]>> {
        let params = Params::new(self.mem_kib, self.iters, self.parallelism, Some(32))
            .map_err(|e| CryptoError::Kdf(e.to_string()))?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
        let salt = self.salt_bytes()?;
        let mut kek = Zeroizing::new([0u8; 32]);
        argon2
            .hash_password_into(password.as_bytes(), &salt, kek.as_mut())
            .map_err(|e| CryptoError::Kdf(e.to_string()))?;
        Ok(kek)
    }
}

/// Wrap (encrypt) the DEK with a password-derived KEK. Returns the sealed blob
/// to persist in the keystore. Safe to store on disk.
pub fn wrap_dek(dek: &Dek, password: &str, params: &KdfParams) -> Result<Vec<u8>> {
    let kek = params.derive_kek(password)?;
    aead::seal(&kek, dek.as_bytes())
}

/// Unwrap (decrypt) the DEK using the master password. A GCM auth failure means
/// the password was wrong, surfaced as `InvalidPassword`.
pub fn unwrap_dek(blob: &[u8], password: &str, params: &KdfParams) -> Result<Dek> {
    let kek = params.derive_kek(password)?;
    let bytes = aead::open(&kek, blob).map_err(|_| CryptoError::InvalidPassword)?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| CryptoError::InvalidKeyMaterial("unwrapped key not 32 bytes".into()))?;
    Ok(Dek::from_bytes(arr))
}
