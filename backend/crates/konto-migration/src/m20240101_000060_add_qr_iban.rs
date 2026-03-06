use sea_orm_migration::prelude::*;

use crate::m20240101_000010_create_invoices::Invoices;
use crate::m20240101_000011_create_settings::BankAccounts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum BankAccountsExtra {
    QrIban,
}

#[derive(DeriveIden)]
enum InvoicesExtra {
    BankAccountId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add qr_iban to bank_accounts (QR-IBAN for QRR references, IID 30000-31999)
        manager
            .alter_table(
                Table::alter()
                    .table(BankAccounts::Table)
                    .add_column(ColumnDef::new(BankAccountsExtra::QrIban).string().null())
                    .to_owned(),
            )
            .await?;

        // Add bank_account_id to invoices (override default bank for payment slip)
        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .add_column(
                        ColumnDef::new(InvoicesExtra::BankAccountId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BankAccounts::Table)
                    .drop_column(BankAccountsExtra::QrIban)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Invoices::Table)
                    .drop_column(InvoicesExtra::BankAccountId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
