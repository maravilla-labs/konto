use chrono::{Datelike, NaiveDate};
use konto_common::error::AppError;
use konto_common::enums::{InvoiceStatus, JournalStatus};
use konto_db::entities::{account, invoice, journal_entry, journal_line};
use rust_decimal::Decimal;
use sea_orm::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
pub struct CashFlowItem {
    pub description: String,
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct CashFlowSection {
    pub label: String,
    pub inflows: Decimal,
    pub outflows: Decimal,
    pub net: Decimal,
    pub items: Vec<CashFlowItem>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CashFlowReport {
    pub sections: Vec<CashFlowSection>,
    pub net_change: Decimal,
    pub opening_balance: Decimal,
    pub closing_balance: Decimal,
    pub reconciliation_difference: Decimal,
    pub from_date: String,
    pub to_date: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct MonthlyCashFlow {
    pub month: String,
    pub inflows: Decimal,
    pub outflows: Decimal,
    pub net: Decimal,
    pub cumulative_balance: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct CashFlowMonthlyReport {
    pub months: Vec<MonthlyCashFlow>,
    pub initial_balance: Decimal,
    pub ending_balance: Decimal,
    pub total_inflows: Decimal,
    pub total_outflows: Decimal,
    pub net_variation: Decimal,
}

#[derive(Debug, Serialize, Clone)]
pub struct AgingBucket {
    pub bucket: String,
    pub count: i64,
    pub total: Decimal,
}

pub struct CashFlowService;

impl CashFlowService {
    pub async fn cash_flow_report(
        db: &DatabaseConnection,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<CashFlowReport, AppError> {
        let accounts: Vec<account::Model> = account::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let acct_map: HashMap<String, account::Model> =
            accounts.into_iter().map(|a| (a.id.clone(), a)).collect();

        let cash_account_ids: Vec<String> = acct_map
            .iter()
            .filter(|(_, a)| a.number >= 1000 && a.number <= 1099)
            .map(|(id, _)| id.clone())
            .collect();

        // Closing balance: all cash account movements through to_date (unfiltered = real bank balance)
        let closing_balance: Decimal = if cash_account_ids.is_empty() {
            Decimal::ZERO
        } else {
            let all: Vec<journal_line::Model> = journal_line::Entity::find()
                .inner_join(journal_entry::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.lte(to_date))
                .filter(journal_line::Column::AccountId.is_in(&cash_account_ids))
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            all.iter().map(|l| l.debit_amount - l.credit_amount).sum()
        };

        // Base opening: cash movements strictly before from_date
        let mut opening_balance: Decimal = if cash_account_ids.is_empty() {
            Decimal::ZERO
        } else {
            let pre: Vec<journal_line::Model> = journal_line::Entity::find()
                .inner_join(journal_entry::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.lt(from_date))
                .filter(journal_line::Column::AccountId.is_in(&cash_account_ids))
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            pre.iter().map(|l| l.debit_amount - l.credit_amount).sum()
        };

        let lines: Vec<journal_line::Model> = journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .filter(journal_entry::Column::Date.gte(from_date))
            .filter(journal_entry::Column::Date.lte(to_date))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut entry_lines: HashMap<String, Vec<&journal_line::Model>> = HashMap::new();
        for line in &lines {
            entry_lines
                .entry(line.journal_entry_id.clone())
                .or_default()
                .push(line);
        }

        let mut operating_items: HashMap<String, Decimal> = HashMap::new();
        let mut investing_items: HashMap<String, Decimal> = HashMap::new();
        let mut financing_items: HashMap<String, Decimal> = HashMap::new();

        for lines in entry_lines.values() {
            let cash_lines: Vec<&&journal_line::Model> = lines
                .iter()
                .filter(|l| is_cash_account(&acct_map, &l.account_id))
                .collect();

            if cash_lines.is_empty() {
                continue;
            }

            let counterpart_lines: Vec<&&journal_line::Model> = lines
                .iter()
                .filter(|l| {
                    !is_cash_account(&acct_map, &l.account_id)
                        && classify_account_number(&acct_map, &l.account_id)
                            != FlowCategory::Exclude
                })
                .collect();

            let cash_net: Decimal = cash_lines
                .iter()
                .map(|l| l.debit_amount - l.credit_amount)
                .sum();

            if cash_net == Decimal::ZERO {
                continue;
            }

            // Proportional distribution across counterpart lines
            let total_counterpart_abs: Decimal = counterpart_lines
                .iter()
                .map(|l| (l.debit_amount + l.credit_amount).abs())
                .sum();

            if total_counterpart_abs == Decimal::ZERO || counterpart_lines.is_empty() {
                // System/opening balance entry (e.g. Eröffnungsbilanz 9xxx → 1000)
                // Not a cash flow — roll into opening balance so the chain stays consistent
                opening_balance += cash_net;
                continue;
            }

            for cp in &counterpart_lines {
                let cp_abs = (cp.debit_amount + cp.credit_amount).abs();
                let proportion = cp_abs / total_counterpart_abs;
                let allocated = cash_net * proportion;

                let category = classify_account_number(&acct_map, &cp.account_id);
                let acct_label = acct_map
                    .get(&cp.account_id)
                    .map(|a| format!("{} {}", a.number, a.name))
                    .unwrap_or_else(|| "Other".to_string());

                let items = match category {
                    FlowCategory::Operating => &mut operating_items,
                    FlowCategory::Investing => &mut investing_items,
                    FlowCategory::Financing => &mut financing_items,
                    FlowCategory::Exclude => continue,
                };
                *items.entry(acct_label).or_insert(Decimal::ZERO) += allocated;
            }
        }

        let sections = vec![
            build_section("Operating Activities", operating_items),
            build_section("Investing Activities", investing_items),
            build_section("Financing Activities", financing_items),
        ];

        let net_change: Decimal = sections.iter().map(|s| s.net).sum();
        let expected_closing = opening_balance + net_change;
        let reconciliation_difference = closing_balance - expected_closing;

        Ok(CashFlowReport {
            sections,
            net_change,
            opening_balance,
            closing_balance,
            reconciliation_difference,
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
        })
    }

    pub async fn monthly_cash_flow(
        db: &DatabaseConnection,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<CashFlowMonthlyReport, AppError> {
        let accounts: Vec<account::Model> = account::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let acct_map: HashMap<String, account::Model> =
            accounts.into_iter().map(|a| (a.id.clone(), a)).collect();

        let has_cash_accounts = acct_map.values().any(|a| a.number >= 1000 && a.number <= 1099);
        if !has_cash_accounts {
            return Ok(CashFlowMonthlyReport {
                months: vec![],
                initial_balance: Decimal::ZERO,
                ending_balance: Decimal::ZERO,
                total_inflows: Decimal::ZERO,
                total_outflows: Decimal::ZERO,
                net_variation: Decimal::ZERO,
            });
        }

        // Base initial balance: cash movements strictly before from_date
        let cash_account_ids: Vec<String> = acct_map
            .iter()
            .filter(|(_, a)| a.number >= 1000 && a.number <= 1099)
            .map(|(id, _)| id.clone())
            .collect();

        let pre_lines: Vec<journal_line::Model> = journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
            .filter(journal_entry::Column::Date.lt(from_date))
            .filter(journal_line::Column::AccountId.is_in(&cash_account_ids))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut initial_balance: Decimal = pre_lines
            .iter()
            .map(|l| l.debit_amount - l.credit_amount)
            .sum();

        // --- Range entries ---
        // Load ALL lines for posted entries in the date range
        #[derive(Debug, Clone)]
        struct LineWithDate {
            account_id: String,
            debit_amount: Decimal,
            credit_amount: Decimal,
            journal_entry_id: String,
            entry_date: NaiveDate,
        }

        let range_all: Vec<(journal_line::Model, Option<journal_entry::Model>)> =
            journal_line::Entity::find()
                .find_also_related(journal_entry::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(journal_entry::Column::Date.lte(to_date))
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        let range_lines: Vec<LineWithDate> = range_all
            .into_iter()
            .filter_map(|(line, entry)| {
                entry.map(|e| LineWithDate {
                    account_id: line.account_id,
                    debit_amount: line.debit_amount,
                    credit_amount: line.credit_amount,
                    journal_entry_id: line.journal_entry_id,
                    entry_date: e.date,
                })
            })
            .collect();

        // Group by journal entry
        let mut entry_groups: HashMap<String, Vec<&LineWithDate>> = HashMap::new();
        for line in &range_lines {
            entry_groups
                .entry(line.journal_entry_id.clone())
                .or_default()
                .push(line);
        }

        // For each entry, apply same filter: skip if all counterparts are system/excluded
        let mut monthly_map: HashMap<String, (Decimal, Decimal)> = HashMap::new();

        for lines in entry_groups.values() {
            let cash_lines: Vec<&&LineWithDate> = lines
                .iter()
                .filter(|l| is_cash_account(&acct_map, &l.account_id))
                .collect();

            if cash_lines.is_empty() {
                continue;
            }

            let has_real_counterpart = lines.iter().any(|l| {
                !is_cash_account(&acct_map, &l.account_id)
                    && classify_account_number(&acct_map, &l.account_id)
                        != FlowCategory::Exclude
            });

            let cash_net: Decimal = cash_lines
                .iter()
                .map(|l| l.debit_amount - l.credit_amount)
                .sum();

            if cash_net == Decimal::ZERO {
                continue;
            }

            if !has_real_counterpart {
                // System/opening balance entry — not a flow, roll into initial balance
                initial_balance += cash_net;
                continue;
            }

            let entry_date = cash_lines[0].entry_date;
            let month_key = entry_date.format("%Y-%m").to_string();
            let entry = monthly_map
                .entry(month_key)
                .or_insert((Decimal::ZERO, Decimal::ZERO));
            if cash_net > Decimal::ZERO {
                entry.0 += cash_net;
            } else {
                entry.1 += cash_net;
            }
        }

        // Build ordered month list
        let mut current = from_date;
        let mut months = Vec::new();
        let mut cumulative = initial_balance;
        let mut total_inflows = Decimal::ZERO;
        let mut total_outflows = Decimal::ZERO;

        while current <= to_date {
            let month_key = current.format("%Y-%m").to_string();
            let (inflows, outflows) = monthly_map
                .remove(&month_key)
                .unwrap_or((Decimal::ZERO, Decimal::ZERO));
            let net = inflows + outflows;
            cumulative += net;
            total_inflows += inflows;
            total_outflows += outflows;

            months.push(MonthlyCashFlow {
                month: month_key,
                inflows,
                outflows,
                net,
                cumulative_balance: cumulative,
            });

            let (y, m) = (current.year(), current.month());
            current = if m == 12 {
                NaiveDate::from_ymd_opt(y + 1, 1, 1)
            } else {
                NaiveDate::from_ymd_opt(y, m + 1, 1)
            }
            .unwrap_or(to_date);
            if current > to_date && current != to_date {
                break;
            }
        }

        // Ending balance = real cash on hand (unfiltered, including system entries)
        let total_range_cash: Decimal = range_lines
            .iter()
            .filter(|l| is_cash_account(&acct_map, &l.account_id))
            .map(|l| l.debit_amount - l.credit_amount)
            .sum();
        let ending_balance = initial_balance + total_range_cash;
        let net_variation = total_inflows + total_outflows;

        Ok(CashFlowMonthlyReport {
            months,
            initial_balance,
            ending_balance,
            total_inflows,
            total_outflows,
            net_variation,
        })
    }

    pub async fn ar_aging_report(
        db: &DatabaseConnection,
    ) -> Result<Vec<AgingBucket>, AppError> {
        let today = chrono::Utc::now().naive_utc().date();

        let invoices: Vec<invoice::Model> = invoice::Entity::find()
            .filter(
                Condition::any()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
            )
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut buckets: [AgingBucket; 5] = [
            AgingBucket { bucket: "Current".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "1-30 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "31-60 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "61-90 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "90+ days".to_string(), count: 0, total: Decimal::ZERO },
        ];

        for inv in invoices {
            let days_overdue = (today - inv.due_date).num_days();
            let idx = if days_overdue <= 0 {
                0
            } else if days_overdue <= 30 {
                1
            } else if days_overdue <= 60 {
                2
            } else if days_overdue <= 90 {
                3
            } else {
                4
            };
            buckets[idx].count += 1;
            buckets[idx].total += inv.total;
        }

        Ok(buckets.to_vec())
    }

    pub async fn ap_aging_report(
        db: &DatabaseConnection,
    ) -> Result<Vec<AgingBucket>, AppError> {
        let _ = db;
        Ok(vec![
            AgingBucket { bucket: "Current".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "1-30 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "31-60 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "61-90 days".to_string(), count: 0, total: Decimal::ZERO },
            AgingBucket { bucket: "90+ days".to_string(), count: 0, total: Decimal::ZERO },
        ])
    }
}

fn is_cash_account(acct_map: &HashMap<String, account::Model>, id: &str) -> bool {
    acct_map
        .get(id)
        .map(|a| a.number >= 1000 && a.number <= 1099)
        .unwrap_or(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlowCategory {
    Operating,
    Investing,
    Financing,
    Exclude,
}

fn classify_account_number(
    acct_map: &HashMap<String, account::Model>,
    account_id: &str,
) -> FlowCategory {
    let n = match acct_map.get(account_id) {
        Some(a) => a.number,
        None => return FlowCategory::Operating,
    };

    match n {
        1000..=1099 => FlowCategory::Exclude,  // Cash itself
        1100..=1399 => FlowCategory::Operating, // Working capital (receivables, inventory, accruals)
        1400..=1799 => FlowCategory::Investing, // Fixed assets (financial, mobile, immobile, intangible)
        2000..=2099 => FlowCategory::Operating, // Trade payables
        2100..=2199 => FlowCategory::Financing, // Short-term financial liabilities (IAS 7.17)
        2200..=2399 => FlowCategory::Operating, // Other current liabilities, accruals
        2400..=2799 => FlowCategory::Financing, // Long-term liabilities + provisions
        2800..=2999 => FlowCategory::Financing, // Equity
        3000..=3999 => FlowCategory::Operating, // Revenue
        4000..=8999 => FlowCategory::Operating, // Expenses
        9000..=9999 => FlowCategory::Exclude,   // System / opening balance
        _ => FlowCategory::Operating,
    }
}

fn build_section(label: &str, items: HashMap<String, Decimal>) -> CashFlowSection {
    let mut item_list: Vec<CashFlowItem> = items
        .into_iter()
        .map(|(description, amount)| CashFlowItem { description, amount })
        .collect();
    item_list.sort_by(|a, b| b.amount.abs().cmp(&a.amount.abs()));

    let inflows: Decimal = item_list
        .iter()
        .filter(|i| i.amount > Decimal::ZERO)
        .map(|i| i.amount)
        .sum();
    let outflows: Decimal = item_list
        .iter()
        .filter(|i| i.amount < Decimal::ZERO)
        .map(|i| i.amount)
        .sum();

    CashFlowSection {
        label: label.to_string(),
        inflows,
        outflows,
        net: inflows + outflows,
        items: item_list,
    }
}
