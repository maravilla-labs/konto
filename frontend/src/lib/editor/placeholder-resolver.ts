import type {
  Block,
  TextBlockData,
  QuoteBlockData,
  SignatureBlockData,
  PlaceholderData,
  ContactInfoData,
  DocMetaData,
} from './types';
import { resolveInlinePlaceholders } from './inline-placeholder';

export interface PlaceholderContext {
  company_name?: string;
  company_address?: string;
  company_vat?: string;
  company_logo?: string;
  company_email?: string;
  company_phone?: string;
  company_website?: string;
  bank_name?: string;
  bank_iban?: string;
  bank_bic?: string;
  client_name?: string;
  client_contact?: string;
  client_address?: string;
  client_email?: string;
  customer_number?: string;
  doc_number?: string;
  doc_date?: string;
  doc_valid_until?: string;
  doc_title?: string;
  subtotal?: string;
  vat_rate?: string;
  vat_amount?: string;
  total?: string;
  current_date?: string;
  page_number?: string;
  total_pages?: string;
  [key: string]: string | undefined;
}

/** Resolve all {{placeholder}} variables in a string */
export function resolveString(
  text: string,
  ctx: PlaceholderContext,
): string {
  return text.replace(/\{\{(\w+)\}\}/g, (match, key: string) => {
    return ctx[key] ?? match;
  });
}

/** Resolve placeholders in a block's data (returns new block, doesn't mutate) */
export function resolveBlock(
  block: Block,
  ctx: PlaceholderContext,
): Block {
  const resolved: Block = { ...block, data: { ...block.data } };

  switch (block.type) {
    case 'h1':
    case 'h2':
    case 'h3':
    case 'p': {
      const data = resolved.data as TextBlockData;
      data.text = resolveString(data.text, ctx);
      data._html = resolveInlinePlaceholders(resolveString(data._html, ctx), ctx);
      break;
    }
    case 'placeholder': {
      const data = resolved.data as PlaceholderData;
      data.resolved = ctx[data.variable];
      break;
    }
    case 'signature': {
      const data = resolved.data as SignatureBlockData;
      data.parties = data.parties.map((p) => ({
        ...p,
        company: resolveString(p.company, ctx),
        location: resolveString(p.location, ctx),
        date: resolveString(p.date, ctx),
        lines: p.lines.map((l) => ({
          label: l.label,
          value: resolveString(l.value, ctx),
        })),
      }));
      break;
    }
    case 'blockquote': {
      const data = resolved.data as QuoteBlockData;
      if (data.quote) data.quote = resolveString(data.quote, ctx);
      if (data.author) data.author = resolveString(data.author, ctx);
      break;
    }
    case 'contact_info': {
      const data = resolved.data as ContactInfoData;
      data.lines = data.lines.map((line) => resolveString(line, ctx));
      break;
    }
    case 'doc_meta': {
      const data = resolved.data as DocMetaData;
      data.projectName = resolveString(data.projectName, ctx);
      break;
    }
  }

  return resolved;
}

/** Resolve all blocks in a document */
export function resolveAll(
  blocks: Block[],
  ctx: PlaceholderContext,
): Block[] {
  return blocks.map((b) => resolveBlock(b, ctx));
}

/** Extract all placeholder variable names from blocks */
export function extractPlaceholders(blocks: Block[]): string[] {
  const vars = new Set<string>();
  const regex = /\{\{(\w+)\}\}/g;

  function scan(text: string) {
    let match;
    while ((match = regex.exec(text)) !== null) {
      vars.add(match[1]);
    }
  }

  for (const block of blocks) {
    const data = block.data as unknown as Record<string, unknown>;

    if (typeof data.text === 'string') scan(data.text);
    if (typeof data._html === 'string') scan(data._html);
    if (typeof data.quote === 'string') scan(data.quote);
    if (typeof data.author === 'string') scan(data.author);
    if (typeof data.variable === 'string') vars.add(data.variable as string);

    if (Array.isArray(data.parties)) {
      for (const p of data.parties as Array<{
        company: string;
        location: string;
        date: string;
        lines: Array<{ value: string }>;
      }>) {
        scan(p.company);
        scan(p.location);
        scan(p.date);
        for (const l of p.lines) scan(l.value);
      }
    }
  }

  return Array.from(vars);
}
