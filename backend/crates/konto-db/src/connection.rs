use konto_crypto::Dek;
use sea_orm::{DatabaseConnection, DbErr, SqlxSqliteConnector};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::Path;

const MAX_CONNECTIONS: u32 = 100;
const MIN_CONNECTIONS: u32 = 5;

/// Connect to a database by URL (used for non-SQLite backends such as
/// PostgreSQL, where at-rest encryption is handled by the server/infra, and for
/// in-memory test databases). No SQLCipher key is applied.
pub async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    use sea_orm::{ConnectOptions, Database};
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .sqlx_logging(false);
    Database::connect(opt).await
}

/// Connect to a SQLite database file, optionally encrypted at rest with
/// SQLCipher. When `key` is `Some`, the raw 32-byte DEK is applied via the
/// `key` PRAGMA (which sqlx guarantees runs before any other statement), so the
/// entire database file is AES-256 encrypted on disk.
pub async fn establish_sqlite(
    path: &Path,
    key: Option<&Dek>,
) -> Result<DatabaseConnection, DbErr> {
    let mut opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    if let Some(dek) = key {
        // Raw-key form `"x'<hex>'"` uses the 32 bytes directly (no PBKDF2 over a
        // passphrase) since the DEK is already a full-entropy random key. The
        // value must be a quoted string literal so `PRAGMA key = "x'..'"` parses.
        opts = opts.pragma("key", format!("\"x'{}'\"", dek.to_hex()));
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .min_connections(MIN_CONNECTIONS)
        .connect_with(opts)
        .await
        .map_err(|e| DbErr::Conn(sea_orm::RuntimeErr::SqlxError(e)))?;

    Ok(SqlxSqliteConnector::from_sqlx_sqlite_pool(pool))
}
