import type { Block, TextBlockData, QuoteBlockData } from './types';

export function computeWordCount(blocks: Block[]): number {
  let count = 0;
  for (const block of blocks) {
    if (['h1', 'h2', 'h3', 'p'].includes(block.type)) {
      const text = (block.data as TextBlockData).text || '';
      count += text.split(/\s+/).filter(Boolean).length;
    }
    if (block.type === 'blockquote') {
      const data = block.data as QuoteBlockData;
      if (data.quote) count += data.quote.split(/\s+/).filter(Boolean).length;
      if (data.author) count += data.author.split(/\s+/).filter(Boolean).length;
    }
  }
  return count;
}

export function computeBlockCount(blocks: Block[]): number {
  return blocks.length;
}
