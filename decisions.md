# Technical Decisions

## Sprint 01

### TD-001: SQLite as Default Database
- **Decision**: Use SQLite as primary database, with SeaORM abstraction for PostgreSQL switchability
- **Rationale**: Single-tenant tool, no concurrent multi-user load initially, simpler deployment
- **Status**: Active

### TD-002: UUID Primary Keys
- **Decision**: Use UUID v4 for all primary keys
- **Rationale**: No sequential ID guessing, safe for distributed scenarios, SeaORM native support
- **Status**: Active

### TD-003: rust_decimal for Money
- **Decision**: Use `rust_decimal` for all monetary amounts, never floating point
- **Rationale**: Exact decimal arithmetic required for accounting, Swiss tax compliance
- **Status**: Active

### TD-004: JWT Authentication with Refresh Tokens
- **Decision**: JWT access tokens (15min) + refresh tokens (7 days), argon2 password hashing
- **Rationale**: Stateless auth, no session store needed, argon2 is memory-hard (resists GPU attacks)
- **Status**: Active

## Sprint 03

### TD-005: Invoice Number Format RE-YYYY-NNN
- **Decision**: Auto-increment per year, assigned on send (not create)
- **Rationale**: Swiss convention, numbers only assigned to finalized invoices, no gaps in draft state
- **Status**: Active

### TD-006: Service/Workflow Split Pattern
- **Decision**: Split complex domains into `*_service.rs` (CRUD) + `*_workflow.rs` (state transitions)
- **Rationale**: Keeps files under 300-line limit, separates concerns (data vs business logic)
- **Applied to**: invoices, documents
- **Status**: Active

## Sprint 04

### TD-007: Custom ContentEditable Editor (No TipTap/ProseMirror)
- **Decision**: Build WYSIWYG editor from scratch using contentEditable
- **Rationale**: Full control over A4 pagination, block-first architecture, no library limitations for Swiss document standards
- **Status**: Active

### TD-008: Browser Print for Document PDF
- **Decision**: Use CSS @page and browser print for document PDFs
- **Rationale**: Same pagination algorithm as editor, WYSIWYG = PDF, no server-side rendering needed for documents
- **Status**: Active (documents only; invoices use server-side printpdf)

## Sprint 06

### TD-009: typst-as-lib for Invoice PDF (Not printpdf)
- **Decision**: Use `typst-as-lib` 0.15 + `typst-pdf` 0.14 for server-side invoice PDF generation
- **Rationale**: printpdf 0.9's `from_html` is experimental and produces garbled output (overlapping text, no table layout). typst-as-lib provides professional typography, native table support, and template-based approach. Typst's embedded fonts (Noto Sans) ensure consistent rendering across systems.
- **Alternative rejected**: `printpdf` — from_html is broken for production use, manual coordinate-based rendering too tedious for tables
- **Status**: Active

### TD-010: SCOR Reference (Not QRR) for Swiss QR-Bill
- **Decision**: Use SCOR (ISO 11649 creditor reference) with standard IBAN, not QRR with QR-IBAN
- **Rationale**: SCOR works with regular IBAN (no special QR-IBAN needed from bank), simpler setup for small GmbH, ISO standard
- **Status**: Active

### TD-011: lettre 0.11 for SMTP Email
- **Decision**: Use `lettre` 0.11 with `tokio1-rustls-tls` for email sending
- **Rationale**: Most mature Rust SMTP library, async tokio support, TLS/STARTTLS, attachment support
- **Status**: Active

### TD-012: Structured Addresses Only (Type S) for QR-Bill
- **Decision**: Only support Type S (structured) addresses in QR-bill, not Type K (combined)
- **Rationale**: SIX spec v2.3 (Nov 2025) deprecated Type K, banks will reject after Sept 2026, no need to support legacy format
- **Status**: Active

### TD-013: CSV Export via Query Parameter
- **Decision**: Reuse existing list endpoints with `?format=csv` instead of separate export endpoints
- **Rationale**: DRY — same filtering/sorting logic, no duplicate endpoints, CSV response detected by format parameter
- **Status**: Active

