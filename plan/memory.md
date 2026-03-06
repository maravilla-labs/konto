# Hope Project Memory

## Project Overview
- Hope (Maravilla Konto) is an open-source accounting tool for Swiss SMEs
- Swiss GmbH requiring double-entry bookkeeping
- Production-ready tool, not a prototype

## Tech Stack
- Backend: Rust + Axum 0.8 + SeaORM 1.1 + SQLite (switchable to PostgreSQL)
- Frontend: React 19 + TypeScript + Vite 6 + Tailwind CSS 4 + shadcn/ui
- Auth: JWT + argon2 + RBAC
- API Docs: utoipa (OpenAPI) + Swagger UI

## Key Architecture Decisions
- UUID primary keys everywhere
- rust_decimal for all money (no floating point)
- Crate layering: server → api → core → db → common
- Mobile-first responsive design
- Audit log for legal compliance
- File size limit: 300 lines (500 max)

## Multilingual Baseline
- Supported language set is fixed to `en`, `de`, `fr`, `it`
- Language normalization exists on backend (`konto-core::services::language`) and frontend (`frontend/src/lib/language.ts`)
- Frontend translation catalog uses domain-key namespaces in `frontend/src/i18n/messages.ts` (e.g. `dashboard.*`, `invoices.*`, `recurring.*`, `invoice_form.*`, `invoice_dialogs.*`, `documents.*`)
- Language-aware fields:
  - contacts.language
  - projects.language
  - invoices.language
  - documents.language
  - company_settings.ui_language
  - users.language
- Output fallback chain (invoice/document): explicit -> project -> contact -> company ui_language -> en
- UI fallback chain:
  - authenticated: user.language -> localStorage -> browser -> en
  - logged out: localStorage -> browser -> en
- Settings-aware formatting:
  - Date rendering in dashboard/invoice/document/recurring flows uses `company_settings.date_format`
  - Number rendering in key list pages uses `company_settings.number_format`

## User Profile
- `users` now includes `language` and `avatar_url`
- Profile endpoints:
  - `GET /api/v1/auth/me`
  - `PUT /api/v1/auth/me`
  - `PUT /api/v1/auth/me/language`
  - `POST /api/v1/auth/me/avatar`
- Avatar files stored in `uploads/avatars/`

## Sample Data Scope
- ~78 accounts in Swiss KMU Kontenrahmen
- VAT codes: UN77, UO77, UO81, US77/23, VB77, VIM, VM77, VM81, VSF
- VAT method: Saldosteuersatz (flat-rate)
- Currencies: CHF (primary), EUR

## Sprint Status
- Sprint 01 (Foundation): Complete
- Sprint 02 (Financial Core & Reporting): Complete
- Sprint 03 (Invoicing & Bug Fixes): Complete
- Sprint 04 (Documents, Settings & Editor): Complete
- Sprint 05 (True Block-First WYSIWYG Editor): Complete
- Sprint 06 (Functional Completeness): Complete
- Sprint 07 (Production-Ready Accounting): Complete — 9 migrations, 13 new entities, 55 pages, 113 routes, 37 sidebar items
- Sprint 08 (Usability & E2E Flows): Complete — 4 migrations, 8 phases, storage abstraction, spreadsheet journal, unified expenses, validation hardening
- Sprint 09 (Jahresrechnung): Complete — 5 migrations, Swiss account grouping engine, Typst PDF (6 files), shareholders CRUD, annual report wizard
- Sprint 10 (Tauri Desktop + Nav Redesign): Complete — Tauri 2.10.2, command palette, sidebar redesign (~14 items), role guards, native features
- Sprint 11 (Contacts Rework & Project Management): Complete — 11 migrations, ~70 new backend files, ~35 new routes, ~30 new frontend files, 7 new domains, 5 technical decisions (TD-033-037)
- Sprint 12 (Year-End Closing, Depreciation, Payroll): Complete — 8 migrations (000061-000068), ~36 new backend files, ~18 new frontend files, fixed year-end closing, depreciation engine, Swiss payroll system (employees, settings, runs, calculation, payslip PDF, salary certificate PDF, pain.001 export), 5 technical decisions (TD-038-042)

## Sprint 04 Notes
- Company settings & bank accounts: singleton settings table, bank_accounts with IBAN/BIC/linked ledger account
- Document templates: template system with content_json (DocumentModel), header/footer overrides, per-type defaults
- Documents: quotes, offers, SOWs, contracts with full workflow (draft→sent→accepted/signed→completed)
- Document numbering: AN-NNNNN format (assigned on send)
- Document conversion: offer→SOW, SOW→contract, SOW→project
- Word-like editor: custom contentEditable (no TipTap/ProseMirror), A4 pagination, WYSIWYG 1:1 with PDF
- Block types: h1, h2, h3, p, blockquote, table, image, divider, placeholder, signature, spacer
- Template system: {{placeholder}} variables + locked blocks (non-editable in document mode)
- Signature block: multi-party columns matching SOW AN-00005 format
- PDF generation: browser print with CSS @page, same pagination algorithm as editor
- 5 seeded templates: letterhead, invoice, SOW, quote, contract
- 5 new migrations (000011-000015), 5 new DB tables (24 total), 23 new API endpoints
- 37 new backend files, 39 new frontend files
- Backend split pattern: document_service.rs (CRUD) + document_workflow.rs (send/accept/reject/convert)

