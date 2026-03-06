use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::dashboard_service::DashboardService;
use konto_core::services::export_service::ExportService;
use konto_core::services::journal_service::{JournalLineInput, JournalService};
use konto_core::services::report_service::ReportService;
use konto_core::services::vat_xml_service::{VatDeclarationInput, VatXmlService};
use konto_db::repository::account_repo::AccountRepo;
use konto_db::repository::bank_account_repo::BankAccountRepo;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::dto::report::*;
use crate::state::AppState;

#[utoipa::path(
    get, path = "/api/v1/reports/trial-balance",
    params(AsOfDateParams),
    responses((status = 200, body = Vec<TrialBalanceRowResponse>))
)]
pub async fn trial_balance(
    State(state): State<AppState>,
    Query(params): Query<AsOfDateParams>,
) -> Result<impl IntoResponse, AppError> {
    let as_of = params
        .as_of
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid as_of date".to_string()))
        })
        .transpose()?;

    let rows = ReportService::trial_balance(&state.db, as_of).await?;

    let data: Vec<TrialBalanceRowResponse> = rows
        .into_iter()
        .map(|r| TrialBalanceRowResponse {
            account_id: r.account_id,
            account_number: r.account_number,
            account_name: r.account_name,
            account_type: r.account_type,
            total_debit: r.total_debit,
            total_credit: r.total_credit,
            balance: r.balance,
        })
        .collect();

    if params.format.as_deref() == Some("csv") {
        let csv_bytes = ExportService::to_csv(&data)?;
        return Ok(csv_response(csv_bytes, "trial-balance.csv").into_response());
    }

    Ok(Json(data).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/reports/balance-sheet",
    params(AsOfDateParams),
    responses((status = 200, body = BalanceSheetResponse))
)]
pub async fn balance_sheet(
    State(state): State<AppState>,
    Query(params): Query<AsOfDateParams>,
) -> Result<impl IntoResponse, AppError> {
    let as_of = params
        .as_of
        .as_deref()
        .map(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|_| AppError::Validation("Invalid as_of date".to_string()))?
        .unwrap_or_else(|| chrono::Utc::now().naive_utc().date());

    let bs = ReportService::balance_sheet(&state.db, as_of).await?;

    let assets: Vec<TrialBalanceRowResponse> = bs.assets.into_iter().map(Into::into).collect();
    let liabilities: Vec<TrialBalanceRowResponse> = bs.liabilities.into_iter().map(Into::into).collect();
    let equity: Vec<TrialBalanceRowResponse> = bs.equity.into_iter().map(Into::into).collect();

    if params.format.as_deref() == Some("csv") {
        let mut all_rows: Vec<TrialBalanceRowResponse> = Vec::new();
        all_rows.extend(assets);
        all_rows.extend(liabilities);
        all_rows.extend(equity);
        let csv_bytes = ExportService::to_csv(&all_rows)?;
        return Ok(csv_response(csv_bytes, "balance-sheet.csv").into_response());
    }

    Ok(Json(BalanceSheetResponse {
        as_of: bs.as_of,
        assets,
        liabilities,
        equity,
        total_assets: bs.total_assets,
        total_liabilities_equity: bs.total_liabilities_equity,
    }).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/reports/profit-loss",
    params(DateRangeParams),
    responses((status = 200, body = ProfitLossResponse))
)]
pub async fn profit_loss(
    State(state): State<AppState>,
    Query(params): Query<DateRangeParams>,
) -> Result<impl IntoResponse, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&params.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".to_string()))?;
    let to = chrono::NaiveDate::parse_from_str(&params.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".to_string()))?;

    let pl = ReportService::profit_loss(&state.db, from, to).await?;

    let revenue: Vec<TrialBalanceRowResponse> = pl.revenue.into_iter().map(Into::into).collect();
    let expenses: Vec<TrialBalanceRowResponse> = pl.expenses.into_iter().map(Into::into).collect();

    if params.format.as_deref() == Some("csv") {
        let mut all_rows: Vec<TrialBalanceRowResponse> = Vec::new();
        all_rows.extend(revenue);
        all_rows.extend(expenses);
        let csv_bytes = ExportService::to_csv(&all_rows)?;
        return Ok(csv_response(csv_bytes, "profit-loss.csv").into_response());
    }

    Ok(Json(ProfitLossResponse {
        from_date: pl.from_date,
        to_date: pl.to_date,
        revenue,
        expenses,
        total_revenue: pl.total_revenue,
        total_expenses: pl.total_expenses,
        net_income: pl.net_income,
    }).into_response())
}

