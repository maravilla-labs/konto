use konto_common::error::AppError;
use konto_common::enums::PayrollRunStatus;
use konto_db::entities::{employee, payroll_run, payroll_run_line};
use konto_db::repository::employee_repo::EmployeeRepo;
use konto_db::repository::payroll_run_line_repo::PayrollRunLineRepo;
use konto_db::repository::payroll_run_repo::PayrollRunRepo;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use utoipa::ToSchema;

/// Aggregated annual data for one employee's Lohnausweis.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LohnausweisData {
    pub employee: EmployeeSummary,
    pub year: i32,
    pub months_worked: i32,
    #[schema(value_type = f64)]
    pub total_gross: Decimal,
    #[schema(value_type = f64)]
    pub total_ahv_employee: Decimal,
    #[schema(value_type = f64)]
    pub total_alv_employee: Decimal,
    #[schema(value_type = f64)]
    pub total_bvg_employee: Decimal,
    #[schema(value_type = f64)]
    pub total_nbu_employee: Decimal,
    #[schema(value_type = f64)]
    pub total_ktg_employee: Decimal,
    #[schema(value_type = f64)]
    pub total_quellensteuer: Decimal,
    #[schema(value_type = f64)]
    pub total_child_allowance: Decimal,
    #[schema(value_type = f64)]
    pub total_net: Decimal,
    #[schema(value_type = f64)]
    pub total_payout: Decimal,
    #[schema(value_type = f64)]
    pub total_social_deductions: Decimal,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct EmployeeSummary {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub ahv_number: String,
    pub date_of_birth: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub employment_start: String,
    pub employment_end: Option<String>,
    pub marital_status: String,
}

pub struct LohnausweisService;

impl LohnausweisService {
    /// Aggregate all payroll run lines for a specific employee in a given year.
    pub async fn get_for_employee(
        db: &DatabaseConnection,
        year: i32,
        employee_id: &str,
    ) -> Result<LohnausweisData, AppError> {
        let emp = EmployeeRepo::find_by_id(db, employee_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;

        let runs = PayrollRunRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let year_runs: Vec<&payroll_run::Model> = runs
            .iter()
            .filter(|r| r.period_year == year && (r.status == PayrollRunStatus::Approved.as_str() || r.status == PayrollRunStatus::Paid.as_str()))
            .collect();

        let mut all_lines: Vec<payroll_run_line::Model> = Vec::new();
        for run in &year_runs {
            let lines = PayrollRunLineRepo::find_by_run(db, &run.id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            for line in lines {
                if line.employee_id == employee_id {
                    all_lines.push(line);
                }
            }
        }

        Ok(aggregate(&emp, year, &all_lines))
    }

    /// Get Lohnausweis data for all employees with payroll in a given year.
    pub async fn list_for_year(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<Vec<LohnausweisData>, AppError> {
        let runs = PayrollRunRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let year_runs: Vec<&payroll_run::Model> = runs
            .iter()
            .filter(|r| r.period_year == year && (r.status == PayrollRunStatus::Approved.as_str() || r.status == PayrollRunStatus::Paid.as_str()))
            .collect();

        // Collect all lines grouped by employee
        let mut emp_lines: std::collections::HashMap<String, Vec<payroll_run_line::Model>> =
            std::collections::HashMap::new();
        for run in &year_runs {
            let lines = PayrollRunLineRepo::find_by_run(db, &run.id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            for line in lines {
                emp_lines.entry(line.employee_id.clone()).or_default().push(line);
            }
        }

        let mut results = Vec::new();
        for (emp_id, lines) in &emp_lines {
            let emp = EmployeeRepo::find_by_id(db, emp_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound(format!("Employee {emp_id} not found")))?;
            results.push(aggregate(&emp, year, lines));
        }

        results.sort_by(|a, b| a.employee.last_name.cmp(&b.employee.last_name));
        Ok(results)
    }
}

fn aggregate(
    emp: &employee::Model,
    year: i32,
    lines: &[payroll_run_line::Model],
) -> LohnausweisData {
    let zero = Decimal::ZERO;
    let mut data = LohnausweisData {
        employee: EmployeeSummary {
            id: emp.id.clone(),
            first_name: emp.first_name.clone(),
            last_name: emp.last_name.clone(),
            ahv_number: emp.ahv_number.clone(),
            date_of_birth: emp.date_of_birth.format("%d.%m.%Y").to_string(),
            street: emp.street.clone(),
            postal_code: emp.postal_code.clone(),
            city: emp.city.clone(),
            country: emp.country.clone(),
            employment_start: emp.employment_start.format("%d.%m.%Y").to_string(),
            employment_end: emp.employment_end.map(|d| d.format("%d.%m.%Y").to_string()),
            marital_status: emp.marital_status.clone(),
        },
        year,
        months_worked: lines.len() as i32,
        total_gross: zero,
        total_ahv_employee: zero,
        total_alv_employee: zero,
        total_bvg_employee: zero,
        total_nbu_employee: zero,
        total_ktg_employee: zero,
        total_quellensteuer: zero,
        total_child_allowance: zero,
        total_net: zero,
        total_payout: zero,
        total_social_deductions: zero,
    };

    for line in lines {
        data.total_gross += line.gross_salary;
        data.total_ahv_employee += line.ahv_employee;
        data.total_alv_employee += line.alv_employee;
        data.total_bvg_employee += line.bvg_employee;
        data.total_nbu_employee += line.nbu_employee;
        data.total_ktg_employee += line.ktg_employee;
        data.total_quellensteuer += line.quellensteuer;
        data.total_child_allowance += line.child_allowance;
        data.total_net += line.net_salary;
        data.total_payout += line.payout_amount;
    }

    data.total_social_deductions = data.total_ahv_employee
        + data.total_alv_employee
        + data.total_nbu_employee
        + data.total_ktg_employee;

    data
}
