use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PayoutEntries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayoutEntries::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PayoutEntries::PayrollRunId).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::EmployeeId).string().not_null())
                    .col(
                        ColumnDef::new(PayoutEntries::Amount)
                            .decimal_len(15, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PayoutEntries::Iban).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::Bic).string().null())
                    .col(ColumnDef::new(PayoutEntries::RecipientName).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::RecipientStreet).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::RecipientPostalCode).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::RecipientCity).string().not_null())
                    .col(
                        ColumnDef::new(PayoutEntries::RecipientCountry)
                            .string()
                            .not_null()
                            .default("CH"),
                    )
                    .col(
                        ColumnDef::new(PayoutEntries::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(PayoutEntries::PaidAt).timestamp().null())
                    .col(ColumnDef::new(PayoutEntries::PaymentReference).string().not_null())
                    .col(ColumnDef::new(PayoutEntries::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PayoutEntries::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(PayoutEntries::Table, PayoutEntries::PayrollRunId)
                            .to(PayrollRuns::Table, PayrollRuns::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PayoutEntries::Table, PayoutEntries::EmployeeId)
                            .to(Employees::Table, Employees::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PayoutEntries::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PayoutEntries {
    Table,
    Id,
    PayrollRunId,
    EmployeeId,
    Amount,
    Iban,
    Bic,
    RecipientName,
    RecipientStreet,
    RecipientPostalCode,
    RecipientCity,
    RecipientCountry,
    Status,
    PaidAt,
    PaymentReference,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PayrollRuns {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Employees {
    Table,
    Id,
}
