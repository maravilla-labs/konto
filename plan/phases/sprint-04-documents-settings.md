# Sprint 04 — Document Management, Settings & Word-like Editor

## Goal
Build a reusable Word-like document editor (ported from the Svelte wordlike-app to React), company/business settings, template system, and full document lifecycle for quotes, offers, SOWs, contracts — with PDF generation, letterhead configuration, and document-to-project conversion.

---

## Prerequisites
- Sprint 01–03 complete (auth, accounting, invoicing, CRM, projects)
- Sprint 01–03 foundation

---

## Phase 1: Company Settings & Bank Accounts

### 1.1 Backend — Settings Tables & API

**Migration: `m20240101_000011_create_settings.rs`**

New tables:

```
company_settings (singleton-ish, one row per tenant)
  id             UUID PK
  legal_name     TEXT NOT NULL        -- "Your Company GmbH"
  trade_name     TEXT                 -- "Maravilla Labs"
  street         TEXT NOT NULL
  postal_code    TEXT NOT NULL
  city           TEXT NOT NULL
  country        TEXT NOT NULL DEFAULT 'CH'
  email          TEXT
  phone          TEXT
  website        TEXT
  vat_number     TEXT                 -- "CH-234.973.545 MWST"
  vat_method     TEXT NOT NULL DEFAULT 'flat_rate'  -- flat_rate | effective
  register_number TEXT               -- Commercial register number
  logo_url       TEXT                 -- path to uploaded logo
  created_at     TIMESTAMP
  updated_at     TIMESTAMP

bank_accounts
  id             UUID PK
  name           TEXT NOT NULL        -- "UBS CHF", "UBS EUR"
  bank_name      TEXT NOT NULL        -- "UBS Switzerland AG"
  iban           TEXT NOT NULL        -- "CH37 0023 3233 2445 7901 F"
  bic            TEXT                 -- "UBSWCHZH80A"
  currency_id    UUID FK → currencies
  account_id     UUID FK → accounts   -- linked ledger account (e.g. 1020 Bank)
  is_default     BOOLEAN DEFAULT false
  created_at     TIMESTAMP
  updated_at     TIMESTAMP
```

**API endpoints:**
- `GET    /api/v1/settings` — get company settings
- `PUT    /api/v1/settings` — update company settings
- `POST   /api/v1/settings/logo` — upload logo (multipart)
- `GET    /api/v1/bank-accounts` — list bank accounts
- `POST   /api/v1/bank-accounts` — create
- `PUT    /api/v1/bank-accounts/{id}` — update
- `DELETE /api/v1/bank-accounts/{id}` — delete

**Backend files:**
- `konto-db/src/entities/company_setting.rs`
- `konto-db/src/entities/bank_account.rs`
- `konto-db/src/repository/settings_repo.rs`
- `konto-db/src/repository/bank_account_repo.rs`
- `konto-core/src/services/settings_service.rs`
- `konto-api/src/dto/settings.rs`
- `konto-api/src/handlers/settings_handler.rs`
- `konto-api/src/handlers/bank_account_handler.rs`

### 1.2 Frontend — Settings Pages

**Pages:**
- `/settings` — settings hub with tabs/sections
- `/settings/company` — edit company info, logo upload, VAT config
- `/settings/bank-accounts` — CRUD bank accounts linked to ledger accounts

**Files:**
- `pages/settings/CompanySettingsPage.tsx`
- `pages/settings/BankAccountsPage.tsx`
- `components/settings/CompanyForm.tsx`
- `components/settings/LogoUpload.tsx`
- `components/settings/BankAccountForm.tsx`

---

## Phase 2: Reusable Word-like Editor Component

Port the custom contentEditable editor from the Svelte wordlike-app to React. This becomes a shared component used for all document editing (invoices, quotes, SOWs, contracts, letterheads).

### 2.1 Editor Core — `components/editor/`

