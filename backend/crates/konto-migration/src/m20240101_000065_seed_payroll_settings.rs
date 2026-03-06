use sea_orm_migration::prelude::*;

use crate::m20240101_000064_create_payroll_settings::PayrollSettings;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = chrono::Utc::now().naive_utc().to_string();

        let insert = Query::insert()
            .into_table(PayrollSettings::Table)
            .columns([
                PayrollSettings::Id,
                PayrollSettings::AhvIvEoRateEmployee,
                PayrollSettings::AhvIvEoRateEmployer,
                PayrollSettings::AlvRateEmployee,
                PayrollSettings::AlvRateEmployer,
                PayrollSettings::AlvSalaryCap,
                PayrollSettings::BvgCoordinationDeduction,
                PayrollSettings::BvgEntryThreshold,
                PayrollSettings::BvgMinInsuredSalary,
                PayrollSettings::BvgMaxInsuredSalary,
                PayrollSettings::BvgRate2534,
                PayrollSettings::BvgRate3544,
                PayrollSettings::BvgRate4554,
                PayrollSettings::BvgRate5565,
                PayrollSettings::BvgRiskRate,
                PayrollSettings::BvgEmployerSharePct,
                PayrollSettings::NbuRateEmployee,
                PayrollSettings::BuRateEmployer,
                PayrollSettings::KtgRateEmployee,
                PayrollSettings::KtgRateEmployer,
                PayrollSettings::FakRateEmployer,
                PayrollSettings::UvgMaxSalary,
                PayrollSettings::CreatedAt,
                PayrollSettings::UpdatedAt,
            ])
            .values_panic([
                "payroll-settings-default".into(),
                "5.30".into(),
                "5.30".into(),
                "1.10".into(),
                "1.10".into(),
                "148200.00".into(),
                "26460.00".into(),
                "22680.00".into(),
                "3780.00".into(),
                "64260.00".into(),
                "7.00".into(),
                "10.00".into(),
                "15.00".into(),
                "18.00".into(),
                "2.50".into(),
                "50.00".into(),
                "1.50".into(),
                "0.10".into(),
                "0.50".into(),
                "0.50".into(),
                "1.60".into(),
                "148200.00".into(),
                now.clone().into(),
                now.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DELETE FROM payroll_settings").await?;
        Ok(())
    }
}
