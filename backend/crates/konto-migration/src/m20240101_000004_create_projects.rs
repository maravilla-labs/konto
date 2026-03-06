use sea_orm_migration::prelude::*;

use crate::m20240101_000003_create_contacts::Contacts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Activity types
        manager
            .create_table(
                Table::create()
                    .table(ActivityTypes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(ActivityTypes::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(ActivityTypes::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(ActivityTypes::IsActive).boolean().not_null().default(true))
                    .to_owned(),
            )
            .await?;

        // Projects
        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Projects::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Projects::Number).string().null())
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::ContactId).string().null())
                    .col(ColumnDef::new(Projects::ContactPersonName).string().null())
                    .col(ColumnDef::new(Projects::StartDate).date().null())
                    .col(ColumnDef::new(Projects::EndDate).date().null())
                    .col(ColumnDef::new(Projects::Status).string().not_null().default("active"))
                    .col(ColumnDef::new(Projects::Description).text().null())
                    .col(ColumnDef::new(Projects::ProjectType).string().null())
                    .col(ColumnDef::new(Projects::BudgetHours).decimal().null())
                    .col(ColumnDef::new(Projects::BudgetAmount).decimal().null())
                    .col(ColumnDef::new(Projects::HourlyRate).decimal().null())
                    .col(ColumnDef::new(Projects::BexioId).integer().null())
                    .col(ColumnDef::new(Projects::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Projects::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Projects::Table, Projects::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Time entries
        manager
            .create_table(
                Table::create()
                    .table(TimeEntries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TimeEntries::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(TimeEntries::ProjectId).string().null())
                    .col(ColumnDef::new(TimeEntries::ContactId).string().null())
                    .col(ColumnDef::new(TimeEntries::UserId).string().null())
                    .col(ColumnDef::new(TimeEntries::ActivityTypeId).string().null())
                    .col(ColumnDef::new(TimeEntries::Date).date().not_null())
                    .col(ColumnDef::new(TimeEntries::EstimatedMinutes).integer().null())
                    .col(ColumnDef::new(TimeEntries::ActualMinutes).integer().not_null())
                    .col(ColumnDef::new(TimeEntries::FlatAmount).decimal().null())
                    .col(ColumnDef::new(TimeEntries::Description).text().null())
                    .col(ColumnDef::new(TimeEntries::TravelMinutes).integer().null())
                    .col(ColumnDef::new(TimeEntries::TravelFlatRate).decimal().null())
                    .col(ColumnDef::new(TimeEntries::TravelDistance).decimal().null())
                    .col(ColumnDef::new(TimeEntries::Status).string().not_null().default("active"))
                    .col(ColumnDef::new(TimeEntries::BexioId).integer().null())
                    .col(ColumnDef::new(TimeEntries::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(TimeEntries::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(TimeEntries::Table, TimeEntries::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(TimeEntries::Table, TimeEntries::ContactId)
                            .to(Contacts::Table, Contacts::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TimeEntries::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Projects::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ActivityTypes::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum ActivityTypes {
    Table,
    Id,
    Name,
    IsActive,
}

#[derive(DeriveIden)]
pub enum Projects {
    Table,
    Id,
    Number,
    Name,
    ContactId,
    ContactPersonName,
    StartDate,
    EndDate,
    Status,
    Description,
    ProjectType,
    BudgetHours,
    BudgetAmount,
    HourlyRate,
    BexioId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum TimeEntries {
    Table,
    Id,
    ProjectId,
    ContactId,
    UserId,
    ActivityTypeId,
    Date,
    EstimatedMinutes,
    ActualMinutes,
    FlatAmount,
    Description,
    TravelMinutes,
    TravelFlatRate,
    TravelDistance,
    Status,
    BexioId,
    CreatedAt,
    UpdatedAt,
}
