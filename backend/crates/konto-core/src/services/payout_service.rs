use chrono::Utc;
use konto_common::error::AppError;
use konto_common::enums::PayrollRunStatus;
use konto_common::enums::ExpenseStatus;
use konto_db::entities::payout_entry;
use konto_db::repository::employee_repo::EmployeeRepo;
use konto_db::repository::payroll_run_line_repo::PayrollRunLineRepo;
use konto_db::repository::payroll_run_repo::PayrollRunRepo;
use konto_db::repository::payout_entry_repo::PayoutEntryRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct PayoutService;

impl PayoutService {
    pub async fn generate_payouts(
        db: &DatabaseConnection,
        payroll_run_id: &str,
    ) -> Result<Vec<payout_entry::Model>, AppError> {
        let run = PayrollRunRepo::find_by_id(db, payroll_run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll run not found".into()))?;

        if run.status != PayrollRunStatus::Approved.as_str() && run.status != PayrollRunStatus::Paid.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot generate payouts for run with status '{}'. Must be 'approved' or 'paid'.",
                run.status
            )));
        }

        // Delete existing payout entries for this run
        PayoutEntryRepo::delete_by_run_id(db, payroll_run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Load all payroll run lines
        let lines = PayrollRunLineRepo::find_by_run(db, payroll_run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if lines.is_empty() {
            return Err(AppError::Validation("No payroll lines found for this run".into()));
        }

        let now = Utc::now().naive_utc();
        let mut entries = Vec::new();

        for line in &lines {
            let employee = EmployeeRepo::find_by_id(db, &line.employee_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound(format!("Employee {} not found", line.employee_id)))?;

            let payment_reference = format!(
                "LOHN-{:04}-{:02}-{}",
                run.period_year, run.period_month, employee.last_name
            );

            let model = payout_entry::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                payroll_run_id: Set(payroll_run_id.to_string()),
                employee_id: Set(employee.id.clone()),
                amount: Set(line.payout_amount),
                iban: Set(employee.iban.clone()),
                bic: Set(employee.bic.clone()),
                recipient_name: Set(format!("{} {}", employee.first_name, employee.last_name)),
                recipient_street: Set(employee.street.clone()),
                recipient_postal_code: Set(employee.postal_code.clone()),
                recipient_city: Set(employee.city.clone()),
                recipient_country: Set(employee.country.clone()),
                status: Set(ExpenseStatus::Pending.to_string()),
                paid_at: Set(None),
                payment_reference: Set(payment_reference),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let entry = PayoutEntryRepo::create(db, model)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn list_by_run(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<Vec<payout_entry::Model>, AppError> {
        PayoutEntryRepo::find_by_run_id(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn mark_paid(
        db: &DatabaseConnection,
        entry_id: &str,
    ) -> Result<payout_entry::Model, AppError> {
        let entry = PayoutEntryRepo::find_by_id(db, entry_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payout entry not found".into()))?;

        if entry.status == ExpenseStatus::Paid.as_str() {
            return Err(AppError::Validation("Entry is already paid".into()));
        }

        let now = Utc::now().naive_utc();
        let mut model: payout_entry::ActiveModel = entry.into();
        model.status = Set(ExpenseStatus::Paid.to_string());
        model.paid_at = Set(Some(now));
        model.updated_at = Set(now);

        PayoutEntryRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn mark_all_paid(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<(), AppError> {
        let entries = PayoutEntryRepo::find_by_run_id(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let now = Utc::now().naive_utc();

        for entry in entries {
            if entry.status != ExpenseStatus::Paid.as_str() {
                let mut model: payout_entry::ActiveModel = entry.into();
                model.status = Set(ExpenseStatus::Paid.to_string());
                model.paid_at = Set(Some(now));
                model.updated_at = Set(now);

                PayoutEntryRepo::update(db, model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }

        Ok(())
    }
}
