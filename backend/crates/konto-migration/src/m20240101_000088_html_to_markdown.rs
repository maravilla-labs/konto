use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

/// Tables and columns containing HTML content from legacy imports that need conversion to Markdown.
const TARGETS: &[(&str, &str)] = &[
    ("time_entries", "description"),
    ("projects", "description"),
    ("project_items", "description"),
    ("project_milestones", "description"),
    ("contacts", "notes"),
    ("invoice_lines", "description"),
    ("credit_note_lines", "description"),
    ("document_line_items", "description"),
    ("invoices", "notes"),
    ("invoices", "header_text"),
    ("invoices", "footer_text"),
    ("credit_notes", "notes"),
    ("employees", "notes"),
];

/// Convert basic HTML to Markdown inline (no external crate needed in migration).
/// Handles common HTML patterns: <br />, <ul><li>, <strong>, <em>, <p>.
fn html_to_md(html: &str) -> String {
    let mut s = html.to_string();

    // Normalize self-closing tags
    s = s.replace("<br />", "\n").replace("<br/>", "\n").replace("<br>", "\n");

    // Block-level elements
    s = s.replace("</p>", "\n\n").replace("<p>", "");

    // Bold / italic
    s = s.replace("<strong>", "**").replace("</strong>", "**");
    s = s.replace("<b>", "**").replace("</b>", "**");
    s = s.replace("<em>", "*").replace("</em>", "*");
    s = s.replace("<i>", "*").replace("</i>", "*");

    // Lists: convert <ul>/<ol> with <li>
    s = s.replace("<ul>", "\n").replace("</ul>", "\n");
    s = s.replace("<ol>", "\n").replace("</ol>", "\n");
    s = s.replace("<li>", "- ").replace("</li>", "\n");

    // Strip any remaining HTML tags
    let mut result = String::with_capacity(s.len());
    let mut in_tag = false;
    for ch in s.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => { in_tag = false; }
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    // Clean up excessive newlines (3+ -> 2)
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    result.trim().to_string()
}

/// Check if a string contains HTML tags
fn looks_like_html(s: &str) -> bool {
    s.contains("<br") || s.contains("<p>") || s.contains("<ul") || s.contains("<ol")
        || s.contains("<li") || s.contains("<strong") || s.contains("<em")
        || s.contains("<b>") || s.contains("<i>") || s.contains("<div")
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        for (table, column) in TARGETS {
            // Select rows where the column contains HTML-like content
            let rows = db.query_all(
                sea_orm_migration::sea_orm::Statement::from_string(
                    sea_orm_migration::sea_orm::DatabaseBackend::Sqlite,
                    format!(
                        "SELECT id, \"{col}\" FROM \"{tbl}\" WHERE \"{col}\" IS NOT NULL AND \"{col}\" != ''",
                        tbl = table, col = column
                    ),
                )
            ).await?;

            for row in &rows {
                let id: String = row.try_get("", "id")?;
                let value: String = row.try_get("", column)?;

                if !looks_like_html(&value) {
                    continue;
                }

                let md = html_to_md(&value);
                if md == value {
                    continue;
                }

                db.execute(
                    sea_orm_migration::sea_orm::Statement::from_string(
                        sea_orm_migration::sea_orm::DatabaseBackend::Sqlite,
                        format!(
                            "UPDATE \"{tbl}\" SET \"{col}\" = '{val}' WHERE id = '{id}'",
                            tbl = table,
                            col = column,
                            val = md.replace('\'', "''"),
                            id = id.replace('\'', "''"),
                        ),
                    )
                ).await?;
            }
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No rollback — converted data stays as Markdown
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_md_br() {
        assert_eq!(html_to_md("line1<br />line2"), "line1\nline2");
        assert_eq!(html_to_md("- vision brainstorming<br />- system setup locally<br /><br />"), "- vision brainstorming\n- system setup locally");
    }

    #[test]
    fn test_html_to_md_list() {
        let html = "<ul>\n<li>meeting with gregor</li>\n<li>analysing code</li>\n</ul>";
        let md = html_to_md(html);
        assert!(md.contains("- meeting with gregor"));
        assert!(md.contains("- analysing code"));
    }

    #[test]
    fn test_html_to_md_mixed() {
        let html = "Activities:<br />\n<ul>\n<li>Attending full day team meetings </li>\n<li>creating notes</li>\n</ul>";
        let md = html_to_md(html);
        assert!(md.starts_with("Activities:"));
        assert!(md.contains("- Attending full day team meetings"));
    }

    #[test]
    fn test_plain_text_passthrough() {
        assert_eq!(html_to_md("just plain text"), "just plain text");
        assert!(!looks_like_html("just plain text"));
    }

    #[test]
    fn test_bold_italic() {
        assert_eq!(html_to_md("<strong>bold</strong> and <em>italic</em>"), "**bold** and *italic*");
    }
}