## Sprint 08

### TD-014: Unified Expense System (Single Expenses + Expense Reports)
- **Decision**: Merge single expenses and expense reports into one module with `expense_type` field (single/report). Single = vendor bills (AP), Report = employee reimbursements with 7-category grid.
- **Rationale**: Avoids duplicate modules, same approval/payment workflow, unified reporting. expense_lines table adds multi-line support for reports without breaking existing single-expense flow.
- **Status**: Active

### TD-015: Spreadsheet-Style Journal Grid (Banana Accounting Style)
- **Decision**: Full-page journal entry form with spreadsheet-like grid. Type account numbers directly with autocomplete, Tab between cells, keyboard-first navigation.
- **Rationale**: Swiss accountants expect Banana Accounting-style UX. Dialog-based entry is too limiting for double-entry bookkeeping. Account number autocomplete reduces errors.
- **Status**: Active

### TD-016: Abstract File Storage (StorageTrait)
- **Decision**: `StorageService` trait with `LocalStorage` (default, `./data/uploads/`) and `S3Storage` implementations. All file uploads (journal attachments, expense receipts) use this abstraction.
- **Rationale**: Swiss audit compliance requires 10-year document retention. Abstraction allows starting with local storage and migrating to S3 without code changes. Digital archiving legal since Jan 2025 under revised OlICo.
- **Status**: Active

### TD-017: Default Account Configuration
- **Decision**: `default_accounts` table with Swiss KMU defaults (AR=1100, AP=2000, Revenue=3200, etc.). Used by invoice/expense workflows instead of hardcoded account numbers.
- **Rationale**: Eliminates hardcoded account references, allows customization per company, Swiss KMU Kontenrahmen defaults out-of-the-box.
- **Status**: Active

## Sprint 09

### TD-018: Convention-Based Account Grouping for Swiss KMU
- **Decision**: Group accounts by number ranges (Swiss KMU Kontenrahmen) for Jahresrechnung. Jurisdiction-scoped module (`ch_account_groups.rs`), future DE adds `de_account_groups.rs`.
- **Rationale**: Swiss annual report requires accounts grouped by Bilanz/Erfolgsrechnung categories per OR Art. 959a/959b. Number-range grouping matches KMU Kontenrahmen convention.
- **Status**: Active

### TD-019: Shareholders as Separate Table
- **Decision**: `shareholders` table with 1-to-many relationship (multiple Gesellschafter per company), CRUD, used in Anhang and Antrag.
- **Rationale**: Shareholder data is needed in multiple places (Anhang §2, Antrag signature), separate table avoids duplication and allows independent management.
- **Status**: Active

### TD-020: Annual Report Notes as Section-Keyed JSON
- **Decision**: `annual_report_notes` table with `fiscal_year_id + section_key` unique constraint. 8 sections with different data shapes, independent editing per section.
- **Rationale**: Each Anhang section has different structure (freetext, tables, FX rates). JSON content per section allows flexible storage without rigid schema per note type.
- **Status**: Active

### TD-021: Annual Report PDF via Split Typst Modules
- **Decision**: Split annual report PDF generation across 6 helper files (cover, balance_sheet, income_statement, notes, proposal + main entry point) to stay under 300-line limit per file.
- **Rationale**: Annual report PDF is complex (~8 pages, multiple sections). Single file would exceed 300-line limit. Each helper exports `fn render() -> String` returning Typst markup.
- **Status**: Active

## Sprint 10

### TD-022: src-tauri as Separate Rust Project
- **Decision**: Place Tauri at `/src-tauri/` outside the backend workspace, referencing backend crates via `path` dependencies.
- **Rationale**: Tauri CLI expects this layout. Keeps backend workspace clean while allowing Tauri to reuse all backend crates.
- **Status**: Active

### TD-023: Embedded Axum on Localhost
- **Decision**: Spawn real Axum server on `127.0.0.1:0` (OS-assigned port) inside Tauri's `setup()` hook. Frontend connects via HTTP.
- **Rationale**: Zero API changes needed. Same code path for web and desktop. Port 0 avoids conflicts.
- **Status**: Active

