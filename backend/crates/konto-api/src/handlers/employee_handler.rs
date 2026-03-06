use axum::extract::{Path, State};
use axum::{Extension, Json};
use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_core::auth::jwt::Claims;
use konto_core::services::audit_service::AuditService;
use konto_core::services::employee_service::{CreateEmployeeInput, EmployeeService, UpdateEmployeeInput};
use konto_core::services::employee_user_link_service::EmployeeUserLinkService;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::dto::employee::*;
use crate::state::AppState;

/// List all employees.
#[utoipa::path(
    get, path = "/api/v1/employees",
    responses((status = 200, body = Vec<EmployeeResponse>)),
    security(("bearer" = [])),
    tag = "employees"
)]
pub async fn list_employees(
    State(state): State<AppState>,
) -> Result<Json<Vec<EmployeeResponse>>, AppError> {
    let employees = EmployeeService::list(&state.db).await?;
    let data = employees.into_iter().map(EmployeeResponse::from).collect();
    Ok(Json(data))
}

/// Get employee by ID.
#[utoipa::path(
    get, path = "/api/v1/employees/{id}",
    responses((status = 200, body = EmployeeResponse)),
    security(("bearer" = [])),
    tag = "employees"
)]
pub async fn get_employee(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EmployeeResponse>, AppError> {
    let employee = EmployeeService::get(&state.db, &id).await?;
    Ok(Json(EmployeeResponse::from(employee)))
}

/// Create a new employee.
#[utoipa::path(
    post, path = "/api/v1/employees",
    request_body = CreateEmployeeRequest,
    responses((status = 201, body = CreateEmployeeResponse)),
    security(("bearer" = [])),
    tag = "employees"
)]
pub async fn create_employee(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(body): Json<CreateEmployeeRequest>,
) -> Result<Json<CreateEmployeeResponse>, AppError> {
    let create_user = body.create_user.unwrap_or(false);
    let user_role_id = body.user_role_id.clone();

    let input = CreateEmployeeInput {
        number: None,
        first_name: body.first_name,
        last_name: body.last_name,
        email: body.email,
        phone: body.phone,
        ahv_number: body.ahv_number,
        date_of_birth: parse_date(&body.date_of_birth)?,
        nationality: body.nationality,
        street: body.street,
        postal_code: body.postal_code,
        city: body.city,
        country: body.country,
        iban: body.iban,
        bic: body.bic,
        bank_name: body.bank_name,
        employment_start: parse_date(&body.employment_start)?,
        employment_end: parse_optional_date(&body.employment_end)?,
        position: body.position,
        department: body.department,
        employment_percentage: to_decimal(body.employment_percentage)?,
        gross_monthly_salary: to_decimal(body.gross_monthly_salary)?,
        annual_salary_13th: body.annual_salary_13th,
        has_children: body.has_children,
        number_of_children: body.number_of_children,
        child_allowance_amount: to_decimal(body.child_allowance_amount)?,
        education_allowance_amount: to_decimal(body.education_allowance_amount)?,
        bvg_insured: body.bvg_insured,
        uvg_insured: body.uvg_insured,
        ktg_insured: body.ktg_insured,
        is_quellensteuer: body.is_quellensteuer,
        quellensteuer_tariff: body.quellensteuer_tariff,
        quellensteuer_rate: body.quellensteuer_rate.map(to_decimal).transpose()?,
        marital_status: body.marital_status,
        canton: body.canton,
        user_id: body.user_id,
        notes: body.notes,
    };

    let emp = EmployeeService::create(&state.db, input).await?;

    // Optionally provision a user account
    let provisioned_user = if create_user {
        let role_id = user_role_id.ok_or_else(|| {
            AppError::BadRequest("user_role_id is required when create_user is true".into())
        })?;
        let result = EmployeeUserLinkService::provision_user(&state.db, &emp, &role_id).await?;
        Some(ProvisionedUserInfo {
            user_id: result.user_id,
            temp_password: result.temp_password,
        })
    } else {
        None
    };

    let resp = EmployeeResponse::from(emp.clone());
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "create", "employee",
        Some(&emp.id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(CreateEmployeeResponse {
        employee: resp,
        provisioned_user,
    }))
}

/// Update an existing employee.
#[utoipa::path(
    put, path = "/api/v1/employees/{id}",
    request_body = UpdateEmployeeRequest,
    responses((status = 200, body = EmployeeResponse)),
    security(("bearer" = [])),
    tag = "employees"
)]
pub async fn update_employee(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateEmployeeRequest>,
) -> Result<Json<EmployeeResponse>, AppError> {
    let input = UpdateEmployeeInput {
        first_name: body.first_name,
        last_name: body.last_name,
        email: body.email,
        phone: body.phone,
        ahv_number: body.ahv_number,
        date_of_birth: parse_date(&body.date_of_birth)?,
        nationality: body.nationality,
        street: body.street,
        postal_code: body.postal_code,
        city: body.city,
        country: body.country,
        iban: body.iban,
        bic: body.bic,
        bank_name: body.bank_name,
        employment_start: parse_date(&body.employment_start)?,
        employment_end: parse_optional_date(&body.employment_end)?,
        position: body.position,
        department: body.department,
        employment_percentage: to_decimal(body.employment_percentage)?,
        gross_monthly_salary: to_decimal(body.gross_monthly_salary)?,
        annual_salary_13th: body.annual_salary_13th,
        has_children: body.has_children,
        number_of_children: body.number_of_children,
        child_allowance_amount: to_decimal(body.child_allowance_amount)?,
        education_allowance_amount: to_decimal(body.education_allowance_amount)?,
        bvg_insured: body.bvg_insured,
        uvg_insured: body.uvg_insured,
        ktg_insured: body.ktg_insured,
        is_quellensteuer: body.is_quellensteuer,
        quellensteuer_tariff: body.quellensteuer_tariff,
        quellensteuer_rate: body.quellensteuer_rate.map(to_decimal).transpose()?,
        marital_status: body.marital_status,
        canton: body.canton,
        status: body.status,
        user_id: body.user_id,
        notes: body.notes,
    };

    let emp = EmployeeService::update(&state.db, &id, input).await?;

    let resp = EmployeeResponse::from(emp);
    let new_vals = serde_json::to_string(&resp).ok();
    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "update", "employee",
        Some(&id), None, new_vals.as_deref(),
    ).await;

    Ok(Json(resp))
}

/// Delete an employee.
#[utoipa::path(
    delete, path = "/api/v1/employees/{id}",
    responses((status = 200)),
    security(("bearer" = [])),
    tag = "employees"
)]
pub async fn delete_employee(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    EmployeeService::delete(&state.db, &id).await?;

    let _ = AuditService::log(
        &state.db, Some(&claims.sub), "delete", "employee",
        Some(&id), None, None,
    ).await;

    Ok(Json(serde_json::json!({"deleted": true})))
}

fn parse_date(s: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format, expected YYYY-MM-DD".into()))
}

fn parse_optional_date(s: &Option<String>) -> Result<Option<NaiveDate>, AppError> {
    match s {
        Some(v) if !v.is_empty() => parse_date(v).map(Some),
        _ => Ok(None),
    }
}

fn to_decimal(v: f64) -> Result<Decimal, AppError> {
    Decimal::from_str(&v.to_string())
        .map_err(|_| AppError::BadRequest("Invalid decimal value".into()))
}
