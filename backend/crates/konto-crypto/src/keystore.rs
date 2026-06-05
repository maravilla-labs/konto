use crate::error::{CryptoError, Result};
use crate::password::KdfParams;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// How the DEK is protected on this install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyMode {
    /// DEK stored raw in the OS keychain. Transparent unlock, no prompt.
    Keychain,
    /// DEK wrapped by an Argon2id-derived key from a master password.
    Password,
}

/// On-disk descriptor (`keystore.json`) telling the resolver how to obtain the
/// DEK. Contains no plaintext key material — in password mode it holds only the
/// salt/params and the *wrapped* (encrypted) DEK, which is safe at rest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystore {
    pub version: u32,
    pub mode: KeyMode,
    /// Argon2id parameters (password mode only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kdf: Option<KdfParams>,
    /// Base64 of the AES-GCM-sealed DEK (password mode only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped_dek: Option<String>,
}

impl Keystore {
    pub fn keychain() -> Self {
        Self { version: 1, mode: KeyMode::Keychain, kdf: None, wrapped_dek: None }
    }

    pub fn password(kdf: KdfParams, wrapped: &[u8]) -> Self {
        use base64::Engine;
        Self {
            version: 1,
            mode: KeyMode::Password,
            kdf: Some(kdf),
            wrapped_dek: Some(base64::engine::general_purpose::STANDARD.encode(wrapped)),
        }
    }

    /// Decode the wrapped DEK blob (password mode).
    pub fn wrapped_dek_bytes(&self) -> Result<Vec<u8>> {
        use base64::Engine;
        let s = self
            .wrapped_dek
            .as_ref()
            .ok_or_else(|| CryptoError::Keystore("missing wrapped_dek".into()))?;
        base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(|e| CryptoError::Keystore(format!("bad wrapped_dek: {e}")))
    }

    pub fn path_in(dir: &Path) -> PathBuf {
        dir.join("keystore.json")
    }

    /// Load the keystore from `dir/keystore.json`, or `None` if it doesn't exist.
    pub fn load(dir: &Path) -> Result<Option<Self>> {
        let path = Self::path_in(dir);
        match std::fs::read(&path) {
            Ok(bytes) => {
                let ks = serde_json::from_slice(&bytes)
                    .map_err(|e| CryptoError::Keystore(e.to_string()))?;
                Ok(Some(ks))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(CryptoError::Io(e)),
        }
    }

    /// Atomically persist to `dir/keystore.json`.
    pub fn save(&self, dir: &Path) -> Result<()> {
        let path = Self::path_in(dir);
        let tmp = path.with_extension("json.tmp");
        let json = serde_json::to_vec_pretty(self)
            .map_err(|e| CryptoError::Keystore(e.to_string()))?;
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, &path)?;
        Ok(())
    }
}
