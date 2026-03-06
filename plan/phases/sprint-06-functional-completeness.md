# Sprint 06 — Functional Completeness

## Review of Progress

### Sprint 01 (Foundation) — COMPLETE
Delivered: Monorepo scaffold, 17-table schema, JWT+RBAC auth, REST API with OpenAPI docs, CSV/XLSX import engine.

### Sprint 02 (Financial Core & Reporting) — COMPLETE
Delivered: 5 financial reports, fiscal year management, exchange rates, journal post/reverse workflow, dashboard KPIs.

### Sprint 03 (Invoicing & Bug Fixes) — COMPLETE
Delivered: Full invoice lifecycle (draft→sent→paid→cancelled), 6 API contract fixes, invoice number format RE-YYYY-NNN. 19 DB tables.

### Sprint 04 (Documents, Settings & Editor) — COMPLETE
Delivered: Company settings, bank accounts, document templates, word-like editor, documents with workflow, PDF via browser print. 24 DB tables.

### Sprint 05 (Block-First WYSIWYG) — COMPLETE
Delivered: Block-first editor with contact_info, doc_meta, invoice_table block types, adaptive inspector, doc-sync utilities.

---

## Sprint 06 Goals

**Theme:** Close all functional gaps to make Hope usable as a daily accounting tool.

Before Sprint 06, Hope could not:
- Generate or download invoice PDFs
- Include Swiss QR-bill payment slips (legally required since 2022)
- Email invoices to customers
- Manage users beyond the single admin
- View audit logs (legal compliance gap)
- Export data to CSV for accountants/auditors
- ~10 handlers were missing audit logging

---

## Phase 1: Invoice PDF + Swiss QR-Bill

### 1.1 PDF Generation (`pdf_invoice.rs`)
- A4 PDF using `printpdf` 0.9 crate (pure Rust, no external binaries)
- Layout: company header, recipient address, invoice details, line items table, subtotal/VAT/total
- Data sourced from existing tables: `company_settings`, `bank_accounts`, `contacts`, `invoices`, `invoice_lines`
- Endpoint: `GET /api/v1/invoices/{id}/pdf` → `application/pdf`

### 1.2 Swiss QR-Bill (`qr_bill.rs`)
- SIX spec v2.3 (November 2025) compliant
- SPC v0200 payload format with structured addresses (Type S only)
- Reference type: SCOR (ISO 11649 creditor reference) with standard IBAN
- `generate_creditor_reference()` — ISO 11649 with mod97 check digits
- `generate_spc_payload()` — all required fields per spec
- `generate_qr_png()` — QR code to PNG via `qrcode` 0.14 + `image` 0.25 crates
- Payment slip rendered at bottom of invoice PDF (105mm x 210mm)

### 1.3 Frontend
- "Download PDF" button on `InvoiceDetailPage.tsx` (all invoice statuses)
- `downloadPdf(id)` in `api/invoices.ts` with blob response handling

### New Dependencies
```toml
printpdf = "0.9"
qrcode = { version = "0.14", default-features = false }
image = { version = "0.25", default-features = false, features = ["png"] }
```

### New Files
| File | Purpose |
|------|---------|
| `konto-core/src/services/pdf_invoice.rs` | Invoice PDF rendering via printpdf |
| `konto-core/src/services/qr_bill.rs` | Swiss QR-bill SPC payload + QR PNG generation |

---

## Phase 2: User Management

### 2.1 Backend
- `UserService` — create, update, change password, deactivate (reuses argon2 from auth)
- `UserHandler` — 6 endpoints, admin-only (except password change allows self)
- `UserRepo` — extended with create, update, set_password, set_active
- Password never returned in API responses
- Audit logging on all mutations

### 2.2 API Endpoints
```
GET    /api/v1/users                 → list users with role names
POST   /api/v1/users                 → create user
GET    /api/v1/users/{id}            → get user detail
PUT    /api/v1/users/{id}            → update user
PUT    /api/v1/users/{id}/password   → change password (admin or self)
GET    /api/v1/roles                 → list roles
```

### 2.3 Frontend
- `UsersPage.tsx` — table with create/edit/password-change dialogs, active toggle
- Route: `/settings/users`
- Sidebar: "Users" under Settings with UserCog icon

### New Files
| File | Purpose |
|------|---------|
| `konto-core/src/services/user_service.rs` | User CRUD + password management |
| `konto-api/src/handlers/user_handler.rs` | REST endpoints (admin-only) |
| `konto-api/src/dto/user.rs` | Request/response DTOs |
| `frontend/src/types/user.ts` | TypeScript types |
| `frontend/src/api/users.ts` | API client |
| `frontend/src/pages/settings/UsersPage.tsx` | User management page |

---

## Phase 3: Email Service (SMTP)

### 3.1 Database
- New table: `email_settings` (migration 000016)
- Fields: smtp_host, smtp_port, smtp_username, smtp_password, smtp_encryption, from_email, from_name, reply_to_email, bcc_email, is_active

### 3.2 Backend
- `EmailService` — SMTP sending via `lettre` 0.11, TLS/STARTTLS support, attachment support
- `EmailHandler` — settings CRUD (password masked in responses), test email endpoint
- Invoice email: generates PDF, attaches to email, sends to contact's email address
- Validation: draft invoices cannot be emailed, contacts must have email address

### 3.3 API Endpoints
```
GET    /api/v1/settings/email        → get SMTP config (password masked)
PUT    /api/v1/settings/email        → update SMTP config
POST   /api/v1/settings/email/test   → send test email
POST   /api/v1/invoices/{id}/email   → email invoice PDF to contact
```

### 3.4 Frontend
- `EmailSettingsPage.tsx` — SMTP configuration form with test button
- "Email to Customer" button on `InvoiceDetailPage.tsx` (sent/paid/overdue invoices only)
- Route: `/settings/email`
- Sidebar: "Email" under Settings with Mail icon

