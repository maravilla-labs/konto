use thiserror::Error;

/// Errors from the encryption-at-rest layer.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// The active keystore is in password mode but no password was supplied.
    #[error("master password required")]
    PasswordRequired,

    /// The supplied master password did not decrypt the data key.
    #[error("invalid master password")]
    InvalidPassword,

    /// A master key was provided via env/keystore but could not be parsed.
    #[error("invalid key material: {0}")]
    InvalidKeyMaterial(String),

    /// OS keychain access failed.
    #[error("keychain error: {0}")]
    Keychain(String),

    /// AES-GCM encrypt/decrypt failure (wrong key or tampered ciphertext).
    #[error("cipher error: {0}")]
    Cipher(String),

    /// Argon2 key derivation failure.
    #[error("key derivation error: {0}")]
    Kdf(String),

    /// keystore.json read/write/parse failure.
    #[error("keystore error: {0}")]
    Keystore(String),

    /// Underlying I/O failure.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CryptoError>;
