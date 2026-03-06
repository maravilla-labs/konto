use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::{ImportStatus, TimeEntryStatus};
use konto_db::entities::import_job;
use konto_db::repository::import_repo::ImportRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::accounts_csv::parse_accounts_csv;
use super::contacts_csv::parse_contacts_csv;
use super::contacts_import::execute_contacts_import;
use super::journal_xlsx::parse_journal_xlsx;
use super::projects_xlsx::parse_projects_xlsx;
use super::time_entries_csv::parse_time_entries_csv;

pub struct ImportService;

impl ImportService {
    pub async fn upload(
        db: &DatabaseConnection,
        import_type: &str,
        file_name: &str,
        file_data: Vec<u8>,
        created_by: Option<String>,
    ) -> Result<import_job::Model, AppError> {
        let now = Utc::now().naive_utc();

        let model = import_job::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            import_type: Set(import_type.to_string()),
            file_name: Set(file_name.to_string()),
            file_data: Set(file_data),
            status: Set("uploaded".to_string()),
            total_rows: Set(None),
            imported_rows: Set(None),
            error_rows: Set(None),
            preview_data: Set(None),
            error_log: Set(None),
            created_by: Set(created_by),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ImportRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn preview(
        db: &DatabaseConnection,
        import_id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let job = ImportRepo::find_by_id(db, import_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Import job not found".to_string()))?;

        let preview = match job.import_type.as_str() {
            "accounts" => {
                let rows = parse_accounts_csv(&job.file_data)?;
                let total = rows.len();
                let preview: Vec<_> = rows.into_iter().take(20).collect();
                serde_json::json!({ "total": total, "preview": preview })
            }
            "contacts" => {
                let rows = parse_contacts_csv(&job.file_data)?;
                let total = rows.len();
                let preview: Vec<_> = rows.into_iter().take(20).collect();
                serde_json::json!({ "total": total, "preview": preview })
            }
            "time_entries" => {
                let rows = parse_time_entries_csv(&job.file_data)?;
                let total = rows.len();
                let preview: Vec<_> = rows.into_iter().take(20).collect();
                serde_json::json!({ "total": total, "preview": preview })
            }
            "projects" => {
                let rows = parse_projects_xlsx(&job.file_data)?;
                let total = rows.len();
                let preview: Vec<_> = rows.into_iter().take(20).collect();
                serde_json::json!({ "total": total, "preview": preview })
            }
            "journal" => {
                let rows = parse_journal_xlsx(&job.file_data)?;
                let total = rows.len();
                let preview: Vec<_> = rows.into_iter().take(20).collect();
                serde_json::json!({ "total": total, "preview": preview })
            }
            _ => return Err(AppError::BadRequest("Unknown import type".to_string())),
        };

        // Update job with preview
        let mut active: import_job::ActiveModel = job.into();
        active.status = Set("previewed".to_string());
        active.preview_data = Set(Some(preview.to_string()));
        active.total_rows = Set(preview.get("total").and_then(|v| v.as_i64()).map(|v| v as i32));
        active.updated_at = Set(Utc::now().naive_utc());
        ImportRepo::update(db, active)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(preview)
    }

    pub async fn execute(
        db: &DatabaseConnection,
        import_id: &str,
    ) -> Result<import_job::Model, AppError> {
        let job = ImportRepo::find_by_id(db, import_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Import job not found".to_string()))?;

        if job.status != ImportStatus::Previewed.as_str() && job.status != ImportStatus::Uploaded.as_str() {
            return Err(AppError::BadRequest("Import already executed or failed".to_string()));
        }

        let (imported, errors, error_log) = match job.import_type.as_str() {
            "accounts" => execute_accounts_import(db, &job.file_data).await?,
            "contacts" => execute_contacts_import(db, &job.file_data).await?,
            "time_entries" => {
                let (i, e) = execute_time_entries_import(db, &job.file_data).await?;
                (i, e, Vec::new())
            }
            "projects" => {
                let (i, e) = execute_projects_import(db, &job.file_data).await?;
                (i, e, Vec::new())
            }
            "journal" => {
                execute_journal_import(db, &job.file_data).await?
            }
            _ => return Err(AppError::BadRequest("Unknown import type".to_string())),
        };

        let error_log_str = if error_log.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&error_log).unwrap_or_default())
        };

        let mut active: import_job::ActiveModel = job.into();
        active.status = Set("executed".to_string());
        active.imported_rows = Set(Some(imported));
        active.error_rows = Set(Some(errors));
        active.error_log = Set(error_log_str);
        active.updated_at = Set(Utc::now().naive_utc());

        ImportRepo::update(db, active)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }
}

