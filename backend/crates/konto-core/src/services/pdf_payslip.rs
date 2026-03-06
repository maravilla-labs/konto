use konto_common::error::AppError;
use konto_db::entities::{company_setting, employee, payroll_run, payroll_run_line, payroll_setting};
use konto_db::repository::employee_repo::EmployeeRepo;
use konto_db::repository::payroll_run_line_repo::PayrollRunLineRepo;
use konto_db::repository::payroll_run_repo::PayrollRunRepo;
use konto_db::repository::payroll_setting_repo::PayrollSettingRepo;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;

use super::payroll_calculation::{calculate_age, get_bvg_savings_rate};

pub struct PdfPayslipService;

impl PdfPayslipService {
    pub async fn generate(
        db: &DatabaseConnection,
        run_id: &str,
        employee_id: &str,
    ) -> Result<Vec<u8>, AppError> {
        let data = fetch_data(db, run_id, employee_id).await?;
        render_typst_pdf(&data)
    }

    pub async fn generate_all(
        db: &DatabaseConnection,
        run_id: &str,
    ) -> Result<Vec<(String, Vec<u8>)>, AppError> {
        let run = PayrollRunRepo::find_by_id(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll run not found".into()))?;
        let lines = PayrollRunLineRepo::find_by_run(db, run_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
        let payroll_settings = PayrollSettingRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Payroll settings not configured".into()))?;

        let mut results = Vec::new();
        for line in &lines {
            let emp = EmployeeRepo::find_by_id(db, &line.employee_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;
            let name = format!("{}-{}", emp.last_name, emp.first_name);
            let data = PayslipData {
                run: run.clone(),
                line: line.clone(),
                employee: emp,
                settings: settings.clone(),
                payroll_settings: payroll_settings.clone(),
            };
            let pdf = render_typst_pdf(&data)?;
            results.push((name, pdf));
        }
        Ok(results)
    }
}

struct PayslipData {
    run: payroll_run::Model,
    line: payroll_run_line::Model,
    employee: employee::Model,
    settings: company_setting::Model,
    payroll_settings: payroll_setting::Model,
}

async fn fetch_data(
    db: &DatabaseConnection,
    run_id: &str,
    employee_id: &str,
) -> Result<PayslipData, AppError> {
    let run = PayrollRunRepo::find_by_id(db, run_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Payroll run not found".into()))?;
    let lines = PayrollRunLineRepo::find_by_run(db, run_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    let line = lines
        .into_iter()
        .find(|l| l.employee_id == employee_id)
        .ok_or_else(|| AppError::NotFound("Payroll line not found for employee".into()))?;
    let employee = EmployeeRepo::find_by_id(db, employee_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;
    let settings = SettingsRepo::find(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
    let payroll_settings = PayrollSettingRepo::find(db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Payroll settings not configured".into()))?;

    Ok(PayslipData { run, line, employee, settings, payroll_settings })
}

fn render_typst_pdf(data: &PayslipData) -> Result<Vec<u8>, AppError> {
    use typst::layout::PagedDocument;
    use typst_as_lib::TypstEngine;
    use typst_as_lib::typst_kit_options::TypstKitFontOptions;

    let typst_source = build_typst_source(data);

    let mut binaries: Vec<(&str, Vec<u8>)> = Vec::new();
    if let Some(logo) = load_logo(&data.settings) {
        binaries.push(("logo.png", logo));
    }
    let bin_refs: Vec<(&str, &[u8])> = binaries
        .iter()
        .map(|(name, data)| (*name, data.as_slice()))
        .collect();

    let font_opts = TypstKitFontOptions::new()
        .include_system_fonts(true)
        .include_embedded_fonts(true);

    let engine = TypstEngine::builder()
        .main_file(typst_source.as_str())
        .search_fonts_with(font_opts)
        .with_static_file_resolver(bin_refs)
        .build();

    let doc: PagedDocument = engine
        .compile()
        .output
        .map_err(|e| AppError::Internal(format!("Typst compile error: {e}")))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| AppError::Internal(format!("PDF generation failed: {e:?}")))?;
    Ok(pdf)
}

const MONTH_NAMES: [&str; 12] = [
    "Januar", "Februar", "März", "April", "Mai", "Juni",
    "Juli", "August", "September", "Oktober", "November", "Dezember",
];

fn build_typst_source(data: &PayslipData) -> String {
    let s = &data.settings;
    let e = &data.employee;
    let l = &data.line;
    let r = &data.run;
    let ps = &data.payroll_settings;

    let has_logo = load_logo(s).is_some();
    let logo_block = if has_logo {
        r#"#image("logo.png", width: 35mm)"#.to_string()
    } else {
        let name = s.trade_name.as_deref().unwrap_or(&s.legal_name);
        format!(r#"#text(size: 16pt, weight: "bold")[{}]"#, esc(name))
    };

    let company_addr = format!(
        "{}\\\n{} {}",
        &s.street,
        &s.postal_code,
        &s.city
    );

    let period = format!(
        "{} {}",
        MONTH_NAMES.get((r.period_month - 1) as usize).unwrap_or(&""),
        r.period_year
    );

    let total_deductions = l.ahv_employee + l.alv_employee + l.bvg_employee
        + l.nbu_employee + l.ktg_employee + l.quellensteuer;

    // Calculate BVG effective rate for display
    let age = calculate_age(e.date_of_birth, r.run_date);
    let bvg_savings_rate = get_bvg_savings_rate(age, ps);
    let bvg_total_rate = bvg_savings_rate + ps.bvg_risk_rate;
    let bvg_ee_share = Decimal::from(100) - ps.bvg_employer_share_pct;
    let bvg_ee_display_rate = bvg_total_rate * bvg_ee_share / Decimal::from(100);

    let qs_row = if !l.quellensteuer.is_zero() {
        let qs_rate = e.quellensteuer_rate.unwrap_or(Decimal::ZERO);
        format!(
            "  [Quellensteuer ({qs_rate}%)], [#align(right)[- {qs_amt}]],\n",
            qs_rate = fmt_rate(qs_rate),
            qs_amt = fmt_money(l.quellensteuer),
        )
    } else {
        String::new()
    };

    let child_row = if !l.child_allowance.is_zero() {
        format!("  [Kinderzulagen], [#align(right)[+ {}]],\n", fmt_money(l.child_allowance))
    } else {
        String::new()
    };

    format!(
        r##"#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm))
#set text(size: 9pt, font: "Noto Sans")

#grid(
  columns: (1fr, auto),
  align(left)[
    {logo_block}
    #v(2pt)
    #text(size: 8pt, fill: gray)[{company_addr}]
  ],
  align(right)[
    #text(size: 8pt, fill: gray)[{legal}]
  ],
)

#v(10mm)

#grid(
  columns: (1fr, 1fr),
  [
    #text(weight: "bold")[{emp_name}]\
    {emp_addr}\
    {emp_postal} {emp_city}
  ],
  align(right)[
    #text(size: 8pt)[
      AHV-Nr: {ahv}\
      Eintritt: {entry}\
      Pensum: {pensum}%
    ]
  ],
)

#v(8mm)
#text(size: 14pt, weight: "bold")[Lohnabrechnung {period}]
#v(6mm)

#table(
  columns: (1fr, 100pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  table.hline(stroke: 0.6pt),
  [*Bruttolohn*], [#align(right)[*{gross}*]],
  table.hline(stroke: 0.4pt),
)

#v(4mm)

#table(
  columns: (1fr, 100pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  table.hline(stroke: 0.6pt),
  table.header([*Abzuege Arbeitnehmer*], []),
  table.hline(stroke: 0.4pt),
  [AHV / IV / EO ({ahv_rate}%)], [#align(right)[- {ahv_ee}]],
  [ALV ({alv_rate}%)], [#align(right)[- {alv_ee}]],
  [BVG ({bvg_rate}%)], [#align(right)[- {bvg_ee}]],
  [NBU ({nbu_rate}%)], [#align(right)[- {nbu_ee}]],
  [KTG ({ktg_rate}%)], [#align(right)[- {ktg_ee}]],
{qs_row}  table.hline(stroke: 0.4pt),
  [*Total Abzuege*], [#align(right)[*- {total_ded}*]],
  table.hline(stroke: 0.6pt),
)

#v(4mm)

#table(
  columns: (1fr, 100pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  table.hline(stroke: 0.6pt),
  [*Nettolohn*], [#align(right)[*{net}*]],
{child_row}  table.hline(stroke: 0.4pt),
  [*#text(size: 11pt)[Auszahlung]*], [#align(right)[*#text(size: 11pt)[{payout}]*]],
  table.hline(stroke: 0.6pt),
)

#v(10mm)
#line(length: 100%, stroke: 0.3pt + gray)
#text(size: 7pt, fill: gray)[
  Auszahlung auf Konto: {iban}\
  Erstellt am: {created}
]
"##,
        logo_block = logo_block,
        company_addr = esc(&company_addr),
        legal = esc(&s.legal_name),
        emp_name = esc(&format!("{} {}", e.first_name, e.last_name)),
        emp_addr = esc(&e.street),
        emp_postal = esc(&e.postal_code),
        emp_city = esc(&e.city),
        ahv = esc(&e.ahv_number),
        entry = e.employment_start.format("%d.%m.%Y"),
        pensum = e.employment_percentage,
        period = esc(&period),
        gross = fmt_money(l.gross_salary),
        ahv_rate = fmt_rate(ps.ahv_iv_eo_rate_employee),
        ahv_ee = fmt_money(l.ahv_employee),
        alv_rate = fmt_rate(ps.alv_rate_employee),
        alv_ee = fmt_money(l.alv_employee),
        bvg_rate = fmt_rate(bvg_ee_display_rate),
        bvg_ee = fmt_money(l.bvg_employee),
        nbu_rate = fmt_rate(ps.nbu_rate_employee),
        nbu_ee = fmt_money(l.nbu_employee),
        ktg_rate = fmt_rate(ps.ktg_rate_employee),
        ktg_ee = fmt_money(l.ktg_employee),
        qs_row = qs_row,
        total_ded = fmt_money(total_deductions),
        net = fmt_money(l.net_salary),
        child_row = child_row,
        payout = fmt_money(l.payout_amount),
        iban = esc(&e.iban),
        created = r.run_date.format("%d.%m.%Y"),
    )
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('@', "\\@")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('$', "\\$")
}

fn fmt_rate(d: Decimal) -> String {
    let s = format!("{}", d.normalize());
    s
}

fn fmt_money(d: Decimal) -> String {
    let s = format!("{:.2}", d);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");
    let neg = int_part.starts_with('-');
    let digits: String = int_part.chars().filter(|c| c.is_ascii_digit()).collect();
    let formatted: String = digits
        .chars()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("'")
        .chars()
        .rev()
        .collect();
    if neg {
        format!("-{formatted}.{dec_part}")
    } else {
        format!("{formatted}.{dec_part}")
    }
}

fn load_logo(settings: &company_setting::Model) -> Option<Vec<u8>> {
    settings
        .logo_url
        .as_ref()
        .and_then(|url| std::fs::read(url).ok())
        .or_else(|| std::fs::read("uploads/logo.png").ok())
}
