use konto_common::error::AppError;
use konto_db::entities::contact;
use konto_db::repository::vat_rate_repo::VatRateRepo;
use sea_orm::DatabaseConnection;

const EU_COUNTRIES: &[&str] = &[
    "AT", "BE", "BG", "HR", "CY", "CZ", "DK", "EE", "FI", "FR",
    "DE", "GR", "HU", "IE", "IT", "LV", "LT", "LU", "MT", "NL",
    "PL", "PT", "RO", "SK", "SI", "ES", "SE",
];

pub struct VatResolutionService;

impl VatResolutionService {
    /// Resolve the effective VAT mode for a contact.
    /// If vat_mode is not "auto", return it directly.
    /// Otherwise auto-detect from country: CH → normal, EU → reverse_charge, other → export_exempt.
    pub fn resolve_vat_mode(contact: &contact::Model) -> String {
        if contact.vat_mode != "auto" {
            return contact.vat_mode.clone();
        }

        let country = contact
            .country
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_uppercase();

        if country == "CH" || country.is_empty() {
            "normal".to_string()
        } else if EU_COUNTRIES.contains(&country.as_str()) {
            "reverse_charge".to_string()
        } else {
            "export_exempt".to_string()
        }
    }

    /// Return the default VAT rate ID for a resolved mode.
    /// - "reverse_charge" → vat-rc0
    /// - "export_exempt" → vat-ex0
    /// - "normal" → None (use standard rates)
    pub async fn default_vat_rate_for_mode(
        db: &DatabaseConnection,
        mode: &str,
    ) -> Result<Option<String>, AppError> {
        let rate_id = match mode {
            "reverse_charge" => Some("vat-rc0"),
            "export_exempt" => Some("vat-ex0"),
            _ => None,
        };

        if let Some(id) = rate_id {
            // Verify the rate exists
            let exists = VatRateRepo::find_by_id(db, id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
            if exists.is_some() {
                Ok(Some(id.to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
