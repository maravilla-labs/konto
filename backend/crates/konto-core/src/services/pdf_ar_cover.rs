use super::pdf_annual_report::esc;
use super::report_types::AnnualReportData;

/// Renders the cover page of the Jahresrechnung.
pub fn render(data: &AnnualReportData) -> String {
    let year = &data.fiscal_year_end[..4];
    let company = esc(&data.company_name);
    let city = esc(&data.company_city);
    let entity_type = esc(&data.legal_entity_type);

    format!(
        r##"
#set page(paper: "a4", margin: (top: 20mm, bottom: 20mm, left: 25mm, right: 25mm))

// Cover page
#v(1fr)

#align(center)[
  #block(
    width: 100%,
    fill: rgb("#1a56db"),
    inset: (x: 20pt, y: 30pt),
    radius: 4pt,
  )[
    #text(size: 24pt, weight: "bold", fill: white)[{company}]
    #v(4mm)
    #text(size: 14pt, fill: white)[{entity_type}, {city}]
  ]
]

#v(30mm)

#align(center)[
  #text(size: 20pt, weight: "bold")[JAHRESRECHNUNG]
  #v(6mm)
  #text(size: 16pt)[PER 31. DEZEMBER {year}]
]

#v(1fr)

#align(center)[
  #text(size: 10pt, fill: gray)[Geschäftsjahr {fy_name}]
]

#v(20mm)
"##,
        company = company,
        entity_type = entity_type,
        city = city,
        year = year,
        fy_name = esc(&data.fiscal_year_name),
    )
}
