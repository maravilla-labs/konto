use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_db::entities::bank_transaction;
use konto_db::repository::bank_transaction_repo::BankTransactionRepo;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use std::str::FromStr;
use uuid::Uuid;

/// A parsed CAMT.053 entry before DB insert.
#[derive(Debug, Clone)]
pub struct ParsedTransaction {
    pub transaction_date: NaiveDate,
    pub value_date: NaiveDate,
    pub amount: Decimal,
    pub currency: String,
    pub description: String,
    pub counterparty_name: Option<String>,
    pub counterparty_iban: Option<String>,
    pub reference: Option<String>,
    pub bank_reference: Option<String>,
}

pub struct BankImportService;

impl BankImportService {
    /// Parse CAMT.053 XML (ISO 20022 BkToCstmrStmt) into transactions.
    pub fn parse_camt053(xml: &str) -> Result<Vec<ParsedTransaction>, AppError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut transactions = Vec::new();
        let mut buf = Vec::new();
        let mut path: Vec<String> = Vec::new();

        // Current entry state
        let mut in_ntry = false;
        let mut entry = EntryBuilder::default();
        let mut current_text = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                    path.push(name.clone());
                    if name == "Ntry" {
                        in_ntry = true;
                        entry = EntryBuilder::default();
                    }
                    current_text.clear();
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                    if in_ntry {
                        entry.handle_end(&path, &current_text);
                    }
                    if name == "Ntry" {
                        let built = std::mem::take(&mut entry).build();
                        if let Some(tx) = built {
                            transactions.push(tx);
                        }
                        in_ntry = false;
                    }
                    path.pop();
                    current_text.clear();
                }
                Ok(Event::Text(e)) => {
                    if let Ok(t) = e.unescape() {
                        current_text = t.to_string();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(AppError::Validation(format!(
                        "Invalid CAMT.053 XML: {e}"
                    )));
                }
                _ => {}
            }
            buf.clear();
        }

        Ok(transactions)
    }

    /// Import parsed transactions into the database.
    pub async fn import(
        db: &DatabaseConnection,
        bank_account_id: &str,
        transactions: &[ParsedTransaction],
    ) -> Result<u64, AppError> {
        let now = Utc::now().naive_utc();
        let batch_id = Uuid::new_v4().to_string();
        let mut count = 0u64;

        for tx in transactions {
            let model = bank_transaction::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                bank_account_id: Set(bank_account_id.to_string()),
                transaction_date: Set(tx.transaction_date),
                value_date: Set(tx.value_date),
                amount: Set(tx.amount),
                currency_id: Set(None),
                description: Set(tx.description.clone()),
                counterparty_name: Set(tx.counterparty_name.clone()),
                counterparty_iban: Set(tx.counterparty_iban.clone()),
                reference: Set(tx.reference.clone()),
                bank_reference: Set(tx.bank_reference.clone()),
                status: Set("unmatched".to_string()),
                matched_invoice_id: Set(None),
                matched_expense_id: Set(None),
                matched_journal_entry_id: Set(None),
                import_batch_id: Set(Some(batch_id.clone())),
                created_at: Set(now),
            };

            BankTransactionRepo::create(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            count += 1;
        }

        Ok(count)
    }
}

/// Builder for assembling a ParsedTransaction from XML paths.
#[derive(Default)]
struct EntryBuilder {
    booking_date: Option<NaiveDate>,
    value_date: Option<NaiveDate>,
    amount: Option<Decimal>,
    currency: Option<String>,
    credit_debit: Option<String>,
    description: Option<String>,
    counterparty_name: Option<String>,
    counterparty_iban: Option<String>,
    reference: Option<String>,
    bank_reference: Option<String>,
}

impl EntryBuilder {
    fn handle_end(&mut self, path: &[String], text: &str) {
        if text.is_empty() {
            return;
        }
        let tail = path_tail(path);

        match tail.as_str() {
            // Amount: Ntry > Amt
            "Ntry|Amt" => {
                self.amount = Decimal::from_str(text).ok();
            }
            // Credit/Debit indicator
            "Ntry|CdtDbtInd" => {
                self.credit_debit = Some(text.to_string());
            }
            // Booking date: Ntry > BookgDt > Dt
            "Ntry|BookgDt|Dt" => {
                self.booking_date = NaiveDate::parse_from_str(text, "%Y-%m-%d").ok();
            }
            // Value date: Ntry > ValDt > Dt
            "Ntry|ValDt|Dt" => {
                self.value_date = NaiveDate::parse_from_str(text, "%Y-%m-%d").ok();
            }
            // Bank reference: Ntry > AcctSvcrRef
            "Ntry|AcctSvcrRef" => {
                self.bank_reference = Some(text.to_string());
            }
            _ => {}
        }

        // Deeper paths for transaction details (NtryDtls > TxDtls)
        let deep = path_from_ntry(path);
        match deep.as_str() {
            // Remittance info - structured reference
            s if s.ends_with("RmtInf|Strd|CdtrRefInf|Ref") => {
                self.reference = Some(text.to_string());
            }
            // Remittance info - unstructured
            s if s.ends_with("RmtInf|Ustrd") => {
                if self.description.is_none() {
                    self.description = Some(text.to_string());
                }
            }
            // Additional info
            s if s.ends_with("AddtlNtryInf") => {
                if self.description.is_none() {
                    self.description = Some(text.to_string());
                }
            }
            // Counterparty name
            s if s.contains("RltdPties|Dbtr|Nm") || s.contains("RltdPties|Cdtr|Nm") => {
                self.counterparty_name = Some(text.to_string());
            }
            // Counterparty IBAN
            s if s.contains("RltdPties|DbtrAcct|Id|IBAN")
                || s.contains("RltdPties|CdtrAcct|Id|IBAN") =>
            {
                self.counterparty_iban = Some(text.to_string());
            }
            _ => {}
        }

        // Currency from amount attribute captured separately would need attr parsing;
        // use default CHF if not set
        if self.currency.is_none() {
            self.currency = Some("CHF".to_string());
        }
    }

    fn build(self) -> Option<ParsedTransaction> {
        let booking = self.booking_date?;
        let value = self.value_date.unwrap_or(booking);
        let mut amount = self.amount?;

        // Negate for debit entries
        if self.credit_debit.as_deref() == Some("DBIT") {
            amount = -amount;
        }

        Some(ParsedTransaction {
            transaction_date: booking,
            value_date: value,
            amount,
            currency: self.currency.unwrap_or_else(|| "CHF".into()),
            description: self.description.unwrap_or_default(),
            counterparty_name: self.counterparty_name,
            counterparty_iban: self.counterparty_iban,
            reference: self.reference,
            bank_reference: self.bank_reference,
        })
    }
}

/// Build a |-separated tail of path elements starting from Ntry.
fn path_tail(path: &[String]) -> String {
    if let Some(pos) = path.iter().position(|s| s == "Ntry") {
        path[pos..].join("|")
    } else {
        path.join("|")
    }
}

fn path_from_ntry(path: &[String]) -> String {
    path_tail(path)
}
