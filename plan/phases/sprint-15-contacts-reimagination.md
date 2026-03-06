# Sprint 15: Contacts Reimagination + International VAT

## Goals
1. Miller Columns UX for browsing company→person hierarchies
2. Consolidate contact_persons into contact_relationships (N:M single source of truth)
3. International VAT auto-detection based on contact country

## Technical Decisions
- TD-050: Consolidate contact_persons → contact_relationships (N:M only)
- TD-051: Country-based VAT auto-detection with manual override (vat_mode field)
- TD-052: Miller Columns contact browser for company→person navigation
- TD-053: ContactPicker reusable combobox component

## Phases

### Phase 1: Database Migrations (000079–000081)
- M-000079: Add `vat_mode` to contacts (auto|normal|reverse_charge|export_exempt)
- M-000080: Add `vat_category` to vat_rates + seed RC0 and EX0 rates
- M-000081: Migrate contact_persons → contacts + contact_relationships

### Phase 2: Backend — Entities & Services
- Entity updates: contact.rs (vat_mode), vat_rate.rs (vat_category)
- New: vat_resolution_service.rs (EU detection, VAT mode resolution)
- New: contact_vat_handler.rs (GET /contacts/{id}/vat-info)
- Updates: contact_service, invoice_service, pdf_invoice

### Phase 3: Frontend — Miller Columns Contact Browser
- ContactBrowser.tsx: two-panel layout
- ContactBrowserPanel.tsx: searchable contact list
- PersonsPanel.tsx: persons for selected company

### Phase 4: Frontend — ContactPicker + VAT Integration
- ContactPicker.tsx: cmdk-based combobox
- VatModeBanner.tsx: reverse charge / export exempt info
- VatModeSelector.tsx: contact edit VAT mode select
- InvoiceForm updates: ContactPicker + VAT auto-detection

### Phase 5: i18n, Types, Hooks
- ~35 new i18n keys × 4 languages
- Contact/VatInfo type updates
- New hooks: useContactPersonsViaRelationships, useContactVatInfo

## Status: Complete

## Deliverables
- 14 new files, 25 modified files
- 3 migrations (000079–000081)
- 2 new backend services (vat_resolution_service, contact_vat_handler)
- 6 new frontend components (ContactBrowser, ContactBrowserPanel, PersonsPanel, ContactPicker, VatModeBanner, VatModeSelector)
- 1 new type file (vat-info.ts)
- ~20 i18n keys × 4 languages
- 4 technical decisions (TD-050 through TD-053)
- cargo build: clean
- tsc --noEmit: clean
