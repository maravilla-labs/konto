use sea_orm_migration::prelude::*;

use crate::m20240101_000003_create_contacts::Contacts;
use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecurringInvoices::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RecurringInvoices::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RecurringInvoices::ContactId).string().not_null())
                    .col(ColumnDef::new(RecurringInvoices::ProjectId).string().null())
                    .col(ColumnDef::new(RecurringInvoices::TemplateData).text().not_null())
                    .col(ColumnDef::new(RecurringInvoices::Frequency).string().not_null())
                    .col(ColumnDef::new(RecurringInvoices::IntervalDays).integer().null())
                    .col(ColumnDef::new(RecurringInvoices::NextRunDate).date().not_null())
                    .col(ColumnDef::new(RecurringInvoices::EndDate).date().null())
                    .col(
                        ColumnDef::new(RecurringInvoices::AutoSend)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(RecurringInvoices::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(RecurringInvoices::LastGeneratedAt).timestamp().null())
                    .col(ColumnDef::new(RecurringInvoices::CreatedBy).string().null())
                    .col(ColumnDef::new(RecurringInvoices::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(RecurringInvoices::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecurringInvoices::Table, RecurringInvoices::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecurringInvoices::Table, RecurringInvoices::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecurringInvoices::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RecurringInvoices {
    Table,
    Id,
    ContactId,
    ProjectId,
    TemplateData,
    Frequency,
    IntervalDays,
    NextRunDate,
    EndDate,
    AutoSend,
    IsActive,
    LastGeneratedAt,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
