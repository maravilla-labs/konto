use super::pdf_annual_report::{esc, fmt_money};
use super::report_types::AnnualReportData;

/// Renders the Erfolgsrechnung (Income Statement) pages.
pub fn render(data: &AnnualReportData) -> String {
    let year = &data.fiscal_year_end[..4];
    let prior_year = data
        .income_statement_prior
        .as_ref()
        .map(|is| &is.to_date[..4])
        .unwrap_or("-");

    let st = &data.income_statement_current.subtotals;
    let prior_st = data.income_statement_prior.as_ref().map(|is| &is.subtotals);

    let mut out = format!(
        r##"
#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm), header: align(right)[#text(size: 7pt, fill: gray)[{company}, {city}]])
#pagebreak()

#text(size: 14pt, weight: "bold")[ERFOLGSRECHNUNG {year}]
#v(6mm)

#table(
  columns: (1fr, 80pt, 80pt),
  stroke: none,
  inset: (x: 4pt, y: 3pt),
"##,
        company = esc(&data.company_name),
        city = esc(&data.company_city),
        year = year,
    );

    // Render each section
    for section in &data.income_statement_current.sections {
        let prior_section = data
            .income_statement_prior
            .as_ref()
            .and_then(|is| is.sections.iter().find(|s| s.key == section.key));

        // Section header with blue background — use table.cell(fill:) per cell
        out.push_str(&format!(
            "  table.cell(fill: rgb(\"#e8f0fe\"))[#text(weight: \"bold\")[{}]], table.cell(fill: rgb(\"#e8f0fe\"))[#align(right)[#text(weight: \"bold\", size: 8pt)[{}]]], table.cell(fill: rgb(\"#e8f0fe\"))[#align(right)[#text(weight: \"bold\", size: 8pt)[{}]]],\n",
            esc(&section.label),
            year,
            prior_year,
        ));

        // Account lines within each group
        for group in &section.groups {
            let prior_sub = prior_section
                .and_then(|ps| ps.groups.iter().find(|g| g.label == group.label))
                .map(|g| fmt_money(g.subtotal))
                .unwrap_or_default();

            for acct in &group.accounts {
                out.push_str(&format!(
                    "  [#h(10pt)#text(size: 8pt)[{} {}]], [#align(right)[#text(size: 8pt)[{}]]], [],\n",
                    acct.account_number,
                    esc(&acct.account_name),
                    fmt_money(acct.balance),
                ));
            }

            if group.accounts.len() > 1 {
                out.push_str(&format!(
                    "  [#text(size: 8pt, style: \"italic\")[{}]], [#align(right)[#text(size: 8pt, weight: \"bold\")[{}]]], [#align(right)[#text(size: 8pt)[{}]]],\n",
                    esc(&group.total_label),
                    fmt_money(group.subtotal),
                    prior_sub,
                ));
            }
        }

        // Section total
        let prior_total = prior_section
            .map(|ps| fmt_money(ps.total))
            .unwrap_or_default();
        out.push_str(&format!(
            "  table.hline(stroke: 0.4pt),\n  [#text(weight: \"bold\")[Total {}]], [#align(right)[#text(weight: \"bold\")[{}]]], [#align(right)[{}]],\n",
            esc(&section.label),
            fmt_money(section.total),
            prior_total,
        ));

        // Add intermediate subtotals after key sections
        match section.key.as_str() {
            "operating_revenue" => {
                let p = prior_st.map(|s| fmt_money(s.operating_revenue)).unwrap_or_default();
                out.push_str(&subtotal_row("Betriebsertrag", st.operating_revenue, &p));
            }
            "material_expense" => {
                let p = prior_st.map(|s| fmt_money(s.gross_profit_material)).unwrap_or_default();
                out.push_str(&subtotal_row("Bruttoergebnis nach Material", st.gross_profit_material, &p));
            }
            "personnel_expense" => {
                let p = prior_st.map(|s| fmt_money(s.gross_profit_personnel)).unwrap_or_default();
                out.push_str(&subtotal_row("Bruttoergebnis nach Personal", st.gross_profit_personnel, &p));
            }
            "other_opex" => {
                let p = prior_st.map(|s| fmt_money(s.ebitda)).unwrap_or_default();
                out.push_str(&subtotal_row("EBITDA", st.ebitda, &p));
            }
            "depreciation" => {
                let p = prior_st.map(|s| fmt_money(s.ebit)).unwrap_or_default();
                out.push_str(&subtotal_row("EBIT", st.ebit, &p));
            }
            "financial_result" => {
                let p = prior_st.map(|s| fmt_money(s.ebt)).unwrap_or_default();
                out.push_str(&subtotal_row("EBT", st.ebt, &p));
            }
            _ => {}
        }
    }

    // Final net result
    let prior_net = prior_st.map(|s| fmt_money(s.net_result)).unwrap_or_default();
    out.push_str(&format!(
        "  table.hline(stroke: 1.2pt),\n  [#text(size: 11pt, weight: \"bold\")[JAHRESERGEBNIS]], [#align(right)[#text(size: 11pt, weight: \"bold\")[{}]]], [#align(right)[#text(weight: \"bold\")[{}]]],\n  table.hline(stroke: 1.2pt),\n)\n",
        fmt_money(st.net_result),
        prior_net,
    ));

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

fn subtotal_row(label: &str, current: rust_decimal::Decimal, prior: &str) -> String {
    format!(
        "  table.cell(fill: rgb(\"#f0f4ff\"))[#text(weight: \"bold\", size: 9pt)[{}]], table.cell(fill: rgb(\"#f0f4ff\"))[#align(right)[#text(weight: \"bold\", size: 9pt)[{}]]], table.cell(fill: rgb(\"#f0f4ff\"))[#align(right)[#text(size: 9pt)[{}]]],\n",
        label,
        fmt_money(current),
        prior,
    )
}
