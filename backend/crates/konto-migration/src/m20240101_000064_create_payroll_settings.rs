use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PayrollSettings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PayrollSettings::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PayrollSettings::AhvIvEoRateEmployee).decimal_len(5, 2).not_null().default("5.30"))
                    .col(ColumnDef::new(PayrollSettings::AhvIvEoRateEmployer).decimal_len(5, 2).not_null().default("5.30"))
                    .col(ColumnDef::new(PayrollSettings::AlvRateEmployee).decimal_len(5, 2).not_null().default("1.10"))
                    .col(ColumnDef::new(PayrollSettings::AlvRateEmployer).decimal_len(5, 2).not_null().default("1.10"))
                    .col(ColumnDef::new(PayrollSettings::AlvSalaryCap).decimal_len(15, 2).not_null().default("148200.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgCoordinationDeduction).decimal_len(15, 2).not_null().default("26460.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgEntryThreshold).decimal_len(15, 2).not_null().default("22680.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgMinInsuredSalary).decimal_len(15, 2).not_null().default("3780.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgMaxInsuredSalary).decimal_len(15, 2).not_null().default("64260.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgRate2534).decimal_len(5, 2).not_null().default("7.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgRate3544).decimal_len(5, 2).not_null().default("10.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgRate4554).decimal_len(5, 2).not_null().default("15.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgRate5565).decimal_len(5, 2).not_null().default("18.00"))
                    .col(ColumnDef::new(PayrollSettings::BvgRiskRate).decimal_len(5, 2).not_null().default("2.50"))
                    .col(ColumnDef::new(PayrollSettings::BvgEmployerSharePct).decimal_len(5, 2).not_null().default("50.00"))
                    .col(ColumnDef::new(PayrollSettings::NbuRateEmployee).decimal_len(5, 2).not_null().default("1.50"))
                    .col(ColumnDef::new(PayrollSettings::BuRateEmployer).decimal_len(5, 2).not_null().default("0.10"))
                    .col(ColumnDef::new(PayrollSettings::KtgRateEmployee).decimal_len(5, 2).not_null().default("0.50"))
                    .col(ColumnDef::new(PayrollSettings::KtgRateEmployer).decimal_len(5, 2).not_null().default("0.50"))
                    .col(ColumnDef::new(PayrollSettings::FakRateEmployer).decimal_len(5, 2).not_null().default("1.60"))
                    .col(ColumnDef::new(PayrollSettings::UvgMaxSalary).decimal_len(15, 2).not_null().default("148200.00"))
                    .col(ColumnDef::new(PayrollSettings::PaymentBankAccountId).string().null())
                    .col(ColumnDef::new(PayrollSettings::CompanyClearingNumber).string().null())
                    .col(ColumnDef::new(PayrollSettings::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PayrollSettings::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PayrollSettings::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PayrollSettings {
    Table,
    Id,
    AhvIvEoRateEmployee,
    AhvIvEoRateEmployer,
    AlvRateEmployee,
    AlvRateEmployer,
    AlvSalaryCap,
    BvgCoordinationDeduction,
    BvgEntryThreshold,
    BvgMinInsuredSalary,
    BvgMaxInsuredSalary,
    BvgRate2534,
    BvgRate3544,
    BvgRate4554,
    BvgRate5565,
    BvgRiskRate,
    BvgEmployerSharePct,
    NbuRateEmployee,
    BuRateEmployer,
    KtgRateEmployee,
    KtgRateEmployer,
    FakRateEmployer,
    UvgMaxSalary,
    PaymentBankAccountId,
    CompanyClearingNumber,
    CreatedAt,
    UpdatedAt,
}