#[utoipa::path(
    get, path = "/api/v1/reports/ledger/{account_id}",
    params(LedgerParams),
    responses((status = 200, body = Vec<LedgerLineResponse>))
)]
pub async fn account_ledger(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    Query(params): Query<LedgerParams>,
) -> Result<Json<Vec<LedgerLineResponse>>, AppError> {
    let from = params
        .from_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid from_date".to_string()))
        })
        .transpose()?;
    let to = params
        .to_date
        .as_deref()
        .map(|s| {
            chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map_err(|_| AppError::Validation("Invalid to_date".to_string()))
        })
        .transpose()?;

    let lines =
        ReportService::account_ledger(&state.db, &account_id, from, to).await?;

    let data = lines
        .into_iter()
        .map(|l| LedgerLineResponse {
            date: l.date,
            entry_id: l.entry_id,
            description: l.description,
            debit: l.debit,
            credit: l.credit,
            running_balance: l.running_balance,
        })
        .collect();

    Ok(Json(data))
}

#[utoipa::path(
    get, path = "/api/v1/reports/vat",
    params(DateRangeParams),
    responses((status = 200, body = VatReportResponse))
)]
pub async fn vat_report(
    State(state): State<AppState>,
    Query(params): Query<DateRangeParams>,
) -> Result<impl IntoResponse, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&params.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".to_string()))?;
    let to = chrono::NaiveDate::parse_from_str(&params.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".to_string()))?;

    let report = ReportService::vat_report(&state.db, from, to).await?;

    let map_entry = |e: konto_core::services::report_types::VatReportEntry| VatReportEntryResponse {
        vat_code: e.vat_code,
        vat_name: e.vat_name,
        rate: e.rate,
        vat_type: e.vat_type,
        taxable_amount: e.taxable_amount,
        vat_amount: e.vat_amount,
    };

    if params.format.as_deref() == Some("csv") {
        // For CSV, combine all entries
        let all: Vec<VatReportEntryResponse> = report
            .output_entries
            .into_iter()
            .chain(report.input_entries)
            .map(map_entry)
            .collect();
        let csv_bytes = ExportService::to_csv(&all)?;
        return Ok(csv_response(csv_bytes, "vat-report.csv").into_response());
    }

    let output_entries: Vec<VatReportEntryResponse> =
        report.output_entries.into_iter().map(map_entry).collect();
    let input_entries: Vec<VatReportEntryResponse> =
        report.input_entries.into_iter().map(map_entry).collect();

    Ok(Json(VatReportResponse {
        vat_method: report.vat_method,
        output_entries,
        input_entries,
        total_output_taxable: report.total_output_taxable,
        total_output_vat: report.total_output_vat,
        total_input_taxable: report.total_input_taxable,
        total_input_vat: report.total_input_vat,
        net_vat_owed: report.net_vat_owed,
        from_date: params.from_date.clone(),
        to_date: params.to_date.clone(),
        flat_rate_percentage: report.flat_rate_percentage,
        gross_revenue: report.gross_revenue,
        flat_rate_vat_owed: report.flat_rate_vat_owed,
        collected_vat: report.collected_vat,
        saldo_ertrag: report.saldo_ertrag,
    })
    .into_response())
}

