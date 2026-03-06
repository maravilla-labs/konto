/// Template seed data: returns (id, name, template_type, content_json,
/// header_json, footer_json, page_setup_json) tuples.
///
/// All JSON uses the DocumentModel block format with placeholder variables.
/// Standard page setup shared by most templates.
const PAGE_SETUP: &str = r#"{"format":"a4","orientation":"portrait","margins":{"top":20,"right":20,"bottom":20,"left":20},"headerHeight":30,"footerHeight":25}"#;

/// Default meta block (no special flags).
fn meta(align: &str, locked: bool, keep: bool, page_break: bool) -> String {
    format!(
        r#"{{"fontSize":null,"align":"{}","lineHeight":1.5,"font":"system","keepWithNext":{},"locked":{},"pageBreakBefore":{}}}"#,
        align, keep, locked, page_break
    )
}

fn m_default() -> String {
    meta("left", false, false, false)
}
fn m_locked() -> String {
    meta("left", true, false, false)
}
fn m_right_locked() -> String {
    meta("right", true, false, false)
}
fn m_keep() -> String {
    meta("left", false, true, false)
}
fn m_keep_break() -> String {
    meta("left", false, true, true)
}
fn m_sig() -> String {
    meta("left", true, false, false)
}

fn block(id: &str, btype: &str, data: &str, meta: &str) -> String {
    format!(
        r#"{{"id":"{}","type":"{}","data":{},"meta":{}}}"#,
        id, btype, data, meta
    )
}

