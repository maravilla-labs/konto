use chrono::{Datelike, NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::{InvoiceStatus, JournalStatus, ProjectStatus as ProjectStatusEnum};
use konto_db::entities::{account, contact, invoice, journal_entry, journal_line, project};
use rust_decimal::Decimal;
use sea_orm::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub account_count: u64,
    pub active_contacts: u64,
    pub journal_entry_count: u64,
    pub active_projects: u64,
    pub revenue_mtd: Decimal,
    pub expenses_mtd: Decimal,
    pub cash_balance: Decimal,
    pub open_invoices_count: u64,
    pub total_outstanding: Decimal,
    pub recent_entries: Vec<RecentEntry>,
}

#[derive(Debug, Serialize)]
pub struct RecentEntry {
    pub id: String,
    pub date: String,
    pub reference: Option<String>,
    pub description: String,
    pub status: String,
}

pub struct DashboardService;

impl DashboardService {
    pub async fn get_stats(db: &DatabaseConnection) -> Result<DashboardStats, AppError> {
        let account_count = account::Entity::find()
            .count(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let active_contacts = contact::Entity::find()
            .filter(contact::Column::IsActive.eq(true))
            .count(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let journal_entry_count = journal_entry::Entity::find()
            .count(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let active_projects = project::Entity::find()
            .filter(project::Column::Status.eq(ProjectStatusEnum::Active.as_str()))
            .count(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let now = Utc::now().naive_utc().date();
        let month_start =
            NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);

        // Revenue MTD: credits to accounts 3000-3999 for posted entries this month
        let revenue_mtd = Self::sum_by_account_range(
            db,
            3000,
            3999,
            month_start,
            now,
            false,
        )
        .await?;

        // Expenses MTD: debits to accounts 4000-8999 for posted entries this month
        let expenses_mtd = Self::sum_by_account_range(
            db,
            4000,
            8999,
            month_start,
            now,
            true,
        )
        .await?;

        // Cash balance: balance of accounts 1000-1099
        let cash_balance = Self::cash_balance(db).await?;

        // Open invoices (sent + overdue)
        let open_invoices_count = invoice::Entity::find()
            .filter(
                Condition::any()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
            )
            .count(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let open_invoices: Vec<invoice::Model> = invoice::Entity::find()
            .filter(
                Condition::any()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
            )
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let total_outstanding = open_invoices
            .iter()
            .fold(Decimal::ZERO, |acc, inv| acc + inv.total);

        // Last 10 journal entries
        let recent: Vec<journal_entry::Model> = journal_entry::Entity::find()
            .order_by_desc(journal_entry::Column::Date)
            .limit(10)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let recent_entries = recent
            .into_iter()
            .map(|e| RecentEntry {
                id: e.id,
                date: e.date.to_string(),
                reference: e.reference,
                description: e.description,
                status: e.status,
            })
            .collect();

        Ok(DashboardStats {
            account_count,
            active_contacts,
            journal_entry_count,
            active_projects,
            revenue_mtd,
            expenses_mtd,
            cash_balance,
            open_invoices_count,
            total_outstanding,
            recent_entries,
        })
    }

    async fn sum_by_account_range(
        db: &DatabaseConnection,
        from_num: i32,
        to_num: i32,
        from_date: NaiveDate,
        to_date: NaiveDate,
        sum_debits: bool,
    ) -> Result<Decimal, AppError> {
        let rows: Vec<journal_line::Model> = journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .inner_join(account::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .filter(journal_entry::Column::Date.gte(from_date))
            .filter(journal_entry::Column::Date.lte(to_date))
            .filter(account::Column::Number.gte(from_num))
            .filter(account::Column::Number.lte(to_num))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let total = rows.iter().fold(Decimal::ZERO, |acc, line| {
            if sum_debits {
                acc + line.debit_amount
            } else {
                acc + line.credit_amount
            }
        });

        Ok(total)
    }

    async fn cash_balance(db: &DatabaseConnection) -> Result<Decimal, AppError> {
        let rows: Vec<journal_line::Model> = journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .inner_join(account::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .filter(account::Column::Number.gte(1000))
            .filter(account::Column::Number.lte(1099))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let balance = rows.iter().fold(Decimal::ZERO, |acc, line| {
            acc + line.debit_amount - line.credit_amount
        });

        Ok(balance)
    }
}
