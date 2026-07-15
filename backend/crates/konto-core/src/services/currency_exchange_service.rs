use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::currency_exchange;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use konto_db::repository::currency_exchange_repo::CurrencyExchangeRepo;
use konto_db::repository::journal_repo::JournalRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::default_account_service::DefaultAccountService;
use super::journal_service::{JournalLineInput, JournalService};

pub struct CurrencyExchangeService;

impl CurrencyExchangeService {
    pub async fn list(
        db: &DatabaseConnection,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<currency_exchange::Model>, u64), AppError> {
        CurrencyExchangeRepo::find_paginated(db, page, per_page)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Record a transfer of funds between two bank accounts of different currencies
    /// at the real rate the bank actually gave (i.e. `to_amount`, the base-currency
    /// value received) — this is the point at which FX gain/loss is realized, per
    /// OR Art. 957a: money just sitting in a matching-currency account carries no
    /// gain/loss, only an actual conversion does.
    #[allow(clippy::too_many_arguments)]
    pub async fn record_transfer(
        db: &DatabaseConnection,
        from_bank_account_id: &str,
        to_bank_account_id: &str,
        from_amount: Decimal,
        to_amount: Decimal,
        date: NaiveDate,
        notes: Option<String>,
        user_id: &str,
    ) -> Result<currency_exchange::Model, AppError> {
        if from_amount <= Decimal::ZERO || to_amount <= Decimal::ZERO {
            return Err(AppError::Validation(
                "Transfer amounts must be positive".to_string(),
            ));
        }
        if from_bank_account_id == to_bank_account_id {
            return Err(AppError::Validation(
                "Source and destination bank account must differ".to_string(),
            ));
        }

        let from_account = BankAccountRepo::find_by_id(db, from_bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Source bank account not found".to_string()))?;
        let to_account = BankAccountRepo::find_by_id(db, to_bank_account_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Destination bank account not found".to_string()))?;

        let from_gl = from_account
            .account_id
            .clone()
            .ok_or_else(|| AppError::Validation("Source bank account has no linked ledger account".to_string()))?;
        let to_gl = to_account
            .account_id
            .clone()
            .ok_or_else(|| AppError::Validation("Destination bank account has no linked ledger account".to_string()))?;

        // Weighted-average base-currency cost of the funds leaving the source
        // account, derived from its own FX-annotated journal history — no
        // separate running-balance column needed.
        let avg_rate = CurrencyExchangeRepo::weighted_average_rate(db, &from_gl, date)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .unwrap_or(Decimal::ONE);
        let book_value_removed = (from_amount * avg_rate).round_dp(2);
        let diff = (to_amount - book_value_removed).round_dp(2);

        let description = format!("Currency exchange: {} → {}", from_account.name, to_account.name);

        let mut lines = vec![
            JournalLineInput {
                account_id: to_gl.clone(),
                debit_amount: to_amount,
                credit_amount: Decimal::ZERO,
                description: Some(description.clone()),
                vat_rate_id: None,
            },
            JournalLineInput {
                account_id: from_gl.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: book_value_removed,
                description: Some(description.clone()),
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

        // The two bank legs use different rates (source: historical weighted-average,
        // destination: today's actual transfer) — not a single entry-level rate, so
        // skip the automatic uniform derivation and annotate the source leg directly.
        let (entry, created_lines) = JournalService::create(
            db,
            date,
            &description,
            None,
            from_account.currency_id.clone(),
            None,
            Some(user_id.to_string()),
            lines,
        )
        .await?;

        JournalService::post_entry(db, &entry.id).await?;

        if let Some(from_line) = created_lines.iter().find(|l| l.account_id == from_gl) {
            let mut model: konto_db::entities::journal_line::ActiveModel = from_line.clone().into();
            model.currency_amount = Set(Some(from_amount));
            model.base_currency_amount = Set(Some(book_value_removed));
            JournalRepo::update_line(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        let now = Utc::now().naive_utc();
        let record = currency_exchange::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            from_bank_account_id: Set(from_bank_account_id.to_string()),
            to_bank_account_id: Set(to_bank_account_id.to_string()),
            from_amount: Set(from_amount),
            to_amount: Set(to_amount),
            date: Set(date),
            journal_entry_id: Set(entry.id),
            notes: Set(notes),
            created_at: Set(now),
            updated_at: Set(now),
        };

        CurrencyExchangeRepo::create(db, record)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}
