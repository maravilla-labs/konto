pub mod local;

use async_trait::async_trait;
use konto_common::error::AppError;

#[async_trait]
pub trait StorageService: Send + Sync {
    async fn upload(&self, path: &str, data: &[u8], mime: &str) -> Result<String, AppError>;
    async fn download(&self, key: &str) -> Result<Vec<u8>, AppError>;
    async fn delete(&self, key: &str) -> Result<(), AppError>;
}
