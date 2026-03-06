use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_common::enums::JournalStatus;
use konto_db::entities::{account, company_setting, journal_entry, journal_line, vat_rate};
use rust_decimal::Decimal;
use sea_orm::*;
use std::collections::HashMap;

pub use super::report_types::*;
use super::ch_account_groups;

pub struct ReportService;

impl ReportService {
    pub async fn trial_balance(
        db: &DatabaseConnection,
        as_of: Option<NaiveDate>,
    ) -> Result<Vec<TrialBalanceRow>, AppError> {
        let mut query = journal_line::Entity::find()
            .inner_join(journal_entry::Entity)
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()));

        if let Some(date) = as_of {
            query = query.filter(journal_entry::Column::Date.lte(date));
        }

        let rows: Vec<(journal_line::Model, Option<account::Model>)> = query
            .find_also_related(account::Entity)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut map: HashMap<String, TrialBalanceRow> = HashMap::new();

        for (line, acct_opt) in rows {
            if let Some(acct) = acct_opt {
                let entry = map.entry(acct.id.clone()).or_insert(TrialBalanceRow {
                    account_id: acct.id.clone(),
                    account_number: acct.number,
                    account_name: acct.name.clone(),
                    account_type: acct.account_type.clone(),
                    total_debit: Decimal::ZERO,
                    total_credit: Decimal::ZERO,
                    balance: Decimal::ZERO,
                });
                entry.total_debit += line.debit_amount;
                entry.total_credit += line.credit_amount;
                entry.balance = entry.total_debit - entry.total_credit;
            }
        }

        let mut result: Vec<TrialBalanceRow> = map.into_values().collect();
        result.sort_by_key(|r| r.account_number);
        Ok(result)
    }

    pub async fn balance_sheet(
        db: &DatabaseConnection,
        as_of: NaiveDate,
    ) -> Result<BalanceSheet, AppError> {
        let tb = Self::trial_balance(db, Some(as_of)).await?;

        let assets: Vec<TrialBalanceRow> = tb
            .iter()
            .filter(|r| r.account_type == "asset")
            .cloned()
            .collect();
        let liabilities: Vec<TrialBalanceRow> = tb
            .iter()
            .filter(|r| r.account_type == "liability")
            .cloned()
            .collect();
        let equity: Vec<TrialBalanceRow> = tb
            .iter()
            .filter(|r| r.account_type == "equity")
            .cloned()
            .collect();

        let total_assets: Decimal = assets.iter().map(|r| r.balance).sum();
        let total_liab: Decimal = liabilities.iter().map(|r| r.balance).sum();
        let total_eq: Decimal = equity.iter().map(|r| r.balance).sum();

        Ok(BalanceSheet {
            as_of: as_of.to_string(),
            assets,
            liabilities,
            equity,
            total_assets,
            total_liabilities_equity: total_liab + total_eq,
        })
    }

    pub async fn profit_loss(
        db: &DatabaseConnection,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<ProfitLoss, AppError> {
        let rows: Vec<(journal_line::Model, Option<account::Model>)> =
            journal_line::Entity::find()
                .inner_join(journal_entry::Entity)
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(journal_entry::Column::Date.lte(to_date))
                .find_also_related(account::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        let mut map: HashMap<String, TrialBalanceRow> = HashMap::new();

        for (line, acct_opt) in rows {
            if let Some(acct) = acct_opt {
                let entry = map.entry(acct.id.clone()).or_insert(TrialBalanceRow {
                    account_id: acct.id.clone(),
                    account_number: acct.number,
                    account_name: acct.name.clone(),
                    account_type: acct.account_type.clone(),
                    total_debit: Decimal::ZERO,
                    total_credit: Decimal::ZERO,
                    balance: Decimal::ZERO,
                });
                entry.total_debit += line.debit_amount;
                entry.total_credit += line.credit_amount;
                entry.balance = entry.total_debit - entry.total_credit;
            }
        }

        let mut all: Vec<TrialBalanceRow> = map.into_values().collect();
        all.sort_by_key(|r| r.account_number);

        let revenue: Vec<TrialBalanceRow> = all
            .iter()
            .filter(|r| r.account_type == "revenue")
            .cloned()
            .collect();
        let expenses: Vec<TrialBalanceRow> = all
            .iter()
            .filter(|r| r.account_type == "expense")
            .cloned()
            .collect();

        let total_revenue: Decimal = revenue.iter().map(|r| r.total_credit - r.total_debit).sum();
        let total_expenses: Decimal = expenses.iter().map(|r| r.total_debit - r.total_credit).sum();

        Ok(ProfitLoss {
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
            revenue,
            expenses,
            total_revenue,
            total_expenses,
            net_income: total_revenue - total_expenses,
        })
    }

    pub async fn account_ledger(
        db: &DatabaseConnection,
        account_id: &str,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<Vec<LedgerLine>, AppError> {
        let mut query = journal_line::Entity::find()
            .find_also_related(journal_entry::Entity)
            .filter(journal_line::Column::AccountId.eq(account_id))
            .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()));

        if let Some(from) = from_date {
            query = query.filter(journal_entry::Column::Date.gte(from));
        }
        if let Some(to) = to_date {
            query = query.filter(journal_entry::Column::Date.lte(to));
        }

        let rows: Vec<(journal_line::Model, Option<journal_entry::Model>)> = query
            .order_by_asc(journal_entry::Column::Date)
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut result = Vec::new();
        let mut running = Decimal::ZERO;

        for (line, entry_opt) in rows {
            running += line.debit_amount - line.credit_amount;

            let (date_str, desc) = match entry_opt {
                Some(e) => (e.date.to_string(), e.description),
                None => (String::new(), String::new()),
            };

            result.push(LedgerLine {
                date: date_str,
                entry_id: line.journal_entry_id,
                description: desc,
                debit: line.debit_amount,
                credit: line.credit_amount,
                running_balance: running,
            });
        }

        Ok(result)
    }

    pub async fn vat_report(
        db: &DatabaseConnection,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<VatReport, AppError> {
        use super::report_types::VatReportEntry;

        // Load company settings to detect SSS mode
        let settings = company_setting::Entity::find()
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let vat_method = settings
            .as_ref()
            .map(|s| s.vat_method.as_str())
            .unwrap_or("effective");
        let flat_pct = settings.as_ref().and_then(|s| s.flat_rate_percentage);

        // Load all VAT rates
        let vat_rates: Vec<vat_rate::Model> = vat_rate::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let vat_map: HashMap<String, vat_rate::Model> =
            vat_rates.into_iter().map(|v| (v.id.clone(), v)).collect();

        // Get all lines with VAT in date range
        let rows: Vec<(journal_line::Model, Option<journal_entry::Model>)> =
            journal_line::Entity::find()
                .filter(journal_entry::Column::Status.eq(JournalStatus::Posted.as_str()))
                .filter(journal_entry::Column::Date.gte(from_date))
                .filter(journal_entry::Column::Date.lte(to_date))
                .filter(journal_line::Column::VatRateId.is_not_null())
                .find_also_related(journal_entry::Entity)
                .all(db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        let mut sums: HashMap<String, Decimal> = HashMap::new();

        for (line, _) in &rows {
            if let Some(vat_id) = &line.vat_rate_id {
                let taxable = line.debit_amount + line.credit_amount;
                *sums.entry(vat_id.clone()).or_insert(Decimal::ZERO) += taxable;
            }
        }

        let hundred = Decimal::from(100);
        let mut output_entries: Vec<VatReportEntry> = Vec::new();
        let mut input_entries: Vec<VatReportEntry> = Vec::new();

        for (vat_id, taxable) in sums {
            if let Some(vr) = vat_map.get(&vat_id) {
                let entry = VatReportEntry {
                    vat_code: vr.code.clone(),
                    vat_name: vr.name.clone(),
                    rate: vr.rate,
                    vat_type: vr.vat_type.clone(),
                    taxable_amount: taxable,
                    vat_amount: taxable * vr.rate / hundred,
                };
                if vr.vat_type == "input" {
                    input_entries.push(entry);
                } else {
                    output_entries.push(entry);
                }
            }
        }

        output_entries.sort_by(|a, b| a.vat_code.cmp(&b.vat_code));
        input_entries.sort_by(|a, b| a.vat_code.cmp(&b.vat_code));

        let total_output_taxable: Decimal = output_entries.iter().map(|e| e.taxable_amount).sum();
        let total_output_vat: Decimal = output_entries.iter().map(|e| e.vat_amount).sum();
        let total_input_taxable: Decimal = input_entries.iter().map(|e| e.taxable_amount).sum();
        let total_input_vat: Decimal = input_entries.iter().map(|e| e.vat_amount).sum();

        // SSS (flat-rate) mode: net owed = gross_revenue * flat_rate / 100
        let (net_vat_owed, gross_revenue, flat_rate_vat_owed, collected_vat, saldo_ertrag) =
            if vat_method == "flat_rate" {
                if let Some(pct) = flat_pct {
                    // Gross revenue = taxable + collected VAT
                    let gross = total_output_taxable + total_output_vat;
                    let owed = gross * pct / hundred;
                    let collected = total_output_vat;
                    let ertrag = collected - owed;
                    (owed, Some(gross), Some(owed), Some(collected), Some(ertrag))
                } else {
                    // No flat rate configured, fall back to effective
                    let net = total_output_vat - total_input_vat;
                    (net, None, None, None, None)
                }
            } else {
                let net = total_output_vat - total_input_vat;
                (net, None, None, None, None)
            };

        Ok(VatReport {
            vat_method: vat_method.to_string(),
            output_entries,
            input_entries,
            total_output_taxable,
            total_output_vat,
            total_input_taxable,
            total_input_vat,
            net_vat_owed,
            flat_rate_percentage: flat_pct,
            gross_revenue,
            flat_rate_vat_owed,
            collected_vat,
            saldo_ertrag,
        })
    }

    /// Swiss grouped balance sheet per KMU Kontenrahmen.
    pub async fn swiss_balance_sheet(
        db: &DatabaseConnection,
        as_of: NaiveDate,
    ) -> Result<SwissBalanceSheet, AppError> {
        let tb = Self::trial_balance(db, Some(as_of)).await?;

        let asset_defs = ch_account_groups::ch_balance_sheet_assets();
        let liab_defs = ch_account_groups::ch_balance_sheet_liabilities();

        let assets = ch_account_groups::build_grouped_sections(&asset_defs, &tb, false);
        let liabilities = ch_account_groups::build_grouped_sections(&liab_defs, &tb, true);

        let total_assets: Decimal = assets.iter().map(|s| s.total).sum();
        let total_liabilities: Decimal = liabilities.iter().map(|s| s.total).sum();

        Ok(SwissBalanceSheet {
            as_of: as_of.to_string(),
            assets,
            liabilities,
            total_assets,
            total_liabilities,
        })
    }

    /// Swiss grouped income statement per KMU Kontenrahmen with subtotals.
    pub async fn swiss_income_statement(
        db: &DatabaseConnection,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<SwissIncomeStatement, AppError> {
        let pl = Self::profit_loss(db, from_date, to_date).await?;
        let all_rows: Vec<TrialBalanceRow> = [pl.revenue, pl.expenses].concat();

        let section_defs = ch_account_groups::ch_income_statement_sections();
        let sections = ch_account_groups::build_grouped_sections(&section_defs, &all_rows, false);

        let section_total = |key: &str| -> Decimal {
            sections.iter().find(|s| s.key == key).map(|s| s.total).unwrap_or(Decimal::ZERO)
        };

        // Revenue accounts have credit balances (negative in debit-centric view)
        let operating_revenue = -section_total("operating_revenue");
        let material_expense = section_total("material_expense");
        let personnel_expense = section_total("personnel_expense");
        let other_opex = section_total("other_opex");
        let depreciation = section_total("depreciation");
        let financial_result = section_total("financial_result");
        let extraordinary = section_total("extraordinary");
        let taxes = section_total("taxes");

        let gross_profit_material = operating_revenue - material_expense;
        let gross_profit_personnel = gross_profit_material - personnel_expense;
        let ebitda = gross_profit_personnel - other_opex;
        let ebit = ebitda - depreciation;
        let ebt = ebit - financial_result;
        let net_result = ebt - extraordinary - taxes;

        Ok(SwissIncomeStatement {
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
            sections,
            subtotals: SwissIncomeSubtotals {
                operating_revenue,
                gross_profit_material,
                gross_profit_personnel,
                ebitda,
                ebit,
                ebt,
                net_result,
            },
        })
    }
}
