use chrono::Datelike;
use konto_common::error::AppError;
use konto_common::enums::InvoiceStatus;
use konto_db::entities::{invoice, project_activity_type, project_member, time_entry};
use konto_db::repository::project_repo::ProjectRepo;
use konto_db::repository::user_repo::UserRepo;
use rust_decimal::Decimal;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use super::activity_type_service::ActivityTypeService;
use super::project_member_service::ProjectMemberService;

pub struct BudgetAnalytics {
    pub total_hours: Decimal,
    pub billable_hours: Decimal,
    pub non_billable_hours: Decimal,
    pub billed_hours: Decimal,
    pub unbilled_hours: Decimal,
    pub budget_hours: Option<Decimal>,
    pub actual_amount: Decimal,
    pub invoiced_amount: Decimal,
    pub per_member: Vec<MemberBudgetRow>,
    pub per_activity: Vec<ActivityBudgetRow>,
    pub timeline: Vec<WeeklyBudgetPoint>,
}

pub struct MemberBudgetRow {
    pub user_id: String,
    pub user_name: String,
    pub budget_hours: Option<Decimal>,
    pub actual_hours: Decimal,
    pub rate: Option<Decimal>,
    pub actual_amount: Decimal,
}

pub struct ActivityBudgetRow {
    pub activity_type_id: String,
    pub activity_name: String,
    pub budget_hours: Option<Decimal>,
    pub actual_hours: Decimal,
    pub chargeable: bool,
    pub rate: Option<Decimal>,
    pub actual_amount: Decimal,
}

pub struct WeeklyBudgetPoint {
    pub week_start: String,
    pub cumulative_budget: Decimal,
    pub cumulative_actual: Decimal,
}

pub struct ProjectBudgetService;

