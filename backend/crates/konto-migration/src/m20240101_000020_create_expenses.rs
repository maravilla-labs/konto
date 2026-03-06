use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Currencies;
use crate::m20240101_000002_create_accounting::{Accounts, JournalEntries, VatRates};
use crate::m20240101_000003_create_contacts::Contacts;
use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Expense Categories
        manager
            .create_table(
                Table::create()
                    .table(ExpenseCategories::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ExpenseCategories::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ExpenseCategories::Name).string().not_null())
                    .col(ColumnDef::new(ExpenseCategories::AccountId).string().null())
                    .col(
                        ColumnDef::new(ExpenseCategories::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(ExpenseCategories::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ExpenseCategories::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ExpenseCategories::Table, ExpenseCategories::AccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Expenses
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Expenses::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Expenses::ExpenseNumber).string().null().unique_key())
                    .col(ColumnDef::new(Expenses::ContactId).string().null())
                    .col(ColumnDef::new(Expenses::CategoryId).string().null())
                    .col(ColumnDef::new(Expenses::Description).text().not_null())
                    .col(ColumnDef::new(Expenses::Amount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Expenses::CurrencyId).string().not_null())
                    .col(ColumnDef::new(Expenses::VatRateId).string().null())
                    .col(ColumnDef::new(Expenses::VatAmount).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Expenses::Total).decimal().not_null().default(0.0))
                    .col(ColumnDef::new(Expenses::ExpenseDate).date().not_null())
                    .col(ColumnDef::new(Expenses::DueDate).date().null())
                    .col(
                        ColumnDef::new(Expenses::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(Expenses::PaymentAccountId).string().null())
                    .col(ColumnDef::new(Expenses::ReceiptUrl).string().null())
                    .col(ColumnDef::new(Expenses::ProjectId).string().null())
                    .col(ColumnDef::new(Expenses::JournalEntryId).string().null())
                    .col(ColumnDef::new(Expenses::PaymentJournalEntryId).string().null())
                    .col(ColumnDef::new(Expenses::CreatedBy).string().null())
                    .col(ColumnDef::new(Expenses::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Expenses::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::CategoryId)
                            .to(ExpenseCategories::Table, ExpenseCategories::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::CurrencyId)
                            .to(Currencies::Table, Currencies::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::VatRateId)
                            .to(VatRates::Table, VatRates::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::PaymentAccountId)
                            .to(Accounts::Table, Accounts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::JournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Expenses::Table, Expenses::PaymentJournalEntryId)
                            .to(JournalEntries::Table, JournalEntries::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Seed expense categories (with NULL account_id — user configures later)
        seed_expense_categories(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ExpenseCategories::Table).to_owned())
            .await?;
        Ok(())
    }
}

async fn seed_expense_categories(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let now = chrono::Utc::now().naive_utc().to_string();
    let categories = vec![
        ("Office Supplies", "6500"),
        ("Software & SaaS", "4400"),
        ("Travel", "4710"),
        ("Marketing", "4700"),
        ("Professional Services", "4500"),
        ("Telecommunications", "4420"),
        ("Insurance", "6220"),
        ("Miscellaneous", "4800"),
    ];

    for (name, account_code) in categories {
        let cat_id = uuid::Uuid::new_v4().to_string();
        // Try to find account by code; if not found, leave account_id NULL
        let account_id = find_account_id_by_code(manager, account_code).await;

        let mut cols = vec![
            ExpenseCategories::Id,
            ExpenseCategories::Name,
            ExpenseCategories::IsActive,
            ExpenseCategories::CreatedAt,
            ExpenseCategories::UpdatedAt,
        ];
        let mut vals: Vec<SimpleExpr> = vec![
            cat_id.into(),
            name.into(),
            true.into(),
            now.clone().into(),
            now.clone().into(),
        ];

        if let Some(ref aid) = account_id {
            cols.push(ExpenseCategories::AccountId);
            vals.push(aid.clone().into());
        }

        let insert = Query::insert()
            .into_table(ExpenseCategories::Table)
            .columns(cols)
            .values_panic(vals)
            .to_owned();
        manager.exec_stmt(insert).await?;
    }

    Ok(())
}

async fn find_account_id_by_code(
    manager: &SchemaManager<'_>,
    code: &str,
) -> Option<String> {
    let db = manager.get_connection();
    let result = db
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            format!("SELECT id FROM accounts WHERE number = {code} LIMIT 1"),
        ))
        .await
        .ok()?;
    let row = result?;
    row.try_get::<String>("", "id").ok()
}

#[derive(DeriveIden)]
pub enum ExpenseCategories {
    Table,
    Id,
    Name,
    AccountId,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Expenses {
    Table,
    Id,
    ExpenseNumber,
    ContactId,
    CategoryId,
    Description,
    Amount,
    CurrencyId,
    VatRateId,
    VatAmount,
    Total,
    ExpenseDate,
    DueDate,
    Status,
    PaymentAccountId,
    ReceiptUrl,
    ProjectId,
    JournalEntryId,
    PaymentJournalEntryId,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