async fn execute_accounts_import(
    db: &DatabaseConnection,
    data: &[u8],
) -> Result<(i32, i32, Vec<String>), AppError> {
    use konto_db::entities::account;
    use konto_db::repository::account_repo::AccountRepo;
    use std::collections::HashMap;

    let rows = parse_accounts_csv(data)?;
    let mut imported = 0i32;
    let mut errors = 0i32;
    let mut error_log = Vec::new();

    // Sort by number to ensure parents are created before children
    let mut sorted = rows.clone();
    sorted.sort_by_key(|r| r.number);

    // Track created accounts: number → id
    let mut number_to_id: HashMap<i32, String> = HashMap::new();

    // Load existing accounts into map
    let existing = AccountRepo::find_all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    for acct in &existing {
        number_to_id.insert(acct.number, acct.id.clone());
    }

    for row in &sorted {
        // Skip if account already exists
        if number_to_id.contains_key(&row.number) {
            continue;
        }

        // Auto-assign parent_id based on Swiss KMU hierarchy
        let parent_id = find_parent_id(row.number, &number_to_id);

        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4().to_string();

        let model = account::ActiveModel {
            id: sea_orm::Set(id.clone()),
            number: sea_orm::Set(row.number),
            name: sea_orm::Set(row.name.clone()),
            account_type: sea_orm::Set(row.account_type.clone()),
            description: sea_orm::Set(row.description.clone()),
            parent_id: sea_orm::Set(parent_id),
            currency_id: sea_orm::Set(None),
            is_active: sea_orm::Set(true),
            created_at: sea_orm::Set(now),
            updated_at: sea_orm::Set(now),
        };

        match AccountRepo::create(db, model).await {
            Ok(_) => {
                number_to_id.insert(row.number, id);
                imported += 1;
            }
            Err(e) => {
                error_log.push(format!("Account {}: {}", row.number, e));
                errors += 1;
            }
        }
    }

    Ok((imported, errors, error_log))
}

/// Find parent account ID using Swiss KMU hierarchy convention.
/// 4-digit (1020) → look for group 102, 10, 1 (in that order).
/// 3-digit (102) → look for group 10, 1.
/// 2-digit (10) → look for group 1.
fn find_parent_id(number: i32, map: &std::collections::HashMap<i32, String>) -> Option<String> {
    let mut candidate = number / 10;
    while candidate > 0 {
        if let Some(id) = map.get(&candidate) {
            return Some(id.clone());
        }
        candidate /= 10;
    }
    None
}