### TD-024: Platform Detection
- **Decision**: Frontend detects desktop mode via `window.__TAURI_INTERNALS__`. All platform-specific behavior gated behind `isTauri()` check.
- **Rationale**: Simple, reliable detection. No build-time flags needed. Same frontend bundle works in both modes.
- **Status**: Active

### TD-025: Command Palette
- **Decision**: Cmd+K/Ctrl+K command palette using cmdk + shadcn CommandDialog. Central navigation registry shared by sidebar, palette, and role filtering.
- **Rationale**: Standard UX pattern. Replaces deep sidebar navigation. German + English fuzzy search keywords.
- **Status**: Active

### TD-026: Settings Hub Page
- **Decision**: Move all 17 settings items out of sidebar into card-grid Settings page. Sidebar shows gear icon link.
- **Rationale**: Reduces sidebar from 37 to ~14 items (62% reduction). Settings are rarely accessed, don't need sidebar space.
- **Status**: Active

### TD-027: Adaptive Visual Style
- **Decision**: Use native vibrancy (macOS NSVisualEffectMaterial::Sidebar, Windows 11 Mica), fallback to solid theme on unsupported platforms.
- **Rationale**: Native look and feel where available. CSS glassmorphism as graceful fallback.
- **Status**: Active

### TD-028: Cross-Platform Builds
- **Decision**: Target macOS (DMG), Windows (NSIS), Linux (AppImage/deb) via GitHub Actions with tauri-apps/tauri-action.
- **Rationale**: Single CI workflow produces installers for all platforms on tag push.
- **Status**: Active

## Sprint 11

### TD-029: Four-Language Domain Standard (en/de/fr/it)
- **Decision**: Standardize all language-aware entities and outputs on `en`, `de`, `fr`, `it`, with normalization and strict fallback.
- **Rationale**: Swiss SMEs require multilingual customer-facing output and UI consistency; a fixed language set keeps maintenance and template coverage manageable.
- **Status**: Active

### TD-030: Language Resolution Precedence
- **Decision**: Use deterministic fallback chains:
  - UI (authenticated): `user.profile.language -> localStorage -> browser language -> en`
  - UI (logged out): `localStorage -> browser language -> en`
  - Invoices/Documents output: `explicit document/invoice language -> project language -> contact language -> company ui_language -> en`
- **Rationale**: Prevents ambiguous language behavior and guarantees a valid language for every render/export/email path.
- **Status**: Active

### TD-031: Profile-Centric Preferences and Avatar
- **Decision**: Persist user language and avatar in `users` (`language`, `avatar_url`) and expose profile endpoints under `/api/v1/auth/me` (`GET/PUT`, `/language`, `/avatar`).
- **Rationale**: User-specific preferences must follow the account across sessions/devices; avatar is part of common profile identity.
- **Status**: Active

### TD-032: Domain-Keyed I18n Catalog + Settings-Aware Formatting
- **Decision**: Localize UI by stable domain keys (`dashboard.*`, `invoices.*`, `recurring.*`, `invoice_form.*`, `invoice_dialogs.*`, `documents.*`) with complete `en/de/fr/it` dictionaries, and format visible dates via `company_settings.date_format`.
- **Rationale**: Key-based catalogs keep translations maintainable at scale; domain grouping reduces collisions and improves discoverability. Applying settings-aware date formatting avoids locale mismatches in list/detail views.
- **Status**: Active

### TD-033: N:M Contact Relationships
- **Decision**: `contact_relationships` junction table linking person contacts to organization contacts with role, position, department, is_primary fields. Coexists with legacy `contact_persons` for simple quick-add UX.
- **Rationale**: Data imports show persons exist both as standalone contacts AND as Kontaktperson under companies (N:M pattern). Junction table supports real-world relationships without breaking existing 1:N contact_persons.
- **Status**: Active

### TD-034: Single-Table WBS Hierarchy
- **Decision**: `project_items` with `item_type` discriminator (phase/work_package/task) and self-referencing `parent_id`. Supports all 3 granularity levels with one table.
- **Rationale**: Business logic validates nesting (phase > work_package > task). Single table simplifies queries and avoids joins across separate phase/WP/task tables. In-memory tree building from flat DB results.
- **Status**: Active