## Sprint 05 Notes
- Block-first WYSIWYG: everything is a block on the A4 page (client info, doc metadata, invoice table, signature)
- New block types: contact_info, doc_meta; table block gains mode (generic/invoice)
- New data types: ContactInfoData, DocMetaData; TableBlockData.mode added
- Block renderers: ContactInfoBlock, DocMetaBlock, InvoiceTableBlock (all forwardRef)
- Adaptive inspector: replaces 3-tab system with single panel routing by block type
- 8 inspector panels: TextPanel, TablePanel, ContactPanel, DocMetaPanel, ImagePanel, SignaturePanel, SimplePanel, Common
- EditorContext interface: contacts, projects, templates, docType passed through editor to inspector
- doc-sync.ts: extractDocMeta(), extractLines(), injectDocMeta(), resolveContactLines(), calcRowTotal()
- DocumentCreatePage/EditPage: no more separate metadata state; extract from blocks on save, inject on load
- SlashMenu: added Client Info, Document Info, Invoice Table items
- Deleted: InspectorFormatTab.tsx, InspectorDocumentTab.tsx, InspectorItemsTab.tsx
- 12 new files, 8 modified files, 3 deleted files

## Sprint 06 Notes
- Invoice PDF: printpdf 0.9 based A4 PDF with company header, line items table, totals, Swiss QR-bill
- Swiss QR-Bill: SPC v0200 payload, SCOR reference (ISO 11649), structured addresses (Type S), qrcode crate for PNG
- PDF service: PdfInvoiceService::generate(db, id) → Vec<u8>, sources data from company_settings, bank_accounts, contacts, invoices
- User Management: CRUD for users, role assignment, password change (admin or self), 4 seeded roles
- Email Service: lettre 0.11 SMTP with TLS/STARTTLS, attachments, email_settings table (migration 000016)
- Invoice email: generates PDF, attaches to email, sends to contact's email address
- Audit Log Viewer: paginated table with filters (entity_type, action, user_id, date range), admin/auditor access
- Consistent Audit Logging: 48 AuditService::log calls across all 15 handlers (every mutation logged)
- CSV Export: ExportService::to_csv<T: Serialize>, ?format=csv query param on list endpoints
- Frontend CSV: downloadCsv() utility in lib/export.ts, buttons on 7 pages (invoices, contacts, journal, 4 reports)
- New API endpoints: 9 (users CRUD, roles, email settings, test email, invoice PDF, invoice email, audit log)
- New DB table: email_settings (25 total)
- New backend files: ~15 (services, handlers, DTOs, entities, repos, migration)
- New frontend files: ~10 (pages, API clients, types, export utility)

## Sprint 07 Notes
- Recurring Invoices: recurring_invoices table, CRUD + generate_due_invoices, hourly background scheduler, frequency (monthly/quarterly/semi_annual/annual/custom)
- Credit Notes: credit_notes + credit_note_lines tables, GS-YYYY-NNN numbering, issue workflow with reversing journal entries, Typst-based PDF
- Dunning: dunning_levels (3 seeded) + dunning_entries tables, 3-level system (14/30/45 days), fees, email sending, daily batch scheduler
- Expenses & AP: expenses + expense_categories tables, EX-YYYY-NNN numbering, approve/pay workflow with journal entries, receipt upload, 8 seeded categories
- Bank Reconciliation: bank_transactions table, CAMT.053 XML import (ISO 20022), auto-match by SCOR reference + amount, manual match, auto-triggers invoice pay/expense pay
- Email Templates: email_templates table, 12 seeded templates (6 types × DE/EN), {{variable}} rendering, live preview editor
- Overdue Detection: daily background scheduler flips sent→overdue invoices past due date
- Contact Enhancements: contact_tags + contact_tag_assignments tables, contact persons CRUD, contact detail page with tabs (overview/persons/invoices/documents/time entries), notes field
- Project Enhancements: budget_hours + budget_amount fields, project detail page with KPI cards, delete endpoint
- Time→Invoice: POST /invoices/from-time-entries, billed flag on time entries, checkbox selection UI
- Dashboard Charts: recharts bar charts (revenue/expenses 12 months), pie chart (invoice aging), top outstanding contacts
- Cash Flow Report: operating/investing/financing sections from cash account journal entries
- AR/AP Aging: bucket reports (current, 1-30, 31-60, 61-90, 90+ days)
- Comparative Reports: P&L and Balance Sheet support comparison period with variance
- Partial Payments: invoice_payments table, record_payment workflow, progress bar UI, auto-mark paid when fully paid
- Settings: VAT rates CRUD, currencies CRUD, activity types CRUD, expense categories CRUD
- Sidebar: fully restructured (Dashboard, Sales, Finance, CRM, Reports, Settings, Data groups)
- Document PDF: Typst-based server-side rendering replacing JSON placeholder, handles all block types
- New DB tables: recurring_invoices, credit_notes, credit_note_lines, dunning_levels, dunning_entries, expenses, expense_categories, bank_transactions, email_templates, contact_tags, contact_tag_assignments, invoice_payments + time_entry_billed column
- 25 total migrations (000001-000027, gaps at 024/025), 38 entities, 43 services, 33 handlers, 55 pages, 113 routes, 37 sidebar items, 29 type files

