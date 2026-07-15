use chrono::Utc;
use konto_common::enums::InvoiceStatus;
use konto_common::error::AppError;
use konto_db::entities::{account, bank_account, currency, invoice, invoice_line, invoice_payment};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use konto_db::repository::invoice_payment_repo::InvoicePaymentRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use konto_db::repository::journal_repo::JournalRepo;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use super::default_account_service::DefaultAccountService;
use super::exchange_rate_service::ExchangeRateService;
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

    // If the invoice is in a foreign currency, resolve the rate to the base
    // (primary) currency as of the issue date; same-currency invoices are
    // unaffected (exchange_rate stays None, booked at face value as before).
    let exchange_rate = resolve_invoice_exchange_rate(db, &inv).await?;

    // Create journal entry: Debit 1100 (Debitoren), Credit revenue + VAT
    let journal_lines = build_send_journal_lines(db, &inv, &lines, exchange_rate).await?;
    let description = format!("Invoice {inv_number}");

    let (entry, _) = JournalService::create(
        db,
        inv.issue_date,
        &description,
        Some(inv_number.clone()),
        inv.currency_id.clone(),
        exchange_rate,
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
    bank_account_id: &str,
    actual_base_amount: Option<Decimal>,
    user_id: &str,
) -> Result<invoice::Model, AppError> {
    let inv = InvoiceService::get_invoice_model(db, id).await?;
    if inv.status != InvoiceStatus::Sent.as_str() && inv.status != InvoiceStatus::Overdue.as_str() {
        return Err(AppError::Validation(
            "Only sent or overdue invoices can be marked as paid".to_string(),
        ));
    }

    let bank_account = resolve_bank_account(db, bank_account_id).await?;
    let (journal_lines, entry_rate) =
        build_payment_journal_lines(db, &inv, &bank_account, inv.total, actual_base_amount).await?;

    let inv_num = inv.invoice_number.as_deref().unwrap_or(&inv.id);
    let description = format!("Payment for {inv_num}");

    let (entry, _) = JournalService::create(
        db,
        payment_date,
        &description,
        Some(format!("PAY-{inv_num}")),
        inv.currency_id.clone(),
        entry_rate,
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
    bank_account_id: &str,
    actual_base_amount: Option<Decimal>,
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

    let bank_account = resolve_bank_account(db, bank_account_id).await?;
    // `amount` is denominated in the invoice's own currency (its face value) —
    // for a base-currency invoice that's identical to the base amount, so
    // existing single-currency behavior is unchanged.
    let (journal_lines, entry_rate) =
        build_payment_journal_lines(db, &inv, &bank_account, amount, actual_base_amount).await?;

    // Create journal entry for this payment
    let inv_num = inv.invoice_number.as_deref().unwrap_or(&inv.id);
    let description = format!("Partial payment for {inv_num}");

    let (entry, _) = JournalService::create(
        db,
        payment_date,
        &description,
        Some(format!("PAY-{inv_num}-{}", Uuid::new_v4().to_string().split('-').next().unwrap_or("x"))),
        inv.currency_id.clone(),
        entry_rate,
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

async fn resolve_bank_account(
    db: &DatabaseConnection,
    bank_account_id: &str,
) -> Result<bank_account::Model, AppError> {
    let account = BankAccountRepo::find_by_id(db, bank_account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Bank account not found".to_string()))?;
    if account.account_id.is_none() {
        return Err(AppError::Validation(
            "Bank account has no linked ledger account".to_string(),
        ));
    }
    Ok(account)
}

/// The exchange rate (base units per 1 invoice-currency unit) recorded on the
/// invoice's original send journal entry — payments must clear the receivable
/// at that same rate, not a freshly looked-up "latest" rate, or the receivable
/// would never zero out exactly.
async fn original_invoice_exchange_rate(
    db: &DatabaseConnection,
    inv: &invoice::Model,
) -> Result<Option<Decimal>, AppError> {
    let Some(entry_id) = inv.journal_entry_id.as_deref() else {
        return Ok(None);
    };
    let entry = JournalRepo::find_entry_by_id(db, entry_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(entry.and_then(|e| e.exchange_rate))
}

/// Build the journal lines that clear (all or part of) an invoice's receivable
/// into a bank account, handling currency:
/// - bank account currency matches the invoice's currency (or both are the base
///   currency): the value just moves at the invoice's original booking rate,
///   no gain/loss recognized yet.
/// - bank account currency differs (money was converted on the way in, e.g. a
///   EUR invoice paid directly into a CHF account): the caller must supply the
///   real base-currency amount received, and the difference against the
///   original booking rate is realized immediately as FX gain/loss.
async fn build_payment_journal_lines(
    db: &DatabaseConnection,
    inv: &invoice::Model,
    bank_account: &bank_account::Model,
    amount_in_invoice_currency: Decimal,
    actual_base_amount: Option<Decimal>,
) -> Result<(Vec<JournalLineInput>, Option<Decimal>), AppError> {
    let debitoren = find_account_by_number(db, 1100).await?;
    let bank_gl_account_id = bank_account
        .account_id
        .clone()
        .expect("checked by resolve_bank_account");

    let original_rate = original_invoice_exchange_rate(db, inv).await?;
    let rate = original_rate.unwrap_or(Decimal::ONE);
    let base_amount_cleared = (amount_in_invoice_currency * rate).round_dp(2);

    let same_currency = match (&bank_account.currency_id, &inv.currency_id) {
        (Some(bank_cur), Some(inv_cur)) => bank_cur == inv_cur,
        (None, None) => true,
        _ => false,
    };

    if same_currency {
        let lines = vec![
            JournalLineInput {
                account_id: bank_gl_account_id,
                debit_amount: base_amount_cleared,
                credit_amount: Decimal::ZERO,
                description: Some("Payment received".to_string()),
                vat_rate_id: None,
            },
            JournalLineInput {
                account_id: debitoren.id.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: base_amount_cleared,
                description: Some("Payment received".to_string()),
                vat_rate_id: None,
            },
        ];
        return Ok((lines, original_rate));
    }

    let actual = actual_base_amount.ok_or_else(|| {
        AppError::Validation(
            "This payment converts currency (the bank account's currency differs from the \
             invoice's) — the actual amount received must be provided"
                .to_string(),
        )
    })?;
    let diff = (actual - base_amount_cleared).round_dp(2);

    let mut lines = vec![
        JournalLineInput {
            account_id: bank_gl_account_id,
            debit_amount: actual,
            credit_amount: Decimal::ZERO,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
        JournalLineInput {
            account_id: debitoren.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: base_amount_cleared,
            description: Some("Payment received".to_string()),
            vat_rate_id: None,
        },
    ];

    if diff > Decimal::ZERO {
        let fx_gain = DefaultAccountService::get_by_key(db, "fx_gain_account")
            .await?
            .and_then(|d| d.account_id)
            .ok_or_else(|| AppError::Validation("fx_gain_account not configured".to_string()))?;
        lines.push(JournalLineInput {
            account_id: fx_gain,
            debit_amount: Decimal::ZERO,
            credit_amount: diff,
            description: Some("Realized FX gain".to_string()),
            vat_rate_id: None,
        });
    } else if diff < Decimal::ZERO {
        let fx_loss = DefaultAccountService::get_by_key(db, "fx_loss_account")
            .await?
            .and_then(|d| d.account_id)
            .ok_or_else(|| AppError::Validation("fx_loss_account not configured".to_string()))?;
        lines.push(JournalLineInput {
            account_id: fx_loss,
            debit_amount: diff.abs(),
            credit_amount: Decimal::ZERO,
            description: Some("Realized FX loss".to_string()),
            vat_rate_id: None,
        });
    }

    // Mixed rates within one entry (original booking rate for the receivable vs.
    // the actual conversion outcome for the bank/gain-loss lines) — skip the
    // uniform per-line currency_amount annotation rather than mislabel it.
    Ok((lines, None))
}

/// Look up the company's primary (base/ledger) currency.
async fn find_primary_currency(db: &DatabaseConnection) -> Result<currency::Model, AppError> {
    currency::Entity::find()
        .filter(currency::Column::IsPrimary.eq(true))
        .one(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("No primary currency configured".to_string()))
}

/// Resolve the exchange rate (base units per 1 invoice-currency unit) to apply when
/// booking this invoice. Returns `None` when the invoice has no currency set or is
/// already in the primary/base currency — same-currency invoices book at face value.
async fn resolve_invoice_exchange_rate(
    db: &DatabaseConnection,
    inv: &invoice::Model,
) -> Result<Option<Decimal>, AppError> {
    let Some(invoice_currency_id) = inv.currency_id.as_deref() else {
        return Ok(None);
    };

    let primary = find_primary_currency(db).await?;
    if invoice_currency_id == primary.id {
        return Ok(None);
    }

    let rate = ExchangeRateService::get_latest(
        db,
        invoice_currency_id,
        &primary.id,
        Some(inv.issue_date),
    )
    .await?;
    Ok(Some(rate.rate))
}

async fn build_send_journal_lines(
    db: &DatabaseConnection,
    inv: &invoice::Model,
    lines: &[invoice_line::Model],
    exchange_rate: Option<Decimal>,
) -> Result<Vec<JournalLineInput>, AppError> {
    let debitoren = find_account_by_number(db, 1100).await?;
    let vat_liability = find_account_by_number(db, 2200).await?;
    let rate = exchange_rate.unwrap_or(Decimal::ONE);

    let mut journal_lines = Vec::new();
    let mut total_credit = Decimal::ZERO;

    // Credit: revenue accounts per line, converted to base currency
    for line in lines {
        if line.line_total > Decimal::ZERO {
            let amount = (line.line_total * rate).round_dp(2);
            total_credit += amount;
            journal_lines.push(JournalLineInput {
                account_id: line.account_id.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: amount,
                description: Some(line.description.clone()),
                vat_rate_id: line.vat_rate_id.clone(),
            });
        }
    }

    // Credit: VAT liability (2200) if applicable, converted to base currency
    if inv.vat_amount > Decimal::ZERO {
        let amount = (inv.vat_amount * rate).round_dp(2);
        total_credit += amount;
        journal_lines.push(JournalLineInput {
            account_id: vat_liability.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: amount,
            description: Some("VAT liability".to_string()),
            vat_rate_id: None,
        });
    }

    // Debit: Debitoren (1100) for the sum of the (rounded) credit lines above —
    // guarantees debit == credit exactly regardless of per-line FX rounding.
    journal_lines.insert(
        0,
        JournalLineInput {
            account_id: debitoren.id.clone(),
            debit_amount: total_credit,
            credit_amount: Decimal::ZERO,
            description: Some("Accounts receivable".to_string()),
            vat_rate_id: None,
        },
    );

    Ok(journal_lines)
}
