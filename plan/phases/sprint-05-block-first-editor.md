# Sprint 05 — True Block-First WYSIWYG Editor

## Review of Progress

### Sprint 01 (Foundation) — COMPLETE
Delivered: Monorepo scaffold, 17-table schema, JWT+RBAC auth, REST API with OpenAPI docs, CSV/XLSX import engine.

### Sprint 02 (Financial Core & Reporting) — COMPLETE
Delivered: 5 financial reports, fiscal year management, exchange rates, journal post/reverse workflow, dashboard KPIs.

### Sprint 03 (Invoicing & Bug Fixes) — COMPLETE
Delivered: Full invoice lifecycle (draft→sent→paid→cancelled), 6 API contract fixes, invoice number format RE-YYYY-NNN.

### Sprint 04 (Documents, Settings & Editor) — COMPLETE
Delivered: Company settings, bank accounts, document templates, word-like editor, documents with workflow, PDF via browser print.

---

## Sprint 05 Goals

**Theme:** Transform the document editor from a text-focused editor into a true block-first system where everything on the A4 page is a block — client information, document metadata, invoice tables, signatures.

---

## Phase 1: New Block Types

### 1.1 Contact Info Block (`contact_info`)
- Renders selected contact's name, address, postal code, city, country
- Data type: `ContactInfoData { contactId: string }`
- Block renderer: `ContactInfoBlock.tsx` (forwardRef)
- Inspector panel: `ContactPanel.tsx` — contact search/selection

### 1.2 Document Meta Block (`doc_meta`)
- Renders document number, date, due date, project name
- Data type: `DocMetaData { documentNumber, date, dueDate, projectId, title }`
- Block renderer: `DocMetaBlock.tsx` (forwardRef)
- Inspector panel: `DocMetaPanel.tsx` — date pickers, project dropdown

### 1.3 Invoice Table Mode
- Extended `TableBlockData` with `mode: 'generic' | 'invoice'`
- Invoice mode: fixed columns (Description, Qty, Price, VAT, Total) with calculation
- Block renderer: `InvoiceTableBlock.tsx` (forwardRef)
- Row total calculation: `calcRowTotal()` in doc-sync.ts

---

## Phase 2: Adaptive Inspector

### 2.1 Single Panel Routing
- Replaced 3-tab inspector (Format, Document, Items) with type-based routing
- Inspector automatically shows the correct panel for the selected block type
- 8 inspector panels: Text, Table, Contact, DocMeta, Image, Signature, Simple, Common

### 2.2 Editor Context
- `EditorContext` interface passes contacts, projects, templates, docType through editor
- Inspector panels access context data for dropdowns and lookups

---

## Phase 3: Document Sync Utilities

### 3.1 Block ↔ API Data Sync (`doc-sync.ts`)
- `extractDocMeta(blocks)` — extracts document metadata from blocks for API payload
- `extractLines(blocks)` — extracts invoice line items from table blocks
- `injectDocMeta(blocks, data)` — injects API data back into blocks on load
- `resolveContactLines(blocks, contacts)` — resolves contact data into contact_info blocks
- `calcRowTotal(row)` — calculates line total from qty × price

### 3.2 Create/Edit Page Integration
- No more separate metadata state in DocumentCreatePage/DocumentEditPage
- Metadata extracted from blocks on save, injected on load
- Single source of truth: the block array

---

## Phase 4: SlashMenu Updates

- Added 3 new items: Client Info, Document Info, Invoice Table
- `createInvoiceTable()` factory in block-factory.ts

---

## Files Changed

### New Files (12)
- `frontend/src/components/editor/blocks/ContactInfoBlock.tsx`
- `frontend/src/components/editor/blocks/DocMetaBlock.tsx`
- `frontend/src/components/editor/blocks/InvoiceTableBlock.tsx`
- `frontend/src/components/editor/inspector/ContactPanel.tsx`
- `frontend/src/components/editor/inspector/DocMetaPanel.tsx`
- `frontend/src/components/editor/inspector/ImagePanel.tsx`
- `frontend/src/components/editor/inspector/SignaturePanel.tsx`
- `frontend/src/components/editor/inspector/SimplePanel.tsx`
- `frontend/src/components/editor/inspector/TextPanel.tsx`
- `frontend/src/components/editor/inspector/TablePanel.tsx`
- `frontend/src/components/editor/inspector/CommonPanel.tsx`
- `frontend/src/lib/editor/doc-sync.ts`

### Modified Files (8)
- `frontend/src/lib/editor/types.ts` — ContactInfoData, DocMetaData, TableBlockData.mode
- `frontend/src/lib/editor/block-factory.ts` — createInvoiceTable, INVOICE_HEADERS, EMPTY_INVOICE_ROW
- `frontend/src/components/editor/DocumentEditor.tsx` — EditorContext provider
- `frontend/src/components/editor/EditorBlock.tsx` — new block renderers
- `frontend/src/components/editor/Inspector.tsx` — adaptive panel routing
- `frontend/src/components/editor/SlashMenu.tsx` — new menu items
- `frontend/src/pages/documents/DocumentCreatePage.tsx` — block-based metadata
- `frontend/src/pages/documents/DocumentEditPage.tsx` — block-based metadata

### Deleted Files (3)
- `frontend/src/components/editor/InspectorFormatTab.tsx`
- `frontend/src/components/editor/InspectorDocumentTab.tsx`
- `frontend/src/components/editor/InspectorItemsTab.tsx`

---

## Summary
- 0 new database tables (24 total unchanged)
- 0 new API endpoints
- 12 new frontend files, 8 modified, 3 deleted
- Editor now fully block-first: all document content represented as typed blocks
