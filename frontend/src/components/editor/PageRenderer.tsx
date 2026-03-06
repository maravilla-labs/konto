import { useMemo } from 'react';
import type { Block, PageSetup, ViewMode } from '../../lib/editor/types';
import { PAGE_W, PAGE_H } from '../../lib/editor/types';
import { paginate } from '../../lib/editor/pagination';

interface PageRendererProps {
  blocks: Block[];
  pageSetup: PageSetup;
  header: Block[];
  footer: Block[];
  renderBlock: (block: Block, pageIndex: number) => React.ReactNode;
  renderHeader?: (
    pageIndex: number,
    totalPages: number,
  ) => React.ReactNode;
  renderFooter?: (
    pageIndex: number,
    totalPages: number,
  ) => React.ReactNode;
  blockHeights: Map<string, number>;
  viewMode: ViewMode;
  zoom: number;
}

export function PageRenderer({
  blocks,
  pageSetup,
  renderBlock,
  renderHeader,
  renderFooter,
  blockHeights,
  viewMode,
  zoom,
}: PageRendererProps) {
  const layout = useMemo(
    () => paginate(blocks, blockHeights, pageSetup),
    [blocks, blockHeights, pageSetup],
  );

  // Canvas mode: no pagination, just render all blocks flowing
  if (viewMode === 'canvas') {
    return (
      <div
        className="max-w-[794px] mx-auto bg-white p-10"
        style={{ transform: `scale(${zoom})`, transformOrigin: 'top center' }}
      >
        {blocks.map((block) => (
          <div key={block.id}>{renderBlock(block, 0)}</div>
        ))}
      </div>
    );
  }

  const isSpread = viewMode === 'spread';

  return (
    <div
      className={`flex ${
        isSpread
          ? 'flex-row flex-wrap justify-center gap-4'
          : 'flex-col items-center gap-8'
      }`}
      style={{ transform: `scale(${zoom})`, transformOrigin: 'top center' }}
    >
      {layout.pages.map((pageBlocks, pageIdx) => (
        <div
          key={pageIdx}
          className="bg-white shadow-lg relative"
          style={{
            width: PAGE_W,
            minHeight: PAGE_H,
            padding: `${layout.margins.top}px ${layout.margins.right}px ${layout.margins.bottom}px ${layout.margins.left}px`,
          }}
        >
          {/* Header zone */}
          {renderHeader && (
            <div
              style={{ height: layout.headerHeight, marginBottom: 8 }}
              className="border-b border-transparent"
            >
              {renderHeader(pageIdx, layout.pages.length)}
            </div>
          )}

          {/* Content zone */}
          <div style={{ minHeight: layout.contentHeight }}>
            {pageBlocks.map((block) => (
              <div key={block.id}>{renderBlock(block, pageIdx)}</div>
            ))}
          </div>

          {/* Footer zone */}
          {renderFooter && (
            <div
              style={{ height: layout.footerHeight }}
              className="border-t border-transparent mt-auto"
            >
              {renderFooter(pageIdx, layout.pages.length)}
            </div>
          )}

          {/* Page number */}
          <div className="absolute bottom-4 right-6 text-xs text-gray-400">
            Page {pageIdx + 1} / {layout.pages.length}
          </div>
        </div>
      ))}
    </div>
  );
}
