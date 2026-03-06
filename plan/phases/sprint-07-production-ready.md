# Sprint 07 — Production-Ready Accounting

## Review of Progress

### Sprints 01–06 — COMPLETE
Delivered: Full foundation (auth, RBAC, 25 DB tables), double-entry bookkeeping with Swiss KMU chart of accounts, 5 financial reports + dashboard, invoice lifecycle with Typst PDF + Swiss QR-bill, word-like block editor for documents, user management, audit log, email, CSV export, data import.

### What's Still Missing vs. Commercial Tools

After researching commercial tools (CH and DE markets), including lexoffice, sevDesk, Run my Accounts, and Abacus/AbaNinja, these features are **standard across all tools** but missing from Hope:

| Gap | Business Impact |
|-----|----------------|
| Recurring invoices | Manual re-creation every month for retainer clients |
| Credit notes | Cannot issue refunds or corrections (Swiss law requires) |
| Payment reminders / Dunning | No systematic follow-up on unpaid invoices |
| Partial payments | Can only mark fully paid, not partial |
| Expenses / Supplier bills (AP) | No accounts payable tracking, manual journal only |
| Bank reconciliation (CAMT.053) | Cannot match bank transactions to invoices/bills |
| Overdue auto-detection | No scheduler to flag overdue invoices |
| Email templates | Hardcoded email body, no customization |
| Contact detail page | No profile view, no history, no sub-contacts UI |
| Project detail page | No budget tracking, time summary, or profitability |
| Time → Invoice conversion | Cannot bill tracked hours |
| VAT/Currency/Activity settings UI | Seeded but no management screens |
| Dashboard charts & trends | Text-only KPIs, no visual charts |
| Cash flow report | Missing standard financial report |
| Document server-side PDF | Placeholder returning JSON, not real PDF |

---

## Sprint 07 Scope

**Theme:** Close the gap between Hope and commercial tools. After this sprint, Hope is a daily-driver accounting tool.

**Organized into 8 phases**, roughly in dependency order.

---

## Phase 1: Recurring Invoices & Credit Notes

### 1.1 Recurring Invoices

**New DB table: `recurring_invoices`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| contact_id | UUID | FK → contacts |
| project_id | UUID? | FK → projects |
| template_data | JSON | Invoice line items, notes, currency, etc. |
| frequency | ENUM | monthly, quarterly, semi_annual, annual, custom |
| interval_days | INT? | For custom frequency |
| next_run_date | DATE | When next invoice should be generated |
| end_date | DATE? | NULL = indefinite |
| auto_send | BOOL | If true, auto-send on generation |
| is_active | BOOL | Pause/resume |
| last_generated_at | TIMESTAMP? | |
| created_at / updated_at | TIMESTAMP | |

**Backend:**
- `RecurringInvoiceService` — CRUD + `generate_due_invoices(db)` method
- `RecurringInvoiceHandler` — 6 endpoints: list, create, get, update, delete, trigger (manual run)
- `RecurringInvoiceScheduler` — `tokio::spawn` background task with configurable interval (default: hourly check), calls `generate_due_invoices`
- On generation: creates invoice in draft (or sends if `auto_send`), updates `next_run_date`, logs audit

**Frontend:**
- `/invoices/recurring` → `RecurringInvoicesPage.tsx` — list with active/paused filter
- Create/edit dialog reusing `InvoiceForm.tsx` fields + frequency picker
- "Create Recurring" button on invoice detail (pre-fill from existing invoice)

### 1.2 Credit Notes

**New DB table: `credit_notes`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| credit_note_number | VARCHAR | GS-YYYY-NNN format |
| invoice_id | UUID? | FK → invoices (linked original) |
| contact_id | UUID | FK → contacts |
| status | ENUM | draft, issued, applied, cancelled |
| issue_date | DATE | |
| currency_id | UUID | FK → currencies |
| notes | TEXT? | |
| created_at / updated_at | TIMESTAMP | |

**New DB table: `credit_note_lines`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| credit_note_id | UUID | FK → credit_notes |
| description | VARCHAR | |
| quantity | DECIMAL | |
| unit_price | DECIMAL | |
| vat_rate_id | UUID? | FK → vat_rates |
| account_id | UUID | FK → accounts (revenue reversal) |
| sort_order | INT | |

