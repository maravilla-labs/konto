# Sprint 14 — User Onboarding Wizard

## Goal
Replace hardcoded company-specific seed data with a first-run onboarding wizard so any Swiss company can install Hope.

## Technical Decisions
- TD-048: Setup-First Architecture — public `/api/v1/setup/status` checks if any users exist; `/setup` wizard creates admin + company config
- TD-049: Migration-Based Seed Cleanup — migration 000078 removes company-specific seeds without modifying existing migrations

## Migrations
| # | Name | Description |
|---|------|-------------|
| 000078 | remove_company_seeds | Delete seed admin user, shareholders; rename bank accounts to generic |

## New API Routes
| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/v1/setup/status` | Public | Returns `{ setup_needed: bool }` |
| POST | `/api/v1/setup/complete` | Public | Creates admin + settings + fiscal year, returns JWT |
| GET | `/api/v1/setup/branding` | Public | Returns company branding or null |

## Phases
1. **Migration + Backend Setup Service** — migration 000078, setup_service, setup DTOs, setup handler, router/openapi registration
2. **Frontend Setup API + Types** — API client, TanStack Query hooks
3. **Frontend Setup Wizard UI** — 5-step wizard (language, admin, company, accounting, review)
4. **Routing + Auth Flow Integration** — /setup route, AuthGuard redirect, dynamic branding
5. **i18n** — ~50 keys x 4 languages

## Files Changed
- **Backend new**: migration 000078, setup_service.rs, setup.rs (DTO), setup_handler.rs
- **Backend modified**: lib.rs, handlers/mod.rs, dto/mod.rs, services/mod.rs, router.rs, openapi.rs, settings_service.rs
- **Frontend new**: api/setup.ts, hooks/useSetup.ts, SetupPage.tsx, 5 step components
- **Frontend modified**: App.tsx, AuthGuard.tsx, LoginPage.tsx, Sidebar.tsx, i18n/messages.ts
