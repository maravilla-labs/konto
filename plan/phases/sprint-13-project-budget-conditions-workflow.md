# Sprint 13 — Project Budget Tracking, Conditions & Time Entry Workflow

## Goal
Close the project management gaps: invoicing method configuration, per-member/per-activity budgets, time entry status workflow, rounding on invoice import, auto-assigned project numbers, and sub-statuses.

## Technical Decisions
- **TD-043**: Invoicing method as project-level discriminator (hourly/fixed_price/flat_rate/non_billable)
- **TD-044**: Time entry status workflow — `pending` → `in_progress` → `done` → `invoiced` → `closed` with `billable` boolean
- **TD-045**: Time rounding on invoice import — per-project rounding_method + rounding_factor_minutes
- **TD-046**: Project number auto-assignment — settings-based pattern with prefix + zero-padded number
- **TD-047**: Project sub-statuses — configurable table seeded with 8 industry defaults

## Migrations (000070–000077)
| # | Name | Description |
|---|------|-------------|
| 070 | alter_projects_conditions | Add invoicing_method, currency, rounding_method, rounding_factor_minutes, flat_rate_total |
| 071 | alter_project_members_budget | Add budget_hours to project_members |
| 072 | alter_project_activity_types_budget | Add budget_hours, chargeable to project_activity_types |
| 073 | alter_time_entries_status_workflow | Add billable, start_time, end_time + data migration |
| 074 | alter_company_settings_project_numbering | Add project_number_* settings |
| 075 | create_project_sub_statuses | New table |
| 076 | seed_project_sub_statuses | 8 industry defaults |
| 077 | alter_projects_sub_status | Add sub_status_id FK |

## Phases

### Phase 1: Project Conditions & Configuration
- Entities: project, project_member, project_activity_type — new fields
- DTOs: request/response updates
- Services: wire new fields through create/update
- Frontend: ProjectEditDialog conditions section, member budget column, activity type budget/chargeable

### Phase 2: Time Entry Status Workflow
- New: time_entry_workflow.rs (status transitions, validation)
- Modified: time_entry entity/DTO/service/handler — billable, start_time, end_time
- New route: PUT /api/v1/time-entries/{id}/transition
- Frontend: billable toggle, time inputs, status badges, status filter

### Phase 3: Enhanced Budget Analytics
- New: project_budget_service.rs — per-member, per-activity, timeline analytics
- New: project_budget DTO
- New route: GET /api/v1/projects/{id}/budget-analytics
- Frontend: BudgetAnalyticsChart (recharts), BudgetBreakdownTable, enhanced ProjectOverviewTab

### Phase 4: Invoice Time Import Rounding
- Backend: round_minutes helper in invoice_handler, applies project rounding settings
- Validation: only done+billable entries can be invoiced, auto-transition to invoiced
- Frontend: rounding preview in TimeEntryImportDialog

### Phase 5: Project Number Auto-Assignment
- Settings: project_number_auto, prefix, restart_yearly, start, min_length
- Service: auto_assign_number on project create
- Frontend: CompanySettingsPage numbering card with preview

### Phase 6: Project Sub-Statuses
- New entity/repo/service/DTO/handler for project_sub_statuses
- Routes: CRUD /api/v1/project-sub-statuses
- Frontend: ProjectSubStatusesPage, colored badges on projects

## New API Routes
| Method | Path | Description |
|--------|------|-------------|
| GET | /api/v1/projects/{id}/budget-analytics | Budget analytics breakdown |
| PUT | /api/v1/time-entries/{id}/transition | Time entry status transition |
| GET | /api/v1/project-sub-statuses | List sub-statuses |
| POST | /api/v1/project-sub-statuses | Create sub-status |
| PUT | /api/v1/project-sub-statuses/{id} | Update sub-status |
| DELETE | /api/v1/project-sub-statuses/{id} | Delete sub-status |

## Summary
| Metric | Count |
|--------|-------|
| Migrations | 8 |
| New backend files | ~8 |
| Modified backend files | ~18 |
| New frontend files | ~5 |
| Modified frontend files | ~12 |
| New API routes | 6 |
| i18n keys | ~140 (en/de/fr/it) |
| Technical decisions | 5 (TD-043–TD-047) |
