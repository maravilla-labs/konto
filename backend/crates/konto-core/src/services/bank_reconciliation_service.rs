use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::InvoiceStatus;
use konto_common::enums::BankTransactionStatus;
use konto_db::entities::bank_transaction;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use konto_db::repository::bank_transaction_repo::BankTransactionRepo;
use konto_db::repository::invoice_repo::InvoiceRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};

use super::expense_workflow;
use super::invoice_workflow;
use super::journal_service::{JournalLineInput, JournalService};
use super::qr_bill;

pub struct BankReconciliationService;

/// Result of an auto-match run.
pub struct AutoMatchResult {
    pub matched_count: u64,
    pub unmatched_count: u64,
}

impl BankReconciliationService {
    /// Auto-match unmatched bank transactions for a given bank account.
    /// Strategy: 1) SCOR reference → invoice, 2) amount + date proximity.
    pub async fn auto_match(
        db: &DatabaseConnection,
        bank_account_id: &str,
        user_id: &str,
    ) -> Result<AutoMatchResult, AppError> {
        let bank_account = BankAccountRepo::find_by_id(db, bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Bank account not found".into()))?;

        let payment_account_id = bank_account
            .account_id
            .as_deref()
            .ok_or_else(|| {
                AppError::Validation("Bank account has no linked ledger account".into())
            })?;

        let transactions = BankTransactionRepo::find_unmatched_by_account(db, bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Load open invoices (sent or overdue) for SCOR matching
        let open_invoices = InvoiceRepo::find_by_status_list(db, &[InvoiceStatus::Sent.as_str(), InvoiceStatus::Overdue.as_str()])
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut matched = 0u64;

        for tx in &transactions {
            // Only try to match credit (incoming) transactions to invoices
            if tx.amount > Decimal::ZERO {
                if let Some(ref tx_ref) = tx.reference {
                    // Strategy 1: SCOR reference match
                    let found = open_invoices.iter().find(|inv| {
                        if let Some(ref inv_num) = inv.invoice_number {
                            let scor = qr_bill::generate_creditor_reference(inv_num);
                            tx_ref == &scor || tx_ref.contains(&scor)
                        } else {
                            false
                        }
                    });

                    if let Some(inv) = found {
                        // Trigger invoice pay workflow
                        let pay_result = invoice_workflow::mark_paid(
                            db,
                            &inv.id,
                            tx.transaction_date,
                            payment_account_id,
                            user_id,
                        )
                        .await;

                        if pay_result.is_ok() {
                            update_tx_matched(db, &tx.id, "invoice", &inv.id).await?;
                            matched += 1;
                            continue;
                        }
                    }
                }

                // Strategy 2: exact amount match on open invoices
                let amount_match = open_invoices.iter().find(|inv| inv.total == tx.amount);
                if let Some(inv) = amount_match {
                    let pay_result = invoice_workflow::mark_paid(
                        db,
                        &inv.id,
                        tx.transaction_date,
                        payment_account_id,
                        user_id,
                    )
                    .await;

                    if pay_result.is_ok() {
                        update_tx_matched(db, &tx.id, "invoice", &inv.id).await?;
                        matched += 1;
                        continue;
                    }
                }
            }

            // Debit (outgoing) transactions could match expenses — skip for auto
        }

        let unmatched = transactions.len() as u64 - matched;
        Ok(AutoMatchResult {
            matched_count: matched,
            unmatched_count: unmatched,
        })
    }

    /// Manually match a bank transaction to an invoice or expense.
    pub async fn manual_match(
        db: &DatabaseConnection,
        transaction_id: &str,
        target_type: &str,
        target_id: &str,
        user_id: &str,
    ) -> Result<bank_transaction::Model, AppError> {
        let tx = BankTransactionRepo::find_by_id(db, transaction_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

        if tx.status != BankTransactionStatus::Unmatched.as_str() {
            return Err(AppError::Validation(
                "Transaction is already matched or ignored".into(),
            ));
        }

        let bank_account = BankAccountRepo::find_by_id(db, &tx.bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Bank account not found".into()))?;

        let payment_account_id = bank_account.account_id.as_deref().ok_or_else(|| {
            AppError::Validation("Bank account has no linked ledger account".into())
        })?;

        match target_type {
            "invoice" => {
                invoice_workflow::mark_paid(
                    db,
                    target_id,
                    tx.transaction_date,
                    payment_account_id,
                    user_id,
                )
                .await?;
                update_tx_matched(db, transaction_id, "invoice", target_id).await
            }
            "expense" => {
                expense_workflow::pay_expense(db, target_id, payment_account_id, user_id)
                    .await?;
                update_tx_matched(db, transaction_id, "expense", target_id).await
            }
            _ => Err(AppError::Validation(
                "target_type must be 'invoice' or 'expense'".into(),
            )),
        }
    }

    /// Create a journal entry from an unmatched transaction.
    pub async fn create_journal_entry(
        db: &DatabaseConnection,
        transaction_id: &str,
        debit_account_id: &str,
        credit_account_id: &str,
        user_id: &str,
    ) -> Result<bank_transaction::Model, AppError> {
        let tx = BankTransactionRepo::find_by_id(db, transaction_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

        if tx.status != BankTransactionStatus::Unmatched.as_str() {
            return Err(AppError::Validation(
                "Transaction is already matched or ignored".into(),
            ));
        }

        let abs_amount = tx.amount.abs();
        let journal_lines = vec![
            JournalLineInput {
                account_id: debit_account_id.to_string(),
                debit_amount: abs_amount,
                credit_amount: Decimal::ZERO,
                description: Some(tx.description.clone()),
                vat_rate_id: None,
            },
            JournalLineInput {
                account_id: credit_account_id.to_string(),
                debit_amount: Decimal::ZERO,
                credit_amount: abs_amount,
                description: Some(tx.description.clone()),
                vat_rate_id: None,
            },
        ];

        let description = format!("Bank: {}", tx.description);
        let (entry, _) = JournalService::create(
            db,
            tx.transaction_date,
            &description,
            tx.bank_reference.clone(),
            None,
            None,
            Some(user_id.to_string()),
            journal_lines,
        )
        .await?;

        JournalService::post_entry(db, &entry.id).await?;

        // Update transaction status
        let now = Utc::now().naive_utc();
        let mut model: bank_transaction::ActiveModel = tx.into();
        model.status = Set(BankTransactionStatus::Matched.to_string());
        model.matched_journal_entry_id = Set(Some(entry.id));
        model.created_at = Set(now);

        BankTransactionRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Mark a transaction as ignored.
    pub async fn ignore(
        db: &DatabaseConnection,
        transaction_id: &str,
    ) -> Result<bank_transaction::Model, AppError> {
        let tx = BankTransactionRepo::find_by_id(db, transaction_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

        if tx.status != BankTransactionStatus::Unmatched.as_str() {
            return Err(AppError::Validation(
                "Transaction is already matched or ignored".into(),
            ));
        }

        let mut model: bank_transaction::ActiveModel = tx.into();
        model.status = Set(BankTransactionStatus::Ignored.to_string());

        BankTransactionRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}

async fn update_tx_matched(
    db: &DatabaseConnection,
    tx_id: &str,
    target_type: &str,
    target_id: &str,
) -> Result<bank_transaction::Model, AppError> {
    let tx = BankTransactionRepo::find_by_id(db, tx_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Transaction not found".into()))?;

    let mut model: bank_transaction::ActiveModel = tx.into();
    model.status = Set(BankTransactionStatus::Matched.to_string());

    match target_type {
        "invoice" => model.matched_invoice_id = Set(Some(target_id.to_string())),
        "expense" => model.matched_expense_id = Set(Some(target_id.to_string())),
        _ => {}
    }

    BankTransactionRepo::update(db, model)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
}
