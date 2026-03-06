use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::CreditNoteStatus;
use konto_db::entities::{account, credit_note};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::credit_note_repo::CreditNoteRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};

use super::credit_note_service::CreditNoteService;
use super::journal_service::{JournalLineInput, JournalService};

/// Transition draft → issued: assign GS number and create reversing journal entry.
pub async fn issue(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<credit_note::Model, AppError> {
    let cn = CreditNoteService::get_model(db, id).await?;
    if cn.status != CreditNoteStatus::Draft.as_str() {
        return Err(AppError::Validation(
            "Only draft credit notes can be issued".into(),
        ));
    }

    let lines = CreditNoteRepo::find_lines_by_credit_note(db, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let year = cn.issue_date.format("%Y").to_string().parse::<i32>().unwrap_or(2026);
    let cn_number = CreditNoteRepo::next_credit_note_number(db, year)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Build journal entry: Credit 1100 (Debitoren), Debit revenue accounts per line
    let debitoren = find_account_by_number(db, 1100).await?;
    let vat_liability = find_account_by_number(db, 2200).await?;

    let mut journal_lines = Vec::new();

    // Credit: Debitoren (1100) for total — reduces receivables
    journal_lines.push(JournalLineInput {
        account_id: debitoren.id.clone(),
        debit_amount: Decimal::ZERO,
        credit_amount: cn.total,
        description: Some("Credit note — reduce receivables".into()),
        vat_rate_id: None,
    });

    // Debit: revenue accounts per line — reduces revenue
    for line in &lines {
        if line.line_total > Decimal::ZERO {
            journal_lines.push(JournalLineInput {
                account_id: line.account_id.clone(),
                debit_amount: line.line_total,
                credit_amount: Decimal::ZERO,
                description: Some(line.description.clone()),
                vat_rate_id: line.vat_rate_id.clone(),
            });
        }
    }

    // Debit: VAT liability (2200) if applicable — reduces VAT liability
    if cn.vat_amount > Decimal::ZERO {
        journal_lines.push(JournalLineInput {
            account_id: vat_liability.id.clone(),
            debit_amount: cn.vat_amount,
            credit_amount: Decimal::ZERO,
            description: Some("VAT correction".into()),
            vat_rate_id: None,
        });
    }

    let description = format!("Credit Note {cn_number}");
    let (entry, _) = JournalService::create(
        db,
        cn.issue_date,
        &description,
        Some(cn_number.clone()),
        cn.currency_id.clone(),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    let now = Utc::now().naive_utc();
    let mut model: credit_note::ActiveModel = cn.into();
    model.status = Set(CreditNoteStatus::Issued.to_string());
        tracing::info!(credit_note_id = %id, action = "issued", "Credit note issued");
    model.credit_note_number = Set(Some(cn_number));
    model.journal_entry_id = Set(Some(entry.id));
    model.updated_at = Set(now);

    CreditNoteRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Transition issued → applied.
pub async fn apply(
    db: &DatabaseConnection,
    id: &str,
) -> Result<credit_note::Model, AppError> {
    let cn = CreditNoteService::get_model(db, id).await?;
    if cn.status != CreditNoteStatus::Issued.as_str() {
        return Err(AppError::Validation(
            "Only issued credit notes can be applied".into(),
        ));
    }

    let now = Utc::now().naive_utc();
    let mut model: credit_note::ActiveModel = cn.into();
    model.status = Set(CreditNoteStatus::Applied.to_string());
        tracing::info!(credit_note_id = %id, action = "applied", "Credit note applied");
    model.updated_at = Set(now);

    CreditNoteRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Cancel a credit note. Reverse journal entry if one was created.
pub async fn cancel(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<credit_note::Model, AppError> {
    let cn = CreditNoteService::get_model(db, id).await?;
    if cn.status != CreditNoteStatus::Issued.as_str() && cn.status != CreditNoteStatus::Draft.as_str() {
        return Err(AppError::Validation(
            "Only draft or issued credit notes can be cancelled".into(),
        ));
    }

    if let Some(ref je_id) = cn.journal_entry_id {
        JournalService::reverse_entry(db, je_id, user_id).await?;
    }

    let now = Utc::now().naive_utc();
    let mut model: credit_note::ActiveModel = cn.into();
    model.status = Set(CreditNoteStatus::Cancelled.to_string());
        tracing::info!(credit_note_id = %id, action = "cancelled", "Credit note cancelled");
    model.updated_at = Set(now);

    CreditNoteRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

async fn find_account_by_number(
    db: &DatabaseConnection,
    number: i32,
) -> Result<account::Model, AppError> {
    AccountRepo::find_by_number(db, number)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound(format!("Account {number} not found")))
}
