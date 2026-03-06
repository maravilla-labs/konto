use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::{ActivityTypes, Projects};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectActivityTypes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectActivityTypes::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectActivityTypes::ProjectId).string().not_null())
                    .col(ColumnDef::new(ProjectActivityTypes::ActivityTypeId).string().not_null())
                    .col(
                        ColumnDef::new(ProjectActivityTypes::Rate)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(ColumnDef::new(ProjectActivityTypes::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ProjectActivityTypes::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectActivityTypes::Table, ProjectActivityTypes::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectActivityTypes::Table, ProjectActivityTypes::ActivityTypeId)
                            .to(ActivityTypes::Table, ActivityTypes::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // UNIQUE(project_id, activity_type_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_project_activity_types_project_activity")
                    .table(ProjectActivityTypes::Table)
                    .col(ProjectActivityTypes::ProjectId)
                    .col(ProjectActivityTypes::ActivityTypeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectActivityTypes::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProjectActivityTypes {
    Table,
    Id,
    ProjectId,
    ActivityTypeId,
    Rate,
    CreatedAt,
    UpdatedAt,
}
