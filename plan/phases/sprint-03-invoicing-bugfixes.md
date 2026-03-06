# Sprint 03 — Invoicing & Quality Fixes

## Review of Progress

### Sprint 01 (Foundation) — COMPLETE
Delivered: Monorepo scaffold, 17-table schema, JWT+RBAC auth, REST API with OpenAPI docs, CSV/XLSX import engine, full frontend shell with sidebar/topbar/mobile nav, pages for Dashboard, Accounts, Contacts, Journal, Projects, Time Entries, and Import Wizard.

### Sprint 02 (Financial Core & Reporting) — COMPLETE
Delivered: 5 financial reports (Trial Balance, Balance Sheet, P&L, Account Ledger, VAT), fiscal year/period management with close workflow, exchange rate management, journal post/reverse workflow, dashboard KPIs, pagination component.

### Known Issues Discovered in Code Review
The following bugs exist from Sprint 01/02 where **frontend types don't match backend API responses**:

| Area | Problem |
|------|---------|
| Dashboard | Backend returns `account_count`, frontend expects `total_accounts` (and similar for other fields) |
| Trial Balance | Backend: `total_debit`/`total_credit`, Frontend: `debit_total`/`credit_total` |
| Profit & Loss | Backend: `net_income`, Frontend: `net_result` |
| VAT Report | Backend returns flat `Vec<VatReportRow>`, Frontend expects `{ entries, total_revenue, total_vat_owed }` |
| Account Ledger | Backend: `entry_id` + params `from_date`/`to_date`, Frontend: `journal_entry_id` + `from`/`to` |
| Exchange Rates | Frontend sends currency codes ("CHF"), backend expects UUIDs |
| Time Entries | Table shows raw `project_id` (UUID) instead of project name |
| Sidebar | Account Ledger route exists but is missing from sidebar navigation |
| Pagination | Contacts, Projects, Time Entries pages have no pagination controls |

---

## Sprint 03 Goals

**Theme:** Fix production-blocking bugs, then deliver Invoice Management — the single most important missing feature for daily business operations.

---

## Part A — Bug Fixes & Quality (Priority: Critical)

These must be fixed before any new feature work since they make existing pages non-functional.

### A1. Fix API Contract Mismatches
- [x] **Dashboard**: Aligned frontend `DashboardStats` type with backend response fields (`account_count`, `journal_entry_count`)
- [x] **Trial Balance**: Fixed `debit_total`/`credit_total` → `total_debit`/`total_credit` in frontend
- [x] **Profit & Loss**: Fixed `net_result` → `net_income` in frontend type + page
- [x] **VAT Report**: Wrapped backend response in `VatReportResponse { entries, total_revenue, total_vat_owed, from_date, to_date }`
- [x] **Account Ledger**: Fixed `journal_entry_id` → `entry_id` and query param names, fixed URL to `/reports/ledger/${account_id}`
- [x] **Exchange Rates**: Backend now resolves 3-char currency codes to UUIDs via `resolve_currency_id()` helper

### A2. Fix UI Gaps
- [x] **Sidebar**: Added Account Ledger to `reportsNav` in Sidebar.tsx
- [x] **Time Entries table**: Resolves project_id to project name via projectMap
- [x] **Pagination**: Wired pagination on Contacts, Projects, and Time Entries pages

### A3. End-to-End Smoke Test
- [ ] Start backend + frontend, log in, verify every page renders without errors
- [ ] Test data import with actual files from `plan/supporting-docs/`
- [ ] Verify all 5 reports render correct data after import

---

## Part B — Invoice Management (Priority: High)

The core feature that makes Hope usable as a daily accounting tool. Invoices are the primary way companies bill clients.

### B1. Database Schema (new tables)

```
invoices
├── id              UUID PK
├── invoice_number  TEXT UNIQUE NOT NULL  (auto-generated, e.g. "RE-2025-001")
├── contact_id      UUID FK → contacts
├── project_id      UUID FK → projects (nullable)
├── status          TEXT (draft / sent / paid / overdue / cancelled)
├── issue_date      DATE NOT NULL
├── due_date        DATE NOT NULL
├── currency_id     UUID FK → currencies
├── subtotal        DECIMAL(19,4)
├── vat_amount      DECIMAL(19,4)
├── total           DECIMAL(19,4)
├── notes           TEXT
├── payment_terms   TEXT
├── bexio_id        INTEGER (nullable, for legacy import)
├── created_by      UUID FK → users
├── created_at      TIMESTAMP
├── updated_at      TIMESTAMP

invoice_lines
├── id              UUID PK
├── invoice_id      UUID FK → invoices (CASCADE DELETE)
├── position        INTEGER NOT NULL
├── description     TEXT NOT NULL
├── quantity         DECIMAL(19,4) NOT NULL
├── unit_price      DECIMAL(19,4) NOT NULL
├── vat_rate_id     UUID FK → vat_rates (nullable)
├── vat_amount      DECIMAL(19,4)
├── line_total      DECIMAL(19,4)
├── account_id      UUID FK → accounts (revenue account for booking)
├── created_at      TIMESTAMP
├── updated_at      TIMESTAMP
```

