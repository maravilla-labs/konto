# Sprint 12 — Year-End Closing Fix, Depreciation Engine, Swiss Payroll System

## Context

The Buchhaltungshandbuch analysis revealed three gaps: (1) year-end closing books zero amounts instead of actual P&L, (2) no depreciation engine for fixed assets, (3) no payroll system. Hope covers ~85% of the handbook via automated workflows + manual journal. This sprint closes the remaining 15%.

Migrations start at **000061** (last existing: 000060 `add_qr_iban`).

## New Technical Decisions

- **TD-038**: Employee as separate entity from User (not all employees need system login)
- **TD-039**: Payroll settings as singleton table (like company_settings) with Swiss 2025 defaults
- **TD-040**: BVG age-based rate selection using employee DOB at payroll run date
- **TD-041**: pain.001 via quick-xml (no Swiss-specific Rust crate exists for ch.03 variant)
- **TD-042**: Lohnausweis via Typst (same pipeline as invoice/annual report PDFs)

## Phase 1: Fix Year-End Closing (1 file change)

**Problem**: `fiscal_year_service.rs:157-200` creates closing journal entry with `Decimal::ZERO` for accounts 2979/2970.

**Fix**: Before creating the closing entry, call `ReportService::profit_loss(db, start_date, end_date)` to get `net_income`, then:
- Profit (net_income > 0): Debit 2979 Jahresergebnis / Credit 2970 Gewinnvortrag
- Loss (net_income < 0): Debit 2970 / Credit 2979 for abs(net_income)
- Zero: Skip closing entry, just close the year

**Files to modify**:
- `backend/crates/konto-core/src/services/fiscal_year_service.rs` — add `use super::report_service::ReportService`, call `profit_loss()` in `close()`, use result for journal line amounts

**No migrations, no new files.**

---

## Phase 2: Depreciation Engine

### Migrations
- **000061** `create_fixed_assets`: `fixed_assets` table — id (PK string), name (string, not null), description (text, nullable), account_id (string FK accounts, e.g. 1521), depreciation_account_id (string FK accounts, e.g. 6822), acquisition_date (date, not null), acquisition_cost (Decimal 15,2), residual_value (Decimal 15,2), useful_life_years (integer), depreciation_method (string: "straight_line" | "declining_balance"), declining_rate (Decimal 5,4, nullable, e.g. 0.40), status (string: "active" | "fully_depreciated" | "disposed"), disposed_date (date nullable), created_at (timestamp), updated_at (timestamp)
- **000062** `create_depreciation_entries`: `depreciation_entries` table — id (PK string), fixed_asset_id (string FK fixed_assets), fiscal_year_id (string FK fiscal_years), journal_entry_id (string FK journal_entries), amount (Decimal 15,2), accumulated (Decimal 15,2), book_value (Decimal 15,2), period_date (date), created_at (timestamp)

### Backend (8 files)
- **Entity**: `konto-db/src/entities/fixed_asset.rs`, `konto-db/src/entities/depreciation_entry.rs`
- **Repo**: `konto-db/src/repository/fixed_asset_repo.rs`, `konto-db/src/repository/depreciation_entry_repo.rs`
- **Service**: `konto-core/src/services/fixed_asset_service.rs` (CRUD + status management), `konto-core/src/services/depreciation_service.rs` (calculation + run)
- **Handler**: `konto-api/src/handlers/fixed_asset_handler.rs` (CRUD + POST run_depreciation)
- **DTO**: `konto-api/src/dto/fixed_asset.rs`

### Depreciation Logic (`depreciation_service.rs`)
- `calculate(asset, fiscal_year)`:
  - Straight-line: `(acquisition_cost - residual_value) / useful_life_years`
  - Declining balance: `current_book_value * declining_rate` (floor at residual_value)
- `run_depreciation(db, fiscal_year_id)`: for each active asset, calculate, create journal entry (Debit depreciation_account / Credit asset_account), record depreciation_entry, update accumulated/book_value
- `get_schedule(db, asset_id)`: list all depreciation_entries for an asset

