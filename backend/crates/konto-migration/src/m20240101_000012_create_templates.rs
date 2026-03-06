use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DocumentTemplates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DocumentTemplates::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DocumentTemplates::Name).string().not_null())
                    .col(ColumnDef::new(DocumentTemplates::TemplateType).string().not_null())
                    .col(ColumnDef::new(DocumentTemplates::ContentJson).text().not_null())
                    .col(ColumnDef::new(DocumentTemplates::HeaderJson).text().null())
                    .col(ColumnDef::new(DocumentTemplates::FooterJson).text().null())
                    .col(ColumnDef::new(DocumentTemplates::PageSetupJson).text().null())
                    .col(
                        ColumnDef::new(DocumentTemplates::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(DocumentTemplates::CreatedBy).string().null())
                    .col(ColumnDef::new(DocumentTemplates::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(DocumentTemplates::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(DocumentTemplates::Table, DocumentTemplates::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DocumentTemplates::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum DocumentTemplates {
    Table,
    Id,
    Name,
    TemplateType,
    ContentJson,
    HeaderJson,
    FooterJson,
    PageSetupJson,
    IsDefault,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
