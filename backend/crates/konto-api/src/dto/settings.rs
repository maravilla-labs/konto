use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct CompanySettingsResponse {
    pub id: String,
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub vat_number: Option<String>,
    pub vat_method: String,
    #[schema(value_type = Option<String>)]
    pub flat_rate_percentage: Option<Decimal>,
    pub register_number: Option<String>,
    pub logo_url: Option<String>,
    pub default_currency_id: Option<String>,
    pub date_format: String,
    pub number_format: String,
    pub ui_language: String,
    pub fiscal_year_start_month: i32,
    pub tax_id_label: String,
    pub audit_optout: bool,
    pub project_number_auto: bool,
    pub project_number_prefix: String,
    pub project_number_restart_yearly: bool,
    pub project_number_start: i32,
    pub project_number_min_length: i32,
    pub customer_number_auto: bool,
    pub customer_number_prefix: String,
    pub customer_number_restart_yearly: bool,
    pub customer_number_start: i32,
    pub customer_number_min_length: i32,
    pub employee_number_auto: bool,
    pub employee_number_prefix: String,
    pub employee_number_restart_yearly: bool,
    pub employee_number_start: i32,
    pub employee_number_min_length: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateCompanySettingsRequest {
    pub legal_name: String,
    pub trade_name: Option<String>,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub vat_number: Option<String>,
    pub vat_method: String,
    #[schema(value_type = Option<String>)]
    pub flat_rate_percentage: Option<Decimal>,
    pub register_number: Option<String>,
    pub default_currency_id: Option<String>,
    pub date_format: Option<String>,
    pub number_format: Option<String>,
    pub ui_language: Option<String>,
    pub fiscal_year_start_month: Option<i32>,
    pub tax_id_label: Option<String>,
    pub audit_optout: Option<bool>,
    pub project_number_auto: Option<bool>,
    pub project_number_prefix: Option<String>,
    pub project_number_restart_yearly: Option<bool>,
    pub project_number_start: Option<i32>,
    pub project_number_min_length: Option<i32>,
    pub customer_number_auto: Option<bool>,
    pub customer_number_prefix: Option<String>,
    pub customer_number_restart_yearly: Option<bool>,
    pub customer_number_start: Option<i32>,
    pub customer_number_min_length: Option<i32>,
    pub employee_number_auto: Option<bool>,
    pub employee_number_prefix: Option<String>,
    pub employee_number_restart_yearly: Option<bool>,
    pub employee_number_start: Option<i32>,
    pub employee_number_min_length: Option<i32>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BankAccountResponse {
    pub id: String,
    pub name: String,
    pub bank_name: String,
    pub iban: String,
    pub bic: Option<String>,
    pub currency_id: Option<String>,
    pub account_id: Option<String>,
    pub qr_iban: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBankAccountRequest {
    pub name: String,
    pub bank_name: String,
    pub iban: String,
    pub bic: Option<String>,
    pub currency_id: Option<String>,
    pub account_id: Option<String>,
    pub qr_iban: Option<String>,
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBankAccountRequest {
    pub name: String,
    pub bank_name: String,
    pub iban: String,
    pub bic: Option<String>,
    pub currency_id: Option<String>,
    pub account_id: Option<String>,
    pub qr_iban: Option<String>,
    #[serde(default)]
    pub is_default: bool,
}

impl From<konto_db::entities::company_setting::Model> for CompanySettingsResponse {
    fn from(m: konto_db::entities::company_setting::Model) -> Self {
        Self {
            id: m.id,
            legal_name: m.legal_name,
            trade_name: m.trade_name,
            street: m.street,
            postal_code: m.postal_code,
            city: m.city,
            country: m.country,
            email: m.email,
            phone: m.phone,
            website: m.website,
            vat_number: m.vat_number,
            vat_method: m.vat_method,
            flat_rate_percentage: m.flat_rate_percentage,
            register_number: m.register_number,
            logo_url: m.logo_url,
            default_currency_id: m.default_currency_id,
            date_format: m.date_format,
            number_format: m.number_format,
            ui_language: m.ui_language,
            fiscal_year_start_month: m.fiscal_year_start_month,
            tax_id_label: m.tax_id_label,
            audit_optout: m.audit_optout,
            project_number_auto: m.project_number_auto,
            project_number_prefix: m.project_number_prefix,
            project_number_restart_yearly: m.project_number_restart_yearly,
            project_number_start: m.project_number_start,
            project_number_min_length: m.project_number_min_length,
            customer_number_auto: m.customer_number_auto,
            customer_number_prefix: m.customer_number_prefix,
            customer_number_restart_yearly: m.customer_number_restart_yearly,
            customer_number_start: m.customer_number_start,
            customer_number_min_length: m.customer_number_min_length,
            employee_number_auto: m.employee_number_auto,
            employee_number_prefix: m.employee_number_prefix,
            employee_number_restart_yearly: m.employee_number_restart_yearly,
            employee_number_start: m.employee_number_start,
            employee_number_min_length: m.employee_number_min_length,
        }
    }
}

impl From<konto_db::entities::bank_account::Model> for BankAccountResponse {
    fn from(m: konto_db::entities::bank_account::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            bank_name: m.bank_name,
            iban: m.iban,
            bic: m.bic,
            currency_id: m.currency_id,
            account_id: m.account_id,
            qr_iban: m.qr_iban,
            is_default: m.is_default,
        }
    }
}