### Routes
- `GET /api/v1/fixed-assets` — list
- `POST /api/v1/fixed-assets` — create
- `GET /api/v1/fixed-assets/:id` — detail
- `PUT /api/v1/fixed-assets/:id` — update
- `DELETE /api/v1/fixed-assets/:id` — delete
- `GET /api/v1/fixed-assets/:id/schedule` — depreciation history
- `POST /api/v1/fixed-assets/run-depreciation` — run for a fiscal year (body: fiscal_year_id)

### Frontend (4 files)
- **Types**: `frontend/src/types/fixed-asset.ts`
- **API**: `frontend/src/api/fixed-assets.ts`
- **Pages**: `frontend/src/pages/settings/FixedAssetsPage.tsx` (list + create/edit dialog + depreciation schedule + run button)
- **i18n domain**: `fixed_assets.*`
- Add to Settings Hub under "Accounting" section
- Add to navigation registry

---

## Phase 3: Payroll — Data Model & Staff Management

### Migrations
- **000063** `create_employees`: `employees` table — id (PK string), user_id (string FK users, nullable), first_name (string), last_name (string), ahv_number (string, 756.XXXX.XXXX.XX), date_of_birth (date), street (string), postal_code (string), city (string), country (string default "CH"), iban (string), bic (string nullable), bank_name (string nullable), employment_start (date), employment_end (date nullable), employment_percentage (Decimal 5,2 default 100.00), gross_monthly_salary (Decimal 15,2), has_children (bool default false), number_of_children (integer default 0), child_allowance_amount (Decimal 10,2 default 215.00), education_allowance_amount (Decimal 10,2 default 268.00), is_quellensteuer (bool default false), quellensteuer_tariff (string nullable), quellensteuer_rate (Decimal 5,4 nullable), marital_status (string default "single"), canton (string 2-char default "BS"), status (string: "active" | "terminated"), created_at (timestamp), updated_at (timestamp)
- **000064** `create_payroll_settings`: `payroll_settings` singleton — id (PK string), ahv_iv_eo_rate_employee (Decimal default 5.30), ahv_iv_eo_rate_employer (Decimal default 5.30), alv_rate_employee (Decimal default 1.10), alv_rate_employer (Decimal default 1.10), alv_salary_cap (Decimal default 148200), bvg_coordination_deduction (Decimal default 26460), bvg_entry_threshold (Decimal default 22680), bvg_min_insured_salary (Decimal default 3780), bvg_max_insured_salary (Decimal default 64260), bvg_rate_25_34 (Decimal default 7.0), bvg_rate_35_44 (Decimal default 10.0), bvg_rate_45_54 (Decimal default 15.0), bvg_rate_55_65 (Decimal default 18.0), bvg_risk_rate (Decimal default 2.5), bvg_employer_share_pct (Decimal default 50.0), nbu_rate_employee (Decimal default 1.50), bu_rate_employer (Decimal default 0.10), ktg_rate_employee (Decimal default 0.50), ktg_rate_employer (Decimal default 0.50), fak_rate_employer (Decimal default 1.60), uvg_max_salary (Decimal default 148200), payment_bank_account_id (string FK bank_accounts, nullable), company_clearing_number (string nullable), created_at (timestamp), updated_at (timestamp)
- **000065** `seed_payroll_settings`: insert default row with Swiss 2025 rates

### Backend (10 files)
- **Entities**: `employee.rs`, `payroll_setting.rs`
- **Repos**: `employee_repo.rs`, `payroll_setting_repo.rs`
- **Services**: `employee_service.rs` (CRUD, validate AHV format), `payroll_settings_service.rs` (get/update singleton)
- **Handlers**: `employee_handler.rs`, `payroll_settings_handler.rs`
- **DTOs**: `employee.rs`, `payroll_settings.rs`

