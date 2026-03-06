use super::pdf_annual_report::{esc, fmt_money};
use super::report_types::AnnualReportData;

/// Renders the Anhang (Notes) pages.
pub fn render(data: &AnnualReportData) -> String {
    let year = &data.fiscal_year_end[..4];

    let mut out = format!(
        r##"
#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm), header: align(right)[#text(size: 7pt, fill: gray)[{company}, {city}]])
#pagebreak()

#text(size: 14pt, weight: "bold")[ANHANG ZUR JAHRESRECHNUNG {year}]
#v(6mm)
"##,
        company = esc(&data.company_name),
        city = esc(&data.company_city),
        year = year,
    );

    // Data-driven rendering: iterate over ordered notes
    let mut section_num = 0;
    for note in &data.ordered_notes {
        // Skip audit_optout section when audit_optout is disabled
        if note.section_key == "audit_optout" && !data.audit_optout {
            continue;
        }

        section_num += 1;

        match note.section_type.as_str() {
            "auto_company_info" => {
                out.push_str(&render_company_info(data, section_num, &note.label));
            }
            "auto_fx_rates" => {
                out.push_str(&render_fx_rates(data, section_num, &note.label));
            }
            "employees" => {
                out.push_str(&render_employees(
                    &note.content,
                    section_num,
                    &note.label,
                ));
            }
            _ => {
                // text type (default)
                out.push_str(&render_text_section(
                    &note.content,
                    &note.section_key,
                    section_num,
                    &note.label,
                ));
            }
        }
    }

    // Signature
    out.push_str(
        r##"
#v(1fr)
#grid(
  columns: (1fr, 1fr),
  column-gutter: 30pt,
  [
    #v(15mm)
    #line(length: 100%, stroke: 0.4pt)
    #v(2mm)
    #text(size: 8pt)[Ort, Datum]
  ],
  [
    #v(15mm)
    #line(length: 100%, stroke: 0.4pt)
    #v(2mm)
    #text(size: 8pt)[Unterschrift]
  ],
)
"##,
    );

    out
}

fn render_text_section(
    content: &serde_json::Value,
    section_key: &str,
    num: usize,
    label: &str,
) -> String {
    let text_key = if section_key == "extraordinary" {
        "explanation"
    } else {
        "text"
    };
    let text = content
        .get(text_key)
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let display_text = if text.is_empty() && section_key == "extraordinary" {
        "Keine ausserordentlichen Positionen."
    } else {
        text
    };
    format!(
        r##"
#text(size: 11pt, weight: "bold")[{num}. {label}]
#v(3mm)
#text(size: 9pt)[{text}]
#v(6mm)
"##,
        num = num,
        label = esc(label),
        text = esc(display_text),
    )
}

fn render_company_info(data: &AnnualReportData, num: usize, label: &str) -> String {
    let entity_type = esc(&data.legal_entity_type);
    let company = esc(&data.company_name);
    let city = esc(&data.company_city);

    let mut shareholders_table = String::new();
    if !data.shareholders.is_empty() {
        shareholders_table.push_str(
            r##"#table(
  columns: (1fr, auto, 1fr, 1fr),
  stroke: none,
  inset: (x: 4pt, y: 3pt),
  table.hline(stroke: 0.4pt),
  [#text(weight: "bold", size: 8pt)[Name]], [#text(weight: "bold", size: 8pt)[Wohnort]], [#text(weight: "bold", size: 8pt)[Funktion]], [#text(weight: "bold", size: 8pt)[Zeichnungsberechtigung]],
  table.hline(stroke: 0.3pt),
"##,
        );
        for sh in &data.shareholders {
            let signing = sh.signing_rights.as_deref().unwrap_or("-");
            shareholders_table.push_str(&format!(
                "  [#text(size: 8pt)[{}]], [#text(size: 8pt)[{}]], [#text(size: 8pt)[{}]], [#text(size: 8pt)[{}]],\n",
                esc(&sh.name),
                esc(&sh.city),
                esc(&sh.role),
                esc(signing),
            ));
        }
        shareholders_table.push_str("  table.hline(stroke: 0.4pt),\n)\n");
    }

    format!(
        r##"
#text(size: 11pt, weight: "bold")[{num}. {label}]
#v(3mm)
#text(size: 9pt)[
  *Firma:* {company}\
  *Sitz:* {city}\
  *Rechtsform:* {entity_type}
]
#v(3mm)
#text(size: 9pt, weight: "bold")[Organe]
#v(2mm)
{shareholders_table}
#v(6mm)
"##,
        num = num,
        label = esc(label),
        entity_type = entity_type,
        company = company,
        city = city,
        shareholders_table = shareholders_table,
    )
}

fn render_employees(
    content: &serde_json::Value,
    num: usize,
    label: &str,
) -> String {
    let entries = content
        .get("entries")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut table = String::new();
    if !entries.is_empty() {
        table.push_str(
            r##"#table(
  columns: (1fr, auto),
  stroke: none,
  inset: (x: 4pt, y: 3pt),
  table.hline(stroke: 0.4pt),
  [#text(weight: "bold", size: 8pt)[Standort]], [#text(weight: "bold", size: 8pt)[Anzahl Arbeitsstellen]],
  table.hline(stroke: 0.3pt),
"##,
        );
        for entry in &entries {
            let location = entry
                .get("location")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let count = entry
                .get("count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            table.push_str(&format!(
                "  [#text(size: 8pt)[{}]], [#align(right)[#text(size: 8pt)[{}]]],\n",
                esc(location),
                count,
            ));
        }
        table.push_str("  table.hline(stroke: 0.4pt),\n)\n");
    } else {
        table.push_str(
            "#text(size: 9pt)[Keine Angaben zu Arbeitsstellen im Jahresdurchschnitt.]\n",
        );
    }

    format!(
        r##"
#text(size: 11pt, weight: "bold")[{num}. {label}]
#v(3mm)
{table}
#v(6mm)
"##,
        num = num,
        label = esc(label),
        table = table,
    )
}

fn render_fx_rates(data: &AnnualReportData, num: usize, label: &str) -> String {
    let mut table = String::new();
    if !data.fx_rates.is_empty() {
        table.push_str(
            r##"#table(
  columns: (1fr, auto, auto),
  stroke: none,
  inset: (x: 4pt, y: 3pt),
  table.hline(stroke: 0.4pt),
  [#text(weight: "bold", size: 8pt)[Währung]], [#text(weight: "bold", size: 8pt)[Kurs]], [#text(weight: "bold", size: 8pt)[Stichtag]],
  table.hline(stroke: 0.3pt),
"##,
        );
        for rate in &data.fx_rates {
            table.push_str(&format!(
                "  [#text(size: 8pt)[1 {} = CHF]], [#align(right)[#text(size: 8pt)[{}]]], [#text(size: 8pt)[{}]],\n",
                esc(&rate.currency_from),
                fmt_money(rate.rate),
                esc(&rate.valid_date),
            ));
        }
        table.push_str("  table.hline(stroke: 0.4pt),\n)\n");
    } else {
        table.push_str("#text(size: 9pt)[Keine Fremdwährungspositionen.]\n");
    }

    format!(
        r##"
#text(size: 11pt, weight: "bold")[{num}. {label}]
#v(3mm)
{table}
#v(6mm)
"##,
        num = num,
        label = esc(label),
        table = table,
    )
}