**Backend:**
- `CreditNoteService` — CRUD + `issue(id)` (assigns number, creates reversing journal entry: credit Debitoren, debit revenue accounts)
- `CreditNoteHandler` — list, create, get, update, delete, issue, pdf
- `credit_note_workflow.rs` — issue and apply workflows
- PDF: reuse Typst template from invoices, adapted header "Credit Note / Gutschrift"
- "Create Credit Note" action on invoice detail → pre-fills from invoice lines

**Frontend:**
- `/credit-notes` → `CreditNotesPage.tsx`
- `/credit-notes/new` → `CreditNoteCreatePage.tsx`
- `/credit-notes/:id` → `CreditNoteDetailPage.tsx`
- Sidebar nav item under Invoicing group

---

## Phase 2: Payment Reminders (Dunning)

### 2.1 Dunning Configuration

**New DB table: `dunning_levels`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| level | INT | 1, 2, 3 |
| days_after_due | INT | e.g. 14, 30, 45 |
| fee_amount | DECIMAL | Dunning fee (e.g. 0, 10, 20 CHF) |
| subject_template | TEXT | Email subject with {{variables}} |
| body_template | TEXT | Email body with {{variables}} |
| is_active | BOOL | |

Seed 3 default levels:
- Level 1: 14 days, 0 CHF fee, friendly reminder
- Level 2: 30 days, 10 CHF fee, second reminder
- Level 3: 45 days, 20 CHF fee, final warning

### 2.2 Dunning History

**New DB table: `dunning_entries`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| invoice_id | UUID | FK → invoices |
| dunning_level_id | UUID | FK → dunning_levels |
| sent_at | TIMESTAMP | |
| fee_amount | DECIMAL | Actual fee charged |
| email_sent | BOOL | |
| notes | TEXT? | |

**Backend:**
- `DunningService` — `get_overdue_invoices()`, `create_reminder(invoice_id, level)`, `run_dunning()` (batch process)
- `DunningHandler` — list dunning history per invoice, manual send reminder, run dunning batch, CRUD dunning levels
- `DunningScheduler` — daily check via `tokio::spawn`, finds invoices past due + level threshold, creates `dunning_entries`, optionally sends email
- Dunning fee creates journal entry (debit Debitoren, credit 3900 Mahngebühren)

**Frontend:**
- `/settings/dunning` → `DunningSettingsPage.tsx` — configure 3 levels (days, fee, email templates)
- Dunning badge on invoice list (shows current dunning level)
- "Send Reminder" button on invoice detail with level selection
- `/invoices/:id` dunning history tab showing sent reminders

---

## Phase 3: Expenses & Supplier Bills (Accounts Payable)

### 3.1 Expenses

**New DB table: `expenses`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| expense_number | VARCHAR | EX-YYYY-NNN |
| contact_id | UUID? | FK → contacts (supplier) |
| category | VARCHAR | e.g. office, travel, software, etc. |
| description | TEXT | |
| amount | DECIMAL | Total amount |
| currency_id | UUID | FK → currencies |
| vat_rate_id | UUID? | FK → vat_rates |
| expense_date | DATE | |
| due_date | DATE? | |
| status | ENUM | pending, approved, paid, cancelled |
| payment_account_id | UUID? | FK → accounts (bank/cash account used) |
| receipt_url | VARCHAR? | Uploaded receipt file path |
| project_id | UUID? | FK → projects |
| journal_entry_id | UUID? | FK → journal_entries (created on approve) |
| created_at / updated_at | TIMESTAMP | |

**New DB table: `expense_categories`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| name | VARCHAR | |
| account_id | UUID | FK → accounts (default expense account) |
| is_active | BOOL | |

Seed categories: Office Supplies (4400), Software & SaaS (4410), Travel (4600), Marketing (4700), Professional Services (4500), Telecommunications (4420), Insurance (4900), Misc (4800).

**Backend:**
- `ExpenseService` — CRUD + `approve(id)` (creates journal entry: debit expense account, credit Kreditoren 2000 or bank), `pay(id)` (debit Kreditoren, credit bank), `upload_receipt(id, file)`
- `ExpenseHandler` — list (with filters: status, category, date range, supplier), create, get, update, delete, approve, pay, receipt upload
- `ExpenseCategoryHandler` — CRUD for categories
- CSV export via `?format=csv`

