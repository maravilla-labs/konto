use chrono::Utc;
use konto_common::error::AppError;
use konto_db::entities::annual_report_note;
use konto_db::repository::annual_report_note_repo::AnnualReportNoteRepo;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct AnnualReportNoteService;

impl AnnualReportNoteService {
    pub async fn get_all_for_year(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<Vec<annual_report_note::Model>, AppError> {
        AnnualReportNoteRepo::find_by_fiscal_year(db, fiscal_year_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_section(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        section_key: &str,
    ) -> Result<Option<annual_report_note::Model>, AppError> {
        AnnualReportNoteRepo::find_by_key(db, fiscal_year_id, section_key)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Insert or update a note section for a fiscal year.
    pub async fn upsert_section(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        section_key: &str,
        content_json: &str,
        label: Option<&str>,
        sort_order: Option<i32>,
    ) -> Result<annual_report_note::Model, AppError> {
        let now = Utc::now().naive_utc();
        let existing =
            AnnualReportNoteRepo::find_by_key(db, fiscal_year_id, section_key)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;

        match existing {
            Some(note) => {
                let mut model: annual_report_note::ActiveModel = note.into();
                model.content_json = Set(content_json.to_string());
                if let Some(l) = label {
                    model.label = Set(l.to_string());
                }
                if let Some(o) = sort_order {
                    model.sort_order = Set(o);
                }
                model.updated_at = Set(now);
                AnnualReportNoteRepo::update(db, model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))
            }
            None => {
                let max_order = AnnualReportNoteRepo::find_max_sort_order(
                    db,
                    fiscal_year_id,
                )
                .await
                .unwrap_or(0);
                let model = annual_report_note::ActiveModel {
                    id: Set(Uuid::new_v4().to_string()),
                    fiscal_year_id: Set(fiscal_year_id.to_string()),
                    section_key: Set(section_key.to_string()),
                    content_json: Set(content_json.to_string()),
                    sort_order: Set(sort_order.unwrap_or(max_order + 1)),
                    label: Set(label.unwrap_or("").to_string()),
                    section_type: Set("text".to_string()),
                    created_at: Set(now),
                    updated_at: Set(now),
                };
                AnnualReportNoteRepo::upsert(db, model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))
            }
        }
    }

    /// Create a custom note section.
    pub async fn create_custom(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        label: &str,
        sort_order: Option<i32>,
    ) -> Result<annual_report_note::Model, AppError> {
        let now = Utc::now().naive_utc();
        let key = format!("custom_{}", Uuid::new_v4());
        let max_order = AnnualReportNoteRepo::find_max_sort_order(
            db,
            fiscal_year_id,
        )
        .await
        .unwrap_or(0);

        let model = annual_report_note::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            fiscal_year_id: Set(fiscal_year_id.to_string()),
            section_key: Set(key),
            content_json: Set(serde_json::json!({"text": ""}).to_string()),
            sort_order: Set(sort_order.unwrap_or(max_order + 1)),
            label: Set(label.to_string()),
            section_type: Set("text".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        AnnualReportNoteRepo::upsert(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Delete a custom section (only sections starting with "custom_").
    pub async fn delete_section(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
        section_key: &str,
    ) -> Result<(), AppError> {
        if !section_key.starts_with("custom_") {
            return Err(AppError::Validation(
                "Only custom sections can be deleted".to_string(),
            ));
        }
        AnnualReportNoteRepo::delete(db, fiscal_year_id, section_key)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    /// Seed default notes for all 8 sections if not already present.
    pub async fn seed_defaults(
        db: &DatabaseConnection,
        fiscal_year_id: &str,
    ) -> Result<(), AppError> {
        let defaults = default_sections();
        for (key, content, sort_order, label, section_type) in defaults {
            let existing =
                AnnualReportNoteRepo::find_by_key(db, fiscal_year_id, &key)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            if existing.is_none() {
                let now = Utc::now().naive_utc();
                let model = annual_report_note::ActiveModel {
                    id: Set(Uuid::new_v4().to_string()),
                    fiscal_year_id: Set(fiscal_year_id.to_string()),
                    section_key: Set(key),
                    content_json: Set(content),
                    sort_order: Set(sort_order),
                    label: Set(label),
                    section_type: Set(section_type),
                    created_at: Set(now),
                    updated_at: Set(now),
                };
                AnnualReportNoteRepo::upsert(db, model)
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;
            }
        }
        Ok(())
    }
}

fn default_sections() -> Vec<(String, String, i32, String, String)> {
    vec![
        ("accounting_principles".into(), serde_json::json!({
            "text": "Die vorliegende Jahresrechnung wurde gemäss den Bestimmungen des Schweizerischen Obligationenrechts (OR), insbesondere der Art. 957 bis 962, erstellt. Die angewandten Bewertungsgrundsätze entsprechen den gesetzlichen Vorschriften."
        }).to_string(), 1, "Grundsätze der Rechnungslegung".into(), "text".into()),
        ("general_info".into(), serde_json::json!({
            "text": "Auto-populated from company settings and shareholders."
        }).to_string(), 2, "Angaben zur Gesellschaft".into(), "auto_company_info".into()),
        ("audit_optout".into(), serde_json::json!({
            "text": "Die Gesellschaft hat auf eine eingeschränkte Revision (Opting-out) im Sinne von Art. 727a Abs. 2 OR verzichtet."
        }).to_string(), 3, "Verzicht auf Revision (Opting-out)".into(), "text".into()),
        ("employees".into(), serde_json::json!({
            "entries": []
        }).to_string(), 4, "Anzahl Arbeitsstellen im Jahresdurchschnitt".into(), "employees".into()),
        ("guarantees".into(), serde_json::json!({
            "text": "Es bestehen keine Eventualverpflichtungen."
        }).to_string(), 5, "Eventualverpflichtungen".into(), "text".into()),
        ("fx_rates".into(), serde_json::json!({
            "rates": [],
            "override": false
        }).to_string(), 6, "Verwendete Fremdwährungskurse".into(), "auto_fx_rates".into()),
        ("extraordinary".into(), serde_json::json!({
            "explanation": ""
        }).to_string(), 7, "Ausserordentliche Positionen".into(), "text".into()),
        ("post_balance_events".into(), serde_json::json!({
            "text": "Zwischen dem Bilanzstichtag und der Genehmigung der Jahresrechnung sind keine wesentlichen Ereignisse eingetreten, die eine Anpassung der Jahresrechnung erfordern würden."
        }).to_string(), 8, "Ereignisse nach dem Bilanzstichtag".into(), "text".into()),
    ]
}
