use axum::extract::{Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Datelike;
use konto_common::error::AppError;
use konto_core::services::cash_flow_service::CashFlowService;
use konto_core::services::dashboard_charts_service::DashboardChartsService;
use konto_core::services::export_service::ExportService;

use crate::dto::dashboard_charts::*;
use crate::dto::report::DateRangeParams;
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/dashboard/monthly-revenue",
    params(MonthsParams),
    responses((status = 200, body = Vec<MonthlyAmountResponse>))
)]
pub async fn monthly_revenue(
    State(state): State<AppState>,
    Query(params): Query<MonthsParams>,
) -> Result<Json<Vec<MonthlyAmountResponse>>, AppError> {
    let months = params.months.unwrap_or(12);
    let data = DashboardChartsService::monthly_revenue(&state.db, months).await?;
    Ok(Json(
        data.into_iter()
            .map(|m| MonthlyAmountResponse {
                month: m.month,
                amount: m.amount,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/dashboard/monthly-expenses",
    params(MonthsParams),
    responses((status = 200, body = Vec<MonthlyAmountResponse>))
)]
pub async fn monthly_expenses(
    State(state): State<AppState>,
    Query(params): Query<MonthsParams>,
) -> Result<Json<Vec<MonthlyAmountResponse>>, AppError> {
    let months = params.months.unwrap_or(12);
    let data = DashboardChartsService::monthly_expenses(&state.db, months).await?;
    Ok(Json(
        data.into_iter()
            .map(|m| MonthlyAmountResponse {
                month: m.month,
                amount: m.amount,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/dashboard/invoice-aging",
    responses((status = 200, body = Vec<InvoiceAgingBucketResponse>))
)]
pub async fn invoice_aging(
    State(state): State<AppState>,
) -> Result<Json<Vec<InvoiceAgingBucketResponse>>, AppError> {
    let data = DashboardChartsService::invoice_aging(&state.db).await?;
    Ok(Json(
        data.into_iter()
            .map(|b| InvoiceAgingBucketResponse {
                status: b.status,
                count: b.count,
                total: b.total,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/dashboard/top-outstanding",
    params(LimitParams),
    responses((status = 200, body = Vec<OutstandingContactResponse>))
)]
pub async fn top_outstanding(
    State(state): State<AppState>,
    Query(params): Query<LimitParams>,
) -> Result<Json<Vec<OutstandingContactResponse>>, AppError> {
    let limit = params.limit.unwrap_or(5);
    let data =
        DashboardChartsService::top_outstanding_contacts(&state.db, limit).await?;
    Ok(Json(
        data.into_iter()
            .map(|c| OutstandingContactResponse {
                contact_id: c.contact_id,
                contact_name: c.contact_name,
                outstanding_amount: c.outstanding_amount,
                invoice_count: c.invoice_count,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/dashboard/overview",
    params(OverviewParams),
    responses((status = 200, body = OverviewResponse))
)]
pub async fn dashboard_overview(
    State(state): State<AppState>,
    Query(params): Query<OverviewParams>,
) -> Result<Json<OverviewResponse>, AppError> {
    let year = params
        .year
        .unwrap_or_else(|| chrono::Utc::now().naive_utc().date().year());
    let data = DashboardChartsService::overview(&state.db, year).await?;
    Ok(Json(OverviewResponse {
        year: data.year,
        months: data
            .months
            .into_iter()
            .map(|m| OverviewMonthResponse {
                month: m.month,
                income: m.income,
                expenses: m.expenses,
                cumulative_income: m.cumulative_income,
                cumulative_expenses: m.cumulative_expenses,
            })
            .collect(),
        total_income: data.total_income,
        total_expenses: data.total_expenses,
        difference: data.difference,
        available_years: data.available_years,
    }))
}

#[utoipa::path(
    get, path = "/api/v1/reports/cash-flow",
    params(DateRangeParams),
    responses((status = 200, body = CashFlowReportResponse))
)]
pub async fn cash_flow(
    State(state): State<AppState>,
    Query(params): Query<DateRangeParams>,
) -> Result<impl IntoResponse, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&params.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".to_string()))?;
    let to = chrono::NaiveDate::parse_from_str(&params.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".to_string()))?;

    let report = CashFlowService::cash_flow_report(&state.db, from, to).await?;

    if params.format.as_deref() == Some("csv") {
        let csv_rows: Vec<CsvCashFlowRow> = report
            .sections
            .iter()
            .flat_map(|s| {
                s.items.iter().map(|item| CsvCashFlowRow {
                    section: s.label.clone(),
                    description: item.description.clone(),
                    amount: item.amount,
                })
            })
            .collect();
        let csv_bytes = ExportService::to_csv(&csv_rows)?;
        return Ok(csv_response(csv_bytes, "cash-flow.csv").into_response());
    }

    let resp = CashFlowReportResponse {
        sections: report
            .sections
            .into_iter()
            .map(|s| CashFlowSectionResponse {
                label: s.label,
                inflows: s.inflows,
                outflows: s.outflows,
                net: s.net,
                items: s
                    .items
                    .into_iter()
                    .map(|i| CashFlowItemResponse {
                        description: i.description,
                        amount: i.amount,
                    })
                    .collect(),
            })
            .collect(),
        net_change: report.net_change,
        opening_balance: report.opening_balance,
        closing_balance: report.closing_balance,
        reconciliation_difference: report.reconciliation_difference,
        from_date: report.from_date,
        to_date: report.to_date,
    };

    Ok(Json(resp).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/reports/ar-aging",
    responses((status = 200, body = Vec<AgingBucketResponse>))
)]
pub async fn ar_aging(
    State(state): State<AppState>,
) -> Result<Json<Vec<AgingBucketResponse>>, AppError> {
    let data = CashFlowService::ar_aging_report(&state.db).await?;
    Ok(Json(
        data.into_iter()
            .map(|b| AgingBucketResponse {
                bucket: b.bucket,
                count: b.count,
                total: b.total,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/reports/ap-aging",
    responses((status = 200, body = Vec<AgingBucketResponse>))
)]
pub async fn ap_aging(
    State(state): State<AppState>,
) -> Result<Json<Vec<AgingBucketResponse>>, AppError> {
    let data = CashFlowService::ap_aging_report(&state.db).await?;
    Ok(Json(
        data.into_iter()
            .map(|b| AgingBucketResponse {
                bucket: b.bucket,
                count: b.count,
                total: b.total,
            })
            .collect(),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/reports/cash-flow/monthly",
    params(DateRangeParams),
    responses((status = 200, body = CashFlowMonthlyReportResponse))
)]
pub async fn cash_flow_monthly(
    State(state): State<AppState>,
    Query(params): Query<DateRangeParams>,
) -> Result<Json<CashFlowMonthlyReportResponse>, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&params.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".to_string()))?;
    let to = chrono::NaiveDate::parse_from_str(&params.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".to_string()))?;

    let report = CashFlowService::monthly_cash_flow(&state.db, from, to).await?;

    Ok(Json(CashFlowMonthlyReportResponse {
        months: report
            .months
            .into_iter()
            .map(|m| MonthlyCashFlowResponse {
                month: m.month,
                inflows: m.inflows,
                outflows: m.outflows,
                net: m.net,
                cumulative_balance: m.cumulative_balance,
            })
            .collect(),
        initial_balance: report.initial_balance,
        ending_balance: report.ending_balance,
        total_inflows: report.total_inflows,
        total_outflows: report.total_outflows,
        net_variation: report.net_variation,
    }))
}

#[derive(serde::Serialize)]
struct CsvCashFlowRow {
    section: String,
    description: String,
    #[serde(serialize_with = "rust_decimal::serde::str::serialize")]
    amount: rust_decimal::Decimal,
}

fn csv_response(
    bytes: Vec<u8>,
    filename: &str,
) -> ([(header::HeaderName, String); 2], Vec<u8>) {
    (
        [
            (header::CONTENT_TYPE, "text/csv".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        bytes,
    )
}
