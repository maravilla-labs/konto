use async_trait::async_trait;
use konto_common::error::AppError;
use std::path::PathBuf;
use tokio::fs;

pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self { base_dir: base_dir.into() }
    }
}

#[async_trait]
impl super::StorageService for LocalStorage {
    async fn upload(&self, path: &str, data: &[u8], _mime: &str) -> Result<String, AppError> {
        let full_path = self.base_dir.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to create dir: {e}")))?;
        }
        fs::write(&full_path, data)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to write file: {e}")))?;
        Ok(path.to_string())
    }

    async fn download(&self, key: &str) -> Result<Vec<u8>, AppError> {
        let full_path = self.base_dir.join(key);
        fs::read(&full_path)
            .await
            .map_err(|e| AppError::NotFound(format!("File not found: {e}")))
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        let full_path = self.base_dir.join(key);
        if full_path.exists() {
            fs::remove_file(&full_path)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to delete: {e}")))?;
        }
        Ok(())
    }
}
