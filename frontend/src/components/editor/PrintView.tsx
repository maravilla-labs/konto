import { useMemo } from 'react';
import type { DocumentModel, TextBlockData } from '../../lib/editor/types';
import { PAGE_W, PAGE_H } from '../../lib/editor/types';
import { paginate } from '../../lib/editor/pagination';
import { PRINT_CSS } from '../../lib/editor/pdf-export';
import { EditorBlock } from './EditorBlock';

interface PrintViewProps {
  document: DocumentModel;
  blockHeights: Map<string, number>;
}

/**
 * Print-ready view of the document. Uses the same pagination algorithm
 * as the editor to ensure 1:1 correspondence.
 */
export function PrintView({ document: doc, blockHeights }: PrintViewProps) {
  const layout = useMemo(
    () => paginate(doc.blocks, blockHeights, doc.pageSetup),
    [doc.blocks, blockHeights, doc.pageSetup],
  );

  return (
    <>
      <style>{PRINT_CSS}</style>
      <div id="print-document">
        {layout.pages.map((pageBlocks, pageIdx) => (
          <div
            key={pageIdx}
            className="editor-page bg-white"
            style={{
              width: PAGE_W,
              minHeight: PAGE_H,
              padding: `${layout.margins.top}px ${layout.margins.right}px ${layout.margins.bottom}px ${layout.margins.left}px`,
              position: 'relative',
            }}
          >
            {/* Header */}
            {doc.header.length > 0 && (
              <div style={{ height: layout.headerHeight, marginBottom: 8 }}>
                {doc.header.map((b) => (
                  <div
                    key={b.id}
                    className="text-xs text-gray-400"
                    dangerouslySetInnerHTML={{
                      __html:
                        (b.data as TextBlockData)._html ||
                        (b.data as TextBlockData).text ||
                        '',
                    }}
                  />
                ))}
              </div>
            )}

            {/* Content */}
            <div style={{ minHeight: layout.contentHeight }}>
              {pageBlocks.map((block) => (
                <div
                  key={block.id}
                  data-keep-with-next={
                    block.meta.keepWithNext ? 'true' : undefined
                  }
                  data-page-break-before={
                    block.meta.pageBreakBefore ? 'true' : undefined
                  }
                >
                  <EditorBlock
                    block={block}
                    onChange={() => {}}
                    onKeyDown={() => {}}
                    onFocus={() => {}}
                    isActive={false}
                    templateMode={false}
                    blockRef={() => {}}
                  />
                </div>
              ))}
            </div>

            {/* Footer */}
            <div
              style={{ height: layout.footerHeight }}
              className="absolute bottom-0 left-0 right-0 px-6"
            >
              <div className="flex justify-between text-xs text-gray-500">
                {doc.footer.map((b) => (
                  <span
                    key={b.id}
                    dangerouslySetInnerHTML={{
                      __html:
                        (b.data as TextBlockData)._html ||
                        (b.data as TextBlockData).text ||
                        '',
                    }}
                  />
                ))}
                <span>
                  Page {pageIdx + 1} / {layout.pages.length}
                </span>
              </div>
            </div>
          </div>
        ))}
      </div>
    </>
  );
}
