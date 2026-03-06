# Sprint 02 - Financial Core & Reporting

## Goal
Complete the accounting engine with financial reports, fiscal year management, exchange rates, dashboard with live data, journal workflow (post/reverse), and full CRUD for all entities. Makes Hope usable for any Swiss company's accountant and auditor.

## Deliverables

### 1. Infrastructure Fixes (Housekeeping)
- [x] Migration: fiscal_years, fiscal_periods, exchange_rates tables
- [x] Migration: Seed missing VAT codes (UN77, VB77, VIM, VM77, VM81, VSF)
- [x] Entities + repos for fiscal_years, fiscal_periods, exchange_rates
- [x] Wire audit logging into all mutation handlers
- [x] Time Entry full CRUD (POST, PUT, DELETE)
- [x] Frontend pagination component + wired into list pages

### 2. Financial Reports (Core Feature)
- [x] Report service: trial balance, balance sheet, P&L, account ledger, VAT report
- [x] Report DTOs and handler endpoints
- [x] Frontend: ReportsPage (hub)
- [x] Frontend: TrialBalancePage
- [x] Frontend: BalanceSheetPage
- [x] Frontend: ProfitLossPage
- [x] Frontend: AccountLedgerPage
- [x] Frontend: VatReportPage

### 3. Fiscal Year & Period Management
- [x] Fiscal year service with auto-generated monthly periods
- [x] Close fiscal year (generates closing journal entry 2979 → 2970)
- [x] API endpoints: CRUD + close
- [x] Frontend: FiscalYearsPage (under Settings)

### 4. Exchange Rate Management
- [x] Exchange rate service with latest rate lookup
- [x] API endpoints: CRUD + latest
- [x] Frontend: ExchangeRatesPage (under Settings)

### 5. Dashboard Rewrite
- [x] Dashboard service: aggregated KPIs (accounts, contacts, entries, projects, revenue/expenses MTD, cash balance)
- [x] API endpoint: GET /dashboard/stats
- [x] Frontend: Dashboard rewrite with live data

### 6. Journal Entry Workflow
- [x] Post entry (draft → posted, locks entry)
- [x] Reverse entry (creates reversing entry)
- [x] API endpoints: POST /{id}/post, POST /{id}/reverse
- [x] Frontend: Status badges, Post/Reverse buttons

## New Database Tables

| Table | Key Columns |
|-------|-------------|
| `fiscal_years` | id, name, start_date, end_date, status (open/closed) |
| `fiscal_periods` | id, fiscal_year_id FK, name, start_date, end_date, period_number, status |
| `exchange_rates` | id, from_currency_id FK, to_currency_id FK, rate, valid_date, source |

## New VAT Codes (seeded)
- UN77: Umsatzsteuer Nicht geschuldet 7.7%
- VB77: Vorsteuer auf Betriebsaufwand 7.7%
- VIM: Vorsteuer auf Investitionen/Material
- VM77: Vorsteuer auf Materialaufwand 7.7%
- VM81: Vorsteuer auf Materialaufwand 8.1%
- VSF: Vorsteuer pauschal

## API Endpoints (new)
All under `/api/v1/`:
- Fiscal Years: CRUD + close
- Exchange Rates: CRUD + latest
- Reports: trial-balance, balance-sheet, profit-loss, account-ledger, vat-report
- Dashboard: stats
- Journal: post, reverse
- Time Entries: create, update, delete

## Backend Status
- [x] Migrations (2 new)
- [x] Entities (3 new)
- [x] Repositories (3 new)
- [x] Services (5 new + 2 updated)
- [x] DTOs (4 new + 1 updated)
- [x] Handlers (4 new + 2 updated)
- [x] Router (all new routes)
- [x] Audit wiring (all mutation handlers)
- [x] OpenAPI/Swagger updated
- [x] Clean compilation (zero warnings)

## Frontend Status
- [x] Type definitions (4 new: fiscal-year, exchange-rate, report, dashboard)
- [x] API modules (5 new: fiscal-years, exchange-rates, reports, dashboard, time-entries updated)
- [x] useApi hooks (17 new hooks for all endpoints)
- [x] Pagination component (reusable with page numbers + ellipsis)
- [x] Report pages (6 new: hub, trial balance, balance sheet, P&L, ledger, VAT)
- [x] Settings pages (2 new: Fiscal Years, Exchange Rates)
- [x] Dashboard rewrite (live KPIs, cash balance, recent entries, quick actions)
- [x] Journal page updates (color status badges, Post/Reverse with confirmation)
- [x] Time entries page updates (full CRUD with dialogs)
- [x] Route updates (App.tsx - 9 new routes)
- [x] Navigation updates (Sidebar: Reports + Settings groups, TopBar titles, MobileNav)
- [x] Extracted components (JournalCreateForm, TimeEntryForm) to keep files < 300 lines
- [x] Utility: formatCHF/formatAmount helpers
- [x] alert-dialog shadcn component installed
- [x] Frontend builds cleanly (zero TypeScript errors)
