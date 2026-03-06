use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::ExpenseStatus;
use konto_db::entities::{account, expense};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::expense_category_repo::ExpenseCategoryRepo;
use konto_db::repository::expense_repo::ExpenseRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};

use super::expense_service::ExpenseService;
use super::journal_service::{JournalLineInput, JournalService};

/// Transition pending → approved: create journal entry (debit expense, credit Kreditoren).
pub async fn approve_expense(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<expense::Model, AppError> {
    let exp = ExpenseService::get_expense_model(db, id).await?;
    if exp.status != ExpenseStatus::Pending.as_str() {
        return Err(AppError::Validation(
            "Only pending expenses can be approved".into(),
        ));
    }

    // Build journal lines
    let journal_lines = build_approve_journal_lines(db, &exp).await?;
    let exp_num = exp.expense_number.as_deref().unwrap_or(&exp.id);
    let description = format!("Expense {exp_num}");

    let (entry, _) = JournalService::create(
        db,
        exp.expense_date,
        &description,
        Some(exp_num.to_string()),
        Some(exp.currency_id.clone()),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    let now = Utc::now().naive_utc();
    let mut model: expense::ActiveModel = exp.into();
    model.status = Set(ExpenseStatus::Approved.to_string());
        tracing::info!(expense_id = %id, action = "approved", "Expense approved");
    model.journal_entry_id = Set(Some(entry.id));
    model.updated_at = Set(now);

    ExpenseRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Transition approved → paid: create payment journal entry (debit Kreditoren, credit bank).
pub async fn pay_expense(
    db: &DatabaseConnection,
    id: &str,
    payment_account_id: &str,
    user_id: &str,
) -> Result<expense::Model, AppError> {
    let exp = ExpenseService::get_expense_model(db, id).await?;
    if exp.status != ExpenseStatus::Approved.as_str() {
        return Err(AppError::Validation(
            "Only approved expenses can be paid".into(),
        ));
    }

    // Verify payment account exists
    AccountRepo::find_by_id(db, payment_account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Payment account not found".into()))?;

    let kreditoren = find_account_by_number(db, 2000).await?;

    let journal_lines = vec![
        JournalLineInput {
            account_id: kreditoren.id.clone(),
            debit_amount: exp.total,
            credit_amount: Decimal::ZERO,
            description: Some("Supplier payment".to_string()),
            vat_rate_id: None,
        },
        JournalLineInput {
            account_id: payment_account_id.to_string(),
            debit_amount: Decimal::ZERO,
            credit_amount: exp.total,
            description: Some("Supplier payment".to_string()),
            vat_rate_id: None,
        },
    ];

    let exp_num = exp.expense_number.as_deref().unwrap_or(&exp.id);
    let description = format!("Payment for {exp_num}");

    let (entry, _) = JournalService::create(
        db,
        exp.expense_date,
        &description,
        Some(format!("PAY-{exp_num}")),
        Some(exp.currency_id.clone()),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    let now = Utc::now().naive_utc();
    let mut model: expense::ActiveModel = exp.into();
    model.status = Set(ExpenseStatus::Paid.to_string());
        tracing::info!(expense_id = %id, action = "paid", "Expense paid");
    model.payment_account_id = Set(Some(payment_account_id.to_string()));
    model.payment_journal_entry_id = Set(Some(entry.id));
    model.updated_at = Set(now);

    ExpenseRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Cancel an approved or pending expense by reversing the journal entry if present.
pub async fn cancel_expense(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<expense::Model, AppError> {
    let exp = ExpenseService::get_expense_model(db, id).await?;
    if exp.status != ExpenseStatus::Pending.as_str() && exp.status != "approved" {
        return Err(AppError::Validation(
            "Only pending or approved expenses can be cancelled".into(),
        ));
    }

    // Reverse the approval journal entry if present
    if let Some(ref je_id) = exp.journal_entry_id {
        JournalService::reverse_entry(db, je_id, user_id).await?;
    }

    let now = Utc::now().naive_utc();
    let mut model: expense::ActiveModel = exp.into();
    model.status = Set(ExpenseStatus::Cancelled.to_string());
        tracing::info!(expense_id = %id, action = "cancelled", "Expense cancelled");
    model.updated_at = Set(now);

    ExpenseRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

// --- helpers ---

async fn find_account_by_number(
    db: &DatabaseConnection,
    number: i32,
) -> Result<account::Model, AppError> {
    AccountRepo::find_by_number(db, number)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Account {number} not found")))
}

async fn build_approve_journal_lines(
    db: &DatabaseConnection,
    exp: &expense::Model,
) -> Result<Vec<JournalLineInput>, AppError> {
    let kreditoren = find_account_by_number(db, 2000).await?;

    // Determine expense account: from category or fallback to generic expense
    let expense_account_id = if let Some(ref cat_id) = exp.category_id {
        let cat = ExpenseCategoryRepo::find_by_id(db, cat_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        cat.and_then(|c| c.account_id)
    } else {
        None
    };

    // Fallback to a generic expense account (4400) if no category account
    let expense_account = if let Some(aid) = expense_account_id {
        AccountRepo::find_by_id(db, &aid)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Expense account not found".into()))?
    } else {
        find_account_by_number(db, 4400).await?
    };

    let mut journal_lines = Vec::new();

    // Debit: expense account for net amount
    journal_lines.push(JournalLineInput {
        account_id: expense_account.id.clone(),
        debit_amount: exp.amount,
        credit_amount: Decimal::ZERO,
        description: Some(exp.description.clone()),
        vat_rate_id: exp.vat_rate_id.clone(),
    });

    // Debit: input VAT (1170) if applicable
    if exp.vat_amount > Decimal::ZERO {
        let vorsteuer = find_account_by_number(db, 1170).await?;
        journal_lines.push(JournalLineInput {
            account_id: vorsteuer.id.clone(),
            debit_amount: exp.vat_amount,
            credit_amount: Decimal::ZERO,
            description: Some("Input VAT".to_string()),
            vat_rate_id: None,
        });
    }

    // Credit: Kreditoren (2000) for total
    journal_lines.push(JournalLineInput {
        account_id: kreditoren.id.clone(),
        debit_amount: Decimal::ZERO,
        credit_amount: exp.total,
        description: Some("Accounts payable".to_string()),
        vat_rate_id: None,
    });

    Ok(journal_lines)
}
