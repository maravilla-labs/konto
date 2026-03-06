import type { Block, TableBlockData } from '../../lib/editor/types';
import { INVOICE_HEADERS, EMPTY_INVOICE_ROW } from '../../lib/editor/block-factory';
import { InspectorCommon, Field } from './InspectorCommon';

interface InspectorTablePanelProps {
  block: Block;
  onChange: (block: Block) => void;
  templateMode?: boolean;
}

export function InspectorTablePanel({
  block,
  onChange,
  templateMode,
}: InspectorTablePanelProps) {
  const data = block.data as TableBlockData;
  const mode = data.mode ?? 'generic';
  const isInvoice = mode === 'invoice';

  function updateData(updates: Partial<TableBlockData>) {
    onChange({
      ...block,
      data: { ...data, ...updates } as TableBlockData,
    });
  }

  function switchToInvoice() {
    updateData({
      mode: 'invoice',
      headers: [...INVOICE_HEADERS],
      rows: [[...EMPTY_INVOICE_ROW]],
      columnWidths: undefined,
    });
  }

  function switchToGeneric() {
    updateData({
      mode: 'generic',
      headers: ['Column 1', 'Column 2', 'Column 3'],
      rows: [['', '', '']],
      columnWidths: undefined,
    });
  }

  function addColumn() {
    const colNum = data.headers.length + 1;
    updateData({
      headers: [...data.headers, `Column ${colNum}`],
      rows: data.rows.map((row) => [...row, '']),
    });
  }

  function removeColumn() {
    if (data.headers.length <= 1) return;
    updateData({
      headers: data.headers.slice(0, -1),
      rows: data.rows.map((row) => row.slice(0, -1)),
    });
  }

  function addRow() {
    const emptyRow = isInvoice
      ? [...EMPTY_INVOICE_ROW]
      : new Array(data.headers.length).fill('');
    updateData({ rows: [...data.rows, emptyRow] });
  }

  return (
    <div className="space-y-4">
      <Field label="Type">
        <span className="text-sm font-mono">table</span>
      </Field>

      <Field label="Mode">
        <div className="flex gap-1">
          <button
            onClick={switchToGeneric}
            className={`flex-1 py-1 rounded text-xs ${
              !isInvoice ? 'bg-gray-200 font-medium' : 'hover:bg-gray-100'
            }`}
          >
            Generic
          </button>
          <button
            onClick={switchToInvoice}
            className={`flex-1 py-1 rounded text-xs ${
              isInvoice ? 'bg-gray-200 font-medium' : 'hover:bg-gray-100'
            }`}
          >
            Invoice
          </button>
        </div>
      </Field>

      {!isInvoice && (
        <Field label="Columns">
          <div className="flex items-center gap-2">
            <span className="text-sm">{data.headers.length}</span>
            <button
              onClick={removeColumn}
              disabled={data.headers.length <= 1}
              className="px-2 py-0.5 rounded border text-xs hover:bg-gray-100 disabled:opacity-40"
            >
              -
            </button>
            <button
              onClick={addColumn}
              className="px-2 py-0.5 rounded border text-xs hover:bg-gray-100"
            >
              +
            </button>
          </div>
        </Field>
      )}

      <Field label="Rows">
        <div className="flex items-center gap-2">
          <span className="text-sm">{data.rows.length}</span>
          <button
            onClick={addRow}
            className="px-2 py-0.5 rounded border text-xs hover:bg-gray-100"
          >
            + Add Row
          </button>
        </div>
      </Field>

      <InspectorCommon block={block} onChange={onChange} templateMode={templateMode} />
    </div>
  );
}
