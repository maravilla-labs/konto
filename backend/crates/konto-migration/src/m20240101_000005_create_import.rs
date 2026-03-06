use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ImportJobs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ImportJobs::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ImportJobs::ImportType).string().not_null())
                    .col(ColumnDef::new(ImportJobs::FileName).string().not_null())
                    .col(ColumnDef::new(ImportJobs::FileData).binary().not_null())
                    .col(ColumnDef::new(ImportJobs::Status).string().not_null().default("uploaded"))
                    .col(ColumnDef::new(ImportJobs::TotalRows).integer().null())
                    .col(ColumnDef::new(ImportJobs::ImportedRows).integer().null())
                    .col(ColumnDef::new(ImportJobs::ErrorRows).integer().null())
                    .col(ColumnDef::new(ImportJobs::PreviewData).text().null())
                    .col(ColumnDef::new(ImportJobs::ErrorLog).text().null())
                    .col(ColumnDef::new(ImportJobs::CreatedBy).string().null())
                    .col(ColumnDef::new(ImportJobs::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ImportJobs::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ImportJobs::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ImportJobs {
    Table,
    Id,
    ImportType,
    FileName,
    FileData,
    Status,
    TotalRows,
    ImportedRows,
    ErrorRows,
    PreviewData,
    ErrorLog,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