**Frontend:**
- `/expenses` → `ExpensesPage.tsx` — table with status/category filters, CSV export
- `/expenses/new` → `ExpenseCreatePage.tsx` — form with receipt upload
- `/expenses/:id` → `ExpenseDetailPage.tsx` — approve/pay actions, receipt preview
- `/settings/expense-categories` → `ExpenseCategoriesPage.tsx`
- Sidebar: new "Expenses" item in Finance group

---

## Phase 4: Bank Reconciliation

### 4.1 Bank Statement Import (CAMT.053)

**New DB table: `bank_transactions`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| bank_account_id | UUID | FK → bank_accounts |
| transaction_date | DATE | |
| value_date | DATE | |
| amount | DECIMAL | Positive = credit, negative = debit |
| currency_id | UUID | FK → currencies |
| description | TEXT | Bank description/reference |
| counterparty_name | VARCHAR? | |
| counterparty_iban | VARCHAR? | |
| reference | VARCHAR? | Payment reference (SCOR, etc.) |
| bank_reference | VARCHAR? | Bank's internal reference |
| status | ENUM | unmatched, matched, ignored |
| matched_invoice_id | UUID? | FK → invoices |
| matched_expense_id | UUID? | FK → expenses |
| matched_journal_entry_id | UUID? | FK → journal_entries |
| import_batch_id | UUID? | |
| created_at | TIMESTAMP | |

**Backend:**
- `BankImportService` — parse CAMT.053 XML (ISO 20022), create `bank_transactions`
- `BankReconciliationService`:
  - `auto_match(bank_account_id)` — match by SCOR reference (exact), amount + date (fuzzy), counterparty IBAN
  - `manual_match(transaction_id, target_type, target_id)` — user assigns match
  - `create_journal_entry(transaction_id)` — for unmatched transactions, create manual journal entry
  - `ignore(transaction_id)` — mark as ignored (e.g. internal transfers)
- `BankReconciliationHandler` — upload CAMT.053, list transactions (filter by status/date/account), auto-match, manual match, ignore, create entry
- On match to invoice: auto-trigger `InvoiceWorkflow::pay()`
- On match to expense: auto-trigger `ExpenseService::pay()`

**New dependency:**
```toml
quick-xml = "0.37" # CAMT.053 XML parsing
```

**Frontend:**
- `/banking` → `BankingPage.tsx` — bank account selector, transaction list with match status
- `/banking/import` → `BankImportPage.tsx` — upload CAMT.053, preview transactions
- `/banking/reconcile` → `ReconciliationPage.tsx`:
  - Left panel: unmatched bank transactions
  - Right panel: open invoices/expenses
  - Drag-drop or click to match
  - "Auto-Match" button for batch matching
  - "Create Entry" for manual journal creation from transaction
- Sidebar: new "Banking" top-level item

---

## Phase 5: Overdue Detection & Email Templates

### 5.1 Invoice Overdue Scheduler

- Background task via `tokio::spawn` (runs daily at configurable time)
- Query: `SELECT * FROM invoices WHERE status = 'sent' AND due_date < CURRENT_DATE`
- Update status to `overdue`, create audit log entry
- Optionally trigger dunning level 1 if dunning is configured

### 5.2 Email Templates

**New DB table: `email_templates`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| template_type | ENUM | invoice_send, invoice_reminder_1, invoice_reminder_2, invoice_reminder_3, credit_note, document_send |
| subject | TEXT | With {{variables}} |
| body_html | TEXT | With {{variables}} |
| language | VARCHAR | de, en, fr, it |
| is_default | BOOL | |
| created_at / updated_at | TIMESTAMP | |

**Variables:** `{{company_name}}`, `{{contact_name}}`, `{{invoice_number}}`, `{{amount}}`, `{{due_date}}`, `{{document_number}}`, `{{credit_note_number}}`

Seed defaults for DE and EN (invoice send, 3 reminder levels, credit note, document send).

**Backend:**
- `EmailTemplateService` — CRUD + `render(template_type, language, variables)`
- Update `EmailService::send_invoice_email` to use templates
- `EmailTemplateHandler` — list, get, update, preview (render with sample data)

**Frontend:**
- `/settings/email-templates` → `EmailTemplatesPage.tsx` — list by type, edit with live preview
- Variable insertion toolbar in template editor

---