### TD-035: Rate Function Abstraction
- **Decision**: `rate_functions` define billing categories with default rates. `project_members` reference a function and optionally override rate. Three-tier fallback: user-project rate > function rate > project default.
- **Rationale**: Separates billing role definitions from per-project assignments. Allows reuse across projects while permitting project-specific overrides.
- **Status**: Active

### TD-036: Soft & Hard Budgets
- **Decision**: Projects support `soft_budget` (warning threshold) and `hard_budget` (limit) for both hours and amount. Per-phase and per-work-package budgets via `project_items.budget_hours/budget_amount`.
- **Rationale**: Two-tier budgets match common project management practice. Soft budget triggers warnings, hard budget enforces limits. Granular budgets on WBS items enable detailed tracking.
- **Status**: Active

### TD-037: Timesheet Approval Workflow
- **Decision**: Separate `timesheets` table with draft/submitted/approved/locked status. Groups time entries by user + period. Locked timesheets prevent retroactive edits.
- **Rationale**: Follows TD-006 service/workflow split. Approval workflow ensures time entry integrity for billing and payroll. Lock mechanism prevents post-approval changes.
- **Status**: Active

## Sprint 12

### TD-038: Employee as Separate Entity from User
- **Decision**: `employees` table separate from `users`, with optional `user_id` FK. Not all employees need system login.
- **Rationale**: Payroll data (AHV, salary, bank details) is distinct from auth data. Employees may be external staff without system access.
- **Status**: Active

### TD-039: Payroll Settings as Singleton Table
- **Decision**: `payroll_settings` singleton table (like `company_settings`) with Swiss 2025 default rates for AHV/IV/EO, ALV, BVG, NBU, BU, KTG, FAK.
- **Rationale**: Rates change annually; singleton allows admin updates without code changes. Seeded with 2025 Swiss rates.
- **Status**: Active

### TD-040: BVG Age-Based Rate Selection
- **Decision**: BVG contribution rate selected by employee age at payroll run date: 7% (25-34), 10% (35-44), 15% (45-54), 18% (55-65). Coordinated salary = gross annual - coordination deduction.
- **Rationale**: Swiss BVG law mandates age-based rates. Calculation uses DOB from employee record.
- **Status**: Active

### TD-041: pain.001 via quick-xml
- **Decision**: Generate Swiss `pain.001.001.09.ch.03` XML using `quick-xml` crate for salary payment file export.
- **Rationale**: No Swiss-specific Rust crate exists for the ch.03 variant. quick-xml provides fast, correct XML serialization.
- **Status**: Active

### TD-042: Lohnausweis via Typst
- **Decision**: Generate ESTV Formular 11 (Lohnausweis) PDF using `typst-as-lib`, same pipeline as invoice and annual report PDFs.
- **Rationale**: Consistent PDF generation stack. Typst handles complex form layouts with tables and fields.
- **Status**: Active

## Sprint 13

### TD-043: Invoicing Method as Project-Level Discriminator
- **Decision**: `invoicing_method` enum on projects: `hourly`, `fixed_price`, `flat_rate`, `non_billable`. Drives budget display, time entry billing behavior, and invoice creation flow.
- **Rationale**: Different project types require different billing logic. Discriminator at project level keeps time entries and invoices simple while enabling method-specific behavior in services.
- **Status**: Active

### TD-044: Time Entry Status Workflow
- **Decision**: Time entries follow `pending` → `in_progress` → `done` → `invoiced` → `closed` workflow with separate `billable` boolean. Follows TD-006 service/workflow split via `time_entry_workflow.rs`.
- **Rationale**: Separates billing eligibility (`billable`) from lifecycle status. Only `done` + `billable=true` entries can be invoiced. Workflow enforces valid transitions, preventing invalid state changes.
- **Status**: Active