#[utoipa::path(
    post, path = "/api/v1/reports/vat/payment",
    request_body = CreateVatPaymentRequest,
    responses((status = 200, body = VatPaymentResponse)),
    security(("bearer" = []))
)]
pub async fn create_vat_payment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateVatPaymentRequest>,
) -> Result<Json<VatPaymentResponse>, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&body.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".to_string()))?;
    let to = chrono::NaiveDate::parse_from_str(&body.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".to_string()))?;
    let pay_date = chrono::NaiveDate::parse_from_str(&body.payment_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid payment_date".to_string()))?;

    // Generate the VAT report for the period
    let report = ReportService::vat_report(&state.db, from, to).await?;

    if report.vat_method != "flat_rate" {
        return Err(AppError::Validation(
            "VAT payment workflow is only available for flat-rate (Saldosteuersatz) method".to_string(),
        ));
    }

    let flat_rate_vat = report
        .flat_rate_vat_owed
        .ok_or_else(|| AppError::Validation("No flat rate configured".to_string()))?;
    let collected = report.collected_vat.unwrap_or(Decimal::ZERO);
    let saldo_ertrag = report.saldo_ertrag.unwrap_or(Decimal::ZERO);

    if flat_rate_vat == Decimal::ZERO {
        return Err(AppError::Validation("No VAT amount to pay".to_string()));
    }

    // Look up required accounts
    // 2200 = Geschuldete MWST
    let acct_2200 = AccountRepo::find_by_number(&state.db, 2200)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::Validation("Account 2200 (Geschuldete MWST) not found".to_string()))?;

    // 3809 = MWST Saldosteuersatz (Ertrag)
    let acct_3809 = AccountRepo::find_by_number(&state.db, 3809)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::Validation("Account 3809 (MWST Saldosteuersatz) not found".to_string()))?;

    // Bank account → linked chart-of-accounts account
    let bank = BankAccountRepo::find_by_id(&state.db, &body.bank_account_id)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::Validation("Bank account not found".to_string()))?;

    let bank_acct_id = bank
        .account_id
        .ok_or_else(|| AppError::Validation("Bank account has no linked chart-of-accounts account".to_string()))?;

    // Build journal lines:
    // 1. Debit 2200 (clear MWST liability) = collected_vat
    // 2. Credit 3809 (Saldosteuerertrag) = saldo_ertrag (difference kept)
    // 3. Credit Bank (actual payment to ESTV) = flat_rate_vat
    let mut lines = Vec::new();

    // Debit 2200: clear the collected VAT
    lines.push(JournalLineInput {
        account_id: acct_2200.id.clone(),
        debit_amount: collected,
        credit_amount: Decimal::ZERO,
        description: Some("MWST-Schuld Abrechnung".to_string()),
        vat_rate_id: None,
    });

    // Credit 3809: Saldosteuerertrag (difference)
    if saldo_ertrag > Decimal::ZERO {
        lines.push(JournalLineInput {
            account_id: acct_3809.id.clone(),
            debit_amount: Decimal::ZERO,
            credit_amount: saldo_ertrag,
            description: Some("Saldosteuerertrag".to_string()),
            vat_rate_id: None,
        });
    }

    // Credit Bank: actual ESTV payment
    lines.push(JournalLineInput {
        account_id: bank_acct_id,
        debit_amount: Decimal::ZERO,
        credit_amount: flat_rate_vat,
        description: Some("MWST-Zahlung an ESTV".to_string()),
        vat_rate_id: None,
    });

    let quarter_label = format!(
        "MWST-Abrechnung {} - {}",
        body.from_date, body.to_date
    );
    let reference = format!("MWST-{}-{}", body.from_date, body.to_date);

    let (entry, _lines) = JournalService::create(
        &state.db,
        pay_date,
        &quarter_label,
        Some(reference),
        None,
        None,
        Some(claims.sub.clone()),
        lines,
    )
    .await?;

    // Auto-post the entry
    JournalService::post_entry(&state.db, &entry.id).await?;

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "create",
        "vat_payment",
        Some(&entry.id),
        None,
        Some(&format!("VAT payment {flat_rate_vat} for {quarter_label}")),
    )
    .await?;

    Ok(Json(VatPaymentResponse {
        journal_entry_id: entry.id,
        description: quarter_label,
        vat_owed: flat_rate_vat,
        saldo_ertrag,
        bank_payment: flat_rate_vat,
    }))
}

