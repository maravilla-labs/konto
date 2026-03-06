use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::ExpenseStatus;
use konto_db::entities::{expense, vat_rate};
use konto_db::repository::contact_repo::ContactRepo;
use konto_db::repository::expense_category_repo::ExpenseCategoryRepo;
use konto_db::repository::expense_repo::ExpenseRepo;
use konto_db::repository::project_repo::ProjectRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use super::expense_workflow;

pub struct ExpenseDetail {
    pub expense: expense::Model,
    pub contact_name: Option<String>,
    pub category_name: Option<String>,
    pub project_name: Option<String>,
}

pub struct ExpenseService;

impl ExpenseService {
    #[allow(clippy::too_many_arguments)]
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
        status: Option<&str>,
        category_id: Option<&str>,
        contact_id: Option<&str>,
        date_from: Option<&str>,
        date_to: Option<&str>,
        search: Option<&str>,
    ) -> Result<(Vec<expense::Model>, u64), AppError> {
        ExpenseRepo::find_paginated(
            db, page, per_page, status, category_id, contact_id,
            date_from, date_to, search,
        )
        .await
        .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<ExpenseDetail, AppError> {
        let exp = Self::get_expense_model(db, id).await?;

        let contact_name = if let Some(ref cid) = exp.contact_id {
            ContactRepo::find_by_id(db, cid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|c| c.name1)
        } else {
            None
        };

        let category_name = if let Some(ref cat_id) = exp.category_id {
            ExpenseCategoryRepo::find_by_id(db, cat_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|c| c.name)
        } else {
            None
        };

        let project_name = if let Some(ref pid) = exp.project_id {
            ProjectRepo::find_by_id(db, pid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|p| p.name)
        } else {
            None
        };

        Ok(ExpenseDetail { expense: exp, contact_name, category_name, project_name })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        db: &DatabaseConnection,
        contact_id: Option<String>,
        category_id: Option<String>,
        description: &str,
        amount: Decimal,
        currency_id: &str,
        vat_rate_id: Option<String>,
        expense_date: chrono::NaiveDate,
        due_date: Option<chrono::NaiveDate>,
        project_id: Option<String>,
        user_id: Option<String>,
    ) -> Result<expense::Model, AppError> {
        if amount <= Decimal::ZERO {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }

        if due_date.is_some_and(|dd| dd < expense_date) {
            return Err(AppError::Validation(
                "Due date must be on or after expense date".to_string(),
            ));
        }

        // Validate category if provided
        if let Some(ref cat_id) = category_id {
            ExpenseCategoryRepo::find_by_id(db, cat_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Expense category not found".into()))?;
        }

        // Validate contact if provided
        if let Some(ref cid) = contact_id {
            ContactRepo::find_by_id(db, cid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;
        }

        // Compute VAT
        let vat_amount = compute_vat(db, amount, vat_rate_id.as_deref()).await?;
        let total = amount + vat_amount;

        // Assign expense number on create
        let year = expense_date.format("%Y").to_string().parse::<i32>().unwrap_or(2024);
        let exp_number = ExpenseRepo::next_expense_number(db, year)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let now = Utc::now().naive_utc();
        let model = expense::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            expense_number: Set(Some(exp_number)),
            contact_id: Set(contact_id),
            category_id: Set(category_id),
            description: Set(description.to_string()),
            amount: Set(amount),
            currency_id: Set(currency_id.to_string()),
            vat_rate_id: Set(vat_rate_id),
            vat_amount: Set(vat_amount),
            total: Set(total),
            expense_date: Set(expense_date),
            due_date: Set(due_date),
            status: Set(ExpenseStatus::Pending.to_string()),
            payment_account_id: Set(None),
            receipt_url: Set(None),
            project_id: Set(project_id),
            journal_entry_id: Set(None),
            payment_journal_entry_id: Set(None),
            created_by: Set(user_id),
            created_at: Set(now),
            updated_at: Set(now),
            expense_type: Set("single".to_string()),
            purpose: Set(None),
            employee_id: Set(None),
            period_from: Set(None),
            period_to: Set(None),
            advances: Set(Decimal::ZERO),
            total_reimbursement: Set(Decimal::ZERO),
            approved_by: Set(None),
            approved_at: Set(None),
            rejected_reason: Set(None),
        };

        ExpenseRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        contact_id: Option<String>,
        category_id: Option<String>,
        description: &str,
        amount: Decimal,
        currency_id: &str,
        vat_rate_id: Option<String>,
        expense_date: chrono::NaiveDate,
        due_date: Option<chrono::NaiveDate>,
        project_id: Option<String>,
    ) -> Result<expense::Model, AppError> {
        let existing = Self::get_expense_model(db, id).await?;
        if existing.status != ExpenseStatus::Pending.as_str() {
            return Err(AppError::Validation(
                "Only pending expenses can be updated".into(),
            ));
        }

        if amount <= Decimal::ZERO {
            return Err(AppError::Validation("Amount must be greater than 0".to_string()));
        }

        if due_date.is_some_and(|dd| dd < expense_date) {
            return Err(AppError::Validation(
                "Due date must be on or after expense date".to_string(),
            ));
        }

        // Validate category if provided
        if let Some(ref cat_id) = category_id {
            ExpenseCategoryRepo::find_by_id(db, cat_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Expense category not found".into()))?;
        }

        // Validate contact if provided
        if let Some(ref cid) = contact_id {
            ContactRepo::find_by_id(db, cid)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;
        }

        let vat_amount = compute_vat(db, amount, vat_rate_id.as_deref()).await?;
        let total = amount + vat_amount;
        let now = Utc::now().naive_utc();

        let mut model: expense::ActiveModel = existing.into();
        model.contact_id = Set(contact_id);
        model.category_id = Set(category_id);
        model.description = Set(description.to_string());
        model.amount = Set(amount);
        model.currency_id = Set(currency_id.to_string());
        model.vat_rate_id = Set(vat_rate_id);
        model.vat_amount = Set(vat_amount);
        model.total = Set(total);
        model.expense_date = Set(expense_date);
        model.due_date = Set(due_date);
        model.project_id = Set(project_id);
        model.updated_at = Set(now);

        ExpenseRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        let existing = Self::get_expense_model(db, id).await?;
        if existing.status != ExpenseStatus::Pending.as_str() {
            return Err(AppError::Validation(
                "Only pending expenses can be deleted".into(),
            ));
        }
        ExpenseRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn approve(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<expense::Model, AppError> {
        expense_workflow::approve_expense(db, id, user_id).await
    }

    pub async fn pay(
        db: &DatabaseConnection,
        id: &str,
        payment_account_id: &str,
        user_id: &str,
    ) -> Result<expense::Model, AppError> {
        expense_workflow::pay_expense(db, id, payment_account_id, user_id).await
    }

    pub async fn cancel(
        db: &DatabaseConnection,
        id: &str,
        user_id: &str,
    ) -> Result<expense::Model, AppError> {
        expense_workflow::cancel_expense(db, id, user_id).await
    }

    pub async fn upload_receipt(
        db: &DatabaseConnection,
        id: &str,
        receipt_url: &str,
    ) -> Result<expense::Model, AppError> {
        let existing = Self::get_expense_model(db, id).await?;
        let now = Utc::now().naive_utc();
        let mut model: expense::ActiveModel = existing.into();
        model.receipt_url = Set(Some(receipt_url.to_string()));
        model.updated_at = Set(now);
        ExpenseRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub(crate) async fn get_expense_model(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<expense::Model, AppError> {
        ExpenseRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Expense not found".into()))
    }
}

async fn compute_vat(
    db: &DatabaseConnection,
    amount: Decimal,
    vat_rate_id: Option<&str>,
) -> Result<Decimal, AppError> {
    if let Some(vat_id) = vat_rate_id {
        let vat = vat_rate::Entity::find_by_id(vat_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound(format!("VAT rate {vat_id} not found")))?;
        Ok(amount * vat.rate / Decimal::from(100))
    } else {
        Ok(Decimal::ZERO)
    }
}
