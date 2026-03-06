# Sprint 17: Rich Text / Markdown for Description Fields

## Goal
Replace plain text inputs with inline WYSIWYG Markdown editors for all description/notes fields. Store content as Markdown internally. Migrate existing HTML data (from legacy imports) to Markdown. Render Markdown as HTML in frontend and as formatted text in Typst PDF generation.

## Context
Legacy exports contain HTML markup (`<br />`, `<ul><li>`, `<strong>`, etc.) in description fields. Currently these are stored as-is and rendered as plain text, losing formatting. Users need lightweight rich text editing (bold, italic, lists, line breaks) without a full block editor.

## Architecture Decisions

### TD-058: Markdown as Internal Rich Text Format
- **Decision**: Store all description/notes fields as Markdown. Convert existing HTML to Markdown via migration. Frontend uses inline WYSIWYG Markdown editor, backend converts Markdown to Typst markup for PDFs.
- **Rationale**: Markdown is human-readable in raw form, lightweight, well-supported by libraries. Avoids HTML sanitization complexity. Clean separation: storage (Markdown) -> display (HTML via frontend) -> PDF (Typst markup via backend).

### TD-059: TipTap Inline Editor for Markdown Fields
- **Decision**: Use TipTap (headless ProseMirror wrapper) with `@tiptap/starter-kit` + `@tiptap/extension-placeholder` for inline WYSIWYG editing. Minimal toolbar (bold, italic, bullet list, ordered list). Export as Markdown via tiptap-markdown extension.
- **Rationale**: TipTap is the most mature headless rich text editor for React. Starter kit includes bold/italic/lists/headings out of the box. Bubble menu provides formatting without permanent toolbar. Lightweight compared to full editors.
- **Note**: This is for inline description fields, NOT the existing block-based document editor (TD-007) which remains unchanged.

### TD-060: HTML-to-Markdown Migration via Rust
- **Decision**: Backend migration (M088) converts all HTML content in description/notes columns to Markdown using the `html2md` Rust crate. Runs once on upgrade.
- **Rationale**: One-time migration keeps data clean. `html2md` handles `<br>`, `<ul>/<li>`, `<strong>`, `<em>` — all patterns found in legacy exports.

## Phase 1: Backend — Migration & Markdown Support

### Migration M088: Convert HTML to Markdown
- Targets columns (13 columns across 10 tables):
  - `time_entries.description`
  - `projects.description`
  - `project_items.description`
  - `project_milestones.description`
  - `contacts.notes`
  - `invoice_lines.description`
  - `credit_note_lines.description`
  - `document_line_items.description`
  - `invoices.notes`, `invoices.header_text`, `invoices.footer_text`
  - `credit_notes.notes`
  - `employees.notes`
- Logic: SELECT all rows, run `html2md::parse_html()`, UPDATE if changed
- Safety: skip NULL/empty, only update if output differs from input

### Backend Crate Changes
- **konto-core/Cargo.toml**: Add `html2md = "0.2"` (migration only)
- **konto-migration/Cargo.toml**: Add `html2md = "0.2"`
- **pdf_invoice.rs**: Add `md_to_typst()` helper — converts Markdown to Typst markup:
  - `**bold**` -> `*bold*` (Typst bold)
  - `*italic*` -> `_italic_` (Typst italic)
  - `- item` -> `- item` (Typst lists, same syntax)
  - `1. item` -> `+ item` (Typst numbered lists)
  - Line breaks preserved
  - Plain text passthrough (backward compatible)
- **pdf_annual_report.rs**: Same `md_to_typst()` for any notes fields rendered in PDF

### Shared Utility
- **konto-common/src/markdown.rs**: `md_to_typst(input: &str) -> String` function, reusable by all PDF generators

## Phase 2: Frontend — RichTextEditor Component

### New Dependencies
- `@tiptap/react` — React bindings
- `@tiptap/starter-kit` — bold, italic, lists, headings, blockquote, code
- `@tiptap/extension-placeholder` — placeholder text
- `tiptap-markdown` — Markdown serialization/deserialization

### New Component: `RichTextEditor`
- **Path**: `frontend/src/components/ui/rich-text-editor.tsx`
- **Props**: `value: string` (Markdown), `onChange: (md: string) => void`, `placeholder?: string`, `minimal?: boolean`, `className?: string`
- **Features**:
  - Bubble menu on text selection: Bold, Italic, Bullet List, Ordered List
  - Renders Markdown content on load
  - Outputs Markdown on change
  - `minimal` mode: single-line feel, no headings (for invoice line descriptions)
  - Matches shadcn Input/Textarea styling (border, focus ring, padding)
  - Keyboard shortcuts: Cmd+B (bold), Cmd+I (italic)

