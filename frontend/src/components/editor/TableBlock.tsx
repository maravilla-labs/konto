import type { Block, TableBlockData } from '../../lib/editor/types';

interface TableBlockProps {
  block: Block;
  onChange: (block: Block) => void;
  locked?: boolean;
}

export function TableBlock({ block, onChange, locked }: TableBlockProps) {
  const data = block.data as TableBlockData;

  function updateCell(row: number, col: number, value: string) {
    const newRows = data.rows.map((r, ri) =>
      ri === row ? r.map((c, ci) => (ci === col ? value : c)) : r,
    );
    onChange({ ...block, data: { ...data, rows: newRows } });
  }

  function updateHeader(col: number, value: string) {
    const newHeaders = data.headers.map((h, i) => (i === col ? value : h));
    onChange({ ...block, data: { ...data, headers: newHeaders } });
  }

  function addRow() {
    onChange({
      ...block,
      data: {
        ...data,
        rows: [...data.rows, data.headers.map(() => '')],
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

  function addColumn() {
    onChange({
      ...block,
      data: {
        ...data,
        headers: [...data.headers, 'Column'],
        rows: data.rows.map((r) => [...r, '']),
      },
    });
  }

  function removeColumn(index: number) {
    if (data.headers.length <= 1) return;
    onChange({
      ...block,
      data: {
        ...data,
        headers: data.headers.filter((_, i) => i !== index),
        rows: data.rows.map((r) => r.filter((_, i) => i !== index)),
      },
    });
  }

  return (
    <div className="my-2">
      <table className="w-full border-collapse border border-gray-300 text-sm">
        <thead>
          <tr className="bg-gray-50">
            {data.headers.map((h, ci) => (
              <th
                key={ci}
                className="border border-gray-300 px-3 py-2 text-left font-semibold"
              >
                {locked ? (
                  h
                ) : (
                  <input
                    value={h}
                    onChange={(e) => updateHeader(ci, e.target.value)}
                    className="w-full bg-transparent outline-none font-semibold"
                  />
                )}
              </th>
            ))}
            {!locked && (
              <th className="w-8 border border-gray-300" />
            )}
          </tr>
        </thead>
        <tbody>
          {data.rows.map((row, ri) => (
            <tr key={ri}>
              {row.map((cell, ci) => (
                <td
                  key={ci}
                  className="border border-gray-300 px-3 py-1.5"
                >
                  {locked ? (
                    cell
                  ) : (
                    <input
                      value={cell}
                      onChange={(e) => updateCell(ri, ci, e.target.value)}
                      className="w-full bg-transparent outline-none"
                    />
                  )}
                </td>
              ))}
              {!locked && (
                <td className="border border-gray-300 px-1">
                  <button
                    onClick={() => removeRow(ri)}
                    className="text-gray-400 hover:text-red-500 text-xs"
                  >
                    x
                  </button>
                </td>
              )}
            </tr>
          ))}
        </tbody>
      </table>
      {!locked && (
        <div className="flex gap-2 mt-1">
          <button
            onClick={addRow}
            className="text-xs text-blue-600 hover:underline"
          >
            + Add row
          </button>
          <button
            onClick={addColumn}
            className="text-xs text-blue-600 hover:underline"
          >
            + Add column
          </button>
          {data.headers.length > 1 && (
            <button
              onClick={() => removeColumn(data.headers.length - 1)}
              className="text-xs text-red-500 hover:underline"
            >
              - Remove column
            </button>
          )}
        </div>
      )}
    </div>
  );
}
