use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PayrollRunResponse {
    pub id: String,
    pub period_month: i32,
    pub period_year: i32,
    pub status: String,
    pub run_date: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub paid_at: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payment_file_generated: bool,
    pub total_gross: f64,
    pub total_net: f64,
    pub total_employer_cost: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PayrollRunLineResponse {
    pub id: String,
    pub payroll_run_id: String,
    pub employee_id: String,
    pub employee_name: Option<String>,
    pub gross_salary: f64,
    pub ahv_employee: f64,
    pub ahv_employer: f64,
    pub alv_employee: f64,
    pub alv_employer: f64,
    pub bvg_employee: f64,
    pub bvg_employer: f64,
    pub nbu_employee: f64,
    pub bu_employer: f64,
    pub ktg_employee: f64,
    pub ktg_employer: f64,
    pub fak_employer: f64,
    pub quellensteuer: f64,
    pub child_allowance: f64,
    pub net_salary: f64,
    pub payout_amount: f64,
    pub total_employer_cost: f64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PayrollRunDetailResponse {
    pub run: PayrollRunResponse,
    pub lines: Vec<PayrollRunLineResponse>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePayrollRunRequest {
    pub month: i32,
    pub year: i32,
}

impl From<konto_db::entities::payroll_run::Model> for PayrollRunResponse {
    fn from(m: konto_db::entities::payroll_run::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            period_month: m.period_month,
            period_year: m.period_year,
            status: m.status,
            run_date: m.run_date.to_string(),
            approved_by: m.approved_by,
            approved_at: m.approved_at.map(|d| d.to_string()),
            paid_at: m.paid_at.map(|d| d.to_string()),
            journal_entry_id: m.journal_entry_id,
            payment_file_generated: m.payment_file_generated,
            total_gross: m.total_gross.to_f64().unwrap_or(0.0),
            total_net: m.total_net.to_f64().unwrap_or(0.0),
            total_employer_cost: m.total_employer_cost.to_f64().unwrap_or(0.0),
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}

impl PayrollRunLineResponse {
    pub fn from_model(
        m: konto_db::entities::payroll_run_line::Model,
        employee_name: Option<String>,
    ) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            payroll_run_id: m.payroll_run_id,
            employee_id: m.employee_id,
            employee_name,
            gross_salary: m.gross_salary.to_f64().unwrap_or(0.0),
            ahv_employee: m.ahv_employee.to_f64().unwrap_or(0.0),
            ahv_employer: m.ahv_employer.to_f64().unwrap_or(0.0),
            alv_employee: m.alv_employee.to_f64().unwrap_or(0.0),
            alv_employer: m.alv_employer.to_f64().unwrap_or(0.0),
            bvg_employee: m.bvg_employee.to_f64().unwrap_or(0.0),
            bvg_employer: m.bvg_employer.to_f64().unwrap_or(0.0),
            nbu_employee: m.nbu_employee.to_f64().unwrap_or(0.0),
            bu_employer: m.bu_employer.to_f64().unwrap_or(0.0),
            ktg_employee: m.ktg_employee.to_f64().unwrap_or(0.0),
            ktg_employer: m.ktg_employer.to_f64().unwrap_or(0.0),
            fak_employer: m.fak_employer.to_f64().unwrap_or(0.0),
            quellensteuer: m.quellensteuer.to_f64().unwrap_or(0.0),
            child_allowance: m.child_allowance.to_f64().unwrap_or(0.0),
            net_salary: m.net_salary.to_f64().unwrap_or(0.0),
            payout_amount: m.payout_amount.to_f64().unwrap_or(0.0),
            total_employer_cost: m.total_employer_cost.to_f64().unwrap_or(0.0),
        }
    }
}