### New Component: `MarkdownPreview`
- **Path**: `frontend/src/components/ui/markdown-preview.tsx`
- **Props**: `content: string` (Markdown), `className?: string`
- **Features**:
  - Read-only Markdown-to-HTML rendering for display contexts (detail pages, tables)
  - Uses TipTap in read-only mode (consistent rendering) or simple regex-based converter
  - Renders inline: bold, italic, lists, line breaks
  - Sanitized output (no script injection)

## Phase 3: Frontend Integration — Replace Textarea/Input Fields

### Invoice Form (`InvoiceForm.tsx`)
- `notes` field: Textarea -> RichTextEditor
- `payment_terms` field: Textarea -> RichTextEditor
- `header_text` field: Textarea -> RichTextEditor
- `footer_text` field: Textarea -> RichTextEditor
- Line item `description`: Input -> RichTextEditor (minimal mode)

### Project Components
- `ProjectEditDialog.tsx`: description Textarea -> RichTextEditor
- `ProjectItemDialog.tsx`: description Textarea -> RichTextEditor
- `MilestoneDialog.tsx`: description Textarea -> RichTextEditor

### Contact Components
- `ContactOverview.tsx`: notes Textarea -> RichTextEditor

### Credit Notes
- `CreditNoteForm.tsx`: notes Textarea -> RichTextEditor
- Line item `description`: Input -> RichTextEditor (minimal mode)

### Time Entries
- Time entry description field: Input -> RichTextEditor (minimal mode)

### Employee
- `EmployeesPage.tsx` / editor: notes field -> RichTextEditor

### Display/Read-Only Contexts
- `ProjectDetailPage.tsx`: render description with MarkdownPreview
- `ContactDetailPage.tsx`: render notes with MarkdownPreview
- Invoice detail/PDF preview: render notes/terms with MarkdownPreview
- Table cells with descriptions: truncated MarkdownPreview (strip formatting, show plain text preview)

## Phase 4: i18n

### New Keys (4 languages)
- `rich_text.bold` / `rich_text.italic` / `rich_text.bullet_list` / `rich_text.ordered_list`
- `rich_text.placeholder_description` / `rich_text.placeholder_notes`

## Implementation Order
1. Add `html2md` to migration crate, write M088
2. Add `md_to_typst()` to konto-common
3. Update pdf_invoice.rs to use md_to_typst for description fields
4. Install TipTap npm packages
5. Build RichTextEditor + MarkdownPreview components
6. Replace fields in InvoiceForm (highest visibility)
7. Replace fields in Project/Contact/CreditNote/TimeEntry/Employee components
8. Add MarkdownPreview to all detail/read-only views
9. Add i18n keys
10. Test PDF generation with Markdown content

## Files Changed

### New Files (4)
- `backend/crates/konto-migration/src/m20240101_000088_html_to_markdown.rs`
- `backend/crates/konto-common/src/markdown.rs`
- `frontend/src/components/ui/rich-text-editor.tsx`
- `frontend/src/components/ui/markdown-preview.tsx`

### Modified Files (~15)
- `backend/crates/konto-migration/Cargo.toml` — html2md dep
- `backend/crates/konto-migration/src/lib.rs` — register M088
- `backend/crates/konto-common/Cargo.toml` — (if needed)
- `backend/crates/konto-common/src/lib.rs` — pub mod markdown
- `backend/crates/konto-core/src/services/pdf_invoice.rs` — md_to_typst usage
- `backend/crates/konto-core/src/services/pdf_annual_report.rs` — md_to_typst usage (if notes rendered)
- `frontend/package.json` — TipTap dependencies
- `frontend/src/components/invoice/InvoiceForm.tsx`
- `frontend/src/components/projects/ProjectEditDialog.tsx`
- `frontend/src/components/contacts/ContactOverview.tsx`
- `frontend/src/pages/ProjectDetailPage.tsx`
- `frontend/src/pages/ContactDetailPage.tsx`
- `frontend/src/components/credit-notes/CreditNoteForm.tsx`
- `frontend/src/i18n/messages.ts`

## Risk & Notes
- TipTap adds ~80KB gzipped to bundle — acceptable for the functionality
- Existing plain text data renders fine in TipTap (treated as paragraph)
- Migration is idempotent: running on already-converted data is a no-op
- The document editor (TD-007) is NOT affected — it uses its own block-based JSON format
