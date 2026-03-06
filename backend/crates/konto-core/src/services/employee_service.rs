use chrono::NaiveDate;
use konto_common::error::AppError;
use konto_common::enums::EmployeeStatus;
use konto_db::entities::employee;
use konto_db::repository::employee_repo::EmployeeRepo;
use rust_decimal::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

pub struct EmployeeService;

pub struct CreateEmployeeInput {
    pub number: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: NaiveDate,
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: NaiveDate,
    pub employment_end: Option<NaiveDate>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub employment_percentage: Decimal,
    pub gross_monthly_salary: Decimal,
    pub annual_salary_13th: bool,
    pub has_children: bool,
    pub number_of_children: i32,
    pub child_allowance_amount: Decimal,
    pub education_allowance_amount: Decimal,
    pub bvg_insured: bool,
    pub uvg_insured: bool,
    pub ktg_insured: bool,
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    pub quellensteuer_rate: Option<Decimal>,
    pub marital_status: String,
    pub canton: String,
    pub user_id: Option<String>,
    pub notes: Option<String>,
}

pub struct UpdateEmployeeInput {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ahv_number: String,
    pub date_of_birth: NaiveDate,
    pub nationality: String,
    pub street: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    pub iban: String,
    pub bic: Option<String>,
    pub bank_name: Option<String>,
    pub employment_start: NaiveDate,
    pub employment_end: Option<NaiveDate>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub employment_percentage: Decimal,
    pub gross_monthly_salary: Decimal,
    pub annual_salary_13th: bool,
    pub has_children: bool,
    pub number_of_children: i32,
    pub child_allowance_amount: Decimal,
    pub education_allowance_amount: Decimal,
    pub bvg_insured: bool,
    pub uvg_insured: bool,
    pub ktg_insured: bool,
    pub is_quellensteuer: bool,
    pub quellensteuer_tariff: Option<String>,
    pub quellensteuer_rate: Option<Decimal>,
    pub marital_status: String,
    pub canton: String,
    pub status: String,
    pub user_id: Option<String>,
    pub notes: Option<String>,
}

