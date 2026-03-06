import type { Block, TableBlockData } from '../../lib/editor/types';
import { EMPTY_INVOICE_ROW } from '../../lib/editor/block-factory';
import { Plus, Trash2 } from 'lucide-react';

interface InvoiceTableBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  locked?: boolean;
}

function parseNum(val: string): number {
  const n = parseFloat(val);
  return isNaN(n) ? 0 : n;
}

function calcTotal(row: string[]): string {
  const qty = parseNum(row[1]);
  const price = parseNum(row[3]);
  const disc = parseNum(row[4]);
  return (qty * price * (1 - disc / 100)).toFixed(2);
}

function calcSubtotal(rows: string[][]): string {
  let sum = 0;
  for (const row of rows) {
    sum += parseNum(calcTotal(row));
  }
  return sum.toFixed(2);
}

const INPUT_CLASS =
  'border-0 bg-transparent text-sm outline-none focus:bg-blue-50 transition-colors px-2 py-1.5';

export function InvoiceTableBlock({ block, onChange, locked }: InvoiceTableBlockProps) {
  const data = block.data as TableBlockData;

  function updateCell(row: number, col: number, value: string) {
    const newRows = data.rows.map((r, ri) =>
      ri === row ? r.map((c, ci) => (ci === col ? value : c)) : r,
    );
    onChange({ ...block, data: { ...data, rows: newRows } });
  }

  function addRow() {
    onChange({
      ...block,
      data: {
        ...data,
        rows: [...data.rows, [...EMPTY_INVOICE_ROW]],
      },
    });
  }

  function removeRow(index: number) {
    if (data.rows.length <= 1) return;
    onChange({
      ...block,
      data: { ...data, rows: data.rows.filter((_, i) => i !== index) },
    });
  }

  return (
    <div className="my-2">
      <table className="w-full border border-gray-200 rounded text-sm">
        <thead>
          <tr className="bg-gray-50">
            {data.headers.map((h, ci) => (
              <th
                key={ci}
                className={`px-2 py-2 text-xs font-medium text-gray-500 uppercase border-b border-gray-200 ${
                  ci === 0 ? 'text-left' : 'text-right'
                }`}
              >
                {h}
              </th>
            ))}
            {!locked && <th className="w-8 border-b border-gray-200" />}
          </tr>
        </thead>
        <tbody>
          {data.rows.map((row, ri) => {
            const total = calcTotal(row);
            return (
              <tr key={ri} className="group border-b border-gray-100">
                {row.map((cell, ci) => {
                  if (ci === 5) {
                    return (
                      <td
                        key={ci}
                        className="px-2 py-1.5 text-right font-mono text-sm text-gray-900"
                      >
                        {total}
                      </td>
                    );
                  }
                  return (
                    <td key={ci} className="px-0 py-0">
                      <input
                        value={cell}
                        onChange={(e) => updateCell(ri, ci, e.target.value)}
                        readOnly={locked}
                        className={`${INPUT_CLASS} w-full ${
                          ci === 0 ? 'text-left' : 'text-right'
                        }`}
                      />
                    </td>
                  );
                })}
                {!locked && (
                  <td className="px-1 py-1.5 text-center">
                    <button
                      onClick={() => removeRow(ri)}
                      className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 transition-opacity"
                    >
                      <Trash2 className="w-3.5 h-3.5" />
                    </button>
                  </td>
                )}
              </tr>
            );
          })}
        </tbody>
        <tfoot>
          <tr className="border-t-2 border-gray-300">
            <td
              colSpan={5}
              className="px-2 py-2 text-right text-sm font-semibold text-gray-700"
            >
              Subtotal
            </td>
            <td className="px-2 py-2 text-right font-semibold font-mono text-sm text-gray-900">
              {calcSubtotal(data.rows)}
            </td>
            {!locked && <td />}
          </tr>
        </tfoot>
      </table>
      {!locked && (
        <div className="mt-1">
          <button
            onClick={addRow}
            className="flex items-center gap-1 text-xs text-blue-600 hover:underline"
          >
            <Plus className="w-3 h-3" />
            Add row
          </button>
        </div>
      )}
    </div>
  );
}
