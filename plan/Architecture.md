# Hope Architecture

Based on DRY, modularize and maintainability. Latest version of all frameworks used and secure by design.
No file bigger than 300 max 500 lines, must be enforced for maintainability.

## Principles
- Mobile First
- Realtime updates in UI
- Notification support with push notification and configurable
- Clean API auto-generated documentation (OpenAPI/Swagger)
- Data domain model
- Abstracted storage so user can switch from SQLite (primary default) to PostgreSQL
- Abstract file storage to use default filesystem and can switch to S3
- Real audit log for legal reasons
- Clean component architecture
- React component library reusable throughout the project (see @frontend.md)
- Backend in Rust and frontend in React
- Plan implementation in phases as sprint markdowns in @plan/phases/sprint-*.md

## Tech Stack (Sprint 01)

| Layer | Technology | Version |
|-------|-----------|---------|
| Backend | Rust + Axum | 1.85+ / 0.8 |
| ORM | SeaORM | 1.1 |
| Database | SQLite (default), PostgreSQL (switchable) | - |
| Frontend | React + TypeScript + Vite | 19 / 6 |
| Styling | Tailwind CSS + shadcn/ui | 4.x / latest |
| Auth | JWT + argon2 + RBAC | - |
| API Docs | utoipa (OpenAPI) + Swagger UI | 5.x |

## Backend Crate Architecture

```
konto-server → konto-api → konto-core → konto-db → konto-common
```

- **konto-common**: Shared enums, Money/AccountNumber types, pagination, errors, config
- **konto-migration**: SeaORM migrations for 25 tables + seed data (16 migrations)
- **konto-db**: SeaORM entities, connection factory, repository layer
- **konto-core**: Auth (JWT + argon2 + RBAC), domain services, data import
- **konto-api**: REST handlers, DTOs, auth middleware, OpenAPI
- **konto-server**: Config, AppState, router assembly, entry point

### Key Decisions
- UUID primary keys everywhere
- `rust_decimal` for all money amounts (no floating point)
- SQLite via `sqlx-sqlite` feature, switchable to PostgreSQL
- JWT access tokens (15min) + refresh tokens (7 days)
- RBAC with JSON permissions on roles
- Append-only audit log for all mutations
- `calamine` for XLSX parsing, `csv` crate for CSV parsing/export
- `typst-as-lib` 0.15 + `typst-pdf` 0.14 for server-side PDF generation (invoices with Swiss QR-bill)
- `qrcode` 0.14 + `image` 0.25 for Swiss QR-bill QR code PNG generation
- `lettre` 0.11 for SMTP email sending with attachment support
- Multilingual standard for UI/output: `en`, `de`, `fr`, `it`
- Profile-level preferences: `users.language` + `users.avatar_url`

## Localization & Profile Architecture

- **Supported languages**: `en`, `de`, `fr`, `it` (normalized server-side and client-side)
- **Translation catalog strategy**:
  - Domain-keyed message namespaces (`dashboard.*`, `invoices.*`, `recurring.*`, `invoice_form.*`, `invoice_dialogs.*`, `documents.*`, `common.*`, `status.*`)
  - Single source of truth in `frontend/src/i18n/messages.ts` for all 4 languages
  - Fallback behavior: active language -> `en` -> inline fallback text
- **Language fields**:
  - `contacts.language` (existing)
  - `projects.language` (migration 000040)
  - `invoices.language` (migration 000040)
  - `documents.language` (migration 000040)
  - `company_settings.ui_language` (migration 000040)
  - `users.language` (migration 000041)
- **Profile avatar**:
  - `users.avatar_url` (migration 000042)
  - Files stored under `/uploads/avatars/{user_id}.{ext}`
- **Resolution chains**:
  - UI authenticated: `user.language -> localStorage -> browser -> en`
  - UI logged out: `localStorage -> browser -> en`
  - Invoice/Document output: `explicit -> project -> contact -> company ui_language -> en`
- **Formatting adaptation**:
  - Date rendering in key list/detail pages uses `company_settings.date_format`
  - Number/currency rendering in key list pages uses `company_settings.number_format`

## Frontend Architecture

- **Router**: react-router-dom v7
- **State**: Zustand (auth), TanStack React Query v5 (server state)
- **Forms**: react-hook-form + zod validation
- **UI**: shadcn/ui components (New York style)
- **API**: Axios with JWT interceptor (auto-refresh on 401)

## Database (25 tables)
Auth: users, roles, user_roles
Accounting: currencies, exchange_rates, vat_rates, fiscal_years, accounts, journal_entries, journal_lines
Invoicing: invoices, invoice_lines
CRM: contacts, contact_persons
Projects: projects, activity_types, time_entries
Documents: documents, document_line_items, document_templates
Settings: company_settings, bank_accounts, email_settings
System: audit_log, import_batches

## Invoice API Endpoints (Sprint 03)
- `POST   /api/v1/invoices` — create draft invoice with lines
- `GET    /api/v1/invoices` — list with status/contact/search filter + pagination
- `GET    /api/v1/invoices/{id}` — get with lines, contact name, project name
- `PUT    /api/v1/invoices/{id}` — update (draft only)
- `DELETE /api/v1/invoices/{id}` — delete (draft only)
- `POST   /api/v1/invoices/{id}/send` — draft→sent, assigns number RE-YYYY-NNN, creates journal entry
- `POST   /api/v1/invoices/{id}/pay` — sent→paid, creates payment journal entry
- `POST   /api/v1/invoices/{id}/cancel` — sent/overdue→cancelled, creates reversing entry

## Frontend Pages (Sprint 03)
- `/invoices` — list with status tabs, search, pagination
- `/invoices/new` — create with dynamic line items form
- `/invoices/:id` — detail view with status-based actions (PDF download, email)
- `/invoices/:id/edit` — edit draft invoices

## Sprint 06 API Additions

### Invoice PDF & Email
- `GET  /api/v1/invoices/{id}/pdf` — download invoice as PDF with Swiss QR-bill
- `POST /api/v1/invoices/{id}/email` — email invoice PDF to contact

### User Management (admin-only)
- `GET    /api/v1/users` — list users with role names
- `POST   /api/v1/users` — create user
- `GET    /api/v1/users/{id}` — get user detail
- `PUT    /api/v1/users/{id}` — update user
- `PUT    /api/v1/users/{id}/password` — change password (admin or self)
- `GET    /api/v1/roles` — list roles

### Email Settings (admin-only)
- `GET  /api/v1/settings/email` — get SMTP config (password masked)
- `PUT  /api/v1/settings/email` — update SMTP config
- `POST /api/v1/settings/email/test` — send test email

### User Profile (authenticated)
- `GET  /api/v1/auth/me` — get own profile (incl. language + avatar_url)
- `PUT  /api/v1/auth/me` — update own profile (`full_name`, `language`)
- `PUT  /api/v1/auth/me/language` — update own language quickly
- `POST /api/v1/auth/me/avatar` — upload own profile image

### Audit Log (admin/auditor)
- `GET /api/v1/audit-log` — paginated list with filters (entity_type, action, user_id, date range)

### CSV Export
- `?format=csv` on: invoices, contacts, journal, trial-balance, balance-sheet, profit-loss, vat

## Swiss QR-Bill (Sprint 06)
- SIX spec v2.3 (November 2025)
- SPC v0200 payload, structured addresses (Type S) only
- SCOR reference type (ISO 11649 creditor reference)
- QR code generated via `qrcode` crate, embedded in PDF via `printpdf`
- Payment slip: 105mm x 210mm at bottom of A4 invoice
