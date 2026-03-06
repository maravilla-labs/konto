use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InvoicePayment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InvoicePayment::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(InvoicePayment::InvoiceId).string().not_null())
                    .col(ColumnDef::new(InvoicePayment::Amount).decimal_len(15, 2).not_null())
                    .col(ColumnDef::new(InvoicePayment::PaymentDate).date().not_null())
                    .col(ColumnDef::new(InvoicePayment::PaymentMethod).string().null())
                    .col(ColumnDef::new(InvoicePayment::Reference).string().null())
                    .col(ColumnDef::new(InvoicePayment::BankTransactionId).string().null())
                    .col(ColumnDef::new(InvoicePayment::JournalEntryId).string().null())
                    .col(ColumnDef::new(InvoicePayment::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(InvoicePayment::Table, InvoicePayment::InvoiceId)
                            .to(Invoice::Table, Invoice::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(InvoicePayment::Table).to_owned()).await
    }
}

#[derive(Iden)]
enum InvoicePayment {
    Table,
    Id,
    InvoiceId,
    Amount,
    PaymentDate,
    PaymentMethod,
    Reference,
    BankTransactionId,
    JournalEntryId,
    CreatedAt,
}

#[derive(Iden)]
enum Invoice {
    Table,
    Id,
}
