# Sprint 01 - Foundation

## Goal
Set up the complete monorepo, database schema, auth, core CRUD APIs, frontend shell, and data import.

## Tech Stack
- Backend: Rust + Axum 0.8 + SeaORM 1.1 + SQLite
- Frontend: React 19 + TypeScript + Vite 6 + Tailwind CSS 4 + shadcn/ui
- Auth: JWT + argon2 + RBAC

## Deliverables

### Backend
- [x] Monorepo scaffold with Cargo workspace
- [x] konto-common: Shared types, Money, AccountNumber, errors
- [x] konto-migration: 17 table migrations + seed data
- [x] konto-db: SeaORM entities + repositories
- [x] konto-core: Auth, domain services, data import
- [x] konto-api: REST handlers, DTOs, OpenAPI/Swagger
- [x] konto-server: Config, router, entry point

### Frontend
- [x] Vite + React + TypeScript scaffold
- [x] Tailwind CSS + shadcn/ui setup
- [x] API client with JWT interceptor
- [x] Auth (Zustand store, login page, route guard)
- [x] App layout (sidebar, topbar, mobile nav)
- [x] Pages: Dashboard, Accounts, Contacts, Journal, Projects, Time Entries
- [x] Import wizard (upload → preview → execute)

### Database Tables
1. users
2. roles
3. user_roles
4. currencies
5. exchange_rates
6. vat_rates
7. fiscal_years
8. accounts
9. journal_entries
10. journal_lines
11. contacts
12. contact_persons
13. projects
14. activity_types
15. time_entries
16. audit_log
17. import_batches

### Seed Data
- Swiss KMU chart of accounts (~78 accounts)
- Currencies: CHF (primary), EUR, USD
- VAT rates: 8.1%, 2.6%, 3.8%, 7.7% (historical)
- Roles: admin, accountant, auditor, employee
- Admin user: created via setup wizard

### Data Import
- Contacts CSV (comma, 107 columns, 62 records)
- Time entries CSV (semicolon, 30 columns, 776 records)
- Projects XLSX (24 columns, 24 records)
- Journal XLSX (12 columns, 860 records)

## API Endpoints
All under `/api/v1/`:
- Auth: login, refresh, me
- Accounts: CRUD + tree view
- Contacts: CRUD with contact persons
- Journal: list, create, get with lines
- Projects: CRUD
- Import: upload, preview, execute
- Swagger UI: /api-docs/
