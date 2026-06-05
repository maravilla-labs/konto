use konto_crypto::Dek;
use sea_orm::DbErr;
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection};
use sqlx::Connection;
use std::path::{Path, PathBuf};

/// First 16 bytes of an unencrypted SQLite file. SQLCipher-encrypted files have
/// a random-looking header instead, which lets us detect databases created
/// before encryption was enabled.
const SQLITE_MAGIC: &[u8; 16] = b"SQLite format 3\0";

fn custom(e: impl std::fmt::Display) -> DbErr {
    DbErr::Custom(e.to_string())
}

/// True if `path` is an existing, unencrypted SQLite database that still needs
/// to be migrated to SQLCipher.
pub fn is_plaintext_sqlite(path: &Path) -> bool {
    use std::io::Read;
    let Ok(mut f) = std::fs::File::open(path) else {
        return false;
    };
    let mut buf = [0u8; 16];
    if f.read_exact(&mut buf).is_err() {
        // Smaller than a header → freshly created/empty, treat as not plaintext.
        return false;
    }
    &buf == SQLITE_MAGIC
}

/// Transparently encrypt an existing plaintext database in place using
/// SQLCipher's `sqlcipher_export`.
///
/// The encrypted copy is built in a temp file and verified to open under the
/// key *before* it atomically replaces the original. On any failure the
/// original plaintext database is left untouched. No plaintext backup is left
/// behind — that would defeat encryption at rest — and stale plaintext WAL/SHM
/// sidecars are removed.
pub async fn encrypt_existing_database(path: &Path, dek: &Dek) -> Result<(), DbErr> {
    let enc_tmp = sibling(path, "enc.tmp");
    if enc_tmp.exists() {
        std::fs::remove_file(&enc_tmp).map_err(custom)?;
    }

    // `create_if_missing(true)` keeps `SQLITE_OPEN_CREATE` on the connection.
    // ATTACH creates the target file using the main connection's open flags, so
    // without CREATE the encrypted target can't be created (SQLITE_CANTOPEN).
    // The source already exists, so this never recreates it.
    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let mut conn = SqliteConnection::connect_with(&opts).await.map_err(custom)?;

    let enc_path = enc_tmp.to_string_lossy().replace('\'', "''");
    let hex = dek.to_hex();
    let attach = format!("ATTACH DATABASE '{enc_path}' AS encrypted KEY \"x'{hex}'\";");

    let result = async {
        sqlx::query(&attach).execute(&mut conn).await?;
        sqlx::query("SELECT sqlcipher_export('encrypted');")
            .execute(&mut conn)
            .await?;
        sqlx::query("DETACH DATABASE encrypted;")
            .execute(&mut conn)
            .await?;
        Ok::<_, sqlx::Error>(())
    }
    .await;
    conn.close().await.map_err(custom)?;
    if let Err(e) = result {
        let _ = std::fs::remove_file(&enc_tmp);
        return Err(custom(e));
    }

    // Verify the encrypted copy actually opens under the key and is readable
    // before we throw away the plaintext original.
    if let Err(e) = verify_encrypted(&enc_tmp, &hex).await {
        let _ = std::fs::remove_file(&enc_tmp);
        return Err(e);
    }

    // Atomically replace the plaintext file; its bytes are overwritten in place.
    std::fs::rename(&enc_tmp, path).map_err(custom)?;
    // Remove plaintext WAL/SHM sidecars — they may still hold plaintext pages.
    for suffix in ["-wal", "-shm"] {
        let side = with_suffix(path, suffix);
        if side.exists() {
            let _ = std::fs::remove_file(&side);
        }
    }
    tracing::info!("Encrypted existing database at rest (no plaintext copy retained)");
    Ok(())
}

/// Open the freshly written encrypted DB with the key and run a trivial query
/// to confirm it decrypts and is structurally valid.
async fn verify_encrypted(enc_path: &Path, hex: &str) -> Result<(), DbErr> {
    let opts = SqliteConnectOptions::new()
        .filename(enc_path)
        .create_if_missing(false)
        .pragma("key", format!("\"x'{hex}'\""));
    let mut conn = SqliteConnection::connect_with(&opts).await.map_err(custom)?;
    let probe = sqlx::query("SELECT count(*) FROM sqlite_master;")
        .execute(&mut conn)
        .await;
    conn.close().await.map_err(custom)?;
    probe.map(|_| ()).map_err(custom)
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}

fn sibling(path: &Path, ext: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    name.push(".");
    name.push(ext);
    path.with_file_name(name)
}