async fn execute_time_entries_import(db: &DatabaseConnection, data: &[u8]) -> Result<(i32, i32), AppError> {
    use konto_db::entities::time_entry;
    use konto_db::repository::project_repo::TimeEntryRepo;

    let rows = parse_time_entries_csv(data)?;
    let mut imported = 0i32;
    let mut errors = 0i32;

    for row in rows {
        let date = chrono::NaiveDate::parse_from_str(&row.date, "%Y-%m-%d");
        let date = match date {
            Ok(d) => d,
            Err(_) => { errors += 1; continue; }
        };

        let now = Utc::now().naive_utc();
        let model = time_entry::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            project_id: Set(None),
            contact_id: Set(None),
            user_id: Set(None),
            activity_type_id: Set(None),
            date: Set(date),
            estimated_minutes: Set(row.estimated_minutes),
            actual_minutes: Set(row.actual_minutes),
            flat_amount: Set(None),
            description: Set(row.description.clone()),
            travel_minutes: Set(row.travel_minutes),
            travel_flat_rate: Set(None),
            travel_distance: Set(None),
            quantity: Set(None),
            status: Set({
                let raw = row.status.as_deref().unwrap_or("pending").to_lowercase();
                match raw.as_str() {
                    "erledigt" | "done" => TimeEntryStatus::Done.as_str(),
                    "fakturiert" | "invoiced" => TimeEntryStatus::Invoiced.as_str(),
                    "in arbeit" | "in_progress" => TimeEntryStatus::InProgress.as_str(),
                    "offen" | "pending" | "active" => TimeEntryStatus::Pending.as_str(),
                    _ => TimeEntryStatus::Pending.as_str(),
                }.to_string()
            }),
            billed: Set({
                let raw = row.status.as_deref().unwrap_or("").to_lowercase();
                raw == "fakturiert" || raw == "invoiced"
            }),
            billable: Set(true),
            start_time: Set(None),
            end_time: Set(None),
            bexio_id: Set(None),
            task_id: Set(None),
            timesheet_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        match TimeEntryRepo::create(db, model).await {
            Ok(_) => imported += 1,
            Err(_) => errors += 1,
        }
    }

    Ok((imported, errors))
}

