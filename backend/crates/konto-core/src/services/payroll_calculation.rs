use chrono::{Datelike, NaiveDate};
use konto_db::entities::{employee, payroll_setting};
use rust_decimal::Decimal;

const HUNDRED: Decimal = Decimal::from_parts(100, 0, 0, false, 0);
const TWELVE: Decimal = Decimal::from_parts(12, 0, 0, false, 0);

/// Result of calculating a single employee's payroll line.
#[derive(Debug, Clone)]
pub struct PayrollLineCalc {
    pub gross_salary: Decimal,
    pub ahv_employee: Decimal,
    pub ahv_employer: Decimal,
    pub alv_employee: Decimal,
    pub alv_employer: Decimal,
    pub bvg_employee: Decimal,
    pub bvg_employer: Decimal,
    pub nbu_employee: Decimal,
    pub bu_employer: Decimal,
    pub ktg_employee: Decimal,
    pub ktg_employer: Decimal,
    pub fak_employer: Decimal,
    pub quellensteuer: Decimal,
    pub child_allowance: Decimal,
    pub net_salary: Decimal,
    pub payout_amount: Decimal,
    pub total_employer_cost: Decimal,
}

/// Calculate AHV/IV/EO contribution: gross * rate / 100
pub fn calculate_ahv(gross_monthly: Decimal, rate_pct: Decimal) -> Decimal {
    round2(gross_monthly * rate_pct / HUNDRED)
}

/// Calculate ALV contribution (monthly, capped at annual salary cap).
/// If annual gross exceeds cap, ALV is based on cap/12.
pub fn calculate_alv(
    gross_monthly: Decimal,
    gross_annual: Decimal,
    rate_pct: Decimal,
    salary_cap: Decimal,
) -> Decimal {
    let basis = if gross_annual > salary_cap {
        salary_cap / TWELVE
    } else {
        gross_monthly
    };
    round2(basis * rate_pct / HUNDRED)
}

/// Calculate BVG (occupational pension) based on employee age and coordinated salary.
/// Returns (employee_share, employer_share).
pub fn calculate_bvg(
    gross_annual: Decimal,
    employee_dob: NaiveDate,
    run_date: NaiveDate,
    settings: &payroll_setting::Model,
) -> (Decimal, Decimal) {
    // Check if salary meets entry threshold
    if gross_annual < settings.bvg_entry_threshold {
        return (Decimal::ZERO, Decimal::ZERO);
    }

    // Coordinated (insured) salary = annual gross - coordination deduction
    let coordinated = gross_annual - settings.bvg_coordination_deduction;
    let coordinated = coordinated.max(settings.bvg_min_insured_salary);
    let coordinated = coordinated.min(settings.bvg_max_insured_salary);

    // Age-based savings rate
    let age = calculate_age(employee_dob, run_date);
    let savings_rate = get_bvg_savings_rate(age, settings);

    // Total rate = savings + risk
    let total_rate = savings_rate + settings.bvg_risk_rate;

    // Monthly total contribution
    let monthly_total = round2(coordinated * total_rate / HUNDRED / TWELVE);

    // Split between employee and employer
    let employer_share = settings.bvg_employer_share_pct / HUNDRED;
    let employer_amount = round2(monthly_total * employer_share);
    let employee_amount = monthly_total - employer_amount;

    (employee_amount, employer_amount)
}

/// Calculate NBU (non-occupational accident insurance): monthly, capped.
pub fn calculate_nbu(
    gross_monthly: Decimal,
    gross_annual: Decimal,
    rate_pct: Decimal,
    uvg_max_salary: Decimal,
) -> Decimal {
    let basis = if gross_annual > uvg_max_salary {
        uvg_max_salary / TWELVE
    } else {
        gross_monthly
    };
    round2(basis * rate_pct / HUNDRED)
}

