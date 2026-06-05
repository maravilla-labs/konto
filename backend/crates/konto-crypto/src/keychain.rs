use crate::dek::Dek;
use crate::error::{CryptoError, Result};
use keyring::Entry;

/// Thin wrapper over the OS keychain (macOS Keychain, Windows Credential
/// Manager, Linux Secret Service) for storing the raw DEK in keychain mode.
pub struct Keychain {
    service: String,
}

const DEK_ACCOUNT: &str = "db-dek";
const JWT_ACCOUNT: &str = "jwt-secret";

impl Keychain {
    pub fn new(service: impl Into<String>) -> Self {
        Self { service: service.into() }
    }

    fn entry(&self, account: &str) -> Result<Entry> {
        Entry::new(&self.service, account).map_err(|e| CryptoError::Keychain(e.to_string()))
    }

    /// Read the DEK from the keychain, if present.
    pub fn get_dek(&self) -> Result<Option<Dek>> {
        match self.entry(DEK_ACCOUNT)?.get_password() {
            Ok(hex) => Ok(Some(Dek::from_str_material(&hex)?)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CryptoError::Keychain(e.to_string())),
        }
    }

    /// Store the DEK in the keychain (hex-encoded).
    pub fn set_dek(&self, dek: &Dek) -> Result<()> {
        self.entry(DEK_ACCOUNT)?
            .set_password(&dek.to_hex())
            .map_err(|e| CryptoError::Keychain(e.to_string()))
    }

    /// Remove the DEK from the keychain (e.g. when switching to password mode).
    pub fn delete_dek(&self) -> Result<()> {
        match self.entry(DEK_ACCOUNT)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CryptoError::Keychain(e.to_string())),
        }
    }

    /// Read the JWT secret from the keychain, if present.
    pub fn get_jwt_secret(&self) -> Result<Option<String>> {
        match self.entry(JWT_ACCOUNT)?.get_password() {
            Ok(s) => Ok(Some(s)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CryptoError::Keychain(e.to_string())),
        }
    }

    /// Store the JWT secret in the keychain.
    pub fn set_jwt_secret(&self, secret: &str) -> Result<()> {
        self.entry(JWT_ACCOUNT)?
            .set_password(secret)
            .map_err(|e| CryptoError::Keychain(e.to_string()))
    }
}