impl ProjectBudgetService {
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    pub async fn get_budget_analytics(
        db: &DatabaseConnection,
        project_id: &str,
    ) -> Result<BudgetAnalytics, AppError> {
        let project = ProjectRepo::find_by_id(db, project_id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

        let entries = time_entry::Entity::find()
            .filter(time_entry::Column::ProjectId.eq(project_id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let sixty = Decimal::from(60);

        let total_minutes: i32 = entries.iter().map(|e| e.actual_minutes).sum();
        let total_hours = Decimal::from(total_minutes) / sixty;

        let billable_minutes: i32 = entries.iter().filter(|e| e.billable).map(|e| e.actual_minutes).sum();
        let billable_hours = Decimal::from(billable_minutes) / sixty;
        let non_billable_hours = total_hours - billable_hours;

        let billed_minutes: i32 = entries.iter().filter(|e| e.billed).map(|e| e.actual_minutes).sum();
        let billed_hours = Decimal::from(billed_minutes) / sixty;
        let unbilled_hours = billable_hours - billed_hours;

        // Invoiced amount
        let invoices = invoice::Entity::find()
            .filter(invoice::Column::ProjectId.eq(project_id))
            .filter(
                Condition::any()
                    .add(invoice::Column::Status.eq(InvoiceStatus::Sent.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Paid.as_str()))
                    .add(invoice::Column::Status.eq(InvoiceStatus::Overdue.as_str())),
            )
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        let invoiced_amount: Decimal = invoices.iter().map(|i| i.total).sum();

        // Build rate lookup: activity_type_id → effective rate (project override > default)
        let pats = project_activity_type::Entity::find()
            .filter(project_activity_type::Column::ProjectId.eq(project_id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut activity_rate_map: std::collections::HashMap<String, Option<Decimal>> =
            std::collections::HashMap::new();
        let mut activity_name_map: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        for pat in &pats {
            let at = ActivityTypeService::get_by_id(db, &pat.activity_type_id).await.ok();
            let name = at.as_ref().map(|a| a.name.clone()).unwrap_or_else(|| "Unknown".to_string());
            let default_rate = at.as_ref().and_then(|a| a.default_rate);
            let effective_rate = pat.rate.or(default_rate);
            activity_rate_map.insert(pat.activity_type_id.clone(), effective_rate);
            activity_name_map.insert(pat.activity_type_id.clone(), name);
        }

        let project_hourly_rate = project.hourly_rate;

        // Resolve rate for a single entry: activity type rate > member rate > project hourly rate
        let resolve_entry_rate = |e: &time_entry::Model| -> Option<Decimal> {
            if let Some(rate) = e
                .activity_type_id
                .as_ref()
                .and_then(|at_id| activity_rate_map.get(at_id).copied().flatten())
            {
                return Some(rate);
            }
            project_hourly_rate
        };

        // Compute actual_amount from all billable entries
        let mut actual_amount = Decimal::ZERO;
        for e in &entries {
            if e.billable {
                let hours = Decimal::from(e.actual_minutes) / sixty;
                if let Some(rate) = resolve_entry_rate(e) {
                    actual_amount += hours * rate;
                }
            }
        }

        // Per-member breakdown
        let members = project_member::Entity::find()
            .filter(project_member::Column::ProjectId.eq(project_id))
            .all(db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut per_member = Vec::new();
        for m in &members {
            let user_name = UserRepo::find_by_id(db, &m.user_id)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
                .map(|u| u.full_name)
                .unwrap_or_else(|| "Unknown".to_string());

            let member_entries: Vec<&time_entry::Model> = entries
                .iter()
                .filter(|e| e.billable && e.user_id.as_deref() == Some(&m.user_id))
                .collect();
            let member_minutes: i32 = member_entries.iter().map(|e| e.actual_minutes).sum();
            let member_hours = Decimal::from(member_minutes) / sixty;

            let rate = ProjectMemberService::resolve_rate(db, project_id, &m.user_id).await?;
            let member_amount: Decimal = member_entries.iter().map(|e| {
                let hours = Decimal::from(e.actual_minutes) / sixty;
                resolve_entry_rate(e).or(rate).map(|r| hours * r).unwrap_or(Decimal::ZERO)
            }).sum();

            per_member.push(MemberBudgetRow {
                user_id: m.user_id.clone(),
                user_name,
                budget_hours: m.budget_hours,
                actual_hours: member_hours,
                rate,
                actual_amount: member_amount,
            });
        }

        // Per-activity breakdown (billable entries only)
        let mut per_activity = Vec::new();
        for pat in &pats {
            let activity_name = activity_name_map.get(&pat.activity_type_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            let effective_rate = activity_rate_map.get(&pat.activity_type_id).copied().flatten();

            let activity_minutes: i32 = entries
                .iter()
                .filter(|e| e.billable && e.activity_type_id.as_deref() == Some(&pat.activity_type_id))
                .map(|e| e.actual_minutes)
                .sum();
            let activity_hours = Decimal::from(activity_minutes) / sixty;
            let activity_amount = effective_rate.map(|r| activity_hours * r).unwrap_or(Decimal::ZERO);

            per_activity.push(ActivityBudgetRow {
                activity_type_id: pat.activity_type_id.clone(),
                activity_name,
                budget_hours: pat.budget_hours,
                actual_hours: activity_hours,
                chargeable: pat.chargeable,
                rate: effective_rate,
                actual_amount: activity_amount,
            });
        }

        // Timeline: weekly cumulative budget vs actual
        let budget_hours = project.budget_hours.or(project.hard_budget_hours);
        let mut timeline = Vec::new();

        if !entries.is_empty() {
            let mut sorted = entries.clone();
            sorted.sort_by_key(|e| e.date);

            let start = sorted.first().expect("entries is non-empty").date;
            let end = sorted.last().expect("entries is non-empty").date;

            let mut current = start;
            // Align to Monday
            let weekday = current.weekday().num_days_from_monday();
            current -= chrono::Duration::days(weekday as i64);

            let total_weeks = ((end - current).num_days() / 7 + 1).max(1);

            let mut cumulative_actual = Decimal::ZERO;
            while current <= end + chrono::Duration::days(7) {
                let week_end = current + chrono::Duration::days(7);
                let week_minutes: i32 = sorted
                    .iter()
                    .filter(|e| e.date >= current && e.date < week_end)
                    .map(|e| e.actual_minutes)
                    .sum();
                cumulative_actual += Decimal::from(week_minutes) / sixty;

                let weeks_elapsed = ((current - start).num_days() / 7 + 1).max(1);
                let cumulative_budget = budget_hours
                    .map(|bh| bh * Decimal::from(weeks_elapsed) / Decimal::from(total_weeks))
                    .unwrap_or(Decimal::ZERO);

                timeline.push(WeeklyBudgetPoint {
                    week_start: current.to_string(),
                    cumulative_budget,
                    cumulative_actual,
                });

                current = week_end;
            }
        }

        Ok(BudgetAnalytics {
            total_hours,
            billable_hours,
            non_billable_hours,
            billed_hours,
            unbilled_hours,
            budget_hours,
            actual_amount,
            invoiced_amount,
            per_member,
            per_activity,
            timeline,
        })
    }
}
