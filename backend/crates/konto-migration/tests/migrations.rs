//! Migration-runner regression tests.
//!
//! These run the real migrator against in-memory SQLite. The cases here exist
//! because of the v0.1.0-beta.7 boot crash: `m20240101_000089` added
//! `projects.currency_id`, then aborted on a project whose free-text currency
//! was the symbol `€`. SQLite kept the committed `ALTER TABLE` but sea-orm never
//! recorded the migration, so every later launch retried it and died with
//! `duplicate column name: currency_id` before the app could start.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use konto_migration::Migrator;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use sea_orm_migration::MigratorTrait;

const CURRENCY_FK: &str = "m20240101_000089_alter_projects_currency_fk";
const FX_SEED: &str = "m20240101_000091_seed_fx_default_accounts";

async fn connect() -> DatabaseConnection {
    Database::connect("sqlite::memory:").await.unwrap()
}

/// `Migrator::up` takes a *count* of migrations to apply, not a version, and the
/// list index does not track the `m20240101_0000NN_` numbering. Resolve the
/// count that stops immediately before the named migration.
fn steps_before(name: &str) -> u32 {
    let idx = Migrator::migrations()
        .iter()
        .position(|m| m.name().starts_with(name))
        .unwrap_or_else(|| panic!("no migration named {name}"));
    u32::try_from(idx).unwrap()
}

async fn exec(db: &DatabaseConnection, sql: &str) {
    db.execute(Statement::from_string(DatabaseBackend::Sqlite, sql.to_owned()))
        .await
        .unwrap_or_else(|e| panic!("failed: {sql}\n{e}"));
}

async fn scalar(db: &DatabaseConnection, sql: &str) -> Option<String> {
    db.query_one(Statement::from_string(DatabaseBackend::Sqlite, sql.to_owned()))
        .await
        .unwrap()
        .and_then(|row| row.try_get::<Option<String>>("", "v").ok().flatten())
}

async fn has_column(db: &DatabaseConnection, table: &str, column: &str) -> bool {
    scalar(
        db,
        &format!("SELECT name AS v FROM pragma_table_info('{table}') WHERE name = '{column}'"),
    )
    .await
    .is_some()
}

/// The baseline: a clean database migrates all the way up.
#[tokio::test]
async fn migrates_from_empty() {
    let db = connect().await;
    Migrator::up(&db, None).await.expect("fresh migration run");

    assert!(has_column(&db, "projects", "currency_id").await);
    assert!(!has_column(&db, "projects", "currency").await);
}

/// Migrations must be re-runnable over their own partial output. SQLite commits
/// DDL as each statement executes and sea-orm only records a migration once `up`
/// returns `Ok`, so an abort mid-migration leaves the finished statements behind
/// and the whole migration is retried on the next boot.
#[tokio::test]
async fn recovers_from_half_applied_currency_migration() {
    let db = connect().await;

    // Stop just before the migration that half-applied in beta.7.
    Migrator::up(&db, Some(steps_before(CURRENCY_FK)))
        .await
        .expect("migrate up to the currency migration");
    exec(
        &db,
        "INSERT INTO projects (id, name, status, currency, created_at, updated_at)
         VALUES ('p1', 'Half applied', 'active', 'CHF', '2026-01-01', '2026-01-01')",
    )
    .await;

    // Reproduce exactly the state the crashing installs were left in: the column
    // exists and is backfilled, but the migration was never recorded.
    exec(&db, "ALTER TABLE projects ADD COLUMN currency_id TEXT NULL").await;
    exec(
        &db,
        "UPDATE projects SET currency_id = (SELECT id FROM currencies WHERE code = 'CHF')",
    )
    .await;

    Migrator::up(&db, None)
        .await
        .expect("must recover from a half-applied 089 instead of `duplicate column name`");

    assert!(!has_column(&db, "projects", "currency").await);
    assert_eq!(
        scalar(&db, "SELECT currency_id AS v FROM projects WHERE id = 'p1'").await,
        scalar(&db, "SELECT id AS v FROM currencies WHERE code = 'CHF'").await,
    );
}

/// The other half of the beta.7 failure: a currency stored as a symbol rather
/// than an ISO code. It must map to the right currency, and — critically — must
/// never abort the migration run, because a user whose app cannot boot has no
/// way to correct the data.
#[tokio::test]
async fn maps_symbols_and_never_aborts_on_unknown_currency() {
    let db = connect().await;
    Migrator::up(&db, Some(steps_before(CURRENCY_FK)))
        .await
        .expect("migrate up to the currency migration");

    for (id, currency) in [
        ("p-symbol", "€"),
        ("p-lowercase", "chf"),
        ("p-padded", " USD "),
        ("p-garbage", "Schweizer Franken"),
    ] {
        exec(
            &db,
            &format!(
                "INSERT INTO projects (id, name, status, currency, created_at, updated_at)
                 VALUES ('{id}', '{id}', 'active', '{currency}', '2026-01-01', '2026-01-01')"
            ),
        )
        .await;
    }

    Migrator::up(&db, None)
        .await
        .expect("an unmappable currency must not fail the migration run");

    let code_of = |id: &'static str| {
        let db = &db;
        async move {
            scalar(
                db,
                &format!(
                    "SELECT c.code AS v FROM projects p
                     LEFT JOIN currencies c ON c.id = p.currency_id
                     WHERE p.id = '{id}'"
                ),
            )
            .await
        }
    };

    assert_eq!(code_of("p-symbol").await.as_deref(), Some("EUR"));
    assert_eq!(code_of("p-lowercase").await.as_deref(), Some("CHF"));
    assert_eq!(code_of("p-padded").await.as_deref(), Some("USD"));
    // Unmappable: left unset for the user to fix, not guessed at.
    assert_eq!(code_of("p-garbage").await, None);
}

/// Seed migrations must not double-insert when replayed after a partial run.
#[tokio::test]
async fn fx_default_accounts_seed_is_idempotent() {
    let db = connect().await;
    Migrator::up(&db, Some(steps_before(FX_SEED)))
        .await
        .expect("migrate up to the FX seed migration");

    // A previous aborted run got one of the two rows in.
    exec(
        &db,
        "INSERT INTO default_accounts (id, setting_key, description)
         VALUES ('leftover', 'fx_gain_account', 'partial run')",
    )
    .await;

    Migrator::up(&db, None).await.expect("replay seed");

    assert_eq!(
        scalar(
            &db,
            "SELECT COUNT(*) || '' AS v FROM default_accounts
             WHERE setting_key IN ('fx_gain_account', 'fx_loss_account')"
        )
        .await
        .as_deref(),
        Some("2"),
    );
}