### Routes
- `GET/POST /api/v1/employees` — list/create
- `GET/PUT/DELETE /api/v1/employees/:id` — detail/update/delete
- `GET/PUT /api/v1/payroll-settings` — get/update singleton

### Frontend (4 files)
- **Types**: `frontend/src/types/employee.ts`, `frontend/src/types/payroll-settings.ts`
- **API**: `frontend/src/api/employees.ts`, `frontend/src/api/payroll-settings.ts`
- **Pages**: `frontend/src/pages/settings/EmployeesPage.tsx` (list + create/edit dialog with all fields)
- **Pages**: `frontend/src/pages/settings/PayrollSettingsPage.tsx` (form with all rate fields, grouped by insurance type)
- **i18n domains**: `employees.*`, `payroll_settings.*`
- Add both to Settings Hub: Employees under "General", Payroll Settings under "Billing"
- Add to navigation registry

---

## Phase 4: Payroll Runs & Calculation Engine (depends on Phase 3)

### Migrations
- **000066** `create_payroll_runs`: `payroll_runs` table — id (PK string), period_month (integer), period_year (integer), status (string: "draft" | "calculated" | "approved" | "paid"), run_date (date), approved_by (string nullable), approved_at (timestamp nullable), paid_at (timestamp nullable), journal_entry_id (string FK journal_entries, nullable), payment_file_generated (bool default false), total_gross (Decimal 15,2), total_net (Decimal 15,2), total_employer_cost (Decimal 15,2), created_at (timestamp), updated_at (timestamp). Unique constraint on (period_month, period_year).
- **000067** `create_payroll_run_lines`: `payroll_run_lines` table — id (PK string), payroll_run_id (string FK), employee_id (string FK), gross_salary (Decimal 15,2), ahv_employee (Decimal 15,2), ahv_employer (Decimal 15,2), alv_employee (Decimal 15,2), alv_employer (Decimal 15,2), bvg_employee (Decimal 15,2), bvg_employer (Decimal 15,2), nbu_employee (Decimal 15,2), bu_employer (Decimal 15,2), ktg_employee (Decimal 15,2), ktg_employer (Decimal 15,2), fak_employer (Decimal 15,2), quellensteuer (Decimal 15,2 default 0), child_allowance (Decimal 15,2 default 0), net_salary (Decimal 15,2), payout_amount (Decimal 15,2), total_employer_cost (Decimal 15,2), created_at (timestamp)

### Backend (8 files)
- **Entities**: `payroll_run.rs`, `payroll_run_line.rs`
- **Repos**: `payroll_run_repo.rs`, `payroll_run_line_repo.rs`
- **Service**: `payroll_calculation.rs` — pure calculation functions:
  - `calculate_ahv(gross, rate) -> Decimal`
  - `calculate_alv(gross_annual, rate, cap) -> Decimal` (monthly, capped)
  - `calculate_bvg(gross_annual, age, settings) -> (employee, employer)` (coordinated salary, age-based rate)
  - `calculate_nbu(gross, rate, cap) -> Decimal`
  - `calculate_payroll_line(employee, settings) -> PayrollLineCalc` — orchestrates all
- **Service**: `payroll_run_service.rs` — CRUD + workflow:
  - `create(db, month, year)` → draft run
  - `calculate(db, run_id)` → for each active employee, call calculation, save lines, update totals → status "calculated"
  - `approve(db, run_id, user_id)` → generate compound journal entry, status "approved"
  - `mark_paid(db, run_id)` → status "paid", paid_at
  - `recalculate(db, run_id)` → delete lines, re-run (only if draft/calculated)

