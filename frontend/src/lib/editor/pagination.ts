import type { Block, PageSetup } from './types';
import { PAGE_W, PAGE_H } from './types';

/** Convert mm to px at 96dpi */
function mmToPx(mm: number): number {
  return (mm / 25.4) * 96;
}

export interface PageLayout {
  pages: Block[][];
  pageHeight: number;
  contentHeight: number;
  contentWidth: number;
  margins: { top: number; right: number; bottom: number; left: number };
  headerHeight: number;
  footerHeight: number;
}

/**
 * Paginate blocks into pages based on measured heights.
 *
 * @param blocks - All document blocks
 * @param blockHeights - Map of block id to measured height in px
 * @param pageSetup - Page dimensions and margins
 * @returns PageLayout with blocks split into pages
 */
export function paginate(
  blocks: Block[],
  blockHeights: Map<string, number>,
  pageSetup: PageSetup,
): PageLayout {
  const margins = {
    top: mmToPx(pageSetup.margins.top),
    right: mmToPx(pageSetup.margins.right),
    bottom: mmToPx(pageSetup.margins.bottom),
    left: mmToPx(pageSetup.margins.left),
  };
  const headerHeight = mmToPx(pageSetup.headerHeight);
  const footerHeight = mmToPx(pageSetup.footerHeight);
  const contentHeight =
    PAGE_H - margins.top - margins.bottom - headerHeight - footerHeight;
  const contentWidth = PAGE_W - margins.left - margins.right;

  const pages: Block[][] = [];
  let currentPage: Block[] = [];
  let usedHeight = 0;

  for (let i = 0; i < blocks.length; i++) {
    const block = blocks[i];
    const height = blockHeights.get(block.id) || 0;

    // Force page break before this block
    if (block.meta.pageBreakBefore && currentPage.length > 0) {
      pages.push(currentPage);
      currentPage = [];
      usedHeight = 0;
    }

    // Would this block overflow the page?
    if (usedHeight + height > contentHeight && currentPage.length > 0) {
      pages.push(currentPage);
      currentPage = [];
      usedHeight = 0;
    }

    currentPage.push(block);
    usedHeight += height;

    // keepWithNext: if this block has keepWithNext and the NEXT block would
    // start on a new page, move this block to the new page too
    if (block.meta.keepWithNext && i + 1 < blocks.length) {
      const nextHeight = blockHeights.get(blocks[i + 1].id) || 0;
      if (usedHeight + nextHeight > contentHeight && currentPage.length > 1) {
        // Remove this block from current page, start new page with it
        currentPage.pop();
        usedHeight -= height;
        pages.push(currentPage);
        currentPage = [block];
        usedHeight = height;
      }
    }
  }

  if (currentPage.length > 0) {
    pages.push(currentPage);
  }

  // Ensure at least one empty page
  if (pages.length === 0) {
    pages.push([]);
  }

  return {
    pages,
    pageHeight: PAGE_H,
    contentHeight,
    contentWidth,
    margins,
    headerHeight,
    footerHeight,
  };
}