**Architecture (ported from wordlike-app, adapted to React):**
- Custom `contentEditable` blocks (no TipTap/ProseMirror — matches wordlike-app approach)
- A4 page layout with real pagination
- Inline formatting toolbar on selection
- Slash menu for block insertion
- Keyboard navigation between blocks

**Block types:**
- `h1`, `h2`, `h3` — headings
- `p` — paragraph
- `blockquote` — quotes
- `table` — line-item tables (for invoices, SOW deliverables)
- `image` — embedded images (logo, signatures)
- `divider` — horizontal rule / section break
- `placeholder` — template variable (e.g. `{{client_name}}`, `{{date}}`, `{{total}}`)

**Data model (TypeScript):**
```typescript
interface Block {
  id: string;
  type: BlockType;
  data: TextBlockData | TableBlockData | ImageBlockData | PlaceholderData;
  meta: BlockMeta;
}

interface BlockMeta {
  fontSize: number | null;    // pt
  align: 'left' | 'center' | 'right' | 'justify';
  lineHeight: number;         // default 1.5
  font: 'system' | 'serif' | 'mono';
  keepWithNext: boolean;      // page break control
  locked: boolean;            // template-locked blocks (header/footer)
}

interface DocumentModel {
  id: string;
  blocks: Block[];
  pageSetup: PageSetup;
  header: Block[];            // repeated on every page
  footer: Block[];            // repeated on every page
}

interface PageSetup {
  format: 'a4' | 'letter';
  orientation: 'portrait' | 'landscape';
  margins: { top: number; right: number; bottom: number; left: number }; // mm
  headerHeight: number;       // mm
  footerHeight: number;       // mm
}
```

**Editor component files:**
- `components/editor/DocumentEditor.tsx` — main editor shell (A4 pages, toolbar, inspector)
- `components/editor/EditorToolbar.tsx` — top toolbar (view modes, export, block-type selector)
- `components/editor/EditorBlock.tsx` — contentEditable block renderer
- `components/editor/FormatBar.tsx` — floating inline format bar (bold, italic, etc.)
- `components/editor/SlashMenu.tsx` — `/` command palette for block insertion
- `components/editor/PageRenderer.tsx` — A4 page layout with page numbers
- `components/editor/HeaderFooterEditor.tsx` — header/footer zone editor
- `components/editor/BlockInspector.tsx` — right panel for block metadata
- `components/editor/TableBlock.tsx` — editable table block (for line items)
- `components/editor/ImageBlock.tsx` — image block (for logo, etc.)
- `components/editor/PlaceholderBlock.tsx` — template variable display/edit

**Utility files:**
- `lib/editor/types.ts` — all TypeScript interfaces
- `lib/editor/pagination.ts` — page break algorithm (from wordlike-app)
- `lib/editor/caret.ts` — caret/selection management
- `lib/editor/commands.ts` — formatting commands (execCommand wrappers)
- `lib/editor/pdf-export.ts` — PDF generation
- `lib/editor/placeholder-resolver.ts` — resolve `{{variables}}` with real data

### 2.2 PDF Generation

**Approach:** Browser-based PDF via `@react-pdf/renderer` or `html2pdf.js` / `jsPDF + html2canvas`

**Recommended: `@react-pdf/renderer`** for precise A4 control:
- Render document model → React-PDF components
- Exact A4 dimensions, margins, header/footer on each page
- Embed logo, tables, formatted text
- Generate as blob for preview or download

**PDF features:**
- Live preview panel (side-by-side with editor)
- Download as PDF
- Print (browser print dialog with proper A4 CSS)
- Embed QR codes (for Swiss QR invoices later)

**Files:**
- `lib/editor/pdf-renderer.tsx` — React-PDF document component
- `lib/editor/pdf-preview.tsx` — preview panel wrapper
- `components/editor/PdfPreviewPanel.tsx` — side panel with live PDF

### 2.3 View Modes (from wordlike-app)

