use sea_orm_migration::prelude::*;

use crate::m20240101_000001_create_tables::Users;
use crate::m20240101_000004_create_projects::Projects;
use crate::m20240101_000045_create_rate_functions::RateFunctions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectMembers::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProjectMembers::ProjectId).string().not_null())
                    .col(ColumnDef::new(ProjectMembers::UserId).string().not_null())
                    .col(ColumnDef::new(ProjectMembers::RateFunctionId).string().null())
                    .col(
                        ColumnDef::new(ProjectMembers::HourlyRate)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(ColumnDef::new(ProjectMembers::RoleLabel).string().null())
                    .col(ColumnDef::new(ProjectMembers::JoinedAt).timestamp().not_null())
                    .col(ColumnDef::new(ProjectMembers::LeftAt).timestamp().null())
                    .col(ColumnDef::new(ProjectMembers::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(ProjectMembers::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectMembers::Table, ProjectMembers::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectMembers::Table, ProjectMembers::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProjectMembers::Table, ProjectMembers::RateFunctionId)
                            .to(RateFunctions::Table, RateFunctions::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // UNIQUE(project_id, user_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_project_members_project_user")
                    .table(ProjectMembers::Table)
                    .col(ProjectMembers::ProjectId)
                    .col(ProjectMembers::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectMembers::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ProjectMembers {
    Table,
    Id,
    ProjectId,
    UserId,
    RateFunctionId,
    HourlyRate,
    RoleLabel,
    JoinedAt,
    LeftAt,
    CreatedAt,
    UpdatedAt,
}
