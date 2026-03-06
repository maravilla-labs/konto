use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::payroll_setting;
use konto_db::repository::payroll_setting_repo::PayrollSettingRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};

pub struct PayrollSettingsService;

impl PayrollSettingsService {
    pub async fn get(db: &DatabaseConnection) -> Result<payroll_setting::Model, AppError> {
        PayrollSettingRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll settings not found".into()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        ahv_iv_eo_rate_employee: Decimal,
        ahv_iv_eo_rate_employer: Decimal,
        alv_rate_employee: Decimal,
        alv_rate_employer: Decimal,
        alv_salary_cap: Decimal,
        bvg_coordination_deduction: Decimal,
        bvg_entry_threshold: Decimal,
        bvg_min_insured_salary: Decimal,
        bvg_max_insured_salary: Decimal,
        bvg_rate_25_34: Decimal,
        bvg_rate_35_44: Decimal,
        bvg_rate_45_54: Decimal,
        bvg_rate_55_65: Decimal,
        bvg_risk_rate: Decimal,
        bvg_employer_share_pct: Decimal,
        nbu_rate_employee: Decimal,
        bu_rate_employer: Decimal,
        ktg_rate_employee: Decimal,
        ktg_rate_employer: Decimal,
        fak_rate_employer: Decimal,
        uvg_max_salary: Decimal,
        payment_bank_account_id: Option<String>,
        company_clearing_number: Option<String>,
    ) -> Result<payroll_setting::Model, AppError> {
        let existing = Self::get(db).await?;
        let now = Utc::now().naive_utc();

        let mut model: payroll_setting::ActiveModel = existing.into();
        model.ahv_iv_eo_rate_employee = Set(ahv_iv_eo_rate_employee);
        model.ahv_iv_eo_rate_employer = Set(ahv_iv_eo_rate_employer);
        model.alv_rate_employee = Set(alv_rate_employee);
        model.alv_rate_employer = Set(alv_rate_employer);
        model.alv_salary_cap = Set(alv_salary_cap);
        model.bvg_coordination_deduction = Set(bvg_coordination_deduction);
        model.bvg_entry_threshold = Set(bvg_entry_threshold);
        model.bvg_min_insured_salary = Set(bvg_min_insured_salary);
        model.bvg_max_insured_salary = Set(bvg_max_insured_salary);
        model.bvg_rate_25_34 = Set(bvg_rate_25_34);
        model.bvg_rate_35_44 = Set(bvg_rate_35_44);
        model.bvg_rate_45_54 = Set(bvg_rate_45_54);
        model.bvg_rate_55_65 = Set(bvg_rate_55_65);
        model.bvg_risk_rate = Set(bvg_risk_rate);
        model.bvg_employer_share_pct = Set(bvg_employer_share_pct);
        model.nbu_rate_employee = Set(nbu_rate_employee);
        model.bu_rate_employer = Set(bu_rate_employer);
        model.ktg_rate_employee = Set(ktg_rate_employee);
        model.ktg_rate_employer = Set(ktg_rate_employer);
        model.fak_rate_employer = Set(fak_rate_employer);
        model.uvg_max_salary = Set(uvg_max_salary);
        model.payment_bank_account_id = Set(payment_bank_account_id);
        model.company_clearing_number = Set(company_clearing_number);
        model.updated_at = Set(now);

        PayrollSettingRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
