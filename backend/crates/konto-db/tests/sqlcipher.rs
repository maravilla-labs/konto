use konto_crypto::Dek;
use konto_db::connection::establish_sqlite;
use konto_db::encrypt_migrate::{encrypt_existing_database, is_plaintext_sqlite};
use sea_orm::{ConnectionTrait, Statement};

fn tmp(name: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("konto-sqlcipher-test-{}-{}.db", std::process::id(), name));
    let _ = std::fs::remove_file(&p);
    p
}

async fn write_row(db: &sea_orm::DatabaseConnection, val: &str) {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        "CREATE TABLE IF NOT EXISTS t (v TEXT);".to_string(),
    ))
    .await
    .unwrap();
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("INSERT INTO t (v) VALUES ('{val}');"),
    ))
    .await
    .unwrap();
}

/// An encrypted DB must not contain its plaintext payload on disk, and must be
/// reopenable with the same key.
#[tokio::test]
async fn encrypts_at_rest_and_reopens() {
    let path = tmp("at-rest");
    let dek = Dek::from_bytes([7u8; 32]);

    let db = establish_sqlite(&path, Some(&dek)).await.unwrap();
    write_row(&db, "SECRET_MARKER_42").await;
    drop(db);

    // The marker must not be readable in the raw file bytes.
    let bytes = std::fs::read(&path).unwrap();
    assert!(
        !bytes.windows(16).any(|w| w == b"SECRET_MARKER_42"),
        "plaintext marker found in encrypted DB file"
    );
    assert!(!is_plaintext_sqlite(&path), "file should not be plaintext SQLite");

    // Reopen with the correct key and read it back.
    let db = establish_sqlite(&path, Some(&dek)).await.unwrap();
    let rows = db
        .query_all(Statement::from_string(
            db.get_database_backend(),
            "SELECT v FROM t;".to_string(),
        ))
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    drop(db);
    let _ = std::fs::remove_file(&path);
}

/// Opening an encrypted DB with the wrong key must fail.
#[tokio::test]
async fn wrong_key_fails() {
    let path = tmp("wrong-key");
    let db = establish_sqlite(&path, Some(&Dek::from_bytes([1u8; 32])))
        .await
        .unwrap();
    write_row(&db, "x").await;
    drop(db);

    let db = establish_sqlite(&path, Some(&Dek::from_bytes([2u8; 32])))
        .await
        .unwrap();
    let res = db
        .query_all(Statement::from_string(
            db.get_database_backend(),
            "SELECT v FROM t;".to_string(),
        ))
        .await;
    assert!(res.is_err(), "wrong key should not decrypt the database");
    let _ = std::fs::remove_file(&path);
}

/// A plaintext DB is detected and can be migrated to an encrypted one.
#[tokio::test]
async fn migrates_plaintext_to_encrypted() {
    let path = tmp("migrate");
    let db = establish_sqlite(&path, None).await.unwrap();
    write_row(&db, "LEGACY_DATA_99").await;
    drop(db);

    assert!(is_plaintext_sqlite(&path), "fresh unkeyed DB should be plaintext");

    let dek = Dek::from_bytes([9u8; 32]);
    encrypt_existing_database(&path, &dek).await.unwrap();

    assert!(!is_plaintext_sqlite(&path), "migrated DB should be encrypted");
    let bytes = std::fs::read(&path).unwrap();
    assert!(!bytes.windows(14).any(|w| w == b"LEGACY_DATA_99"));

    // No plaintext copy or temp file may be left behind at rest.
    for leftover in [".plaintext.bak", ".enc.tmp"] {
        let p = std::path::PathBuf::from(format!("{}{leftover}", path.display()));
        assert!(!p.exists(), "leftover plaintext artifact: {}", p.display());
    }

    let db = establish_sqlite(&path, Some(&dek)).await.unwrap();
    let rows = db
        .query_all(Statement::from_string(
            db.get_database_backend(),
            "SELECT v FROM t;".to_string(),
        ))
        .await
        .unwrap();
    assert_eq!(rows.len(), 1, "data must survive migration");
    drop(db);
    let _ = std::fs::remove_file(&path);
}