### Journal Entry on Approve
Single compound journal entry with these lines:
- Debit 5000 Löhne: sum gross_salary
- Debit 5700 AHV AG: sum ahv_employer
- Debit 5720 BVG AG: sum bvg_employer
- Debit 5730 UVG AG: sum bu_employer
- Debit 5740 KTG AG: sum ktg_employer
- Debit 5710 FAK AG: sum fak_employer
- Credit 2271 KK AHV: sum (ahv_employee + ahv_employer + alv_employee + alv_employer)
- Credit 2270 KK PK: sum (bvg_employee + bvg_employer)
- Credit 2273 KK UVG: sum (nbu_employee + bu_employer)
- Credit 2274 KK KTG: sum (ktg_employee + ktg_employer)
- Credit 2272 KK FAK: sum (fak_employer - child_allowance total)
- Credit 1020 Bank: sum payout_amount

### Handler: `payroll_run_handler.rs`
### DTO: `payroll_run.rs`

### Routes
- `GET /api/v1/payroll-runs` — list
- `POST /api/v1/payroll-runs` — create (body: month, year)
- `GET /api/v1/payroll-runs/:id` — detail with lines
- `POST /api/v1/payroll-runs/:id/calculate` — trigger calculation
- `POST /api/v1/payroll-runs/:id/approve` — approve + journal
- `POST /api/v1/payroll-runs/:id/mark-paid` — mark paid
- `DELETE /api/v1/payroll-runs/:id` — delete (draft only)

### Frontend (3 files)
- **Types**: `frontend/src/types/payroll-run.ts`
- **API**: `frontend/src/api/payroll-runs.ts`
- **Pages**: `frontend/src/pages/PayrollRunsPage.tsx` (list + create dialog)
- **Pages**: `frontend/src/pages/PayrollRunDetailPage.tsx` (employee lines table, totals summary, approve/pay buttons)
- **i18n domain**: `payroll.*`
- Add to sidebar navigation (main section, near Time Entries)

---

## Phase 5: Monthly Pay Slip PDF (depends on Phase 4)

### Backend (3 files)
- **Service**: `konto-core/src/services/pdf_payslip.rs` — Typst template for monthly Lohnabrechnung
  - Employee header: name, address, AHV-Nr, period
  - Bruttolohn section
  - Deductions table: AHV/IV/EO, ALV, BVG, NBU, KTG, Quellensteuer
  - = Nettolohn
  - + Kinderzulagen
  - = Auszahlungsbetrag
  - Employer costs section (informational)
  - Company logo + address in header
- **Service**: `konto-core/src/services/pdf_payslip_helpers.rs` — table rendering helpers if needed

### Routes (added to payroll_run_handler.rs)
- `GET /api/v1/payroll-runs/:id/payslip/:employee_id` — returns PDF bytes
- `GET /api/v1/payroll-runs/:id/payslips` — returns ZIP of all payslips

### Frontend
- Download button per employee on PayrollRunDetailPage
- "Download All Payslips" button (ZIP)

---

## Phase 6: Annual Lohnausweis PDF — Form 11 (depends on Phase 4)

### Backend (3 files)
- **Service**: `konto-core/src/services/pdf_lohnausweis.rs` — ESTV Formular 11 layout via Typst
  - Header: employer + employee identification, AHV number, employment period
  - Field 1: Lohn (sum gross_salary for year)
  - Field 8: Bruttolohn total
  - Field 9: AHV/IV/EO/ALV/NBU deductions
  - Field 10: BVG deductions
  - Field 11: Nettolohn
  - Field 12: Quellensteuer
  - Field 13: Kinderzulagen
- **Service**: `konto-core/src/services/lohnausweis_service.rs` — aggregation: sum payroll_run_lines per employee per year
- **Handler additions**: routes in a new `lohnausweis_handler.rs`

### Routes
- `GET /api/v1/lohnausweis/:year/:employee_id` — returns PDF
- `GET /api/v1/lohnausweis/:year` — returns ZIP of all employee certificates

### Frontend (2 files)
- **Pages**: `frontend/src/pages/LohnausweisPage.tsx` — year selector, employee list, download per employee or all
- Add to Reports hub

---

## Phase 7: Payment Export — pain.001 + Payout Schedule (depends on Phase 4)