## Sprint 08 Notes (Complete)
- Branding: "Hope" → "Maravilla Konto", sidebar shows company name dynamically
- TopBar: Made sticky, added breadcrumb-style category navigation for all pages
- Fixed: RecurringInvoiceDialog SelectItem empty value crash, missing DialogDescription
- Fixed: InvoicesPage showing contact_id UUID → resolved to name via client-side join
- Fixed: ExpensesPage showing category_id UUID → resolved to name, added contact column
- TD-014: Unified Expense System (single + report), TD-015: Spreadsheet Journal Grid
- TD-016: Abstract File Storage (StorageTrait), TD-017: Default Account Configuration
- Phase 1: Invoice UX fixes, cross-module navigation (contact→invoice, project→invoice), payment terms
- Phase 2: Default accounts table seeded with Swiss KMU defaults, settings page, wired into workflows
- Phase 3: Chart of accounts tree with Swiss KMU class grouping, balances, search, clickable links
- Phase 4: Full-page journal create (spreadsheet grid), journal detail page, AccountSelect autocomplete, StorageService trait with LocalStorage, journal attachments (upload/download/delete)
- Phase 5: Unified expense system — expense_type (single/report), expense_lines (7-category grid), expense_receipts via StorageService, type filter tabs on list page
- Phase 6: Company settings locale (date_format, number_format, fiscal_year_start_month, tax_id_label, default_currency), locale.ts formatting functions, Regional Settings card
- Phase 7: Accounts import (CSV with hierarchy auto-assignment via Swiss KMU convention), import UX polish (progress bar, error details display), error_log in import results
- Phase 8: Validation hardening — journal: account existence + active check + fiscal year validation; invoice: due_date/quantity/unit_price validation in update(); expense: amount/due_date/category/contact validation in update(); frontend: extractErrorMessage() on all key form pages
- New migrations: 000028 (default_accounts), 000029 (journal_attachments), 000030 (expense_unified), 000031 (company_settings_locale)

## Sprint 09 Notes (Complete)
- Jahresrechnung: 8-page Swiss legal format PDF (Cover, Bilanz 2p, Erfolgsrechnung 2p, Anhang 2p, Antrag 1p)
- Swiss account grouping: ch_account_groups.rs groups accounts by KMU Kontenrahmen number ranges
- TD-018: Convention-Based Account Grouping, TD-019: Shareholders Table, TD-020: Section-Keyed JSON Notes, TD-021: Split Typst Modules
- Shareholders: CRUD table at /settings/shareholders, configurable per company
- Annual Report Notes: 8 section types (accounting_principles, general_info, audit_optout, employees, guarantees, fx_rates, extraordinary, post_balance_events)
- Swiss reports: swiss_balance_sheet() and swiss_income_statement() methods on ReportService
- Income statement subtotals: Betriebsertrag → Bruttoergebnis nach Material → nach Personal → EBITDA → EBIT → EBT → Jahresergebnis
- Typst PDF: 6 files (main + cover + balance_sheet + income_statement + notes + proposal), Swiss apostrophe number format
- Antrag: computes Verlustvortrag (account 2970) + Jahresergebnis, 5% legal reserve, carry forward
- FX rates: auto-populated from exchange_rates as of fiscal year end
- Prior year comparison: auto-detected from fiscal_years table
- Frontend: AnnualReportPage wizard with fiscal year selector, tabbed preview (Bilanz/Erfolgsrechnung/Anhang), generate/download/finalize
- Jurisdiction field added to company_settings (CH default), legal_entity_type (GmbH default)
- New migrations: 000032 (jurisdiction), 000033 (shareholders), 000034 (annual_report_notes), 000035 (annual_reports), 000036 (seed shareholders)
- New entities: shareholder, annual_report_note, annual_report
- 14 new API routes, 3 new services, 6 Typst PDF files, 9 new frontend files

## Sprint 03 Notes
- Fixed 6 API contract mismatches (frontend types → backend response fields)
- Fixed 3 UI gaps (sidebar, time entries project names, pagination)
- Backend: Wrapped VAT report in response struct, added reference to recent entries, currency code→UUID resolution
- Invoice Management: Full lifecycle (draft→sent→paid→cancelled) with journal entry generation
- Invoice number format: RE-YYYY-NNN (assigned on send, auto-increment per year)
- Invoice backend split: invoice_service.rs (CRUD) + invoice_workflow.rs (send/pay/cancel)
- 19 database tables (added invoices + invoice_lines)
