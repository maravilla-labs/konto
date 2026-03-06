use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BankTransactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BankTransactions::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BankTransactions::BankAccountId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BankTransactions::TransactionDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BankTransactions::ValueDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BankTransactions::Amount)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(BankTransactions::CurrencyId).string())
                    .col(
                        ColumnDef::new(BankTransactions::Description)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BankTransactions::CounterpartyName).string())
                    .col(ColumnDef::new(BankTransactions::CounterpartyIban).string())
                    .col(ColumnDef::new(BankTransactions::Reference).string())
                    .col(ColumnDef::new(BankTransactions::BankReference).string())
                    .col(
                        ColumnDef::new(BankTransactions::Status)
                            .string()
                            .not_null()
                            .default("unmatched"),
                    )
                    .col(ColumnDef::new(BankTransactions::MatchedInvoiceId).string())
                    .col(ColumnDef::new(BankTransactions::MatchedExpenseId).string())
                    .col(
                        ColumnDef::new(BankTransactions::MatchedJournalEntryId)
                            .string(),
                    )
                    .col(ColumnDef::new(BankTransactions::ImportBatchId).string())
                    .col(
                        ColumnDef::new(BankTransactions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BankTransactions::Table, BankTransactions::BankAccountId)
                            .to(BankAccounts::Table, BankAccounts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bank_tx_account_status")
                    .table(BankTransactions::Table)
                    .col(BankTransactions::BankAccountId)
                    .col(BankTransactions::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BankTransactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum BankTransactions {
    Table,
    Id,
    BankAccountId,
    TransactionDate,
    ValueDate,
    Amount,
    CurrencyId,
    Description,
    CounterpartyName,
    CounterpartyIban,
    Reference,
    BankReference,
    Status,
    MatchedInvoiceId,
    MatchedExpenseId,
    MatchedJournalEntryId,
    ImportBatchId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum BankAccounts {
    Table,
    Id,
}
