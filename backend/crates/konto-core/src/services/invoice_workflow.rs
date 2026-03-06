use chrono::Utc;
use konto_common::enums::InvoiceStatus;
use konto_common::error::AppError;
use konto_db::entities::{account, invoice, invoice_line, invoice_payment};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::invoice_payment_repo::InvoicePaymentRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::invoice_service::InvoiceService;
use super::journal_service::{JournalLineInput, JournalService};

/// Transition draft → sent: assign invoice number and create journal entry.
pub async fn send_invoice(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<invoice::Model, AppError> {
    let inv = InvoiceService::get_invoice_model(db, id).await?;
    if inv.status != InvoiceStatus::Draft.as_str() {
        return Err(AppError::Validation(
            "Only draft invoices can be sent".to_string(),
        ));
    }

    let lines = InvoiceRepo::find_lines_by_invoice(db, id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Assign invoice number
    let year = inv.issue_date.format("%Y").to_string().parse::<i32>().unwrap_or(2024);
    let inv_number = InvoiceRepo::next_invoice_number(db, year)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Create journal entry: Debit 1100 (Debitoren), Credit revenue + VAT
    let journal_lines = build_send_journal_lines(db, &inv, &lines).await?;
    let description = format!("Invoice {inv_number}");

    let (entry, _) = JournalService::create(
        db,
        inv.issue_date,
        &description,
        Some(inv_number.clone()),
        inv.currency_id.clone(),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    // Update invoice
    let now = Utc::now().naive_utc();
    let mut model: invoice::ActiveModel = inv.into();
    model.status = Set(InvoiceStatus::Sent.to_string());
    model.invoice_number = Set(Some(inv_number));
    model.journal_entry_id = Set(Some(entry.id));
    model.updated_at = Set(now);

    let result = InvoiceRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    tracing::info!(invoice_id = %id, action = "sent", "Invoice sent");
    Ok(result)
}

/// Transition sent → paid
        // tracing done below: create payment journal entry.
pub async fn mark_paid(
    db: &DatabaseConnection,
    id: &str,
    payment_date: chrono::NaiveDate,
    payment_account_id: &str,
    user_id: &str,
) -> Result<invoice::Model, AppError> {
    let inv = InvoiceService::get_invoice_model(db, id).await?;
    if inv.status != InvoiceStatus::Sent.as_str() && inv.status != InvoiceStatus::Overdue.as_str() {
        return Err(AppError::Validation(
            "Only sent or overdue invoices can be marked as paid".to_string(),
        ));
    }

    // Verify payment account exists
    AccountRepo::find_by_id(db, payment_account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Payment account not found".to_string()))?;

    // Find debitoren account (1100)
    let debitoren = find_account_by_number(db, 1100).await?;

    // Debit bank/cash, Credit 1100 (Debitoren)
    let journal_lines = vec![
        JournalLineInput {
            account_id: payment_account_id.to_string(),
            debit_amount: inv.total,
            credit_amount: Decimal::ZERO,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
        JournalLineInput {
            account_id: debitoren.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: inv.total,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
    ];

    let inv_num = inv.invoice_number.as_deref().unwrap_or(&inv.id);
    let description = format!("Payment for {inv_num}");

    let (entry, _) = JournalService::create(
        db,
        payment_date,
        &description,
        Some(format!("PAY-{inv_num}")),
        inv.currency_id.clone(),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    let now = Utc::now().naive_utc();
    let mut model: invoice::ActiveModel = inv.into();
    model.status = Set(InvoiceStatus::Paid.to_string());
    model.payment_journal_entry_id = Set(Some(entry.id));
    model.updated_at = Set(now);

    let result = InvoiceRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    tracing::info!(invoice_id = %id, action = "paid", "Invoice marked as paid");
    Ok(result)
}

/// Cancel a sent/overdue invoice by creating a reversing journal entry.
pub async fn cancel_invoice(
    db: &DatabaseConnection,
    id: &str,
    user_id: &str,
) -> Result<invoice::Model, AppError> {
    let inv = InvoiceService::get_invoice_model(db, id).await?;
    if inv.status != InvoiceStatus::Sent.as_str() && inv.status != InvoiceStatus::Overdue.as_str() {
        return Err(AppError::Validation(
            "Only sent or overdue invoices can be cancelled".to_string(),
        ));
    }

    // Reverse the original journal entry if present
    if let Some(ref je_id) = inv.journal_entry_id {
        JournalService::reverse_entry(db, je_id, user_id).await?;
    }

    let now = Utc::now().naive_utc();
    let mut model: invoice::ActiveModel = inv.into();
    model.status = Set(InvoiceStatus::Cancelled.to_string());
    model.updated_at = Set(now);

    let result = InvoiceRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    tracing::info!(invoice_id = %id, action = "cancelled", "Invoice cancelled");
    Ok(result)
}

/// Record a partial (or full) payment on an invoice.
#[allow(clippy::too_many_arguments)]
pub async fn record_payment(
    db: &DatabaseConnection,
    invoice_id: &str,
    amount: Decimal,
    payment_date: chrono::NaiveDate,
    payment_account_id: &str,
    payment_method: Option<String>,
    reference: Option<String>,
    user_id: &str,
) -> Result<invoice_payment::Model, AppError> {
    let inv = InvoiceService::get_invoice_model(db, invoice_id).await?;
    if inv.status != InvoiceStatus::Sent.as_str() && inv.status != InvoiceStatus::Overdue.as_str() && inv.status != InvoiceStatus::Partial.as_str() {
        return Err(AppError::Validation(
            "Only sent, overdue or partially-paid invoices can receive payments".to_string(),
        ));
    }

    if amount <= Decimal::ZERO {
        return Err(AppError::Validation("Payment amount must be positive".into()));
    }

    // Verify payment account
    AccountRepo::find_by_id(db, payment_account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Payment account not found".to_string()))?;

    let debitoren = find_account_by_number(db, 1100).await?;

    // Create journal entry for this payment
    let inv_num = inv.invoice_number.as_deref().unwrap_or(&inv.id);
    let description = format!("Partial payment for {inv_num}");

    let journal_lines = vec![
        JournalLineInput {
            account_id: payment_account_id.to_string(),
            debit_amount: amount,
            credit_amount: Decimal::ZERO,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
        JournalLineInput {
            account_id: debitoren.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: amount,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
    ];

    let (entry, _) = JournalService::create(
        db,
        payment_date,
        &description,
        Some(format!("PAY-{inv_num}-{}", Uuid::new_v4().to_string().split('-').next().unwrap_or("x"))),
        inv.currency_id.clone(),
        None,
        Some(user_id.to_string()),
        journal_lines,
    )
    .await?;

    JournalService::post_entry(db, &entry.id).await?;

    // Create payment record
    let now = Utc::now().naive_utc();
    let payment = invoice_payment::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        invoice_id: Set(invoice_id.to_string()),
        amount: Set(amount),
        payment_date: Set(payment_date),
        payment_method: Set(payment_method),
        reference: Set(reference),
        bank_transaction_id: Set(None),
        journal_entry_id: Set(Some(entry.id)),
        created_at: Set(now),
    };

    let payment = InvoicePaymentRepo::create(db, payment)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    // Check if fully paid
    let total_paid = InvoicePaymentRepo::sum_by_invoice(db, invoice_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let new_status = if total_paid >= inv.total {
        InvoiceStatus::Paid
    } else {
        InvoiceStatus::Partial
    };

    let mut model: invoice::ActiveModel = inv.into();
    model.status = Set(new_status.to_string());
    if new_status == InvoiceStatus::Paid {
        model.payment_journal_entry_id = Set(Some(payment.journal_entry_id.clone().unwrap_or_default()));
    }
    model.updated_at = Set(now);

    InvoiceRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(payment)
}

/// List payments for an invoice.
pub async fn list_payments(
    db: &DatabaseConnection,
    invoice_id: &str,
) -> Result<Vec<invoice_payment::Model>, AppError> {
    InvoicePaymentRepo::find_by_invoice(db, invoice_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}

/// Get the total amount paid for an invoice.
pub async fn amount_paid(
    db: &DatabaseConnection,
    invoice_id: &str,
) -> Result<Decimal, AppError> {
    InvoicePaymentRepo::sum_by_invoice(db, invoice_id)
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

async fn build_send_journal_lines(
    db: &DatabaseConnection,
    inv: &invoice::Model,
    lines: &[invoice_line::Model],
) -> Result<Vec<JournalLineInput>, AppError> {
    let debitoren = find_account_by_number(db, 1100).await?;
    let vat_liability = find_account_by_number(db, 2200).await?;

    let mut journal_lines = Vec::new();

    // Debit: Debitoren (1100) for total
    journal_lines.push(JournalLineInput {
        account_id: debitoren.id.clone(),
        debit_amount: inv.total,
        credit_amount: Decimal::ZERO,
        description: Some("Accounts receivable".to_string()),
        vat_rate_id: None,
    });

    // Credit: revenue accounts per line
    for line in lines {
        if line.line_total > Decimal::ZERO {
            journal_lines.push(JournalLineInput {
                account_id: line.account_id.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: line.line_total,
                description: Some(line.description.clone()),
                vat_rate_id: line.vat_rate_id.clone(),
            });
        }
    }

    // Credit: VAT liability (2200) if applicable
    if inv.vat_amount > Decimal::ZERO {
        journal_lines.push(JournalLineInput {
            account_id: vat_liability.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: inv.vat_amount,
            description: Some("VAT liability".to_string()),
            vat_rate_id: None,
        });
    }

    Ok(journal_lines)
}