impl EmployeeService {
    pub async fn list(db: &DatabaseConnection) -> Result<Vec<employee::Model>, AppError> {
        EmployeeRepo::find_all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get(db: &DatabaseConnection, id: &str) -> Result<employee::Model, AppError> {
        EmployeeRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        input: CreateEmployeeInput,
    ) -> Result<employee::Model, AppError> {
        Self::validate_ahv(&input.ahv_number)?;

        let resolved_number = if input.number.is_some() {
            input.number
        } else {
            Self::auto_assign_number(db).await?
        };

        let now = chrono::Utc::now().naive_utc();
        let model = employee::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            number: Set(resolved_number),
            user_id: Set(input.user_id),
            first_name: Set(input.first_name),
            last_name: Set(input.last_name),
            email: Set(input.email),
            phone: Set(input.phone),
            ahv_number: Set(input.ahv_number),
            date_of_birth: Set(input.date_of_birth),
            nationality: Set(input.nationality),
            street: Set(input.street),
            postal_code: Set(input.postal_code),
            city: Set(input.city),
            country: Set(input.country),
            iban: Set(input.iban),
            bic: Set(input.bic),
            bank_name: Set(input.bank_name),
            employment_start: Set(input.employment_start),
            employment_end: Set(input.employment_end),
            position: Set(input.position),
            department: Set(input.department),
            employment_percentage: Set(input.employment_percentage),
            gross_monthly_salary: Set(input.gross_monthly_salary),
            annual_salary_13th: Set(input.annual_salary_13th),
            has_children: Set(input.has_children),
            number_of_children: Set(input.number_of_children),
            child_allowance_amount: Set(input.child_allowance_amount),
            education_allowance_amount: Set(input.education_allowance_amount),
            bvg_insured: Set(input.bvg_insured),
            uvg_insured: Set(input.uvg_insured),
            ktg_insured: Set(input.ktg_insured),
            is_quellensteuer: Set(input.is_quellensteuer),
            quellensteuer_tariff: Set(input.quellensteuer_tariff),
            quellensteuer_rate: Set(input.quellensteuer_rate),
            marital_status: Set(input.marital_status),
            canton: Set(input.canton),
            status: Set(EmployeeStatus::Active.to_string()),
            notes: Set(input.notes),
            created_at: Set(now),
            updated_at: Set(now),
        };

        EmployeeRepo::create(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: &str,
        input: UpdateEmployeeInput,
    ) -> Result<employee::Model, AppError> {
        Self::validate_ahv(&input.ahv_number)?;

        let existing = EmployeeRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;

        let now = chrono::Utc::now().naive_utc();
        let mut model: employee::ActiveModel = existing.into();
        model.user_id = Set(input.user_id);
        model.first_name = Set(input.first_name);
        model.last_name = Set(input.last_name);
        model.email = Set(input.email);
        model.phone = Set(input.phone);
        model.ahv_number = Set(input.ahv_number);
        model.date_of_birth = Set(input.date_of_birth);
        model.nationality = Set(input.nationality);
        model.street = Set(input.street);
        model.postal_code = Set(input.postal_code);
        model.city = Set(input.city);
        model.country = Set(input.country);
        model.iban = Set(input.iban);
        model.bic = Set(input.bic);
        model.bank_name = Set(input.bank_name);
        model.employment_start = Set(input.employment_start);
        model.employment_end = Set(input.employment_end);
        model.position = Set(input.position);
        model.department = Set(input.department);
        model.employment_percentage = Set(input.employment_percentage);
        model.gross_monthly_salary = Set(input.gross_monthly_salary);
        model.annual_salary_13th = Set(input.annual_salary_13th);
        model.has_children = Set(input.has_children);
        model.number_of_children = Set(input.number_of_children);
        model.child_allowance_amount = Set(input.child_allowance_amount);
        model.education_allowance_amount = Set(input.education_allowance_amount);
        model.bvg_insured = Set(input.bvg_insured);
        model.uvg_insured = Set(input.uvg_insured);
        model.ktg_insured = Set(input.ktg_insured);
        model.is_quellensteuer = Set(input.is_quellensteuer);
        model.quellensteuer_tariff = Set(input.quellensteuer_tariff);
        model.quellensteuer_rate = Set(input.quellensteuer_rate);
        model.marital_status = Set(input.marital_status);
        model.canton = Set(input.canton);
        model.status = Set(input.status);
        model.notes = Set(input.notes);
        model.updated_at = Set(now);

        EmployeeRepo::update(db, model)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), AppError> {
        EmployeeRepo::find_by_id(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Employee not found".into()))?;

        EmployeeRepo::delete(db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn auto_assign_number(
        db: &DatabaseConnection,
    ) -> Result<Option<String>, AppError> {
        use konto_db::entities::company_setting;

        let settings = company_setting::Entity::find()
            .one(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let settings = match settings {
            Some(s) if s.employee_number_auto => s,
            _ => return Ok(None),
        };

        let all_employees = employee::Entity::find()
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let prefix = &settings.employee_number_prefix;
        let year = chrono::Utc::now().format("%Y").to_string();
        let year_prefix = if settings.employee_number_restart_yearly {
            format!("{}{}-", prefix, year)
        } else {
            prefix.clone()
        };

        let max_num = all_employees
            .iter()
            .filter_map(|e| {
                e.number.as_ref().and_then(|n| {
                    n.strip_prefix(&year_prefix)
                        .and_then(|s| s.parse::<i32>().ok())
                })
            })
            .max()
            .unwrap_or(settings.employee_number_start - 1);

        let next = max_num + 1;
        let min_len = settings.employee_number_min_length as usize;
        let number_str = format!("{:0>width$}", next, width = min_len);

        Ok(Some(format!("{}{}", year_prefix, number_str)))
    }

    /// Validate Swiss AHV number format: 756.XXXX.XXXX.XX
    fn validate_ahv(ahv: &str) -> Result<(), AppError> {
        let stripped: String = ahv.chars().filter(|c| c.is_ascii_digit()).collect();
        if stripped.len() != 13 || !stripped.starts_with("756") {
            return Err(AppError::BadRequest(
                "Invalid AHV number. Expected format: 756.XXXX.XXXX.XX".into(),
            ));
        }
        Ok(())
    }
}