- **Pages** — single-column A4 pages stacked vertically
- **Spread** — two-page side-by-side view
- **Canvas** — free-flowing text without page breaks (for quick editing)

---

## Phase 3: Template System

### 3.1 Backend — Templates

**Migration: `m20240101_000012_create_templates.rs`**

```
document_templates
  id              UUID PK
  name            TEXT NOT NULL          -- "Standard Invoice", "SOW Template"
  template_type   TEXT NOT NULL          -- invoice | quote | offer | sow | contract | letter
  content_json    TEXT NOT NULL          -- JSON serialized DocumentModel
  header_json     TEXT                   -- JSON header blocks (overrides per template)
  footer_json     TEXT                   -- JSON footer blocks
  page_setup_json TEXT                   -- JSON PageSetup overrides
  is_default      BOOLEAN DEFAULT false
  created_by      UUID FK → users
  created_at      TIMESTAMP
  updated_at      TIMESTAMP
```

**API endpoints:**
- `GET    /api/v1/templates` — list (filter by template_type)
- `GET    /api/v1/templates/{id}` — get with full content
- `POST   /api/v1/templates` — create
- `PUT    /api/v1/templates/{id}` — update
- `DELETE /api/v1/templates/{id}` — delete
- `POST   /api/v1/templates/{id}/duplicate` — clone template

**Backend files:**
- `konto-db/src/entities/document_template.rs`
- `konto-db/src/repository/template_repo.rs`
- `konto-core/src/services/template_service.rs`
- `konto-api/src/dto/template.rs`
- `konto-api/src/handlers/template_handler.rs`

### 3.2 Letterhead Template

A special template type for defining company letterhead (header + footer) reused across all document types:
- **Header zone**: Company logo, company name, address — positioned blocks
- **Footer zone**: Bank details, VAT number, contact info, page numbers
- Configured once via the editor, applied to all document templates
- Stored as `template_type = 'letterhead'` with `is_default = true`

### 3.3 Placeholder Variables

Templates use `{{variable}}` placeholders resolved at render time:

**Common placeholders:**
| Variable | Description |
|----------|-------------|
| `{{company_name}}` | From company_settings.legal_name |
| `{{company_address}}` | Full formatted address |
| `{{company_vat}}` | VAT number |
| `{{company_logo}}` | Logo image |
| `{{company_email}}` | Company email |
| `{{company_phone}}` | Company phone |
| `{{company_website}}` | Company website |
| `{{bank_name}}` | Default bank name |
| `{{bank_iban}}` | Default bank IBAN |
| `{{bank_bic}}` | Default bank BIC |

**Client placeholders:**
| Variable | Description |
|----------|-------------|
| `{{client_name}}` | Contact company name |
| `{{client_contact}}` | Contact person name |
| `{{client_address}}` | Contact full address |
| `{{client_email}}` | Contact email |
| `{{customer_number}}` | Contact customer number |

**Document placeholders:**
| Variable | Description |
|----------|-------------|
| `{{doc_number}}` | Document number (AN-NNNNN, RE-YYYY-NNN) |
| `{{doc_date}}` | Issue date |
| `{{doc_valid_until}}` | Validity date (quotes/offers) |
| `{{doc_title}}` | Document title |
| `{{subtotal}}` | Before VAT |
| `{{vat_rate}}` | VAT percentage |
| `{{vat_amount}}` | VAT amount |
| `{{total}}` | Grand total |
| `{{line_items}}` | Rendered table of items |
| `{{current_date}}` | Today's date |
| `{{page_number}}` | Current page |
| `{{total_pages}}` | Total pages |

### 3.4 Frontend — Template Management

**Pages:**
- `/settings/templates` — list all templates grouped by type
- `/settings/templates/new` — create template using editor
- `/settings/templates/{id}` — edit template
- `/settings/letterhead` — special page for letterhead config