## Phase 6: Contact & Project Enhancements

### 6.1 Contact Detail Page

**Frontend:**
- `/contacts/:id` → `ContactDetailPage.tsx`:
  - Header: name, company, contact info, edit button
  - Tabs:
    - **Overview** — address, email, phone, website, VAT number, notes
    - **Invoices** — list of invoices for this contact (linked from invoices table)
    - **Documents** — list of documents for this contact
    - **Time Entries** — time tracked for this contact
    - **Expenses** — expenses from this supplier
    - **Activity** — audit log filtered by entity
  - Quick actions: create invoice, create document, create time entry

**Backend:**
- `ContactPersonHandler` — CRUD for contact_persons (table exists, no API yet)
- Add `GET /contacts/{id}/invoices`, `GET /contacts/{id}/documents`, `GET /contacts/{id}/time-entries` convenience endpoints

### 6.2 Contact Tags & Notes

**New DB table: `contact_tags`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| name | VARCHAR | e.g. "customer", "supplier", "partner" |
| color | VARCHAR | Hex color for badge |

**New DB table: `contact_tag_assignments`**
| Column | Type | Notes |
|--------|------|-------|
| contact_id | UUID | FK → contacts |
| tag_id | UUID | FK → contact_tags |
| PK | (contact_id, tag_id) | Composite |

**New column on contacts:** `notes TEXT`

### 6.3 Project Detail Page

**Frontend:**
- `/projects/:id` → `ProjectDetailPage.tsx`:
  - Header: name, client, status, dates
  - KPI cards: total hours, billable hours, budget remaining, invoiced amount
  - Tabs:
    - **Time Entries** — filtered time entries for this project
    - **Invoices** — invoices linked to this project
    - **Documents** — documents linked to this project
    - **Expenses** — expenses linked to this project

**Backend:**
- `ProjectService` additions: `get_project_summary(id)` returning hours/budget/invoiced totals
- Add budget fields to projects table: `budget_hours DECIMAL?`, `budget_amount DECIMAL?`
- Add `DELETE /projects/{id}` endpoint (soft delete via status)

### 6.4 Time → Invoice Conversion

**Backend:**
- `POST /api/v1/invoices/from-time-entries` — accepts list of time_entry_ids, creates invoice with line items derived from time entries (description, hours × hourly rate)
- Mark time entries as `billed = true` (new column on `time_entries`)
- Prevent double-billing (reject already-billed entries)

**Frontend:**
- Checkbox column on time entries list
- "Create Invoice from Selected" button → opens invoice create with pre-filled lines
- Filter: "unbilled only" toggle on time entries page

---

## Phase 7: Dashboard & Reporting Upgrades

### 7.1 Dashboard Charts

**New frontend dependency:**
```
recharts (lightweight React charting)
```

**Dashboard enhancements:**
- **Revenue chart** — monthly revenue bar chart (last 12 months)
- **Expense chart** — monthly expenses bar chart (last 12 months)
- **Cash flow mini-chart** — line chart showing bank balance trend
- **Invoice aging donut** — draft / sent / overdue / paid distribution
- **Outstanding by customer** — top 5 customers by outstanding amount
- **Quick actions** — create invoice, record expense, new time entry

**Backend:**
- `DashboardService` additions:
  - `get_monthly_revenue(months: u32)` — revenue by month
  - `get_monthly_expenses(months: u32)` — expenses by month
  - `get_invoice_aging()` — count/amount by status
  - `get_top_outstanding_contacts(limit)` — top contacts by unpaid invoices

### 7.2 Cash Flow Report

**Backend:**
- `ReportService` addition: `cash_flow_report(from_date, to_date)`
- Categories: Operating (revenue collected, expenses paid), Investing, Financing
- Derived from journal entries on cash/bank accounts (1000, 1010, 1020)

**Frontend:**
- `/reports/cash-flow` → `CashFlowPage.tsx`
- Three sections: Operating / Investing / Financing with net change
- Date range filter, CSV export

### 7.3 Accounts Receivable & Payable Aging

**Backend:**
- `ReportService` additions:
  - `ar_aging_report()` — invoices grouped by age bucket (current, 1-30, 31-60, 61-90, 90+)
  - `ap_aging_report()` — expenses grouped by age bucket