#[utoipa::path(
    get, path = "/api/v1/dashboard",
    responses((status = 200, body = DashboardStatsResponse))
)]
pub async fn dashboard_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStatsResponse>, AppError> {
    let stats = DashboardService::get_stats(&state.db).await?;

    Ok(Json(DashboardStatsResponse {
        account_count: stats.account_count,
        active_contacts: stats.active_contacts,
        journal_entry_count: stats.journal_entry_count,
        active_projects: stats.active_projects,
        revenue_mtd: stats.revenue_mtd,
        expenses_mtd: stats.expenses_mtd,
        cash_balance: stats.cash_balance,
        open_invoices_count: stats.open_invoices_count,
        total_outstanding: stats.total_outstanding,
        recent_entries: stats
            .recent_entries
            .into_iter()
            .map(|e| RecentEntryResponse {
                id: e.id,
                date: e.date,
                reference: e.reference,
                description: e.description,
                status: e.status,
            })
            .collect(),
    }))
}

#[utoipa::path(
    post, path = "/api/v1/reports/vat/xml",
    request_body = ExportVatXmlRequest,
    responses((status = 200, description = "eCH-0217 V2.0.0 XML", content_type = "application/xml")),
    security(("bearer" = []))
)]
pub async fn export_vat_xml(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<ExportVatXmlRequest>,
) -> Result<impl IntoResponse, AppError> {
    let from = chrono::NaiveDate::parse_from_str(&body.from_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid from_date".into()))?;
    let to = chrono::NaiveDate::parse_from_str(&body.to_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid to_date".into()))?;

    let parse_dec = |s: &Option<String>| -> Result<Decimal, AppError> {
        match s {
            Some(v) if !v.is_empty() => {
                Decimal::from_str(v).map_err(|_| AppError::Validation(format!("Invalid decimal: {v}")))
            }
            _ => Ok(Decimal::ZERO),
        }
    };

    let input = VatDeclarationInput {
        from_date: from,
        to_date: to,
        type_of_submission: body.type_of_submission,
        form_of_reporting: body.form_of_reporting.unwrap_or(1),
        business_reference_id: body.business_reference_id,
        total_consideration: Decimal::from_str(&body.total_consideration)
            .map_err(|_| AppError::Validation("Invalid total_consideration".into()))?,
        supplies_to_foreign: parse_dec(&body.supplies_to_foreign)?,
        supplies_abroad: parse_dec(&body.supplies_abroad)?,
        transfer_notification: parse_dec(&body.transfer_notification)?,
        supplies_exempt: parse_dec(&body.supplies_exempt)?,
        reduction_of_consideration: parse_dec(&body.reduction_of_consideration)?,
        various_deduction: parse_dec(&body.various_deduction)?,
        subsidies: parse_dec(&body.subsidies)?,
        donations: parse_dec(&body.donations)?,
    };

    let xml_bytes = VatXmlService::generate(&state.db, input).await?;

    let filename = format!("mwst-abrechnung-{}-{}.xml", body.from_date, body.to_date);

    AuditService::log(
        &state.db,
        Some(&claims.sub),
        "export",
        "vat_xml",
        None,
        None,
        Some(&format!("eCH-0217 XML export for {} to {}", body.from_date, body.to_date)),
    )
    .await?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/xml".to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        xml_bytes,
    ))
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

impl From<konto_core::services::report_service::TrialBalanceRow>
    for TrialBalanceRowResponse
{
    fn from(r: konto_core::services::report_service::TrialBalanceRow) -> Self {
        Self {
            account_id: r.account_id,
            account_number: r.account_number,
            account_name: r.account_name,
            account_type: r.account_type,
            total_debit: r.total_debit,
            total_credit: r.total_credit,
            balance: r.balance,
        }
    }
}
