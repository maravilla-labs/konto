use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DocumentResponse {
    pub id: String,
    pub doc_type: String,
    pub doc_number: Option<String>,
    pub title: String,
    pub status: String,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub template_id: Option<String>,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    #[schema(value_type = String)]
    pub subtotal: Decimal,
    #[schema(value_type = String)]
    pub vat_rate: Decimal,
    #[schema(value_type = String)]
    pub vat_amount: Decimal,
    #[schema(value_type = String)]
    pub total: Decimal,
    pub valid_until: Option<String>,
    pub issued_at: Option<String>,
    pub signed_at: Option<String>,
    pub converted_from: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DocumentLineItemResponse {
    pub id: String,
    pub document_id: String,
    pub position: i32,
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    pub unit: Option<String>,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    #[schema(value_type = String)]
    pub discount_pct: Decimal,
    #[schema(value_type = String)]
    pub total: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DocumentDetailResponse {
    #[serde(flatten)]
    pub document: DocumentResponse,
    pub content_json: String,
    pub lines: Vec<DocumentLineItemResponse>,
    pub contact_name: Option<String>,
    pub project_name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDocumentRequest {
    pub doc_type: String,
    pub title: String,
    pub contact_id: String,
    pub project_id: Option<String>,
    pub template_id: Option<String>,
    pub content_json: String,
    pub language: Option<String>,
    pub currency_id: Option<String>,
    pub valid_until: Option<String>,
    pub lines: Vec<CreateDocumentLineRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDocumentLineRequest {
    pub description: String,
    #[schema(value_type = String)]
    pub quantity: Decimal,
    pub unit: Option<String>,
    #[schema(value_type = String)]
    pub unit_price: Decimal,
    #[schema(value_type = String, default = json!("0"))]
    #[serde(default)]
    pub discount_pct: Decimal,
}

pub type UpdateDocumentRequest = CreateDocumentRequest;

#[derive(Debug, Deserialize, IntoParams)]
pub struct DocumentListParams {
    #[param(default = 1, minimum = 1)]
    pub page: Option<u64>,
    #[param(default = 50, minimum = 1, maximum = 200)]
    pub per_page: Option<u64>,
    pub doc_type: Option<String>,
    pub status: Option<String>,
    pub contact_id: Option<String>,
    pub search: Option<String>,
}

impl DocumentListParams {
    pub fn page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }
    pub fn per_page(&self) -> u64 {
        self.per_page.unwrap_or(50).clamp(1, 200)
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ConvertDocumentRequest {
    pub target_type: String,
}

impl From<konto_db::entities::document::Model> for DocumentResponse {
    fn from(m: konto_db::entities::document::Model) -> Self {
        Self {
            id: m.id,
            doc_type: m.doc_type,
            doc_number: m.doc_number,
            title: m.title,
            status: m.status,
            contact_id: m.contact_id,
            project_id: m.project_id,
            template_id: m.template_id,
            language: m.language,
            currency_id: m.currency_id,
            subtotal: m.subtotal,
            vat_rate: m.vat_rate,
            vat_amount: m.vat_amount,
            total: m.total,
            valid_until: m.valid_until.map(|d| d.to_string()),
            issued_at: m.issued_at.map(|d| d.to_string()),
            signed_at: m.signed_at.map(|d| d.to_string()),
            converted_from: m.converted_from,
            created_by: m.created_by,
        }
    }
}

impl From<konto_db::entities::document_line_item::Model> for DocumentLineItemResponse {
    fn from(m: konto_db::entities::document_line_item::Model) -> Self {
        Self {
            id: m.id,
            document_id: m.document_id,
            position: m.position,
            description: m.description,
            quantity: m.quantity,
            unit: m.unit,
            unit_price: m.unit_price,
            discount_pct: m.discount_pct,
            total: m.total,
        }
    }
}