### Migrations
- **000068** `create_payout_entries`: `payout_entries` table — id (PK string), payroll_run_id (string FK), employee_id (string FK), amount (Decimal 15,2), iban (string), bic (string nullable), recipient_name (string), recipient_street (string), recipient_postal_code (string), recipient_city (string), recipient_country (string), status (string: "pending" | "exported" | "paid"), paid_at (timestamp nullable), payment_reference (string), created_at (timestamp), updated_at (timestamp)

### Backend (4 files)
- **Entity**: `payout_entry.rs`
- **Repo**: `payout_entry_repo.rs`
- **Service**: `payout_service.rs` — create entries from approved payroll run, mark paid
- **Service**: `pain_001_service.rs` — generate Swiss `pain.001.001.09.ch.03` XML via `quick-xml`

### New dependency
- `quick-xml = "0.37"` in konto-core Cargo.toml

### Routes
- `GET /api/v1/payroll-runs/:id/payout-entries` — list payout entries
- `POST /api/v1/payroll-runs/:id/generate-payouts` — create payout entries from run
- `POST /api/v1/payroll-runs/:id/export-pain001` — returns XML file
- `PUT /api/v1/payout-entries/:id/mark-paid` — mark single entry paid
- `POST /api/v1/payroll-runs/:id/mark-all-paid` — mark all entries paid

### Frontend (3 files)
- **Pages**: `frontend/src/pages/PayoutSchedulePage.tsx` — list payouts per run, export button, mark paid buttons
- **Types**: `frontend/src/types/payout-entry.ts`
- **API**: `frontend/src/api/payout-entries.ts`
- Link from PayrollRunDetailPage

---

## Integration Points (after all phases)

### Navigation Changes
- Settings Hub "Accounting" section: add "Fixed Assets"
- Settings Hub "General" section: add "Employees"
- Settings Hub "Billing" section: add "Payroll Settings"
- Sidebar: add "Payroll" as main navigation item (between Time Entries and Invoices)
- Reports Hub: add "Lohnausweis" card

### Module Registration (all phases)
- `entities/mod.rs`: add all new entities
- `repository.rs`: add all new repos
- `services/mod.rs`: add all new services
- `handlers/mod.rs`: add all new handlers
- `dto/mod.rs`: add all new DTOs
- `router.rs`: add all new routes
- `openapi.rs`: add all new DTOs/handlers to OpenAPI spec
- `migration/lib.rs`: add all new migrations
- `App.tsx`: add all new routes
- `navigation.ts`: add all new navigation items
- `i18n/messages.ts`: add all new i18n domains (en/de/fr/it)

---

## Migration Summary

| # | Name | Phase |
|---|------|-------|
| 000061 | create_fixed_assets | 2 |
| 000062 | create_depreciation_entries | 2 |
| 000063 | create_employees | 3 |
| 000064 | create_payroll_settings | 3 |
| 000065 | seed_payroll_settings | 3 |
| 000066 | create_payroll_runs | 4 |
| 000067 | create_payroll_run_lines | 4 |
| 000068 | create_payout_entries | 7 |

## i18n Domains

- `fixed_assets.*` — asset management labels
- `employees.*` — staff management
- `payroll_settings.*` — rate configuration
- `payroll.*` — payroll runs, pay slips, Lohnausweis
- `payout.*` — payout schedule, pain.001 export

All labels in en/de/fr/it per TD-029.

## File Count Estimate

| Phase | Backend | Frontend | Migrations | Total |
|-------|---------|----------|------------|-------|
| 1 | 1 modified | 0 | 0 | 1 |
| 2 | 8 new | 4 new | 2 | 14 |
| 3 | 10 new | 6 new | 3 | 19 |
| 4 | 8 new | 4 new | 2 | 14 |
| 5 | 2 new | 0 (modify existing) | 0 | 2 |
| 6 | 3 new | 1 new | 0 | 4 |
| 7 | 4 new | 3 new | 1 | 8 |
| **Total** | **~36 new** | **~18 new** | **8** | **~62** |
