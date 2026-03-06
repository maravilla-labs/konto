import type {
  Block,
  BlockType,
  BlockData,
  BlockMeta,
  DocumentModel,
  TextBlockData,
  TableBlockData,
  ImageBlockData,
  PlaceholderData,
  SignatureBlockData,
  SpacerData,
  ContactInfoData,
  DocMetaData,
} from './types';
import { DEFAULT_META, DEFAULT_PAGE_SETUP } from './types';

export const INVOICE_HEADERS = ['Description', 'Qty', 'Unit', 'Price', 'Disc%', 'Total'];
export const EMPTY_INVOICE_ROW = ['', '1', '', '', '0', '0.00'];

let counter = 0;

export function generateBlockId(): string {
  return 'b' + ++counter + '_' + Math.random().toString(36).slice(2, 7);
}

function getDefaultData(
  type: BlockType,
  partial?: Partial<BlockData>,
): BlockData {
  switch (type) {
    case 'h1':
    case 'h2':
    case 'h3':
    case 'p':
      return { text: '', _html: '', ...partial } as TextBlockData;
    case 'blockquote':
      return { text: '', _html: '', ...partial } as TextBlockData;
    case 'table':
      return {
        headers: ['Column 1', 'Column 2', 'Column 3'],
        rows: [['', '', '']],
        mode: 'generic',
        ...partial,
      } as TableBlockData;
    case 'image':
      return {
        src: '',
        alt: '',
        width: 400,
        height: 300,
        ...partial,
      } as ImageBlockData;
    case 'divider':
      return { text: '', _html: '' } as TextBlockData;
    case 'placeholder':
      return {
        variable: 'variable_name',
        ...partial,
      } as PlaceholderData;
    case 'signature':
      return {
        parties: [
          {
            role: 'Service Provider',
            company: '{{company_name}}',
            location: '',
            date: '{{current_date}}',
            lines: [
              { label: 'Name:', value: '' },
              { label: 'Title:', value: '' },
            ],
          },
          {
            role: 'Client',
            company: '{{client_name}}',
            location: '',
            date: '',
            lines: [
              { label: 'Name:', value: '' },
              { label: 'Title:', value: '' },
            ],
          },
        ],
        ...partial,
      } as SignatureBlockData;
    case 'spacer':
      return { height: 40, ...partial } as SpacerData;
    case 'contact_info':
      return { contactId: '', lines: [], ...partial } as ContactInfoData;
    case 'doc_meta':
      return {
        docDate: new Date().toISOString().slice(0, 10),
        validUntil: '',
        projectId: '',
        projectName: '',
        docNumber: 'DRAFT',
        ...partial,
      } as DocMetaData;
  }
}

export function createInvoiceTable(): Block {
  return createBlock('table', {
    headers: [...INVOICE_HEADERS],
    rows: [[...EMPTY_INVOICE_ROW]],
    mode: 'invoice',
  } as Partial<TableBlockData>);
}

export function createBlock(
  type: BlockType,
  data?: Partial<BlockData>,
  metaOverrides?: Partial<BlockMeta>,
): Block {
  const meta: BlockMeta = { ...DEFAULT_META, ...metaOverrides };

  // For headings, set keepWithNext by default
  if (type === 'h1' || type === 'h2' || type === 'h3') {
    meta.keepWithNext = true;
  }

  return {
    id: generateBlockId(),
    type,
    data: getDefaultData(type, data),
    meta,
  };
}

export function createDefaultDocument(): DocumentModel {
  return {
    id: generateBlockId(),
    blocks: [createBlock('h1'), createBlock('p')],
    pageSetup: { ...DEFAULT_PAGE_SETUP },
    header: [],
    footer: [],
  };
}