### B2. Backend — Invoice CRUD + Workflow
- [x] **Migration**: Created `invoices` + `invoice_lines` tables (`m20240101_000010_create_invoices.rs`)
- [x] **SeaORM entities**: `invoice.rs` + `invoice_line.rs` with full relations
- [x] **Repository**: `invoice_repo.rs` with paginated find, CRUD, line ops, next_invoice_number
- [x] **DTOs**: `invoice.rs` with request/response structs + From impls
- [x] **Invoice service** (konto-core): Split into `invoice_service.rs` (CRUD) + `invoice_workflow.rs` (send/pay/cancel)
  - [x] `create_invoice` — create draft with lines, compute totals
  - [x] `update_invoice` — only if status=draft
  - [x] `get_by_id` — with lines, contact name, project name
  - [x] `list` — paginated, filterable by status/contact/search
  - [x] `delete` — only if status=draft
  - [x] `send_invoice` — draft→sent, assigns invoice number `RE-YYYY-NNN`, creates journal entry (debit 1100, credit revenue per line)
  - [x] `mark_paid` — sent→paid, creates payment journal entry (debit bank, credit 1100)
  - [x] `cancel_invoice` — sent/overdue→cancelled, creates reversing journal entry
- [x] **API handlers** (8 endpoints): create, list, get, update, delete, send, pay, cancel
- [x] **OpenAPI annotations** on all handlers with utoipa

### B3. Frontend — Invoice Pages
- [x] **InvoicesPage** (`/invoices`): Table with status filter tabs (All/Draft/Sent/Paid/Overdue/Cancelled), search, pagination
- [x] **InvoiceCreatePage** (`/invoices/new`): Multi-line form with Save as Draft / Save & Send
  - Contact selector, Project selector (optional), date fields, notes, payment terms
  - Dynamic line items: description, quantity, unit price, VAT rate, account
  - Auto-calculated: line totals, subtotal, VAT, grand total
- [x] **InvoiceDetailPage** (`/invoices/{id}`): Read-only view with status-based action buttons, pay dialog, linked journal entries
- [x] **InvoiceEditPage** (`/invoices/{id}/edit`): Pre-filled form, draft only
- [x] **InvoiceForm** component: Reusable form extracted to `components/invoice/InvoiceForm.tsx`
- [x] **Sidebar**: Added Invoices with ReceiptText icon after Journal
- [x] **TanStack Query hooks**: All 8 hooks (useInvoices, useInvoice, etc.)
- [x] **Dashboard update**: Added open_invoices_count + total_outstanding KPIs + New Invoice quick action

### B4. Invoice Number Generation
- [x] Auto-increment per fiscal year: `RE-{YYYY}-{NNN}` (e.g., RE-2026-001)
- [x] Assigned on send (not on draft creation) — `next_invoice_number()` in repo

---

## Out of Scope for Sprint 03
The following are planned for future sprints:
- PDF generation / export
- Email sending
- QR-Bill (Swiss QR invoice)
- Invoice layout designer
- Recurring invoices
- Bank sync / CAMT import
- Document management
- Expense tracking
- Employee management
- User management UI

---

## Deliverables Checklist

### Part A — Bug Fixes
- [x] All 6 API contract mismatches fixed
- [x] All 3 UI gaps fixed
- [ ] Smoke test passes on all pages

### Part B — Invoicing
- [x] 2 new database tables (invoices, invoice_lines)
- [x] 8 API endpoints with OpenAPI docs
- [x] Invoice lifecycle: draft → sent → paid (with journal entry generation)
- [x] 4 frontend pages (list, create, detail, edit)
- [x] Dashboard updated with invoice KPIs
- [x] Sidebar navigation updated

### Quality Gates
- [x] Zero TypeScript errors (`npx tsc --noEmit` passes)
- [x] Zero Rust compiler warnings (`cargo build` passes)
- [x] All files under 300 lines (500 max)
- [x] Audit log wired on all invoice mutations