**Files:**
- `pages/settings/TemplatesPage.tsx`
- `pages/settings/TemplateEditorPage.tsx`
- `pages/settings/LetterheadPage.tsx`
- `components/templates/TemplateCard.tsx`
- `components/templates/PlaceholderPicker.tsx` — sidebar to insert placeholders

---

## Phase 4: Document Types (Quotes, Offers, SOWs, Contracts)

### 4.1 Backend — Documents Table

**Migration: `m20240101_000013_create_documents.rs`**

```
documents
  id              UUID PK
  doc_type        TEXT NOT NULL          -- quote | offer | sow | contract
  doc_number      TEXT                   -- AN-NNNNN (assigned on finalize)
  title           TEXT NOT NULL
  status          TEXT NOT NULL DEFAULT 'draft'
  contact_id      UUID FK → contacts
  project_id      UUID FK → projects (nullable)
  template_id     UUID FK → document_templates (nullable)
  content_json    TEXT NOT NULL          -- resolved DocumentModel (with real data)
  currency_id     UUID FK → currencies
  subtotal        DECIMAL(19,4)
  vat_rate        DECIMAL(5,2)
  vat_amount      DECIMAL(19,4)
  total           DECIMAL(19,4)
  valid_until     DATE                   -- for quotes/offers
  issued_at       DATE
  signed_at       DATE                   -- for SOWs/contracts
  converted_from  UUID FK → documents    -- self-ref: offer→SOW, SOW→contract
  created_by      UUID FK → users
  created_at      TIMESTAMP
  updated_at      TIMESTAMP

document_line_items
  id              UUID PK
  document_id     UUID FK → documents
  position        INT NOT NULL
  description     TEXT NOT NULL
  quantity         DECIMAL(10,2)
  unit            TEXT                   -- hours, days, pieces, flat
  unit_price      DECIMAL(19,4)
  discount_pct    DECIMAL(5,2) DEFAULT 0
  total           DECIMAL(19,4)
  created_at      TIMESTAMP
```

**Status workflow per type:**

| Type | Statuses |
|------|----------|
| Quote | draft → sent → accepted → rejected → expired |
| Offer | draft → sent → accepted → rejected → expired |
| SOW | draft → sent → signed → completed → cancelled |
| Contract | draft → sent → signed → active → completed → cancelled |

**Document numbering:**
- Format: `AN-NNNNN` (matching existing convention from supporting docs)
- Assigned on `send` action (like invoices use RE-YYYY-NNN)
- Auto-increment per year

**API endpoints:**
- `POST   /api/v1/documents` — create draft
- `GET    /api/v1/documents` — list (filter by type, status, contact, search)
- `GET    /api/v1/documents/{id}` — get with content, lines, contact
- `PUT    /api/v1/documents/{id}` — update draft
- `DELETE /api/v1/documents/{id}` — delete draft
- `POST   /api/v1/documents/{id}/send` — finalize, assign number, mark sent
- `POST   /api/v1/documents/{id}/accept` — mark accepted/signed
- `POST   /api/v1/documents/{id}/reject` — mark rejected
- `POST   /api/v1/documents/{id}/convert` — convert to next type (offer→SOW, SOW→project)
- `GET    /api/v1/documents/{id}/pdf` — generate PDF on-the-fly

**Backend files:**
- `konto-db/src/entities/document.rs`
- `konto-db/src/entities/document_line_item.rs`
- `konto-db/src/repository/document_repo.rs`
- `konto-core/src/services/document_service.rs` — CRUD
- `konto-core/src/services/document_workflow.rs` — send/accept/reject/convert
- `konto-core/src/services/document_pdf.rs` — PDF generation
- `konto-api/src/dto/document.rs`
- `konto-api/src/handlers/document_handler.rs`

### 4.2 Document Conversion Flow

