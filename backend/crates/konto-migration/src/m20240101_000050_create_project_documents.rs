use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;
use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Local iden for project_items FK (defined in migration 000048).
#[derive(DeriveIden)]
enum ProjectItems {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectDocuments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectDocuments::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::ProjectId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::ProjectItemId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::FileName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::FilePath)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::FileSize)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::ContentType)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::UploadedBy)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectDocuments::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectDocuments::Table, ProjectDocuments::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectDocuments::Table, ProjectDocuments::ProjectItemId)
                            .to(ProjectItems::Table, ProjectItems::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectDocuments::Table, ProjectDocuments::UploadedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectDocuments::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProjectDocuments {
    Table,
    Id,
    ProjectId,
    ProjectItemId,
    FileName,
    FilePath,
    FileSize,
    ContentType,
    UploadedBy,
    CreatedAt,
}
