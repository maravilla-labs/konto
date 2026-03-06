# Sprint 09 — Jahresrechnung (Annual Financial Statements)

## Context

Swiss SMEs often rely on external accountants to produce the annual Jahresrechnung PDF. The goal is to auto-generate or assist-generate this report within Maravilla Konto, matching the exact Swiss legal format (OR Art. 957-962). The system must support Switzerland now and be architecturally open for Germany (HGB) later, controlled via a jurisdiction setting in company settings.

The Jahresrechnung is an 8-page PDF: Cover, Bilanz (2p), Erfolgsrechnung (2p), Anhang (2p), Antrag (1p). Numbers are auto-computed from journal data; the Anhang contains auto-populated data + user-editable freetext.

**Lohnausweise** (salary certificates) are deferred to a future sprint — they require a payroll module.

## New Technical Decisions

- TD-018: Convention-Based Account Grouping for Swiss KMU
- TD-019: Shareholders as Separate Table
- TD-020: Annual Report Notes as Section-Keyed JSON
- TD-021: Annual Report PDF via Split Typst Modules

## Phases

### Phase 1: Database & Entities
- Migration 000032: company_settings_jurisdiction (jurisdiction, legal_entity_type)
- Migration 000033: create_shareholders
- Migration 000034: create_annual_report_notes
- Migration 000035: create_annual_reports
- Migration 000036: seed_shareholders_and_notes
- 3 new entities: shareholder, annual_report_note, annual_report
- 3 new repositories
- Modify company_setting entity

### Phase 2: Swiss Account Grouping Engine
- ch_account_groups.rs — Swiss KMU Kontenrahmen grouping by account ranges
- Swiss report types (SwissBalanceSheet, SwissIncomeStatement, etc.)
- Extended report_service with swiss_balance_sheet() and swiss_income_statement()

### Phase 3: Backend Services & API
- shareholder_service.rs — CRUD
- annual_report_note_service.rs — per-section upsert
- annual_report_service.rs — orchestrator with build_data()
- annual_report_handler.rs — all endpoints
- annual_report_dto.rs — request/response types
- Router updates

### Phase 4: Typst PDF Generation
- pdf_annual_report.rs — entry point
- pdf_ar_cover.rs — cover page
- pdf_ar_balance_sheet.rs — Bilanz (Aktiven + Passiven)
- pdf_ar_income_statement.rs — Erfolgsrechnung
- pdf_ar_notes.rs — Anhang (8 sections)
- pdf_ar_proposal.rs — Antrag

### Phase 5: Frontend
- ShareholdersPage — settings CRUD
- AnnualReportPage — wizard with fiscal year selector
- AnnualReportNotesEditor — 8 collapsible sections
- SwissBalanceSheetPreview — hierarchical grouped view
- SwissIncomeStatementPreview — multi-step P&L
- Types, hooks, sidebar/router updates

## Auto-Generated vs User Input

| Section | Auto-Generated | User Input |
|---------|---------------|------------|
| Cover | Company name, city, year | — |
| Bilanz | All numbers from journals | — |
| Erfolgsrechnung | All numbers from journals | — |
| Anhang §1 (Principles) | Pre-seeded OR 957-962 text | Editable override |
| Anhang §2 (General Info) | Settings + shareholders table | — |
| Anhang §3 (Audit Opt-out) | Pre-seeded text | Editable override |
| Anhang §4 (Employees) | — | Location + headcount |
| Anhang §5 (Guarantee) | Pre-seeded text | Editable override |
| Anhang §6 (FX Rates) | From exchange_rates as of 31.12 | Manual override |
| Anhang §7 (Extraordinary) | Amounts from accounts 8000-8799 | Explanation freetext |
| Anhang §8 (Post Events) | — | Freetext |
| Antrag | Verlustvortrag + Jahresergebnis | Allocation amounts |