```
Quote/Offer  ──send──→  Sent  ──accept──→  Accepted
                                              │
                                     ┌────────┴────────┐
                                     ▼                  ▼
                               Convert to SOW    Convert to Project
                                     │
                                     ▼
                                SOW (draft)  ──send──→  Sent  ──sign──→  Signed
                                                                          │
                                                                 ┌────────┴────────┐
                                                                 ▼                  ▼
                                                           Convert to         Mark completed
                                                            Project
```

**Convert action:**
- Copies relevant fields (client, line items, content) to new document or project
- Sets `converted_from` reference on the new document
- Original document remains (linked, not modified)
- When converting SOW → Project: creates project with SOW line items as time budget

### 4.3 Frontend — Document Pages

**Pages:**
- `/documents` — list all documents with type tabs (quotes, offers, SOWs, contracts)
- `/documents/new?type=quote` — create from template using editor
- `/documents/{id}` — detail view with status actions + PDF preview
- `/documents/{id}/edit` — edit in editor
- `/documents/{id}/pdf` — full-screen PDF preview

**Files:**
- `pages/DocumentsPage.tsx`
- `pages/DocumentCreatePage.tsx`
- `pages/DocumentDetailPage.tsx`
- `pages/DocumentEditPage.tsx`
- `pages/DocumentPdfPage.tsx`
- `components/document/DocumentForm.tsx` — metadata (client, dates, line items)
- `components/document/DocumentStatusActions.tsx` — action buttons per status
- `components/document/DocumentLineItems.tsx` — reusable line-items table (shared with invoices)
- `components/document/DocumentConvertDialog.tsx` — conversion confirmation

---

## Phase 5: Invoice Template Integration

### 5.1 Upgrade Existing Invoices

Extend existing invoice system to use the editor + templates:
- Add `content_json` column to `invoices` table (optional, for custom layout)
- Add `template_id` column to `invoices` table
- When generating invoice PDF, merge template + invoice data + letterhead
- Existing invoice CRUD remains unchanged; editor is an enhancement

**Migration: `m20240101_000014_add_invoice_template.rs`**
```
ALTER invoices ADD template_id UUID FK → document_templates (nullable)
ALTER invoices ADD content_json TEXT (nullable)
```

### 5.2 Default Invoice Templates

Seed 1-2 default invoice templates matching current invoice format:
- Header: logo + company info
- Body: invoice number, client address, line items table, totals
- Footer: bank details, VAT number, payment terms

---

## Phase 6: Sidebar Navigation Update

Add new sections to the app sidebar:

```
Dashboard
─────────────
Accounting
  Accounts
  Journal
  Reports ▸
─────────────
Sales
  Invoices
  Documents ▸    ← NEW (Quotes, Offers, SOWs, Contracts)
─────────────
CRM
  Contacts
  Projects
  Time Entries
─────────────
Settings         ← NEW
  Company
  Bank Accounts
  Templates
  Letterhead
```

---

## Implementation Order

| Step | What | Depends On | Estimate |
|------|------|-----------|----------|
| 1 | Company settings backend (tables, API) | — | |
| 2 | Bank accounts backend (tables, API) | Step 1 | |
| 3 | Settings frontend pages | Steps 1-2 | |
| 4 | Editor core — block system, contentEditable, formatting | — | |
| 5 | Editor — A4 pagination, page renderer | Step 4 | |
| 6 | Editor — toolbar, format bar, slash menu, inspector | Step 4 | |
| 7 | Editor — table block, image block | Step 4 | |
| 8 | Editor — header/footer zones | Steps 4-5 | |
| 9 | Editor — placeholder system | Step 4 | |
| 10 | PDF generation (React-PDF or html2canvas approach) | Steps 4-8 | |
| 11 | Template backend (tables, API) | Step 1 | |
| 12 | Template frontend + letterhead editor | Steps 4-11 | |
| 13 | Documents backend (tables, API, workflow) | Steps 1-2, 11 | |
| 14 | Documents frontend (CRUD, editor integration) | Steps 4-13 | |
| 15 | Document conversion (offer→SOW→project) | Step 13 | |
| 16 | Invoice template integration | Steps 10-12 | |
| 17 | Sidebar navigation update | Steps 3, 14 | |
| 18 | Seed default templates (invoice, quote, SOW, contract) | Steps 11-12 | |