**Frontend:**
- `/reports/ar-aging` → `ArAgingPage.tsx` — table with buckets + totals
- `/reports/ap-aging` → `ApAgingPage.tsx` — table with buckets + totals

### 7.4 Comparative Reports

- Add `comparison_period` parameter to P&L and Balance Sheet endpoints
- Returns current period + comparison period side by side with variance (amount + %)

---

## Phase 8: Settings & Polish

### 8.1 VAT Rate Management

**Backend:**
- `VatRateHandler` — full CRUD (currently only read)
- Ability to add custom rates, deactivate old rates

**Frontend:**
- `/settings/vat-rates` → `VatRatesPage.tsx` — table with create/edit/deactivate

### 8.2 Currency Management

**Backend:**
- `CurrencyHandler` — full CRUD (currently only read)

**Frontend:**
- `/settings/currencies` → `CurrenciesPage.tsx` — table with create/edit

### 8.3 Activity Type Management

**Backend:**
- `ActivityTypeHandler` — full CRUD (table exists, no API)

**Frontend:**
- `/settings/activity-types` → `ActivityTypesPage.tsx` — table with create/edit

### 8.4 Partial Payments on Invoices

**New DB table: `invoice_payments`**
| Column | Type | Notes |
|--------|------|-------|
| id | UUID | PK |
| invoice_id | UUID | FK → invoices |
| amount | DECIMAL | Payment amount |
| payment_date | DATE | |
| payment_method | VARCHAR? | bank_transfer, cash, card |
| reference | VARCHAR? | Bank reference |
| bank_transaction_id | UUID? | FK → bank_transactions (if reconciled) |
| journal_entry_id | UUID? | FK → journal_entries |
| created_at | TIMESTAMP | |

**Backend:**
- `InvoiceWorkflow` update: `record_payment(invoice_id, amount, date)` — creates partial payment, journal entry (debit bank, credit Debitoren for amount), auto-marks paid when total payments >= invoice total
- `GET /invoices/{id}/payments` — list payments
- Invoice response gains: `amount_paid`, `amount_remaining`

**Frontend:**
- "Record Payment" button on invoice detail → amount + date dialog
- Payment history list on invoice detail
- Progress bar showing paid/remaining

### 8.5 Sidebar & Navigation Restructure

Reorganize sidebar for the expanded feature set:

```
Dashboard
─────────────
SALES
  Invoices
  Recurring Invoices
  Credit Notes
  Quotes & Documents
─────────────
FINANCE
  Journal Entries
  Expenses
  Banking
─────────────
CRM
  Contacts
  Projects
  Time Tracking
─────────────
REPORTS
  Trial Balance
  Balance Sheet
  Profit & Loss
  Cash Flow
  VAT Report
  AR Aging
  AP Aging
  Account Ledger
─────────────
SETTINGS
  Company
  Bank Accounts
  Fiscal Years
  VAT Rates
  Currencies
  Exchange Rates
  Activity Types
  Expense Categories
  Dunning
  Email Settings
  Email Templates
  Templates & Letterhead
  Users
  Audit Log
```

### 8.6 Document Server-Side PDF

- Replace placeholder `get_document_pdf` endpoint with real Typst-based rendering
- Reuse `typst-as-lib` from invoice PDF, new template for documents
- Render block content (headings, paragraphs, tables, signatures) to Typst markup
- Header/footer from template, company logo from settings

---

## New Database Tables Summary

| # | Table | Phase |
|---|-------|-------|
| 1 | recurring_invoices | 1 |
| 2 | credit_notes | 1 |
| 3 | credit_note_lines | 1 |
| 4 | dunning_levels | 2 |
| 5 | dunning_entries | 2 |
| 6 | expenses | 3 |
| 7 | expense_categories | 3 |
| 8 | bank_transactions | 4 |
| 9 | email_templates | 5 |
| 10 | contact_tags | 6 |
| 11 | contact_tag_assignments | 6 |
| 12 | invoice_payments | 8 |

**Total after Sprint 07: 37 tables** (25 existing + 12 new)

---

## New Migrations

