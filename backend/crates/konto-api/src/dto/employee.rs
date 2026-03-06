use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmployeeResponse {
    pub id: String,
    pub number: Option<String>,
    pub user_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: String,
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: String,
    pub employment_end: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub employment_percentage: f64,
    pub gross_monthly_salary: f64,
    pub annual_salary_13th: bool,
    pub has_children: bool,
    pub number_of_children: i32,
    pub child_allowance_amount: f64,
    pub education_allowance_amount: f64,
    pub bvg_insured: bool,
    pub uvg_insured: bool,
    pub ktg_insured: bool,
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    pub quellensteuer_rate: Option<f64>,
    pub marital_status: String,
    pub canton: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: String,
    #[serde(default = "default_country")]
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    #[serde(default = "default_country")]
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: String,
    pub employment_end: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    #[serde(default = "default_percentage")]
    pub employment_percentage: f64,
    pub gross_monthly_salary: f64,
    #[serde(default)]
    pub annual_salary_13th: bool,
    #[serde(default)]
    pub has_children: bool,
    #[serde(default)]
    pub number_of_children: i32,
    #[serde(default = "default_child_allowance")]
    pub child_allowance_amount: f64,
    #[serde(default = "default_education_allowance")]
    pub education_allowance_amount: f64,
    #[serde(default = "default_true")]
    pub bvg_insured: bool,
    #[serde(default = "default_true")]
    pub uvg_insured: bool,
    #[serde(default = "default_true")]
    pub ktg_insured: bool,
    #[serde(default)]
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    pub quellensteuer_rate: Option<f64>,
    #[serde(default = "default_marital_status")]
    pub marital_status: String,
    #[serde(default = "default_canton")]
    pub canton: String,
    pub user_id: Option<String>,
    pub notes: Option<String>,
    pub create_user: Option<bool>,
    pub user_role_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: String,
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: String,
    pub employment_end: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub employment_percentage: f64,
    pub gross_monthly_salary: f64,
    pub annual_salary_13th: bool,
    pub has_children: bool,
    pub number_of_children: i32,
    pub child_allowance_amount: f64,
    pub education_allowance_amount: f64,
    pub bvg_insured: bool,
    pub uvg_insured: bool,
    pub ktg_insured: bool,
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    pub quellensteuer_rate: Option<f64>,
    pub marital_status: String,
    pub canton: String,
    pub status: String,
    pub user_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CreateEmployeeResponse {
    pub employee: EmployeeResponse,
    pub provisioned_user: Option<ProvisionedUserInfo>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProvisionedUserInfo {
    pub user_id: String,
    pub temp_password: String,
}

fn default_country() -> String { "CH".to_string() }
fn default_percentage() -> f64 { 100.0 }
fn default_child_allowance() -> f64 { 215.0 }
fn default_education_allowance() -> f64 { 268.0 }
fn default_marital_status() -> String { "single".to_string() }
fn default_canton() -> String { "BS".to_string() }
fn default_true() -> bool { true }

impl From<konto_db::entities::employee::Model> for EmployeeResponse {
    fn from(m: konto_db::entities::employee::Model) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: m.id,
            number: m.number,
            user_id: m.user_id,
            first_name: m.first_name,
            last_name: m.last_name,
            email: m.email,
            phone: m.phone,
            ahv_number: m.ahv_number,
            date_of_birth: m.date_of_birth.to_string(),
            nationality: m.nationality,
            street: m.street,
            postal_code: m.postal_code,
            city: m.city,
            country: m.country,
            iban: m.iban,
            bic: m.bic,
            bank_name: m.bank_name,
            employment_start: m.employment_start.to_string(),
            employment_end: m.employment_end.map(|d| d.to_string()),
            position: m.position,
            department: m.department,
            employment_percentage: m.employment_percentage.to_f64().unwrap_or(100.0),
            gross_monthly_salary: m.gross_monthly_salary.to_f64().unwrap_or(0.0),
            annual_salary_13th: m.annual_salary_13th,
            has_children: m.has_children,
            number_of_children: m.number_of_children,
            child_allowance_amount: m.child_allowance_amount.to_f64().unwrap_or(215.0),
            education_allowance_amount: m.education_allowance_amount.to_f64().unwrap_or(268.0),
            bvg_insured: m.bvg_insured,
            uvg_insured: m.uvg_insured,
            ktg_insured: m.ktg_insured,
            is_quellensteuer: m.is_quellensteuer,
            quellensteuer_tariff: m.quellensteuer_tariff,
            quellensteuer_rate: m.quellensteuer_rate.and_then(|d| d.to_f64()),
            marital_status: m.marital_status,
            canton: m.canton,
            status: m.status,
            notes: m.notes,
            created_at: m.created_at.to_string(),
            updated_at: m.updated_at.to_string(),
        }
    }
}
