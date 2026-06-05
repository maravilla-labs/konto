use crate::error::Result;
use crate::keychain::Keychain;
use rand::distr::{Alphanumeric, SampleString};
use std::path::Path;

/// Resolve the JWT signing secret for desktop mode, keeping it out of plaintext
/// files. Order: keychain → migrate a legacy `secret.key` file into the keychain
/// (then delete it) → generate a fresh secret and store it.
///
/// On the server, `JWT_SECRET` is supplied via env and this is not used.
pub fn resolve_jwt_secret(keychain: &Keychain, legacy_path: &Path) -> Result<String> {
    if let Some(secret) = keychain.get_jwt_secret()? {
        if !secret.trim().is_empty() {
            return Ok(secret);
        }
    }

    // Migrate an existing plaintext secret.key into the keychain, then remove it.
    if let Ok(existing) = std::fs::read_to_string(legacy_path) {
        let existing = existing.trim().to_string();
        if !existing.is_empty() {
            keychain.set_jwt_secret(&existing)?;
            let _ = std::fs::remove_file(legacy_path);
            tracing::info!("Migrated JWT secret from secret.key into keychain");
            return Ok(existing);
        }
    }

    let secret = Alphanumeric.sample_string(&mut rand::rng(), 64);
    keychain.set_jwt_secret(&secret)?;
    tracing::info!("Generated new JWT secret in keychain");
    Ok(secret)
}
