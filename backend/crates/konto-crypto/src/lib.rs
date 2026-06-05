//! Encryption-at-rest primitives for Maravilla Konto.
//!
//! Provides a single 32-byte Data Encryption Key (DEK) that encrypts both the
//! SQLCipher database and uploaded files. The DEK is resolved from (in order):
//! the `KONTO_MASTER_KEY` env var (server), the OS keychain (desktop), or an
//! Argon2id-wrapped blob unlocked by a master password.
//!
//! See [`KeyResolver`] for the resolution chain and password management, and
//! [`aead`] for the file/value cipher.

pub mod aead;
pub mod dek;
pub mod error;
pub mod key;
pub mod keychain;
pub mod keystore;
pub mod password;
pub mod secret;

pub use dek::Dek;
pub use error::{CryptoError, Result};
pub use key::{KeyResolver, KeyStatus, DEFAULT_SERVICE, ENV_MASTER_KEY};
pub use keychain::Keychain;
pub use keystore::{KeyMode, Keystore};
