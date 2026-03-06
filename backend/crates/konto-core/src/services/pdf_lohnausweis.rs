use konto_common::error::AppError;
use konto_db::entities::company_setting;
use konto_db::repository::settings_repo::SettingsRepo;
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;

use super::lohnausweis_service::{LohnausweisData, LohnausweisService};

pub struct PdfLohnausweisService;

impl PdfLohnausweisService {
    pub async fn generate(
        db: &DatabaseConnection,
        year: i32,
        employee_id: &str,
    ) -> Result<Vec<u8>, AppError> {
        let data = LohnausweisService::get_for_employee(db, year, employee_id).await?;
        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;
        render_typst_pdf(&data, &settings)
    }

    pub async fn generate_all(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<Vec<(String, Vec<u8>)>, AppError> {
        let all = LohnausweisService::list_for_year(db, year).await?;
        let settings = SettingsRepo::find(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Company settings not configured".into()))?;

        let mut results = Vec::new();
        for data in &all {
            let name = format!("{}-{}", data.employee.last_name, data.employee.first_name);
            let pdf = render_typst_pdf(data, &settings)?;
            results.push((name, pdf));
        }
        Ok(results)
    }
}

fn render_typst_pdf(
    data: &LohnausweisData,
    settings: &company_setting::Model,
) -> Result<Vec<u8>, AppError> {
    use typst::layout::PagedDocument;
    use typst_as_lib::TypstEngine;
    use typst_as_lib::typst_kit_options::TypstKitFontOptions;

    let typst_source = build_typst_source(data, settings);

    let font_opts = TypstKitFontOptions::new()
        .include_system_fonts(true)
        .include_embedded_fonts(true);

    let engine = TypstEngine::builder()
        .main_file(typst_source.as_str())
        .search_fonts_with(font_opts)
        .build();

    let doc: PagedDocument = engine
        .compile()
        .output
        .map_err(|e| AppError::Internal(format!("Typst compile error: {e}")))?;

    let pdf = typst_pdf::pdf(&doc, &Default::default())
        .map_err(|e| AppError::Internal(format!("PDF generation failed: {e:?}")))?;
    Ok(pdf)
}

fn build_typst_source(data: &LohnausweisData, settings: &company_setting::Model) -> String {
    let e = &data.employee;
    let employer_name = settings.trade_name.as_deref().unwrap_or(&settings.legal_name);
    let employer_addr = format!(
        "{}, {} {}",
        &settings.street,
        &settings.postal_code,
        &settings.city
    );

    let emp_end = e.employment_end.as_deref().unwrap_or("—");

    format!(
        r##"#set page(paper: "a4", margin: (top: 15mm, bottom: 15mm, left: 20mm, right: 20mm))
#set text(size: 9pt, font: "Noto Sans")

#align(center)[
  #text(size: 16pt, weight: "bold")[Lohnausweis {year}]
  #v(2pt)
  #text(size: 10pt)[ESTV Formular 11]
]

#v(8mm)

// Arbeitgeber
#rect(width: 100%, stroke: 0.5pt, inset: 8pt)[
  #text(size: 8pt, fill: gray)[Arbeitgeber / Employeur]\
  #text(weight: "bold")[{employer}]\
  {employer_addr}
]

#v(4mm)

// Arbeitnehmer
#rect(width: 100%, stroke: 0.5pt, inset: 8pt)[
  #grid(
    columns: (1fr, 1fr),
    [
      #text(size: 8pt, fill: gray)[Arbeitnehmer / Employe]\
      #text(weight: "bold")[{emp_name}]\
      {emp_addr}\
      {emp_postal} {emp_city}
    ],
    [
      #text(size: 8pt, fill: gray)[AHV-Nr / No AVS]\
      {ahv}\
      #v(4pt)
      #text(size: 8pt, fill: gray)[Geburtsdatum / Date de naissance]\
      {dob}\
      #v(4pt)
      #text(size: 8pt, fill: gray)[Zivilstand / Etat civil]\
      {marital}
    ],
  )
]

#v(4mm)

// Beschaeftigungszeitraum
#rect(width: 100%, stroke: 0.5pt, inset: 8pt)[
  #grid(
    columns: (1fr, 1fr),
    [
      #text(size: 8pt, fill: gray)[Eintritt / Entree]\
      {emp_start}
    ],
    [
      #text(size: 8pt, fill: gray)[Austritt / Sortie]\
      {emp_end}
    ],
  )
]

#v(6mm)

// Lohnbestandteile
#table(
  columns: (40pt, 1fr, 120pt),
  stroke: none,
  inset: (x: 6pt, y: 6pt),
  table.hline(stroke: 0.8pt),
  [*Ziff.*], [*Bezeichnung*], [*#align(right)[CHF]*],
  table.hline(stroke: 0.5pt),

  [1.], [Lohn (inkl. Naturalleistungen)], [],
  [], [Bruttolohn ({months} Monate)], [#align(right)[{gross}]],
  table.hline(stroke: 0.3pt),

  [8.], [*Bruttolohn total*], [*#align(right)[{gross}]*],
  table.hline(stroke: 0.5pt),

  [9.], [Beitraege AHV/IV/EO/ALV/NBU/KTG], [#align(right)[- {social}]],
  table.hline(stroke: 0.3pt),

  [10.], [BVG-Beitraege (Berufliche Vorsorge)], [#align(right)[- {bvg}]],
  table.hline(stroke: 0.3pt),

  [11.], [*Nettolohn*], [*#align(right)[{net}]*],
  table.hline(stroke: 0.5pt),

  {qs_rows}

  [13.], [Kinderzulagen / Allocations familiales], [#align(right)[{child}]],
  table.hline(stroke: 0.8pt),
)

#v(8mm)

// Bemerkungen
#rect(width: 100%, stroke: 0.5pt, inset: 8pt)[
  #text(size: 8pt, fill: gray)[Bemerkungen / Remarques]\
  #text(size: 8pt)[Dieses Dokument wurde maschinell erstellt und ist ohne Unterschrift gueltig.]
]

#v(12mm)

// Unterschrift
#grid(
  columns: (1fr, 1fr),
  [
    #text(size: 8pt, fill: gray)[Ort, Datum / Lieu, date]\
    #v(8mm)
    #line(length: 80%, stroke: 0.5pt)
  ],
  [
    #text(size: 8pt, fill: gray)[Stempel und Unterschrift / Timbre et signature]\
    #v(8mm)
    #line(length: 80%, stroke: 0.5pt)
  ],
)
"##,
        year = data.year,
        employer = esc(employer_name),
        employer_addr = esc(&employer_addr),
        emp_name = esc(&format!("{} {}", e.first_name, e.last_name)),
        emp_addr = esc(&e.street),
        emp_postal = esc(&e.postal_code),
        emp_city = esc(&e.city),
        ahv = esc(&e.ahv_number),
        dob = esc(&e.date_of_birth),
        marital = esc(&e.marital_status),
        emp_start = esc(&e.employment_start),
        emp_end = esc(emp_end),
        months = data.months_worked,
        gross = fmt_money(data.total_gross),
        social = fmt_money(data.total_social_deductions),
        bvg = fmt_money(data.total_bvg_employee),
        net = fmt_money(data.total_net),
        qs_rows = if !data.total_quellensteuer.is_zero() {
            format!(
                "  [12.], [Quellensteuer / Impot a la source], [#align(right)[- {}]],\n  table.hline(stroke: 0.3pt),\n",
                fmt_money(data.total_quellensteuer)
            )
        } else {
            String::new()
        },
        child = fmt_money(data.total_child_allowance),
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
