use sea_orm_migration::prelude::*;

use crate::m20240101_000004_create_projects::Projects;
use crate::m20240101_000048_create_project_items::ProjectItems;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectMilestones::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectMilestones::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::ProjectId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::ProjectItemId)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::Name)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectMilestones::Description).text().null())
                    .col(
                        ColumnDef::new(ProjectMilestones::TargetDate)
                            .date()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::ReachedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectMilestones::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ProjectMilestones::Table,
                                ProjectMilestones::ProjectId,
                            )
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                ProjectMilestones::Table,
                                ProjectMilestones::ProjectItemId,
                            )
                            .to(ProjectItems::Table, ProjectItems::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProjectMilestones::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProjectMilestones {
    Table,
    Id,
    ProjectId,
    ProjectItemId,
    Name,
    Description,
    TargetDate,
    Status,
    ReachedAt,
    CreatedAt,
    UpdatedAt,
}
