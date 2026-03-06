use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExpenseResponse {
    pub id: String,
    pub expense_number: Option<String>,
    pub contact_id: Option<String>,
    pub category_id: Option<String>,
    pub description: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
    pub currency_id: String,
    pub vat_rate_id: Option<String>,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
    #[schema(value_type = String)]
    pub total: Decimal,
    pub expense_date: String,
    pub due_date: Option<String>,
    pub status: String,
    pub payment_account_id: Option<String>,
    pub receipt_url: Option<String>,
    pub project_id: Option<String>,
    pub journal_entry_id: Option<String>,
    pub payment_journal_entry_id: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExpenseDetailResponse {
    #[serde(flatten)]
    pub expense: ExpenseResponse,
    pub contact_name: Option<String>,
    pub category_name: Option<String>,
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExpenseRequest {
    pub contact_id: Option<String>,
    pub category_id: Option<String>,
    pub description: String,
    #[schema(value_type = String)]
    pub amount: Decimal,
    pub currency_id: String,
    pub vat_rate_id: Option<String>,
    pub expense_date: String,
    pub due_date: Option<String>,
    pub project_id: Option<String>,
}

pub type UpdateExpenseRequest = CreateExpenseRequest;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ExpenseListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub category_id: Option<String>,
    pub contact_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub search: Option<String>,
    pub format: Option<String>,
}

impl ExpenseListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PayExpenseRequest {
    pub payment_account_id: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ExpenseCategoryResponse {
    pub id: String,
    pub name: String,
    pub account_id: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateExpenseCategoryRequest {
    pub name: String,
    pub account_id: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateExpenseCategoryRequest {
    pub name: String,
    pub account_id: Option<String>,
    pub is_active: bool,
}

impl From<konto_db::entities::expense::Model> for ExpenseResponse {
    fn from(m: konto_db::entities::expense::Model) -> Self {
        Self {
            id: m.id,
            expense_number: m.expense_number,
            contact_id: m.contact_id,
            category_id: m.category_id,
            description: m.description,
            amount: m.amount,
            currency_id: m.currency_id,
            vat_rate_id: m.vat_rate_id,
            vat_amount: m.vat_amount,
            total: m.total,
            expense_date: m.expense_date.to_string(),
            due_date: m.due_date.map(|d| d.to_string()),
            status: m.status,
            payment_account_id: m.payment_account_id,
            receipt_url: m.receipt_url,
            project_id: m.project_id,
            journal_entry_id: m.journal_entry_id,
            payment_journal_entry_id: m.payment_journal_entry_id,
            created_by: m.created_by,
        }
    }
}

impl From<konto_db::entities::expense_category::Model> for ExpenseCategoryResponse {
    fn from(m: konto_db::entities::expense_category::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            account_id: m.account_id,
            is_active: m.is_active,
        }
    }
}
