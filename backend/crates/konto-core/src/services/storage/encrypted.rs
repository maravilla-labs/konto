use async_trait::async_trait;
use konto_common::error::AppError;
use konto_crypto::{aead, Dek};
use std::path::Path;
use std::sync::Arc;

use super::StorageService;

/// Storage decorator that encrypts every payload at rest with AES-256-GCM using
/// the same DEK as the database. Wraps any inner [`StorageService`] (local FS,
/// S3, …) so the encryption is transparent to callers.
///
/// On download, content that fails to authenticate is assumed to be a legacy
/// plaintext file written before encryption was enabled and is returned as-is,
/// so existing uploads keep working. Use [`encrypt_existing_dir`] to convert
/// them eagerly.
pub struct EncryptedStorage {
    inner: Arc<dyn StorageService>,
    dek: Dek,
}

impl EncryptedStorage {
    pub fn new(inner: Arc<dyn StorageService>, dek: Dek) -> Self {
        Self { inner, dek }
    }
}

#[async_trait]
impl StorageService for EncryptedStorage {
    async fn upload(&self, path: &str, data: &[u8], mime: &str) -> Result<String, AppError> {
        let sealed = aead::seal(self.dek.as_bytes(), data)
            .map_err(|e| AppError::Internal(format!("encrypt upload: {e}")))?;
        self.inner.upload(path, &sealed, mime).await
    }

    async fn download(&self, key: &str) -> Result<Vec<u8>, AppError> {
        let blob = self.inner.download(key).await?;
        match aead::open(self.dek.as_bytes(), &blob) {
            Ok(plain) => Ok(plain),
            // Legacy plaintext file (pre-encryption): return unchanged.
            Err(_) => Ok(blob),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        self.inner.delete(key).await
    }
}

/// One-time eager migration: walk a local uploads directory and encrypt any
/// file that isn't already an AES-GCM blob under `dek`. Idempotent — already
/// encrypted files authenticate and are skipped.
pub fn encrypt_existing_dir(base_dir: &Path, dek: &Dek) -> std::io::Result<usize> {
    let mut count = 0;
    if !base_dir.exists() {
        return Ok(0);
    }
    let mut stack = vec![base_dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let bytes = std::fs::read(&path)?;
            // Already encrypted? Skip.
            if aead::open(dek.as_bytes(), &bytes).is_ok() {
                continue;
            }
            match aead::seal(dek.as_bytes(), &bytes) {
                Ok(sealed) => {
                    let tmp = path.with_extension("enc.tmp");
                    std::fs::write(&tmp, &sealed)?;
                    std::fs::rename(&tmp, &path)?;
                    count += 1;
                }
                Err(e) => {
                    tracing::error!("Failed to encrypt upload {}: {e}", path.display());
                }
            }
        }
    }
    if count > 0 {
        tracing::info!("Encrypted {count} existing upload file(s) at rest");
    }
    Ok(count)
}
