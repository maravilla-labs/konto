import type { DocumentModel } from '../../lib/editor/types';
import { PrintView } from './PrintView';

interface PdfPreviewPanelProps {
  document: DocumentModel;
  blockHeights: Map<string, number>;
  onPrint: () => void;
}

/**
 * Side panel that shows a scaled-down live preview of the document
 * exactly as it will appear when printed to PDF.
 */
export function PdfPreviewPanel({
  document: doc,
  blockHeights,
  onPrint,
}: PdfPreviewPanelProps) {
  return (
    <div className="w-80 border-l bg-gray-50 flex flex-col">
      <div className="flex items-center justify-between px-4 py-2 border-b bg-white">
        <h3 className="text-sm font-semibold">PDF Preview</h3>
        <button
          onClick={onPrint}
          className="px-3 py-1 bg-blue-600 text-white text-xs rounded hover:bg-blue-700"
        >
          Print / Save PDF
        </button>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        <div
          style={{
            transform: 'scale(0.35)',
            transformOrigin: 'top left',
            width: 794,
          }}
        >
          <PrintView document={doc} blockHeights={blockHeights} />
        </div>
      </div>
    </div>
  );
}