### New Dependency
```toml
lettre = { version = "0.11", default-features = false, features = ["smtp-transport", "tokio1-rustls-tls", "builder"] }
```

### New Files
| File | Purpose |
|------|---------|
| `konto-migration/src/m20240101_000016_create_email_settings.rs` | Migration |
| `konto-db/src/entities/email_setting.rs` | SeaORM entity |
| `konto-db/src/repository/email_settings_repo.rs` | Repository |
| `konto-core/src/services/email_service.rs` | SMTP sending via lettre |
| `konto-api/src/handlers/email_handler.rs` | Settings CRUD + test |
| `konto-api/src/dto/email.rs` | DTOs |
| `frontend/src/types/email.ts` | TypeScript types |
| `frontend/src/api/email.ts` | API client |
| `frontend/src/hooks/useEmailApi.ts` | TanStack Query hooks |
| `frontend/src/pages/settings/EmailSettingsPage.tsx` | SMTP config page |

---

## Phase 4: Audit Log Viewer

### 4.1 Backend
- `AuditHandler` — paginated list with filtering, admin/auditor role access
- `AuditRepo` — extended `find_filtered()` with optional entity_type, action, user_id, date range filters
- DTOs: `AuditLogResponse`, `AuditLogParams`

### 4.2 API Endpoint
```
GET /api/v1/audit-log?page=1&per_page=50&entity_type=invoice&action=create&from=2026-01-01&to=2026-12-31
```

### 4.3 Frontend
- `AuditLogPage.tsx` — paginated table with filter dropdowns, JSON detail dialog for old/new values
- Route: `/settings/audit-log`
- Sidebar: "Audit Log" under Settings with ScrollText icon

### New Files
| File | Purpose |
|------|---------|
| `konto-api/src/handlers/audit_handler.rs` | List with filtering |
| `konto-api/src/dto/audit.rs` | DTOs |
| `frontend/src/types/audit.ts` | TypeScript types |
| `frontend/src/api/audit.ts` | API client |
| `frontend/src/hooks/useAuditApi.ts` | TanStack Query hooks |
| `frontend/src/pages/settings/AuditLogPage.tsx` | Audit log page |

---

## Phase 5: Consistent Audit Logging

Added `AuditService::log()` calls to all mutation handlers that were missing them.

### Handlers Updated
| Handler | Mutations | Audit Calls Added |
|---------|-----------|-------------------|
| `account_handler.rs` | create, update, delete | 3 |
| `contact_handler.rs` | create, update, delete | 3 |
| `journal_handler.rs` | create, post, reverse | already had |
| `project_handler.rs` | create, update | 2 |
| `time_entry_handler.rs` | create, update, delete | already had |
| `fiscal_year_handler.rs` | create, update, close | already had |
| `exchange_rate_handler.rs` | create, update, delete | already had |
| `bank_account_handler.rs` | create, update, delete | already had |
| `template_handler.rs` | create, update, delete, duplicate | already had |
| `import_handler.rs` | execute | 1 |

**Total: 48 AuditService::log calls across 15 handlers** — every mutation in the application is now audited.

---

## Phase 6: CSV Data Export

### 6.1 Backend
- `ExportService::to_csv<T: Serialize>()` — generic CSV generation using the `csv` crate
- Added `?format=csv` query parameter support to 7 list/report endpoints
- CSV mode fetches all records (bypasses pagination) for complete export

### 6.2 Endpoints with CSV Support
```
GET /api/v1/invoices?format=csv
GET /api/v1/contacts?format=csv
GET /api/v1/journal?format=csv
GET /api/v1/reports/trial-balance?format=csv
GET /api/v1/reports/balance-sheet?format=csv
GET /api/v1/reports/profit-loss?format=csv
GET /api/v1/reports/vat?format=csv
```

### 6.3 Frontend
- `downloadCsv()` utility in `lib/export.ts` — fetch blob + trigger download
- "Export CSV" buttons added to 7 pages:
  - `InvoicesPage.tsx`, `ContactsPage.tsx`, `JournalPage.tsx`
  - `TrialBalancePage.tsx`, `BalanceSheetPage.tsx`, `ProfitLossPage.tsx`, `VatReportPage.tsx`

### New Files
| File | Purpose |
|------|---------|
| `konto-core/src/services/export_service.rs` | Generic CSV generation |
| `frontend/src/lib/export.ts` | CSV download utility |

---

## Summary

| Phase | Feature | New Files | Modified Files | New Tables |
|-------|---------|-----------|----------------|------------|
| 1 | PDF + QR-Bill | 2 backend | 5 | 0 |
| 2 | User Management | 3 backend, 3 frontend | 5 | 0 |
| 3 | Email Service | 5 backend, 4 frontend | 8 | 1 (email_settings) |
| 4 | Audit Log Viewer | 2 backend, 4 frontend | 2 | 0 |
| 5 | Consistent Audit | 0 | 9 | 0 |
| 6 | CSV Export | 1 backend, 1 frontend | 11 | 0 |
| **Total** | | **~25 new files** | **~40 modified** | **1** |

### Cumulative Stats After Sprint 06
- **Database**: 25 tables, 16 migrations
- **API**: 59 routes, ~91 handler functions
- **Frontend**: ~56 pages
- **Audit**: 48 AuditService::log calls (100% mutation coverage)
- **New dependencies**: printpdf 0.9, qrcode 0.14, image 0.25, lettre 0.11

### Deferred to Sprint 07
- Recurring invoices (requires scheduler/cron)
- Document server-side PDF (WYSIWYG blocks → PDF, complex)
- Bank reconciliation / CAMT.053 import
- Expense tracking module
