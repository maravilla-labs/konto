use super::pdf_annual_report::{esc, fmt_money};
use super::report_types::AnnualReportData;

/// Renders the Antrag (Profit Allocation Proposal) page.
pub fn render(data: &AnnualReportData) -> String {
    let year = &data.fiscal_year_end[..4];
    let net_result = data.current_net_result;
    let retained = data.prior_retained_earnings;
    let available = retained + net_result;

    // Default allocation: 5% to legal reserve, rest carried forward
    let reserve_5pct = net_result * rust_decimal::Decimal::new(5, 2);
    let carry_forward = available - reserve_5pct;

    let result_label = if net_result >= rust_decimal::Decimal::ZERO {
        "Jahresgewinn"
    } else {
        "Jahresverlust"
    };

    let retained_label = if retained >= rust_decimal::Decimal::ZERO {
        "Gewinnvortrag"
    } else {
        "Verlustvortrag"
    };

    format!(
        r##"
#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm), header: align(right)[#text(size: 7pt, fill: gray)[{company}, {city}]])
#pagebreak()

#text(size: 14pt, weight: "bold")[ANTRAG ÜBER DIE VERWENDUNG DES JAHRESERGEBNISSES {year}]
#v(8mm)

#text(size: 9pt)[Die Geschäftsführung beantragt der Gesellschafterversammlung, über das Jahresergebnis wie folgt zu beschliessen:]

#v(6mm)

#table(
  columns: (1fr, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  [{retained_label} aus Vorjahr], [#align(right)[{retained_amount}]],
  [{result_label} {year}], [#align(right)[{net_result}]],
  table.hline(stroke: 0.6pt),
  [#text(weight: "bold")[Zur Verfügung der Gesellschafterversammlung]], [#align(right)[#text(weight: "bold")[{available}]]],
)

#v(8mm)

#text(size: 9pt, weight: "bold")[Verwendung:]

#v(3mm)

#table(
  columns: (1fr, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 5pt),
  [Zuweisung an die gesetzliche Gewinnreserve (5%)], [#align(right)[{reserve}]],
  [Vortrag auf neue Rechnung], [#align(right)[{carry_forward}]],
  table.hline(stroke: 0.6pt),
  [#text(weight: "bold")[Total]], [#align(right)[#text(weight: "bold")[{available_check}]]],
)

#v(1fr)

// Signature block
#text(size: 9pt)[{city}, den \_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_\_]

#v(8mm)

#text(size: 9pt, weight: "bold")[Für die Geschäftsführung:]

#v(4mm)
{signatures}
"##,
        company = esc(&data.company_name),
        city = esc(&data.company_city),
        year = year,
        retained_label = retained_label,
        retained_amount = fmt_money(retained),
        result_label = result_label,
        net_result = fmt_money(net_result),
        available = fmt_money(available),
        reserve = fmt_money(reserve_5pct),
        carry_forward = fmt_money(carry_forward),
        available_check = fmt_money(available),
        signatures = render_signatures(data),
    )
}

fn render_signatures(data: &AnnualReportData) -> String {
    let signers: Vec<&super::report_types::ShareholderData> = data
        .shareholders
        .iter()
        .filter(|s| s.signing_rights.is_some())
        .collect();

    if signers.is_empty() {
        return String::new();
    }

    let cols = signers.len();
    let col_spec = (0..cols).map(|_| "1fr").collect::<Vec<_>>().join(", ");

    let mut lines = String::new();
    for signer in &signers {
        lines.push_str(&format!(
            "  [\n    #v(15mm)\n    #line(length: 100%, stroke: 0.4pt)\n    #v(2mm)\n    #text(size: 8pt)[{}]\n    #text(size: 7pt, fill: gray)[{}]\n  ],\n",
            esc(&signer.name),
            esc(&signer.role),
        ));
    }

    format!(
        "#grid(\n  columns: ({col_spec}),\n  column-gutter: 20pt,\n{lines})\n",
        col_spec = col_spec,
        lines = lines,
    )
}
