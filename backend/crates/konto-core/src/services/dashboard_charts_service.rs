use chrono::{Datelike, NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::{InvoiceStatus, JournalStatus};
use konto_db::entities::{account, contact, invoice, journal_entry, journal_line};
use rust_decimal::Decimal;
use sea_orm::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct MonthlyAmount {
    pub month: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct InvoiceAgingBucket {
    pub status: String,
    pub count: i64,
    pub total: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct OutstandingContact {
    pub contact_id: String,
    pub contact_name: String,
    pub outstanding_amount: Decimal,
    pub invoice_count: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct OverviewMonth {
    pub month: String,
    pub income: Decimal,
    pub expenses: Decimal,
    pub cumulative_income: Decimal,
    pub cumulative_expenses: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct OverviewData {
    pub year: i32,
    pub months: Vec<OverviewMonth>,
    pub total_income: Decimal,
    pub total_expenses: Decimal,
    pub difference: Decimal,
    pub available_years: Vec<i32>,
}

pub struct DashboardChartsService;

impl DashboardChartsService {
    pub async fn monthly_revenue(
        db: &DatabaseConnection,
        months: u32,
    ) -> Result<Vec<MonthlyAmount>, AppError> {
        Self::monthly_by_account_range(db, months, 3000, 3999, false).await
    }

    pub async fn monthly_expenses(
        db: &DatabaseConnection,
        months: u32,
    ) -> Result<Vec<MonthlyAmount>, AppError> {
        Self::monthly_by_account_range(db, months, 4000, 8999, true).await
    }

    async fn monthly_by_account_range(
        db: &DatabaseConnection,
        months: u32,
        from_num: i32,
        to_num: i32,
        sum_debits: bool,
    ) -> Result<Vec<MonthlyAmount>, AppError> {
        let now = Utc::now().naive_utc().date();
        let start_month = now.year() * 12 + now.month() as i32 - months as i32;
        let start_year = start_month.div_euclid(12);
        let start_m = start_month.rem_euclid(12) + 1;
        let from_date =
            NaiveDate::from_ymd_opt(start_year, start_m as u32, 1).unwrap_or(now);

        // Single query: inner_join for filtering, find_also_related for getting entry dates
        let lines_with_entries: Vec<(journal_line::Model, Option<journal_entry::Model>)> =
            journal_line::Entity::find()
                .inner_join(account::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(account::Column::Number.gte(from_num))
                .filter(account::Column::Number.lte(to_num))
                .find_also_related(journal_entry::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        use std::collections::BTreeMap;
        let mut monthly: BTreeMap<String, Decimal> = BTreeMap::new();

        // Pre-fill all months with zero
        for i in 0..months {
            let m_offset = now.year() * 12 + now.month() as i32 - (months as i32 - 1 - i as i32);
            let y = m_offset.div_euclid(12);
            let m = m_offset.rem_euclid(12) + 1;
            let key = format!("{:04}-{:02}", y, m);
            monthly.entry(key).or_insert(Decimal::ZERO);
        }

        for (line, entry_opt) in &lines_with_entries {
            if let Some(entry) = entry_opt {
                let key = format!(
                    "{:04}-{:02}",
                    entry.date.year(),
                    entry.date.month()
                );
                let amount = if sum_debits {
                    line.debit_amount
                } else {
                    line.credit_amount
                };
                *monthly.entry(key).or_insert(Decimal::ZERO) += amount;
            }
        }

        Ok(monthly
            .into_iter()
            .map(|(month, amount)| MonthlyAmount { month, amount })
            .collect())
    }

    pub async fn overview(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<OverviewData, AppError> {
        let from_date = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or_else(|| AppError::Validation("Invalid year".to_string()))?;
        let to_date = NaiveDate::from_ymd_opt(year, 12, 31)
            .ok_or_else(|| AppError::Validation("Invalid year".to_string()))?;

        // Query journal lines: inner_join account for filter, find_also_related for entry dates
        let income_lines: Vec<(journal_line::Model, Option<journal_entry::Model>)> =
            journal_line::Entity::find()
                .inner_join(account::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(journal_entry::Column::Date.lte(to_date))
                .filter(account::Column::Number.gte(3000))
                .filter(account::Column::Number.lte(3999))
                .find_also_related(journal_entry::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        let expense_lines: Vec<(journal_line::Model, Option<journal_entry::Model>)> =
            journal_line::Entity::find()
                .inner_join(account::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(journal_entry::Column::Date.lte(to_date))
                .filter(account::Column::Number.gte(4000))
                .filter(account::Column::Number.lte(8999))
                .find_also_related(journal_entry::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        use std::collections::BTreeMap;
        let mut monthly_income: BTreeMap<u32, Decimal> = BTreeMap::new();
        let mut monthly_expenses: BTreeMap<u32, Decimal> = BTreeMap::new();

        // Pre-fill 12 months
        for m in 1..=12u32 {
            monthly_income.insert(m, Decimal::ZERO);
            monthly_expenses.insert(m, Decimal::ZERO);
        }

        for (line, entry_opt) in &income_lines {
            if let Some(entry) = entry_opt {
                let m = entry.date.month();
                *monthly_income.entry(m).or_insert(Decimal::ZERO) += line.credit_amount;
            }
        }

        for (line, entry_opt) in &expense_lines {
            if let Some(entry) = entry_opt {
                let m = entry.date.month();
                *monthly_expenses.entry(m).or_insert(Decimal::ZERO) += line.debit_amount;
            }
        }

        // Build cumulative
        let mut cum_income = Decimal::ZERO;
        let mut cum_expenses = Decimal::ZERO;
        let mut months = Vec::with_capacity(12);

        for m in 1..=12u32 {
            let inc = monthly_income.get(&m).copied().unwrap_or(Decimal::ZERO);
            let exp = monthly_expenses.get(&m).copied().unwrap_or(Decimal::ZERO);
            cum_income += inc;
            cum_expenses += exp;
            months.push(OverviewMonth {
                month: format!("{:04}-{:02}", year, m),
                income: inc,
                expenses: exp,
                cumulative_income: cum_income,
                cumulative_expenses: cum_expenses,
            });
        }

        // Available years: distinct years from journal entries
        let all_entries: Vec<journal_entry::Model> = journal_entry::Entity::find()
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut year_set: std::collections::BTreeSet<i32> = std::collections::BTreeSet::new();
        for entry in &all_entries {
            year_set.insert(entry.date.year());
        }
        // Always include current year
        year_set.insert(Utc::now().naive_utc().date().year());
        let available_years: Vec<i32> = year_set.into_iter().rev().collect();

        Ok(OverviewData {
            year,
            months,
            total_income: cum_income,
            total_expenses: cum_expenses,
            difference: cum_income - cum_expenses,
            available_years,
        })
    }

    pub async fn invoice_aging(
        db: &DatabaseConnection,
    ) -> Result<Vec<InvoiceAgingBucket>, AppError> {
        let invoices: Vec<invoice::Model> = invoice::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        use std::collections::HashMap;
        let mut buckets: HashMap<String, (i64, Decimal)> = HashMap::new();

        for inv in invoices {
            let entry = buckets.entry(inv.status.clone()).or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += inv.total;
        }

        let order = [InvoiceStatus::Draft.as_str(), InvoiceStatus::Sent.as_str(), InvoiceStatus::Overdue.as_str(), InvoiceStatus::Paid.as_str(), InvoiceStatus::Cancelled.as_str()];
        let mut result: Vec<InvoiceAgingBucket> = order
            .iter()
            .filter_map(|s| {
                buckets.get(*s).map(|(count, total)| InvoiceAgingBucket {
                    status: ToString::to_string(s),
                    count: *count,
                    total: *total,
                })
            })
            .collect();

        // Include any statuses not in the standard order
        for (status, (count, total)) in &buckets {
            if !order.contains(&status.as_str()) {
                result.push(InvoiceAgingBucket {
                    status: status.clone(),
                    count: *count,
                    total: *total,
                });
            }
        }

        Ok(result)
    }

    pub async fn top_outstanding_contacts(
        db: &DatabaseConnection,
        limit: u32,
    ) -> Result<Vec<OutstandingContact>, AppError> {
        let invoices: Vec<(invoice::Model, Option<contact::Model>)> =
            invoice::Entity::find()
                .filter(
                    Condition::any()
                        .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                        .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
                )
                .find_also_related(contact::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        use std::collections::HashMap;
        let mut map: HashMap<String, (String, Decimal, i64)> = HashMap::new();

        for (inv, contact_opt) in invoices {
            let name = contact_opt
                .map(|c| c.name1.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            let entry = map
                .entry(inv.contact_id.clone())
                .or_insert((name, Decimal::ZERO, 0));
            entry.1 += inv.total;
            entry.2 += 1;
        }

        let mut result: Vec<OutstandingContact> = map
            .into_iter()
            .map(|(contact_id, (name, amount, count))| OutstandingContact {
                contact_id,
                contact_name: name,
                outstanding_amount: amount,
                invoice_count: count,
            })
            .collect();

        result.sort_by(|a, b| b.outstanding_amount.cmp(&a.outstanding_amount));
        result.truncate(limit as usize);

        Ok(result)
    }
}
