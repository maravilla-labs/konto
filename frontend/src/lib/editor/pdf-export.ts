import type { DocumentModel } from './types';
import { resolveAll, type PlaceholderContext } from './placeholder-resolver';

/**
 * Export document as PDF via browser print dialog.
 * Opens a new window with the print view and triggers print.
 */
export function printDocument(containerId: string): void {
  const container = document.getElementById(containerId);
  if (!container) return;

  const printWindow = window.open('', '_blank');
  if (!printWindow) return;

  // Clone styles from main document
  const styles = Array.from(document.styleSheets)
    .map((sheet) => {
      try {
        return Array.from(sheet.cssRules)
          .map((r) => r.cssText)
          .join('\n');
      } catch {
        return '';
      }
    })
    .join('\n');

  printWindow.document.write(`
    <!DOCTYPE html>
    <html>
    <head>
      <style>
        ${styles}
        ${PRINT_CSS}
      </style>
    </head>
    <body class="print-view">
      ${container.innerHTML}
    </body>
    </html>
  `);
  printWindow.document.close();

  // Wait for images to load, then print
  printWindow.onload = () => {
    setTimeout(() => {
      printWindow.print();
      printWindow.close();
    }, 250);
  };
}

/**
 * Prepare document for PDF: resolve placeholders and return finalized model.
 */
export function prepareForPdf(
  doc: DocumentModel,
  ctx: PlaceholderContext,
): DocumentModel {
  return {
    ...doc,
    blocks: resolveAll(doc.blocks, ctx),
    header: resolveAll(doc.header, ctx),
    footer: resolveAll(doc.footer, ctx),
  };
}

/** Print CSS that ensures 1:1 match with editor */
export const PRINT_CSS = `
@media print {
  @page {
    size: A4;
    margin: 0;
  }

  body {
    margin: 0;
    padding: 0;
    -webkit-print-color-adjust: exact;
    print-color-adjust: exact;
  }

  .print-view {
    background: white;
  }

  .editor-toolbar,
  .block-inspector,
  .format-bar,
  .slash-menu,
  .page-gap,
  .no-print {
    display: none !important;
  }

  .editor-page {
    width: 794px;
    min-height: 1123px;
    page-break-after: always;
    break-after: page;
    box-shadow: none;
    margin: 0;
    padding: inherit;
  }

  .editor-page:last-child {
    page-break-after: auto;
    break-after: auto;
  }

  /* Keep-with-next blocks */
  [data-keep-with-next="true"] {
    break-after: avoid;
    page-break-after: avoid;
  }

  /* Page break before */
  [data-page-break-before="true"] {
    break-before: page;
    page-break-before: always;
  }

  /* Tables don't break across pages */
  table {
    break-inside: avoid;
    page-break-inside: avoid;
  }

  /* Signature blocks stay together */
  .signature-block {
    break-inside: avoid;
    page-break-inside: avoid;
  }

  /* Invoice table styling */
  .invoice-table {
    width: 100%;
    border-collapse: collapse;
    break-inside: avoid;
    page-break-inside: avoid;
  }

  .invoice-table th {
    background: #f9fafb;
    font-size: 10px;
    font-weight: 500;
    text-transform: uppercase;
    color: #6b7280;
    padding: 6px 8px;
    border-bottom: 2px solid #e5e7eb;
  }

  .invoice-table td {
    padding: 4px 8px;
    font-size: 13px;
    border-bottom: 1px solid #f3f4f6;
  }

  .invoice-table .text-right {
    text-align: right;
  }

  .invoice-table tfoot td {
    font-weight: 600;
    border-top: 2px solid #d1d5db;
    padding-top: 8px;
  }

  /* Contact info block */
  .contact-info-block {
    break-inside: avoid;
    page-break-inside: avoid;
  }

  /* Doc meta block */
  .doc-meta-block {
    break-inside: avoid;
    page-break-inside: avoid;
  }

  .doc-meta-block .label {
    color: #6b7280;
    font-size: 11px;
  }

  .doc-meta-block .value {
    color: #111827;
    font-size: 13px;
  }
}
`;
