use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;
use crate::m20240101_000004_create_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectItems::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectItems::ProjectId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectItems::ParentId).string().null())
                    .col(
                        ColumnDef::new(ProjectItems::ItemType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectItems::Name).string().not_null())
                    .col(ColumnDef::new(ProjectItems::Description).text().null())
                    .col(
                        ColumnDef::new(ProjectItems::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(ProjectItems::AssigneeId).string().null())
                    .col(ColumnDef::new(ProjectItems::StartDate).date().null())
                    .col(ColumnDef::new(ProjectItems::DueDate).date().null())
                    .col(
                        ColumnDef::new(ProjectItems::EstimatedHours)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectItems::BudgetHours)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectItems::BudgetAmount)
                            .decimal_len(15, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProjectItems::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(ProjectItems::CreatedBy).string().null())
                    .col(ColumnDef::new(ProjectItems::UpdatedBy).string().null())
                    .col(
                        ColumnDef::new(ProjectItems::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectItems::UpdatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectItems::Table, ProjectItems::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectItems::Table, ProjectItems::ParentId)
                            .to(ProjectItems::Table, ProjectItems::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectItems::Table, ProjectItems::AssigneeId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectItems::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProjectItems {
    Table,
    Id,
    ProjectId,
    ParentId,
    ItemType,
    Name,
    Description,
    Status,
    AssigneeId,
    StartDate,
    DueDate,
    EstimatedHours,
    BudgetHours,
    BudgetAmount,
    SortOrder,
    CreatedBy,
    UpdatedBy,
    CreatedAt,
    UpdatedAt,
}
