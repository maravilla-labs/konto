use chrono::{NaiveDate, Utc};
use konto_common::error::AppError;
use konto_common::enums::PayrollRunStatus;
use konto_db::entities::{payroll_run, payroll_run_line};
use konto_db::repository::employee_repo::EmployeeRepo;
use konto_db::repository::payroll_run_line_repo::PayrollRunLineRepo;
use konto_db::repository::payroll_run_repo::PayrollRunRepo;
use konto_db::repository::payroll_setting_repo::PayrollSettingRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

use super::journal_service::{JournalLineInput, JournalService};
use super::payroll_calculation::calculate_payroll_line;

pub struct PayrollRunService;

impl PayrollRunService {
    pub async fn list(
        db: &DatabaseConnection,
    ) -> Result<Vec<payroll_run::Model>, AppError> {
        PayrollRunRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<payroll_run::Model, AppError> {
        PayrollRunRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll run not found".into()))
    }

    pub async fn get_lines(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<Vec<payroll_run_line::Model>, AppError> {
        PayrollRunLineRepo::find_by_run(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_with_lines(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<(payroll_run::Model, Vec<payroll_run_line::Model>), AppError> {
        let run = Self::get(db, id).await?;
        let lines = Self::get_lines(db, id).await?;
        Ok((run, lines))
    }

    pub async fn create(
        db: &DatabaseConnection,
        month: i32,
        year: i32,
    ) -> Result<payroll_run::Model, AppError> {
        if !(1..=12).contains(&month) {
            return Err(AppError::Validation("Month must be 1-12".into()));
        }
        if year < 2000 {
            return Err(AppError::Validation("Invalid year".into()));
        }

        if PayrollRunRepo::find_by_period(db, month, year)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .is_some()
        {
            return Err(AppError::Conflict(format!(
                "Payroll run already exists for {month}/{year}"
            )));
        }

        let now = Utc::now().naive_utc();
        let run_date = NaiveDate::from_ymd_opt(year, month as u32, 25)
            .or_else(|| NaiveDate::from_ymd_opt(year, month as u32, 1))
            .ok_or_else(|| AppError::Validation(format!("Invalid date: {year}-{month}-25")))?;

        let model = payroll_run::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            period_month: Set(month),
            period_year: Set(year),
            status: Set(PayrollRunStatus::Draft.to_string()),
            run_date: Set(run_date),
            approved_by: Set(None),
            approved_at: Set(None),
            paid_at: Set(None),
            journal_entry_id: Set(None),
            payment_file_generated: Set(false),
            total_gross: Set(Decimal::ZERO),
            total_net: Set(Decimal::ZERO),
            total_employer_cost: Set(Decimal::ZERO),
            created_at: Set(now),
            updated_at: Set(now),
        };

        PayrollRunRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn calculate(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<(payroll_run::Model, Vec<payroll_run_line::Model>), AppError> {
        let run = Self::get(db, run_id).await?;
        if run.status != PayrollRunStatus::Draft.as_str() && run.status != PayrollRunStatus::Calculated.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot calculate run with status '{}'", run.status
            )));
        }

        PayrollRunLineRepo::delete_by_run(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let settings = PayrollSettingRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll settings not found".into()))?;

        let employees = EmployeeRepo::find_active(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if employees.is_empty() {
            return Err(AppError::Validation("No active employees found".into()));
        }

        let now = Utc::now().naive_utc();
        let mut total_gross = Decimal::ZERO;
        let mut total_net = Decimal::ZERO;
        let mut total_employer_cost = Decimal::ZERO;

        for emp in &employees {
            let calc = calculate_payroll_line(emp, &settings, run.run_date);

            let line = payroll_run_line::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                payroll_run_id: Set(run_id.to_string()),
                employee_id: Set(emp.id.clone()),
                gross_salary: Set(calc.gross_salary),
                ahv_employee: Set(calc.ahv_employee),
                ahv_employer: Set(calc.ahv_employer),
                alv_employee: Set(calc.alv_employee),
                alv_employer: Set(calc.alv_employer),
                bvg_employee: Set(calc.bvg_employee),
                bvg_employer: Set(calc.bvg_employer),
                nbu_employee: Set(calc.nbu_employee),
                bu_employer: Set(calc.bu_employer),
                ktg_employee: Set(calc.ktg_employee),
                ktg_employer: Set(calc.ktg_employer),
                fak_employer: Set(calc.fak_employer),
                quellensteuer: Set(calc.quellensteuer),
                child_allowance: Set(calc.child_allowance),
                net_salary: Set(calc.net_salary),
                payout_amount: Set(calc.payout_amount),
                total_employer_cost: Set(calc.total_employer_cost),
                created_at: Set(now),
            };

            PayrollRunLineRepo::create(db, line)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

            total_gross += calc.gross_salary;
            total_net += calc.payout_amount;
            total_employer_cost += calc.total_employer_cost;
        }

        let mut run_model: payroll_run::ActiveModel = run.into();
        run_model.status = Set(PayrollRunStatus::Calculated.to_string());
        tracing::info!(payroll_run_id = %run_id, action = "calculated", "Payroll run calculated");
        run_model.total_gross = Set(total_gross);
        run_model.total_net = Set(total_net);
        run_model.total_employer_cost = Set(total_employer_cost);
        run_model.updated_at = Set(now);

        let updated = PayrollRunRepo::update(db, run_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let lines = Self::get_lines(db, run_id).await?;
        Ok((updated, lines))
    }

    pub async fn approve(
        db: &DatabaseConnection,
        run_id: &str,
        user_id: &str,
    ) -> Result<payroll_run::Model, AppError> {
        let run = Self::get(db, run_id).await?;
        if run.status != PayrollRunStatus::Calculated.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot approve run with status '{}'. Must be 'calculated'.", run.status
            )));
        }

        let lines = PayrollRunLineRepo::find_by_run(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if lines.is_empty() {
            return Err(AppError::Validation("No payroll lines to approve".into()));
        }

        let journal_lines = build_payroll_journal_lines(&lines);
        let description = format!(
            "Payroll {}/{}", run.period_month, run.period_year
        );

        let (entry, _) = JournalService::create(
            db,
            run.run_date,
            &description,
            Some(format!("PAYROLL-{:02}/{}", run.period_month, run.period_year)),
            None, None,
            Some(user_id.to_string()),
            journal_lines,
        ).await?;

        JournalService::post_entry(db, &entry.id).await?;

        let now = Utc::now().naive_utc();
        let mut run_model: payroll_run::ActiveModel = run.into();
        run_model.status = Set(PayrollRunStatus::Approved.to_string());
        tracing::info!(payroll_run_id = %run_id, action = "approved", "Payroll run approved");
        run_model.approved_by = Set(Some(user_id.to_string()));
        run_model.approved_at = Set(Some(now));
        run_model.journal_entry_id = Set(Some(entry.id));
        run_model.updated_at = Set(now);

        PayrollRunRepo::update(db, run_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn mark_paid(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<payroll_run::Model, AppError> {
        let run = Self::get(db, run_id).await?;
        if run.status != PayrollRunStatus::Approved.as_str() {
            return Err(AppError::Validation(format!(
                "Cannot mark as paid with status '{}'. Must be 'approved'.", run.status
            )));
        }

        let now = Utc::now().naive_utc();
        let mut run_model: payroll_run::ActiveModel = run.into();
        run_model.status = Set(PayrollRunStatus::Paid.to_string());
        tracing::info!(payroll_run_id = %run_id, action = "paid", "Payroll run paid");
        run_model.paid_at = Set(Some(now));
        run_model.updated_at = Set(now);

        PayrollRunRepo::update(db, run_model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<(), AppError> {
        let run = Self::get(db, run_id).await?;
        if run.status != PayrollRunStatus::Draft.as_str() && run.status != PayrollRunStatus::Calculated.as_str() {
            return Err(AppError::Validation(
                "Only draft or calculated payroll runs can be deleted".into(),
            ));
        }

        PayrollRunRepo::delete(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

fn build_payroll_journal_lines(lines: &[payroll_run_line::Model]) -> Vec<JournalLineInput> {
    let mut sg = Decimal::ZERO; // sum gross
    let mut sae = Decimal::ZERO; // sum ahv employee
    let mut sar = Decimal::ZERO; // sum ahv employer
    let mut sle = Decimal::ZERO; // sum alv employee
    let mut slr = Decimal::ZERO; // sum alv employer
    let mut sbe = Decimal::ZERO; // sum bvg employee
    let mut sbr = Decimal::ZERO; // sum bvg employer
    let mut sne = Decimal::ZERO; // sum nbu employee
    let mut sub = Decimal::ZERO; // sum bu employer
    let mut ske = Decimal::ZERO; // sum ktg employee
    let mut skr = Decimal::ZERO; // sum ktg employer
    let mut sfr = Decimal::ZERO; // sum fak employer
    let mut sc = Decimal::ZERO;  // sum child allowance
    let mut sp = Decimal::ZERO;  // sum payout

    for l in lines {
        sg += l.gross_salary;
        sae += l.ahv_employee; sar += l.ahv_employer;
        sle += l.alv_employee; slr += l.alv_employer;
        sbe += l.bvg_employee; sbr += l.bvg_employer;
        sne += l.nbu_employee; sub += l.bu_employer;
        ske += l.ktg_employee; skr += l.ktg_employer;
        sfr += l.fak_employer; sc += l.child_allowance;
        sp += l.payout_amount;
    }

    let mut sq = Decimal::ZERO; // sum quellensteuer
    for l in lines {
        sq += l.quellensteuer;
    }

    let mut j = Vec::new();
    let z = Decimal::ZERO;

    // Debit side
    j.push(jl("acct-5000", sg, z, "Bruttolöhne"));
    let sv_ag = sar + slr; // AHV + ALV employer combined
    if !sv_ag.is_zero() { j.push(jl("acct-5700", sv_ag, z, "AHV/IV/EO/ALV AG")); }
    if !sfr.is_zero() { j.push(jl("acct-5710", sfr, z, "FAK AG")); }
    if !sbr.is_zero() { j.push(jl("acct-5720", sbr, z, "BVG AG")); }
    if !sub.is_zero() { j.push(jl("acct-5730", sub, z, "UVG/BU AG")); }
    if !skr.is_zero() { j.push(jl("acct-5740", skr, z, "KTG AG")); }

    // Credit side
    let kk_ahv = sae + sar + sle + slr;
    if !kk_ahv.is_zero() { j.push(jl("acct-2271", z, kk_ahv, "KK AHV/ALV")); }

    let kk_pk = sbe + sbr;
    if !kk_pk.is_zero() { j.push(jl("acct-2270", z, kk_pk, "KK PK/BVG")); }

    let kk_uvg = sne + sub;
    if !kk_uvg.is_zero() { j.push(jl("acct-2273", z, kk_uvg, "KK UVG")); }

    let kk_ktg = ske + skr;
    if !kk_ktg.is_zero() { j.push(jl("acct-2274", z, kk_ktg, "KK KTG")); }

    let kk_fak = sfr - sc;
    if kk_fak > z {
        j.push(jl("acct-2272", z, kk_fak, "KK FAK"));
    } else if kk_fak < z {
        j.push(jl("acct-2272", kk_fak.abs(), z, "KK FAK (Überschuss Kinderzulagen)"));
    }

    if !sq.is_zero() { j.push(jl("acct-2279", z, sq, "Quellensteuer")); }

    if !sp.is_zero() { j.push(jl("acct-1020", z, sp, "Lohnauszahlung")); }

    j
}

fn jl(account: &str, debit: Decimal, credit: Decimal, desc: &str) -> JournalLineInput {
    JournalLineInput {
        account_id: account.to_string(),
        debit_amount: debit,
        credit_amount: credit,
        description: Some(desc.to_string()),
        vat_rate_id: None,
    }
}
