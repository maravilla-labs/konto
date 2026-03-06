use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PayrollSettingsResponse {
    pub id: String,
    pub ahv_iv_eo_rate_employee: f64,
    pub ahv_iv_eo_rate_employer: f64,
    pub alv_rate_employee: f64,
    pub alv_rate_employer: f64,
    pub alv_salary_cap: f64,
    pub bvg_coordination_deduction: f64,
    pub bvg_entry_threshold: f64,
    pub bvg_min_insured_salary: f64,
    pub bvg_max_insured_salary: f64,
    pub bvg_rate_25_34: f64,
    pub bvg_rate_35_44: f64,
    pub bvg_rate_45_54: f64,
    pub bvg_rate_55_65: f64,
    pub bvg_risk_rate: f64,
    pub bvg_employer_share_pct: f64,
    pub nbu_rate_employee: f64,
    pub bu_rate_employer: f64,
    pub ktg_rate_employee: f64,
    pub ktg_rate_employer: f64,
    pub fak_rate_employer: f64,
    pub uvg_max_salary: f64,
    pub payment_bank_account_id: Option<String>,
    pub company_clearing_number: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePayrollSettingsRequest {
    pub ahv_iv_eo_rate_employee: f64,
    pub ahv_iv_eo_rate_employer: f64,
    pub alv_rate_employee: f64,
    pub alv_rate_employer: f64,
    pub alv_salary_cap: f64,
    pub bvg_coordination_deduction: f64,
    pub bvg_entry_threshold: f64,
    pub bvg_min_insured_salary: f64,
    pub bvg_max_insured_salary: f64,
    pub bvg_rate_25_34: f64,
    pub bvg_rate_35_44: f64,
    pub bvg_rate_45_54: f64,
    pub bvg_rate_55_65: f64,
    pub bvg_risk_rate: f64,
    pub bvg_employer_share_pct: f64,
    pub nbu_rate_employee: f64,
    pub bu_rate_employer: f64,
    pub ktg_rate_employee: f64,
    pub ktg_rate_employer: f64,
    pub fak_rate_employer: f64,
    pub uvg_max_salary: f64,
    pub payment_bank_account_id: Option<String>,
    pub company_clearing_number: Option<String>,
}

impl From<konto_db::entities::payroll_setting::Model> for PayrollSettingsResponse {
    fn from(m: konto_db::entities::payroll_setting::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            ahv_iv_eo_rate_employee: m.ahv_iv_eo_rate_employee.to_f64().unwrap_or(0.0),
            ahv_iv_eo_rate_employer: m.ahv_iv_eo_rate_employer.to_f64().unwrap_or(0.0),
            alv_rate_employee: m.alv_rate_employee.to_f64().unwrap_or(0.0),
            alv_rate_employer: m.alv_rate_employer.to_f64().unwrap_or(0.0),
            alv_salary_cap: m.alv_salary_cap.to_f64().unwrap_or(0.0),
            bvg_coordination_deduction: m.bvg_coordination_deduction.to_f64().unwrap_or(0.0),
            bvg_entry_threshold: m.bvg_entry_threshold.to_f64().unwrap_or(0.0),
            bvg_min_insured_salary: m.bvg_min_insured_salary.to_f64().unwrap_or(0.0),
            bvg_max_insured_salary: m.bvg_max_insured_salary.to_f64().unwrap_or(0.0),
            bvg_rate_25_34: m.bvg_rate_25_34.to_f64().unwrap_or(0.0),
            bvg_rate_35_44: m.bvg_rate_35_44.to_f64().unwrap_or(0.0),
            bvg_rate_45_54: m.bvg_rate_45_54.to_f64().unwrap_or(0.0),
            bvg_rate_55_65: m.bvg_rate_55_65.to_f64().unwrap_or(0.0),
            bvg_risk_rate: m.bvg_risk_rate.to_f64().unwrap_or(0.0),
            bvg_employer_share_pct: m.bvg_employer_share_pct.to_f64().unwrap_or(0.0),
            nbu_rate_employee: m.nbu_rate_employee.to_f64().unwrap_or(0.0),
            bu_rate_employer: m.bu_rate_employer.to_f64().unwrap_or(0.0),
            ktg_rate_employee: m.ktg_rate_employee.to_f64().unwrap_or(0.0),
            ktg_rate_employer: m.ktg_rate_employer.to_f64().unwrap_or(0.0),
            fak_rate_employer: m.fak_rate_employer.to_f64().unwrap_or(0.0),
            uvg_max_salary: m.uvg_max_salary.to_f64().unwrap_or(0.0),
            payment_bank_account_id: m.payment_bank_account_id,
            company_clearing_number: m.company_clearing_number,
        }
    }
}
