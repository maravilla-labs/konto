use axum::extract::State;
use axum::Json;
use konto_common::error::AppError;
use konto_core::services::setup_service::{SetupInput, SetupService};

use crate::dto::setup::{
    BrandingResponse, SetupCompleteRequest, SetupCompleteResponse, SetupStatusResponse,
};
use crate::state::AppState;

#[utoipa::path(
    get,
    path = "/api/v1/setup/status",
    responses((status = 200, body = SetupStatusResponse)),
    tag = "setup"
)]
pub async fn setup_status(
    State(state): State<AppState>,
) -> Result<Json<SetupStatusResponse>, AppError> {
    let needed = SetupService::is_setup_needed(&state.db).await?;
    Ok(Json(SetupStatusResponse {
        setup_needed: needed,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/setup/complete",
    request_body = SetupCompleteRequest,
    responses(
        (status = 200, body = SetupCompleteResponse),
        (status = 409, description = "Setup already completed")
    ),
    tag = "setup"
)]
pub async fn setup_complete(
    State(state): State<AppState>,
    Json(body): Json<SetupCompleteRequest>,
) -> Result<Json<SetupCompleteResponse>, AppError> {
    state.setup_limiter.check("setup").await?;

    let input = SetupInput {
        admin_email: body.admin_email,
        admin_password: body.admin_password,
        admin_full_name: body.admin_full_name,
        admin_language: body.admin_language,
        legal_name: body.legal_name,
        trade_name: body.trade_name,
        street: body.street,
        postal_code: body.postal_code,
        city: body.city,
        country: body.country,
        legal_entity_type: body.legal_entity_type,
        default_currency: body.default_currency,
        vat_method: body.vat_method,
        flat_rate_percentage: body.flat_rate_percentage,
        date_format: body.date_format,
        fiscal_year_start_month: body.fiscal_year_start_month,
    };

    let result = SetupService::complete_setup(&state.db, &state.jwt, input).await?;

    Ok(Json(SetupCompleteResponse {
        access_token: result.access_token,
        refresh_token: result.refresh_token,
        token_type: "Bearer".to_string(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/setup/branding",
    responses((status = 200, body = BrandingResponse)),
    tag = "setup"
)]
pub async fn get_branding(
    State(state): State<AppState>,
) -> Result<Json<BrandingResponse>, AppError> {
    let branding = SetupService::get_branding(&state.db).await?;

    match branding {
        Some((legal_name, trade_name, logo_url)) => Ok(Json(BrandingResponse {
            legal_name: Some(legal_name),
            trade_name,
            logo_url,
        })),
        None => Ok(Json(BrandingResponse {
            legal_name: None,
            trade_name: None,
            logo_url: None,
        })),
    }
}
