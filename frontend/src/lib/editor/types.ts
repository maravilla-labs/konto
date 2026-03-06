export type BlockType =
  | 'h1'
  | 'h2'
  | 'h3'
  | 'p'
  | 'blockquote'
  | 'table'
  | 'image'
  | 'divider'
  | 'placeholder'
  | 'signature'
  | 'spacer'
  | 'contact_info'
  | 'doc_meta';

export interface Block {
  id: string;
  type: BlockType;
  data: BlockData;
  meta: BlockMeta;
}

export interface BlockMeta {
  fontSize: number | null;
  align: 'left' | 'center' | 'right' | 'justify';
  lineHeight: number;
  font: 'system' | 'serif' | 'mono';
  keepWithNext: boolean;
  locked: boolean;
  pageBreakBefore: boolean;
}

export interface TextBlockData {
  text: string;
  _html: string;
}

export interface QuoteBlockData {
  quote: string;
  author: string;
  _quoteHtml: string;
}

export interface TableBlockData {
  headers: string[];
  rows: string[][];
  columnWidths?: number[];
  mode?: 'generic' | 'invoice';
}

export interface ContactInfoData {
  contactId: string;
  lines: string[];
}

export interface DocMetaData {
  docDate: string;
  validUntil: string;
  projectId: string;
  projectName: string;
  docNumber: string;
}

export interface ImageBlockData {
  src: string;
  alt: string;
  width: number;
  height: number;
}

export interface PlaceholderData {
  variable: string;
  resolved?: string;
}

export interface SignatureBlockData {
  parties: SignatureParty[];
}

export interface SignatureParty {
  role: string;
  company: string;
  location: string;
  date: string;
  lines: { label: string; value: string }[];
}

export interface SpacerData {
  height: number;
}

export type BlockData =
  | TextBlockData
  | QuoteBlockData
  | TableBlockData
  | ImageBlockData
  | PlaceholderData
  | SignatureBlockData
  | SpacerData
  | ContactInfoData
  | DocMetaData;

export interface DocumentModel {
  id: string;
  blocks: Block[];
  pageSetup: PageSetup;
  header: Block[];
  footer: Block[];
}

export interface PageSetup {
  format: 'a4' | 'letter';
  orientation: 'portrait' | 'landscape';
  margins: { top: number; right: number; bottom: number; left: number };
  headerHeight: number;
  footerHeight: number;
}

export type ViewMode = 'pages' | 'spread' | 'canvas';

export interface ActiveState {
  blockId: string | null;
  field: 'main' | 'quote' | 'author' | 'title';
  caret: number;
}

export const PAGE_W = 794; // A4 width at 96dpi
export const PAGE_H = 1123; // A4 height at 96dpi

export const DEFAULT_PAGE_SETUP: PageSetup = {
  format: 'a4',
  orientation: 'portrait',
  margins: { top: 20, right: 20, bottom: 20, left: 20 },
  headerHeight: 30,
  footerHeight: 25,
};

export const DEFAULT_META: BlockMeta = {
  fontSize: null,
  align: 'left',
  lineHeight: 1.5,
  font: 'system',
  keepWithNext: false,
  locked: false,
  pageBreakBefore: false,
};