---

## New Database Tables (this sprint)

| # | Table | Purpose |
|---|-------|---------|
| 20 | company_settings | Legal entity info, VAT, logo |
| 21 | bank_accounts | IBAN, BIC, linked ledger account |
| 22 | document_templates | Template content + page setup |
| 23 | documents | Quotes, offers, SOWs, contracts |
| 24 | document_line_items | Line items for documents |

Total after sprint: **24 tables**

---

## New API Endpoints (this sprint)

| Method | Path | Purpose |
|--------|------|---------|
| GET | /api/v1/settings | Get company settings |
| PUT | /api/v1/settings | Update company settings |
| POST | /api/v1/settings/logo | Upload company logo |
| GET | /api/v1/bank-accounts | List bank accounts |
| POST | /api/v1/bank-accounts | Create bank account |
| PUT | /api/v1/bank-accounts/{id} | Update bank account |
| DELETE | /api/v1/bank-accounts/{id} | Delete bank account |
| GET | /api/v1/templates | List templates |
| GET | /api/v1/templates/{id} | Get template |
| POST | /api/v1/templates | Create template |
| PUT | /api/v1/templates/{id} | Update template |
| DELETE | /api/v1/templates/{id} | Delete template |
| POST | /api/v1/templates/{id}/duplicate | Clone template |
| POST | /api/v1/documents | Create document |
| GET | /api/v1/documents | List documents |
| GET | /api/v1/documents/{id} | Get document |
| PUT | /api/v1/documents/{id} | Update document |
| DELETE | /api/v1/documents/{id} | Delete document |
| POST | /api/v1/documents/{id}/send | Send/finalize |
| POST | /api/v1/documents/{id}/accept | Accept/sign |
| POST | /api/v1/documents/{id}/reject | Reject |
| POST | /api/v1/documents/{id}/convert | Convert type |
| GET | /api/v1/documents/{id}/pdf | Generate PDF |

**23 new endpoints** (total API surface: ~40+ endpoints)

---

## New Frontend Pages (this sprint)

| Route | Page |
|-------|------|
| /settings | Settings hub |
| /settings/company | Company settings |
| /settings/bank-accounts | Bank accounts |
| /settings/templates | Template list |
| /settings/templates/new | Template editor |
| /settings/templates/{id} | Edit template |
| /settings/letterhead | Letterhead editor |
| /documents | Document list |
| /documents/new | Create document |
| /documents/{id} | Document detail |
| /documents/{id}/edit | Edit document |
| /documents/{id}/pdf | PDF preview |

**12 new pages** (total: ~25 pages)

---

## Technical Notes

### Editor: Why Custom contentEditable (not TipTap/ProseMirror)
The wordlike-app already has a proven custom implementation with:
- Direct DOM control for precise A4 pagination
- Minimal bundle size (no large editor framework)
- Full control over block behavior and rendering
- The port to React uses `useRef` + `useEffect` for imperative DOM — same pattern

### PDF: Recommended Approach
Evaluate in order of preference:
1. **`@react-pdf/renderer`** — pure React, precise layout, no browser dependency
2. **`html2canvas` + `jsPDF`** — screenshot approach, pixel-perfect but heavier
3. **Server-side with `printpdf` (Rust)** — if client-side proves insufficient

### Template Variables: Resolved Lazily
- Templates store `{{placeholder}}` in content_json
- On document creation from template, placeholders remain until user fills in data or system resolves them
- PDF export resolves all remaining placeholders from DB data (client, company, totals)
- Unresolved placeholders show as highlighted warnings in the editor

### File Size Compliance
All files kept under 300 lines (500 max) per project convention. The editor is split into many small components intentionally.
