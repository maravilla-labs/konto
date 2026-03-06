use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::{Currencies, Roles, Users};
use crate::m20240101_000002_create_accounting::{Accounts, VatRates};
use crate::m20240101_000004_create_projects::ActivityTypes;

mod seed_accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

fn now_str() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[allow(clippy::expect_used)]
fn hash_password(password: &str) -> String {
    use argon2::password_hash::rand_core::OsRng;
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = now_str();

        // Seed currencies
        for (id, code, name, symbol, primary) in [
            ("cur-chf", "CHF", "Swiss Franc", "CHF", true),
            ("cur-eur", "EUR", "Euro", "EUR", false),
            ("cur-usd", "USD", "US Dollar", "USD", false),
        ] {
            manager.exec_stmt(
                Query::insert().into_table(Currencies::Table)
                    .columns([Currencies::Id, Currencies::Code, Currencies::Name, Currencies::Symbol, Currencies::IsPrimary])
                    .values_panic([id.into(), code.into(), name.into(), symbol.into(), primary.into()])
                    .to_owned(),
            ).await?;
        }

        // Seed roles
        let admin_perms = r#"{"all":true}"#;
        let accountant_perms = r#"{"accounting":true,"contacts":true,"projects":true,"time_entries":true,"import":true}"#;
        let auditor_perms = r#"{"read_all":true}"#;
        let employee_perms = r#"{"time_entries":true,"own_data":true}"#;

        for (id, name, perms) in [
            ("role-admin", "admin", admin_perms),
            ("role-accountant", "accountant", accountant_perms),
            ("role-auditor", "auditor", auditor_perms),
            ("role-employee", "employee", employee_perms),
        ] {
            manager.exec_stmt(
                Query::insert().into_table(Roles::Table)
                    .columns([Roles::Id, Roles::Name, Roles::Permissions, Roles::CreatedAt, Roles::UpdatedAt])
                    .values_panic([id.into(), name.into(), perms.into(), now.clone().into(), now.clone().into()])
                    .to_owned(),
            ).await?;
        }

        // Seed admin user
        let pw_hash = hash_password("changeme");
        manager.exec_stmt(
            Query::insert().into_table(Users::Table)
                .columns([Users::Id, Users::Email, Users::PasswordHash, Users::FullName, Users::RoleId, Users::IsActive, Users::CreatedAt, Users::UpdatedAt])
                .values_panic(["user-admin".into(), "admin@example.com".into(), pw_hash.into(), "Administrator".into(), "role-admin".into(), true.into(), now.clone().into(), now.clone().into()])
                .to_owned(),
        ).await?;

        // Seed VAT rates
        for (id, code, name, rate) in seed_accounts::vat_rates() {
            manager.exec_stmt(
                Query::insert().into_table(VatRates::Table)
                    .columns([VatRates::Id, VatRates::Code, VatRates::Name, VatRates::Rate, VatRates::IsActive])
                    .values_panic([id.into(), code.into(), name.into(), rate.into(), true.into()])
                    .to_owned(),
            ).await?;
        }

        // Seed chart of accounts
        for (number, name, account_type) in seed_accounts::chart_of_accounts() {
            let id = format!("acct-{number}");
            manager.exec_stmt(
                Query::insert().into_table(Accounts::Table)
                    .columns([Accounts::Id, Accounts::Number, Accounts::Name, Accounts::AccountType, Accounts::IsActive, Accounts::CreatedAt, Accounts::UpdatedAt])
                    .values_panic([id.into(), number.into(), name.into(), account_type.into(), true.into(), now.clone().into(), now.clone().into()])
                    .to_owned(),
            ).await?;
        }

        // Seed activity types
        for (id, name) in seed_accounts::activity_types() {
            manager.exec_stmt(
                Query::insert().into_table(ActivityTypes::Table)
                    .columns([ActivityTypes::Id, ActivityTypes::Name, ActivityTypes::IsActive])
                    .values_panic([id.into(), name.into(), true.into()])
                    .to_owned(),
            ).await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Seed data removal handled by table drops
        Ok(())
    }
}
