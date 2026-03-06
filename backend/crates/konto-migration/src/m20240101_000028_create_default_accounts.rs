use sea_orm_migration::prelude::*;

use crate::m20240101_000002_create_accounting::Accounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DefaultAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DefaultAccounts::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DefaultAccounts::SettingKey)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(DefaultAccounts::AccountId).string().null())
                    .col(ColumnDef::new(DefaultAccounts::Description).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(DefaultAccounts::Table, DefaultAccounts::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        seed_defaults(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DefaultAccounts::Table).to_owned())
            .await?;
        Ok(())
    }
}

async fn seed_defaults(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let defaults = vec![
        ("ar_account", "1100", "Debitoren (Accounts Receivable)"),
        ("ap_account", "2000", "Kreditoren (Accounts Payable)"),
        ("revenue_default", "3200", "Dienstleistungsertrag (Service Revenue)"),
        ("expense_default", "4000", "Materialaufwand (Material Expense)"),
        ("vat_payable", "2200", "MWST Schuld (VAT Payable)"),
        ("vat_receivable", "1170", "Vorsteuer (Input VAT)"),
        ("bank_default", "1020", "Bank (Default Bank Account)"),
        ("cash_account", "1000", "Kasse (Cash)"),
        ("wage_expense", "5000", "Lohnaufwand (Wage Expense)"),
        ("retained_earnings", "2970", "Gewinnvortrag (Retained Earnings)"),
    ];

    for (key, account_number, description) in defaults {
        let id = uuid::Uuid::new_v4().to_string();

        // Look up account by number
        let account_id = find_account_by_number(manager, account_number).await;

        let account_val: SimpleExpr = match &account_id {
            Some(aid) => aid.clone().into(),
            None => Keyword::Null.into(),
        };

        let insert = Query::insert()
            .into_table(DefaultAccounts::Table)
            .columns([
                DefaultAccounts::Id,
                DefaultAccounts::SettingKey,
                DefaultAccounts::AccountId,
                DefaultAccounts::Description,
            ])
            .values_panic([
                id.into(),
                key.into(),
                account_val,
                description.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}

async fn find_account_by_number(
    manager: &SchemaManager<'_>,
    number: &str,
) -> Option<String> {
    let db = manager.get_connection();
    let result = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!("SELECT id FROM accounts WHERE number = {number} LIMIT 1"),
        ))
        .await
        .ok()?;
    let row = result?;
    row.try_get::<String>("", "id").ok()
}

#[derive(DeriveIden)]
pub enum DefaultAccounts {
    Table,
    Id,
    SettingKey,
    AccountId,
    Description,
}