async fn execute_projects_import(db: &DatabaseConnection, data: &[u8]) -> Result<(i32, i32), AppError> {
    let rows = parse_projects_xlsx(data)?;
    let mut imported = 0i32;
    let mut errors = 0i32;

    for row in rows {
        let result = crate::services::project_service::ProjectService::create(
            db,
            &row.name,
            row.number.clone(),
            None,
            None,
            None,
            None,
            row.description.clone(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None, // invoicing_method
            None, // currency
            None, // rounding_method
            None, // rounding_factor_minutes
            None, // flat_rate_total
            None, // owner_id
        ).await;

        match result {
            Ok(_) => imported += 1,
            Err(_) => errors += 1,
        }
    }

    Ok((imported, errors))
}

/// Extract the VAT code from MWST strings like "UO81 (8.10%)" → "UO81"
fn extract_vat_code(raw: &str) -> &str {
    raw.split_whitespace().next().unwrap_or(raw).trim()
}

/// Returns true if the account is an income/expense account (Swiss KMU 3xxx-8xxx).
/// Only these accounts should carry vat_rate_id — they represent the taxable transaction.
/// Balance sheet accounts (1xxx assets, 2xxx liabilities) must NOT get vat_rate_id
/// because they'd cause double-counting in the VAT report.
fn is_taxable_account(account_number: i32) -> bool {
    (3000..=8999).contains(&account_number)
}

async fn execute_journal_import(db: &DatabaseConnection, data: &[u8]) -> Result<(i32, i32, Vec<String>), AppError> {
    use crate::services::journal_service::{JournalLineInput, JournalService};
    use konto_db::entities::journal_line;
    use konto_db::repository::account_repo::AccountRepo;
    use konto_db::repository::journal_repo::JournalRepo;
    use konto_db::repository::vat_rate_repo::VatRateRepo;
    use rust_decimal::Decimal;
    use std::collections::HashMap;
    use std::str::FromStr;

    let rows = parse_journal_xlsx(data)?;
    let mut imported = 0i32;
    let mut errors = 0i32;
    let mut error_log: Vec<String> = Vec::new();

    // Build VAT code → id lookup map
    let all_vat_rates = VatRateRepo::find_all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    let vat_code_map: HashMap<String, String> = all_vat_rates
        .into_iter()
        .map(|v| (v.code.clone(), v.id.clone()))
        .collect();

    // Build account number → id lookup (for matching lines on existing entries)
    let all_accounts = AccountRepo::find_all(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    let account_num_to_id: HashMap<i32, String> = all_accounts
        .iter()
        .map(|a| (a.number, a.id.clone()))
        .collect();
    let account_id_to_num: HashMap<String, i32> = all_accounts
        .iter()
        .map(|a| (a.id.clone(), a.number))
        .collect();

    for row in rows {
        let date = chrono::NaiveDate::parse_from_str(&row.date, "%Y-%m-%d")
            .or_else(|_| chrono::NaiveDate::parse_from_str(&row.date, "%d.%m.%Y"));

        let date = match date {
            Ok(d) => d,
            Err(_) => { errors += 1; error_log.push(format!("Row '{}': invalid date", row.date)); continue; }
        };

        let amount = match row.amount.as_deref().and_then(|s| Decimal::from_str(s).ok()) {
            Some(a) => a,
            None => { errors += 1; error_log.push(format!("Row date={}: invalid amount", row.date)); continue; }
        };

        // Resolve VAT rate from MWST column
        let vat_rate_id = row.vat_code.as_deref().and_then(|raw| {
            let code = extract_vat_code(raw);
            vat_code_map.get(code).cloned()
        });

        // --- Duplicate detection by reference ---
        if let Some(ref_str) = row.reference.as_ref().filter(|r| !r.is_empty()) {
                let existing = JournalRepo::find_entries_by_reference(db, ref_str)
                    .await
                    .unwrap_or_default();

                if !existing.is_empty() {
                    // Entry exists — patch VAT on its lines:
                    // 1. Set vat_rate_id on taxable (3xxx-8xxx) lines that are missing it
                    // 2. Remove vat_rate_id from non-taxable lines (fix prior bad imports)
                    for entry in &existing {
                        let lines = JournalRepo::find_lines_by_entry(db, &entry.id)
                            .await
                            .unwrap_or_default();

                        for line in lines {
                            let acct_num = account_id_to_num.get(&line.account_id).copied().unwrap_or(0);

                            if is_taxable_account(acct_num) {
                                // Taxable line — set vat_rate_id if missing and we have one
                                if line.vat_rate_id.is_none() && vat_rate_id.is_some() {
                                    let mut active: journal_line::ActiveModel = line.into();
                                    active.vat_rate_id = Set(vat_rate_id.clone());
                                    let _ = JournalRepo::update_line(db, active).await;
                                }
                            } else if line.vat_rate_id.is_some() {
                                // Non-taxable line has vat_rate_id — remove it (cleanup)
                                let mut active: journal_line::ActiveModel = line.into();
                                active.vat_rate_id = Set(None);
                                let _ = JournalRepo::update_line(db, active).await;
                            }
                        }
                    }
                    // Skip — already imported
                    continue;
                }
        }

        // --- Create new entry ---
        let mut lines = Vec::new();

        if let Some(acct_id) = row.debit_account.and_then(|num| account_num_to_id.get(&num).map(|id| (num, id))) {
            let (debit_num, acct_id) = acct_id;
            let line_vat = if is_taxable_account(debit_num) { vat_rate_id.clone() } else { None };
            lines.push(JournalLineInput {
                account_id: acct_id.clone(),
                debit_amount: amount,
                credit_amount: Decimal::ZERO,
                description: row.description.clone(),
                vat_rate_id: line_vat,
            });
        }

        if let Some((credit_num, acct_id)) = row.credit_account.and_then(|num| account_num_to_id.get(&num).map(|id| (num, id))) {
            let line_vat = if is_taxable_account(credit_num) { vat_rate_id.clone() } else { None };
            lines.push(JournalLineInput {
                account_id: acct_id.clone(),
                debit_amount: Decimal::ZERO,
                credit_amount: amount,
                description: row.description.clone(),
                vat_rate_id: line_vat,
            });
        }

        if lines.len() < 2 {
            errors += 1;
            error_log.push(format!("Row ref={:?} date={}: could not resolve both accounts (debit={:?} credit={:?})",
                row.reference, row.date, row.debit_account, row.credit_account));
            continue;
        }

        let desc = row.description.unwrap_or_else(|| "Imported entry".to_string());
        match JournalService::create(db, date, &desc, row.reference.clone(), None, None, None, lines).await {
            Ok((entry, _)) => {
                // Auto-post imported entries since they are already verified in the source system
                let _ = JournalService::post_entry(db, &entry.id).await;
                imported += 1;
            }
            Err(e) => {
                errors += 1;
                error_log.push(format!("Row ref={:?} date={}: {}", row.reference, row.date, e));
            }
        }
    }

    Ok((imported, errors, error_log))
}