### TD-045: Time Rounding on Invoice Import
- **Decision**: Per-project `rounding_method` (up/down/nearest) + `rounding_factor_minutes` (5/10/15/30/60). Applied server-side when creating invoices from time entries.
- **Rationale**: Common accounting tools support time rounding per project. Rounding at invoice creation (not entry creation) preserves original tracking data while producing clean billing.
- **Status**: Active

### TD-046: Project Number Auto-Assignment
- **Decision**: Company settings control auto-assignment: `project_number_auto`, `project_number_prefix`, `project_number_restart_yearly`, `project_number_start`, `project_number_min_length`. Pattern: `{prefix}{zero-padded number}` or `{prefix}{year}-{number}`.
- **Rationale**: Legacy systems auto-assign project numbers. Settings-based approach allows customization without code changes. Restart-yearly option supports annual numbering schemes.
- **Status**: Active

### TD-047: Project Sub-Statuses
- **Decision**: Configurable `project_sub_statuses` table with name, sort_order, color, is_active. Seeded with 8 defaults (Acquisition, Tender, Lost, Preparation, In Progress, Final Stage, Documentation, Delivery). Projects reference via nullable `sub_status_id` FK.
- **Rationale**: Common accounting software has fixed sub-statuses; configurable table allows customization while providing familiar defaults.
- **Status**: Active

## Sprint 14

### TD-048: Setup-First Architecture
- **Decision**: App detects first-run state via `GET /api/v1/setup/status` (checks if any users exist). If no users, frontend redirects to `/setup` wizard. Setup endpoint is public (no auth), guarded by "no users exist" check. Swiss KMU chart of accounts remains always seeded in migration (not wizard-configurable). Product branding stays "Maravilla Konto"; company name is dynamic.
- **Rationale**: Clean separation between generic seed data (chart of accounts, VAT rates, roles) and company-specific data (admin user, company settings, shareholders). Setup endpoint is safe because it rejects requests once any user exists.
- **Status**: Active

### TD-049: Migration-Based Seed Cleanup
- **Decision**: Add new migration `000078` to remove company-specific seeds rather than modifying existing migrations. Deletes seed admin user, shareholders, and renames company-specific bank accounts to generic names.
- **Rationale**: Never modify already-run migrations. New migration is safe for both fresh installs (seed created by 000006 then removed by 000078) and existing databases (only removes exact seed values).
- **Status**: Active

## Sprint 15

### TD-050: Consolidate contact_persons → contact_relationships
- **Decision**: Migrate all contact_persons into contacts table (as person-category contacts) + contact_relationships (N:M links). Migration 000081 creates person contacts with 'cp-' prefixed IDs and updates invoice/project FKs. Legacy contact_persons table kept as safety net.
- **Rationale**: Single source of truth for all people. N:M relationships support real-world scenarios (one person at multiple companies). Eliminates dual lookup paths.
- **Status**: Active

### TD-051: Country-Based VAT Auto-Detection
- **Decision**: `vat_mode` field on contacts (auto|normal|reverse_charge|export_exempt). Auto-detection: CH → normal, EU-27 → reverse_charge, other → export_exempt. Manual override always takes precedence. Seeded VAT rates: "Reverse Charge 0%" (vat-rc0) and "Export Exempt 0%" (vat-ex0) with `vat_category` discriminator.
- **Rationale**: Swiss companies must apply different VAT treatment for domestic, EU (reverse charge), and non-EU (export exempt) sales. Auto-detection reduces manual errors while allowing overrides.
- **Status**: Active

### TD-052: Miller Columns Contact Browser
- **Decision**: Replace flat contact table with two-panel Miller Columns layout. Left panel: searchable/filterable contact list. Right panel: persons linked to selected company via contact_relationships.
- **Rationale**: Company→person hierarchy is the natural mental model for B2B contacts. Miller Columns provide progressive disclosure without deep navigation.
- **Status**: Active

### TD-053: ContactPicker Reusable Combobox
- **Decision**: cmdk-based combobox replacing Select dropdowns in InvoiceForm and ProjectEditDialog. Shows companies and individuals with type icons. Company selection optionally shows person sub-picker from relationships.
- **Rationale**: Standard Select dropdowns don't scale beyond ~50 contacts. Combobox with search is the standard pattern for entity pickers. cmdk already in deps from command palette.
- **Status**: Active