| # | Migration | Description |
|---|-----------|-------------|
| 000017 | recurring_invoices | Recurring invoice schedules |
| 000018 | credit_notes + credit_note_lines | Credit note tables |
| 000019 | dunning_levels + dunning_entries | Dunning configuration and history |
| 000020 | expenses + expense_categories | Expense tracking with categories |
| 000021 | bank_transactions | Bank statement import and reconciliation |
| 000022 | email_templates | Customizable email templates |
| 000023 | contact_tags + contact_tag_assignments | Contact tagging |
| 000024 | invoice_payments | Partial payment tracking |
| 000025 | project_budget_fields | Add budget_hours, budget_amount to projects |
| 000026 | time_entry_billed | Add billed flag to time_entries |
| 000027 | contact_notes | Add notes column to contacts |

---

## New API Endpoints Summary

| Phase | Endpoints | Count |
|-------|-----------|-------|
| 1 — Recurring Invoices | list, create, get, update, delete, trigger | 6 |
| 1 — Credit Notes | list, create, get, update, delete, issue, pdf | 7 |
| 2 — Dunning | list levels, update level, dunning history, send reminder, run batch | 5 |
| 3 — Expenses | list, create, get, update, delete, approve, pay, receipt upload | 8 |
| 3 — Expense Categories | list, create, update, delete | 4 |
| 4 — Banking | upload CAMT, list transactions, auto-match, manual match, ignore, create entry | 6 |
| 5 — Email Templates | list, get, update, preview | 4 |
| 6 — Contact Persons | list, create, update, delete | 4 |
| 6 — Contact Tags | list, create, delete | 3 |
| 6 — Contact sub-resources | invoices, documents, time-entries per contact | 3 |
| 6 — Project summary + delete | summary, delete | 2 |
| 6 — Time → Invoice | create invoice from time entries | 1 |
| 7 — Dashboard charts | monthly revenue, monthly expenses, invoice aging, top outstanding | 4 |
| 7 — Reports | cash flow, AR aging, AP aging, comparative P&L, comparative BS | 5 |
| 8 — VAT Rates | create, update, deactivate | 3 |
| 8 — Currencies | create, update | 2 |
| 8 — Activity Types | list, create, update, delete | 4 |
| 8 — Invoice Payments | list, create | 2 |

**Total new endpoints: ~73**

---

## New Frontend Pages Summary

| Page | Route |
|------|-------|
| RecurringInvoicesPage | /invoices/recurring |
| CreditNotesPage | /credit-notes |
| CreditNoteCreatePage | /credit-notes/new |
| CreditNoteDetailPage | /credit-notes/:id |
| DunningSettingsPage | /settings/dunning |
| ExpensesPage | /expenses |
| ExpenseCreatePage | /expenses/new |
| ExpenseDetailPage | /expenses/:id |
| ExpenseCategoriesPage | /settings/expense-categories |
| BankingPage | /banking |
| BankImportPage | /banking/import |
| ReconciliationPage | /banking/reconcile |
| EmailTemplatesPage | /settings/email-templates |
| ContactDetailPage | /contacts/:id |
| ProjectDetailPage | /projects/:id |
| CashFlowPage | /reports/cash-flow |
| ArAgingPage | /reports/ar-aging |
| ApAgingPage | /reports/ap-aging |
| VatRatesPage | /settings/vat-rates |
| CurrenciesPage | /settings/currencies |
| ActivityTypesPage | /settings/activity-types |

**Total new pages: 21**

---

## Estimated New Files

| Area | New Files |
|------|-----------|
| Backend entities | ~10 |
| Backend repositories | ~10 |
| Backend services | ~15 |
| Backend handlers | ~12 |
| Backend DTOs | ~10 |
| Migrations | 11 |
| Frontend pages | 21 |
| Frontend API hooks | ~8 |
| Frontend components | ~15 |
| Frontend types | ~6 |

**Total: ~118 new files**

---

## Deferred to Sprint 08+

| Feature | Reason |
|---------|--------|
| Payroll / Swissdec | Major domain, needs separate sprint |
| OCR receipt scanning | Needs ML service or 3rd party API |
| Direct bank API connection | Requires bank partnerships / finAPI integration |
| eBill (Swiss e-banking delivery) | Needs bank integration partner |
| Multi-language documents | UI complexity, low priority for single-user |
| Inventory / stock management | Not needed for service company |
| Purchase orders (PO) | Can be handled via expenses for now |
| Customer portal | External-facing feature, separate sprint |
| Granular RBAC (permission matrix) | Current role-based system sufficient for < 10 users |
| Mobile app | Responsive web sufficient initially |