fn text_data(text: &str) -> String {
    let escaped = text.replace('"', r#"\""#);
    format!(r#"{{"text":"{}","_html":"{}"}}"#, escaped, escaped)
}

fn table_data(headers: &[&str], rows: &[Vec<&str>]) -> String {
    let hdr: Vec<String> = headers.iter().map(|h| format!(r#""{}""#, h)).collect();
    let row_strs: Vec<String> = rows
        .iter()
        .map(|row| {
            let cells: Vec<String> = row.iter().map(|c| format!(r#""{}""#, c)).collect();
            format!("[{}]", cells.join(","))
        })
        .collect();
    format!(
        r#"{{"headers":[{}],"rows":[{}]}}"#,
        hdr.join(","),
        row_strs.join(",")
    )
}

fn signature_block(id: &str) -> String {
    let data = r#"{"parties":[{"role":"Service Provider","company":"{{company_name}}","location":"","date":"{{current_date}}","lines":[{"label":"Name:","value":""},{"label":"Title:","value":""}]},{"role":"Client","company":"{{client_name}}","location":"","date":"","lines":[{"label":"Name:","value":""},{"label":"Title:","value":""}]}]}"#;
    block(id, "signature", data, &m_sig())
}

// ── Letterhead ──────────────────────────────────────────────

fn letterhead_header() -> String {
    let blocks = [
        block("lh-h1", "image", &text_data("{{company_logo}}"), &m_default()),
        block("lh-h2", "paragraph", &text_data("**{{company_name}}**"), &m_default()),
        block("lh-h3", "paragraph", &text_data("{{company_address}}"), &m_default()),
    ];
    format!("[{}]", blocks.join(","))
}

fn letterhead_footer() -> String {
    let blocks = [
        block(
            "lh-f1",
            "paragraph",
            &text_data("{{bank_name}} | IBAN: {{bank_iban}} | BIC: {{bank_bic}}"),
            &m_default(),
        ),
        block(
            "lh-f2",
            "paragraph",
            &text_data("{{company_vat}} | {{company_email}} | {{company_phone}}"),
            &m_default(),
        ),
    ];
    format!("[{}]", blocks.join(","))
}

fn letterhead_content() -> String {
    format!(r#"{{"id":"tmpl-letterhead","blocks":[],"pageSetup":{},"header":{},"footer":{}}}"#,
        PAGE_SETUP, letterhead_header(), letterhead_footer())
}

// ── Invoice ─────────────────────────────────────────────────

fn invoice_content() -> String {
    let blocks = vec![
        block("inv-1", "h1", &text_data("Invoice {{doc_number}}"), &m_locked()),
        block("inv-2", "paragraph", &text_data("{{client_name}}\\n{{client_address}}"), &m_locked()),
        block("inv-3", "paragraph", &text_data("Date: {{doc_date}}"), &m_locked()),
        block("inv-4", "divider", r#"{}"#, &m_default()),
        block("inv-5", "table", &table_data(
            &["Description", "Quantity", "Unit Price", "Total"],
            &[vec!["{{line_description}}", "{{line_qty}}", "{{line_unit_price}}", "{{line_total}}"]],
        ), &m_default()),
        block("inv-6", "divider", r#"{}"#, &m_default()),
        block("inv-7", "paragraph", &text_data("Subtotal: {{subtotal}}"), &m_right_locked()),
        block("inv-8", "paragraph", &text_data("VAT ({{vat_rate}}%): {{vat_amount}}"), &m_right_locked()),
        block("inv-9", "paragraph", &text_data("**Total: {{total}}**"), &m_right_locked()),
        block("inv-10", "spacer", r#"{}"#, &m_default()),
        block("inv-11", "paragraph", &text_data("Payment terms: 30 days net"), &m_default()),
        block("inv-12", "paragraph", &text_data("Bank: {{bank_name}} | IBAN: {{bank_iban}}"), &m_default()),
    ];
    format!(
        r#"{{"id":"tmpl-invoice","blocks":[{}],"pageSetup":{},"header":[],"footer":[]}}"#,
        blocks.join(","),
        PAGE_SETUP
    )
}

// ── SOW ─────────────────────────────────────────────────────

fn sow_content() -> String {
    let blocks = vec![
        block("sow-1", "h1", &text_data("Statement of Work"), &m_locked()),
        block("sow-2", "paragraph", &text_data("No. {{doc_number}}"), &m_locked()),
        block("sow-3", "paragraph", &text_data("{{client_name}}"), &m_locked()),
        block("sow-4", "h2", &text_data("{{doc_title}}"), &m_default()),
        block("sow-5", "divider", r#"{}"#, &m_default()),
        block("sow-6", "table", &table_data(
            &["Field", "Value"],
            &[
                vec!["Author", "{{author}}"],
                vec!["Status", "{{status}}"],
                vec!["Date", "{{doc_date}}"],
                vec!["Version", "{{version}}"],
            ],
        ), &m_default()),
        block("sow-7", "paragraph",
            &text_data("\\u00a9 {{company_name}}. All rights reserved."), &m_locked()),
        block("sow-8", "h2", &text_data("Introduction"), &m_keep()),
        block("sow-9", "paragraph", &text_data(
            "THIS STATEMENT OF WORK (\\\"SOW\\\") is entered into by and between {{client_name}} (\\\"Client\\\") and {{company_name}} (\\\"Service Provider\\\")."
        ), &m_default()),
        block("sow-10", "h2", &text_data("Description of Services and Deliverables"), &m_keep()),
        block("sow-11", "paragraph", &text_data("[Describe services here]"), &m_default()),
        block("sow-12", "h2", &text_data("Term/Schedule"), &m_keep()),
        block("sow-13", "table", &table_data(
            &["Role", "Est. Start Date", "Est. Period of Performance"],
            &[vec!["{{role}}", "{{start_date}}", "{{period}}"]],
        ), &m_default()),
        block("sow-14", "h2", &text_data("Deliverables"), &m_keep_break()),
        block("sow-15", "table", &table_data(
            &["Package", "Topics", "Sub-Topics"],
            &[vec!["{{package}}", "{{topics}}", "{{sub_topics}}"]],
        ), &m_default()),
        block("sow-16", "h2", &text_data("Assumptions"), &m_keep()),
        block("sow-17", "paragraph", &text_data("[List assumptions here]"), &m_default()),
        block("sow-18", "h2", &text_data("Project Communication"), &m_keep()),
        block("sow-19", "paragraph", &text_data("[Describe communication plan here]"), &m_default()),
        block("sow-20", "h2", &text_data("Price and Payment Schedule"), &m_keep_break()),
        block("sow-21", "table", &table_data(
            &["Role", "Hours", "Rate", "Total"],
            &[
                vec!["{{role}}", "{{hours}}", "{{rate}}", "{{role_total}}"],
                vec!["Discount", "", "", "{{discount}}"],
                vec!["VAT", "", "", "{{vat}}"],
                vec!["Grand Total", "", "", "{{grand_total}}"],
            ],
        ), &m_default()),
        block("sow-22", "h2", &text_data("Invoice Schedule"), &m_keep()),
        block("sow-23", "paragraph", &text_data(
            "The Service Provider will invoice the Client on a monthly basis for actual hours expended."
        ), &m_default()),
        signature_block("sow-24"),
    ];
    format!(
        r#"{{"id":"tmpl-sow","blocks":[{}],"pageSetup":{},"header":[],"footer":[]}}"#,
        blocks.join(","),
        PAGE_SETUP
    )
}

// ── Quote ───────────────────────────────────────────────────

fn quote_content() -> String {
    let blocks = vec![
        block("qt-1", "h1", &text_data("Quote {{doc_number}}"), &m_locked()),
        block("qt-2", "paragraph", &text_data("{{client_name}}\\n{{client_address}}"), &m_locked()),
        block("qt-3", "paragraph",
            &text_data("Date: {{doc_date}} | Valid until: {{doc_valid_until}}"), &m_locked()),
        block("qt-4", "divider", r#"{}"#, &m_default()),
        block("qt-5", "h2", &text_data("Scope"), &m_keep()),
        block("qt-6", "paragraph", &text_data("[Describe scope here]"), &m_default()),
        block("qt-7", "divider", r#"{}"#, &m_default()),
        block("qt-8", "table", &table_data(
            &["Description", "Quantity", "Unit", "Unit Price", "Total"],
            &[vec!["{{line_description}}", "{{line_qty}}", "{{line_unit}}", "{{line_unit_price}}", "{{line_total}}"]],
        ), &m_default()),
        block("qt-9", "paragraph",
            &text_data("Subtotal: {{subtotal}}\\nVAT ({{vat_rate}}%): {{vat_amount}}\\n**Total: {{total}}**"),
            &m_right_locked()),
        block("qt-10", "paragraph", &text_data("Terms & Conditions apply."), &m_default()),
    ];
    format!(
        r#"{{"id":"tmpl-quote","blocks":[{}],"pageSetup":{},"header":[],"footer":[]}}"#,
        blocks.join(","),
        PAGE_SETUP
    )
}

// ── Contract ────────────────────────────────────────────────

fn contract_content() -> String {
    let blocks = vec![
        block("ct-1", "h1", &text_data("Contract {{doc_number}}"), &m_locked()),
        block("ct-2", "paragraph",
            &text_data("Between {{company_name}} and {{client_name}}"), &m_locked()),
        block("ct-3", "paragraph", &text_data("Date: {{doc_date}}"), &m_locked()),
        block("ct-4", "divider", r#"{}"#, &m_default()),
        block("ct-5", "h2", &text_data("1. Scope of Work"), &m_keep()),
        block("ct-6", "paragraph", &text_data("[Describe scope here]"), &m_default()),
        block("ct-7", "h2", &text_data("2. Terms and Conditions"), &m_keep()),
        block("ct-8", "paragraph", &text_data("[Describe terms here]"), &m_default()),
        block("ct-9", "h2", &text_data("3. Compensation"), &m_keep()),
        block("ct-10", "table", &table_data(
            &["Item", "Amount", "Currency", "Notes"],
            &[vec!["{{item}}", "{{amount}}", "{{currency}}", "{{notes}}"]],
        ), &m_default()),
        block("ct-11", "h2", &text_data("4. Duration"), &m_keep()),
        block("ct-12", "paragraph", &text_data("[Specify duration here]"), &m_default()),
        signature_block("ct-13"),
    ];
    format!(
        r#"{{"id":"tmpl-contract","blocks":[{}],"pageSetup":{},"header":[],"footer":[]}}"#,
        blocks.join(","),
        PAGE_SETUP
    )
}

// ── Public API ──────────────────────────────────────────────

/// Each tuple: (id, name, template_type, content_json, header_json,
/// footer_json, page_setup_json)
#[allow(clippy::type_complexity)]
pub fn default_templates() -> Vec<(
    &'static str,
    &'static str,
    &'static str,
    String,
    Option<String>,
    Option<String>,
    String,
)> {
    vec![
        (
            "tmpl-letterhead",
            "Default Letterhead",
            "letterhead",
            letterhead_content(),
            Some(letterhead_header()),
            Some(letterhead_footer()),
            PAGE_SETUP.to_string(),
        ),
        (
            "tmpl-invoice",
            "Invoice Template",
            "invoice",
            invoice_content(),
            None,
            None,
            PAGE_SETUP.to_string(),
        ),
        (
            "tmpl-sow",
            "Statement of Work",
            "sow",
            sow_content(),
            None,
            None,
            PAGE_SETUP.to_string(),
        ),
        (
            "tmpl-quote",
            "Quote Template",
            "quote",
            quote_content(),
            None,
            None,
            PAGE_SETUP.to_string(),
        ),
        (
            "tmpl-contract",
            "Contract Template",
            "contract",
            contract_content(),
            None,
            None,
            PAGE_SETUP.to_string(),
        ),
    ]
}