## Sprint 16

### TD-054: Customer Number Auto-Assignment
- **Decision**: Configurable auto-assignment for contact customer numbers, reusing TD-046 project number pattern. Settings: `customer_number_auto`, `customer_number_prefix` (default "K-"), `customer_number_start`, `customer_number_min_length`, `customer_number_restart_yearly`. Backfill migration (M087) copies legacy import ID as zero-padded 6-digit customer number.
- **Rationale**: Invoices need a customer reference number. Configurable numbering matches project number pattern and allows customization. Backfill preserves continuity for existing contacts.
- **Status**: Active

### TD-055: Employee Number Auto-Assignment
- **Decision**: Configurable auto-assignment for employee personnel numbers, same pattern as TD-054. Settings: `employee_number_auto`, `employee_number_prefix` (default "M-"), `employee_number_start`, `employee_number_min_length`, `employee_number_restart_yearly`.
- **Rationale**: Personnel numbers are standard in Swiss HR/payroll. Reuses proven auto-numbering pattern from projects and contacts.
- **Status**: Active

### TD-056: Bidirectional Employee-User Link
- **Decision**: `employee_id` FK on `users` table + existing `user_id` FK on `employees` table. Employee creation optionally provisions a user account with temp password (argon2 hashed). `EmployeeUserLinkService` handles provisioning, linking, and unlinking.
- **Rationale**: Bidirectional link enables lookups in both directions. Temp password provisioning streamlines onboarding. Not all employees need system access, so linking remains optional.
- **Status**: Active

### TD-057: Project Owner via Employee FK
- **Decision**: `owner_id` FK on `projects` referencing `employees` table (not `users`). Owner displayed in ProjectEditDialog via employee dropdown.
- **Rationale**: Project ownership is a business role, not a system access role. Using employee FK allows assigning ownership to staff who may not have system accounts.
- **Status**: Active

## Sprint 17

### TD-058: Markdown as Internal Rich Text Format
- **Decision**: Store all description/notes fields as Markdown. Convert existing HTML (from legacy imports) to Markdown via migration M088. Frontend uses TipTap WYSIWYG editor, backend converts Markdown to Typst markup for PDFs.
- **Rationale**: Markdown is human-readable in raw form, lightweight, well-supported. Avoids HTML sanitization complexity. Clean separation: storage (Markdown) -> display (HTML via frontend) -> PDF (Typst markup via backend).
- **Status**: Active

### TD-059: TipTap Inline Editor for Markdown Fields
- **Decision**: Use TipTap 3.20 with `@tiptap/starter-kit` + `tiptap-markdown` for inline WYSIWYG editing. Bubble menu on selection (bold, italic, bullet/ordered list). Outputs Markdown. `minimal` mode for single-line contexts (invoice line descriptions).
- **Rationale**: TipTap is the most mature headless rich text editor for React. Bubble menu provides formatting without permanent toolbar. Separate from existing block-based document editor (TD-007).
- **Status**: Active

### TD-061: macOS Code Signing & Notarization
- **Decision**: Add Apple Developer ID code signing and notarization to the GitHub Actions release workflow. Certificate installed into temporary keychain on runner; Tauri picks up signing identity and notarization credentials from env vars.
- **Rationale**: Without signing, macOS Gatekeeper blocks the DMG. Notarization is required for apps distributed outside the App Store since macOS 10.15.
- **Status**: Active (requires Apple Developer Program enrollment and GitHub secrets setup)

### TD-060: HTML-to-Markdown Data Migration
- **Decision**: Migration M088 converts HTML content in 13 columns across 10 tables to Markdown. Inline conversion (no external crate) handles `<br>`, `<ul>/<li>`, `<strong>`, `<em>`, `<p>` — all patterns found in legacy HTML imports. Only processes rows containing HTML tags.
- **Rationale**: One-time migration keeps data clean. Idempotent: plain text passes through unchanged.
- **Status**: Active
