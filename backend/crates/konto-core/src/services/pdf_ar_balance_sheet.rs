use super::ch_account_groups::GroupedSection;
use super::pdf_annual_report::{esc, fmt_money};
use super::report_types::AnnualReportData;

/// Renders Bilanz (Aktiven + Passiven) pages.
pub fn render(data: &AnnualReportData) -> String {
    let year = &data.fiscal_year_end[..4];
    let prior_year = data
        .balance_sheet_prior
        .as_ref()
        .map(|bs| &bs.as_of[..4])
        .unwrap_or("-");

    let header = format!(
        r##"
#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm), header: align(right)[#text(size: 7pt, fill: gray)[{company}, {city}]])
#pagebreak()

#text(size: 14pt, weight: "bold")[BILANZ PER 31. DEZEMBER {year}]
#v(6mm)
"##,
        company = esc(&data.company_name),
        city = esc(&data.company_city),
        year = year,
    );

    // AKTIVEN
    let mut aktiven = String::from(
        r#"#text(size: 12pt, weight: "bold")[AKTIVEN]
#v(4mm)
"#,
    );

    for section in &data.balance_sheet_current.assets {
        let prior_section = data
            .balance_sheet_prior
            .as_ref()
            .and_then(|bs| bs.assets.iter().find(|s| s.key == section.key));
        aktiven.push_str(&render_section(section, prior_section, year, prior_year));
    }

    // Total Aktiven
    let prior_total_assets = data
        .balance_sheet_prior
        .as_ref()
        .map(|bs| fmt_money(bs.total_assets))
        .unwrap_or_default();
    aktiven.push_str(&format!(
        r##"
#table(
  columns: (1fr, 80pt, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 4pt),
  table.hline(stroke: 1.2pt),
  [#text(weight: "bold")[TOTAL AKTIVEN]], [#align(right)[#text(weight: "bold")[{current}]]], [#align(right)[#text(weight: "bold")[{prior}]]],
  table.hline(stroke: 1.2pt),
)
"##,
        current = fmt_money(data.balance_sheet_current.total_assets),
        prior = prior_total_assets,
    ));

    // PASSIVEN
    let mut passiven = String::from(
        r##"
#pagebreak()

#text(size: 14pt, weight: "bold")[BILANZ PER 31. DEZEMBER "##,
    );
    passiven.push_str(year);
    passiven.push_str(
        r##"]
#v(6mm)
#text(size: 12pt, weight: "bold")[PASSIVEN]
#v(4mm)
"##,
    );

    for section in &data.balance_sheet_current.liabilities {
        let prior_section = data
            .balance_sheet_prior
            .as_ref()
            .and_then(|bs| bs.liabilities.iter().find(|s| s.key == section.key));
        passiven.push_str(&render_section(section, prior_section, year, prior_year));
    }

    // Total Passiven
    let prior_total_liab = data
        .balance_sheet_prior
        .as_ref()
        .map(|bs| fmt_money(bs.total_liabilities))
        .unwrap_or_default();
    passiven.push_str(&format!(
        r##"
#table(
  columns: (1fr, 80pt, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 4pt),
  table.hline(stroke: 1.2pt),
  [#text(weight: "bold")[TOTAL PASSIVEN]], [#align(right)[#text(weight: "bold")[{current}]]], [#align(right)[#text(weight: "bold")[{prior}]]],
  table.hline(stroke: 1.2pt),
)
"##,
        current = fmt_money(data.balance_sheet_current.total_liabilities),
        prior = prior_total_liab,
    ));

    // Signature lines
    passiven.push_str(signature_lines());

    format!("{header}{aktiven}{passiven}")
}

fn render_section(
    section: &GroupedSection,
    prior: Option<&GroupedSection>,
    year: &str,
    prior_year: &str,
) -> String {
    let mut out = String::new();

    // Section header
    out.push_str(&format!(
        r##"#table(
  columns: (1fr, 80pt, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 3pt),
  fill: (_, row) => if row == 0 {{ rgb("#e8f0fe") }} else {{ none }},
  [#text(weight: "bold")[{label}]], [#align(right)[#text(weight: "bold", size: 8pt)[{year}]]], [#align(right)[#text(weight: "bold", size: 8pt)[{prior_year}]]],
"##,
        label = esc(&section.label),
        year = year,
        prior_year = prior_year,
    ));

    for group in &section.groups {
        let prior_subtotal = prior
            .and_then(|ps| ps.groups.iter().find(|g| g.label == group.label))
            .map(|g| fmt_money(g.subtotal))
            .unwrap_or_default();

        // Account lines
        for acct in &group.accounts {
            out.push_str(&format!(
                "  [#h(10pt)#text(size: 8pt)[{num} {name}]], [#align(right)[#text(size: 8pt)[{bal}]]], [],\n",
                num = acct.account_number,
                name = esc(&acct.account_name),
                bal = fmt_money(acct.balance),
            ));
        }

        // Group subtotal
        out.push_str(&format!(
            "  [#text(size: 8pt, style: \"italic\")[{label}]], [#align(right)[#text(size: 8pt, weight: \"bold\")[{current}]]], [#align(right)[#text(size: 8pt)[{prior}]]],\n",
            label = esc(&group.total_label),
            current = fmt_money(group.subtotal),
            prior = prior_subtotal,
        ));
    }

    // Section total
    let prior_total = prior.map(|p| fmt_money(p.total)).unwrap_or_default();
    out.push_str(&format!(
        "  table.hline(stroke: 0.4pt),\n  [#text(weight: \"bold\")[Total {label}]], [#align(right)[#text(weight: \"bold\")[{current}]]], [#align(right)[{prior}]],\n)\n#v(4mm)\n",
        label = esc(&section.label),
        current = fmt_money(section.total),
        prior = prior_total,
    ));

    out
}

fn signature_lines() -> &'static str {
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
"##
}
