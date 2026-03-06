import type {
  Block,
  DocumentModel,
  TextBlockData,
  ContactInfoData,
  DocMetaData,
  TableBlockData,
} from './types';
import type { Contact } from '@/types/contacts';
import type { Project } from '@/types/projects';
import type { CreateDocumentLine } from '@/types/document';
import { createBlock, INVOICE_HEADERS } from './block-factory';

/** Extract document metadata from blocks for API payload */
export function extractDocMeta(blocks: Block[]): {
  title: string;
  contactId: string;
  projectId: string;
  validUntil: string;
} {
  let title = '';
  let contactId = '';
  let projectId = '';
  let validUntil = '';

  for (const b of blocks) {
    if (b.type === 'h1' && !title) {
      title = (b.data as TextBlockData).text;
    }
    if (b.type === 'contact_info') {
      contactId = (b.data as ContactInfoData).contactId;
    }
    if (b.type === 'doc_meta') {
      const meta = b.data as DocMetaData;
      projectId = meta.projectId;
      validUntil = meta.validUntil;
    }
  }

  return { title, contactId, projectId, validUntil };
}

/** Extract invoice lines from invoice-mode table blocks */
export function extractLines(blocks: Block[]): CreateDocumentLine[] {
  const lines: CreateDocumentLine[] = [];
  for (const b of blocks) {
    if (b.type !== 'table') continue;
    const tbl = b.data as TableBlockData;
    if (tbl.mode !== 'invoice') continue;
    for (const row of tbl.rows) {
      if (!row[0] && !row[3]) continue; // skip empty rows
      lines.push({
        description: row[0] || '',
        quantity: row[1] || '0',
        unit: row[2] || undefined,
        unit_price: row[3] || '0',
        discount_pct: row[4] || '0',
      });
    }
  }
  return lines;
}

/** Resolve contact fields into display lines */
export function resolveContactLines(contact: Contact): string[] {
  const lines: string[] = [];
  if (contact.name2) lines.push(contact.name2);
  lines.push(contact.name1);
  if (contact.address) lines.push(contact.address);
  const cityLine = [contact.postal_code, contact.city].filter(Boolean).join(' ');
  if (cityLine) lines.push(cityLine);
  if (contact.country && contact.country !== 'CH') lines.push(contact.country);
  return lines;
}

/** Calculate row total: qty * price * (1 - disc/100) */
export function calcRowTotal(row: string[]): string {
  const qty = parseFloat(row[1] || '0');
  const price = parseFloat(row[3] || '0');
  const disc = parseFloat(row[4] || '0');
  return (qty * price * (1 - disc / 100)).toFixed(2);
}

/** Inject API data into document blocks (for loading existing documents) */
export function injectDocMeta(
  doc: DocumentModel,
  apiData: { contact_id: string; project_id?: string | null; valid_until?: string | null; doc_number?: string | null; lines?: { description: string; quantity: string; unit?: string | null; unit_price: string; discount_pct: string; total: string }[] },
  contacts: Contact[],
  projects: Project[],
): DocumentModel {
  const blocks = [...doc.blocks];
  const contact = contacts.find((c) => c.id === apiData.contact_id);
  const project = apiData.project_id ? projects.find((p) => p.id === apiData.project_id) : undefined;

  // Ensure contact_info block exists
  let hasContactInfo = blocks.some((b) => b.type === 'contact_info');
  if (!hasContactInfo) {
    const insertIdx = blocks.findIndex((b) => b.type === 'h1');
    const contactBlock = createBlock('contact_info', {
      contactId: apiData.contact_id,
      lines: contact ? resolveContactLines(contact) : [],
    });
    blocks.splice(insertIdx === -1 ? 0 : insertIdx, 0, contactBlock);
    hasContactInfo = true;
  } else {
    // Update existing contact_info block
    for (let i = 0; i < blocks.length; i++) {
      if (blocks[i].type === 'contact_info') {
        blocks[i] = { ...blocks[i], data: { contactId: apiData.contact_id, lines: contact ? resolveContactLines(contact) : [] } as ContactInfoData };
      }
    }
  }

  // Ensure doc_meta block exists
  let hasDocMeta = blocks.some((b) => b.type === 'doc_meta');
  if (!hasDocMeta) {
    const ciIdx = blocks.findIndex((b) => b.type === 'contact_info');
    const metaBlock = createBlock('doc_meta', {
      docDate: new Date().toISOString().slice(0, 10),
      validUntil: apiData.valid_until ?? '',
      projectId: apiData.project_id ?? '',
      projectName: project?.name ?? '',
      docNumber: apiData.doc_number ?? 'DRAFT',
    });
    blocks.splice(ciIdx === -1 ? 1 : ciIdx + 1, 0, metaBlock);
    hasDocMeta = true;
  } else {
    for (let i = 0; i < blocks.length; i++) {
      if (blocks[i].type === 'doc_meta') {
        blocks[i] = { ...blocks[i], data: { docDate: new Date().toISOString().slice(0, 10), validUntil: apiData.valid_until ?? '', projectId: apiData.project_id ?? '', projectName: project?.name ?? '', docNumber: apiData.doc_number ?? 'DRAFT' } as DocMetaData };
      }
    }
  }

  // Inject invoice lines into invoice-mode table
  if (apiData.lines && apiData.lines.length > 0) {
    let hasInvoiceTable = blocks.some((b) => b.type === 'table' && (b.data as TableBlockData).mode === 'invoice');
    if (!hasInvoiceTable) {
      const invoiceBlock = createBlock('table', {
        headers: [...INVOICE_HEADERS],
        rows: apiData.lines.map((l) => [l.description, l.quantity, l.unit ?? '', l.unit_price, l.discount_pct, l.total]),
        mode: 'invoice',
      } as Partial<TableBlockData>);
      // Insert before signature or at end
      const sigIdx = blocks.findIndex((b) => b.type === 'signature');
      blocks.splice(sigIdx === -1 ? blocks.length : sigIdx, 0, invoiceBlock);
    } else {
      for (let i = 0; i < blocks.length; i++) {
        if (blocks[i].type === 'table' && (blocks[i].data as TableBlockData).mode === 'invoice') {
          const tbl = blocks[i].data as TableBlockData;
          blocks[i] = { ...blocks[i], data: { ...tbl, rows: apiData.lines.map((l) => [l.description, l.quantity, l.unit ?? '', l.unit_price, l.discount_pct, l.total]) } };
        }
      }
    }
  }

  return { ...doc, blocks };
}
