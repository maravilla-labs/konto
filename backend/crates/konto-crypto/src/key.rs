use crate::dek::Dek;
use crate::error::{CryptoError, Result};
use crate::keychain::Keychain;
use crate::keystore::{KeyMode, Keystore};
use crate::password::{self, KdfParams};
use std::path::{Path, PathBuf};

/// Env var that supplies the DEK directly (hex or base64) on headless/server
/// deployments. When set it takes precedence over keychain/keystore, mirroring
/// how `DATABASE_URL`/`JWT_SECRET` are injected.
pub const ENV_MASTER_KEY: &str = "KONTO_MASTER_KEY";

/// Keychain service name (namespaces all keychain entries for this app).
pub const DEFAULT_SERVICE: &str = "maravilla-konto";

/// What the app needs to do at boot to obtain the DEK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyStatus {
    /// DEK comes from `KONTO_MASTER_KEY` — unlock is automatic (server).
    Env,
    /// No keystore yet — first run will generate one transparently.
    Uninitialized,
    /// Keychain mode — unlock is automatic (desktop).
    Keychain,
    /// Password mode — a master password must be supplied to unlock.
    PasswordLocked,
}

/// Resolves and manages the DEK across env, OS keychain, and password modes.
/// The resolution chain is: `KONTO_MASTER_KEY` → keystore.json (keychain or
/// password) → first-run generate-into-keychain.
pub struct KeyResolver {
    service: String,
    dir: PathBuf,
}

impl KeyResolver {
    /// `dir` is the app data directory holding `keystore.json`.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { service: DEFAULT_SERVICE.to_string(), dir: dir.into() }
    }

    pub fn with_service(mut self, service: impl Into<String>) -> Self {
        self.service = service.into();
        self
    }

    fn keychain(&self) -> Keychain {
        Keychain::new(self.service.clone())
    }

    fn env_key() -> Option<String> {
        std::env::var(ENV_MASTER_KEY).ok().filter(|v| !v.trim().is_empty())
    }

    /// Determine what unlock action boot needs, without unlocking.
    pub fn status(&self) -> Result<KeyStatus> {
        if Self::env_key().is_some() {
            return Ok(KeyStatus::Env);
        }
        match Keystore::load(&self.dir)? {
            None => Ok(KeyStatus::Uninitialized),
            Some(ks) => match ks.mode {
                KeyMode::Keychain => Ok(KeyStatus::Keychain),
                KeyMode::Password => Ok(KeyStatus::PasswordLocked),
            },
        }
    }

    /// Obtain the DEK. `password` is only consulted in password mode; pass
    /// `None` for env/keychain/first-run.
    pub fn resolve(&self, password: Option<&str>) -> Result<Dek> {
        if let Some(material) = Self::env_key() {
            return Dek::from_str_material(&material);
        }
        match Keystore::load(&self.dir)? {
            None => self.first_run(),
            Some(ks) => match ks.mode {
                KeyMode::Keychain => self
                    .keychain()
                    .get_dek()?
                    .ok_or_else(|| CryptoError::Keychain("DEK missing from keychain".into())),
                KeyMode::Password => {
                    let password = password.ok_or(CryptoError::PasswordRequired)?;
                    let kdf = ks
                        .kdf
                        .as_ref()
                        .ok_or_else(|| CryptoError::Keystore("missing kdf params".into()))?;
                    password::unwrap_dek(&ks.wrapped_dek_bytes()?, password, kdf)
                }
            },
        }
    }

    /// First-run: generate a DEK, store it in the keychain, persist a
    /// keychain-mode keystore.
    fn first_run(&self) -> Result<Dek> {
        let dek = Dek::generate()?;
        self.keychain().set_dek(&dek)?;
        Keystore::keychain().save(&self.dir)?;
        tracing::info!("Encryption initialized (keychain mode)");
        Ok(dek)
    }

    /// Switch from keychain mode to password mode. Reads the current DEK from
    /// the keychain, wraps it under the new password, and removes the keychain
    /// copy so the password becomes mandatory.
    pub fn enable_password(&self, new_password: &str) -> Result<()> {
        let dek = self
            .keychain()
            .get_dek()?
            .ok_or_else(|| CryptoError::Keychain("DEK missing from keychain".into()))?;
        let kdf = KdfParams::new_random()?;
        let wrapped = password::wrap_dek(&dek, new_password, &kdf)?;
        Keystore::password(kdf, &wrapped).save(&self.dir)?;
        self.keychain().delete_dek()?;
        tracing::info!("Master password enabled");
        Ok(())
    }

    /// Re-wrap the DEK under a new password (password mode only).
    pub fn change_password(&self, old_password: &str, new_password: &str) -> Result<()> {
        let ks = Keystore::load(&self.dir)?
            .filter(|k| k.mode == KeyMode::Password)
            .ok_or_else(|| CryptoError::Keystore("not in password mode".into()))?;
        let kdf = ks
            .kdf
            .as_ref()
            .ok_or_else(|| CryptoError::Keystore("missing kdf params".into()))?;
        let dek = password::unwrap_dek(&ks.wrapped_dek_bytes()?, old_password, kdf)?;
        let new_kdf = KdfParams::new_random()?;
        let wrapped = password::wrap_dek(&dek, new_password, &new_kdf)?;
        Keystore::password(new_kdf, &wrapped).save(&self.dir)?;
        tracing::info!("Master password changed");
        Ok(())
    }

    /// Switch from password mode back to keychain mode (requires the password).
    pub fn disable_password(&self, password: &str) -> Result<()> {
        let ks = Keystore::load(&self.dir)?
            .filter(|k| k.mode == KeyMode::Password)
            .ok_or_else(|| CryptoError::Keystore("not in password mode".into()))?;
        let kdf = ks
            .kdf
            .as_ref()
            .ok_or_else(|| CryptoError::Keystore("missing kdf params".into()))?;
        let dek = password::unwrap_dek(&ks.wrapped_dek_bytes()?, password, kdf)?;
        self.keychain().set_dek(&dek)?;
        Keystore::keychain().save(&self.dir)?;
        tracing::info!("Master password disabled (keychain mode)");
        Ok(())
    }

    pub fn data_dir(&self) -> &Path {
        &self.dir
    }
}