/// Calculate the full payroll line for one employee.
pub fn calculate_payroll_line(
    employee: &employee::Model,
    settings: &payroll_setting::Model,
    run_date: NaiveDate,
) -> PayrollLineCalc {
    let pct = employee.employment_percentage / HUNDRED;
    let gross_monthly = round2(employee.gross_monthly_salary * pct);
    let gross_annual = gross_monthly * TWELVE;

    // AHV/IV/EO
    let ahv_employee = calculate_ahv(gross_monthly, settings.ahv_iv_eo_rate_employee);
    let ahv_employer = calculate_ahv(gross_monthly, settings.ahv_iv_eo_rate_employer);

    // ALV
    let alv_employee = calculate_alv(
        gross_monthly, gross_annual, settings.alv_rate_employee, settings.alv_salary_cap,
    );
    let alv_employer = calculate_alv(
        gross_monthly, gross_annual, settings.alv_rate_employer, settings.alv_salary_cap,
    );

    // BVG
    let (bvg_employee, bvg_employer) =
        calculate_bvg(gross_annual, employee.date_of_birth, run_date, settings);

    // NBU (employee pays)
    let nbu_employee = calculate_nbu(
        gross_monthly, gross_annual, settings.nbu_rate_employee, settings.uvg_max_salary,
    );

    // BU (employer pays)
    let bu_employer = calculate_nbu(
        gross_monthly, gross_annual, settings.bu_rate_employer, settings.uvg_max_salary,
    );

    // KTG
    let ktg_employee = round2(gross_monthly * settings.ktg_rate_employee / HUNDRED);
    let ktg_employer = round2(gross_monthly * settings.ktg_rate_employer / HUNDRED);

    // FAK (employer pays)
    let fak_employer = round2(gross_monthly * settings.fak_rate_employer / HUNDRED);

    // Quellensteuer (withholding tax)
    let quellensteuer = if employee.is_quellensteuer {
        let rate = employee.quellensteuer_rate.unwrap_or(Decimal::ZERO);
        round2(gross_monthly * rate / HUNDRED)
    } else {
        Decimal::ZERO
    };

    // Child allowance
    let child_allowance = if employee.has_children {
        round2(employee.child_allowance_amount * Decimal::from(employee.number_of_children))
    } else {
        Decimal::ZERO
    };

    // Net salary = gross - employee deductions - quellensteuer
    let employee_deductions = ahv_employee + alv_employee + bvg_employee + nbu_employee + ktg_employee;
    let net_salary = gross_monthly - employee_deductions - quellensteuer;

    // Payout = net + child allowance
    let payout_amount = net_salary + child_allowance;

    // Total employer cost = gross + employer contributions
    let employer_contributions = ahv_employer + alv_employer + bvg_employer + bu_employer + ktg_employer + fak_employer;
    let total_employer_cost = gross_monthly + employer_contributions;

    PayrollLineCalc {
        gross_salary: gross_monthly,
        ahv_employee, ahv_employer,
        alv_employee, alv_employer,
        bvg_employee, bvg_employer,
        nbu_employee, bu_employer,
        ktg_employee, ktg_employer,
        fak_employer,
        quellensteuer,
        child_allowance,
        net_salary,
        payout_amount,
        total_employer_cost,
    }
}

pub fn calculate_age(dob: NaiveDate, on_date: NaiveDate) -> i32 {
    let mut age = on_date.year() - dob.year();
    if (on_date.month(), on_date.day()) < (dob.month(), dob.day()) {
        age -= 1;
    }
    age
}

pub fn get_bvg_savings_rate(age: i32, settings: &payroll_setting::Model) -> Decimal {
    match age {
        25..=34 => settings.bvg_rate_25_34,
        35..=44 => settings.bvg_rate_35_44,
        45..=54 => settings.bvg_rate_45_54,
        55..=65 => settings.bvg_rate_55_65,
        _ => Decimal::ZERO, // Under 25 or over 65: no BVG savings
    }
}

fn round2(d: Decimal) -> Decimal {
    d.round_dp(2)
}
