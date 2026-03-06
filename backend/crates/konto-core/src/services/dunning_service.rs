use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::InvoiceStatus;
use konto_db::entities::{dunning_entry, dunning_level, invoice};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::dunning_repo::DunningRepo;
use rust_decimal::Decimal;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use super::journal_service::{JournalLineInput, JournalService};

pub struct DunningService;

#[derive(Debug, serde::Serialize)]
pub struct DunningEntryDetail {
    #[serde(flatten)]
    pub entry: dunning_entry::Model,
    pub level_name: String,
    pub level_number: i32,
}

#[derive(Debug)]
pub struct DunningRunResult {
    pub reminders_sent: u32,
    pub errors: Vec<String>,
}

impl DunningService {
    // ── Levels ──

    pub async fn list_levels(
        db: &DatabaseConnection,
    ) -> Result<Vec<dunning_level::Model>, AppError> {
        DunningRepo::find_all_levels(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update_level(
        db: &DatabaseConnection,
        id: &str,
        days_after_due: i32,
        fee_amount: Decimal,
        subject_template: &str,
        body_template: &str,
        is_active: bool,
    ) -> Result<dunning_level::Model, AppError> {
        let existing = DunningRepo::find_level_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Dunning level not found".into()))?;

        let now = Utc::now().naive_utc();
        let model = dunning_level::ActiveModel {
            id: Set(existing.id),
            level: Set(existing.level),
            days_after_due: Set(days_after_due),
            fee_amount: Set(fee_amount),
            subject_template: Set(subject_template.to_string()),
            body_template: Set(body_template.to_string()),
            is_active: Set(is_active),
            created_at: Set(existing.created_at),
            updated_at: Set(now),
        };

        DunningRepo::update_level(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    // ── Entries ──

    pub async fn get_dunning_history(
        db: &DatabaseConnection,
        invoice_id: &str,
    ) -> Result<Vec<DunningEntryDetail>, AppError> {
        let entries = DunningRepo::find_entries_by_invoice(db, invoice_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let levels = DunningRepo::find_all_levels(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let details = entries
            .into_iter()
            .map(|entry| {
                let level = levels.iter().find(|l| l.id == entry.dunning_level_id);
                DunningEntryDetail {
                    level_name: level
                        .map(|l| format!("Level {}", l.level))
                        .unwrap_or_else(|| "Unknown".into()),
                    level_number: level.map(|l| l.level).unwrap_or(0),
                    entry,
                }
            })
            .collect();

        Ok(details)
    }

    /// Send a manual reminder for a specific invoice at a given level.
    pub async fn send_reminder(
        db: &DatabaseConnection,
        invoice_id: &str,
        dunning_level_id: &str,
        send_email: bool,
        user_id: Option<&str>,
    ) -> Result<dunning_entry::Model, AppError> {
        let inv = invoice::Entity::find_by_id(invoice_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Invoice not found".into()))?;

        if inv.status != InvoiceStatus::Sent.as_str() {
            return Err(AppError::BadRequest(
                "Can only send reminders for sent (unpaid) invoices".into(),
            ));
        }

        let level = DunningRepo::find_level_by_id(db, dunning_level_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Dunning level not found".into()))?;

        let now = Utc::now().naive_utc();
        let mut journal_entry_id: Option<String> = None;

        // Create journal entry for fee if > 0
        if level.fee_amount > Decimal::ZERO {
            let je_id = Self::create_fee_journal(
                db,
                &inv,
                level.fee_amount,
                &format!("Dunning fee L{} for {}", level.level, inv.invoice_number.as_deref().unwrap_or("?")),
                user_id,
            )
            .await?;
            journal_entry_id = Some(je_id);
        }

        // Send email if requested
        let mut email_sent = false;
        if send_email {
            email_sent = Self::send_dunning_email(db, &inv, &level).await.is_ok();
        }

        let entry = dunning_entry::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            invoice_id: Set(invoice_id.to_string()),
            dunning_level_id: Set(dunning_level_id.to_string()),
            sent_at: Set(now),
            fee_amount: Set(level.fee_amount),
            email_sent: Set(email_sent),
            journal_entry_id: Set(journal_entry_id),
            notes: Set(None),
            created_at: Set(now),
        };

        DunningRepo::create_entry(db, entry)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Run automated dunning: find overdue invoices, apply appropriate levels.
    pub async fn run_dunning(
        db: &DatabaseConnection,
    ) -> Result<DunningRunResult, AppError> {
        let levels = DunningRepo::find_active_levels(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if levels.is_empty() {
            return Ok(DunningRunResult {
                reminders_sent: 0,
                errors: vec![],
            });
        }

        let today = Utc::now().naive_utc().date();
        let overdue_invoices = Self::get_overdue_invoices(db).await?;

        let mut sent = 0u32;
        let mut errors = Vec::new();

        for inv in &overdue_invoices {
            let days_overdue = (today - inv.due_date).num_days();

            // Find the highest applicable level
            let applicable_level = levels
                .iter()
                .filter(|l| i64::from(l.days_after_due) <= days_overdue)
                .next_back();

            let Some(level) = applicable_level else {
                continue;
            };

            // Check if already dunned at this level
            let latest = DunningRepo::find_latest_entry_for_invoice(db, &inv.id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            if latest.as_ref().is_some_and(|e| e.dunning_level_id == level.id) {
                continue; // Already dunned at this level
            }

            let now = Utc::now().naive_utc();
            let mut journal_entry_id: Option<String> = None;

            if level.fee_amount > Decimal::ZERO {
                match Self::create_fee_journal(
                    db,
                    inv,
                    level.fee_amount,
                    &format!("Dunning fee L{} for {}", level.level, inv.invoice_number.as_deref().unwrap_or("?")),
                    None,
                )
                .await
                {
                    Ok(id) => journal_entry_id = Some(id),
                    Err(e) => {
                        errors.push(format!("Fee journal for {}: {e}", inv.id));
                        continue;
                    }
                }
            }

            let email_sent = Self::send_dunning_email(db, inv, level).await.is_ok();

            let entry = dunning_entry::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                invoice_id: Set(inv.id.clone()),
                dunning_level_id: Set(level.id.clone()),
                sent_at: Set(now),
                fee_amount: Set(level.fee_amount),
                email_sent: Set(email_sent),
                journal_entry_id: Set(journal_entry_id),
                notes: Set(Some("Automated dunning run".to_string())),
                created_at: Set(now),
            };

            match DunningRepo::create_entry(db, entry).await {
                Ok(_) => sent += 1,
                Err(e) => errors.push(format!("Entry for {}: {e}", inv.id)),
            }
        }

        Ok(DunningRunResult {
            reminders_sent: sent,
            errors,
        })
    }

    /// Get all invoices that are overdue (status=sent, due_date < today).
    pub async fn get_overdue_invoices(
        db: &DatabaseConnection,
    ) -> Result<Vec<invoice::Model>, AppError> {
        let today = Utc::now().naive_utc().date();

        invoice::Entity::find()
            .filter(
                Condition::all()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::DueDate.lt(today)),
            )
            .order_by_asc(invoice::Column::DueDate)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Create journal entry for dunning fee: Debit Debitoren 1100, Credit Mahngebühren 8590
    async fn create_fee_journal(
        db: &DatabaseConnection,
        invoice: &invoice::Model,
        fee_amount: Decimal,
        description: &str,
        user_id: Option<&str>,
    ) -> Result<String, AppError> {
        // Look up accounts
        let debitoren = AccountRepo::find_by_number(db, 1100)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::Internal("Account 1100 Debitoren not found".into()))?;

        // Mahngebühren (dunning income) — use 8590 or fall back to 8000
        let fee_account = AccountRepo::find_by_number(db, 8590)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let fee_account = match fee_account {
            Some(a) => a,
            None => AccountRepo::find_by_number(db, 8000)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| {
                    AppError::Internal("No fee income account found (8590 or 8000)".into())
                })?,
        };

        let today = Utc::now().naive_utc().date();
        let lines = vec![
            JournalLineInput {
                account_id: debitoren.id.clone(),
                debit_amount: fee_amount,
                credit_amount: Decimal::ZERO,
                description: Some(description.to_string()),
                vat_rate_id: None,
            },
            JournalLineInput {
                account_id: fee_account.id.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: fee_amount,
                description: Some(description.to_string()),
                vat_rate_id: None,
            },
        ];

        let (entry, _) = JournalService::create(
            db,
            today,
            description,
            invoice.invoice_number.clone(),
            invoice.currency_id.clone(),
            None,
            user_id.map(|s| s.to_string()),
            lines,
        )
        .await?;

        // Auto-post the entry
        JournalService::post_entry(db, &entry.id).await?;

        Ok(entry.id)
    }

    /// Send email for a dunning entry using templates.
    async fn send_dunning_email(
        db: &DatabaseConnection,
        invoice: &invoice::Model,
        level: &dunning_level::Model,
    ) -> Result<(), AppError> {
        use konto_db::entities::contact;
        use super::email_service::EmailService;
        use super::settings_service::SettingsService;

        let contact = contact::Entity::find_by_id(&invoice.contact_id)
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Contact not found".into()))?;

        let email = contact.email.as_deref().unwrap_or_default();
        if email.is_empty() {
            return Err(AppError::BadRequest("Contact has no email address".into()));
        }

        let settings = SettingsService::get_or_create(db).await?;
        let company_name = settings.trade_name.as_deref().unwrap_or(&settings.legal_name);

        let subject = render_template(
            &level.subject_template,
            company_name,
            &contact.name1,
            invoice.invoice_number.as_deref().unwrap_or("—"),
            &invoice.total.to_string(),
            &invoice.due_date.to_string(),
        );

        let body = render_template(
            &level.body_template,
            company_name,
            &contact.name1,
            invoice.invoice_number.as_deref().unwrap_or("—"),
            &invoice.total.to_string(),
            &invoice.due_date.to_string(),
        );

        EmailService::send_email(db, email, &subject, &body, vec![]).await
    }
}

fn render_template(
    template: &str,
    company_name: &str,
    contact_name: &str,
    invoice_number: &str,
    amount: &str,
    due_date: &str,
) -> String {
    template
        .replace("{{company_name}}", company_name)
        .replace("{{contact_name}}", contact_name)
        .replace("{{invoice_number}}", invoice_number)
        .replace("{{amount}}", amount)
        .replace("{{due_date}}", due_date)
}
